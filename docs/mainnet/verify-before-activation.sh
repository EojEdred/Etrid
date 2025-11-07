#!/usr/bin/env bash
# Ëtrid FlareChain - Pre-Activation Verification
# Run this BEFORE executing activate_all_validators.sh

set -e

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Pre-Activation Verification             ║"
echo "║  CRITICAL: Verify network state before inserting keys       ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Test RPC endpoints
AZURE_VMS=(
    "20.69.26.209:9944|VM1"
    "20.186.91.207:9944|VM2"
    "52.252.142.146:9944|VM3"
)

RPC_ENDPOINT=""
for vm in "${AZURE_VMS[@]}"; do
    IFS='|' read -r rpc name <<< "$vm"
    echo "Testing $name ($rpc)..."
    if curl -s -m 5 -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://$rpc >/dev/null 2>&1; then
        RPC_ENDPOINT="http://$rpc"
        echo "✓ Connected to $name"
        echo ""
        break
    fi
done

if [[ -z "$RPC_ENDPOINT" ]]; then
    echo "❌ ERROR: Cannot connect to any Azure VM"
    echo ""
    echo "Trying alternative validator IPs from activation script..."

    # Try validators from activation script
    TEST_IPS=("20.224.104.239:9944" "98.71.91.84:9944" "51.142.203.160:9944")
    for ip in "${TEST_IPS[@]}"; do
        echo "Testing $ip..."
        if curl -s -m 5 -X POST \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip >/dev/null 2>&1; then
            RPC_ENDPOINT="http://$ip"
            echo "✓ Connected to $ip"
            echo ""
            break
        fi
    done
fi

if [[ -z "$RPC_ENDPOINT" ]]; then
    echo "❌ CRITICAL: No RPC endpoints accessible"
    echo ""
    echo "Cannot verify network state. Possible reasons:"
    echo "  - All validators are down"
    echo "  - Firewall blocking RPC ports"
    echo "  - Network connectivity issues"
    echo ""
    echo "⛔ DO NOT PROCEED with key insertion until connectivity restored"
    exit 1
fi

echo "Using RPC endpoint: $RPC_ENDPOINT"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "CHECK 1: Network Health"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

health=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
    $RPC_ENDPOINT)

echo "$health" | jq '.' 2>/dev/null || echo "$health"
echo ""

peer_count=$(echo "$health" | jq -r '.result.peers' 2>/dev/null || echo "0")
is_syncing=$(echo "$health" | jq -r '.result.isSyncing' 2>/dev/null || echo "unknown")

echo "Summary:"
echo "  Peers: $peer_count"
echo "  Syncing: $is_syncing"
echo ""

if [[ "$peer_count" -lt 5 ]]; then
    echo "⚠️  WARNING: Low peer count ($peer_count)"
    echo "   Network may be fragmented or starting up"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "CHECK 2: Current Block Height & Finality"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

sync_state=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_syncState","params":[],"id":1}' \
    $RPC_ENDPOINT)

current_block=$(echo "$sync_state" | jq -r '.result.currentBlock' 2>/dev/null || echo "unknown")
highest_block=$(echo "$sync_state" | jq -r '.result.highestBlock' 2>/dev/null || echo "unknown")

echo "Current block: #$current_block"
echo "Highest block: #$highest_block"

# Get finalized block
header=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
    $RPC_ENDPOINT)

finalized_hash=$(echo "$header" | jq -r '.result' 2>/dev/null)

if [[ -n "$finalized_hash" && "$finalized_hash" != "null" ]]; then
    finalized_header=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"$finalized_hash\"],\"id\":1}" \
        $RPC_ENDPOINT)

    finalized_number=$(echo "$finalized_header" | jq -r '.result.number' 2>/dev/null | xargs printf "%d" 2>/dev/null || echo "unknown")
    echo "Finalized block: #$finalized_number"

    if [[ "$current_block" != "unknown" && "$finalized_number" != "unknown" ]]; then
        finality_lag=$((current_block - finalized_number))
        echo "Finality lag: $finality_lag blocks"

        if [[ $finality_lag -le 5 ]]; then
            echo "  ✓ Finality healthy (lag ≤ 5 blocks)"
        elif [[ $finality_lag -le 20 ]]; then
            echo "  ⚠️  Finality slightly delayed ($finality_lag blocks)"
        else
            echo "  ❌ Finality severely delayed ($finality_lag blocks)"
        fi
    fi
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "CHECK 3: Active Validator Count (CRITICAL)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Querying session validators via SessionApi_validators..."

validators_response=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"state_call","params":["SessionApi_validators","0x"],"id":1}' \
    $RPC_ENDPOINT 2>/dev/null)

echo "Raw response:"
echo "$validators_response" | jq '.' 2>/dev/null || echo "$validators_response"
echo ""

# Try to decode the result
validator_count=$(echo "$validators_response" | jq -r '.result' 2>/dev/null | xxd -r -p 2>/dev/null | grep -o "0x" | wc -l | tr -d ' ' || echo "0")

if [[ "$validator_count" -eq 0 ]]; then
    # Try alternative: query authorities from consensus
    echo "Alternative query: Checking AURA authorities..."

    aura_response=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"state_call","params":["AuraApi_authorities","0x"],"id":1}' \
        $RPC_ENDPOINT 2>/dev/null)

    echo "$aura_response" | jq '.' 2>/dev/null || echo "$aura_response"
    echo ""
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "CHECK 4: Recent Block Authors (CRITICAL)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Analyzing last 20 blocks to count unique validators..."
echo ""

# Get current block
current=$(curl -s -X POST \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
    $RPC_ENDPOINT | jq -r '.result.number' | xargs printf "%d" 2>/dev/null)

if [[ -z "$current" || "$current" == "0" ]]; then
    echo "❌ Cannot determine current block number"
else
    echo "Current block: #$current"
    echo ""

    start_block=$((current - 20))
    if [[ $start_block -lt 0 ]]; then
        start_block=0
    fi

    echo "Block Authors (#$start_block to #$current):"
    echo ""

    declare -A authors

    for ((block=$start_block; block<=$current; block++)); do
        block_hash=$(curl -s -X POST \
            -H "Content-Type: application/json" \
            -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlockHash\",\"params\":[$block],\"id\":1}" \
            $RPC_ENDPOINT | jq -r '.result')

        if [[ -n "$block_hash" && "$block_hash" != "null" ]]; then
            author=$(curl -s -X POST \
                -H "Content-Type: application/json" \
                -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"$block_hash\"],\"id\":1}" \
                $RPC_ENDPOINT | jq -r '.result.author // .result.digest.logs[0] // "unknown"')

            if [[ -n "$author" && "$author" != "null" && "$author" != "unknown" ]]; then
                printf "Block #%-6d: %s\n" "$block" "$author"
                ((authors["$author"]++)) || authors["$author"]=1
            fi
        fi
    done

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Unique block authors:"
    echo ""

    for author in "${!authors[@]}"; do
        echo "  $author: ${authors[$author]} blocks"
    done

    unique_count=${#authors[@]}
    echo ""
    echo "TOTAL UNIQUE AUTHORS: $unique_count"
    echo ""
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "CHECK 5: Test Validator #6 Key Status"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Testing if validator #6 (20.224.104.239) already has AURA key..."
echo "Target key: 0xf44ee1c6da7cf209998874f2fa612e75de439afb385625281e123ec8b15ea42f"
echo ""

# Try to query the validator directly if it has RPC
key_check=$(curl -s -m 5 -X POST \
    -H "Content-Type: application/json" \
    -d '{
        "id":1,
        "jsonrpc":"2.0",
        "method":"author_hasKey",
        "params":["0xf44ee1c6da7cf209998874f2fa612e75de439afb385625281e123ec8b15ea42f","aura"]
    }' \
    http://20.224.104.239:9944 2>/dev/null)

if [[ -n "$key_check" ]]; then
    echo "Response from validator #6:"
    echo "$key_check" | jq '.'
    echo ""

    has_key=$(echo "$key_check" | jq -r '.result' 2>/dev/null)

    if [[ "$has_key" == "true" ]]; then
        echo "⚠️  CRITICAL: Validator #6 ALREADY HAS this AURA key!"
        echo "   DO NOT insert keys - risk of duplication!"
    elif [[ "$has_key" == "false" ]]; then
        echo "✓ Validator #6 does NOT have this key"
        echo "  Safe to insert keys"
    else
        echo "⚠️  Unknown key status: $has_key"
    fi
else
    echo "⚠️  Cannot connect to validator #6 RPC"
    echo "   (This is expected if RPC is not exposed publicly)"
fi

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  VERIFICATION COMPLETE                                       ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "ANALYSIS & RECOMMENDATION"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

if [[ -n "$unique_count" ]]; then
    echo "Active validators producing blocks: $unique_count"
    echo ""

    if [[ $unique_count -le 5 ]]; then
        echo "✅ HYPOTHESIS CONFIRMED:"
        echo "   Only 5 validators are active (24% committee)"
        echo ""
        echo "🟢 SAFE TO PROCEED with /tmp/activate_all_validators.sh"
        echo ""
        echo "Expected result:"
        echo "  - Committee will expand from $unique_count to 21 validators"
        echo "  - Byzantine tolerance will improve significantly"
        echo "  - ASF finality will increase to 95%+"
        echo ""
        echo "Next step:"
        echo "  bash /tmp/activate_all_validators.sh"

    elif [[ $unique_count -ge 15 ]]; then
        echo "❌ HYPOTHESIS INCORRECT:"
        echo "   $unique_count validators are already active (NOT 5)"
        echo ""
        echo "🔴 DO NOT PROCEED with /tmp/activate_all_validators.sh"
        echo ""
        echo "Reason:"
        echo "  - Keys may already be inserted"
        echo "  - Risk of key duplication and equivocation"
        echo "  - Could cause consensus failure"
        echo ""
        echo "Alternative investigation needed:"
        echo "  - Determine which 3-6 validators are missing"
        echo "  - Check if they are validators 2-4 (Azure VMs)"
        echo "  - Use targeted key insertion for missing validators only"
        echo ""
        echo "Refer to:"
        echo "  /Users/macbook/Desktop/etrid/docs/mainnet/COMMITTEE_ANALYSIS_AND_PLAN.md"

    else
        echo "⚠️  PARTIAL COMMITTEE:"
        echo "   $unique_count validators active (between 5 and 16)"
        echo ""
        echo "🟡 PROCEED WITH CAUTION"
        echo ""
        echo "Recommendation:"
        echo "  1. Determine which validators are already active"
        echo "  2. Modify /tmp/activate_all_validators.sh to ONLY insert"
        echo "     keys for validators that are NOT already producing blocks"
        echo "  3. Use staged rollout (one validator at a time)"
        echo ""
    fi
else
    echo "⚠️  Could not determine active validator count"
    echo ""
    echo "🟡 INSUFFICIENT DATA"
    echo ""
    echo "Recommendation:"
    echo "  - Manually SSH to validators and check keystore directories"
    echo "  - Use query-validator-set.sh for detailed analysis"
    echo "  - Do NOT proceed with mass key insertion until verified"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
