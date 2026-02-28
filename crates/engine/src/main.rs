use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use persona_shield::OperatorShield;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
struct HealthStatus {
    status: String,
    shield_active: bool,
    version: String,
}

struct AppState {
    shield: Mutex<OperatorShield>,
}

#[tokio::main]
async fn main() {
    // Initialize the Corporate Logging Layer
    tracing_subscriber::fmt::init();

    // Create the Protected State
    let shared_state = Arc::new(AppState {
        shield: Mutex::new(OperatorShield::new(0.5)), // 50% smoothing by default
    });

    // Define the Glassy API Routes
    let app = Router::new()
        .route("/api/personas", get(list_personas))
        .route("/health", get(health_check))
        .with_state(shared_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 [ENGINE] Matrix Heartbeat started on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "ONLINE".to_string(),
        shield_active: true,
        version: "3.0.0-Corporate".to_string(),
    })
}

async fn list_personas() -> Json<Vec<String>> {
    Json(vec!["Old Man Yeller".to_string(), "The Manifestor".to_string()])
}
