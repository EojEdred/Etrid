#!/usr/bin/env bash
# Insert validator keys (AURA, GRANDPA, ASF) on accessible validators #14-21
# Fixed to work with actual JSON structure from validator-keys-complete.json

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

    # Extract session keys seed using Python
    local session_seed=$(python3 -c "
import json
import sys

try:
    with open('$KEYS_FILE') as f:
        data = json.load(f)

    validators = data.get('validators', [])
    for v in validators:
        if v.get('validatorIndex') == $validator_num:
            session_keys = v.get('sessionKeys', {})
            seed = session_keys.get('seed', '')
            if seed:
                print(seed)
                sys.exit(0)

    print('ERROR: Validator $validator_num not found', file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f'ERROR: {e}', file=sys.stderr)
    sys.exit(1)
" 2>&1)

    if echo "$session_seed" | grep -q "ERROR:"; then
        echo -e "${RED}❌ Failed to extract session seed for validator #${validator_num}: $session_seed${NC}"
        return 1
    fi

    if [ -z "$session_seed" ]; then
        echo -e "${RED}❌ Empty session seed for validator #${validator_num}${NC}"
        return 1
    fi

    echo "  Session seed: ${session_seed:0:20}..."

    # SSH command to insert keys
    # The session seed contains all three keys - we'll derive them using substrate key derive
    local insert_cmd="
set -e

# Check if flarechain-node exists
if ! command -v flarechain-node &> /dev/null; then
    echo 'ERROR: flarechain-node not found in PATH'
    exit 1
fi

# Create keystore directory
sudo mkdir -p /var/lib/flarechain/chains/flarechain_mainnet/keystore

# Check if chainspec exists
if [ ! -f /etc/flarechain/flarechain_mainnet_chainspec.json ]; then
    echo 'WARNING: Chainspec not found, will use default'
    CHAIN_ARG='--chain flarechain'
else
    CHAIN_ARG='--chain /etc/flarechain/flarechain_mainnet_chainspec.json'
fi

# Insert AURA key (Sr25519)
echo 'Inserting AURA key...'
sudo flarechain-node key insert \
    --base-path /var/lib/flarechain \
    \$CHAIN_ARG \
    --scheme Sr25519 \
    --suri '$session_seed' \
    --key-type aura

# Insert GRANDPA key (Ed25519)
echo 'Inserting GRANDPA key...'
sudo flarechain-node key insert \
    --base-path /var/lib/flarechain \
    \$CHAIN_ARG \
    --scheme Ed25519 \
    --suri '$session_seed' \
    --key-type gran

# Insert ASF (Approval Voting) key (Sr25519)
echo 'Inserting ASF key...'
sudo flarechain-node key insert \
    --base-path /var/lib/flarechain \
    \$CHAIN_ARG \
    --scheme Sr25519 \
    --suri '$session_seed//asf' \
    --key-type asf_

echo '✅ All 3 keys inserted successfully!'

# List keystore contents
echo ''
echo 'Keystore contents:'
sudo ls -la /var/lib/flarechain/chains/flarechain_mainnet/keystore/ 2>/dev/null || echo 'Keystore directory created but empty (keys may be in different format)'

# Count keys
key_count=\$(sudo ls /var/lib/flarechain/chains/flarechain_mainnet/keystore/ 2>/dev/null | wc -l)
echo \"Keys in keystore: \$key_count\"
"

    # Execute on remote host
    local output
    output=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" "$insert_cmd" 2>&1)

    local exit_code=$?

    if [ $exit_code -eq 0 ] && echo "$output" | grep -q "successfully"; then
        echo -e "${GREEN}✅ Validator #${validator_num} - Keys inserted successfully${NC}"
        return 0
    else
        echo -e "${RED}❌ Validator #${validator_num} - Key insertion failed${NC}"
        echo "Output: $output"
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
    echo ""
    echo "Next steps:"
    echo "1. Verify keys are in keystore on each validator"
    echo "2. Start validator nodes with --validator flag"
    echo "3. Check validator appears in session keys"
    exit 0
elif [ $successful -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Partial success - ${successful} validators have keys inserted${NC}"
    echo "Review errors above and retry failed validators"
    exit 2
else
    echo -e "${RED}❌ All key insertions failed${NC}"
    echo "Check that:"
    echo "1. flarechain-node binary exists on validators"
    echo "2. SSH access is working"
    echo "3. Sufficient permissions to write to /var/lib/flarechain"
    exit 1
fi
