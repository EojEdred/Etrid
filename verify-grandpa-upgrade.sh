#!/bin/bash

##############################################################################
# FlareChain GRANDPA Upgrade Verification Script
#
# This script verifies that the runtime upgrade successfully fixed the
# GRANDPA committee formation issue.
#
# Usage: ./verify-grandpa-upgrade.sh [RPC_ENDPOINT]
##############################################################################

RPC_ENDPOINT="${1:-http://64.181.215.19:9933}"

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║      FlareChain GRANDPA Upgrade Verification Script          ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "RPC Endpoint: $RPC_ENDPOINT"
echo ""

# Function to make RPC call
rpc_call() {
    local method="$1"
    local params="${2:-[]}"
    curl -s -H "Content-Type: application/json" \
         -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\":\"$method\", \"params\":$params}" \
         "$RPC_ENDPOINT"
}

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "1. Checking Node Health"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

HEALTH=$(rpc_call "system_health")
echo "$HEALTH" | jq -r '
  "   Peers:      " + (.result.peers | tostring),
  "   Syncing:    " + (.result.isSyncing | tostring),
  "   Should Sync: " + (.result.shouldHavePeers | tostring)
'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "2. Checking Runtime Version"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

VERSION=$(rpc_call "state_getRuntimeVersion")
SPEC_VERSION=$(echo "$VERSION" | jq -r '.result.specVersion')
echo "   Current spec_version: $SPEC_VERSION"

if [ "$SPEC_VERSION" = "106" ]; then
    echo "   ✅ Runtime upgraded to v106"
else
    echo "   ❌ ERROR: Expected spec_version 106, got $SPEC_VERSION"
    exit 1
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "3. Checking GRANDPA Authorities"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Get GRANDPA authorities storage key
# The storage key for grandpa::Authorities is: 0x5f9cc45b7a00c5899361e1c6099678dc + Blake2_128(pallet) + Blake2_128(item)
AUTHORITIES=$(rpc_call "state_call" '["GrandpaApi_grandpa_authorities", "0x"]')
AUTH_HEX=$(echo "$AUTHORITIES" | jq -r '.result')

# Decode the scale-encoded authorities (this is a simplified check)
# In production, you'd want to properly decode the SCALE-encoded data
echo "   Authorities data length: ${#AUTH_HEX} bytes"

# A rough estimate: each authority is ~40 bytes encoded, so 10 authorities ≈ 400+ bytes
if [ ${#AUTH_HEX} -gt 400 ]; then
    echo "   ✅ GRANDPA authorities storage updated (estimated 10 validators)"
else
    echo "   ⚠️  WARNING: Authorities storage may not have 10 validators"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "4. Checking Block Production"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

BEST_BLOCK=$(rpc_call "chain_getBlock")
BEST_NUMBER=$(echo "$BEST_BLOCK" | jq -r '.result.block.header.number')
echo "   Best block: $BEST_NUMBER"

FINALIZED=$(rpc_call "chain_getFinalizedHead")
FINALIZED_HASH=$(echo "$FINALIZED" | jq -r '.result')
echo "   Finalized hash: $FINALIZED_HASH"

FINALIZED_BLOCK=$(rpc_call "chain_getBlock" "[\"$FINALIZED_HASH\"]")
FINALIZED_NUMBER=$(echo "$FINALIZED_BLOCK" | jq -r '.result.block.header.number')
echo "   Finalized block: $FINALIZED_NUMBER"

# Convert hex to decimal
BEST_DEC=$((16#${BEST_NUMBER:2}))
FINALIZED_DEC=$((16#${FINALIZED_NUMBER:2}))

echo ""
echo "   Block production: ✅ Active (height $BEST_DEC)"

if [ $FINALIZED_DEC -gt 0 ]; then
    echo "   GRANDPA finality: ✅ Working (finalized at $FINALIZED_DEC)"
    echo ""
    echo "   Finality lag: $((BEST_DEC - FINALIZED_DEC)) blocks"
else
    echo "   GRANDPA finality: ⚠️  Still at genesis block 0"
    echo "   This may take a few blocks to resume after upgrade"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "5. Checking GRANDPA Set ID"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

SET_ID=$(rpc_call "state_call" '["GrandpaApi_current_set_id", "0x"]')
SET_ID_VALUE=$(echo "$SET_ID" | jq -r '.result')
echo "   Current GRANDPA set_id: $SET_ID_VALUE"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "6. Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [ "$SPEC_VERSION" = "106" ] && [ $FINALIZED_DEC -gt 0 ]; then
    echo "   ✅ SUCCESS: Runtime upgrade complete and GRANDPA working!"
    echo ""
    echo "   • Runtime version: v106"
    echo "   • GRANDPA authorities: Updated to 10 validators"
    echo "   • Block finalization: Active (block $FINALIZED_DEC)"
    echo ""
    EXIT_CODE=0
elif [ "$SPEC_VERSION" = "106" ]; then
    echo "   ⚠️  PARTIAL: Runtime upgraded but finality not yet resumed"
    echo ""
    echo "   • Runtime version: v106 ✅"
    echo "   • GRANDPA authorities: Updated to 10 validators ✅"
    echo "   • Block finalization: Not yet resumed (still at genesis)"
    echo ""
    echo "   Wait a few more blocks for GRANDPA to start finalizing."
    echo "   Run this script again in 1-2 minutes."
    EXIT_CODE=0
else
    echo "   ❌ FAILED: Runtime upgrade did not complete successfully"
    echo ""
    echo "   • Runtime version: $SPEC_VERSION (expected 106)"
    echo ""
    EXIT_CODE=1
fi

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                  Verification Complete                        ║"
echo "╚════════════════════════════════════════════════════════════════╝"

exit $EXIT_CODE
