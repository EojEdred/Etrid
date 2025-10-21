#!/bin/bash

# Quick Multi-Node Network Test
# Simplified version that just tests if nodes can connect

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
BIN_DIR="$ETRID_ROOT/target/release"
DATA_DIR="$ETRID_ROOT/.test-network"
LOGS_DIR="$DATA_DIR/logs"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 Shutting down test network...${NC}"
    pkill -f flarechain-node || true
    sleep 1
    echo -e "${GREEN}✅ Network stopped${NC}"
}

trap cleanup EXIT INT TERM

echo "=========================================="
echo "🧪 Ëtrid Quick Network Test"
echo "=========================================="
echo ""

# Clean start
rm -rf "$DATA_DIR"
mkdir -p "$LOGS_DIR"

echo -e "${GREEN}Starting Alice (Validator)...${NC}"
$BIN_DIR/flarechain-node \
    --dev \
    --tmp \
    --alice \
    --validator \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    > "$LOGS_DIR/alice.log" 2>&1 &

ALICE_PID=$!
echo -e "   ${BLUE}PID: $ALICE_PID${NC}"
echo -e "   ${BLUE}RPC: http://localhost:9944${NC}"
echo ""

sleep 3

echo -e "${GREEN}Starting Bob (Validator)...${NC}"
$BIN_DIR/flarechain-node \
    --dev \
    --tmp \
    --bob \
    --validator \
    --port 30334 \
    --rpc-port 9945 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp" \
    > "$LOGS_DIR/bob.log" 2>&1 &

BOB_PID=$!
echo -e "   ${BLUE}PID: $BOB_PID${NC}"
echo -e "   ${BLUE}RPC: http://localhost:9945${NC}"
echo ""

sleep 3

echo "=========================================="
echo "✅ Network Test Running"
echo "=========================================="
echo ""
echo -e "${BLUE}Checking node status...${NC}"
echo ""

# Check Alice
echo -n "Alice: "
if ps -p $ALICE_PID > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Running${NC}"
else
    echo -e "${YELLOW}⚠️  Stopped${NC}"
fi

# Check Bob
echo -n "Bob:   "
if ps -p $BOB_PID > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Running${NC}"
else
    echo -e "${YELLOW}⚠️  Stopped${NC}"
fi

echo ""
echo -e "${BLUE}Waiting 10 seconds for blocks...${NC}"
sleep 10

echo ""
echo "=========================================="
echo "📊 Node Status"
echo "=========================================="
echo ""

# Query Alice
echo -e "${BLUE}Alice Status:${NC}"
curl -s -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
     http://localhost:9944 | jq '.' 2>/dev/null || echo "RPC not responding"

echo ""

# Query Bob
echo -e "${BLUE}Bob Status:${NC}"
curl -s -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
     http://localhost:9945 | jq '.' 2>/dev/null || echo "RPC not responding"

echo ""
echo "=========================================="
echo "📋 Recent Logs"
echo "=========================================="
echo ""
echo -e "${BLUE}Alice (last 5 lines):${NC}"
tail -5 "$LOGS_DIR/alice.log"
echo ""
echo -e "${BLUE}Bob (last 5 lines):${NC}"
tail -5 "$LOGS_DIR/bob.log"

echo ""
echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
echo ""

wait
