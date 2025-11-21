#!/bin/bash
# Enable unsafe RPC methods on all validators

SSH_KEY="$HOME/.ssh/contabo-validators"

VMS=(
  vmi2896906
  vmi2896907
  vmi2896908
  vmi2896909
  vmi2896910
  vmi2896911
  vmi2896914
  vmi2896915
  vmi2896916
  vmi2896917
  vmi2896918
  vmi2896921
  vmi2896922
  vmi2896923
  vmi2896924
  vmi2896925
  vmi2897381
  vmi2897382
  vmi2897383
  vmi2897384
)

echo "Enabling unsafe RPC methods on all validators..."
echo ""

for VM in "${VMS[@]}"; do
    echo "Updating $VM..."

    # Stop service
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" 'systemctl stop flarechain-validator' 2>/dev/null

    # Update systemd file to add --rpc-methods=unsafe
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" \
        'sed -i "s/--unsafe-rpc-external/--unsafe-rpc-external \\\\\n    --rpc-methods unsafe/" /etc/systemd/system/flarechain-validator.service' 2>/dev/null

    # Reload systemd and start service
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" \
        'systemctl daemon-reload && systemctl start flarechain-validator' 2>/dev/null

    echo "✅ $VM updated and restarted"
done

echo ""
echo "All validators updated! Waiting 20 seconds for services to start..."
sleep 20

echo ""
echo "Checking status..."
ACTIVE=0
for VM in "${VMS[@]}"; do
    STATUS=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 "root@$VM" 'systemctl is-active flarechain-validator 2>/dev/null' 2>/dev/null)
    if [ "$STATUS" = "active" ]; then
        echo "✅ $VM: ACTIVE"
        ACTIVE=$((ACTIVE + 1))
    else
        echo "⚠️  $VM: $STATUS"
    fi
done

echo ""
echo "Summary: $ACTIVE/20 validators active"
