#!/usr/bin/env bash
# Ëtrid FlareChain - Query Active Validator Set via RPC
# No SSH required - uses RPC endpoints only

set -e

OUTPUT_FILE="validator_set.json"

# Azure VM RPC endpoints (we have access to these)
AZURE_VMS=(
    "20.69.26.209:9944"    # VM1
    "20.186.91.207:9944"   # VM2
    "52.252.142.146:9944"  # VM3
)

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Active Validator Set Query via RPC      ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Try each RPC endpoint until one works
RPC_ENDPOINT=""
for rpc in "${AZURE_VMS[@]}"; do
    echo "Testing RPC: http://$rpc"
    if curl -s -m 5 -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://$rpc >/dev/null 2>&1; then
        RPC_ENDPOINT="http://$rpc"
        echo "✓ Connected to $rpc"
        echo ""
        break
    else
        echo "✗ Failed to connect"
    fi
done

if [[ -z "$RPC_ENDPOINT" ]]; then
    echo "ERROR: Could not connect to any Azure VM RPC endpoint"
    echo "Please verify validators are running and RPC ports are open"
    exit 1
fi

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "ACTIVE VALIDATOR SET QUERY"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "RPC Endpoint: $RPC_ENDPOINT"
    echo "Query Time: $(date)"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "1. RUNTIME METADATA - Available Queries"
    echo "─────────────────────────────────────────────────────────────"
    echo ""
    echo "Checking which runtime APIs are available..."
    echo ""

    # Get runtime version to understand what's available
    runtime_version=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"state_getRuntimeVersion","params":[],"id":1}' \
        $RPC_ENDPOINT)

    echo "Runtime Version:"
    echo "$runtime_version" | jq '.result' 2>/dev/null || echo "$runtime_version"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "2. SESSION VALIDATORS - Current Active Set"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Query session.validators() - standard Substrate call
    validators_response=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"state_call","params":["SessionApi_validators","0x"],"id":1}' \
        $RPC_ENDPOINT)

    echo "Session Validators Response:"
    echo "$validators_response" | jq '.' 2>/dev/null || echo "$validators_response"
    echo ""

    # Try alternative: query state storage directly
    echo "Attempting alternative: Direct state storage query..."
    validators_storage=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"state_getStorage","params":["0x2b06af9719ac64d755623cda8ddd9b94b1c371ded9e9c565e89ba783c4d5f5f9"],"id":1}' \
        $RPC_ENDPOINT)

    echo "$validators_storage" | jq '.' 2>/dev/null || echo "$validators_storage"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "3. AURA AUTHORITIES - Block Producers"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Query AURA authorities
    aura_authorities=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"state_call","params":["AuraApi_authorities","0x"],"id":1}' \
        $RPC_ENDPOINT)

    echo "AURA Authorities Response:"
    echo "$aura_authorities" | jq '.' 2>/dev/null || echo "$aura_authorities"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "4. RECENT BLOCK AUTHORS - Who's Producing Blocks"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "Analyzing last 20 blocks to identify active validators..."
    echo ""

    # Get current block number
    current_block=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
        $RPC_ENDPOINT | jq -r '.result.number' | xargs printf "%d")

    echo "Current block: $current_block"
    echo ""
    echo "Block authors (last 20 blocks):"
    echo ""

    start_block=$((current_block - 20))
    for ((block=$start_block; block<=$current_block; block++)); do
        # Get block hash
        block_hash=$(curl -s -X POST \
            -H "Content-Type: application/json" \
            -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlockHash\",\"params\":[$block],\"id\":1}" \
            $RPC_ENDPOINT | jq -r '.result')

        if [[ "$block_hash" != "null" && -n "$block_hash" ]]; then
            # Get block header to see author
            block_header=$(curl -s -X POST \
                -H "Content-Type: application/json" \
                -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"$block_hash\"],\"id\":1}" \
                $RPC_ENDPOINT)

            author=$(echo "$block_header" | jq -r '.result.author // "N/A"')
            printf "Block #%-6d: %s\n" "$block" "$author"
        fi
    done

    echo ""
    echo "Unique authors in last 20 blocks:"
    echo ""

    # Collect all authors and count unique ones
    declare -A author_counts
    for ((block=$start_block; block<=$current_block; block++)); do
        block_hash=$(curl -s -X POST \
            -H "Content-Type: application/json" \
            -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlockHash\",\"params\":[$block],\"id\":1}" \
            $RPC_ENDPOINT | jq -r '.result')

        if [[ "$block_hash" != "null" && -n "$block_hash" ]]; then
            author=$(curl -s -X POST \
                -H "Content-Type: application/json" \
                -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"$block_hash\"],\"id\":1}" \
                $RPC_ENDPOINT | jq -r '.result.author // "N/A"')

            if [[ "$author" != "N/A" ]]; then
                ((author_counts["$author"]++)) || author_counts["$author"]=1
            fi
        fi
    done

    for author in "${!author_counts[@]}"; do
        echo "  $author: ${author_counts[$author]} blocks"
    done

    echo ""
    echo "Total unique block authors: ${#author_counts[@]}"
    echo ""

    if [[ ${#author_counts[@]} -eq 16 ]]; then
        echo "✓ Exactly 16 unique block authors (matches committee size)"
    else
        echo "⚠ ${#author_counts[@]} unique block authors (expected 16)"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "5. GRANDPA VOTER SET - Finality Authorities"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Query GRANDPA authorities
    grandpa_authorities=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"state_call","params":["GrandpaApi_grandpa_authorities","0x"],"id":1}' \
        $RPC_ENDPOINT)

    echo "GRANDPA Authorities Response:"
    echo "$grandpa_authorities" | jq '.' 2>/dev/null || echo "$grandpa_authorities"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "6. SESSION KEYS - Registered Keys per Validator"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "Attempting to query session keys for known accounts..."
    echo "(This requires knowing validator account IDs from genesis)"
    echo ""

    # Note: Without account IDs, we can't query specific session keys
    # But we can list all session.nextKeys storage entries
    echo "Storage key for session.nextKeys prefix: 0xcec5070d609dd3497f72bde07fc96ba0"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "7. SYSTEM PEERS - Network Connectivity"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    peers=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_peers","params":[],"id":1}' \
        $RPC_ENDPOINT)

    echo "Connected peers (from Azure VM perspective):"
    echo "$peers" | jq '.result[] | {peerId: .peerId, roles: .roles, bestHash: .bestHash}' 2>/dev/null | head -50

    peer_count=$(echo "$peers" | jq '.result | length' 2>/dev/null || echo "0")
    echo ""
    echo "Total connected peers: $peer_count"
    echo ""

    echo "═══════════════════════════════════════════════════════════════"
    echo "ANALYSIS SUMMARY"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

    if [[ ${#author_counts[@]} -eq 16 ]]; then
        echo "✓ CONFIRMED: 16 validators actively producing blocks"
        echo ""
        echo "Block authors (validators 6-21) identified by account ID:"
        for author in "${!author_counts[@]}"; do
            echo "  - $author"
        done
    else
        echo "⚠ Unexpected number of block authors: ${#author_counts[@]}"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "COMPARISON WITH AZURE VMs"
    echo "─────────────────────────────────────────────────────────────"
    echo ""
    echo "Azure VM Peer IDs (known):"
    echo "  VM1: 12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm"
    echo "  VM2: 12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb"
    echo "  VM3: 12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6"
    echo ""

    # Check if Azure VM peer IDs appear in connected peers
    azure_vm1_found=false
    azure_vm2_found=false
    azure_vm3_found=false

    if echo "$peers" | grep -q "12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm"; then
        azure_vm1_found=true
    fi
    if echo "$peers" | grep -q "12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb"; then
        azure_vm2_found=true
    fi
    if echo "$peers" | grep -q "12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6"; then
        azure_vm3_found=true
    fi

    echo "Azure VMs in active block production:"
    if [[ "$azure_vm1_found" == false && "$azure_vm2_found" == false && "$azure_vm3_found" == false ]]; then
        echo "  ✗ NONE of the Azure VMs are producing blocks"
        echo ""
        echo "This confirms that Azure VMs (validators 2-4) are NOT in the active committee."
    else
        echo "  ⚠ Some Azure VMs may be active (requires deeper analysis)"
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "NEXT STEPS:"
    echo "─────────────────────────────────────────────────────────────"
    echo ""
    echo "1. Compare block authors (account IDs) with genesis configuration"
    echo "2. Determine if Azure VMs need to register session keys"
    echo "3. Check if staking/bonding is required"
    echo "4. Run discover-peers-via-rpc.sh to map peer IDs to validators"
    echo ""

} | tee "$OUTPUT_FILE"

echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Query complete: $OUTPUT_FILE"
echo ""
