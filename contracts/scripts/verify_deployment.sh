#!/bin/bash

################################################################################
# Deployment Verification Script
#
# Performs comprehensive verification of all deployed contracts:
#   - Checks all contracts are deployed
#   - Verifies Address Registry has all addresses
#   - Checks all roles are granted correctly
#   - Verifies wiring (Tier 1 → Tier 2, routers, etc.)
#   - Tests basic functionality
################################################################################

set -e

NETWORK=$1
ADDRESSES_FILE=$2

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

ERRORS=0
WARNINGS=0

# Print header
echo -e "${BLUE}"
echo "═══════════════════════════════════════════════════════════════"
echo "  ĒTRID DEPLOYMENT VERIFICATION"
echo "═══════════════════════════════════════════════════════════════"
echo -e "${NC}"
echo "Network: $NETWORK"
echo "Addresses file: $ADDRESSES_FILE"
echo ""

# Load addresses
REGISTRY_ADDRESS=$(jq -r '.address_registry' "$ADDRESSES_FILE")

if [ "$REGISTRY_ADDRESS" == "null" ]; then
    echo -e "${RED}ERROR: Address Registry not found in addresses file${NC}"
    exit 1
fi

echo "Address Registry: $REGISTRY_ADDRESS"
echo ""

# Verification functions
check_wrapped_tokens() {
    echo -e "${YELLOW}Verifying Wrapped Tokens...${NC}"

    CURRENCIES=("BTC" "ETH" "SOL" "BNB" "TRX" "XRP" "ADA" "DOGE" "LINK" "XLM" "MATIC")

    cd ../registry/address-registry

    for CURRENCY in "${CURRENCIES[@]}"; do
        SYMBOL="w${CURRENCY}"

        # Check in addresses file
        FILE_ADDR=$(jq -r ".wrapped_tokens.${SYMBOL}" "$ADDRESSES_FILE")
        if [ "$FILE_ADDR" == "null" ]; then
            echo -e "${RED}✗ $SYMBOL not in addresses file${NC}"
            ((ERRORS++))
            continue
        fi

        # Check in registry
        REGISTRY_ADDR=$(cargo contract call \
            --contract "$REGISTRY_ADDRESS" \
            --message get_wrapped_token \
            --args "$SYMBOL" \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

        if [ "$REGISTRY_ADDR" == "$FILE_ADDR" ]; then
            echo -e "${GREEN}✓ $SYMBOL verified${NC}"
        else
            echo -e "${RED}✗ $SYMBOL mismatch (file: $FILE_ADDR, registry: $REGISTRY_ADDR)${NC}"
            ((ERRORS++))
        fi
    done

    echo ""
}

check_tier1_pools() {
    echo -e "${YELLOW}Verifying Tier 1 Pools...${NC}"

    CURRENCIES=("BTC" "ETH" "SOL" "BNB" "TRX" "XRP" "ADA" "DOGE" "LINK" "XLM" "MATIC")

    cd ../registry/address-registry

    for CURRENCY in "${CURRENCIES[@]}"; do
        # Check in addresses file
        FILE_ADDR=$(jq -r ".tier1_pools.${CURRENCY}" "$ADDRESSES_FILE")
        if [ "$FILE_ADDR" == "null" ]; then
            echo -e "${RED}✗ $CURRENCY Tier 1 pool not in addresses file${NC}"
            ((ERRORS++))
            continue
        fi

        # Check in registry
        REGISTRY_ADDR=$(cargo contract call \
            --contract "$REGISTRY_ADDRESS" \
            --message get_tier1_pool \
            --args "$CURRENCY" \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

        if [ "$REGISTRY_ADDR" == "$FILE_ADDR" ]; then
            echo -e "${GREEN}✓ $CURRENCY Tier 1 pool verified${NC}"
        else
            echo -e "${RED}✗ $CURRENCY Tier 1 pool mismatch${NC}"
            ((ERRORS++))
        fi

        # Check minter role
        WRAPPED_TOKEN=$(jq -r ".wrapped_tokens.w${CURRENCY}" "$ADDRESSES_FILE")

        cd ../../primeswap/wrapped-tokens/wrapped-token-template

        HAS_MINTER=$(cargo contract call \
            --contract "$WRAPPED_TOKEN" \
            --message has_role \
            --args 0 "$FILE_ADDR" \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

        if [ "$HAS_MINTER" == "true" ]; then
            echo -e "${GREEN}  └─ MINTER_ROLE granted${NC}"
        else
            echo -e "${RED}  └─ MINTER_ROLE missing${NC}"
            ((ERRORS++))
        fi

        cd ../../../registry/address-registry
    done

    echo ""
}

check_tier2_pools() {
    echo -e "${YELLOW}Verifying Tier 2 Pools...${NC}"

    CURRENCIES=("BTC" "ETH" "SOL" "BNB" "TRX" "XRP" "ADA" "DOGE" "LINK" "XLM" "MATIC")

    cd ../registry/address-registry

    for CURRENCY in "${CURRENCIES[@]}"; do
        # Check in addresses file
        TIER2_ADDR=$(jq -r ".tier2_pools.${CURRENCY}" "$ADDRESSES_FILE")
        if [ "$TIER2_ADDR" == "null" ]; then
            echo -e "${RED}✗ $CURRENCY Tier 2 pool not in addresses file${NC}"
            ((ERRORS++))
            continue
        fi

        # Check in registry
        REGISTRY_ADDR=$(cargo contract call \
            --contract "$REGISTRY_ADDRESS" \
            --message get_tier2_pool \
            --args "$CURRENCY" \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

        if [ "$REGISTRY_ADDR" == "$TIER2_ADDR" ]; then
            echo -e "${GREEN}✓ $CURRENCY Tier 2 pool verified${NC}"
        else
            echo -e "${RED}✗ $CURRENCY Tier 2 pool mismatch${NC}"
            ((ERRORS++))
        fi

        # Check Tier 1 → Tier 2 wiring
        TIER1_ADDR=$(jq -r ".tier1_pools.${CURRENCY}" "$ADDRESSES_FILE")

        cd ../../primeswap/tier1/external-currency-pool

        WIRED_TIER2=$(cargo contract call \
            --contract "$TIER1_ADDR" \
            --message get_tier2_pool \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

        if [ "$WIRED_TIER2" == "$TIER2_ADDR" ]; then
            echo -e "${GREEN}  └─ Tier 1 → Tier 2 wiring correct${NC}"
        else
            echo -e "${RED}  └─ Tier 1 → Tier 2 wiring INCORRECT${NC}"
            ((ERRORS++))
        fi

        cd ../../../registry/address-registry
    done

    echo ""
}

check_edsc_system() {
    echo -e "${YELLOW}Verifying EDSC System...${NC}"

    cd ../registry/address-registry

    # Check EDSCToken
    TOKEN_ADDR=$(jq -r '.edsc_system.edsc_token' "$ADDRESSES_FILE")
    REGISTRY_TOKEN=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_edsc_token \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

    if [ "$REGISTRY_TOKEN" == "$TOKEN_ADDR" ]; then
        echo -e "${GREEN}✓ EDSCToken verified${NC}"
    else
        echo -e "${RED}✗ EDSCToken mismatch${NC}"
        ((ERRORS++))
    fi

    # Check EDSCReserveVault
    VAULT_ADDR=$(jq -r '.edsc_system.edsc_reserve_vault' "$ADDRESSES_FILE")
    REGISTRY_VAULT=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_edsc_reserve_vault \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

    if [ "$REGISTRY_VAULT" == "$VAULT_ADDR" ]; then
        echo -e "${GREEN}✓ EDSCReserveVault verified${NC}"
    else
        echo -e "${RED}✗ EDSCReserveVault mismatch${NC}"
        ((ERRORS++))
    fi

    # Check EDSCMintingEngine
    ENGINE_ADDR=$(jq -r '.edsc_system.edsc_minting_engine' "$ADDRESSES_FILE")
    REGISTRY_ENGINE=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_edsc_minting_engine \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

    if [ "$REGISTRY_ENGINE" == "$ENGINE_ADDR" ]; then
        echo -e "${GREEN}✓ EDSCMintingEngine verified${NC}"
    else
        echo -e "${RED}✗ EDSCMintingEngine mismatch${NC}"
        ((ERRORS++))
    fi

    # Check minter role
    cd ../../edsc/core/edsc-token

    HAS_MINTER=$(cargo contract call \
        --contract "$TOKEN_ADDR" \
        --message has_role \
        --args 0 "$ENGINE_ADDR" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_MINTER" == "true" ]; then
        echo -e "${GREEN}  └─ Minting engine has MINTER_ROLE${NC}"
    else
        echo -e "${RED}  └─ Minting engine missing MINTER_ROLE${NC}"
        ((ERRORS++))
    fi

    # Check reserve ratio
    cd ../reserve-vault

    RESERVE_RATIO=$(cargo contract call \
        --contract "$VAULT_ADDR" \
        --message get_reserve_ratio \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "0")

    if [ "$RESERVE_RATIO" -ge "99" ]; then
        echo -e "${GREEN}  └─ Reserve ratio: ${RESERVE_RATIO}% (adequate)${NC}"
    else
        echo -e "${YELLOW}  └─ Reserve ratio: ${RESERVE_RATIO}% (below 99%)${NC}"
        ((WARNINGS++))
    fi

    echo ""
}

check_intent_router() {
    echo -e "${YELLOW}Verifying Intent Router System...${NC}"

    cd ../../../registry/address-registry

    # Check IntentRouter
    ROUTER_ADDR=$(jq -r '.intent_router_system.intent_router' "$ADDRESSES_FILE")
    REGISTRY_ROUTER=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_intent_router \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

    if [ "$REGISTRY_ROUTER" == "$ROUTER_ADDR" ]; then
        echo -e "${GREEN}✓ IntentRouter verified${NC}"
    else
        echo -e "${RED}✗ IntentRouter mismatch${NC}"
        ((ERRORS++))
    fi

    # Check AutoSwapExecutor
    EXECUTOR_ADDR=$(jq -r '.intent_router_system.auto_swap_executor' "$ADDRESSES_FILE")
    REGISTRY_EXECUTOR=$(cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message get_auto_swap_executor \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "null")

    if [ "$REGISTRY_EXECUTOR" == "$EXECUTOR_ADDR" ]; then
        echo -e "${GREEN}✓ AutoSwapExecutor verified${NC}"
    else
        echo -e "${RED}✗ AutoSwapExecutor mismatch${NC}"
        ((ERRORS++))
    fi

    # Check CALLER permissions
    cd ../../intent-router/core/auto-swap-executor

    HAS_CALLER=$(cargo contract call \
        --contract "$EXECUTOR_ADDR" \
        --message has_role \
        --args 0 "$ROUTER_ADDR" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_CALLER" == "true" ]; then
        echo -e "${GREEN}  └─ IntentRouter has CALLER_ROLE on AutoSwapExecutor${NC}"
    else
        echo -e "${RED}  └─ IntentRouter missing CALLER_ROLE${NC}"
        ((ERRORS++))
    fi

    echo ""
}

check_contract_balances() {
    echo -e "${YELLOW}Checking Contract Balances...${NC}"

    CURRENCIES=("BTC" "ETH" "SOL" "BNB" "TRX" "XRP" "ADA" "DOGE" "LINK" "XLM" "MATIC")

    cd ../../../primeswap/tier2/etr-wrapped-pool

    for CURRENCY in "${CURRENCIES[@]}"; do
        TIER2_ADDR=$(jq -r ".tier2_pools.${CURRENCY}" "$ADDRESSES_FILE")

        # Check ETR reserves
        ETR_RESERVE=$(cargo contract call \
            --contract "$TIER2_ADDR" \
            --message get_reserves \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok.etr_reserve' || echo "0")

        if [ "$ETR_RESERVE" != "0" ]; then
            echo -e "${GREEN}✓ $CURRENCY Tier 2 pool has ÉTR liquidity${NC}"
        else
            echo -e "${YELLOW}⚠ $CURRENCY Tier 2 pool has ZERO ÉTR liquidity${NC}"
            ((WARNINGS++))
        fi
    done

    echo ""
}

# Run all checks
check_wrapped_tokens
check_tier1_pools
check_tier2_pools
check_edsc_system
check_intent_router
check_contract_balances

# Summary
echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  VERIFICATION SUMMARY${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED${NC}"
    echo ""
    echo "Deployment is VERIFIED and ready for use!"
    echo ""
    echo "Next steps:"
    echo "  1. Transfer admin roles to multi-sig wallet"
    echo "  2. Run integration tests"
    echo "  3. Monitor contracts for 24 hours"
    echo "  4. Announce launch to users"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠ PASSED WITH WARNINGS${NC}"
    echo ""
    echo "Errors: $ERRORS"
    echo "Warnings: $WARNINGS"
    echo ""
    echo "Deployment is functional but has warnings."
    echo "Review warnings before production launch."
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED${NC}"
    echo ""
    echo "Errors: $ERRORS"
    echo "Warnings: $WARNINGS"
    echo ""
    echo "DO NOT PROCEED TO PRODUCTION."
    echo "Fix all errors and re-run verification."
    exit 1
fi
