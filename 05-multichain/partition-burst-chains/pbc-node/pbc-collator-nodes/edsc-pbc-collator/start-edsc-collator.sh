#!/bin/bash
# EDSC-PBC Collator Startup Script with DETR P2P Configuration
#
# This script demonstrates how to start the EDSC collator with full P2P configuration.
# Customize the variables below for your deployment environment.

set -e

# ============================================================================
# CONFIGURATION
# ============================================================================

# Your server's public IP address (REQUIRED for production)
# If not set, the collator will attempt STUN auto-detection
PUBLIC_IP="${PUBLIC_IP:-}"

# P2P bind address (address to listen on)
BIND_ADDRESS="${BIND_ADDRESS:-0.0.0.0:30333}"

# Bootstrap peers (comma-separated list)
# Format: peer_id@ip:port,peer_id@ip:port
# Example: abc123...@192.168.1.100:30333,def456...@192.168.1.101:30333
BOOTSTRAP_PEERS="${BOOTSTRAP_PEERS:-}"

# PBC ID for EDSC (default: 12)
PBC_ID="${PBC_ID:-12}"

# Relay chain RPC endpoint
RELAY_CHAIN_RPC="${RELAY_CHAIN_RPC:-ws://127.0.0.1:9944}"

# Node name (for telemetry)
NODE_NAME="${NODE_NAME:-edsc-collator-1}"

# Data directory
BASE_PATH="${BASE_PATH:-./data}"

# Enable P2P maintenance tasks
ENABLE_MAINTENANCE="${ENABLE_MAINTENANCE:-true}"

# Validator key (optional - for deterministic node ID)
VALIDATOR_PUBLIC_KEY="${VALIDATOR_PUBLIC_KEY:-}"

# ============================================================================
# ENVIRONMENT SETUP
# ============================================================================

# Export P2P configuration to environment
if [ -n "$PUBLIC_IP" ]; then
    export DETR_P2P_ANNOUNCE_IP="$PUBLIC_IP"
    echo "📢 Using announce IP: $PUBLIC_IP"
else
    echo "⚠️ No PUBLIC_IP set - will attempt auto-detection via STUN"
    echo "⚠️ For production, set PUBLIC_IP environment variable"
fi

if [ -n "$BOOTSTRAP_PEERS" ]; then
    export DETR_P2P_BOOTSTRAP_PEERS="$BOOTSTRAP_PEERS"
    echo "👥 Bootstrap peers: $BOOTSTRAP_PEERS"
else
    echo "⚠️ No bootstrap peers configured - node will run standalone"
fi

if [ -n "$VALIDATOR_PUBLIC_KEY" ]; then
    export VALIDATOR_PUBLIC_KEY="$VALIDATOR_PUBLIC_KEY"
    echo "🔑 Using validator public key for deterministic node ID"
fi

# ============================================================================
# BUILD CHECK
# ============================================================================

BINARY="./target/release/edsc-pbc-collator"

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found: $BINARY"
    echo "Building collator..."
    cargo build --release
fi

# ============================================================================
# START COLLATOR
# ============================================================================

echo ""
echo "🚀 Starting EDSC-PBC Collator..."
echo "   Node Name: $NODE_NAME"
echo "   PBC ID: $PBC_ID"
echo "   Bind Address: $BIND_ADDRESS"
echo "   Base Path: $BASE_PATH"
echo "   Relay Chain RPC: $RELAY_CHAIN_RPC"
echo "   Maintenance Tasks: $ENABLE_MAINTENANCE"
echo ""

exec "$BINARY" \
    --name "$NODE_NAME" \
    --base-path "$BASE_PATH" \
    --chain local \
    --pbc-id "$PBC_ID" \
    --relay-chain-rpc "$RELAY_CHAIN_RPC" \
    --p2p-bind-address "$BIND_ADDRESS" \
    --p2p-enable-maintenance "$ENABLE_MAINTENANCE" \
    --validator \
    --rpc-port 9933 \
    --rpc-external \
    --rpc-cors all \
    --ws-port 9944 \
    --ws-external \
    --prometheus-external \
    "$@"
