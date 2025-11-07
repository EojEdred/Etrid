#!/usr/bin/env python3
"""
Complete Fix for pallet-etr-lock Configuration
Adds BridgeOrigin and LockIdentifier to all 11 PBC runtimes
"""

import os
import re
from pathlib import Path
from datetime import datetime

ETRID_ROOT = "/Users/macbook/Desktop/etrid"
PBC_DIR = Path(ETRID_ROOT) / "05-multichain/partition-burst-chains/pbc-chains"

# All PBCs that need fixing (excluding edsc)
PBC_CHAINS = [
    "btc-pbc", "sol-pbc", "xrp-pbc", "bnb-pbc",
    "trx-pbc", "ada-pbc", "matic-pbc", "link-pbc",
    "sc-usdt-pbc", "doge-pbc", "xlm-pbc"
]

def fix_lib_rs(lib_path):
    """Add BridgeOrigin and LockIdentifier to pallet_etr_lock::Config"""
    with open(lib_path, 'r') as f:
        content = f.read()

    # Pattern to find the incomplete Config implementation
    pattern = r'(impl pallet_etr_lock::Config for Runtime \{[^}]*type DefaultLockPeriod = LockPeriod;)\n(\})'

    # Replacement with complete Config
    complete_config = r'''\1
    type BridgeOrigin = frame_system::EnsureRoot<AccountId>;
    type LockIdentifier = EtrLockId;
\2'''

    if re.search(pattern, content):
        content = re.sub(pattern, complete_config, content)
        print("  ✅ Added BridgeOrigin and LockIdentifier to Config")
    else:
        print("  ⚠️  Could not find Config pattern - may already be fixed")
        return False

    # Add LockIdentifier constant definition before the Config impl
    lock_id_pattern = r'(// ETR Lock Configuration)'
    lock_id_def = r'''// Lock identifier for ETR locking
const ETR_LOCK_ID: [u8; 8] = *b"etr/lock";

\1'''

    if 'const ETR_LOCK_ID' not in content:
        content = re.sub(lock_id_pattern, lock_id_def, content)
        print("  ✅ Added ETR_LOCK_ID constant")

    # Add parameter_types for LockIdentifier
    lock_param_pattern = r'(pub const LockPeriod: BlockNumber = 7 \* DAYS;\n\})'
    lock_param_addition = r'''\1

parameter_types! {
    pub const EtrLockId: [u8; 8] = ETR_LOCK_ID;
}'''

    if 'pub const EtrLockId' not in content:
        content = re.sub(lock_param_pattern, lock_param_addition, content)
        print("  ✅ Added EtrLockId parameter_types")

    with open(lib_path, 'w') as f:
        f.write(content)

    return True

def main():
    print("╔════════════════════════════════════════════════════════════╗")
    print("║  Complete pallet-etr-lock Fix                             ║")
    print("║  Adding BridgeOrigin & LockIdentifier                     ║")
    print("╚════════════════════════════════════════════════════════════╝")
    print()

    fixed_count = 0
    skipped_count = 0

    for pbc in PBC_CHAINS:
        print(f"━━━ Fixing: {pbc} ━━━")

        lib_rs = PBC_DIR / pbc / "runtime" / "src" / "lib.rs"

        if not lib_rs.exists():
            print(f"  ⚠️  lib.rs not found")
            skipped_count += 1
            continue

        try:
            if fix_lib_rs(lib_rs):
                print(f"✅ {pbc} fixed")
                fixed_count += 1
            else:
                skipped_count += 1
        except Exception as e:
            print(f"❌ Error: {e}")
            skipped_count += 1
        print()

    print("╔════════════════════════════════════════════════════════════╗")
    print("║  FIX COMPLETE                                              ║")
    print("╚════════════════════════════════════════════════════════════╝")
    print(f"✅ Fixed: {fixed_count} PBCs")
    print(f"⚠️  Skipped: {skipped_count} PBCs")
    print()
    print("Next: Resume cargo build")

if __name__ == "__main__":
    main()
