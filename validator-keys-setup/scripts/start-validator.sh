#!/bin/bash
set -e

# =============================================================================
# Ëtrid FlareChain Validator Start Script
# =============================================================================
# Simple script to start a validator node
# =============================================================================

# Configuration
BINARY="${FLARECHAIN_BINARY:-./flarechain-node}"
CHAIN="${CHAIN:-local}"
BASE_PATH="${BASE_PATH:-/var/lib/etrid}"
NODE_NAME="${NODE_NAME:-etrid-validator-$(hostname)}"

# Network ports
RPC_PORT="${RPC_PORT:-9944}"
WS_PORT="${WS_PORT:-9945}"
P2P_PORT="${P2P_PORT:-30333}"

# Parse arguments
EXTRA_ARGS=""
while [[ $# -gt 0 ]]; do
  case $1 in
    --bootnode)
      EXTRA_ARGS="$EXTRA_ARGS --bootnodes $2"
      shift 2
      ;;
    *)
      EXTRA_ARGS="$EXTRA_ARGS $1"
      shift
      ;;
  esac
done

echo "================================================================================"
echo "  Ëtrid FlareChain Validator"
echo "================================================================================"
echo "  Starting: $NODE_NAME"
echo "  Chain:    $CHAIN"
echo "  Base:     $BASE_PATH"
echo "================================================================================"
echo ""

# Check if bootstrap is needed
if [ ! -d "$BASE_PATH/keystore" ] || [ ! -f "$BASE_PATH/network/secret_ed25519" ]; then
  echo "⚠  Node not bootstrapped. Running bootstrap first..."
  echo ""

  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  if [ -f "$SCRIPT_DIR/bootstrap-validator.sh" ]; then
    bash "$SCRIPT_DIR/bootstrap-validator.sh" --base-path="$BASE_PATH" --chain="$CHAIN" --name="$NODE_NAME"
  else
    echo "❌ Error: bootstrap-validator.sh not found"
    echo "   Please run bootstrap-validator.sh first"
    exit 1
  fi

  echo ""
  echo "Bootstrap complete. Starting node in 3 seconds..."
  sleep 3
  echo ""
fi

# Start the node
exec $BINARY \
  --chain="$CHAIN" \
  --base-path="$BASE_PATH" \
  --name="$NODE_NAME" \
  --validator \
  --rpc-external \
  --ws-external \
  --prometheus-external \
  --rpc-port="$RPC_PORT" \
  --ws-port="$WS_PORT" \
  --port="$P2P_PORT" \
  --rpc-cors=all \
  $EXTRA_ARGS
