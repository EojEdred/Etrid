#!/bin/bash

VALIDATORS=(
  "100.93.43.18" "100.71.127.127" "100.68.185.50" "100.70.73.10"
  "100.88.104.58" "100.117.43.53" "100.109.252.56" "100.80.84.82"
  "100.125.147.88" "100.86.111.37" "100.95.0.72" "100.113.226.111"
  "100.114.244.62" "100.125.251.60" "100.74.204.23" "100.124.117.73"
  "100.89.102.75" "100.74.84.28" "100.71.242.104" "100.102.128.51"
)

echo "Checking all validators for sync stall..."
echo ""

declare -A block_counts

for ip in "${VALIDATORS[@]}"; do
  BLOCK=$(curl -s -m 2 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
    http://$ip:9944 2>/dev/null | \
    python3 -c "import sys, json; print(int(json.load(sys.stdin)['result']['block']['header']['number'], 16))" 2>/dev/null)

  if [ -n "$BLOCK" ]; then
    block_counts[$BLOCK]=$((${block_counts[$BLOCK]:-0} + 1))
  fi
done

echo "Block distribution:"
for block in "${!block_counts[@]}"; do
  count=${block_counts[$block]}
  echo "  Block $block: $count validators"
done

echo ""
echo "If all validators are at the same block with 0.0 bps, the chain is stalled."
