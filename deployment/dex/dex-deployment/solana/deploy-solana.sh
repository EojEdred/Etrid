#!/bin/bash

################################################################################
# ËTRID Solana SPL Token Deployment Script
# Deploy ÉTR to Solana for Raydium CLMM listing
################################################################################

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     ËTRID DEX DEPLOYMENT - SOLANA (SPL) TOKEN            ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}\n"

# Check if Solana CLI is installed
if ! command -v solana &> /dev/null; then
    echo -e "${RED}❌ Solana CLI not found!${NC}"
    echo -e "${YELLOW}Installing Solana CLI...${NC}"
    sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
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
    echo "Get SOL at:"
    if [ "$CLUSTER" == "devnet" ]; then
        echo "  Devnet faucet: https://faucet.solana.com"
    else
        echo "  Buy SOL: https://www.coinbase.com or https://www.binance.com"
    fi
    exit 1
fi

# Create SPL Token
echo -e "\n${BLUE}Creating SPL Token...${NC}"
echo "Token specs:"
echo "  Name: Etrid Coin"
echo "  Symbol: ETR"
echo "  Decimals: 9 (Solana standard)"
echo "  Initial Supply: 100,000,000 ÉTR"

TOKEN_RESULT=$(spl-token create-token --decimals 9 --url $CLUSTER_URL)
TOKEN_ADDRESS=$(echo "$TOKEN_RESULT" | grep -oP 'Creating token \K[A-Za-z0-9]+')

if [ -z "$TOKEN_ADDRESS" ]; then
    echo -e "${RED}❌ Failed to create token${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Token created: $TOKEN_ADDRESS${NC}"

# Create token account
echo -e "\n${BLUE}Creating token account...${NC}"
spl-token create-account $TOKEN_ADDRESS --url $CLUSTER_URL

# Mint initial supply (100M ÉTR with 9 decimals)
echo -e "\n${BLUE}Minting initial supply...${NC}"
MINT_AMOUNT=100000000000000000  # 100M * 10^9
spl-token mint $TOKEN_ADDRESS $MINT_AMOUNT --url $CLUSTER_URL

echo -e "${GREEN}✅ Minted 100,000,000 ÉTR${NC}"

# Get token account address
TOKEN_ACCOUNT=$(spl-token accounts --url $CLUSTER_URL | grep $TOKEN_ADDRESS | awk '{print $1}')

# Save deployment info
DEPLOY_FILE="deployments/solana-${CLUSTER}-$(date +%s).json"
mkdir -p deployments

cat > $DEPLOY_FILE << EOF
{
  "network": "${CLUSTER}",
  "tokenAddress": "${TOKEN_ADDRESS}",
  "tokenAccount": "${TOKEN_ACCOUNT}",
  "deployer": "$(solana address)",
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "name": "Etrid Coin",
  "symbol": "ETR",
  "decimals": 9,
  "totalSupply": "100000000",
  "clusterUrl": "${CLUSTER_URL}"
}
EOF

echo -e "${GREEN}💾 Deployment info saved: $DEPLOY_FILE${NC}"

# Add metadata
echo -e "\n${BLUE}Adding token metadata...${NC}"
if command -v metaboss &> /dev/null; then
    # Create metadata JSON
    METADATA_FILE="metadata-etr.json"
    cat > $METADATA_FILE << EOF
{
  "name": "Etrid Coin",
  "symbol": "ETR",
  "description": "Native token of Ëtrid Multichain Protocol - Decentralized, Democratic, Transparent blockchain with FODDoS governance",
  "image": "https://etrid.org/images/etr-logo.png",
  "external_url": "https://etrid.org",
  "attributes": [
    {
      "trait_type": "Chain",
      "value": "Solana"
    },
    {
      "trait_type": "Type",
      "value": "SPL Token"
    },
    {
      "trait_type": "Total Supply",
      "value": "1,000,000,000"
    }
  ]
}
EOF
    echo "Metadata file created: $METADATA_FILE"
    echo -e "${YELLOW}Upload $METADATA_FILE to IPFS or Arweave and use the URI in Metaplex${NC}"
else
    echo -e "${YELLOW}⚠️  Metaboss not installed. Install with: cargo install metaboss${NC}"
    echo "   Then update metadata manually"
fi

# Next steps
echo -e "\n${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}🎉 SOLANA DEPLOYMENT COMPLETE!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}\n"

echo -e "${YELLOW}📝 Next Steps:${NC}\n"

echo "1. Add Token Metadata:"
echo "   - Upload metadata.json to Arweave/IPFS"
echo "   - Use Metaplex to set metadata URI"
echo "   OR use Metaboss:"
echo "   metaboss update uri -a $TOKEN_ADDRESS -u <YOUR_METADATA_URI> --keypair ~/.config/solana/id.json"

echo -e "\n2. Create Raydium CLMM Pool:"
echo "   - Go to: https://raydium.io/liquidity/create/"
echo "   - Token A: $TOKEN_ADDRESS (ÉTR)"
echo "   - Token B: So11111111111111111111111111111111111111112 (SOL)"
echo "   - Fee tier: 0.25%"
echo "   - Initial price: Set based on BSC price"
echo "   - Add liquidity: 25,000,000 ÉTR + ~13,333 SOL ($2M @ \$150/SOL)"

echo -e "\n3. Submit to Solana Token List:"
echo "   - PR to: https://github.com/solana-labs/token-list"
echo "   - Include: $METADATA_FILE"

echo -e "\n4. Submit to Jupiter Aggregator:"
echo "   - https://station.jup.ag/token-list"

echo -e "\n5. Update etrid.org with Solana token address"

echo -e "\n${GREEN}Token Address: $TOKEN_ADDRESS${NC}"
echo -e "${GREEN}Token Account: $TOKEN_ACCOUNT${NC}\n"

# Create Raydium pool helper script
cat > create-raydium-pool.sh << 'EOFSCRIPT'
#!/bin/bash
echo "Creating Raydium CLMM pool..."
echo ""
echo "Visit: https://raydium.io/liquidity/create/"
echo ""
echo "Pool Configuration:"
echo "  Token A: %TOKEN_ADDRESS% (ÉTR)"
echo "  Token B: So11111111111111111111111111111111111111112 (SOL)"
echo "  Fee Tier: 0.25%"
echo "  Liquidity: 25M ÉTR + 13,333 SOL"
echo ""
echo "Make sure you have enough ÉTR and SOL in your wallet!"
EOFSCRIPT

sed -i "s/%TOKEN_ADDRESS%/$TOKEN_ADDRESS/g" create-raydium-pool.sh
chmod +x create-raydium-pool.sh

echo -e "${BLUE}Helper script created: ./create-raydium-pool.sh${NC}\n"
