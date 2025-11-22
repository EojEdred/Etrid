#!/bin/bash
# V26 Batch ASF Key Registration Script
# Registers ASF keys for all validators in the network
# Usage: ./v26-batch-register-validators.sh

set -e
set -u
set -o pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configuration
SSH_KEY="${HOME}/.ssh/contabo-validators"
RPC_ENDPOINT="ws://localhost:9944"
REGISTRATION_SCRIPT="$(dirname "$0")/v26-register-asf-keys.js"
RESULTS_DIR="${HOME}/.etrid-v26-registration"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="$RESULTS_DIR/registration-results-$TIMESTAMP.log"

# Validator VM configuration
VALIDATOR_VMS=(
    "vmi2896906" "vmi2896907" "vmi2896908" "vmi2896909" "vmi2896910"
    "vmi2896911" "vmi2896912" "vmi2896913" "vmi2896914" "vmi2896915"
    "vmi2896916" "vmi2896917" "vmi2896918" "vmi2896919" "vmi2896920"
    "vmi2896921" "vmi2896922" "vmi2896923" "vmi2896924" "vmi2896925"
)

# Validator stash addresses (update these with your actual validator addresses)
declare -A VALIDATOR_ADDRESSES=(
    ["vmi2896906"]="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"  # Alice
    ["vmi2896907"]="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"  # Bob
    ["vmi2896908"]="5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"  # Charlie
    # Add remaining validators...
)

# Validator seed phrases (IMPORTANT: These should be loaded from secure storage)
# For production, use environment variables or encrypted key files
declare -A VALIDATOR_SEEDS=(
    ["vmi2896906"]="//Alice"  # Dev accounts for testing
    ["vmi2896907"]="//Bob"
    ["vmi2896908"]="//Charlie"
    # Add remaining validators...
)

# Statistics
TOTAL_VALIDATORS=${#VALIDATOR_VMS[@]}
SUCCESS_COUNT=0
FAILED_COUNT=0
SKIPPED_COUNT=0

declare -a SUCCESSFUL_VALIDATORS
declare -a FAILED_VALIDATORS
declare -a SKIPPED_VALIDATORS

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   V26 Batch ASF Key Registration${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Verify prerequisites
echo -e "${GREEN}Checking prerequisites...${NC}"

if [ ! -f "$SSH_KEY" ]; then
    echo -e "${RED}Error: SSH key not found: $SSH_KEY${NC}"
    exit 1
fi

if [ ! -f "$REGISTRATION_SCRIPT" ]; then
    echo -e "${RED}Error: Registration script not found: $REGISTRATION_SCRIPT${NC}"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js not found${NC}"
    echo "Please install Node.js"
    exit 1
fi

# Check if @polkadot/api is installed
if ! node -e "require('@polkadot/api')" 2>/dev/null; then
    echo -e "${RED}Error: @polkadot/api not installed${NC}"
    echo "Please run: npm install @polkadot/api"
    exit 1
fi

# Create results directory
mkdir -p "$RESULTS_DIR"

echo -e "${GREEN}Prerequisites OK${NC}"
echo ""

# Function to extract ASF public key from validator VM
extract_asf_public_key() {
    local vm_name=$1
    local vm_host="${vm_name}.contabo.host"  # Adjust domain as needed

    echo -e "${CYAN}Extracting ASF public key from $vm_name...${NC}"

    # Path to keystore on validator
    local keystore_path="\${HOME}/.local/share/flarechain/chains/flarechain_mainnet/keystore"

    # Find ASF key file (starts with 'asfk')
    local ssh_command="find $keystore_path -name 'asfk*' -type f 2>/dev/null | head -1"
    local key_file=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 root@"$vm_host" "$ssh_command" 2>/dev/null || echo "")

    if [ -z "$key_file" ]; then
        echo -e "${YELLOW}  No ASF key found in keystore${NC}"
        return 1
    fi

    # Extract public key from filename (format: asfk<public-key-hex>)
    local public_key=$(basename "$key_file" | sed 's/^asfk//')

    if [ ${#public_key} -ne 64 ]; then
        echo -e "${RED}  Invalid public key format: $public_key${NC}"
        return 1
    fi

    echo -e "${GREEN}  Public key: 0x$public_key${NC}"
    echo "0x$public_key"
    return 0
}

# Function to register a single validator
register_validator() {
    local vm_name=$1
    local validator_address=${VALIDATOR_ADDRESSES[$vm_name]:-}
    local validator_seed=${VALIDATOR_SEEDS[$vm_name]:-}

    echo -e "\n${BLUE}----------------------------------------${NC}"
    echo -e "${BLUE}Processing: $vm_name${NC}"
    echo -e "${BLUE}----------------------------------------${NC}"

    # Check if validator configuration exists
    if [ -z "$validator_address" ] || [ -z "$validator_seed" ]; then
        echo -e "${YELLOW}Skipping $vm_name: Missing configuration${NC}" | tee -a "$RESULTS_FILE"
        SKIPPED_VALIDATORS+=("$vm_name - Missing configuration")
        ((SKIPPED_COUNT++))
        return 1
    fi

    echo -e "  Address: $validator_address"

    # Extract ASF public key from VM
    local public_key
    if ! public_key=$(extract_asf_public_key "$vm_name"); then
        echo -e "${YELLOW}Skipping $vm_name: Could not extract public key${NC}" | tee -a "$RESULTS_FILE"
        SKIPPED_VALIDATORS+=("$vm_name - No ASF key in keystore")
        ((SKIPPED_COUNT++))
        return 1
    fi

    # Register the key on-chain
    echo -e "${CYAN}  Registering ASF key on-chain...${NC}"

    if node "$REGISTRATION_SCRIPT" \
        --public-key "$public_key" \
        --validator-uri "$validator_seed" \
        --rpc-endpoint "$RPC_ENDPOINT" 2>&1 | tee -a "$RESULTS_FILE"; then

        echo -e "${GREEN}SUCCESS: $vm_name registered${NC}" | tee -a "$RESULTS_FILE"
        SUCCESSFUL_VALIDATORS+=("$vm_name - $public_key")
        ((SUCCESS_COUNT++))
        return 0
    else
        echo -e "${RED}FAILED: $vm_name registration failed${NC}" | tee -a "$RESULTS_FILE"
        FAILED_VALIDATORS+=("$vm_name - Registration failed")
        ((FAILED_COUNT++))
        return 1
    fi
}

# Main registration loop
echo -e "${GREEN}Starting batch registration for $TOTAL_VALIDATORS validators...${NC}"
echo -e "Results will be saved to: ${BLUE}$RESULTS_FILE${NC}"
echo ""

for vm_name in "${VALIDATOR_VMS[@]}"; do
    register_validator "$vm_name" || true

    # Add a small delay between registrations to avoid overwhelming the node
    sleep 2
done

# Generate summary report
echo -e "\n${BLUE}========================================${NC}"
echo -e "${BLUE}   Registration Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

echo -e "Total Validators: ${CYAN}$TOTAL_VALIDATORS${NC}"
echo -e "Successful:       ${GREEN}$SUCCESS_COUNT${NC}"
echo -e "Failed:           ${RED}$FAILED_COUNT${NC}"
echo -e "Skipped:          ${YELLOW}$SKIPPED_COUNT${NC}"
echo ""

if [ ${#SUCCESSFUL_VALIDATORS[@]} -gt 0 ]; then
    echo -e "${GREEN}Successful Registrations:${NC}"
    for validator in "${SUCCESSFUL_VALIDATORS[@]}"; do
        echo -e "  ${GREEN}✓${NC} $validator"
    done
    echo ""
fi

if [ ${#FAILED_VALIDATORS[@]} -gt 0 ]; then
    echo -e "${RED}Failed Registrations:${NC}"
    for validator in "${FAILED_VALIDATORS[@]}"; do
        echo -e "  ${RED}✗${NC} $validator"
    done
    echo ""
fi

if [ ${#SKIPPED_VALIDATORS[@]} -gt 0 ]; then
    echo -e "${YELLOW}Skipped Validators:${NC}"
    for validator in "${SKIPPED_VALIDATORS[@]}"; do
        echo -e "  ${YELLOW}○${NC} $validator"
    done
    echo ""
fi

echo -e "Full log saved to: ${BLUE}$RESULTS_FILE${NC}"
echo ""

echo -e "${YELLOW}Next Steps:${NC}"
echo -e "  1. Review the registration results above"
echo -e "  2. For failed validators, check logs and retry manually"
echo -e "  3. Run verification script: ${BLUE}node scripts/v26-verify-asf-keys.js${NC}"
echo -e "  4. Restart all validator nodes to load ASF keys"
echo ""

if [ $FAILED_COUNT -eq 0 ]; then
    echo -e "${GREEN}All registrations completed successfully!${NC}"
    exit 0
else
    echo -e "${YELLOW}Some registrations failed. Please review and retry.${NC}"
    exit 1
fi
