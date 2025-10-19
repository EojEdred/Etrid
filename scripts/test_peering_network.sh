#!/bin/bash

# Test Multi-Node Network with Proper Peering
# Both nodes use same chain spec and should connect

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
BIN_DIR="$ETRID_ROOT/target/release"
DATA_DIR="$ETRID_ROOT/.peering-test"
LOGS_DIR="$DATA_DIR/logs"
SPECS_DIR="$ETRID_ROOT/chain-specs"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

cleanup() {
    echo ""
    echo -e "${YELLOW}🛑 Shutting down network...${NC}"
    pkill -f flarechain-node || true
    sleep 1
    echo -e "${GREEN}✅ Network stopped${NC}"
}

trap cleanup EXIT INT TERM

echo "=========================================="
echo "🌐 Ëtrid Peering Network Test"
echo "=========================================="
echo ""

# Clean start
rm -rf "$DATA_DIR"
mkdir -p "$LOGS_DIR"

# Check if chain spec exists, create if not
if [ ! -f "$SPECS_DIR/flarechain-local.json" ]; then
    echo -e "${YELLOW}Generating chain spec...${NC}"
    mkdir -p "$SPECS_DIR"
    $BIN_DIR/flarechain-node build-spec --chain local > "$SPECS_DIR/flarechain-local.json" 2>/dev/null || {
        echo -e "${YELLOW}Using default local chain${NC}"
    }
fi

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}Starting Validator Network${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Get Alice's peer ID from node-key
ALICE_PEER_ID="12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"

# Start Alice
echo -e "${GREEN}🚀 Starting Alice (Validator 1)${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --alice \
    --validator \
    --base-path "$DATA_DIR/alice" \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    --unsafe-rpc-external \
    --name "Alice" \
    > "$LOGS_DIR/alice.log" 2>&1 &

ALICE_PID=$!
echo -e "   ${BLUE}PID:${NC} $ALICE_PID"
echo -e "   ${BLUE}RPC:${NC} http://localhost:9944"
echo -e "   ${BLUE}P2P:${NC} /ip4/127.0.0.1/tcp/30333/p2p/$ALICE_PEER_ID"
echo ""

sleep 5

# Start Bob
echo -e "${GREEN}🚀 Starting Bob (Validator 2)${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --bob \
    --validator \
    --base-path "$DATA_DIR/bob" \
    --port 30334 \
    --rpc-port 9945 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --unsafe-rpc-external \
    --name "Bob" \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/$ALICE_PEER_ID" \
    > "$LOGS_DIR/bob.log" 2>&1 &

BOB_PID=$!
echo -e "   ${BLUE}PID:${NC} $BOB_PID"
echo -e "   ${BLUE}RPC:${NC} http://localhost:9945"
echo -e "   ${BLUE}Bootnode:${NC} Alice"
echo ""

sleep 5

# Start Charlie (non-validator full node)
echo -e "${GREEN}🚀 Starting Charlie (Full Node)${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --charlie \
    --base-path "$DATA_DIR/charlie" \
    --port 30335 \
    --rpc-port 9946 \
    --rpc-cors all \
    --rpc-methods=unsafe \
    --unsafe-rpc-external \
    --name "Charlie" \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/$ALICE_PEER_ID" \
    > "$LOGS_DIR/charlie.log" 2>&1 &

CHARLIE_PID=$!
echo -e "   ${BLUE}PID:${NC} $CHARLIE_PID"
echo -e "   ${BLUE}RPC:${NC} http://localhost:9946"
echo -e "   ${BLUE}Bootnode:${NC} Alice"
echo ""

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}Checking Node Status${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

sleep 5

# Check processes
echo -e "${BLUE}Process Status:${NC}"
for name in Alice Bob Charlie; do
    NAME_UPPER=$(echo "$name" | tr '[:lower:]' '[:upper:]')
    pid_var="${NAME_UPPER}_PID"
    pid=${!pid_var}
    echo -n "  $name: "
    if ps -p $pid > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Running (PID $pid)${NC}"
    else
        echo -e "${YELLOW}⚠️  Stopped${NC}"
    fi
done

echo ""
echo -e "${BLUE}Waiting 10 seconds for consensus...${NC}"
sleep 10

echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}Network Health Check${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

# Query each node
for port in 9944 9945 9946; do
    name="Unknown"
    case $port in
        9944) name="Alice" ;;
        9945) name="Bob" ;;
        9946) name="Charlie" ;;
    esac

    echo -e "${BLUE}$name (localhost:$port):${NC}"

    # Get health
    health=$(curl -s -H "Content-Type: application/json" \
         -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
         http://localhost:$port 2>/dev/null)

    if [ ! -z "$health" ]; then
        peers=$(echo $health | jq -r '.result.peers' 2>/dev/null)
        syncing=$(echo $health | jq -r '.result.isSyncing' 2>/dev/null)

        echo -e "  Peers: ${GREEN}$peers${NC}"
        echo -e "  Syncing: $syncing"

        # Get latest block
        header=$(curl -s -H "Content-Type: application/json" \
             -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
             http://localhost:$port 2>/dev/null)

        block_num=$(echo $header | jq -r '.result.number' 2>/dev/null)
        echo -e "  Latest block: #$block_num"
    else
        echo -e "  ${YELLOW}⚠️  RPC not responding${NC}"
    fi
    echo ""
done

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}Recent Activity${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

for name in alice bob charlie; do
    NAME_UPPER=$(echo "$name" | tr '[:lower:]' '[:upper:]')
    echo -e "${BLUE}$NAME_UPPER (last 8 lines):${NC}"
    tail -8 "$LOGS_DIR/${name}.log" | grep -E "(Imported|peers|Idle|Authored|Syncing|Finalized)" || echo "  (No recent activity)"
    echo ""
done

echo "=========================================="
echo "✅ Network Running"
echo "=========================================="
echo ""
echo -e "${GREEN}Nodes:${NC}"
echo "  • Alice   (Validator) - http://localhost:9944"
echo "  • Bob     (Validator) - http://localhost:9945"
echo "  • Charlie (Full Node) - http://localhost:9946"
echo ""
echo -e "${BLUE}📋 Monitor logs:${NC}"
echo "  tail -f $LOGS_DIR/alice.log"
echo "  tail -f $LOGS_DIR/bob.log"
echo "  tail -f $LOGS_DIR/charlie.log"
echo ""
echo -e "${BLUE}🔍 Check peers:${NC}"
echo "  curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_peers\"}' http://localhost:9944 | jq"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop the network${NC}"
echo ""

wait
