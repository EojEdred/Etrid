#!/usr/bin/env python3
"""
Fix remaining runtime issues in PBC runtimes after ASF migration.
Removes AURA API implementations and fixes syntax errors.
"""

import re
from pathlib import Path

# PBCs with known issues (excluding btc and eth which are already working)
PBCS_TO_FIX = [
    "doge-pbc",
    "xlm-pbc",
    "bnb-pbc",
    "trx-pbc",
    "ada-pbc",
    "link-pbc",
    "matic-pbc",
    "sc-usdt-pbc",
    "sol-pbc",
    "xrp-pbc",
]

BASE_PATH = Path("05-multichain/partition-burst-chains/pbc-chains")

def fix_runtime(pbc_name: str) -> bool:
    """Fix runtime issues for a given PBC"""
    runtime_path = BASE_PATH / pbc_name / "runtime" / "src" / "lib.rs"

    if not runtime_path.exists():
        print(f"  ❌ Runtime not found for {pbc_name}")
        return False

    content = runtime_path.read_text()
    original_content = content

    # 1. Remove AURA API implementation block
    aura_api_pattern = r'\s+impl sp_consensus_aura::AuraApi<Block, sp_consensus_aura::sr25519::AuthorityId> for Runtime \{[^}]+fn slot_duration\(\)[^}]+\}[^}]+fn authorities\(\)[^}]+\}\s+\}'
    content = re.sub(aura_api_pattern, '', content, flags=re.DOTALL)

    # 2. Check for extra closing braces (common from script artifacts)
    # Count opening and closing braces in impl_runtime_apis! block
    impl_start = content.find('impl_runtime_apis!')
    if impl_start != -1:
        # Find the impl_runtime_apis! block
        impl_block_pattern = r'impl_runtime_apis!\s*\{.*\}(?:\s*\n)*$'
        match = re.search(impl_block_pattern, content[impl_start:], re.DOTALL)
        if match:
            block = match.group(0)
            # Count braces
            open_count = block.count('{')
            close_count = block.count('}')

            if close_count > open_count:
                # Remove extra closing braces at the end
                extra_braces = close_count - open_count
                # Remove trailing } characters
                for _ in range(extra_braces):
                    # Find last standalone }
                    last_brace_pattern = r'\s*\}\s*$'
                    content = re.sub(last_brace_pattern, '', content, count=1)
                print(f"  🔧 Removed {extra_braces} extra closing brace(s)")

    # 3. Fix any Aura:: references to use Consensus::
    content = content.replace('Aura::slot_duration()', 'Consensus::slot_duration()')
    content = content.replace('pallet_aura::', 'pallet_consensus::')

    # 4. Remove any sp_consensus_aura imports in the runtime APIs section
    content = re.sub(r'use sp_consensus_aura;?\s*\n', '', content)

    if content != original_content:
        runtime_path.write_text(content)
        print(f"  ✅ Fixed {pbc_name} runtime")
        return True
    else:
        print(f"  ℹ️  No changes needed for {pbc_name}")
        return True

def main():
    print("🔧 PBC Runtime Issue Fixer")
    print(f"Fixing {len(PBCS_TO_FIX)} runtimes...\n")

    success_count = 0
    failed = []

    for pbc in PBCS_TO_FIX:
        print(f"📦 Processing {pbc}...")
        try:
            if fix_runtime(pbc):
                success_count += 1
            else:
                failed.append(pbc)
        except Exception as e:
            print(f"  ❌ Error: {e}")
            failed.append(pbc)

    print(f"\n{'='*60}")
    print(f"✅ Successfully processed: {success_count}/{len(PBCS_TO_FIX)}")

    if failed:
        print(f"❌ Failed: {', '.join(failed)}")
    else:
        print("🎉 All runtimes fixed!")

    print(f"{'='*60}")

if __name__ == "__main__":
    main()
