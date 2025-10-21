#!/bin/bash

# EDSC Bridge Local Testnet Teardown Script
# Stops all services started by setup-local-testnet.sh

set -e

echo "╔══════════════════════════════════════════════════════════╗"
echo "║     EDSC Bridge Local Testnet Teardown                  ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Function to kill process on port
kill_port() {
    local port=$1
    local name=$2

    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo -n "Stopping $name on port $port..."
        kill $(lsof -ti:$port) 2>/dev/null || true
        sleep 1
        echo -e " ${GREEN}✓${NC}"
    else
        echo -e "$name (port $port): ${YELLOW}not running${NC}"
    fi
}

echo "🛑 Stopping services..."
echo ""

# Stop services by port
kill_port 8545 "Hardhat"
kill_port 9944 "Substrate"
kill_port 3000 "Attestation Service"
kill_port 3001 "Relayer Service (optional)"

echo ""
echo "🧹 Cleaning up..."

# Clean up any remaining node processes (be careful!)
pkill -f "hardhat node" 2>/dev/null || true
pkill -f "edsc-pbc-node" 2>/dev/null || true
pkill -f "attestation-service" 2>/dev/null || true
pkill -f "relayer-service" 2>/dev/null || true

sleep 1

echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║              ✅ TEARDOWN COMPLETE                         ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""
echo "All services stopped."
echo ""
