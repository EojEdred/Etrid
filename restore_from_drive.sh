#!/bin/bash
# Restore script - if you need to work without external drive
# This removes symlinks and restores local directories

set -e

echo "Restoring local build directories..."

# Remove symlinks
rm -f "$HOME/Desktop/etrid-binaries"
rm -f "$HOME/.cargo/registry"
rm -f "$HOME/.cargo/git"
rm -f "$HOME/Desktop/etrid/target"

# Restore directories
mkdir -p "$HOME/Desktop/etrid-binaries"
mkdir -p "$HOME/.cargo/registry"
mkdir -p "$HOME/.cargo/git"
mkdir -p "$HOME/Desktop/etrid/target"

# Reset npm cache
npm config delete cache

echo "✓ Restored to local directories"
echo "Note: You'll need to rebuild (cargo build)"
