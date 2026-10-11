use academy_core::{Academy, RunRequest};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use std::{path::PathBuf, sync::Arc};

async fn api(
    State(app): State<Arc<Academy>>,
    Path(command): Path<String>,
    Json(input): Json<Value>,
) -> std::result::Result<Json<Value>, (StatusCode, Json<Value>)> {
    let result: std::result::Result<Value, String> = async {
        match command.as_str() {
            "course" => Ok(app.course()),
            "progress" => app.progress(),
            "resources" => {
                app.resources(input["id"].as_str().ok_or("Missing exercise id")?)
                    .await
            }
            "save" => {
                app.save(
                    input["id"].as_str().ok_or("Missing exercise id")?,
                    input["sql"].as_str().ok_or("Missing SQL draft")?,
                )?;
                Ok(json!(true))
            }
            "solution" => Ok(json!(
                app.solution(input["id"].as_str().ok_or("Missing exercise id")?)?
            )),
            "run" => Ok(json!(
                app.run(serde_json::from_value::<RunRequest>(input).map_err(|e| e.to_string())?)
                    .await?
            )),
            "check" => Ok(json!(
                app.check(serde_json::from_value::<RunRequest>(input).map_err(|e| e.to_string())?)
                    .await?
            )),
            "cancel" => Ok(json!(
                app.cancel(input["id"].as_str().ok_or("Missing run id")?)
                    .await?
            )),
            "reset" => {
                app.reset(input["id"].as_str().ok_or("Missing exercise id")?)
                    .await?;
                Ok(json!(true))
            }
            _ => Err("Unknown command".into()),
        }
    }
    .await;
    result
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error":e}))))
}
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var_os("ACADEMY_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".academy/preview"));
    let postgres = std::env::var_os("ACADEMY_POSTGRES")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("src-tauri/resources/postgres"));
    let academy = Arc::new(Academy::start(&home, &postgres).await?);
    let router = Router::new()
        .route("/api/{command}", post(api))
        .with_state(academy.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4318").await?;
    println!("Local test bridge ready at http://127.0.0.1:4318");
    axum::serve(listener, router)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    academy.shutdown().await?;
    Ok(())
}
