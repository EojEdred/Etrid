#!/bin/bash
# Insert ASF Consensus Keys for Production Test
# This generates the required 'asfk' key type for ASF consensus testing

set -e

ETRID_ROOT="$HOME/Desktop/etrid"
NODE_BIN="$ETRID_ROOT/target/release/flarechain-node"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}=== Inserting ASF Consensus Test Keys ===${NC}"
echo -e "${BLUE}Generating keys for 21 validators${NC}"
echo ""

# Check binary exists
if [ ! -f "$NODE_BIN" ]; then
    echo -e "${RED}Error: Node binary not found at $NODE_BIN${NC}"
    exit 1
fi

echo -e "${YELLOW}Cleaning old validator data...${NC}"
rm -rf /tmp/validator-*
echo -e "${GREEN}✓ Cleaned${NC}"
echo ""

echo -e "${GREEN}Inserting ASF keys for validators...${NC}"

for i in {1..21}; do
    BASE_PATH="/tmp/validator-$(printf "%02d" $i)"

    echo -n "Validator $(printf "%02d" $i): "

    # Create base path
    mkdir -p "$BASE_PATH"

    # Insert ASF consensus key (asfk)
    "$NODE_BIN" key insert \
        --base-path "$BASE_PATH" \
        --chain dev \
        --key-type asfk \
        --scheme sr25519 \
        --suri "//Validator$i" \
        > /dev/null 2>&1

    # Insert GRANDPA key (gran) for hybrid finality
    "$NODE_BIN" key insert \
        --base-path "$BASE_PATH" \
        --chain dev \
        --key-type gran \
        --scheme ed25519 \
        --suri "//Validator$i" \
        > /dev/null 2>&1

    echo -e "${GREEN}✓ ASF + GRANDPA keys inserted${NC}"
done

echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  ✅ All 21 validators have ASF keys    ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo "Key types inserted for each validator:"
echo "  - asfk (ASF consensus key, sr25519)"
echo "  - gran (GRANDPA finality key, ed25519)"
echo ""
echo -e "${BLUE}Ready to run: ./run-validator-production-test.sh${NC}"
