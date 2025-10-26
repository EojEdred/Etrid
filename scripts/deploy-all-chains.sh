#!/bin/bash

# ═══════════════════════════════════════════════════════════════
# Ëtrid Multi-Chain Deployment Script
# ═══════════════════════════════════════════════════════════════
# Deploys ÉTR and EDSC tokens across all supported chains:
# - Ethereum (Uniswap)
# - Base L2 (Uniswap)
# - BSC (PancakeSwap)
# - Solana (Raydium)
#
# Usage:
#   ./scripts/deploy-all-chains.sh [--testnet]
#
# Flags:
#   --testnet   Deploy to testnets instead of mainnet
# ═══════════════════════════════════════════════════════════════

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'  # No Color

# Check if testnet flag is set
TESTNET=false
if [[ "$1" == "--testnet" ]]; then
    TESTNET=true
fi

echo "═══════════════════════════════════════════════════════════════"
echo "  🚀 Ëtrid Multi-Chain Deployment"
echo "═══════════════════════════════════════════════════════════════"
echo ""

if [[ "$TESTNET" == true ]]; then
    echo "⚠️  TESTNET MODE"
    echo "   Deploying to: Sepolia, Base Testnet, BSC Testnet, Solana Devnet"
else
    echo "⚠️  MAINNET MODE"
    echo "   Deploying to: Ethereum, Base, BSC, Solana"
    echo ""
    read -p "   Are you sure? This will use real funds! (y/N): " confirm
    if [[ "$confirm" != "y" && "$confirm" != "Y" ]]; then
        echo "❌ Deployment cancelled"
        exit 1
    fi
fi

echo ""

# ═══════════════════════════════════════════════════════════════
# Phase 1: Ethereum
# ═══════════════════════════════════════════════════════════════
echo "────────────────────────────────────────────────────────────────"
echo "📦 Phase 1: Ethereum"
echo "────────────────────────────────────────────────────────────────"
echo ""

cd contracts/ethereum

if [[ "$TESTNET" == true ]]; then
    echo "   Deploying to Sepolia..."
    npm run deploy:sepolia
else
    echo "   Deploying to Ethereum mainnet..."
    npm run deploy:mainnet
fi

echo ""
echo -e "${GREEN}✅ Ethereum deployment complete${NC}"
echo ""

# Save contract addresses
ETR_ETH_ADDRESS=$(grep "ÉTR Token:" -A 1 deployment.log | tail -1)
EDSC_ETH_ADDRESS=$(grep "EDSC Token:" -A 1 deployment.log | tail -1)

echo "   ÉTR.e:  $ETR_ETH_ADDRESS"
echo "   EDSC.e: $EDSC_ETH_ADDRESS"
echo ""

# ═══════════════════════════════════════════════════════════════
# Phase 2: Base L2
# ═══════════════════════════════════════════════════════════════
echo "────────────────────────────────────────────────────────────────"
echo "📦 Phase 2: Base L2"
echo "────────────────────────────────────────────────────────────────"
echo ""

cd ../base

if [[ "$TESTNET" == true ]]; then
    echo "   Deploying to Base Testnet..."
    npm run deploy:base-testnet
else
    echo "   Deploying to Base mainnet..."
    npm run deploy:base
fi

echo ""
echo -e "${GREEN}✅ Base L2 deployment complete${NC}"
echo ""

ETR_BASE_ADDRESS=$(grep "ÉTR Token:" -A 1 deployment.log | tail -1)
EDSC_BASE_ADDRESS=$(grep "EDSC Token:" -A 1 deployment.log | tail -1)

echo "   ÉTR.b:  $ETR_BASE_ADDRESS"
echo "   EDSC.b: $EDSC_BASE_ADDRESS"
echo ""

# ═══════════════════════════════════════════════════════════════
# Phase 3: BSC
# ═══════════════════════════════════════════════════════════════
echo "────────────────────────────────────────────────────────────────"
echo "📦 Phase 3: Binance Smart Chain"
echo "────────────────────────────────────────────────────────────────"
echo ""

cd ../bsc

if [[ "$TESTNET" == true ]]; then
    echo "   Deploying to BSC Testnet..."
    npm run deploy:bsc-testnet
else
    echo "   Deploying to BSC mainnet..."
    npm run deploy:bsc
fi

echo ""
echo -e "${GREEN}✅ BSC deployment complete${NC}"
echo ""

ETR_BSC_ADDRESS=$(grep "ÉTR Token:" -A 1 deployment.log | tail -1)
EDSC_BSC_ADDRESS=$(grep "EDSC Token:" -A 1 deployment.log | tail -1)

echo "   ÉTR.bsc: $ETR_BSC_ADDRESS"
echo "   EDSC.bsc: $EDSC_BSC_ADDRESS"
echo ""

# ═══════════════════════════════════════════════════════════════
# Phase 4: Solana
# ═══════════════════════════════════════════════════════════════
echo "────────────────────────────────────────────────────────────────"
echo "📦 Phase 4: Solana"
echo "────────────────────────────────────────────────────────────────"
echo ""

cd ../solana

if [[ "$TESTNET" == true ]]; then
    echo "   Deploying to Solana Devnet..."
    anchor deploy --provider.cluster devnet
else
    echo "   Deploying to Solana mainnet..."
    anchor deploy --provider.cluster mainnet
fi

echo ""
echo -e "${GREEN}✅ Solana deployment complete${NC}"
echo ""

ETR_SOL_ADDRESS=$(solana address -k target/deploy/etr_solana-keypair.json)
EDSC_SOL_ADDRESS=$(solana address -k target/deploy/edsc_solana-keypair.json)

echo "   ÉTR.s:  $ETR_SOL_ADDRESS"
echo "   EDSC.s: $EDSC_SOL_ADDRESS"
echo ""

# ═══════════════════════════════════════════════════════════════
# Save Deployment Summary
# ═══════════════════════════════════════════════════════════════
echo "────────────────────────────────────────────────────────────────"
echo "💾 Saving deployment summary..."
echo "────────────────────────────────────────────────────────────────"
echo ""

cd ../../..

cat > DEPLOYMENT_ADDRESSES.json << EOF
{
  "network": "$(if [[ "$TESTNET" == true ]]; then echo "testnet"; else echo "mainnet"; fi)",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "ethereum": {
    "etr": "$ETR_ETH_ADDRESS",
    "edsc": "$EDSC_ETH_ADDRESS",
    "explorer": "https://etherscan.io"
  },
  "base": {
    "etr": "$ETR_BASE_ADDRESS",
    "edsc": "$EDSC_BASE_ADDRESS",
    "explorer": "https://basescan.org"
  },
  "bsc": {
    "etr": "$ETR_BSC_ADDRESS",
    "edsc": "$EDSC_BSC_ADDRESS",
    "explorer": "https://bscscan.com"
  },
  "solana": {
    "etr": "$ETR_SOL_ADDRESS",
    "edsc": "$EDSC_SOL_ADDRESS",
    "explorer": "https://solscan.io"
  }
}
EOF

echo "✅ Deployment addresses saved to DEPLOYMENT_ADDRESSES.json"
echo ""

# ═══════════════════════════════════════════════════════════════
# Deployment Summary
# ═══════════════════════════════════════════════════════════════
echo "═══════════════════════════════════════════════════════════════"
echo "  ✅ All Deployments Complete!"
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "📋 Contract Addresses:"
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "Ethereum:"
echo "  ÉTR.e:  $ETR_ETH_ADDRESS"
echo "  EDSC.e: $EDSC_ETH_ADDRESS"
echo ""
echo "Base L2:"
echo "  ÉTR.b:  $ETR_BASE_ADDRESS"
echo "  EDSC.b: $EDSC_BASE_ADDRESS"
echo ""
echo "BSC:"
echo "  ÉTR.bsc: $ETR_BSC_ADDRESS"
echo "  EDSC.bsc: $EDSC_BSC_ADDRESS"
echo ""
echo "Solana:"
echo "  ÉTR.s:  $ETR_SOL_ADDRESS"
echo "  EDSC.s: $EDSC_SOL_ADDRESS"
echo ""
echo "────────────────────────────────────────────────────────────────"
echo ""
echo "⚠️  Next Steps:"
echo "────────────────────────────────────────────────────────────────"
echo "1. Verify contracts on block explorers"
echo "2. Create liquidity pools (run scripts/seed-liquidity.sh)"
echo "3. Start bridge adapters (run scripts/start-bridges.sh)"
echo "4. Update frontend with contract addresses"
echo "5. Submit to CoinGecko and CoinMarketCap"
echo ""
echo "═══════════════════════════════════════════════════════════════"
