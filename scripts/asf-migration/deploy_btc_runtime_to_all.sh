#!/bin/bash
# Copy entire working btc-pbc runtime to all 9 broken runtimes and customize

BROKEN_PBCS=(
    "doge:Doge:DOGE:Dogecoin:doge:Doge Doge :60000:30000:Koinus:Much crypto, such decentralized, wow!"
    "xrp:Xrp:XRP:XRP Ledger:xrp:XRP Ripple:5000:2500:Drops:Enterprise-grade cross-border payments"
    "bnb:Bnb:BNB:Binance Smart Chain:bnb:BNB BNB:3000:1500:Jager:DeFi powerhouse, Binance style"
    "trx:Trx:TRX:Tron:trx:Tron Tron:3000:1500:Sun:Decentralize the web, Justin style"
    "ada:Ada:ADA:Cardano:ada:Cardano Ada:20000:10000:Lovelace:Peer-reviewed blockchain perfection"
    "link:Link:LINK:Chainlink:link:Link Link:12000:6000:Juel:Oracles connecting on-chain to off-chain"
    "matic:Matic:MATIC:Polygon:matic:Polygon Matic:2000:1000:Wei:Ethereum scaling, gas so low"
    "sc-usdt:ScUsdt:SC-USDT:Stellar USDT:scusdt:Stellar USDT:5000:2500:Stroops:Stablecoin + Stellar speed = perfect"
    "sol:Sol:SOL:Solana:sol:Solana Sol:400:200:Lamport:So fast, very web3, much throughput"
)

BASE_DIR="05-multichain/partition-burst-chains/pbc-chains"

echo "🔧 Deploying working btc-pbc runtime to all 9 runtimes..."
echo ""

for pbc_config in "${BROKEN_PBCS[@]}"; do
    IFS=':' read -r pbc_lower pbc_pascal pbc_upper chain_name pbc_name_lower pbc_name_full block_time min_period unit_name flavor <<< "$pbc_config"

    echo "=== ${pbc_lower}-pbc ==="

    source_file="$BASE_DIR/btc-pbc/runtime/src/lib.rs"
    dest_file="$BASE_DIR/${pbc_lower}-pbc/runtime/src/lib.rs"

    # Backup
    cp "$dest_file" "${dest_file}.pre_btc_copy"

    # Copy btc runtime
    cp "$source_file" "$dest_file"

    # Replace BTC-specific values
    sed -i '' "s/BTC-PBC RUNTIME/${pbc_upper}-PBC RUNTIME/g" "$dest_file"
    sed -i '' "s/Bitcoin Partition Burst Chain/${chain_name} Partition Burst Chain/g" "$dest_file"
    sed -i '' "s/Priority #1/Priority varies/g" "$dest_file"
    sed -i '' "s/Features: 10-minute blocks, SHA-256, 8 decimal precision (Satoshis)/Features: ${flavor}/g" "$dest_file"
    sed -i '' "s/btc_pbc/${pbc_lower}_pbc/g" "$dest_file"
    sed -i '' "s/btc-pbc/${pbc_lower}-pbc/g" "$dest_file"
    sed -i '' "s/Btc/${pbc_pascal}/g" "$dest_file"
    sed -i '' "s/BTC/${pbc_upper}/g" "$dest_file"
    sed -i '' "s/BtcPbc/${pbc_pascal}Pbc/g" "$dest_file"
    sed -i '' "s/BTC PBC/${pbc_upper} PBC/g" "$dest_file"
    sed -i '' "s/Bitcoin/${pbc_name_full}/g" "$dest_file"
    sed -i '' "s/Satoshi/${unit_name}/g" "$dest_file"

    # Fix block times (btc is 600000/300000)
    sed -i '' "s/600000/${block_time}/g" "$dest_file"
    sed -i '' "s/300000/${min_period}/g" "$dest_file"

    echo "  ✅ Deployed and customized runtime"
done

echo ""
echo "✅ All 9 runtime files deployed!"
