#!/bin/bash

# Create ÉTR SPL Token on Solana
# Run this after setup-solana.sh

set -e

echo "🪙 Creating ÉTR SPL Token on Solana"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check if SPL Token CLI is installed
if ! command -v spl-token &> /dev/null; then
    echo -e "${RED}❌ SPL Token CLI not found${NC}"
    echo "Run: ./scripts/setup-solana.sh first"
    exit 1
fi

# Check network
NETWORK_URL=$(solana config get | grep "RPC URL" | awk '{print $3}')
if [[ "$NETWORK_URL" == *"devnet"* ]]; then
    NETWORK="Devnet"
elif [[ "$NETWORK_URL" == *"mainnet"* ]]; then
    NETWORK="Mainnet"
else
    NETWORK="Unknown"
fi

echo -e "📍 Network: ${YELLOW}$NETWORK${NC}"
echo -e "   RPC URL: $NETWORK_URL"
echo ""

# Check balance
BALANCE=$(solana balance 2>/dev/null || echo "0 SOL")
echo -e "💰 Balance: ${YELLOW}$BALANCE${NC}"

if [[ "$BALANCE" == "0 SOL" ]]; then
    echo -e "${RED}❌ Insufficient balance${NC}"
    if [[ "$NETWORK" == "Devnet" ]]; then
        echo "Get devnet SOL: solana airdrop 2"
        echo "Or visit: https://solfaucet.com/"
    else
        echo "You need to buy SOL and send to your wallet"
    fi
    exit 1
fi
echo ""

# Confirm deployment
if [[ "$NETWORK" == "Mainnet" ]]; then
    echo -e "${RED}⚠️  WARNING: You are deploying to MAINNET!${NC}"
    echo "This will cost real money (~$0.50)"
    echo ""
    echo -e "${YELLOW}Are you sure you want to continue? (yes/no)${NC}"
    read -r RESPONSE
    if [[ "$RESPONSE" != "yes" ]]; then
        echo "Deployment cancelled"
        exit 0
    fi
    echo ""
fi

# Token parameters
TOKEN_NAME="Etrid Coin"
TOKEN_SYMBOL="ÉTR"
TOKEN_DECIMALS=9

echo "📝 Token Configuration:"
echo "   Name:     $TOKEN_NAME"
echo "   Symbol:   $TOKEN_SYMBOL"
echo "   Decimals: $TOKEN_DECIMALS"
echo ""

# Create token
echo "⏳ Creating SPL token..."
echo ""

TOKEN_OUTPUT=$(spl-token create-token --decimals $TOKEN_DECIMALS 2>&1)
TOKEN_ADDRESS=$(echo "$TOKEN_OUTPUT" | grep -oE '[1-9A-HJ-NP-Za-km-z]{32,44}' | head -1)

if [ -z "$TOKEN_ADDRESS" ]; then
    echo -e "${RED}❌ Token creation failed${NC}"
    echo "$TOKEN_OUTPUT"
    exit 1
fi

echo -e "${GREEN}✅ Token created successfully!${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}🎉 ÉTR SPL TOKEN DEPLOYED${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 Token Details:"
echo -e "   Address: ${YELLOW}$TOKEN_ADDRESS${NC}"
echo "   Name:    $TOKEN_NAME"
echo "   Symbol:  $TOKEN_SYMBOL"
echo "   Decimals: $TOKEN_DECIMALS"
echo ""

# Create token account
echo "⏳ Creating token account..."
ACCOUNT_OUTPUT=$(spl-token create-account $TOKEN_ADDRESS 2>&1)
echo -e "${GREEN}✅ Token account created${NC}"
echo ""

# Save to file
cat > token-deployment.json << EOF
{
  "network": "$NETWORK",
  "tokenAddress": "$TOKEN_ADDRESS",
  "name": "$TOKEN_NAME",
  "symbol": "$TOKEN_SYMBOL",
  "decimals": $TOKEN_DECIMALS,
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo -e "${GREEN}💾 Deployment info saved to: token-deployment.json${NC}"
echo ""

# Explorer link
if [[ "$NETWORK" == "Devnet" ]]; then
    EXPLORER_URL="https://explorer.solana.com/address/$TOKEN_ADDRESS?cluster=devnet"
else
    EXPLORER_URL="https://explorer.solana.com/address/$TOKEN_ADDRESS"
fi

echo "🔍 View on Solana Explorer:"
echo "   $EXPLORER_URL"
echo ""

# Next steps
echo "📝 Next Steps:"
echo ""
if [[ "$NETWORK" == "Devnet" ]]; then
    echo "1. Test minting:"
    echo "   spl-token mint $TOKEN_ADDRESS 1000"
    echo ""
    echo "2. Check balance:"
    echo "   spl-token balance $TOKEN_ADDRESS"
    echo ""
    echo "3. Test transfer:"
    echo "   spl-token transfer $TOKEN_ADDRESS 100 <RECIPIENT_ADDRESS>"
    echo ""
else
    echo "1. Set token metadata (recommended):"
    echo "   # Upload logo to IPFS/Arweave"
    echo "   # Use Metaplex Token Metadata program"
    echo ""
    echo "2. Create Raydium pool:"
    echo "   https://raydium.io/liquidity/create/"
    echo ""
    echo "3. Submit to Jupiter aggregator:"
    echo "   https://station.jup.ag/partners/token-partner"
    echo ""
fi

echo "4. Save this token address:"
echo "   Add to .env or config file"
echo "   ETR_TOKEN_ADDRESS_SOLANA=$TOKEN_ADDRESS"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
