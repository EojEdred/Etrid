#!/bin/bash
# Fix bridge dependencies in all runtime Cargo.toml files
# Use correct package names with renaming

BASE="05-multichain/partition-burst-chains/pbc-chains"

# BTC
sed -i '' 's|pallet_bitcoin_bridge = { path.*|pallet_bitcoin_bridge = { package = "pallet-bitcoin-bridge", path = "../../../../../05-multichain/bridge-protocols/bitcoin-bridge", default-features = false }|' "$BASE/btc-pbc/runtime/Cargo.toml"

# ETH
sed -i '' 's|pallet_ethereum_bridge = { path.*|pallet_ethereum_bridge = { package = "eth-bridge", path = "../../../../../05-multichain/bridge-protocols/ethereum-bridge", default-features = false }|' "$BASE/eth-pbc/runtime/Cargo.toml"

# DOGE
sed -i '' 's|pallet_doge_bridge = { path.*|pallet_doge_bridge = { package = "pallet-doge-bridge", path = "../../../../../05-multichain/bridge-protocols/doge-bridge", default-features = false }|' "$BASE/doge-pbc/runtime/Cargo.toml"

# XLM (Stellar)
sed -i '' 's|pallet_stellar_bridge = { path.*|pallet_stellar_bridge = { package = "stellar-bridge", path = "../../../../../05-multichain/bridge-protocols/stellar-bridge", default-features = false }|' "$BASE/xlm-pbc/runtime/Cargo.toml"

# XRP
sed -i '' 's|pallet_xrp_bridge = { path.*|pallet_xrp_bridge = { package = "xrp-bridge", path = "../../../../../05-multichain/bridge-protocols/xrp-bridge", default-features = false }|' "$BASE/xrp-pbc/runtime/Cargo.toml"

# BNB
sed -i '' 's|pallet_bnb_bridge = { path.*|pallet_bnb_bridge = { package = "bnb-bridge", path = "../../../../../05-multichain/bridge-protocols/bnb-bridge", default-features = false }|' "$BASE/bnb-pbc/runtime/Cargo.toml"

# TRX (Tron)
sed -i '' 's|pallet_tron_bridge = { path.*|pallet_tron_bridge = { package = "trx-bridge", path = "../../../../../05-multichain/bridge-protocols/tron-bridge", default-features = false }|' "$BASE/trx-pbc/runtime/Cargo.toml"

# ADA (Cardano)
sed -i '' 's|pallet_cardano_bridge = { path.*|pallet_cardano_bridge = { package = "pallet-cardano-bridge", path = "../../../../../05-multichain/bridge-protocols/cardano-bridge", default-features = false }|' "$BASE/ada-pbc/runtime/Cargo.toml"

# LINK (Chainlink)
sed -i '' 's|pallet_chainlink_bridge = { path.*|pallet_chainlink_bridge = { package = "chainlink-bridge", path = "../../../../../05-multichain/bridge-protocols/chainlink-bridge", default-features = false }|' "$BASE/link-pbc/runtime/Cargo.toml"

# MATIC (Polygon)
sed -i '' 's|pallet_polygon_bridge = { path.*|pallet_polygon_bridge = { package = "polygon-bridge", path = "../../../../../05-multichain/bridge-protocols/polygon-bridge", default-features = false }|' "$BASE/matic-pbc/runtime/Cargo.toml"

# SC-USDT (Stablecoin USDT)
sed -i '' 's|pallet_stablecoin_usdt_bridge = { path.*|pallet_stablecoin_usdt_bridge = { package = "stablecoin-usdt-bridge", path = "../../../../../05-multichain/bridge-protocols/stablecoin-usdt-bridge", default-features = false }|' "$BASE/sc-usdt-pbc/runtime/Cargo.toml"

# SOL (Solana)
sed -i '' 's|pallet_solana_bridge = { path.*|pallet_solana_bridge = { package = "sol-bridge", path = "../../../../../05-multichain/bridge-protocols/solana-bridge", default-features = false }|' "$BASE/sol-pbc/runtime/Cargo.toml"

echo "✅ All bridge dependencies fixed with correct package names"
