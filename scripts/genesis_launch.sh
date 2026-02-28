#!/bin/bash
echo "🚀 [GENESIS] Initializing Persona Matrix v3.0 Stack..."

# 1. Directory & Model Scaffolding
mkdir -p models logs static/css static/js
echo "📂 [GENESIS] Directory structure verified."

# 2. Build Containerized Infrastructure
# Note: We use the local context since we are in the root
echo "🏗️ [GENESIS] Building immutable containers..."
docker-compose build --quiet

# 3. Launch
echo "🚢 [GENESIS] Deploying services..."
docker-compose up -d

echo "================================================================"
echo "✅ PERSONA MATRIX DEPLOYED SUCCESSFULLY"
echo "▶ CONTROL PANEL: http://localhost:3000"
echo "================================================================"
