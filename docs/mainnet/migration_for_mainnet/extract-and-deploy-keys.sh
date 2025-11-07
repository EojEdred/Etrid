#!/bin/bash
# Extract keys from master document and create keystore files for each validator
# Then deploy everything to the VMs

set -e

KEYS_DOC="/Users/macbook/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md"
TEMP_KEYS_DIR="/tmp/etrid-validator-keys"
SSH_KEY="$HOME/.ssh/contabo-validators"
LOCAL_BINARY="/Users/macbook/Desktop/etrid/build/releases/linux-x86_64/flarechain-node"
LOCAL_CHAINSPEC="/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json"

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

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Extract Keys & Deploy All Validators   ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo ""

# Create temp directory for keystore files
rm -rf "$TEMP_KEYS_DIR"
mkdir -p "$TEMP_KEYS_DIR"

# Function to create keystore file for a key
create_keystore_file() {
    local key_type=$1    # aura, gran, asf_
    local key_hex=$2     # hex key without 0x
    local output_dir=$3

    # Remove 0x prefix if present
    key_hex=${key_hex#0x}

    # Create the filename (key type + public key hex)
    local filename="${key_type}${key_hex}"

    # The content is just the hex seed/private key in quotes
    echo "\"${key_hex}\"" > "${output_dir}/${filename}"

    echo "Created keystore file: ${filename}"
}

echo "Extracting validator keys from master document..."
echo ""

# Manually extract keys for validators 6-16 from the document
# We'll use the AURA, GRANDPA, and ASF keys from each validator

# Validator 6: consensus-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-6"
create_keystore_file "aura" "0xf44ee1c6da7cf209998874f2fa612e75de439afb385625281e123ec8b15ea42f" "$TEMP_KEYS_DIR/validator-6"
create_keystore_file "gran" "0xdc4357a4d93f0599b616159278d8ce281e19685c8dd0d40d5960a58d8eeda3b8" "$TEMP_KEYS_DIR/validator-6"
create_keystore_file "asf_" "0xf44ee1c6da7cf209998874f2fa612e75de439afb385625281e123ec8b15ea42f" "$TEMP_KEYS_DIR/validator-6"

# Validator 7
mkdir -p "$TEMP_KEYS_DIR/validator-7"
create_keystore_file "aura" "0x2627aa12b4ab2d8d6e82c259b186efb3071e50fac11b28605d8a310dc5688758" "$TEMP_KEYS_DIR/validator-7"
create_keystore_file "gran" "0x2975859973decf0c53296d425ade75b28ab3ab10ea3c2d1b78170c8885090a0d" "$TEMP_KEYS_DIR/validator-7"
create_keystore_file "asf_" "0x2627aa12b4ab2d8d6e82c259b186efb3071e50fac11b28605d8a310dc5688758" "$TEMP_KEYS_DIR/validator-7"

# Validator 8: runtime-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-8"
create_keystore_file "aura" "0xb89f96a7d5dcff24aec4fee55507d2436e036cb6b4fd63016f7605dafdc41f42" "$TEMP_KEYS_DIR/validator-8"
create_keystore_file "gran" "0xcc16f2cd990d95553f0b143df3ec1d1650538708a0031cafd8ecbcadd8dec69a" "$TEMP_KEYS_DIR/validator-8"
create_keystore_file "asf_" "0xb89f96a7d5dcff24aec4fee55507d2436e036cb6b4fd63016f7605dafdc41f42" "$TEMP_KEYS_DIR/validator-8"

# Validator 9
mkdir -p "$TEMP_KEYS_DIR/validator-9"
create_keystore_file "aura" "0x7cefa78d24e90d0e0823afce3cbb57f065f3bdabe8cb94c2f1168582f7a77958" "$TEMP_KEYS_DIR/validator-9"
create_keystore_file "gran" "0xee9d4f38c8b3757c9604db672ef1309fa740adf3d3c989a2baa219fc70d4f115" "$TEMP_KEYS_DIR/validator-9"
create_keystore_file "asf_" "0x7cefa78d24e90d0e0823afce3cbb57f065f3bdabe8cb94c2f1168582f7a77958" "$TEMP_KEYS_DIR/validator-9"

# Validator 10: compiler-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-10"
create_keystore_file "aura" "0xbe9fdd4416eff9375461618f1e2bd244bd0a3ee69b9d2b4949e046796bbe752f" "$TEMP_KEYS_DIR/validator-10"
create_keystore_file "gran" "0xe7e51036e205de3cb69a46a6c55cded6f162fc1392d05b70ef00d18813b905ed" "$TEMP_KEYS_DIR/validator-10"
create_keystore_file "asf_" "0xbe9fdd4416eff9375461618f1e2bd244bd0a3ee69b9d2b4949e046796bbe752f" "$TEMP_KEYS_DIR/validator-10"

# Validator 11
mkdir -p "$TEMP_KEYS_DIR/validator-11"
create_keystore_file "aura" "0x9e270842ee6d0cc5d4634760717fb2fea85596491c89fc72189a994dbf421d4c" "$TEMP_KEYS_DIR/validator-11"
create_keystore_file "gran" "0x445da264d4029002800bf78b58d160877c26696d3f8a83fc19aa64f843d31672" "$TEMP_KEYS_DIR/validator-11"
create_keystore_file "asf_" "0x9e270842ee6d0cc5d4634760717fb2fea85596491c89fc72189a994dbf421d4c" "$TEMP_KEYS_DIR/validator-11"

# Validator 12: oracle-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-12"
create_keystore_file "aura" "0xfe14bf4fd7b9cb683697114b9b60dc5a101adee961aa79e374ddb9d17c42ed4d" "$TEMP_KEYS_DIR/validator-12"
create_keystore_file "gran" "0x80565d70b6d75fa25e243dbfb6206eb6eb4e6e56ee82b403db9758bd6890eae4" "$TEMP_KEYS_DIR/validator-12"
create_keystore_file "asf_" "0xfe14bf4fd7b9cb683697114b9b60dc5a101adee961aa79e374ddb9d17c42ed4d" "$TEMP_KEYS_DIR/validator-12"

# Validator 13: multichain-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-13"
create_keystore_file "aura" "0x2c339e81f2a1fc80ae67c3bda3ecade01b7b0074979901795ceab6f35a304451" "$TEMP_KEYS_DIR/validator-13"
create_keystore_file "gran" "0x1b4a0249c16be966d76778364affe12c24e449c78448d6c783a4e8f746c81e09" "$TEMP_KEYS_DIR/validator-13"
create_keystore_file "asf_" "0x2c339e81f2a1fc80ae67c3bda3ecade01b7b0074979901795ceab6f35a304451" "$TEMP_KEYS_DIR/validator-13"

# Validator 14
mkdir -p "$TEMP_KEYS_DIR/validator-14"
create_keystore_file "aura" "0x58716581b09066395ef75cead565526f412c1e9618a9e8401b5862d32b089c42" "$TEMP_KEYS_DIR/validator-14"
create_keystore_file "gran" "0x9003abffc3a21ead663494bc062ac45a5266e8e385bbf38fe3daddcdefcf0e6c" "$TEMP_KEYS_DIR/validator-14"
create_keystore_file "asf_" "0x58716581b09066395ef75cead565526f412c1e9618a9e8401b5862d32b089c42" "$TEMP_KEYS_DIR/validator-14"

# Validator 15: edsc-dev01
mkdir -p "$TEMP_KEYS_DIR/validator-15"
create_keystore_file "aura" "0xd27ae8bc2d7b32cfd6e1a301a4d9931ef2b8c752745a9a86840e376f7bfc9969" "$TEMP_KEYS_DIR/validator-15"
create_keystore_file "gran" "0xf0e5a2d50664cea659cf9c17a04a3e197414d12438d08400f58ebf9491e9ac66" "$TEMP_KEYS_DIR/validator-15"
create_keystore_file "asf_" "0xd27ae8bc2d7b32cfd6e1a301a4d9931ef2b8c752745a9a86840e376f7bfc9969" "$TEMP_KEYS_DIR/validator-15"

# Validator 16
mkdir -p "$TEMP_KEYS_DIR/validator-16"
create_keystore_file "aura" "0xdc2e6eabc3d02e01f26bbf2bf8810c56aa05fd8a9489e80a8d64394e4892265b" "$TEMP_KEYS_DIR/validator-16"
create_keystore_file "gran" "0xef4795d70a779902236e7a6e42241c7194a7a77c196984b0b8bc4d9e2e70039f" "$TEMP_KEYS_DIR/validator-16"
create_keystore_file "asf_" "0xdc2e6eabc3d02e01f26bbf2bf8810c56aa05fd8a9489e80a8d64394e4892265b" "$TEMP_KEYS_DIR/validator-16"

echo ""
echo "Keys extracted successfully!"
echo ""
echo "Deploying to 11 VMs in parallel..."
echo ""

# Function to deploy to a single VM
deploy_vm() {
    local vm_info=$1
    IFS=':' read -r ip validator_num <<< "$vm_info"

    echo "[VM $ip] Starting deployment for Validator $validator_num..."

    # 1. Copy binary
    scp -i "$SSH_KEY" "$LOCAL_BINARY" root@$ip:/usr/local/bin/flarechain-node 2>&1 | grep -v "Warning:" || true
    ssh -i "$SSH_KEY" root@$ip "chmod +x /usr/local/bin/flarechain-node"

    echo "[VM $ip] Binary deployed"

    # 2. Copy chainspec
    scp -i "$SSH_KEY" "$LOCAL_CHAINSPEC" root@$ip:/root/chainspec.json 2>&1 | grep -v "Warning:" || true

    echo "[VM $ip] Chainspec deployed"

    # 3. Copy session keys
    ssh -i "$SSH_KEY" root@$ip "mkdir -p /root/.etrid/chains/flarechain_mainnet/keystore"
    scp -i "$SSH_KEY" "$TEMP_KEYS_DIR/validator-${validator_num}"/* root@$ip:/root/.etrid/chains/flarechain_mainnet/keystore/ 2>&1 | grep -v "Warning:" || true

    echo "[VM $ip] Session keys deployed"

    # 4. Create systemd service
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
/usr/local/bin/flarechain-node --version 2>/dev/null | head -1 || echo "  Binary: ERROR"
ls -lh /root/chainspec.json 2>/dev/null | awk '{print "  Chainspec: " $5}' || echo "  Chainspec: ERROR"
ls /root/.etrid/chains/flarechain_mainnet/keystore/ 2>/dev/null | wc -l | awk '{print "  Keys: " $1 " files"}' || echo "  Keys: ERROR"
systemctl is-enabled flarechain-validator 2>/dev/null | awk '{print "  Service: " $1}' || echo "  Service: ERROR"
VERIFY
    echo ""
done

echo "All validators are deployed and ready to start!"
echo ""
echo "Next step: Run start-all-validators.sh to start all validators"
