#!/usr/bin/env bash
# Ëtrid FlareChain - Peer Discovery & Committee Mapping
# Method 3: Extract peer IDs from validator logs and build network map

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
OUTPUT_FILE="committee_peer_mapping.txt"
TEMP_DIR="/tmp/etrid_peer_discovery"

mkdir -p "$TEMP_DIR"

# Known Azure VM Peer IDs (from session)
AZURE_VM1_PEER="12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm"
AZURE_VM2_PEER="12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb"
AZURE_VM3_PEER="12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Peer Discovery & Committee Mapping      ║"
echo "║  Method 3: Peer Discovery Trace                             ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Clear output file
> "$OUTPUT_FILE"

echo "Starting peer discovery from validators 6-21..."
echo ""

# Phase 1: Extract own peer IDs from each validator
echo "=== PHASE 1: Extracting Validator Peer IDs ===" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

# Validator data: num|name|ip
cat > "$TEMP_DIR/validators.txt" << 'EOF'
6|Runtime-Dev|20.224.104.239
7|Compiler-Dev|98.71.91.84
8|Network-Dev|20.169.114.25
9|SDK-Dev|20.75.92.203
10|DevTools-Dev|20.55.31.30
11|API-Dev|20.73.34.17
12|Docs-Dev|20.109.102.30
13|QA-Dev|52.250.61.132
14|Perf-Dev|20.218.66.251
15|Community-Dev|20.109.219.185
16|Analytics-Dev|20.83.208.17
17|Ethics-Dev|172.177.175.132
18|FlareNode-16|20.84.231.225
19|FlareNode-19|4.175.83.133
20|FlareNode-20|52.184.47.99
21|FlareNode-21|4.178.181.122
EOF

> "$TEMP_DIR/validator_peers.csv"

while IFS='|' read -r val_num val_name val_ip; do
    echo -n "[$val_num] $val_name ($val_ip)... "

    # Extract peer ID from logs
    peer_id=$(ssh -i "$SSH_KEY" -o ConnectTimeout=10 -o StrictHostKeyChecking=no ubuntu@"$val_ip" \
        "sudo journalctl -u flarechain-validator 2>/dev/null | grep 'Local node identity' | tail -1 | awk '{print \$NF}'" 2>/dev/null || echo "")

    if [[ -n "$peer_id" && "$peer_id" != "FAILED" ]]; then
        echo "✓ $peer_id" | tee -a "$OUTPUT_FILE"
        echo "$val_num|$val_name|$val_ip|$peer_id" >> "$TEMP_DIR/validator_peers.csv"
    else
        echo "✗ Could not connect" | tee -a "$OUTPUT_FILE"
        echo "$val_num|$val_name|$val_ip|UNKNOWN" >> "$TEMP_DIR/validator_peers.csv"
    fi
done < "$TEMP_DIR/validators.txt"

echo "" | tee -a "$OUTPUT_FILE"
echo "=== PHASE 2: Discovering Network Peers ===" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

# Phase 2: From accessible validators, get discovered peer addresses
while IFS='|' read -r val_num val_name val_ip; do
    echo "Querying peers from [$val_num] $val_name..." | tee -a "$OUTPUT_FILE"

    # Get discovered external addresses (these show peer IDs with IPs)
    discovered=$(ssh -i "$SSH_KEY" -o ConnectTimeout=10 -o StrictHostKeyChecking=no ubuntu@"$val_ip" \
        "sudo journalctl -u flarechain-validator 2>/dev/null | grep 'Discovered new external' | tail -20" 2>/dev/null || echo "")

    if [[ -n "$discovered" ]]; then
        echo "$discovered" >> "$TEMP_DIR/discovered_peers_val${val_num}.txt"
        count=$(echo "$discovered" | wc -l)
        echo "  Found $count peer discoveries" | tee -a "$OUTPUT_FILE"
    else
        echo "  No data available" | tee -a "$OUTPUT_FILE"
    fi
done < "$TEMP_DIR/validators.txt"

echo "" | tee -a "$OUTPUT_FILE"
echo "=== PHASE 3: Building Committee Map ===" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

if [[ -f "$TEMP_DIR/validator_peers.csv" ]]; then
    echo "Validator # | Name              | IP Address       | Peer ID" | tee -a "$OUTPUT_FILE"
    echo "-----------|-------------------|------------------|------------------------------------------" | tee -a "$OUTPUT_FILE"

    sort -t'|' -k1 -n "$TEMP_DIR/validator_peers.csv" | while IFS='|' read -r num name ip peer; do
        printf "Val %-6s | %-17s | %-16s | %s\n" "$num" "$name" "$ip" "$peer" | tee -a "$OUTPUT_FILE"
    done
fi

echo "" | tee -a "$OUTPUT_FILE"
echo "=== PHASE 4: Azure VM Comparison ===" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

echo "Known Azure VM Peer IDs:" | tee -a "$OUTPUT_FILE"
echo "  VM1 (EojEdred):    $AZURE_VM1_PEER" | tee -a "$OUTPUT_FILE"
echo "  VM2 (Governance):  $AZURE_VM2_PEER" | tee -a "$OUTPUT_FILE"
echo "  VM3 (Security):    $AZURE_VM3_PEER" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

# Check if Azure VMs appear in discovered peers
echo "Checking if Azure VMs are discovered in network..." | tee -a "$OUTPUT_FILE"

if grep -r "$AZURE_VM1_PEER" "$TEMP_DIR"/ 2>/dev/null | grep -q "12D3Koo"; then
    echo "  ✓ VM1 found in network peer discovery" | tee -a "$OUTPUT_FILE"
else
    echo "  ✗ VM1 NOT found in network peer discovery" | tee -a "$OUTPUT_FILE"
fi

if grep -r "$AZURE_VM2_PEER" "$TEMP_DIR"/ 2>/dev/null | grep -q "12D3Koo"; then
    echo "  ✓ VM2 found in network peer discovery" | tee -a "$OUTPUT_FILE"
else
    echo "  ✗ VM2 NOT found in network peer discovery" | tee -a "$OUTPUT_FILE"
fi

if grep -r "$AZURE_VM3_PEER" "$TEMP_DIR"/ 2>/dev/null | grep -q "12D3Koo"; then
    echo "  ✓ VM3 found in network peer discovery" | tee -a "$OUTPUT_FILE"
else
    echo "  ✗ VM3 NOT found in network peer discovery" | tee -a "$OUTPUT_FILE"
fi

echo "" | tee -a "$OUTPUT_FILE"
echo "=== PHASE 5: Network Topology Analysis ===" | tee -a "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

# Count unique peer IDs discovered
if compgen -G "$TEMP_DIR/discovered_peers_*.txt" > /dev/null; then
    unique_peers=$(cat "$TEMP_DIR"/discovered_peers_*.txt 2>/dev/null | \
        grep -oE '12D3Koo[A-Za-z0-9]+' | sort -u | wc -l || echo "0")
    echo "Total unique peer IDs discovered: $unique_peers" | tee -a "$OUTPUT_FILE"

    # Show all unique peer IDs
    echo "" | tee -a "$OUTPUT_FILE"
    echo "All discovered peer IDs:" | tee -a "$OUTPUT_FILE"
    cat "$TEMP_DIR"/discovered_peers_*.txt 2>/dev/null | \
        grep -oE '12D3Koo[A-Za-z0-9]+' | sort -u | nl | tee -a "$OUTPUT_FILE"
else
    echo "No peer discovery data collected" | tee -a "$OUTPUT_FILE"
fi

echo "" | tee -a "$OUTPUT_FILE"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Analysis Complete!                                          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Results saved to: $OUTPUT_FILE"
echo "Raw data saved to: $TEMP_DIR/"
echo ""
echo "Next steps:"
echo "1. Review $OUTPUT_FILE for complete committee mapping"
echo "2. Check if Azure VMs appear in network"
echo "3. Identify validator positions (6-21)"
echo "4. Plan integration strategy based on findings"
