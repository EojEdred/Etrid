#!/bin/bash
# Ëtrid Protocol - Optimized Archive Node Startup Script
# Full history with RPC access

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

ETRID_ROOT="${ETRID_ROOT:-$(pwd)}"
BINARY="$ETRID_ROOT/target/release/flarechain-node"

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════╗"
echo "║                                                          ║"
echo "║     ËTRID PROTOCOL - OPTIMIZED ARCHIVE NODE            ║"
echo "║                                                          ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo -e "${YELLOW}Warning: Binary not found at $BINARY${NC}"
    echo "Build with: cargo build --release -p flarechain-node"
    exit 1
fi

# Configuration
CHAIN="${CHAIN:-flare-mainnet}"
NODE_NAME="${NODE_NAME:-etrid-archive-01}"
BASE_PATH="${BASE_PATH:-$ETRID_ROOT/data/archive}"

echo -e "${GREEN}Configuration:${NC}"
echo "  Chain: $CHAIN"
echo "  Node Name: $NODE_NAME"
echo "  Base Path: $BASE_PATH"
echo "  Binary: $BINARY"
echo "  Mode: Archive (full history)"
echo ""

# Create base path
mkdir -p "$BASE_PATH"

echo -e "${GREEN}Starting optimized archive node...${NC}"
echo ""

$BINARY \
  --chain "$CHAIN" \
  --name "$NODE_NAME" \
  --base-path "$BASE_PATH" \
  \
  `# Network ports` \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  --rpc-cors all \
  --unsafe-rpc-external \
  --unsafe-ws-external \
  \
  `# Archive configuration` \
  --pruning archive \
  --db-cache 8192 \
  --state-cache-size 4294967296 \
  \
  `# Network optimization (higher for archive)` \
  --max-parallel-downloads 16 \
  --in-peers 50 \
  --out-peers 50 \
  --kademlia-disjoint-query-paths \
  --sync warp \
  \
  `# RPC limits` \
  --rpc-max-connections 1000 \
  --rpc-max-request-size 15 \
  --rpc-max-response-size 15 \
  --rpc-max-subscriptions-per-connection 1024 \
  \
  `# Performance` \
  --wasm-execution compiled \
  --execution native-else-wasm \
  \
  `# Monitoring` \
  --prometheus-external \
  --prometheus-port 9615 \
  \
  `# Logging` \
  --log info,runtime=debug \
  \
  "$@"
