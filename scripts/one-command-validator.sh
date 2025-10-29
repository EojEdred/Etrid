#!/bin/bash
set -e

# =============================================================================
# Ëtrid FlareChain ONE-COMMAND Validator Setup & Start
# =============================================================================
# This script does EVERYTHING in one command:
#   1. Auto-detects if it's first node (bootstrap) or joining node
#   2. Generates all keys if they don't exist
#   3. Inserts all validator keys (AURA, GRANDPA, ASF)
#   4. Starts the validator node immediately
# =============================================================================

# Configuration - can be overridden with environment variables
BINARY="${FLARECHAIN_BINARY:-/opt/etrid/flarechain-node}"
CHAIN_SPEC="${CHAIN_SPEC:-/opt/etrid/chainspec.json}"
BASE_PATH="${BASE_PATH:-/var/lib/etrid}"
NODE_NAME="${NODE_NAME:-etrid-validator-$(hostname)}"
VALIDATOR_KEY="${VALIDATOR_KEY:-}"  # e.g. "//Alice" or leave empty for auto-generate

# Network configuration
RPC_PORT="${RPC_PORT:-9944}"
WS_PORT="${WS_PORT:-9945}"
P2P_PORT="${P2P_PORT:-30333}"

# Bootnode (optional - leave empty for first node)
BOOTNODE="${BOOTNODE:-}"  # e.g. "/ip4/172.16.0.5/tcp/30333/p2p/12D3KooW..."

# Private IP for Azure VMs (auto-detected if not set)
PRIVATE_IP="${PRIVATE_IP:-}"

# =============================================================================
# Detect Private IP
# =============================================================================
if [ -z "$PRIVATE_IP" ]; then
  # Try to detect Azure private IP
  PRIVATE_IP=$(ip addr show eth0 2>/dev/null | grep "inet " | awk '{print $2}' | cut -d/ -f1 || echo "")
  if [ -z "$PRIVATE_IP" ]; then
    PRIVATE_IP=$(hostname -I | awk '{print $1}')
  fi
fi

echo "================================================================================"
echo "  Ëtrid FlareChain ONE-COMMAND Validator"
echo "================================================================================"
echo "  Node Name:     $NODE_NAME"
echo "  Chain Spec:    $CHAIN_SPEC"
echo "  Base Path:     $BASE_PATH"
echo "  Private IP:    $PRIVATE_IP"
echo "  Bootnode:      ${BOOTNODE:-<none - will be bootstrap node>}"
echo "================================================================================"
echo ""

# Create directories
mkdir -p "$BASE_PATH"
KEYS_DIR="$BASE_PATH/keys"
mkdir -p "$KEYS_DIR"

# =============================================================================
# Network Key Setup
# =============================================================================
echo "[1/4] Network Key Setup..."

# Determine the correct chain-specific network path
CHAIN_ID=$(grep '"id"' "$CHAIN_SPEC" | head -1 | sed 's/.*"id": "\(.*\)".*/\1/')
NETWORK_DIR="$BASE_PATH/chains/$CHAIN_ID/network"
NETWORK_KEY_PATH="$NETWORK_DIR/secret_ed25519"

if [ -f "$NETWORK_KEY_PATH" ]; then
  echo "  ✓ Network key already exists at: $NETWORK_KEY_PATH"
else
  echo "  Generating new network key..."
  mkdir -p "$NETWORK_DIR"

  # Generate network key directly in the correct location
  $BINARY key generate-node-key --file="$NETWORK_KEY_PATH" > /dev/null 2>&1

  # Also save a backup copy
  cp "$NETWORK_KEY_PATH" "$KEYS_DIR/network_secret"
  echo "  ✓ Network key generated at: $NETWORK_KEY_PATH"
  echo "  ✓ Backup saved to: $KEYS_DIR/network_secret"
fi

# Extract peer ID
PEER_ID=$($BINARY key inspect-node-key --file="$NETWORK_KEY_PATH" 2>/dev/null)
echo "  ✓ Peer ID: $PEER_ID"
echo ""

# =============================================================================
# Validator Key Setup
# =============================================================================
echo "[2/4] Validator Key Setup..."

if [ -z "$VALIDATOR_KEY" ]; then
  # Check for existing seed
  if [ -f "$KEYS_DIR/validator_seed" ]; then
    VALIDATOR_KEY=$(cat "$KEYS_DIR/validator_seed")
    echo "  ✓ Using existing validator seed"
  else
    echo "  Generating new random validator key..."
    VALIDATOR_KEY=$($BINARY key generate --scheme sr25519 --output-type text 2>/dev/null | grep "Secret seed" | awk '{print $3}')
    echo "$VALIDATOR_KEY" > "$KEYS_DIR/validator_seed"
    chmod 600 "$KEYS_DIR/validator_seed"
    echo "  ✓ Validator seed saved to: $KEYS_DIR/validator_seed"
    echo "  ⚠  BACKUP THIS FILE SECURELY!"
  fi
else
  echo "  Using provided validator key: $VALIDATOR_KEY"
  echo "$VALIDATOR_KEY" > "$KEYS_DIR/validator_seed"
  chmod 600 "$KEYS_DIR/validator_seed"
fi

# Derive public keys
AURA_KEY=$($BINARY key inspect --scheme sr25519 "$VALIDATOR_KEY" 2>/dev/null | grep "SS58 Address" | awk '{print $3}')
GRANDPA_KEY=$($BINARY key inspect --scheme ed25519 "$VALIDATOR_KEY" 2>/dev/null | grep "SS58 Address" | awk '{print $3}')

echo "  ✓ AURA (sr25519):   $AURA_KEY"
echo "  ✓ GRANDPA (ed25519): $GRANDPA_KEY"
echo ""

# =============================================================================
# Insert All Validator Keys to Keystore
# =============================================================================
echo "[3/4] Inserting Keys to Keystore..."

# AURA key
$BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN_SPEC" \
  --key-type=aura \
  --scheme=sr25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1
echo "  ✓ AURA key inserted"

# GRANDPA key
$BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN_SPEC" \
  --key-type=gran \
  --scheme=ed25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1
echo "  ✓ GRANDPA key inserted"

# ASF key (Critical for Async Finality!)
$BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN_SPEC" \
  --key-type=asfk \
  --scheme=sr25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1
echo "  ✓ ASF key inserted"
echo ""

# =============================================================================
# Save Bootstrap Info
# =============================================================================
cat > "$KEYS_DIR/node_info.txt" <<EOF
Node: $NODE_NAME
Chain: $CHAIN_ID
Generated: $(date)

=== Keys ===
Secret Seed: $VALIDATOR_KEY
AURA Key:    $AURA_KEY
GRANDPA Key: $GRANDPA_KEY
Peer ID:     $PEER_ID

=== Bootnode Address ===
Share this with other validators:
/ip4/$PRIVATE_IP/tcp/$P2P_PORT/p2p/$PEER_ID

=== Backup Files ===
$KEYS_DIR/validator_seed
$KEYS_DIR/network_secret
EOF

chmod 600 "$KEYS_DIR/node_info.txt"

echo "================================================================================"
echo "  Bootstrap Complete!"
echo "================================================================================"
echo ""
if [ -z "$BOOTNODE" ]; then
  echo "  This will be a BOOTSTRAP NODE (first node)"
  echo ""
  echo "  Share this bootnode address with other validators:"
  echo "  /ip4/$PRIVATE_IP/tcp/$P2P_PORT/p2p/$PEER_ID"
  echo ""
else
  echo "  This node will connect to bootstrap node:"
  echo "  $BOOTNODE"
  echo ""
fi
echo "  All bootstrap info saved to: $KEYS_DIR/node_info.txt"
echo "================================================================================"
echo ""

# =============================================================================
# Start Validator
# =============================================================================
echo "[4/4] Starting Validator..."
echo ""
sleep 2

# Build node command
NODE_CMD="$BINARY \
  --chain=$CHAIN_SPEC \
  --base-path=$BASE_PATH \
  --name=$NODE_NAME \
  --validator \
  --rpc-external \
  --ws-external \
  --prometheus-external \
  --rpc-port=$RPC_PORT \
  --ws-port=$WS_PORT \
  --port=$P2P_PORT \
  --rpc-cors=all"

# Add public-addr for bootstrap node (no bootnode parameter)
if [ -z "$BOOTNODE" ]; then
  NODE_CMD="$NODE_CMD --public-addr=/ip4/$PRIVATE_IP/tcp/$P2P_PORT"
else
  # Add bootnode for joining nodes
  NODE_CMD="$NODE_CMD --bootnodes $BOOTNODE"
fi

echo "Starting with command:"
echo "$NODE_CMD"
echo ""

exec $NODE_CMD
