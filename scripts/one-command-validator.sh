#!/bin/bash
set -e

# =============================================================================
# Ëtrid FlareChain ONE-COMMAND Validator Setup & Start
# =============================================================================
# This script works for ANY node operator who clones the repository.
#
# Usage:
#   ./scripts/one-command-validator.sh                    # Auto-generate keys
#   ./scripts/one-command-validator.sh --bootnode <addr>  # Join existing network
#   VALIDATOR_KEY="//Alice" ./scripts/one-command-validator.sh  # Use specific key
# =============================================================================

# =============================================================================
# Auto-detect Repository and Binary Locations
# =============================================================================

# Find repository root (where this script lives)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Try to find the binary in common locations
find_binary() {
  local candidates=(
    "$REPO_ROOT/target/release/flarechain-node"
    "$REPO_ROOT/target/debug/flarechain-node"
    "$REPO_ROOT/flarechain-node"
    "./flarechain-node"
    "/opt/etrid/flarechain-node"
    "$(which flarechain-node 2>/dev/null || echo '')"
  )

  for binary in "${candidates[@]}"; do
    if [ -n "$binary" ] && [ -x "$binary" ]; then
      echo "$binary"
      return 0
    fi
  done

  return 1
}

# Try to find chain spec in common locations
find_chain_spec() {
  local candidates=(
    "$REPO_ROOT/infrastructure/chain-specs/flarechain-local-raw.json"
    "$REPO_ROOT/chainspec.json"
    "/opt/etrid/chainspec.json"
    "./chainspec.json"
  )

  for spec in "${candidates[@]}"; do
    if [ -f "$spec" ]; then
      echo "$spec"
      return 0
    fi
  done

  return 1
}

# =============================================================================
# Configuration with Smart Defaults
# =============================================================================

# Binary location (auto-detect or use environment variable)
if [ -z "$FLARECHAIN_BINARY" ]; then
  FLARECHAIN_BINARY=$(find_binary)
  if [ -z "$FLARECHAIN_BINARY" ]; then
    echo "❌ Error: Could not find flarechain-node binary"
    echo ""
    echo "Please either:"
    echo "  1. Build the binary: cargo build --release"
    echo "  2. Set FLARECHAIN_BINARY environment variable"
    echo "  3. Put flarechain-node in your PATH"
    exit 1
  fi
fi

# Chain spec location (auto-detect or use environment variable)
if [ -z "$CHAIN_SPEC" ]; then
  CHAIN_SPEC=$(find_chain_spec)
  if [ -z "$CHAIN_SPEC" ]; then
    echo "❌ Error: Could not find chain spec file"
    echo ""
    echo "Please either:"
    echo "  1. Use default location: infrastructure/chain-specs/flarechain-local-raw.json"
    echo "  2. Set CHAIN_SPEC environment variable"
    exit 1
  fi
fi

# Data directory (use ~/.etrid by default, or /var/lib/etrid if running as root)
if [ -z "$BASE_PATH" ]; then
  if [ "$(id -u)" -eq 0 ]; then
    BASE_PATH="/var/lib/etrid"
  else
    BASE_PATH="$HOME/.etrid"
  fi
fi

# Node name (default to username + hostname)
if [ -z "$NODE_NAME" ]; then
  NODE_NAME="etrid-$(whoami)-$(hostname -s)"
fi

# Validator key (empty = auto-generate random key)
VALIDATOR_KEY="${VALIDATOR_KEY:-}"

# Network ports
RPC_PORT="${RPC_PORT:-9944}"
WS_PORT="${WS_PORT:-9945}"
P2P_PORT="${P2P_PORT:-30333}"

# Bootnode (optional - leave empty for first node in network)
BOOTNODE="${BOOTNODE:-}"

# =============================================================================
# Parse Command Line Arguments
# =============================================================================

while [[ $# -gt 0 ]]; do
  case $1 in
    --bootnode)
      BOOTNODE="$2"
      shift 2
      ;;
    --validator-key)
      VALIDATOR_KEY="$2"
      shift 2
      ;;
    --base-path)
      BASE_PATH="$2"
      shift 2
      ;;
    --name)
      NODE_NAME="$2"
      shift 2
      ;;
    --rpc-port)
      RPC_PORT="$2"
      shift 2
      ;;
    --ws-port)
      WS_PORT="$2"
      shift 2
      ;;
    --port)
      P2P_PORT="$2"
      shift 2
      ;;
    --help|-h)
      echo "Ëtrid FlareChain One-Command Validator"
      echo ""
      echo "Usage: $0 [OPTIONS]"
      echo ""
      echo "Options:"
      echo "  --bootnode <addr>       Connect to existing node (multiaddr format)"
      echo "  --validator-key <key>   Use specific validator key (e.g. //Alice)"
      echo "  --base-path <path>      Data directory (default: ~/.etrid or /var/lib/etrid)"
      echo "  --name <name>           Node name (default: etrid-<user>-<hostname>)"
      echo "  --rpc-port <port>       RPC port (default: 9944)"
      echo "  --ws-port <port>        WebSocket port (default: 9945)"
      echo "  --port <port>           P2P port (default: 30333)"
      echo "  --help, -h              Show this help"
      echo ""
      echo "Environment Variables:"
      echo "  FLARECHAIN_BINARY       Path to flarechain-node binary"
      echo "  CHAIN_SPEC              Path to chain spec JSON file"
      echo "  BASE_PATH               Data directory"
      echo "  NODE_NAME               Node name"
      echo "  VALIDATOR_KEY           Validator key"
      echo "  BOOTNODE                Bootnode address"
      echo ""
      echo "Examples:"
      echo "  # First node (bootstrap)"
      echo "  $0"
      echo ""
      echo "  # Join existing network"
      echo "  $0 --bootnode /ip4/192.168.1.100/tcp/30333/p2p/12D3KooW..."
      echo ""
      echo "  # Use specific validator key"
      echo "  VALIDATOR_KEY=\"//Alice\" $0"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

# =============================================================================
# Detect Network Interface and IP
# =============================================================================

# Try to detect the best IP address to advertise
detect_ip() {
  # Try to get primary interface IP
  local ip=""

  # Method 1: ip command (Linux)
  if command -v ip &> /dev/null; then
    ip=$(ip route get 1.1.1.1 2>/dev/null | grep -oP 'src \K\S+' || echo "")
  fi

  # Method 2: ifconfig (Mac/BSD)
  if [ -z "$ip" ] && command -v ifconfig &> /dev/null; then
    ip=$(ifconfig | grep "inet " | grep -v 127.0.0.1 | head -1 | awk '{print $2}')
  fi

  # Method 3: hostname -I (Linux fallback)
  if [ -z "$ip" ] && command -v hostname &> /dev/null; then
    ip=$(hostname -I 2>/dev/null | awk '{print $1}' || echo "")
  fi

  # Fallback: localhost (will only work for local testing)
  if [ -z "$ip" ]; then
    ip="127.0.0.1"
  fi

  echo "$ip"
}

DETECTED_IP=$(detect_ip)

# =============================================================================
# Display Configuration
# =============================================================================

echo "================================================================================"
echo "  Ëtrid FlareChain ONE-COMMAND Validator"
echo "================================================================================"
echo "  Binary:        $FLARECHAIN_BINARY"
echo "  Chain Spec:    $CHAIN_SPEC"
echo "  Base Path:     $BASE_PATH"
echo "  Node Name:     $NODE_NAME"
echo "  Detected IP:   $DETECTED_IP"
echo "  P2P Port:      $P2P_PORT"
echo "  RPC Port:      $RPC_PORT"
echo "  WS Port:       $WS_PORT"
if [ -n "$BOOTNODE" ]; then
  echo "  Bootnode:      $BOOTNODE"
else
  echo "  Mode:          Bootstrap (first node)"
fi
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

# Extract chain ID from chain spec
CHAIN_ID=$(grep '"id"' "$CHAIN_SPEC" | head -1 | sed 's/.*"id": "\(.*\)".*/\1/')
NETWORK_DIR="$BASE_PATH/chains/$CHAIN_ID/network"
NETWORK_KEY_PATH="$NETWORK_DIR/secret_ed25519"

if [ -f "$NETWORK_KEY_PATH" ]; then
  echo "  ✓ Network key already exists at: $NETWORK_KEY_PATH"
elif [ -f "$KEYS_DIR/network_secret" ]; then
  echo "  ✓ Found backup network key, restoring..."
  mkdir -p "$NETWORK_DIR"
  cp "$KEYS_DIR/network_secret" "$NETWORK_KEY_PATH"
else
  echo "  Generating new network key..."
  mkdir -p "$NETWORK_DIR"

  # Generate network key directly in the correct location
  $FLARECHAIN_BINARY key generate-node-key --file="$NETWORK_KEY_PATH" > /dev/null 2>&1

  # Save a backup copy
  cp "$NETWORK_KEY_PATH" "$KEYS_DIR/network_secret"
  chmod 600 "$KEYS_DIR/network_secret"
  echo "  ✓ Network key generated"
  echo "  ✓ Backup saved to: $KEYS_DIR/network_secret"
fi

# Extract peer ID
PEER_ID=$($FLARECHAIN_BINARY key inspect-node-key --file="$NETWORK_KEY_PATH" 2>/dev/null)
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
    VALIDATOR_KEY=$($FLARECHAIN_BINARY key generate --scheme sr25519 --output-type text 2>/dev/null | grep "Secret seed" | awk '{print $3}')
    echo "$VALIDATOR_KEY" > "$KEYS_DIR/validator_seed"
    chmod 600 "$KEYS_DIR/validator_seed"
    echo "  ✓ Validator seed saved to: $KEYS_DIR/validator_seed"
    echo "  ⚠  IMPORTANT: Backup this file securely!"
  fi
else
  echo "  Using provided validator key"
  # Only save if it doesn't exist (don't overwrite existing keys)
  if [ ! -f "$KEYS_DIR/validator_seed" ]; then
    echo "$VALIDATOR_KEY" > "$KEYS_DIR/validator_seed"
    chmod 600 "$KEYS_DIR/validator_seed"
  fi
fi

# Derive public keys
AURA_KEY=$($FLARECHAIN_BINARY key inspect --scheme sr25519 "$VALIDATOR_KEY" 2>/dev/null | grep "SS58 Address" | awk '{print $3}')
GRANDPA_KEY=$($FLARECHAIN_BINARY key inspect --scheme ed25519 "$VALIDATOR_KEY" 2>/dev/null | grep "SS58 Address" | awk '{print $3}')

echo "  ✓ AURA (sr25519):   $AURA_KEY"
echo "  ✓ GRANDPA (ed25519): $GRANDPA_KEY"
echo ""

# =============================================================================
# Insert All Validator Keys to Keystore
# =============================================================================
echo "[3/4] Inserting Keys to Keystore..."

# AURA key
$FLARECHAIN_BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN_SPEC" \
  --key-type=aura \
  --scheme=sr25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1
echo "  ✓ AURA key inserted"

# GRANDPA key
$FLARECHAIN_BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN_SPEC" \
  --key-type=gran \
  --scheme=ed25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1
echo "  ✓ GRANDPA key inserted"

# ASF key (Async Finality)
$FLARECHAIN_BINARY key insert \
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
Binary: $FLARECHAIN_BINARY
Chain Spec: $CHAIN_SPEC

=== Keys ===
Secret Seed: $VALIDATOR_KEY
AURA Key:    $AURA_KEY
GRANDPA Key: $GRANDPA_KEY
Peer ID:     $PEER_ID

=== Bootnode Address ===
Share this with other validators:
/ip4/$DETECTED_IP/tcp/$P2P_PORT/p2p/$PEER_ID

=== Backup Files ===
$KEYS_DIR/validator_seed (CRITICAL - backup securely!)
$KEYS_DIR/network_secret

=== To Join This Network ===
Other nodes can join by running:
$0 --bootnode /ip4/$DETECTED_IP/tcp/$P2P_PORT/p2p/$PEER_ID
EOF

chmod 600 "$KEYS_DIR/node_info.txt"

echo "================================================================================"
echo "  Bootstrap Complete!"
echo "================================================================================"
echo ""
if [ -z "$BOOTNODE" ]; then
  echo "  🚀 This will be a BOOTSTRAP NODE (first node in the network)"
  echo ""
  echo "  📋 Share this bootnode address with other validators:"
  echo "     /ip4/$DETECTED_IP/tcp/$P2P_PORT/p2p/$PEER_ID"
  echo ""
  echo "  Others can join by running:"
  echo "     $0 --bootnode /ip4/$DETECTED_IP/tcp/$P2P_PORT/p2p/$PEER_ID"
  echo ""
else
  echo "  🔗 This node will connect to: $BOOTNODE"
  echo ""
fi
echo "  💾 All info saved to: $KEYS_DIR/node_info.txt"
echo "  🔑 Backup location: $KEYS_DIR/validator_seed"
echo "================================================================================"
echo ""

# =============================================================================
# Start Validator
# =============================================================================
echo "[4/4] Starting Validator..."
echo ""
sleep 2

# Build node command
NODE_CMD="$FLARECHAIN_BINARY \
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

# Add public-addr for bootstrap node, or bootnodes for joining nodes
if [ -z "$BOOTNODE" ]; then
  NODE_CMD="$NODE_CMD --public-addr=/ip4/$DETECTED_IP/tcp/$P2P_PORT"
else
  NODE_CMD="$NODE_CMD --bootnodes $BOOTNODE"
fi

echo "Starting with command:"
echo "$NODE_CMD"
echo ""

exec $NODE_CMD
