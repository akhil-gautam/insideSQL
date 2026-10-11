use super::*;

fn runtime() -> std::path::PathBuf {
    std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into())
}
fn request(exercise: &Exercise, mode: &str) -> RunRequest {
    RunRequest {
        exercise_id: exercise.id.clone(),
        sql: exercise.solution_sql.clone(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    }
}
fn measured_first(academy: &mut Academy) -> Exercise {
    academy.chapters[0].exercises[0].require_analyze = true;
    academy.chapters[0].exercises[0].clone()
}

#[tokio::test]
async fn identity_historical_oid_reuse_does_not_restore_retained_proof() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut academy);
    academy.run(request(&e, "analyze")).await.unwrap();
    academy.store.pass(&e.id, &e.revision).unwrap();
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let original_oid: u32 = admin
        .query_one("SELECT oid FROM pg_database WHERE datname='lab_01_01'", &[])
        .await
        .unwrap()
        .get(0);
    let sqlite = rusqlite::Connection::open(home.path().join("progress.sqlite")).unwrap();
    sqlite.execute_batch("CREATE TRIGGER fail_analysis_delete BEFORE DELETE ON analysis BEGIN SELECT RAISE(FAIL,'retained historical proof'); END;").unwrap();
    assert!(academy
        .reset(&e.id)
        .await
        .unwrap_err()
        .contains("retained historical proof"));
    let second_oid: u32 = admin
        .query_one("SELECT oid FROM pg_database WHERE datname='lab_01_01'", &[])
        .await
        .unwrap()
        .get(0);
    assert_ne!(original_oid, second_oid);
    assert_eq!(academy.progress().await.unwrap()[&e.id]["analyzed"], false);

    // Reproduce A→B→A deterministically, including interruption before allocation registration.
    academy.cluster.drop_database("lab_01_01").await.unwrap();
    admin
        .batch_execute(&format!(
            "CREATE DATABASE lab_01_01 WITH OID={original_oid}"
        ))
        .await
        .unwrap();
    let recycled = academy
        .cluster
        .connect("lab_01_01", "academy_admin")
        .await
        .unwrap();
    let setup = Fixture::Orders.sql(0).unwrap();
    recycled.batch_execute(&format!("REVOKE ALL ON DATABASE lab_01_01 FROM PUBLIC; GRANT CONNECT ON DATABASE lab_01_01 TO student_01_01; REVOKE CREATE ON SCHEMA public FROM PUBLIC; {setup} GRANT USAGE ON SCHEMA public TO student_01_01; GRANT SELECT ON ALL TABLES IN SCHEMA public TO student_01_01;")).await.unwrap();
    drop(recycled);
    drop(admin);
    academy.shutdown().await.unwrap();
    drop(academy);
    let mut reopened = Academy::start(home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut reopened);
    assert!(reopened
        .store
        .has_analysis(&e.id, &e.revision, &e.solution_sql)
        .unwrap());
    assert_eq!(
        reopened.progress().await.unwrap()[&e.id]["analyzed"],
        false,
        "A recycled OID must not resurrect retained proof"
    );
    let rejected = reopened.check(request(&e, "run")).await.unwrap();
    assert!(
        !rejected.passed && rejected.variants_passed == 0 && rejected.message.contains("Analyze")
    );
    assert!(reopened.store.passed(&e.id, &e.revision).unwrap());
    assert_eq!(
        reopened.store.script(&e.id).unwrap(),
        Some(e.solution_sql.clone())
    );
    assert!(reopened
        .run(request(&e, "analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    assert!(reopened.check(request(&e, "run")).await.unwrap().passed);
    reopened.shutdown().await.unwrap();
    drop(reopened);
    let mut again = Academy::start(home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut again);
    assert_eq!(again.progress().await.unwrap()[&e.id]["analyzed"], true);
    assert!(again.check(request(&e, "run")).await.unwrap().passed);
    again.shutdown().await.unwrap();
}

#[tokio::test]
async fn identity_legacy_two_field_proof_requires_fresh_analysis() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut academy);
    assert!(academy
        .run(request(&e, "analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    academy.store.pass(&e.id, &e.revision).unwrap();
    let learner = academy
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    let row = learner.query_one("SELECT system_identifier::text, (SELECT oid FROM pg_database WHERE datname=current_database()) FROM pg_control_system()", &[]).await.unwrap();
    let legacy = format!("{}:{}", row.get::<_, String>(0), row.get::<_, u32>(1));
    let sqlite = rusqlite::Connection::open(home.path().join("progress.sqlite")).unwrap();
    sqlite
        .execute(
            "UPDATE analysis SET lab_identity=?1 WHERE exercise=?2",
            rusqlite::params![legacy, e.id],
        )
        .unwrap();
    assert_eq!(
        academy.progress().await.unwrap()[&e.id]["analyzed"],
        false,
        "Legacy evidence has no allocation generation"
    );
    assert!(!academy.check(request(&e, "run")).await.unwrap().passed);
    assert!(academy.store.passed(&e.id, &e.revision).unwrap());
    assert_eq!(
        academy.store.script(&e.id).unwrap(),
        Some(e.solution_sql.clone())
    );
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn identity_promoted_reset_rejects_stale_proof_when_sqlite_delete_fails() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut academy);
    assert!(academy
        .run(request(&e, "analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    assert!(academy.check(request(&e, "run")).await.unwrap().passed);
    let sqlite = rusqlite::Connection::open(home.path().join("progress.sqlite")).unwrap();
    sqlite.execute_batch("CREATE TRIGGER fail_analysis_delete BEFORE DELETE ON analysis BEGIN SELECT RAISE(FAIL,'test invalidation failure'); END;").unwrap();
    let before = academy
        .cluster
        .connect("lab_01_01", "academy_admin")
        .await
        .unwrap();
    let original: String = before
        .query_one("SELECT total::text FROM orders WHERE id=1", &[])
        .await
        .unwrap()
        .get(0);
    before
        .batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
        .await
        .unwrap();
    drop(before);
    assert!(academy
        .reset(&e.id)
        .await
        .unwrap_err()
        .contains("test invalidation failure"));
    let restored = academy
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    assert_eq!(
        restored
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        original
    );
    assert!(
        academy
            .store
            .has_analysis(&e.id, &e.revision, &e.solution_sql)
            .unwrap(),
        "The deliberately stale physical record remains"
    );
    assert_eq!(
        academy.progress().await.unwrap()[&e.id]["analyzed"],
        false,
        "Former-database proof must not be displayed as current"
    );
    let rejected = academy.check(request(&e, "run")).await.unwrap();
    assert!(!rejected.passed);
    assert_eq!(rejected.variants_passed, 0);
    assert!(rejected.message.contains("Analyze"));
    assert!(academy.store.passed(&e.id, &e.revision).unwrap());
    assert_eq!(
        academy.store.script(&e.id).unwrap(),
        Some(e.solution_sql.clone())
    );
    assert!(academy
        .run(request(&e, "analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    assert!(academy.progress().await.unwrap()[&e.id]["analyzed"]
        .as_bool()
        .unwrap());
    assert!(academy.check(request(&e, "run")).await.unwrap().passed);
    sqlite
        .execute_batch("DROP TRIGGER fail_analysis_delete")
        .unwrap();
    drop(sqlite);
    drop(restored);
    academy.shutdown().await.unwrap();
    drop(academy);
    let mut reopened = Academy::start(home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut reopened);
    assert!(
        reopened.progress().await.unwrap()[&e.id]["analyzed"]
            .as_bool()
            .unwrap(),
        "Same cluster/database restart must retain current proof"
    );
    assert!(reopened.check(request(&e, "run")).await.unwrap().passed);
    reopened.shutdown().await.unwrap();
}

#[tokio::test]
async fn identity_equal_database_oid_in_a_different_cluster_rejects_copied_proof() {
    let first_home = tempfile::tempdir().unwrap();
    let mut first = Academy::start(first_home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut first);
    assert!(first
        .run(request(&e, "analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    first.store.pass(&e.id, &e.revision).unwrap();
    let names = vec!["lab_01_01".to_owned()];
    let old_identity = first.cluster.lab_identities(&names).await.unwrap()["lab_01_01"].clone();
    let old_client = first
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    let old_oid: u32 = old_client
        .query_one(
            "SELECT oid FROM pg_catalog.pg_database WHERE datname=pg_catalog.current_database()",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    drop(old_client);
    first.shutdown().await.unwrap();
    drop(first);
    let second_home = tempfile::tempdir().unwrap();
    std::fs::copy(
        first_home.path().join("progress.sqlite"),
        second_home.path().join("progress.sqlite"),
    )
    .unwrap();
    let mut second = Academy::start(second_home.path(), &runtime())
        .await
        .unwrap();
    let e = measured_first(&mut second);
    let new_client = second
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    let new_oid: u32 = new_client
        .query_one(
            "SELECT oid FROM pg_catalog.pg_database WHERE datname=pg_catalog.current_database()",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    assert_eq!(
        new_oid, old_oid,
        "Independently initialized identical fixtures reproduce OID reuse"
    );
    let new_identity = second.cluster.lab_identities(&names).await.unwrap()["lab_01_01"].clone();
    assert_ne!(
        new_identity, old_identity,
        "Cluster identity must distinguish equal OIDs"
    );
    let actual_system: String = new_client
        .query_one(
            "SELECT system_identifier::text FROM pg_catalog.pg_control_system()",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    assert!(
        new_identity.starts_with(&format!("{actual_system}:{new_oid}:")),
        "The binding must reflect the actual server, not an invented identity"
    );
    assert!(uuid::Uuid::parse_str(new_identity.rsplit(':').next().unwrap()).is_ok());
    assert!(!second.progress().await.unwrap()[&e.id]["analyzed"]
        .as_bool()
        .unwrap());
    assert!(
        second.store.passed(&e.id, &e.revision).unwrap() && second.store.unlocked(&e.id).unwrap()
    );
    assert_eq!(
        second.store.script(&e.id).unwrap(),
        Some(e.solution_sql.clone())
    );
    let rejected = second.check(request(&e, "run")).await.unwrap();
    assert!(!rejected.passed && rejected.variants_passed == 0);
    assert!(rejected.message.contains("Analyze"));
    assert!(second
        .run(request(&e, "analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    assert!(second.check(request(&e, "run")).await.unwrap().passed);
    second.shutdown().await.unwrap();
}

#[tokio::test]
async fn identity_progress_does_not_allocate_labs_or_expose_tokens_and_fails_closed_offline() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let e = measured_first(&mut academy);
    assert!(academy
        .run(request(&e, "analyze"))
        .await
        .unwrap()
        .error
        .is_none());
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let before: i64 = admin
        .query_one("SELECT COUNT(*) FROM pg_catalog.pg_database", &[])
        .await
        .unwrap()
        .get(0);
    let progress = academy.progress().await.unwrap();
    for exercise in progress.as_object().unwrap().values() {
        let fields = exercise.as_object().unwrap();
        assert_eq!(fields.len(), 4);
        for key in ["sql", "unlocked", "passed", "analyzed"] {
            assert!(fields.contains_key(key));
        }
    }
    assert!(progress[&e.id]["analyzed"].as_bool().unwrap());
    assert_eq!(progress["02-01"]["analyzed"], false);
    let after: i64 = admin
        .query_one("SELECT COUNT(*) FROM pg_catalog.pg_database", &[])
        .await
        .unwrap()
        .get(0);
    assert_eq!(after, before, "Progress must not prepare a lazy lab");
    let resources = academy.resources(&e.id).await.unwrap();
    assert_eq!(resources.as_object().unwrap().len(), 6);
    assert!(resources.get("lab_identity").is_none());
    academy.shutdown().await.unwrap();
    assert!(
        academy.progress().await.is_err(),
        "An unavailable runtime cannot confirm current measurement proof"
    );
    assert_eq!(academy.store.script(&e.id).unwrap(), Some(e.solution_sql));
}
