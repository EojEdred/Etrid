#!/bin/bash
# Quick start script for LINK-PBC Collator in development mode

set -e

echo "==================================="
echo "LINK-PBC Collator - Development Mode"
echo "==================================="
echo ""

# Check if binary exists
if [ ! -f "target/release/link-pbc-collator" ] && [ ! -f "target/debug/link-pbc-collator" ]; then
    echo "Binary not found. Building..."
    cargo build --release
fi

# Determine which binary to use
BINARY=""
if [ -f "target/release/link-pbc-collator" ]; then
    BINARY="target/release/link-pbc-collator"
    echo "Using release binary"
elif [ -f "target/debug/link-pbc-collator" ]; then
    BINARY="target/debug/link-pbc-collator"
    echo "Using debug binary"
fi

echo ""
echo "Starting LINK-PBC Collator..."
echo ""
echo "Configuration:"
echo "  Listen Address: ${DETR_P2P_LISTEN_ADDR:-0.0.0.0:30333 (default)}"
echo "  Node Identity: ${DETR_P2P_NODE_IDENTITY:-Auto-generated}"
echo "  Announce IP: ${DETR_P2P_ANNOUNCE_IP:-Auto-detected via STUN}"
echo "  Bootstrap Peers: ${DETR_P2P_BOOTSTRAP_PEERS:-None (standalone mode)}"
echo ""
echo "Press Ctrl+C to stop"
echo "==================================="
echo ""

# Set log level if not already set
export RUST_LOG="${RUST_LOG:-info,link_pbc_collator=debug,detrp2p=info}"

# Run the collator
exec $BINARY
