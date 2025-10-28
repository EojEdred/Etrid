#!/bin/bash
set -e

# =============================================================================
# Add Bootnode to Chain Spec
# =============================================================================
# Updates a chain spec JSON file with a new bootnode address
# =============================================================================

if [ $# -lt 2 ]; then
  echo "Usage: $0 <chain-spec.json> <bootnode-multiaddr>"
  echo ""
  echo "Example:"
  echo "  $0 flarechain-local.json /ip4/20.186.91.207/tcp/30333/p2p/12D3KooW..."
  exit 1
fi

CHAIN_SPEC="$1"
BOOTNODE="$2"

if [ ! -f "$CHAIN_SPEC" ]; then
  echo "Error: Chain spec file not found: $CHAIN_SPEC"
  exit 1
fi

# Backup original
cp "$CHAIN_SPEC" "${CHAIN_SPEC}.backup"

# Add bootnode using jq
if command -v jq &> /dev/null; then
  # Use jq for proper JSON manipulation
  jq --arg bootnode "$BOOTNODE" '.bootNodes += [$bootnode] | .bootNodes |= unique' "$CHAIN_SPEC" > "${CHAIN_SPEC}.tmp"
  mv "${CHAIN_SPEC}.tmp" "$CHAIN_SPEC"
  echo "✓ Added bootnode to $CHAIN_SPEC"
  echo "  Bootnode: $BOOTNODE"
  echo "  Backup: ${CHAIN_SPEC}.backup"
else
  echo "Error: jq is required but not installed"
  echo "Install: brew install jq (macOS) or apt install jq (Linux)"
  exit 1
fi
