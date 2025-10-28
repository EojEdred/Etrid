#!/bin/bash
set -e

# =============================================================================
# Ëtrid FlareChain Validator Bootstrap Script
# =============================================================================
# This script sets up a validator node with all required keys.
# Can be used by any node operator to join the network.
# =============================================================================

# Configuration
BINARY="${FLARECHAIN_BINARY:-./flarechain-node}"
CHAIN="${CHAIN:-local}"
BASE_PATH="${BASE_PATH:-/var/lib/etrid}"
KEYS_DIR="${BASE_PATH}/keys"
NODE_NAME="${NODE_NAME:-etrid-validator-$(hostname)}"

# Parse arguments
BOOTNODES=""
VALIDATOR_KEY=""
RPC_PORT="${RPC_PORT:-9944}"
WS_PORT="${WS_PORT:-9945}"
P2P_PORT="${P2P_PORT:-30333}"

while [[ $# -gt 0 ]]; do
  case $1 in
    --bootnode)
      BOOTNODES="$BOOTNODES --bootnodes $2"
      shift 2
      ;;
    --validator-key)
      VALIDATOR_KEY="$2"
      shift 2
      ;;
    --base-path)
      BASE_PATH="$2"
      KEYS_DIR="${BASE_PATH}/keys"
      shift 2
      ;;
    --chain)
      CHAIN="$2"
      shift 2
      ;;
    --name)
      NODE_NAME="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      echo "Usage: $0 [--bootnode <multiaddr>] [--validator-key <seed>] [--base-path <path>] [--chain <chain>] [--name <name>]"
      exit 1
      ;;
  esac
done

echo "================================================================================"
echo "  Ëtrid FlareChain Validator Bootstrap"
echo "================================================================================"
echo "  Node Name:     $NODE_NAME"
echo "  Chain:         $CHAIN"
echo "  Base Path:     $BASE_PATH"
echo "  Keys Dir:      $KEYS_DIR"
echo "================================================================================"
echo ""

# Create directories
mkdir -p "$BASE_PATH" "$KEYS_DIR"

# =============================================================================
# Step 1: Generate Network Key
# =============================================================================
echo "Step 1/5: Network Key Setup"
echo "----------------------------"

if [ -f "$KEYS_DIR/network_secret" ]; then
  echo "✓ Network key already exists"
  NETWORK_KEY_PATH="$KEYS_DIR/network_secret"
else
  echo "Generating new network key..."
  $BINARY key generate-node-key --base-path="$BASE_PATH" > /dev/null 2>&1

  # Save the network key for persistence
  if [ -f "$BASE_PATH/network/secret_ed25519" ]; then
    cp "$BASE_PATH/network/secret_ed25519" "$KEYS_DIR/network_secret"
    NETWORK_KEY_PATH="$KEYS_DIR/network_secret"
    echo "✓ Network key generated and saved to: $KEYS_DIR/network_secret"
  else
    echo "⚠ Network key generated but not saved"
    NETWORK_KEY_PATH="$BASE_PATH/network/secret_ed25519"
  fi
fi

# Extract and display peer ID
PEER_ID=$($BINARY key inspect-node-key --file="$NETWORK_KEY_PATH" 2>/dev/null || echo "unknown")
echo "✓ Peer ID: $PEER_ID"
echo ""

# =============================================================================
# Step 2: Generate or Use Validator Key
# =============================================================================
echo "Step 2/5: Validator Key Setup"
echo "------------------------------"

if [ -z "$VALIDATOR_KEY" ]; then
  # Generate new random validator key
  if [ -f "$KEYS_DIR/validator_seed" ]; then
    echo "Using existing validator seed from: $KEYS_DIR/validator_seed"
    VALIDATOR_KEY=$(cat "$KEYS_DIR/validator_seed")
  else
    echo "Generating new validator key..."
    VALIDATOR_KEY=$($BINARY key generate --scheme sr25519 --output-type text 2>/dev/null | grep "Secret seed" | awk '{print $3}')
    echo "$VALIDATOR_KEY" > "$KEYS_DIR/validator_seed"
    echo "✓ Validator seed saved to: $KEYS_DIR/validator_seed"
    echo "  IMPORTANT: Back up this file securely!"
  fi
else
  echo "Using provided validator key"
  echo "$VALIDATOR_KEY" > "$KEYS_DIR/validator_seed"
fi

# Derive validator public keys
echo "Deriving validator public keys..."
AURA_KEY=$($BINARY key inspect --scheme sr25519 "$VALIDATOR_KEY" 2>/dev/null | grep "SS58 Address" | awk '{print $3}')
GRANDPA_KEY=$($BINARY key inspect --scheme ed25519 "$VALIDATOR_KEY" 2>/dev/null | grep "SS58 Address" | awk '{print $3}')

echo "✓ AURA Key (sr25519):   $AURA_KEY"
echo "✓ GRANDPA Key (ed25519): $GRANDPA_KEY"

# Save public keys for reference
cat > "$KEYS_DIR/validator_keys.txt" <<EOF
Validator Keys for: $NODE_NAME
Generated: $(date)

Secret Seed: $VALIDATOR_KEY
AURA Key (sr25519):   $AURA_KEY
GRANDPA Key (ed25519): $GRANDPA_KEY
Peer ID: $PEER_ID

=== To add to chain spec ===
AURA: "$AURA_KEY"
GRANDPA: "$GRANDPA_KEY"
EOF

echo "✓ Keys saved to: $KEYS_DIR/validator_keys.txt"
echo ""

# =============================================================================
# Step 3: Insert AURA Key
# =============================================================================
echo "Step 3/5: Inserting AURA Key"
echo "----------------------------"

$BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN" \
  --key-type=aura \
  --scheme=sr25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1

echo "✓ AURA key inserted into keystore"
echo ""

# =============================================================================
# Step 4: Insert GRANDPA Key
# =============================================================================
echo "Step 4/5: Inserting GRANDPA Key"
echo "--------------------------------"

$BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN" \
  --key-type=gran \
  --scheme=ed25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1

echo "✓ GRANDPA key inserted into keystore"
echo ""

# =============================================================================
# Step 5: Insert ASF Key (Async Finality)
# =============================================================================
echo "Step 5/5: Inserting ASF Key"
echo "---------------------------"

$BINARY key insert \
  --base-path="$BASE_PATH" \
  --chain="$CHAIN" \
  --key-type=asfk \
  --scheme=sr25519 \
  --suri="$VALIDATOR_KEY" > /dev/null 2>&1

echo "✓ ASF key inserted into keystore"
echo ""

# =============================================================================
# Display Bootstrap Information
# =============================================================================
echo "================================================================================"
echo "  Bootstrap Complete!"
echo "================================================================================"
echo ""
echo "Your validator node is ready to start."
echo ""
echo "=== Connection Information ==="
echo "Bootnode Address (share this with other nodes):"
echo "  /ip4/YOUR_PUBLIC_IP/tcp/$P2P_PORT/p2p/$PEER_ID"
echo ""
echo "=== Next Steps ==="
echo ""
echo "1. Note your public IP address:"
echo "   curl -s ifconfig.me"
echo ""
echo "2. Start the validator:"
echo "   $BINARY \\"
echo "     --chain=$CHAIN \\"
echo "     --base-path=$BASE_PATH \\"
echo "     --name=\"$NODE_NAME\" \\"
echo "     --validator \\"
echo "     --rpc-external \\"
echo "     --ws-external \\"
echo "     --prometheus-external \\"
echo "     --rpc-port=$RPC_PORT \\"
echo "     --ws-port=$WS_PORT \\"
echo "     --port=$P2P_PORT \\"
echo "     --rpc-cors=all"

if [ -n "$BOOTNODES" ]; then
  echo "     $BOOTNODES"
fi

echo ""
echo "3. Share your bootnode address with other validators"
echo ""
echo "=== Important Files ==="
echo "  Validator Seed:  $KEYS_DIR/validator_seed"
echo "  Network Key:     $KEYS_DIR/network_secret"
echo "  Public Keys:     $KEYS_DIR/validator_keys.txt"
echo ""
echo "⚠  BACKUP THESE FILES SECURELY!"
echo "================================================================================"
