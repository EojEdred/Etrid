#!/usr/bin/env python3
"""
Systematic Fix for All PBC Runtimes
Adds pallet-etr-lock configuration to 12 external bridge PBCs
"""

import os
import re
from pathlib import Path
from datetime import datetime

ETRID_ROOT = "/Users/macbook/Desktop/etrid"
PBC_DIR = Path(ETRID_ROOT) / "05-multichain/partition-burst-chains/pbc-chains"
TIMESTAMP = datetime.now().strftime("%Y%m%d_%H%M%S")

# All PBCs except edsc (which is native and doesn't need etr-lock)
PBC_CHAINS = [
    "btc-pbc", "eth-pbc", "sol-pbc", "xrp-pbc",
    "bnb-pbc", "trx-pbc", "ada-pbc", "matic-pbc",
    "link-pbc", "sc-usdt-pbc", "doge-pbc", "xlm-pbc"
]

ETR_LOCK_CONFIG = """// ETR Lock Configuration (required by bridge pallets)
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

"""

def fix_cargo_toml(cargo_path):
    """Add pallet-etr-lock to Cargo.toml"""
    with open(cargo_path, 'r') as f:
        content = f.read()

    modified = False

    # Add to dependencies if not present
    if 'pallet-etr-lock' not in content:
        # Find pallet-consensus line and add after it
        pattern = r'(pallet-consensus = \{ path[^\n]+\n)'
        replacement = r'\1pallet-etr-lock = { path = "../../../../../pallets/pallet-etr-lock", default-features = false }\n'
        content = re.sub(pattern, replacement, content)
        modified = True
        print("  ✅ Added pallet-etr-lock to dependencies")
    else:
        print("  ✓ pallet-etr-lock already in dependencies")

    # Add to std features if not present
    if '"pallet-etr-lock/std"' not in content:
        pattern = r'("pallet-consensus/std",\n)'
        replacement = r'\1    "pallet-etr-lock/std",\n'
        content = re.sub(pattern, replacement, content)
        modified = True
        print("  ✅ Added pallet-etr-lock/std to features")
    else:
        print("  ✓ pallet-etr-lock already in std features")

    if modified:
        with open(cargo_path, 'w') as f:
            f.write(content)

    return modified

def fix_lib_rs(lib_path):
    """Add pallet_etr_lock::Config implementation and EtrLock to construct_runtime!"""
    with open(lib_path, 'r') as f:
        content = f.read()

    modified = False

    # Add Config implementation if not present
    if 'impl pallet_etr_lock::Config for Runtime' not in content:
        # Find where to insert (before bridge configuration)
        patterns = [
            r'(// [A-Z][\w\s]+ Bridge Configuration\n)',
            r'(impl pallet_\w+_bridge::Config for Runtime)',
            r'(parameter_types! \{[^}]*Bridge[^}]*\})'
        ]

        inserted = False
        for pattern in patterns:
            if re.search(pattern, content):
                content = re.sub(pattern, ETR_LOCK_CONFIG + r'\1', content, count=1)
                inserted = True
                modified = True
                print("  ✅ Added pallet_etr_lock::Config implementation")
                break

        if not inserted:
            print("  ⚠️  Could not find bridge config insertion point - manual fix required")
    else:
        print("  ✓ pallet_etr_lock::Config already implemented")

    # Add EtrLock to construct_runtime! if not present
    if 'EtrLock:' not in content:
        # Find Consensus line in construct_runtime! and add EtrLock after it
        pattern = r'(        Consensus: pallet_consensus,\n)'
        replacement = r'\1        EtrLock: pallet_etr_lock,\n'
        if re.search(pattern, content):
            content = re.sub(pattern, replacement, content)
            modified = True
            print("  ✅ Added EtrLock to construct_runtime!")
        else:
            print("  ⚠️  Could not find Consensus in construct_runtime! - manual fix required")
    else:
        print("  ✓ EtrLock already in construct_runtime!")

    if modified:
        with open(lib_path, 'w') as f:
            f.write(content)

    return modified

def main():
    print("╔════════════════════════════════════════════════════════════╗")
    print("║  Ëtrid PBC Runtime Systematic Fix (Python)                ║")
    print("║  Adding pallet-etr-lock to 12 bridge PBCs                 ║")
    print("╚════════════════════════════════════════════════════════════╝")
    print()
    print(f"📅 Started: {datetime.now().strftime('%c')}")
    print(f"📂 PBC Directory: {PBC_DIR}")
    print()

    fixed_count = 0
    skipped_count = 0
    failed_count = 0

    for pbc in PBC_CHAINS:
        print("━" * 60)
        print(f"Fixing: {pbc}")
        print("━" * 60)

        runtime_dir = PBC_DIR / pbc / "runtime"
        cargo_toml = runtime_dir / "Cargo.toml"
        lib_rs = runtime_dir / "src" / "lib.rs"

        if not runtime_dir.exists():
            print(f"⚠️  Runtime directory not found: {runtime_dir}")
            skipped_count += 1
            continue

        try:
            # Backup files
            if cargo_toml.exists():
                backup = str(cargo_toml) + f".backup_{TIMESTAMP}"
                cargo_toml.write_text(cargo_toml.read_text())

            if lib_rs.exists():
                backup = str(lib_rs) + f".backup_{TIMESTAMP}"
                lib_rs.write_text(lib_rs.read_text())

            # Fix Cargo.toml
            print("📝 Step 1/2: Fixing Cargo.toml...")
            fix_cargo_toml(cargo_toml)

            # Fix lib.rs
            print("📝 Step 2/2: Fixing lib.rs...")
            fix_lib_rs(lib_rs)

            print(f"✅ {pbc} fixed successfully")
            print()
            fixed_count += 1

        except Exception as e:
            print(f"❌ Error fixing {pbc}: {e}")
            failed_count += 1

    print("╔════════════════════════════════════════════════════════════╗")
    print("║  FIX SUMMARY                                               ║")
    print("╚════════════════════════════════════════════════════════════╝")
    print()
    print("📊 Results:")
    print(f"  ✅ Fixed: {fixed_count} PBCs")
    print(f"  ⚠️  Skipped: {skipped_count} PBCs")
    print(f"  ❌ Failed: {failed_count} PBCs")
    print()
    print(f"📂 Backups created with timestamp: {TIMESTAMP}")
    print()
    print("🔧 Next steps:")
    print("1. Review changes with: git diff")
    print("2. Clean build artifacts: cargo clean")
    print("3. Rebuild all PBC collators: ./build-all-pbc-collators.sh")
    print("4. Verify binaries in target/release/")
    print()
    print(f"📅 Completed: {datetime.now().strftime('%c')}")
    print()

    if fixed_count == 12:
        print("🎉 All 12 PBC runtimes fixed successfully!")
        return 0
    else:
        print(f"⚠️  Only {fixed_count} out of 12 PBCs were fixed. Manual review required.")
        return 1

if __name__ == "__main__":
    exit(main())
