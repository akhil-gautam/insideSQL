use super::*;

#[tokio::test]
async fn resource_reset_setup_failure_preserves_the_previous_database() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let academy = Academy::start(home.path(), &runtime).await.unwrap();
    academy
        .save("01-01", "SELECT COUNT(*) FROM orders")
        .unwrap();
    let admin = academy
        .cluster
        .connect("lab_01_01", "academy_admin")
        .await
        .unwrap();
    admin
        .batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
        .await
        .unwrap();
    drop(admin);
    let template = academy
        .cluster
        .connect("template1", "academy_admin")
        .await
        .unwrap();
    template
        .batch_execute("CREATE TABLE orders (id integer)")
        .await
        .unwrap();
    drop(template);
    let catalog = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    for _ in 0..50 {
        if !catalog
            .query_one(
                "SELECT EXISTS(SELECT FROM pg_stat_activity WHERE datname='template1')",
                &[],
            )
            .await
            .unwrap()
            .get::<_, bool>(0)
        {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert!(academy.reset("01-01").await.is_err());
    let admin = academy
        .cluster
        .connect("lab_01_01", "academy_admin")
        .await
        .expect("A rejected replacement must preserve the previous database");
    assert_eq!(
        admin
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "999.99"
    );
    assert_eq!(
        academy.store.script("01-01").unwrap(),
        Some("SELECT COUNT(*) FROM orders".into())
    );
    let leftover:bool=catalog.query_one("SELECT EXISTS(SELECT FROM pg_database WHERE datname LIKE 'reset_%' OR datname LIKE 'retired_%')",&[]).await.unwrap().get(0);
    assert!(
        !leftover,
        "Failed staging must leave no replacement/retired database"
    );
    let template = academy
        .cluster
        .connect("template1", "academy_admin")
        .await
        .unwrap();
    template.batch_execute("DROP TABLE orders").await.unwrap();
    drop(template);
    for _ in 0..50 {
        if !catalog
            .query_one(
                "SELECT EXISTS(SELECT FROM pg_stat_activity WHERE datname='template1')",
                &[],
            )
            .await
            .unwrap()
            .get::<_, bool>(0)
        {
            break;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    drop(admin);
    academy.reset("01-01").await.unwrap();
    let learner = academy
        .cluster
        .connect("lab_01_01", "student_01_01")
        .await
        .unwrap();
    assert_ne!(
        learner
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "999.99"
    );
    let leftover:bool=catalog.query_one("SELECT EXISTS(SELECT FROM pg_database WHERE datname LIKE 'reset_%' OR datname LIKE 'retired_%')",&[]).await.unwrap().get(0);
    assert!(!leftover);
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn resource_snapshot_is_scoped_and_low_disk_preserves_labs_and_progress() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    assert!(academy.resources("02-01").await.is_err());
    assert!(academy.resources("postgres").await.is_err());
    let observed = academy.resources("01-01").await.unwrap();
    assert_eq!(observed["work_mem"], "4MB");
    assert_eq!(observed["shared_buffers"], "64MB");
    assert_eq!(observed["temp_file_limit"], "128MB");
    assert_eq!(observed["minimum_free_disk_bytes"], 512u64 * 1024 * 1024);
    assert!(observed["available_disk_bytes"].as_u64().unwrap() > 0);
    let admin = academy
        .cluster
        .connect("lab_01_01", "academy_admin")
        .await
        .unwrap();
    assert_eq!(
        observed["database_bytes"].as_i64().unwrap(),
        admin
            .query_one("SELECT pg_database_size(current_database())", &[])
            .await
            .unwrap()
            .get::<_, i64>(0)
    );
    assert_eq!(
        observed.as_object().unwrap().len(),
        6,
        "Snapshot must expose only authored fields"
    );
    let sql = "SELECT COUNT(*) FROM orders";
    let req = |mode: &str| RunRequest {
        exercise_id: "01-01".into(),
        sql: sql.into(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    };
    let measured = academy.run(req("analyze")).await.unwrap();
    let revision = academy.chapters[0].exercises[0].revision.clone();
    academy
        .store
        .record_analysis("01-01", &revision, sql, &measured.plan.unwrap())
        .unwrap();
    academy.store.pass("01-01", &revision).unwrap();
    admin
        .batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
        .await
        .unwrap();
    let before: String = admin
        .query_one(
            "SELECT md5(string_agg(row_to_json(o)::text,'|' ORDER BY id)) FROM orders o",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    academy.cluster.minimum_free_disk_bytes = u64::MAX;
    assert!(
        academy.resources("01-01").await.is_ok(),
        "An existing lab snapshot is readable below the allocation floor"
    );
    for result in [
        academy.run(req("run")).await.map(|_| ()),
        academy.check(req("run")).await.map(|_| ()),
        academy.reset("01-01").await,
    ] {
        let error = result.unwrap_err();
        assert!(error.contains("Free up disk space"), "{error}");
    }
    assert!(academy
        .cluster
        .create_lab("low_disk_lab", "low_disk_student", Fixture::Orders, 0)
        .await
        .is_err());
    let catalog = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let leftover:bool=catalog.query_one("SELECT EXISTS(SELECT FROM pg_database WHERE datname='low_disk_lab' OR datname LIKE 'grade_%' OR datname LIKE 'reference_%') OR EXISTS(SELECT FROM pg_roles WHERE rolname='low_disk_student')",&[]).await.unwrap().get(0);
    assert!(
        !leftover,
        "Rejected allocations must create no roles or assessment databases"
    );
    let after: String = admin
        .query_one(
            "SELECT md5(string_agg(row_to_json(o)::text,'|' ORDER BY id)) FROM orders o",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    assert_eq!(
        before, after,
        "Rejected Reset preserves the edited baseline"
    );
    assert_eq!(academy.store.script("01-01").unwrap(), Some(sql.into()));
    assert!(academy.store.passed("01-01", &revision).unwrap());
    assert!(academy.store.has_analysis("01-01", &revision, sql).unwrap());
    academy.cluster.minimum_free_disk_bytes = 512 * 1024 * 1024;
    assert!(academy.run(req("run")).await.unwrap().error.is_none());
    academy.reset("01-01").await.unwrap();
    assert!(!academy.store.has_analysis("01-01", &revision, sql).unwrap());
    academy.shutdown().await.unwrap();
}
