#!/bin/bash
# Script to deploy GenesisBuilder API to all remaining PBC runtimes
# Based on successful BTC PBC implementation

set -e

PBCS=("eth" "doge" "sol" "xlm" "xrp" "bnb" "trx" "ada" "link" "matic" "sc-usdt")
BASE_DIR="05-multichain/partition-burst-chains/pbc-chains"
BTC_PRESET_DIR="$BASE_DIR/btc-pbc/runtime/presets"

echo "========================================"
echo "GenesisBuilder Deployment to 11 PBCs"
echo "========================================"
echo ""

for pbc in "${PBCS[@]}"; do
    echo "Processing $pbc-pbc..."

    PBC_DIR="$BASE_DIR/$pbc-pbc/runtime"

    # Step 1: Create presets directory
    echo "  - Creating presets directory..."
    mkdir -p "$PBC_DIR/presets"

    # Step 2: Copy preset files from BTC PBC
    echo "  - Copying preset files..."
    cp "$BTC_PRESET_DIR/development.json" "$PBC_DIR/presets/"
    cp "$BTC_PRESET_DIR/local_testnet.json" "$PBC_DIR/presets/"

    # Step 3: Add sp-genesis-builder to Cargo.toml
    echo "  - Adding sp-genesis-builder dependency..."

    # Check if dependency already exists
    if ! grep -q "sp-genesis-builder" "$PBC_DIR/Cargo.toml"; then
        # Add to dependencies section (after sp-consensus-asf line)
        sed -i '' '/sp-consensus-asf = {.*}/a\
sp-genesis-builder = { default-features = false, git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2506" }
' "$PBC_DIR/Cargo.toml"

        # Add to std features (after sp-genesis-builder line would be added)
        sed -i '' '/    "sp-consensus-asf\/std",/a\
    "sp-genesis-builder/std",
' "$PBC_DIR/Cargo.toml"

        echo "    ✓ Dependency added"
    else
        echo "    ⚠ Dependency already exists, skipping"
    fi

    # Step 4: Add GenesisBuilder implementation to lib.rs
    echo "  - Adding GenesisBuilder implementation to lib.rs..."

    # Check if GenesisBuilder already implemented
    if ! grep -q "impl sp_genesis_builder::GenesisBuilder" "$PBC_DIR/src/lib.rs"; then
        # Find the closing brace of impl_runtime_apis! block
        # Insert GenesisBuilder implementation before it

        cat >> "$PBC_DIR/src/lib.rs.genesis_addition" << 'EOF'

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
EOF

        # Need to insert this before the last closing brace of impl_runtime_apis!
        # This is tricky with sed, so let's use a Python script
        python3 - <<PYTHON_SCRIPT
import re

pbc = "$pbc"
file_path = "$PBC_DIR/src/lib.rs"

with open(file_path, 'r') as f:
    content = f.read()

# Read the GenesisBuilder addition
with open(file_path + '.genesis_addition', 'r') as f:
    genesis_code = f.read()

# Find the impl_runtime_apis! block and insert before its closing brace
# Look for the pattern: closing brace followed by closing brace of impl_runtime_apis!
pattern = r'(\n    \})\n\}'
replacement = genesis_code + r'\1\n}'

# Check if we need to add it
if 'impl sp_genesis_builder::GenesisBuilder' not in content:
    content = re.sub(pattern, replacement, content, count=1)

    with open(file_path, 'w') as f:
        f.write(content)
    print(f"    ✓ GenesisBuilder added to {pbc}-pbc")
else:
    print(f"    ⚠ GenesisBuilder already exists in {pbc}-pbc")
PYTHON_SCRIPT

        rm -f "$PBC_DIR/src/lib.rs.genesis_addition"
        echo "    ✓ GenesisBuilder implementation added"
    else
        echo "    ⚠ GenesisBuilder already implemented, skipping"
    fi

    echo "  ✓ $pbc-pbc completed"
    echo ""
done

echo "========================================"
echo "Deployment Complete!"
echo "========================================"
echo ""
echo "Next steps:"
echo "1. Build all PBC runtimes with WASM"
echo "2. Test each collator can generate chain specs"
echo ""
echo "To build all at once (will take ~60-90 minutes):"
echo "  for pbc in eth doge sol xlm xrp bnb trx ada link matic sc-usdt; do"
echo "    cargo build --release -p \${pbc}-pbc-collator &"
echo "  done"
echo "  wait"
