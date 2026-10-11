use crate::{fixtures::Fixture, Result};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Mutex,
    time::Duration,
};
use tokio_postgres::{Client, Config, NoTls};
pub struct Cluster {
    pub root: PathBuf,
    pub port: u16,
    password: String,
    bin: PathBuf,
    child: Mutex<Option<Child>>,
}
impl Cluster {
    pub async fn start(home: &Path, postgres: &Path) -> Result<Self> {
        fs::create_dir_all(home).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(home, fs::Permissions::from_mode(0o700))
                .map_err(|e| e.to_string())?;
        }
        let root = home.canonicalize().map_err(|e| e.to_string())?;
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
        let cluster = Self {
            root,
            port,
            password,
            bin,
            child: Mutex::new(Some(child)),
        };
        for _ in 0..200 {
            if let Ok(admin) = cluster.connect("postgres", "academy_admin").await {
                admin.batch_execute("REVOKE CONNECT ON DATABASE postgres FROM PUBLIC; REVOKE CONNECT ON DATABASE template1 FROM PUBLIC; REVOKE CONNECT ON DATABASE template0 FROM PUBLIC;").await.map_err(|e|e.to_string())?;
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
    pub async fn create_lab(
        &self,
        database: &str,
        role: &str,
        fixture: Fixture,
        seed: u32,
    ) -> Result<()> {
        let setup = fixture.sql(seed)?;
        let admin = self.connect("postgres", "academy_admin").await?;
        admin.batch_execute(&format!("CREATE ROLE {role} LOGIN PASSWORD '{}' NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;",self.password_for(role))).await.map_err(|e|e.to_string())?;
        if let Err(error) = admin
            .batch_execute(&format!("CREATE DATABASE {database};"))
            .await
        {
            admin
                .batch_execute(&format!("DROP ROLE {role};"))
                .await
                .map_err(|cleanup| {
                    format!("Lab creation failed: {error}; role cleanup failed: {cleanup}")
                })?;
            return Err(error.to_string());
        }
        if let Err(error) = self.seed(database, role, &setup).await {
            self.drop_lab(database, role).await.map_err(|cleanup| {
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
    async fn seed(&self, database: &str, role: &str, setup: &str) -> Result<()> {
        let admin = self.connect(database, "academy_admin").await?;
        admin.batch_execute(&format!("REVOKE ALL ON DATABASE {database} FROM PUBLIC; GRANT CONNECT ON DATABASE {database} TO {role}; REVOKE CREATE ON SCHEMA public FROM PUBLIC; {setup} GRANT USAGE ON SCHEMA public TO {role}; GRANT SELECT ON ALL TABLES IN SCHEMA public TO {role};")).await.map_err(|e|e.to_string())?;
        Ok(())
    }
    pub async fn drop_lab(&self, database: &str, role: &str) -> Result<()> {
        let admin = self.connect("postgres", "academy_admin").await?;
        admin.execute("SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname=$1 AND pid<>pg_backend_pid()",&[&database]).await.map_err(|e|e.to_string())?;
        admin
            .batch_execute(&format!("DROP DATABASE IF EXISTS {database};"))
            .await
            .map_err(|e| e.to_string())?;
        admin
            .batch_execute(&format!("DROP ROLE IF EXISTS {role};"))
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
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
