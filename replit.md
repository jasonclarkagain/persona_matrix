# Persona Matrix

A Rust-based web service ("Neural Puppetry Interface") built with Axum.

## Architecture
- **Workspace** (`Cargo.toml`) with three crates:
  - `crates/engine` — `persona_engine` binary; Axum HTTP server, serves the UI and JSON API.
  - `crates/shield` — `persona_shield` library; input damping/sanitization helpers.
  - `crates/analytics` — `persona_analytics` library; in-memory metric history.
- **Static UI** (`static/control_panel.html`) served at `/`.
- **JSON endpoints**: `GET /api/personas`, `POST /api/activate`.

## Replit Setup
- Single web server, bound to `0.0.0.0:5000`.
- Workflow `Start application` runs `cargo run --bin persona_engine`.
- Deployment target: `vm` (long-running server).
  - Build: `cargo build --release --bin persona_engine`
  - Run: `./target/release/persona_engine`

## Notes
- Original source had `ax_core` import; corrected to `axum` so the project compiles.
- Original port `3000` changed to `5000` to match Replit's frontend port requirement.
