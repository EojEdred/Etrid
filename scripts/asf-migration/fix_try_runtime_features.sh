#!/bin/bash
# Script to remove AURA references from try-runtime features in all PBC runtime Cargo.toml files

set -e

echo "🔧 Removing AURA from try-runtime features in all PBC Cargo.toml files..."
echo "=================================================================="

for pbc in btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt
do
    CARGO_FILE="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/Cargo.toml"

    echo "Fixing ${pbc}-pbc runtime Cargo.toml try-runtime features..."

    if [ -f "$CARGO_FILE" ]; then
        # Remove pallet-aura/try-runtime from try-runtime features
        sed -i '' '/^[[:space:]]*"pallet-aura\/try-runtime",/d' "$CARGO_FILE"

        echo "  ✅ Removed AURA try-runtime references from ${pbc}-pbc"
    else
        echo "  ⚠️  $CARGO_FILE not found"
    fi
done

echo ""
echo "=================================================================="
echo "✨ All PBC runtime Cargo.toml try-runtime features updated!"
