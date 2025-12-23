#!/bin/bash

################################################################################
# Phase 5: Intent Router System Deployment
#
# Deploys:
#   - TwoTierBridgeRouter
#   - AutoSwapExecutor
#   - StablecoinRouter
#   - IntentRouter
#
# Wires:
#   - All routers to Address Registry
#   - Grants CALLER permissions
################################################################################

set -e

NETWORK=$1
ADDRESSES_FILE=$2
CONFIG_FILE=$3

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Load configuration
REGISTRY_ADDRESS=$(jq -r '.address_registry' "$ADDRESSES_FILE")
MULTISIG=$(jq -r '.multisig_wallet' "$CONFIG_FILE")

echo -e "${YELLOW}Phase 5.1: Deploying TwoTierBridgeRouter...${NC}"

cd ../intent-router/core/two-tier-bridge-router
cargo contract build --release

# Note: Bridge pallet ID will be configured after Phase 6
BRIDGE_PALLET_ID=42

BRIDGE_ROUTER_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$BRIDGE_PALLET_ID" "$REGISTRY_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ TwoTierBridgeRouter deployed: $BRIDGE_ROUTER_ADDRESS${NC}"

jq --arg addr "$BRIDGE_ROUTER_ADDRESS" '.intent_router_system.two_tier_bridge_router = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 5.2: Deploying AutoSwapExecutor...${NC}"

cd ../auto-swap-executor
cargo contract build --release

EXECUTOR_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$BRIDGE_ROUTER_ADDRESS" "$REGISTRY_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ AutoSwapExecutor deployed: $EXECUTOR_ADDRESS${NC}"

jq --arg addr "$EXECUTOR_ADDRESS" '.intent_router_system.auto_swap_executor = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 5.3: Deploying StablecoinRouter...${NC}"

cd ../../routing/stablecoin-router
cargo contract build --release

# EDSC-PBC parachain ID (from configuration)
EDSC_PBC_PARACHAIN_ID=14

EDSC_MINTING_ENGINE=$(jq -r '.edsc_system.edsc_minting_engine' "$ADDRESSES_FILE")

STABLECOIN_ROUTER_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$EDSC_MINTING_ENGINE" "$EDSC_PBC_PARACHAIN_ID" \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ StablecoinRouter deployed: $STABLECOIN_ROUTER_ADDRESS${NC}"

jq --arg addr "$STABLECOIN_ROUTER_ADDRESS" '.intent_router_system.stablecoin_router = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 5.4: Deploying IntentRouter...${NC}"

cd ../../core/intent-router
cargo contract build --release

TREASURY=$(jq -r '.treasury' "$CONFIG_FILE")

INTENT_ROUTER_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$EXECUTOR_ADDRESS" "$STABLECOIN_ROUTER_ADDRESS" "$REGISTRY_ADDRESS" "$TREASURY" \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ IntentRouter deployed: $INTENT_ROUTER_ADDRESS${NC}"

jq --arg addr "$INTENT_ROUTER_ADDRESS" '.intent_router_system.intent_router = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 5.5: Granting CALLER permissions...${NC}"

# Grant AutoSwapExecutor → TwoTierBridgeRouter
echo "  Granting CALLER_ROLE: AutoSwapExecutor → TwoTierBridgeRouter..."
cd ../two-tier-bridge-router

cargo contract call \
    --contract "$BRIDGE_ROUTER_ADDRESS" \
    --message grant_role \
    --args 0 "$EXECUTOR_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ CALLER_ROLE granted${NC}"

# Grant IntentRouter → AutoSwapExecutor
echo "  Granting CALLER_ROLE: IntentRouter → AutoSwapExecutor..."
cd ../auto-swap-executor

cargo contract call \
    --contract "$EXECUTOR_ADDRESS" \
    --message grant_role \
    --args 0 "$INTENT_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ CALLER_ROLE granted${NC}"

# Grant IntentRouter → StablecoinRouter
echo "  Granting CALLER_ROLE: IntentRouter → StablecoinRouter..."
cd ../../routing/stablecoin-router

cargo contract call \
    --contract "$STABLECOIN_ROUTER_ADDRESS" \
    --message grant_role \
    --args 0 "$INTENT_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ CALLER_ROLE granted${NC}"

# Grant StablecoinRouter → EDSCMintingEngine
echo "  Granting CALLER_ROLE: StablecoinRouter → EDSCMintingEngine..."
cd ../../../edsc/core/minting-engine

cargo contract call \
    --contract "$EDSC_MINTING_ENGINE" \
    --message grant_role \
    --args 2 "$STABLECOIN_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ CALLER_ROLE granted${NC}"

# Grant AutoSwapExecutor → Tier 2 pools
echo "  Granting CALLER_ROLE: AutoSwapExecutor → All Tier 2 pools..."

CURRENCIES=($(jq -r '.currencies[]' "$CONFIG_FILE"))

cd ../../../primeswap/tier2/etr-wrapped-pool

for CURRENCY in "${CURRENCIES[@]}"; do
    TIER2_ADDRESS=$(jq -r ".tier2_pools.${CURRENCY}" "$ADDRESSES_FILE")

    cargo contract call \
        --contract "$TIER2_ADDRESS" \
        --message grant_role \
        --args 0 "$EXECUTOR_ADDRESS" \
        --suri "//Alice" \
        --execute \
        --skip-confirm

    echo -e "${GREEN}✓ CALLER_ROLE granted to $CURRENCY Tier 2 pool${NC}"
done

echo ""
echo -e "${YELLOW}Phase 5.6: Registering Intent Router system in Address Registry...${NC}"

cd ../../../registry/address-registry

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_two_tier_bridge_router \
    --args "$BRIDGE_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_auto_swap_executor \
    --args "$EXECUTOR_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_stablecoin_router \
    --args "$STABLECOIN_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_intent_router \
    --args "$INTENT_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ Intent Router system registered${NC}"

echo ""
echo -e "${YELLOW}Phase 5.7: Verifying Intent Router system...${NC}"

# Verify all routers registered
REGISTERED_INTENT=$(cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message get_intent_router \
    --suri "//Alice" \
    --dry-run \
    --output-json | jq -r '.data.Ok')

if [ "$REGISTERED_INTENT" == "$INTENT_ROUTER_ADDRESS" ]; then
    echo -e "${GREEN}✓ IntentRouter registered correctly${NC}"
else
    echo -e "${RED}✗ IntentRouter NOT registered${NC}"
    exit 1
fi

# Verify AutoSwapExecutor has permission to call TwoTierBridgeRouter
cd ../../intent-router/core/two-tier-bridge-router

HAS_CALLER=$(cargo contract call \
    --contract "$BRIDGE_ROUTER_ADDRESS" \
    --message has_role \
    --args 0 "$EXECUTOR_ADDRESS" \
    --suri "//Alice" \
    --dry-run \
    --output-json | jq -r '.data.Ok')

if [ "$HAS_CALLER" == "true" ]; then
    echo -e "${GREEN}✓ AutoSwapExecutor has CALLER_ROLE on TwoTierBridgeRouter${NC}"
else
    echo -e "${RED}✗ AutoSwapExecutor missing CALLER_ROLE${NC}"
    exit 1
fi

# Verify IntentRouter has permission to call AutoSwapExecutor
cd ../auto-swap-executor

HAS_CALLER=$(cargo contract call \
    --contract "$EXECUTOR_ADDRESS" \
    --message has_role \
    --args 0 "$INTENT_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --dry-run \
    --output-json | jq -r '.data.Ok')

if [ "$HAS_CALLER" == "true" ]; then
    echo -e "${GREEN}✓ IntentRouter has CALLER_ROLE on AutoSwapExecutor${NC}"
else
    echo -e "${RED}✗ IntentRouter missing CALLER_ROLE${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  PHASE 5 COMPLETE${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Deployed:"
echo "  - TwoTierBridgeRouter"
echo "  - AutoSwapExecutor"
echo "  - StablecoinRouter"
echo "  - IntentRouter"
echo ""
echo "Configured:"
echo "  - All CALLER permissions granted"
echo "  - Registered in Address Registry"
echo ""
echo "Intent Router system is operational!"
echo ""
echo "Users can now call:"
echo "  intent_router.convert_to_etr(BTC, amount) → receive ÉTR"
echo "  intent_router.convert_from_etr(ÉTR, amount, BTC) → receive BTC"
echo ""
echo "Wrapped tokens are completely hidden from users!"
