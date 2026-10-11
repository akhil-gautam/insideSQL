use super::*;

pub(super) struct ResetOperation {
    database: String,
    old_oid: u32,
    pub(super) new_oid: u32,
    owner_oid: u32,
    pub(super) replacement: String,
    retired: String,
    committed: bool,
}

impl ResetOperation {
    fn validate_names(&self) -> Result<()> {
        let db = self.database.as_bytes();
        let suffix = self.replacement.strip_prefix("reset_").unwrap_or("");
        if db.len() != 9
            || !db.starts_with(b"lab_")
            || db[6] != b'_'
            || !db[4..6].iter().chain(&db[7..9]).all(u8::is_ascii_digit)
            || suffix.len() != 32
            || !suffix
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            || self.retired != format!("retired_{suffix}")
            || self.old_oid < 16384
            || self.new_oid < 16384
            || self.old_oid == self.new_oid
        {
            return Err("Reset recovery record is invalid; database copies were kept".into());
        }
        Ok(())
    }
}

impl Cluster {
    pub(super) async fn initialize_reset_recovery(&self) -> Result<()> {
        let admin = self.connect("postgres", "academy_admin").await?;
        admin
            .batch_execute(
                "CREATE SCHEMA IF NOT EXISTS academy_runtime AUTHORIZATION academy_admin;
REVOKE ALL ON SCHEMA academy_runtime FROM PUBLIC;
CREATE TABLE IF NOT EXISTS academy_runtime.reset_operations (
 database_name text PRIMARY KEY, old_oid oid NOT NULL, new_oid oid NOT NULL UNIQUE,
 owner_oid oid NOT NULL, replacement_name text NOT NULL UNIQUE,
 retired_name text NOT NULL UNIQUE, committed boolean NOT NULL DEFAULT false);
REVOKE ALL ON TABLE academy_runtime.reset_operations FROM PUBLIC;",
            )
            .await
            .map_err(|e| e.to_string())?;
        let valid: bool = admin.query_one("SELECT n.nspowner=r.oid AND c.relowner=r.oid AND c.relkind='r' FROM pg_namespace n JOIN pg_class c ON c.relnamespace=n.oid JOIN pg_roles r ON r.rolname='academy_admin' WHERE n.nspname='academy_runtime' AND c.relname='reset_operations'",&[]).await.map_err(|e|e.to_string())?.get(0);
        if !valid {
            return Err("Reset journal ownership is invalid; database copies were kept".into());
        }
        self.recover_resets().await?;
        Ok(())
    }

    pub(super) async fn reserve_reset(&self, database: &str, role: &str) -> Result<ResetOperation> {
        self.require_disk_space()?;
        let admin = self.connect("postgres", "academy_admin").await?;
        let old = admin.query_one("SELECT d.oid,d.datdba,r.oid FROM pg_database d CROSS JOIN pg_roles r WHERE d.datname=$1 AND r.rolname='academy_admin'",&[&database]).await.map_err(|e|e.to_string())?;
        let owner_oid: u32 = old.get(1);
        if owner_oid != old.get::<_, u32>(2)
            || role != format!("student_{}", database.strip_prefix("lab_").unwrap_or(""))
        {
            return Err("Reset target ownership is invalid; existing lab was kept".into());
        }
        for _ in 0..8 {
            let nonce = uuid::Uuid::new_v4();
            let new_oid =
                u32::from_be_bytes(nonce.as_bytes()[..4].try_into().unwrap()) | 0x4000_0000;
            let suffix = nonce.simple().to_string();
            let operation = ResetOperation {
                database: database.into(),
                old_oid: old.get(0),
                new_oid,
                owner_oid,
                replacement: format!("reset_{suffix}"),
                retired: format!("retired_{suffix}"),
                committed: false,
            };
            operation.validate_names()?;
            let names = [&operation.replacement, &operation.retired];
            let free: bool = admin.query_one("SELECT NOT EXISTS(SELECT FROM pg_database WHERE datname=ANY($1) OR oid=$2) AND NOT EXISTS(SELECT FROM academy_runtime.reset_operations WHERE new_oid=$2)",&[&names.as_slice(),&new_oid]).await.map_err(|e|e.to_string())?.get(0);
            if !free {
                continue;
            }
            admin.execute("INSERT INTO academy_runtime.reset_operations(database_name,old_oid,new_oid,owner_oid,replacement_name,retired_name) VALUES ($1,$2,$3,$4,$5,$6)",&[&operation.database,&operation.old_oid,&operation.new_oid,&operation.owner_oid,&operation.replacement,&operation.retired]).await.map_err(|e|e.to_string())?;
            return Ok(operation);
        }
        Err("Could not reserve a replacement database identity; existing lab was kept".into())
    }

    pub(super) async fn promote_reset(&self, operation: &ResetOperation) -> Result<()> {
        self.require_disk_space()?;
        operation.validate_names()?;
        let admin = self.connect("postgres", "academy_admin").await?;
        self.validate_reset_catalog(&admin, operation).await?;
        let names = [&operation.database, &operation.replacement];
        admin.execute("SELECT pg_terminate_backend(pid,2000) FROM pg_stat_activity WHERE datname=ANY($1) AND pid<>pg_backend_pid()",&[&names.as_slice()]).await.map_err(|e|e.to_string())?;
        let renamed: Result<()> = async {
            admin.batch_execute(&format!("BEGIN; ALTER DATABASE {} RENAME TO {}; ALTER DATABASE {} RENAME TO {};",operation.database,operation.retired,operation.replacement,operation.database)).await.map_err(|e|e.to_string())?;
            let updated=admin.execute("UPDATE academy_runtime.reset_operations SET committed=true WHERE database_name=$1 AND old_oid=$2 AND new_oid=$3 AND NOT committed",&[&operation.database,&operation.old_oid,&operation.new_oid]).await.map_err(|e|e.to_string())?;
            if updated != 1 { return Err("Reset journal changed during promotion; database copies were kept".into()); }
            // Catalog names and durable outcome commit together, including a lost response.
            admin.batch_execute("COMMIT").await.map_err(|e|e.to_string())
        }.await;
        if renamed.is_err() {
            let _ = admin.batch_execute("ROLLBACK").await;
        }
        renamed
    }

    async fn validate_reset_catalog(
        &self,
        admin: &Client,
        operation: &ResetOperation,
    ) -> Result<()> {
        operation.validate_names()?;
        let names = [
            &operation.database,
            &operation.replacement,
            &operation.retired,
        ];
        let oids = [operation.old_oid, operation.new_oid];
        let rows = admin.query("SELECT d.datname,d.oid,d.datdba,r.oid FROM pg_database d CROSS JOIN pg_roles r WHERE r.rolname='academy_admin' AND (d.datname=ANY($1) OR d.oid=ANY($2))",&[&names.as_slice(),&oids.as_slice()]).await.map_err(|e|e.to_string())?;
        let mut catalog = HashMap::new();
        for row in rows {
            let name: String = row.get(0);
            if !names.contains(&&name)
                || row.get::<_, u32>(2) != operation.owner_oid
                || row.get::<_, u32>(3) != operation.owner_oid
            {
                return Err(
                    "Reset recovery catalog identity is ambiguous; database copies were kept"
                        .into(),
                );
            }
            catalog.insert(name, row.get::<_, u32>(1));
        }
        let primary = if operation.committed {
            operation.new_oid
        } else {
            operation.old_oid
        };
        let temporary = if operation.committed {
            (
                &operation.retired,
                operation.old_oid,
                &operation.replacement,
            )
        } else {
            (
                &operation.replacement,
                operation.new_oid,
                &operation.retired,
            )
        };
        if catalog.get(&operation.database) != Some(&primary)
            || catalog
                .get(temporary.0)
                .is_some_and(|oid| *oid != temporary.1)
            || catalog.contains_key(temporary.2)
        {
            return Err(
                "Reset recovery catalog identity is ambiguous; database copies were kept".into(),
            );
        }
        Ok(())
    }

    pub(super) async fn recover_resets(&self) -> Result<HashMap<String, bool>> {
        let admin = self.connect("postgres", "academy_admin").await?;
        let rows = admin.query("SELECT database_name,old_oid,new_oid,owner_oid,replacement_name,retired_name,committed FROM academy_runtime.reset_operations ORDER BY database_name",&[]).await.map_err(|e|e.to_string())?;
        let mut recovered = HashMap::new();
        for row in rows {
            let read = || -> std::result::Result<ResetOperation, tokio_postgres::Error> {
                Ok(ResetOperation {
                    database: row.try_get(0)?,
                    old_oid: row.try_get(1)?,
                    new_oid: row.try_get(2)?,
                    owner_oid: row.try_get(3)?,
                    replacement: row.try_get(4)?,
                    retired: row.try_get(5)?,
                    committed: row.try_get(6)?,
                })
            };
            let operation = read().map_err(|e| {
                format!("Invalid Reset recovery record: {e}; database copies were kept")
            })?;
            self.validate_reset_catalog(&admin, &operation).await?;
            let obsolete = if operation.committed {
                &operation.retired
            } else {
                &operation.replacement
            };
            self.drop_database(obsolete).await?;
            let deleted=admin.execute("DELETE FROM academy_runtime.reset_operations WHERE database_name=$1 AND old_oid=$2 AND new_oid=$3 AND committed=$4",&[&operation.database,&operation.old_oid,&operation.new_oid,&operation.committed]).await.map_err(|e|e.to_string())?;
            if deleted != 1 {
                return Err("Reset journal changed during recovery; active lab was kept".into());
            }
            recovered.insert(operation.database, operation.committed);
        }
        Ok(recovered)
    }

    pub(crate) async fn replace_lab(
        &self,
        database: &str,
        role: &str,
        fixture: Fixture,
        seed: u32,
    ) -> Result<()> {
        let setup = fixture.sql(seed)?;
        self.recover_resets().await?;
        let operation = self.reserve_reset(database, role).await?;
        let attempted: Result<()> = async {
            self.create_database(
                &operation.replacement,
                role,
                fixture,
                &setup,
                Some(operation.new_oid),
            )
            .await?;
            self.promote_reset(&operation).await
        }
        .await;
        let recovered = self.recover_resets().await.map_err(|recovery| {
            format!(
                "{}Reset recovery is pending: {recovery}",
                attempted
                    .as_ref()
                    .err()
                    .map(|e| format!("{e}; "))
                    .unwrap_or_default()
            )
        })?;
        if recovered.get(database) == Some(&true) {
            return Ok(());
        }
        attempted?;
        Err("Reset outcome could not be confirmed; database copies were kept".into())
    }
}
