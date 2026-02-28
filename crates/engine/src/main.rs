use ax_core::{routing::{get, post}, Router, Json};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- PERSONA ARCHETYPES & PROFILES ---

#[derive(Serialize, Deserialize, Clone)]
pub struct Persona {
    pub id: String,
    pub name: String,
    pub behavior_profile: BehaviorProfile,
    pub vocal_profile: VocalProfile,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BehaviorProfile {
    pub expressiveness: f32,
    pub emotional_volatility: f32,
    pub resting_valence: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VocalProfile {
    pub base_pitch: f32,
    pub hoarseness: f32,
    pub filler_words: Vec<String>,
}

// --- ENGINE STATE ---

struct AppState {
    active_persona: Mutex<Option<Persona>>,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        active_persona: Mutex::new(None),
    });

    let app = Router::new()
        .route("/", get(serve_ui))
        .route("/api/personas", get(list_personas))
        .route("/api/activate", post(activate_persona))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌌 [MATRIX v3.0] Neural Puppetry Interface live at http://localhost:3000");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_ui() -> axum::response::Html<String> {
    axum::response::Html(std::fs::read_to_string("static/control_panel.html").unwrap_or_default())
}

async fn list_personas() -> Json<Vec<String>> {
    Json(vec!["old_man_yeller".to_string(), "south_central_commie".to_string(), "sigma_male".to_string()])
}

async fn activate_persona(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(payload): Json<HashMap<String, String>>
) -> Json<serde_json::Value> {
    let mut active = state.active_persona.lock().await;
    // In a real impl, we'd pull the full struct from a library
    println!("🚀 Transmuting to: {:?}", payload.get("persona_id"));
    Json(serde_json::json!({ "status": "transmutation_complete" }))
}
