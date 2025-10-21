#!/bin/bash

# Extract Config trait requirements from all 12 bridge pallets

echo "==========================================="
echo "📋 Extracting All Bridge Config Traits"
echo "==========================================="
echo ""

BRIDGES=(
    "bitcoin-bridge:pallet_bitcoin_bridge"
    "ethereum-bridge:pallet_ethereum_bridge"
    "doge-bridge:pallet_doge_bridge"
    "stellar-bridge:pallet_stellar_bridge"
    "xrp-bridge:pallet_xrp_bridge"
    "bnb-bridge:pallet_bnb_bridge"
    "tron-bridge:pallet_trx_bridge"
    "cardano-bridge:pallet_cardano_bridge"
    "chainlink-bridge:pallet_chainlink_bridge"
    "polygon-bridge:pallet_polygon_bridge"
    "stablecoin-usdt-bridge:pallet_stablecoin_usdt_bridge"
    "solana-bridge:pallet_sol_bridge"
)

for bridge_info in "${BRIDGES[@]}"; do
    IFS=':' read -r bridge_dir pallet_name <<< "$bridge_info"

    echo "========== $pallet_name ($bridge_dir) =========="

    bridge_path="05-multichain/bridge-protocols/$bridge_dir/src/lib.rs"

    if [ ! -f "$bridge_path" ]; then
        echo "❌ NOT FOUND: $bridge_path"
        echo ""
        continue
    fi

    # Extract Config trait (from "pub trait Config" to closing brace)
    sed -n '/pub trait Config.*{/,/^[[:space:]]*}/p' "$bridge_path" | head -40

    echo ""
done

echo "==========================================="
echo "✅ Config trait extraction complete"
echo "==========================================="
