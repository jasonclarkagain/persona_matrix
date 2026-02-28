#!/bin/bash
# --- PERSONA MATRIX TEST FLIGHT ---
echo "🧪 [TEST] Verifying Workspace Integrity..."

# 1. Check for Cargo.toml at root
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Root Cargo.toml not found."
    exit 1
fi

# 2. Attempt Compilation of all crates
echo "📦 [TEST] Compiling crates (Shield, Engine, Analytics)..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ [SUCCESS] Workspace logic is sound."
    echo "🚀 You are clear to run ./scripts/genesis_launch.sh"
else
    echo "❌ [FAIL] Compilation errors detected. Check the output above."
    exit 1
fi
