#!/bin/bash

# Generate Chain Specifications for Local Testnet
# Creates chain specs for FlareChain and PBC collators

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
BIN_DIR="$ETRID_ROOT/target/release"
SPECS_DIR="$ETRID_ROOT/chain-specs"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo "=========================================="
echo "📋 Generating Chain Specifications"
echo "=========================================="
echo ""

# Create specs directory
mkdir -p "$SPECS_DIR"

# Check if FlareChain node exists
if [ ! -f "$BIN_DIR/flarechain-node" ]; then
    echo -e "${RED}❌ FlareChain node binary not found${NC}"
    echo "Run: ./scripts/build_all_nodes.sh"
    exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌟 FlareChain Specifications"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Generate FlareChain development chain spec
echo -e "${GREEN}Generating FlareChain development spec...${NC}"
$BIN_DIR/flarechain-node build-spec --chain dev > "$SPECS_DIR/flarechain-dev.json" 2>/dev/null || {
    echo -e "${YELLOW}⚠️  build-spec command not available, using manual config${NC}"
    cat > "$SPECS_DIR/flarechain-dev.json" << 'EOF'
{
  "name": "FlareChain Development",
  "id": "flarechain_dev",
  "chainType": "Development",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "etrid",
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {},
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000],
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000],
          ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 1000000000000000]
        ]
      }
    }
  }
}
EOF
}
echo -e "   ${BLUE}Saved: $SPECS_DIR/flarechain-dev.json${NC}"
echo ""

# Generate FlareChain local testnet spec
echo -e "${GREEN}Generating FlareChain local testnet spec...${NC}"
$BIN_DIR/flarechain-node build-spec --chain local > "$SPECS_DIR/flarechain-local.json" 2>/dev/null || {
    echo -e "${YELLOW}⚠️  build-spec command not available, using manual config${NC}"
    cat > "$SPECS_DIR/flarechain-local.json" << 'EOF'
{
  "name": "FlareChain Local Testnet",
  "id": "flarechain_local",
  "chainType": "Local",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "etrid",
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {},
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000],
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000],
          ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 1000000000000000],
          ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 1000000000000000]
        ]
      },
      "asf": {
        "initialValidators": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
        ]
      }
    }
  }
}
EOF
}
echo -e "   ${BLUE}Saved: $SPECS_DIR/flarechain-local.json${NC}"
echo ""

# Generate raw chain spec (for production)
echo -e "${GREEN}Generating FlareChain raw spec...${NC}"
$BIN_DIR/flarechain-node build-spec --chain "$SPECS_DIR/flarechain-local.json" --raw > "$SPECS_DIR/flarechain-local-raw.json" 2>/dev/null || {
    echo -e "${YELLOW}⚠️  Raw spec generation skipped (requires full node implementation)${NC}"
}
if [ -f "$SPECS_DIR/flarechain-local-raw.json" ]; then
    echo -e "   ${BLUE}Saved: $SPECS_DIR/flarechain-local-raw.json${NC}"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💎 PBC Collator Specifications"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Generate specs for key PBC collators
PBCS=("btc" "eth" "doge")

for pbc in "${PBCS[@]}"; do
    if [ -f "$BIN_DIR/${pbc}-pbc-collator" ]; then
        PBC_UPPER=$(echo "$pbc" | tr '[:lower:]' '[:upper:]')
        echo -e "${GREEN}Generating ${PBC_UPPER} PBC spec...${NC}"

        # Try to generate spec from binary
        $BIN_DIR/${pbc}-pbc-collator build-spec --chain local > "$SPECS_DIR/pbc-${pbc}-local.json" 2>/dev/null || {
            # Fallback: create manual spec
            cat > "$SPECS_DIR/pbc-${pbc}-local.json" << EOF
{
  "name": "${PBC_UPPER} PBC Local",
  "id": "pbc_${pbc}_local",
  "chainType": "Local",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "pbc-${pbc}",
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "relay_chain": "flarechain_local",
  "para_id": $((1000 + $(echo "$pbc" | md5 | head -c 2 | xargs -I {} echo "0x{}" | xargs printf "%d") % 100)),
  "genesis": {
    "runtime": {
      "system": {},
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000]
        ]
      }
    }
  }
}
EOF
        }
        echo -e "   ${BLUE}Saved: $SPECS_DIR/pbc-${pbc}-local.json${NC}"
    else
        echo -e "${YELLOW}⚠️  ${PBC_UPPER} collator binary not found, skipping${NC}"
    fi
done

echo ""
echo "=========================================="
echo "✅ Chain Specifications Generated"
echo "=========================================="
echo ""
echo -e "${BLUE}Specifications saved in: $SPECS_DIR/${NC}"
ls -lh "$SPECS_DIR"/*.json 2>/dev/null || echo "No specs generated"
echo ""
echo -e "${GREEN}Next step:${NC} Run ./scripts/deploy_local_testnet.sh"
echo ""
