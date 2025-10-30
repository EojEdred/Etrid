#!/bin/bash
# Ëtrid Protocol - Optimized Validator Node Startup Script
# Production-ready configuration with performance optimizations

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
echo "║    ËTRID PROTOCOL - OPTIMIZED VALIDATOR NODE           ║"
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
NODE_NAME="${NODE_NAME:-etrid-validator-01}"
BASE_PATH="${BASE_PATH:-$ETRID_ROOT/data/validator}"

echo -e "${GREEN}Configuration:${NC}"
echo "  Chain: $CHAIN"
echo "  Node Name: $NODE_NAME"
echo "  Base Path: $BASE_PATH"
echo "  Binary: $BINARY"
echo ""

# Create base path
mkdir -p "$BASE_PATH"

echo -e "${GREEN}Starting optimized validator node...${NC}"
echo ""

$BINARY \
  --chain "$CHAIN" \
  --name "$NODE_NAME" \
  --base-path "$BASE_PATH" \
  --validator \
  \
  `# Network ports` \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  \
  `# Database optimization` \
  --db-cache 4096 \
  --pruning 256 \
  --state-cache-size 1073741824 \
  \
  `# Network optimization` \
  --max-parallel-downloads 8 \
  --in-peers 25 \
  --out-peers 25 \
  --kademlia-disjoint-query-paths \
  --sync warp \
  --enable-offchain-indexing true \
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
