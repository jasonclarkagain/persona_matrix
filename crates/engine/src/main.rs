use axum::{routing::get, Json, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use persona_shield::OperatorShield;
use persona_analytics::AnalyticsEngine;

struct AppState {
    shield: Mutex<OperatorShield>,
    analytics: Mutex<AnalyticsEngine>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        shield: Mutex::new(OperatorShield::new(0.5)),
        analytics: Mutex::new(AnalyticsEngine::new()),
    });

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/health", get(|| async { "ONLINE" }))
        .route("/api/stats", get(get_stats))
        .with_state(state);

    // Setting port to 0 for dynamic allocation
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
