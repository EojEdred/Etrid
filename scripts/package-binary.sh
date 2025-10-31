#!/bin/bash
# Ëtrid FlareChain Node Binary Packaging Script
# Prepares the node binary for deployment and distribution

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
BINARY_PATH="$ETRID_ROOT/target/release/flarechain-node"
VERSION="0.1.0"
PACKAGE_NAME="flarechain-node-v${VERSION}-$(uname -s)-$(uname -m)"
PACKAGE_DIR="$ETRID_ROOT/release-packages"

cd "$ETRID_ROOT"

echo "=== Ëtrid FlareChain Node Binary Packaging ==="
echo ""

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    echo "Please build the binary first with: cargo build --release -p flarechain-node"
    exit 1
fi

# Get binary size and hash
BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
BINARY_SHA256=$(shasum -a 256 "$BINARY_PATH" | cut -d' ' -f1)

echo "Binary Information:"
echo "  Path: $BINARY_PATH"
echo "  Size: $BINARY_SIZE"
echo "  SHA256: $BINARY_SHA256"
echo ""

# Create package directory
mkdir -p "$PACKAGE_DIR/$PACKAGE_NAME"

echo "Step 1: Copying binary..."
cp -v "$BINARY_PATH" "$PACKAGE_DIR/$PACKAGE_NAME/"

echo ""
echo "Step 2: Copying essential files..."
cp -v README.md "$PACKAGE_DIR/$PACKAGE_NAME/"
cp -v LICENSE "$PACKAGE_DIR/$PACKAGE_NAME/"
cp -v SECURITY.md "$PACKAGE_DIR/$PACKAGE_NAME/"
cp -v CHANGELOG.md "$PACKAGE_DIR/$PACKAGE_NAME/"

echo ""
echo "Step 2a: Copying FlareSwap deployment integration..."
mkdir -p "$PACKAGE_DIR/$PACKAGE_NAME/flareswap"
cp -v contracts/flareswap/scripts/deploy-etwasm.js "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/"
cp -v contracts/flareswap/ETWASM_DEPLOYMENT_GUIDE.md "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/"
cp -v contracts/flareswap/ETWASM_INTEGRATION_COMPLETE.md "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/"
cp -v contracts/flareswap/package.json "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/"
cp -v contracts/flareswap/package-lock.json "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/"

echo "Copying FlareSwap compiled artifacts..."
mkdir -p "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/artifacts"
cp -r contracts/flareswap/artifacts/src "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/artifacts/"
cp -r contracts/flareswap/artifacts/build-info "$PACKAGE_DIR/$PACKAGE_NAME/flareswap/artifacts/"

echo "FlareSwap deployment files included ✓"

echo ""
echo "Step 3: Creating installation script..."
cat > "$PACKAGE_DIR/$PACKAGE_NAME/install.sh" << 'INSTALL_SCRIPT'
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
INSTALL_SCRIPT

chmod +x "$PACKAGE_DIR/$PACKAGE_NAME/install.sh"

echo ""
echo "Step 4: Creating README for package..."
cat > "$PACKAGE_DIR/$PACKAGE_NAME/PACKAGE_README.md" << EOF
# FlareChain Node Binary Package

Version: ${VERSION}
Platform: $(uname -s) $(uname -m)
Build Date: $(date)

## Binary Information

- **File**: flarechain-node
- **Size**: ${BINARY_SIZE}
- **SHA256**: ${BINARY_SHA256}

## Quick Start

### Installation

Run the installation script:

\`\`\`bash
./install.sh
\`\`\`

Or manually copy the binary:

\`\`\`bash
sudo cp flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node
\`\`\`

### Running the Node

Start a validator node:

\`\`\`bash
flarechain-node \\
    --chain=chainspec.json \\
    --validator \\
    --alice \\
    --base-path=/tmp/validator \\
    --port=30333 \\
    --rpc-port=9944
\`\`\`

For more options:

\`\`\`bash
flarechain-node --help
\`\`\`

## System Requirements

- **OS**: Linux, macOS, or Windows (WSL)
- **RAM**: 4GB minimum, 8GB recommended
- **Storage**: 50GB minimum
- **Network**: Stable internet connection

## FlareSwap DEX Integration

This package includes FlareSwap DEX deployment integration for ËtwasmVM:

\`\`\`bash
cd flareswap
npm install
node deploy-etwasm.js --network=local
\`\`\`

See \`flareswap/ETWASM_DEPLOYMENT_GUIDE.md\` for complete instructions.

## Features Included

- ✅ FlareChain validator node binary
- ✅ ASF consensus with PPFA proposer
- ✅ ËtwasmVM (EVM-compatible smart contracts)
- ✅ FlareSwap DEX deployment tools
- ✅ Cross-chain bridge support (15 chains)
- ✅ PBC router for multichain swaps

## Documentation

For complete documentation, visit: https://docs.etrid.org

## Support

- GitHub: https://github.com/etrid/etrid
- Issues: https://github.com/etrid/etrid/issues
- Discord: https://discord.gg/etrid

## License

See LICENSE file for details.
EOF

echo ""
echo "Step 5: Creating checksum file..."
(cd "$PACKAGE_DIR/$PACKAGE_NAME" && shasum -a 256 flarechain-node > SHA256SUMS)

echo ""
echo "Step 6: Creating tarball..."
cd "$PACKAGE_DIR"
tar -czf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME"
TARBALL_SIZE=$(du -h "${PACKAGE_NAME}.tar.gz" | cut -f1)
TARBALL_SHA256=$(shasum -a 256 "${PACKAGE_NAME}.tar.gz" | cut -d' ' -f1)

echo ""
echo "=== Packaging Complete ==="
echo ""
echo "Package Details:"
echo "  Name: ${PACKAGE_NAME}.tar.gz"
echo "  Size: ${TARBALL_SIZE}"
echo "  SHA256: ${TARBALL_SHA256}"
echo "  Location: $PACKAGE_DIR/${PACKAGE_NAME}.tar.gz"
echo ""
echo "Contents:"
ls -lh "$PACKAGE_NAME"
echo ""
echo "To extract:"
echo "  tar -xzf ${PACKAGE_NAME}.tar.gz"
echo ""
