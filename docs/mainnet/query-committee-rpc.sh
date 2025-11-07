#!/usr/bin/env bash
# Alternative Method: Query committee via RPC instead of SSH

# Try Azure VMs first (if IPs updated)
AZURE_VMS=(
    "20.69.26.209:9944"    # VM1
    "20.186.91.207:9944"   # VM2
    "52.252.142.146:9944"  # VM3
)

echo "=== Querying FlareChain Committee via RPC ==="
echo ""

# Try each RPC endpoint
for rpc in "${AZURE_VMS[@]}"; do
    echo "Trying RPC: $rpc"

    # Query current validator set
    curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"author_hasSessionKeys","params":[],"id":1}' \
        http://$rpc 2>/dev/null || echo "  Connection failed"

    echo ""
done

# Query validator set from genesis
echo ""
echo "=== Alternative: Check peer count ==="
for rpc in "${AZURE_VMS[@]}"; do
    echo "RPC: $rpc"
    curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_peers","params":[],"id":1}' \
        http://$rpc | jq '.result | length' 2>/dev/null || echo "  Failed"
    echo ""
done
