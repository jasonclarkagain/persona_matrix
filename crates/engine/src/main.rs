use axum::{routing::{get, post}, Json, Router, extract::State};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::fs::{read_to_string, write};
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct PersonaConfig { name: String, mode: String }
struct AppState { active_persona: Mutex<PersonaConfig> }

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        active_persona: Mutex::new(PersonaConfig { name: "Old Man Yeller".to_string(), mode: "Aggressive".to_string() }),
    });

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/api/stats", get(get_stats))
        .route("/api/switch", post(switch_persona))
        .route("/api/panic", post(panic_wipe))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 0));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 [ENGINE] Matrix Interface live at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn panic_wipe() -> Json<serde_json::Value> {
    let _ = Command::new("bash").arg("scripts/panic_wipe.sh").spawn();
    Json(serde_json::json!({ "status": "wiping" }))
}

async fn get_stats(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let persona = state.active_persona.lock().unwrap();
    let content = read_to_string("logs/sentinel.log").unwrap_or_default();
    let last_load = content.lines().filter(|l| l.contains("CPU Load:")).last()
        .and_then(|l| l.split("CPU Load: ").last()).and_then(|l| l.split('%').next()).and_then(|l| l.parse::<f64>().ok()).unwrap_or(0.0);
    Json(serde_json::json!({ "stability": (100.0 - last_load) / 100.0, "active_persona": persona.name, "mode": persona.mode }))
}

async fn switch_persona(State(state): State<Arc<AppState>>, Json(payload): Json<PersonaConfig>) -> Json<serde_json::Value> {
    let mut persona = state.active_persona.lock().unwrap();
    *persona = payload.clone();
    let _ = write("logs/matrix_mode.conf", &payload.mode);
    Json(serde_json::json!({ "status": "switched" }))
}
