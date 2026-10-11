use crate::Result;
use rusqlite::{params, Connection, OptionalExtension};
use std::{path::Path, sync::Mutex};

pub struct Progress {
    connection: Mutex<Connection>,
}
impl Progress {
    pub fn open(path: &Path) -> Result<Self> {
        let connection = Connection::open(path).map_err(|e| e.to_string())?;
        connection.execute_batch("PRAGMA journal_mode=WAL; CREATE TABLE IF NOT EXISTS progress (exercise TEXT PRIMARY KEY, sql TEXT NOT NULL DEFAULT '', unlocked INTEGER NOT NULL DEFAULT 0, passed_revision TEXT); CREATE TABLE IF NOT EXISTS analysis (exercise TEXT PRIMARY KEY, revision TEXT NOT NULL, sql TEXT NOT NULL, plan TEXT NOT NULL);").map_err(|e|e.to_string())?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }
    pub fn attempt(&self, exercise: &str, sql: &str, starter: &str) -> Result<bool> {
        if !has_content(sql) || sql.trim() == starter.trim() {
            return Ok(false);
        }
        self.connection.lock().map_err(|e|e.to_string())?.execute("INSERT INTO progress(exercise,sql,unlocked) VALUES (?1,?2,1) ON CONFLICT(exercise) DO UPDATE SET sql=excluded.sql,unlocked=1",params![exercise,sql]).map_err(|e|e.to_string())?;
        Ok(true)
    }
    pub fn save_script(&self, exercise: &str, sql: &str) -> Result<()> {
        self.connection.lock().map_err(|e|e.to_string())?.execute(
            "INSERT INTO progress(exercise,sql) VALUES (?1,?2) ON CONFLICT(exercise) DO UPDATE SET sql=excluded.sql",
            params![exercise,sql]).map_err(|e|e.to_string())?;
        Ok(())
    }
    pub fn unlocked(&self, exercise: &str) -> Result<bool> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .query_row(
                "SELECT unlocked FROM progress WHERE exercise=?1",
                [exercise],
                |r| r.get(0),
            )
            .optional()
            .map(|v| v.unwrap_or(false))
            .map_err(|e| e.to_string())
    }
    pub fn pass(&self, exercise: &str, revision: &str) -> Result<()> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .execute(
                "UPDATE progress SET passed_revision=?2 WHERE exercise=?1",
                params![exercise, revision],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn passed(&self, exercise: &str, revision: &str) -> Result<bool> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .query_row(
                "SELECT passed_revision=?2 FROM progress WHERE exercise=?1",
                params![exercise, revision],
                |r| r.get::<_, Option<bool>>(0),
            )
            .optional()
            .map(|v| v.flatten().unwrap_or(false))
            .map_err(|e| e.to_string())
    }
    pub fn script(&self, exercise: &str) -> Result<Option<String>> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .query_row(
                "SELECT sql FROM progress WHERE exercise=?1",
                [exercise],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| e.to_string())
    }
    pub fn record_analysis(
        &self,
        exercise: &str,
        revision: &str,
        sql: &str,
        plan: &serde_json::Value,
    ) -> Result<()> {
        let node = &plan[0]["Plan"];
        if !node["Actual Rows"].is_number() || !node["Actual Loops"].is_number() {
            return Err("Only a measured PostgreSQL plan can be recorded".into());
        }
        self.connection.lock().map_err(|e|e.to_string())?.execute("INSERT INTO analysis(exercise,revision,sql,plan) VALUES (?1,?2,?3,?4) ON CONFLICT(exercise) DO UPDATE SET revision=excluded.revision,sql=excluded.sql,plan=excluded.plan",params![exercise,revision,sql,plan.to_string()]).map_err(|e|e.to_string())?;
        Ok(())
    }
    pub fn has_analysis(&self, exercise: &str, revision: &str, sql: &str) -> Result<bool> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM analysis WHERE exercise=?1 AND revision=?2 AND sql=?3)",
                params![exercise, revision, sql],
                |r| r.get(0),
            )
            .map_err(|e| e.to_string())
    }
    pub fn clear_analysis(&self, exercise: &str) -> Result<()> {
        self.connection
            .lock()
            .map_err(|e| e.to_string())?
            .execute("DELETE FROM analysis WHERE exercise=?1", [exercise])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
fn has_content(sql: &str) -> bool {
    let b = sql.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if b[i..].starts_with(b"--") {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b[i..].starts_with(b"/*") {
            i += 2;
            let mut depth = 1;
            while i < b.len() && depth > 0 {
                if b[i..].starts_with(b"/*") {
                    depth += 1;
                    i += 2;
                } else if b[i..].starts_with(b"*/") {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        return true;
    }
    false
}
