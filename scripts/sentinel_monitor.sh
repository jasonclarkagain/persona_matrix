#!/bin/bash
# --- SENTINEL GUARDIAN v1.0 ---
LOG_FILE="logs/sentinel.log"
mkdir -p logs

echo "$(date) [SENTINEL] Guardian Active." >> $LOG_FILE

while true; do
    CPU_LOAD=$(top -bn1 | grep "Cpu(s)" | awk '{print $2 + $4}')
    echo "$(date) [HEARTBEAT] CPU Load: $CPU_LOAD%" >> $LOG_FILE
    sleep 10
done
