#!/bin/bash
set -e

cd /Users/macbook/Desktop/etrid

echo "=== Building Remaining PBC Collators ==="
echo ""

# Build each remaining collator
cargo build --release -p trx-pbc-collator && echo "✓ trx-pbc-collator complete" || echo "✗ trx failed"
cargo build --release -p ada-pbc-collator && echo "✓ ada-pbc-collator complete" || echo "✗ ada failed"
cargo build --release -p link-pbc-collator && echo "✓ link-pbc-collator complete" || echo "✗ link failed"
cargo build --release -p sc-usdt-pbc-collator && echo "✓ sc-usdt-pbc-collator complete" || echo "✗ sc-usdt failed"
cargo build --release -p doge-pbc-collator && echo "✓ doge-pbc-collator complete" || echo "✗ doge failed"
cargo build --release -p xlm-pbc-collator && echo "✓ xlm-pbc-collator complete" || echo "✗ xlm failed"

echo ""
echo "=== FINAL STATUS ==="
ls -lh target/release/*-pbc-collator
echo ""
echo "Total: $(ls target/release/*-pbc-collator | wc -l) / 12 collators built"
