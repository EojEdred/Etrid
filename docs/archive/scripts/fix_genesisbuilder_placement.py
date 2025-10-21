#!/usr/bin/env python3
"""
Fix GenesisBuilder placement in PBC runtime lib.rs files.
The deployment script incorrectly placed it inside the opaque module.
It needs to be inside the impl_runtime_apis! block instead.
"""

import re
import sys

PBCS = ["eth", "doge", "sol", "xlm", "xrp", "bnb", "trx", "ada", "link", "matic", "sc-usdt"]
BASE_DIR = "05-multichain/partition-burst-chains/pbc-chains"

GENESIS_BUILDER_CODE = """
    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            frame_support::genesis_builder_helper::build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            frame_support::genesis_builder_helper::get_preset::<RuntimeGenesisConfig>(id, |name| {
                match name.as_ref() {
                    sp_genesis_builder::DEV_RUNTIME_PRESET => {
                        Some(include_bytes!("../presets/development.json").to_vec())
                    },
                    sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => {
                        Some(include_bytes!("../presets/local_testnet.json").to_vec())
                    },
                    _ => None,
                }
            })
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![
                sp_genesis_builder::DEV_RUNTIME_PRESET.into(),
                sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET.into(),
            ]
        }
    }
"""

for pbc in PBCS:
    file_path = f"{BASE_DIR}/{pbc}-pbc/runtime/src/lib.rs"

    print(f"Processing {pbc}-pbc...")

    with open(file_path, 'r') as f:
        content = f.read()

    # Step 1: Remove the incorrectly placed GenesisBuilder from opaque module
    # Pattern: Find the GenesisBuilder block that was inserted inside opaque
    pattern = r'(    impl_opaque_keys! \{\n        pub struct SessionKeys \{\n            pub grandpa: Grandpa,\n        \}\n)    impl sp_genesis_builder::GenesisBuilder.*?        \}\n    \}\n\n(    \}\n\})'

    # Replace with just the correct opaque module structure
    replacement = r'\1    }\n\2'

    content = re.sub(pattern, replacement, content, flags=re.DOTALL)

    # Step 2: Find the impl_runtime_apis! block and insert GenesisBuilder before its closing brace
    # Look for the pattern: closing brace of last impl, then closing brace of impl_runtime_apis!
    # We want to insert before the final closing brace

    # Find the last occurrence of "    }\n}" which should be the end of impl_runtime_apis!
    pattern2 = r'(\n    \}\n)\}'

    # Count occurrences to find the last one
    matches = list(re.finditer(pattern2, content))

    if matches:
        # Get the position of the last match
        last_match = matches[-1]

        # Insert GenesisBuilder before the closing brace
        insert_pos = last_match.start(1)

        content = content[:insert_pos] + GENESIS_BUILDER_CODE + content[insert_pos:]

        print(f"  ✓ Fixed {pbc}-pbc")
    else:
        print(f"  ❌ Could not find insertion point in {pbc}-pbc")
        continue

    # Write the fixed content
    with open(file_path, 'w') as f:
        f.write(content)

    print(f"  ✓ {pbc}-pbc completed")

print("\nAll files fixed!")
print("\nNext step: Rebuild all PBCs")
