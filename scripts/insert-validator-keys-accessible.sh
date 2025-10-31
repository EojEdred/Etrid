#!/bin/bash
# Insert validator keys (AURA, GRANDPA, ASF) on all 16 accessible validators
# Keys are loaded from validator-keys-complete.json

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
KEYS_FILE="$HOME/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "🔑 Inserting Validator Keys on 16 Accessible Validators"
echo "======================================================="
echo ""

# Check if keys file exists
if [ ! -f "$KEYS_FILE" ]; then
    echo -e "${RED}❌ Keys file not found: $KEYS_FILE${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Keys file found${NC}"
echo ""

# Validator mappings (validator # => SSH host)
declare -A VALIDATOR_HOSTS=(
  [6]="consensus-dev01@20.224.104.239"
  [7]="runtime-dev01@108.142.205.177"
  [8]="runtime-dev01@4.180.238.67"
  [9]="compiler-dev01@4.180.59.25"
  [10]="compiler-dev01@98.71.91.84"
  [11]="multichain-dev01@68.219.230.63"
  [12]="multichain-dev01@98.71.219.106"
  [13]="oracle-dev01@172.167.8.217"
  [14]="audit-dev01@51.142.203.160"
  [15]="flarenode15@172.166.164.19"
  [16]="flarenode16@172.166.187.180"
  [17]="flarenode17@172.166.210.244"
  [18]="flarenode18@4.251.115.186"
  [19]="flarenode19@52.143.191.232"
  [20]="flarenode20@4.211.206.210"
  [21]="flarenode21@4.178.181.122"
)

# Function to extract keys from JSON
extract_keys() {
    local validator_num=$1
    local key_type=$2

    # Use Python to parse JSON (more reliable than jq on all systems)
    python3 << EOF
import json
import sys

with open('$KEYS_FILE', 'r') as f:
    data = json.load(f)

validator = data['validators'][$((validator_num - 1))]
session_keys = validator['sessionKeys']

if '$key_type' == 'seed':
    print(session_keys['seed'])
elif '$key_type' == 'aura':
    print(session_keys['auraKey'])
elif '$key_type' == 'grandpa':
    print(session_keys['grandpaKey'])
elif '$key_type' == 'asf':
    print(session_keys['asfKey'])
elif '$key_type' == 'phrase':
    print(session_keys['phrase'])
EOF
}

# Function to insert keys on a validator
insert_keys_on_validator() {
    local validator_num=$1
    local host=$2

    echo -e "${BLUE}=== Validator #${validator_num} ===${NC}"
    echo -e "${YELLOW}Host: ${host}${NC}"

    # Extract keys
    SEED=$(extract_keys $validator_num "seed")
    AURA_KEY=$(extract_keys $validator_num "aura")
    GRANDPA_KEY=$(extract_keys $validator_num "grandpa")
    ASF_KEY=$(extract_keys $validator_num "asf")

    echo "Extracted keys:"
    echo "  AURA:    ${AURA_KEY:0:20}..."
    echo "  GRANDPA: ${GRANDPA_KEY:0:20}..."
    echo "  ASF:     ${ASF_KEY:0:20}..."

    # Create RPC commands to insert keys
    ssh -i "$SSH_KEY" -o ConnectTimeout=10 "$host" << REMOTE
#!/bin/bash

# Wait for node to be ready (if running)
RPC_URL="http://localhost:9944"

echo "Inserting keys via RPC..."

# Insert AURA key
curl -s -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "author_insertKey",
  "params": ["aura", "$SEED", "$AURA_KEY"],
  "id": 1
}' \$RPC_URL | grep -q "result" && echo "✅ AURA key inserted" || echo "⏳ AURA (node may not be running yet)"

# Insert GRANDPA key
curl -s -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "author_insertKey",
  "params": ["gran", "$SEED", "$GRANDPA_KEY"],
  "id": 1
}' \$RPC_URL | grep -q "result" && echo "✅ GRANDPA key inserted" || echo "⏳ GRANDPA (node may not be running yet)"

# Insert ASF key
curl -s -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "author_insertKey",
  "params": ["asfi", "$SEED", "$ASF_KEY"],
  "id": 1
}' \$RPC_URL | grep -q "result" && echo "✅ ASF key inserted" || echo "⏳ ASF (node may not be running yet)"

# Alternative: Save keys to file for manual insertion later if node isn't running
mkdir -p ~/validator-keys
cat > ~/validator-keys/keys-validator-${validator_num}.json << 'KEYS'
{
  "validatorIndex": ${validator_num},
  "seed": "$SEED",
  "auraKey": "$AURA_KEY",
  "grandpaKey": "$GRANDPA_KEY",
  "asfKey": "$ASF_KEY"
}
KEYS

echo "✅ Keys also saved to ~/validator-keys/keys-validator-${validator_num}.json"
echo "   (Use these if RPC insertion failed)"
REMOTE

    echo -e "${GREEN}✅ Completed${NC}"
    echo ""
}

# Deploy to each validator
SUCCESS=0
FAILED=0

for validator_num in {6..21}; do
    host="${VALIDATOR_HOSTS[$validator_num]}"

    if [ -z "$host" ]; then
        continue
    fi

    if insert_keys_on_validator $validator_num "$host" 2>&1; then
        ((SUCCESS++))
    else
        echo -e "${RED}❌ Failed for Validator #${validator_num}${NC}"
        ((FAILED++))
    fi
done

echo ""
echo "=== Summary ==="
echo -e "${GREEN}✅ Successfully processed: ${SUCCESS}/16${NC}"
echo -e "${RED}❌ Failed: ${FAILED}/16${NC}"
echo ""
echo "Note: Keys were saved to ~/validator-keys/ on each validator"
echo "      If RPC insertion failed (node not running), you can insert manually later"
echo ""
echo "To insert keys manually after starting node:"
echo '  curl -H "Content-Type: application/json" -d '"'"'{'
echo '    "jsonrpc": "2.0",'
echo '    "method": "author_insertKey",'
echo '    "params": ["aura", "SEED_HERE", "KEY_HERE"],'
echo '    "id": 1'
echo '  }'"'"' http://localhost:9944'
