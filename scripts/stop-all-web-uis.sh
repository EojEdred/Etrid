#!/bin/bash

# ÉTRID Web UI Suite - Stop All Applications
# This script stops all running web UIs

set -e

echo "=========================================="
echo "ÉTRID Web UI Suite - Stopping All Apps"
echo "=========================================="
echo ""

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to stop an app
stop_app() {
    local app_name=$1
    local pid_file="/tmp/etrid-$app_name.pid"

    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if ps -p $pid > /dev/null 2>&1; then
            echo -e "${YELLOW}Stopping $app_name (PID: $pid)...${NC}"
            kill $pid
            rm "$pid_file"
            echo -e "${GREEN}✓ $app_name stopped${NC}"
        else
            echo -e "${RED}Process $pid not found for $app_name${NC}"
            rm "$pid_file"
        fi
    else
        echo -e "${YELLOW}No PID file found for $app_name${NC}"
    fi
}

# Stop all applications
stop_app "lightning-landing"
stop_app "masterchef-dashboard"
stop_app "validator-dashboard"
stop_app "watchtower-monitor"
stop_app "wallet-web"

echo ""
echo "=========================================="
echo -e "${GREEN}All applications stopped${NC}"
echo "=========================================="
echo ""

# Clean up log files if they exist
if ls /tmp/etrid-*.log 1> /dev/null 2>&1; then
    echo "Cleaning up log files..."
    rm /tmp/etrid-*.log
    echo -e "${GREEN}✓ Log files cleaned${NC}"
fi

echo ""
