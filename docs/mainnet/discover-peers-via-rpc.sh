#!/usr/bin/env bash
# Ëtrid FlareChain - Discover Committee Peer IDs via RPC
# Method A: Use system_peers RPC to identify validators 6-21
# No SSH required

set -e

OUTPUT_FILE="peer_id_mapping.json"
TEMP_DIR="/tmp/etrid_rpc_discovery"

mkdir -p "$TEMP_DIR"

# Known Azure VM Peer IDs
AZURE_VM1_PEER="12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm"
AZURE_VM2_PEER="12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb"
AZURE_VM3_PEER="12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6"

# Azure VM RPC endpoints
AZURE_VMS=(
    "20.69.26.209:9944|VM1"
    "20.186.91.207:9944|VM2"
    "52.252.142.146:9944|VM3"
)

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Peer ID Discovery via RPC               ║"
echo "║  Mapping Committee Structure Without SSH                    ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "PEER ID DISCOVERY VIA RPC"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Discovery Time: $(date)"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "PHASE 1: Query Connected Peers from Each Azure VM"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Collect peers from all Azure VMs
    > "$TEMP_DIR/all_peers.txt"

    for vm in "${AZURE_VMS[@]}"; do
        IFS='|' read -r rpc name <<< "$vm"
        echo "[$name] Querying peers from $rpc..."

        peers=$(curl -s -X POST \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_peers","params":[],"id":1}' \
            http://$rpc 2>/dev/null || echo "")

        if [[ -n "$peers" ]]; then
            echo "$peers" > "$TEMP_DIR/peers_${name}.json"

            peer_count=$(echo "$peers" | jq '.result | length' 2>/dev/null || echo "0")
            echo "  ✓ Found $peer_count peers"

            # Extract peer IDs
            echo "$peers" | jq -r '.result[].peerId' 2>/dev/null >> "$TEMP_DIR/all_peers.txt"
        else
            echo "  ✗ Could not connect"
        fi
        echo ""
    done

    echo "─────────────────────────────────────────────────────────────"
    echo "PHASE 2: Aggregate Unique Peer IDs"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Get unique peer IDs
    sort -u "$TEMP_DIR/all_peers.txt" > "$TEMP_DIR/unique_peers.txt"

    unique_count=$(wc -l < "$TEMP_DIR/unique_peers.txt" | tr -d ' ')
    echo "Total unique peer IDs discovered: $unique_count"
    echo ""

    echo "All unique peer IDs in network:"
    cat "$TEMP_DIR/unique_peers.txt" | nl
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "PHASE 3: Identify Azure VM Peer IDs"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "Known Azure VM Peer IDs:"
    echo "  VM1 (20.69.26.209):    $AZURE_VM1_PEER"
    echo "  VM2 (20.186.91.207):   $AZURE_VM2_PEER"
    echo "  VM3 (52.252.142.146):  $AZURE_VM3_PEER"
    echo ""

    # Check if Azure VMs appear in discovered peers
    vm1_in_network=false
    vm2_in_network=false
    vm3_in_network=false

    if grep -q "$AZURE_VM1_PEER" "$TEMP_DIR/unique_peers.txt"; then
        echo "  ✓ VM1 found in network"
        vm1_in_network=true
    else
        echo "  ✗ VM1 NOT found (expected - querying FROM this VM)"
    fi

    if grep -q "$AZURE_VM2_PEER" "$TEMP_DIR/unique_peers.txt"; then
        echo "  ✓ VM2 found in network"
        vm2_in_network=true
    else
        echo "  ✗ VM2 NOT found (expected - querying FROM this VM)"
    fi

    if grep -q "$AZURE_VM3_PEER" "$TEMP_DIR/unique_peers.txt"; then
        echo "  ✓ VM3 found in network"
        vm3_in_network=true
    else
        echo "  ✗ VM3 NOT found (expected - querying FROM this VM)"
    fi

    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "PHASE 4: Identify Validators 6-21 Peer IDs (By Elimination)"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "Peer IDs that are NOT Azure VMs (likely validators 6-21):"
    echo ""

    > "$TEMP_DIR/committee_peers.txt"
    while read -r peer; do
        if [[ "$peer" != "$AZURE_VM1_PEER" && "$peer" != "$AZURE_VM2_PEER" && "$peer" != "$AZURE_VM3_PEER" ]]; then
            echo "$peer" >> "$TEMP_DIR/committee_peers.txt"
        fi
    done < "$TEMP_DIR/unique_peers.txt"

    committee_peer_count=$(wc -l < "$TEMP_DIR/committee_peers.txt" | tr -d ' ')
    echo "Potential committee member peer IDs: $committee_peer_count"
    echo ""

    cat "$TEMP_DIR/committee_peers.txt" | nl
    echo ""

    if [[ $committee_peer_count -eq 16 ]]; then
        echo "✓ EXCELLENT: Found exactly 16 peer IDs"
        echo "  These are highly likely to be validators 6-21"
    elif [[ $committee_peer_count -lt 16 ]]; then
        echo "⚠ Found only $committee_peer_count peer IDs (expected 16)"
        echo "  Some validators may not be peered with Azure VMs yet"
    else
        echo "⚠ Found $committee_peer_count peer IDs (expected 16)"
        echo "  May include non-validator full nodes"
    fi

    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "PHASE 5: Analyze Peer Roles and Best Block"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "Analyzing peer metadata to identify validators..."
    echo ""

    # For each peer, extract role and best block
    for vm in "${AZURE_VMS[@]}"; do
        IFS='|' read -r rpc name <<< "$vm"

        if [[ -f "$TEMP_DIR/peers_${name}.json" ]]; then
            echo "From [$name]:"
            jq -r '.result[] | "\(.peerId) | Role: \(.roles) | Best: #\(.bestNumber // "N/A")"' \
                "$TEMP_DIR/peers_${name}.json" 2>/dev/null | \
                grep -v "$AZURE_VM1_PEER\|$AZURE_VM2_PEER\|$AZURE_VM3_PEER" | \
                head -20
            echo ""
        fi
    done

    echo "─────────────────────────────────────────────────────────────"
    echo "PHASE 6: Cross-Reference with Known Validator IPs"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "Known validator IPs (validators 6-21):"
    echo ""
    cat << 'EOF'
  6  | Runtime-Dev     | 20.224.104.239
  7  | Compiler-Dev    | 98.71.91.84
  8  | Network-Dev     | 20.169.114.25
  9  | SDK-Dev         | 20.75.92.203
  10 | DevTools-Dev    | 20.55.31.30
  11 | API-Dev         | 20.73.34.17
  12 | Docs-Dev        | 20.109.102.30
  13 | QA-Dev          | 52.250.61.132
  14 | Perf-Dev        | 20.218.66.251
  15 | Community-Dev   | 20.109.219.185
  16 | Analytics-Dev   | 20.83.208.17
  17 | Ethics-Dev      | 172.177.175.132
  18 | FlareNode-16    | 20.84.231.225
  19 | FlareNode-19    | 4.175.83.133
  20 | FlareNode-20    | 52.184.47.99
  21 | FlareNode-21    | 4.178.181.122
EOF
    echo ""

    echo "Attempting to map peer IDs to IPs..."
    echo "(Checking peer addresses from RPC metadata)"
    echo ""

    for vm in "${AZURE_VMS[@]}"; do
        IFS='|' read -r rpc name <<< "$vm"

        if [[ -f "$TEMP_DIR/peers_${name}.json" ]]; then
            echo "From [$name] - Peer addresses:"
            jq -r '.result[] | select(.peerId != "'$AZURE_VM1_PEER'" and .peerId != "'$AZURE_VM2_PEER'" and .peerId != "'$AZURE_VM3_PEER'") | "\(.peerId): \(.knownAddresses[0] // "no address")"' \
                "$TEMP_DIR/peers_${name}.json" 2>/dev/null | head -20
            echo ""
        fi
    done

    echo "─────────────────────────────────────────────────────────────"
    echo "PHASE 7: Export Structured Mapping"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    # Create JSON output
    {
        echo "{"
        echo "  \"discoveryTime\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
        echo "  \"azureVMs\": ["
        echo "    {"
        echo "      \"name\": \"VM1\","
        echo "      \"ip\": \"20.69.26.209\","
        echo "      \"peerId\": \"$AZURE_VM1_PEER\","
        echo "      \"validatorIndex\": 2,"
        echo "      \"inCommittee\": false"
        echo "    },"
        echo "    {"
        echo "      \"name\": \"VM2\","
        echo "      \"ip\": \"20.186.91.207\","
        echo "      \"peerId\": \"$AZURE_VM2_PEER\","
        echo "      \"validatorIndex\": 3,"
        echo "      \"inCommittee\": false"
        echo "    },"
        echo "    {"
        echo "      \"name\": \"VM3\","
        echo "      \"ip\": \"52.252.142.146\","
        echo "      \"peerId\": \"$AZURE_VM3_PEER\","
        echo "      \"validatorIndex\": 4,"
        echo "      \"inCommittee\": false"
        echo "    }"
        echo "  ],"
        echo "  \"committeeMembers\": ["

        # Add discovered peer IDs
        first=true
        while read -r peer; do
            if [[ "$first" == true ]]; then
                first=false
            else
                echo ","
            fi
            echo -n "    {"
            echo -n "\"peerId\": \"$peer\", "
            echo -n "\"validatorIndex\": \"unknown\", "
            echo -n "\"ip\": \"unknown\""
            echo -n "}"
        done < "$TEMP_DIR/committee_peers.txt"

        echo ""
        echo "  ],"
        echo "  \"summary\": {"
        echo "    \"totalUniquePeers\": $unique_count,"
        echo "    \"committeeMembers\": $committee_peer_count,"
        echo "    \"azureVMs\": 3,"
        echo "    \"expectedCommitteeSize\": 16"
        echo "  }"
        echo "}"
    } > "$OUTPUT_FILE"

    echo "Structured mapping exported to: $OUTPUT_FILE"
    echo ""

    echo "═══════════════════════════════════════════════════════════════"
    echo "DISCOVERY SUMMARY"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

    echo "Network Composition:"
    echo "  Total unique peers: $unique_count"
    echo "  Azure VMs: 3"
    echo "  Potential committee members: $committee_peer_count"
    echo ""

    if [[ $committee_peer_count -eq 16 ]]; then
        echo "✓ SUCCESS: Discovered all 16 committee member peer IDs"
        echo ""
        echo "Committee peer IDs (validators 6-21):"
        cat "$TEMP_DIR/committee_peers.txt" | nl
        echo ""
        echo "NOTE: Cannot determine exact validator index → peer ID mapping"
        echo "      without SSH access to validators or telemetry service."
    else
        echo "⚠ Partial discovery: $committee_peer_count peer IDs found"
        echo ""
        if [[ $committee_peer_count -lt 16 ]]; then
            echo "Possible reasons for incomplete discovery:"
            echo "  - Not all validators have peered with Azure VMs yet"
            echo "  - Some validators may be behind NAT/firewall"
            echo "  - Network partitioning preventing peer discovery"
        else
            echo "More peer IDs than expected:"
            echo "  - May include non-validator full nodes"
            echo "  - May include other network participants"
            echo "  - Requires role analysis to filter validators"
        fi
    fi

    echo ""
    echo "─────────────────────────────────────────────────────────────"
    echo "NEXT STEPS:"
    echo "─────────────────────────────────────────────────────────────"
    echo ""
    echo "1. Use query-validator-set.sh to get block author account IDs"
    echo "2. If telemetry enabled, check telemetry service for full mapping"
    echo "3. Monitor network as more validators connect to Azure VMs"
    echo "4. Proceed with integration strategy from COMMITTEE_ANALYSIS_AND_PLAN.md"
    echo ""

} | tee "${OUTPUT_FILE%.json}.txt"

echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "Discovery complete!"
echo "  JSON output: $OUTPUT_FILE"
echo "  Text report: ${OUTPUT_FILE%.json}.txt"
echo "  Raw data: $TEMP_DIR/"
echo ""
