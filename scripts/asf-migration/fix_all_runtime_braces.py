#!/usr/bin/env python3
"""
Fix all runtime brace issues by copying the working structure from btc-pbc.
"""

import re
from pathlib import Path

BROKEN_RUNTIMES = [
    "doge-pbc",
    "xrp-pbc",
    "bnb-pbc",
    "trx-pbc",
    "ada-pbc",
    "link-pbc",
    "matic-pbc",
    "sc-usdt-pbc",
    "sol-pbc",
]

BASE_PATH = Path("05-multichain/partition-burst-chains/pbc-chains")

def fix_runtime_braces(pbc_name):
    """Fix runtime by looking for the pattern of extra braces after AsfApi"""
    runtime_path = BASE_PATH / pbc_name / "runtime" / "src" / "lib.rs"

    if not runtime_path.exists():
        print(f"  ❌ Runtime not found: {runtime_path}")
        return False

    content = runtime_path.read_text()
    original = content

    # Find the AsfApi implementation block
    # Pattern: The impl block ending, then an extra }, then possibly AccountNonceApi
    # We want to remove the extra } that comes right after the AsfApi impl closes

    # Method 1: Look for double }} pattern after active_validators
    pattern1 = r'(fn active_validators\(\) -> Vec<AccountId> \{\s+Consensus::active_validators\(\)\s+\}\s+\})\s+\}'
    if re.search(pattern1, content):
        content = re.sub(pattern1, r'\1', content)
        print(f"  🔧 Removed extra brace (pattern 1)")

    # Method 2: Look for } followed by } with optional whitespace before impl or closing
    pattern2 = r'(\s+impl sp_consensus_asf::AsfApi<Block, AccountId> for Runtime \{.*?active_validators\(\)\s+\}\s+\})\s+\}(\s+(?:impl|$))'
    if re.search(pattern2, content, re.DOTALL):
        content = re.sub(pattern2, r'\1\2', content, flags=re.DOTALL)
        print(f"  🔧 Removed extra brace (pattern 2)")

    # Method 3: Simpler - just find }\n    } after the last function of AsfApi
    pattern3 = r'(Consensus::active_validators\(\)\s+\}\s+\})\s+\}(\s+impl frame_system_rpc_runtime_api::AccountNonceApi)'
    if re.search(pattern3, content, re.DOTALL):
        content = re.sub(pattern3, r'\1\2', content, flags=re.DOTALL)
        print(f"  🔧 Removed extra brace (pattern 3)")

    # Also check if tests module is missing closing brace
    if '#[cfg(test)]' in content and 'mod tests {' in content:
        # Count braces from #[cfg(test)] to end
        test_start = content.find('#[cfg(test)]')
        if test_start != -1:
            test_section = content[test_start:]
            open_count = test_section.count('{')
            close_count = test_section.count('}')

            if open_count > close_count:
                # Add missing closing brace
                content += '\n}\n'
                print(f"  🔧 Added missing test module closing brace")

    if content != original:
        runtime_path.write_text(content)
        return True
    else:
        print(f"  ℹ️  No automatic fix found - needs manual inspection")
        return False

def main():
    print("🔧 Fixing All Runtime Brace Issues...")
    print(f"Fixing {len(BROKEN_RUNTIMES)} runtimes...\n")

    fixed = 0
    needs_manual = []

    for pbc in BROKEN_RUNTIMES:
        print(f"📦 Fixing {pbc}...")
        if fix_runtime_braces(pbc):
            print(f"  ✅ Fixed {pbc}")
            fixed += 1
        else:
            needs_manual.append(pbc)

    print(f"\n{'='*60}")
    print(f"✅ Auto-fixed: {fixed}/{len(BROKEN_RUNTIMES)}")
    if needs_manual:
        print(f"⚠️  Needs manual fix: {', '.join(needs_manual)}")
    print(f"{'='*60}")

if __name__ == "__main__":
    main()
