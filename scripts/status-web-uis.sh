#!/bin/bash

###############################################################################
# Check Status of All ËTRID Web UIs
# Shows running status and URLs
###############################################################################

echo "📊 ËTRID Web UIs Status"
echo "=========================="
echo ""

# PID directory
PID_DIR="/tmp"

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Applications with ports
declare -A APPS=(
    ["lightning-landing"]="3000"
    ["masterchef-dashboard"]="3001"
    ["validator-dashboard"]="3002"
    ["watchtower-monitor"]="3003"
    ["wallet-web"]="3004"
)

running_count=0
total_count=${#APPS[@]}

for app_name in "${!APPS[@]}"; do
    port="${APPS[$app_name]}"
    pid_file="${PID_DIR}/etrid-${app_name}.pid"

    printf "%-25s " "$app_name:"

    if [ -f "$pid_file" ]; then
        pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            echo -e "${GREEN}🟢 Running${NC} (PID: $pid, Port: $port)"
            echo "   URL: http://localhost:$port"
            ((running_count++))
        else
            echo -e "${RED}🔴 Stopped${NC} (stale PID file)"
        fi
    else
        # Check if port is in use anyway
        if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1 ; then
            echo -e "${YELLOW}🟡 Running${NC} (Port: $port, no PID file)"
            echo "   URL: http://localhost:$port"
        else
            echo -e "${RED}🔴 Stopped${NC}"
        fi
    fi
done

echo ""
echo "=========================="
echo -e "Status: ${GREEN}$running_count${NC}/$total_count running"
echo ""

if [ $running_count -eq 0 ]; then
    echo "To start: ./scripts/start-all-web-uis.sh"
elif [ $running_count -lt $total_count ]; then
    echo "To start all: ./scripts/start-all-web-uis.sh"
    echo "To stop all:  ./scripts/stop-all-web-uis.sh"
else
    echo "All services running! 🎉"
    echo "To stop: ./scripts/stop-all-web-uis.sh"
fi
echo ""
