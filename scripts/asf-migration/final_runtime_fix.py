#!/usr/bin/env python3
"""
Final comprehensive fix - copy exact working structure from btc-pbc.
"""

import re
from pathlib import Path

BROKEN = ["doge-pbc", "xrp-pbc", "bnb-pbc", "trx-pbc", "ada-pbc", "link-pbc", "matic-pbc", "sc-usdt-pbc", "sol-pbc"]
BASE = Path("05-multichain/partition-burst-chains/pbc-chains")

ASF_API_BLOCK = """    impl sp_consensus_asf::AsfApi<Block, AccountId> for Runtime {
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

def fix_runtime(pbc):
    file_path = BASE / pbc / "runtime" / "src" / "lib.rs"
    if not file_path.exists():
        print(f"  ❌ Not found: {pbc}")
        return False

    content = file_path.read_text()

    # Remove any existing AsfApi implementation (might be malformed)
    content = re.sub(
        r'impl sp_consensus_asf::AsfApi<Block, AccountId> for Runtime \{.*?\n    \}(\n    \})?',
        '',
        content,
        flags=re.DOTALL
    )

    # Find where to insert - after GrandpaApi implementation
    # Look for the closing of GrandpaApi impl
    pattern = r'(impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime \{.*?\n    \})'

    match = re.search(pattern, content, re.DOTALL)
    if match:
        # Insert ASF API right after GrandpaApi
        insert_pos = match.end()
        content = content[:insert_pos] + "\n" + ASF_API_BLOCK + content[insert_pos:]
        file_path.write_text(content)
        print(f"  ✅ Fixed {pbc}")
        return True
    else:
        print(f"  ⚠️  Could not find GrandpaApi in {pbc}")
        return False

print("🔧 Final Runtime Fix - Copying working structure...")
fixed = 0
for pbc in BROKEN:
    print(f"📦 {pbc}...")
    if fix_runtime(pbc):
        fixed += 1

print(f"\n✅ Fixed {fixed}/{len(BROKEN)} runtimes")
