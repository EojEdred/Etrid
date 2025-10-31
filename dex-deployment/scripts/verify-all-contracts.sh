#!/bin/bash
################################################################################
# Verify All ÉTR Token Contracts on Block Explorers
#
# This script verifies deployed contracts on all supported block explorers
################################################################################

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║       VERIFY ALL ÉTR CONTRACTS ON BLOCK EXPLORERS         ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to verify a contract
verify_contract() {
  local chain=$1
  local network=$2
  local contract_address=$3
  local deployer_address=$4
  local contract_dir=$5

  echo -e "${BLUE}Verifying $chain contract...${NC}"
  cd "$contract_dir"

  if npx hardhat verify --network "$network" "$contract_address" "$deployer_address" 2>&1 | tee /tmp/verify.log; then
    if grep -q "Successfully verified" /tmp/verify.log || grep -q "Already Verified" /tmp/verify.log; then
      echo -e "${GREEN}✅ $chain verification successful!${NC}"
      return 0
    fi
  fi

  echo -e "${YELLOW}⚠️  $chain verification failed (may already be verified)${NC}"
  return 1
}

# Load deployment addresses
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEX_DIR="$(dirname "$SCRIPT_DIR")"

echo "📋 Loading deployment addresses..."
echo ""

# BSC
echo "═══ BSC (PancakeSwap) ═══"
if [ -f "$DEX_DIR/bsc/deployments/bscMainnet-deployment.json" ]; then
  BSC_ADDRESS=$(jq -r '.contractAddress' "$DEX_DIR/bsc/deployments/bscMainnet-deployment.json")
  BSC_DEPLOYER=$(jq -r '.deployerAddress' "$DEX_DIR/bsc/deployments/bscMainnet-deployment.json")
  echo "Contract: $BSC_ADDRESS"
  echo "Deployer: $BSC_DEPLOYER"
  verify_contract "BSC" "bscMainnet" "$BSC_ADDRESS" "$BSC_DEPLOYER" "$DEX_DIR/bsc"
else
  echo -e "${YELLOW}⚠️  BSC deployment not found${NC}"
fi
echo ""

# Ethereum
echo "═══ Ethereum (Uniswap) ═══"
if [ -f "$DEX_DIR/ethereum/deployments/mainnet-deployment.json" ]; then
  ETH_ADDRESS=$(jq -r '.contractAddress' "$DEX_DIR/ethereum/deployments/mainnet-deployment.json")
  ETH_DEPLOYER=$(jq -r '.deployerAddress' "$DEX_DIR/ethereum/deployments/mainnet-deployment.json")
  echo "Contract: $ETH_ADDRESS"
  echo "Deployer: $ETH_DEPLOYER"
  verify_contract "Ethereum" "mainnet" "$ETH_ADDRESS" "$ETH_DEPLOYER" "$DEX_DIR/ethereum"
else
  echo -e "${YELLOW}⚠️  Ethereum deployment not found${NC}"
fi
echo ""

# Polygon
echo "═══ Polygon (QuickSwap) ═══"
if [ -f "$DEX_DIR/polygon/deployments/polygon-deployment.json" ]; then
  POLY_ADDRESS=$(jq -r '.contractAddress' "$DEX_DIR/polygon/deployments/polygon-deployment.json")
  POLY_DEPLOYER=$(jq -r '.deployerAddress' "$DEX_DIR/polygon/deployments/polygon-deployment.json")
  echo "Contract: $POLY_ADDRESS"
  echo "Deployer: $POLY_DEPLOYER"
  verify_contract "Polygon" "polygon" "$POLY_ADDRESS" "$POLY_DEPLOYER" "$DEX_DIR/polygon"
else
  echo -e "${YELLOW}⚠️  Polygon deployment not found${NC}"
fi
echo ""

# Solana (SPL tokens don't need verification, but we can display info)
echo "═══ Solana (Raydium) ═══"
if [ -f "$DEX_DIR/solana/deployments/solana-deployment.json" ]; then
  SOL_ADDRESS=$(jq -r '.tokenMint' "$DEX_DIR/solana/deployments/solana-deployment.json")
  echo "Token Mint: $SOL_ADDRESS"
  echo -e "${GREEN}✅ Solana SPL tokens auto-verified on Solana Explorer${NC}"
else
  echo -e "${YELLOW}⚠️  Solana deployment not found${NC}"
fi
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║                 VERIFICATION COMPLETE! ✅                  ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "🔍 View verified contracts:"
echo ""
echo "BSC:      https://bscscan.com/address/$BSC_ADDRESS"
echo "Ethereum: https://etherscan.io/address/$ETH_ADDRESS"
echo "Polygon:  https://polygonscan.com/address/$POLY_ADDRESS"
echo "Solana:   https://solscan.io/token/$SOL_ADDRESS"
echo ""
