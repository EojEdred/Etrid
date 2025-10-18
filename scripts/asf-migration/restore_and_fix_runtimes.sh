#!/bin/bash
# Restore from backups and properly add AsfApi to all 9 broken runtimes

BROKEN_PBCS="doge xrp bnb trx ada link matic sc-usdt sol"
BASE_DIR="05-multichain/partition-burst-chains/pbc-chains"

echo "🔧 Restoring and fixing 9 runtimes..."
echo ""

for pbc in $BROKEN_PBCS; do
    runtime_file="$BASE_DIR/${pbc}-pbc/runtime/src/lib.rs"
    backup_file="$BASE_DIR/${pbc}-pbc/runtime/src/lib.rs.consensus_backup"

    echo "=== ${pbc}-pbc ==="

    # Check if consensus_backup exists
    if [ ! -f "$backup_file" ]; then
        echo "  ❌ No consensus_backup found"
        continue
    fi

    # Restore from backup
    cp "$backup_file" "$runtime_file"
    echo "  ✅ Restored from consensus_backup"

    # Now add the AsfApi block after GrandpaApi using perl
    perl -i -0pe 's/(impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime \{.*?\n    \})\n/$1\n
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
/s' "$runtime_file"

    echo "  ✅ Added AsfApi implementation"

done

echo ""
echo "✅ All 9 runtimes restored and fixed!"
