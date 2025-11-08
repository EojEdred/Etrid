#!/bin/bash

###############################################################################
# Stop All ËTRID Web UIs
# Gracefully stops all running web applications
###############################################################################

echo "🛑 Stopping ËTRID Web UIs"
echo "=========================="
echo ""

# PID directory
PID_DIR="/tmp"
LOG_DIR="/tmp"

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Application names
declare -a APPS=(
    "lightning-landing"
    "masterchef-dashboard"
    "validator-dashboard"
    "watchtower-monitor"
    "wallet-web"
)

stopped_count=0

for app_name in "${APPS[@]}"; do
    pid_file="${PID_DIR}/etrid-${app_name}.pid"
    log_file="${LOG_DIR}/etrid-${app_name}.log"

    if [ -f "$pid_file" ]; then
        pid=$(cat "$pid_file")
        echo "Stopping $app_name (PID: $pid)..."

        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null
            sleep 1

            # Force kill if still running
            if kill -0 "$pid" 2>/dev/null; then
                kill -9 "$pid" 2>/dev/null
            fi

            echo -e "${GREEN}✅ Stopped${NC}"
            ((stopped_count++))
        else
            echo -e "${YELLOW}⚠️  Process not running${NC}"
        fi

        # Clean up PID file
        rm "$pid_file"
    else
        echo "$app_name: Not running"
    fi
done

# Clean up log files (optional)
echo ""
read -p "Delete log files? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -f "${LOG_DIR}/etrid-"*.log
    echo -e "${GREEN}✅ Log files deleted${NC}"
fi

echo ""
echo "=========================="
echo -e "${GREEN}Stopped $stopped_count applications${NC}"
echo ""
