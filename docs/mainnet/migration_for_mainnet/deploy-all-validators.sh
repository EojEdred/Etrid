#!/bin/bash
# Deploy all validators to Contabo VMs
# This script will deploy in parallel for speed

set -e

# VM IP to Validator Number mapping
declare -a VMS=(
    "85.239.239.194:6"
    "85.239.239.193:7"
    "85.239.239.190:8"
    "85.239.239.189:9"
    "85.239.239.188:10"
    "80.190.82.186:11"
    "80.190.82.185:12"
    "80.190.82.184:13"
    "80.190.82.183:14"
    "158.220.83.146:15"
    "158.220.83.66:16"
)

SSH_KEY="$HOME/.ssh/contabo-validators"
LOCAL_BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"
LOCAL_CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
KEYS_DIR="/Users/macbook/Desktop/etrid/secrets/validator-keys"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Deploy All Validators                   ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Deploying to 11 VMs in parallel..."
echo ""

# Function to deploy to a single VM
deploy_vm() {
    local vm_info=$1
    IFS=':' read -r ip validator_num <<< "$vm_info"

    echo "[VM $ip] Starting deployment for Validator $validator_num..."

    # 1. Configure firewall
    ssh -i "$SSH_KEY" root@$ip bash <<'FIREWALL'
apt-get update -qq
apt-get install -y -qq ufw
ufw --force reset
ufw allow 22/tcp
ufw allow 30333/tcp
ufw allow 9944/tcp
ufw allow 9615/tcp
ufw --force enable
FIREWALL

    echo "[VM $ip] Firewall configured"

    # 2. Copy binary
    scp -i "$SSH_KEY" "$LOCAL_BINARY" root@$ip:/usr/local/bin/flarechain-node
    ssh -i "$SSH_KEY" root@$ip "chmod +x /usr/local/bin/flarechain-node"

    echo "[VM $ip] Binary deployed"

    # 3. Copy chainspec
    scp -i "$SSH_KEY" "$LOCAL_CHAINSPEC" root@$ip:/root/chainspec.json

    echo "[VM $ip] Chainspec deployed"

    # 4. Copy session keys
    ssh -i "$SSH_KEY" root@$ip "mkdir -p /root/.etrid/keys"
    scp -r -i "$SSH_KEY" "$KEYS_DIR/validator-${validator_num}"/* root@$ip:/root/.etrid/keys/ 2>/dev/null || true

    echo "[VM $ip] Session keys deployed"

    # 5. Create systemd service
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
  --base-path /root/.etrid/validator \\
  --validator \\
  --name "Validator-${validator_num}-Contabo" \\
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
systemctl enable flarechain-validator
REMOTE

    echo "[VM $ip] ✅ Deployment complete for Validator $validator_num"
}

# Deploy to all VMs in parallel
for vm in "${VMS[@]}"; do
    deploy_vm "$vm" &
done

# Wait for all deployments to complete
wait

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  ALL DEPLOYMENTS COMPLETE!                                   ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Verifying deployments..."
echo ""

# Verify each VM
for vm in "${VMS[@]}"; do
    IFS=':' read -r ip validator_num <<< "$vm"
    echo "Validator $validator_num ($ip):"
    ssh -i "$SSH_KEY" root@$ip bash <<'VERIFY'
/usr/local/bin/flarechain-node --version | head -1
ls -lh /root/chainspec.json | awk '{print "  Chainspec: " $5}'
ls /root/.etrid/keys/ | wc -l | awk '{print "  Keys: " $1 " files"}'
systemctl is-enabled flarechain-validator | awk '{print "  Service: " $1}'
VERIFY
    echo ""
done

echo "All validators are deployed and ready to start!"
echo ""
echo "Next step: Run start-all-validators.sh to start all validators"
