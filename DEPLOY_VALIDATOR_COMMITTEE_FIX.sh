#!/bin/bash
# Deploy ValidatorCommittee Fix to Production Validators
# This deploys the binary with the serde fix for ValidatorCommittee genesis parsing

set -e

VALIDATORS=(
    "100.95.0.72" "100.86.111.37" "100.125.147.88" "100.80.84.82"
    "100.109.252.56" "100.117.43.53" "100.88.104.58" "100.70.73.10"
    "100.68.185.50" "100.71.127.127" "100.93.43.18" "100.124.117.73"
    "100.74.204.23" "100.125.251.60" "100.114.244.62" "100.113.226.111"
    "100.102.128.51" "100.71.242.104" "100.74.84.28" "100.89.102.75"
    "100.120.184.56" "100.82.45.88"
)

SSH_KEY="~/.ssh/contabo-validators"
TEST_VALIDATOR="100.95.0.72"
BINARY_PATH="/tmp/flarechain-node-v109"
CHAINSPEC_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/chainspec_v109_FIXED.json"

echo "=========================================="
echo "Deploy ValidatorCommittee Fix"
echo "=========================================="
echo ""
echo "Fix: Removed conflicting serde derives"
echo "Binary: $BINARY_PATH"
echo "Chainspec: $CHAINSPEC_PATH"
echo ""

# Verify binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Binary not found at $BINARY_PATH"
    echo "Please build first: cd 05-multichain/flare-chain && cargo build --release"
    echo "Then copy: cp target/release/etrid $BINARY_PATH"
    exit 1
fi

# Verify chainspec exists
if [ ! -f "$CHAINSPEC_PATH" ]; then
    echo "❌ Chainspec not found at $CHAINSPEC_PATH"
    exit 1
fi

echo "→ Testing on single validator first: $TEST_VALIDATOR"
echo ""

# Stop test validator
echo "→ Stopping validator..."
ssh -i $SSH_KEY root@${TEST_VALIDATOR} 'sudo systemctl stop flarechain-validator' || {
    echo "⚠️  Could not stop validator (may already be stopped)"
}

# Upload binary
echo "→ Uploading binary..."
scp -i $SSH_KEY "$BINARY_PATH" root@${TEST_VALIDATOR}:/usr/local/bin/flarechain-node || {
    echo "❌ Failed to upload binary"
    exit 1
}
echo "✓ Binary uploaded"

# Upload chainspec
echo "→ Uploading chainspec..."
scp -i $SSH_KEY "$CHAINSPEC_PATH" root@${TEST_VALIDATOR}:/root/chainspec_v109_FIXED.json || {
    echo "❌ Failed to upload chainspec"
    exit 1
}
echo "✓ Chainspec uploaded"

# Update systemd service to use fixed chainspec
echo "→ Updating systemd service..."
ssh -i $SSH_KEY root@${TEST_VALIDATOR} "
    sed -i 's|--chain /root/chainspec_v109.json|--chain /root/chainspec_v109_FIXED.json|g' /etc/systemd/system/flarechain-validator.service
    sed -i 's|--chain chainspec_v109.json|--chain /root/chainspec_v109_FIXED.json|g' /etc/systemd/system/flarechain-validator.service
    systemctl daemon-reload
" || {
    echo "❌ Failed to update systemd service"
    exit 1
}
echo "✓ Service updated"

# Purge old chain data
echo "→ Purging old chain database..."
ssh -i $SSH_KEY root@${TEST_VALIDATOR} "
    CHAIN_ID='flarechain_prod_21val_v109'
    if [ -d /root/flarechain-data/chains/\${CHAIN_ID} ]; then
        # Preserve network keys
        mkdir -p /tmp/network-backup
        cp -r /root/flarechain-data/chains/\${CHAIN_ID}/network /tmp/network-backup/ 2>/dev/null || true

        # Remove chain data
        rm -rf /root/flarechain-data/chains/\${CHAIN_ID}

        # Restore network keys
        mkdir -p /root/flarechain-data/chains/\${CHAIN_ID}/network
        cp -r /tmp/network-backup/network/* /root/flarechain-data/chains/\${CHAIN_ID}/network/ 2>/dev/null || true
        rm -rf /tmp/network-backup

        echo 'Database purged, network keys preserved'
    else
        echo 'No existing database found'
    fi
"
echo "✓ Database purged"

# Start validator
echo "→ Starting validator..."
ssh -i $SSH_KEY root@${TEST_VALIDATOR} 'sudo systemctl start flarechain-validator'
echo "✓ Validator started"
echo ""

# Wait and check logs
echo "⏳ Waiting 45 seconds for initialization..."
sleep 45
echo ""

echo "→ Checking logs for ValidatorCommittee initialization..."
LOGS=$(ssh -i $SSH_KEY root@${TEST_VALIDATOR} 'journalctl -u flarechain-validator -n 100 --no-pager')

if echo "$LOGS" | grep -q "Error parsing"; then
    echo "❌ FAILED - Genesis parsing error still occurring!"
    echo ""
    echo "Last 50 lines of logs:"
    echo "$LOGS" | tail -50
    exit 1
elif echo "$LOGS" | grep -q "unknown field"; then
    echo "❌ FAILED - Unknown field error!"
    echo ""
    echo "Last 50 lines of logs:"
    echo "$LOGS" | tail -50
    exit 1
elif echo "$LOGS" | grep -qE "(Initialized|genesis|committee|block 0|Imported #0)"; then
    echo "✅ SUCCESS - Genesis initialized!"
    echo ""
    echo "Relevant log lines:"
    echo "$LOGS" | grep -iE "(initialized|genesis|committee|validator|imported #0|block 0)" | tail -20
    echo ""
else
    echo "⚠️  Validator started but no clear success indicators"
    echo ""
    echo "Last 50 lines of logs:"
    echo "$LOGS" | tail -50
    echo ""
fi

echo ""
echo "=========================================="
echo "Test Complete on $TEST_VALIDATOR"
echo "=========================================="
echo ""
read -p "Deploy to all 22 validators? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Deployment cancelled."
    exit 0
fi

echo ""
echo "=========================================="
echo "Deploying to All Validators"
echo "=========================================="
echo ""

SUCCESS_COUNT=0
FAIL_COUNT=0

for IP in "${VALIDATORS[@]}"; do
    echo "→ $IP"

    # Stop validator
    ssh -i $SSH_KEY root@${IP} 'sudo systemctl stop flarechain-validator' 2>/dev/null || true

    # Deploy binary
    if scp -i $SSH_KEY "$BINARY_PATH" root@${IP}:/usr/local/bin/flarechain-node 2>/dev/null; then
        # Upload chainspec
        scp -i $SSH_KEY "$CHAINSPEC_PATH" root@${IP}:/root/chainspec_v109_FIXED.json 2>/dev/null

        # Update systemd and purge database
        ssh -i $SSH_KEY root@${IP} "
            sed -i 's|--chain /root/chainspec_v109.json|--chain /root/chainspec_v109_FIXED.json|g' /etc/systemd/system/flarechain-validator.service
            sed -i 's|--chain chainspec_v109.json|--chain /root/chainspec_v109_FIXED.json|g' /etc/systemd/system/flarechain-validator.service
            systemctl daemon-reload

            # Purge database with network key preservation
            CHAIN_ID='flarechain_prod_21val_v109'
            if [ -d /root/flarechain-data/chains/\${CHAIN_ID} ]; then
                mkdir -p /tmp/network-backup
                cp -r /root/flarechain-data/chains/\${CHAIN_ID}/network /tmp/network-backup/ 2>/dev/null || true
                rm -rf /root/flarechain-data/chains/\${CHAIN_ID}
                mkdir -p /root/flarechain-data/chains/\${CHAIN_ID}/network
                cp -r /tmp/network-backup/network/* /root/flarechain-data/chains/\${CHAIN_ID}/network/ 2>/dev/null || true
                rm -rf /tmp/network-backup
            fi
        " 2>/dev/null

        echo "  ✓ Deployed"
        ((SUCCESS_COUNT++))
    else
        echo "  ✗ Failed"
        ((FAIL_COUNT++))
    fi

    echo ""
done

echo "========================================"
echo "RESULT: $SUCCESS_COUNT successful, $FAIL_COUNT failed"
echo "========================================"
echo ""

if [ $SUCCESS_COUNT -ge 18 ]; then
    echo "✅ Binary deployed to $SUCCESS_COUNT validators"
    echo ""
    echo "Starting all validators..."
    echo ""

    for IP in "${VALIDATORS[@]}"; do
        ssh -i $SSH_KEY root@${IP} 'sudo systemctl start flarechain-validator' 2>/dev/null &
    done
    wait

    echo "✅ All validators started"
    echo ""
    echo "⏳ Waiting 60 seconds for genesis initialization..."
    sleep 60
    echo ""
    echo "Checking logs on test validator:"
    ssh -i $SSH_KEY root@${TEST_VALIDATOR} 'journalctl -u flarechain-validator -n 150 --no-pager | grep -iE "(initialized|genesis|committee|validator|imported|block)" | tail -30'

    exit 0
else
    echo "❌ Only $SUCCESS_COUNT/22 validators deployed successfully."
    echo ""
    echo "Manual recovery needed for failed validators."
    exit 1
fi
