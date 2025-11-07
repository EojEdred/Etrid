#!/usr/bin/env bash
# Ëtrid FlareChain - Build ALL PBC Collators
# Comprehensive build script for all 13 Partition Burst Chain collators

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
BUILD_LOG_DIR="${ETRID_ROOT}/docs/mainnet/build-logs"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Create log directory
mkdir -p "$BUILD_LOG_DIR"

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid FlareChain - Building ALL PBC Collators            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📅 Started: $(date)"
echo "📂 Root: $ETRID_ROOT"
echo "📝 Logs: $BUILD_LOG_DIR"
echo ""

cd "$ETRID_ROOT"

# Array of all PBC collators
PBC_COLLATORS=(
    "edsc-pbc-collator"      # Priority 1: ËDSC Stablecoin
    "btc-pbc-collator"       # Priority 2: Bitcoin Bridge
    "eth-pbc-collator"       # Priority 3: Ethereum Bridge
    "sol-pbc-collator"       # Solana Bridge
    "xrp-pbc-collator"       # Ripple Bridge
    "bnb-pbc-collator"       # BNB Chain Bridge
    "trx-pbc-collator"       # Tron Bridge
    "ada-pbc-collator"       # Cardano Bridge
    "matic-pbc-collator"     # Polygon Bridge
    "link-pbc-collator"      # Chainlink Bridge
    "sc-usdt-pbc-collator"   # USDT Stablecoin
    "doge-pbc-collator"      # Dogecoin Bridge
    "xlm-pbc-collator"       # Stellar Bridge
)

# Track build results
SUCCESSFUL_BUILDS=()
FAILED_BUILDS=()
ALREADY_BUILT=()

# Build each collator
for collator in "${PBC_COLLATORS[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Building: $collator"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Check if binary already exists
    BINARY_PATH="target/release/$collator"
    if [[ -f "$BINARY_PATH" ]]; then
        SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
        echo "✓ $collator already built!"
        echo "  Location: $BINARY_PATH"
        echo "  Size: $SIZE"
        echo ""
        ALREADY_BUILT+=("$collator")
        continue
    fi

    # Build the collator
    echo "Building $collator..."
    echo "Command: cargo build --release -p $collator"
    echo ""

    LOG_FILE="${BUILD_LOG_DIR}/build_${collator}_${TIMESTAMP}.log"

    if cargo build --release -p "$collator" 2>&1 | tee "$LOG_FILE"; then
        if [[ -f "$BINARY_PATH" ]]; then
            SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
            echo ""
            echo "✅ $collator built successfully!"
            echo "  Location: $BINARY_PATH"
            echo "  Size: $SIZE"
            echo "  Log: $LOG_FILE"
            echo ""
            SUCCESSFUL_BUILDS+=("$collator")
        else
            echo ""
            echo "⚠️  Build succeeded but binary not found at $BINARY_PATH"
            echo "  Log: $LOG_FILE"
            echo ""
            FAILED_BUILDS+=("$collator (binary not found)")
        fi
    else
        echo ""
        echo "❌ $collator build failed!"
        echo "  Check log: $LOG_FILE"
        echo ""
        FAILED_BUILDS+=("$collator")
    fi

    echo ""
done

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  BUILD SUMMARY                                             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📊 Build Statistics:"
echo "  Total collators: ${#PBC_COLLATORS[@]}"
echo "  Already built: ${#ALREADY_BUILT[@]}"
echo "  Successfully built: ${#SUCCESSFUL_BUILDS[@]}"
echo "  Failed: ${#FAILED_BUILDS[@]}"
echo ""

if [[ ${#ALREADY_BUILT[@]} -gt 0 ]]; then
    echo "✓ Already Built (${#ALREADY_BUILT[@]}):"
    for collator in "${ALREADY_BUILT[@]}"; do
        echo "  • $collator"
    done
    echo ""
fi

if [[ ${#SUCCESSFUL_BUILDS[@]} -gt 0 ]]; then
    echo "✅ Successfully Built (${#SUCCESSFUL_BUILDS[@]}):"
    for collator in "${SUCCESSFUL_BUILDS[@]}"; do
        echo "  • $collator"
    done
    echo ""
fi

if [[ ${#FAILED_BUILDS[@]} -gt 0 ]]; then
    echo "❌ Failed Builds (${#FAILED_BUILDS[@]}):"
    for collator in "${FAILED_BUILDS[@]}"; do
        echo "  • $collator"
    done
    echo ""
    echo "⚠️  Check build logs in: $BUILD_LOG_DIR"
    echo ""
fi

echo "📂 All PBC Collator Binaries:"
echo ""
for collator in "${PBC_COLLATORS[@]}"; do
    BINARY_PATH="target/release/$collator"
    if [[ -f "$BINARY_PATH" ]]; then
        SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
        echo "  ✓ $collator: $SIZE"
    else
        echo "  ✗ $collator: NOT FOUND"
    fi
done

echo ""
echo "📅 Completed: $(date)"
echo ""

# Exit with error if any builds failed
if [[ ${#FAILED_BUILDS[@]} -gt 0 ]]; then
    echo "⚠️  WARNING: Some builds failed. Review logs before deploying."
    exit 1
else
    echo "🎉 All PBC collators built successfully!"
    echo ""
    echo "Next steps:"
    echo "1. Review binaries in target/release/"
    echo "2. Generate chainspecs for each PBC"
    echo "3. Deploy to validators 6-21"
    echo "4. Generate and insert session keys"
    echo "5. Register PBCs on FlareChain"
    echo ""
    echo "See: PBC_DEPLOYMENT_GUIDE.md for full instructions"
    exit 0
fi
