#!/bin/bash
# Update flarechain binary on all VMs

VMS="vmi2896906 vmi2896907 vmi2896908 vmi2896909 vmi2896910 vmi2896911 vmi2896914 vmi2896915 vmi2896916 vmi2896917 vmi2896918 vmi2896921 vmi2896922 vmi2896923 vmi2896924 vmi2896925 vmi2897381 vmi2897382 vmi2897383 vmi2897384"
SSH_KEY="$HOME/.ssh/contabo-validators"
BINARY_PATH="/Users/macbook/Desktop/etrid/target/release/flarechain-node"

echo "Updating binary on all VMs..."
echo "Binary: $(ls -lh $BINARY_PATH | awk '{print $5}')"
echo ""

for VM in $VMS; do
    echo "Updating $VM..."
    # Stop service
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" 'systemctl stop flarechain-validator' 2>/dev/null

    # Update binary
    cat "$BINARY_PATH" | ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" \
        'cat > /usr/local/bin/flarechain-node && chmod +x /usr/local/bin/flarechain-node'

    # Start service
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" 'systemctl start flarechain-validator' 2>/dev/null

    echo "✅ $VM updated"
done

echo ""
echo "All binaries updated! Waiting 10 seconds for services to start..."
sleep 10

echo ""
echo "Checking status..."
for VM in $VMS; do
    STATUS=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "root@$VM" 'systemctl is-active flarechain-validator 2>/dev/null' 2>/dev/null)
    if [ "$STATUS" = "active" ]; then
        echo "✅ $VM: ACTIVE"
    else
        echo "❌ $VM: $STATUS"
    fi
done
