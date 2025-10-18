#!/bin/bash
# Copy BOTH Cargo.toml and lib.rs from btc-pbc to all 9 runtimes
# Then customize for each PBC

PBCS="doge xrp bnb trx ada link matic sc-usdt sol"
BASE="05-multichain/partition-burst-chains/pbc-chains"

echo "🚀 Complete Fix: Copying BTC runtime structure to all 9"
echo "=========================================================="

for pbc in $PBCS; do
    echo ""
    echo "📦 $pbc-pbc"

    # Copy Cargo.toml
    cp "$BASE/btc-pbc/runtime/Cargo.toml" "$BASE/$pbc-pbc/runtime/Cargo.toml"
    sed -i '' "s/btc-pbc-runtime/${pbc}-pbc-runtime/g" "$BASE/$pbc-pbc/runtime/Cargo.toml"
    sed -i '' "s/btc_pbc_runtime/${pbc//-/_}_pbc_runtime/g" "$BASE/$pbc-pbc/runtime/Cargo.toml"
    echo "  ✅ Cargo.toml copied and customized"

    # Copy lib.rs (already done by Python script, but ensure it's there)
    if [ ! -f "$BASE/$pbc-pbc/runtime/src/lib.rs" ]; then
        cp "$BASE/btc-pbc/runtime/src/lib.rs" "$BASE/$pbc-pbc/runtime/src/lib.rs"
    fi

    # Now customize lib.rs for bridge pallet names
    case $pbc in
        "doge")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_doge_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/DogeBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "xrp")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_ripple_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/RippleBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "bnb")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_binance_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/BinanceBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "trx")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_tron_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/TronBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "ada")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_cardano_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/CardanoBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "link")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_chainlink_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/ChainlinkBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "matic")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_polygon_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/PolygonBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "sc-usdt")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_stellar_usdt_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/StellarUsdtBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
        "sol")
            sed -i '' 's/pallet_bitcoin_bridge/pallet_solana_bridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            sed -i '' 's/BitcoinBridge/SolanaBridge/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
            ;;
    esac

    echo "  ✅ Bridge pallet customized for $pbc"

    # Now comment out the bridge sections since Config impls differ
    perl -i -0pe 's/(pub use pallet_\w+_bridge;)/\/\/ $1  \/\/ TODO: Bridge Config/g' "$BASE/$pbc-pbc/runtime/src/lib.rs"
    perl -i -0pe 's/(impl pallet_\w+_bridge::Config for Runtime \{[^\}]*\})/my $x = $1; $x =~ s\/^\/\/\/ \/gm; $x/gse' "$BASE/$pbc-pbc/runtime/src/lib.rs"
    perl -i -0pe 's/(\s+\w+Bridge: pallet_\w+_bridge,)$/\/\/ $1  \/\/ TODO/gm' "$BASE/$pbc-pbc/runtime/src/lib.rs"

    echo "  ✅ Bridge sections commented out"
done

echo ""
echo "=========================================================="
echo "✅ All 9 runtimes updated with complete BTC structure"
echo "=========================================================="
