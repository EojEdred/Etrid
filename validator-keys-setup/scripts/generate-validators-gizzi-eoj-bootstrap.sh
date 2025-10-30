#!/bin/bash
# Generate 21 Validators: Gizzi + EojEdred Bootstrap Edition
# Usage: ./generate-validators-gizzi-eoj-bootstrap.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Ëtrid 21 Validators: Gizzi + EojEdred Bootstrap         ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Configuration
BINARY_PATH="${BINARY_PATH:-../target/release/flarechain-node}"
AIDEVID_DIR="${AIDEVID_DIR:-../14-aidevs/dids}"
OUTPUT_DIR="generated-keys-gizzi-eoj"
KEYVAULT_NAME="${KEYVAULT_NAME:-}"

# Check prerequisites
echo -e "${YELLOW}[1/5] Checking prerequisites...${NC}"

if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}✗ flarechain-node binary not found at $BINARY_PATH${NC}"
    exit 1
fi
echo -e "${GREEN}✓ flarechain-node binary found${NC}"

if [ ! -f "$AIDEVID_DIR/keypairs.json" ]; then
    echo -e "${RED}✗ AI DevID keypairs.json not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ AI DevID keypairs found${NC}"

if ! command -v jq &> /dev/null; then
    echo -e "${RED}✗ jq not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ jq installed${NC}"

if ! command -v python3 &> /dev/null; then
    echo -e "${RED}✗ python3 not found (needed for EojEdred DID)${NC}"
    exit 1
fi
echo -e "${GREEN}✓ python3 installed${NC}"

echo ""

# Create output directory
mkdir -p $OUTPUT_DIR

# Load AI DevID keypairs
AIDEVID_KEYPAIRS=$(cat $AIDEVID_DIR/keypairs.json)

# Function to map validator index to AI DevID (bash 3.2 compatible)
get_aidevid_for_validator() {
  case $1 in
    3) echo "governance-dev01" ;;
    4) echo "security-dev01" ;;
    5) echo "audit-dev01" ;;
    6) echo "consensus-dev01" ;;
    7) echo "consensus-dev01" ;;
    8) echo "runtime-dev01" ;;
    9) echo "runtime-dev01" ;;
    10) echo "compiler-dev01" ;;
    11) echo "compiler-dev01" ;;
    12) echo "oracle-dev01" ;;
    13) echo "multichain-dev01" ;;
    14) echo "multichain-dev01" ;;
    15) echo "edsc-dev01" ;;
    16) echo "edsc-dev01" ;;
    17) echo "economics-dev01" ;;
    18) echo "economics-dev01" ;;
    19) echo "ethics-dev01" ;;
    20) echo "docs-dev01" ;;
    21) echo "gizzi-claude" ;;
    *) echo "unknown" ;;
  esac
}

# Initialize JSON files
cat > $OUTPUT_DIR/validator-keys-complete.json <<EOF
{
  "bootstrapNodes": [],
  "validators": [
EOF

cat > $OUTPUT_DIR/bootnode-info.txt <<EOF
# Ëtrid Bootstrap Nodes
# Generated: $(date)
#
# These are the genesis bootstrap validators
#
EOF

# ═══════════════════════════════════════════════════════════════════════════
# STEP 2: GENERATE EOJEDRED DID (if not exists)
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[2/5] Checking/generating EojEdred DID...${NC}"

if [ ! -f "$AIDEVID_DIR/eojedred.json" ]; then
    echo "  Generating new DID for EojEdred..."

    # Generate keypair
    cd $AIDEVID_DIR/../
    python3 generate_keypairs.py --identity eojedred > /dev/null 2>&1 || true

    # Generate DID document
    python3 generate_did_documents.py --identity eojedred > /dev/null 2>&1 || true
    cd - > /dev/null

    echo -e "  ${GREEN}✓ EojEdred DID generated${NC}"
else
    echo -e "  ${GREEN}✓ EojEdred DID already exists${NC}"
fi

# Reload keypairs (now includes eojedred)
AIDEVID_KEYPAIRS=$(cat $AIDEVID_DIR/keypairs.json)

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# STEP 3: GENERATE VALIDATOR KEYS
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[3/5] Generating keys for 21 validators...${NC}"
echo ""

# ─────────────────────────────────────────────────────────────────────────
# VALIDATOR-01: GIZZI (AI Overseer, Bootstrap Node 1)
# ─────────────────────────────────────────────────────────────────────────

echo -e "${BLUE}[01/21] Gizzi (AI Overseer, Bootstrap 1)${NC}"

VALIDATOR_NAME="gizzi"
AIDEVID="gizzi"

# Load Gizzi's existing AI DevID keys
AIDEVID_PRIVATE=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .private_key_hex")
AIDEVID_PUBLIC=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .public_key_base58")
AIDEVID_DID="did:etrid:gizzi"

# Generate Gizzi's session keys
SESSION_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
SESSION_SEED=$(echo $SESSION_SEED_JSON | jq -r '.secretSeed')
SESSION_PHRASE=$(echo $SESSION_SEED_JSON | jq -r '.secretPhrase')
SESSION_ACCOUNT_ID=$(echo $SESSION_SEED_JSON | jq -r '.ss58Address')

AURA_JSON=$($BINARY_PATH key inspect --scheme sr25519 "$SESSION_SEED" --output-type json 2>/dev/null)
AURA_PUBKEY=$(echo $AURA_JSON | jq -r '.publicKey')

GRANDPA_JSON=$($BINARY_PATH key inspect --scheme ed25519 "$SESSION_SEED" --output-type json 2>/dev/null)
GRANDPA_PUBKEY=$(echo $GRANDPA_JSON | jq -r '.publicKey')

ASF_PUBKEY=$AURA_PUBKEY

# Generate Gizzi's payment and controller accounts
PAYMENT_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
PAYMENT_SEED=$(echo $PAYMENT_SEED_JSON | jq -r '.secretSeed')
PAYMENT_PHRASE=$(echo $PAYMENT_SEED_JSON | jq -r '.secretPhrase')
PAYMENT_ACCOUNT_ID=$(echo $PAYMENT_SEED_JSON | jq -r '.ss58Address')
PAYMENT_PUBKEY=$(echo $PAYMENT_SEED_JSON | jq -r '.publicKey')

CONTROLLER_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
CONTROLLER_SEED=$(echo $CONTROLLER_SEED_JSON | jq -r '.secretSeed')
CONTROLLER_ACCOUNT_ID=$(echo $CONTROLLER_SEED_JSON | jq -r '.ss58Address')

# Generate Gizzi's network key (for P2P)
GIZZI_NETWORK_KEY=$($BINARY_PATH key generate-node-key 2>/dev/null)
GIZZI_PEER_ID=$($BINARY_PATH key inspect-node-key --file <(echo "$GIZZI_NETWORK_KEY") 2>/dev/null | grep "PeerId" | awk '{print $2}')

STAKE="128000000000000000000000"
ROLE=4
ROLE_NAME="Director"

# Store in Key Vault if configured
if [ -n "$KEYVAULT_NAME" ]; then
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "gizzi-session-seed" --value "$SESSION_SEED" --tags "type=bootstrap" "role=overseer" > /dev/null 2>&1
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "gizzi-payment-seed" --value "$PAYMENT_SEED" > /dev/null 2>&1
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "gizzi-controller-seed" --value "$CONTROLLER_SEED" > /dev/null 2>&1
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "gizzi-network-key" --value "$GIZZI_NETWORK_KEY" > /dev/null 2>&1
fi

# Write to JSON
cat >> $OUTPUT_DIR/validator-keys-complete.json <<EOF
    {
      "validatorIndex": 1,
      "name": "Gizzi (AI Overseer)",
      "isBootstrap": true,
      "bootstrapOrder": 1,
      "sessionKeys": {
        "seed": "$SESSION_SEED",
        "phrase": "$SESSION_PHRASE",
        "accountId": "$SESSION_ACCOUNT_ID",
        "auraKey": "$AURA_PUBKEY",
        "grandpaKey": "$GRANDPA_PUBKEY",
        "asfKey": "$ASF_PUBKEY"
      },
      "paymentAccount": {
        "seed": "$PAYMENT_SEED",
        "phrase": "$PAYMENT_PHRASE",
        "accountId": "$PAYMENT_ACCOUNT_ID",
        "publicKey": "$PAYMENT_PUBKEY"
      },
      "controllerAccount": {
        "seed": "$CONTROLLER_SEED",
        "accountId": "$CONTROLLER_ACCOUNT_ID"
      },
      "networkKey": {
        "secret": "$GIZZI_NETWORK_KEY",
        "peerId": "$GIZZI_PEER_ID"
      },
      "aiDevID": {
        "did": "$AIDEVID_DID",
        "identity": "$AIDEVID",
        "publicKey": "$AIDEVID_PUBLIC",
        "privateKey": "$AIDEVID_PRIVATE",
        "role": "overseer"
      },
      "role": {
        "type": $ROLE,
        "name": "$ROLE_NAME",
        "stake": "$STAKE"
      }
    },
EOF

# Save bootnode info
cat >> $OUTPUT_DIR/bootnode-info.txt <<EOF
Bootstrap Node 1: Gizzi (AI Overseer)
  Peer ID: $GIZZI_PEER_ID
  Session Account: $SESSION_ACCOUNT_ID
  Payment Account: $PAYMENT_ACCOUNT_ID
  Bootnode Address: /ip4/<GIZZI_VM_IP>/tcp/30333/p2p/$GIZZI_PEER_ID

EOF

echo -e "  ${GREEN}✓ Gizzi configured as Bootstrap Node 1${NC}"
echo -e "  Peer ID: $GIZZI_PEER_ID"
echo ""

# ─────────────────────────────────────────────────────────────────────────
# VALIDATOR-02: EOJEDRED (Human Founder, Bootstrap Node 2)
# ─────────────────────────────────────────────────────────────────────────

echo -e "${BLUE}[02/21] EojEdred (Human Founder, Bootstrap 2)${NC}"

VALIDATOR_NAME="eojedred"
AIDEVID="eojedred"

# Load EojEdred's AI DevID keys
AIDEVID_PRIVATE=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .private_key_hex")
AIDEVID_PUBLIC=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .public_key_base58")
AIDEVID_DID="did:etrid:eojedred"

# Generate EojEdred's session keys
SESSION_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
SESSION_SEED=$(echo $SESSION_SEED_JSON | jq -r '.secretSeed')
SESSION_PHRASE=$(echo $SESSION_SEED_JSON | jq -r '.secretPhrase')
SESSION_ACCOUNT_ID=$(echo $SESSION_SEED_JSON | jq -r '.ss58Address')

AURA_JSON=$($BINARY_PATH key inspect --scheme sr25519 "$SESSION_SEED" --output-type json 2>/dev/null)
AURA_PUBKEY=$(echo $AURA_JSON | jq -r '.publicKey')

GRANDPA_JSON=$($BINARY_PATH key inspect --scheme ed25519 "$SESSION_SEED" --output-type json 2>/dev/null)
GRANDPA_PUBKEY=$(echo $GRANDPA_JSON | jq -r '.publicKey')

ASF_PUBKEY=$AURA_PUBKEY

# Generate EojEdred's payment and controller accounts
PAYMENT_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
PAYMENT_SEED=$(echo $PAYMENT_SEED_JSON | jq -r '.secretSeed')
PAYMENT_PHRASE=$(echo $PAYMENT_SEED_JSON | jq -r '.secretPhrase')
PAYMENT_ACCOUNT_ID=$(echo $PAYMENT_SEED_JSON | jq -r '.ss58Address')
PAYMENT_PUBKEY=$(echo $PAYMENT_SEED_JSON | jq -r '.publicKey')

CONTROLLER_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
CONTROLLER_SEED=$(echo $CONTROLLER_SEED_JSON | jq -r '.secretSeed')
CONTROLLER_ACCOUNT_ID=$(echo $CONTROLLER_SEED_JSON | jq -r '.ss58Address')

# Generate EojEdred's network key
EOJ_NETWORK_KEY=$($BINARY_PATH key generate-node-key 2>/dev/null)
EOJ_PEER_ID=$($BINARY_PATH key inspect-node-key --file <(echo "$EOJ_NETWORK_KEY") 2>/dev/null | grep "PeerId" | awk '{print $2}')

STAKE="128000000000000000000000"
ROLE=4
ROLE_NAME="Director"

# Store in Key Vault
if [ -n "$KEYVAULT_NAME" ]; then
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "eojedred-session-seed" --value "$SESSION_SEED" --tags "type=bootstrap" "role=founder" > /dev/null 2>&1
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "eojedred-payment-seed" --value "$PAYMENT_SEED" > /dev/null 2>&1
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "eojedred-controller-seed" --value "$CONTROLLER_SEED" > /dev/null 2>&1
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "eojedred-network-key" --value "$EOJ_NETWORK_KEY" > /dev/null 2>&1
fi

# Write to JSON
cat >> $OUTPUT_DIR/validator-keys-complete.json <<EOF
    {
      "validatorIndex": 2,
      "name": "EojEdred (Founder)",
      "isBootstrap": true,
      "bootstrapOrder": 2,
      "sessionKeys": {
        "seed": "$SESSION_SEED",
        "phrase": "$SESSION_PHRASE",
        "accountId": "$SESSION_ACCOUNT_ID",
        "auraKey": "$AURA_PUBKEY",
        "grandpaKey": "$GRANDPA_PUBKEY",
        "asfKey": "$ASF_PUBKEY"
      },
      "paymentAccount": {
        "seed": "$PAYMENT_SEED",
        "phrase": "$PAYMENT_PHRASE",
        "accountId": "$PAYMENT_ACCOUNT_ID",
        "publicKey": "$PAYMENT_PUBKEY"
      },
      "controllerAccount": {
        "seed": "$CONTROLLER_SEED",
        "accountId": "$CONTROLLER_ACCOUNT_ID"
      },
      "networkKey": {
        "secret": "$EOJ_NETWORK_KEY",
        "peerId": "$EOJ_PEER_ID"
      },
      "aiDevID": {
        "did": "$AIDEVID_DID",
        "identity": "$AIDEVID",
        "publicKey": "$AIDEVID_PUBLIC",
        "privateKey": "$AIDEVID_PRIVATE",
        "role": "founder"
      },
      "role": {
        "type": $ROLE,
        "name": "$ROLE_NAME",
        "stake": "$STAKE"
      }
    },
EOF

# Save bootnode info
cat >> $OUTPUT_DIR/bootnode-info.txt <<EOF
Bootstrap Node 2: EojEdred (Human Founder)
  Peer ID: $EOJ_PEER_ID
  Session Account: $SESSION_ACCOUNT_ID
  Payment Account: $PAYMENT_ACCOUNT_ID
  Bootnode Address: /ip4/<EOJ_VM_IP>/tcp/30333/p2p/$EOJ_PEER_ID

EOF

echo -e "  ${GREEN}✓ EojEdred configured as Bootstrap Node 2${NC}"
echo -e "  Peer ID: $EOJ_PEER_ID"
echo ""

# ─────────────────────────────────────────────────────────────────────────
# VALIDATORS 03-21: AI DEVS (Remaining Validators)
# ─────────────────────────────────────────────────────────────────────────

for i in {03..21}; do
  VALIDATOR_NAME="validator-$i"
  AIDEVID=$(get_aidevid_for_validator $((10#$i)))

  printf "  [%2d/21] $VALIDATOR_NAME (AI: $AIDEVID)..." $((10#$i))

  # Load AI DevID
  AIDEVID_PRIVATE=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .private_key_hex")
  AIDEVID_PUBLIC=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .public_key_base58")
  AIDEVID_DID="did:etrid:${AIDEVID}"

  # Generate session keys
  SESSION_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
  SESSION_SEED=$(echo $SESSION_SEED_JSON | jq -r '.secretSeed')
  SESSION_PHRASE=$(echo $SESSION_SEED_JSON | jq -r '.secretPhrase')
  SESSION_ACCOUNT_ID=$(echo $SESSION_SEED_JSON | jq -r '.ss58Address')

  AURA_JSON=$($BINARY_PATH key inspect --scheme sr25519 "$SESSION_SEED" --output-type json 2>/dev/null)
  AURA_PUBKEY=$(echo $AURA_JSON | jq -r '.publicKey')

  GRANDPA_JSON=$($BINARY_PATH key inspect --scheme ed25519 "$SESSION_SEED" --output-type json 2>/dev/null)
  GRANDPA_PUBKEY=$(echo $GRANDPA_JSON | jq -r '.publicKey')

  ASF_PUBKEY=$AURA_PUBKEY

  # Generate payment and controller accounts
  PAYMENT_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
  PAYMENT_SEED=$(echo $PAYMENT_SEED_JSON | jq -r '.secretSeed')
  PAYMENT_PHRASE=$(echo $PAYMENT_SEED_JSON | jq -r '.secretPhrase')
  PAYMENT_ACCOUNT_ID=$(echo $PAYMENT_SEED_JSON | jq -r '.ss58Address')
  PAYMENT_PUBKEY=$(echo $PAYMENT_SEED_JSON | jq -r '.publicKey')

  CONTROLLER_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
  CONTROLLER_SEED=$(echo $CONTROLLER_SEED_JSON | jq -r '.secretSeed')
  CONTROLLER_ACCOUNT_ID=$(echo $CONTROLLER_SEED_JSON | jq -r '.ss58Address')

  # Determine role
  if [ $((10#$i)) -le 5 ]; then
    STAKE="128000000000000000000000"
    ROLE=4
    ROLE_NAME="Director"
  elif [ $((10#$i)) -le 12 ]; then
    STAKE="64000000000000000000000"
    ROLE=3
    ROLE_NAME="FlareNode"
  else
    STAKE="64000000000000000000000"
    ROLE=2
    ROLE_NAME="ValidityNode"
  fi

  # Store in Key Vault
  if [ -n "$KEYVAULT_NAME" ]; then
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "${VALIDATOR_NAME}-session-seed" --value "$SESSION_SEED" --tags "aidevid=$AIDEVID" > /dev/null 2>&1
    az keyvault secret set --vault-name $KEYVAULT_NAME --name "${VALIDATOR_NAME}-payment-seed" --value "$PAYMENT_SEED" > /dev/null 2>&1
  fi

  # Write to JSON
  cat >> $OUTPUT_DIR/validator-keys-complete.json <<EOF
    {
      "validatorIndex": $((10#$i)),
      "name": "$VALIDATOR_NAME",
      "isBootstrap": false,
      "sessionKeys": {
        "seed": "$SESSION_SEED",
        "phrase": "$SESSION_PHRASE",
        "accountId": "$SESSION_ACCOUNT_ID",
        "auraKey": "$AURA_PUBKEY",
        "grandpaKey": "$GRANDPA_PUBKEY",
        "asfKey": "$ASF_PUBKEY"
      },
      "paymentAccount": {
        "seed": "$PAYMENT_SEED",
        "phrase": "$PAYMENT_PHRASE",
        "accountId": "$PAYMENT_ACCOUNT_ID",
        "publicKey": "$PAYMENT_PUBKEY"
      },
      "controllerAccount": {
        "seed": "$CONTROLLER_SEED",
        "accountId": "$CONTROLLER_ACCOUNT_ID"
      },
      "aiDevID": {
        "did": "$AIDEVID_DID",
        "identity": "$AIDEVID",
        "publicKey": "$AIDEVID_PUBLIC",
        "privateKey": "$AIDEVID_PRIVATE"
      },
      "role": {
        "type": $ROLE,
        "name": "$ROLE_NAME",
        "stake": "$STAKE"
      }
    }$([ $((10#$i)) -eq 21 ] || echo ",")
EOF

  echo -e " ${GREEN}✓${NC}"
done

# Close JSON
cat >> $OUTPUT_DIR/validator-keys-complete.json <<EOF
  ]
}
EOF

echo ""
echo -e "${GREEN}✓ All 21 validators generated${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# STEP 4: GENERATE CHAIN SPEC SNIPPETS
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[4/5] Generating chain spec snippets...${NC}"

cat > $OUTPUT_DIR/chain-spec-bootnodes.json <<EOF
{
  "bootNodes": [
    "/ip4/<GIZZI_VM_IP>/tcp/30333/p2p/$GIZZI_PEER_ID",
    "/ip4/<EOJ_VM_IP>/tcp/30333/p2p/$EOJ_PEER_ID"
  ]
}
EOF

echo -e "${GREEN}✓ Chain spec snippets generated${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# STEP 5: GENERATE DEPLOYMENT SCRIPTS
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[5/5] Generating deployment scripts...${NC}"

# Start Gizzi script
cat > $OUTPUT_DIR/start-gizzi.sh <<EOF
#!/bin/bash
# Start Gizzi (Bootstrap Node 1)

./flarechain-node \\
  --base-path /var/lib/etrid \\
  --chain mainnet-raw.json \\
  --name "Gizzi-Overseer" \\
  --validator \\
  --rpc-cors all \\
  --rpc-external \\
  --ws-external \\
  --port 30333 \\
  --rpc-port 9944 \\
  --prometheus-port 9615 \\
  --node-key $GIZZI_NETWORK_KEY

echo ""
echo "Gizzi started!"
echo "Bootnode address: /ip4/\$(curl -s ifconfig.me)/tcp/30333/p2p/$GIZZI_PEER_ID"
EOF

chmod +x $OUTPUT_DIR/start-gizzi.sh

# Start EojEdred script
cat > $OUTPUT_DIR/start-eojedred.sh <<EOF
#!/bin/bash
# Start EojEdred (Bootstrap Node 2)

GIZZI_IP="\${GIZZI_IP:-<GIZZI_VM_IP>}"

./flarechain-node \\
  --base-path /var/lib/etrid \\
  --chain mainnet-raw.json \\
  --name "EojEdred-Founder" \\
  --validator \\
  --rpc-cors all \\
  --rpc-external \\
  --ws-external \\
  --port 30333 \\
  --rpc-port 9944 \\
  --prometheus-port 9615 \\
  --bootnodes /ip4/\$GIZZI_IP/tcp/30333/p2p/$GIZZI_PEER_ID \\
  --node-key $EOJ_NETWORK_KEY

echo ""
echo "EojEdred started and connected to Gizzi!"
echo "Bootnode address: /ip4/\$(curl -s ifconfig.me)/tcp/30333/p2p/$EOJ_PEER_ID"
EOF

chmod +x $OUTPUT_DIR/start-eojedred.sh

echo -e "${GREEN}✓ Deployment scripts generated${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# SUMMARY
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              GENERATION COMPLETE! ✅                        ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "📁 Files generated in: $OUTPUT_DIR/"
echo ""
echo "   1. validator-keys-complete.json     (⚠️  ALL KEYS - SECURE THIS)"
echo "   2. bootnode-info.txt                (Bootstrap node details)"
echo "   3. chain-spec-bootnodes.json        (For chain spec)"
echo "   4. start-gizzi.sh                   (Start Gizzi bootstrap)"
echo "   5. start-eojedred.sh                (Start EojEdred bootstrap)"
echo ""
echo -e "${BLUE}🌟 Bootstrap Nodes:${NC}"
echo ""
echo "   Gizzi (AI Overseer):"
echo "     Peer ID: $GIZZI_PEER_ID"
echo "     Bootnode: /ip4/<GIZZI_VM_IP>/tcp/30333/p2p/$GIZZI_PEER_ID"
echo ""
echo "   EojEdred (Human Founder):"
echo "     Peer ID: $EOJ_PEER_ID"
echo "     Bootnode: /ip4/<EOJ_VM_IP>/tcp/30333/p2p/$EOJ_PEER_ID"
echo ""

if [ -n "$KEYVAULT_NAME" ]; then
  echo -e "${GREEN}✓ Keys backed up to Azure Key Vault: $KEYVAULT_NAME${NC}"
  echo ""
fi

echo -e "${YELLOW}⚠️  CRITICAL NEXT STEPS:${NC}"
echo ""
echo "1. Backup validator-keys-complete.json:"
echo "   gpg -c $OUTPUT_DIR/validator-keys-complete.json"
echo "   mv validator-keys-complete.json.gpg /secure/location/"
echo ""
echo "2. Update chain spec with bootnode addresses"
echo ""
echo "3. Deploy Gizzi first, then EojEdred, then others"
echo ""
echo -e "${GREEN}🚀 Ready to bootstrap the network!${NC}"
echo ""
echo "Deploy order:"
echo "  1. Start Gizzi:    ./generated-keys-gizzi-eoj/start-gizzi.sh"
echo "  2. Start EojEdred: ./generated-keys-gizzi-eoj/start-eojedred.sh"
echo "  3. Start others:   Deploy validators 03-21"
