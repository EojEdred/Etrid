#!/usr/bin/env python3
"""
Remove leftover AURA code fragments from collator service.rs files.
"""

import re
from pathlib import Path

COLLATORS = [
    "doge-pbc-collator",
    "xlm-pbc-collator",
    "bnb-pbc-collator",
    "trx-pbc-collator",
    "ada-pbc-collator",
    "link-pbc-collator",
    "matic-pbc-collator",
    "sc-usdt-pbc-collator",
    "sol-pbc-collator",
    "xrp-pbc-collator",
]

BASE_PATH = Path("05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes")

def clean_collator(collator_name):
    """Remove AURA leftover code from collator"""
    service_path = BASE_PATH / collator_name / "src" / "service.rs"

    if not service_path.exists():
        return False

    content = service_path.read_text()
    original = content

    # Remove leftover AURA code after import_queue
    # Pattern: Lines with indentation that reference AURA stuff between import_queue and Ok(sc_service::PartialComponents
    pattern = r'(\.map_err\(.*?import queue error.*?\)\?\;)\s+let timestamp = sp_timestamp.*?\}\s*\)\?\;'

    content = re.sub(pattern, r'\1', content, flags=re.DOTALL)

    # Also remove any standalone AURA inherent data provider blocks
    pattern2 = r'\s+let timestamp = sp_timestamp::InherentDataProvider::from_system_time\(\);\s+let slot =\s+sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration\(.*?\}\s+\},\s+spawner:.*?compatibility_mode:.*?\},\s+\)\?\;'

    content = re.sub(pattern2, '', content, flags=re.DOTALL)

    if content != original:
        service_path.write_text(content)
        return True
    return False

def main():
    print("🧹 Removing AURA leftover code...")

    fixed = 0
    for collator in COLLATORS:
        if clean_collator(collator):
            print(f"  ✅ Cleaned {collator}")
            fixed += 1
        else:
            print(f"  ℹ️  {collator} - no changes")

    print(f"\n✅ Cleaned {fixed} collators")

if __name__ == "__main__":
    main()
