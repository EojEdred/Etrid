#!/bin/bash
# Fix validators by adding network keys
# Network keys from COMPLETE_VALIDATOR_NETWORK_MAP.md

set -e

SSH_KEY="$HOME/.ssh/contabo-validators"

# VM IP to Validator Number and Network Key mapping
declare -a VALIDATORS=(
    "85.239.239.194:6:f44ee1c6da7cf209998874f2fa612e75de439afb385625281e123ec8b15ea42f"
    "85.239.239.193:7:2627aa12b4ab2d8d6e82c259b186efb3071e50fac11b28605d8a310dc5688758"
    "85.239.239.190:8:b89f96a7d5dcff24aec4fee55507d2436e036cb6b4fd63016f7605dafdc41f42"
    "85.239.239.189:9:7cefa78d24e90d0e0823afce3cbb57f065f3bdabe8cb94c2f1168582f7a77958"
    "85.239.239.188:10:be9fdd4416eff9375461618f1e2bd244bd0a3ee69b9d2b4949e046796bbe752f"
    "80.190.82.186:11:9e270842ee6d0cc5d4634760717fb2fea85596491c89fc72189a994dbf421d4c"
    "80.190.82.185:12:fe14bf4fd7b9cb683697114b9b60dc5a101adee961aa79e374ddb9d17c42ed4d"
    "80.190.82.184:13:2c339e81f2a1fc80ae67c3bda3ecade01b7b0074979901795ceab6f35a304451"
    "80.190.82.183:14:58716581b09066395ef75cead565526f412c1e9618a9e8401b5862d32b089c42"
    "158.220.83.146:15:d27ae8bc2d7b32cfd6e1a301a4d9931ef2b8c752745a9a86840e376f7bfc9969"
    "158.220.83.66:16:dc2e6eabc3d02e01f26bbf2bf8810c56aa05fd8a9489e80a8d64394e4892265b"
)

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Fix Validators with Network Keys       ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

fix_validator() {
    local validator_info=$1
    IFS=':' read -r ip validator_num node_key <<< "$validator_info"

    echo "[VM $ip] Fixing Validator $validator_num..."

    # Stop the service
    ssh -i "$SSH_KEY" root@$ip "systemctl stop flarechain-validator 2>/dev/null || true"

    # Update systemd service to use --node-key instead of expecting secret_ed25519 file
    ssh -i "$SSH_KEY" root@$ip bash <<REMOTE
cat > /etc/systemd/system/flarechain-validator.service <<'EOF'
[Unit]
Description=Ëtrid FlareChain Validator - Validator ${validator_num}
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=/usr/local/bin/flarechain-node \\
  --chain /root/chainspec.json \\
  --base-path /root/.etrid \\
  --validator \\
  --name "Validator-${validator_num}-Contabo" \\
  --node-key ${node_key} \\
  --public-addr /ip4/${ip}/tcp/30333 \\
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \\
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm \\
  --rpc-cors all \\
  --rpc-external \\
  --port 30333 \\
  --rpc-port 9944 \\
  --prometheus-external \\
  --prometheus-port 9615

Restart=always
RestartSec=10s
KillSignal=SIGTERM
TimeoutStopSec=60s

LimitNOFILE=65536
LimitNPROC=4096

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
REMOTE

    echo "[VM $ip] ✅ Fixed Validator $validator_num"
}

# Fix all validators in parallel
for validator in "${VALIDATORS[@]}"; do
    fix_validator "$validator" &
done

wait

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  All Validators Fixed!                                       ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Starting all validators..."
echo ""

# Start all validators
for validator in "${VALIDATORS[@]}"; do
    IFS=':' read -r ip validator_num node_key <<< "$validator"
    echo "[VM $ip] Starting Validator $validator_num..."
    ssh -i "$SSH_KEY" root@$ip "systemctl start flarechain-validator" &
done

wait

echo ""
echo "Waiting 15 seconds for validators to initialize..."
sleep 15
echo ""

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Validator Status                                            ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Check status
for validator in "${VALIDATORS[@]}"; do
    IFS=':' read -r ip validator_num node_key <<< "$validator"

    echo "Validator $validator_num ($ip):"
    ssh -i "$SSH_KEY" root@$ip bash <<'STATUS'
STATUS=$(systemctl is-active flarechain-validator)
if [ "$STATUS" = "active" ]; then
    echo "  Status: ✅ RUNNING"
    # Show importing block info
    journalctl -u flarechain-validator -n 5 --no-pager 2>/dev/null | grep -E "(Imported|Idle|Syncing|Starting)" | tail -2 | sed 's/^/  /'
else
    echo "  Status: ❌ $STATUS"
    # Show error
    journalctl -u flarechain-validator -n 3 --no-pager 2>/dev/null | tail -2 | sed 's/^/  /'
fi
STATUS
    echo ""
done

echo ""
echo "All validators should now be syncing!"
echo ""
echo "Monitor progress with: ssh root@<IP> journalctl -u flarechain-validator -f"
