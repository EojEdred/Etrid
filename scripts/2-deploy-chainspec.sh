#!/bin/bash
# Script 2: Deploy mainnet chainspec to all VMs
# Run from Mac after pulling from GitHub

set -e

echo "📦 DEPLOYING MAINNET CHAINSPEC TO ALL VMs"
echo "==========================================="
echo ""

CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json"

# Verify chainspec exists locally
if [ ! -f "$CHAINSPEC" ]; then
    echo "❌ Chainspec not found at: $CHAINSPEC"
    echo "Run 'git pull origin main' first"
    exit 1
fi

echo "✅ Found chainspec: $CHAINSPEC"
echo ""

# VM List - 3 Gizzi-controlled validator VMs
VMS=(
    "azureuser@20.69.26.209"      # eojedred-validator
    "azureuser@20.186.91.207"     # etrid-mainnet_127500e4
    "azureuser@52.252.142.146"    # SecurityDev
)

SSH_KEY="$HOME/.ssh/gizzi-validator"

for vm in "${VMS[@]}"; do
    echo "📡 Deploying to $vm..."

    # Create node directory if it doesn't exist
    ssh -i "$SSH_KEY" -o ConnectTimeout=10 -o StrictHostKeyChecking=no "$vm" "mkdir -p ~/flarechain-node" 2>/dev/null || {
        echo "   ❌ Cannot connect to $vm (offline?)"
        continue
    }

    # Copy chainspec
    scp -i "$SSH_KEY" -o ConnectTimeout=10 -o StrictHostKeyChecking=no "$CHAINSPEC" "$vm:~/flarechain-node/chainspec-mainnet-raw.json" && \
        echo "   ✅ Chainspec deployed" || \
        echo "   ❌ Failed to copy chainspec"
done

echo ""
echo "✅ Chainspec deployment complete"
echo ""
echo "📋 Next step: Run ./3-insert-all-keys.sh"
