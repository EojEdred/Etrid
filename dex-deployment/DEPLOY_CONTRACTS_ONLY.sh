#!/bin/bash
################################################################################
# ËTRID CONTRACTS-ONLY DEPLOYMENT
#
# Deploys token contracts WITHOUT creating liquidity pools
# Cost: $15.50 (gas only, no liquidity needed)
#
# Use this when:
# - You want tokens on-chain but aren't ready for trading
# - Building community/holder base first
# - Waiting for more funds to add liquidity later
################################################################################

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}"
cat << 'EOF'
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║     ËTRID CONTRACTS-ONLY DEPLOYMENT                       ║
║                                                            ║
║     Deploy contracts • No liquidity pools • $15.50        ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}\n"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Explain what this does
echo -e "${BLUE}What this script does:${NC}\n"
echo "✅ Deploys ÉTR token contracts to 3 chains"
echo "✅ Verifies all contracts on block explorers"
echo "✅ Mints 100K ÉTR on each chain"
echo ""
echo "❌ Does NOT create liquidity pools"
echo "❌ Does NOT enable trading"
echo ""
echo -e "${YELLOW}You can add liquidity later when you're ready!${NC}\n"

# Budget
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  COST BREAKDOWN                                            ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo "Gas Fees:"
echo "  • Polygon deploy:     $5"
echo "  • BSC deploy:         $6"
echo "  • Solana deploy:      $4.50"
echo "  ─────────────────────────"
echo -e "  ${GREEN}TOTAL:                $15.50${NC}"
echo ""
echo "Liquidity: $0 (not creating pools)"
echo ""

# Confirm
echo -e "${YELLOW}⚠️  This will spend REAL money for gas fees!${NC}\n"
read -p "Continue with contracts-only deployment? (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
  echo "Deployment cancelled."
  exit 0
fi

# Check env files
echo -e "\n${BLUE}📋 Pre-flight checks...${NC}\n"

if [ ! -f "polygon/.env" ]; then
  echo -e "${RED}❌ polygon/.env not found${NC}"
  echo "   Run: cp polygon/.env.example polygon/.env"
  exit 1
fi

if [ ! -f "bsc/.env" ]; then
  echo -e "${RED}❌ bsc/.env not found${NC}"
  echo "   Run: cp bsc/.env.example bsc/.env"
  exit 1
fi

echo -e "${GREEN}✅ Pre-flight checks passed${NC}\n"

# Array to store deployment info
declare -A ADDRESSES

# Deploy to Polygon
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  STEP 1/3: DEPLOY TO POLYGON                               ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

cd polygon
if [ ! -d "node_modules" ]; then
  echo "Installing dependencies..."
  npm install --silent
fi

echo "Deploying EtridPoly to Polygon mainnet..."
npm run deploy:mainnet

if [ $? -eq 0 ]; then
  echo -e "\n${GREEN}✅ Polygon deployment successful!${NC}"

  POLY_ADDRESS=$(ls -t deployments/ | head -1 | xargs -I {} cat deployments/{} | jq -r '.tokenAddress')
  ADDRESSES["polygon"]=$POLY_ADDRESS
  echo "Contract: $POLY_ADDRESS"
  echo "Explorer: https://polygonscan.com/address/$POLY_ADDRESS"
  echo "Supply: 100,000 ÉTR"
else
  echo -e "\n${RED}❌ Polygon deployment failed!${NC}"
  exit 1
fi

cd ..

# Deploy to BSC
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  STEP 2/3: DEPLOY TO BSC                                   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

cd bsc
if [ ! -d "node_modules" ]; then
  echo "Installing dependencies..."
  npm install --silent
fi

echo "Deploying EtridBSC to BSC mainnet..."
npm run deploy:mainnet

if [ $? -eq 0 ]; then
  echo -e "\n${GREEN}✅ BSC deployment successful!${NC}"

  BSC_ADDRESS=$(ls -t deployments/ | head -1 | xargs -I {} cat deployments/{} | jq -r '.tokenAddress')
  ADDRESSES["bsc"]=$BSC_ADDRESS
  echo "Contract: $BSC_ADDRESS"
  echo "Explorer: https://bscscan.com/address/$BSC_ADDRESS"
  echo "Supply: 100,000 ÉTR"
else
  echo -e "\n${YELLOW}⚠️  BSC deployment failed, continuing...${NC}"
fi

cd ..

# Deploy to Solana
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  STEP 3/3: DEPLOY TO SOLANA                                ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

if command -v solana &> /dev/null; then
  cd solana

  echo "Deploying ÉTR SPL token to Solana mainnet..."
  ./deploy-solana.sh

  if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}✅ Solana deployment successful!${NC}"

    if [ -f "deployments/solana-deployment.json" ]; then
      SOL_MINT=$(jq -r '.tokenMint' deployments/solana-deployment.json)
      ADDRESSES["solana"]=$SOL_MINT
      echo "Token Mint: $SOL_MINT"
      echo "Explorer: https://solscan.io/token/$SOL_MINT"
      echo "Supply: 100,000 ÉTR"
    fi
  else
    echo -e "\n${YELLOW}⚠️  Solana deployment failed, continuing...${NC}"
  fi

  cd ..
else
  echo -e "${YELLOW}⚠️  Skipping Solana (CLI not installed)${NC}"
fi

# Final report
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                 DEPLOYMENT COMPLETE! ✅                     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}Contracts deployed (ready to use):${NC}\n"

if [ ! -z "${ADDRESSES[polygon]}" ]; then
  echo "✅ Polygon:"
  echo "   Contract: ${ADDRESSES[polygon]}"
  echo "   Explorer: https://polygonscan.com/address/${ADDRESSES[polygon]}"
  echo "   Supply: 100,000 ÉTR"
  echo ""
fi

if [ ! -z "${ADDRESSES[bsc]}" ]; then
  echo "✅ BSC:"
  echo "   Contract: ${ADDRESSES[bsc]}"
  echo "   Explorer: https://bscscan.com/address/${ADDRESSES[bsc]}"
  echo "   Supply: 100,000 ÉTR"
  echo ""
fi

if [ ! -z "${ADDRESSES[solana]}" ]; then
  echo "✅ Solana:"
  echo "   Token Mint: ${ADDRESSES[solana]}"
  echo "   Explorer: https://solscan.io/token/${ADDRESSES[solana]}"
  echo "   Supply: 100,000 ÉTR"
  echo ""
fi

echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"

echo -e "${GREEN}What you can do NOW:${NC}"
echo ""
echo "✅ Transfer ÉTR between addresses"
echo "✅ Add ÉTR to MetaMask/Phantom"
echo "✅ Airdrop to community"
echo "✅ Pay team/validators"
echo "✅ Integrate with other contracts"
echo ""

echo -e "${YELLOW}What you CANNOT do yet:${NC}"
echo ""
echo "❌ Trade on DEXes (no pools created)"
echo "❌ Provide liquidity (no pools)"
echo "❌ List on CoinGecko (needs trading activity)"
echo ""

echo -e "${BLUE}When you're ready to enable trading:${NC}\n"

echo "1. Accumulate liquidity funds ($500-$10,000)"
echo ""
echo "2. Create pools on DEXes:"
echo ""

if [ ! -z "${ADDRESSES[polygon]}" ]; then
  echo "   Polygon (QuickSwap):"
  echo "   • Go to: https://quickswap.exchange/#/pools"
  echo "   • Token A: ${ADDRESSES[polygon]}"
  echo "   • Token B: WMATIC"
  echo "   • Add liquidity: e.g., 50K ÉTR + $5,000 MATIC"
  echo ""
fi

if [ ! -z "${ADDRESSES[bsc]}" ]; then
  echo "   BSC (PancakeSwap):"
  echo "   • Go to: https://pancakeswap.finance/liquidity"
  echo "   • Token A: ${ADDRESSES[bsc]}"
  echo "   • Token B: WBNB"
  echo "   • Add liquidity: e.g., 50K ÉTR + $5,000 BNB"
  echo ""
fi

echo "3. Trading is then LIVE! 🎉"
echo ""

echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"

echo -e "${GREEN}Cost summary:${NC}"
echo "  Gas fees: ~$15.50 ✅"
echo "  Liquidity: $0 (pools not created)"
echo "  ────────────────────"
echo "  Total spent: ~$15.50"
echo ""

echo -e "${BLUE}💡 Tip:${NC} This approach lets you:"
echo "  • Build community first"
echo "  • Distribute tokens before trading"
echo "  • Wait for proper liquidity"
echo "  • Launch trading when ready (days/weeks/months later)"
echo ""

echo "Good luck with your launch! 🚀"
echo ""
