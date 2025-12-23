#!/bin/bash

################################################################################
# Phase 2: Tier 1 Reserve Pools Deployment
#
# Deploys:
#   - 11 Tier 1 ExternalCurrencyPool contracts
#   - Grants minter/burner roles to pools
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
MULTISIG=$(jq -r '.multisig_wallet' "$CONFIG_FILE")
CURRENCIES=($(jq -r '.currencies[]' "$CONFIG_FILE"))
REGISTRY_ADDRESS=$(jq -r '.address_registry' "$ADDRESSES_FILE")

echo -e "${YELLOW}Phase 2.1: Building Tier 1 Pool Contract...${NC}"

cd ../primeswap/tier1/external-currency-pool
cargo contract build --release

echo -e "${GREEN}✓ Tier 1 pool contract built${NC}"
echo ""

# Transaction limits by currency
declare -A MAX_TX_LIMITS
MAX_TX_LIMITS["BTC"]="1000000000"           # 10 BTC (8 decimals)
MAX_TX_LIMITS["ETH"]="100000000000000000000" # 100 ETH
MAX_TX_LIMITS["SOL"]="10000000000000000000000" # 10,000 SOL
MAX_TX_LIMITS["BNB"]="1000000000000000000000" # 1,000 BNB
MAX_TX_LIMITS["TRX"]="1000000000000000000000000" # 1M TRX
MAX_TX_LIMITS["XRP"]="100000000000000000000000" # 100K XRP
MAX_TX_LIMITS["ADA"]="100000000000000000000000" # 100K ADA
MAX_TX_LIMITS["DOGE"]="10000000000000000000000000" # 10M DOGE
MAX_TX_LIMITS["LINK"]="10000000000000000000000" # 10K LINK
MAX_TX_LIMITS["XLM"]="100000000000000000000000" # 100K XLM
MAX_TX_LIMITS["MATIC"]="100000000000000000000000" # 100K MATIC

declare -A DAILY_LIMITS
DAILY_LIMITS["BTC"]="10000000000"           # 100 BTC
DAILY_LIMITS["ETH"]="1000000000000000000000" # 1,000 ETH
DAILY_LIMITS["SOL"]="100000000000000000000000" # 100,000 SOL
DAILY_LIMITS["BNB"]="10000000000000000000000" # 10,000 BNB
DAILY_LIMITS["TRX"]="10000000000000000000000000" # 10M TRX
DAILY_LIMITS["XRP"]="1000000000000000000000000" # 1M XRP
DAILY_LIMITS["ADA"]="1000000000000000000000000" # 1M ADA
DAILY_LIMITS["DOGE"]="100000000000000000000000000" # 100M DOGE
DAILY_LIMITS["LINK"]="100000000000000000000000" # 100K LINK
DAILY_LIMITS["XLM"]="1000000000000000000000000" # 1M XLM
DAILY_LIMITS["MATIC"]="1000000000000000000000000" # 1M MATIC

# Full names
declare -A FULL_NAMES
FULL_NAMES["BTC"]="Bitcoin"
FULL_NAMES["ETH"]="Ethereum"
FULL_NAMES["SOL"]="Solana"
FULL_NAMES["BNB"]="BNB"
FULL_NAMES["TRX"]="Tron"
FULL_NAMES["XRP"]="Ripple"
FULL_NAMES["ADA"]="Cardano"
FULL_NAMES["DOGE"]="Dogecoin"
FULL_NAMES["LINK"]="Chainlink"
FULL_NAMES["XLM"]="Stellar"
FULL_NAMES["MATIC"]="Polygon"

echo -e "${YELLOW}Phase 2.2: Deploying Tier 1 Pools...${NC}"
echo ""

POOL_ADDRESSES=()
POOL_CURRENCIES=()

for CURRENCY in "${CURRENCIES[@]}"; do
    echo -e "${YELLOW}Deploying Tier 1 pool for $CURRENCY...${NC}"

    # Get wrapped token address
    WRAPPED_TOKEN=$(jq -r ".wrapped_tokens.w${CURRENCY}" "$ADDRESSES_FILE")

    if [ "$WRAPPED_TOKEN" == "null" ]; then
        echo -e "${RED}ERROR: Wrapped token w${CURRENCY} not found in addresses file${NC}"
        exit 1
    fi

    NAME="${FULL_NAMES[$CURRENCY]}"
    MAX_TX="${MAX_TX_LIMITS[$CURRENCY]}"
    DAILY_LIMIT="${DAILY_LIMITS[$CURRENCY]}"

    # Deploy Tier 1 pool
    # Note: tier2_pool will be set to zero address and updated in Phase 3
    ZERO_ADDRESS="5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM"

    POOL_ADDRESS=$(cargo contract instantiate \
        --constructor new \
        --args "$NAME" "$WRAPPED_TOKEN" "$ZERO_ADDRESS" "$MULTISIG" "$MAX_TX" "$DAILY_LIMIT" \
        --suri "//Alice" \
        --execute \
        --skip-confirm \
        --output-json | jq -r '.contract')

    echo -e "${GREEN}✓ Tier 1 $CURRENCY pool deployed: $POOL_ADDRESS${NC}"

    # Save to addresses file
    jq --arg curr "$CURRENCY" --arg addr "$POOL_ADDRESS" \
        '.tier1_pools[$curr] = $addr' \
        "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

    POOL_ADDRESSES+=("$POOL_ADDRESS")
    POOL_CURRENCIES+=("$CURRENCY")

    echo ""
done

echo -e "${YELLOW}Phase 2.3: Granting roles to Tier 1 pools...${NC}"
echo ""

# Grant MINTER_ROLE and BURNER_ROLE to each pool
cd ../../wrapped-tokens/wrapped-token-template

for i in "${!POOL_CURRENCIES[@]}"; do
    CURRENCY="${POOL_CURRENCIES[$i]}"
    POOL_ADDRESS="${POOL_ADDRESSES[$i]}"
    WRAPPED_TOKEN=$(jq -r ".wrapped_tokens.w${CURRENCY}" "$ADDRESSES_FILE")

    echo -e "${YELLOW}Granting roles to $CURRENCY pool...${NC}"

    # Grant MINTER_ROLE (role ID = 0)
    cargo contract call \
        --contract "$WRAPPED_TOKEN" \
        --message grant_role \
        --args 0 "$POOL_ADDRESS" \
        --suri "//Alice" \
        --execute \
        --skip-confirm

    echo -e "${GREEN}✓ MINTER_ROLE granted${NC}"

    # Grant BURNER_ROLE (role ID = 1)
    cargo contract call \
        --contract "$WRAPPED_TOKEN" \
        --message grant_role \
        --args 1 "$POOL_ADDRESS" \
        --suri "//Alice" \
        --execute \
        --skip-confirm

    echo -e "${GREEN}✓ BURNER_ROLE granted${NC}"
    echo ""
done

echo -e "${YELLOW}Phase 2.4: Registering Tier 1 pools in Address Registry...${NC}"

cd ../../../registry/address-registry

# Build JSON arrays for batch registration
CURRENCIES_JSON=$(printf '%s\n' "${POOL_CURRENCIES[@]}" | jq -R . | jq -s .)
ADDRESSES_JSON=$(printf '%s\n' "${POOL_ADDRESSES[@]}" | jq -R . | jq -s .)

# Call register_all_tier1_pools
cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_all_tier1_pools \
    --args "$CURRENCIES_JSON" "$ADDRESSES_JSON" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ All Tier 1 pools registered${NC}"
echo ""

echo -e "${YELLOW}Phase 2.5: Verifying Tier 1 pools...${NC}"
echo ""

for CURRENCY in "${POOL_CURRENCIES[@]}"; do
    # Verify registry
    REGISTERED_ADDR=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_tier1_pool \
        --args "$CURRENCY" \
        --suri "//Alice" \
        --dry-run \
        --output-json | jq -r '.data.Ok')

    if [ "$REGISTERED_ADDR" != "null" ]; then
        echo -e "${GREEN}✓ $CURRENCY Tier 1 pool registered correctly${NC}"
    else
        echo -e "${RED}✗ $CURRENCY Tier 1 pool NOT registered${NC}"
        exit 1
    fi

    # Verify minter role
    WRAPPED_TOKEN=$(jq -r ".wrapped_tokens.w${CURRENCY}" "$ADDRESSES_FILE")
    POOL_ADDRESS=$(jq -r ".tier1_pools.${CURRENCY}" "$ADDRESSES_FILE")

    cd ../../primeswap/wrapped-tokens/wrapped-token-template

    HAS_ROLE=$(cargo contract call \
        --contract "$WRAPPED_TOKEN" \
        --message has_role \
        --args 0 "$POOL_ADDRESS" \
        --suri "//Alice" \
        --dry-run \
        --output-json | jq -r '.data.Ok')

    if [ "$HAS_ROLE" == "true" ]; then
        echo -e "${GREEN}✓ $CURRENCY pool has MINTER_ROLE${NC}"
    else
        echo -e "${RED}✗ $CURRENCY pool missing MINTER_ROLE${NC}"
        exit 1
    fi

    cd ../../../registry/address-registry
done

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  PHASE 2 COMPLETE${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Deployed:"
echo "  - 11 Tier 1 Reserve Pools"
echo "  - Granted minter/burner roles"
echo "  - Registered in Address Registry"
echo ""
echo "Next: Phase 3 will deploy Tier 2 pools and wire Tier 1 → Tier 2"
