#!/usr/bin/env bash
# Build 11 working PBC collators (excluding eth-pbc due to Frontier version conflict)

cd /Users/macbook/Desktop/etrid

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Building 11 Working PBC Collators                         ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "Excluded: eth-pbc (Frontier stable2506/stable2509 conflict)"
echo ""

# Build each PBC collator
cargo build --release -p btc-pbc-collator
cargo build --release -p sol-pbc-collator
cargo build --release -p xrp-pbc-collator
cargo build --release -p bnb-pbc-collator
cargo build --release -p trx-pbc-collator
cargo build --release -p ada-pbc-collator
cargo build --release -p matic-pbc-collator
cargo build --release -p link-pbc-collator
cargo build --release -p sc-usdt-pbc-collator
cargo build --release -p doge-pbc-collator
cargo build --release -p xlm-pbc-collator

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Build Complete - Checking Binaries                        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

ls -lh target/release/*-pbc-collator 2>/dev/null || echo "No binaries found"
