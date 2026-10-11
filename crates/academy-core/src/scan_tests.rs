use super::*;
use bigdecimal::BigDecimal;
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
async fn scan_fixture_has_100k_rows_and_valid_indexes_without_write_privileges() {
    let fixture = serde_json::from_str::<Fixture>("\"scan_orders\"")
        .expect("The 100k scan fixture must be registered");
    assert!(fixture.writable_tables().is_empty() && fixture.sql(3).is_err());
    let home = tempfile::tempdir().unwrap();
    let cluster = Cluster::start(home.path(), &runtime()).await.unwrap();
    let mut fingerprints = HashSet::new();
    let mut original = None;
    for seed in [0, 1, 2, 0] {
        let database = format!("scan_{seed}");
        let role = format!("scan_student_{seed}");
        let started = Instant::now();
        cluster
            .create_lab(&database, &role, fixture, seed)
            .await
            .unwrap();
        let client = cluster.connect(&database, &role).await.unwrap();
        let stats=client.query_one("SELECT COUNT(*),COUNT(*) FILTER(WHERE customer_id=1),COUNT(*) FILTER(WHERE total IS NULL),MIN(length(payload)),MAX(length(payload)),COUNT(*) FILTER(WHERE customer_id=777 AND total IS NULL),COUNT(*) FILTER(WHERE customer_id=777 AND total IS NOT NULL),EXISTS(SELECT FROM orders WHERE total IS NOT NULL GROUP BY total HAVING COUNT(*)>1),EXISTS(SELECT FROM orders WHERE id=1 AND ordered_at='2026-01-03 08:00:00+00'),EXISTS(SELECT FROM orders WHERE id=2 AND ordered_at='2026-01-03 08:30:00+00'),EXISTS(SELECT FROM orders WHERE id=3 AND ordered_at='2026-01-03 07:59:59+00') FROM orders",&[]).await.unwrap();
        assert_eq!(stats.get::<_, i64>(0), 100000);
        assert_eq!(stats.get::<_, i64>(1), 65000 - i64::from(seed) * 20000);
        assert!(stats.get::<_, i64>(2) > 0);
        assert_eq!(stats.get::<_, i32>(3), 192);
        assert_eq!(stats.get::<_, i32>(4), 192);
        assert!(
            stats.get::<_, i64>(5) > 0 && stats.get::<_, i64>(6) > 0,
            "Cold key must contain both amount populations"
        );
        for i in 7..11 {
            assert!(stats.get::<_, bool>(i), "Teaching case {i} seed {seed}");
        }
        let metadata = fixtures::catalog();
        let expected = metadata["scan_orders"]["tables"][0]["columns"]
            .as_object()
            .unwrap();
        let columns=client.query("SELECT attname,format_type(atttypid,atttypmod) FROM pg_attribute WHERE attrelid='orders'::regclass AND attnum>0 AND NOT attisdropped",&[]).await.unwrap();
        assert_eq!(columns.len(), expected.len());
        for column in columns {
            let name: String = column.get(0);
            let actual: String = column.get(1);
            let ty = expected[&name]
                .as_str()
                .unwrap()
                .split(" · ")
                .next()
                .unwrap();
            assert_eq!(
                actual,
                if ty == "timestamptz" {
                    "timestamp with time zone"
                } else {
                    ty
                }
            );
        }
        let indexes=client.query("SELECT indexrelid::regclass::text,indisvalid,indisready FROM pg_index WHERE indrelid='orders'::regclass",&[]).await.unwrap();
        assert_eq!(indexes.len(), 4);
        let names: HashSet<_> = indexes
            .iter()
            .map(|r| {
                assert!(r.get::<_, bool>(1) && r.get::<_, bool>(2));
                r.get::<_, String>(0)
            })
            .collect();
        assert_eq!(
            names,
            [
                "orders_pkey",
                "orders_customer_idx",
                "orders_time_idx",
                "orders_status_idx"
            ]
            .into_iter()
            .map(str::to_owned)
            .collect()
        );
        for sql in [
            "UPDATE orders SET total=0 WHERE id=1",
            "CREATE INDEX forbidden_idx ON orders(total)",
            "DROP INDEX orders_customer_idx",
        ] {
            let error = client.batch_execute(sql).await.unwrap_err();
            assert_eq!(error.as_db_error().unwrap().code().code(), "42501");
        }
        let fingerprint:String=client.query_one("SELECT md5(string_agg(concat_ws('|',id,customer_id,status,total,ordered_at,payload),';' ORDER BY id)) FROM orders WHERE id%1000=0",&[]).await.unwrap().get(0);
        if seed == 0 && original.is_some() {
            assert_eq!(Some(fingerprint), original);
        } else {
            assert!(fingerprints.insert(fingerprint.clone()));
            if seed == 0 {
                original = Some(fingerprint);
            }
        }
        let sizes = client
            .query_one(
                "SELECT pg_relation_size('orders'),pg_total_relation_size('orders')",
                &[],
            )
            .await
            .unwrap();
        eprintln!(
            "scan fixture seed {seed}: heap {} bytes, total {} bytes, setup+checks {} ms",
            sizes.get::<_, i64>(0),
            sizes.get::<_, i64>(1),
            started.elapsed().as_millis()
        );
        drop(client);
        cluster.drop_lab(&database, &role).await.unwrap();
    }
    cluster.stop().unwrap();
}
struct Order {
    id: i32,
    customer: i32,
    status: String,
    total: Option<BigDecimal>,
    timestamp: String,
}
#[derive(Default)]
struct Measure {
    count: i64,
    known: i64,
    total: BigDecimal,
}
impl Measure {
    fn add(&mut self, o: &Order) {
        self.count += 1;
        if let Some(n) = &o.total {
            self.known += 1;
            self.total += n;
        }
    }
    fn cells(&self) -> Vec<Option<String>> {
        vec![
            Some(self.count.to_string()),
            Some(self.known.to_string()),
            Some(self.total.to_string()),
        ]
    }
}
fn oracle(orders: &[Order]) -> Vec<QueryResult> {
    assert_eq!(orders.len(), 100000);
    let event = |o: &Order| {
        vec![
            Some(o.id.to_string()),
            Some(o.customer.to_string()),
            Some(o.status.clone()),
            o.total.as_ref().map(ToString::to_string),
        ]
    };
    let point = orders.iter().find(|o| o.id == 45678).unwrap();
    let mut row = event(point);
    row.push(Some(point.timestamp.clone()));
    let first = vec![row];
    let mut days: BTreeMap<String, Measure> = BTreeMap::new();
    let mut broad = Measure::default();
    let mut hot = Measure::default();
    let mut interval = Vec::new();
    let mut known = 0;
    let mut unknown = 0;
    for o in orders {
        if o.customer == 777 {
            days.entry(o.timestamp[..10].to_owned()).or_default().add(o);
            if o.total.is_some() {
                known += 1;
            } else {
                unknown += 1;
            }
        }
        if o.total
            .as_ref()
            .is_some_and(|n| n >= &BigDecimal::from(100))
        {
            broad.add(o);
        }
        if o.customer == 1 && o.status == "paid" {
            hot.add(o);
        }
        if o.timestamp.as_str() >= "2026-01-03 08:00:00+00"
            && o.timestamp.as_str() < "2026-01-03 08:30:00+00"
        {
            interval.push(o);
        }
    }
    assert!(known > 0 && unknown > 0);
    assert!(
        days.values().any(|m| m.count > 0 && m.known == 0),
        "Daily report must exercise an all-null amount group"
    );
    let second = days
        .into_iter()
        .map(|(day, m)| {
            let mut r = vec![Some(day)];
            r.extend(m.cells());
            r
        })
        .collect();
    interval.sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then(a.id.cmp(&b.id)));
    assert!(interval.iter().any(|o| o.id == 1) && !interval.iter().any(|o| o.id == 2 || o.id == 3));
    let third = interval
        .into_iter()
        .map(|o| {
            vec![
                Some(o.id.to_string()),
                Some(o.timestamp.clone()),
                o.total.as_ref().map(ToString::to_string),
            ]
        })
        .collect();
    assert!(
        broad.count > 50000,
        "Broad report must account for substantial input"
    );
    let fourth = vec![vec![
        Some(broad.count.to_string()),
        Some(broad.total.to_string()),
    ]];
    let fifth = vec![hot.cells()];
    let sixth = orders.iter().rev().take(20).map(event).collect();
    [
        (vec![23, 23, 25, 1700, 1184], first),
        (vec![1082, 20, 20, 1700], second),
        (vec![23, 1184, 1700], third),
        (vec![20, 1700], fourth),
        (vec![20, 20, 1700], fifth),
        (vec![23, 23, 25, 1700], sixth),
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
async fn six_scan_queries_match_independent_results_on_every_distribution() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let chapter = academy
        .chapters
        .get(7)
        .expect("Chapter eight must be authored");
    assert!(!chapter.published && chapter.exercises.len() == 6);
    assert!(chapter.exercises.iter().all(|e| e.require_analyze));
    let exercises = chapter.exercises.clone();
    assert!(academy
        .run(request("08-01", "SELECT 1", "run"))
        .await
        .is_err());
    for c in &mut academy.chapters {
        c.published = true;
        for e in &c.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    for seed in 0..3 {
        let database = format!("oracle_{seed}");
        let role = format!("oracle_student_{seed}");
        academy
            .cluster
            .create_lab(&database, &role, Fixture::ScanOrders, seed)
            .await
            .unwrap();
        let client = academy.cluster.connect(&database, &role).await.unwrap();
        let raw = client
            .query(
                "SELECT id,customer_id,status,total::text,ordered_at::text FROM orders ORDER BY id",
                &[],
            )
            .await
            .unwrap();
        let orders: Vec<_> = raw
            .into_iter()
            .map(|r| Order {
                id: r.get(0),
                customer: r.get(1),
                status: r.get(2),
                total: r
                    .get::<_, Option<String>>(3)
                    .map(|s| BigDecimal::from_str(&s).unwrap()),
                timestamp: r.get(4),
            })
            .collect();
        for (e, expected) in exercises.iter().zip(oracle(&orders)) {
            assert!(expected.rows.len() <= 1000);
            let actual = execute(&client, &e.solution_sql, "run", 1000)
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
            let estimate = execute(&client, &e.solution_sql, "explain", 1000)
                .await
                .unwrap();
            assert!(estimate.error.is_none());
            assert!(estimate.plan.as_ref().unwrap()[0]["Plan"]["Actual Rows"].is_null());
            let measured = execute(&client, &e.solution_sql, "analyze", 1000)
                .await
                .unwrap();
            assert!(measured.error.is_none());
            assert_eq!(
                measured.plan.as_ref().unwrap()[0]["Plan"]["Actual Rows"].as_f64(),
                Some(expected.rows.len() as f64)
            );
            fn visit(n: &Value, types: &mut Vec<String>) {
                types.push(n["Node Type"].as_str().unwrap().into());
                assert!(n["Actual Rows"].is_number() && n["Actual Loops"].is_number());
                if let Some(children) = n["Plans"].as_array() {
                    for child in children {
                        visit(child, types);
                    }
                }
            }
            let mut types = vec![];
            visit(&measured.plan.as_ref().unwrap()[0]["Plan"], &mut types);
            eprintln!("{} seed {seed}: {}", e.id, types.join(" -> "));
        }
        drop(client);
        academy.cluster.drop_lab(&database, &role).await.unwrap();
    }
    for e in &exercises {
        let measured = academy
            .run(request(&e.id, &e.solution_sql, "analyze"))
            .await
            .unwrap();
        assert!(measured.error.is_none());
        assert!(
            academy
                .check(request(&e.id, &e.solution_sql, "run"))
                .await
                .unwrap()
                .passed
        );
    }
    for(id,sql)in [
        ("08-02","SELECT (ordered_at AT TIME ZONE 'UTC')::date,COUNT(*),COUNT(total),COALESCE(SUM(total),0) FROM orders WHERE customer_id=777 AND total IS NOT NULL GROUP BY 1 ORDER BY 1"),
        ("08-02","SELECT (ordered_at AT TIME ZONE 'UTC')::date,COUNT(*),COUNT(total),SUM(total) FROM orders WHERE customer_id=777 GROUP BY 1 ORDER BY 1"),
        ("08-03","SELECT id,ordered_at,total FROM orders WHERE ordered_at>='2026-01-03 08:00:00+00' AND ordered_at<='2026-01-03 08:30:00+00' ORDER BY ordered_at,id"),
        ("08-03","SELECT id,ordered_at,total FROM orders WHERE ordered_at>'2026-01-03 08:00:00+00' AND ordered_at<'2026-01-03 08:30:00+00' ORDER BY ordered_at,id"),
        ("08-04","SELECT COUNT(*),COALESCE(SUM(total),0) FROM orders WHERE total>=100 AND status='paid'"),
        ("08-05","SELECT COUNT(total),COUNT(total),COALESCE(SUM(total),0) FROM orders WHERE customer_id=1 AND status='paid'"),
        ("08-06","SELECT id,customer_id,status,total FROM orders ORDER BY id LIMIT 20"),
    ]{let measured=academy.run(request(id,sql,"analyze")).await.unwrap();assert!(measured.error.is_none());assert!(!academy.check(request(id,sql,"run")).await.unwrap().passed,"Wrong report {id} must fail");}
    let sql = &exercises[0].solution_sql;
    let before = academy
        .run(request("08-01", sql, "run"))
        .await
        .unwrap()
        .rows;
    let admin = academy
        .cluster
        .connect("lab_08_01", "academy_admin")
        .await
        .unwrap();
    admin
        .batch_execute("UPDATE orders SET total=0 WHERE id=45678")
        .await
        .unwrap();
    drop(admin);
    assert_ne!(
        academy
            .run(request("08-01", sql, "run"))
            .await
            .unwrap()
            .rows,
        before
    );
    academy.reset("08-01").await.unwrap();
    assert_eq!(
        academy
            .run(request("08-01", sql, "run"))
            .await
            .unwrap()
            .rows,
        before
    );
    assert_eq!(
        academy.progress().await.unwrap()["08-01"]["analyzed"],
        false
    );
    let admin = academy
        .cluster
        .connect("postgres", "academy_admin")
        .await
        .unwrap();
    let leftovers:i64=admin.query_one("SELECT COUNT(*) FROM pg_database WHERE datname LIKE 'grade_%' OR datname LIKE 'reference_%'",&[]).await.unwrap().get(0);
    assert_eq!(leftovers, 0);
    academy.shutdown().await.unwrap();
}
