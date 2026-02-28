#!/bin/bash
# --- ADAPTIVE SENTINEL GUARDIAN v1.1 ---
LOG_FILE="logs/sentinel.log"
MODE_FILE="logs/matrix_mode.conf"
mkdir -p logs

echo "$(date) [SENTINEL] Adaptive Guardian Active." >> $LOG_FILE

while true; do
    # Check current persona mode
    CURRENT_MODE=$(cat $MODE_FILE 2>/dev/null || echo "Aggressive")

    # Capture CPU load
    CPU_LOAD=$(top -bn1 | grep "Cpu(s)" | awk '{print $2 + $4}')
    echo "$(date) [HEARTBEAT] ($CURRENT_MODE) CPU Load: $CPU_LOAD%" >> $LOG_FILE
    
    # Adjust sleep interval based on Persona
    if [ "$CURRENT_MODE" == "Stealth" ]; then
        sleep 60
    else
        sleep 10
    fi
done
