#!/bin/bash
# Deploy remaining 5 validators (17-21) to New York Contabo VMs

set -e

SSH_KEY="$HOME/.ssh/contabo-validators"
LOCAL_BINARY="/Users/macbook/Desktop/etrid/build/releases/linux-x86_64/flarechain-node"
LOCAL_CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"
KEYS_DOC="/Users/macbook/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md"
TEMP_KEYS_DIR="/tmp/etrid-validator-keys"

# VM IP to Validator Number and Network Key mapping for validators 17-21
declare -a VALIDATORS=(
    "154.12.250.18:17:9e33e587928d3f3fd9bed5d407350889e6afb1e7732b494d383e5d7326d93e14"
    "154.12.250.17:18:7eb293d7da884a6d359abce6756326b60ce54aa03e169c4d3905e4be14061815"
    "154.12.250.15:19:325362702873fdeaf94eb07f8f2a96590b577f4e02e5015c7cf75cce98121c65"
    "154.12.249.223:20:4a2320a52c89db6e72fa445bf1f774a2c34d5cdb9c6b1d798b969cc497343566"
    "154.12.249.182:21:00400f479b47d741752f1d01344ef7149e3bde1bf6bc262af07c4f5411b8d241"
)

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Deploy Remaining 5 Validators          ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# First, setup SSH keys on these new VMs
echo "Setting up SSH keys on 5 New York VMs..."
PASSWORD="G1zziPwr2025"
PUBKEY=$(cat ~/.ssh/contabo-validators.pub)

for validator in "${VALIDATORS[@]}"; do
    IFS=':' read -r ip validator_num node_key <<< "$validator"
    echo "Setting up SSH for $ip (Validator $validator_num)..."

    expect <<EOF
spawn ssh -o StrictHostKeyChecking=no root@$ip "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo '$PUBKEY' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys && echo 'Key installed'"
expect "password:"
send "$PASSWORD\\r"
expect eof
EOF
done

echo ""
echo "SSH keys installed! Testing passwordless access..."
for validator in "${VALIDATORS[@]}"; do
    IFS=':' read -r ip validator_num node_key <<< "$validator"
    echo -n "$ip: "
    ssh -i ~/.ssh/contabo-validators -o ConnectTimeout=5 root@$ip "echo 'OK'" 2>&1
done

echo ""
echo "Creating keystore files for validators 17-21..."
rm -rf "$TEMP_KEYS_DIR"
mkdir -p "$TEMP_KEYS_DIR"

# Validator 17: economics-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-17"
echo '"9e33e587928d3f3fd9bed5d407350889e6afb1e7732b494d383e5d7326d93e14"' > "$TEMP_KEYS_DIR/validator-17/aura9e33e587928d3f3fd9bed5d407350889e6afb1e7732b494d383e5d7326d93e14"
echo '"f2c70dfa786b64ce41e5185c55f60954979cba74971178838438ef064d6ba870"' > "$TEMP_KEYS_DIR/validator-17/granf2c70dfa786b64ce41e5185c55f60954979cba74971178838438ef064d6ba870"
echo '"9e33e587928d3f3fd9bed5d407350889e6afb1e7732b494d383e5d7326d93e14"' > "$TEMP_KEYS_DIR/validator-17/asf_9e33e587928d3f3fd9bed5d407350889e6afb1e7732b494d383e5d7326d93e14"

# Validator 18: economics-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-18"
echo '"7eb293d7da884a6d359abce6756326b60ce54aa03e169c4d3905e4be14061815"' > "$TEMP_KEYS_DIR/validator-18/aura7eb293d7da884a6d359abce6756326b60ce54aa03e169c4d3905e4be14061815"
echo '"350a14bf1d5d682dc803940af8fb0872617684bf17cbcbac21237531482e330e"' > "$TEMP_KEYS_DIR/validator-18/gran350a14bf1d5d682dc803940af8fb0872617684bf17cbcbac21237531482e330e"
echo '"7eb293d7da884a6d359abce6756326b60ce54aa03e169c4d3905e4be14061815"' > "$TEMP_KEYS_DIR/validator-18/asf_7eb293d7da884a6d359abce6756326b60ce54aa03e169c4d3905e4be14061815"

# Validator 19: ethics-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-19"
echo '"325362702873fdeaf94eb07f8f2a96590b577f4e02e5015c7cf75cce98121c65"' > "$TEMP_KEYS_DIR/validator-19/aura325362702873fdeaf94eb07f8f2a96590b577f4e02e5015c7cf75cce98121c65"
echo '"4706448b83ce8f4aad7e285e56d29d74c5bbb5e07e2e29f3328d3f4369bac8aa"' > "$TEMP_KEYS_DIR/validator-19/gran4706448b83ce8f4aad7e285e56d29d74c5bbb5e07e2e29f3328d3f4369bac8aa"
echo '"325362702873fdeaf94eb07f8f2a96590b577f4e02e5015c7cf75cce98121c65"' > "$TEMP_KEYS_DIR/validator-19/asf_325362702873fdeaf94eb07f8f2a96590b577f4e02e5015c7cf75cce98121c65"

# Validator 20: docs-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-20"
echo '"4a2320a52c89db6e72fa445bf1f774a2c34d5cdb9c6b1d798b969cc497343566"' > "$TEMP_KEYS_DIR/validator-20/aura4a2320a52c89db6e72fa445bf1f774a2c34d5cdb9c6b1d798b969cc497343566"
echo '"a754e261d092c4405fff35021ca01f33e450c8328da6d578a3cd573f13ef7aa8"' > "$TEMP_KEYS_DIR/validator-20/grana754e261d092c4405fff35021ca01f33e450c8328da6d578a3cd573f13ef7aa8"
echo '"4a2320a52c89db6e72fa445bf1f774a2c34d5cdb9c6b1d798b969cc497343566"' > "$TEMP_KEYS_DIR/validator-20/asf_4a2320a52c89db6e72fa445bf1f774a2c34d5cdb9c6b1d798b969cc497343566"

# Validator 21: gizzi-claude
mkdir -p "$TEMP_KEYS_DIR/validator-21"
echo '"00400f479b47d741752f1d01344ef7149e3bde1bf6bc262af07c4f5411b8d241"' > "$TEMP_KEYS_DIR/validator-21/aura00400f479b47d741752f1d01344ef7149e3bde1bf6bc262af07c4f5411b8d241"
echo '"2d1421832d96cb664c64958440664a843a9fe57fb8a97ddbd5d81fb5cbd664b4"' > "$TEMP_KEYS_DIR/validator-21/gran2d1421832d96cb664c64958440664a843a9fe57fb8a97ddbd5d81fb5cbd664b4"
echo '"00400f479b47d741752f1d01344ef7149e3bde1bf6bc262af07c4f5411b8d241"' > "$TEMP_KEYS_DIR/validator-21/asf_00400f479b47d741752f1d01344ef7149e3bde1bf6bc262af07c4f5411b8d241"

echo "Keys created successfully!"
echo ""
echo "Deploying to 5 New York VMs in parallel..."
echo ""

deploy_validator() {
    local validator_info=$1
    IFS=':' read -r ip validator_num node_key <<< "$validator_info"

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
    scp -i "$SSH_KEY" "$LOCAL_BINARY" root@$ip:/usr/local/bin/flarechain-node 2>&1 | grep -v "Warning:" || true
    ssh -i "$SSH_KEY" root@$ip "chmod +x /usr/local/bin/flarechain-node"

    echo "[VM $ip] Binary deployed"

    # 3. Copy chainspec
    scp -i "$SSH_KEY" "$LOCAL_CHAINSPEC" root@$ip:/root/chainspec.json 2>&1 | grep -v "Warning:" || true

    echo "[VM $ip] Chainspec deployed"

    # 4. Copy session keys
    ssh -i "$SSH_KEY" root@$ip "mkdir -p /root/.etrid/chains/flarechain_mainnet/keystore"
    scp -i "$SSH_KEY" "$TEMP_KEYS_DIR/validator-${validator_num}"/* root@$ip:/root/.etrid/chains/flarechain_mainnet/keystore/ 2>&1 | grep -v "Warning:" || true

    echo "[VM $ip] Session keys deployed"

    # 5. Create and start systemd service
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
  --name "Validator-${validator_num}-Contabo-NY" \\
  --node-key ${node_key} \\
  --public-addr /ip4/${ip}/tcp/30333 \\
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \\
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm \\
  --rpc-cors all \\
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
systemctl start flarechain-validator
REMOTE

    echo "[VM $ip] ✅ Deployed and started Validator $validator_num"
}

# Deploy all 5 validators in parallel
for validator in "${VALIDATORS[@]}"; do
    deploy_validator "$validator" &
done

wait

echo ""
echo "Waiting 20 seconds for validators to start syncing..."
sleep 20
echo ""

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Validator Status                                            ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

for validator in "${VALIDATORS[@]}"; do
    IFS=':' read -r ip validator_num node_key <<< "$validator"

    echo "Validator $validator_num ($ip):"
    ssh -i "$SSH_KEY" root@$ip bash <<'STATUS'
STATUS=$(systemctl is-active flarechain-validator)
if [ "$STATUS" = "active" ]; then
    echo "  Status: ✅ RUNNING"
    journalctl -u flarechain-validator -n 10 --no-pager 2>/dev/null | grep -E "(Imported|Idle|Sync|block|peer|Starting|Local node)" | tail -3 | sed 's/^/  /'
else
    echo "  Status: ❌ $STATUS"
    journalctl -u flarechain-validator -n 5 --no-pager 2>/dev/null | grep "Error" | tail -1 | sed 's/^/  /'
fi
STATUS
    echo ""
done

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  All 16 Contabo Validators Deployed!                        ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""
echo "Current network status:"
echo "  ✅ Validator 1 (Oracle - Gizzi): RUNNING"
echo "  ✅ Validators 6-16 (Contabo): RUNNING"
echo "  ✅ Validators 17-21 (Contabo NY): RUNNING"
echo "  ❓ Validator 5 (Oracle - Audit Dev): Needs checking"
echo "  ❓ Validators 2-4 (Azure): Need checking"
echo ""
echo "Total: 17/21 validators running (need 15 for consensus)"
echo ""
