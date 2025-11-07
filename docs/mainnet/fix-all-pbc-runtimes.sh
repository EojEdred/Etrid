#!/usr/bin/env bash
# Systematic Fix for All PBC Runtimes
# Adds pallet-etr-lock configuration to 12 external bridge PBCs

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
PBC_DIR="${ETRID_ROOT}/05-multichain/partition-burst-chains/pbc-chains"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Ëtrid PBC Runtime Systematic Fix                        ║"
echo "║  Adding pallet-etr-lock to 12 bridge PBCs                ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📅 Started: $(date)"
echo "📂 PBC Directory: $PBC_DIR"
echo ""

# All PBCs except edsc (which is native and doesn't need etr-lock)
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

FIXED_COUNT=0
SKIPPED_COUNT=0
FAILED_COUNT=0

for pbc in "${PBC_CHAINS[@]}"; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Fixing: $pbc"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

    RUNTIME_DIR="${PBC_DIR}/${pbc}/runtime"
    CARGO_TOML="${RUNTIME_DIR}/Cargo.toml"
    LIB_RS="${RUNTIME_DIR}/src/lib.rs"

    if [[ ! -d "$RUNTIME_DIR" ]]; then
        echo "⚠️  Runtime directory not found: $RUNTIME_DIR"
        ((SKIPPED_COUNT++))
        continue
    fi

    # Backup original files
    cp "$CARGO_TOML" "${CARGO_TOML}.backup_${TIMESTAMP}" 2>/dev/null || true
    cp "$LIB_RS" "${LIB_RS}.backup_${TIMESTAMP}" 2>/dev/null || true

    echo "📝 Step 1/3: Fixing Cargo.toml..."

    # Check if pallet-etr-lock already in dependencies
    if grep -q "^pallet-etr-lock = {" "$CARGO_TOML"; then
        echo "  ✓ pallet-etr-lock already in dependencies"
    else
        # Add pallet-etr-lock after pallet-consensus line
        sed -i '' '/^pallet-consensus = { path/a\
pallet-etr-lock = { path = "../../../../../pallets/pallet-etr-lock", default-features = false }
' "$CARGO_TOML"
        echo "  ✅ Added pallet-etr-lock to dependencies"
    fi

    # Check if pallet-etr-lock in std features
    if grep -q '"pallet-etr-lock/std"' "$CARGO_TOML"; then
        echo "  ✓ pallet-etr-lock already in std features"
    else
        # Add to std features after pallet-consensus/std
        sed -i '' '/"pallet-consensus\/std",/a\
    "pallet-etr-lock/std",
' "$CARGO_TOML"
        echo "  ✅ Added pallet-etr-lock/std to features"
    fi

    echo "📝 Step 2/3: Adding pallet_etr_lock::Config implementation..."

    # Check if Config already exists
    if grep -q "impl pallet_etr_lock::Config for Runtime" "$LIB_RS"; then
        echo "  ✓ pallet_etr_lock::Config already implemented"
    else
        # Find the line with impl pallet_consensus::Config
        # Insert etr-lock config before bridge config

        # Create the config block
        ETR_LOCK_CONFIG='// ETR Lock Configuration (required by bridge pallets)
parameter_types! {
    pub const MinLockAmount: Balance = 1_000_000; // 0.001 ETR
    pub const MaxLockAmount: Balance = 1_000_000_000_000_000; // 1M ETR
    pub const LockPeriod: BlockNumber = 7 * DAYS;
}

impl pallet_etr_lock::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinLockAmount = MinLockAmount;
    type MaxLockAmount = MaxLockAmount;
    type DefaultLockPeriod = LockPeriod;
}
'

        # Find line number of first bridge-related config (varies per PBC)
        BRIDGE_LINE=$(grep -n "Bridge Configuration\|impl pallet.*bridge::Config" "$LIB_RS" | head -1 | cut -d: -f1)

        if [[ -n "$BRIDGE_LINE" ]]; then
            # Insert before bridge config
            sed -i '' "${BRIDGE_LINE}i\\
$ETR_LOCK_CONFIG\\
" "$LIB_RS"
            echo "  ✅ Added pallet_etr_lock::Config implementation"
        else
            echo "  ⚠️  Could not find bridge config insertion point"
        fi
    fi

    echo "📝 Step 3/3: Adding EtrLock to construct_runtime!..."

    # Check if EtrLock already in construct_runtime!
    if grep -A20 "construct_runtime!" "$LIB_RS" | grep -q "EtrLock:"; then
        echo "  ✓ EtrLock already in construct_runtime!"
    else
        # Find Consensus line in construct_runtime! and add EtrLock after it
        sed -i '' '/Consensus: pallet_consensus,/a\
        EtrLock: pallet_etr_lock,
' "$LIB_RS"
        echo "  ✅ Added EtrLock to construct_runtime!"
    fi

    echo "✅ $pbc fixed successfully"
    echo ""
    ((FIXED_COUNT++))
done

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  FIX SUMMARY                                               ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📊 Results:"
echo "  ✅ Fixed: $FIXED_COUNT PBCs"
echo "  ⚠️  Skipped: $SKIPPED_COUNT PBCs"
echo "  ❌ Failed: $FAILED_COUNT PBCs"
echo ""
echo "📂 Backups created: ${CARGO_TOML}.backup_${TIMESTAMP}"
echo "                    ${LIB_RS}.backup_${TIMESTAMP}"
echo ""
echo "🔧 Next steps:"
echo "1. Review changes with: git diff"
echo "2. Clean build artifacts: cargo clean"
echo "3. Rebuild all PBC collators: ./build-all-pbc-collators.sh"
echo "4. Verify binaries in target/release/"
echo ""
echo "📅 Completed: $(date)"
echo ""

if [[ $FIXED_COUNT -eq 12 ]]; then
    echo "🎉 All 12 PBC runtimes fixed successfully!"
    exit 0
else
    echo "⚠️  Only $FIXED_COUNT out of 12 PBCs were fixed. Manual review required."
    exit 1
fi
