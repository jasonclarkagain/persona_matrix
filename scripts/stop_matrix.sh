#!/bin/bash
# --- PERSONA MATRIX SHUTDOWN ---
echo "🛑 [SHUTDOWN] Terminating Matrix processes..."

# 1. Kill the Sentinel Monitor
SENTINEL_PID=$(pgrep -f sentinel_monitor.sh)
if [ -n "$SENTINEL_PID" ]; then
    kill $SENTINEL_PID
    echo "📡 [SENTINEL] Process $SENTINEL_PID halted."
else
    echo "📡 [SENTINEL] No active process found."
fi

# 2. Kill the Persona Engine (Rust binary)
ENGINE_PID=$(pgrep -f persona_engine)
if [ -n "$ENGINE_PID" ]; then
    kill $ENGINE_PID
    echo "🚀 [ENGINE] Process $ENGINE_PID halted."
else
    echo "🚀 [ENGINE] No active process found."
fi

echo "✅ [SUCCESS] All systems offline."
