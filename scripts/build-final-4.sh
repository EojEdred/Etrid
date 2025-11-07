#!/bin/bash
cd /Users/macbook/Desktop/etrid

echo "=== Building Final 4 PBC Collators ==="
date

cargo build --release -p trx-pbc-collator && echo "✓ trx complete"
cargo build --release -p ada-pbc-collator && echo "✓ ada complete"
cargo build --release -p link-pbc-collator && echo "✓ link complete"
cargo build --release -p doge-pbc-collator && echo "✓ doge complete"

echo ""
echo "=== FINAL RESULTS ==="
ls -lh target/release/*-pbc-collator
echo ""
echo "TOTAL: $(ls target/release/*-pbc-collator | wc -l) / 12 BUILT"
date
