#!/bin/bash

# Solana Development Environment Setup Script
# This script installs everything needed for Solana development

set -e  # Exit on error

echo "🌟 Solana Development Environment Setup"
echo "========================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running on macOS or Linux
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macOS"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="Linux"
else
    echo -e "${RED}❌ Unsupported OS: $OSTYPE${NC}"
    echo "This script supports macOS and Linux only"
    exit 1
fi

echo -e "${GREEN}✅ Detected OS: $OS${NC}"
echo ""

# Step 1: Check/Install Rust and Cargo
echo "📦 Step 1: Checking Rust installation..."
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✅ Rust already installed: $CARGO_VERSION${NC}"
else
    echo -e "${YELLOW}⚠️  Rust not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo -e "${GREEN}✅ Rust installed successfully${NC}"
fi
echo ""

# Step 2: Install Solana CLI
echo "🔧 Step 2: Installing Solana CLI..."
if command -v solana &> /dev/null; then
    SOLANA_VERSION=$(solana --version)
    echo -e "${GREEN}✅ Solana CLI already installed: $SOLANA_VERSION${NC}"
else
    echo -e "${YELLOW}⚠️  Solana CLI not found. Installing...${NC}"
    sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
    export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
    echo -e "${GREEN}✅ Solana CLI installed${NC}"
fi
echo ""

# Step 3: Install SPL Token CLI
echo "🪙 Step 3: Installing SPL Token CLI..."
if command -v spl-token &> /dev/null; then
    SPL_VERSION=$(spl-token --version)
    echo -e "${GREEN}✅ SPL Token CLI already installed: $SPL_VERSION${NC}"
else
    echo -e "${YELLOW}⚠️  SPL Token CLI not found. Installing...${NC}"
    cargo install spl-token-cli
    echo -e "${GREEN}✅ SPL Token CLI installed${NC}"
fi
echo ""

# Step 4: Configure for Devnet
echo "🌐 Step 4: Configuring Solana for devnet..."
solana config set --url https://api.devnet.solana.com
echo -e "${GREEN}✅ Configured for devnet${NC}"
echo ""

# Step 5: Check if wallet exists, create if not
echo "👛 Step 5: Checking wallet..."
WALLET_PATH="$HOME/.config/solana/devnet-wallet.json"
if [ -f "$WALLET_PATH" ]; then
    echo -e "${GREEN}✅ Wallet already exists: $WALLET_PATH${NC}"
    WALLET_ADDRESS=$(solana address --keypair $WALLET_PATH)
    echo -e "   Address: ${YELLOW}$WALLET_ADDRESS${NC}"
else
    echo -e "${YELLOW}⚠️  No wallet found. Creating new wallet...${NC}"
    solana-keygen new --outfile $WALLET_PATH --no-bip39-passphrase
    solana config set --keypair $WALLET_PATH
    WALLET_ADDRESS=$(solana address)
    echo -e "${GREEN}✅ New wallet created${NC}"
    echo -e "   Address: ${YELLOW}$WALLET_ADDRESS${NC}"
    echo ""
    echo -e "${RED}⚠️  IMPORTANT: Back up your wallet!${NC}"
    echo "   Wallet file: $WALLET_PATH"
fi
echo ""

# Step 6: Check balance and offer airdrop
echo "💰 Step 6: Checking SOL balance..."
BALANCE=$(solana balance 2>/dev/null || echo "0 SOL")
echo "   Current balance: $BALANCE"
echo ""

if [[ "$BALANCE" == "0 SOL" ]]; then
    echo -e "${YELLOW}Would you like to request an airdrop? (y/n)${NC}"
    read -r RESPONSE
    if [[ "$RESPONSE" =~ ^[Yy]$ ]]; then
        echo "Requesting 2 SOL airdrop..."
        solana airdrop 2 || {
            echo -e "${RED}❌ Airdrop failed (rate limited or network issue)${NC}"
            echo "Try web faucet: https://solfaucet.com/"
            echo "Your address: $WALLET_ADDRESS"
        }
        BALANCE=$(solana balance)
        echo -e "${GREEN}✅ New balance: $BALANCE${NC}"
    fi
fi
echo ""

# Summary
echo "========================================"
echo "🎉 Setup Complete!"
echo "========================================"
echo ""
echo "📋 Summary:"
echo "  ✅ Rust/Cargo installed"
echo "  ✅ Solana CLI installed"
echo "  ✅ SPL Token CLI installed"
echo "  ✅ Configured for devnet"
echo "  ✅ Wallet ready"
echo "  📍 Wallet address: $WALLET_ADDRESS"
echo "  💰 Balance: $BALANCE"
echo ""
echo "📝 Next steps:"
echo "  1. Run: ./scripts/create-token.sh (to create ÉTR token)"
echo "  2. Or manually: spl-token create-token --decimals 9"
echo ""
echo "⚠️  Don't forget to add Solana to your PATH:"
echo "    export PATH=\"\$HOME/.local/share/solana/install/active_release/bin:\$PATH\""
echo "    (Add this to ~/.bashrc or ~/.zshrc)"
echo ""
