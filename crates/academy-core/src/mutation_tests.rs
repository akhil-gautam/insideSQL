use super::*;
use bigdecimal::BigDecimal;
use std::collections::HashSet;

fn transform(
    index: usize,
    before: &QueryResult,
    child_orders: &HashSet<i32>,
) -> (QueryResult, QueryResult) {
    let mut rows = before.rows.clone();
    let mut returned = Vec::new();
    let cells = |values: Vec<String>| values.into_iter().map(Some).collect::<Vec<_>>();
    let id = |row: &Vec<Option<String>>| row[0].as_deref().unwrap().parse::<i32>().unwrap();
    match index {
        0 => {
            let row = cells(vec![
                "900001".into(),
                "109".into(),
                "pending".into(),
                "75".into(),
                "2026-01-12 00:00:00+00".into(),
            ]);
            returned.push(row.clone());
            rows.push(row);
        }
        1 => {
            for row in &mut rows {
                if row[2].as_deref() == Some("pending")
                    && row[3]
                        .as_deref()
                        .is_some_and(|n| BigDecimal::from_str(n).unwrap() >= BigDecimal::from(100))
                {
                    row[2] = Some("paid".into());
                    returned.push(vec![row[0].clone(), row[2].clone(), row[3].clone()]);
                }
            }
        }
        2 => {
            for row in &mut rows {
                if row[2].as_deref() == Some("pending") && row[3].is_none() {
                    row[3] = Some("0".into());
                    returned.push(vec![row[0].clone(), row[3].clone()]);
                }
            }
        }
        3 => rows.retain(|row| {
            let remove = row[2].as_deref() == Some("cancelled") && !child_orders.contains(&id(row));
            if remove {
                returned.push(vec![row[0].clone()]);
            }
            !remove
        }),
        4 => {
            if let Some(row) = rows.iter_mut().find(|row| id(row) == 900000) {
                let amount = BigDecimal::from_str(row[3].as_deref().unwrap()).unwrap()
                    + BigDecimal::from(10);
                row[3] = Some(amount.to_string());
                returned.push(vec![row[0].clone(), row[3].clone()]);
            } else {
                rows.push(cells(vec![
                    "900000".into(),
                    "109".into(),
                    "pending".into(),
                    "10".into(),
                    "2026-01-11 00:00:00+00".into(),
                ]));
                returned.push(cells(vec!["900000".into(), "10".into()]));
            }
        }
        5 => {
            let mut count = 0;
            let mut total = BigDecimal::from(0);
            for row in &mut rows {
                if row[2].as_deref() == Some("pending")
                    && &row[4].as_deref().unwrap()[..10] < "2026-02-05"
                {
                    count += 1;
                    if let Some(amount) = &row[3] {
                        total += BigDecimal::from_str(amount).unwrap();
                    }
                    row[2] = Some("cancelled".into());
                }
            }
            returned.push(cells(vec![count.to_string(), total.to_string()]));
        }
        _ => unreachable!(),
    }
    let column_types = [
        vec![23, 23, 25, 1700, 1184],
        vec![23, 25, 1700],
        vec![23, 1700],
        vec![23],
        vec![23, 1700],
        vec![20, 1700],
    ][index]
        .clone();
    (
        QueryResult {
            rows: returned,
            column_types,
            ..Default::default()
        },
        QueryResult {
            rows,
            column_types: before.column_types.clone(),
            ..Default::default()
        },
    )
}

#[tokio::test]
async fn six_mutations_match_independent_before_and_after_calculations() {
    let (_home, academy) = academy().await;
    for seed in 0..3 {
        for (index, e) in academy.chapters[4].exercises.iter().enumerate() {
            let db = format!("write_oracle_{seed}_{index}");
            let role = format!("write_student_{seed}_{index}");
            academy
                .cluster
                .create_lab(&db, &role, Fixture::CommerceWrites, seed)
                .await
                .unwrap();
            let learner = academy.cluster.connect(&db, &role).await.unwrap();
            let read = "SELECT id,customer_id,status,total,ordered_at FROM orders ORDER BY id";
            let before = execute(&learner, read, "run", 10000).await.unwrap();
            let children = execute(
                &learner,
                "SELECT order_id FROM order_items UNION ALL SELECT order_id FROM payments",
                "run",
                10000,
            )
            .await
            .unwrap();
            assert!(
                before.error.is_none()
                    && !before.truncated
                    && children.error.is_none()
                    && !children.truncated
            );
            let child_orders = children
                .rows
                .iter()
                .filter_map(|r| r[0].as_deref().map(|s| s.parse().unwrap()))
                .collect();
            let (returned, state) = transform(index, &before, &child_orders);
            let actual = execute(&learner, &e.solution_sql, "run", 10000)
                .await
                .unwrap();
            assert!(
                actual.error.is_none() && !actual.truncated,
                "{} seed {seed}",
                e.id
            );
            assert!(
                equivalent(&normalized(&actual), &normalized(&returned), false),
                "Independent returned oracle: {} seed {seed}",
                e.id
            );
            let after = execute(&learner, read, "run", 10000).await.unwrap();
            assert!(
                equivalent(&normalized(&after), &normalized(&state), false),
                "Independent state oracle: {} seed {seed}",
                e.id
            );
            academy.cluster.drop_lab(&db, &role).await.unwrap();
        }
    }
    academy.shutdown().await.unwrap();
}

async fn academy() -> (tempfile::TempDir, Academy) {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    assert!(
        academy.chapters.get(4).is_some(),
        "Chapter five must be authored"
    );
    for chapter in &mut academy.chapters[..4] {
        chapter.published = true;
        for e in &chapter.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    assert!(
        academy.solution("05-01").is_err(),
        "Draft chapter must stay locked"
    );
    academy.chapters[4].published = true;
    (home, academy)
}

fn request(id: &str, sql: &str, mode: &str) -> RunRequest {
    RunRequest {
        exercise_id: id.into(),
        sql: sql.into(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    }
}

#[tokio::test]
async fn mutation_grading_rejects_returning_spoofs_and_uses_fresh_reference_state() {
    let (_home, academy) = academy().await;
    let spoof =
        "SELECT 900001,109,'pending'::text,75::numeric,'2026-01-12 00:00:00+00'::timestamptz";
    assert!(
        !academy
            .check(request("05-01", spoof, "run"))
            .await
            .unwrap()
            .passed,
        "SELECT with identical output must not pass without inserting the order"
    );
    for e in &academy.chapters[4].exercises {
        let result = academy
            .check(request(&e.id, &e.solution_sql, "run"))
            .await
            .unwrap();
        assert!(result.passed, "{}: {}", e.id, result.message);
    }
    for (id,sql) in [
        ("05-01","WITH extra AS (UPDATE orders SET total=0 WHERE id=1) INSERT INTO orders VALUES (900001,109,'pending',75,'2026-01-12 00:00:00+00') RETURNING id,customer_id,status,total,ordered_at"),
        ("05-03","UPDATE orders SET total=0 WHERE total IS NULL RETURNING id,total"),
        ("05-05","INSERT INTO orders VALUES (900000,109,'pending',10,'2026-01-11 00:00:00+00') ON CONFLICT(id) DO UPDATE SET total=EXCLUDED.total RETURNING id,total"),
        ("05-06","WITH changed AS (UPDATE orders SET status='cancelled' WHERE status='pending' AND ordered_at<'2026-02-05 00:00:00+00' RETURNING total) SELECT COUNT(*),COALESCE(SUM(total),0) FROM orders WHERE status='cancelled'"),
    ] { assert!(!academy.check(request(id,sql,"run")).await.unwrap().passed,"Wrong output or extra mutation must fail: {id}"); }
    let count = academy
        .run(request(
            "05-01",
            "SELECT COUNT(*) FROM orders WHERE id=900001",
            "run",
        ))
        .await
        .unwrap();
    assert_eq!(
        count.rows[0][0].as_deref(),
        Some("0"),
        "Checking never changes visible lab"
    );
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let leftovers: i64=admin.query_one("SELECT COUNT(*) FROM pg_database WHERE datname LIKE 'grade_%' OR datname LIKE 'reference_%'",&[]).await.unwrap().get(0);
    assert_eq!(leftovers, 0, "Both assessment databases must be removed");
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn write_permissions_and_constraints_remain_narrow_and_atomic() {
    let (_home, academy) = academy().await;
    let read = "SELECT id,customer_id,status,total,ordered_at FROM orders ORDER BY id";
    let before = academy.run(request("05-01", read, "run")).await.unwrap();
    for (sql,code) in [
        ("INSERT INTO orders VALUES (900002,999999,'pending',1,'2026-01-01')","23503"),
        ("INSERT INTO orders VALUES (1,109,'pending',1,'2026-01-01')","23505"),
        ("INSERT INTO orders VALUES (900002,109,'invalid',1,'2026-01-01')","23514"),
        ("INSERT INTO orders VALUES (900002,NULL,'pending',1,'2026-01-01')","23502"),
        ("DELETE FROM orders WHERE id=1","23503"),
        ("WITH valid AS (UPDATE orders SET total=0 WHERE id=900000) INSERT INTO orders VALUES (900002,999999,'pending',1,'2026-01-01')","23503"),
        ("UPDATE customers SET name='Changed' WHERE id=109","42501"),
        ("DELETE FROM order_items WHERE id=1","42501"),
        ("UPDATE payments SET amount=0","42501"),
        ("CREATE TABLE arbitrary(id integer)","42501"),
        ("TRUNCATE orders CASCADE","42501"),
        ("ALTER TABLE orders DISABLE TRIGGER ALL","42501"),
        ("SET ROLE academy_admin","42501"),
    ] {
        let result=academy.run(request("05-01",sql,"run")).await.unwrap();
        assert_eq!(result.error.unwrap().code.as_deref(),Some(code),"{sql}");
        let after=academy.run(request("05-01",read,"run")).await.unwrap();
        assert_eq!(normalized(&after),normalized(&before),"Failed statement must be atomic: {sql}");
    }
    let client = academy
        .cluster
        .connect("lab_05_01", "student_05_01")
        .await
        .unwrap();
    let denied: bool = client
        .query_one(
            "SELECT NOT has_database_privilege(current_user,'lab_01_01','CONNECT')",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    assert!(
        denied,
        "A write role cannot attach another exercise database"
    );
    let old = academy
        .run(request("01-01", "UPDATE orders SET total=0", "run"))
        .await
        .unwrap();
    assert_eq!(old.error.unwrap().code.as_deref(), Some("42501"));
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn cancelled_writes_roll_back_and_assessment_copies_are_cleaned_up() {
    let (_home, academy) = academy().await;
    let academy = std::sync::Arc::new(academy);
    let read = "SELECT total FROM orders WHERE id=900000";
    let before = academy
        .run(request("05-05", read, "run"))
        .await
        .unwrap()
        .rows;
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    for mode in ["run", "analyze", "check"] {
        let runner = academy.clone();
        let run_id = uuid::Uuid::new_v4().to_string();
        let worker_id = run_id.clone();
        let task = tokio::spawn(async move {
            let mut input=request("05-05","WITH changed AS (UPDATE orders SET total=total+10 WHERE id=900000 RETURNING id) SELECT id,pg_sleep(20) FROM changed",mode);
            input.run_id = worker_id;
            if mode == "check" {
                runner.check(input).await.map(|r| json!(r))
            } else {
                runner.run(input).await.map(|r| json!(r))
            }
        });
        let pattern = if mode == "check" {
            "grade_%"
        } else {
            "lab_05_05"
        };
        let mut sleeping = false;
        for _ in 0..200 {
            sleeping=admin.query_one("SELECT EXISTS(SELECT FROM pg_stat_activity WHERE datname LIKE $1 AND wait_event='PgSleep')",&[&pattern]).await.unwrap().get(0);
            if sleeping {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        assert!(
            sleeping,
            "Wait for a real post-update sleep before cancelling {mode}"
        );
        assert!(academy.cancel(&run_id).await.unwrap());
        let result = task.await.unwrap().unwrap();
        if mode == "check" {
            assert_eq!(result["passed"], false);
        } else {
            assert_eq!(result["error"]["code"], "57014");
        }
        assert_eq!(
            academy
                .run(request("05-05", read, "run"))
                .await
                .unwrap()
                .rows,
            before
        );
        let leftovers:i64=admin.query_one("SELECT COUNT(*) FROM pg_database WHERE datname LIKE 'grade_%' OR datname LIKE 'reference_%'",&[]).await.unwrap().get(0);
        assert_eq!(leftovers, 0);
    }
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn mutating_analyze_rolls_back_while_run_persists_and_reset_restores() {
    let (_home, academy) = academy().await;
    let sql = "UPDATE orders SET total=total+10 WHERE id=900000 RETURNING id,total";
    let read = "SELECT total FROM orders WHERE id=900000";
    let before = academy
        .run(request("05-05", read, "run"))
        .await
        .unwrap()
        .rows;
    let measured = academy.run(request("05-05", sql, "analyze")).await.unwrap();
    assert!(measured.error.is_none() && measured.plan.is_some());
    assert_eq!(
        academy
            .run(request("05-05", read, "run"))
            .await
            .unwrap()
            .rows,
        before,
        "Analyze must roll back table changes"
    );
    assert!(academy
        .run(request("05-05", sql, "run"))
        .await
        .unwrap()
        .error
        .is_none());
    assert_eq!(
        academy
            .run(request("05-05", read, "run"))
            .await
            .unwrap()
            .rows[0][0]
            .as_deref(),
        Some("35.00")
    );
    assert_eq!(
        academy
            .run(request("05-01", read, "run"))
            .await
            .unwrap()
            .rows,
        before,
        "Other exercise lab must be independent"
    );
    academy.reset("05-05").await.unwrap();
    assert_eq!(
        academy
            .run(request("05-05", read, "run"))
            .await
            .unwrap()
            .rows,
        before
    );
    academy.shutdown().await.unwrap();
}
