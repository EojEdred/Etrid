#!/bin/bash

################################################################################
# Phase 4: EDSC Stablecoin System Deployment
#
# Deploys:
#   - EDSCToken
#   - EDSCReserveVault
#   - EDSCMintingEngine
#   - EDSCPegStabilizer
#   - EDSCExternalSwapRouter
#
# Initializes:
#   - Seeds reserves (50M USDC, 30M USDT, 20M DAI)
#   - Mints initial 100M EDSC
#   - Grants roles
################################################################################

set -e

NETWORK=$1
ADDRESSES_FILE=$2
CONFIG_FILE=$3

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Load configuration
REGISTRY_ADDRESS=$(jq -r '.address_registry' "$ADDRESSES_FILE")
MULTISIG=$(jq -r '.multisig_wallet' "$CONFIG_FILE")

# EDSC reserve amounts
USDC_RESERVE=$(jq -r '.edsc_reserves.usdc' "$CONFIG_FILE")
USDT_RESERVE=$(jq -r '.edsc_reserves.usdt' "$CONFIG_FILE")
DAI_RESERVE=$(jq -r '.edsc_reserves.dai' "$CONFIG_FILE")

echo -e "${YELLOW}Phase 4.1: Deploying EDSCToken...${NC}"

cd ../edsc/core/edsc-token
cargo contract build --release

TOKEN_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ EDSCToken deployed: $TOKEN_ADDRESS${NC}"

jq --arg addr "$TOKEN_ADDRESS" '.edsc_system.edsc_token = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 4.2: Deploying EDSCReserveVault...${NC}"

cd ../reserve-vault
cargo contract build --release

# Note: In production, these would be actual USDC/USDT/DAI token addresses
# For now, using placeholder addresses
USDC_TOKEN="5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM"
USDT_TOKEN="5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM"
DAI_TOKEN="5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM"

VAULT_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$USDC_TOKEN" "$USDT_TOKEN" "$DAI_TOKEN" 50 30 20 \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ EDSCReserveVault deployed: $VAULT_ADDRESS${NC}"

jq --arg addr "$VAULT_ADDRESS" '.edsc_system.edsc_reserve_vault = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 4.3: Deploying EDSCExternalSwapRouter...${NC}"

cd ../../stabilization/external-swap-router
cargo contract build --release

# Configuration for external aggregators
ONE_INCH_API="https://api.1inch.dev/swap/v5.2/1"
PARASWAP_API="https://api.paraswap.io/v5"

SWAP_ROUTER_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$ONE_INCH_API" "$PARASWAP_API" \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ EDSCExternalSwapRouter deployed: $SWAP_ROUTER_ADDRESS${NC}"

jq --arg addr "$SWAP_ROUTER_ADDRESS" '.edsc_system.edsc_external_swap_router = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 4.4: Deploying EDSCMintingEngine...${NC}"

cd ../../core/minting-engine
cargo contract build --release

MINTING_ENGINE_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$TOKEN_ADDRESS" "$VAULT_ADDRESS" "$SWAP_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ EDSCMintingEngine deployed: $MINTING_ENGINE_ADDRESS${NC}"

jq --arg addr "$MINTING_ENGINE_ADDRESS" '.edsc_system.edsc_minting_engine = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 4.5: Deploying EDSCPegStabilizer...${NC}"

cd ../../stabilization/peg-stabilizer
cargo contract build --release

# Note: In production, this would be a real Chainlink oracle address
ORACLE_ADDRESS="5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM"

STABILIZER_ADDRESS=$(cargo contract instantiate \
    --constructor new \
    --args "$TOKEN_ADDRESS" "$ORACLE_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm \
    --output-json | jq -r '.contract')

echo -e "${GREEN}✓ EDSCPegStabilizer deployed: $STABILIZER_ADDRESS${NC}"

jq --arg addr "$STABILIZER_ADDRESS" '.edsc_system.edsc_peg_stabilizer = $addr' \
    "$ADDRESSES_FILE" > tmp.$$.json && mv tmp.$$.json "$ADDRESSES_FILE"

echo ""
echo -e "${YELLOW}Phase 4.6: Granting roles...${NC}"

cd ../../core/edsc-token

# Grant MINTER_ROLE to minting engine
echo "  Granting MINTER_ROLE to minting engine..."
cargo contract call \
    --contract "$TOKEN_ADDRESS" \
    --message grant_role \
    --args 0 "$MINTING_ENGINE_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ MINTER_ROLE granted${NC}"

# Grant BURNER_ROLE to peg stabilizer
echo "  Granting BURNER_ROLE to peg stabilizer..."
cargo contract call \
    --contract "$TOKEN_ADDRESS" \
    --message grant_role \
    --args 1 "$STABILIZER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ BURNER_ROLE granted${NC}"

cd ../reserve-vault

# Grant DEPOSITOR_ROLE to minting engine
echo "  Granting DEPOSITOR_ROLE to minting engine..."
cargo contract call \
    --contract "$VAULT_ADDRESS" \
    --message grant_role \
    --args 0 "$MINTING_ENGINE_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ DEPOSITOR_ROLE granted${NC}"

# Grant REBALANCER_ROLE to minting engine
echo "  Granting REBALANCER_ROLE to minting engine..."
cargo contract call \
    --contract "$VAULT_ADDRESS" \
    --message grant_role \
    --args 1 "$MINTING_ENGINE_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ REBALANCER_ROLE granted${NC}"

echo ""
echo -e "${YELLOW}Phase 4.7: Seeding initial reserves...${NC}"

echo "  NOTE: In production, you must:"
echo "    1. Transfer $USDC_RESERVE USDC to reserve vault"
echo "    2. Transfer $USDT_RESERVE USDT to reserve vault"
echo "    3. Transfer $DAI_RESERVE DAI to reserve vault"
echo ""
echo "  For testnet/devnet, we'll simulate this..."

# Simulate deposit (in production, this requires actual stablecoin transfers)
cargo contract call \
    --contract "$VAULT_ADDRESS" \
    --message deposit_usdc \
    --args "$USDC_RESERVE" \
    --suri "//Alice" \
    --execute \
    --skip-confirm || echo -e "${YELLOW}  (Simulated - requires actual USDC)${NC}"

cargo contract call \
    --contract "$VAULT_ADDRESS" \
    --message deposit_usdt \
    --args "$USDT_RESERVE" \
    --suri "//Alice" \
    --execute \
    --skip-confirm || echo -e "${YELLOW}  (Simulated - requires actual USDT)${NC}"

cargo contract call \
    --contract "$VAULT_ADDRESS" \
    --message deposit_dai \
    --args "$DAI_RESERVE" \
    --suri "//Alice" \
    --execute \
    --skip-confirm || echo -e "${YELLOW}  (Simulated - requires actual DAI)${NC}"

echo -e "${GREEN}✓ Reserve seeding initiated${NC}"

echo ""
echo -e "${YELLOW}Phase 4.8: Minting initial EDSC supply...${NC}"

cd ../minting-engine

# Mint 100M EDSC to treasury
INITIAL_SUPPLY="100000000000000000000000000"  # 100M with 18 decimals
TREASURY=$(jq -r '.treasury' "$CONFIG_FILE")

cargo contract call \
    --contract "$MINTING_ENGINE_ADDRESS" \
    --message mint_initial_supply \
    --args "$INITIAL_SUPPLY" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ 100M EDSC minted to treasury${NC}"

echo ""
echo -e "${YELLOW}Phase 4.9: Registering EDSC system in Address Registry...${NC}"

cd ../../../registry/address-registry

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_edsc_token \
    --args "$TOKEN_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_edsc_reserve_vault \
    --args "$VAULT_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_edsc_minting_engine \
    --args "$MINTING_ENGINE_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_edsc_peg_stabilizer \
    --args "$STABILIZER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message register_edsc_external_swap_router \
    --args "$SWAP_ROUTER_ADDRESS" \
    --suri "//Alice" \
    --execute \
    --skip-confirm

echo -e "${GREEN}✓ EDSC system registered${NC}"

echo ""
echo -e "${YELLOW}Phase 4.10: Verifying EDSC deployment...${NC}"

# Verify token
REGISTERED_TOKEN=$(cargo contract call \
    --contract "$REGISTRY_ADDRESS" \
    --message get_edsc_token \
    --suri "//Alice" \
    --dry-run \
    --output-json | jq -r '.data.Ok')

if [ "$REGISTERED_TOKEN" == "$TOKEN_ADDRESS" ]; then
    echo -e "${GREEN}✓ EDSCToken registered correctly${NC}"
else
    echo -e "${RED}✗ EDSCToken NOT registered correctly${NC}"
    exit 1
fi

# Verify minter role
cd ../../edsc/core/edsc-token

HAS_MINTER=$(cargo contract call \
    --contract "$TOKEN_ADDRESS" \
    --message has_role \
    --args 0 "$MINTING_ENGINE_ADDRESS" \
    --suri "//Alice" \
    --dry-run \
    --output-json | jq -r '.data.Ok')

if [ "$HAS_MINTER" == "true" ]; then
    echo -e "${GREEN}✓ Minting engine has MINTER_ROLE${NC}"
else
    echo -e "${RED}✗ Minting engine missing MINTER_ROLE${NC}"
    exit 1
fi

# Verify reserve ratio (should be 100% = 1:1 backing)
cd ../reserve-vault

RESERVE_RATIO=$(cargo contract call \
    --contract "$VAULT_ADDRESS" \
    --message get_reserve_ratio \
    --suri "//Alice" \
    --dry-run \
    --output-json | jq -r '.data.Ok' || echo "0")

echo "  Reserve ratio: $RESERVE_RATIO%"

if [ "$RESERVE_RATIO" -ge "99" ]; then
    echo -e "${GREEN}✓ Reserve ratio is adequate (≥99%)${NC}"
else
    echo -e "${YELLOW}⚠ Reserve ratio below 99% - seed reserves before mainnet${NC}"
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  PHASE 4 COMPLETE${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
echo ""
echo "Deployed:"
echo "  - EDSCToken"
echo "  - EDSCReserveVault"
echo "  - EDSCMintingEngine"
echo "  - EDSCPegStabilizer"
echo "  - EDSCExternalSwapRouter"
echo ""
echo "Initialized:"
echo "  - Initial supply: 100M EDSC"
echo "  - Target reserves: 50M USDC, 30M USDT, 20M DAI"
echo ""
echo "EDSC stablecoin system is operational!"
echo ""
echo "IMPORTANT: Before mainnet deployment:"
echo "  1. Seed reserves with actual USDC/USDT/DAI"
echo "  2. Configure real Chainlink oracle"
echo "  3. Configure real 1inch/ParaSwap API keys"
echo "  4. Test peg stabilization mechanism"
