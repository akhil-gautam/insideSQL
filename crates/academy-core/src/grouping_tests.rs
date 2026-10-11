use super::*;
use bigdecimal::BigDecimal;
use std::collections::BTreeMap;

#[derive(Default)]
struct Measure {
    count: i64,
    known: i64,
    total: BigDecimal,
}
impl Measure {
    fn add(&mut self, amount: &Option<String>) {
        self.count += 1;
        if let Some(amount) = amount {
            self.known += 1;
            self.total += BigDecimal::from_str(amount).unwrap();
        }
    }
}

fn oracle(customers: &QueryResult, orders: &QueryResult) -> Vec<QueryResult> {
    let mut paid: BTreeMap<i32, Measure> = BTreeMap::new();
    let mut by_status: BTreeMap<(String, String), Measure> = BTreeMap::new();
    let mut days: BTreeMap<String, ([i64; 4], BigDecimal)> = BTreeMap::new();
    for row in &orders.rows {
        let customer = row[0].as_deref().unwrap().parse().unwrap();
        let status = row[1].as_deref().unwrap();
        let day = row[3].as_deref().unwrap()[..10].to_owned();
        by_status
            .entry((day.clone(), status.into()))
            .or_default()
            .add(&row[2]);
        let daily = days.entry(day).or_default();
        let index = ["paid", "pending", "cancelled", "refunded"]
            .iter()
            .position(|s| *s == status)
            .unwrap();
        daily.0[index] += 1;
        if status == "paid" {
            paid.entry(customer).or_default().add(&row[2]);
            if let Some(amount) = &row[2] {
                daily.1 += BigDecimal::from_str(amount).unwrap();
            }
        }
    }
    assert!(
        paid.values().any(|m| m.known == 0),
        "Fixture must include an all-null paid amount group"
    );
    let cells = |values: Vec<String>| values.into_iter().map(Some).collect::<Vec<_>>();
    let summary = |id: &i32, m: &Measure| {
        cells(vec![
            id.to_string(),
            m.count.to_string(),
            m.total.to_string(),
        ])
    };
    let first = paid.iter().map(|(id, m)| summary(id, m)).collect();
    let second = customers
        .rows
        .iter()
        .map(|row| {
            let id = row[0].as_deref().unwrap().parse().unwrap();
            let zero = Measure::default();
            let m = paid.get(&id).unwrap_or(&zero);
            cells(vec![
                id.to_string(),
                m.count.to_string(),
                m.known.to_string(),
                m.total.to_string(),
            ])
        })
        .collect();
    let mut qualified: Vec<_> = paid
        .iter()
        .filter(|(_, m)| m.count >= 2 && m.total > BigDecimal::from(200))
        .collect();
    qualified.sort_by(|(a, x), (b, y)| y.total.cmp(&x.total).then(a.cmp(b)));
    let third = qualified
        .into_iter()
        .map(|(id, m)| summary(id, m))
        .collect();
    let fourth = by_status
        .into_iter()
        .map(|((day, status), m)| {
            cells(vec![
                day,
                status,
                m.count.to_string(),
                m.known.to_string(),
                m.total.to_string(),
            ])
        })
        .collect();
    let mut ranked: Vec<_> = paid.iter().collect();
    ranked.sort_by(|(a, x), (b, y)| y.count.cmp(&x.count).then(a.cmp(b)));
    assert!(
        ranked[..5]
            .windows(2)
            .any(|pair| pair[0].1.count == pair[1].1.count),
        "Top five must exercise tied activity counts"
    );
    let fifth = ranked
        .into_iter()
        .take(5)
        .map(|(id, m)| summary(id, m))
        .collect();
    let sixth = days
        .into_iter()
        .map(|(day, (counts, total))| {
            let mut values = vec![day];
            values.extend(counts.map(|c| c.to_string()));
            values.push(total.to_string());
            cells(values)
        })
        .collect();
    [
        (vec![23, 20, 1700], first),
        (vec![23, 20, 20, 1700], second),
        (vec![23, 20, 1700], third),
        (vec![1082, 25, 20, 20, 1700], fourth),
        (vec![23, 20, 1700], fifth),
        (vec![1082, 20, 20, 20, 20, 1700], sixth),
    ]
    .into_iter()
    .map(|(column_types, rows)| QueryResult {
        column_types,
        rows,
        ..Default::default()
    })
    .collect()
}

#[tokio::test]
async fn six_grouping_answers_match_raw_row_oracles_on_every_variant() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    assert!(
        academy.chapters.get(3).is_some(),
        "Chapter four must be authored"
    );
    for chapter in &mut academy.chapters {
        chapter.published = true;
        for e in &chapter.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    let exercises = academy.chapters[3].exercises.clone();
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
            "SELECT id FROM customers ORDER BY id",
            "run",
            1000,
        )
        .await
        .unwrap();
        let orders = execute(
            &learner,
            "SELECT customer_id,status,total,ordered_at FROM orders ORDER BY id",
            "run",
            1000,
        )
        .await
        .unwrap();
        let expected = oracle(&customers, &orders);
        learner
            .batch_execute("SET TIME ZONE 'Asia/Kolkata'")
            .await
            .unwrap();
        for (e, expected) in exercises.iter().zip(expected) {
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
                "Independent raw-row oracle: {} seed {seed}",
                e.id
            );
        }
        academy.cluster.drop_lab(&database, &role).await.unwrap();
    }
    let request = |id: &str, sql: &str, mode: &str| RunRequest {
        exercise_id: id.into(),
        sql: sql.into(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    };
    for e in &exercises {
        assert!(
            academy
                .check(request(&e.id, &e.solution_sql, "run"))
                .await
                .unwrap()
                .passed
        );
    }
    for (id,sql) in [
        ("04-02","SELECT c.id,COUNT(*),COUNT(o.total),COALESCE(SUM(o.total),0) FROM customers c LEFT JOIN orders o ON o.customer_id=c.id AND o.status='paid' GROUP BY c.id ORDER BY c.id"),
        ("04-03","SELECT customer_id,COUNT(*),COALESCE(SUM(total),0) FROM orders WHERE status='paid' AND total>200 GROUP BY customer_id HAVING COUNT(*)>=2 ORDER BY COALESCE(SUM(total),0) DESC,customer_id"),
        ("04-05","SELECT customer_id,COUNT(*),COALESCE(SUM(total),0) FROM (SELECT * FROM orders WHERE status='paid' ORDER BY id LIMIT 5) o GROUP BY customer_id ORDER BY COUNT(*) DESC,customer_id LIMIT 5"),
        ("04-05","SELECT customer_id,COUNT(*),COALESCE(SUM(total),0) FROM orders WHERE status='paid' GROUP BY customer_id ORDER BY COUNT(*) DESC,customer_id DESC LIMIT 5"),
        ("04-06","SELECT (ordered_at AT TIME ZONE 'UTC')::date,COUNT(CASE WHEN status='paid' THEN 1 ELSE 0 END),COUNT(*) FILTER(WHERE status='pending'),COUNT(*) FILTER(WHERE status='cancelled'),COUNT(*) FILTER(WHERE status='refunded'),COALESCE(SUM(total) FILTER(WHERE status='paid'),0) FROM orders GROUP BY 1 ORDER BY 1"),
    ] { assert!(!academy.check(request(id,sql,"run")).await.unwrap().passed,"Common wrong query {id} must fail"); }
    let measured = academy
        .run(request("04-05", &exercises[4].solution_sql, "analyze"))
        .await
        .unwrap();
    assert!(measured.error.is_none());
    fn visit(node: &Value, types: &mut Vec<String>) {
        types.push(node["Node Type"].as_str().unwrap().into());
        assert!(node["Actual Rows"].is_number() && node["Actual Loops"].is_number());
        if let Some(children) = node["Plans"].as_array() {
            for child in children {
                visit(child, types);
            }
        }
    }
    let mut types = Vec::new();
    visit(&measured.plan.unwrap()[0]["Plan"], &mut types);
    assert!(types.iter().any(|t| t == "Aggregate"));
    assert!(types.iter().any(|t| t == "Sort" || t == "Incremental Sort"));
    assert!(types.iter().any(|t| t == "Limit"));
    academy.shutdown().await.unwrap();
}

#[tokio::test]
async fn grouping_chapter_is_authored_but_unavailable_until_published() {
    let home = tempfile::tempdir().unwrap();
    let runtime = std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
    let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
    assert!(
        academy.chapters.get(3).is_some(),
        "Chapter four must be authored"
    );
    for chapter in &mut academy.chapters[..3] {
        chapter.published = true;
        for e in &chapter.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    let request = |mode: &str| RunRequest {
        exercise_id: "04-01".into(),
        sql: "SELECT 1".into(),
        mode: mode.into(),
        run_id: uuid::Uuid::new_v4().to_string(),
    };
    assert!(academy.solution("04-01").is_err());
    assert!(academy.save("04-01", "SELECT 1").is_err());
    assert!(academy.reset("04-01").await.is_err());
    assert!(academy.check(request("run")).await.is_err());
    for mode in ["run", "explain", "analyze"] {
        assert!(academy.run(request(mode)).await.is_err());
    }
    academy.shutdown().await.unwrap();
}
