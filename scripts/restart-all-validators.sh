#!/bin/bash
# Restart all validators to load the newly inserted keys

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

echo "Restarting all validators to load new keys..."
echo ""

for VM in "${VMS[@]}"; do
    echo "Restarting $VM..."
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no root@$VM 'systemctl restart flarechain-validator' 2>/dev/null
    echo "✅ $VM restarted"
done

echo ""
echo "All validators restarted! Waiting 30 seconds for startup..."
sleep 30

echo ""
echo "Checking status..."
ACTIVE=0
for VM in "${VMS[@]}"; do
    STATUS=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=5 root@$VM 'systemctl is-active flarechain-validator 2>/dev/null' 2>/dev/null)
    if [ "$STATUS" = "active" ]; then
        echo "✅ $VM: ACTIVE"
        ACTIVE=$((ACTIVE + 1))
    else
        echo "⚠️  $VM: $STATUS"
    fi
done

echo ""
echo "Summary: $ACTIVE/20 validators active"
