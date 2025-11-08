#!/bin/bash

# ÉTRID Web UI Suite - Start All Applications
# This script starts all web UIs on their designated ports

set -e

echo "=========================================="
echo "ÉTRID Web UI Suite - Starting All Apps"
echo "=========================================="
echo ""

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# Color codes for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}Project Root: $PROJECT_ROOT${NC}"
echo ""

# Function to start an app in the background
start_app() {
    local app_name=$1
    local app_path=$2
    local port=$3

    echo -e "${YELLOW}Starting $app_name on port $port...${NC}"
    cd "$PROJECT_ROOT/$app_path"

    # Check if node_modules exists
    if [ ! -d "node_modules" ]; then
        echo "  Installing dependencies for $app_name..."
        if [[ "$app_path" == *"wallet-web"* ]]; then
            npm install --legacy-peer-deps
        else
            npm install
        fi
    fi

    # Check if build exists
    if [ ! -d ".next" ] && [ ! -d "out" ]; then
        echo "  Building $app_name..."
        npm run build
    fi

    # Start the application
    PORT=$port npm start > /tmp/etrid-$app_name.log 2>&1 &
    echo $! > /tmp/etrid-$app_name.pid
    echo -e "${GREEN}✓ $app_name started (PID: $!)${NC}"
    echo ""
}

# Start all applications
start_app "lightning-landing" "apps/lightning-landing" 3000
start_app "masterchef-dashboard" "apps/masterchef-dashboard" 3001
start_app "validator-dashboard" "apps/validator-dashboard" 3002
start_app "watchtower-monitor" "apps/watchtower-monitor" 3003
start_app "wallet-web" "apps/wallet-web/etrid-crypto-website" 3004

echo "=========================================="
echo -e "${GREEN}All applications started successfully!${NC}"
echo "=========================================="
echo ""
echo "Access the applications at:"
echo "  • Lightning Landing:      http://localhost:3000"
echo "  • MasterChef Dashboard:   http://localhost:3001"
echo "  • Validator Dashboard:    http://localhost:3002"
echo "  • Watchtower Monitor:     http://localhost:3003"
echo "  • Wallet Web:             http://localhost:3004"
echo ""
echo "Logs are stored in /tmp/etrid-*.log"
echo "To stop all apps, run: ./scripts/stop-all-web-uis.sh"
echo ""
