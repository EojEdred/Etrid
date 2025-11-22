#!/bin/bash
# Restore backup binaries on all validators

VMS=(vmi2896906 vmi2896907 vmi2896908 vmi2896909 vmi2896910 vmi2896911 vmi2896914 vmi2896915 vmi2896916 vmi2896917 vmi2896918 vmi2896921 vmi2896922 vmi2896923 vmi2896924 vmi2896925 vmi2897381 vmi2897382 vmi2897383 vmi2897384)
SSH_KEY="$HOME/.ssh/contabo-validators"

echo "════════════════════════════════════════════════════════════════════"
echo "Restoring backup binaries on all validators..."
echo "════════════════════════════════════════════════════════════════════"
echo ""

RESTORED=0
FAILED=0

for VM in "${VMS[@]}"; do
    echo "─── Restoring $VM..."

    if ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" '
        if [ -f /usr/local/bin/primearc-node.backup ]; then
            cp /usr/local/bin/primearc-node.backup /usr/local/bin/primearc-node
            chmod +x /usr/local/bin/primearc-node
            systemctl restart primearc-validator
            echo "restored"
        else
            echo "no_backup"
        fi
    ' 2>/dev/null | grep -q "restored"; then
        echo "    ✅ $VM restored"
        RESTORED=$((RESTORED + 1))
    else
        echo "    ❌ $VM failed"
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo "════════════════════════════════════════════════════════════════════"
echo "✅ Restored: $RESTORED/20 validators"
echo "❌ Failed: $FAILED/20 validators"
echo "════════════════════════════════════════════════════════════════════"
