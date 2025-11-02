#!/bin/bash
# Script for VMs to download and install the binary

set -e

echo "Downloading flarechain-node binary..."

# Option 1: Download from GitHub artifact (requires gh CLI and auth)
if command -v gh &> /dev/null; then
    echo "Using GitHub CLI to download from Actions..."
    gh run download 19018648993 -n flarechain-node-linux-x86_64 -D /tmp/ -R EojEdred/Etrid
    BINARY="/tmp/flarechain-node"
else
    # Option 2: Pull from git (binary is in repo)
    echo "Pulling latest from git..."
    cd ~/etrid
    git pull origin main
    BINARY="~/etrid/release-packages/linux-x86_64/flarechain-node"
fi

echo "Installing binary..."
mkdir -p ~/etrid/target/release
cp $BINARY ~/etrid/target/release/
chmod +x ~/etrid/target/release/flarechain-node

echo "✓ Binary installed successfully!"
ls -lh ~/etrid/target/release/flarechain-node
file ~/etrid/target/release/flarechain-node
