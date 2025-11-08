#!/bin/bash

###############################################################################
# Start All ËTRID Web UIs
# Launches all 5 web applications as background processes
###############################################################################

set -e

echo "🚀 Starting ËTRID Web UIs"
echo "=========================="
echo ""

# Project root
PROJECT_ROOT="/Users/macbook/Desktop/etrid"

# PID directory
PID_DIR="/tmp"
LOG_DIR="/tmp"

# Color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Array of applications
declare -a APPS=(
    "lightning-landing:3000"
    "masterchef-dashboard:3001"
    "validator-dashboard:3002"
    "watchtower-monitor:3003"
    "wallet-web/etrid-crypto-website:3004"
)

# Function to check if port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1 ; then
        return 0  # Port is in use
    else
        return 1  # Port is free
    fi
}

# Function to start an app
start_app() {
    local app_path=$1
    local port=$2
    local app_name=$(basename $(dirname $app_path))

    if [ "$app_name" = "etrid-crypto-website" ]; then
        app_name="wallet-web"
    fi

    local pid_file="${PID_DIR}/etrid-${app_name}.pid"
    local log_file="${LOG_DIR}/etrid-${app_name}.log"

    echo -e "${BLUE}Starting ${app_name}...${NC}"

    # Check if already running
    if [ -f "$pid_file" ]; then
        local old_pid=$(cat "$pid_file")
        if kill -0 "$old_pid" 2>/dev/null; then
            echo -e "${YELLOW}⚠️  Already running (PID: $old_pid)${NC}"
            return 0
        else
            rm "$pid_file"
        fi
    fi

    # Check if port is in use
    if check_port $port; then
        echo -e "${YELLOW}⚠️  Port $port is already in use${NC}"
        echo -e "${YELLOW}   Run: lsof -i :$port to see what's using it${NC}"
        return 1
    fi

    # Start the application
    cd "$PROJECT_ROOT/apps/$app_path"

    # Check if node_modules exists
    if [ ! -d "node_modules" ]; then
        echo -e "${YELLOW}   Installing dependencies...${NC}"
        if [[ "$app_path" == *"wallet-web"* ]]; then
            npm install --legacy-peer-deps > "$log_file" 2>&1
        else
            npm install > "$log_file" 2>&1
        fi
    fi

    # Start in background
    PORT=$port npm run dev > "$log_file" 2>&1 &
    local pid=$!
    echo $pid > "$pid_file"

    # Wait a moment and check if it started
    sleep 2
    if kill -0 "$pid" 2>/dev/null; then
        echo -e "${GREEN}✅ Started on port $port (PID: $pid)${NC}"
        echo -e "   Log: $log_file"
        echo -e "   URL: http://localhost:$port"
    else
        echo -e "${RED}❌ Failed to start${NC}"
        echo -e "   Check log: $log_file"
        return 1
    fi

    echo ""
}

# Start all applications
echo "Starting all web UIs..."
echo ""

success_count=0
for app_info in "${APPS[@]}"; do
    IFS=':' read -r app_path port <<< "$app_info"
    if start_app "$app_path" "$port"; then
        ((success_count++))
    fi
done

echo "=========================="
echo -e "${GREEN}Started $success_count/${#APPS[@]} applications${NC}"
echo ""
echo "📊 Status:"
echo "  Check status: ./scripts/status-web-uis.sh"
echo "  Stop all:     ./scripts/stop-all-web-uis.sh"
echo "  View logs:    tail -f /tmp/etrid-*.log"
echo ""
echo "🌐 Access:"
echo "  Lightning Landing:   http://localhost:3000"
echo "  MasterChef:          http://localhost:3001"
echo "  Validator Dashboard: http://localhost:3002"
echo "  Watchtower:          http://localhost:3003"
echo "  Wallet Web:          http://localhost:3004"
echo ""
