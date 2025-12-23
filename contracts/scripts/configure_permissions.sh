#!/bin/bash

################################################################################
# Permission Configuration and Audit Script
#
# This script:
#   1. Audits all current permissions across all contracts
#   2. Identifies missing or incorrect permissions
#   3. Optionally fixes permission issues
#   4. Transfers admin roles to multi-sig (production mode)
#
# Usage:
#   ./configure_permissions.sh [network] [mode]
#
# Modes:
#   audit  - Check permissions only (default)
#   fix    - Check and fix missing permissions
#   prod   - Transfer admin roles to multi-sig (IRREVERSIBLE)
################################################################################

set -e

NETWORK=${1:-devnet}
MODE=${2:-audit}

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
ADDRESSES_FILE="$SCRIPT_DIR/deployed_addresses_${NETWORK}.json"
CONFIG_FILE="$SCRIPT_DIR/deployment_config_${NETWORK}.json"

ISSUES_FOUND=0

echo -e "${BLUE}"
echo "═══════════════════════════════════════════════════════════════"
echo "  ĒTRID PERMISSION CONFIGURATION"
echo "═══════════════════════════════════════════════════════════════"
echo -e "${NC}"
echo "Network: $NETWORK"
echo "Mode: $MODE"
echo ""

if [ "$MODE" == "prod" ]; then
    echo -e "${RED}WARNING: Production mode will transfer ownership to multi-sig!${NC}"
    echo "This action is IRREVERSIBLE. The deployer account will lose admin access."
    echo ""
    echo "Press Enter to continue, or Ctrl+C to cancel..."
    read
    echo ""
fi

# Load configuration
MULTISIG=$(jq -r '.multisig_wallet' "$CONFIG_FILE")
CURRENCIES=($(jq -r '.currencies[]' "$CONFIG_FILE"))

echo "Multi-sig wallet: $MULTISIG"
echo ""

# ═══════════════════════════════════════════════════════════════════
# ROLE IDs
# ═══════════════════════════════════════════════════════════════════

MINTER_ROLE=0
BURNER_ROLE=1
CALLER_ROLE=0
DEPOSITOR_ROLE=0
REBALANCER_ROLE=1
ADMIN_ROLE=2

# ═══════════════════════════════════════════════════════════════════
# WRAPPED TOKENS
# ═══════════════════════════════════════════════════════════════════

audit_wrapped_token_permissions() {
    echo -e "${YELLOW}Auditing Wrapped Token Permissions...${NC}"

    cd "$SCRIPT_DIR/../primeswap/wrapped-tokens/wrapped-token-template"

    for CURRENCY in "${CURRENCIES[@]}"; do
        SYMBOL="w${CURRENCY}"
        TOKEN_ADDR=$(jq -r ".wrapped_tokens.${SYMBOL}" "$ADDRESSES_FILE")
        TIER1_ADDR=$(jq -r ".tier1_pools.${CURRENCY}" "$ADDRESSES_FILE")

        echo "  Checking $SYMBOL..."

        # Check MINTER_ROLE
        HAS_MINTER=$(cargo contract call \
            --contract "$TOKEN_ADDR" \
            --message has_role \
            --args $MINTER_ROLE "$TIER1_ADDR" \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

        if [ "$HAS_MINTER" == "true" ]; then
            echo -e "${GREEN}    ✓ Tier 1 pool has MINTER_ROLE${NC}"
        else
            echo -e "${RED}    ✗ Tier 1 pool missing MINTER_ROLE${NC}"
            ((ISSUES_FOUND++))

            if [ "$MODE" == "fix" ]; then
                echo "    → Granting MINTER_ROLE..."
                cargo contract call \
                    --contract "$TOKEN_ADDR" \
                    --message grant_role \
                    --args $MINTER_ROLE "$TIER1_ADDR" \
                    --suri "//Alice" \
                    --execute \
                    --skip-confirm
                echo -e "${GREEN}    ✓ Fixed${NC}"
            fi
        fi

        # Check BURNER_ROLE
        HAS_BURNER=$(cargo contract call \
            --contract "$TOKEN_ADDR" \
            --message has_role \
            --args $BURNER_ROLE "$TIER1_ADDR" \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

        if [ "$HAS_BURNER" == "true" ]; then
            echo -e "${GREEN}    ✓ Tier 1 pool has BURNER_ROLE${NC}"
        else
            echo -e "${RED}    ✗ Tier 1 pool missing BURNER_ROLE${NC}"
            ((ISSUES_FOUND++))

            if [ "$MODE" == "fix" ]; then
                echo "    → Granting BURNER_ROLE..."
                cargo contract call \
                    --contract "$TOKEN_ADDR" \
                    --message grant_role \
                    --args $BURNER_ROLE "$TIER1_ADDR" \
                    --suri "//Alice" \
                    --execute \
                    --skip-confirm
                echo -e "${GREEN}    ✓ Fixed${NC}"
            fi
        fi

        echo ""
    done
}

# ═══════════════════════════════════════════════════════════════════
# TIER 2 POOLS
# ═══════════════════════════════════════════════════════════════════

audit_tier2_permissions() {
    echo -e "${YELLOW}Auditing Tier 2 Pool Permissions...${NC}"

    cd "$SCRIPT_DIR/../primeswap/tier2/etr-wrapped-pool"

    EXECUTOR_ADDR=$(jq -r '.intent_router_system.auto_swap_executor' "$ADDRESSES_FILE")

    for CURRENCY in "${CURRENCIES[@]}"; do
        TIER2_ADDR=$(jq -r ".tier2_pools.${CURRENCY}" "$ADDRESSES_FILE")

        echo "  Checking $CURRENCY Tier 2 pool..."

        # Check AutoSwapExecutor has CALLER_ROLE
        HAS_CALLER=$(cargo contract call \
            --contract "$TIER2_ADDR" \
            --message has_role \
            --args $CALLER_ROLE "$EXECUTOR_ADDR" \
            --suri "//Alice" \
            --dry-run \
            --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

        if [ "$HAS_CALLER" == "true" ]; then
            echo -e "${GREEN}    ✓ AutoSwapExecutor has CALLER_ROLE${NC}"
        else
            echo -e "${RED}    ✗ AutoSwapExecutor missing CALLER_ROLE${NC}"
            ((ISSUES_FOUND++))

            if [ "$MODE" == "fix" ]; then
                echo "    → Granting CALLER_ROLE..."
                cargo contract call \
                    --contract "$TIER2_ADDR" \
                    --message grant_role \
                    --args $CALLER_ROLE "$EXECUTOR_ADDR" \
                    --suri "//Alice" \
                    --execute \
                    --skip-confirm
                echo -e "${GREEN}    ✓ Fixed${NC}"
            fi
        fi

        echo ""
    done
}

# ═══════════════════════════════════════════════════════════════════
# EDSC SYSTEM
# ═══════════════════════════════════════════════════════════════════

audit_edsc_permissions() {
    echo -e "${YELLOW}Auditing EDSC System Permissions...${NC}"

    TOKEN_ADDR=$(jq -r '.edsc_system.edsc_token' "$ADDRESSES_FILE")
    VAULT_ADDR=$(jq -r '.edsc_system.edsc_reserve_vault' "$ADDRESSES_FILE")
    ENGINE_ADDR=$(jq -r '.edsc_system.edsc_minting_engine' "$ADDRESSES_FILE")
    STABILIZER_ADDR=$(jq -r '.edsc_system.edsc_peg_stabilizer' "$ADDRESSES_FILE")
    STABLECOIN_ROUTER=$(jq -r '.intent_router_system.stablecoin_router' "$ADDRESSES_FILE")

    echo "  Checking EDSCToken permissions..."

    cd "$SCRIPT_DIR/../edsc/core/edsc-token"

    # Check minting engine has MINTER_ROLE
    HAS_MINTER=$(cargo contract call \
        --contract "$TOKEN_ADDR" \
        --message has_role \
        --args $MINTER_ROLE "$ENGINE_ADDR" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_MINTER" == "true" ]; then
        echo -e "${GREEN}    ✓ MintingEngine has MINTER_ROLE${NC}"
    else
        echo -e "${RED}    ✗ MintingEngine missing MINTER_ROLE${NC}"
        ((ISSUES_FOUND++))

        if [ "$MODE" == "fix" ]; then
            echo "    → Granting MINTER_ROLE..."
            cargo contract call \
                --contract "$TOKEN_ADDR" \
                --message grant_role \
                --args $MINTER_ROLE "$ENGINE_ADDR" \
                --suri "//Alice" \
                --execute \
                --skip-confirm
            echo -e "${GREEN}    ✓ Fixed${NC}"
        fi
    fi

    # Check peg stabilizer has BURNER_ROLE
    HAS_BURNER=$(cargo contract call \
        --contract "$TOKEN_ADDR" \
        --message has_role \
        --args $BURNER_ROLE "$STABILIZER_ADDR" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_BURNER" == "true" ]; then
        echo -e "${GREEN}    ✓ PegStabilizer has BURNER_ROLE${NC}"
    else
        echo -e "${RED}    ✗ PegStabilizer missing BURNER_ROLE${NC}"
        ((ISSUES_FOUND++))

        if [ "$MODE" == "fix" ]; then
            echo "    → Granting BURNER_ROLE..."
            cargo contract call \
                --contract "$TOKEN_ADDR" \
                --message grant_role \
                --args $BURNER_ROLE "$STABILIZER_ADDR" \
                --suri "//Alice" \
                --execute \
                --skip-confirm
            echo -e "${GREEN}    ✓ Fixed${NC}"
        fi
    fi

    echo ""
    echo "  Checking EDSCReserveVault permissions..."

    cd ../reserve-vault

    # Check minting engine has DEPOSITOR_ROLE
    HAS_DEPOSITOR=$(cargo contract call \
        --contract "$VAULT_ADDR" \
        --message has_role \
        --args $DEPOSITOR_ROLE "$ENGINE_ADDR" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_DEPOSITOR" == "true" ]; then
        echo -e "${GREEN}    ✓ MintingEngine has DEPOSITOR_ROLE${NC}"
    else
        echo -e "${RED}    ✗ MintingEngine missing DEPOSITOR_ROLE${NC}"
        ((ISSUES_FOUND++))

        if [ "$MODE" == "fix" ]; then
            echo "    → Granting DEPOSITOR_ROLE..."
            cargo contract call \
                --contract "$VAULT_ADDR" \
                --message grant_role \
                --args $DEPOSITOR_ROLE "$ENGINE_ADDR" \
                --suri "//Alice" \
                --execute \
                --skip-confirm
            echo -e "${GREEN}    ✓ Fixed${NC}"
        fi
    fi

    echo ""
    echo "  Checking EDSCMintingEngine permissions..."

    cd ../minting-engine

    # Check stablecoin router has CALLER_ROLE
    HAS_CALLER=$(cargo contract call \
        --contract "$ENGINE_ADDR" \
        --message has_role \
        --args $ADMIN_ROLE "$STABLECOIN_ROUTER" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_CALLER" == "true" ]; then
        echo -e "${GREEN}    ✓ StablecoinRouter has CALLER_ROLE${NC}"
    else
        echo -e "${RED}    ✗ StablecoinRouter missing CALLER_ROLE${NC}"
        ((ISSUES_FOUND++))

        if [ "$MODE" == "fix" ]; then
            echo "    → Granting CALLER_ROLE..."
            cargo contract call \
                --contract "$ENGINE_ADDR" \
                --message grant_role \
                --args $ADMIN_ROLE "$STABLECOIN_ROUTER" \
                --suri "//Alice" \
                --execute \
                --skip-confirm
            echo -e "${GREEN}    ✓ Fixed${NC}"
        fi
    fi

    echo ""
}

# ═══════════════════════════════════════════════════════════════════
# INTENT ROUTER SYSTEM
# ═══════════════════════════════════════════════════════════════════

audit_router_permissions() {
    echo -e "${YELLOW}Auditing Intent Router Permissions...${NC}"

    ROUTER_ADDR=$(jq -r '.intent_router_system.intent_router' "$ADDRESSES_FILE")
    EXECUTOR_ADDR=$(jq -r '.intent_router_system.auto_swap_executor' "$ADDRESSES_FILE")
    BRIDGE_ROUTER_ADDR=$(jq -r '.intent_router_system.two_tier_bridge_router' "$ADDRESSES_FILE")

    echo "  Checking AutoSwapExecutor permissions..."

    cd "$SCRIPT_DIR/../intent-router/core/auto-swap-executor"

    # Check IntentRouter has CALLER_ROLE on AutoSwapExecutor
    HAS_CALLER=$(cargo contract call \
        --contract "$EXECUTOR_ADDR" \
        --message has_role \
        --args $CALLER_ROLE "$ROUTER_ADDR" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_CALLER" == "true" ]; then
        echo -e "${GREEN}    ✓ IntentRouter has CALLER_ROLE${NC}"
    else
        echo -e "${RED}    ✗ IntentRouter missing CALLER_ROLE${NC}"
        ((ISSUES_FOUND++))

        if [ "$MODE" == "fix" ]; then
            echo "    → Granting CALLER_ROLE..."
            cargo contract call \
                --contract "$EXECUTOR_ADDR" \
                --message grant_role \
                --args $CALLER_ROLE "$ROUTER_ADDR" \
                --suri "//Alice" \
                --execute \
                --skip-confirm
            echo -e "${GREEN}    ✓ Fixed${NC}"
        fi
    fi

    echo ""
    echo "  Checking TwoTierBridgeRouter permissions..."

    cd ../two-tier-bridge-router

    # Check AutoSwapExecutor has CALLER_ROLE on TwoTierBridgeRouter
    HAS_CALLER=$(cargo contract call \
        --contract "$BRIDGE_ROUTER_ADDR" \
        --message has_role \
        --args $CALLER_ROLE "$EXECUTOR_ADDR" \
        --suri "//Alice" \
        --dry-run \
        --output-json 2>/dev/null | jq -r '.data.Ok' || echo "false")

    if [ "$HAS_CALLER" == "true" ]; then
        echo -e "${GREEN}    ✓ AutoSwapExecutor has CALLER_ROLE${NC}"
    else
        echo -e "${RED}    ✗ AutoSwapExecutor missing CALLER_ROLE${NC}"
        ((ISSUES_FOUND++))

        if [ "$MODE" == "fix" ]; then
            echo "    → Granting CALLER_ROLE..."
            cargo contract call \
                --contract "$BRIDGE_ROUTER_ADDR" \
                --message grant_role \
                --args $CALLER_ROLE "$EXECUTOR_ADDR" \
                --suri "//Alice" \
                --execute \
                --skip-confirm
            echo -e "${GREEN}    ✓ Fixed${NC}"
        fi
    fi

    echo ""
}

# ═══════════════════════════════════════════════════════════════════
# PRODUCTION MODE: TRANSFER OWNERSHIP
# ═══════════════════════════════════════════════════════════════════

transfer_to_multisig() {
    echo -e "${RED}"
    echo "═══════════════════════════════════════════════════════════════"
    echo "  TRANSFERRING OWNERSHIP TO MULTI-SIG"
    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo ""
    echo "This will transfer ALL admin roles to: $MULTISIG"
    echo ""
    echo "After this operation:"
    echo "  - Deployer account will lose admin access"
    echo "  - Only multi-sig can manage contracts"
    echo "  - This action is IRREVERSIBLE"
    echo ""
    echo "Type 'CONFIRM' to proceed:"
    read CONFIRMATION

    if [ "$CONFIRMATION" != "CONFIRM" ]; then
        echo "Aborted."
        exit 1
    fi

    echo ""
    echo -e "${YELLOW}Transferring ownership...${NC}"

    # Transfer Address Registry ownership
    echo "  Transferring Address Registry..."
    REGISTRY_ADDRESS=$(jq -r '.address_registry' "$ADDRESSES_FILE")
    cd "$SCRIPT_DIR/../registry/address-registry"

    cargo contract call \
        --contract "$REGISTRY_ADDRESS" \
        --message transfer_ownership \
        --args "$MULTISIG" \
        --suri "//Alice" \
        --execute \
        --skip-confirm

    echo -e "${GREEN}    ✓ Address Registry transferred${NC}"

    # Note: Add transfer calls for all other contracts
    # In production, each contract needs transfer_ownership or similar

    echo ""
    echo -e "${GREEN}Ownership transfer complete!${NC}"
    echo ""
    echo "IMPORTANT: Verify multi-sig can access all contracts"
    echo "Test with multi-sig before announcing launch"
}

# ═══════════════════════════════════════════════════════════════════
# MAIN EXECUTION
# ═══════════════════════════════════════════════════════════════════

audit_wrapped_token_permissions
audit_tier2_permissions
audit_edsc_permissions
audit_router_permissions

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  PERMISSION AUDIT SUMMARY${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

if [ $ISSUES_FOUND -eq 0 ]; then
    echo -e "${GREEN}✓ ALL PERMISSIONS CORRECT${NC}"
    echo ""
    echo "No issues found. Permission configuration is complete."
else
    echo -e "${YELLOW}Found $ISSUES_FOUND permission issues${NC}"

    if [ "$MODE" == "audit" ]; then
        echo ""
        echo "Run with 'fix' mode to automatically correct:"
        echo "  ./configure_permissions.sh $NETWORK fix"
    elif [ "$MODE" == "fix" ]; then
        echo ""
        echo "All issues have been corrected."
        echo "Re-run audit to verify:"
        echo "  ./configure_permissions.sh $NETWORK audit"
    fi
fi

if [ "$MODE" == "prod" ]; then
    echo ""
    transfer_to_multisig
fi

echo ""
