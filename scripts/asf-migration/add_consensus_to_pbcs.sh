#!/bin/bash
# Script to add missing Grandpa + Consensus (ASF) to incomplete PBC runtimes

set -e

echo "🔧 Adding Grandpa + Consensus to incomplete PBC runtimes..."
echo "=============================================================="

# List of PBCs that need Grandpa + Consensus added
INCOMPLETE_PBCS="eth doge sol xlm xrp bnb trx ada link matic sc-usdt"

for pbc in $INCOMPLETE_PBCS; do
    RUNTIME_FILE="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/src/lib.rs"

    if [ ! -f "$RUNTIME_FILE" ]; then
        echo "⚠️  Skipping ${pbc}-pbc: Runtime file not found"
        continue
    fi

    echo "📝 Processing ${pbc}-pbc-runtime..."

    # Backup original file
    cp "$RUNTIME_FILE" "${RUNTIME_FILE}.consensus_backup"

    # Step 1: Add missing imports if not present
    if ! grep -q "pub use pallet_consensus;" "$RUNTIME_FILE"; then
        # Find the line with custom pallets comment and add after it
        sed -i.bak1 '/^\/\/ Custom pallets/a\
pub use pallet_consensus;' "$RUNTIME_FILE"
    fi

    # Step 2: Add pallet_insecure_randomness_collective_flip::Config if missing
    if ! grep -q "impl pallet_insecure_randomness_collective_flip::Config for Runtime" "$RUNTIME_FILE"; then
        # Add after pallet_balances::Config
        perl -i.bak2 -0pe 's/(impl pallet_balances::Config for Runtime \{[^\}]*\})\n/\1\n\nimpl pallet_insecure_randomness_collective_flip::Config for Runtime {}\n/s' "$RUNTIME_FILE"
    fi

    # Step 3: Add Grandpa configuration if missing
    if ! grep -q "impl pallet_grandpa::Config for Runtime" "$RUNTIME_FILE"; then
        # Add after RandomnessCollectiveFlip config
        perl -i.bak3 -0pe 's/(impl pallet_insecure_randomness_collective_flip::Config for Runtime \{\})\n/\1\n\nimpl pallet_grandpa::Config for Runtime {\n    type RuntimeEvent = RuntimeEvent;\n    type WeightInfo = ();\n    type MaxAuthorities = frame_support::traits::ConstU32<32>;\n    type MaxSetIdSessionEntries = frame_support::traits::ConstU64<0>;\n    type MaxNominators = frame_support::traits::ConstU32<0>;\n    type KeyOwnerProof = sp_core::Void;\n    type EquivocationReportSystem = ();\n}\n/s' "$RUNTIME_FILE"
    fi

    # Step 4: Add ASF Consensus configuration if missing
    if ! grep -q "impl pallet_consensus::Config for Runtime" "$RUNTIME_FILE"; then
        # Add after Grandpa config
        perl -i.bak4 -0pe 's/(impl pallet_grandpa::Config for Runtime \{[^\}]*\})\n/\1\n\n\/\/ ASF Consensus Configuration\nimpl pallet_consensus::Config for Runtime {\n    type RuntimeEvent = RuntimeEvent;\n    type Currency = Balances;\n    type RandomnessSource = RandomnessCollectiveFlip;\n    type Time = Timestamp;\n    type MinValidityStake = frame_support::traits::ConstU128<64_000_000_000_000_000_000_000>; \/\/ 64 ETR\n    type ValidatorReward = frame_support::traits::ConstU128<100_000_000_000_000_000_000>; \/\/ 0.1 ETR\n    type CommitteeSize = frame_support::traits::ConstU32<21>; \/\/ PPFA committee\n    type EpochDuration = frame_support::traits::ConstU32<2400>; \/\/ ~4 hours at 6s\/block\n    type BaseSlotDuration = frame_support::traits::ConstU64<6000>; \/\/ 6 seconds\n}\n/s' "$RUNTIME_FILE"
    fi

    # Step 5: Add pallets to construct_runtime! if missing
    # Add RandomnessCollectiveFlip if missing
    if ! grep -q "RandomnessCollectiveFlip:" "$RUNTIME_FILE"; then
        sed -i.bak5 's/System: frame_system,/System: frame_system,\n        RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,/' "$RUNTIME_FILE"
    fi

    # Add Grandpa if missing
    if ! grep -q "Grandpa: pallet_grandpa," "$RUNTIME_FILE"; then
        sed -i.bak6 's/Timestamp: pallet_timestamp,/Timestamp: pallet_timestamp,\n        Grandpa: pallet_grandpa,/' "$RUNTIME_FILE"
    fi

    # Add Consensus if missing
    if ! grep -q "Consensus: pallet_consensus," "$RUNTIME_FILE"; then
        # Add after Balances
        sed -i.bak7 's/Balances: pallet_balances,/Balances: pallet_balances,\n\n        \/\/ Ëtrid Core\n        Consensus: pallet_consensus,/' "$RUNTIME_FILE"
    fi

    # Step 6: Add GrandpaApi implementation if missing
    if ! grep -q "impl sp_consensus_grandpa::GrandpaApi" "$RUNTIME_FILE"; then
        # Find the sp_session::SessionKeys implementation and add GrandpaApi after it
        perl -i.bak8 -0pe 's/(impl sp_session::SessionKeys<Block> for Runtime \{[^\}]*\}[^\}]*\})\n/\1\n\n    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {\n        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {\n            Grandpa::grandpa_authorities()\n        }\n\n        fn current_set_id() -> sp_consensus_grandpa::SetId {\n            Grandpa::current_set_id()\n        }\n\n        fn submit_report_equivocation_unsigned_extrinsic(\n            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<\n                <Block as BlockT>::Hash,\n                NumberFor<Block>,\n            >,\n            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,\n        ) -> Option<()> {\n            None\n        }\n\n        fn generate_key_ownership_proof(\n            _set_id: sp_consensus_grandpa::SetId,\n            _authority_id: sp_consensus_grandpa::AuthorityId,\n        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {\n            None\n        }\n    }\n/s' "$RUNTIME_FILE"
    fi

    # Clean up backup files
    rm -f "${RUNTIME_FILE}.bak"*

    echo "✅ ${pbc}-pbc-runtime: Grandpa + Consensus added"
done

echo ""
echo "=============================================================="
echo "✨ All incomplete PBC runtimes updated with ASF consensus!"
echo "Next: Build and verify all 12 PBC runtimes"
