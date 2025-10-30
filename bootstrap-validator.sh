#!/bin/bash
# Ëtrid Validator Bootstrap Script
# This script automates the complete setup of a validator node
# Usage: sudo ./bootstrap-validator.sh <validator-name> <session-seed> <aura-key> <grandpa-key> <asf-key> <node-key>

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Ëtrid Validator Bootstrap Script                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}⚠️  This script must be run as root or with sudo${NC}"
    echo "Usage: sudo ./bootstrap-validator.sh <validator-name> <session-seed> <aura-key> <grandpa-key> <asf-key> <node-key>"
    exit 1
fi

# Check arguments
if [ "$#" -ne 6 ]; then
    echo -e "${RED}❌ Error: Missing required arguments${NC}"
    echo ""
    echo "Usage: sudo ./bootstrap-validator.sh <validator-name> <session-seed> <aura-key> <grandpa-key> <asf-key> <node-key>"
    echo ""
    echo "Arguments:"
    echo "  validator-name : Name of your validator (e.g., 'Gizzi-Overseer')"
    echo "  session-seed   : Session key seed (0x...)"
    echo "  aura-key       : AURA public key (0x...)"
    echo "  grandpa-key    : GRANDPA public key (0x...)"
    echo "  asf-key        : ASF public key (0x...)"
    echo "  node-key       : P2P network key (hex string without 0x)"
    echo ""
    echo "Example:"
    echo "  sudo ./bootstrap-validator.sh 'My-Validator' \\"
    echo "    '0x1234...abcd' \\"
    echo "    '0x5678...efgh' \\"
    echo "    '0x9abc...ijkl' \\"
    echo "    '0x5678...mnop' \\"
    echo "    'd19cfdcc168877d8c772f629e5a1010ae620acb6a6e0302c3690a6bc4402a4a8'"
    exit 1
fi

VALIDATOR_NAME="$1"
SESSION_SEED="$2"
AURA_KEY="$3"
GRANDPA_KEY="$4"
ASF_KEY="$5"
NODE_KEY="$6"

BINARY_PATH="/usr/local/bin/flarechain-node"
BASE_PATH="/var/lib/etrid"
SERVICE_FILE="/etc/systemd/system/etrid-validator.service"

# Verify binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}❌ Error: Validator binary not found at $BINARY_PATH${NC}"
    echo "Please install the binary first:"
    echo "  sudo cp /path/to/flarechain-node /usr/local/bin/"
    exit 1
fi

echo -e "${GREEN}✅ Validator binary found${NC}"
echo ""

# Step 1: Create directory structure and set permissions
echo "📁 Creating directory structure..."
mkdir -p "$BASE_PATH/keys"
chown -R ubuntu:ubuntu "$BASE_PATH"
chmod 755 "$BASE_PATH"
chmod 700 "$BASE_PATH/keys"
echo -e "${GREEN}✅ Directory structure created${NC}"
echo ""

# Step 2: Insert validator keys
echo "🔑 Inserting validator session keys..."

# Insert AURA key (Sr25519)
echo "  Inserting AURA key..."
sudo -u ubuntu "$BINARY_PATH" key insert \
  --base-path "$BASE_PATH" \
  --chain dev \
  --key-type aura \
  --suri "$SESSION_SEED" \
  --scheme Sr25519 2>&1 | grep -v "^$" || true

# Insert GRANDPA key (Ed25519)
echo "  Inserting GRANDPA key..."
sudo -u ubuntu "$BINARY_PATH" key insert \
  --base-path "$BASE_PATH" \
  --chain dev \
  --key-type gran \
  --suri "$SESSION_SEED" \
  --scheme Ed25519 2>&1 | grep -v "^$" || true

# Insert ASF key (Sr25519) - NOTE: key-type is 'asfk' not 'asf_'
echo "  Inserting ASF key..."
sudo -u ubuntu "$BINARY_PATH" key insert \
  --base-path "$BASE_PATH" \
  --chain dev \
  --key-type asfk \
  --suri "$SESSION_SEED" \
  --scheme Sr25519 2>&1 | grep -v "^$" || true

# Verify keys were inserted
KEYSTORE_PATH=$(find "$BASE_PATH" -name keystore -type d | head -1)
if [ -z "$KEYSTORE_PATH" ]; then
    echo -e "${RED}❌ Error: Keystore directory not found${NC}"
    exit 1
fi

KEY_COUNT=$(ls -1 "$KEYSTORE_PATH" | wc -l)
if [ "$KEY_COUNT" -ne 3 ]; then
    echo -e "${YELLOW}⚠️  Warning: Expected 3 keys, found $KEY_COUNT${NC}"
    echo "Keys in keystore:"
    ls -lh "$KEYSTORE_PATH"
fi

echo -e "${GREEN}✅ Session keys inserted ($KEY_COUNT keys)${NC}"
echo "   Keystore: $KEYSTORE_PATH"
echo ""

# Step 3: Create systemd service
echo "🔧 Creating systemd service..."

cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Ëtrid FlareChain Validator Node ($VALIDATOR_NAME)
After=network.target
StartLimitIntervalSec=0

[Service]
Type=simple
Restart=always
RestartSec=10
User=ubuntu
ExecStart=$BINARY_PATH \\
  --base-path $BASE_PATH \\
  --chain dev \\
  --name "$VALIDATOR_NAME" \\
  --validator \\
  --port 30333 \\
  --rpc-port 9944 \\
  --prometheus-port 9615 \\
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \\
  --node-key $NODE_KEY

[Install]
WantedBy=multi-user.target
EOF

echo -e "${GREEN}✅ Systemd service created${NC}"
echo ""

# Step 4: Enable and start service
echo "🚀 Enabling and starting validator service..."
systemctl daemon-reload
systemctl enable etrid-validator
systemctl start etrid-validator

# Wait for service to start
sleep 5

# Check service status
if systemctl is-active --quiet etrid-validator; then
    echo -e "${GREEN}✅ Validator service is running${NC}"
else
    echo -e "${RED}❌ Validator service failed to start${NC}"
    echo "Checking logs..."
    journalctl -u etrid-validator -n 50 --no-pager
    exit 1
fi

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║              Validator Bootstrap Complete!                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Validator Configuration:"
echo "  Name: $VALIDATOR_NAME"
echo "  Base Path: $BASE_PATH"
echo "  Keystore: $KEYSTORE_PATH"
echo "  Service: etrid-validator.service"
echo ""
echo "Service Management Commands:"
echo "  sudo systemctl status etrid-validator   # Check status"
echo "  sudo systemctl stop etrid-validator     # Stop validator"
echo "  sudo systemctl restart etrid-validator  # Restart validator"
echo "  sudo journalctl -u etrid-validator -f   # View logs"
echo ""
echo "Checking validator status..."
systemctl status etrid-validator --no-pager -l
echo ""
echo -e "${GREEN}✅ Bootstrap complete! Your validator is now running.${NC}"
