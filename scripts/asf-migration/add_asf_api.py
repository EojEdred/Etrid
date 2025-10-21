#!/usr/bin/env python3
"""
Add ASF Runtime API to all PBC runtimes
"""

import os
import re

ASF_API_IMPL = """
    impl sp_consensus_asf::AsfApi<Block, AccountId> for Runtime {
        fn committee() -> Vec<AccountId> {
            Consensus::committee()
        }

        fn ppfa_index() -> u32 {
            Consensus::ppfa_index()
        }

        fn slot_duration() -> sp_consensus_asf::SlotDuration {
            sp_consensus_asf::SlotDuration::from_millis(Consensus::slot_duration())
        }

        fn should_propose(validator: AccountId) -> bool {
            Consensus::should_propose(validator)
        }

        fn current_epoch() -> u32 {
            Consensus::current_epoch()
        }

        fn active_validators() -> Vec<AccountId> {
            Consensus::active_validators()
        }
    }
"""

PBCS = ["eth", "doge", "sol", "xlm", "xrp", "bnb", "trx", "ada", "link", "matic", "sc-usdt"]
BASE_PATH = "/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains"

def add_asf_api_to_runtime(pbc_name):
    lib_rs = f"{BASE_PATH}/{pbc_name}-pbc/runtime/src/lib.rs"

    if not os.path.exists(lib_rs):
        print(f"  ⚠️  {lib_rs} not found - SKIPPING")
        return False

    with open(lib_rs, 'r') as f:
        content = f.read()

    # Check if already added
    if "impl sp_consensus_asf::AsfApi" in content:
        print(f"  ✓ ASF API already present in {pbc_name}-pbc")
        return True

    # Find the insertion point - after GrandpaApi closing brace, before AccountNonceApi
    # Look for the pattern: closing brace of GrandpaApi block
    pattern = r'(impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime \{[\s\S]*?\n    \})\n'

    match = re.search(pattern, content)
    if not match:
        print(f"  ⚠️  Could not find GrandpaApi block in {pbc_name}-pbc")
        return False

    # Insert ASF API after GrandpaApi
    new_content = content[:match.end()] + ASF_API_IMPL + content[match.end():]

    # Write back
    with open(lib_rs, 'w') as f:
        f.write(new_content)

    print(f"  ✓ Added ASF API to {pbc_name}-pbc")
    return True

def main():
    print("=" * 50)
    print("Adding ASF Runtime API to all PBC runtimes")
    print("=" * 50)
    print()

    success_count = 0
    for pbc in PBCS:
        print(f"Processing {pbc}-pbc...")
        if add_asf_api_to_runtime(pbc):
            success_count += 1
        print()

    print("=" * 50)
    print(f"Complete! Updated {success_count}/{len(PBCS)} runtimes")
    print("=" * 50)

if __name__ == "__main__":
    main()
