#!/bin/bash
# Script 1: Purge chain databases on all VMs
# Run from Mac to SSH into each VM and purge

set -e

echo "🗑️  PURGING CHAIN DATABASES ON ALL VMs"
echo "========================================"
echo ""
echo "⚠️  WARNING: This will DELETE all blockchain data on all VMs"
read -p "Continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "❌ Aborted"
    exit 1
fi

# VM List - 3 Gizzi-controlled validator VMs
VMS=(
    "azureuser@20.69.26.209"      # eojedred-validator
    "azureuser@20.186.91.207"     # etrid-mainnet_127500e4
    "azureuser@52.252.142.146"    # SecurityDev
)

SSH_KEY="$HOME/.ssh/gizzi-validator"

for vm in "${VMS[@]}"; do
    echo ""
    echo "📡 Connecting to $vm..."

    ssh -i "$SSH_KEY" -o ConnectTimeout=10 -o StrictHostKeyChecking=no "$vm" bash << 'ENDSSH'
        echo "   🛑 Stopping flarechain-node..."
        sudo pkill -f flarechain-node || echo "   No node running"
        sleep 2

        echo "   🗑️  Purging database..."
        if [ -d ~/flarechain-node/data ]; then
            rm -rf ~/flarechain-node/data
            echo "   ✅ Database purged"
        else
            echo "   ℹ️  No database found"
        fi

        echo "   📁 Creating fresh data directory..."
        mkdir -p ~/flarechain-node/data
        echo "   ✅ Ready for fresh start"
ENDSSH

    if [ $? -eq 0 ]; then
        echo "   ✅ $vm completed"
    else
        echo "   ❌ $vm failed (might be offline)"
    fi
done

echo ""
echo "✅ Purge complete on all accessible VMs"
echo ""
echo "📋 Next step: Run ./2-deploy-chainspec.sh"
