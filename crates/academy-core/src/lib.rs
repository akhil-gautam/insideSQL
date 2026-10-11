mod fixtures;
mod progress;
mod resources;
mod runtime;
mod service;
pub use progress::Progress;
pub use service::Academy;
pub type Result<T> = std::result::Result<T, String>;
pub fn equivalent(
    actual: &[Vec<Option<String>>],
    expected: &[Vec<Option<String>>],
    ordered: bool,
) -> bool {
    if ordered {
        return actual == expected;
    }
    let mut actual = actual.to_vec();
    let mut expected = expected.to_vec();
    actual.sort();
    expected.sort();
    actual == expected
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct RunRequest {
    pub exercise_id: String,
    pub sql: String,
    pub mode: String,
    pub run_id: String,
}
#[derive(Debug, Default, serde::Serialize)]
pub struct QueryError {
    pub message: String,
    pub code: Option<String>,
    pub position: Option<u32>,
}
#[derive(Debug, Default, serde::Serialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub column_types: Vec<u32>,
    pub rows: Vec<Vec<Option<String>>>,
    pub plan: Option<serde_json::Value>,
    pub error: Option<QueryError>,
    pub truncated: bool,
    pub elapsed_ms: u128,
}
#[derive(Debug, Default, serde::Serialize)]
pub struct CheckResult {
    pub passed: bool,
    pub message: String,
    pub variants_passed: usize,
    pub variants_total: usize,
}
