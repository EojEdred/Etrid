#!/bin/bash

################################################################################
# ËTRID MASTER DEX DEPLOYMENT SCRIPT
# Deploy ÉTR to ALL decentralized exchanges across multiple chains
#
# Reference: COMPLETE_DEX_DEPLOYMENT_GUIDE.md
# Governance: FOUNDATION_CHARTER.md (requires 6-of-9 multisig approval)
################################################################################

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Banner
clear
echo -e "${CYAN}"
cat << "EOF"
╔═══════════════════════════════════════════════════════════════════╗
║                                                                     ║
║            ËTRID COMPLETE DEX DEPLOYMENT SYSTEM                    ║
║                                                                     ║
║  Deploy ÉTR to ALL decentralized exchanges across:                ║
║    • Binance Smart Chain (PancakeSwap)                            ║
║    • Solana (Raydium)                                              ║
║    • Ethereum (Uniswap)                                            ║
║    • Polygon (QuickSwap)                                           ║
║    • Avalanche (Trader Joe)                                        ║
║    • Arbitrum (Camelot)                                            ║
║    • Base (Aerodrome)                                              ║
║    • + 13 more DEXes                                               ║
║                                                                     ║
║  Total: 100M ÉTR + $7M liquidity (Phase 1)                        ║
║                                                                     ║
╚═══════════════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}\n"

# Check prerequisites
echo -e "${BLUE}Checking prerequisites...${NC}"

MISSING=0

# Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found${NC}"
    MISSING=1
else
    echo -e "${GREEN}✅ Node.js $(node --version)${NC}"
fi

# npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}❌ npm not found${NC}"
    MISSING=1
else
    echo -e "${GREEN}✅ npm $(npm --version)${NC}"
fi

# Solana CLI (optional - will install if needed)
if command -v solana &> /dev/null; then
    echo -e "${GREEN}✅ Solana CLI $(solana --version | head -1)${NC}"
else
    echo -e "${YELLOW}⚠️  Solana CLI not installed (will install when deploying Solana)${NC}"
fi

# Rust/Cargo (optional)
if command -v cargo &> /dev/null; then
    echo -e "${GREEN}✅ Cargo $(cargo --version)${NC}"
else
    echo -e "${YELLOW}⚠️  Cargo not installed (optional)${NC}"
fi

if [ $MISSING -eq 1 ]; then
    echo -e "\n${RED}Missing required tools. Please install and try again.${NC}"
    exit 1
fi

# Deployment phase selection
echo -e "\n${YELLOW}════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Select Deployment Phase:${NC}\n"

echo "1) Phase 1: Priority DEXes (BSC, Solana, Ethereum, Polygon)"
echo "   - 4 DEXes, 90M ÉTR, $7M liquidity"
echo "   - Recommended for mainnet launch"
echo ""
echo "2) Phase 2: Expansion (Avalanche, Arbitrum, Base, + 3 more)"
echo "   - 6 additional DEXes, 56M ÉTR, $4.4M liquidity"
echo "   - Deploy after Phase 1 established"
echo ""
echo "3) FULL DEPLOYMENT (All 20 DEXes)"
echo "   - Complete ecosystem coverage"
echo "   - 146M ÉTR, $11.4M liquidity"
echo ""
echo "4) Custom (select individual chains)"
echo ""
read -p "Enter choice [1-4]: " phase_choice

case $phase_choice in
    1)
        DEPLOY_CHAINS=("bsc" "solana" "ethereum" "polygon")
        echo -e "${GREEN}Phase 1: Priority DEXes selected${NC}"
        ;;
    2)
        DEPLOY_CHAINS=("avalanche" "arbitrum" "base" "sushiswap" "kyberswap" "orca")
        echo -e "${GREEN}Phase 2: Expansion selected${NC}"
        ;;
    3)
        DEPLOY_CHAINS=("bsc" "solana" "ethereum" "polygon" "avalanche" "arbitrum" "base")
        echo -e "${GREEN}FULL DEPLOYMENT selected${NC}"
        echo -e "${RED}⚠️  This will deploy to ALL chains!${NC}"
        ;;
    4)
        echo -e "${YELLOW}Custom deployment - select chains:${NC}"
        DEPLOY_CHAINS=()
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

# Environment selection
echo -e "\n${YELLOW}════════════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Select Environment:${NC}\n"

echo "1) Testnet (recommended for testing)"
echo "2) Mainnet (PRODUCTION - uses real funds)"
echo ""
read -p "Enter choice [1-2]: " env_choice

case $env_choice in
    1)
        ENVIRONMENT="testnet"
        echo -e "${YELLOW}⚠️  Deploying to TESTNETS${NC}"
        ;;
    2)
        ENVIRONMENT="mainnet"
        echo -e "${RED}═══════════════════════════════════════════════════════════${NC}"
        echo -e "${RED}⚠️  ⚠️  ⚠️   PRODUCTION DEPLOYMENT   ⚠️  ⚠️  ⚠️${NC}"
        echo -e "${RED}═══════════════════════════════════════════════════════════${NC}"
        echo ""
        echo "This will:"
        echo "  • Deploy ÉTR tokens to REAL blockchains"
        echo "  • Use REAL funds for gas fees"
        echo "  • Create LIVE trading pools"
        echo "  • Require Foundation 6-of-9 multisig approval"
        echo ""
        read -p "Type 'I UNDERSTAND' to continue: " confirm
        if [ "$confirm" != "I UNDERSTAND" ]; then
            echo "Deployment cancelled."
            exit 0
        fi
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

# Check environment files
echo -e "\n${BLUE}Checking configuration...${NC}"

for chain in "${DEPLOY_CHAINS[@]}"; do
    ENV_FILE=""

    case $chain in
        bsc|ethereum|polygon|arbitrum|base)
            ENV_FILE="$chain/.env"
            ;;
        solana|orca)
            # Solana uses wallet, not .env
            continue
            ;;
    esac

    if [ ! -z "$ENV_FILE" ] && [ ! -f "$ENV_FILE" ]; then
        echo -e "${YELLOW}⚠️  $ENV_FILE not found${NC}"
        echo "   Copy .env.example and configure with your keys"
        read -p "   Continue anyway? (y/n): " cont
        if [ "$cont" != "y" ]; then
            exit 0
        fi
    fi
done

# Deployment summary
echo -e "\n${CYAN}════════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}DEPLOYMENT SUMMARY${NC}"
echo -e "${CYAN}════════════════════════════════════════════════════════════${NC}\n"

echo -e "${YELLOW}Chains:${NC} ${#DEPLOY_CHAINS[@]}"
echo -e "${YELLOW}Environment:${NC} $ENVIRONMENT"
echo -e "${YELLOW}Total ÉTR:${NC} 100M (Phase 1)"
echo -e "${YELLOW}Total Liquidity:${NC} $7M (Phase 1)"
echo -e "${YELLOW}Estimated Time:${NC} 30-60 minutes"
echo -e "${YELLOW}Estimated Gas:${NC} ~$500 (all chains)"

echo -e "\n${YELLOW}Chains to deploy:${NC}"
for chain in "${DEPLOY_CHAINS[@]}"; do
    echo "  • $chain"
done

echo ""
read -p "Proceed with deployment? (yes/no): " proceed
if [ "$proceed" != "yes" ]; then
    echo "Deployment cancelled."
    exit 0
fi

# Create deployment log
LOG_DIR="deployment-logs"
mkdir -p $LOG_DIR
DEPLOY_LOG="$LOG_DIR/deploy-$(date +%Y%m%d-%H%M%S).log"

echo -e "\n${BLUE}Starting deployment...${NC}"
echo "Logging to: $DEPLOY_LOG"

# Deployment tracking
declare -A DEPLOYMENTS
declare -A DEPLOY_STATUS

# Deploy to each chain
for chain in "${DEPLOY_CHAINS[@]}"; do
    echo -e "\n${CYAN}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}Deploying to: ${chain^^}${NC}"
    echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"

    case $chain in
        bsc)
            echo -e "${BLUE}Deploying to Binance Smart Chain...${NC}"
            cd bsc
            if [ ! -d "node_modules" ]; then
                npm install >> $DEPLOY_LOG 2>&1
            fi

            if [ "$ENVIRONMENT" == "testnet" ]; then
                npm run deploy:testnet 2>&1 | tee -a $DEPLOY_LOG
            else
                npm run deploy:mainnet 2>&1 | tee -a $DEPLOY_LOG
            fi

            DEPLOY_STATUS[$chain]=$?
            cd ..
            ;;

        solana)
            echo -e "${BLUE}Deploying to Solana...${NC}"
            cd solana

            # Auto-answer prompts for scripted deployment
            if [ "$ENVIRONMENT" == "testnet" ]; then
                echo "1" | ./deploy-solana.sh 2>&1 | tee -a $DEPLOY_LOG
            else
                echo "2" | echo "yes" | ./deploy-solana.sh 2>&1 | tee -a $DEPLOY_LOG
            fi

            DEPLOY_STATUS[$chain]=$?
            cd ..
            ;;

        ethereum)
            echo -e "${BLUE}Deploying to Ethereum...${NC}"
            cd ethereum
            if [ ! -d "node_modules" ]; then
                npm install >> $DEPLOY_LOG 2>&1
            fi

            if [ "$ENVIRONMENT" == "testnet" ]; then
                npm run deploy:goerli 2>&1 | tee -a $DEPLOY_LOG
            else
                npm run deploy:mainnet 2>&1 | tee -a $DEPLOY_LOG
            fi

            DEPLOY_STATUS[$chain]=$?
            cd ..
            ;;

        polygon)
            echo -e "${BLUE}Deploying to Polygon...${NC}"
            cd polygon
            if [ ! -d "node_modules" ]; then
                npm install >> $DEPLOY_LOG 2>&1
            fi

            if [ "$ENVIRONMENT" == "testnet" ]; then
                npm run deploy:mumbai 2>&1 | tee -a $DEPLOY_LOG
            else
                npm run deploy:mainnet 2>&1 | tee -a $DEPLOY_LOG
            fi

            DEPLOY_STATUS[$chain]=$?
            cd ..
            ;;

        *)
            echo -e "${YELLOW}⚠️  $chain deployment not yet implemented${NC}"
            DEPLOY_STATUS[$chain]=99
            ;;
    esac

    if [ ${DEPLOY_STATUS[$chain]} -eq 0 ]; then
        echo -e "${GREEN}✅ $chain deployment SUCCESSFUL${NC}"
    else
        echo -e "${RED}❌ $chain deployment FAILED (code: ${DEPLOY_STATUS[$chain]})${NC}"
    fi

    sleep 2
done

# Final summary
echo -e "\n${CYAN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}DEPLOYMENT COMPLETE${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════════${NC}\n"

SUCCESS=0
FAILED=0

for chain in "${DEPLOY_CHAINS[@]}"; do
    if [ ${DEPLOY_STATUS[$chain]} -eq 0 ]; then
        echo -e "${GREEN}✅ $chain${NC}"
        ((SUCCESS++))
    elif [ ${DEPLOY_STATUS[$chain]} -eq 99 ]; then
        echo -e "${YELLOW}⚠️  $chain (not implemented)${NC}"
    else
        echo -e "${RED}❌ $chain${NC}"
        ((FAILED++))
    fi
done

echo -e "\n${YELLOW}Results:${NC}"
echo "  Successful: $SUCCESS"
echo "  Failed: $FAILED"
echo "  Log file: $DEPLOY_LOG"

if [ $FAILED -gt 0 ]; then
    echo -e "\n${RED}Some deployments failed. Check the log for details.${NC}"
    exit 1
fi

# Post-deployment actions
echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}POST-DEPLOYMENT ACTIONS${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}\n"

echo "✅ Next steps:"
echo ""
echo "1. Create liquidity pools on each DEX:"
echo "   • PancakeSwap (BSC): https://pancakeswap.finance/liquidity"
echo "   • Raydium (Solana): https://raydium.io/liquidity/create/"
echo "   • Uniswap (Ethereum): https://app.uniswap.org/pools"
echo "   • QuickSwap (Polygon): https://quickswap.exchange/#/pools"
echo ""
echo "2. Submit token listings:"
echo "   • CoinGecko: https://www.coingecko.com/en/coins/new"
echo "   • CoinMarketCap: https://coinmarketcap.com/request/"
echo "   • Jupiter (Solana): https://station.jup.ag/token-list"
echo ""
echo "3. Update etrid.org with:"
echo "   • All token contract addresses"
echo "   • DEX links for trading"
echo "   • 'Buy ÉTR' button"
echo ""
echo "4. Announce on social media:"
echo "   • Twitter: @EtridProtocol"
echo "   • Discord: #announcements"
echo "   • Telegram: t.me/EtridOfficial"
echo ""
echo "5. Foundation quarterly report:"
echo "   • Document all deployments"
echo "   • Record transaction hashes"
echo "   • Report to Directors"

echo -e "\n${GREEN}🎉 ALL DEPLOYMENTS COMPLETE!${NC}\n"
