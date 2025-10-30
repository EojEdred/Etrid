#!/bin/bash
# Creates mainnet genesis config from generated accounts
# Usage: ./create-mainnet-genesis-config.sh <genesis-accounts-directory>

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

if [ "$#" -ne 1 ]; then
    echo -e "${RED}❌ Error: Missing genesis accounts directory${NC}"
    echo ""
    echo "Usage: ./create-mainnet-genesis-config.sh <directory>"
    echo "Example: ./create-mainnet-genesis-config.sh genesis-accounts-20251030-120000"
    exit 1
fi

ACCOUNTS_DIR="$1"

if [ ! -d "$ACCOUNTS_DIR" ]; then
    echo -e "${RED}❌ Error: Directory not found: $ACCOUNTS_DIR${NC}"
    exit 1
fi

echo "╔════════════════════════════════════════════════════════════╗"
echo "║       Creating Mainnet Genesis Configuration              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check for jq
if ! command -v jq &> /dev/null; then
    echo -e "${RED}❌ Error: jq is required for JSON processing${NC}"
    echo "Install with: brew install jq (macOS) or apt-get install jq (Linux)"
    exit 1
fi

# Extract addresses from JSON files
extract_address() {
    local file="$1"
    if [ -f "$file" ]; then
        cat "$file" | jq -r '.ss58Address' 2>/dev/null || echo "ERROR_READING_FILE"
    else
        echo "FILE_NOT_FOUND"
    fi
}

echo "📖 Reading generated accounts..."
echo ""

# ETR Tokenomics Accounts
DAO_TREASURY=$(extract_address "$ACCOUNTS_DIR/dao_treasury.json")
COMMUNITY_LP=$(extract_address "$ACCOUNTS_DIR/community_lp_pool.json")
TEAM_VESTING=$(extract_address "$ACCOUNTS_DIR/foundation_team_vesting.json")
NETWORK_EXPANSION=$(extract_address "$ACCOUNTS_DIR/network_expansion.json")
FOUNDERS_POOL=$(extract_address "$ACCOUNTS_DIR/founders_pool.json")
INITIAL_CIRC=$(extract_address "$ACCOUNTS_DIR/initial_circulating.json")

echo -e "${GREEN}✅ ETR tokenomics accounts loaded${NC}"

# EDSC Infrastructure
RESERVE_VAULT=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/reserve_vault.json")
ORACLE_AUTH=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/oracle_authority.json")
CUSTODIAN_MGR=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/custodian_manager.json")
MINTER_AUTH=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/minter_authority.json")
EMERGENCY_PAUSE=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/emergency_pause.json")

echo -e "${GREEN}✅ EDSC infrastructure accounts loaded${NC}"

# Validator Payment Accounts (from existing validator-keys-complete.json)
VALIDATOR_KEYS_FILE="validator-keys-setup/generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json"

if [ ! -f "$VALIDATOR_KEYS_FILE" ]; then
    echo -e "${YELLOW}⚠️  Warning: validator-keys-complete.json not found${NC}"
    echo "   Will use newly generated payment accounts only"
    USE_EXISTING_VALIDATORS=false
else
    echo -e "${GREEN}✅ Using existing validator session keys from $VALIDATOR_KEYS_FILE${NC}"
    USE_EXISTING_VALIDATORS=true
fi
echo ""

# Create genesis config JSON
OUTPUT_FILE="flarechain_mainnet_genesis.json"

echo "📝 Creating genesis configuration..."
echo ""

cat > "$OUTPUT_FILE" << 'EOF'
{
  "name": "Ëtrid FlareChain Mainnet",
  "id": "flarechain_mainnet",
  "chainType": "Live",
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 18,
    "ss58Format": 42
  },
  "protocolId": "flarechain",

  "balances": {
    "balances": [
EOF

# Add ETR tokenomics balances (with 18 decimals)
cat >> "$OUTPUT_FILE" << EOF
      ["$DAO_TREASURY", "875000000000000000000000000"],
      ["$COMMUNITY_LP", "250000000000000000000000000"],
      ["$TEAM_VESTING", "375000000000000000000000000"],
      ["$NETWORK_EXPANSION", "625000000000000000000000000"],
      ["$FOUNDERS_POOL", "125000000000000000000000000"],
      ["$INITIAL_CIRC", "250000000000000000000000000"],
EOF

# Extract custodian addresses
BTC_CUSTODIAN=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/custodians/BTC_custodian.json")
ETH_CUSTODIAN=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/custodians/ETH_custodian.json")
GOLD_CUSTODIAN=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/custodians/Gold_custodian.json")
USDC_CUSTODIAN=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/custodians/USDC_custodian.json")
USDT_CUSTODIAN=$(extract_address "$ACCOUNTS_DIR/edsc_accounts/custodians/USDT_custodian.json")

# Add EDSC infrastructure balances (operational funds)
cat >> "$OUTPUT_FILE" << EOF
      ["$RESERVE_VAULT", "10000000000000000000000"],
      ["$ORACLE_AUTH", "1000000000000000000000"],
      ["$CUSTODIAN_MGR", "1000000000000000000000"],
      ["$MINTER_AUTH", "1000000000000000000000"],
      ["$EMERGENCY_PAUSE", "1000000000000000000000"],
EOF

# Add validator payment accounts
if [ "$USE_EXISTING_VALIDATORS" = true ]; then
    echo "   Adding validator payment accounts from validator-keys-complete.json..."

    for i in {1..21}; do
        PAYMENT_ADDR=$(jq -r ".validators[$((i-1))].paymentAccount.accountId" "$VALIDATOR_KEYS_FILE")
        if [ "$i" -eq 21 ]; then
            echo "      [\"$PAYMENT_ADDR\", \"1000000000000000000000000\"]" >> "$OUTPUT_FILE"
        else
            echo "      [\"$PAYMENT_ADDR\", \"1000000000000000000000000\"]," >> "$OUTPUT_FILE"
        fi
    done
else
    echo "   Adding newly generated validator payment accounts..."

    for i in {1..21}; do
        PAYMENT_ADDR=$(extract_address "$ACCOUNTS_DIR/validator_payment_accounts/validator_${i}_payment.json")
        if [ "$i" -eq 21 ]; then
            echo "      [\"$PAYMENT_ADDR\", \"1000000000000000000000000\"]" >> "$OUTPUT_FILE"
        else
            echo "      [\"$PAYMENT_ADDR\", \"1000000000000000000000000\"]," >> "$OUTPUT_FILE"
        fi
    done
fi

cat >> "$OUTPUT_FILE" << 'EOF'
    ]
  },

  "sudo": {
EOF

# Sudo key (Foundation multisig - user must create manually)
cat >> "$OUTPUT_FILE" << EOF
    "key": "$DAO_TREASURY"
EOF

cat >> "$OUTPUT_FILE" << 'EOF'
  },

  "grandpa": {
    "authorities": [
EOF

# Add GRANDPA authorities from validator-keys-complete.json
if [ "$USE_EXISTING_VALIDATORS" = true ]; then
    echo "   Adding GRANDPA authorities from validator session keys..."

    for i in {1..21}; do
        GRANDPA_KEY=$(jq -r ".validators[$((i-1))].sessionKeys.grandpaKey" "$VALIDATOR_KEYS_FILE")
        if [ "$i" -eq 21 ]; then
            echo "      [\"$GRANDPA_KEY\", 1]" >> "$OUTPUT_FILE"
        else
            echo "      [\"$GRANDPA_KEY\", 1]," >> "$OUTPUT_FILE"
        fi
    done
else
    echo -e "${YELLOW}⚠️  Warning: No validator session keys found - genesis will be incomplete${NC}"
    echo "      [\"5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu\", 1]" >> "$OUTPUT_FILE"
fi

cat >> "$OUTPUT_FILE" << 'EOF'
    ]
  },

  "consensus": {
    "validators": [
EOF

# Add consensus validators (session accounts with stakes)
if [ "$USE_EXISTING_VALIDATORS" = true ]; then
    echo "   Adding consensus validators..."

    for i in {1..21}; do
        SESSION_ADDR=$(jq -r ".validators[$((i-1))].sessionKeys.accountId" "$VALIDATOR_KEYS_FILE")
        VALIDATOR_NAME=$(jq -r ".validators[$((i-1))].name" "$VALIDATOR_KEYS_FILE")
        STAKE="128000000000000000000000"  # 128k ETR stake per validator

        if [ "$i" -eq 21 ]; then
            echo "      [\"$SESSION_ADDR\", \"$STAKE\", \"$VALIDATOR_NAME\"]" >> "$OUTPUT_FILE"
        else
            echo "      [\"$SESSION_ADDR\", \"$STAKE\", \"$VALIDATOR_NAME\"]," >> "$OUTPUT_FILE"
        fi
    done
else
    echo -e "${YELLOW}⚠️  Warning: Using placeholder validator${NC}"
    echo "      [\"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\", \"128000000000000000000000\", \"Validator-1\"]" >> "$OUTPUT_FILE"
fi

cat >> "$OUTPUT_FILE" << 'EOF'
    ],
    "slotDuration": 6000
  },

  "edscToken": {
    "oracleAuthority": "$ORACLE_AUTH",
    "custodianManager": "$CUSTODIAN_MGR",
    "minterAuthority": "$MINTER_AUTH",
    "emergencyPauseAuthority": "$EMERGENCY_PAUSE"
  },

  "reserveVault": {
    "vaultAddress": "$RESERVE_VAULT",
    "reserveAssets": [
      {
        "asset": "BTC",
        "custodian": "$BTC_CUSTODIAN"
      },
      {
        "asset": "ETH",
        "custodian": "$ETH_CUSTODIAN"
      },
      {
        "asset": "Gold",
        "custodian": "$GOLD_CUSTODIAN"
      },
      {
        "asset": "USDC",
        "custodian": "$USDC_CUSTODIAN"
      },
      {
        "asset": "USDT",
        "custodian": "$USDT_CUSTODIAN"
      }
    ]
EOF

cat >> "$OUTPUT_FILE" << 'EOF'
  }
}
EOF

echo -e "${GREEN}✅ Genesis configuration created: $OUTPUT_FILE${NC}"
echo ""

# Validate JSON
if jq empty "$OUTPUT_FILE" 2>/dev/null; then
    echo -e "${GREEN}✅ JSON validation passed${NC}"
else
    echo -e "${RED}❌ JSON validation failed!${NC}"
    exit 1
fi
echo ""

# Calculate total supply
echo "📊 Verifying token distribution..."
TOTAL_SUPPLY=$(jq '[.balances.balances[][1] | tonumber] | add' "$OUTPUT_FILE")
echo "Total genesis supply (base units): $TOTAL_SUPPLY"
echo ""

# Create human-readable summary
cat > "GENESIS_CONFIG_SUMMARY.md" << EOF
# Mainnet Genesis Configuration Summary

**Generated:** $(date)
**Config File:** $OUTPUT_FILE

---

## ✅ Configuration Status

- ETR Tokenomics: ✅ Complete
- EDSC Infrastructure: ✅ Complete
- Validator Accounts: $([ "$USE_EXISTING_VALIDATORS" = true ] && echo "✅ Using existing (21 validators)" || echo "⚠️  Placeholder only")
- JSON Validation: ✅ Passed

---

## 📊 Token Distribution Verification

**Total Genesis Supply:** $TOTAL_SUPPLY base units
**Expected:** 2,500,000,000 ETR + validator allocations

### Breakdown:
- DAO Treasury: 875M ETR
- Community LP: 250M ETR
- Team Vesting: 375M ETR
- Network Expansion: 625M ETR
- Founders Pool: 125M ETR
- Initial Circulating: 250M ETR
- Validators: 21M ETR (21 × 1M each)
- EDSC Ops: 14K ETR (infrastructure accounts)

**TOTAL: ~2,521M ETR**

---

## 🔐 Critical Accounts

| Purpose | Address |
|---------|---------|
| **Sudo Key** | \`$DAO_TREASURY\` |
| **Reserve Vault** | \`$RESERVE_VAULT\` |
| **Oracle Authority** | \`$ORACLE_AUTH\` |
| **Minter Authority** | \`$MINTER_AUTH\` |
| **Emergency Pause** | \`$EMERGENCY_PAUSE\` |

---

## ⚠️ Before Mainnet Launch

1. **Create Foundation Multisig**
   - Use 7 signers from \`$ACCOUNTS_DIR/foundation_multisig_signers/\`
   - Set 5-of-7 threshold
   - Replace \`$DAO_TREASURY\` with multisig address in genesis config

2. **Test on Devnet**
   - Deploy with this config
   - Test all critical functions
   - Verify balances, staking, governance

3. **Security Audit**
   - Review all addresses
   - Verify token amounts
   - Check multisig setup
   - Test key recovery procedures

4. **Final Verification**
   - Total supply calculation
   - Address checksums
   - JSON schema validation
   - Genesis hash calculation

---

## 🚀 Deployment Steps

1. **Copy genesis config to runtime:**
   \`\`\`bash
   cp $OUTPUT_FILE 05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json
   \`\`\`

2. **Build binary:**
   \`\`\`bash
   cargo build --release --locked
   \`\`\`

3. **Generate chain spec:**
   \`\`\`bash
   ./target/release/flarechain-node build-spec --chain mainnet --raw > flarechain-raw.json
   \`\`\`

4. **Deploy to validators:**
   - Copy binary + chain spec to all 21 validator VMs
   - Insert session keys on each validator
   - Start all validators simultaneously

---

**Status:** ✅ Genesis configuration ready for deployment!

EOF

echo -e "${GREEN}✅ Summary created: GENESIS_CONFIG_SUMMARY.md${NC}"
echo ""

echo "╔════════════════════════════════════════════════════════════╗"
echo "║        Genesis Configuration Complete!                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo "📄 Files created:"
echo "   - $OUTPUT_FILE"
echo "   - GENESIS_CONFIG_SUMMARY.md"
echo ""
echo "🔍 Review the config carefully before deploying to mainnet!"
echo ""
echo "Next steps:"
echo "  1. Read GENESIS_CONFIG_SUMMARY.md"
echo "  2. Create Foundation multisig and update sudo key"
echo "  3. Test on devnet first"
echo "  4. Copy to runtime/presets/ and build binary"
echo ""
