#!/bin/bash
# --- PERSONA MATRIX PANIC WIPE ---
echo "⚠️ [PANIC] Initializing Total Wipe Protocol..."

# 1. Kill the Sentinel and Engine immediately
pkill -f sentinel_monitor.sh
pkill -f persona_engine

# 2. Scrub the logs and temporary configs
rm -rf logs/*.log
rm -f logs/matrix_mode.conf
touch logs/.gitkeep

echo "🧹 [CLEAN] Processes halted. Logs purged."
