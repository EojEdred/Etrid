#!/bin/bash
# Deploy checkpoint finality fix to all 22 validators
set -e

# Build on validator-6 (already done)
SOURCE_VALIDATOR="100.95.0.72"
BINARY_PATH="/root/etrid/target/release/flarechain-node"

# All validator IPs
ALL_VALIDATORS=(
    "100.95.0.72"    # Validator-6 (already deployed)
    "100.86.111.37"  # Validator-7
    "100.125.147.88" # Validator-8
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

# Oracle Directors (need source build due to glibc)
DIRECTORS=(
    "100.122.19.7"    # Gizzi
    "100.126.54.89"   # AuditDev
    "100.102.128.51"  # Director-3
    "100.71.242.104"  # Director-4
    "100.74.84.28"    # Director-5
    "100.89.102.75"   # Director-6
)

echo "════════════════════════════════════════════"
echo "ËTRID CHECKPOINT FINALITY FIX - ALL VALIDATORS"
echo "════════════════════════════════════════════"
echo ""

# Deploy to remaining Contabo validators (copy binary from validator-6)
echo "Phase 1: Deploying to 15 Contabo validators..."
for ip in "${ALL_VALIDATORS[@]:1}"; do  # Skip first (validator-6)
    echo ""
    echo "→ Deploying to $ip..."

    # Copy binary from validator-6
    ssh -i ~/.ssh/contabo-validators root@$SOURCE_VALIDATOR "scp -i ~/.ssh/id_rsa $BINARY_PATH root@$ip:/tmp/flarechain-node-fix" 2>/dev/null || {
        echo "⚠️  Direct copy failed, trying via local machine..."
        scp -i ~/.ssh/contabo-validators root@$SOURCE_VALIDATOR:$BINARY_PATH /tmp/flarechain-node-fix
        scp -i ~/.ssh/contabo-validators /tmp/flarechain-node-fix root@$ip:/tmp/flarechain-node-fix
    }

    # Deploy and restart
    ssh -i ~/.ssh/contabo-validators root@$ip << 'ENDSSH'
        systemctl stop flarechain-validator
        cp /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup-pre-fix || true
        cp /tmp/flarechain-node-fix /usr/local/bin/flarechain-node
        chmod +x /usr/local/bin/flarechain-node
        systemctl start flarechain-validator
        sleep 2
        systemctl status flarechain-validator --no-pager -l | head -10
ENDSSH

    echo "✅ Deployed to $ip"
done

echo ""
echo "Phase 2: Building and deploying to Oracle Directors..."
for ip in "${DIRECTORS[@]}"; do
    echo ""
    echo "→ Building on $ip..."

    # Determine SSH key and user
    if [[ "$ip" == "100.122.19.7" ]] || [[ "$ip" == "100.126.54.89" ]]; then
        SSH_KEY="~/.ssh/gizzi-validator"
        SSH_USER="ubuntu"
    else
        SSH_KEY="~/.ssh/contabo-validators"
        SSH_USER="root"
    fi

    # Build from source
    ssh -i $SSH_KEY $SSH_USER@$ip << 'ENDSSH'
        cd /root/etrid || cd ~/etrid
        git fetch && git pull
        source ~/.cargo/env
        cargo build --release -p flarechain-node 2>&1 | tail -5

        sudo systemctl stop flarechain-validator 2>/dev/null || pkill -9 flarechain-node
        sudo cp /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup-pre-fix 2>/dev/null || true
        sudo cp target/release/flarechain-node /usr/local/bin/flarechain-node
        sudo chmod +x /usr/local/bin/flarechain-node
        sudo systemctl start flarechain-validator
        sleep 2
        systemctl status flarechain-validator --no-pager -l | head -10
ENDSSH

    echo "✅ Built and deployed on $ip"
done

echo ""
echo "════════════════════════════════════════════"
echo "DEPLOYMENT COMPLETE - 22/22 VALIDATORS"
echo "════════════════════════════════════════════"
echo ""
echo "Check checkpoint finality:"
echo "  ssh root@100.95.0.72 'journalctl -u flarechain-validator -f | grep Checkpoint'"
echo ""
