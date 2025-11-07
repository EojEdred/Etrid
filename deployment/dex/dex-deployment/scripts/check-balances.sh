#!/bin/bash
################################################################################
# Check Balances Across All Chains
#
# Quick script to check if you have sufficient funds for deployment
################################################################################

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║            CHECK DEPLOYMENT FUND BALANCES                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Load private key from env
if [ -z "$PRIVATE_KEY" ]; then
  echo -e "${YELLOW}⚠️  PRIVATE_KEY not set. Please export it first.${NC}"
  echo "   export PRIVATE_KEY=your_private_key_here"
  echo ""
  echo "Or use .env files in each deployment directory."
  exit 1
fi

# Function to check balance using cast (Foundry)
check_eth_balance() {
  local chain=$1
  local rpc=$2
  local min_balance=$3
  local currency=$4

  echo -e "═══ $chain ═══"

  if ! command -v cast &> /dev/null; then
    echo -e "${YELLOW}⚠️  'cast' not found. Install Foundry: https://getfoundry.sh${NC}"
    echo "   Skipping $chain balance check."
    echo ""
    return
  fi

  # Derive address from private key
  ADDRESS=$(cast wallet address --private-key "$PRIVATE_KEY")
  echo "Address: $ADDRESS"

  # Get balance
  BALANCE=$(cast balance --rpc-url "$rpc" "$ADDRESS")
  BALANCE_ETH=$(cast --from-wei "$BALANCE")

  echo "Balance: $BALANCE_ETH $currency"
  echo "Minimum required: $min_balance $currency"

  # Compare balances (basic)
  if (( $(echo "$BALANCE_ETH >= $min_balance" | bc -l) )); then
    echo -e "${GREEN}✅ Sufficient funds${NC}"
  else
    echo -e "${RED}❌ Insufficient funds! Need at least $min_balance $currency${NC}"
  fi
  echo ""
}

# Check BSC
check_eth_balance "BSC Mainnet" "https://bsc-dataseed1.binance.org" "0.05" "BNB"

# Check Ethereum
check_eth_balance "Ethereum Mainnet" "https://eth.llamarpc.com" "0.1" "ETH"

# Check Polygon
check_eth_balance "Polygon Mainnet" "https://polygon-rpc.com" "10" "MATIC"

# Check Solana
echo "═══ Solana Mainnet ═══"
if command -v solana &> /dev/null; then
  SOL_ADDRESS=$(solana-keygen pubkey ~/.config/solana/id.json 2>/dev/null || echo "keypair not found")
  echo "Address: $SOL_ADDRESS"

  if [ "$SOL_ADDRESS" != "keypair not found" ]; then
    SOL_BALANCE=$(solana balance --url https://api.mainnet-beta.solana.com "$SOL_ADDRESS" 2>/dev/null || echo "0 SOL")
    echo "Balance: $SOL_BALANCE"
    echo "Minimum required: 0.05 SOL"

    if [[ "$SOL_BALANCE" == *"0 SOL"* ]] || [[ "$SOL_BALANCE" == *"Error"* ]]; then
      echo -e "${RED}❌ Insufficient funds!${NC}"
    else
      echo -e "${GREEN}✅ Sufficient funds${NC}"
    fi
  else
    echo -e "${YELLOW}⚠️  Solana keypair not found at ~/.config/solana/id.json${NC}"
  fi
else
  echo -e "${YELLOW}⚠️  'solana' CLI not found. Install: https://docs.solana.com/cli/install-solana-cli-tools${NC}"
fi
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                  BALANCE CHECK COMPLETE                   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "💡 To add funds:"
echo ""
echo "BSC:      Buy BNB on Binance, send to your address"
echo "Ethereum: Buy ETH on Coinbase/Binance, send to your address"
echo "Polygon:  Bridge from Ethereum: https://portal.polygon.technology/"
echo "Solana:   Buy SOL on Coinbase/Binance, send to your address"
echo ""
echo "⚠️  IMPORTANT: Use the same address across all EVM chains (BSC, ETH, Polygon)"
echo "   But use a separate Solana keypair for Solana deployments"
echo ""
