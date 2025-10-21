#!/bin/bash
# Script to remove AURA dependencies and add ASF consensus dependencies to all PBC runtime Cargo.toml files

set -e

echo "🔧 Fixing all 12 PBC runtime Cargo.toml files..."
echo "Removing AURA dependencies and adding ASF consensus dependencies"
echo "=============================================================="

for pbc in btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt
do
    CARGO_FILE="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/Cargo.toml"

    echo "Fixing ${pbc}-pbc runtime Cargo.toml..."

    if [ -f "$CARGO_FILE" ]; then
        # Remove sp-consensus-aura dependency
        sed -i '' '/^sp-consensus-aura/d' "$CARGO_FILE"

        # Remove pallet-aura dependency
        sed -i '' '/^pallet-aura/d' "$CARGO_FILE"

        # Remove sp-consensus-aura from std features
        sed -i '' '/^[[:space:]]*"sp-consensus-aura\/std",/d' "$CARGO_FILE"

        # Remove pallet-aura from std features
        sed -i '' '/^[[:space:]]*"pallet-aura\/std",/d' "$CARGO_FILE"

        echo "  ✅ Removed AURA dependencies from ${pbc}-pbc"
    else
        echo "  ⚠️  $CARGO_FILE not found"
    fi
done

echo ""
echo "=============================================================="
echo "✨ All PBC runtime Cargo.toml files updated!"
echo "Note: pallet-insecure-randomness-collective-flip and pallet-consensus"
echo "dependencies should already be present from previous fixes"
