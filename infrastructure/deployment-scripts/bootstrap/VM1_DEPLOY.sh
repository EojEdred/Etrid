#!/bin/bash
# =============================================================================
# VM #1 - Bootstrap Node (Uses Generic Script)
# IP: 20.186.91.207 (public), 172.16.0.5 (private)
# =============================================================================

set -e

echo "================================================================================"
echo "  Ëtrid VM #1 - Bootstrap Node Deployment"
echo "================================================================================"
echo ""
echo "This script sets up Alice as the bootstrap validator using the"
echo "generic one-command-validator.sh script."
echo ""
read -p "Press Enter to continue..."

# Export VM-specific configuration
export FLARECHAIN_BINARY="/opt/etrid/flarechain-node"
export CHAIN_SPEC="/opt/etrid/chainspec.json"
export BASE_PATH="/var/lib/etrid"
export NODE_NAME="etrid-validator-01"
export VALIDATOR_KEY="//Alice"

# SECURITY: Bind DETR P2P to private IP instead of 0.0.0.0
export DETR_P2P_IP="172.16.0.5"   # VM #1 private IP
export DETR_P2P_PORT="30334"

# BOOTNODE is left empty = bootstrap mode

echo ""
echo "Starting bootstrap node with Alice's keys..."
echo ""

# Run the generic one-command validator script
exec /opt/etrid/one-command-validator.sh
