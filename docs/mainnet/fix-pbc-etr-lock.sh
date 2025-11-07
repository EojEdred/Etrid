#!/usr/bin/env bash
# Fix all PBC runtimes to include pallet-etr-lock configuration

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
PBC_CHAINS_DIR="${ETRID_ROOT}/05-multichain/partition-burst-chains/pbc-chains"

# List of PBC chains that need etr-lock (all except edsc which is native)
PBC_CHAINS=(
    "btc-pbc"
    "eth-pbc"
    "sol-pbc"
    "xrp-pbc"
    "bnb-pbc"
    "trx-pbc"
    "ada-pbc"
    "matic-pbc"
    "link-pbc"
    "sc-usdt-pbc"
    "doge-pbc"
    "xlm-pbc"
)

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Fixing PBC Runtimes - Adding pallet-etr-lock             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

for pbc in "${PBC_CHAINS[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Fixing: $pbc"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    RUNTIME_DIR="${PBC_CHAINS_DIR}/${pbc}/runtime"
    RUNTIME_LIB="${RUNTIME_DIR}/src/lib.rs"
    CARGO_TOML="${RUNTIME_DIR}/Cargo.toml"

    if [[ ! -f "$RUNTIME_LIB" ]]; then
        echo "⚠️  Runtime not found: $RUNTIME_LIB"
        continue
    fi

    # Check if already has pallet-etr-lock config
    if grep -q "impl pallet_etr_lock::Config for Runtime" "$RUNTIME_LIB"; then
        echo "✓ $pbc already has pallet-etr-lock config"
    else
        echo "📝 Adding pallet-etr-lock Config to $pbc..."

        # Find the line with bridge configuration and add etr-lock before it
        # This will vary per PBC, so we'll use a more generic approach

        # Check if Cargo.toml has pallet-etr-lock dependency
        if ! grep -q "pallet-etr-lock" "$CARGO_TOML"; then
            echo "  → Adding pallet-etr-lock to Cargo.toml dependencies"

            # Add to dependencies section
            sed -i.bak '/^pallet-consensus = { path/a\
pallet-etr-lock = { path = "../../../../../pallets/pallet-etr-lock", default-features = false }
' "$CARGO_TOML"

            # Add to std features
            sed -i.bak2 '/"pallet-consensus\/std",/a\
    "pallet-etr-lock/std",
' "$CARGO_TOML"

            rm -f "${CARGO_TOML}.bak" "${CARGO_TOML}.bak2"
        fi

        echo "  ✅ Cargo.toml updated"
    fi

    # Check if EtrLock is in construct_runtime!
    if grep -A20 "construct_runtime!" "$RUNTIME_LIB" | grep -q "EtrLock:"; then
        echo "✓ $pbc already has EtrLock in construct_runtime!"
    else
        echo "  ⚠️  Need manual addition of EtrLock to construct_runtime! macro"
    fi

    echo ""
done

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Fix Complete                                              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Next step: Manually add impl pallet_etr_lock::Config and EtrLock"
echo "to construct_runtime! for each PBC that needs it."
echo ""
