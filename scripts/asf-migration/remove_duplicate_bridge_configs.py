#!/usr/bin/env python3
"""
Remove duplicate bridge Config implementations.
Keep the new one (with ConstU32/ConstU64), remove the old commented one.
"""

import re
from pathlib import Path

PBCS = ["btc", "eth", "doge", "xlm", "xrp", "bnb", "trx", "ada", "link", "matic", "sc-usdt", "sol"]
BASE_DIR = Path("05-multichain/partition-burst-chains/pbc-chains")

def remove_duplicates(pbc_name):
    """Remove duplicate bridge Config implementations"""

    runtime_path = BASE_DIR / f"{pbc_name}-pbc" / "runtime" / "src" / "lib.rs"

    if not runtime_path.exists():
        return False

    content = runtime_path.read_text()

    # Remove old commented Config blocks
    # Pattern: // impl pallet_*_bridge::Config for Runtime { ... // }
    content = re.sub(
        r'// impl pallet_\w+_bridge::Config for Runtime \{[^}]*// \}',
        '',
        content,
        flags=re.DOTALL
    )

    # Also remove any multi-line commented Config blocks
    lines = content.split('\n')
    new_lines = []
    skip_block = False
    brace_count = 0

    for line in lines:
        # Check if starting a commented impl block
        if re.match(r'^\s*// impl pallet_\w+_bridge::Config', line):
            skip_block = True
            brace_count = 0
            continue

        if skip_block:
            # Count braces in commented lines
            if line.strip().startswith('//'):
                brace_count += line.count('{')
                brace_count -= line.count('}')
                if brace_count <= 0 and '}' in line:
                    skip_block = False
                continue
            else:
                skip_block = False

        new_lines.append(line)

    content = '\n'.join(new_lines)

    runtime_path.write_text(content)
    return True

def main():
    print("🔧 Removing Duplicate Bridge Configurations")
    print("=" * 60)

    fixed = 0
    for pbc in PBCS:
        print(f"📦 {pbc}-pbc...", end=" ")
        if remove_duplicates(pbc):
            print("✅")
            fixed += 1
        else:
            print("❌")

    print(f"\n{'=' * 60}")
    print(f"✅ Cleaned {fixed}/{len(PBCS)} runtimes")
    print(f"{'=' * 60}")

if __name__ == "__main__":
    main()
