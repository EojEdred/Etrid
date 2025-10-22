#!/bin/bash
# Genesis Configuration Generator for Ëtrid Protocol
# Creates production-ready genesis configurations for different environments
# Supports: Local, Testnet, Mainnet configurations

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ETRID_ROOT="${ETRID_ROOT:-$(cd "$SCRIPT_DIR/../.." && pwd)}"
OUTPUT_DIR="${OUTPUT_DIR:-$ETRID_ROOT/chain-specs}"
BIN_DIR="$ETRID_ROOT/target/release"

log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ============================================================================
# Genesis Configuration Templates
# ============================================================================

generate_local_config() {
    local output_file="$1"

    log_info "Generating LOCAL genesis configuration..."

    cat > "$output_file" <<'EOF'
{
  "name": "Ëtrid Local Testnet",
  "id": "etrid_local",
  "chainType": "Local",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "etr",
  "properties": {
    "tokenSymbol": "ËTR",
    "tokenDecimals": 18,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {},
      "balances": {
        "balances": [
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000000000],
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 1000000000000000000000],
          ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 1000000000000000000000],
          ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 1000000000000000000000],
          ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 1000000000000000000000]
        ]
      },
      "validatorCommittee": {
        "members": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
        ]
      },
      "asfConsensus": {
        "genesisValidators": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
        ],
        "epochDuration": 600,
        "votingTimeout": 30000,
        "finalityThreshold": 67
      },
      "edscToken": {
        "initialSupply": 0,
        "maxSupply": 1000000000000000000000000000
      },
      "edscRedemption": {
        "custodians": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
        ],
        "threshold": 2,
        "minCollateralRatio": 110,
        "circuitBreakerThreshold": 105
      },
      "sudo": {
        "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
      }
    }
  }
}
EOF

    log_success "Local genesis config written to: $output_file"
}

generate_testnet_config() {
    local output_file="$1"

    log_info "Generating TESTNET genesis configuration..."

    # Ask user for testnet parameters
    read -p "Testnet name [Ember]: " testnet_name
    testnet_name=${testnet_name:-Ember}

    read -p "Chain ID [etrid-testnet-stable2506]: " chain_id
    chain_id=${chain_id:-etrid-testnet-stable2506}

    read -p "Number of initial validators [5]: " num_validators
    num_validators=${num_validators:-5}

    read -p "Initial ËTR supply per validator (in ËTR) [1000000]: " initial_supply
    initial_supply=${initial_supply:-1000000}

    # Convert to wei (18 decimals)
    initial_supply_wei="${initial_supply}000000000000000000"

    log_info "Generating config with:"
    log_info "  Name: $testnet_name"
    log_info "  Chain ID: $chain_id"
    log_info "  Validators: $num_validators"
    log_info "  Initial Supply: $initial_supply ËTR per validator"

    cat > "$output_file" <<EOF
{
  "name": "Ëtrid $testnet_name Testnet",
  "id": "$chain_id",
  "chainType": "Live",
  "bootNodes": [],
  "telemetryEndpoints": [
    [
      "/dns/telemetry.polkadot.io/tcp/443/x-parity-wss/%2Fsubmit%2F",
      0
    ]
  ],
  "protocolId": "etr",
  "properties": {
    "tokenSymbol": "ËTR",
    "tokenDecimals": 18,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {},
      "balances": {
        "balances": []
      },
      "validatorCommittee": {
        "members": []
      },
      "asfConsensus": {
        "genesisValidators": [],
        "epochDuration": 600,
        "votingTimeout": 30000,
        "finalityThreshold": 67
      },
      "edscToken": {
        "initialSupply": 0,
        "maxSupply": "1000000000000000000000000000"
      },
      "edscRedemption": {
        "custodians": [],
        "threshold": 3,
        "minCollateralRatio": 110,
        "circuitBreakerThreshold": 105
      }
    }
  }
}
EOF

    log_success "Testnet genesis config written to: $output_file"
    log_warning "IMPORTANT: Add validator addresses manually before deployment"
}

generate_mainnet_config() {
    local output_file="$1"

    log_info "Generating MAINNET genesis configuration..."

    cat > "$output_file" <<'EOF'
{
  "name": "Ëtrid Mainnet",
  "id": "etrid-mainnet",
  "chainType": "Live",
  "bootNodes": [],
  "telemetryEndpoints": [
    [
      "/dns/telemetry.polkadot.io/tcp/443/x-parity-wss/%2Fsubmit%2F",
      0
    ]
  ],
  "protocolId": "etr",
  "properties": {
    "tokenSymbol": "ËTR",
    "tokenDecimals": 18,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "system": {},
      "balances": {
        "balances": []
      },
      "validatorCommittee": {
        "members": []
      },
      "asfConsensus": {
        "genesisValidators": [],
        "epochDuration": 600,
        "votingTimeout": 30000,
        "finalityThreshold": 67
      },
      "edscToken": {
        "initialSupply": 0,
        "maxSupply": "1000000000000000000000000000"
      },
      "edscRedemption": {
        "custodians": [],
        "threshold": 5,
        "minCollateralRatio": 120,
        "circuitBreakerThreshold": 110
      }
    }
  }
}
EOF

    log_success "Mainnet genesis config written to: $output_file"
    log_error "CRITICAL: Review and customize before mainnet deployment!"
    log_warning "- Add real validator addresses"
    log_warning "- Configure custodian multisig"
    log_warning "- Set initial token distribution"
    log_warning "- Remove or configure sudo key appropriately"
}

# ============================================================================
# Convert to Raw Chain Spec
# ============================================================================

convert_to_raw() {
    local input_file="$1"
    local output_file="$2"

    log_info "Converting to raw chain spec..."

    if [ ! -f "$BIN_DIR/flarechain-node" ]; then
        log_error "FlareChain binary not found at $BIN_DIR/flarechain-node"
        log_info "Build with: cargo build --release -p flarechain-node"
        return 1
    fi

    $BIN_DIR/flarechain-node build-spec \
        --chain "$input_file" \
        --raw \
        --disable-default-bootnode \
        > "$output_file" 2>&1

    if [ $? -eq 0 ]; then
        log_success "Raw chain spec generated: $output_file"
        return 0
    else
        log_error "Failed to generate raw chain spec"
        return 1
    fi
}

# ============================================================================
# Main Menu
# ============================================================================

show_menu() {
    clear
    echo -e "${PURPLE}"
    echo "╔══════════════════════════════════════════════════════════════╗"
    echo "║                                                              ║"
    echo "║         ËTRID GENESIS CONFIGURATION GENERATOR               ║"
    echo "║                                                              ║"
    echo "╚══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
    echo ""
    echo "Select genesis configuration type:"
    echo ""
    echo "  1) Local Development (3 validators, pre-funded accounts)"
    echo "  2) Public Testnet (customizable)"
    echo "  3) Mainnet (production configuration)"
    echo "  4) Exit"
    echo ""
    read -p "Enter choice [1-4]: " choice

    mkdir -p "$OUTPUT_DIR"

    case $choice in
        1)
            output_file="$OUTPUT_DIR/local-genesis.json"
            generate_local_config "$output_file"
            raw_file="$OUTPUT_DIR/local-genesis-raw.json"
            convert_to_raw "$output_file" "$raw_file"
            ;;
        2)
            output_file="$OUTPUT_DIR/testnet-genesis.json"
            generate_testnet_config "$output_file"
            raw_file="$OUTPUT_DIR/testnet-genesis-raw.json"
            convert_to_raw "$output_file" "$raw_file"
            ;;
        3)
            output_file="$OUTPUT_DIR/mainnet-genesis.json"
            generate_mainnet_config "$output_file"
            log_warning "Mainnet config requires manual validator setup"
            log_info "Convert to raw manually after adding validators"
            ;;
        4)
            log_info "Exiting..."
            exit 0
            ;;
        *)
            log_error "Invalid choice"
            sleep 2
            show_menu
            ;;
    esac

    echo ""
    log_success "Genesis configuration complete!"
    echo ""
    log_info "Output directory: $OUTPUT_DIR"
    echo ""

    read -p "Generate another configuration? (y/N): " again
    if [[ $again =~ ^[Yy]$ ]]; then
        show_menu
    fi
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    # Check dependencies
    if ! command -v jq &> /dev/null; then
        log_warning "jq not installed (optional, but recommended)"
        log_info "Install: brew install jq (macOS) or apt install jq (Linux)"
    fi

    show_menu
}

main
