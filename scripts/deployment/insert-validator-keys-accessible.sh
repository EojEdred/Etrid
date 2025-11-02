#!/bin/bash
# Insert validator keys (AURA, GRANDPA, ASF) on all 16 accessible validators
# Keys are loaded from validator-keys-complete.json
set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
KEYS_FILE="$HOME/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  ËTRID Validator Keys Insertion (16 Validators)              ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if keys file exists
if [ ! -f "$KEYS_FILE" ]; then
    echo -e "${RED}✗${NC} Keys file not found: $KEYS_FILE"
    exit 1
fi

# Array of accessible validators (validators #6-#21)
declare -A VALIDATORS=(
    [6]="runtime-dev01:20.224.104.239"
    [7]="compiler-dev01:98.71.91.84"
    [8]="network-dev01:20.169.114.25"
    [9]="sdk-dev01:20.75.92.203"
    [10]="devtools-dev01:20.55.31.30"
    [11]="api-dev01:20.73.34.17"
    [12]="docs-dev01:20.109.102.30"
    [13]="qa-dev01:52.250.61.132"
    [14]="perf-dev01:20.218.66.251"
    [15]="community-dev01:20.109.219.185"
    [16]="analytics-dev01:20.83.208.17"
    [17]="ethics-dev01:172.177.175.132"
    [18]="flarenode16:20.84.231.225"
    [19]="flarenode19:4.175.83.133"
    [20]="flarenode20:52.184.47.99"
    [21]="flarenode21:4.178.181.122"
)

SUCCESS_COUNT=0
FAIL_COUNT=0

# Function to insert keys via RPC
insert_keys() {
    local validator_num=$1
    local user=$2
    local ip=$3

    echo -e "${YELLOW}Processing Validator #${validator_num} (${user}@${ip})...${NC}"

    # Extract keys from JSON file for this validator
    local aura_seed=$(jq -r ".validators[$(($validator_num - 1))].keys.aura.seed" "$KEYS_FILE")
    local grandpa_seed=$(jq -r ".validators[$(($validator_num - 1))].keys.grandpa.seed" "$KEYS_FILE")
    local asf_seed=$(jq -r ".validators[$(($validator_num - 1))].keys.asf.seed" "$KEYS_FILE")

    if [ "$aura_seed" == "null" ] || [ "$grandpa_seed" == "null" ] || [ "$asf_seed" == "null" ]; then
        echo -e "${RED}✗${NC} Keys not found in JSON for validator #${validator_num}"
        ((FAIL_COUNT++))
        return 1
    fi

    # Insert AURA key
    echo "  Inserting AURA key..."
    if ! ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 "$user@$ip" \
        "curl -s -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"aura\",\"$aura_seed\",\"0x$(echo -n \"aura\" | xxd -p)\"],\"id\":1}' http://localhost:9933" > /dev/null; then
        echo -e "${RED}  ✗${NC} Failed to insert AURA key"
        ((FAIL_COUNT++))
        return 1
    fi

    # Insert GRANDPA key
    echo "  Inserting GRANDPA key..."
    if ! ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 "$user@$ip" \
        "curl -s -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"gran\",\"$grandpa_seed\",\"0x$(echo -n \"gran\" | xxd -p)\"],\"id\":1}' http://localhost:9933" > /dev/null; then
        echo -e "${RED}  ✗${NC} Failed to insert GRANDPA key"
        ((FAIL_COUNT++))
        return 1
    fi

    # Insert ASF key
    echo "  Inserting ASF key..."
    if ! ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 "$user@$ip" \
        "curl -s -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"asf\",\"$asf_seed\",\"0x$(echo -n \"asf\" | xxd -p)\"],\"id\":1}' http://localhost:9933" > /dev/null; then
        echo -e "${RED}  ✗${NC} Failed to insert ASF key"
        ((FAIL_COUNT++))
        return 1
    fi

    echo -e "${GREEN}✓${NC} Successfully inserted all keys for validator #${validator_num}"
    ((SUCCESS_COUNT++))
    echo ""
}

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo -e "${RED}✗${NC} jq is not installed. Please install it: brew install jq"
    exit 1
fi

# Insert keys for each validator
for validator_num in "${!VALIDATORS[@]}"; do
    IFS=: read -r user ip <<< "${VALIDATORS[$validator_num]}"
    insert_keys "$validator_num" "$user" "$ip" || true
done

echo ""
echo -e "${BLUE}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Key Insertion Summary                                       ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Successful:${NC} $SUCCESS_COUNT validators"
echo -e "${RED}Failed:${NC} $FAIL_COUNT validators"
echo ""

if [ $SUCCESS_COUNT -gt 0 ]; then
    echo -e "${YELLOW}Next Steps:${NC}"
    echo "1. Restart validators to load new keys"
    echo "2. Verify keys: curl -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"method\":\"author_hasKey\",\"params\":[\"<public_key>\",\"aura\"],\"id\":1}' http://localhost:9933"
    echo ""
fi
