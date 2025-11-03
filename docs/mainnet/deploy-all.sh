#!/bin/bash
# Master deployment script - deploys chainspec to all 21 validators
# Usage: ./deploy-all.sh

set -e

CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"

# Verify files exist
if [ ! -f "$CHAINSPEC" ]; then
    echo "❌ Chainspec not found: $CHAINSPEC"
    exit 1
fi

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found: $BINARY"
    exit 1
fi

echo "╔════════════════════════════════════════════════════════════╗"
echo "║    Ëtrid FlareChain - 21 Validator Deployment             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Files to deploy:"
echo "  Chainspec: $CHAINSPEC ($(ls -lh $CHAINSPEC | awk '{print $5}'))"
echo "  Binary:    $BINARY ($(ls -lh $BINARY | awk '{print $5}'))"
echo ""

# Deploy function
deploy_to_vm() {
    local VM=$1
    local USER=$2
    local NAME=$3

    echo "→ Deploying to $NAME ($VM)..."

    # Create directories
    ssh $USER@$VM "sudo mkdir -p /var/lib/flarechain /usr/local/bin" 2>/dev/null || {
        echo "  ⚠️  SSH failed for $VM - skipping"
        return
    }

    # Copy files
    scp -q $CHAINSPEC $USER@$VM:/tmp/chainspec.json
    scp -q $BINARY $USER@$VM:/tmp/flarechain-node

    # Move to final location
    ssh $USER@$VM "
        sudo mv /tmp/chainspec.json /var/lib/flarechain/chainspec-mainnet-raw.json
        sudo mv /tmp/flarechain-node /usr/local/bin/flarechain-node
        sudo chmod +x /usr/local/bin/flarechain-node
        sudo chown -R $USER:$USER /var/lib/flarechain
    "

    echo "  ✓ $NAME deployed"
}

# Export function for parallel execution
export -f deploy_to_vm
export CHAINSPEC
export BINARY

# List of all validators (UPDATE THESE WITH YOUR ACTUAL IPs!)
declare -A VALIDATORS=(
    # Bootstrap Validators (5)
    ["64.181.215.19"]="ubuntu:Gizzi"
    ["localhost"]="$USER:EojEdred"
    ["TBD1"]="ubuntu:governance-dev01"
    ["52.252.142.146"]="azureuser:security-dev01"
    ["129.80.122.34"]="ubuntu:audit-dev01"

    # Validity Nodes (16) - ADD YOUR VM IPs HERE
    # ["VM_IP"]="username:ValidatorName"
)

echo "Starting parallel deployment to ${#VALIDATORS[@]} validators..."
echo ""

# Deploy in parallel (4 at a time to avoid overwhelming network)
COUNT=0
for VM in "${!VALIDATORS[@]}"; do
    IFS=':' read -r USER NAME <<< "${VALIDATORS[$VM]}"

    if [ "$VM" == "localhost" ]; then
        echo "→ Deploying locally ($NAME)..."
        sudo mkdir -p /var/lib/flarechain /usr/local/bin
        sudo cp $CHAINSPEC /var/lib/flarechain/chainspec-mainnet-raw.json
        sudo cp $BINARY /usr/local/bin/flarechain-node
        sudo chmod +x /usr/local/bin/flarechain-node
        echo "  ✓ $NAME deployed"
    elif [ "$VM" == "TBD1" ]; then
        echo "  ⏭️  Skipping $NAME (IP not configured)"
    else
        deploy_to_vm "$VM" "$USER" "$NAME" &
    fi

    # Limit to 4 parallel deployments at a time
    COUNT=$((COUNT + 1))
    if [ $((COUNT % 4)) -eq 0 ]; then
        wait
    fi
done

# Wait for remaining deployments
wait

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                 Deployment Complete!                       ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo "  1. Start nodes on each VM (see QUICK_START.md)"
echo "  2. Insert session keys for each validator"
echo "  3. Verify network connectivity"
echo ""
