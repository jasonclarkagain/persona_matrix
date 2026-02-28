use axum::{routing::get, Json, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::fs::read_to_string;
use persona_shield::OperatorShield;
use persona_analytics::AnalyticsEngine;

struct AppState {
    _shield: Mutex<OperatorShield>,
    _analytics: Mutex<AnalyticsEngine>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        _shield: Mutex::new(OperatorShield::new(0.5)),
        _analytics: Mutex::new(AnalyticsEngine::new()),
    });

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/api/stats", get(get_stats))
        .route("/api/logs", get(get_logs))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 0));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    
    println!("🚀 [ENGINE] Matrix Interface live at http://{}", local_addr);
    axum::serve(listener, app).await.unwrap();
}

async fn get_stats() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "stability": 0.98,
        "active_persona": "Old Man Yeller",
        "shield_status": "Aggressive"
    }))
}

async fn get_logs() -> Json<serde_json::Value> {
    let content = read_to_string("logs/sentinel.log").unwrap_or_else(|_| "No logs found".to_string());
    let last_lines: Vec<&str> = content.lines().rev().take(5).collect();
    Json(serde_json::json!({ "entries": last_lines }))
}
