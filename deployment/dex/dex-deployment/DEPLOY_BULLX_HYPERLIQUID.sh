#!/bin/bash
################################################################################
# ËTRID DEPLOYMENT - BULLX NEO + HYPERLIQUID FOCUSED
#
# Deploys to chains that BullX NEO supports + Hyperliquid (mandatory)
#
# BullX Auto-Detection: Once you create pools on Raydium/PancakeSwap,
# BullX NEO will automatically detect and list ÉTR. No manual submission!
#
# Chains deployed:
# 1. Solana ($4.50) → BullX primary chain, Raydium, Orca
# 2. BSC ($6) → BullX supported, PancakeSwap
# 3. Base ($1) → BullX supported, Aerodrome
# 4. Arbitrum ($1) → BullX supported, Camelot
# 5. Hyperliquid (~$3-5) → MANDATORY per user request
#
# Total: ~$15.50-17.50
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
║     ËTRID DEPLOYMENT: BULLX NEO + HYPERLIQUID            ║
║                                                            ║
║     BullX Compatible • Hyperliquid Included               ║
║                                                            ║
╚════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}\n"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Explain BullX auto-detection
echo -e "${BLUE}🎯 How BullX NEO Works:${NC}\n"

echo "BullX NEO is a DEX aggregator that AUTOMATICALLY detects tokens!"
echo ""
echo "When you:"
echo "  1. Deploy ÉTR to Solana/BSC/Base/Arbitrum"
echo "  2. Create a pool on Raydium/PancakeSwap/etc."
echo ""
echo "BullX will:"
echo "  ✅ Automatically scan the blockchain"
echo "  ✅ Detect your new token and pool"
echo "  ✅ Add ÉTR to BullX NEO trading interface"
echo "  ✅ Users can trade immediately!"
echo ""
echo -e "${GREEN}No manual submission needed! 🎉${NC}\n"

# Show deployment plan
echo -e "${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  DEPLOYMENT PLAN                                           ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo "Chains being deployed (BullX compatible):"
echo ""
echo "1. ✅ Solana ($4.50)"
echo "   • Primary BullX chain"
echo "   • DEXes: Raydium, Orca, Jupiter"
echo "   • BullX: Auto-detects from Raydium"
echo ""

echo "2. ✅ BSC ($6)"
echo "   • BullX supported"
echo "   • DEXes: PancakeSwap, Biswap"
echo "   • BullX: Auto-detects from PancakeSwap"
echo ""

echo "3. ✅ Base ($1)"
echo "   • BullX supported"
echo "   • DEXes: Aerodrome, Uniswap V3"
echo "   • BullX: Auto-detects from Aerodrome"
echo ""

echo "4. ✅ Arbitrum ($1)"
echo "   • BullX supported"
echo "   • DEXes: Camelot, Uniswap V3"
echo "   • BullX: Auto-detects from Camelot"
echo ""

echo "5. ✅ Hyperliquid (~$3-5) ⭐ MANDATORY"
echo "   • Perpetual futures DEX"
echo "   • Advanced trading features"
echo "   • User requested as must-have"
echo ""

echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Total Cost: $15.50 - $17.50${NC}"
echo -e "${GREEN}BullX Compatible Chains: 4${NC}"
echo -e "${GREEN}Total DEXes: 15+${NC}"
echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"

# Note about Polygon
echo -e "${YELLOW}⚠️  Note: Skipping Polygon${NC}"
echo "   Polygon is NOT supported by BullX NEO"
echo "   Focusing on BullX-compatible chains only"
echo ""

# Note about Ethereum
echo -e "${YELLOW}⚠️  Note: Skipping Ethereum (for now)${NC}"
echo "   Ethereum gas is $150+ (too expensive)"
echo "   BullX supports it but deploy later when budget allows"
echo ""

# Confirm
read -p "Continue with BullX + Hyperliquid deployment? (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
  echo "Deployment cancelled."
  exit 0
fi

# Check which chains are ready
echo -e "\n${BLUE}📋 Pre-flight checks...${NC}\n"

READY_COUNT=0

# Check Solana
if command -v solana &> /dev/null; then
  echo -e "${GREEN}✅ Solana CLI ready${NC}"
  ((READY_COUNT++))
else
  echo -e "${YELLOW}⚠️  Solana CLI not installed${NC}"
fi

# Check BSC
if [ -f "bsc/.env" ]; then
  echo -e "${GREEN}✅ BSC config ready${NC}"
  ((READY_COUNT++))
else
  echo -e "${YELLOW}⚠️  BSC .env not found${NC}"
fi

# Check Base
if [ -f "base/.env" ]; then
  echo -e "${GREEN}✅ Base config ready${NC}"
  ((READY_COUNT++))
else
  echo -e "${YELLOW}⚠️  Base config not found (will create)${NC}"
fi

# Check Arbitrum
if [ -f "arbitrum/.env" ]; then
  echo -e "${GREEN}✅ Arbitrum config ready${NC}"
  ((READY_COUNT++))
else
  echo -e "${YELLOW}⚠️  Arbitrum config not found (will create)${NC}"
fi

# Check Hyperliquid
if [ -f "hyperliquid/.env" ]; then
  echo -e "${GREEN}✅ Hyperliquid config ready${NC}"
  ((READY_COUNT++))
else
  echo -e "${YELLOW}⚠️  Hyperliquid config not found (will create)${NC}"
fi

echo ""
echo -e "${BLUE}Ready chains: ${GREEN}${READY_COUNT}/5${NC}"
echo ""

if [ $READY_COUNT -eq 0 ]; then
  echo -e "${RED}❌ No chains configured! Please set up .env files.${NC}"
  exit 1
fi

# Array to store deployment results
declare -A ADDRESSES
TOTAL_COST=0

# ============================================================================
# DEPLOY TO SOLANA (BullX Primary Chain)
# ============================================================================
if command -v solana &> /dev/null; then
  echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
  echo -e "${CYAN}║  DEPLOYING TO SOLANA (BullX Primary)                       ║${NC}"
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
      echo -e "${BLUE}DEXes available:${NC}"
      echo "  • Raydium CLMM"
      echo "  • Orca"
      echo "  • Jupiter (auto-aggregator)"
      echo ""
      echo -e "${GREEN}🎯 BullX NEO will auto-detect once you create Raydium pool!${NC}"
    fi
  else
    echo -e "\n${RED}❌ Solana deployment failed!${NC}"
  fi

  cd ..
else
  echo -e "\n${RED}❌ Solana CLI not installed. Please install first.${NC}"
  echo "Visit: https://docs.solana.com/cli/install-solana-cli-tools"
fi

# ============================================================================
# DEPLOY TO BSC (BullX Supported)
# ============================================================================
if [ -f "bsc/.env" ]; then
  echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
  echo -e "${CYAN}║  DEPLOYING TO BSC (BullX Supported)                        ║${NC}"
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
    echo -e "${BLUE}DEXes available:${NC}"
    echo "  • PancakeSwap V3"
    echo "  • Biswap"
    echo ""
    echo -e "${GREEN}🎯 BullX NEO will auto-detect once you create PancakeSwap pool!${NC}"
  else
    echo -e "\n${RED}❌ BSC deployment failed!${NC}"
  fi

  cd ..
else
  echo -e "\n${YELLOW}⚠️  Skipping BSC (no .env file)${NC}"
fi

# ============================================================================
# DEPLOY TO BASE (BullX Supported) - TODO
# ============================================================================
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  BASE DEPLOYMENT                                           ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

if [ -f "base/.env" ]; then
  echo -e "${YELLOW}⚠️  Base deployment config exists but script not implemented yet${NC}"
  echo "   To deploy to Base manually:"
  echo "   1. Use same contract as Ethereum/Polygon (ERC-20)"
  echo "   2. Change hardhat config to Base RPC"
  echo "   3. RPC: https://mainnet.base.org"
  echo "   4. Deploy: npm run deploy:mainnet"
  echo ""
else
  echo -e "${YELLOW}⚠️  Base not configured yet${NC}"
  echo "   Cost: ~$1"
  echo "   DEX: Aerodrome (largest on Base)"
  echo "   BullX: Supported"
  echo ""
  echo "   To set up:"
  echo "   1. Create base/ folder (copy from polygon/)"
  echo "   2. Update hardhat.config.js with Base RPC"
  echo "   3. Add .env with PRIVATE_KEY"
  echo ""
fi

# ============================================================================
# DEPLOY TO ARBITRUM (BullX Supported) - TODO
# ============================================================================
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  ARBITRUM DEPLOYMENT                                       ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

if [ -f "arbitrum/.env" ]; then
  echo -e "${YELLOW}⚠️  Arbitrum deployment config exists but script not implemented yet${NC}"
  echo "   To deploy manually (same as above)"
else
  echo -e "${YELLOW}⚠️  Arbitrum not configured yet${NC}"
  echo "   Cost: ~$1"
  echo "   DEX: Camelot (largest on Arbitrum)"
  echo "   BullX: Supported"
  echo ""
fi

# ============================================================================
# DEPLOY TO HYPERLIQUID (MANDATORY) - TODO
# ============================================================================
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  HYPERLIQUID DEPLOYMENT ⭐ MANDATORY                       ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${YELLOW}⚠️  Hyperliquid deployment not yet implemented${NC}\n"

echo "Hyperliquid Details:"
echo "  • Chain: HyperEVM (custom L1)"
echo "  • Type: Perpetual futures platform"
echo "  • Deploy cost: ~$3-5"
echo ""

echo "To deploy to Hyperliquid:"
echo "  1. Get HyperEVM RPC endpoint"
echo "  2. Create hyperliquid/ folder with ERC-20 contract"
echo "  3. Configure hardhat with HyperEVM network"
echo "  4. Deploy standard ERC-20 token"
echo "  5. Create perp market (may require Hyperliquid approval)"
echo ""

echo "Resources:"
echo "  • Docs: https://hyperliquid.gitbook.io/"
echo "  • Discord: https://discord.gg/hyperliquid"
echo ""

echo -e "${BLUE}💡 Recommendation:${NC}"
echo "   Contact Hyperliquid team for listing requirements"
echo "   They may have specific token standards or approval process"
echo ""

# ============================================================================
# FINAL REPORT
# ============================================================================
echo -e "\n${CYAN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                 DEPLOYMENT REPORT                          ║${NC}"
echo -e "${CYAN}╚════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}Successfully deployed to ${#ADDRESSES[@]} chains:${NC}\n"

if [ ! -z "${ADDRESSES[solana]}" ]; then
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}✅ SOLANA (BullX Primary)${NC}"
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo "Token Mint: ${ADDRESSES[solana]}"
  echo "Explorer: https://solscan.io/token/${ADDRESSES[solana]}"
  echo ""
  echo "🎯 How to get on BullX NEO:"
  echo "  1. Go to Raydium: https://raydium.io/liquidity/create/"
  echo "  2. Create ÉTR/SOL pool"
  echo "  3. Add liquidity (recommended $5k+)"
  echo "  4. BullX auto-detects within 1-2 hours! ✅"
  echo ""
  echo "💡 Use Phantom wallet to trade:"
  echo "   https://phantom.app/"
  echo ""
fi

if [ ! -z "${ADDRESSES[bsc]}" ]; then
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}✅ BSC (BullX Supported)${NC}"
  echo -e "${MAGENTA}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo "Contract: ${ADDRESSES[bsc]}"
  echo "Explorer: https://bscscan.com/address/${ADDRESSES[bsc]}"
  echo ""
  echo "🎯 How to get on BullX NEO:"
  echo "  1. Go to PancakeSwap: https://pancakeswap.finance/liquidity"
  echo "  2. Create ÉTR/BNB pool"
  echo "  3. Add liquidity"
  echo "  4. BullX auto-detects! ✅"
  echo ""
fi

echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"

echo -e "${GREEN}📊 Cost Summary:${NC}"
echo "  Gas fees: \$${TOTAL_COST}"
echo "  ────────────────────"
echo "  Total spent: \$${TOTAL_COST}"
echo ""

echo -e "${BLUE}📋 Still Need to Deploy:${NC}"
echo "  • Base (~$1) - BullX supported"
echo "  • Arbitrum (~$1) - BullX supported"
echo "  • Hyperliquid (~$3-5) - MANDATORY (your requirement)"
echo ""
echo "  These need configuration first (see instructions above)"
echo ""

echo -e "${GREEN}🎯 Next Steps for BullX Listing:${NC}\n"

echo "1. Create pools on DEXes (need $5k-10k liquidity)"
echo "   • Raydium (Solana) - Primary for BullX"
echo "   • PancakeSwap (BSC) - Secondary for BullX"
echo ""

echo "2. BullX will auto-detect within 1-2 hours ✅"
echo "   • No manual submission needed"
echo "   • Just create pool + add liquidity"
echo "   • BullX scans blockchain automatically"
echo ""

echo "3. Monitor BullX NEO for your token"
echo "   • Visit: https://bullx.io/"
echo "   • Search for ÉTR or paste contract address"
echo "   • Should appear automatically after pool creation"
echo ""

echo -e "${YELLOW}⚠️  Important:${NC}"
echo "   Base, Arbitrum, and Hyperliquid still need deployment"
echo "   See instructions above for manual deployment steps"
echo ""

echo "Good luck with BullX and Hyperliquid launch! 🚀"
echo ""
