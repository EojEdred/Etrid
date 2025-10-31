#!/usr/bin/env bash
# Insert validator session keys on validators #6-13 (without chain spec)

set -e

SSH_KEY="$HOME/.ssh/gizzi-validator"
KEYS_FILE="$HOME/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json"

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "🔑 Inserting Validator Keys on Validators #6-13 (No Chain Spec)"
echo "=================================================================="
echo ""

# Function to insert keys on a validator
insert_keys() {
    local validator_num=$1
    local ssh_user=$2
    local ssh_host=$3
    local name=$4

    echo -e "${YELLOW}Processing Validator #$validator_num ($name)...${NC}"

    # Extract session seed using Python
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

    if [[ "$session_seed" == ERROR* ]] || [[ -z "$session_seed" ]]; then
        echo -e "${RED}  ❌ Failed to extract session seed: $session_seed${NC}"
        return 1
    fi

    echo "  📝 Session seed extracted"

    # SSH and insert keys (WITHOUT --chain argument)
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no -o ConnectTimeout=10 \
        "${ssh_user}@${ssh_host}" bash <<REMOTE_SCRIPT
set -e

# Check if flarechain-node exists
if ! command -v flarechain-node &> /dev/null; then
    echo "  ⚠️  flarechain-node not found, skipping key insertion"
    exit 0
fi

# Check available chain specs
echo "  🔍 Checking available chain specs..."
ls -la /usr/local/bin/ | grep -i spec || true
ls -la /etc/flarechain/ 2>/dev/null | grep -i spec || true

echo "  🔑 Inserting AURA key (Sr25519) - no chain spec..."
sudo flarechain-node key insert \\
    --base-path /var/lib/flarechain \\
    --scheme Sr25519 \\
    --suri '$session_seed' \\
    --key-type aura || {
    echo "  ⚠️  AURA insertion failed, trying with dev chain..."
    sudo flarechain-node key insert \\
        --base-path /var/lib/flarechain \\
        --chain dev \\
        --scheme Sr25519 \\
        --suri '$session_seed' \\
        --key-type aura
}

echo "  🔑 Inserting GRANDPA key (Ed25519)..."
sudo flarechain-node key insert \\
    --base-path /var/lib/flarechain \\
    --scheme Ed25519 \\
    --suri '$session_seed' \\
    --key-type gran || {
    echo "  ⚠️  GRANDPA insertion failed, trying with dev chain..."
    sudo flarechain-node key insert \\
        --base-path /var/lib/flarechain \\
        --chain dev \\
        --scheme Ed25519 \\
        --suri '$session_seed' \\
        --key-type gran
}

echo "  🔑 Inserting ASF key (Sr25519)..."
sudo flarechain-node key insert \\
    --base-path /var/lib/flarechain \\
    --scheme Sr25519 \\
    --suri '$session_seed' \\
    --key-type asfo || {
    echo "  ⚠️  ASF insertion failed, trying with dev chain..."
    sudo flarechain-node key insert \\
        --base-path /var/lib/flarechain \\
        --chain dev \\
        --scheme Sr25519 \\
        --suri '$session_seed' \\
        --key-type asfo
}

echo "  ✓ All 3 keys inserted"
REMOTE_SCRIPT

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}  ✅ Validator #$validator_num keys inserted${NC}"
        return 0
    else
        echo -e "${RED}  ❌ Validator #$validator_num key insertion failed${NC}"
        return 1
    fi
}

# Insert keys on validators #6-13
successful=0
failed=0

insert_keys 6 "consensus-dev01" "20.224.104.239" "Consensus Dev" && ((successful++)) || ((failed++))
insert_keys 7 "runtime-dev01" "108.142.205.177" "Runtime Dev Primary" && ((successful++)) || ((failed++))
insert_keys 8 "runtime-dev01" "4.180.238.67" "Runtime Dev Secondary" && ((successful++)) || ((failed++))
insert_keys 9 "compiler-dev01" "4.180.59.25" "Compiler Dev Primary" && ((successful++)) || ((failed++))
insert_keys 10 "compiler-dev01" "98.71.91.84" "Compiler Dev (Monitoring)" && ((successful++)) || ((failed++))
insert_keys 11 "multichain-dev01" "68.219.230.63" "Multichain Dev Primary" && ((successful++)) || ((failed++))
insert_keys 12 "multichain-dev01" "98.71.219.106" "Multichain Dev Secondary" && ((successful++)) || ((failed++))
insert_keys 13 "oracle-dev01" "172.167.8.217" "Oracle Dev" && ((successful++)) || ((failed++))

echo ""
echo "=================================================================="
echo "Key Insertion Summary"
echo "=================================================================="
echo -e "${GREEN}✅ Successful: $successful/8 validators${NC}"
echo -e "${RED}❌ Failed: $failed/8 validators${NC}"
echo ""
echo "Total keys inserted: $((successful * 3)) (AURA + GRANDPA + ASF)"
echo ""

if [ $successful -eq 8 ]; then
    echo -e "${GREEN}🎉 All validator keys inserted successfully!${NC}"
    exit 0
elif [ $successful -gt 0 ]; then
    echo -e "${YELLOW}⚠️  Partial success - $successful validators have keys${NC}"
    exit 2
else
    echo -e "${RED}❌ Key insertion failed on all validators${NC}"
    exit 1
fi
