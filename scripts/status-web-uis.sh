#!/bin/bash

# ÉTRID Web UI Suite - Status Check
# This script checks the status of all web UIs

echo "=========================================="
echo "ÉTRID Web UI Suite - Status"
echo "=========================================="
echo ""

# Color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to check app status
check_app() {
    local app_name=$1
    local port=$2
    local pid_file="/tmp/etrid-$app_name.pid"

    printf "%-25s " "$app_name:"

    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if ps -p $pid > /dev/null 2>&1; then
            # Check if port is listening
            if lsof -i:$port > /dev/null 2>&1 || netstat -tuln 2>/dev/null | grep -q ":$port "; then
                echo -e "${GREEN}RUNNING${NC} (PID: $pid, Port: $port)"
                echo "                           URL: ${BLUE}http://localhost:$port${NC}"
            else
                echo -e "${YELLOW}RUNNING${NC} (PID: $pid) - ${RED}Port $port not listening${NC}"
            fi
        else
            echo -e "${RED}STOPPED${NC} (stale PID file)"
        fi
    else
        # Check if port is in use by another process
        if lsof -i:$port > /dev/null 2>&1 || netstat -tuln 2>/dev/null | grep -q ":$port "; then
            echo -e "${YELLOW}UNKNOWN${NC} - Port $port in use by another process"
        else
            echo -e "${RED}STOPPED${NC}"
        fi
    fi
}

# Check all applications
check_app "lightning-landing" 3000
echo ""
check_app "masterchef-dashboard" 3001
echo ""
check_app "validator-dashboard" 3002
echo ""
check_app "watchtower-monitor" 3003
echo ""
check_app "wallet-web" 3004
echo ""

echo "=========================================="
echo "Log files location: /tmp/etrid-*.log"
echo "=========================================="
echo ""

# Show available scripts
echo "Available commands:"
echo "  Start all:  ./scripts/start-all-web-uis.sh"
echo "  Stop all:   ./scripts/stop-all-web-uis.sh"
echo "  Build all:  ./scripts/build-all-web-uis.sh"
echo "  Status:     ./scripts/status-web-uis.sh"
echo ""
