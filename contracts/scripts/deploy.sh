#!/bin/bash

################################################################################
# ĒTRID Contract Deployment Script
#
# Orchestrates the complete deployment of all ĒTRID contracts in the correct
# order, wires them together, and verifies the deployment.
#
# Usage:
#   ./deploy.sh [network] [phase]
#
# Arguments:
#   network: devnet | testnet | mainnet (default: devnet)
#   phase: 1-6 | all (default: all)
#
# Examples:
#   ./deploy.sh devnet all          # Deploy everything to devnet
#   ./deploy.sh testnet 1           # Deploy Phase 1 only to testnet
#   ./deploy.sh mainnet             # Deploy everything to mainnet
################################################################################

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NETWORK=${1:-devnet}
PHASE=${2:-all}
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$SCRIPT_DIR/.."
ADDRESSES_FILE="$SCRIPT_DIR/deployed_addresses_${NETWORK}.json"
CONFIG_FILE="$SCRIPT_DIR/deployment_config_${NETWORK}.json"

# Print header
echo -e "${BLUE}"
echo "═══════════════════════════════════════════════════════════════"
echo "  ĒTRID CONTRACT DEPLOYMENT SCRIPT"
echo "═══════════════════════════════════════════════════════════════"
echo -e "${NC}"
echo "Network: $NETWORK"
echo "Phase: $PHASE"
echo "Addresses file: $ADDRESSES_FILE"
echo ""

# Check prerequisites
check_prerequisites() {
    echo -e "${BLUE}Checking prerequisites...${NC}"

    # Check cargo-contract
    if ! command -v cargo-contract &> /dev/null; then
        echo -e "${RED}ERROR: cargo-contract not found. Install with:${NC}"
        echo "  cargo install cargo-contract --force"
        exit 1
    fi

    # Check node running
    if ! curl -s http://localhost:9944 &> /dev/null; then
        echo -e "${RED}ERROR: No node running on localhost:9944${NC}"
        echo "Start a node first:"
        echo "  cd $PROJECT_ROOT/../05-multichain/primearc-core-chain"
        echo "  ./target/release/etrid-node --dev"
        exit 1
    fi

    # Check configuration file exists
    if [ ! -f "$CONFIG_FILE" ]; then
        echo -e "${YELLOW}WARNING: Config file not found. Creating default...${NC}"
        create_default_config
    fi

    # Initialize addresses file if needed
    if [ ! -f "$ADDRESSES_FILE" ]; then
        echo "{}" > "$ADDRESSES_FILE"
    fi

    echo -e "${GREEN}✓ Prerequisites check passed${NC}"
    echo ""
}

# Create default configuration
create_default_config() {
    cat > "$CONFIG_FILE" <<EOF
{
  "network": "$NETWORK",
  "node_url": "ws://localhost:9944",
  "deployer_account": "//Alice",
  "multisig_wallet": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "treasury": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
  "currencies": ["BTC", "ETH", "SOL", "BNB", "TRX", "XRP", "ADA", "DOGE", "LINK", "XLM", "MATIC"],
  "pool_allocations": {
    "BTC": {"etr": "845750000000000000000000000", "virtual": "3383000000"},
    "ETH": {"etr": "191400000000000000000000000", "virtual": "95700000000000000000"},
    "SOL": {"etr": "44500000000000000000000000", "virtual": "2225000000000000000000"},
    "BNB": {"etr": "40000000000000000000000000", "virtual": "160000000000000000000"},
    "TRX": {"etr": "6600000000000000000000000", "virtual": "660000000000000000000000"},
    "XRP": {"etr": "62400000000000000000000000", "virtual": "312000000000000000000000"},
    "ADA": {"etr": "15600000000000000000000000", "virtual": "78000000000000000000000"},
    "DOGE": {"etr": "26800000000000000000000000", "virtual": "2680000000000000000000000"},
    "LINK": {"etr": "8900000000000000000000000", "virtual": "890000000000000000000"},
    "XLM": {"etr": "4500000000000000000000000", "virtual": "45000000000000000000000"},
    "MATIC": {"etr": "3500000000000000000000000", "virtual": "7000000000000000000000"}
  },
  "edsc_reserves": {
    "usdc": "50000000000000000000000000",
    "usdt": "30000000000000000000000000",
    "dai": "20000000000000000000000000"
  }
}
EOF
    echo -e "${GREEN}Created default config at $CONFIG_FILE${NC}"
}

# Deploy Phase 1: Foundation
deploy_phase_1() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  PHASE 1: FOUNDATION LAYER${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""

    # Deploy Address Registry
    echo -e "${YELLOW}Deploying Address Registry...${NC}"
    bash "$SCRIPT_DIR/deploy_phase1_foundation.sh" "$NETWORK" "$ADDRESSES_FILE" "$CONFIG_FILE"

    echo -e "${GREEN}✓ Phase 1 complete${NC}"
    echo ""
}

# Deploy Phase 2: Tier 1 Pools
deploy_phase_2() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  PHASE 2: TIER 1 RESERVE POOLS${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""

    bash "$SCRIPT_DIR/deploy_phase2_tier1.sh" "$NETWORK" "$ADDRESSES_FILE" "$CONFIG_FILE"

    echo -e "${GREEN}✓ Phase 2 complete${NC}"
    echo ""
}

# Deploy Phase 3: Tier 2 Pools
deploy_phase_3() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  PHASE 3: TIER 2 TRADING POOLS${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""

    bash "$SCRIPT_DIR/deploy_phase3_tier2.sh" "$NETWORK" "$ADDRESSES_FILE" "$CONFIG_FILE"

    echo -e "${GREEN}✓ Phase 3 complete${NC}"
    echo ""
}

# Deploy Phase 4: EDSC System
deploy_phase_4() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  PHASE 4: EDSC STABLECOIN SYSTEM${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""

    bash "$SCRIPT_DIR/deploy_phase4_edsc.sh" "$NETWORK" "$ADDRESSES_FILE" "$CONFIG_FILE"

    echo -e "${GREEN}✓ Phase 4 complete${NC}"
    echo ""
}

# Deploy Phase 5: Intent Router
deploy_phase_5() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  PHASE 5: INTENT ROUTER SYSTEM${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""

    bash "$SCRIPT_DIR/deploy_phase5_router.sh" "$NETWORK" "$ADDRESSES_FILE" "$CONFIG_FILE"

    echo -e "${GREEN}✓ Phase 5 complete${NC}"
    echo ""
}

# Deploy Phase 6: Bridge Pallets
deploy_phase_6() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  PHASE 6: BRIDGE INFRASTRUCTURE${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""

    bash "$SCRIPT_DIR/deploy_phase6_bridge.sh" "$NETWORK" "$ADDRESSES_FILE" "$CONFIG_FILE"

    echo -e "${GREEN}✓ Phase 6 complete${NC}"
    echo ""
}

# Verify deployment
verify_deployment() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}  VERIFICATION${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
    echo ""

    bash "$SCRIPT_DIR/verify_deployment.sh" "$NETWORK" "$ADDRESSES_FILE"

    echo -e "${GREEN}✓ Verification complete${NC}"
    echo ""
}

# Main execution
main() {
    check_prerequisites

    case $PHASE in
        1)
            deploy_phase_1
            ;;
        2)
            deploy_phase_2
            ;;
        3)
            deploy_phase_3
            ;;
        4)
            deploy_phase_4
            ;;
        5)
            deploy_phase_5
            ;;
        6)
            deploy_phase_6
            ;;
        all)
            deploy_phase_1
            deploy_phase_2
            deploy_phase_3
            deploy_phase_4
            deploy_phase_5
            deploy_phase_6
            verify_deployment
            ;;
        *)
            echo -e "${RED}ERROR: Invalid phase '$PHASE'. Use 1-6 or 'all'${NC}"
            exit 1
            ;;
    esac

    echo -e "${GREEN}"
    echo "═══════════════════════════════════════════════════════════════"
    echo "  DEPLOYMENT COMPLETE"
    echo "═══════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo "Deployed addresses saved to: $ADDRESSES_FILE"
    echo ""
    echo "Next steps:"
    echo "  1. Review deployed addresses"
    echo "  2. Transfer admin roles to multi-sig wallet"
    echo "  3. Run integration tests"
    echo "  4. Monitor contracts for 24 hours"
    echo ""
}

# Run main
main
