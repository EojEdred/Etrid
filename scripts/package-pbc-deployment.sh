#!/bin/bash
set -e

# Package PBC collators for deployment to Oracle Cloud VMs
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

LINUX_BINARIES="$SCRIPT_DIR/target/linux-release"
DEPLOY_PACKAGE="$SCRIPT_DIR/pbc-deployment-$(date +%Y%m%d-%H%M%S)"

echo "=== Creating PBC Deployment Package ==="
echo ""

# Check if Linux binaries exist
if [ ! -d "$LINUX_BINARIES" ]; then
    echo "❌ Error: Linux binaries not found. Run ./build-pbc-linux.sh first."
    exit 1
fi

# Create deployment package structure
mkdir -p "$DEPLOY_PACKAGE"/{binaries,systemd,scripts,chainspecs}

echo "Step 1/4: Copying Linux binaries..."
cp "$LINUX_BINARIES"/*-pbc-collator "$DEPLOY_PACKAGE/binaries/"

echo "Step 2/4: Creating systemd service templates..."
# Create a systemd service template for each collator
for COLLATOR in btc sol bnb edsc xrp matic sc-usdt xlm trx ada link doge; do
    cat > "$DEPLOY_PACKAGE/systemd/${COLLATOR}-pbc-collator.service" <<EOF
[Unit]
Description=${COLLATOR^^} PBC Collator Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=etrid
Group=etrid
WorkingDirectory=/opt/etrid/${COLLATOR}-pbc
ExecStart=/opt/etrid/bin/${COLLATOR}-pbc-collator \\
  --chain=/opt/etrid/chainspecs/${COLLATOR}-pbc-chainspec.json \\
  --base-path=/opt/etrid/${COLLATOR}-pbc/data \\
  --port=3033\${NODE_ID} \\
  --rpc-port=944\${NODE_ID} \\
  --ws-port=933\${NODE_ID} \\
  --prometheus-port=961\${NODE_ID} \\
  --name=${COLLATOR^^}-Validator-\${NODE_ID} \\
  --validator \\
  --rpc-cors=all \\
  --rpc-methods=Safe \\
  --rpc-external \\
  --ws-external
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF
done

echo "Step 3/4: Creating deployment scripts..."
cat > "$DEPLOY_PACKAGE/scripts/deploy.sh" <<'EOF'
#!/bin/bash
# Deploy PBC collators to Oracle Cloud VM
set -e

if [ "$EUID" -ne 0 ]; then
    echo "Please run as root (use sudo)"
    exit 1
fi

# Install binaries
echo "Installing PBC collator binaries..."
mkdir -p /opt/etrid/bin
cp binaries/*-pbc-collator /opt/etrid/bin/
chmod +x /opt/etrid/bin/*-pbc-collator

# Create etrid user
if ! id -u etrid > /dev/null 2>&1; then
    echo "Creating etrid user..."
    useradd -r -s /bin/bash -d /opt/etrid -m etrid
fi

# Create data directories
echo "Creating data directories..."
for COLLATOR in btc sol bnb edsc xrp matic sc-usdt xlm trx ada link doge; do
    mkdir -p /opt/etrid/${COLLATOR}-pbc/data
    chown -R etrid:etrid /opt/etrid/${COLLATOR}-pbc
done

# Install systemd services
echo "Installing systemd services..."
cp systemd/*.service /etc/systemd/system/
systemctl daemon-reload

echo ""
echo "✓ Deployment complete!"
echo ""
echo "To start a collator:"
echo "  NODE_ID=01 systemctl start btc-pbc-collator"
echo "  systemctl enable btc-pbc-collator"
echo ""
echo "Available collators:"
ls -1 /opt/etrid/bin/*-pbc-collator | xargs -n1 basename
EOF

chmod +x "$DEPLOY_PACKAGE/scripts/deploy.sh"

echo "Step 4/4: Creating deployment README..."
cat > "$DEPLOY_PACKAGE/README.md" <<EOF
# PBC Collators Deployment Package

Generated: $(date)

## Contents

- **binaries/**: All 12 PBC collator Linux binaries (x86_64)
- **systemd/**: Systemd service files for each collator
- **scripts/**: Deployment and management scripts
- **chainspecs/**: Chain specification files (to be generated)

## Deployment Instructions

### 1. Upload to VM

\`\`\`bash
scp -r pbc-deployment-* opc@<vm-ip>:~/
\`\`\`

### 2. Deploy on VM

\`\`\`bash
ssh opc@<vm-ip>
cd pbc-deployment-*
sudo ./scripts/deploy.sh
\`\`\`

### 3. Generate Chainspecs

Before starting collators, generate chainspecs for each chain:

\`\`\`bash
# On the VM
/opt/etrid/bin/btc-pbc-collator build-spec --chain local > /opt/etrid/chainspecs/btc-pbc-chainspec.json
# Repeat for all 12 collators
\`\`\`

### 4. Start Collators

\`\`\`bash
# Start individual collator
NODE_ID=01 sudo systemctl start btc-pbc-collator
sudo systemctl enable btc-pbc-collator

# Check status
sudo systemctl status btc-pbc-collator
sudo journalctl -u btc-pbc-collator -f
\`\`\`

## Available PBC Collators

1. btc-pbc-collator - Bitcoin Bridge
2. sol-pbc-collator - Solana Bridge
3. bnb-pbc-collator - Binance Smart Chain
4. edsc-pbc-collator - Etrid Designated Source Chain
5. xrp-pbc-collator - Ripple Bridge
6. matic-pbc-collator - Polygon Bridge
7. sc-usdt-pbc-collator - USDT Stablecoin Bridge
8. xlm-pbc-collator - Stellar Bridge
9. trx-pbc-collator - Tron Bridge
10. ada-pbc-collator - Cardano Bridge
11. link-pbc-collator - Chainlink Oracle Bridge
12. doge-pbc-collator - Dogecoin Bridge

## VM Assignment (Oracle Cloud)

Map each collator to your validator VMs (nodes 6-21):
- VM-6,7: btc-pbc, sol-pbc
- VM-8,9: bnb-pbc, edsc-pbc
- VM-10,11: xrp-pbc, matic-pbc
- VM-12,13: sc-usdt-pbc, xlm-pbc
- VM-14,15: trx-pbc, ada-pbc
- VM-16,17: link-pbc, doge-pbc
- VM-18-21: Backup/redundancy

## Monitoring

Check Prometheus metrics: http://<vm-ip>:9615/metrics
EOF

echo ""
echo "=== PACKAGE CREATED ==="
echo "Location: $DEPLOY_PACKAGE"
echo "Size: $(du -sh $DEPLOY_PACKAGE | cut -f1)"
echo ""
echo "Package contents:"
tree -L 2 "$DEPLOY_PACKAGE" 2>/dev/null || find "$DEPLOY_PACKAGE" -type f
echo ""
echo "Next: Upload to Oracle Cloud VMs and run deploy.sh"
