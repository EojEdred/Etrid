#!/bin/bash

################################################################################
# EDSC (Ëtrid Dollar Stablecoin) - Solana SPL Token Deployment
# Deploy EDSC stablecoin to Solana for stable pool listing on Raydium
################################################################################

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0;0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     EDSC STABLECOIN DEPLOYMENT - SOLANA (SPL) TOKEN      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}\n"

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo -e "${RED}❌ Solana CLI not found!${NC}"
    echo -e "${YELLOW}Please install Solana CLI or use web interface deployment.${NC}"
    echo -e "${YELLOW}See: ../solana/QUICKEST_DEPLOYMENT.md${NC}"
    exit 1
fi

# Check if SPL Token CLI is installed
if ! command -v spl-token &> /dev/null; then
    echo -e "${YELLOW}Installing SPL Token CLI...${NC}"
    cargo install spl-token-cli
fi

# Environment selection
echo -e "${YELLOW}Select deployment environment:${NC}"
echo "1) Devnet (testing)"
echo "2) Mainnet-beta (PRODUCTION)"
read -p "Enter choice [1-2]: " env_choice

case $env_choice in
    1)
        CLUSTER_URL="https://api.devnet.solana.com"
        CLUSTER="devnet"
        echo -e "${YELLOW}⚠️  Deploying to DEVNET for testing${NC}"
        ;;
    2)
        CLUSTER_URL="https://api.mainnet-beta.solana.com"
        CLUSTER="mainnet-beta"
        echo -e "${RED}⚠️  PRODUCTION DEPLOYMENT to MAINNET${NC}"
        read -p "Are you sure? This will use real SOL! (yes/no): " confirm
        if [ "$confirm" != "yes" ]; then
            echo "Deployment cancelled."
            exit 0
        fi
        ;;
    *)
        echo -e "${RED}Invalid choice${NC}"
        exit 1
        ;;
esac

# Set Solana config
solana config set --url $CLUSTER_URL

# Check wallet balance
echo -e "\n${BLUE}Checking wallet...${NC}"
BALANCE=$(solana balance | awk '{print $1}')
echo "Wallet: $(solana address)"
echo "Balance: $BALANCE SOL"

# Minimum balance check
MIN_BALANCE=0.05
if (( $(echo "$BALANCE < $MIN_BALANCE" | bc -l) )); then
    echo -e "${RED}❌ Insufficient SOL balance!${NC}"
    echo "Need at least $MIN_BALANCE SOL for deployment"
    exit 1
fi

# Create SPL Token
echo -e "\n${BLUE}Creating EDSC SPL Token...${NC}"
echo "Token specs:"
echo "  Name: Etrid Dollar Stablecoin"
echo "  Symbol: EDSC"
echo "  Decimals: 9 (Solana standard)"
echo "  Initial Supply: 100,000,000 EDSC"
echo "  Peg: \$1.00 USD"
echo "  Backing: 150% collateral on FlareChain"

TOKEN_RESULT=$(spl-token create-token --decimals 9 --url $CLUSTER_URL)
TOKEN_ADDRESS=$(echo "$TOKEN_RESULT" | grep -oP 'Creating token \K[A-Za-z0-9]+')

if [ -z "$TOKEN_ADDRESS" ]; then
    echo -e "${RED}❌ Failed to create token${NC}"
    exit 1
fi

echo -e "${GREEN}✅ EDSC token created: $TOKEN_ADDRESS${NC}"

# Create token account
echo -e "\n${BLUE}Creating token account...${NC}"
spl-token create-account $TOKEN_ADDRESS --url $CLUSTER_URL

# Mint initial supply (100M EDSC with 9 decimals)
echo -e "\n${BLUE}Minting initial supply...${NC}"
MINT_AMOUNT=100000000000000000  # 100M * 10^9
spl-token mint $TOKEN_ADDRESS $MINT_AMOUNT --url $CLUSTER_URL

echo -e "${GREEN}✅ Minted 100,000,000 EDSC${NC}"

# Get token account address
TOKEN_ACCOUNT=$(spl-token accounts --url $CLUSTER_URL | grep $TOKEN_ADDRESS | awk '{print $1}')

# Save deployment info
DEPLOY_FILE="deployments/edsc-solana-${CLUSTER}-$(date +%s).json"
mkdir -p deployments

cat > $DEPLOY_FILE << EOF
{
  "network": "${CLUSTER}",
  "token": "EDSC",
  "type": "Stablecoin",
  "tokenAddress": "${TOKEN_ADDRESS}",
  "tokenAccount": "${TOKEN_ACCOUNT}",
  "deployer": "$(solana address)",
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "name": "Etrid Dollar Stablecoin",
  "symbol": "EDSC",
  "decimals": 9,
  "totalSupply": "100000000",
  "targetPeg": "\$1.00 USD",
  "clusterUrl": "${CLUSTER_URL}"
}
EOF

echo -e "${GREEN}💾 Deployment info saved: $DEPLOY_FILE${NC}"

# Add metadata
echo -e "\n${BLUE}Token metadata for EDSC...${NC}"
METADATA_FILE="metadata-edsc.json"
cat > $METADATA_FILE << EOF
{
  "name": "Etrid Dollar Stablecoin",
  "symbol": "EDSC",
  "description": "USD-pegged stablecoin from Ëtrid Protocol - 150% collateralized with ËTR, sBTC, sETH. Stable at \$1.00.",
  "image": "https://etrid.org/images/edsc-logo.png",
  "external_url": "https://etrid.org/edsc",
  "attributes": [
    {
      "trait_type": "Type",
      "value": "Stablecoin"
    },
    {
      "trait_type": "Peg",
      "value": "\$1.00 USD"
    },
    {
      "trait_type": "Chain",
      "value": "Solana"
    },
    {
      "trait_type": "Collateralization",
      "value": "150%"
    },
    {
      "trait_type": "Backing",
      "value": "ËTR, sBTC, sETH"
    }
  ]
}
EOF
echo "Metadata file created: $METADATA_FILE"

# Next steps
echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}🎉 EDSC SOLANA DEPLOYMENT COMPLETE!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}\n"

echo -e "${YELLOW}📝 Next Steps:${NC}\n"

echo "1. Upload metadata to Arweave/IPFS"
echo "   Then update token metadata with Metaplex"

echo -e "\n2. Create Raydium Stable Pool (EDSC/USDC):"
echo "   - Go to: https://raydium.io/liquidity/create/"
echo "   - Token A: $TOKEN_ADDRESS (EDSC)"
echo "   - Token B: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v (USDC)"
echo "   - Pool Type: Stable Pool (NOT standard pool!)"
echo "   - Fee tier: 0.01% (stablecoin pool)"
echo "   - Initial price: ~\$1.00"
echo "   - Add liquidity: 100,000 EDSC + 100,000 USDC"

echo -e "\n3. Lock equivalent EDSC on FlareChain:"
echo "   - Lock 100,000 EDSC on FlareChain reserve"
echo "   - Maintain 1:1 backing between Solana and FlareChain"

echo -e "\n4. Submit to Solana Token List:"
echo "   - PR to: https://github.com/solana-labs/token-list"

echo -e "\n5. Submit to Jupiter Aggregator:"
echo "   - https://station.jup.ag/token-list"

echo -e "\n${GREEN}EDSC Token Address: $TOKEN_ADDRESS${NC}"
echo -e "${GREEN}Token Account: $TOKEN_ACCOUNT${NC}"
echo -e "${GREEN}Target Peg: \$1.00 USD${NC}\n"
