#!/bin/bash
# --- PERSONA MATRIX GENESIS LAUNCH ---
echo "🌌 [GENESIS] Initializing Matrix Environment..."

# 1. Start the Sentinel in the background
./scripts/sentinel_monitor.sh &
SENTINEL_PID=$!
echo "📡 [SENTINEL] Active with PID: $SENTINEL_PID"

# 2. Launch the Rust Engine
echo "🚀 [ENGINE] Starting Persona Engine on Port 3000..."
cargo run -p persona_engine
