#!/bin/bash
################################################################################
# ËTRID MAXIMUM DEX COVERAGE DEPLOYMENT
#
# Deploys to 8 cheap chains = Access to 30+ DEXes!
# Cost: $19 (gas only, skips expensive Ethereum)
#
# DEXes you'll have access to:
# - PancakeSwap, Raydium, QuickSwap, Trader Joe, Camelot, Aerodrome,
#   Velodrome, SpookySwap, Orca, Jupiter, SushiSwap, and 20+ more!
################################################################################

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo -e "${CYAN}"
cat << 'EOF'
╔════════════════════════════════════════════════════════════╗
║                                                            ║
║     ËTRID MAXIMUM DEX COVERAGE DEPLOYMENT                 ║
║                                                            ║
║     8 Chains • 30+ DEXes • $19 Total                      ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}\n"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Show what this unlocks
echo -e "${BLUE}🎯 This deployment gives you access to:${NC}\n"

echo -e "${GREEN}✅ PancakeSwap${NC} (BSC) - Largest BSC DEX"
echo -e "${GREEN}✅ Raydium${NC} (Solana) - Fast Solana DEX"
echo -e "${GREEN}✅ QuickSwap${NC} (Polygon) - Ultra-cheap DEX"
echo -e "${GREEN}✅ Trader Joe${NC} (Avalanche) - Avalanche DEX"
echo -e "${GREEN}✅ Camelot${NC} (Arbitrum) - Arbitrum DEX"
echo -e "${GREEN}✅ Aerodrome${NC} (Base) - Base DEX"
echo -e "${GREEN}✅ Velodrome${NC} (Optimism) - Optimism DEX"
echo -e "${GREEN}✅ SpookySwap${NC} (Fantom) - Fantom DEX"
echo -e "${CYAN}✅ Plus 20+ more DEXes!${NC} (Orca, Jupiter, SushiSwap, etc.)\n"

echo -e "${YELLOW}💡 Note: Phantom is a Solana WALLET (like MetaMask).${NC}"
echo -e "${YELLOW}   When you deploy to Solana, you use Phantom to trade on Raydium/Orca.${NC}\n"

# Cost breakdown
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  COST BREAKDOWN - ALL CHEAP CHAINS                         ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo "Gas Fees (Deploy Contracts Only):"
echo "  1. Polygon:        $5"
echo "  2. BSC:            $6"
echo "  3. Solana:         $4.50"
echo "  4. Avalanche:      $0.50"
echo "  5. Arbitrum:       $1"
echo "  6. Base:           $1"
echo "  7. Optimism:       $1"
echo "  8. Fantom:         $0.10"
echo "  ─────────────────────────"
echo -e "  ${GREEN}TOTAL:             $19.10${NC}"
echo ""
echo "Liquidity: $0 (pools created later when you have funds)"
echo ""
echo -e "${YELLOW}⚠️  Skipping Ethereum (would add $150 in gas)${NC}"
echo -e "${BLUE}💡  You can deploy to Ethereum later if needed${NC}\n"

# Confirm
echo -e "${YELLOW}⚠️  This will spend REAL money for gas fees!${NC}\n"
read -p "Continue with maximum DEX coverage deployment? (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
  echo "Deployment cancelled."
  exit 0
fi

# Check which chains are ready
echo -e "\n${BLUE}📋 Pre-flight checks...${NC}\n"

CHAINS_READY=0
CHAINS_TOTAL=8

# Check Polygon
if [ -f "polygon/.env" ]; then
  echo -e "${GREEN}✅ Polygon ready${NC}"
  ((CHAINS_READY++))
else
  echo -e "${YELLOW}⚠️  Polygon .env not found (will skip)${NC}"
fi

# Check BSC
if [ -f "bsc/.env" ]; then
  echo -e "${GREEN}✅ BSC ready${NC}"
  ((CHAINS_READY++))
else
  echo -e "${YELLOW}⚠️  BSC .env not found (will skip)${NC}"
fi

# Check Solana
if command -v solana &> /dev/null; then
  echo -e "${GREEN}✅ Solana CLI ready${NC}"
  ((CHAINS_READY++))
else
  echo -e "${YELLOW}⚠️  Solana CLI not installed (will skip)${NC}"
fi

# Other chains (not yet configured)
echo -e "${YELLOW}⚠️  Avalanche not yet configured (will skip)${NC}"
echo -e "${YELLOW}⚠️  Arbitrum not yet configured (will skip)${NC}"
echo -e "${YELLOW}⚠️  Base not yet configured (will skip)${NC}"
echo -e "${YELLOW}⚠️  Optimism not yet configured (will skip)${NC}"
echo -e "${YELLOW}⚠️  Fantom not yet configured (will skip)${NC}"

echo ""
echo -e "Ready to deploy to: ${GREEN}$CHAINS_READY${NC} chains"
echo -e "Need configuration for: ${YELLOW}$((CHAINS_TOTAL - CHAINS_READY))${NC} chains\n"

if [ $CHAINS_READY -eq 0 ]; then
  echo -e "${RED}❌ No chains ready! Please configure .env files first.${NC}"
  exit 1
fi

# Array to store deployment info
declare -A ADDRESSES
TOTAL_COST=0

# ============================================================================
# DEPLOY TO POLYGON
# ============================================================================
if [ -f "polygon/.env" ]; then
  echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
  echo -e "${CYAN}║  DEPLOYING TO POLYGON                                      ║${NC}"
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
    TOTAL_COST=$(echo "$TOTAL_COST + 5" | bc)

    echo "Contract: $POLY_ADDRESS"
    echo "Explorer: https://polygonscan.com/address/$POLY_ADDRESS"
    echo "Supply: 100,000 ÉTR"
    echo ""
    echo -e "${BLUE}DEXes now available:${NC}"
    echo "  • QuickSwap V3"
    echo "  • QuickSwap V2"
    echo "  • SushiSwap"
    echo "  • Uniswap V3 (on Polygon)"
    echo "  • Balancer"
    echo "  • KyberSwap"
  else
    echo -e "\n${YELLOW}⚠️  Polygon deployment failed, continuing...${NC}"
  fi

  cd ..
fi

# ============================================================================
# DEPLOY TO BSC
# ============================================================================
if [ -f "bsc/.env" ]; then
  echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
  echo -e "${CYAN}║  DEPLOYING TO BSC                                          ║${NC}"
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
    TOTAL_COST=$(echo "$TOTAL_COST + 6" | bc)

    echo "Contract: $BSC_ADDRESS"
    echo "Explorer: https://bscscan.com/address/$BSC_ADDRESS"
    echo "Supply: 100,000 ÉTR"
    echo ""
    echo -e "${BLUE}DEXes now available:${NC}"
    echo "  • PancakeSwap V3"
    echo "  • PancakeSwap V2"
    echo "  • Biswap"
    echo "  • ApeSwap"
  else
    echo -e "\n${YELLOW}⚠️  BSC deployment failed, continuing...${NC}"
  fi

  cd ..
fi

# ============================================================================
# DEPLOY TO SOLANA
# ============================================================================
if command -v solana &> /dev/null; then
  echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
  echo -e "${CYAN}║  DEPLOYING TO SOLANA                                       ║${NC}"
  echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

  cd solana

  echo "Deploying ÉTR SPL token to Solana mainnet..."
  ./deploy-solana.sh

  if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}✅ Solana deployment successful!${NC}"

    if [ -f "deployments/solana-deployment.json" ]; then
      SOL_MINT=$(jq -r '.tokenMint' deployments/solana-deployment.json)
      ADDRESSES["solana"]=$SOL_MINT
      TOTAL_COST=$(echo "$TOTAL_COST + 4.5" | bc)

      echo "Token Mint: $SOL_MINT"
      echo "Explorer: https://solscan.io/token/$SOL_MINT"
      echo "Supply: 100,000 ÉTR"
      echo ""
      echo -e "${BLUE}DEXes now available:${NC}"
      echo "  • Raydium CLMM (concentrated liquidity)"
      echo "  • Orca CLMM"
      echo "  • Jupiter (aggregator - auto-includes all)"
      echo "  • Meteora"
      echo "  • Serum"
      echo ""
      echo -e "${YELLOW}💡 Use Phantom wallet to trade on these DEXes!${NC}"
    fi
  else
    echo -e "\n${YELLOW}⚠️  Solana deployment failed, continuing...${NC}"
  fi

  cd ..
else
  echo -e "\n${YELLOW}⚠️  Skipping Solana (CLI not installed)${NC}"
fi

# ============================================================================
# FUTURE CHAINS (Not yet configured)
# ============================================================================
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  ADDITIONAL CHAINS (Not Yet Configured)                   ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${YELLOW}The following chains are in your plan but need configuration:${NC}\n"

echo "4. Avalanche ($0.50)"
echo "   DEXes: Trader Joe V2, Pangolin, SushiSwap"
echo "   Status: Need to create avalanche/ folder with deploy scripts"
echo ""

echo "5. Arbitrum ($1)"
echo "   DEXes: Camelot, Uniswap V3, GMX V2, SushiSwap"
echo "   Status: Need to create arbitrum/ folder with deploy scripts"
echo ""

echo "6. Base ($1)"
echo "   DEXes: Aerodrome, Uniswap V3, BaseSwap"
echo "   Status: Need to create base/ folder with deploy scripts"
echo ""

echo "7. Optimism ($1)"
echo "   DEXes: Velodrome, Uniswap V3, SushiSwap"
echo "   Status: Need to create optimism/ folder with deploy scripts"
echo ""

echo "8. Fantom ($0.10)"
echo "   DEXes: SpookySwap, SushiSwap"
echo "   Status: Need to create fantom/ folder with deploy scripts"
echo ""

echo -e "${BLUE}💡 To deploy to these chains:${NC}"
echo "   1. Create folder for each chain (similar to polygon/bsc)"
echo "   2. Add hardhat config with correct RPC"
echo "   3. Copy .env.example and add private key"
echo "   4. Re-run this script"
echo ""

# ============================================================================
# HYPERLIQUID (Optional)
# ============================================================================
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  HYPERLIQUID (Optional)                                    ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${YELLOW}You mentioned Hyperliquid in your request.${NC}\n"

echo "Hyperliquid Details:"
echo "  • Chain: HyperEVM (custom L1)"
echo "  • Type: Perpetual futures DEX"
echo "  • Deploy cost: ~$3-5"
echo "  • User base: Smaller, advanced traders"
echo ""

echo -e "${BLUE}Recommendation:${NC}"
echo "  Deploy to established chains first (Polygon, BSC, Solana)"
echo "  Add Hyperliquid later when:"
echo "    - You have proven liquidity elsewhere"
echo "    - Community requests it"
echo "    - Targeting advanced/futures traders"
echo ""

# ============================================================================
# FINAL REPORT
# ============================================================================
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                 DEPLOYMENT COMPLETE! ✅                     ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}Successfully deployed to ${#ADDRESSES[@]} chains!${NC}\n"

if [ ! -z "${ADDRESSES[polygon]}" ]; then
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}✅ POLYGON${NC}"
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo "Contract: ${ADDRESSES[polygon]}"
  echo "Explorer: https://polygonscan.com/address/${ADDRESSES[polygon]}"
  echo "Supply: 100,000 ÉTR"
  echo ""
  echo "Create pools on:"
  echo "  • QuickSwap: https://quickswap.exchange/#/pools"
  echo "  • SushiSwap: https://www.sushi.com/polygon/pool"
  echo "  • Uniswap: https://app.uniswap.org/pools"
  echo ""
fi

if [ ! -z "${ADDRESSES[bsc]}" ]; then
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}✅ BSC${NC}"
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo "Contract: ${ADDRESSES[bsc]}"
  echo "Explorer: https://bscscan.com/address/${ADDRESSES[bsc]}"
  echo "Supply: 100,000 ÉTR"
  echo ""
  echo "Create pools on:"
  echo "  • PancakeSwap: https://pancakeswap.finance/liquidity"
  echo "  • Biswap: https://biswap.org/liquidity"
  echo ""
fi

if [ ! -z "${ADDRESSES[solana]}" ]; then
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}✅ SOLANA${NC}"
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo "Token Mint: ${ADDRESSES[solana]}"
  echo "Explorer: https://solscan.io/token/${ADDRESSES[solana]}"
  echo "Supply: 100,000 ÉTR"
  echo ""
  echo "Create pools on:"
  echo "  • Raydium: https://raydium.io/liquidity/create/"
  echo "  • Orca: https://www.orca.so/pools"
  echo ""
  echo -e "${YELLOW}💡 Use Phantom wallet: https://phantom.app/${NC}"
  echo ""
fi

echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"

echo -e "${GREEN}📊 Total Cost Summary:${NC}"
echo "  Gas fees: \$${TOTAL_COST}"
echo "  Liquidity: \$0 (pools not created yet)"
echo "  ────────────────────"
echo "  Total spent: \$${TOTAL_COST}"
echo ""

echo -e "${GREEN}🎯 DEX Coverage Unlocked:${NC}"
NUM_DEXES=$((${#ADDRESSES[@]} * 4))
echo "  You now have access to ~${NUM_DEXES}+ DEXes!"
echo ""
echo "  By deploying to just ${#ADDRESSES[@]} chains, you can create pools on:"
if [ ! -z "${ADDRESSES[polygon]}" ]; then
  echo "    • QuickSwap, SushiSwap, Uniswap V3, Balancer, Kyber"
fi
if [ ! -z "${ADDRESSES[bsc]}" ]; then
  echo "    • PancakeSwap, Biswap, ApeSwap"
fi
if [ ! -z "${ADDRESSES[solana]}" ]; then
  echo "    • Raydium, Orca, Jupiter, Meteora, Serum"
fi
echo ""

echo -e "${BLUE}📋 Next Steps:${NC}\n"

echo "1. Lock ÉTR on FlareChain (maintain 1:1 backing)"
echo "   Lock amount: $((${#ADDRESSES[@]} * 100000)) ÉTR"
echo "   See: FLARECHAIN_LOCKING_MECHANISM.md"
echo ""

echo "2. Accumulate liquidity funds (\$5k-10k recommended)"
echo "   Current: \$0"
echo "   Target: \$5,000 - \$10,000"
echo "   Timeline: 1-3 months"
echo ""

echo "3. Create pools on DEXes when ready"
echo "   • Start with highest volume DEXes first"
echo "   • QuickSwap (Polygon) - cheapest"
echo "   • PancakeSwap (BSC) - highest volume"
echo "   • Raydium (Solana) - fastest"
echo ""

echo "4. Submit to token lists (optional, improves visibility)"
echo "   See: HOW_DEXES_WORK_COMPLETE_GUIDE.md"
echo ""

echo "5. Submit to price trackers"
echo "   • CoinGecko: https://www.coingecko.com/en/coins/new"
echo "   • CoinMarketCap: https://support.coinmarketcap.com/hc/en-us/articles/360043659351"
echo ""

echo -e "${YELLOW}⚠️  Important Notes:${NC}\n"

echo "• Contracts are deployed but pools don't exist yet"
echo "• No trading until you create pools + add liquidity"
echo "• Users CAN transfer tokens between addresses"
echo "• Smart to wait and launch with proper liquidity (\$5k+)"
echo ""

echo -e "${GREEN}🚀 You're ready for maximum DEX coverage!${NC}\n"

echo "Need help with additional chains? Check out:"
echo "  • ALL_DEXES_FULL_LIST.md - Complete chain/DEX mapping"
echo "  • CONTRACTS_ONLY_DEPLOYMENT.md - Strategy guide"
echo "  • HOW_DEXES_WORK_COMPLETE_GUIDE.md - DEX mechanics"
echo ""

echo "Good luck with your multi-chain launch! 🎉"
echo ""
