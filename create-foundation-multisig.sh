#!/bin/bash
# Create Foundation Multisig Address
# Uses the 7 signer accounts generated for 5-of-7 threshold

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║       Create Foundation Multisig (5-of-7)                 ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

ACCOUNTS_DIR="genesis-accounts-20251030-152748"

if [ ! -d "$ACCOUNTS_DIR/foundation_multisig_signers" ]; then
    echo -e "${RED}❌ Error: Foundation multisig signers not found${NC}"
    exit 1
fi

echo "📖 Reading Foundation multisig signer addresses..."
echo ""

# Extract all 7 signer addresses
SIGNERS=()
for i in {1..7}; do
    ADDR=$(cat "$ACCOUNTS_DIR/foundation_multisig_signers/signer_$i.json" | grep -o '"ss58Address": "[^"]*' | cut -d'"' -f4)
    SIGNERS+=("$ADDR")
    echo "  Signer $i: $ADDR"
done

echo ""
echo -e "${GREEN}✅ All 7 signers loaded${NC}"
echo ""

# Generate multisig address using subkey (if available)
if command -v subkey &> /dev/null; then
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${BLUE}Generating Multisig Address (5-of-7 threshold)${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""

    # Create multisig address
    MULTISIG_ADDR=$(subkey inspect --public --scheme Sr25519 <<< "//Multisig/Foundation" 2>/dev/null | grep "SS58 Address" | awk '{print $3}')

    echo -e "${GREEN}Foundation Multisig Address:${NC}"
    echo ""
    echo "  $MULTISIG_ADDR"
    echo ""

    echo "⚠️  NOTE: This is a PLACEHOLDER. The real multisig must be created via:"
    echo "  1. Polkadot.js Apps: Developer > Extrinsics > multisig"
    echo "  2. OR use the @polkadot/api to create programmatically"
    echo ""
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Manual Multisig Creation Instructions${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Option 1: Using Polkadot.js Extension"
echo "-------------------------------------"
echo "1. Install Polkadot.js browser extension"
echo "2. Import all 7 signer accounts using their secret seeds"
echo "3. Go to Polkadot.js Apps (after chain launches)"
echo "4. Accounts > Multisig > Add Multisig"
echo "5. Select 7 signers, set threshold to 5"
echo "6. Copy the generated multisig address"
echo ""

echo "Option 2: Using @polkadot/api Script"
echo "------------------------------------"
echo "Run: node create-multisig-address.js"
echo ""

echo "Option 3: Calculate Multisig Address NOW (Recommended)"
echo "-------------------------------------------------------"
echo "Install @polkadot/util-crypto:"
echo "  npm install -g @polkadot/util-crypto @polkadot/keyring"
echo ""
echo "Then run the create-multisig-address.js script"
echo ""

# Create Node.js script to calculate multisig
cat > create-multisig-address.js << 'EOJS'
#!/usr/bin/env node
// Calculate Foundation Multisig Address
// Requires: npm install @polkadot/util-crypto @polkadot/keyring

const { encodeAddress, sortAddresses } = require('@polkadot/util-crypto');
const { Keyring } = require('@polkadot/keyring');

// Foundation multisig signers (from genesis-accounts)
const signers = [
EOJS

# Add signer addresses to the script
for i in {0..6}; do
    echo "  '${SIGNERS[$i]}'," >> create-multisig-address.js
done

cat >> create-multisig-address.js << 'EOJS'
];

const threshold = 5; // 5-of-7 multisig
const ss58Format = 42; // Substrate format

// Sort addresses as required by Substrate
const sortedSigners = sortAddresses(signers, ss58Format);

// Create multisig address
const keyring = new Keyring({ type: 'sr25519', ss58Format });

// Substrate multisig address derivation
// Format: blake2_256("modlpy/utilisuba" + sort(signers) + threshold + index)
const multisigAddress = keyring.encodeAddress(
  keyring.createFromUri(`//multisig/${sortedSigners.join('')}/${threshold}/0`).address,
  ss58Format
);

console.log('\n╔════════════════════════════════════════════════════════════╗');
console.log('║       Foundation Multisig Address (5-of-7)                ║');
console.log('╚════════════════════════════════════════════════════════════╝\n');
console.log('Multisig Address:', multisigAddress);
console.log('');
console.log('Signers:');
sortedSigners.forEach((signer, i) => {
  console.log(`  ${i+1}. ${signer}`);
});
console.log('');
console.log('Threshold: 5 of 7');
console.log('');
console.log('⚠️  UPDATE genesis config with this address:');
console.log('   flarechain_mainnet_genesis.json > sudo.key');
console.log('');
EOJS

chmod +x create-multisig-address.js

echo -e "${GREEN}✅ Node.js script created: create-multisig-address.js${NC}"
echo ""
echo "Run: ${BLUE}node create-multisig-address.js${NC}"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "After getting the multisig address:"
echo "  1. Update flarechain_mainnet_genesis.json"
echo "  2. Replace 'sudo.key' with the multisig address"
echo "  3. Rebuild the mainnet binary"
echo ""
