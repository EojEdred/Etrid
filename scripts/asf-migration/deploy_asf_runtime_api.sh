#!/bin/bash
# Deploy ASF Runtime API to all 12 PBC runtimes
# This script adds sp-consensus-asf dependency and Runtime API implementation

set -e

echo "=========================================="
echo "ASF Runtime API Deployment Script"
echo "=========================================="
echo ""

# List of all PBCs (btc already done)
PBCS=("eth" "doge" "sol" "xlm" "xrp" "bnb" "trx" "ada" "link" "matic" "sc-usdt")

BASE_PATH="/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains"

for pbc in "${PBCS[@]}"; do
    echo "=== Processing ${pbc}-pbc ===="

    RUNTIME_PATH="${BASE_PATH}/${pbc}-pbc/runtime"
    CARGO_TOML="${RUNTIME_PATH}/Cargo.toml"
    LIB_RS="${RUNTIME_PATH}/src/lib.rs"

    # Check if files exist
    if [ ! -f "$CARGO_TOML" ]; then
        echo "  ⚠️  Cargo.toml not found at $CARGO_TOML - SKIPPING"
        continue
    fi

    if [ ! -f "$LIB_RS" ]; then
        echo "  ⚠️  lib.rs not found at $LIB_RS - SKIPPING"
        continue
    fi

    # Step 1: Add dependency to Cargo.toml (if not already present)
    if grep -q "sp-consensus-asf" "$CARGO_TOML"; then
        echo "  ✓ sp-consensus-asf dependency already present in Cargo.toml"
    else
        echo "  + Adding sp-consensus-asf dependency to Cargo.toml"

        # Add after sp-consensus-grandpa line
        sed -i.bak '/^sp-consensus-grandpa = { workspace = true }$/a\
sp-consensus-asf = { path = "../../../../../09-consensus/primitives/consensus-asf", default-features = false }
' "$CARGO_TOML"

        # Add to features.std
        sed -i.bak2 '/    "sp-consensus-grandpa\/std",$/a\
    "sp-consensus-asf/std",
' "$CARGO_TOML"

        echo "  ✓ Updated Cargo.toml"
    fi

    # Step 2: Add Runtime API implementation to lib.rs (if not already present)
    if grep -q "impl sp_consensus_asf::AsfApi" "$LIB_RS"; then
        echo "  ✓ ASF Runtime API already implemented in lib.rs"
    else
        echo "  + Adding ASF Runtime API implementation to lib.rs"

        # Find the line with GrandpaApi closing brace and add ASF API after it
        # This is a simplified approach - insert after "impl sp_consensus_grandpa::GrandpaApi" block

        # Create the API implementation block
        API_BLOCK='
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
'

        # Find the GrandpaApi impl block and add ASF API after it
        # Using awk to find the right insertion point
        awk -v api="$API_BLOCK" '
        /impl frame_system_rpc_runtime_api::AccountNonceApi/ {
            print api
        }
        { print }
        ' "$LIB_RS" > "$LIB_RS.tmp" && mv "$LIB_RS.tmp" "$LIB_RS"

        echo "  ✓ Updated lib.rs"
    fi

    echo "  ✓ ${pbc}-pbc updated successfully"
    echo ""
done

echo "=========================================="
echo "Deployment Complete!"
echo "=========================================="
echo ""
echo "Next steps:"
echo "1. Verify compilation: env SKIP_WASM_BUILD=1 cargo check"
echo "2. Build all runtimes: ./build_all_pbc_runtimes.sh"
echo "3. Proceed with collator integration"
echo ""
