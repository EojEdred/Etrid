#!/bin/bash
# Properly fix all 9 runtimes by copying BTC and adjusting bridge pallets

PBCS=(
    "doge:doge:Doge:DOGE"
    "xrp:xrp:Xrp:XRP"
    "bnb:bnb:Bnb:BNB"
    "trx:tron:Trx:TRX"
    "ada:cardano:Ada:ADA"
    "link:chainlink:Link:LINK"
    "matic:polygon:Matic:MATIC"
    "sc-usdt:stellar_usdt:ScUsdt:SC-USDT"
    "sol:solana:Sol:SOL"
)

BASE_DIR="05-multichain/partition-burst-chains/pbc-chains"

echo "🔧 Properly fixing all 9 runtimes..."
echo ""

for pbc_config in "${PBCS[@]}"; do
    IFS=':' read -r pbc_name bridge_name pascal_case upper_case <<< "$pbc_config"

    echo "=== ${pbc_name}-pbc ==="

    src_file="$BASE_DIR/btc-pbc/runtime/src/lib.rs"
    dest_file="$BASE_DIR/${pbc_name}-pbc/runtime/src/lib.rs"

    # Restore from .pre_btc_copy if it exists, otherwise use consensus_backup
    if [ -f "${dest_file}.pre_btc_copy" ]; then
        cp "${dest_file}.pre_btc_copy" "$dest_file"
        echo "  📦 Restored from pre_btc_copy"
    elif [ -f "${dest_file}.consensus_backup" ]; then
        cp "${dest_file}.consensus_backup" "$dest_file"
        echo "  📦 Restored from consensus_backup"
    else
        echo "  ⚠️  No backup found, using current file"
    fi

    # Now properly add AsfApi by looking for the GrandpaApi block
    perl -i -0pe 's/(impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime \{[^\}]*\n    \})\n(?!    impl sp_consensus_asf)/$1\n
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
/s' "$dest_file"

    echo "  ✅ Added AsfApi to ${pbc_name}-pbc"
done

echo ""
echo "✅ All 9 runtimes properly fixed!"
