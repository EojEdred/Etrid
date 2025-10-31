#!/usr/bin/env bash
# Insert validator keys (AURA, GRANDPA, ASF) on accessible validators #14-21
# Compatible with macOS/Linux bash

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
KEYS_FILE="$HOME/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json"

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔑 Inserting Validator Keys on Validators #14-21"
echo "================================================="
echo ""

# Check if keys file exists
if [ ! -f "$KEYS_FILE" ]; then
    echo -e "${RED}❌ Keys file not found: $KEYS_FILE${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Keys file found${NC}"
echo ""

# Function to insert keys for a validator
insert_keys() {
    local validator_num=$1
    local ssh_user=$2
    local ssh_host=$3
    local validator_name=$4

    echo -e "${YELLOW}Inserting keys for Validator #${validator_num} (${validator_name}@${ssh_host})...${NC}"

    # Extract keys from JSON using Python
    local keys=$(python3 -c "
import json
import sys

with open('$KEYS_FILE') as f:
    data = json.load(f)

validators = data.get('validators', [])
validator = None
for v in validators:
    if v.get('number') == $validator_num:
        validator = v
        break

if not validator:
    print('ERROR: Validator $validator_num not found', file=sys.stderr)
    sys.exit(1)

print(json.dumps(validator))
" 2>&1)

    if echo "$keys" | grep -q "ERROR:"; then
        echo -e "${RED}❌ Failed to extract keys for validator #${validator_num}${NC}"
        return 1
    fi

    # Parse individual key components
    local aura_seed=$(echo "$keys" | python3 -c "import sys, json; v=json.load(sys.stdin); print(v['keys']['aura']['seed'])")
    local aura_type=$(echo "$keys" | python3 -c "import sys, json; v=json.load(sys.stdin); print(v['keys']['aura']['keyType'])")
    local grandpa_seed=$(echo "$keys" | python3 -c "import sys, json; v=json.load(sys.stdin); print(v['keys']['grandpa']['seed'])")
    local grandpa_type=$(echo "$keys" | python3 -c "import sys, json; v=json.load(sys.stdin); print(v['keys']['grandpa']['keyType'])")
    local asf_seed=$(echo "$keys" | python3 -c "import sys, json; v=json.load(sys.stdin); print(v['keys']['asf']['seed'])")
    local asf_type=$(echo "$keys" | python3 -c "import sys, json; v=json.load(sys.stdin); print(v['keys']['asf']['keyType'])")

    # SSH command to insert keys
    local insert_cmd="
set -e

# Create keystore directory if it doesn't exist
sudo mkdir -p /var/lib/flarechain/chains/flarechain_mainnet/keystore

# Insert AURA key
echo 'Inserting AURA key...'
flarechain-node key insert \
    --base-path /var/lib/flarechain \
    --chain /etc/flarechain/flarechain_mainnet_chainspec.json \
    --scheme Sr25519 \
    --suri '$aura_seed' \
    --key-type $aura_type

# Insert GRANDPA key
echo 'Inserting GRANDPA key...'
flarechain-node key insert \
    --base-path /var/lib/flarechain \
    --chain /etc/flarechain/flarechain_mainnet_chainspec.json \
    --scheme Ed25519 \
    --suri '$grandpa_seed' \
    --key-type $grandpa_type

# Insert ASF key
echo 'Inserting ASF key...'
flarechain-node key insert \
    --base-path /var/lib/flarechain \
    --chain /etc/flarechain/flarechain_mainnet_chainspec.json \
    --scheme Sr25519 \
    --suri '$asf_seed' \
    --key-type $asf_type

echo 'All keys inserted successfully!'

# List keys
echo 'Current keystore:'
sudo ls -la /var/lib/flarechain/chains/flarechain_mainnet/keystore/ || echo 'Keystore is empty'
"

    # Execute on remote host
    if ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" "$insert_cmd" 2>&1 | grep -q "successfully"; then
        echo -e "${GREEN}✅ Validator #${validator_num} - Keys inserted successfully${NC}"
        return 0
    else
        echo -e "${RED}❌ Validator #${validator_num} - Key insertion failed${NC}"
        return 1
    fi
}

# Validator configurations
successful=0
failed=0

# Validator #14
if insert_keys 14 "audit-dev01" "51.142.203.160" "audit-dev01"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Validator #15
if insert_keys 15 "flarenode15" "172.166.164.19" "flarenode15"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Validator #16
if insert_keys 16 "flarenode16" "172.166.187.180" "flarenode16"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Validator #17
if insert_keys 17 "flarenode17" "172.166.210.244" "flarenode17"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Validator #18
if insert_keys 18 "flarenode18" "4.251.115.186" "flarenode18"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Validator #19
if insert_keys 19 "flarenode19" "52.143.191.232" "flarenode19"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Validator #20
if insert_keys 20 "flarenode20" "4.211.206.210" "flarenode20"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Validator #21
if insert_keys 21 "flarenode21" "4.178.181.122" "flarenode21"; then
    ((successful++))
else
    ((failed++))
fi
echo ""

# Summary
echo "================================================="
echo "Key Insertion Summary"
echo "================================================="
echo -e "${GREEN}✅ Successful: ${successful}/8${NC}"
echo -e "${RED}❌ Failed: ${failed}/8${NC}"
echo ""

if [ $successful -eq 8 ]; then
    echo -e "${GREEN}🎉 All validator keys inserted successfully!${NC}"
    exit 0
elif [ $successful -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Partial success - some keys were inserted${NC}"
    exit 2
else
    echo -e "${RED}❌ All key insertions failed${NC}"
    exit 1
fi
