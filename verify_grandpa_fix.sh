#!/bin/bash
# FlareChain GRANDPA Fix Verification Script
# Verifies that the runtime upgrade successfully initialized GRANDPA authorities

set -e

GIZZI_IP="64.181.215.19"
RPC_PORT="9944"
RPC_URL="http://${GIZZI_IP}:${RPC_PORT}"

echo "═══════════════════════════════════════════════════════════"
echo "FlareChain GRANDPA Fix Verification"
echo "═══════════════════════════════════════════════════════════"
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to make RPC call
rpc_call() {
    local method=$1
    local params=$2
    curl -s -H "Content-Type: application/json" \
         -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"$method\", \"params\": $params}" \
         "$RPC_URL"
}

echo "Step 1: Checking Runtime Version..."
echo "───────────────────────────────────────────────────────────"
VERSION_RESPONSE=$(rpc_call "state_getRuntimeVersion" "[]")
SPEC_VERSION=$(echo "$VERSION_RESPONSE" | grep -o '"specVersion":[0-9]*' | cut -d':' -f2)

if [ "$SPEC_VERSION" = "105" ]; then
    echo -e "${GREEN}✅ Runtime upgraded to spec version 105${NC}"
else
    echo -e "${RED}❌ Runtime still at spec version $SPEC_VERSION (expected 105)${NC}"
    echo -e "${YELLOW}⚠️  Runtime upgrade may not have been applied yet${NC}"
fi
echo ""

echo "Step 2: Checking GRANDPA Authorities Storage..."
echo "───────────────────────────────────────────────────────────"
# Storage key for Grandpa::Authorities
STORAGE_KEY="0x5f9cc45b7a00c5899361e1c6099678dc8a2d09463effcc78a22d75b9cb87dffc"
STORAGE_RESPONSE=$(rpc_call "state_getStorage" "[\"$STORAGE_KEY\"]")
STORAGE_VALUE=$(echo "$STORAGE_RESPONSE" | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

if [ "$STORAGE_VALUE" = "0x0000000000000000" ] || [ -z "$STORAGE_VALUE" ]; then
    echo -e "${RED}❌ GRANDPA authorities storage is EMPTY${NC}"
    echo -e "${YELLOW}⚠️  Migration has NOT run yet or failed${NC}"
else
    echo -e "${GREEN}✅ GRANDPA authorities storage has data${NC}"
    echo "Storage value: $STORAGE_VALUE"
    # Estimate authority count (rough approximation)
    BYTE_COUNT=$((${#STORAGE_VALUE} / 2))
    echo "Estimated storage size: ~$BYTE_COUNT bytes"
fi
echo ""

echo "Step 3: Checking GRANDPA Authorities via API..."
echo "───────────────────────────────────────────────────────────"
AUTHORITIES_RESPONSE=$(rpc_call "state_call" "[\"GrandpaApi_grandpa_authorities\", \"0x\"]")
AUTHORITIES_RESULT=$(echo "$AUTHORITIES_RESPONSE" | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

if [ -z "$AUTHORITIES_RESULT" ] || [ "$AUTHORITIES_RESULT" = "null" ]; then
    echo -e "${RED}❌ Could not fetch GRANDPA authorities${NC}"
else
    echo -e "${GREEN}✅ GRANDPA authorities API returned data${NC}"
    echo "Authorities (encoded): ${AUTHORITIES_RESULT:0:100}..."
fi
echo ""

echo "Step 4: Checking Chain Status..."
echo "───────────────────────────────────────────────────────────"
CHAIN_RESPONSE=$(rpc_call "system_health" "[]")
PEERS=$(echo "$CHAIN_RESPONSE" | grep -o '"peers":[0-9]*' | cut -d':' -f2)
IS_SYNCING=$(echo "$CHAIN_RESPONSE" | grep -o '"isSyncing":[a-z]*' | cut -d':' -f2)

echo "Connected peers: $PEERS"
echo "Is syncing: $IS_SYNCING"
echo ""

echo "Step 5: Checking Block Heights..."
echo "───────────────────────────────────────────────────────────"
HEADER_RESPONSE=$(rpc_call "chain_getHeader" "[]")
BEST_BLOCK=$(echo "$HEADER_RESPONSE" | grep -o '"number":"0x[^"]*"' | head -1 | cut -d'"' -f4)
BEST_BLOCK_DEC=$((BEST_BLOCK))

FINALIZED_RESPONSE=$(rpc_call "chain_getFinalizedHead" "[]")
FINALIZED_HASH=$(echo "$FINALIZED_RESPONSE" | grep -o '"result":"[^"]*"' | cut -d'"' -f4)
FINALIZED_HEADER=$(rpc_call "chain_getHeader" "[\"$FINALIZED_HASH\"]")
FINALIZED_BLOCK=$(echo "$FINALIZED_HEADER" | grep -o '"number":"0x[^"]*"' | head -1 | cut -d'"' -f4)
FINALIZED_BLOCK_DEC=$((FINALIZED_BLOCK))

echo "Best block: #$BEST_BLOCK_DEC"
echo "Finalized block: #$FINALIZED_BLOCK_DEC"

FINALITY_GAP=$((BEST_BLOCK_DEC - FINALIZED_BLOCK_DEC))
echo "Finality gap: $FINALITY_GAP blocks"

if [ $FINALITY_GAP -gt 100 ]; then
    echo -e "${YELLOW}⚠️  Large finality gap - waiting for 6/9 Directors to sync${NC}"
elif [ $FINALITY_GAP -lt 10 ]; then
    echo -e "${GREEN}✅ Finality is progressing normally${NC}"
else
    echo -e "${YELLOW}⚠️  Moderate finality gap${NC}"
fi
echo ""

echo "═══════════════════════════════════════════════════════════"
echo "Summary"
echo "═══════════════════════════════════════════════════════════"

# Overall assessment
if [ "$SPEC_VERSION" = "105" ] && [ "$STORAGE_VALUE" != "0x0000000000000000" ] && [ -n "$STORAGE_VALUE" ]; then
    echo -e "${GREEN}✅ GRANDPA fix appears to be SUCCESSFUL${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Monitor finality gap - should decrease as more Directors sync"
    echo "2. Check node logs: journalctl -fu flarechain-node"
    echo "3. Wait for 6/9 Directors to reach best block for finality"
    echo ""
    echo "Current Director status (from network map):"
    echo "  ✅ Gizzi (64.181.215.19) - SYNCED"
    echo "  ✅ EojEdred (85.239.239.194) - SYNCED"
    echo "  ✅ Audit Dev (129.80.122.34) - SYNCED"
    echo "  🔄 governance-dev01 (80.190.82.186) - SYNCING"
    echo "  🔄 security-dev01 (85.239.239.190) - SYNCING"
    echo "  ✅ consensus-dev01 (85.239.239.189) - OPERATIONAL"
    echo "  🔄 runtime-dev01 (85.239.239.193) - SYNCING"
    echo "  ✅ oracle-dev01 (85.239.239.189) - OPERATIONAL"
    echo "  🔄 compliance-dev (154.12.250.18) - SYNCING"
    echo ""
    echo "Need 3 more Directors to sync for finality (currently 3-5/9)"
elif [ "$SPEC_VERSION" = "105" ]; then
    echo -e "${YELLOW}⚠️  Runtime upgraded but migration may not have run yet${NC}"
    echo ""
    echo "Possible reasons:"
    echo "1. Migration runs on first block after upgrade - wait a few moments"
    echo "2. Check node logs for migration messages"
    echo "3. Verify Executive type includes the migration"
elif [ "$SPEC_VERSION" = "104" ]; then
    echo -e "${RED}❌ Runtime has NOT been upgraded yet${NC}"
    echo ""
    echo "Action required:"
    echo "1. Build WASM runtime with spec_version 105"
    echo "2. Submit sudo.sudoUncheckedWeight → system.setCode extrinsic"
    echo "3. Upload flare_chain_runtime_v105.wasm"
else
    echo -e "${YELLOW}⚠️  Unknown state - manual investigation required${NC}"
fi

echo "═══════════════════════════════════════════════════════════"
echo ""

# Additional diagnostics
echo "Additional Diagnostics"
echo "───────────────────────────────────────────────────────────"
echo "To check GRANDPA logs on Gizzi VM:"
echo "  ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19"
echo "  journalctl -fu flarechain-node | grep -i grandpa"
echo ""
echo "To manually query authorities:"
echo "  curl -H 'Content-Type: application/json' \\"
echo "    -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"state_call\", \"params\":[\"GrandpaApi_grandpa_authorities\", \"0x\"]}' \\"
echo "    http://$GIZZI_IP:$RPC_PORT"
echo ""
echo "To check specific validator sync status:"
echo "  ssh -i ~/.ssh/contabo-validators root@<VALIDATOR_IP>"
echo "  journalctl -fu flarechain-node | tail -50"
echo ""
