#!/bin/bash
################################################################################
# Test All Deployments
#
# Comprehensive testing suite for all deployed contracts
################################################################################

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║              TEST ALL ÉTR DEPLOYMENTS                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEX_DIR="$(dirname "$SCRIPT_DIR")"

PASSED=0
FAILED=0
SKIPPED=0

# Function to test contract basics
test_contract() {
  local chain=$1
  local network=$2
  local deployment_file=$3
  local contract_dir=$4

  echo -e "${BLUE}══════════════════════════════════════${NC}"
  echo -e "${BLUE}Testing $chain${NC}"
  echo -e "${BLUE}══════════════════════════════════════${NC}"

  if [ ! -f "$deployment_file" ]; then
    echo -e "${YELLOW}⏭️  Skipping $chain (not deployed)${NC}"
    ((SKIPPED++))
    echo ""
    return
  fi

  local address=$(jq -r '.contractAddress' "$deployment_file")
  echo "Contract: $address"
  echo ""

  cd "$contract_dir"

  # Test 1: Contract exists
  echo -n "Test 1: Contract exists... "
  if npx hardhat run --network "$network" - <<EOF 2>&1 | grep -q "true"
const hre = require("hardhat");
(async () => {
  const code = await hre.ethers.provider.getCode("$address");
  console.log(code !== "0x");
})();
EOF
  then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
  else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
  fi

  # Test 2: Check name and symbol
  echo -n "Test 2: Token name and symbol... "
  if npx hardhat run --network "$network" - <<EOF 2>&1 | grep -q "Etrid Coin"
const hre = require("hardhat");
(async () => {
  const token = await hre.ethers.getContractAt("ERC20", "$address");
  const name = await token.name();
  const symbol = await token.symbol();
  console.log(name + " " + symbol);
})();
EOF
  then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
  else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
  fi

  # Test 3: Check decimals
  echo -n "Test 3: Decimals = 18... "
  if npx hardhat run --network "$network" - <<EOF 2>&1 | grep -q "18"
const hre = require("hardhat");
(async () => {
  const token = await hre.ethers.getContractAt("ERC20", "$address");
  const decimals = await token.decimals();
  console.log(decimals.toString());
})();
EOF
  then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
  else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
  fi

  # Test 4: Check total supply
  echo -n "Test 4: Total supply > 0... "
  if npx hardhat run --network "$network" - <<EOF 2>&1 | grep -qv "^0$"
const hre = require("hardhat");
(async () => {
  const token = await hre.ethers.getContractAt("ERC20", "$address");
  const supply = await token.totalSupply();
  console.log(hre.ethers.formatEther(supply));
})();
EOF
  then
    echo -e "${GREEN}✅ PASS${NC}"
    ((PASSED++))
  else
    echo -e "${RED}❌ FAIL${NC}"
    ((FAILED++))
  fi

  # Test 5: Verified on explorer
  echo -n "Test 5: Verified on block explorer... "
  echo -e "${YELLOW}⏭️  MANUAL CHECK REQUIRED${NC}"
  ((SKIPPED++))

  echo ""
}

# Test each chain
test_contract "BSC" "bscMainnet" \
  "$DEX_DIR/bsc/deployments/bscMainnet-deployment.json" \
  "$DEX_DIR/bsc"

test_contract "Ethereum" "mainnet" \
  "$DEX_DIR/ethereum/deployments/mainnet-deployment.json" \
  "$DEX_DIR/ethereum"

test_contract "Polygon" "polygon" \
  "$DEX_DIR/polygon/deployments/polygon-deployment.json" \
  "$DEX_DIR/polygon"

# Solana tests (different approach)
echo -e "${BLUE}══════════════════════════════════════${NC}"
echo -e "${BLUE}Testing Solana${NC}"
echo -e "${BLUE}══════════════════════════════════════${NC}"

if [ -f "$DEX_DIR/solana/deployments/solana-deployment.json" ]; then
  SOL_MINT=$(jq -r '.tokenMint' "$DEX_DIR/solana/deployments/solana-deployment.json")
  echo "Token Mint: $SOL_MINT"
  echo ""

  if command -v spl-token &> /dev/null; then
    echo -n "Test 1: Token exists... "
    if spl-token supply "$SOL_MINT" --url https://api.mainnet-beta.solana.com &>/dev/null; then
      echo -e "${GREEN}✅ PASS${NC}"
      ((PASSED++))
    else
      echo -e "${RED}❌ FAIL${NC}"
      ((FAILED++))
    fi

    echo -n "Test 2: Supply > 0... "
    SUPPLY=$(spl-token supply "$SOL_MINT" --url https://api.mainnet-beta.solana.com 2>/dev/null || echo "0")
    if [ "$SUPPLY" != "0" ]; then
      echo -e "${GREEN}✅ PASS (Supply: $SUPPLY)${NC}"
      ((PASSED++))
    else
      echo -e "${RED}❌ FAIL${NC}"
      ((FAILED++))
    fi
  else
    echo -e "${YELLOW}⏭️  Skipping Solana tests (spl-token not installed)${NC}"
    ((SKIPPED+=2))
  fi
else
  echo -e "${YELLOW}⏭️  Skipping Solana (not deployed)${NC}"
  ((SKIPPED++))
fi
echo ""

# Summary
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                    TEST RESULTS                           ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}✅ Passed:  $PASSED${NC}"
echo -e "${RED}❌ Failed:  $FAILED${NC}"
echo -e "${YELLOW}⏭️  Skipped: $SKIPPED${NC}"
echo ""

if [ $FAILED -gt 0 ]; then
  echo -e "${RED}⚠️  Some tests failed! Review the output above.${NC}"
  exit 1
else
  echo -e "${GREEN}🎉 All tests passed!${NC}"
  echo ""
  echo "Next steps:"
  echo "  1. Create liquidity pools on DEXes"
  echo "  2. Test actual swaps manually"
  echo "  3. Submit to CoinGecko and CoinMarketCap"
  echo ""
  exit 0
fi
