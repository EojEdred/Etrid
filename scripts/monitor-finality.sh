#!/bin/bash
# Monitor finality progression across validators

VMS=(vmi2896906 vmi2896907 vmi2896908 vmi2896909 vmi2896910)
SSH_KEY="$HOME/.ssh/contabo-validators"

echo "════════════════════════════════════════════════════════════════════"
echo "Finality Monitor - Checking 5 validators"
echo "════════════════════════════════════════════════════════════════════"
echo ""

for VM in "${VMS[@]}"; do
    echo "─── $VM ───"
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no "root@$VM" \
        'journalctl -u primearc-validator -n 3 --no-pager | tail -1' 2>/dev/null | \
        grep -oE "(best: #[0-9]+|finalized #[0-9]+|peers\))" || echo "  No data"
    echo ""
done

echo "════════════════════════════════════════════════════════════════════"
