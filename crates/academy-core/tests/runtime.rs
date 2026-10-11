use academy_core::{Academy, RunRequest};
use std::path::PathBuf;

fn runtime_path() -> PathBuf {
    std::env::var_os("ACADEMY_POSTGRES")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/Applications/Postgres.app/Contents/Versions/18"))
}

#[tokio::test]
async fn real_postgres_execution_permissions_grading_reset_and_resume() {
    let directory = tempfile::tempdir().unwrap();
    let academy = Academy::start(directory.path(), &runtime_path())
        .await
        .unwrap();
    let submit = |sql: &str, mode: &str| RunRequest {
        exercise_id: "01-01".into(),
        sql: sql.into(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    };
    let error = academy.run(submit("SELEC", "run")).await.unwrap();
    assert!(error.error.is_some());
    assert!(academy.solution("01-01").unwrap().is_some());
    let result = academy
        .run(submit(
            "SELECT id, total FROM orders WHERE status = 'paid' ORDER BY id",
            "run",
        ))
        .await
        .unwrap();
    assert_eq!(result.rows.len(), 4);
    assert_eq!(
        result.rows[0],
        vec![Some("1".into()), Some("125.50".into())]
    );
    let result = academy
        .run(submit("DROP TABLE orders", "run"))
        .await
        .unwrap();
    assert!(
        result.error.is_some(),
        "learner must not own fixture tables"
    );
    let result = academy
        .run(submit("SELECT pg_read_file('/etc/passwd')", "run"))
        .await
        .unwrap();
    assert!(result.error.is_some(), "learner must not read server files");
    let wrong = academy
        .check(submit(
            "SELECT id, total FROM orders WHERE status = 'paid' LIMIT 4",
            "run",
        ))
        .await
        .unwrap();
    assert!(
        !wrong.passed,
        "a hardcoded small limit must fail hidden variants"
    );
    let correct = academy
        .check(submit(
            "SELECT id, total FROM orders WHERE status = 'paid' ORDER BY id",
            "run",
        ))
        .await
        .unwrap();
    assert!(correct.passed, "{:?}", correct);
    let equivalent = academy
        .check(submit(
            "SELECT id, total::numeric(20,4) FROM orders WHERE status IN ('paid') ORDER BY id",
            "run",
        ))
        .await
        .unwrap();
    assert!(
        equivalent.passed,
        "numeric scale must not change correctness"
    );
    let plan = academy
        .run(submit(
            "SELECT * FROM orders WHERE status = 'paid'",
            "analyze",
        ))
        .await
        .unwrap();
    assert!(plan.plan.as_ref().unwrap()[0]["Plan"]["Actual Rows"].is_number());
    let huge = academy
        .run(submit("SELECT generate_series(1, 100000)", "run"))
        .await
        .unwrap();
    assert!(huge.truncated);
    assert_eq!(huge.rows.len(), 1000);
    academy.reset("01-01").await.unwrap();
    assert!(academy.progress().await.unwrap()["01-01"]["passed"]
        .as_bool()
        .unwrap());
    academy.shutdown().await.unwrap();
    drop(academy);
    let reopened = Academy::start(directory.path(), &runtime_path())
        .await
        .unwrap();
    assert!(reopened.solution("01-01").unwrap().is_some());
    assert!(reopened.progress().await.unwrap()["01-01"]["passed"]
        .as_bool()
        .unwrap());
    reopened.shutdown().await.unwrap();
}

#[tokio::test]
async fn cancellation_stops_the_named_run_and_allows_next_query() {
    let directory = tempfile::tempdir().unwrap();
    let academy = std::sync::Arc::new(
        Academy::start(directory.path(), &runtime_path())
            .await
            .unwrap(),
    );
    let request = RunRequest {
        exercise_id: "01-01".into(),
        sql: "SELECT pg_sleep(20)".into(),
        mode: "run".into(),
        run_id: "cancel-test".into(),
    };
    let other = academy.clone();
    let handle = tokio::spawn(async move { other.run(request).await });
    for _ in 0..100 {
        if academy.running("cancel-test") {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    assert!(academy.cancel("cancel-test").await.unwrap());
    let result = handle.await.unwrap().unwrap();
    assert_eq!(result.error.unwrap().code.as_deref(), Some("57014"));
    let next = academy
        .run(RunRequest {
            exercise_id: "01-01".into(),
            sql: "SELECT 1".into(),
            mode: "run".into(),
            run_id: "after-cancel".into(),
        })
        .await
        .unwrap();
    assert_eq!(next.rows, vec![vec![Some("1".into())]]);
    academy.shutdown().await.unwrap();
}
