#!/usr/bin/env python3
"""
Fix remaining 6 PBC runtimes with correct bridge configuration
Based on successful btc-pbc fixes
"""

import os
import re

# PBCs to fix
PBCS_TO_FIX = [
    ('trx', 'tron'),
    ('ada', 'cardano'),
    ('link', 'chainlink'),
    ('sc-usdt', 'stablecoin-usdt'),
    ('doge', 'dogecoin'),
    ('xlm', 'stellar'),
]

BASE_PATH = "/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains"

def fix_pbc_runtime(pbc_name, bridge_name):
    """Fix a single PBC runtime configuration"""
    runtime_path = f"{BASE_PATH}/{pbc_name}-pbc/runtime/src/lib.rs"
    cargo_path = f"{BASE_PATH}/{pbc_name}-pbc/runtime/Cargo.toml"

    if not os.path.exists(runtime_path):
        print(f"❌ {pbc_name}-pbc runtime not found at {runtime_path}")
        return False

    print(f"\n{'='*60}")
    print(f"Fixing {pbc_name}-pbc runtime...")
    print(f"{'='*60}")

    # Read the runtime file
    with open(runtime_path, 'r') as f:
        content = f.read()

    # Check if already has TreasuryStub
    if 'pub struct TreasuryStub' in content:
        print(f"✅ {pbc_name}-pbc already has TreasuryStub")
        return True

    # Fix 1: Remove incorrect parameter_types and fix etr-lock Config
    # Find the etr-lock Config section
    etr_lock_config_pattern = r'impl pallet_etr_lock::Config for Runtime \{[^}]+\}'
    match = re.search(etr_lock_config_pattern, content, re.DOTALL)

    if match:
        old_config = match.group(0)
        # Ensure it has the 5 required types only
        new_config = f"""impl pallet_etr_lock::Config for Runtime {{
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxLockAmount = MaxLockAmount;
    type BridgeOrigin = frame_system::EnsureRoot<AccountId>;
    type LockIdentifier = EtrLockId;
}}"""
        content = content.replace(old_config, new_config)
        print(f"✅ Fixed pallet_etr_lock::Config")

    # Fix 2: Add TreasuryStub implementation before bridge Config
    # Find where bridge Config starts
    bridge_config_pattern = rf'impl pallet_{bridge_name}_bridge::Config for Runtime'
    bridge_match = re.search(bridge_config_pattern, content)

    if bridge_match:
        # Insert TreasuryStub before bridge Config
        treasury_stub = f"""
// Treasury stub for bridge fee collection
pub struct TreasuryStub;
impl etrid_bridge_common::treasury::TreasuryInterface<AccountId, Balance> for TreasuryStub {{
    fn receive_cross_chain_fees(_amount: Balance) -> frame_support::dispatch::DispatchResult {{
        Ok(()) // Stub implementation - fees are burned for now
    }}
}}

"""
        # Insert before the bridge Config
        insert_pos = bridge_match.start()
        content = content[:insert_pos] + treasury_stub + content[insert_pos:]
        print(f"✅ Added TreasuryStub implementation")

    # Fix 3: Update bridge Config to remove Currency and add Treasury + ValidatorPoolAccount
    bridge_full_config_pattern = rf'impl pallet_{bridge_name}_bridge::Config for Runtime \{{[^}}]+\}}'
    bridge_full_match = re.search(bridge_full_config_pattern, content, re.DOTALL)

    if bridge_full_match:
        old_bridge_config = bridge_full_match.group(0)

        # Extract the existing associated types
        lines = old_bridge_config.split('\n')
        new_lines = [f'impl pallet_{bridge_name}_bridge::Config for Runtime {{']

        for line in lines[1:-1]:  # Skip first and last lines
            stripped = line.strip()
            # Skip Currency (inherited from etr-lock)
            if stripped.startswith('type Currency'):
                continue
            # Skip any MinLockAmount or DefaultLockPeriod that might be there
            if stripped.startswith('type MinLockAmount') or stripped.startswith('type DefaultLockPeriod'):
                continue
            # Keep other types
            if stripped and not stripped.startswith('//'):
                new_lines.append(line)

        # Add Treasury and ValidatorPoolAccount
        new_lines.append('    type Treasury = TreasuryStub;')
        new_lines.append('    type ValidatorPoolAccount = BridgeAuthorityAccount;')
        new_lines.append('}')

        new_bridge_config = '\n'.join(new_lines)
        content = content.replace(old_bridge_config, new_bridge_config)
        print(f"✅ Fixed pallet_{bridge_name}_bridge::Config")

    # Write the fixed content back
    with open(runtime_path, 'w') as f:
        f.write(content)

    print(f"✅ {pbc_name}-pbc runtime fixed")

    # Fix 4: Add etrid-bridge-common dependency to Cargo.toml if not present
    if os.path.exists(cargo_path):
        with open(cargo_path, 'r') as f:
            cargo_content = f.read()

        if 'etrid-bridge-common' not in cargo_content:
            # Find the bridge dependency line to insert after
            bridge_dep_pattern = rf'pallet_{bridge_name}_bridge = {{[^}}]+}}'
            bridge_dep_match = re.search(bridge_dep_pattern, cargo_content)

            if bridge_dep_match:
                # Insert etrid-bridge-common dependency after bridge dependency
                insert_pos = bridge_dep_match.end()
                new_dep = f'\netrid-bridge-common = {{ path = "../../../../bridge-protocols/common", default-features = false }}'
                cargo_content = cargo_content[:insert_pos] + new_dep + cargo_content[insert_pos:]

                # Add to std features
                if '"etrid-bridge-common/std"' not in cargo_content:
                    # Find the std feature section
                    std_pattern = r'std = \[([\s\S]*?)\]'
                    std_match = re.search(std_pattern, cargo_content)

                    if std_match:
                        # Add before the closing bracket
                        std_section = std_match.group(0)
                        if std_section.rstrip().endswith(']'):
                            # Add before the closing bracket
                            new_std = std_section[:-1].rstrip()
                            if not new_std.endswith(','):
                                new_std += ','
                            new_std += '\n    "etrid-bridge-common/std",\n]'
                            cargo_content = cargo_content.replace(std_section, new_std)

                with open(cargo_path, 'w') as f:
                    f.write(cargo_content)
                print(f"✅ Added etrid-bridge-common dependency to Cargo.toml")
            else:
                print(f"⚠️ Could not find bridge dependency pattern in Cargo.toml")
        else:
            print(f"✅ etrid-bridge-common dependency already present")

    return True

def main():
    print("Starting fix for remaining 6 PBCs...")
    print(f"Base path: {BASE_PATH}")

    success_count = 0
    fail_count = 0

    for pbc_name, bridge_name in PBCS_TO_FIX:
        try:
            if fix_pbc_runtime(pbc_name, bridge_name):
                success_count += 1
            else:
                fail_count += 1
        except Exception as e:
            print(f"❌ Error fixing {pbc_name}-pbc: {e}")
            fail_count += 1

    print(f"\n{'='*60}")
    print(f"Fix Summary:")
    print(f"  ✅ Success: {success_count}/6")
    print(f"  ❌ Failed: {fail_count}/6")
    print(f"{'='*60}")

    if success_count > 0:
        print("\nNext steps:")
        print("1. Clean cargo cache: cargo clean")
        print("2. Build all PBCs in parallel:")
        for pbc_name, _ in PBCS_TO_FIX:
            print(f"   cargo build --release -p {pbc_name}-pbc-collator &")

if __name__ == "__main__":
    main()
