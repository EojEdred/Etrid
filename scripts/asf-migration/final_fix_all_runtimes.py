#!/usr/bin/env python3
"""
Final fix for all 9 runtimes - restore and properly add AsfApi
"""

import re
from pathlib import Path

PBCS = [
    "doge", "xrp", "bnb", "trx", "ada", "link", "matic", "sc-usdt", "sol"
]

BASE_PATH = Path("05-multichain/partition-burst-chains/pbc-chains")

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

def fix_runtime(pbc):
    runtime_path = BASE_PATH / f"{pbc}-pbc" / "runtime" / "src" / "lib.rs"
    backup_path = BASE_PATH / f"{pbc}-pbc" / "runtime" / "src" / "lib.rs.syntax_backup"

    if not backup_path.exists():
        print(f"  ❌ No syntax_backup for {pbc}")
        return False

    # Restore from syntax_backup
    content = backup_path.read_text()
    print(f"  📦 Restored from syntax_backup ({len(content)} bytes)")

    # Find the GrandpaApi implementation closing brace
    # Pattern: find "impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {"
    # then find its closing "    }"
    lines = content.split('\n')
    grandpa_start = None
    grandpa_end = None
    brace_count = 0
    in_grandpa = False

    for i, line in enumerate(lines):
        if 'impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime' in line:
            grandpa_start = i
            in_grandpa = True
            brace_count = 0
        elif in_grandpa:
            brace_count += line.count('{')
            brace_count -= line.count('}')
            if brace_count == 0 and '}' in line:
                grandpa_end = i
                break

    if grandpa_start is None or grandpa_end is None:
        print(f"  ❌ Could not find GrandpaApi block in {pbc}")
        return False

    print(f"  🔍 Found GrandpaApi at lines {grandpa_start}-{grandpa_end}")

    # Insert AsfApi after GrandpaApi
    lines.insert(grandpa_end + 1, ASF_API_IMPL)
    content = '\n'.join(lines)

    # Write the fixed content
    runtime_path.write_text(content)
    print(f"  ✅ Added AsfApi to {pbc}-pbc")
    return True

def main():
    print("🔧 Final Fix for All 9 Runtimes")
    print("="*50)

    fixed = 0
    for pbc in PBCS:
        print(f"\n📦 Fixing {pbc}-pbc...")
        if fix_runtime(pbc):
            fixed += 1

    print(f"\n{'='*50}")
    print(f"✅ Fixed {fixed}/{len(PBCS)} runtimes")
    print(f"{'='*50}")

if __name__ == "__main__":
    main()
