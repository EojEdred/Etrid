#!/bin/bash
# Ëtrid Comprehensive Genesis Account Generation
# Generates all accounts needed for mainnet genesis including:
# - ETR tokenomics accounts
# - EDSC stablecoin infrastructure
# - Reserve vault custodians
# - All 21 validator payment accounts
# - Foundation/team/advisor accounts

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║       Ëtrid Genesis Account Generation System             ║"
echo "║         ETR Tokenomics + EDSC Stablecoin + Validators     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if subkey is installed
if ! command -v subkey &> /dev/null; then
    echo -e "${RED}❌ Error: subkey is not installed${NC}"
    echo ""
    echo "Install with:"
    echo "  cargo install --force --git https://github.com/paritytech/polkadot-sdk subkey"
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo -e "${YELLOW}⚠️  Warning: jq not installed (recommended for JSON formatting)${NC}"
    echo "Install with: brew install jq (macOS) or apt-get install jq (Linux)"
    echo ""
fi

# Create output directory
OUTPUT_DIR="genesis-accounts-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$OUTPUT_DIR"
cd "$OUTPUT_DIR"

echo -e "${GREEN}✅ Output directory created: $OUTPUT_DIR${NC}"
echo ""

# ============================================================================
# SECTION 1: ETR TOKENOMICS ACCOUNTS (2.5B ETR Total Supply)
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}SECTION 1: ETR TOKENOMICS ACCOUNTS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# 1. DAO Treasury / Protocol Reserve (35% = 875M ETR)
echo "1️⃣  Generating DAO Treasury account (875M ETR)..."
subkey generate --scheme Sr25519 --network substrate --output-type json > dao_treasury.json
DAO_TREASURY=$(cat dao_treasury.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $DAO_TREASURY${NC}"
echo ""

# 2. Community Liquidity & LP Incentives (10% = 250M ETR)
echo "2️⃣  Generating Community LP Pool account (250M ETR)..."
subkey generate --scheme Sr25519 --network substrate --output-type json > community_lp_pool.json
COMMUNITY_LP=$(cat community_lp_pool.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $COMMUNITY_LP${NC}"
echo ""

# 3. Foundation / Team Vesting (15% = 375M ETR)
echo "3️⃣  Generating Foundation Team Vesting account (375M ETR)..."
subkey generate --scheme Sr25519 --network substrate --output-type json > foundation_team_vesting.json
TEAM_VESTING=$(cat foundation_team_vesting.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $TEAM_VESTING${NC}"
echo ""

# 4. Network Expansion / Partnerships (25% = 625M ETR)
echo "4️⃣  Generating Network Expansion Pool account (625M ETR)..."
subkey generate --scheme Sr25519 --network substrate --output-type json > network_expansion.json
NETWORK_EXPANSION=$(cat network_expansion.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $NETWORK_EXPANSION${NC}"
echo ""

# 5. Founders' Creation Pool (5% = 125M ETR)
echo "5️⃣  Generating Founders Pool account (125M ETR)..."
subkey generate --scheme Sr25519 --network substrate --output-type json > founders_pool.json
FOUNDERS_POOL=$(cat founders_pool.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $FOUNDERS_POOL${NC}"
echo ""

# 6. Initial Circulating Supply (10% = 250M ETR)
echo "6️⃣  Generating Initial Circulating Supply account (250M ETR)..."
subkey generate --scheme Sr25519 --network substrate --output-type json > initial_circulating.json
INITIAL_CIRC=$(cat initial_circulating.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $INITIAL_CIRC${NC}"
echo ""

# ============================================================================
# SECTION 2: FOUNDATION MULTISIG SIGNERS (5-of-7 or 3-of-5)
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}SECTION 2: FOUNDATION MULTISIG SIGNERS${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

mkdir -p foundation_multisig_signers

for i in {1..7}; do
    echo "Generating Foundation Multisig Signer #$i..."
    subkey generate --scheme Sr25519 --network substrate --output-type json > foundation_multisig_signers/signer_$i.json
    SIGNER=$(cat foundation_multisig_signers/signer_$i.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo -e "${GREEN}   ✅ Signer #$i: $SIGNER${NC}"
done
echo ""
echo -e "${YELLOW}⚠️  NOTE: Create multisig address using these 7 signers with 5-of-7 threshold${NC}"
echo ""

# ============================================================================
# SECTION 3: EDSC STABLECOIN INFRASTRUCTURE
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}SECTION 3: EDSC STABLECOIN INFRASTRUCTURE${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

mkdir -p edsc_accounts

# 1. Reserve Vault (Main custody account for multi-asset reserves)
echo "1️⃣  Generating Reserve Vault account..."
subkey generate --scheme Sr25519 --network substrate --output-type json > edsc_accounts/reserve_vault.json
RESERVE_VAULT=$(cat edsc_accounts/reserve_vault.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $RESERVE_VAULT${NC}"
echo ""

# 2. EDSC Oracle Authority (Price feed signer)
echo "2️⃣  Generating EDSC Oracle Authority account..."
subkey generate --scheme Sr25519 --network substrate --output-type json > edsc_accounts/oracle_authority.json
ORACLE_AUTH=$(cat edsc_accounts/oracle_authority.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $ORACLE_AUTH${NC}"
echo ""

# 3. Custodian Registry Manager
echo "3️⃣  Generating Custodian Registry Manager account..."
subkey generate --scheme Sr25519 --network substrate --output-type json > edsc_accounts/custodian_manager.json
CUSTODIAN_MGR=$(cat edsc_accounts/custodian_manager.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $CUSTODIAN_MGR${NC}"
echo ""

# 4-8. Reserve Asset Custodians (BTC, ETH, Gold, USDC, USDT)
echo "4️⃣  Generating Reserve Asset Custodian accounts..."
mkdir -p edsc_accounts/custodians

for asset in BTC ETH Gold USDC USDT; do
    echo "   Generating $asset Custodian..."
    subkey generate --scheme Sr25519 --network substrate --output-type json > edsc_accounts/custodians/${asset}_custodian.json
    CUSTODIAN=$(cat edsc_accounts/custodians/${asset}_custodian.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo -e "${GREEN}   ✅ $asset Custodian: $CUSTODIAN${NC}"
done
echo ""

# 9. EDSC Minter Authority (Authorized to mint/burn EDSC)
echo "5️⃣  Generating EDSC Minter Authority account..."
subkey generate --scheme Sr25519 --network substrate --output-type json > edsc_accounts/minter_authority.json
MINTER_AUTH=$(cat edsc_accounts/minter_authority.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $MINTER_AUTH${NC}"
echo ""

# 10. EDSC Emergency Pause Authority
echo "6️⃣  Generating EDSC Emergency Pause Authority account..."
subkey generate --scheme Sr25519 --network substrate --output-type json > edsc_accounts/emergency_pause.json
EMERGENCY_PAUSE=$(cat edsc_accounts/emergency_pause.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
echo -e "${GREEN}   ✅ $EMERGENCY_PAUSE${NC}"
echo ""

# ============================================================================
# SECTION 4: VALIDATOR PAYMENT ACCOUNTS (All 21 Validators)
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}SECTION 4: VALIDATOR PAYMENT ACCOUNTS (21 Validators)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

mkdir -p validator_payment_accounts

echo "Generating payment accounts for all 21 validators..."
echo ""

for i in {1..21}; do
    echo "Validator #$i Payment Account..."
    subkey generate --scheme Sr25519 --network substrate --output-type json > validator_payment_accounts/validator_${i}_payment.json
    VALIDATOR_PAY=$(cat validator_payment_accounts/validator_${i}_payment.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo -e "${GREEN}   ✅ Validator #$i Payment: $VALIDATOR_PAY${NC}"
done
echo ""
echo -e "${YELLOW}📝 NOTE: These are payment accounts. Session keys come from validator-keys-complete.json${NC}"
echo ""

# ============================================================================
# SECTION 5: TEAM & ADVISOR ACCOUNTS (Vesting)
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}SECTION 5: TEAM & ADVISOR ACCOUNTS (375M ETR with Vesting)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

mkdir -p team_accounts

TEAM_MEMBERS=(
    "CEO_Founder:75000000"
    "CTO:56250000"
    "Core_Dev_1:37500000"
    "Core_Dev_2:37500000"
    "Core_Dev_3:37500000"
    "AI_Director:30000000"
    "Advisor_1:26250000"
    "Advisor_2:26250000"
    "Advisor_3:26250000"
    "Marketing_Lead:23500000"
)

for member in "${TEAM_MEMBERS[@]}"; do
    ROLE=$(echo $member | cut -d':' -f1)
    AMOUNT=$(echo $member | cut -d':' -f2)

    echo "Generating account for $ROLE ($AMOUNT ETR)..."
    subkey generate --scheme Sr25519 --network substrate --output-type json > team_accounts/${ROLE}.json
    TEAM_ADDR=$(cat team_accounts/${ROLE}.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo -e "${GREEN}   ✅ $ROLE: $TEAM_ADDR${NC}"
done
echo ""

# ============================================================================
# SECTION 6: GENERATE SUMMARY REPORT
# ============================================================================

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}GENERATING SUMMARY REPORT${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cat > GENESIS_ACCOUNTS_SUMMARY.md << EOF
# Ëtrid Genesis Accounts Summary

**Generated:** $(date)
**Total Accounts Created:** $(find . -name "*.json" -type f | wc -l)

---

## 📊 ETR Token Distribution (2.5B Total Supply)

| Account Purpose | Amount (ETR) | Percentage | Address |
|----------------|--------------|------------|---------|
| **DAO Treasury / Protocol Reserve** | 875,000,000 | 35% | \`$DAO_TREASURY\` |
| **Community LP & Incentives** | 250,000,000 | 10% | \`$COMMUNITY_LP\` |
| **Foundation / Team Vesting** | 375,000,000 | 15% | \`$TEAM_VESTING\` |
| **Network Expansion / Partnerships** | 625,000,000 | 25% | \`$NETWORK_EXPANSION\` |
| **Founders' Creation Pool** | 125,000,000 | 5% | \`$FOUNDERS_POOL\` |
| **Initial Circulating Supply** | 250,000,000 | 10% | \`$INITIAL_CIRC\` |
| **TOTAL** | **2,500,000,000** | **100%** | |

---

## 🔐 Foundation Multisig Signers (5-of-7 Threshold)

EOF

for i in {1..7}; do
    SIGNER=$(cat foundation_multisig_signers/signer_$i.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo "$i. \`$SIGNER\`" >> GENESIS_ACCOUNTS_SUMMARY.md
done

cat >> GENESIS_ACCOUNTS_SUMMARY.md << EOF

**Action Required:** Create multisig address using these 7 signers with 5-of-7 threshold

---

## 💵 EDSC Stablecoin Infrastructure

| Account Purpose | Address |
|----------------|---------|
| **Reserve Vault** | \`$RESERVE_VAULT\` |
| **Oracle Authority** | \`$ORACLE_AUTH\` |
| **Custodian Manager** | \`$CUSTODIAN_MGR\` |
| **Minter Authority** | \`$MINTER_AUTH\` |
| **Emergency Pause** | \`$EMERGENCY_PAUSE\` |

### Reserve Asset Custodians

EOF

for asset in BTC ETH Gold USDC USDT; do
    CUSTODIAN=$(cat edsc_accounts/custodians/${asset}_custodian.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo "- **$asset Custodian:** \`$CUSTODIAN\`" >> GENESIS_ACCOUNTS_SUMMARY.md
done

cat >> GENESIS_ACCOUNTS_SUMMARY.md << EOF

---

## 👥 Validator Payment Accounts (21 Validators)

Each validator receives genesis balance for staking and operations.

EOF

for i in {1..21}; do
    VALIDATOR_PAY=$(cat validator_payment_accounts/validator_${i}_payment.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo "$i. \`$VALIDATOR_PAY\`" >> GENESIS_ACCOUNTS_SUMMARY.md
done

cat >> GENESIS_ACCOUNTS_SUMMARY.md << EOF

**Note:** Session keys (AURA, GRANDPA, ASF) come from \`validator-keys-complete.json\`

---

## 💼 Team & Advisors (375M ETR with Vesting)

| Role | Amount (ETR) | Address |
|------|--------------|---------|
EOF

for member in "${TEAM_MEMBERS[@]}"; do
    ROLE=$(echo $member | cut -d':' -f1)
    AMOUNT=$(echo $member | cut -d':' -f2)
    TEAM_ADDR=$(cat team_accounts/${ROLE}.json | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    echo "| $ROLE | $AMOUNT | \`$TEAM_ADDR\` |" >> GENESIS_ACCOUNTS_SUMMARY.md
done

cat >> GENESIS_ACCOUNTS_SUMMARY.md << EOF

**Vesting Schedule:**
- CEO/Founder: 12-month cliff, 24-month linear vesting
- CTO: 12-month cliff, 24-month linear vesting
- Core Devs: 6-month cliff, 30-month linear vesting
- AI Director: 6-month cliff, 30-month linear vesting
- Advisors: No cliff, 36-month linear vesting
- Marketing Lead: No cliff, 36-month linear vesting

---

## 📁 File Structure

\`\`\`
$OUTPUT_DIR/
├── dao_treasury.json
├── community_lp_pool.json
├── foundation_team_vesting.json
├── network_expansion.json
├── founders_pool.json
├── initial_circulating.json
├── foundation_multisig_signers/
│   ├── signer_1.json
│   ├── signer_2.json
│   ├── ... (7 total)
├── edsc_accounts/
│   ├── reserve_vault.json
│   ├── oracle_authority.json
│   ├── custodian_manager.json
│   ├── minter_authority.json
│   ├── emergency_pause.json
│   └── custodians/
│       ├── BTC_custodian.json
│       ├── ETH_custodian.json
│       ├── Gold_custodian.json
│       ├── USDC_custodian.json
│       └── USDT_custodian.json
├── validator_payment_accounts/
│   ├── validator_1_payment.json
│   ├── validator_2_payment.json
│   ├── ... (21 total)
├── team_accounts/
│   ├── CEO_Founder.json
│   ├── CTO.json
│   ├── ... (10 total)
└── GENESIS_ACCOUNTS_SUMMARY.md (this file)
\`\`\`

---

## ⚠️ CRITICAL SECURITY WARNINGS

1. **NEVER share private keys from these JSON files**
2. **Backup all JSON files to multiple encrypted locations**
3. **Store Foundation multisig signer keys separately**
4. **Test accounts on testnet before mainnet**
5. **Keep offline copies in fireproof safe**
6. **Use hardware wallets for Foundation signers**
7. **Never commit these files to git**

---

## 🚀 Next Steps

1. **Create Foundation Multisig:**
   - Use Polkadot.js Apps or subkey
   - Set 5-of-7 threshold
   - Note multisig address for genesis config

2. **Generate Genesis Config:**
   - Run: \`./create-mainnet-genesis-config.sh\`
   - This will create the JSON for runtime preset

3. **Backup All Keys:**
   - Encrypt this entire directory
   - Store on USB drive + cloud + paper copies
   - Keep Foundation signer keys separately

4. **Configure Validators:**
   - Insert session keys on each validator VM
   - Use payment accounts for reward destinations
   - Test on devnet first!

5. **Deploy to Mainnet:**
   - Build binary with genesis config
   - Start all validators simultaneously
   - Monitor genesis block creation

---

**Status:** ✅ All accounts generated successfully!
**Total Accounts:** $(find . -name "*.json" -type f | wc -l)
**Ready for:** Genesis configuration and mainnet deployment

EOF

echo -e "${GREEN}✅ Summary report generated: GENESIS_ACCOUNTS_SUMMARY.md${NC}"
echo ""

# ============================================================================
# FINAL OUTPUT
# ============================================================================

echo "╔════════════════════════════════════════════════════════════╗"
echo "║           Genesis Account Generation Complete!             ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}✅ All accounts generated successfully!${NC}"
echo ""
echo "📁 Output Directory: $OUTPUT_DIR"
echo "📊 Total JSON Files: $(find . -name "*.json" -type f | wc -l)"
echo "📄 Summary Report: GENESIS_ACCOUNTS_SUMMARY.md"
echo ""
echo "⚠️  CRITICAL: Backup these files immediately to encrypted storage!"
echo ""
echo "Next Steps:"
echo "  1. Read GENESIS_ACCOUNTS_SUMMARY.md for all addresses"
echo "  2. Create Foundation multisig (5-of-7 threshold)"
echo "  3. Run ./create-mainnet-genesis-config.sh (next script)"
echo "  4. Test on devnet before mainnet deployment"
echo ""
