#!/bin/bash

# Build All Ëtrid Nodes for Multi-Node Testing
# This script builds FlareChain node + all 12 PBC collators

set -e

echo "=========================================="
echo "🏗️  Building Ëtrid Multi-Node Network"
echo "=========================================="
echo ""

BUILD_DIR="target/release"
mkdir -p "$BUILD_DIR"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

BUILT=0
FAILED=0
TOTAL=13  # 1 FlareChain + 12 PBC collators

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 Step 1: Building FlareChain Node"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if env SKIP_WASM_BUILD=1 cargo build --release -p flarechain-node 2>&1 | grep -q "Finished"; then
    echo -e "${GREEN}✅ FlareChain node built successfully${NC}"
    echo "   Binary: $BUILD_DIR/flarechain-node"
    ((BUILT++))
else
    echo -e "${RED}❌ FlareChain node build failed${NC}"
    ((FAILED++))
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 Step 2: Building 12 PBC Collators"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

COLLATORS=(
    "btc-pbc-collator"
    "eth-pbc-collator"
    "doge-pbc-collator"
    "xlm-pbc-collator"
    "xrp-pbc-collator"
    "bnb-pbc-collator"
    "trx-pbc-collator"
    "ada-pbc-collator"
    "link-pbc-collator"
    "matic-pbc-collator"
    "sc-usdt-pbc-collator"
    "sol-pbc-collator"
)

for collator in "${COLLATORS[@]}"; do
    echo -n "Building $collator... "

    if env SKIP_WASM_BUILD=1 cargo build --release -p "$collator" 2>&1 | grep -q "Finished"; then
        echo -e "${GREEN}✅${NC}"
        ((BUILT++))
    else
        echo -e "${RED}❌${NC}"
        ((FAILED++))
    fi
done

echo ""
echo "=========================================="
echo "📊 Build Summary"
echo "=========================================="
echo "Total nodes: $TOTAL"
echo -e "${GREEN}✅ Built: $BUILT${NC}"
if [ $FAILED -gt 0 ]; then
    echo -e "${RED}❌ Failed: $FAILED${NC}"
fi
echo "=========================================="
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 All nodes built successfully!${NC}"
    echo ""
    echo "Binaries located in: $BUILD_DIR/"
    echo ""
    echo "Available binaries:"
    ls -lh "$BUILD_DIR"/flarechain-node 2>/dev/null || echo "  (FlareChain binary not found)"
    ls -lh "$BUILD_DIR"/*-pbc-collator 2>/dev/null || echo "  (No collator binaries found)"
    echo ""
    exit 0
else
    echo -e "${RED}❌ Some builds failed. Check logs above.${NC}"
    exit 1
fi
