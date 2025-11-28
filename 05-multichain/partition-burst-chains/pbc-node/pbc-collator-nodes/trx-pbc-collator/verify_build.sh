#!/bin/bash
# Build verification script for TRX PBC Collator with DETR P2P integration

set -e

echo "============================================"
echo "TRX PBC Collator - Build Verification"
echo "============================================"
echo ""

# Navigate to project directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "📂 Working directory: $SCRIPT_DIR"
echo ""

# Check DETR P2P dependencies exist
echo "🔍 Checking DETR P2P dependencies..."
if [ ! -d "../../../../../01-detr-p2p/detrp2p" ]; then
    echo "❌ ERROR: detrp2p library not found at ../../../../../01-detr-p2p/detrp2p"
    exit 1
fi
echo "   ✅ detrp2p found"

if [ ! -d "../../../../../01-detr-p2p/aecomms" ]; then
    echo "❌ ERROR: etrid-aecomms library not found at ../../../../../01-detr-p2p/aecomms"
    exit 1
fi
echo "   ✅ etrid-aecomms found"

# Check runtime dependency exists
echo ""
echo "🔍 Checking TRX PBC runtime..."
if [ ! -d "../../../pbc-chains/trx-pbc/runtime" ]; then
    echo "❌ ERROR: trx-pbc-runtime not found at ../../../pbc-chains/trx-pbc/runtime"
    exit 1
fi
echo "   ✅ trx-pbc-runtime found"

# Check source files exist
echo ""
echo "🔍 Checking source files..."
required_files=(
    "src/main.rs"
    "src/cli.rs"
    "src/service.rs"
    "src/chain_spec.rs"
    "src/p2p.rs"
    "Cargo.toml"
    "build.rs"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ ERROR: Required file missing: $file"
        exit 1
    fi
    echo "   ✅ $file"
done

echo ""
echo "🔧 Running cargo check..."
if ! cargo check 2>&1 | tee /tmp/trx-pbc-build.log; then
    echo ""
    echo "❌ ERROR: cargo check failed"
    echo "   See /tmp/trx-pbc-build.log for details"
    exit 1
fi

echo ""
echo "============================================"
echo "✅ All checks passed!"
echo "============================================"
echo ""
echo "Next steps:"
echo "  1. Build release binary:"
echo "     cargo build --release"
echo ""
echo "  2. Run locally:"
echo "     ./target/release/trx-pbc-collator --dev"
echo ""
echo "  3. Run with P2P:"
echo "     ./target/release/trx-pbc-collator --p2p-listen 0.0.0.0:30333"
echo ""
echo "  4. Connect to peers:"
echo "     ./target/release/trx-pbc-collator \\"
echo "       --p2p-listen 0.0.0.0:30333 \\"
echo "       --p2p-bootstrap '1.2.3.4:30333,5.6.7.8:30333'"
echo ""
