#!/bin/bash

# Simple 3-Node Validator Test
# All nodes use predefined network keys (safe for development)

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
BIN_DIR="$ETRID_ROOT/target/release"
DATA_DIR="$ETRID_ROOT/.validator-test"
LOGS_DIR="$DATA_DIR/logs"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

cleanup() {
    echo ""
    echo -e "${YELLOW}Stopping validators...${NC}"
    pkill -f flarechain-node || true
    sleep 1
    echo -e "${GREEN}✅ Stopped${NC}"
}

trap cleanup EXIT INT TERM

echo "=========================================="
echo "🏛️  Ëtrid 3-Validator Test Network"
echo "=========================================="
echo ""

rm -rf "$DATA_DIR"
mkdir -p "$LOGS_DIR"

# Predefined network keys (SAFE for development - only for P2P identity)
# These are NOT consensus keys - just libp2p peer IDs
ALICE_KEY="0000000000000000000000000000000000000000000000000000000000000001"
BOB_KEY="0000000000000000000000000000000000000000000000000000000000000002"
CHARLIE_KEY="0000000000000000000000000000000000000000000000000000000000000003"

# Corresponding Peer IDs (calculated from keys above)
ALICE_PEER="12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp"

echo -e "${GREEN}Starting Alice (Validator 1)${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --alice \
    --validator \
    --base-path "$DATA_DIR/alice" \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-cors all \
    --node-key $ALICE_KEY \
    --name "Alice-Validator" \
    > "$LOGS_DIR/alice.log" 2>&1 &

echo -e "   ${BLUE}RPC: http://localhost:9944${NC}"

sleep 3

echo -e "${GREEN}Starting Bob (Validator 2)${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --bob \
    --validator \
    --base-path "$DATA_DIR/bob" \
    --port 30334 \
    --rpc-port 9945 \
    --rpc-cors all \
    --node-key $BOB_KEY \
    --name "Bob-Validator" \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/$ALICE_PEER" \
    > "$LOGS_DIR/bob.log" 2>&1 &

echo -e "   ${BLUE}RPC: http://localhost:9945${NC}"

sleep 3

echo -e "${GREEN}Starting Charlie (Validator 3)${NC}"
$BIN_DIR/flarechain-node \
    --chain local \
    --charlie \
    --validator \
    --base-path "$DATA_DIR/charlie" \
    --port 30335 \
    --rpc-port 9946 \
    --rpc-cors all \
    --node-key $CHARLIE_KEY \
    --name "Charlie-Validator" \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/$ALICE_PEER" \
    > "$LOGS_DIR/charlie.log" 2>&1 &

echo -e "   ${BLUE}RPC: http://localhost:9946${NC}"

echo ""
echo -e "${BLUE}Waiting 15 seconds for network formation...${NC}"
sleep 15

echo ""
echo "=========================================="
echo "📊 Network Status"
echo "=========================================="
echo ""

for port in 9944 9945 9946; do
    name=$([ $port -eq 9944 ] && echo "Alice" || [ $port -eq 9945 ] && echo "Bob" || echo "Charlie")

    echo -e "${GREEN}$name:${NC}"

    # Health check
    health=$(curl -s -H "Content-Type: application/json" \
         -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
         http://localhost:$port 2>/dev/null)

    if [ ! -z "$health" ]; then
        peers=$(echo $health | jq -r '.result.peers' 2>/dev/null)
        syncing=$(echo $health | jq -r '.result.isSyncing' 2>/dev/null)

        echo "  Peers: $peers"
        echo "  Syncing: $syncing"

        # Block number
        header=$(curl -s -H "Content-Type: application/json" \
             -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
             http://localhost:$port 2>/dev/null)

        block=$(echo $header | jq -r '.result.number' 2>/dev/null)
        echo "  Block: $block"
    else
        echo "  ⚠️  Not responding"
    fi
    echo ""
done

echo "=========================================="
echo "✅ Validators Running"
echo "=========================================="
echo ""
echo -e "${BLUE}Monitor:${NC}"
echo "  tail -f $LOGS_DIR/alice.log | grep -E 'Imported|peers|Authored'"
echo ""
echo -e "${BLUE}Check peer connections:${NC}"
echo "  curl -s -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_peers\"}' http://localhost:9944 | jq '.result | length'"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
echo ""

wait
