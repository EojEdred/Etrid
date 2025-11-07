#!/bin/bash
# Testnet Deployment Script for FlareChain and ETH-PBC
# Automates parachain deployment to Rococo/Westend

set -e

# Configuration
RELAY_CHAIN="${RELAY_CHAIN:-rococo}"
FLARECHAIN_PARA_ID="${FLARECHAIN_PARA_ID:-2000}"
ETH_PBC_PARA_ID="${ETH_PBC_PARA_ID:-2001}"
OUTPUT_DIR="./testnet-deployment"

echo "🚀 Ëtrid Testnet Deployment Script"
echo "=================================="
echo "Relay Chain: $RELAY_CHAIN"
echo "FlareChain Para ID: $FLARECHAIN_PARA_ID"
echo "ETH-PBC Para ID: $ETH_PBC_PARA_ID"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"

# ═══════════════════════════════════════════════════════════════════════════════
# Step 1: Build Binaries
# ═══════════════════════════════════════════════════════════════════════════════

echo "📦 Step 1: Building node binaries..."
echo ""

echo "Building FlareChain..."
cargo build --release -p flarechain-node
if [ $? -ne 0 ]; then
    echo "❌ FlareChain build failed"
    exit 1
fi
echo "✅ FlareChain node built"

echo ""
echo "Building ETH-PBC..."
cd 05-multichain/partition-burst-chains/pbc-chains/eth-pbc
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ ETH-PBC build failed"
    exit 1
fi
cd ../../../..
echo "✅ ETH-PBC node built"

# ═══════════════════════════════════════════════════════════════════════════════
# Step 2: Generate Chain Specs
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo "📝 Step 2: Generating chain specifications..."
echo ""

echo "Generating FlareChain testnet spec..."
./target/release/flarechain-node build-spec \
    --chain flarechain-testnet \
    --raw \
    > "$OUTPUT_DIR/flarechain-testnet-raw.json"
echo "✅ FlareChain spec: $OUTPUT_DIR/flarechain-testnet-raw.json"

echo ""
echo "Generating ETH-PBC testnet spec..."
./05-multichain/partition-burst-chains/pbc-chains/eth-pbc/target/release/eth-pbc-node build-spec \
    --chain eth-pbc-testnet \
    --raw \
    > "$OUTPUT_DIR/eth-pbc-testnet-raw.json"
echo "✅ ETH-PBC spec: $OUTPUT_DIR/eth-pbc-testnet-raw.json"

# ═══════════════════════════════════════════════════════════════════════════════
# Step 3: Export Genesis Data
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo "🔬 Step 3: Exporting genesis state and wasm..."
echo ""

echo "Exporting FlareChain genesis..."
./target/release/flarechain-node export-genesis-state \
    --chain "$OUTPUT_DIR/flarechain-testnet-raw.json" \
    > "$OUTPUT_DIR/flarechain-genesis-state"

./target/release/flarechain-node export-genesis-wasm \
    --chain "$OUTPUT_DIR/flarechain-testnet-raw.json" \
    > "$OUTPUT_DIR/flarechain-genesis-wasm"
echo "✅ FlareChain genesis exported"

echo ""
echo "Exporting ETH-PBC genesis..."
./05-multichain/partition-burst-chains/pbc-chains/eth-pbc/target/release/eth-pbc-node export-genesis-state \
    --chain "$OUTPUT_DIR/eth-pbc-testnet-raw.json" \
    > "$OUTPUT_DIR/eth-pbc-genesis-state"

./05-multichain/partition-burst-chains/pbc-chains/eth-pbc/target/release/eth-pbc-node export-genesis-wasm \
    --chain "$OUTPUT_DIR/eth-pbc-testnet-raw.json" \
    > "$OUTPUT_DIR/eth-pbc-genesis-wasm"
echo "✅ ETH-PBC genesis exported"

# ═══════════════════════════════════════════════════════════════════════════════
# Step 4: Create Deployment Package
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo "📦 Step 4: Creating deployment package..."
echo ""

# Copy binaries
echo "Copying binaries..."
cp target/release/flarechain-node "$OUTPUT_DIR/"
cp 05-multichain/partition-burst-chains/pbc-chains/eth-pbc/target/release/eth-pbc-node "$OUTPUT_DIR/"

# Create systemd service files
echo "Creating systemd service files..."

cat > "$OUTPUT_DIR/flarechain-collator.service" << EOF
[Unit]
Description=FlareChain Collator
After=network.target

[Service]
Type=simple
User=flarechain
WorkingDirectory=/opt/flarechain
ExecStart=/opt/flarechain/flarechain-node \\
    --collator \\
    --name "FlareChain-Collator-01" \\
    --chain /opt/flarechain/flarechain-testnet-raw.json \\
    --base-path /data/flarechain \\
    --port 30333 \\
    --rpc-port 9933 \\
    --ws-port 9944 \\
    --unsafe-rpc-external \\
    --unsafe-ws-external \\
    --rpc-cors all \\
    --rpc-methods=Unsafe \\
    -- \\
    --execution wasm \\
    --chain $RELAY_CHAIN \\
    --port 30343 \\
    --rpc-port 9934
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

cat > "$OUTPUT_DIR/eth-pbc-collator.service" << EOF
[Unit]
Description=ETH-PBC Collator
After=network.target

[Service]
Type=simple
User=ethpbc
WorkingDirectory=/opt/eth-pbc
ExecStart=/opt/eth-pbc/eth-pbc-node \\
    --collator \\
    --name "ETH-PBC-Collator-01" \\
    --chain /opt/eth-pbc/eth-pbc-testnet-raw.json \\
    --base-path /data/eth-pbc \\
    --port 30334 \\
    --rpc-port 9937 \\
    --ws-port 9948 \\
    --unsafe-rpc-external \\
    --unsafe-ws-external \\
    --rpc-cors all \\
    --rpc-methods=Unsafe \\
    -- \\
    --execution wasm \\
    --chain $RELAY_CHAIN \\
    --port 30344 \\
    --rpc-port 9938
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

echo "✅ Service files created"

# Create deployment instructions
cat > "$OUTPUT_DIR/DEPLOYMENT_INSTRUCTIONS.md" << EOF
# Testnet Deployment Instructions

## Files Generated

- \`flarechain-node\` - FlareChain collator binary
- \`eth-pbc-node\` - ETH-PBC collator binary
- \`flarechain-testnet-raw.json\` - FlareChain chain specification
- \`eth-pbc-testnet-raw.json\` - ETH-PBC chain specification
- \`flarechain-genesis-state\` - FlareChain genesis state (for parachain registration)
- \`flarechain-genesis-wasm\` - FlareChain genesis wasm (for parachain registration)
- \`eth-pbc-genesis-state\` - ETH-PBC genesis state (for parachain registration)
- \`eth-pbc-genesis-wasm\` - ETH-PBC genesis wasm (for parachain registration)
- \`flarechain-collator.service\` - FlareChain systemd service
- \`eth-pbc-collator.service\` - ETH-PBC systemd service

## Deployment Steps

### 1. Reserve Para IDs

Visit https://polkadot.js.org/apps/?rpc=wss://$RELAY_CHAIN-rpc.polkadot.io

Developer > Extrinsics > registrar > reserve()

Reserve IDs:
- FlareChain: $FLARECHAIN_PARA_ID
- ETH-PBC: $ETH_PBC_PARA_ID

### 2. Register Parachains

Developer > Extrinsics > registrar > forceRegister()

**FlareChain:**
- Para ID: $FLARECHAIN_PARA_ID
- Genesis Head: Upload \`flarechain-genesis-state\`
- Validation Code: Upload \`flarechain-genesis-wasm\`

**ETH-PBC:**
- Para ID: $ETH_PBC_PARA_ID
- Genesis Head: Upload \`eth-pbc-genesis-state\`
- Validation Code: Upload \`eth-pbc-genesis-wasm\`

### 3. Deploy Binaries to Servers

\`\`\`bash
# Copy to servers
scp flarechain-node user@flarechain-server:/opt/flarechain/
scp flarechain-testnet-raw.json user@flarechain-server:/opt/flarechain/
scp flarechain-collator.service user@flarechain-server:/etc/systemd/system/

scp eth-pbc-node user@eth-pbc-server:/opt/eth-pbc/
scp eth-pbc-testnet-raw.json user@eth-pbc-server:/opt/eth-pbc/
scp eth-pbc-collator.service user@eth-pbc-server:/etc/systemd/system/
\`\`\`

### 4. Start Collators

\`\`\`bash
# On FlareChain server
sudo systemctl daemon-reload
sudo systemctl enable flarechain-collator
sudo systemctl start flarechain-collator
sudo systemctl status flarechain-collator

# On ETH-PBC server
sudo systemctl daemon-reload
sudo systemctl enable eth-pbc-collator
sudo systemctl start eth-pbc-collator
sudo systemctl status eth-pbc-collator
\`\`\`

### 5. Monitor Logs

\`\`\`bash
# FlareChain
sudo journalctl -u flarechain-collator -f

# ETH-PBC
sudo journalctl -u eth-pbc-collator -f
\`\`\`

### 6. Setup HRMP Channels

Once both parachains are producing blocks, run:

\`\`\`bash
./setup-hrmp-channels.sh
\`\`\`

### 7. Verify Deployment

- Check block production on both parachains
- Verify HRMP channels are open
- Test XCM message passing
- Deploy and test smart contracts on ETH-PBC

## Endpoints

### FlareChain
- WebSocket: wss://flarechain-testnet.etrid.io
- RPC: https://flarechain-testnet-rpc.etrid.io

### ETH-PBC
- WebSocket: wss://eth-pbc-testnet.etrid.io
- RPC (Substrate): https://eth-pbc-testnet-rpc.etrid.io
- RPC (Ethereum): https://eth-pbc-testnet-eth.etrid.io

## Troubleshooting

- Check logs: \`journalctl -u <service-name> -f\`
- Verify sync: Connect to node via polkadot.js apps
- Check HRMP: Query \`hrmp.hrmpChannels(paraId1, paraId2)\`
- Monitor resources: \`htop\`, \`df -h\`

## Next Steps

1. Monitor parachain performance
2. Setup HRMP channels
3. Deploy example contracts
4. Run integration tests
5. Setup monitoring/alerting
EOF

# Create archive
echo ""
echo "Creating deployment archive..."
tar -czf "$OUTPUT_DIR.tar.gz" -C "$OUTPUT_DIR" .
echo "✅ Archive created: $OUTPUT_DIR.tar.gz"

# ═══════════════════════════════════════════════════════════════════════════════
# Summary
# ═══════════════════════════════════════════════════════════════════════════════

echo ""
echo "🎉 Deployment package ready!"
echo ""
echo "═══════════════════════════════════════════════════"
echo "📁 Output Directory: $OUTPUT_DIR/"
echo "📦 Archive: $OUTPUT_DIR.tar.gz"
echo ""
echo "📋 Next Steps:"
echo "1. Reserve para IDs on $RELAY_CHAIN"
echo "2. Register parachains using genesis files"
echo "3. Deploy binaries to servers"
echo "4. Start collators"
echo "5. Setup HRMP channels"
echo ""
echo "📖 See $OUTPUT_DIR/DEPLOYMENT_INSTRUCTIONS.md for details"
echo "═══════════════════════════════════════════════════"
