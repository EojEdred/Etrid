#!/bin/bash
#
# Etrid Validator - One-Command Launcher
# Starts your validator node + Operations Center
#

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Starting Etrid Validator + Operations Center"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check if this is first run
if [ ! -f "etrid-ops/.initialized" ]; then
    echo -e "${YELLOW}📦 First run detected - setting up Operations Center...${NC}"

    # Install dependencies
    cd etrid-ops/dashboard
    if [ ! -d "node_modules" ]; then
        echo "Installing dependencies..."
        npm install --production --silent
    fi

    cd ../api/etrid
    if [ ! -d "node_modules" ]; then
        npm install --production --silent
    fi

    cd "$SCRIPT_DIR"
    touch etrid-ops/.initialized

    echo -e "${GREEN}✓ Setup complete!${NC}"
    echo ""
fi

# Create data directory if it doesn't exist
mkdir -p data

# Start the validator node
echo -e "${BLUE}Starting validator node...${NC}"

# Check if validator is already running
if pgrep -f "etrid-node" > /dev/null; then
    echo -e "${YELLOW}⚠ Validator already running (PID: $(pgrep -f etrid-node))${NC}"
else
    # Start validator in background
    nohup ./etrid-node \
        --base-path ./data \
        --chain etrid-mainnet \
        --validator \
        --name "$(hostname)" \
        --ws-port 9944 \
        --rpc-port 9933 \
        --rpc-cors all \
        --rpc-methods Safe \
        > logs/validator.log 2>&1 &

    VALIDATOR_PID=$!
    echo $VALIDATOR_PID > .validator.pid
    echo -e "${GREEN}✓ Validator started (PID: $VALIDATOR_PID)${NC}"
fi

# Wait for validator to initialize
echo "Waiting for validator to initialize..."
sleep 3

# Start the Operations Center
echo -e "${BLUE}Starting Operations Center...${NC}"

cd etrid-ops/dashboard

# Check if ops center is already running
if pgrep -f "dashboard/server.js" > /dev/null; then
    echo -e "${YELLOW}⚠ Operations Center already running (PID: $(pgrep -f 'dashboard/server.js'))${NC}"
    OPS_PID=$(pgrep -f 'dashboard/server.js')
else
    # Start ops center in background
    NODE_ENV=production nohup node server.js > ../../logs/ops-center.log 2>&1 &
    OPS_PID=$!
    echo $OPS_PID > ../../.ops.pid
    echo -e "${GREEN}✓ Operations Center started (PID: $OPS_PID)${NC}"
fi

cd "$SCRIPT_DIR"

# Get local IP for remote access
LOCAL_IP=$(hostname -I | awk '{print $1}')

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}✓ Etrid Validator is Running!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📊 Operations Center:"
echo "   Local:  http://localhost:8080"
echo "   Remote: http://${LOCAL_IP}:8080"
echo ""
echo "🔗 Validator RPC:"
echo "   WebSocket: ws://localhost:9944"
echo "   HTTP:      http://localhost:9933"
echo ""

# Check if this is first time (no database)
if [ ! -f "data/etrid-ops.db" ]; then
    echo -e "${YELLOW}🔐 FIRST TIME SETUP REQUIRED${NC}"
    echo ""
    echo "   Open your browser and create your admin account:"
    echo -e "   ${BLUE}http://localhost:8080/register.html${NC}"
    echo ""
    echo "   Your validator will auto-configure once you register!"
else
    echo "🔑 Login at: http://localhost:8080/login.html"
fi

echo ""
echo "📱 Remote Access (Tailscale):"
if command -v tailscale &> /dev/null; then
    TAILSCALE_IP=$(tailscale ip -4 2>/dev/null || echo "not connected")
    if [ "$TAILSCALE_IP" != "not connected" ]; then
        echo "   http://${TAILSCALE_IP}:8080"
        echo -e "   ${GREEN}✓ Tailscale connected${NC}"
    else
        echo "   (run 'tailscale up' to enable)"
    fi
else
    echo "   (install Tailscale for secure remote access)"
fi

echo ""
echo "📝 Logs:"
echo "   Validator:  tail -f logs/validator.log"
echo "   Ops Center: tail -f logs/ops-center.log"
echo ""
echo "🛑 To stop: ./stop-validator.sh"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Save connection info for quick reference
cat > connection-info.txt << EOF
Etrid Validator Connection Info
================================

Operations Center:
  Local:  http://localhost:8080
  Remote: http://${LOCAL_IP}:8080

Validator RPC:
  WebSocket: ws://localhost:9944
  HTTP:      http://localhost:9933

Validator PID: $(cat .validator.pid 2>/dev/null || echo "N/A")
Ops Center PID: $(cat .ops.pid 2>/dev/null || echo "N/A")

Started: $(date)
EOF

# Keep running (optional - comment out to run in background)
# tail -f logs/validator.log logs/ops-center.log
