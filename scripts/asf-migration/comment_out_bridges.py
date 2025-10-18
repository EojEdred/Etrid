#!/usr/bin/env python3
"""
Comment out bridge pallets in all 9 runtimes to get them compiling.
Bridge integration can be completed later - ASF consensus doesn't depend on bridges.
"""

import re
from pathlib import Path

PBCS = ["doge", "xrp", "bnb", "trx", "ada", "link", "matic", "sc-usdt", "sol"]
BASE_DIR = Path("05-multichain/partition-burst-chains/pbc-chains")

def comment_bridge_sections(pbc_name):
    """Comment out bridge pallet usage in runtime"""

    runtime_path = BASE_DIR / f"{pbc_name}-pbc" / "runtime" / "src" / "lib.rs"

    if not runtime_path.exists():
        return False

    content = runtime_path.read_text()

    # Comment out bridge pallet import
    content = re.sub(
        r'^(pub use pallet_\w+_bridge;)$',
        r'// \1  // TODO: Complete bridge Config implementation',
        content,
        flags=re.MULTILINE
    )

    # Comment out bridge Config impl block
    content = re.sub(
        r'(impl pallet_\w+_bridge::Config for Runtime \{.*?\n\})',
        lambda m: '\n'.join('// ' + line if line.strip() else line for line in m.group(1).split('\n')),
        content,
        flags=re.DOTALL
    )

    # Comment out bridge in construct_runtime! macro
    content = re.sub(
        r'(\s+)(\w+Bridge: pallet_\w+_bridge,)',
        r'\1// \2  // TODO: Complete bridge integration',
        content
    )

    runtime_path.write_text(content)
    return True

def main():
    print("🔧 Commenting out bridge pallets in all 9 runtimes")
    print("=" * 60)

    fixed = 0
    for pbc in PBCS:
        print(f"📦 {pbc}-pbc...", end=" ")
        if comment_bridge_sections(pbc):
            print("✅")
            fixed += 1
        else:
            print("❌")

    print(f"\n{'=' * 60}")
    print(f"✅ Fixed {fixed}/{len(PBCS)} runtimes")
    print(f"{'=' * 60}")

if __name__ == "__main__":
    main()
