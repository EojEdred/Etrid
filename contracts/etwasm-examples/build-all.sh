#!/bin/bash

# Build all smart contract examples
# Usage: ./build-all.sh [--release]

set -e  # Exit on error

EXAMPLES_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_MODE="debug"
BUILD_FLAGS=""

# Parse arguments
if [ "$1" == "--release" ]; then
    BUILD_MODE="release"
    BUILD_FLAGS="--release"
fi

echo "🔨 Building all Ëtrid smart contract examples ($BUILD_MODE mode)..."
echo ""

BUILT_CONTRACTS=0
FAILED_CONTRACTS=()

# Array of contract directories
CONTRACTS=(
    "01-hello-world"
    "02-counter"
    "03-erc20-token"
    "04-simple-dao"
    "05-escrow"
)

for contract in "${CONTRACTS[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "📦 Building: $contract"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    cd "$EXAMPLES_DIR/$contract"

    if cargo contract build $BUILD_FLAGS; then
        echo "✅ $contract: Build successful"
        BUILT_CONTRACTS=$((BUILT_CONTRACTS + 1))

        # Show artifact location
        if [ "$BUILD_MODE" == "release" ]; then
            WASM_FILE="target/ink/${contract//-/_}.wasm"
            if [ -f "$WASM_FILE" ]; then
                SIZE=$(du -h "$WASM_FILE" | cut -f1)
                echo "📄 WASM artifact: $SIZE"
            fi
        fi
    else
        echo "❌ $contract: Build failed"
        FAILED_CONTRACTS+=("$contract")
    fi

    echo ""
    cd "$EXAMPLES_DIR"
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Build Summary"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Total contracts: ${#CONTRACTS[@]}"
echo "Successfully built: $BUILT_CONTRACTS"

if [ ${#FAILED_CONTRACTS[@]} -eq 0 ]; then
    echo ""
    echo "🎉 SUCCESS! All contracts built successfully!"
    echo ""

    if [ "$BUILD_MODE" == "release" ]; then
        echo "📦 WASM artifacts are in: */target/ink/*.wasm"
        echo ""
    fi

    exit 0
else
    echo ""
    echo "❌ FAILED contracts:"
    for failed in "${FAILED_CONTRACTS[@]}"; do
        echo "  - $failed"
    done
    echo ""
    exit 1
fi
