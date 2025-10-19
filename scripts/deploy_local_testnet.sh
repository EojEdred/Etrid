#!/bin/bash

# Deploy Local Ëtrid Multi-Node Testnet
# Starts FlareChain + selected PBC collators for testing

set -e

# Configuration
ETRID_ROOT="/Users/macbook/Desktop/etrid"
BIN_DIR="$ETRID_ROOT/target/release"
DATA_DIR="$ETRID_ROOT/.local-testnet"
LOGS_DIR="$DATA_DIR/logs"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Cleanup function
cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 Shutting down testnet...${NC}"
    pkill -f flarechain-node || true
    pkill -f pbc-collator || true
    echo -e "${GREEN}✅ Testnet stopped${NC}"
}

trap cleanup EXIT INT TERM

echo "=========================================="
echo "🚀 Ëtrid Local Testnet Deployment"
echo "=========================================="
echo ""

# Create directories
mkdir -p "$DATA_DIR"
mkdir -p "$LOGS_DIR"

# Check binaries exist
if [ ! -f "$BIN_DIR/flarechain-node" ]; then
    echo -e "${RED}❌ FlareChain node binary not found${NC}"
    echo "Run: ./scripts/build_all_nodes.sh"
    exit 1
fi

echo -e "${BLUE}📁 Data directory: $DATA_DIR${NC}"
echo -e "${BLUE}📋 Logs directory: $LOGS_DIR${NC}"
echo ""

# Purge old chain data (optional - uncomment for fresh start)
# echo -e "${YELLOW}🗑️  Purging old chain data...${NC}"
# rm -rf "$DATA_DIR/flarechain"
# rm -rf "$DATA_DIR/pbc-"*

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌟 Starting FlareChain Nodes"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Node 1: Alice (Validator)
echo -e "${GREEN}Starting FlareChain Node 1 (Alice - Validator)...${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --alice \
    --validator \
    --base-path "$DATA_DIR/flarechain-alice" \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    > "$LOGS_DIR/flarechain-alice.log" 2>&1 &

ALICE_PID=$!
echo -e "   ${BLUE}PID: $ALICE_PID${NC}"
echo -e "   ${BLUE}RPC: http://localhost:9944${NC}"
echo -e "   ${BLUE}Log: $LOGS_DIR/flarechain-alice.log${NC}"
echo ""

sleep 2

# Node 2: Bob (Validator)
echo -e "${GREEN}Starting FlareChain Node 2 (Bob - Validator)...${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --bob \
    --validator \
    --base-path "$DATA_DIR/flarechain-bob" \
    --port 30334 \
    --rpc-port 9945 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp" \
    > "$LOGS_DIR/flarechain-bob.log" 2>&1 &

BOB_PID=$!
echo -e "   ${BLUE}PID: $BOB_PID${NC}"
echo -e "   ${BLUE}RPC: http://localhost:9945${NC}"
echo -e "   ${BLUE}Log: $LOGS_DIR/flarechain-bob.log${NC}"
echo ""

sleep 2

# Node 3: Charlie (Full Node)
echo -e "${GREEN}Starting FlareChain Node 3 (Charlie - Full Node)...${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --charlie \
    --base-path "$DATA_DIR/flarechain-charlie" \
    --port 30335 \
    --rpc-port 9946 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp" \
    > "$LOGS_DIR/flarechain-charlie.log" 2>&1 &

CHARLIE_PID=$!
echo -e "   ${BLUE}PID: $CHARLIE_PID${NC}"
echo -e "   ${BLUE}RPC: http://localhost:9946${NC}"
echo -e "   ${BLUE}Log: $LOGS_DIR/flarechain-charlie.log${NC}"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💎 Starting PBC Collators"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Start a few key PBC collators for testing
COLLATORS_TO_START=("btc" "eth" "doge")
PORT_BASE=40000
RPC_PORT_BASE=8000

for i in "${!COLLATORS_TO_START[@]}"; do
    pbc="${COLLATORS_TO_START[$i]}"
    PORT=$((PORT_BASE + i))
    RPC_PORT=$((RPC_PORT_BASE + i))
    PBC_UPPER=$(echo "$pbc" | tr '[:lower:]' '[:upper:]')

    if [ -f "$BIN_DIR/${pbc}-pbc-collator" ]; then
        echo -e "${GREEN}Starting ${PBC_UPPER} PBC Collator...${NC}"
        $BIN_DIR/${pbc}-pbc-collator \
            --validator \
            --chain local \
            --base-path "$DATA_DIR/pbc-${pbc}" \
            --port $PORT \
            --rpc-port $RPC_PORT \
            --rpc-cors all \
            --rpc-methods=unsafe \
            --name "${PBC_UPPER}-Validator" \
            --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp" \
            > "$LOGS_DIR/pbc-${pbc}.log" 2>&1 &

        PID=$!
        echo -e "   ${BLUE}PID: $PID${NC}"
        echo -e "   ${BLUE}RPC: http://localhost:$RPC_PORT${NC}"
        echo -e "   ${BLUE}Log: $LOGS_DIR/pbc-${pbc}.log${NC}"
        echo ""
    else
        echo -e "${YELLOW}⚠️  ${PBC_UPPER} collator binary not found, skipping${NC}"
        echo ""
    fi
done

echo "=========================================="
echo "✅ Testnet Running!"
echo "=========================================="
echo ""
echo -e "${GREEN}FlareChain Nodes:${NC}"
echo "  • Alice   (Validator) - http://localhost:9944"
echo "  • Bob     (Validator) - http://localhost:9945"
echo "  • Charlie (Full Node) - http://localhost:9946"
echo ""
echo -e "${GREEN}PBC Collators:${NC}"
for i in "${!COLLATORS_TO_START[@]}"; do
    pbc="${COLLATORS_TO_START[$i]}"
    RPC_PORT=$((RPC_PORT_BASE + i))
    PBC_UPPER=$(echo "$pbc" | tr '[:lower:]' '[:upper:]')
    echo "  • ${PBC_UPPER} - http://localhost:$RPC_PORT"
done
echo ""
echo -e "${BLUE}📋 View logs:${NC}"
echo "  tail -f $LOGS_DIR/flarechain-alice.log"
echo "  tail -f $LOGS_DIR/pbc-btc.log"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop the testnet${NC}"
echo ""

# Wait for interrupt
wait
