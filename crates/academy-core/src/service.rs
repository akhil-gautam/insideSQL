use crate::{
    equivalent,
    fixtures::{self, Fixture},
    runtime::Cluster,
    CheckResult, Progress, QueryError, QueryResult, Result, RunRequest,
};
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::HashMap,
    path::Path,
    str::FromStr,
    sync::Mutex,
    time::{Duration, Instant},
};
use tokio::sync::{watch, RwLock};
use tokio_postgres::{Client, NoTls, SimpleQueryMessage};
#[derive(Clone, Deserialize, Serialize)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub prompt: String,
    pub starter: String,
    pub ordered: bool,
    pub revision: String,
    #[serde(default)]
    pub require_analyze: bool,
    #[serde(skip_serializing)]
    pub solution_sql: String,
    #[serde(skip_serializing)]
    pub explanation: String,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct Chapter {
    pub id: String,
    #[serde(default)]
    pub fixture: Fixture,
    #[serde(default)]
    pub published: bool,
    pub title: String,
    pub subtitle: String,
    pub objectives: Vec<String>,
    pub exercises: Vec<Exercise>,
    pub transcript: Vec<String>,
}
pub struct Academy {
    cluster: Cluster,
    store: Progress,
    chapters: Vec<Chapter>,
    active: Mutex<HashMap<String, watch::Sender<bool>>>,
    lifecycle: RwLock<()>,
    lab_setup: tokio::sync::Mutex<()>,
}
struct ActiveRun<'a> {
    active: &'a Mutex<HashMap<String, watch::Sender<bool>>>,
    id: String,
    cancelled: watch::Receiver<bool>,
}
impl Drop for ActiveRun<'_> {
    fn drop(&mut self) {
        if let Ok(mut active) = self.active.lock() {
            active.remove(&self.id);
        }
    }
}
impl Academy {
    pub async fn start(home: &Path, postgres: &Path) -> Result<Self> {
        std::fs::create_dir_all(home).map_err(|e| e.to_string())?;
        let sources: &[&str] = include!(concat!(env!("OUT_DIR"), "/chapters.rs"));
        let chapters: Vec<Chapter> = sources
            .iter()
            .map(|source| serde_json::from_str(source))
            .collect::<std::result::Result<_, _>>()
            .map_err(|e| e.to_string())?;
        validate_chapters(&chapters)?;
        let cluster = Cluster::start(&home.join("runtime"), postgres).await?;
        let academy = Self {
            cluster,
            store: Progress::open(&home.join("progress.sqlite"))?,
            chapters,
            active: Mutex::new(HashMap::new()),
            lifecycle: RwLock::new(()),
            lab_setup: tokio::sync::Mutex::new(()),
        };
        for exercise in &academy.chapters[0].exercises {
            academy.ensure_lab(&exercise.id).await?;
        }
        Ok(academy)
    }
    fn exercise(&self, id: &str) -> Result<&Exercise> {
        for (index, chapter) in self.chapters.iter().enumerate() {
            if let Some(exercise) = chapter.exercises.iter().find(|e| e.id == id) {
                if !chapter.published
                    || self.chapters[..index].iter().any(|prior| {
                        !prior.published
                            || prior
                                .exercises
                                .iter()
                                .any(|e| !self.store.passed(&e.id, &e.revision).unwrap_or(false))
                    })
                {
                    return Err("Pass every required exercise in the preceding chapters to unlock this chapter".into());
                }
                return Ok(exercise);
            }
        }
        Err("Exercise is unavailable or locked".into())
    }
    fn names(&self, id: &str) -> Result<(String, String)> {
        self.exercise(id)?;
        let suffix = id.replace('-', "_");
        Ok((format!("lab_{suffix}"), format!("student_{suffix}")))
    }
    fn fixture(&self, id: &str) -> Result<Fixture> {
        self.exercise(id)?;
        Ok(self
            .chapters
            .iter()
            .find(|c| c.exercises.iter().any(|e| e.id == id))
            .unwrap()
            .fixture)
    }
    async fn ensure_lab(&self, id: &str) -> Result<()> {
        let _setup = self.lab_setup.lock().await;
        let (db, role) = self.names(id)?;
        let admin = self.cluster.connect("postgres", "academy_admin").await?;
        let exists: bool = admin
            .query_one(
                "SELECT EXISTS(SELECT FROM pg_database WHERE datname=$1)",
                &[&db],
            )
            .await
            .map_err(|e| e.to_string())?
            .get(0);
        if exists {
            self.cluster.refresh_role(&role).await
        } else {
            self.cluster
                .create_lab(&db, &role, self.fixture(id)?, 0)
                .await
        }
    }
    pub fn course(&self) -> Value {
        json!({"chapter":self.chapters[0],"chapters":self.chapters,"fixtures":fixtures::catalog(),"planned_chapters":84,"planned_exercises":504,"available_chapters":self.chapters.iter().filter(|c|c.published).count(),"server_version":"18.3"})
    }
    pub fn progress(&self) -> Result<Value> {
        let mut result = serde_json::Map::new();
        for e in self.chapters.iter().flat_map(|c| &c.exercises) {
            let sql = self.store.script(&e.id)?;
            result.insert(e.id.clone(),json!({"unlocked":self.store.unlocked(&e.id)?,"passed":self.store.passed(&e.id,&e.revision)?,"sql":sql,"analyzed":self.store.has_analysis(&e.id,&e.revision,sql.as_deref().unwrap_or(""))?}));
        }
        Ok(Value::Object(result))
    }
    pub fn solution(&self, id: &str) -> Result<Option<Value>> {
        let e = self.exercise(id)?;
        Ok(if self.store.unlocked(id)? {
            Some(json!({"sql":e.solution_sql,"explanation":e.explanation}))
        } else {
            None
        })
    }
    pub fn save(&self, id: &str, sql: &str) -> Result<()> {
        self.exercise(id)?;
        if sql.len() > 100_000 {
            return Err("SQL exceeds the 100 KB exercise limit".into());
        }
        self.store.save_script(id, sql)
    }
    pub fn running(&self, id: &str) -> bool {
        self.active
            .lock()
            .map(|v| v.contains_key(id))
            .unwrap_or(false)
    }
    fn register(&self, id: &str) -> Result<ActiveRun<'_>> {
        if id.is_empty() || id.len() > 128 {
            return Err("Invalid run identifier".into());
        }
        let mut active = self.active.lock().map_err(|e| e.to_string())?;
        if active.contains_key(id) {
            return Err("This run identifier is already active".into());
        }
        let (sender, cancelled) = watch::channel(false);
        active.insert(id.into(), sender);
        Ok(ActiveRun {
            active: &self.active,
            id: id.into(),
            cancelled,
        })
    }
    pub async fn cancel(&self, id: &str) -> Result<bool> {
        let active = self.active.lock().map_err(|e| e.to_string())?;
        Ok(active
            .get(id)
            .is_some_and(|sender| sender.send(true).is_ok()))
    }
    pub async fn run(&self, request: RunRequest) -> Result<QueryResult> {
        let mut active = self.register(&request.run_id)?;
        let _guard = self.lifecycle.read().await;
        let exercise = self.exercise(&request.exercise_id)?;
        if request.sql.len() > 100_000 {
            return Err("SQL exceeds the 100 KB exercise limit".into());
        }
        if !["run", "explain", "analyze"].contains(&request.mode.as_str()) {
            return Err("Unknown execution mode".into());
        }
        if !self
            .store
            .attempt(&exercise.id, &request.sql, &exercise.starter)?
        {
            return Err("Write a query before submitting".into());
        }
        let (db, role) = self.names(&exercise.id)?;
        self.ensure_lab(&exercise.id).await?;
        let client = self.cluster.connect(&db, &role).await?;
        let rollback =
            request.mode == "analyze" && !self.fixture(&exercise.id)?.writable_tables().is_empty();
        if rollback {
            client
                .batch_execute("BEGIN")
                .await
                .map_err(|e| e.to_string())?;
        }
        let result = execute_controlled(
            &client,
            &request.sql,
            &request.mode,
            1000,
            &mut active.cancelled,
        )
        .await;
        if rollback {
            client
                .batch_execute("ROLLBACK")
                .await
                .map_err(|e| e.to_string())?;
        }
        if request.mode == "analyze" && exercise.require_analyze {
            if let Ok(measured) = &result {
                if measured.error.is_none() && !measured.truncated && !*active.cancelled.borrow() {
                    if let Some(plan) = &measured.plan {
                        self.store.record_analysis(
                            &exercise.id,
                            &exercise.revision,
                            &request.sql,
                            plan,
                        )?;
                    }
                }
            }
        }
        result
    }
    pub async fn check(&self, request: RunRequest) -> Result<CheckResult> {
        let mut active = self.register(&request.run_id)?;
        let _guard = self.lifecycle.read().await;
        let e = self.exercise(&request.exercise_id)?;
        if request.sql.len() > 100_000 {
            return Err("SQL exceeds the 100 KB exercise limit".into());
        }
        if !self.store.attempt(&e.id, &request.sql, &e.starter)? {
            return Err("Write a query before checking".into());
        }
        if e.require_analyze && !self.store.has_analysis(&e.id, &e.revision, &request.sql)? {
            return Ok(CheckResult {
                passed: false,
                variants_passed: 0,
                variants_total: 3,
                message:
                    "Analyze this current query successfully, then Check it against fresh fixtures."
                        .into(),
            });
        }
        let fixture = self.fixture(&e.id)?;
        let mutation = !fixture.writable_tables().is_empty();
        let mut passed = 0;
        for seed in 0..3 {
            if *active.cancelled.borrow() {
                break;
            }
            let suffix = uuid::Uuid::new_v4().simple().to_string();
            let db = format!("grade_{suffix}");
            let role = format!("learner_{suffix}");
            let reference_db = format!("reference_{suffix}");
            let reference_role = format!("reference_student_{suffix}");
            self.cluster.create_lab(&db, &role, fixture, seed).await?;
            let mut reference_created = false;
            let checked: Result<bool> = async {
                if mutation {
                    self.cluster.create_lab(&reference_db, &reference_role, fixture, seed).await?;
                    reference_created = true;
                }
                let learner = self.cluster.connect(&db, &role).await?;
                let actual = execute_controlled(
                    &learner,
                    &request.sql,
                    "run",
                    10_000,
                    &mut active.cancelled,
                )
                .await?;
                if actual.error.is_some() || actual.truncated {
                    return Ok(false);
                }
                let expected_db = if mutation { &reference_db } else { &db };
                let admin = self.cluster.connect(expected_db, "academy_admin").await?;
                let expected = execute(&admin, &e.solution_sql, "run", 10_000).await?;
                if expected.error.is_some() || expected.truncated {
                    return Err("Reference assessment failed; progress was not changed".into());
                }
                if actual.columns.len() != expected.columns.len()
                    || !equivalent(&normalized(&actual), &normalized(&expected), e.ordered) { return Ok(false); }
                if mutation {
                    let actual_admin = self.cluster.connect(&db, "academy_admin").await?;
                    // shortcut: full state is capped at 10,000 rows; stream comparisons when larger write fixtures arrive.
                    for table in fixture.writable_tables() {
                        let sql = format!("SELECT * FROM public.{table}");
                        let state = execute(&actual_admin, &sql, "run", 10_000).await?;
                        let reference = execute(&admin, &sql, "run", 10_000).await?;
                        if reference.error.is_some() || reference.truncated {
                            return Err("Reference mutation state could not be verified; progress was not changed".into());
                        }
                        if state.error.is_some() || state.truncated
                            || !equivalent(&normalized(&state), &normalized(&reference), false) { return Ok(false); }
                    }
                }
                Ok(true)
            }
            .await;
            let reference_cleanup = if reference_created {
                self.cluster.drop_lab(&reference_db, &reference_role).await
            } else {
                Ok(())
            };
            let learner_cleanup = self.cluster.drop_lab(&db, &role).await;
            reference_cleanup?;
            learner_cleanup?;
            if checked? && !*active.cancelled.borrow() {
                passed += 1;
            } else {
                break;
            }
        }
        if passed == 3 {
            self.store.pass(&e.id, &e.revision)?;
        }
        Ok(CheckResult {
            passed: passed == 3,
            variants_passed: passed,
            variants_total: 3,
            message: if passed == 3 {
                "Correct across all three fixture variants.".into()
            } else {
                format!("Not yet correct. {passed} of 3 fixture variants passed. Check filters, duplicates, nulls, ordering{}.", if mutation { " and resulting table state" } else { "" })
            },
        })
    }
    pub async fn reset(&self, id: &str) -> Result<()> {
        let _guard = self.lifecycle.write().await;
        let (db, role) = self.names(id)?;
        self.cluster.drop_lab(&db, &role).await?;
        self.cluster
            .create_lab(&db, &role, self.fixture(id)?, 0)
            .await?;
        self.store.clear_analysis(id)
    }
    pub async fn shutdown(&self) -> Result<()> {
        let _guard = self.lifecycle.write().await;
        self.cluster.stop()
    }
}
async fn execute_controlled(
    client: &Client,
    sql: &str,
    mode: &str,
    limit: usize,
    cancelled: &mut watch::Receiver<bool>,
) -> Result<QueryResult> {
    let message = tokio::select! {
        biased;
        _ = cancelled.wait_for(|value| *value) => "Query cancelled",
        result = tokio::time::timeout(Duration::from_secs(10), execute(client,sql,mode,limit)) => {
            match result { Ok(value) => return value, Err(_) => "Query exceeded the exercise time limit" }
        }
    };
    let _ = client.cancel_token().cancel_query(NoTls).await;
    Ok(QueryResult {
        error: Some(QueryError {
            message: message.into(),
            code: Some("57014".into()),
            position: None,
        }),
        ..Default::default()
    })
}
fn validate_chapters(chapters: &[Chapter]) -> Result<()> {
    if chapters.is_empty() || !chapters[0].published {
        return Err("The introductory chapter must be published".into());
    }
    for (index, chapter) in chapters.iter().enumerate() {
        let id = format!("{:02}", index + 1);
        if chapter.id != id || chapter.exercises.len() != 6 {
            return Err(format!(
                "Chapter {id} must have valid identifiers and exactly six exercises"
            ));
        }
        for (position, exercise) in chapter.exercises.iter().enumerate() {
            if exercise.id != format!("{id}-{:02}", position + 1)
                || exercise.revision.is_empty()
                || exercise.solution_sql.trim().is_empty()
            {
                return Err(format!("Invalid exercise definition in chapter {id}"));
            }
        }
    }
    Ok(())
}
fn normalized(result: &QueryResult) -> Vec<Vec<Option<String>>> {
    result
        .rows
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, value)| {
                    value.as_ref().map(|text| {
                        let oid = result.column_types.get(i).copied().unwrap_or(0);
                        if [20, 21, 23, 700, 701, 1700].contains(&oid) {
                            bigdecimal::BigDecimal::from_str(text)
                                .map(|n| format!("number:{}", n.normalized()))
                                .unwrap_or_else(|_| format!("number:{text}"))
                        } else {
                            format!("{oid}:{text}")
                        }
                    })
                })
                .collect()
        })
        .collect()
}
async fn execute(client: &Client, sql: &str, mode: &str, limit: usize) -> Result<QueryResult> {
    let start = Instant::now();
    let mut result = QueryResult::default();
    let executed = async {
        let statement = client.prepare(sql).await?;
        let sql = match mode {
            "explain" => format!("EXPLAIN (FORMAT JSON, VERBOSE, SETTINGS) {sql}"),
            "analyze" => {
                format!("EXPLAIN (ANALYZE, BUFFERS, VERBOSE, SETTINGS, FORMAT JSON) {sql}")
            }
            _ => sql.to_owned(),
        };
        if mode == "run" {
            result.columns = statement
                .columns()
                .iter()
                .map(|c| c.name().into())
                .collect();
            result.column_types = statement
                .columns()
                .iter()
                .map(|c| c.type_().oid())
                .collect();
        }
        let stream = client.simple_query_raw(&sql).await?;
        tokio::pin!(stream);
        let mut bytes = 0;
        while let Some(message) = stream.try_next().await? {
            if let SimpleQueryMessage::Row(row) = message {
                if result.rows.len() >= limit {
                    result.truncated = true;
                    let _ = client.cancel_token().cancel_query(NoTls).await;
                    break;
                }
                let values: Vec<Option<String>> = (0..row.len())
                    .map(|i| row.get(i).map(str::to_owned))
                    .collect();
                bytes += values.iter().flatten().map(String::len).sum::<usize>();
                if bytes > 8_000_000 {
                    result.truncated = true;
                    let _ = client.cancel_token().cancel_query(NoTls).await;
                    break;
                }
                result.rows.push(values);
            }
        }
        Ok::<(), tokio_postgres::Error>(())
    }
    .await;
    if let Err(error) = executed {
        let db = error.as_db_error();
        result.error = Some(QueryError {
            message: db
                .map(|e| e.message().into())
                .unwrap_or_else(|| error.to_string()),
            code: db.map(|e| e.code().code().into()),
            position: db.and_then(|e| match e.position() {
                Some(tokio_postgres::error::ErrorPosition::Original(n)) => Some(*n),
                _ => None,
            }),
        });
    }
    if mode != "run" && result.error.is_none() {
        result.plan = result
            .rows
            .first()
            .and_then(|r| r.first())
            .and_then(|s| s.as_deref())
            .and_then(|s| serde_json::from_str(s).ok());
        result.rows.clear();
    }
    result.elapsed_ms = start.elapsed().as_millis();
    Ok(result)
}

#[cfg(test)]
#[path = "grouping_tests.rs"]
mod grouping_tests;

#[cfg(test)]
#[path = "mutation_tests.rs"]
mod mutation_tests;

#[cfg(test)]
#[path = "plan_tests.rs"]
mod plan_tests;

#[cfg(test)]
#[path = "tree_tests.rs"]
mod tree_tests;

#[cfg(test)]
#[path = "scan_tests.rs"]
mod scan_tests;

#[cfg(test)]
#[path = "join_tests.rs"]
mod join_tests;

#[cfg(test)]
#[path = "sort_aggregate_tests.rs"]
mod sort_aggregate_tests;

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn joins_chapter_uses_commerce_for_visible_grading_and_reset_labs() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
        let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
        assert!(
            academy.chapters.get(2).is_some(),
            "The joins chapter must be authored"
        );
        assert!(academy
            .run(RunRequest {
                exercise_id: "03-01".into(),
                sql: "SELECT 1".into(),
                mode: "run".into(),
                run_id: uuid::Uuid::new_v4().to_string()
            })
            .await
            .is_err());
        for chapter in &mut academy.chapters[..2] {
            chapter.published = true;
            for e in &chapter.exercises {
                academy.store.attempt(&e.id, "SELECT 1", "").unwrap();
                academy.store.pass(&e.id, &e.revision).unwrap();
            }
        }
        assert!(
            academy.solution("03-01").is_err(),
            "An unpublished chapter stays locked after all prior passes"
        );
        academy.chapters[2].published = true;
        let request = |id: &str, sql: &str| RunRequest {
            exercise_id: id.into(),
            sql: sql.into(),
            mode: "run".into(),
            run_id: uuid::Uuid::new_v4().to_string(),
        };
        academy.chapters[1].published = false;
        assert!(
            academy.run(request("03-01", "SELECT 1")).await.is_err(),
            "A withdrawn preceding chapter must lock downstream access even when old passes exist"
        );
        academy.chapters[1].published = true;
        let sql = "SELECT (SELECT COUNT(*) FROM customers), (SELECT COUNT(*) FROM orders), (SELECT COUNT(*) FROM order_items), (SELECT COUNT(*) FROM payments)";
        let counts = academy.run(request("03-01", sql)).await.unwrap();
        assert!(
            counts.error.is_none(),
            "Commerce tables must exist in the visible lab"
        );
        assert_eq!(counts.rows[0][0].as_deref(), Some("52"));
        assert_eq!(counts.rows[0][1].as_deref(), Some("310"));
        let before = counts.rows;
        academy.reset("03-01").await.unwrap();
        assert_eq!(
            academy.run(request("03-01", sql)).await.unwrap().rows,
            before
        );
        let references = academy.chapters[2].exercises.clone();
        let known: serde_json::Value =
            serde_json::from_str(include_str!("../tests/fixtures/chapter03-baseline.json"))
                .unwrap();
        for e in references {
            let mut actual = academy.run(request(&e.id, &e.solution_sql)).await.unwrap();
            assert!(actual.error.is_none());
            let upper = if e.id == "03-02" || e.id == "03-03" {
                200
            } else {
                1000
            };
            actual
                .rows
                .retain(|row| row[0].as_deref().unwrap().parse::<i32>().unwrap() < upper);
            let expected = QueryResult {
                column_types: actual.column_types.clone(),
                rows: serde_json::from_value(known[&e.id].clone()).unwrap(),
                ..Default::default()
            };
            assert_eq!(
                normalized(&actual),
                normalized(&expected),
                "Independent baseline oracle: {}",
                e.id
            );
            let result = academy
                .check(request(&e.id, &e.solution_sql))
                .await
                .unwrap();
            assert!(result.passed, "{}: {}", e.id, result.message);
        }
        let wrong = [
            ("03-02","SELECT c.id, o.id FROM customers c LEFT JOIN orders o ON o.customer_id=c.id WHERE o.status='paid' ORDER BY c.id,o.id"),
            ("03-05","SELECT o.id FROM orders o WHERE o.id NOT IN (SELECT p.order_id FROM payments p WHERE p.status='captured') ORDER BY o.id"),
            ("03-06","SELECT o.id, COALESCE(SUM(i.quantity*i.unit_price),0), COALESCE(SUM(p.amount),0) FROM orders o LEFT JOIN order_items i ON i.order_id=o.id LEFT JOIN payments p ON p.order_id=o.id AND p.status='captured' GROUP BY o.id ORDER BY o.id"),
        ];
        for (id, sql) in wrong {
            assert!(
                !academy.check(request(id, sql)).await.unwrap().passed,
                "Common wrong query {id} must fail"
            );
        }
        assert!(academy
            .run(request(
                "03-01",
                "INSERT INTO customers VALUES (999,'Injected')"
            ))
            .await
            .unwrap()
            .error
            .is_some());
        assert!(academy
            .run(request(
                "03-01",
                "CREATE TABLE arbitrary_table (id integer)"
            ))
            .await
            .unwrap()
            .error
            .is_some());
        academy.shutdown().await.unwrap();
    }
    #[tokio::test]
    async fn cancel_is_remembered_while_a_lab_is_still_being_prepared() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
        let academy = std::sync::Arc::new(Academy::start(home.path(), &runtime).await.unwrap());
        let setup = academy.lab_setup.lock().await;
        let runner = academy.clone();
        let task = tokio::spawn(async move {
            runner
                .run(RunRequest {
                    exercise_id: "01-01".into(),
                    sql: "SELECT pg_sleep(20)".into(),
                    mode: "run".into(),
                    run_id: "preparing".into(),
                })
                .await
        });
        for _ in 0..100 {
            if academy.running("preparing") {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        let cancelled = academy.cancel("preparing").await.unwrap();
        drop(setup);
        let result = task.await.unwrap().unwrap();
        academy.shutdown().await.unwrap();
        assert!(
            cancelled,
            "cancel must register before database setup finishes"
        );
        assert_eq!(result.error.unwrap().code.as_deref(), Some("57014"));
        assert!(
            result.elapsed_ms < 1000,
            "the cancelled query must never start sleeping"
        );
    }
    #[tokio::test]
    async fn chapter_access_requires_every_prior_pass_and_a_published_lesson() {
        let home = tempfile::tempdir().unwrap();
        let runtime = std::env::var_os("ACADEMY_POSTGRES")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into());
        let mut academy = Academy::start(home.path(), &runtime).await.unwrap();
        assert!(academy.solution("02-01").is_err());
        academy.chapters[1].published = true;
        let request = || RunRequest {
            exercise_id: "02-01".into(),
            sql: "SELECT id FROM orders".into(),
            mode: "run".into(),
            run_id: uuid::Uuid::new_v4().to_string(),
        };
        assert!(academy.run(request()).await.is_err());
        for exercise in &academy.chapters[0].exercises[..5] {
            academy.store.attempt(&exercise.id, "SELECT 1", "").unwrap();
            academy
                .store
                .pass(&exercise.id, &exercise.revision)
                .unwrap();
        }
        assert!(
            academy.run(request()).await.is_err(),
            "five passes cannot unlock a six-question chapter"
        );
        let last = &academy.chapters[0].exercises[5];
        academy.store.attempt(&last.id, "SELECT 1", "").unwrap();
        academy.store.pass(&last.id, &last.revision).unwrap();
        let result = academy.run(request()).await.unwrap();
        assert!(result.error.is_none());
        assert_eq!(result.rows.len(), 10);
        let exercises = academy.chapters[1].exercises.clone();
        for exercise in exercises {
            let result = academy
                .check(RunRequest {
                    exercise_id: exercise.id.clone(),
                    sql: exercise.solution_sql,
                    mode: "run".into(),
                    run_id: uuid::Uuid::new_v4().to_string(),
                })
                .await
                .unwrap();
            assert!(result.passed, "{}: {}", exercise.id, result.message);
        }
        academy.store.pass(&last.id.clone(), "obsolete").unwrap();
        assert!(
            academy.run(request()).await.is_err(),
            "a changed required revision must lock downstream labs"
        );
        academy.shutdown().await.unwrap();
    }

    #[test]
    fn catalog_rejects_identifiers_that_could_escape_generated_lab_names() {
        let mut chapters: Vec<Chapter> =
            vec![serde_json::from_str(include_str!("../../../course/chapter01.json")).unwrap()];
        chapters[0].exercises[0].id = "01-01; DROP DATABASE postgres".into();
        assert!(validate_chapters(&chapters).is_err());
    }
    #[test]
    fn authored_chapters_require_exactly_six_exercises() {
        let mut chapter: Chapter =
            serde_json::from_str(include_str!("../../../course/chapter01.json")).unwrap();
        chapter.exercises.pop();
        assert!(
            validate_chapters(&[chapter]).is_err(),
            "Every chapter requires six questions for the 504-exercise contract"
        );
    }
}
