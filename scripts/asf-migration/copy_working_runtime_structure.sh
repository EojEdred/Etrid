#!/bin/bash
# Copy working runtime API structure from btc-pbc to all broken runtimes

BROKEN_PBCS="doge xrp bnb trx ada link matic sc-usdt sol"

for pbc in $BROKEN_PBCS; do
    echo "Fixing ${pbc}-pbc runtime..."

    file="05-multichain/partition-burst-chains/pbc-chains/${pbc}-pbc/runtime/src/lib.rs"

    # Check if file exists
    if [ ! -f "$file" ]; then
        echo "  ❌ File not found: $file"
        continue
    fi

    # Create backup
    cp "$file" "${file}.pre_final_fix"

    # Remove any existing broken AsfApi implementation
    perl -i -0pe 's/impl sp_consensus_asf::AsfApi<Block, AccountId> for Runtime \{.*?\n    \}(\n    \})?//gs' "$file"

    # Find the line after GrandpaApi impl ends
    # Insert the correct AsfApi implementation there
    perl -i -pe 'if (/^    \}$/ && $in_grandpa) {
        $_ .= "\n    impl sp_consensus_asf::AsfApi<Block, AccountId> for Runtime {
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
    }\n";
        $in_grandpa = 0;
    }
    $in_grandpa = 1 if /impl sp_consensus_grandpa::GrandpaApi/;
    ' "$file"

    echo "  ✅ Fixed ${pbc}-pbc"
done

echo ""
echo "✅ All runtimes fixed with working structure!"
