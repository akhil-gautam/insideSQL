use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Fixture {
    #[default]
    Orders,
    Commerce,
    CommerceWrites,
    ScanOrders,
}

impl Fixture {
    pub fn sql(self, seed: u32) -> crate::Result<String> {
        if seed > 2 {
            return Err("Unknown assessment fixture variant".into());
        }
        let source = match self {
            Self::Orders => include_str!("../../../course/fixtures/orders.sql").to_owned(),
            Self::Commerce => include_str!("../../../course/fixtures/commerce.sql").to_owned(),
            Self::ScanOrders => include_str!("../../../course/fixtures/scan-orders.sql").to_owned(),
            Self::CommerceWrites => format!(
                "{}\n{}",
                include_str!("../../../course/fixtures/commerce.sql"),
                include_str!("../../../course/fixtures/commerce-writes.sql")
            ),
        };
        Ok(source.replace("{{seed}}", &seed.to_string()))
    }
    pub fn writable_tables(self) -> &'static [&'static str] {
        match self {
            Self::CommerceWrites => &["orders"],
            _ => &[],
        }
    }
}

pub fn catalog() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../course/fixtures/catalog.json"))
        .expect("Authored fixture catalog must be valid JSON")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Cluster;
    use std::path::PathBuf;

    #[tokio::test]
    async fn commerce_variants_match_the_schema_and_preserve_teaching_cases() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(PathBuf::from)
            .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
        let cluster = Cluster::start(home.path(), &runtime).await.unwrap();
        let mut fingerprints = std::collections::HashSet::new();
        for seed in 0..3 {
            let database = format!("commerce_{seed}");
            let role = format!("commerce_student_{seed}");
            cluster
                .create_lab(&database, &role, Fixture::Commerce, seed)
                .await
                .unwrap();
            let client = cluster.connect(&database, &role).await.unwrap();
            let metadata = catalog();
            for table in metadata["commerce"]["tables"].as_array().unwrap() {
                let name = table["name"].as_str().unwrap();
                let columns=client.query("SELECT a.attname,format_type(a.atttypid,a.atttypmod) FROM pg_attribute a JOIN pg_class c ON c.oid=a.attrelid JOIN pg_namespace n ON n.oid=c.relnamespace WHERE n.nspname='public' AND c.relname=$1 AND a.attnum>0 AND NOT a.attisdropped",&[&name]).await.unwrap();
                assert_eq!(
                    columns.len(),
                    table["columns"].as_object().unwrap().len(),
                    "{name}"
                );
                for column in columns {
                    let field: String = column.get(0);
                    let datatype: String = column.get(1);
                    let expected = table["columns"][&field]
                        .as_str()
                        .unwrap()
                        .split(" · ")
                        .next()
                        .unwrap();
                    assert_eq!(
                        datatype,
                        if expected == "timestamptz" {
                            "timestamp with time zone"
                        } else {
                            expected
                        },
                        "{name}.{field}"
                    );
                }
            }
            let row=client.query_one("SELECT (SELECT COUNT(*) FROM customers), (SELECT COUNT(*) FROM orders), (SELECT COUNT(*) FROM orders WHERE customer_id=201), EXISTS(SELECT FROM customers c WHERE NOT EXISTS(SELECT FROM orders o WHERE o.customer_id=c.id)), EXISTS(SELECT FROM customers c WHERE EXISTS(SELECT FROM orders o WHERE o.customer_id=c.id) AND NOT EXISTS(SELECT FROM orders o WHERE o.customer_id=c.id AND o.status='paid')), EXISTS(SELECT FROM payments WHERE order_id IS NULL AND status='captured'), EXISTS(SELECT FROM order_items GROUP BY order_id,sku,quantity,unit_price HAVING COUNT(*)>1), EXISTS(SELECT FROM payments WHERE status='captured' GROUP BY order_id,amount HAVING COUNT(*)>1), EXISTS(SELECT FROM orders o WHERE (SELECT COUNT(*) FROM order_items i WHERE i.order_id=o.id)>1 AND (SELECT COUNT(*) FROM payments p WHERE p.order_id=o.id AND p.status='captured')>1)",&[]).await.unwrap();
            assert_eq!(row.get::<_, i64>(0), 52);
            assert_eq!(row.get::<_, i64>(1), 310 + i64::from(seed) * 80);
            assert!(
                row.get::<_, i64>(2) > i64::from(300 + seed * 80) / 2,
                "Hot customer must dominate activity"
            );
            for index in 3..9 {
                assert!(
                    row.get::<_, bool>(index),
                    "Teaching case {index} missing from variant {seed}"
                );
            }
            let fingerprint:String=client.query_one("SELECT md5(string_agg(concat_ws('|',id,customer_id,status,total,ordered_at),';' ORDER BY id)) FROM orders",&[]).await.unwrap().get(0);
            assert!(
                fingerprints.insert(fingerprint),
                "Assessment variants must differ"
            );
            drop(client);
            cluster.drop_lab(&database, &role).await.unwrap();
        }
        assert!(serde_json::from_str::<Fixture>("\"untrusted_fixture\"").is_err());
        assert!(Fixture::Commerce.sql(3).is_err());
        cluster.stop().unwrap();
    }
}
