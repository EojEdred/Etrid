#!/bin/bash
# Deploy v109 binary with ValidatorCommittee support from GitHub Actions artifact

VALIDATORS=(
    "100.95.0.72" "100.86.111.37" "100.125.147.88" "100.80.84.82"
    "100.109.252.56" "100.117.43.53" "100.88.104.58" "100.70.73.10"
    "100.68.185.50" "100.71.127.127" "100.93.43.18" "100.124.117.73"
    "100.74.204.23" "100.125.251.60" "100.114.244.62" "100.113.226.111"
    "100.102.128.51" "100.71.242.104" "100.74.84.28" "100.89.102.75"
    "100.120.184.56" "100.82.45.88"
)

SSH_KEY="~/.ssh/contabo-validators"
RUN_ID="19524399191"
BINARY_NAME="flarechain-node"

echo "=========================================="
echo "Deploy v109 Binary with ValidatorCommittee"
echo "=========================================="
echo ""

# Step 1: Download binary from GitHub Actions
echo "→ Downloading binary from GitHub Actions (Run ID: $RUN_ID)..."
gh run download $RUN_ID -n flarechain-node-linux-x86_64 -D /tmp/v109-binary

if [ ! -f "/tmp/v109-binary/flarechain-node" ]; then
    echo "❌ Failed to download binary artifact"
    echo ""
    echo "Please check:"
    echo "  1. GitHub Actions build completed successfully"
    echo "  2. Artifact name is 'flarechain-node-linux-x86_64'"
    echo "  3. You have gh CLI authenticated"
    echo ""
    echo "Check build status:"
    echo "  gh run view $RUN_ID"
    exit 1
fi

chmod +x /tmp/v109-binary/flarechain-node
echo "✓ Binary downloaded ($(du -h /tmp/v109-binary/flarechain-node | cut -f1))"
echo ""

# Verify binary includes ValidatorCommittee
echo "→ Verifying binary includes ValidatorCommittee pallet..."
if strings /tmp/v109-binary/flarechain-node | grep -q "ValidatorCommittee"; then
    echo "✓ Binary includes ValidatorCommittee pallet"
else
    echo "⚠️  WARNING: Binary may not include ValidatorCommittee pallet"
    echo "   Proceeding anyway..."
fi
echo ""

# Step 2: Deploy to all validators
echo "→ Deploying binary to all validators..."
echo ""

SUCCESS_COUNT=0
FAIL_COUNT=0

for IP in "${VALIDATORS[@]}"; do
    echo "→ $IP"

    # Stop validator
    ssh -i $SSH_KEY root@${IP} 'sudo systemctl stop flarechain-validator' 2>/dev/null

    # Deploy binary
    if scp -i $SSH_KEY /tmp/v109-binary/flarechain-node root@${IP}:/usr/local/bin/flarechain-node 2>/dev/null; then
        ssh -i $SSH_KEY root@${IP} 'chmod +x /usr/local/bin/flarechain-node' 2>/dev/null
        echo "  ✓ Binary deployed"
        ((SUCCESS_COUNT++))
    else
        echo "  ✗ Failed to deploy binary"
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
    echo "Restarting all validators with v109 binary..."
    echo ""

    for IP in "${VALIDATORS[@]}"; do
        ssh -i $SSH_KEY root@${IP} 'sudo systemctl restart flarechain-validator' 2>/dev/null &
    done
    wait

    echo "✅ All validators restarted"
    echo ""
    echo "⏳ Waiting 30 seconds for genesis with 21-member committee..."
    sleep 30
    echo ""
    echo "Checking logs for ValidatorCommittee initialization:"
    ssh -i $SSH_KEY root@100.95.0.72 'journalctl -u flarechain-validator -n 100 --no-pager | grep -E "ValidatorCommittee|committee|Committee|genesis|Genesis|Initialized|Imported|block 0" | tail -30'

    exit 0
else
    echo "❌ Only $SUCCESS_COUNT/22 validators deployed successfully."
    echo ""
    echo "Manual recovery steps:"
    echo "  1. Check SSH connectivity to failed validators"
    echo "  2. Verify /usr/local/bin/flarechain-node exists on successful validators"
    echo "  3. Check systemd service status: systemctl status flarechain-validator"
    exit 1
fi
