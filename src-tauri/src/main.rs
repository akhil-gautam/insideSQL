use academy_core::{Academy, RunRequest};
use serde_json::Value;
use std::sync::Arc;
use tauri::{Manager, State};
type AppState = Arc<Academy>;
#[tauri::command]
fn course(app: State<AppState>, handle: tauri::AppHandle) -> Result<Value, String> {
    let mut course = app.course();
    course["media_directory"] = Value::String(
        handle
            .path()
            .resource_dir()
            .map_err(|e| e.to_string())?
            .join("media")
            .to_string_lossy()
            .into_owned(),
    );
    Ok(course)
}
#[tauri::command]
fn progress(app: State<AppState>) -> Result<Value, String> {
    app.progress()
}
#[tauri::command]
async fn resources(app: State<'_, AppState>, id: String) -> Result<Value, String> {
    app.resources(&id).await
}
#[tauri::command]
fn save(app: State<AppState>, id: String, sql: String) -> Result<(), String> {
    app.save(&id, &sql)
}
#[tauri::command]
fn solution(app: State<AppState>, id: String) -> Result<Option<Value>, String> {
    app.solution(&id)
}
#[tauri::command]
async fn run(
    app: State<'_, AppState>,
    request: RunRequest,
) -> Result<academy_core::QueryResult, String> {
    app.run(request).await
}
#[tauri::command]
async fn check(
    app: State<'_, AppState>,
    request: RunRequest,
) -> Result<academy_core::CheckResult, String> {
    app.check(request).await
}
#[tauri::command]
async fn cancel(app: State<'_, AppState>, id: String) -> Result<bool, String> {
    app.cancel(&id).await
}
#[tauri::command]
async fn reset(app: State<'_, AppState>, id: String) -> Result<(), String> {
    app.reset(&id).await
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let home = app.path().app_data_dir()?;
            let postgres = app.path().resource_dir()?.join("postgres");
            let academy = tauri::async_runtime::block_on(Academy::start(&home, &postgres))
                .map_err(std::io::Error::other)?;
            app.manage(Arc::new(academy));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            course, progress, solution, run, check, cancel, reset, save, resources
        ])
        .build(tauri::generate_context!())
        .expect("Could not initialize SQL Internals Academy")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event {
                if let Some(state) = app.try_state::<AppState>() {
                    let _ = tauri::async_runtime::block_on(state.shutdown());
                }
            }
        });
}
