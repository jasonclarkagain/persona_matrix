#!/bin/bash
# --- SENTINEL GUARDIAN v1.0 ---
LOG_FILE="logs/sentinel.log"

echo "$(date) [SENTINEL] Guardian Active. Tracking system stability..." >> $LOG_FILE

while true; do
    # Capture CPU load for the Analytics engine
    CPU_LOAD=$(top -bn1 | grep "Cpu(s)" | awk '{print $2 + $4}')
    echo "$(date) [HEARTBEAT] CPU Load: $CPU_LOAD%" >> $LOG_FILE
    
    # Simple alert logic if the Matrix experiences high stress
    if (( $(echo "$CPU_LOAD > 90.0" | bc -l) )); then
        echo "$(date) [CRITICAL] High System Stress Detected!" >> $LOG_FILE
    fi
    sleep 10
done
