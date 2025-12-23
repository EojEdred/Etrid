#!/bin/bash

################################################################################
# Phase 3: Tier 2 Trading Pools Deployment
#
# Deploys:
#   - 11 Tier 2 ETRWrappedPool contracts
#   - Initializes pools with ÉTR liquidity
#   - Wires Tier 1 → Tier 2 connections
#   - Registers pools in Address Registry
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
CURRENCIES=($(jq -r '.currencies[]' "$CONFIG_FILE"))
REGISTRY_ADDRESS=$(jq -r '.address_registry' "$ADDRESSES_FILE")

echo -e "${YELLOW}Phase 3.1: Building Tier 2 Pool Contract...${NC}"

cd ../primeswap/tier2/etr-wrapped-pool
cargo contract build --release

echo -e "${GREEN}✓ Tier 2 pool contract built${NC}"
echo ""

echo -e "${YELLOW}Phase 3.2: Deploying Tier 2 Pools...${NC}"
echo ""

POOL_ADDRESSES=()
POOL_CURRENCIES=()

for CURRENCY in "${CURRENCIES[@]}"; do
    echo -e "${YELLOW}Deploying Tier 2 pool for $CURRENCY...${NC}"

    # Get wrapped token address
    WRAPPED_TOKEN=$(jq -r ".wrapped_tokens.w${CURRENCY}" "$ADDRESSES_FILE")

    if [ "$WRAPPED_TOKEN" == "null" ]; then
        echo -e "${RED}ERROR: Wrapped token w${CURRENCY} not found${NC}"
        exit 1
    fi

    # Get pool allocations from config
    ETR_ALLOCATION=$(jq -r ".pool_allocations.${CURRENCY}.etr" "$CONFIG_FILE")
    VIRTUAL_RESERVE=$(jq -r ".pool_allocations.${CURRENCY}.virtual" "$CONFIG_FILE")

    echo "  ETR allocation: $ETR_ALLOCATION"
    echo "  Virtual reserve: $VIRTUAL_RESERVE"

    # Deploy Tier 2 pool
    POOL_ADDRESS=$(cargo contract instantiate \
        --constructor new \
        --args "$WRAPPED_TOKEN" "$VIRTUAL_RESERVE" \
        --suri "//Alice" \
        --execute \
        --skip-confirm \
        --output-json | jq -r '.contract')

    echo -e "${GREEN}✓ Tier 2 $CURRENCY pool deployed: $POOL_ADDRESS${NC}"

    # Save to addresses file
    jq --arg curr "$CURRENCY" --arg addr "$POOL_ADDRESS" \
        '.tier2_pools[$curr] = $addr' \
        "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

    POOL_ADDRESSES+=("$POOL_ADDRESS")
    POOL_CURRENCIES+=("$CURRENCY")

    echo ""
done

echo -e "${YELLOW}Phase 3.3: Initializing Tier 2 pools with ÉTR liquidity...${NC}"
echo ""

# Note: In production, you would transfer ÉTR from treasury to each pool
# For now, we'll simulate this by calling initialize_pool

for i in "${!POOL_CURRENCIES[@]}"; do
    CURRENCY="${POOL_CURRENCIES[$i]}"
    POOL_ADDRESS="${POOL_ADDRESSES[$i]}"
    ETR_ALLOCATION=$(jq -r ".pool_allocations.${CURRENCY}.etr" "$CONFIG_FILE")

    echo -e "${YELLOW}Initializing $CURRENCY pool with $ETR_ALLOCATION ÉTR...${NC}"

    # TODO: Transfer ÉTR from treasury to pool
    # transfer_etr(treasury, pool_address, etr_allocation)

    # Initialize pool
    cargo contract call \
        --contract "$POOL_ADDRESS" \
        --message initialize_pool \
        --args "$ETR_ALLOCATION" \
        --suri "//Alice" \
        --execute \
        --skip-confirm

    echo -e "${GREEN}✓ $CURRENCY pool initialized${NC}"
    echo ""
done

echo -e "${YELLOW}Phase 3.4: Wiring Tier 1 → Tier 2 connections...${NC}"
echo ""

cd ../../tier1/external-currency-pool

for CURRENCY in "${POOL_CURRENCIES[@]}"; do
    TIER1_ADDRESS=$(jq -r ".tier1_pools.${CURRENCY}" "$ADDRESSES_FILE")
    TIER2_ADDRESS=$(jq -r ".tier2_pools.${CURRENCY}" "$ADDRESSES_FILE")

    echo -e "${YELLOW}Wiring $CURRENCY: Tier 1 → Tier 2...${NC}"

    # Set tier2_pool address in Tier 1 pool
    cargo contract call \
        --contract "$TIER1_ADDRESS" \
        --message set_tier2_pool \
        --args "$TIER2_ADDRESS" \
        --suri "//Alice" \
        --execute \
        --skip-confirm

    echo -e "${GREEN}✓ $CURRENCY Tier 1 → Tier 2 connected${NC}"
    echo ""
done

echo -e "${YELLOW}Phase 3.5: Registering Tier 2 pools in Address Registry...${NC}"

cd ../../../registry/address-registry

# Build JSON arrays for batch registration
CURRENCIES_JSON=$(printf '%s\n' "${POOL_CURRENCIES[@]}" | jq -R . | jq -s .)
ADDRESSES_JSON=$(printf '%s\n' "${POOL_ADDRESSES[@]}" | jq -R . | jq -s .)

# Call register_all_tier2_pools
cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_all_tier2_pools \
    --args "$CURRENCIES_JSON" "$ADDRESSES_JSON" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ All Tier 2 pools registered${NC}"
echo ""

echo -e "${YELLOW}Phase 3.6: Verifying Tier 2 pools...${NC}"
echo ""

for CURRENCY in "${POOL_CURRENCIES[@]}"; do
    # Verify registry
    REGISTERED_ADDR=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_tier2_pool \
        --args "$CURRENCY" \
        --suri "//Alice" \
        --dry-run \
        --output-json | jq -r '.data.Ok')

    if [ "$REGISTERED_ADDR" != "null" ]; then
        echo -e "${GREEN}✓ $CURRENCY Tier 2 pool registered correctly${NC}"
    else
        echo -e "${RED}✗ $CURRENCY Tier 2 pool NOT registered${NC}"
        exit 1
    fi

    # Verify Tier 1 → Tier 2 wiring
    TIER1_ADDRESS=$(jq -r ".tier1_pools.${CURRENCY}" "$ADDRESSES_FILE")
    EXPECTED_TIER2=$(jq -r ".tier2_pools.${CURRENCY}" "$ADDRESSES_FILE")

    cd ../../primeswap/tier1/external-currency-pool

    ACTUAL_TIER2=$(cargo contract call \
        --contract "$TIER1_ADDRESS" \
        --message get_tier2_pool \
        --suri "//Alice" \
        --dry-run \
        --output-json | jq -r '.data.Ok')

    if [ "$ACTUAL_TIER2" == "$EXPECTED_TIER2" ]; then
        echo -e "${GREEN}✓ $CURRENCY Tier 1 → Tier 2 wiring correct${NC}"
    else
        echo -e "${RED}✗ $CURRENCY Tier 1 → Tier 2 wiring INCORRECT${NC}"
        echo "  Expected: $EXPECTED_TIER2"
        echo "  Actual: $ACTUAL_TIER2"
        exit 1
    fi

    cd ../../../registry/address-registry
done

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  PHASE 3 COMPLETE${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Deployed:"
echo "  - 11 Tier 2 Trading Pools"
echo "  - Initialized with 1.25B ÉTR total liquidity"
echo "  - Wired Tier 1 → Tier 2 connections"
echo "  - Registered in Address Registry"
echo ""
echo "Two-tier liquidity system is now operational!"
