#!/bin/bash
# Test Key Recovery Script
# Validates that all generated keys can be recovered from their seed phrases
# This ensures backup procedures work correctly before mainnet deployment

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║          Key Recovery Test System                         ║"
echo "║      Validates all keys can be recovered from seeds       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check arguments
if [ "$#" -ne 1 ]; then
    echo -e "${RED}❌ Error: Missing genesis accounts directory${NC}"
    echo ""
    echo "Usage: ./test-key-recovery.sh <genesis-accounts-directory>"
    echo "Example: ./test-key-recovery.sh genesis-accounts-20251030-152748"
    exit 1
fi

ACCOUNTS_DIR="$1"

if [ ! -d "$ACCOUNTS_DIR" ]; then
    echo -e "${RED}❌ Error: Directory not found: $ACCOUNTS_DIR${NC}"
    exit 1
fi

# Check if flarechain-node binary exists
BINARY_PATH="$(pwd)/target/release/flarechain-node"
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}❌ Error: flarechain-node binary not found at $BINARY_PATH${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Found flarechain-node binary${NC}"
echo -e "${GREEN}✅ Found accounts directory: $ACCOUNTS_DIR${NC}"
echo ""

# Test recovery function
test_recovery() {
    local json_file="$1"
    local account_name="$2"

    if [ ! -f "$json_file" ]; then
        echo -e "${RED}   ❌ File not found: $json_file${NC}"
        return 1
    fi

    # Extract original data
    ORIGINAL_SEED=$(cat "$json_file" | grep -o '"secretSeed": "[^"]*' | cut -d'"' -f4)
    ORIGINAL_PHRASE=$(cat "$json_file" | grep -o '"secretPhrase": "[^"]*' | cut -d'"' -f4)
    ORIGINAL_ADDRESS=$(cat "$json_file" | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)

    if [ -z "$ORIGINAL_SEED" ] || [ -z "$ORIGINAL_ADDRESS" ]; then
        echo -e "${RED}   ❌ Failed to extract data from $account_name${NC}"
        return 1
    fi

    # Test recovery from seed
    RECOVERED_JSON=$("$BINARY_PATH" key inspect "$ORIGINAL_SEED" --output-type json 2>/dev/null)
    RECOVERED_ADDRESS=$(echo "$RECOVERED_JSON" | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)

    if [ "$ORIGINAL_ADDRESS" = "$RECOVERED_ADDRESS" ]; then
        echo -e "${GREEN}   ✅ $account_name: Recovery successful${NC}"
        return 0
    else
        echo -e "${RED}   ❌ $account_name: Recovery FAILED${NC}"
        echo "      Original:  $ORIGINAL_ADDRESS"
        echo "      Recovered: $RECOVERED_ADDRESS"
        return 1
    fi
}

TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}TESTING ETR TOKENOMICS ACCOUNTS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for account in dao_treasury community_lp_pool foundation_team_vesting network_expansion founders_pool initial_circulating; do
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if test_recovery "$ACCOUNTS_DIR/$account.json" "$account"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}TESTING FOUNDATION MULTISIG SIGNERS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for i in {1..7}; do
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if test_recovery "$ACCOUNTS_DIR/foundation_multisig_signers/signer_$i.json" "Signer_$i"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}TESTING EDSC INFRASTRUCTURE ACCOUNTS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for account in reserve_vault oracle_authority custodian_manager minter_authority emergency_pause; do
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if test_recovery "$ACCOUNTS_DIR/edsc_accounts/$account.json" "$account"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done

for asset in BTC ETH Gold USDC USDT; do
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if test_recovery "$ACCOUNTS_DIR/edsc_accounts/custodians/${asset}_custodian.json" "${asset}_custodian"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}TESTING VALIDATOR PAYMENT ACCOUNTS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for i in {1..21}; do
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if test_recovery "$ACCOUNTS_DIR/validator_payment_accounts/validator_${i}_payment.json" "Validator_${i}"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}TESTING TEAM & ADVISOR ACCOUNTS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

for member in CEO_Founder CTO Core_Dev_1 Core_Dev_2 Core_Dev_3 AI_Director Advisor_1 Advisor_2 Advisor_3 Marketing_Lead; do
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    if test_recovery "$ACCOUNTS_DIR/team_accounts/$member.json" "$member"; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
done
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║              Recovery Test Complete!                       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

echo "📊 Test Results:"
echo -e "   Total Tests:  $TOTAL_TESTS"
echo -e "   ${GREEN}Passed:       $PASSED_TESTS${NC}"
echo -e "   ${RED}Failed:       $FAILED_TESTS${NC}"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo -e "${GREEN}✅ ALL RECOVERY TESTS PASSED!${NC}"
    echo ""
    echo "🔐 Your backup strategy is working correctly."
    echo "   All keys can be recovered from their seed phrases."
    echo ""
    exit 0
else
    echo -e "${RED}❌ SOME RECOVERY TESTS FAILED!${NC}"
    echo ""
    echo "⚠️  WARNING: Do NOT proceed to mainnet until all tests pass."
    echo "   Review the failed accounts and regenerate them if needed."
    echo ""
    exit 1
fi
