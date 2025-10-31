#!/bin/bash
################################################################################
# ËTRID $50 BUDGET DEPLOYMENT
#
# Deploys ÉTR to DEXes with realistic $50 budget
#
# What this does:
# 1. Deploys token contracts to 3 chains ($15.50)
# 2. Creates ONE liquidity pool on Polygon ($34.50)
# 3. Total cost: $50
#
# Prerequisites:
# - Polygon: 10 MATIC ($10) + 34 MATIC for liquidity ($34)
# - BSC: 0.02 BNB ($6)
# - Solana: 0.05 SOL ($7.50)
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
║     ËTRID $50 BUDGET DEX DEPLOYMENT                       ║
║                                                            ║
║     Bootstrap deployment with realistic budget            ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}\n"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check if .env files exist
echo -e "${BLUE}📋 Pre-flight checks...${NC}\n"

CHECKS_PASSED=true

if [ ! -f "polygon/.env" ]; then
  echo -e "${RED}❌ polygon/.env not found${NC}"
  echo "   Run: cp polygon/.env.example polygon/.env"
  CHECKS_PASSED=false
fi

if [ ! -f "bsc/.env" ]; then
  echo -e "${RED}❌ bsc/.env not found${NC}"
  echo "   Run: cp bsc/.env.example bsc/.env"
  CHECKS_PASSED=false
fi

if ! command -v solana &> /dev/null; then
  echo -e "${YELLOW}⚠️  Solana CLI not installed (optional for this deployment)${NC}"
fi

if [ "$CHECKS_PASSED" = false ]; then
  echo -e "\n${RED}Pre-flight checks failed. Fix the issues above and try again.${NC}\n"
  exit 1
fi

echo -e "${GREEN}✅ Pre-flight checks passed${NC}\n"

# Display budget breakdown
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  BUDGET BREAKDOWN                                          ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo "Deployment Costs (Gas):"
echo "  • Polygon deploy:     $5"
echo "  • BSC deploy:         $6"
echo "  • Solana deploy:      $4.50"
echo "  ─────────────────────────"
echo "  Subtotal:             $15.50"
echo ""
echo "Liquidity (Polygon only):"
echo "  • ÉTR amount:         50,000 ÉTR"
echo "  • MATIC amount:       34 MATIC (~$34)"
echo "  • Pool creation gas:  $0.50"
echo "  ─────────────────────────"
echo "  Subtotal:             $34.50"
echo ""
echo -e "${GREEN}TOTAL:                  $50.00${NC}"
echo ""

# Confirm
echo -e "${YELLOW}⚠️  WARNING: This will spend REAL money!${NC}"
echo ""
echo "This script will:"
echo "  1. Deploy ÉTR contract to Polygon (~$5)"
echo "  2. Deploy ÉTR contract to BSC (~$6)"
echo "  3. Deploy ÉTR SPL token to Solana (~$4.50)"
echo "  4. Create liquidity pool on Polygon (~$34.50)"
echo ""
echo "Each contract will mint 100,000 ÉTR (bootstrap amount)"
echo "You'll use 50,000 ÉTR for Polygon liquidity"
echo ""
read -p "Continue? (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
  echo "Deployment cancelled."
  exit 0
fi

# Step 1: Deploy to Polygon
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  STEP 1/4: DEPLOY TO POLYGON                               ║${NC}"
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

  # Extract contract address from latest deployment
  POLY_ADDRESS=$(ls -t deployments/ | head -1 | xargs -I {} cat deployments/{} | jq -r '.tokenAddress')
  echo "Contract address: $POLY_ADDRESS"
  echo ""

  # Save for later
  echo "$POLY_ADDRESS" > /tmp/etrid_polygon_address.txt
else
  echo -e "\n${RED}❌ Polygon deployment failed!${NC}"
  exit 1
fi

cd ..

# Step 2: Deploy to BSC
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  STEP 2/4: DEPLOY TO BSC                                   ║${NC}"
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
  echo "Contract address: $BSC_ADDRESS"
  echo ""

  echo "$BSC_ADDRESS" > /tmp/etrid_bsc_address.txt
else
  echo -e "\n${RED}❌ BSC deployment failed!${NC}"
  echo "Continuing anyway... (you have Polygon deployed)"
fi

cd ..

# Step 3: Deploy to Solana
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  STEP 3/4: DEPLOY TO SOLANA                                ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

if command -v solana &> /dev/null; then
  cd solana
  echo "Deploying ÉTR SPL token to Solana mainnet..."

  # Run Solana deployment (it's interactive, will prompt for mainnet)
  ./deploy-solana.sh

  if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}✅ Solana deployment successful!${NC}"
    echo ""
  else
    echo -e "\n${RED}❌ Solana deployment failed!${NC}"
    echo "Continuing anyway..."
  fi

  cd ..
else
  echo -e "${YELLOW}⚠️  Skipping Solana (CLI not installed)${NC}"
  echo "   You have Polygon and BSC deployed - that's enough!"
  echo ""
fi

# Step 4: Create Polygon Pool
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  STEP 4/4: CREATE POLYGON LIQUIDITY POOL                   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

if [ -f /tmp/etrid_polygon_address.txt ]; then
  POLY_ADDRESS=$(cat /tmp/etrid_polygon_address.txt)

  echo "Now you need to create the liquidity pool manually:"
  echo ""
  echo "1. Go to: https://quickswap.exchange/#/pools"
  echo ""
  echo "2. Connect MetaMask (Polygon network)"
  echo ""
  echo "3. Add ÉTR token to MetaMask:"
  echo "   • Click 'Import Token'"
  echo "   • Paste: $POLY_ADDRESS"
  echo "   • Should show: 100,000 ÉTR"
  echo ""
  echo "4. Make sure you have 34 MATIC in your wallet"
  echo ""
  echo "5. Create pool:"
  echo "   • Click 'Create Pool'"
  echo "   • Token A: $POLY_ADDRESS (ÉTR)"
  echo "   • Token B: WMATIC (auto-fills)"
  echo "   • Fee: 0.30%"
  echo "   • Amount: 50,000 ÉTR + 34 MATIC"
  echo "   • Approve and confirm"
  echo ""
  echo "6. Test a small swap to verify it works"
  echo ""

  read -p "Press Enter when you've created the pool..."

else
  echo -e "${RED}❌ Polygon address not found. Cannot provide pool instructions.${NC}"
fi

# Final summary
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                 DEPLOYMENT COMPLETE! ✅                     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}What you've deployed:${NC}\n"

if [ -f /tmp/etrid_polygon_address.txt ]; then
  echo "✅ Polygon:"
  echo "   Contract: $(cat /tmp/etrid_polygon_address.txt)"
  echo "   Explorer: https://polygonscan.com/address/$(cat /tmp/etrid_polygon_address.txt)"
  echo "   Supply: 100,000 ÉTR"
  echo ""
fi

if [ -f /tmp/etrid_bsc_address.txt ]; then
  echo "✅ BSC:"
  echo "   Contract: $(cat /tmp/etrid_bsc_address.txt)"
  echo "   Explorer: https://bscscan.com/address/$(cat /tmp/etrid_bsc_address.txt)"
  echo "   Supply: 100,000 ÉTR"
  echo ""
fi

if [ -d "solana/deployments" ]; then
  echo "✅ Solana:"
  echo "   Check: solana/deployments/ for token mint address"
  echo "   Supply: 100,000 ÉTR"
  echo ""
fi

echo -e "${GREEN}Liquidity pool (if created):${NC}"
echo "   ✅ Polygon: 50,000 ÉTR + 34 MATIC on QuickSwap"
echo ""

echo -e "${YELLOW}Next steps:${NC}"
echo ""
echo "1. Submit to CoinGecko:"
echo "   https://www.coingecko.com/en/coins/new"
echo ""
echo "2. Submit to CoinMarketCap:"
echo "   https://coinmarketcap.com/request/"
echo ""
echo "3. Announce on social media"
echo ""
echo "4. Add more liquidity when you have more funds"
echo ""
echo "5. Build bridge to connect with FlareChain mainnet"
echo ""

echo -e "${CYAN}Expected Results:${NC}"
echo ""
echo "⚠️  With $34 liquidity:"
echo "   • Pool works but has HIGH slippage"
echo "   • Good for demo/testing"
echo "   • Trades >$10 will have 30-50% price impact"
echo "   • Add more liquidity ASAP for better experience"
echo ""

echo -e "${GREEN}Total spent: ~$50${NC}"
echo ""
echo "Good luck with your launch! 🚀"
echo ""

# Cleanup
rm -f /tmp/etrid_polygon_address.txt
rm -f /tmp/etrid_bsc_address.txt
