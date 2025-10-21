#!/bin/bash
# Check relayer balances (ETH and EDSC)
# Alerts if balances are low

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuration
ETHEREUM_RPC=${ETHEREUM_RPC:-"https://eth-sepolia.g.alchemy.com/v2/YOUR-KEY"}
RELAYER_ADDRESSES=${RELAYER_ADDRESSES:-""}

# Thresholds
ETH_WARNING_THRESHOLD=0.1
ETH_CRITICAL_THRESHOLD=0.05
EDSC_WARNING_THRESHOLD=10
EDSC_CRITICAL_THRESHOLD=5

echo "======================================"
echo "  Relayer Balance Check"
echo "======================================"
echo ""

if [ -z "$RELAYER_ADDRESSES" ]; then
  echo -e "${YELLOW}No RELAYER_ADDRESSES configured${NC}"
  echo "Set RELAYER_ADDRESSES environment variable with comma-separated addresses"
  echo "Example: export RELAYER_ADDRESSES='0x123...,0x456...'"
  exit 1
fi

# Convert comma-separated string to array
IFS=',' read -ra ADDRESSES <<< "$RELAYER_ADDRESSES"

total_eth=0
total_edsc=0
low_balance_count=0

for address in "${ADDRESSES[@]}"; do
  # Trim whitespace
  address=$(echo "$address" | xargs)

  echo "Checking $address..."

  # Get ETH balance
  eth_hex=$(curl -s -X POST "$ETHEREUM_RPC" \
    -H "Content-Type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBalance\",\"params\":[\"$address\",\"latest\"],\"id\":1}" \
    2>/dev/null | jq -r '.result' || echo "error")

  if [ "$eth_hex" != "error" ] && [ "$eth_hex" != "null" ]; then
    # Convert hex to decimal
    eth_wei=$((16#${eth_hex#0x}))
    # Convert to ETH (divide by 10^18)
    eth_balance=$(echo "scale=4; $eth_wei / 1000000000000000000" | bc)

    echo "  ETH: $eth_balance"

    # Check thresholds
    if (( $(echo "$eth_balance < $ETH_CRITICAL_THRESHOLD" | bc -l) )); then
      echo -e "  ${RED}CRITICAL: ETH balance below $ETH_CRITICAL_THRESHOLD!${NC}"
      low_balance_count=$((low_balance_count + 1))
    elif (( $(echo "$eth_balance < $ETH_WARNING_THRESHOLD" | bc -l) )); then
      echo -e "  ${YELLOW}WARNING: ETH balance below $ETH_WARNING_THRESHOLD${NC}"
      low_balance_count=$((low_balance_count + 1))
    else
      echo -e "  ${GREEN}✓ ETH balance OK${NC}"
    fi

    total_eth=$(echo "$total_eth + $eth_balance" | bc)
  else
    echo -e "  ${RED}✗ Could not fetch ETH balance${NC}"
  fi

  echo ""
done

echo "======================================"
echo "  Summary"
echo "======================================"
echo "Total ETH across all relayers: $total_eth"

if [ $low_balance_count -gt 0 ]; then
  echo -e "${RED}$low_balance_count relayer(s) need funding!${NC}"
  echo ""
  echo "To fund relayers:"
  echo "1. Send ETH to low-balance addresses"
  echo "2. Recommended: 0.5 ETH per relayer for Ember testnet"
  exit 1
else
  echo -e "${GREEN}All relayer balances are sufficient${NC}"
  exit 0
fi
