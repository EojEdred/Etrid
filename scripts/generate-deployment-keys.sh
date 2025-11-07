#!/bin/bash
# Generate Unique Deployment Keys for Each Chain
#
# SECURITY: This script generates cryptographically secure random private keys
# for deployment to different blockchain networks.
#
# IMPORTANT:
# - Run this script in a secure environment
# - Store the generated keys in a password manager or HSM
# - NEVER commit the generated .env files to git
# - Use different keys for testnet and mainnet

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Banner
echo -e "${GREEN}"
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                                                           ║"
echo "║     Etrid Deployment Key Generator                        ║"
echo "║     Generate Unique Keys for Each Blockchain             ║"
echo "║                                                           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Warning
echo -e "${RED}⚠️  WARNING: SECURITY CRITICAL ⚠️${NC}"
echo ""
echo "This script will generate NEW private keys for deployment."
echo "The generated keys will have the following properties:"
echo ""
echo "  ✓ Cryptographically secure random generation"
echo "  ✓ Unique key per blockchain network"
echo "  ✓ Properly formatted for Web3 use (0x-prefixed hex)"
echo ""
echo -e "${YELLOW}IMPORTANT SECURITY NOTES:${NC}"
echo "  1. Store these keys in a secure password manager"
echo "  2. Use hardware wallets for mainnet deployments"
echo "  3. Never share or commit these keys to git"
echo "  4. Keep backup in encrypted offline storage"
echo "  5. Use different keys for testnet vs mainnet"
echo ""

# Confirm
read -p "Do you want to proceed? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    echo "Aborted."
    exit 1
fi

echo ""
echo -e "${GREEN}Generating keys...${NC}"
echo ""

# Output directory
OUTPUT_DIR="$(pwd)/generated-keys-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$OUTPUT_DIR"

# Chains to generate keys for
CHAINS=("ethereum" "bsc" "polygon" "arbitrum" "base")

# Generate key for each chain
for chain in "${CHAINS[@]}"; do
    echo -e "${YELLOW}Generating key for ${chain}...${NC}"

    # Generate 32 bytes of random data and convert to hex
    PRIVATE_KEY="0x$(openssl rand -hex 32)"

    # Calculate address using cast (if available) or provide instructions
    if command -v cast &> /dev/null; then
        ADDRESS=$(cast wallet address "$PRIVATE_KEY")
        echo "  Private Key: ${PRIVATE_KEY}"
        echo "  Address: ${ADDRESS}"
    else
        echo "  Private Key: ${PRIVATE_KEY}"
        echo "  (Install 'foundry' to see address: https://getfoundry.sh/)"
    fi

    # Save to file
    cat > "$OUTPUT_DIR/${chain}-key.txt" << EOF
# ${chain} Deployment Key
# Generated: $(date)
#
# KEEP THIS SECRET! NEVER COMMIT TO GIT!

PRIVATE_KEY=${PRIVATE_KEY}

# To get the address:
# npm install -g @ethereumjs/util
# node -e "const {privateToAddress} = require('@ethereumjs/util'); const addr = privateToAddress(Buffer.from('${PRIVATE_KEY:2}', 'hex')); console.log('0x' + addr.toString('hex'));"

# Or use cast (Foundry):
# cast wallet address ${PRIVATE_KEY}
EOF

    echo -e "${GREEN}  ✓ Saved to ${OUTPUT_DIR}/${chain}-key.txt${NC}"
    echo ""
done

# Generate unified contracts key
echo -e "${YELLOW}Generating key for unified contracts...${NC}"
UNIFIED_KEY="0x$(openssl rand -hex 32)"
if command -v cast &> /dev/null; then
    UNIFIED_ADDR=$(cast wallet address "$UNIFIED_KEY")
    echo "  Private Key: ${UNIFIED_KEY}"
    echo "  Address: ${UNIFIED_ADDR}"
else
    echo "  Private Key: ${UNIFIED_KEY}"
fi

cat > "$OUTPUT_DIR/unified-contracts-key.txt" << EOF
# Unified Contracts Deployment Key
# Generated: $(date)
#
# KEEP THIS SECRET! NEVER COMMIT TO GIT!

DEPLOYER_PRIVATE_KEY=${UNIFIED_KEY:2}  # Without 0x prefix for unified-contracts.env
PRIVATE_KEY=${UNIFIED_KEY}             # With 0x prefix for other configs
EOF

echo -e "${GREEN}  ✓ Saved to ${OUTPUT_DIR}/unified-contracts-key.txt${NC}"
echo ""

# Generate master key list
cat > "$OUTPUT_DIR/MASTER_KEY_LIST.md" << EOF
# Etrid Deployment Keys

**Generated**: $(date)
**CRITICAL**: Store this file in encrypted storage. Delete after transferring to password manager.

---

## Keys by Chain

EOF

for chain in "${CHAINS[@]}"; do
    KEY=$(grep "PRIVATE_KEY=" "$OUTPUT_DIR/${chain}-key.txt" | head -1 | cut -d= -f2)
    echo "### ${chain^}" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
    echo "\`\`\`" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
    echo "${KEY}" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
    echo "\`\`\`" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
    echo "" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
done

echo "### Unified Contracts" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
echo "\`\`\`" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
echo "${UNIFIED_KEY}" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
echo "\`\`\`" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"
echo "" >> "$OUTPUT_DIR/MASTER_KEY_LIST.md"

cat >> "$OUTPUT_DIR/MASTER_KEY_LIST.md" << EOF

---

## Security Checklist

Before using these keys:

- [ ] Store in password manager (1Password, Bitwarden, etc.)
- [ ] Create encrypted backup
- [ ] Delete this file after storing keys securely
- [ ] Never commit .env files to git
- [ ] Verify .gitignore includes \`.env\`
- [ ] Use hardware wallet for mainnet (Ledger, Trezor)
- [ ] Implement multi-sig for production contracts
- [ ] Document key rotation schedule

---

## Next Steps

1. **Update .env files**:
   \`\`\`bash
   # For each chain
   cd deployment/dex/dex-deployment/ethereum
   cp .env.example .env
   # Edit .env and paste the key from ethereum-key.txt
   \`\`\`

2. **For unified contracts**:
   \`\`\`bash
   cd secrets
   cp deployment-env/unified-contracts.env.example deployment-env/unified-contracts.env
   # Edit and paste the key from unified-contracts-key.txt
   \`\`\`

3. **Verify gitignore**:
   \`\`\`bash
   git check-ignore deployment/dex/dex-deployment/*/.env
   git check-ignore secrets/deployment-env/*.env
   \`\`\`

4. **Fund deployment addresses**:
   - Get addresses using \`cast wallet address <PRIVATE_KEY>\`
   - Send gas tokens (ETH, BNB, MATIC, etc.)
   - Start with testnet deployments

5. **Security audit**:
   - Review docs/SECURITY_AUDIT_REPORT.md
   - Complete deployment security checklist
   - Document multi-sig setup
EOF

echo -e "${GREEN}"
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                                                           ║"
echo "║     ✓ Keys Generated Successfully                         ║"
echo "║                                                           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo ""
echo "Keys saved to: ${OUTPUT_DIR}/"
echo ""
echo "Files created:"
echo "  • ethereum-key.txt"
echo "  • bsc-key.txt"
echo "  • polygon-key.txt"
echo "  • arbitrum-key.txt"
echo "  • base-key.txt"
echo "  • unified-contracts-key.txt"
echo "  • MASTER_KEY_LIST.md"
echo ""
echo -e "${YELLOW}NEXT STEPS:${NC}"
echo "  1. Review ${OUTPUT_DIR}/MASTER_KEY_LIST.md"
echo "  2. Store keys in password manager"
echo "  3. Update .env files with new keys"
echo "  4. Delete ${OUTPUT_DIR}/ after storing keys"
echo ""
echo -e "${RED}⚠️  SECURITY REMINDER:${NC}"
echo "  • Delete generated files after storing in password manager"
echo "  • Never commit .env files to git"
echo "  • Use hardware wallets for mainnet"
echo "  • Read docs/SECURITY_AUDIT_REPORT.md"
echo ""
