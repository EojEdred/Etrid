#!/bin/bash

################################################################################
# ËTRID COMPLETE DEPLOYMENT - ÉTR + EDSC to All Chains
# Deploys both native token (ÉTR) and stablecoin (EDSC) together
################################################################################

set -e  # Exit on error

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                              ║${NC}"
echo -e "${CYAN}║      ËTRID COMPLETE MULTI-CHAIN DEPLOYMENT                  ║${NC}"
echo -e "${CYAN}║      ÉTR (Native Token) + EDSC (Stablecoin)                ║${NC}"
echo -e "${CYAN}║                                                              ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}\n"

# Deployment configuration
DEPLOY_ETR=true
DEPLOY_EDSC=true
SKIP_ETHEREUM=true  # Ethereum is expensive ($150), skip by default

# Chain selection
echo -e "${YELLOW}Select deployment scope:${NC}"
echo "1) Deploy to ALL chains (Base, Arbitrum, Polygon, BSC, Solana)"
echo "2) Deploy to cheap chains only (Base, Arbitrum, Solana) - ~\$17"
echo "3) Custom chain selection"
echo "4) Deploy Ethereum only (expensive - \$300)"
read -p "Enter choice [1-4]: " scope_choice

case $scope_choice in
    1)
        CHAINS=("base" "arbitrum" "polygon" "bsc" "solana")
        ESTIMATED_COST="~\$28-33"
        ;;
    2)
        CHAINS=("base" "arbitrum" "solana")
        ESTIMATED_COST="~\$17"
        ;;
    3)
        echo -e "\n${YELLOW}Select chains to deploy (y/n for each):${NC}"
        CHAINS=()
        for chain in base arbitrum polygon bsc ethereum solana; do
            read -p "Deploy to $chain? (y/n): " choice
            if [ "$choice" = "y" ]; then
                CHAINS+=("$chain")
            fi
        done
        ESTIMATED_COST="Custom"
        ;;
    4)
        CHAINS=("ethereum")
        ESTIMATED_COST="~\$300 (both tokens)"
        SKIP_ETHEREUM=false
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

# Token selection
echo -e "\n${YELLOW}Select tokens to deploy:${NC}"
echo "1) Both ÉTR + EDSC (recommended for full ecosystem)"
echo "2) ÉTR only (native token)"
echo "3) EDSC only (stablecoin)"
read -p "Enter choice [1-3]: " token_choice

case $token_choice in
    1)
        DEPLOY_ETR=true
        DEPLOY_EDSC=true
        ;;
    2)
        DEPLOY_ETR=true
        DEPLOY_EDSC=false
        ;;
    3)
        DEPLOY_ETR=false
        DEPLOY_EDSC=true
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

# Summary
echo -e "\n${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  DEPLOYMENT SUMMARY${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "Chains: ${GREEN}${CHAINS[@]}${NC}"
if [ "$DEPLOY_ETR" = true ] && [ "$DEPLOY_EDSC" = true ]; then
    echo -e "Tokens: ${GREEN}ÉTR + EDSC${NC}"
elif [ "$DEPLOY_ETR" = true ]; then
    echo -e "Tokens: ${GREEN}ÉTR only${NC}"
else
    echo -e "Tokens: ${GREEN}EDSC only${NC}"
fi
echo -e "Estimated Cost: ${YELLOW}$ESTIMATED_COST${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}\n"

read -p "Proceed with deployment? (yes/no): " proceed
if [ "$proceed" != "yes" ]; then
    echo "Deployment cancelled."
    exit 0
fi

# Deployment tracking
SUCCESSFUL_DEPLOYMENTS=()
FAILED_DEPLOYMENTS=()
ETR_ADDRESSES=()
EDSC_ADDRESSES=()

# Deploy to each chain
for chain in "${CHAINS[@]}"; do
    echo -e "\n${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║  Deploying to: $(printf '%-47s' $chain) ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}\n"

    # Deploy ÉTR
    if [ "$DEPLOY_ETR" = true ]; then
        echo -e "${YELLOW}Deploying ÉTR (Native Token)...${NC}"

        if [ "$chain" = "solana" ]; then
            # Solana uses shell script
            cd solana
            if ./deploy-solana.sh; then
                echo -e "${GREEN}✅ ÉTR deployed to Solana${NC}"
                SUCCESSFUL_DEPLOYMENTS+=("$chain-ETR")
            else
                echo -e "${RED}❌ ÉTR deployment failed on Solana${NC}"
                FAILED_DEPLOYMENTS+=("$chain-ETR")
            fi
            cd ..
        else
            # EVM chains use npm
            cd $chain
            if npm run deploy:mainnet; then
                echo -e "${GREEN}✅ ÉTR deployed to $chain${NC}"
                SUCCESSFUL_DEPLOYMENTS+=("$chain-ETR")

                # Extract contract address from deployment file
                ETR_ADDR=$(ls -t deployment-*-mainnet-*.json 2>/dev/null | head -1 | xargs cat | grep -oP '"address":\s*"\K[^"]+' || echo "N/A")
                ETR_ADDRESSES+=("$chain: $ETR_ADDR")
            else
                echo -e "${RED}❌ ÉTR deployment failed on $chain${NC}"
                FAILED_DEPLOYMENTS+=("$chain-ETR")
            fi
            cd ..
        fi

        sleep 2
    fi

    # Deploy EDSC
    if [ "$DEPLOY_EDSC" = true ]; then
        echo -e "\n${YELLOW}Deploying EDSC (Stablecoin)...${NC}"

        if [ "$chain" = "solana" ]; then
            # Solana EDSC uses shell script
            cd edsc-stablecoin/solana
            if ./deploy-edsc-solana.sh; then
                echo -e "${GREEN}✅ EDSC deployed to Solana${NC}"
                SUCCESSFUL_DEPLOYMENTS+=("$chain-EDSC")
            else
                echo -e "${RED}❌ EDSC deployment failed on Solana${NC}"
                FAILED_DEPLOYMENTS+=("$chain-EDSC")
            fi
            cd ../..
        else
            # EVM chains use npm
            cd edsc-stablecoin/$chain
            if npm run deploy:mainnet; then
                echo -e "${GREEN}✅ EDSC deployed to $chain${NC}"
                SUCCESSFUL_DEPLOYMENTS+=("$chain-EDSC")

                # Extract contract address
                EDSC_ADDR=$(ls -t deployment-edsc-mainnet-*.json 2>/dev/null | head -1 | xargs cat | grep -oP '"address":\s*"\K[^"]+' || echo "N/A")
                EDSC_ADDRESSES+=("$chain: $EDSC_ADDR")
            else
                echo -e "${RED}❌ EDSC deployment failed on $chain${NC}"
                FAILED_DEPLOYMENTS+=("$chain-EDSC")
            fi
            cd ../..
        fi

        sleep 2
    fi

    echo -e "${GREEN}✅ Completed deployment to $chain${NC}\n"
    sleep 1
done

# Final Summary
echo -e "\n${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                              ║${NC}"
echo -e "${CYAN}║              DEPLOYMENT COMPLETE!                            ║${NC}"
echo -e "${CYAN}║                                                              ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}\n"

echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  DEPLOYMENT RESULTS${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"

echo -e "\n${GREEN}Successful Deployments (${#SUCCESSFUL_DEPLOYMENTS[@]}):${NC}"
for deployment in "${SUCCESSFUL_DEPLOYMENTS[@]}"; do
    echo -e "  ✅ $deployment"
done

if [ ${#FAILED_DEPLOYMENTS[@]} -gt 0 ]; then
    echo -e "\n${RED}Failed Deployments (${#FAILED_DEPLOYMENTS[@]}):${NC}"
    for deployment in "${FAILED_DEPLOYMENTS[@]}"; do
        echo -e "  ❌ $deployment"
    done
fi

# Contract Addresses
if [ "$DEPLOY_ETR" = true ] && [ ${#ETR_ADDRESSES[@]} -gt 0 ]; then
    echo -e "\n${YELLOW}ÉTR Contract Addresses:${NC}"
    for addr in "${ETR_ADDRESSES[@]}"; do
        echo -e "  $addr"
    done
fi

if [ "$DEPLOY_EDSC" = true ] && [ ${#EDSC_ADDRESSES[@]} -gt 0 ]; then
    echo -e "\n${YELLOW}EDSC Contract Addresses:${NC}"
    for addr in "${EDSC_ADDRESSES[@]}"; do
        echo -e "  $addr"
    done
fi

echo -e "\n${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  NEXT STEPS${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}\n"

echo -e "${YELLOW}1. Verify contracts on block explorers${NC}"
echo "   - Check each chain's explorer (Etherscan, BSCScan, etc.)"
echo "   - Run verification scripts: npm run verify"

echo -e "\n${YELLOW}2. Create liquidity pools${NC}"
if [ "$DEPLOY_ETR" = true ]; then
    echo "   ÉTR Pools (volatile pairs):"
    echo "   - Base: ÉTR/ETH on Aerodrome or Uniswap V3"
    echo "   - Arbitrum: ÉTR/ETH on Camelot"
    echo "   - BSC: ÉTR/BNB on PancakeSwap"
    echo "   - Solana: ÉTR/SOL on Raydium"
fi

if [ "$DEPLOY_EDSC" = true ]; then
    echo "   EDSC Pools (stablecoin pairs):"
    echo "   - Ethereum: EDSC/USDC/USDT on Curve"
    echo "   - Polygon: EDSC/USDC on Balancer or Curve"
    echo "   - BSC: EDSC/BUSD on PancakeSwap StableSwap"
    echo "   - Solana: EDSC/USDC on Raydium Stable Pool"
fi

echo -e "\n${YELLOW}3. Lock tokens on FlareChain (1:1 backing)${NC}"
echo "   For each chain deployed:"
echo "   - Deployed 100K ÉTR → Lock 100K ÉTR on FlareChain"
echo "   - Deployed 100K EDSC → Lock 100K EDSC on FlareChain"

echo -e "\n${YELLOW}4. Submit to token lists${NC}"
echo "   - CoinGecko: https://www.coingecko.com/en/coins/new"
echo "   - CoinMarketCap: https://coinmarketcap.com/request/"
echo "   - Solana Token List: https://github.com/solana-labs/token-list"

echo -e "\n${YELLOW}5. Update etrid.org with contract addresses${NC}"
echo "   - Add ÉTR addresses to token page"
echo "   - Add EDSC addresses to stablecoin page"
echo "   - Update bridge interface with addresses"

echo -e "\n${GREEN}🎉 Congratulations! Your tokens are deployed!${NC}\n"

# Save summary to file
SUMMARY_FILE="DEPLOYMENT_SUMMARY_$(date +%Y%m%d_%H%M%S).txt"
cat > $SUMMARY_FILE << EOF
ËTRID DEPLOYMENT SUMMARY
Generated: $(date)

Chains Deployed: ${CHAINS[@]}
Tokens Deployed: $([ "$DEPLOY_ETR" = true ] && echo -n "ÉTR ")$([ "$DEPLOY_EDSC" = true ] && echo -n "EDSC")

Successful Deployments (${#SUCCESSFUL_DEPLOYMENTS[@]}):
$(printf '  %s\n' "${SUCCESSFUL_DEPLOYMENTS[@]}")

$(if [ ${#FAILED_DEPLOYMENTS[@]} -gt 0 ]; then
    echo "Failed Deployments (${#FAILED_DEPLOYMENTS[@]}):"
    printf '  %s\n' "${FAILED_DEPLOYMENTS[@]}"
fi)

ÉTR Addresses:
$(printf '  %s\n' "${ETR_ADDRESSES[@]}")

EDSC Addresses:
$(printf '  %s\n' "${EDSC_ADDRESSES[@]}")
EOF

echo -e "${GREEN}📄 Summary saved to: $SUMMARY_FILE${NC}\n"
