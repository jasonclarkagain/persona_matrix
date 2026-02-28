#!/bin/bash
echo "🛡️ [FIELD] Finalizing Matrix Integrity..."
./scripts/stop_matrix.sh
rm -rf logs/*.log
./scripts/genesis_launch.sh
