#!/bin/bash
# =============================================================================
# VM #2 - Validator Node (Uses Generic Script)
# IP: 172.177.44.73 (public), 172.16.0.4 (private)
# =============================================================================

set -e

echo "================================================================================"
echo "  Ëtrid VM #2 - Validator Node Deployment"
echo "================================================================================"
echo ""
echo "This script sets up Bob as a validator using the generic"
echo "one-command-validator.sh script and connects to VM #1."
echo ""
echo "You need Alice's Peer ID from VM #1."
echo ""
echo "On VM #1, find the line that says:"
echo "  'Peer ID: 12D3KooW...'"
echo ""
read -p "Enter Alice's Peer ID (12D3KooW...): " ALICE_PEER_ID

if [ -z "$ALICE_PEER_ID" ]; then
  echo "❌ Error: Peer ID cannot be empty!"
  exit 1
fi

# Construct bootnode address using VM #1's private IP
ALICE_BOOTNODE="/ip4/172.16.0.5/tcp/30333/p2p/$ALICE_PEER_ID"

echo ""
echo "✓ Will connect to: $ALICE_BOOTNODE"
echo ""
read -p "Press Enter to continue..."

# Export VM-specific configuration
export FLARECHAIN_BINARY="/opt/etrid/flarechain-node"
export CHAIN_SPEC="/opt/etrid/chainspec.json"
export BASE_PATH="/var/lib/etrid"
export NODE_NAME="etrid-validator-02"
export VALIDATOR_KEY="//Bob"

# SECURITY: Bind DETR P2P to private IP instead of 0.0.0.0
export DETR_P2P_IP="172.16.0.4"   # VM #2 private IP
export DETR_P2P_PORT="30334"
export DETR_P2P_BOOTSTRAP="172.16.0.5:30334"  # Connect to VM #1's DETR P2P

export BOOTNODE="$ALICE_BOOTNODE"

echo ""
echo "Starting validator and connecting to Alice..."
echo ""

# Run the generic one-command validator script
exec /opt/etrid/one-command-validator.sh
