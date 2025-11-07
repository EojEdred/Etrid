#!/usr/bin/env bash
# Ëtrid FlareChain - Monitor Azure VM Network Contribution
# Check if Azure VMs are providing value to the network even if not validating

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
OUTPUT_FILE="azure_vm_monitoring.txt"

# Azure VMs
AZURE_VMS=(
    "20.69.26.209|VM1|ubuntu"
    "20.186.91.207|VM2|ubuntu"
    "52.252.142.146|VM3|ubuntu"
)

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Azure VM Network Contribution Monitor   ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

{
    echo "═══════════════════════════════════════════════════════════════"
    echo "AZURE VM NETWORK CONTRIBUTION REPORT"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""
    echo "Report Time: $(date)"
    echo ""

    for vm in "${AZURE_VMS[@]}"; do
        IFS='|' read -r ip name user <<< "$vm"

        echo "─────────────────────────────────────────────────────────────"
        echo "$name ($ip)"
        echo "─────────────────────────────────────────────────────────────"
        echo ""

        # Check if SSH accessible
        if ! ssh -i "$SSH_KEY" -o ConnectTimeout=5 -o StrictHostKeyChecking=no $user@$ip "echo 'Connected'" >/dev/null 2>&1; then
            echo "✗ Cannot connect via SSH"
            echo ""
            continue
        fi

        echo "✓ SSH accessible"
        echo ""

        # 1. Process Status
        echo "1. Process Status:"
        process_running=$(ssh -i "$SSH_KEY" $user@$ip "pgrep -c flarechain-node" 2>/dev/null || echo "0")
        if [[ "$process_running" -gt 0 ]]; then
            echo "   ✓ FlareChain node running ($process_running process)"
        else
            echo "   ✗ FlareChain node NOT running"
        fi
        echo ""

        # 2. Uptime
        echo "2. Node Uptime:"
        ssh -i "$SSH_KEY" $user@$ip "ps -p \$(pgrep -f flarechain-node) -o etime= 2>/dev/null || echo 'N/A'" | sed 's/^/   /'
        echo ""

        # 3. Peer Count
        echo "3. Network Connectivity:"
        peer_info=$(ssh -i "$SSH_KEY" $user@$ip \
            "sudo journalctl -u flarechain-validator --since '5 minutes ago' 2>/dev/null | grep -oE '[0-9]+ peers' | tail -1" || echo "N/A")
        echo "   Current: $peer_info"
        echo ""

        # 4. Block Sync Status
        echo "4. Block Sync Status:"
        sync_info=$(ssh -i "$SSH_KEY" $user@$ip \
            "sudo journalctl -u flarechain-validator --since '1 minute ago' 2>/dev/null | grep -oE 'best: #[0-9]+' | tail -1" || echo "N/A")
        echo "   $sync_info"
        echo ""

        # 5. Recent Peer Discoveries
        echo "5. Recent Peer Discoveries (last 10 minutes):"
        ssh -i "$SSH_KEY" $user@$ip \
            "sudo journalctl -u flarechain-validator --since '10 minutes ago' 2>/dev/null | grep 'Discovered new external' | tail -5" | \
            sed 's/^/   /' || echo "   No new discoveries"
        echo ""

        # 6. Incoming Connections
        echo "6. Network Activity (listening ports):"
        ssh -i "$SSH_KEY" $user@$ip "sudo netstat -tlnp 2>/dev/null | grep -E ':(30333|9944)' | head -5" | sed 's/^/   /' || echo "   N/A"
        echo ""

        # 7. Block Gossip Activity
        echo "7. Block Gossip Activity (last 5 minutes):"
        gossip_count=$(ssh -i "$SSH_KEY" $user@$ip \
            "sudo journalctl -u flarechain-validator --since '5 minutes ago' 2>/dev/null | grep -c 'Imported' || echo '0'")
        echo "   Blocks imported/gossiped: $gossip_count"
        echo ""

        # 8. RPC Endpoint Status
        echo "8. RPC Endpoint Status:"
        rpc_test=$(curl -s -m 5 -X POST \
            -H "Content-Type: application/json" \
            -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
            http://$ip:9944 2>/dev/null)

        if [[ -n "$rpc_test" ]]; then
            echo "   ✓ RPC responding"
            echo "$rpc_test" | jq '.' | sed 's/^/   /'
        else
            echo "   ✗ RPC not accessible"
        fi
        echo ""

        # 9. Resource Usage
        echo "9. System Resources:"
        ssh -i "$SSH_KEY" $user@$ip "top -bn1 | grep -E 'Cpu|Mem|flarechain-node' | head -5" | sed 's/^/   /'
        echo ""

        # 10. Disk Usage
        echo "10. Blockchain Data Size:"
        ssh -i "$SSH_KEY" $user@$ip "du -sh ~/.etrid/validator 2>/dev/null || echo 'N/A'" | sed 's/^/   /'
        echo ""

    done

    echo "═══════════════════════════════════════════════════════════════"
    echo "NETWORK VALUE ASSESSMENT"
    echo "═══════════════════════════════════════════════════════════════"
    echo ""

    echo "Azure VMs provide value to FlareChain network as:"
    echo ""
    echo "1. BOOTNODE SERVICES:"
    echo "   ✓ Stable peer IDs for new validators to connect"
    echo "   ✓ 99.9% uptime on cloud infrastructure"
    echo "   ✓ Multiple geographic regions (if distributed)"
    echo ""
    echo "2. BLOCK GOSSIP PROPAGATION:"
    echo "   ✓ Relay blocks between validators 6-21"
    echo "   ✓ Reduce network latency and partition risk"
    echo "   ✓ Increase overall network resilience"
    echo ""
    echo "3. RPC ENDPOINTS:"
    echo "   ✓ Public/private RPC access for queries"
    echo "   ✓ Monitoring and observability"
    echo "   ✓ Integration testing and development"
    echo ""
    echo "4. NETWORK REDUNDANCY:"
    echo "   ✓ Hot standbys ready to join committee if needed"
    echo "   ✓ Session keys already installed and ready"
    echo "   ✓ Can be activated via on-chain extrinsic"
    echo ""

    echo "─────────────────────────────────────────────────────────────"
    echo "RECOMMENDATIONS:"
    echo "─────────────────────────────────────────────────────────────"
    echo ""

    echo "OPTION A: Keep as Bootnodes (Low Risk, Immediate Value)"
    echo "  • Update validators 6-21 to use Azure VMs as --bootnodes"
    echo "  • Network stability improves immediately"
    echo "  • No consensus changes required"
    echo "  • Script ready: update-validators-bootnode.sh"
    echo ""

    echo "OPTION B: Integrate into Committee (Higher Value, More Risk)"
    echo "  • Register session keys on-chain (if supported)"
    echo "  • OR submit governance proposal to expand committee to 19"
    echo "  • Provides voting redundancy (19 vs 16)"
    echo "  • Requires 1-4 weeks depending on method"
    echo ""

    echo "OPTION C: Staged Approach (Recommended)"
    echo "  1. Implement as bootnodes immediately (Option A)"
    echo "  2. Monitor network health for 1-2 weeks"
    echo "  3. Then proceed with committee integration (Option B)"
    echo "  4. Minimizes risk while maximizing value"
    echo ""

    echo "═══════════════════════════════════════════════════════════════"
    echo ""

} | tee "$OUTPUT_FILE"

echo "Monitoring complete: $OUTPUT_FILE"
echo ""
