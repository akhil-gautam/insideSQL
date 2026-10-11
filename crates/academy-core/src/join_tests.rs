use super::*;
use bigdecimal::BigDecimal;
use std::collections::{BTreeMap, HashSet};

fn runtime() -> std::path::PathBuf {
    std::env::var_os("ACADEMY_POSTGRES")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into())
}
fn fixture() -> Fixture {
    serde_json::from_str("\"join_orders\"").expect("The relational 100k fixture must be registered")
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
async fn relational_fixture_preserves_populations_constraints_and_read_only_permissions() {
    let fixture = fixture();
    assert!(fixture.writable_tables().is_empty() && fixture.sql(3).is_err());
    let home = tempfile::tempdir().unwrap();
    let cluster = Cluster::start(home.path(), &runtime()).await.unwrap();
    let mut fingerprints = HashSet::new();
    let mut original = None;
    for seed in [0u32, 1, 2, 0] {
        let database = format!("join_fixture_{seed}");
        let role = format!("join_student_{seed}");
        let started = Instant::now();
        cluster
            .create_lab(&database, &role, fixture, seed)
            .await
            .unwrap();
        let client = cluster.connect(&database, &role).await.unwrap();
        let counts=client.query_one("SELECT (SELECT COUNT(*) FROM customers),(SELECT COUNT(*) FROM orders),(SELECT COUNT(*) FROM orders WHERE customer_id=1),(SELECT COUNT(*) FROM order_items),(SELECT COUNT(*) FROM payments),(SELECT COUNT(DISTINCT name) FROM customers),(SELECT COUNT(*) FROM customers c WHERE NOT EXISTS(SELECT 1 FROM orders o WHERE o.customer_id=c.id)),(SELECT COUNT(*) FROM payments WHERE order_id IS NULL AND status='captured')",&[]).await.unwrap();
        let items: i64 = (1..=100000)
            .filter(|g| g % 7 != 0)
            .map(|g| 1 + (g + seed) % 4)
            .map(i64::from)
            .sum::<i64>()
            + 4;
        let payments: i64 = (1..=100000)
            .filter(|g| g % 11 != 0)
            .map(|g| 1 + (g + seed) % 3)
            .map(i64::from)
            .sum::<i64>()
            + 5;
        assert_eq!(counts.get::<_, i64>(0), 1000);
        assert_eq!(counts.get::<_, i64>(1), 100000);
        assert_eq!(counts.get::<_, i64>(2), 65000 - i64::from(seed) * 20000);
        assert_eq!(counts.get::<_, i64>(3), items);
        assert_eq!(counts.get::<_, i64>(4), payments);
        assert!(counts.get::<_, i64>(5) < 1000 && counts.get::<_, i64>(6) >= 2);
        assert_eq!(counts.get::<_, i64>(7), 1);
        let cases=client.query_one("SELECT EXISTS(SELECT FROM orders WHERE customer_id=777 AND total IS NULL),EXISTS(SELECT FROM orders WHERE customer_id=777 AND total IS NOT NULL),EXISTS(SELECT FROM order_items WHERE order_id=97 AND quantity=3 GROUP BY order_id,sku,quantity,unit_price HAVING COUNT(*)>=2),EXISTS(SELECT FROM payments WHERE order_id=97 AND status='captured' GROUP BY order_id,amount HAVING COUNT(*)>=2),NOT EXISTS(SELECT FROM orders WHERE customer_id IN(999,1000)),EXISTS(SELECT FROM orders o WHERE o.status='paid' AND (SELECT COUNT(*) FROM order_items i WHERE i.order_id=o.id AND i.quantity>=3)>=2)",&[]).await.unwrap();
        for index in 0..6 {
            assert!(
                cases.get::<_, bool>(index),
                "Teaching case {index}, seed {seed}"
            );
        }
        for table in fixtures::catalog()["join_orders"]["tables"]
            .as_array()
            .unwrap()
        {
            let name = table["name"].as_str().unwrap();
            let columns=client.query("SELECT attname,format_type(atttypid,atttypmod) FROM pg_attribute WHERE attrelid=$1::text::regclass AND attnum>0 AND NOT attisdropped",&[&name]).await.unwrap();
            let metadata = table["columns"].as_object().unwrap();
            assert_eq!(columns.len(), metadata.len());
            for column in columns {
                let field: String = column.get(0);
                let actual: String = column.get(1);
                let expected = metadata[&field]
                    .as_str()
                    .unwrap()
                    .split(" · ")
                    .next()
                    .unwrap();
                assert_eq!(
                    actual,
                    if expected == "timestamptz" {
                        "timestamp with time zone"
                    } else {
                        expected
                    }
                );
            }
            let error = client
                .batch_execute(&format!("DELETE FROM {name}"))
                .await
                .unwrap_err();
            assert_eq!(error.as_db_error().unwrap().code().code(), "42501");
        }
        let constraints=client.query("SELECT conname,convalidated FROM pg_constraint WHERE contype='f' AND connamespace='public'::regnamespace",&[]).await.unwrap();
        assert_eq!(constraints.len(), 3);
        assert!(constraints.iter().all(|r| r.get::<_, bool>(1)));
        let indexes=client.query("SELECT indexrelid::regclass::text,indisvalid,indisready FROM pg_index WHERE indrelid IN('orders'::regclass,'customers'::regclass,'order_items'::regclass,'payments'::regclass)",&[]).await.unwrap();
        assert_eq!(indexes.len(), 9);
        assert!(indexes
            .iter()
            .all(|r| r.get::<_, bool>(1) && r.get::<_, bool>(2)));
        let names: HashSet<String> = indexes.iter().map(|r| r.get(0)).collect();
        assert!(names.contains("order_items_order_idx") && names.contains("payments_order_idx"));
        let mut fingerprint = String::new();
        for sql in ["SELECT md5(string_agg(concat_ws('|',id,name),';' ORDER BY id)) FROM customers", "SELECT md5(string_agg(concat_ws('|',id,customer_id,status,total,ordered_at,payload),';' ORDER BY id)) FROM orders WHERE id%1000=0", "SELECT md5(string_agg(concat_ws('|',id,order_id,sku,quantity,unit_price),';' ORDER BY id)) FROM order_items WHERE id%1000=1 OR id>=2000000", "SELECT md5(string_agg(concat_ws('|',id,order_id,status,amount,occurred_at),';' ORDER BY id)) FROM payments WHERE id%1000=1 OR id>=2000000"] {fingerprint.push_str(&client.query_one(sql,&[]).await.unwrap().get::<_,String>(0));}
        if seed == 0 && original.is_some() {
            assert_eq!(Some(fingerprint), original);
        } else {
            assert!(fingerprints.insert(fingerprint.clone()));
            if seed == 0 {
                original = Some(fingerprint);
            }
        }
        let bytes:i64=client.query_one("SELECT SUM(pg_total_relation_size(oid))::bigint FROM pg_class WHERE oid IN('orders'::regclass,'customers'::regclass,'order_items'::regclass,'payments'::regclass)",&[]).await.unwrap().get(0);
        eprintln!("join fixture seed {seed}: {items} lines, {payments} payments, {bytes} total bytes, setup+checks {} ms",started.elapsed().as_millis());
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
    fn add(&mut self, order: &Order) {
        self.count += 1;
        if let Some(amount) = &order.total {
            self.known += 1;
            self.total += amount;
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
#[derive(Default)]
struct Lines {
    count: i64,
    units: i64,
}
#[derive(Default)]
struct Receipts {
    count: i64,
    total: BigDecimal,
}

async fn oracle(client: &Client) -> Vec<QueryResult> {
    let customers: BTreeMap<i32, String> = client
        .query("SELECT id,name FROM customers ORDER BY id", &[])
        .await
        .unwrap()
        .into_iter()
        .map(|r| (r.get(0), r.get(1)))
        .collect();
    let orders: BTreeMap<i32, Order> = client
        .query(
            "SELECT id,customer_id,status,total::text,ordered_at::text FROM orders ORDER BY id",
            &[],
        )
        .await
        .unwrap()
        .into_iter()
        .map(|r| {
            let id = r.get(0);
            (
                id,
                Order {
                    id,
                    customer: r.get(1),
                    status: r.get(2),
                    total: r
                        .get::<_, Option<String>>(3)
                        .map(|s| BigDecimal::from_str(&s).unwrap()),
                    timestamp: r.get(4),
                },
            )
        })
        .collect();
    assert_eq!(orders.len(), 100000);
    assert_eq!(customers.len(), 1000);
    let mut lines: BTreeMap<i32, Lines> = BTreeMap::new();
    let mut qualifying = HashSet::new();
    let mut skus: BTreeMap<String, (i64, i64, BigDecimal)> = BTreeMap::new();
    for r in client
        .query(
            "SELECT order_id,sku,quantity,unit_price::text FROM order_items",
            &[],
        )
        .await
        .unwrap()
    {
        let id: i32 = r.get(0);
        let quantity = i64::from(r.get::<_, i32>(2));
        let value = BigDecimal::from_str(r.get::<_, String>(3).as_str()).unwrap();
        let line = lines.entry(id).or_default();
        line.count += 1;
        line.units += quantity;
        if quantity >= 3 {
            qualifying.insert(id);
        }
        if orders[&id].status == "paid" {
            let sku = skus.entry(r.get(1)).or_default();
            sku.0 += 1;
            sku.1 += quantity;
            sku.2 += value * BigDecimal::from(quantity);
        }
    }
    let mut receipts: BTreeMap<i32, Receipts> = BTreeMap::new();
    let mut unassigned = 0;
    for r in client
        .query("SELECT order_id,status,amount::text FROM payments", &[])
        .await
        .unwrap()
    {
        if r.get::<_, String>(1) != "captured" {
            continue;
        }
        if let Some(id) = r.get::<_, Option<i32>>(0) {
            let receipt = receipts.entry(id).or_default();
            receipt.count += 1;
            receipt.total += BigDecimal::from_str(r.get::<_, String>(2).as_str()).unwrap();
        } else {
            unassigned += 1;
        }
    }
    assert_eq!(unassigned, 1);
    assert!(lines[&97].count >= 2 && receipts[&97].count >= 2);
    let cold: Vec<_> = orders.values().filter(|o| o.customer == 777).collect();
    assert!(cold.iter().any(|o| o.total.is_none()) && cold.iter().any(|o| o.total.is_some()));
    assert!(cold.iter().any(|o| !lines.contains_key(&o.id)));
    assert!(cold.iter().any(|o| !receipts.contains_key(&o.id)));
    let first = cold
        .iter()
        .map(|o| {
            vec![
                Some(o.id.to_string()),
                Some(customers[&o.customer].clone()),
                Some(o.status.clone()),
                o.total.as_ref().map(ToString::to_string),
            ]
        })
        .collect();
    let mut paid: BTreeMap<i32, Measure> = BTreeMap::new();
    let mut exists = Measure::default();
    let mut absence: BTreeMap<String, Measure> = BTreeMap::new();
    for order in orders.values().filter(|o| o.status == "paid") {
        paid.entry(order.customer).or_default().add(order);
        if qualifying.contains(&order.id) {
            exists.add(order);
        }
        if order.customer == 1 && !receipts.contains_key(&order.id) {
            absence
                .entry(order.timestamp[..10].to_owned())
                .or_default()
                .add(order);
        }
    }
    assert!(!absence.is_empty() && exists.count > 0);
    assert!(!paid.contains_key(&999) && !paid.contains_key(&1000));
    let second = customers
        .iter()
        .map(|(id, name)| {
            let mut row = vec![Some(id.to_string()), Some(name.clone())];
            row.extend(
                paid.get(id)
                    .map(Measure::cells)
                    .unwrap_or_else(|| Measure::default().cells()),
            );
            row
        })
        .collect();
    let third = vec![exists.cells()];
    let fourth = absence
        .into_iter()
        .map(|(day, m)| {
            let mut row = vec![Some(day)];
            row.extend(m.cells());
            row
        })
        .collect();
    let fifth = skus
        .into_iter()
        .map(|(sku, (count, units, total))| {
            vec![
                Some(sku),
                Some(count.to_string()),
                Some(units.to_string()),
                Some(total.to_string()),
            ]
        })
        .collect();
    let sixth = cold
        .into_iter()
        .map(|o| {
            let line = lines.get(&o.id);
            let receipt = receipts.get(&o.id);
            vec![
                Some(o.id.to_string()),
                o.total.as_ref().map(ToString::to_string),
                Some(line.map_or(0, |l| l.count).to_string()),
                Some(line.map_or(0, |l| l.units).to_string()),
                Some(receipt.map_or(0, |r| r.count).to_string()),
                Some(
                    receipt
                        .map(|r| r.total.clone())
                        .unwrap_or_default()
                        .to_string(),
                ),
            ]
        })
        .collect();
    [
        (vec![23, 25, 25, 1700], first),
        (vec![23, 25, 20, 20, 1700], second),
        (vec![20, 20, 1700], third),
        (vec![1082, 20, 20, 1700], fourth),
        (vec![25, 20, 20, 1700], fifth),
        (vec![23, 1700, 20, 20, 20, 1700], sixth),
    ]
    .into_iter()
    .map(|(column_types, rows)| QueryResult {
        column_types,
        rows,
        ..Default::default()
    })
    .collect()
}
fn inspect(plan: &Value, nodes: &mut Vec<String>) {
    assert!(plan["Actual Rows"].is_number() && plan["Actual Loops"].is_number());
    nodes.push(plan["Node Type"].as_str().unwrap().into());
    if let Some(children) = plan["Plans"].as_array() {
        for child in children {
            inspect(child, nodes);
        }
    }
}

#[tokio::test]
async fn six_join_reports_match_independent_relations_and_measured_grain() {
    let home = tempfile::tempdir().unwrap();
    let mut academy = Academy::start(home.path(), &runtime()).await.unwrap();
    let chapter = academy
        .chapters
        .get(8)
        .expect("Chapter nine must be authored");
    assert!(
        !chapter.published
            && chapter.exercises.len() == 6
            && chapter.exercises.iter().all(|e| e.require_analyze)
    );
    let exercises = chapter.exercises.clone();
    assert!(academy
        .run(request("09-01", "SELECT 1", "run"))
        .await
        .is_err());
    for chapter in &mut academy.chapters {
        chapter.published = true;
        for e in &chapter.exercises {
            academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
            academy.store.pass(&e.id, &e.revision).unwrap();
        }
    }
    for seed in 0..3 {
        let database = format!("join_oracle_{seed}");
        let role = format!("join_oracle_student_{seed}");
        academy
            .cluster
            .create_lab(&database, &role, fixture(), seed)
            .await
            .unwrap();
        let client = academy.cluster.connect(&database, &role).await.unwrap();
        let expected = oracle(&client).await;
        client
            .batch_execute("SET TIME ZONE 'Asia/Kolkata'")
            .await
            .unwrap();
        for (e, expected) in exercises.iter().zip(&expected) {
            assert!(expected.rows.len() <= 1000);
            let actual = execute(&client, &e.solution_sql, "run", 1000)
                .await
                .unwrap();
            assert!(
                actual.error.is_none() && !actual.truncated,
                "{} seed {seed}: {:?}",
                e.id,
                actual.error
            );
            assert_eq!(
                actual.column_types, expected.column_types,
                "{} seed {seed}: reference must honor the declared result types",
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
            let plan = &measured.plan.as_ref().unwrap()[0]["Plan"];
            assert_eq!(
                plan["Actual Rows"].as_f64(),
                Some(expected.rows.len() as f64)
            );
            let mut nodes = Vec::new();
            inspect(plan, &mut nodes);
            eprintln!("{} seed {seed}: {}", e.id, nodes.join(" > "));
        }
        // Only test-owned connections constrain paths; equivalent algorithms must retain the report.
        for (settings, node) in [
            (
                "SET enable_hashjoin=off; SET enable_mergejoin=off",
                "Nested Loop",
            ),
            (
                "SET enable_nestloop=off; SET enable_mergejoin=off",
                "Hash Join",
            ),
            (
                "SET enable_nestloop=off; SET enable_hashjoin=off",
                "Merge Join",
            ),
        ] {
            client
                .batch_execute(
                    "RESET enable_hashjoin; RESET enable_nestloop; RESET enable_mergejoin",
                )
                .await
                .unwrap();
            client.batch_execute(settings).await.unwrap();
            let sql="SELECT o.id FROM orders o JOIN customers c ON c.id=o.customer_id WHERE o.id>=40000 AND o.id<41000 ORDER BY o.id";
            let measured = execute(&client, sql, "analyze", 1000).await.unwrap();
            assert!(measured.error.is_none());
            let mut nodes = Vec::new();
            inspect(&measured.plan.as_ref().unwrap()[0]["Plan"], &mut nodes);
            assert!(
                nodes.iter().any(|n| n == node),
                "Expected controlled {node}: {nodes:?}"
            );
            let actual = execute(&client, sql, "run", 1000).await.unwrap();
            assert_eq!(
                actual.rows,
                (40000..41000)
                    .map(|id| vec![Some(id.to_string())])
                    .collect::<Vec<_>>()
            );
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
    for (id,sql) in [
        ("09-01","SELECT o.id,c.name,o.status,o.total FROM orders o JOIN customers c ON c.id=o.customer_id WHERE o.customer_id=777 AND o.total IS NOT NULL ORDER BY o.id"),
        ("09-02","SELECT c.id,c.name,COUNT(o.id),COUNT(o.total),COALESCE(SUM(o.total),0) FROM customers c LEFT JOIN orders o ON o.customer_id=c.id WHERE o.status='paid' GROUP BY c.id,c.name ORDER BY c.id"),
        ("09-02","SELECT c.id,c.name,COUNT(*),COUNT(o.total),COALESCE(SUM(o.total),0) FROM customers c LEFT JOIN orders o ON o.customer_id=c.id AND o.status='paid' GROUP BY c.id,c.name ORDER BY c.id"),
        ("09-03","SELECT COUNT(*),COUNT(o.total),COALESCE(SUM(o.total),0) FROM orders o JOIN order_items i ON i.order_id=o.id WHERE o.status='paid' AND i.quantity>=3"),
        ("09-04","SELECT (o.ordered_at AT TIME ZONE 'UTC')::date,COUNT(*),COUNT(o.total),COALESCE(SUM(o.total),0) FROM orders o WHERE o.customer_id=1 AND o.status='paid' AND o.id NOT IN(SELECT p.order_id FROM payments p WHERE p.status='captured') GROUP BY 1 ORDER BY 1"),
        ("09-05","SELECT i.sku,COUNT(*),SUM(i.quantity),COALESCE(SUM(DISTINCT i.quantity*i.unit_price),0) FROM order_items i JOIN orders o ON o.id=i.order_id WHERE o.status='paid' GROUP BY i.sku ORDER BY i.sku"),
        ("09-06","SELECT o.id,o.total,COUNT(i.id),COALESCE(SUM(i.quantity),0),COUNT(p.id),COALESCE(SUM(p.amount),0) FROM orders o LEFT JOIN order_items i ON i.order_id=o.id LEFT JOIN payments p ON p.order_id=o.id AND p.status='captured' WHERE o.customer_id=777 GROUP BY o.id,o.total ORDER BY o.id"),
    ]{let measured=academy.run(request(id,sql,"analyze")).await.unwrap();assert!(measured.error.is_none(),"Wrong report {id}: {:?}",measured.error);assert!(!academy.check(request(id,sql,"run")).await.unwrap().passed,"Wrong report {id} must fail even after measurement");}
    let e = &exercises[0];
    let before = academy
        .run(request(&e.id, &e.solution_sql, "run"))
        .await
        .unwrap()
        .rows;
    let admin = academy
        .cluster
        .connect("lab_09_01", "academy_admin")
        .await
        .unwrap();
    admin
        .batch_execute("UPDATE customers SET name='Changed private lab' WHERE id=777")
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
