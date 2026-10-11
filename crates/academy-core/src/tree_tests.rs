use super::*;
use bigdecimal::BigDecimal;
use std::collections::{BTreeMap, HashSet};
fn oracle(customers: &QueryResult, orders: &QueryResult, items: &QueryResult) -> Vec<QueryResult> {
    let amount = |r: &Vec<Option<String>>| r[3].as_ref().map(|v| BigDecimal::from_str(v).unwrap());
    let key = |r: &Vec<Option<String>>| r[0].as_deref().unwrap().parse::<i32>().unwrap();
    let paid: Vec<_> = orders
        .rows
        .iter()
        .filter(|r| r[2].as_deref() == Some("paid"))
        .collect();
    let mut statuses: BTreeMap<String, (i64, i64, BigDecimal)> = BTreeMap::new();
    let mut groups: BTreeMap<i32, (i64, BigDecimal)> = BTreeMap::new();
    for r in &orders.rows {
        let m = statuses.entry(r[2].clone().unwrap()).or_default();
        m.0 += 1;
        if let Some(n) = amount(r) {
            m.1 += 1;
            m.2 += n;
        }
        if r[2].as_deref() == Some("paid") {
            let g = groups
                .entry(r[1].as_deref().unwrap().parse().unwrap())
                .or_default();
            g.0 += 1;
            if let Some(n) = amount(r) {
                g.1 += n;
            }
        }
    }
    let cells = |values: Vec<String>| values.into_iter().map(Some).collect::<Vec<_>>();
    let first = statuses
        .into_iter()
        .map(|(s, (count, known, total))| {
            cells(vec![
                s,
                count.to_string(),
                known.to_string(),
                total.to_string(),
            ])
        })
        .collect();
    let cold: Vec<_> = orders
        .rows
        .iter()
        .filter(|r| r[1].as_deref() == Some("202"))
        .collect();
    assert!(
        !cold.is_empty()
            && cold.len()
                < orders
                    .rows
                    .iter()
                    .filter(|r| r[1].as_deref() == Some("201"))
                    .count()
    );
    assert!(
        cold.iter().any(|r| r[3].is_none()),
        "Cold customer must exercise unknown-total preservation"
    );
    let second = cold
        .into_iter()
        .map(|r| vec![r[0].clone(), r[2].clone(), r[3].clone()])
        .collect();
    let eligible: HashSet<_> = items
        .rows
        .iter()
        .filter(|r| r[1].as_deref().unwrap().parse::<i32>().unwrap() >= 3)
        .map(key)
        .collect();
    let names: BTreeMap<_, _> = customers
        .rows
        .iter()
        .map(|r| (key(r), r[1].clone()))
        .collect();
    let third = paid
        .iter()
        .filter(|r| eligible.contains(&key(r)))
        .map(|r| {
            vec![
                r[0].clone(),
                names[&r[1].as_deref().unwrap().parse::<i32>().unwrap()].clone(),
                r[3].clone(),
            ]
        })
        .collect();
    let fourth = customers
        .rows
        .iter()
        .filter(|r| !groups.contains_key(&key(r)))
        .cloned()
        .collect();
    let mut qualified: Vec<_> = groups.into_iter().filter(|(_, g)| g.0 >= 3).collect();
    qualified.sort_by(|(a, x), (b, y)| y.0.cmp(&x.0).then(a.cmp(b)));
    let fifth = qualified
        .into_iter()
        .map(|(id, (count, total))| {
            cells(vec![id.to_string(), count.to_string(), total.to_string()])
        })
        .collect();
    let sixth = orders
        .rows
        .iter()
        .filter(|r| {
            r[2].as_deref() == Some("paid") && amount(r).is_some_and(|n| n >= BigDecimal::from(200))
                || r[2].as_deref() == Some("pending") && r[3].is_none()
        })
        .map(|r| vec![r[0].clone(), r[2].clone()])
        .collect();
    [
        (vec![25, 20, 20, 1700], first),
        (vec![23, 25, 1700], second),
        (vec![23, 25, 1700], third),
        (vec![23, 25], fourth),
        (vec![23, 20, 1700], fifth),
        (vec![23, 25], sixth),
    ]
    .into_iter()
    .map(|(column_types, rows)| QueryResult {
        column_types,
        rows,
        ..Default::default()
    })
    .collect()
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
async fn six_tree_queries_match_independent_results_and_measured_grain() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    let chapter = academy
        .chapters
        .get(6)
        .expect("Chapter seven must be authored");
    assert!(!chapter.published && chapter.exercises.len() == 6);
    assert!(chapter.exercises.iter().all(|e| e.require_analyze));
    let exercises = chapter.exercises.clone();
    assert!(
        academy
            .run(request("07-01", "SELECT 1", "run"))
            .await
            .is_err(),
        "Unpublished chapter remains locked"
    );
    for chapter in &mut academy.chapters {
        chapter.published = true;
        for e in &chapter.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    for seed in 0..3 {
        let database = format!("oracle_{seed}");
        let role = format!("oracle_student_{seed}");
        academy
            .cluster
            .create_lab(&database, &role, Fixture::Commerce, seed)
            .await
            .unwrap();
        let learner = academy.cluster.connect(&database, &role).await.unwrap();
        let customers = execute(
            &learner,
            "SELECT id,name FROM customers ORDER BY id",
            "run",
            1000,
        )
        .await
        .unwrap();
        let orders = execute(
            &learner,
            "SELECT id,customer_id,status,total,ordered_at FROM orders ORDER BY id",
            "run",
            1000,
        )
        .await
        .unwrap();
        let items = execute(
            &learner,
            "SELECT order_id,quantity FROM order_items",
            "run",
            10000,
        )
        .await
        .unwrap();
        assert!(!customers.truncated && !orders.truncated && !items.truncated);
        for (e, expected) in exercises.iter().zip(oracle(&customers, &orders, &items)) {
            let actual = execute(&learner, &e.solution_sql, "run", 1000)
                .await
                .unwrap();
            assert!(
                actual.error.is_none() && !actual.truncated,
                "{} seed {seed}",
                e.id
            );
            assert_eq!(
                normalized(&actual),
                normalized(&expected),
                "{} seed {seed}",
                e.id
            );
            let estimates = execute(&learner, &e.solution_sql, "explain", 1000)
                .await
                .unwrap();
            assert!(estimates.error.is_none());
            assert!(estimates.plan.as_ref().unwrap()[0]["Plan"]["Plan Rows"].is_number());
            assert!(estimates.plan.as_ref().unwrap()[0]["Plan"]["Actual Rows"].is_null());
            let measured = execute(&learner, &e.solution_sql, "analyze", 1000)
                .await
                .unwrap();
            assert!(measured.error.is_none());
            assert_eq!(
                measured.plan.as_ref().unwrap()[0]["Plan"]["Actual Rows"].as_f64(),
                Some(expected.rows.len() as f64)
            );
        }
        academy.cluster.drop_lab(&database, &role).await.unwrap();
    }
    for e in &exercises {
        academy
            .run(request(&e.id, &e.solution_sql, "analyze"))
            .await
            .unwrap();
        assert!(
            academy
                .check(request(&e.id, &e.solution_sql, "run"))
                .await
                .unwrap()
                .passed
        );
    }
    for (id,sql) in [
        ("07-02","SELECT id,status,total FROM orders WHERE customer_id=202 AND total IS NOT NULL ORDER BY id"),
        ("07-01","SELECT status,COUNT(total),COUNT(total),COALESCE(SUM(total),0) FROM orders GROUP BY status ORDER BY status"),
        ("07-03","SELECT o.id,c.name,o.total FROM orders o JOIN customers c ON c.id=o.customer_id JOIN order_items i ON i.order_id=o.id WHERE o.status='paid' AND i.quantity>=3 ORDER BY o.id"),
        ("07-04","SELECT c.id,c.name FROM customers c WHERE NOT EXISTS(SELECT 1 FROM orders o WHERE o.customer_id=c.id) ORDER BY c.id"),
        ("07-05","SELECT customer_id,COUNT(*),COALESCE(SUM(total),0) FROM orders WHERE status='paid' AND total IS NOT NULL GROUP BY customer_id HAVING COUNT(*)>=3 ORDER BY COUNT(*) DESC,customer_id"),
        ("07-06","SELECT id,status FROM orders WHERE status='paid' AND total>=200 OR status='pending' ORDER BY id"),
    ]{
        let measured=academy.run(request(id,sql,"analyze")).await.unwrap();assert!(measured.error.is_none());
        assert!(!academy.check(request(id,sql,"run")).await.unwrap().passed,"Wrong report {id} must fail even after Analyze");
    }
    academy.shutdown().await.unwrap();
}
