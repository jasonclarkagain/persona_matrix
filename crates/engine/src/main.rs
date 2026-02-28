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
    // Read the log to find the latest CPU load
    let content = read_to_string("logs/sentinel.log").unwrap_or_default();
    let last_load = content.lines()
        .filter(|l| l.contains("CPU Load:"))
        .last()
        .and_then(|l| l.split("CPU Load: ").last())
        .and_then(|l| l.split('%').next())
        .and_then(|l| l.parse::<f64>().ok())
        .unwrap_or(0.0);

    // Stability is inverse to CPU load (100 - load)
    let stability = (100.0 - last_load).clamp(0.0, 100.0) / 100.0;

    Json(serde_json::json!({
        "stability": stability,
        "active_persona": "Old Man Yeller",
        "shield_status": if stability < 0.5 { "CRITICAL" } else { "Aggressive" }
    }))
}

async fn get_logs() -> Json<serde_json::Value> {
    let content = read_to_string("logs/sentinel.log").unwrap_or_else(|_| "Waiting...".to_string());
    let last_lines: Vec<&str> = content.lines().rev().take(5).collect();
    Json(serde_json::json!({ "entries": last_lines }))
}
