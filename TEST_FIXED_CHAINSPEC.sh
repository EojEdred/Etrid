#!/bin/bash
# Test fixed chainspec with camelCase field names on single validator

TEST_VALIDATOR="100.95.0.72"
SSH_KEY="~/.ssh/contabo-validators"
CHAIN_ID="flarechain_prod_21val_v109"

echo "=========================================="
echo "Test Fixed Chainspec (camelCase fields)"
echo "=========================================="
echo ""
echo "Test validator: $TEST_VALIDATOR"
echo ""

# Upload fixed chainspec
echo "→ Uploading fixed chainspec..."
scp -i $SSH_KEY /Users/macbook/Desktop/etrid/chainspec_v109_FIXED.json root@${TEST_VALIDATOR}:/root/chainspec_v109_FIXED.json

if [ $? -ne 0 ]; then
    echo "❌ Failed to upload chainspec"
    exit 1
fi
echo "✓ Chainspec uploaded"
echo ""

# Stop validator
echo "→ Stopping validator..."
ssh -i $SSH_KEY root@${TEST_VALIDATOR} 'sudo systemctl stop flarechain-validator'
echo "✓ Validator stopped"
echo ""

# Purge database (preserve network keys)
echo "→ Purging chain database..."
ssh -i $SSH_KEY root@${TEST_VALIDATOR} "
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

        echo 'Database purged'
    fi
"
echo "✓ Database purged"
echo ""

# Test chainspec validation
echo "→ Testing chainspec parsing..."
PARSE_TEST=$(ssh -i $SSH_KEY root@${TEST_VALIDATOR} "/usr/local/bin/flarechain-node --chain /root/chainspec_v109_FIXED.json --base-path /tmp/test-chainspec --tmp 2>&1 | head -20")

if echo "$PARSE_TEST" | grep -q "Error parsing"; then
    echo "❌ Chainspec parsing FAILED!"
    echo ""
    echo "$PARSE_TEST"
    exit 1
elif echo "$PARSE_TEST" | grep -q "Storage"; then
    echo "❌ Chainspec parsing FAILED with storage error!"
    echo ""
    echo "$PARSE_TEST"
    exit 1
else
    echo "✓ Chainspec parsing successful!"
    echo ""
    echo "First 15 lines of output:"
    echo "$PARSE_TEST" | head -15
fi
echo ""

# Clean up test
ssh -i $SSH_KEY root@${TEST_VALIDATOR} 'rm -rf /tmp/test-chainspec'

# Start validator with fixed chainspec
echo "→ Starting validator with fixed chainspec..."
ssh -i $SSH_KEY root@${TEST_VALIDATOR} "
    # Update systemd service to use fixed chainspec
    sed -i 's|/root/chainspec_v109.json|/root/chainspec_v109_FIXED.json|g' /etc/systemd/system/flarechain-validator.service

    # Reload systemd
    systemctl daemon-reload

    # Start validator
    systemctl start flarechain-validator
"
echo "✓ Validator started"
echo ""

# Wait for initialization
echo "⏳ Waiting 30 seconds for genesis initialization..."
sleep 30
echo ""

# Check logs
echo "→ Checking validator logs for 21-member committee..."
LOGS=$(ssh -i $SSH_KEY root@${TEST_VALIDATOR} 'journalctl -u flarechain-validator -n 100 --no-pager | tail -50')

if echo "$LOGS" | grep -q "Error parsing"; then
    echo "❌ FAILED - Chainspec parsing error still occurring!"
    echo ""
    echo "Last 30 lines of logs:"
    echo "$LOGS" | tail -30
    exit 1
elif echo "$LOGS" | grep -q "ValidatorCommittee"; then
    echo "✅ SUCCESS - ValidatorCommittee initialization detected!"
    echo ""
    echo "Relevant log lines:"
    echo "$LOGS" | grep -E "ValidatorCommittee|committee|Committee|genesis|Genesis|Imported|block" | tail -20
    exit 0
else
    echo "⚠️  Validator started but no explicit ValidatorCommittee logs found"
    echo ""
    echo "Last 30 lines of logs:"
    echo "$LOGS" | tail -30
    echo ""
    echo "Check if node is producing blocks (this would indicate success)"
fi
