#!/bin/bash
# Script to apply pbc-common to all remaining PBCs

set -e  # Exit on error

PBC_DIR="/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains"

# List of PBCs to update (excluding btc-pbc which is already done)
PBCS=("eth-pbc" "doge-pbc" "sol-pbc" "xlm-pbc" "xrp-pbc" "bnb-pbc" "trx-pbc" "ada-pbc" "link-pbc" "matic-pbc" "sc-usdt-pbc" "edsc-pbc")

echo "========================================="
echo "Applying pbc-common to all PBCs"
echo "========================================="

for pbc in "${PBCS[@]}"; do
    echo ""
    echo "Processing: $pbc"
    echo "-----------------------------------------"

    CARGO_TOML="$PBC_DIR/$pbc/runtime/Cargo.toml"

    if [ ! -f "$CARGO_TOML" ]; then
        echo "⚠️  WARNING: $CARGO_TOML not found, skipping..."
        continue
    fi

    # Check if pbc-common already added
    if grep -q "pbc-common" "$CARGO_TOML"; then
        echo "✓ pbc-common already added to $pbc"
    else
        echo "Adding pbc-common to $pbc Cargo.toml..."

        # Backup
        cp "$CARGO_TOML" "$CARGO_TOML.backup"

        # Add pbc-common dependency after [dependencies]
        sed -i.tmp '/^\[dependencies\]/a\
# PBC Common - Shared runtime code\
pbc-common = { path = "../../../pbc-common", default-features = false }\
' "$CARGO_TOML" && rm "$CARGO_TOML.tmp"

        # Add to std features (after "std = [")
        sed -i.tmp '/^std = \[/a\
    "pbc-common/std",
' "$CARGO_TOML" && rm "$CARGO_TOML.tmp"

        echo "✓ Updated Cargo.toml"
    fi
done

echo ""
echo "========================================="
echo "✅ Cargo.toml updates complete!"
echo "========================================="
echo ""
echo "Next: Update lib.rs files manually for each PBC"
echo "Pattern: Replace imports with 'pub use pbc_common::*;'"
