use super::*;
use std::{path::PathBuf, process::Command};

const REPLACEMENT: &str = "reset_0123456789abcdef0123456789abcdef";
const RETIRED: &str = "retired_0123456789abcdef0123456789abcdef";
const NEW_OID: u32 = 1_900_000_101;
const JOURNAL: &str = "CREATE SCHEMA IF NOT EXISTS academy_runtime AUTHORIZATION academy_admin;
REVOKE ALL ON SCHEMA academy_runtime FROM PUBLIC;
CREATE TABLE IF NOT EXISTS academy_runtime.reset_operations (
 database_name text PRIMARY KEY, old_oid oid NOT NULL, new_oid oid NOT NULL UNIQUE,
 owner_oid oid NOT NULL, replacement_name text NOT NULL UNIQUE,
 retired_name text NOT NULL UNIQUE, committed boolean NOT NULL DEFAULT false);
REVOKE ALL ON TABLE academy_runtime.reset_operations FROM PUBLIC;";

fn runtime() -> PathBuf {
    std::env::var_os("ACADEMY_POSTGRES")
        .map(PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into())
}

async fn reserve_fixture(academy: &Academy) -> Client {
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    admin.batch_execute(JOURNAL).await.unwrap();
    admin.execute("INSERT INTO academy_runtime.reset_operations(database_name,old_oid,new_oid,owner_oid,replacement_name,retired_name) SELECT datname,oid,$1,datdba,$2,$3 FROM pg_database WHERE datname='lab_01_01'", &[&NEW_OID,&REPLACEMENT,&RETIRED]).await.unwrap();
    admin
}

async fn allocate_fixture(academy: &Academy, admin: &Client, seed: bool) {
    admin
        .batch_execute(&format!("CREATE DATABASE {REPLACEMENT} WITH OID={NEW_OID}"))
        .await
        .unwrap();
    if seed {
        let staged = academy
            .cluster
            .connect(REPLACEMENT, "academy_admin")
            .await
            .unwrap();
        let setup = Fixture::Orders.sql(0).unwrap();
        staged.batch_execute(&format!("REVOKE ALL ON DATABASE {REPLACEMENT} FROM PUBLIC; GRANT CONNECT ON DATABASE {REPLACEMENT} TO student_01_01; REVOKE CREATE ON SCHEMA public FROM PUBLIC; {setup} GRANT USAGE ON SCHEMA public TO student_01_01; GRANT SELECT ON ALL TABLES IN SCHEMA public TO student_01_01;")).await.unwrap();
    }
}

// Test-only subprocess: exit deliberately bypasses Cluster::Drop at real SQL boundaries.
#[tokio::test]
#[ignore = "invoked only by reset_recovery_survives_real_app_and_server_interruptions"]
async fn reset_crash_worker() {
    let Some(home) = std::env::var_os("ACADEMY_RESET_CRASH_HOME") else {
        return;
    };
    let phase = std::env::var("ACADEMY_RESET_CRASH_PHASE").unwrap();
    let mut academy = Academy::start(Path::new(&home), &runtime()).await.unwrap();
    academy.chapters[0].exercises[0].require_analyze = true;
    let e = academy.chapters[0].exercises[0].clone();
    let request = |mode: &str| RunRequest {
        exercise_id: e.id.clone(),
        sql: e.solution_sql.clone(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    };
    assert!(academy
        .run(request("analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    academy.store.pass(&e.id, &e.revision).unwrap();
    let old = academy
        .cluster
        .connect("lab_01_01", "academy_admin")
        .await
        .unwrap();
    old.batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
        .await
        .unwrap();
    drop(old);
    let admin = reserve_fixture(&academy).await;
    if phase != "reserved" {
        allocate_fixture(&academy, &admin, phase != "allocated").await;
    }
    if ["renaming", "committed", "cleaned"].contains(&phase.as_str()) {
        admin.execute("SELECT pg_terminate_backend(pid,2000) FROM pg_stat_activity WHERE datname IN ('lab_01_01',$1) AND pid<>pg_backend_pid()",&[&REPLACEMENT]).await.unwrap();
        admin
            .batch_execute(&format!(
                "BEGIN; ALTER DATABASE lab_01_01 RENAME TO {RETIRED};"
            ))
            .await
            .unwrap();
        if phase != "renaming" {
            let mut sql=format!("ALTER DATABASE {REPLACEMENT} RENAME TO lab_01_01; UPDATE academy_runtime.reset_operations SET committed=true WHERE database_name='lab_01_01'; COMMIT;");
            if phase == "committed" {
                sql.push_str("SELECT pg_terminate_backend(pg_backend_pid());");
                assert!(
                    admin.batch_execute(&sql).await.is_err(),
                    "A lost batch response must not be mistaken for rollback"
                );
            } else {
                admin.batch_execute(&sql).await.unwrap();
            }
        }
        if phase == "cleaned" {
            academy.cluster.drop_database(RETIRED).await.unwrap();
        }
    }
    std::process::exit(86);
}

#[tokio::test]
async fn reset_recovery_refuses_ambiguous_catalogs_and_unsafe_records() {
    for corruption in ["oid", "owner", "moved", "unsafe", "committed"] {
        let home = tempfile::tempdir().unwrap();
        let academy = Academy::start(home.path(), &runtime()).await.unwrap();
        academy.save("01-01", "SELECT id FROM orders").unwrap();
        let admin = reserve_fixture(&academy).await;
        allocate_fixture(&academy, &admin, true).await;
        let mut remaining = REPLACEMENT;
        match corruption {
            "oid" => {
                admin
                    .execute(
                        "UPDATE academy_runtime.reset_operations SET new_oid=$1",
                        &[&(NEW_OID + 1)],
                    )
                    .await
                    .unwrap();
            }
            "owner" => {
                admin.batch_execute(&format!("CREATE ROLE foreign_owner NOLOGIN; ALTER DATABASE {REPLACEMENT} OWNER TO foreign_owner")).await.unwrap();
            }
            "moved" => {
                admin
                    .batch_execute(&format!(
                        "ALTER DATABASE {REPLACEMENT} RENAME TO unrelated_copy"
                    ))
                    .await
                    .unwrap();
                remaining = "unrelated_copy";
            }
            "unsafe" => {
                admin
                    .execute(
                        "UPDATE academy_runtime.reset_operations SET replacement_name=$1",
                        &[&"reset_x; DROP DATABASE lab_01_01; --"],
                    )
                    .await
                    .unwrap();
            }
            "committed" => {
                admin
                    .batch_execute("UPDATE academy_runtime.reset_operations SET committed=true")
                    .await
                    .unwrap();
            }
            _ => unreachable!(),
        }
        let error = academy.reset("01-01").await.unwrap_err();
        assert!(error.contains("copies were kept"), "{corruption}: {error}");
        let names = ["lab_01_01", remaining];
        assert_eq!(
            admin
                .query_one(
                    "SELECT COUNT(*) FROM pg_database WHERE datname=ANY($1)",
                    &[&names.as_slice()]
                )
                .await
                .unwrap()
                .get::<_, i64>(0),
            2,
            "{corruption}: neither copy may be deleted"
        );
        assert_eq!(
            academy.store.script("01-01").unwrap(),
            Some("SELECT id FROM orders".into())
        );
        assert_eq!(
            admin
                .query_one("SELECT COUNT(*) FROM academy_runtime.reset_operations", &[])
                .await
                .unwrap()
                .get::<_, i64>(0),
            1
        );
        drop(admin);
        academy.shutdown().await.unwrap();
        drop(academy);
        let startup = Academy::start(home.path(), &runtime()).await.err().unwrap();
        assert!(
            startup.contains("copies were kept"),
            "{corruption}: {startup}"
        );
        assert!(
            !home.path().join("runtime/data/postmaster.pid").exists(),
            "Rejected recovery must stop its new owned server"
        );
    }
}

#[tokio::test]
async fn reset_recovery_preserves_unjournaled_lookalikes_and_denies_learner_access() {
    let home = tempfile::tempdir().unwrap();
    let academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    admin
        .batch_execute(&format!("CREATE DATABASE {REPLACEMENT}"))
        .await
        .unwrap();
    admin
        .batch_execute(&format!("CREATE DATABASE {RETIRED}"))
        .await
        .unwrap();
    assert!(academy
        .cluster
        .connect("postgres", "student_01_01")
        .await
        .is_err());
    admin
        .batch_execute("GRANT CONNECT ON DATABASE postgres TO student_01_01")
        .await
        .unwrap();
    let learner = academy
        .cluster
        .connect("postgres", "student_01_01")
        .await
        .unwrap();
    for sql in [
        "SELECT * FROM academy_runtime.reset_operations",
        "DELETE FROM academy_runtime.reset_operations",
        "CREATE TABLE academy_runtime.forged (id int)",
    ] {
        let error = learner.batch_execute(sql).await.unwrap_err();
        assert_eq!(error.code().unwrap().code(), "42501", "{sql}");
    }
    drop(learner);
    admin
        .batch_execute("REVOKE CONNECT ON DATABASE postgres FROM student_01_01")
        .await
        .unwrap();
    drop(admin);
    academy.shutdown().await.unwrap();
    drop(academy);
    let reopened = Academy::start(home.path(), &runtime()).await.unwrap();
    reopened.reset("01-01").await.unwrap();
    let admin = reopened
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let names = [REPLACEMENT, RETIRED];
    assert_eq!(
        admin
            .query_one(
                "SELECT COUNT(*) FROM pg_database WHERE datname=ANY($1)",
                &[&names.as_slice()]
            )
            .await
            .unwrap()
            .get::<_, i64>(0),
        2,
        "Prefix lookalikes have no journal and must be retained"
    );
    assert_eq!(
        admin
            .query_one("SELECT COUNT(*) FROM academy_runtime.reset_operations", &[])
            .await
            .unwrap()
            .get::<_, i64>(0),
        0
    );
    reopened.shutdown().await.unwrap();
}

#[tokio::test]
async fn reset_recovery_reads_the_catalog_after_a_real_promotion_connection_loss() {
    let home = tempfile::tempdir().unwrap();
    let academy = Academy::start(home.path(), &runtime()).await.unwrap();
    academy.save("01-01", "SELECT id FROM orders").unwrap();
    let old = academy
        .cluster
        .connect("lab_01_01", "academy_admin")
        .await
        .unwrap();
    old.batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
        .await
        .unwrap();
    drop(old);
    let before = academy
        .cluster
        .lab_identities(&["lab_01_01".into()])
        .await
        .unwrap();
    let allocation_lock = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    allocation_lock
        .batch_execute("BEGIN; LOCK TABLE pg_catalog.pg_database IN SHARE MODE")
        .await
        .unwrap();
    let journal_lock = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let observer = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let interrupt = async {
        tokio::time::timeout(Duration::from_secs(5),async {
            loop {
                if observer.query_one("SELECT EXISTS(SELECT FROM academy_runtime.reset_operations WHERE database_name='lab_01_01')",&[]).await.unwrap().get::<_,bool>(0) {break;}
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }).await.expect("Reservation must precede the catalog-blocked allocation");
        journal_lock.batch_execute("BEGIN; SELECT FROM academy_runtime.reset_operations WHERE database_name='lab_01_01' FOR UPDATE").await.unwrap();
        allocation_lock.batch_execute("COMMIT").await.unwrap();
        let pid=tokio::time::timeout(Duration::from_secs(5),async {
            loop {
                if let Some(row)=observer.query_opt("SELECT pid FROM pg_stat_activity WHERE datname='postgres' AND query LIKE 'UPDATE academy_runtime.reset_operations SET committed=true%' AND wait_event_type='Lock'",&[]).await.unwrap() {break row.get::<_,i32>(0);}
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        }).await.expect("Promotion must be observed waiting at the real journal-update boundary");
        assert!(observer
            .query_one("SELECT pg_terminate_backend($1,2000)", &[&pid])
            .await
            .unwrap()
            .get::<_, bool>(0));
        journal_lock.batch_execute("COMMIT").await.unwrap();
    };
    let (reset, ()) = tokio::join!(academy.reset("01-01"), interrupt);
    assert!(
        reset.is_err(),
        "The uncommitted interrupted Reset must report failure"
    );
    assert_eq!(
        academy
            .cluster
            .lab_identities(&["lab_01_01".into()])
            .await
            .unwrap(),
        before
    );
    let learner = academy
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    assert_eq!(
        learner
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "999.99"
    );
    assert_eq!(
        observer
            .query_one("SELECT COUNT(*) FROM academy_runtime.reset_operations", &[])
            .await
            .unwrap()
            .get::<_, i64>(0),
        0
    );
    assert!(!observer.query_one("SELECT EXISTS(SELECT FROM pg_database WHERE datname LIKE 'reset_%' OR datname LIKE 'retired_%')",&[]).await.unwrap().get::<_,bool>(0));
    assert_eq!(
        academy.store.script("01-01").unwrap(),
        Some("SELECT id FROM orders".into())
    );
    drop(learner);
    academy.reset("01-01").await.unwrap();
    let baseline = academy
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    assert_eq!(
        baseline
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "125.50"
    );
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn reset_recovery_retries_failed_retired_cleanup_without_losing_the_baseline() {
    let home = tempfile::tempdir().unwrap();
    let academy = Academy::start(home.path(), &runtime()).await.unwrap();
    academy
        .save("01-01", "SELECT COUNT(*) FROM orders")
        .unwrap();
    let admin = reserve_fixture(&academy).await;
    allocate_fixture(&academy, &admin, true).await;
    admin.execute("SELECT pg_terminate_backend(pid,2000) FROM pg_stat_activity WHERE datname IN ('lab_01_01',$1) AND pid<>pg_backend_pid()",&[&REPLACEMENT]).await.unwrap();
    admin.batch_execute(&format!("BEGIN; ALTER DATABASE lab_01_01 RENAME TO {RETIRED}; ALTER DATABASE {REPLACEMENT} RENAME TO lab_01_01; UPDATE academy_runtime.reset_operations SET committed=true; COMMIT; ALTER DATABASE {RETIRED} IS_TEMPLATE true;")).await.unwrap();
    let error = academy.reset("01-01").await.unwrap_err();
    assert!(error.contains("db error"), "{error}");
    let learner = academy
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    assert_eq!(
        learner
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "125.50"
    );
    assert_eq!(
        admin
            .query_one("SELECT COUNT(*) FROM academy_runtime.reset_operations", &[])
            .await
            .unwrap()
            .get::<_, i64>(0),
        1
    );
    drop(learner);
    admin
        .batch_execute(&format!("ALTER DATABASE {RETIRED} IS_TEMPLATE false"))
        .await
        .unwrap();
    academy.reset("01-01").await.unwrap();
    assert_eq!(
        admin
            .query_one("SELECT COUNT(*) FROM academy_runtime.reset_operations", &[])
            .await
            .unwrap()
            .get::<_, i64>(0),
        0
    );
    assert_eq!(
        academy.store.script("01-01").unwrap(),
        Some("SELECT COUNT(*) FROM orders".into())
    );
    assert!(!admin
        .query_one(
            "SELECT EXISTS(SELECT FROM pg_database WHERE datname=$1)",
            &[&RETIRED]
        )
        .await
        .unwrap()
        .get::<_, bool>(0));
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn reset_recovery_survives_real_app_and_server_interruptions() {
    for phase in [
        "reserved",
        "allocated",
        "staged",
        "renaming",
        "committed",
        "cleaned",
    ] {
        let home = tempfile::tempdir().unwrap();
        let child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                "service::reset_recovery_tests::reset_crash_worker",
                "--nocapture",
                "--test-threads=1",
            ])
            .env("ACADEMY_RESET_CRASH_HOME", home.path())
            .env("ACADEMY_RESET_CRASH_PHASE", phase)
            .output()
            .unwrap();
        assert_eq!(
            child.status.code(),
            Some(86),
            "{phase}: {}",
            String::from_utf8_lossy(&child.stderr)
        );
        let data = home.path().join("runtime/data");
        let pid = std::fs::read_to_string(data.join("postmaster.pid")).unwrap();
        assert_eq!(
            Path::new(pid.lines().nth(1).unwrap()),
            data.canonicalize().unwrap()
        );
        let stopped = Command::new(runtime().join("bin/pg_ctl"))
            .arg("stop")
            .arg("-D")
            .arg(&data)
            .args(["-m", "immediate", "-w", "-t", "5"])
            .output()
            .unwrap();
        assert!(
            stopped.status.success(),
            "{phase}: {}",
            String::from_utf8_lossy(&stopped.stderr)
        );
        let mut reopened = Academy::start(home.path(), &runtime()).await.unwrap();
        reopened.chapters[0].exercises[0].require_analyze = true;
        let e = &reopened.chapters[0].exercises[0];
        let committed = ["committed", "cleaned"].contains(&phase);
        let learner = reopened
            .cluster
            .connect("lab_01_01", "student_01_01")
            .await
            .unwrap();
        let total: String = learner
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get(0);
        assert_eq!(
            total,
            if committed { "125.50" } else { "999.99" },
            "{phase}"
        );
        let admin = reopened
            .cluster
            .connect("postgres", "academy_admin")
            .await
            .unwrap();
        let pending: i64 = admin
            .query_one("SELECT COUNT(*) FROM academy_runtime.reset_operations", &[])
            .await
            .unwrap()
            .get(0);
        assert_eq!(
            pending, 0,
            "{phase}: interrupted Reset must be reconciled before labs open"
        );
        assert!(
            !admin
                .query_one(
                    "SELECT EXISTS(SELECT FROM pg_database WHERE datname IN ($1,$2))",
                    &[&REPLACEMENT, &RETIRED]
                )
                .await
                .unwrap()
                .get::<_, bool>(0),
            "{phase}"
        );
        assert!(admin
            .query_one(
                "SELECT EXISTS(SELECT FROM pg_roles WHERE rolname='student_01_01')",
                &[]
            )
            .await
            .unwrap()
            .get::<_, bool>(0));
        let progress = reopened.progress().await.unwrap();
        assert_eq!(progress[&e.id]["sql"], e.solution_sql);
        assert_eq!(progress[&e.id]["passed"], true);
        assert_eq!(progress[&e.id]["unlocked"], true);
        assert_eq!(
            progress[&e.id]["analyzed"], !committed,
            "{phase}: analysis must follow actual catalog identity"
        );
        drop(learner);
        drop(admin);
        reopened.shutdown().await.unwrap();
        drop(reopened);
        let again = Academy::start(home.path(), &runtime()).await.unwrap();
        assert_eq!(
            again.progress().await.unwrap()["01-01"]["passed"],
            true,
            "Repeated restart must be idempotent"
        );
        again.shutdown().await.unwrap();
    }
}
