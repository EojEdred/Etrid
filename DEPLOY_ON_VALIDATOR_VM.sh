#!/bin/bash
# Run on Validator VM after building
# This deploys the newly built binary and starts the validator

set -e

CHAIN_ID="flarechain_prod_21val_v109"

echo "=========================================="
echo "Deploy Binary on Validator"
echo "=========================================="
echo ""

# Step 1: Copy binary
echo "→ Copying binary to /usr/local/bin..."
cp /root/etrid/target/release/etrid /usr/local/bin/flarechain-node
chmod +x /usr/local/bin/flarechain-node
echo "✓ Binary deployed"
echo ""

# Step 2: Copy chainspec
echo "→ Copying chainspec..."
cp /root/etrid/chainspec_v109_FIXED.json /root/
echo "✓ Chainspec copied"
echo ""

# Step 3: Update systemd service
echo "→ Updating systemd service..."
sed -i 's|--chain /root/chainspec_v109.json|--chain /root/chainspec_v109_FIXED.json|g' /etc/systemd/system/flarechain-validator.service
sed -i 's|--chain chainspec_v109.json|--chain /root/chainspec_v109_FIXED.json|g' /etc/systemd/system/flarechain-validator.service
systemctl daemon-reload
echo "✓ Service updated"
echo ""

# Step 4: Purge old database (preserve network keys)
echo "→ Purging chain database..."
if [ -d /root/flarechain-data/chains/${CHAIN_ID} ]; then
    # Preserve network keys
    mkdir -p /tmp/network-backup
    cp -r /root/flarechain-data/chains/${CHAIN_ID}/network /tmp/network-backup/ 2>/dev/null || true

    # Remove chain data
    rm -rf /root/flarechain-data/chains/${CHAIN_ID}

    # Restore network keys
    mkdir -p /root/flarechain-data/chains/${CHAIN_ID}/network
    cp -r /tmp/network-backup/network/* /root/flarechain-data/chains/${CHAIN_ID}/network/ 2>/dev/null || true
    rm -rf /tmp/network-backup

    echo "✓ Database purged (network keys preserved)"
else
    echo "✓ No existing database found"
fi
echo ""

# Step 5: Start validator
echo "→ Starting validator..."
systemctl start flarechain-validator
echo "✓ Validator started"
echo ""

# Step 6: Check status
echo "⏳ Waiting 30 seconds for initialization..."
sleep 30
echo ""

echo "→ Checking validator status..."
systemctl status flarechain-validator --no-pager | head -20
echo ""

echo "→ Checking recent logs..."
journalctl -u flarechain-validator -n 50 --no-pager | tail -30
echo ""

echo "=========================================="
echo "Deployment Complete!"
echo "=========================================="
echo ""
echo "Monitor logs with:"
echo "  journalctl -u flarechain-validator -f"
echo ""
echo "Check for errors:"
echo "  journalctl -u flarechain-validator -n 100 | grep -i error"
echo ""
echo "Check genesis initialization:"
echo "  journalctl -u flarechain-validator -n 100 | grep -iE '(genesis|initialized|imported #0)'"
