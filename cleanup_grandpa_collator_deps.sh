#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# Remove residual GRANDPA dependencies from PBC collator Cargo.toml files
# Services are already using ASF - this is just dependency cleanup
# ═══════════════════════════════════════════════════════════════════════════════

set -e

COLLATORS=(
    "ada-pbc-collator"
    "bnb-pbc-collator"
    "btc-pbc-collator"
    "doge-pbc-collator"
    "edsc-pbc-collator"
    "eth-pbc-collator"
    "link-pbc-collator"
    "matic-pbc-collator"
    "sc-usdt-pbc-collator"
    "sol-pbc-collator"
    "trx-pbc-collator"
    "xlm-pbc-collator"
    "xrp-pbc-collator"
)

BASE_PATH="/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes"

echo "═══════════════════════════════════════════════════════════════════════════════"
echo "Cleaning up GRANDPA dependencies from 13 PBC collator Cargo.toml files"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

for collator in "${COLLATORS[@]}"; do
    CARGO_TOML="${BASE_PATH}/${collator}/Cargo.toml"

    if [ -f "$CARGO_TOML" ]; then
        echo "Processing $collator..."

        # Create backup
        cp "$CARGO_TOML" "$CARGO_TOML.bak"

        # Remove sc-consensus-grandpa dependency line
        sed -i '' '/^sc-consensus-grandpa.*=.*{.*polkadot-sdk/d' "$CARGO_TOML"

        # Remove sp-consensus-grandpa dependency line
        sed -i '' '/^sp-consensus-grandpa.*=.*{.*polkadot-sdk/d' "$CARGO_TOML"

        echo "  ✅ Removed GRANDPA dependencies from $collator"
    else
        echo "  ⚠️  Not found: $CARGO_TOML"
    fi
done

echo ""
echo "═══════════════════════════════════════════════════════════════════════════════"
echo "✅ Cleanup complete!"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""
echo "Verification:"
echo "  cargo check --workspace"
