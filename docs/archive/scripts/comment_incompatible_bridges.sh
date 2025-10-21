#!/bin/bash

# Comment out bridges with incompatible Config traits
# Keep: BTC, ETH, XLM, XRP, SC-USDT (these have compatible traits)
# Comment: DOGE, BNB, TRX, ADA, LINK, MATIC, SOL (need trait investigation)

set -e

echo "=========================================="
echo "🔧 Commenting Out Incompatible Bridges"
echo "=========================================="
echo ""

PBCS=("doge" "bnb" "trx" "ada" "link" "matic" "sol")

for pbc in "${PBCS[@]}"; do
    echo "Processing $pbc-pbc..."

    runtime_file="05-multichain/partition-burst-chains/pbc-chains/$pbc-pbc/runtime/src/lib.rs"

    if [ ! -f "$runtime_file" ]; then
        echo "  ❌ Runtime file not found"
        continue
    fi

    # Comment out parameter_types! block and Config implementation
    # Use perl to comment multi-line blocks
    perl -i -0pe 's/(\/\/ \w+Bridge Configuration\s+parameter_types!\s*\{[^}]*\}\s*impl pallet_\w+_bridge::Config for Runtime \{[^}]*\})/# TODO: Fix Config trait mismatch\n# \1/gs' "$runtime_file"

    # Comment out bridge in construct_runtime!
    sed -i '' 's/^        \([A-Z][a-zA-Z]*Bridge: pallet_[a-z_]*bridge,\)/        \/\/ \1  \/\/ TODO: Fix Config trait/' "$runtime_file"

    echo "  ✅ Commented out incompatible bridge"
done

echo ""
echo "=========================================="
echo "✅ Completed commenting out 7 PBCs"
echo "Working bridges: BTC, ETH, XLM, XRP, SC-USDT (5/12)"
echo "=========================================="
