#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# Remove GRANDPA from all PBC runtimes and replace with ASF consensus
# ═══════════════════════════════════════════════════════════════════════════════

set -e

PBCS=(
    "doge-pbc"
    "sol-pbc"
    "xlm-pbc"
    "xrp-pbc"
    "bnb-pbc"
    "trx-pbc"
    "ada-pbc"
    "link-pbc"
    "matic-pbc"
    "sc-usdt-pbc"
    "edsc-pbc"
)

echo "═══════════════════════════════════════════════════════════════════════════════"
echo "Removing GRANDPA from 11 PBC Runtimes (eth-pbc and btc-pbc already done)"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""

for pbc in "${PBCS[@]}"; do
    echo "Processing $pbc..."

    RUNTIME_DIR="/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains/$pbc/runtime"

    if [ ! -d "$RUNTIME_DIR" ]; then
        echo "  ⚠️  Runtime directory not found, skipping..."
        continue
    fi

    CARGO_TOML="$RUNTIME_DIR/Cargo.toml"
    LIB_RS="$RUNTIME_DIR/src/lib.rs"

    if [ ! -f "$CARGO_TOML" ]; then
        echo "  ⚠️  Cargo.toml not found, skipping..."
        continue
    fi

    # ═══════════════════════════════════════════════════════════════════════════════
    # 1. Update Cargo.toml - Remove GRANDPA dependencies
    # ═══════════════════════════════════════════════════════════════════════════════
    echo "  📝 Updating Cargo.toml..."

    # Remove sp-consensus-grandpa dependency
    sed -i.bak '/^sp-consensus-grandpa = /d' "$CARGO_TOML"

    # Remove pallet-grandpa dependency
    sed -i.bak '/^pallet-grandpa = /d' "$CARGO_TOML"

    # Remove from std features
    sed -i.bak '/[[:space:]]*"sp-consensus-grandpa\/std",/d' "$CARGO_TOML"
    sed -i.bak '/[[:space:]]*"pallet-grandpa\/std",/d' "$CARGO_TOML"

    echo "  ✅ Cargo.toml updated"

    # ═══════════════════════════════════════════════════════════════════════════════
    # 2. Update lib.rs - Remove GRANDPA code
    # ═══════════════════════════════════════════════════════════════════════════════

    if [ -f "$LIB_RS" ]; then
        echo "  📝 Updating lib.rs..."

        # Create a backup
        cp "$LIB_RS" "$LIB_RS.bak"

        # Use perl for multi-line replacements

        # 1. Remove grandpa from SessionKeys
        perl -i -0pe 's/impl_opaque_keys!\s*\{\s*pub struct SessionKeys\s*\{\s*pub grandpa:\s*Grandpa,\s*\}\s*\}/impl_opaque_keys! {\n        pub struct SessionKeys {\n            \/\/ ASF manages consensus internally - no session keys needed\n        }\n    }/gs' "$LIB_RS"

        # 2. Remove GRANDPA Config impl
        perl -i -0pe 's/impl pallet_grandpa::Config for Runtime \{[^\}]*\}//gs' "$LIB_RS"

        # 3. Remove Grandpa from construct_runtime
        perl -i -0pe 's/\s*Grandpa:\s*pallet_grandpa,\s*//gs' "$LIB_RS"

        # 4. Remove GRANDPA Runtime API impl
        perl -i -0pe 's/impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime \{[^\}]*\}[^\}]*\}//gs' "$LIB_RS"

        echo "  ✅ lib.rs updated"
    else
        echo "  ⚠️  lib.rs not found, skipping..."
    fi

    # Clean up backup files
    rm -f "$CARGO_TOML.bak" "$LIB_RS.bak"

    echo "  ✅ $pbc complete!"
    echo ""
done

echo "═══════════════════════════════════════════════════════════════════════════════"
echo "✅ GRANDPA removal complete for all PBC runtimes!"
echo "═══════════════════════════════════════════════════════════════════════════════"
echo ""
echo "Next steps:"
echo "1. Update PBC collator services to use ASF instead of GRANDPA"
echo "2. Test builds with: cargo check --workspace"
echo "3. Run tests: cargo test --workspace"
