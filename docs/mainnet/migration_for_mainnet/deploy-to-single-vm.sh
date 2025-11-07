#!/bin/bash
# Ëtrid FlareChain - Deploy Validator to Single Contabo VM
# Usage: ./deploy-to-single-vm.sh VM_IP VALIDATOR_NUMBER
# Example: ./deploy-to-single-vm.sh 123.45.67.89 6

set -e

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 VM_IP VALIDATOR_NUMBER"
    echo "Example: $0 123.45.67.89 6"
    exit 1
fi

VM_IP=$1
VALIDATOR_NUM=$2
SSH_KEY="$HOME/.ssh/contabo-validators"
LOCAL_BINARY="/Users/macbook/Desktop/etrid/target/release/flarechain-node"
LOCAL_CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json"
LOCAL_KEYS="/Users/macbook/Desktop/etrid/secrets/validator-keys/validator-${VALIDATOR_NUM}"

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Single VM Deployment                    ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Target VM: $VM_IP"
echo "Validator: #$VALIDATOR_NUM"
echo ""

# Verify local files exist
if [ ! -f "$LOCAL_BINARY" ]; then
    echo "ERROR: Binary not found at $LOCAL_BINARY"
    exit 1
fi

if [ ! -f "$LOCAL_CHAINSPEC" ]; then
    echo "ERROR: Chainspec not found at $LOCAL_CHAINSPEC"
    exit 1
fi

if [ ! -d "$LOCAL_KEYS" ]; then
    echo "ERROR: Session keys not found at $LOCAL_KEYS"
    exit 1
fi

echo "✓ Local files verified"
echo ""

# Test SSH connection
echo "Testing SSH connection..."
if ! ssh -i "$SSH_KEY" -o ConnectTimeout=5 root@$VM_IP "echo 'Connected'" >/dev/null 2>&1; then
    echo "ERROR: Cannot connect to $VM_IP via SSH"
    echo "Check:"
    echo "  1. VM is running"
    echo "  2. SSH key is correct: $SSH_KEY"
    echo "  3. Firewall allows port 22"
    exit 1
fi
echo "✓ SSH connection successful"
echo ""

# Step 1: Copy binary
echo "[1/5] Copying node binary..."
scp -i "$SSH_KEY" "$LOCAL_BINARY" root@$VM_IP:/usr/local/bin/flarechain-node
ssh -i "$SSH_KEY" root@$VM_IP "chmod +x /usr/local/bin/flarechain-node"
echo "✓ Binary deployed"
echo ""

# Step 2: Copy chainspec
echo "[2/5] Copying chainspec..."
scp -i "$SSH_KEY" "$LOCAL_CHAINSPEC" root@$VM_IP:/root/chainspec.json
echo "✓ Chainspec deployed"
echo ""

# Step 3: Copy session keys
echo "[3/5] Copying session keys..."
ssh -i "$SSH_KEY" root@$VM_IP "mkdir -p /root/.etrid/keys"
scp -r -i "$SSH_KEY" "$LOCAL_KEYS"/* root@$VM_IP:/root/.etrid/keys/
echo "✓ Session keys deployed"
echo ""

# Step 4: Create systemd service
echo "[4/5] Creating systemd service..."
ssh -i "$SSH_KEY" root@$VM_IP bash <<'REMOTE'
cat > /etc/systemd/system/flarechain-validator.service <<'EOF'
[Unit]
Description=Ëtrid FlareChain Validator - Validator VALIDATOR_NUM
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=root
WorkingDirectory=/root
ExecStart=/usr/local/bin/flarechain-node \
  --chain /root/chainspec.json \
  --base-path /root/.etrid/validator \
  --validator \
  --name "Validator-VALIDATOR_NUM-Contabo" \
  --public-addr /ip4/VM_IP_PLACEHOLDER/tcp/30333 \
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm \
  --rpc-cors all \
  --rpc-external \
  --port 30333 \
  --rpc-port 9944 \
  --prometheus-external \
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

# Replace placeholders in service file
ssh -i "$SSH_KEY" root@$VM_IP "sed -i 's/VALIDATOR_NUM/$VALIDATOR_NUM/g' /etc/systemd/system/flarechain-validator.service"
ssh -i "$SSH_KEY" root@$VM_IP "sed -i 's/VM_IP_PLACEHOLDER/$VM_IP/g' /etc/systemd/system/flarechain-validator.service"

echo "✓ Systemd service created"
echo ""

# Step 5: Verify deployment
echo "[5/5] Verifying deployment..."
ssh -i "$SSH_KEY" root@$VM_IP bash <<'VERIFY'
echo "Binary version:"
/usr/local/bin/flarechain-node --version

echo ""
echo "Chainspec:"
ls -lh /root/chainspec.json

echo ""
echo "Session keys:"
ls -la /root/.etrid/keys/

echo ""
echo "Service status:"
systemctl status flarechain-validator --no-pager | head -5
VERIFY

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  DEPLOYMENT COMPLETE!                                        ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Validator $VALIDATOR_NUM deployed to $VM_IP"
echo ""
echo "Next steps:"
echo "  1. Start validator: ssh -i $SSH_KEY root@$VM_IP 'systemctl start flarechain-validator'"
echo "  2. Monitor logs: ssh -i $SSH_KEY root@$VM_IP 'journalctl -u flarechain-validator -f'"
echo ""
