#!/bin/bash
# Generate 21 Validators with Payment Accounts + AI DevIDs
# Usage: ./generate-validators-with-payment-aidevid.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Generate 21 Validators: Session + Payment + AI DevID     ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Configuration
BINARY_PATH="${BINARY_PATH:-../target/release/flarechain-node}"
AIDEVID_DIR="${AIDEVID_DIR:-../14-aidevs/dids}"
OUTPUT_DIR="generated-keys"
KEYVAULT_NAME="${KEYVAULT_NAME:-}"

# Check prerequisites
echo -e "${YELLOW}[1/4] Checking prerequisites...${NC}"

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

# Optional: Azure Key Vault
if [ -n "$KEYVAULT_NAME" ]; then
    if ! command -v az &> /dev/null; then
        echo -e "${RED}✗ Azure CLI not found (needed for Key Vault)${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Azure CLI found (Key Vault enabled)${NC}"
else
    echo -e "${YELLOW}⚠ KEYVAULT_NAME not set (skipping Key Vault backup)${NC}"
fi

echo ""

# Load AI DevID keypairs
AIDEVID_KEYPAIRS=$(cat $AIDEVID_DIR/keypairs.json)

# AI DevID mapping (validator index → AI DevID)
declare -A VALIDATOR_TO_AIDEVID=(
  [1]="governance-dev01"
  [2]="security-dev01"
  [3]="audit-dev01"
  [4]="consensus-dev01"
  [5]="consensus-dev01"
  [6]="runtime-dev01"
  [7]="runtime-dev01"
  [8]="compiler-dev01"
  [9]="compiler-dev01"
  [10]="multichain-dev01"
  [11]="multichain-dev01"
  [12]="oracle-dev01"
  [13]="edsc-dev01"
  [14]="edsc-dev01"
  [15]="economics-dev01"
  [16]="economics-dev01"
  [17]="ethics-dev01"
  [18]="ethics-dev01"
  [19]="docs-dev01"
  [20]="docs-dev01"
  [21]="docs-dev01"
)

# Create output directory
mkdir -p $OUTPUT_DIR

# Initialize JSON files
cat > $OUTPUT_DIR/validator-keys-complete.json <<EOF
{
  "validators": [
EOF

cat > $OUTPUT_DIR/payment-accounts.txt <<EOF
# Ëtrid Validator Payment Accounts
# Generated: $(date)
#
# Format: ValidatorName PaymentAccount ControllerAccount AI_DevID
#
EOF

cat > $OUTPUT_DIR/chain-spec-genesis.json <<EOF
{
  "validatorCommittee": {
    "validators": [
EOF

echo -e "${YELLOW}[2/4] Generating keys for 21 validators...${NC}"
echo "This will take ~3-5 minutes..."
echo ""

for i in {01..21}; do
  VALIDATOR_NAME="validator-$i"
  AIDEVID="${VALIDATOR_TO_AIDEVID[$((10#$i))]}"

  printf "  [%2d/21] $VALIDATOR_NAME (AI: $AIDEVID)..." $((10#$i))

  # ═══════════════════════════════════════════════════════════
  # 1. SESSION KEYS (Hot - on validator VM)
  # ═══════════════════════════════════════════════════════════

  SESSION_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
  SESSION_SEED=$(echo $SESSION_SEED_JSON | jq -r '.secretSeed')
  SESSION_PHRASE=$(echo $SESSION_SEED_JSON | jq -r '.secretPhrase')
  SESSION_ACCOUNT_ID=$(echo $SESSION_SEED_JSON | jq -r '.ss58Address')

  # Derive session keys
  AURA_JSON=$($BINARY_PATH key inspect --scheme sr25519 "$SESSION_SEED" --output-type json 2>/dev/null)
  AURA_PUBKEY=$(echo $AURA_JSON | jq -r '.publicKey')

  GRANDPA_JSON=$($BINARY_PATH key inspect --scheme ed25519 "$SESSION_SEED" --output-type json 2>/dev/null)
  GRANDPA_PUBKEY=$(echo $GRANDPA_JSON | jq -r '.publicKey')

  ASF_PUBKEY=$AURA_PUBKEY

  # ═══════════════════════════════════════════════════════════
  # 2. PAYMENT ACCOUNT (Cold - receives rewards)
  # ═══════════════════════════════════════════════════════════

  PAYMENT_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
  PAYMENT_SEED=$(echo $PAYMENT_SEED_JSON | jq -r '.secretSeed')
  PAYMENT_PHRASE=$(echo $PAYMENT_SEED_JSON | jq -r '.secretPhrase')
  PAYMENT_ACCOUNT_ID=$(echo $PAYMENT_SEED_JSON | jq -r '.ss58Address')
  PAYMENT_PUBKEY=$(echo $PAYMENT_SEED_JSON | jq -r '.publicKey')

  # ═══════════════════════════════════════════════════════════
  # 3. CONTROLLER ACCOUNT (Warm - manages validator)
  # ═══════════════════════════════════════════════════════════

  CONTROLLER_SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
  CONTROLLER_SEED=$(echo $CONTROLLER_SEED_JSON | jq -r '.secretSeed')
  CONTROLLER_PHRASE=$(echo $CONTROLLER_SEED_JSON | jq -r '.secretPhrase')
  CONTROLLER_ACCOUNT_ID=$(echo $CONTROLLER_SEED_JSON | jq -r '.ss58Address')
  CONTROLLER_PUBKEY=$(echo $CONTROLLER_SEED_JSON | jq -r '.publicKey')

  # ═══════════════════════════════════════════════════════════
  # 4. AI DEVID KEYS (Warm - identity verification)
  # ═══════════════════════════════════════════════════════════

  AIDEVID_PRIVATE=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .private_key_hex")
  AIDEVID_PUBLIC=$(echo $AIDEVID_KEYPAIRS | jq -r ".[] | select(.identity == \"$AIDEVID\") | .public_key_base58")
  AIDEVID_DID="did:etrid:${AIDEVID}"

  # ═══════════════════════════════════════════════════════════
  # 5. DETERMINE ROLE AND STAKE
  # ═══════════════════════════════════════════════════════════

  if [ $((10#$i)) -le 3 ]; then
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

  # ═══════════════════════════════════════════════════════════
  # 6. STORE IN AZURE KEY VAULT (if configured)
  # ═══════════════════════════════════════════════════════════

  if [ -n "$KEYVAULT_NAME" ]; then
    az keyvault secret set \
      --vault-name $KEYVAULT_NAME \
      --name "${VALIDATOR_NAME}-session-seed" \
      --value "$SESSION_SEED" \
      --tags "validator=$VALIDATOR_NAME" "type=session_seed" "aidevid=$AIDEVID" \
      > /dev/null 2>&1

    az keyvault secret set \
      --vault-name $KEYVAULT_NAME \
      --name "${VALIDATOR_NAME}-payment-seed" \
      --value "$PAYMENT_SEED" \
      --tags "validator=$VALIDATOR_NAME" "type=payment_seed" \
      > /dev/null 2>&1

    az keyvault secret set \
      --vault-name $KEYVAULT_NAME \
      --name "${VALIDATOR_NAME}-payment-account" \
      --value "$PAYMENT_ACCOUNT_ID" \
      > /dev/null 2>&1

    az keyvault secret set \
      --vault-name $KEYVAULT_NAME \
      --name "${VALIDATOR_NAME}-controller-seed" \
      --value "$CONTROLLER_SEED" \
      > /dev/null 2>&1

    az keyvault secret set \
      --vault-name $KEYVAULT_NAME \
      --name "${VALIDATOR_NAME}-aidevid" \
      --value "$AIDEVID_DID" \
      > /dev/null 2>&1
  fi

  # ═══════════════════════════════════════════════════════════
  # 7. WRITE TO JSON FILES
  # ═══════════════════════════════════════════════════════════

  # Complete key export
  cat >> $OUTPUT_DIR/validator-keys-complete.json <<EOF
    {
      "validatorIndex": $((10#$i)),
      "name": "$VALIDATOR_NAME",
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
        "phrase": "$CONTROLLER_PHRASE",
        "accountId": "$CONTROLLER_ACCOUNT_ID",
        "publicKey": "$CONTROLLER_PUBKEY"
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

  # Payment accounts (for easy reference)
  echo "$VALIDATOR_NAME $PAYMENT_ACCOUNT_ID $CONTROLLER_ACCOUNT_ID $AIDEVID_DID" >> $OUTPUT_DIR/payment-accounts.txt

  # Chain spec genesis (validator committee)
  cat >> $OUTPUT_DIR/chain-spec-genesis.json <<EOF
      {
        "sessionAccount": "$SESSION_ACCOUNT_ID",
        "auraKey": "$AURA_PUBKEY",
        "grandpaKey": "$GRANDPA_PUBKEY",
        "asfKey": "$ASF_PUBKEY",
        "paymentAccount": "$PAYMENT_ACCOUNT_ID",
        "controllerAccount": "$CONTROLLER_ACCOUNT_ID",
        "aiDevID": "$AIDEVID_DID",
        "stake": "$STAKE",
        "role": $ROLE
      }$([ $((10#$i)) -eq 21 ] || echo ",")
EOF

  echo -e " ${GREEN}✓${NC}"
done

# Close JSON files
cat >> $OUTPUT_DIR/validator-keys-complete.json <<EOF
  ]
}
EOF

cat >> $OUTPUT_DIR/chain-spec-genesis.json <<EOF
    ]
  }
}
EOF

echo ""
echo -e "${GREEN}✓ All 21 validators generated${NC}"
echo ""

# ═══════════════════════════════════════════════════════════
# 8. GENERATE GENESIS BALANCES
# ═══════════════════════════════════════════════════════════

echo -e "${YELLOW}[3/4] Generating genesis balances...${NC}"

cat > $OUTPUT_DIR/genesis-balances.json <<EOF
{
  "balances": {
    "balances": [
EOF

# Fund each validator's accounts
for i in {01..21}; do
  VALIDATOR=$(jq -r ".validators[$((10#$i - 1))]" $OUTPUT_DIR/validator-keys-complete.json)

  SESSION_ACCOUNT=$(echo $VALIDATOR | jq -r '.sessionKeys.accountId')
  PAYMENT_ACCOUNT=$(echo $VALIDATOR | jq -r '.paymentAccount.accountId')
  CONTROLLER_ACCOUNT=$(echo $VALIDATOR | jq -r '.controllerAccount.accountId')

  # Payment account: 1M ËTR (for rewards)
  echo "      [\"$PAYMENT_ACCOUNT\", \"1000000000000000000000000\"]," >> $OUTPUT_DIR/genesis-balances.json

  # Controller account: 100K ËTR (for tx fees)
  echo "      [\"$CONTROLLER_ACCOUNT\", \"100000000000000000000000\"]," >> $OUTPUT_DIR/genesis-balances.json

  # Session account: 10K ËTR (for consensus tx fees)
  if [ $((10#$i)) -eq 21 ]; then
    echo "      [\"$SESSION_ACCOUNT\", \"10000000000000000000000\"]" >> $OUTPUT_DIR/genesis-balances.json
  else
    echo "      [\"$SESSION_ACCOUNT\", \"10000000000000000000000\"]," >> $OUTPUT_DIR/genesis-balances.json
  fi
done

cat >> $OUTPUT_DIR/genesis-balances.json <<EOF
    ]
  }
}
EOF

echo -e "${GREEN}✓ Genesis balances generated${NC}"
echo ""

# ═══════════════════════════════════════════════════════════
# 9. GENERATE SUMMARY
# ═══════════════════════════════════════════════════════════

echo -e "${YELLOW}[4/4] Generating summary...${NC}"

cat > $OUTPUT_DIR/VALIDATOR_SUMMARY.md <<EOF
# Ëtrid Validator Keys Summary

**Generated:** $(date)
**Total Validators:** 21

---

## 📊 Distribution

- **Directors (Tier 3):** 3 validators (128 ËTR each)
- **FlareNodes (Tier 2a):** 9 validators (64 ËTR each)
- **ValidityNodes (Tier 2b):** 9 validators (64 ËTR each)

---

## 🤖 AI DevID Mapping

| Validator Range | AI DevID | Count |
|----------------|----------|-------|
| validator-01 | governance-dev01 | 1 |
| validator-02 | security-dev01 | 1 |
| validator-03 | audit-dev01 | 1 |
| validator-04-05 | consensus-dev01 | 2 |
| validator-06-07 | runtime-dev01 | 2 |
| validator-08-09 | compiler-dev01 | 2 |
| validator-10-11 | multichain-dev01 | 2 |
| validator-12 | oracle-dev01 | 1 |
| validator-13-14 | edsc-dev01 | 2 |
| validator-15-16 | economics-dev01 | 2 |
| validator-17-18 | ethics-dev01 | 2 |
| validator-19-21 | docs-dev01 | 3 |

---

## 🔑 Key Types

Each validator has 4 key types:

### 1. Session Keys (Hot)
- **Purpose:** Block production, finality voting, consensus
- **Keys:** AURA (sr25519), GRANDPA (ed25519), ASF (sr25519)
- **Storage:** Validator VM keystore (/var/lib/etrid/keystore)
- **Security:** Hot (used continuously)

### 2. Payment Account (Cold)
- **Purpose:** Receives validator rewards
- **Storage:** Hardware wallet / offline
- **Security:** Cold (rarely signs transactions)

### 3. Controller Account (Warm)
- **Purpose:** Manages validator operations
- **Storage:** Azure Key Vault
- **Security:** Warm (occasional use)

### 4. AI DevID (Warm)
- **Purpose:** Identity verification, signed actions
- **Storage:** Encrypted local (14-aidevs/dids/keypairs.json)
- **Security:** Warm (periodic verification)

---

## 📁 Generated Files

1. **validator-keys-complete.json** - All keys (⚠️ SENSITIVE)
2. **payment-accounts.txt** - Payment account addresses
3. **chain-spec-genesis.json** - Genesis validator configuration
4. **genesis-balances.json** - Genesis balance allocations
5. **VALIDATOR_SUMMARY.md** - This file

---

## 🔐 Security Recommendations

### Immediate Actions:
1. **Backup** \`validator-keys-complete.json\` to encrypted USB drive
2. **Store** USB drive in fireproof safe
3. **Print** payment account phrases and store in bank vault
4. **Upload** payment seeds to Azure Key Vault (if not done)
5. **Delete** local \`validator-keys-complete.json\` after backup

### Access Control:
- **Session keys:** Only validator VMs
- **Payment keys:** Only cold storage wallet
- **Controller keys:** Only authorized operators
- **AI DevID keys:** Only orchestrator + Key Vault

### Backup Verification:
\`\`\`bash
# Verify Key Vault backups
az keyvault secret list --vault-name \$KEYVAULT_NAME | grep validator

# Verify USB backup
gpg -d validator-keys-backup.json.gpg | jq '.validators | length'
# Should return: 21
\`\`\`

---

## 💰 Genesis Balances

Total genesis allocation: **23.31M ËTR**

- Payment accounts: 21 × 1M ËTR = **21M ËTR**
- Controller accounts: 21 × 100K ËTR = **2.1M ËTR**
- Session accounts: 21 × 10K ËTR = **210K ËTR**

---

## 🚀 Next Steps

1. **Update chain spec:**
   \`\`\`bash
   # Merge genesis data into chain spec
   jq -s '.[0] * .[1]' chain-spec-base.json generated-keys/chain-spec-genesis.json > mainnet-21-validators.json
   jq -s '.[0].genesis.runtime.balances = .[1].balances' mainnet-21-validators.json generated-keys/genesis-balances.json > mainnet-final.json

   # Convert to raw format
   ./flarechain-node build-spec --chain mainnet-final.json --raw > mainnet-raw.json
   \`\`\`

2. **Deploy validators:**
   \`\`\`bash
   # See: AZURE_21_VALIDATOR_DEPLOYMENT.md
   ./scripts/quick-start-21-validators.sh
   \`\`\`

3. **Verify committee formation:**
   \`\`\`bash
   # Check all 21 validators are in committee
   curl -s http://validator-01-ip:9944 \\
     -H "Content-Type: application/json" \\
     -d '{"id":1, "jsonrpc":"2.0", "method": "etrid_getCommittee"}' \\
     | jq '.result | length'
   # Should return: 21
   \`\`\`

4. **Monitor rewards:**
   \`\`\`bash
   # Check payment account balances
   ./scripts/monitor-validator-rewards.sh
   \`\`\`

---

## ⚠️ CRITICAL WARNINGS

1. **NEVER commit** \`validator-keys-complete.json\` to git
2. **NEVER share** payment account seeds
3. **ALWAYS verify** Key Vault backups before deletion
4. **TEST recovery** from backups before mainnet launch

---

**Generated by:** \`generate-validators-with-payment-aidevid.sh\`
**Documentation:** See \`VALIDATOR_AIDEVID_PAYMENT_INTEGRATION.md\`
EOF

echo -e "${GREEN}✓ Summary generated${NC}"
echo ""

# ═══════════════════════════════════════════════════════════
# 10. FINAL OUTPUT
# ═══════════════════════════════════════════════════════════

echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              GENERATION COMPLETE! ✅                        ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "📁 Files generated in: $OUTPUT_DIR/"
echo ""
echo "   1. validator-keys-complete.json    (⚠️  SENSITIVE - ALL KEYS)"
echo "   2. payment-accounts.txt            (Payment addresses)"
echo "   3. chain-spec-genesis.json         (Genesis config)"
echo "   4. genesis-balances.json           (Balance allocations)"
echo "   5. VALIDATOR_SUMMARY.md            (Documentation)"
echo ""

if [ -n "$KEYVAULT_NAME" ]; then
  echo -e "${GREEN}✓ Keys backed up to Azure Key Vault: $KEYVAULT_NAME${NC}"
  echo ""
fi

echo -e "${YELLOW}⚠️  CRITICAL: Secure validator-keys-complete.json NOW!${NC}"
echo ""
echo "Recommended actions:"
echo "  1. Backup to encrypted USB:"
echo "     gpg -c $OUTPUT_DIR/validator-keys-complete.json"
echo "     mv validator-keys-complete.json.gpg /path/to/usb/"
echo ""
echo "  2. Delete local copy:"
echo "     shred -u $OUTPUT_DIR/validator-keys-complete.json"
echo ""
echo "  3. Verify backup:"
echo "     gpg -d /path/to/usb/validator-keys-complete.json.gpg | jq '.validators | length'"
echo ""
echo -e "${GREEN}🚀 Ready to deploy validators!${NC}"
echo ""
echo "Next: See AZURE_21_VALIDATOR_DEPLOYMENT.md for deployment steps"
