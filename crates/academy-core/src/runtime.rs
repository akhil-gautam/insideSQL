use crate::{fixtures::Fixture, Result};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::{self, File},
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Mutex,
    time::Duration,
};
use tokio_postgres::{Client, Config, NoTls};
mod reset;
pub struct Cluster {
    pub root: PathBuf,
    pub port: u16,
    password: String,
    system_identifier: String,
    bin: PathBuf,
    child: Mutex<Option<Child>>,
    pub(crate) minimum_free_disk_bytes: u64,
}
impl Cluster {
    pub async fn start(home: &Path, postgres: &Path) -> Result<Self> {
        Self::start_with_disk_floor(home, postgres, crate::resources::MIN_FREE_DISK_BYTES).await
    }
    async fn start_with_disk_floor(
        home: &Path,
        postgres: &Path,
        minimum_free_disk_bytes: u64,
    ) -> Result<Self> {
        fs::create_dir_all(home).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(home, fs::Permissions::from_mode(0o700))
                .map_err(|e| e.to_string())?;
        }
        let root = home.canonicalize().map_err(|e| e.to_string())?;
        Self::check_disk_space(&root, minimum_free_disk_bytes)?;
        let bin = postgres.join("bin");
        let version = Command::new(bin.join("postgres"))
            .arg("--version")
            .output()
            .map_err(|e| format!("PostgreSQL runtime missing: {e}"))?;
        if !String::from_utf8_lossy(&version.stdout).contains(" 18.") {
            return Err("This course requires its pinned PostgreSQL 18 runtime".into());
        }
        let password_path = root.join("credentials");
        let password = if password_path.exists() {
            fs::read_to_string(&password_path).map_err(|e| e.to_string())?
        } else {
            let value = uuid::Uuid::new_v4().to_string();
            fs::write(&password_path, &value).map_err(|e| e.to_string())?;
            value
        };
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&password_path, fs::Permissions::from_mode(0o600))
                .map_err(|e| e.to_string())?;
        }
        if !root.join("data/PG_VERSION").exists() {
            let output = Command::new(bin.join("initdb"))
                .arg("-D")
                .arg(root.join("data"))
                .args([
                    "-U",
                    "academy_admin",
                    "--auth=scram-sha-256",
                    "--encoding=UTF8",
                    "--locale=C",
                    "--pwfile",
                ])
                .arg(&password_path)
                .output()
                .map_err(|e| e.to_string())?;
            if !output.status.success() {
                return Err(format!(
                    "Database initialization failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        let port = TcpListener::bind("127.0.0.1:0")
            .map_err(|e| e.to_string())?
            .local_addr()
            .map_err(|e| e.to_string())?
            .port();
        let log = File::create(root.join("postgres.log")).map_err(|e| e.to_string())?;
        let child = Command::new(bin.join("postgres"))
            .arg("-D")
            .arg(root.join("data"))
            .args([
                "-h",
                "127.0.0.1",
                "-p",
                &port.to_string(),
                "-c",
                "unix_socket_directories=",
                "-c",
                "shared_buffers=64MB",
                "-c",
                "work_mem=4MB",
                "-c",
                "temp_file_limit=128MB",
                "-c",
                "max_connections=24",
                "-c",
                "timezone=UTC",
                "-c",
                "log_statement=none",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::from(log.try_clone().map_err(|e| e.to_string())?))
            .stderr(Stdio::from(log))
            .spawn()
            .map_err(|e| e.to_string())?;
        let mut cluster = Self {
            root,
            port,
            password,
            system_identifier: String::new(),
            bin,
            child: Mutex::new(Some(child)),
            minimum_free_disk_bytes,
        };
        for _ in 0..200 {
            if let Ok(admin) = cluster.connect("postgres", "academy_admin").await {
                admin.batch_execute("REVOKE CONNECT ON DATABASE postgres FROM PUBLIC; REVOKE CONNECT ON DATABASE template1 FROM PUBLIC; REVOKE CONNECT ON DATABASE template0 FROM PUBLIC;").await.map_err(|e|e.to_string())?;
                cluster.system_identifier = admin
                    .query_one(
                        "SELECT system_identifier::text FROM pg_catalog.pg_control_system()",
                        &[],
                    )
                    .await
                    .map_err(|e| e.to_string())?
                    .get(0);
                cluster.initialize_reset_recovery().await?;
                return Ok(cluster);
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        Err("PostgreSQL did not start; inspect the app's postgres.log".into())
    }
    pub async fn connect(&self, database: &str, role: &str) -> Result<Client> {
        let mut config = Config::new();
        config
            .host("127.0.0.1")
            .port(self.port)
            .user(role)
            .password(self.password_for(role))
            .dbname(database)
            .application_name("sql-internals-academy")
            .connect_timeout(Duration::from_secs(3));
        let (client, connection) = config.connect(NoTls).await.map_err(|e| e.to_string())?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        client.batch_execute("SET statement_timeout='8s'; SET lock_timeout='2s'; SET idle_in_transaction_session_timeout='10s'; SET timezone='UTC'; SET datestyle='ISO, YMD';").await.map_err(|e|e.to_string())?;
        Ok(client)
    }
    pub(crate) async fn connection_identity(&self, client: &Client) -> Result<String> {
        let row = client.query_one("SELECT datname,oid FROM pg_catalog.pg_database WHERE datname=pg_catalog.current_database()",&[]).await.map_err(|e|e.to_string())?;
        let database: String = row.get(0);
        let oid: u32 = row.get(1);
        let admin = self.connect("postgres", "academy_admin").await?;
        let generation: String = admin.query_one("SELECT g.generation::text FROM academy_runtime.lab_generations g JOIN pg_database d ON d.datname=g.database_name AND d.oid=g.database_oid JOIN pg_roles r ON r.oid=d.datdba WHERE g.database_name=$1 AND g.database_oid=$2 AND r.rolname='academy_admin'",&[&database,&oid]).await.map_err(|e|format!("Could not confirm the measured lab generation: {e}"))?.get(0);
        Ok(format!("{}:{oid}:{generation}", self.system_identifier))
    }
    pub(crate) async fn lab_identities(
        &self,
        databases: &[String],
    ) -> Result<HashMap<String, String>> {
        let admin = self.connect("postgres", "academy_admin").await?;
        let rows = admin
            .query(
                "SELECT d.datname,d.oid,g.generation::text FROM pg_catalog.pg_database d JOIN academy_runtime.lab_generations g ON g.database_name=d.datname AND g.database_oid=d.oid JOIN pg_roles r ON r.oid=d.datdba WHERE d.datname=ANY($1) AND r.rolname='academy_admin'",
                &[&databases],
            )
            .await
            .map_err(|e| e.to_string())?;
        Ok(rows
            .into_iter()
            .map(|row| {
                let database: String = row.get(0);
                let oid: u32 = row.get(1);
                let generation: String = row.get(2);
                (
                    database,
                    format!("{}:{oid}:{generation}", self.system_identifier),
                )
            })
            .collect())
    }
    fn check_disk_space(root: &Path, minimum: u64) -> Result<u64> {
        let available = crate::resources::available_disk_bytes(root)?;
        if available < minimum {
            return Err(format!("Free up disk space before allocating or running this lab: {} MiB available, at least {} MiB required. Existing lab data and saved scripts were kept.", available / 1024 / 1024, minimum / 1024 / 1024));
        }
        Ok(available)
    }
    pub(crate) fn require_disk_space(&self) -> Result<u64> {
        Self::check_disk_space(&self.root, self.minimum_free_disk_bytes)
    }
    pub async fn create_lab(
        &self,
        database: &str,
        role: &str,
        fixture: Fixture,
        seed: u32,
    ) -> Result<()> {
        let setup = fixture.sql(seed)?;
        self.require_disk_space()?;
        let admin = self.connect("postgres", "academy_admin").await?;
        admin.batch_execute(&format!("CREATE ROLE {role} LOGIN PASSWORD '{}' NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;",self.password_for(role))).await.map_err(|e|e.to_string())?;
        if let Err(error) = self
            .create_database(database, role, fixture, &setup, None)
            .await
        {
            admin
                .batch_execute(&format!("DROP ROLE {role};"))
                .await
                .map_err(|cleanup| {
                    format!("Lab creation failed: {error}; role cleanup failed: {cleanup}")
                })?;
            return Err(error);
        }
        Ok(())
    }
    async fn create_database(
        &self,
        database: &str,
        role: &str,
        fixture: Fixture,
        setup: &str,
        oid: Option<u32>,
    ) -> Result<()> {
        self.require_disk_space()?;
        let admin = self.connect("postgres", "academy_admin").await?;
        let oid_clause = oid
            .map(|oid| format!(" WITH OID={oid}"))
            .unwrap_or_default();
        admin
            .batch_execute(&format!("CREATE DATABASE {database}{oid_clause};"))
            .await
            .map_err(|e| e.to_string())?;
        let initialized: Result<()> = async {
            self.seed(database, role, fixture, setup).await?;
            if database.starts_with("lab_") {
                admin.execute("INSERT INTO academy_runtime.lab_generations(database_name,database_oid,generation) SELECT datname,oid,gen_random_uuid() FROM pg_database WHERE datname=$1 ON CONFLICT(database_name) DO UPDATE SET database_oid=excluded.database_oid,generation=excluded.generation",&[&database]).await.map_err(|e|e.to_string())?;
            }
            Ok(())
        }.await;
        if let Err(error) = initialized {
            self.drop_database(database).await.map_err(|cleanup| {
                format!("Lab setup failed: {error}; cleanup failed: {cleanup}")
            })?;
            return Err(error);
        }
        Ok(())
    }
    fn password_for(&self, role: &str) -> String {
        if role == "academy_admin" {
            self.password.clone()
        } else {
            Sha256::digest(format!("sql-academy-learner:{}:{role}", self.password))
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect()
        }
    }
    pub async fn refresh_role(&self, role: &str) -> Result<()> {
        let admin = self.connect("postgres", "academy_admin").await?;
        admin.batch_execute(&format!("ALTER ROLE {role} PASSWORD '{}' NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;",self.password_for(role))).await.map_err(|e|e.to_string())?;
        Ok(())
    }
    async fn seed(&self, database: &str, role: &str, fixture: Fixture, setup: &str) -> Result<()> {
        let admin = self.connect(database, "academy_admin").await?;
        admin.batch_execute(&format!("REVOKE ALL ON DATABASE {database} FROM PUBLIC; GRANT CONNECT ON DATABASE {database} TO {role}; REVOKE CREATE ON SCHEMA public FROM PUBLIC; {setup} GRANT USAGE ON SCHEMA public TO {role}; GRANT SELECT ON ALL TABLES IN SCHEMA public TO {role};")).await.map_err(|e|e.to_string())?;
        for table in fixture.writable_tables() {
            admin
                .batch_execute(&format!(
                    "GRANT INSERT, UPDATE, DELETE ON TABLE public.{table} TO {role}"
                ))
                .await
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
    pub async fn drop_lab(&self, database: &str, role: &str) -> Result<()> {
        self.drop_database(database).await?;
        let admin = self.connect("postgres", "academy_admin").await?;
        admin
            .batch_execute(&format!("DROP ROLE IF EXISTS {role};"))
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    pub(crate) async fn drop_database(&self, database: &str) -> Result<()> {
        let admin = self.connect("postgres", "academy_admin").await?;
        // Invalidate before nontransactional deletion, including a crash before later OID reuse.
        admin
            .execute(
                "DELETE FROM academy_runtime.lab_generations WHERE database_name=$1",
                &[&database],
            )
            .await
            .map_err(|e| e.to_string())?;
        admin.execute("SELECT pg_terminate_backend(pid,2000) FROM pg_stat_activity WHERE datname=$1 AND pid<>pg_backend_pid()", &[&database]).await.map_err(|e|e.to_string())?;
        admin
            .batch_execute(&format!("DROP DATABASE IF EXISTS {database};"))
            .await
            .map_err(|e| e.to_string())
    }
    pub fn stop(&self) -> Result<()> {
        let mut lock = self.child.lock().map_err(|e| e.to_string())?;
        if let Some(mut child) = lock.take() {
            let output = Command::new(self.bin.join("pg_ctl"))
                .arg("stop")
                .arg("-D")
                .arg(self.root.join("data"))
                .args(["-m", "fast", "-t", "5"])
                .output()
                .map_err(|e| e.to_string())?;
            if !output.status.success() {
                let _ = child.kill();
                let _ = child.wait();
                return Err(String::from_utf8_lossy(&output.stderr).into());
            }
            child.wait().map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
impl Drop for Cluster {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn reset_promotion_reusing_a_historical_oid_rotates_the_generation() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(PathBuf::from)
            .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
        let cluster = Cluster::start(home.path(), &runtime).await.unwrap();
        cluster
            .create_lab("lab_01_01", "student_01_01", Fixture::Orders, 0)
            .await
            .unwrap();
        let names = vec!["lab_01_01".to_owned()];
        let first = cluster.lab_identities(&names).await.unwrap()["lab_01_01"].clone();
        let admin = cluster.connect("postgres", "academy_admin").await.unwrap();
        let original_oid: u32 = admin
            .query_one("SELECT oid FROM pg_database WHERE datname='lab_01_01'", &[])
            .await
            .unwrap()
            .get(0);
        cluster
            .replace_lab("lab_01_01", "student_01_01", Fixture::Orders, 0)
            .await
            .unwrap();
        let second = cluster.lab_identities(&names).await.unwrap()["lab_01_01"].clone();
        assert_ne!(first, second);
        let mut operation = cluster
            .reserve_reset("lab_01_01", "student_01_01")
            .await
            .unwrap();
        // Force the otherwise rare allocator collision using the real promotion path.
        admin.execute("UPDATE academy_runtime.reset_operations SET new_oid=$1 WHERE database_name='lab_01_01'", &[&original_oid]).await.unwrap();
        operation.new_oid = original_oid;
        cluster
            .create_database(
                &operation.replacement,
                "student_01_01",
                Fixture::Orders,
                &Fixture::Orders.sql(0).unwrap(),
                Some(original_oid),
            )
            .await
            .unwrap();
        cluster.promote_reset(&operation).await.unwrap();
        cluster.recover_resets().await.unwrap();
        let third = cluster.lab_identities(&names).await.unwrap()["lab_01_01"].clone();
        assert_ne!(
            third, first,
            "Actual promotion must not reuse the historical proof identity"
        );
        let learner = cluster.connect("lab_01_01", "student_01_01").await.unwrap();
        assert_eq!(cluster.connection_identity(&learner).await.unwrap(), third);
        assert_eq!(
            admin
                .query_one("SELECT oid FROM pg_database WHERE datname='lab_01_01'", &[])
                .await
                .unwrap()
                .get::<_, u32>(0),
            original_oid
        );
        drop(learner);
        drop(admin);
        cluster.stop().unwrap();
        drop(cluster);
        let again = Cluster::start(home.path(), &runtime).await.unwrap();
        assert_eq!(
            again.lab_identities(&names).await.unwrap()["lab_01_01"],
            third
        );
        again.stop().unwrap();
    }
    #[tokio::test]
    async fn resource_post_staging_rejection_preserves_old_lab_and_connection() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(PathBuf::from)
            .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
        let mut cluster = Cluster::start(home.path(), &runtime).await.unwrap();
        cluster
            .create_lab("lab_01_01", "student_01_01", Fixture::Orders, 0)
            .await
            .unwrap();
        let admin = cluster.connect("lab_01_01", "academy_admin").await.unwrap();
        admin
            .batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
            .await
            .unwrap();
        let learner = cluster.connect("lab_01_01", "student_01_01").await.unwrap();
        let setup = Fixture::Orders.sql(0).unwrap();
        let reset = cluster
            .reserve_reset("lab_01_01", "student_01_01")
            .await
            .unwrap();
        cluster
            .create_database(
                &reset.replacement,
                "student_01_01",
                Fixture::Orders,
                &setup,
                Some(reset.new_oid),
            )
            .await
            .unwrap();
        // Reject the exact post-staging boundary without filling or racing the filesystem.
        cluster.minimum_free_disk_bytes = u64::MAX;
        let error = cluster
            .promote_reset(&reset)
            .await
            .expect_err("A replacement must not promote below the disk floor");
        assert!(error.contains("Free up disk space"), "{error}");
        cluster.recover_resets().await.unwrap();
        assert_eq!(
            learner
                .query_one("SELECT total::text FROM orders WHERE id=1", &[])
                .await
                .unwrap()
                .get::<_, String>(0),
            "999.99",
            "The original connection and data must survive"
        );
        let catalog = cluster.connect("postgres", "academy_admin").await.unwrap();
        assert!(!catalog.query_one("SELECT EXISTS(SELECT FROM pg_database WHERE datname LIKE 'reset_%' OR datname LIKE 'retired_%')", &[]).await.unwrap().get::<_,bool>(0));
        assert!(catalog
            .query_one(
                "SELECT EXISTS(SELECT FROM pg_roles WHERE rolname='student_01_01')",
                &[]
            )
            .await
            .unwrap()
            .get::<_, bool>(0));
        cluster.stop().unwrap();
    }
    #[tokio::test]
    async fn resource_start_rejection_allocates_no_cluster_files() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(PathBuf::from)
            .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
        let error = Cluster::start_with_disk_floor(home.path(), &runtime, u64::MAX)
            .await
            .err()
            .unwrap();
        assert!(error.contains("Free up disk space"), "{error}");
        for name in ["data", "credentials", "postgres.log"] {
            assert!(!home.path().join(name).exists());
        }
    }
    #[tokio::test]
    async fn resource_temp_limit_is_enforced_on_restricted_connections() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/Applications/Postgres.app/Contents/Versions/18"));
        let cluster = Cluster::start(home.path(), &runtime).await.unwrap();
        cluster
            .create_lab("spill_lab", "spill_student", Fixture::ScanOrders, 0)
            .await
            .unwrap();
        let learner = cluster.connect("spill_lab", "spill_student").await.unwrap();
        let limit: String = learner
            .query_one("SHOW temp_file_limit", &[])
            .await
            .unwrap()
            .get(0);
        assert_eq!(limit, "128MB", "Learner spills must have a process limit");
        for sql in [
            "SET temp_file_limit='256MB'",
            "SELECT set_config('temp_file_limit','256MB',false)",
        ] {
            let error = learner.batch_execute(sql).await.unwrap_err();
            assert_eq!(error.code().unwrap().code(), "42501");
        }
        drop(learner);
        let admin = cluster.connect("postgres", "academy_admin").await.unwrap();
        admin.batch_execute("ALTER ROLE spill_student SET work_mem='64kB'; ALTER ROLE spill_student SET temp_file_limit='64kB'").await.unwrap();
        let learner = cluster.connect("spill_lab", "spill_student").await.unwrap();
        learner.batch_execute("SET enable_indexscan=off; SET enable_indexonlyscan=off; SET enable_bitmapscan=off; SET max_parallel_workers_per_gather=0").await.unwrap();
        let error = learner
            .query("SELECT id,total FROM orders ORDER BY total,id", &[])
            .await
            .unwrap_err();
        assert_eq!(error.code().unwrap().code(), "53400");
        assert_eq!(
            learner
                .query_one("SELECT 1", &[])
                .await
                .unwrap()
                .get::<_, i32>(0),
            1
        );
        admin
            .batch_execute("ALTER ROLE spill_student RESET ALL")
            .await
            .unwrap();
        let normal = cluster.connect("spill_lab", "spill_student").await.unwrap();
        assert_eq!(
            normal
                .query_one("SHOW temp_file_limit", &[])
                .await
                .unwrap()
                .get::<_, String>(0),
            "128MB"
        );
        cluster.stop().unwrap();
    }
    #[tokio::test]
    async fn failed_lab_creation_cleans_only_the_objects_it_created() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/Applications/Postgres.app/Contents/Versions/18"));
        let cluster = Cluster::start(home.path(), &runtime).await.unwrap();
        let admin = cluster.connect("postgres", "academy_admin").await.unwrap();
        assert!(cluster
            .create_lab("postgres", "failed_create", Fixture::Orders, 0)
            .await
            .is_err());
        assert!(
            !admin
                .query_one(
                    "SELECT EXISTS(SELECT FROM pg_roles WHERE rolname='failed_create')",
                    &[]
                )
                .await
                .unwrap()
                .get::<_, bool>(0),
            "Database-create failure must remove its new role"
        );
        assert!(
            cluster.connect("postgres", "academy_admin").await.is_ok(),
            "An existing database must be preserved"
        );
        let template = cluster.connect("template1", "academy_admin").await.unwrap();
        template
            .batch_execute("CREATE TABLE orders (id integer)")
            .await
            .unwrap();
        drop(template);
        for _ in 0..50 {
            let active: bool = admin
                .query_one(
                    "SELECT EXISTS(SELECT FROM pg_stat_activity WHERE datname='template1')",
                    &[],
                )
                .await
                .unwrap()
                .get(0);
            if !active {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert!(cluster
            .create_lab("failed_seed", "failed_seed_student", Fixture::Orders, 0)
            .await
            .is_err());
        let remaining:bool=admin.query_one("SELECT EXISTS(SELECT FROM pg_database WHERE datname='failed_seed') OR EXISTS(SELECT FROM pg_roles WHERE rolname='failed_seed_student')",&[]).await.unwrap().get(0);
        assert!(
            !remaining,
            "Authored setup failure must remove its new database and role"
        );
        cluster.stop().unwrap();
    }
    #[tokio::test]
    async fn learner_credentials_do_not_reuse_the_admin_password() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/Applications/Postgres.app/Contents/Versions/18"));
        let cluster = Cluster::start(home.path(), &runtime).await.unwrap();
        cluster
            .create_lab("credential_lab", "credential_student", Fixture::Orders, 0)
            .await
            .unwrap();
        let mut config = Config::new();
        config
            .host("127.0.0.1")
            .port(cluster.port)
            .dbname("credential_lab")
            .user("credential_student")
            .password(&cluster.password);
        assert!(
            config.connect(NoTls).await.is_err(),
            "Admin and learner passwords must differ"
        );
        cluster.stop().unwrap();
    }
}
