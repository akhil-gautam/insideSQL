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
    assert_eq!(
        new_identity,
        format!("{actual_system}:{new_oid}"),
        "The binding must reflect the actual server, not an invented identity"
    );
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
