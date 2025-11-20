#!/bin/bash
# Purge v109 chain database and restart validators with fresh genesis

VALIDATORS=(
    "100.95.0.72" "100.86.111.37" "100.125.147.88" "100.80.84.82"
    "100.109.252.56" "100.117.43.53" "100.88.104.58" "100.70.73.10"
    "100.68.185.50" "100.71.127.127" "100.93.43.18" "100.124.117.73"
    "100.74.204.23" "100.125.251.60" "100.114.244.62" "100.113.226.111"
    "100.102.128.51" "100.71.242.104" "100.74.84.28" "100.89.102.75"
    "100.120.184.56" "100.82.45.88"
)

SSH_KEY="~/.ssh/contabo-validators"
CHAIN_ID="flarechain_prod_21val_v109"

echo "=========================================="
echo "Purge v109 Database & Restart Validators"
echo "=========================================="
echo ""
echo "⚠️  This will DELETE all chain data for: $CHAIN_ID"
echo "   Network keys will be PRESERVED"
echo ""

SUCCESS_COUNT=0
FAIL_COUNT=0

for IP in "${VALIDATORS[@]}"; do
    echo "→ $IP"

    # Stop validator
    ssh -i $SSH_KEY root@${IP} 'sudo systemctl stop flarechain-validator' 2>/dev/null

    # Purge chain database (preserves network keys)
    PURGE_OUTPUT=$(ssh -i $SSH_KEY root@${IP} "
        if [ -d /root/flarechain-data/chains/${CHAIN_ID} ]; then
            # Preserve network keys
            mkdir -p /tmp/network-backup
            cp -r /root/flarechain-data/chains/${CHAIN_ID}/network /tmp/network-backup/ 2>/dev/null

            # Remove chain data
            rm -rf /root/flarechain-data/chains/${CHAIN_ID}

            # Restore network keys
            mkdir -p /root/flarechain-data/chains/${CHAIN_ID}/network
            cp -r /tmp/network-backup/network/* /root/flarechain-data/chains/${CHAIN_ID}/network/ 2>/dev/null
            rm -rf /tmp/network-backup

            echo 'PURGED'
        else
            echo 'NO_DATA'
        fi
    " 2>/dev/null)

    if [[ "$PURGE_OUTPUT" == *"PURGED"* ]]; then
        echo "  ✓ Database purged (network keys preserved)"
        ((SUCCESS_COUNT++))
    elif [[ "$PURGE_OUTPUT" == *"NO_DATA"* ]]; then
        echo "  ℹ No database found (fresh start)"
        ((SUCCESS_COUNT++))
    else
        echo "  ✗ Failed to purge database"
        ((FAIL_COUNT++))
    fi

    echo ""
done

echo "========================================"
echo "RESULT: $SUCCESS_COUNT purged, $FAIL_COUNT failed"
echo "========================================"
echo ""

if [ $SUCCESS_COUNT -ge 18 ]; then
    echo "✅ Database purged on $SUCCESS_COUNT validators"
    echo ""
    echo "Starting all validators with fresh genesis..."
    echo ""

    for IP in "${VALIDATORS[@]}"; do
        ssh -i $SSH_KEY root@${IP} 'sudo systemctl start flarechain-validator' 2>/dev/null &
    done
    wait

    echo "✅ All validators started"
    echo ""
    echo "⏳ Waiting 45 seconds for genesis initialization..."
    sleep 45
    echo ""
    echo "Checking validator logs for 21-member committee initialization:"
    echo ""
    ssh -i $SSH_KEY root@100.95.0.72 'journalctl -u flarechain-validator -n 150 --no-pager | grep -E "ValidatorCommittee|committee|Committee size|genesis|Genesis|Initialized|Imported|block|Block|epoch|Epoch" | tail -40'

    exit 0
else
    echo "❌ Only $SUCCESS_COUNT/22 validators purged successfully."
    echo ""
    echo "Manual recovery steps:"
    echo "  1. Check SSH connectivity to failed validators"
    echo "  2. Manually purge: rm -rf /root/flarechain-data/chains/$CHAIN_ID"
    echo "  3. Start validator: systemctl start flarechain-validator"
    exit 1
fi
