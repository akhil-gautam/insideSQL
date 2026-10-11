use super::*;
use bigdecimal::{BigDecimal, RoundingMode};
use std::collections::{BTreeMap, HashSet};

fn runtime() -> std::path::PathBuf {
    std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into())
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
async fn sort_aggregate_chapter_is_authored_but_not_available() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let chapter = academy
        .chapters
        .get(9)
        .expect("Chapter ten must be authored");
    assert!(!chapter.published && matches!(chapter.fixture, Fixture::ScanOrders));
    assert_eq!(chapter.exercises.len(), 6);
    assert!(chapter
        .exercises
        .iter()
        .all(|e| e.require_analyze && e.ordered));
    for prior in &mut academy.chapters[..9] {
        prior.published = true;
        for e in &prior.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    for mode in ["run", "explain", "analyze"] {
        assert!(academy
            .run(request("10-01", "SELECT 1", mode))
            .await
            .is_err());
    }
    assert!(academy
        .check(request("10-01", "SELECT 1", "run"))
        .await
        .is_err());
    assert!(academy.save("10-01", "SELECT 1").is_err());
    assert!(academy.reset("10-01").await.is_err());
    assert!(academy.solution("10-01").is_err());
    academy.shutdown().await.unwrap();
}

struct Order {
    id: i32,
    customer: i32,
    status: String,
    amount: Option<BigDecimal>,
    day: String,
}
#[derive(Default)]
struct Measure {
    events: i64,
    known: i64,
    sum: BigDecimal,
    distinct: HashSet<String>,
}
impl Measure {
    fn add(&mut self, o: &Order) {
        self.events += 1;
        if let Some(amount) = &o.amount {
            self.known += 1;
            self.sum += amount;
            self.distinct.insert(amount.normalized().to_string());
        }
    }
    fn cells(&self) -> Vec<Option<String>> {
        vec![
            Some(self.events.to_string()),
            Some(self.known.to_string()),
            Some(self.sum.to_string()),
        ]
    }
    fn average(&self) -> Option<String> {
        (self.known > 0).then(|| {
            (&self.sum / BigDecimal::from(self.known))
                .with_scale_round(2, RoundingMode::HalfUp)
                .to_string()
        })
    }
}
#[derive(Default)]
struct Conditional {
    events: i64,
    paid: Measure,
    pending: i64,
}

async fn oracle(client: &Client) -> Vec<QueryResult> {
    let orders: Vec<Order> = client
        .query(
            "SELECT id,customer_id,status,total::text,ordered_at::text FROM orders ORDER BY id",
            &[],
        )
        .await
        .unwrap()
        .into_iter()
        .map(|r| Order {
            id: r.get(0),
            customer: r.get(1),
            status: r.get(2),
            amount: r
                .get::<_, Option<String>>(3)
                .map(|s| BigDecimal::from_str(&s).unwrap()),
            day: r.get::<_, String>(4)[..10].to_owned(),
        })
        .collect();
    assert_eq!(orders.len(), 100000);
    let mut top: Vec<_> = orders.iter().filter(|o| o.status == "paid").collect();
    top.sort_by(|a, b| {
        match (&a.amount, &b.amount) {
            (Some(x), Some(y)) => y.cmp(x),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
        .then(a.id.cmp(&b.id))
    });
    assert!(top.iter().any(|o| o.amount.is_none()));
    assert!(
        top[..25].windows(2).any(|w| w[0].amount == w[1].amount),
        "Top report must exercise tie breaking"
    );
    let first = top
        .into_iter()
        .take(25)
        .map(|o| {
            vec![
                Some(o.id.to_string()),
                o.amount.as_ref().map(ToString::to_string),
            ]
        })
        .collect();
    let mut days: BTreeMap<String, Measure> = BTreeMap::new();
    let mut customers: BTreeMap<i32, Conditional> = BTreeMap::new();
    let mut levels: BTreeMap<(i32, Option<i32>, Option<String>), Measure> = BTreeMap::new();
    let mut statuses: BTreeMap<String, Measure> = BTreeMap::new();
    for o in &orders {
        if o.customer == 777 {
            days.entry(o.day.clone()).or_default().add(o);
        }
        let customer = customers.entry(o.customer).or_default();
        customer.events += 1;
        if o.status == "paid" {
            customer.paid.add(o);
        }
        if o.status == "pending" {
            customer.pending += 1;
        }
        if [1, 777].contains(&o.customer) {
            for key in [
                (0, Some(o.customer), Some(o.status.clone())),
                (1, Some(o.customer), None),
                (3, None, None),
            ] {
                levels.entry(key).or_default().add(o);
            }
        }
        statuses.entry(o.status.clone()).or_default().add(o);
    }
    assert!(days
        .values()
        .any(|m| m.events > 0 && m.known == 0 && m.average().is_none()));
    assert!(days.values().any(|m| m.known > 0 && m.known < m.events));
    assert!(statuses
        .values()
        .all(|m| (m.distinct.len() as i64) < m.known));
    let second = days
        .into_iter()
        .map(|(day, m)| {
            let mut row = vec![Some(day)];
            row.extend(m.cells());
            row.push(m.average());
            row
        })
        .collect();
    let third = customers
        .iter()
        .map(|(id, m)| {
            let mut row = vec![Some(id.to_string()), Some(m.events.to_string())];
            row.extend(m.paid.cells());
            row.push(Some(m.pending.to_string()));
            row
        })
        .collect();
    let mut qualified: Vec<_> = customers
        .iter()
        .filter(|(_, m)| m.paid.events >= 6 && m.paid.sum >= BigDecimal::from(1500))
        .collect();
    assert!(!qualified.is_empty());
    qualified.sort_by(|(a, x), (b, y)| y.paid.sum.cmp(&x.paid.sum).then(a.cmp(b)));
    let fourth = qualified
        .into_iter()
        .take(20)
        .map(|(id, m)| {
            let mut row = vec![Some(id.to_string())];
            row.extend(m.paid.cells());
            row
        })
        .collect();
    assert!(levels.keys().any(|(mask, _, _)| *mask == 1) && levels.contains_key(&(3, None, None)));
    let fifth = levels
        .into_iter()
        .map(|((mask, id, status), m)| {
            let mut row = vec![Some(mask.to_string()), id.map(|x| x.to_string()), status];
            row.extend(m.cells());
            row
        })
        .collect();
    let sixth = statuses
        .into_iter()
        .map(|(status, m)| {
            vec![
                Some(status),
                Some(m.events.to_string()),
                Some(m.known.to_string()),
                Some(m.distinct.len().to_string()),
                Some(m.sum.to_string()),
            ]
        })
        .collect();
    [
        (vec![23, 1700], first),
        (vec![1082, 20, 20, 1700, 1700], second),
        (vec![23, 20, 20, 20, 1700, 20], third),
        (vec![23, 20, 20, 1700], fourth),
        (vec![23, 23, 25, 20, 20, 1700], fifth),
        (vec![25, 20, 20, 20, 1700], sixth),
    ]
    .into_iter()
    .map(|(column_types, rows)| QueryResult {
        column_types,
        rows,
        ..Default::default()
    })
    .collect()
}
fn nodes<'a>(plan: &'a Value, out: &mut Vec<&'a Value>) {
    assert!(plan["Actual Rows"].is_number() && plan["Actual Loops"].is_number());
    out.push(plan);
    if let Some(children) = plan["Plans"].as_array() {
        for child in children {
            nodes(child, out);
        }
    }
}

#[tokio::test]
async fn six_sort_aggregate_reports_match_independent_results_and_real_spill() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let exercises = academy
        .chapters
        .get(9)
        .expect("Chapter ten must be authored")
        .exercises
        .clone();
    for chapter in &mut academy.chapters {
        chapter.published = true;
        for e in &chapter.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    for seed in 0..3 {
        let database = format!("sort_oracle_{seed}");
        let role = format!("sort_oracle_student_{seed}");
        academy
            .cluster
            .create_lab(&database, &role, Fixture::ScanOrders, seed)
            .await
            .unwrap();
        let client = academy.cluster.connect(&database, &role).await.unwrap();
        let expected = oracle(&client).await;
        client
            .batch_execute("SET TIME ZONE 'Asia/Kolkata'")
            .await
            .unwrap();
        for (e, expected) in exercises.iter().zip(&expected) {
            assert!(!expected.rows.is_empty() && expected.rows.len() <= 1000);
            let actual = execute(&client, &e.solution_sql, "run", 1000)
                .await
                .unwrap();
            assert!(
                actual.error.is_none() && !actual.truncated,
                "{}: {:?}",
                e.id,
                actual.error
            );
            assert_eq!(
                actual.column_types, expected.column_types,
                "{} seed {seed}",
                e.id
            );
            assert_eq!(
                normalized(&actual),
                normalized(expected),
                "{} seed {seed}",
                e.id
            );
            let estimated = execute(&client, &e.solution_sql, "explain", 1000)
                .await
                .unwrap();
            assert!(
                estimated.error.is_none()
                    && estimated.plan.as_ref().unwrap()[0]["Plan"]["Actual Rows"].is_null()
            );
            let measured = execute(&client, &e.solution_sql, "analyze", 1000)
                .await
                .unwrap();
            assert!(measured.error.is_none(), "{}: {:?}", e.id, measured.error);
            let root = &measured.plan.as_ref().unwrap()[0]["Plan"];
            assert_eq!(
                root["Actual Rows"].as_f64(),
                Some(expected.rows.len() as f64)
            );
            let mut operations = Vec::new();
            nodes(root, &mut operations);
            eprintln!(
                "{} seed {seed}: {}",
                e.id,
                operations
                    .iter()
                    .map(|n| n["Node Type"].as_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(" > ")
            );
        }
        // Session controls are owned experiments, never learner algorithm or speed gates.
        client.batch_execute("SET enable_indexscan=off; SET enable_indexonlyscan=off; SET enable_bitmapscan=off; SET max_parallel_workers_per_gather=0").await.unwrap();
        let sql = &exercises[2].solution_sql;
        for (settings, strategy, space) in [
            (
                "SET work_mem='16MB'; SET enable_hashagg=on; SET enable_sort=off",
                "Hashed",
                None,
            ),
            (
                "SET work_mem='64kB'; SET enable_hashagg=off; SET enable_sort=on",
                "Sorted",
                Some("Disk"),
            ),
            (
                "SET work_mem='16MB'; SET enable_hashagg=off; SET enable_sort=on",
                "Sorted",
                Some("Memory"),
            ),
        ] {
            client.batch_execute(settings).await.unwrap();
            let measured = execute(&client, sql, "analyze", 1000).await.unwrap();
            assert!(measured.error.is_none(), "{settings}: {:?}", measured.error);
            let root = &measured.plan.as_ref().unwrap()[0]["Plan"];
            let mut operations = Vec::new();
            nodes(root, &mut operations);
            assert!(operations
                .iter()
                .any(|n| n["Node Type"] == "Aggregate" && n["Strategy"] == strategy));
            if let Some(space) = space {
                let sort = operations
                    .iter()
                    .find(|n| n["Node Type"] == "Sort" && n["Sort Space Type"] == space)
                    .expect("Measured sort space must match the controlled experiment");
                assert!(sort["Sort Space Used"].as_i64().unwrap() > 0);
                if space == "Disk" {
                    assert!(
                        root["Temp Read Blocks"].as_i64().unwrap() > 0
                            && root["Temp Written Blocks"].as_i64().unwrap() > 0
                    );
                }
                eprintln!(
                    "seed {seed} {strategy}: {} / {space}, {} kB, temp reads {}",
                    sort["Sort Method"], sort["Sort Space Used"], root["Temp Read Blocks"]
                );
            }
            let actual = execute(&client, sql, "run", 1000).await.unwrap();
            assert!(actual.error.is_none() && !actual.truncated);
            assert_eq!(actual.column_types, expected[2].column_types);
            assert_eq!(normalized(&actual), normalized(&expected[2]));
        }
        drop(client);
        academy.cluster.drop_lab(&database, &role).await.unwrap();
    }
    for e in &exercises {
        assert!(academy
            .run(request(&e.id, &e.solution_sql, "analyze"))
            .await
            .unwrap()
            .error
            .is_none());
        assert!(
            academy
                .check(request(&e.id, &e.solution_sql, "run"))
                .await
                .unwrap()
                .passed
        );
    }
    for (id,sql) in [
        ("10-01","SELECT id,total FROM orders WHERE status='paid' ORDER BY total DESC,id LIMIT 25"),
        ("10-01","SELECT id,total FROM orders WHERE status='paid' ORDER BY total DESC NULLS LAST,id DESC LIMIT 25"),
        ("10-02","SELECT (ordered_at AT TIME ZONE 'UTC')::date,COUNT(*),COUNT(total),COALESCE(SUM(total),0),ROUND(AVG(COALESCE(total,0)),2) FROM orders WHERE customer_id=777 GROUP BY 1 ORDER BY 1"),
        ("10-02","SELECT (ordered_at AT TIME ZONE 'UTC')::date,COUNT(*),COUNT(total),COALESCE(SUM(total),0),ROUND(SUM(total)/COUNT(*),2) FROM orders WHERE customer_id=777 GROUP BY 1 ORDER BY 1"),
        ("10-03","SELECT customer_id,COUNT(*),COUNT(*) FILTER(WHERE status='paid'),COUNT(total) FILTER(WHERE status='paid'),COALESCE(SUM(total) FILTER(WHERE status='paid'),0),COUNT(*) FILTER(WHERE status='pending') FROM orders WHERE status='paid' GROUP BY customer_id ORDER BY customer_id"),
        ("10-04","SELECT customer_id,COUNT(*) FILTER(WHERE status='paid'),COUNT(total) FILTER(WHERE status='paid'),COALESCE(SUM(total),0) FROM orders GROUP BY customer_id HAVING COUNT(*) FILTER(WHERE status='paid')>=6 AND COALESCE(SUM(total),0)>=1500 ORDER BY 4 DESC,customer_id LIMIT 20"),
        ("10-04","WITH sample AS (SELECT customer_id,total FROM orders WHERE status='paid' ORDER BY id LIMIT 20) SELECT customer_id,COUNT(*),COUNT(total),COALESCE(SUM(total),0) FROM sample GROUP BY customer_id HAVING COUNT(*)>=6 AND COALESCE(SUM(total),0)>=1500 ORDER BY 4 DESC,customer_id LIMIT 20"),
        ("10-05","SELECT GROUPING(customer_id,status),customer_id,status,COUNT(*),COUNT(total),COALESCE(SUM(total),0) FROM orders WHERE customer_id IN(1,777) GROUP BY GROUPING SETS((customer_id,status),()) ORDER BY 1,2 NULLS LAST,3 NULLS LAST"),
        ("10-05","SELECT GROUPING(customer_id,status),COALESCE(customer_id,0),COALESCE(status,'ALL'),COUNT(*),COUNT(total),COALESCE(SUM(total),0) FROM orders WHERE customer_id IN(1,777) GROUP BY GROUPING SETS((customer_id,status),(customer_id),()) ORDER BY 1,2 NULLS LAST,3 NULLS LAST"),
        ("10-06","SELECT status,COUNT(*),COUNT(total),COUNT(DISTINCT total),COALESCE(SUM(DISTINCT total),0) FROM orders GROUP BY status ORDER BY status"),
    ] {
        let measured=academy.run(request(id,sql,"analyze")).await.unwrap();assert!(measured.error.is_none(),"{id}: {:?}",measured.error);
        assert!(!academy.check(request(id,sql,"run")).await.unwrap().passed,"Wrong report {id} must fail");
    }
    let e = &exercises[1];
    let before = academy
        .run(request(&e.id, &e.solution_sql, "run"))
        .await
        .unwrap()
        .rows;
    let admin = academy
        .cluster
        .connect("lab_10_02", "academy_admin")
        .await
        .unwrap();
    admin
        .batch_execute("UPDATE orders SET total=100 WHERE id=97")
        .await
        .unwrap();
    drop(admin);
    assert_ne!(
        academy
            .run(request(&e.id, &e.solution_sql, "run"))
            .await
            .unwrap()
            .rows,
        before
    );
    academy.reset(&e.id).await.unwrap();
    assert_eq!(
        academy
            .run(request(&e.id, &e.solution_sql, "run"))
            .await
            .unwrap()
            .rows,
        before
    );
    assert!(!academy
        .store
        .has_analysis(&e.id, &e.revision, &e.solution_sql)
        .unwrap());
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let copies:i64=admin.query_one("SELECT COUNT(*) FROM pg_database WHERE datname LIKE 'grade_%' OR datname LIKE 'reference_%'",&[]).await.unwrap().get(0);
    assert_eq!(copies, 0);
    drop(admin);
    academy.shutdown().await.unwrap();
}
