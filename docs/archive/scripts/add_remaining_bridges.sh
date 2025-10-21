#!/bin/bash

# Add bridge integration to the 7 remaining PBCs
# doge, bnb, trx, ada, link, matic, sol

set -e

echo "========================================"
echo "🔧 Adding Bridges to Remaining 7 PBCs"
echo "========================================"
echo ""

# Function to add bridge to a runtime
add_bridge() {
    local pbc=$1
    local pallet_crate=$2
    local pallet_name=$3
    local param_prefix=$4
    local min_conf=$5
    local min_dep=$6
    local max_dep=$7
    local min_comment=$8
    local max_comment=$9

    echo "Processing $pbc-pbc ($pallet_name)..."

    local runtime_file="05-multichain/partition-burst-chains/pbc-chains/$pbc-pbc/runtime/src/lib.rs"

    if [ ! -f "$runtime_file" ]; then
        echo "  ❌ Runtime file not found"
        return 1
    fi

    # Check if bridge already exists
    if grep -q "impl $pallet_crate::Config for Runtime" "$runtime_file"; then
        echo "  ✅ Bridge Config already exists"
        return 0
    fi

    # 1. Add parameter_types! and Config implementation before construct_runtime!
    local bridge_config="
// $pallet_name Configuration
parameter_types! {
    pub const Min${param_prefix}Confirmations: u32 = $min_conf;
    pub const Min${param_prefix}DepositAmount: u64 = $min_dep; // $min_comment
    pub const Max${param_prefix}DepositAmount: u64 = $max_dep; // $max_comment
    pub const BridgeAuthorityAccount: AccountId = AccountId::new([0u8; 32]);
}

impl $pallet_crate::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinConfirmations = Min${param_prefix}Confirmations;
    type MinDepositAmount = Min${param_prefix}DepositAmount;
    type MaxDepositAmount = Max${param_prefix}DepositAmount;
    type BridgeAuthority = BridgeAuthorityAccount;
}
"

    # Insert before construct_runtime!
    perl -i -0pe "s/(\/\/ Create the runtime.*?construct_runtime!\()/$bridge_config\n\$1/s" "$runtime_file"

    # 2. Add bridge to construct_runtime! macro (before closing brace)
    local bridge_entry="        \n        // Cross-chain Bridge\n        $pallet_name: $pallet_crate,"

    perl -i -0pe "s/(\/\/ Ëtrid Core\s+Consensus: pallet_consensus,)(\s+\})/\$1$bridge_entry\$2/s" "$runtime_file"

    echo "  ✅ Added bridge to runtime"
}

# DOGE (Dogecoin)
add_bridge "doge" "pallet_doge_bridge" "DogeBridge" "Doge" \
    20 1000000 1000000000000 "1 DOGE" "1M DOGE"

# BNB (Binance)
add_bridge "bnb" "pallet_bnb_bridge" "BnbBridge" "Bnb" \
    15 10000000000000000 100000000000000000000 "0.01 BNB" "100 BNB"

# TRX (Tron)
add_bridge "trx" "pallet_trx_bridge" "TronBridge" "Trx" \
    19 1000000 100000000000 "1 TRX" "100k TRX"

# ADA (Cardano)
add_bridge "ada" "pallet_cardano_bridge" "CardanoBridge" "Ada" \
    15 1000000 100000000000 "1 ADA" "100k ADA"

# LINK (Chainlink)
add_bridge "link" "pallet_chainlink_bridge" "ChainlinkBridge" "Link" \
    12 10000000000000000 10000000000000000000000 "0.01 LINK" "10k LINK"

# MATIC (Polygon)
add_bridge "matic" "pallet_polygon_bridge" "PolygonBridge" "Matic" \
    128 10000000000000000 100000000000000000000000 "0.01 MATIC" "100k MATIC"

# SOL (Solana)
add_bridge "sol" "pallet_sol_bridge" "SolanaBridge" "Sol" \
    32 10000000000000000 100000000000000000000 "0.01 SOL" "100 SOL"

echo ""
echo "========================================"
echo "✅ Completed adding bridges to 7 PBCs"
echo "========================================"
