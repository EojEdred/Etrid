#!/bin/bash
# IMMEDIATE DEPLOYMENT - Checkpoint BFT Finality
# Phase 1: 3 validators, Phase 2: 16 validators

set -e

BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"

# Phase 1 - First 3 validators
PHASE1_IPS=(
    "100.95.0.72"    # Validator-6
    "100.86.111.37"  # Validator-7
    "100.125.147.88" # Validator-8
)

# Phase 2 - Next 13 validators (total 16)
PHASE2_IPS=(
    "100.80.84.82"   # Validator-9
    "100.109.252.56" # Validator-10
    "100.117.43.53"  # Validator-11
    "100.88.104.58"  # Validator-12
    "100.70.73.10"   # Validator-13
    "100.68.185.50"  # Validator-14
    "100.71.127.127" # Validator-15
    "100.93.43.18"   # Validator-16
    "100.124.117.73" # Validator-17
    "100.74.204.23"  # Validator-18
    "100.125.251.60" # Validator-19
    "100.114.244.62" # Validator-20
    "100.113.226.111" # Validator-21
)

echo "════════════════════════════════════════════"
echo "ËTRID CHECKPOINT BFT FINALITY DEPLOYMENT"
echo "════════════════════════════════════════════"
echo ""
echo "Binary: $BINARY ($(ls -lh $BINARY | awk '{print $5}'))"
echo ""

# PHASE 1
echo "════════════════════════════════════════════"
echo "PHASE 1: Deploying to 3 validators"
echo "════════════════════════════════════════════"

for ip in "${PHASE1_IPS[@]}"; do
    echo ""
    echo "→ Deploying to $ip..."

    # Copy binary
    scp "$BINARY" root@$ip:/usr/local/bin/flarechain-node-checkpoint || {
        echo "⚠️  Failed to copy to $ip (check SSH access)"
        continue
    }

    # Deploy and restart
    ssh root@$ip << 'ENDSSH'
        # Stop old node
        systemctl stop flarechain-node 2>/dev/null || pkill -9 flarechain-node || true
        sleep 2

        # Backup old binary
        mv /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup 2>/dev/null || true

        # Install new binary
        mv /usr/local/bin/flarechain-node-checkpoint /usr/local/bin/flarechain-node
        chmod +x /usr/local/bin/flarechain-node

        # Restart with checkpoint finality (systemd will use existing config)
        systemctl restart flarechain-node
        sleep 3

        # Check status
        systemctl status flarechain-node --no-pager -l | head -20
ENDSSH

    echo "✅ Deployed to $ip"
done

echo ""
echo "════════════════════════════════════════════"
echo "PHASE 1 COMPLETE - 3 validators running"
echo "════════════════════════════════════════════"
echo ""
echo "Waiting 30 seconds before Phase 2..."
sleep 30

# PHASE 2
echo ""
echo "════════════════════════════════════════════"
echo "PHASE 2: Deploying to 13 more validators"
echo "Total: 16 validators (BFT activation)"
echo "════════════════════════════════════════════"

for ip in "${PHASE2_IPS[@]}"; do
    echo ""
    echo "→ Deploying to $ip..."

    # Copy binary
    scp "$BINARY" root@$ip:/usr/local/bin/flarechain-node-checkpoint || {
        echo "⚠️  Failed to copy to $ip"
        continue
    }

    # Deploy and restart
    ssh root@$ip << 'ENDSSH'
        systemctl stop flarechain-node 2>/dev/null || pkill -9 flarechain-node || true
        sleep 2
        mv /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup 2>/dev/null || true
        mv /usr/local/bin/flarechain-node-checkpoint /usr/local/bin/flarechain-node
        chmod +x /usr/local/bin/flarechain-node
        systemctl restart flarechain-node
        sleep 3
        systemctl status flarechain-node --no-pager -l | head -10
ENDSSH

    echo "✅ Deployed to $ip"
done

echo ""
echo "════════════════════════════════════════════"
echo "DEPLOYMENT COMPLETE!"
echo "════════════════════════════════════════════"
echo ""
echo "Phase 1: 3 validators ✅"
echo "Phase 2: 16 validators ✅"
echo ""
echo "Monitor checkpoints:"
echo "  ssh root@100.95.0.72 'journalctl -u flarechain-node -f | grep Checkpoint'"
echo ""
echo "Check metrics:"
echo "  curl http://100.95.0.72:9615/metrics | grep checkpoint"
echo ""
