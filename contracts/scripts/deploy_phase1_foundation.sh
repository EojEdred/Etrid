#!/bin/bash

################################################################################
# Phase 1: Foundation Layer Deployment
#
# Deploys:
#   1. Address Registry (central hub)
#   2. 11 Wrapped Token contracts (wBTC, wETH, etc.)
################################################################################

set -e

NETWORK=$1
ADDRESSES_FILE=$2
CONFIG_FILE=$3

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Phase 1.1: Deploying Address Registry...${NC}"

# Build Address Registry
cd ../registry/address-registry
cargo contract build --release

# Deploy Address Registry
REGISTRY_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ Address Registry deployed: $REGISTRY_ADDRESS${NC}"

# Save to addresses file
jq --arg addr "$REGISTRY_ADDRESS" '.address_registry = $addr' "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 1.2: Deploying Wrapped Tokens...${NC}"

# List of currencies
CURRENCIES=("BTC" "ETH" "SOL" "BNB" "TRX" "XRP" "ADA" "DOGE" "LINK" "XLM" "MATIC")

# Decimals for each currency
declare -A DECIMALS
DECIMALS["BTC"]=8
DECIMALS["ETH"]=18
DECIMALS["SOL"]=18
DECIMALS["BNB"]=18
DECIMALS["TRX"]=18
DECIMALS["XRP"]=18
DECIMALS["ADA"]=18
DECIMALS["DOGE"]=18
DECIMALS["LINK"]=18
DECIMALS["XLM"]=18
DECIMALS["MATIC"]=18

# Full names
declare -A NAMES
NAMES["BTC"]="Wrapped Bitcoin"
NAMES["ETH"]="Wrapped Ethereum"
NAMES["SOL"]="Wrapped Solana"
NAMES["BNB"]="Wrapped BNB"
NAMES["TRX"]="Wrapped Tron"
NAMES["XRP"]="Wrapped Ripple"
NAMES["ADA"]="Wrapped Cardano"
NAMES["DOGE"]="Wrapped Dogecoin"
NAMES["LINK"]="Wrapped Chainlink"
NAMES["XLM"]="Wrapped Stellar"
NAMES["MATIC"]="Wrapped Polygon"

# Build wrapped token template
cd ../../primeswap/wrapped-tokens/wrapped-token-template
cargo contract build --release

# Arrays to store addresses for batch registration
SYMBOLS=()
ADDRESSES=()

# Deploy each wrapped token
for CURRENCY in "${CURRENCIES[@]}"; do
    SYMBOL="w${CURRENCY}"
    NAME="${NAMES[$CURRENCY]}"
    DECIMAL="${DECIMALS[$CURRENCY]}"

    echo -e "${YELLOW}Deploying $SYMBOL...${NC}"

    # Deploy wrapped token
    TOKEN_ADDRESS=$(cargo contract instantiate \
        --constructor new \
        --args "$NAME" "$SYMBOL" "$DECIMAL" \
        --suri "//Alice" \
        --execute \
        --skip-confirm \
        --output-json | jq -r '.contract')

    echo -e "${GREEN}✓ $SYMBOL deployed: $TOKEN_ADDRESS${NC}"

    # Save to addresses file
    jq --arg symbol "$SYMBOL" --arg addr "$TOKEN_ADDRESS" \
        '.wrapped_tokens[$symbol] = $addr' \
        "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

    # Add to batch arrays
    SYMBOLS+=("$SYMBOL")
    ADDRESSES+=("$TOKEN_ADDRESS")
done

echo ""
echo -e "${YELLOW}Phase 1.3: Registering wrapped tokens in Address Registry...${NC}"

# Build JSON arrays for batch call
SYMBOLS_JSON=$(printf '%s\n' "${SYMBOLS[@]}" | jq -R . | jq -s .)
ADDRESSES_JSON=$(printf '%s\n' "${ADDRESSES[@]}" | jq -R . | jq -s .)

# Call register_all_wrapped_tokens
cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_all_wrapped_tokens \
    --args "$SYMBOLS_JSON" "$ADDRESSES_JSON" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ All wrapped tokens registered${NC}"

echo ""
echo -e "${YELLOW}Phase 1.4: Verifying Address Registry...${NC}"

# Verify each token is registered
for SYMBOL in "${SYMBOLS[@]}"; do
    REGISTERED_ADDR=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_wrapped_token \
        --args "$SYMBOL" \
        --suri "//Alice" \
        --dry-run \
        --output-json | jq -r '.data.Ok')

    if [ "$REGISTERED_ADDR" != "null" ]; then
        echo -e "${GREEN}✓ $SYMBOL registered correctly${NC}"
    else
        echo -e "${RED}✗ $SYMBOL NOT registered${NC}"
        exit 1
    fi
done

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  PHASE 1 COMPLETE${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Deployed:"
echo "  - 1 Address Registry"
echo "  - 11 Wrapped Tokens"
echo ""
echo "Addresses saved to: $ADDRESSES_FILE"
