#!/bin/bash
# Script to build all 12 PBC runtimes

set -e

echo "🏗️  Building all 12 PBC runtimes..."
echo "===================================="

env SKIP_WASM_BUILD=1 cargo check -p btc-pbc-runtime
echo "✅ btc-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p eth-pbc-runtime
echo "✅ eth-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p doge-pbc-runtime
echo "✅ doge-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p sol-pbc-runtime
echo "✅ sol-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p xlm-pbc-runtime
echo "✅ xlm-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p xrp-pbc-runtime
echo "✅ xrp-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p bnb-pbc-runtime
echo "✅ bnb-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p trx-pbc-runtime
echo "✅ trx-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p ada-pbc-runtime
echo "✅ ada-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p link-pbc-runtime
echo "✅ link-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p matic-pbc-runtime
echo "✅ matic-pbc-runtime compiled successfully"

env SKIP_WASM_BUILD=1 cargo check -p sc-usdt-pbc-runtime
echo "✅ sc-usdt-pbc-runtime compiled successfully"

echo ""
echo "===================================="
echo "✨ All 12 PBC runtimes compiled successfully with ASF consensus!"
