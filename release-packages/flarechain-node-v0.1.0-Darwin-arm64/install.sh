#!/bin/bash
# FlareChain Node Installation Script

set -e

echo "=== FlareChain Node Installation ==="
echo ""

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

echo "System: $OS $ARCH"
echo ""

# Default install location
INSTALL_DIR="/usr/local/bin"

# Check if user has permission to install
if [ ! -w "$INSTALL_DIR" ]; then
    echo "Installation requires sudo privileges."
    SUDO="sudo"
else
    SUDO=""
fi

# Copy binary
echo "Installing flarechain-node to $INSTALL_DIR..."
$SUDO cp flarechain-node "$INSTALL_DIR/"
$SUDO chmod +x "$INSTALL_DIR/flarechain-node"

echo ""
echo "✓ Installation complete!"
echo ""
echo "Run 'flarechain-node --version' to verify installation."
echo "Run 'flarechain-node --help' for usage information."
echo ""
