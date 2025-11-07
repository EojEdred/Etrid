#!/bin/bash
# Zombienet Setup Script for Local XCM Testing
# Downloads and installs required components for testing XCM integration

set -e

echo "🧟 Setting up Zombienet Test Environment"
echo ""

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Darwin*)
        ZOMBIENET_BIN="zombienet-macos"
        POLKADOT_BIN="polkadot"
        ;;
    Linux*)
        ZOMBIENET_BIN="zombienet-linux-x64"
        POLKADOT_BIN="polkadot"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

# Create bin directory
mkdir -p bin
cd bin

echo "📥 Step 1: Installing Zombienet..."
if [ ! -f "$ZOMBIENET_BIN" ]; then
    ZOMBIENET_VERSION="v1.3.105"
    ZOMBIENET_URL="https://github.com/paritytech/zombienet/releases/download/${ZOMBIENET_VERSION}/${ZOMBIENET_BIN}"

    echo "Downloading Zombienet ${ZOMBIENET_VERSION} for ${OS}..."
    curl -L -o "$ZOMBIENET_BIN" "$ZOMBIENET_URL"
    chmod +x "$ZOMBIENET_BIN"
    ln -sf "$ZOMBIENET_BIN" zombienet
    echo "✅ Zombienet installed: bin/$ZOMBIENET_BIN"
else
    echo "✅ Zombienet already installed"
fi

echo ""
echo "📥 Step 2: Installing Polkadot relay chain..."
if [ ! -f "$POLKADOT_BIN" ]; then
    POLKADOT_VERSION="v1.7.0"

    echo "Downloading Polkadot ${POLKADOT_VERSION}..."
    if [ "$OS" = "Darwin" ]; then
        POLKADOT_URL="https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-${POLKADOT_VERSION}/polkadot"
    else
        POLKADOT_URL="https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-${POLKADOT_VERSION}/polkadot"
    fi

    curl -L -o "$POLKADOT_BIN" "$POLKADOT_URL"
    chmod +x "$POLKADOT_BIN"
    echo "✅ Polkadot installed: bin/$POLKADOT_BIN"
else
    echo "✅ Polkadot already installed"
fi

cd ..

echo ""
echo "📦 Step 3: Checking node binaries..."

# Check FlareChain node
if [ -f "target/release/flarechain-node" ]; then
    echo "✅ flarechain-node found"
    ln -sf "../target/release/flarechain-node" bin/flarechain-node
else
    echo "⚠️  flarechain-node not found - will need to build"
    echo "    Run: cargo build --release -p flarechain-node"
fi

# Check ETH-PBC node
if [ -f "target/release/eth-pbc-node" ]; then
    echo "✅ eth-pbc-node found"
    ln -sf "../target/release/eth-pbc-node" bin/eth-pbc-node
elif [ -f "target/release/eth-pbc-collator" ]; then
    echo "✅ eth-pbc-collator found (will use as eth-pbc-node)"
    ln -sf "../target/release/eth-pbc-collator" bin/eth-pbc-node
else
    echo "⚠️  eth-pbc-node not found - will need to build"
    echo "    Run: cd 05-multichain/partition-burst-chains/pbc-chains/eth-pbc && cargo build --release"
fi

echo ""
echo "🎉 Zombienet setup complete!"
echo ""
echo "📋 Next Steps:"
echo "1. Ensure node binaries are built:"
echo "   - flarechain-node"
echo "   - eth-pbc-node"
echo ""
echo "2. Run Zombienet:"
echo "   ./bin/zombienet spawn zombienet-xcm-test.toml"
echo ""
echo "3. Connect to nodes:"
echo "   - Relay chain (Alice): ws://localhost:9944"
echo "   - FlareChain: ws://localhost:9946"
echo "   - ETH-PBC: ws://localhost:9948"
echo ""
echo "4. Monitor logs and test XCM messages"
