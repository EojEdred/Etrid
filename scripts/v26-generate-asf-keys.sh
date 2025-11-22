#!/bin/bash
# V26 ASF Key Generation Script
# Generates sr25519 keypair for ASF checkpoint signing
# Usage: ./v26-generate-asf-keys.sh [--keystore-path /path/to/keystore]

set -e
set -u
set -o pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
KEYSTORE_PATH="${HOME}/.local/share/flarechain/chains/flarechain_mainnet/keystore"
KEY_TYPE="asfk"
BACKUP_DIR="${HOME}/.etrid-asf-keys-backup"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --keystore-path)
            KEYSTORE_PATH="$2"
            shift 2
            ;;
        --backup-dir)
            BACKUP_DIR="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --keystore-path PATH   Path to validator keystore (default: ~/.local/share/flarechain/chains/flarechain_mainnet/keystore)"
            echo "  --backup-dir PATH      Path to backup directory (default: ~/.etrid-asf-keys-backup)"
            echo "  --help, -h             Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   V26 ASF Key Generation Script${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if subkey is available
if ! command -v subkey &> /dev/null; then
    echo -e "${RED}Error: 'subkey' command not found${NC}"
    echo -e "${YELLOW}Please install subkey:${NC}"
    echo "  cargo install --force subkey --git https://github.com/paritytech/polkadot-sdk --locked"
    exit 1
fi

# Verify keystore path exists
if [ ! -d "$KEYSTORE_PATH" ]; then
    echo -e "${YELLOW}Keystore directory does not exist: $KEYSTORE_PATH${NC}"
    echo -e "${YELLOW}Creating keystore directory...${NC}"
    mkdir -p "$KEYSTORE_PATH"
fi

# Create backup directory
mkdir -p "$BACKUP_DIR"
chmod 700 "$BACKUP_DIR"

echo -e "${GREEN}Step 1: Generating sr25519 keypair...${NC}"
echo ""

# Generate new keypair using subkey
KEYPAIR_OUTPUT=$(subkey generate --scheme sr25519 --output-type json)

# Extract values from JSON output
SECRET_PHRASE=$(echo "$KEYPAIR_OUTPUT" | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)
SECRET_SEED=$(echo "$KEYPAIR_OUTPUT" | grep -o '"secretSeed":"[^"]*"' | cut -d'"' -f4)
PUBLIC_KEY=$(echo "$KEYPAIR_OUTPUT" | grep -o '"publicKey":"[^"]*"' | cut -d'"' -f4)
SS58_ADDRESS=$(echo "$KEYPAIR_OUTPUT" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)
ACCOUNT_ID=$(echo "$KEYPAIR_OUTPUT" | grep -o '"accountId":"[^"]*"' | cut -d'"' -f4)

echo -e "${GREEN}Generated ASF Keypair:${NC}"
echo -e "  Public Key:   ${BLUE}$PUBLIC_KEY${NC}"
echo -e "  SS58 Address: ${BLUE}$SS58_ADDRESS${NC}"
echo -e "  Account ID:   ${BLUE}$ACCOUNT_ID${NC}"
echo ""

# Save backup with timestamp
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/asf-key-$TIMESTAMP.json"

cat > "$BACKUP_FILE" << EOF
{
  "secretPhrase": "$SECRET_PHRASE",
  "secretSeed": "$SECRET_SEED",
  "publicKey": "$PUBLIC_KEY",
  "ss58Address": "$SS58_ADDRESS",
  "accountId": "$ACCOUNT_ID",
  "keyType": "$KEY_TYPE",
  "generated": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF

chmod 600 "$BACKUP_FILE"

echo -e "${GREEN}Step 2: Backing up keypair...${NC}"
echo -e "  Saved to: ${BLUE}$BACKUP_FILE${NC}"
echo ""

echo -e "${GREEN}Step 3: Inserting key into validator keystore...${NC}"

# The keystore file format: <key-type><public-key>
KEYSTORE_FILE="$KEYSTORE_PATH/${KEY_TYPE}${PUBLIC_KEY}"

# Write the secret seed to keystore file (without 0x prefix)
SECRET_SEED_CLEAN="${SECRET_SEED#0x}"
echo -n "\"$SECRET_SEED_CLEAN\"" > "$KEYSTORE_FILE"
chmod 600 "$KEYSTORE_FILE"

echo -e "  Keystore file: ${BLUE}$KEYSTORE_FILE${NC}"
echo ""

echo -e "${GREEN}Step 4: Exporting public key for registration...${NC}"

# Create a registration file
REGISTRATION_FILE="$BACKUP_DIR/asf-public-key.txt"
echo "$PUBLIC_KEY" > "$REGISTRATION_FILE"

echo -e "  Public key exported to: ${BLUE}$REGISTRATION_FILE${NC}"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}   Key Generation Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

echo -e "${YELLOW}IMPORTANT - Security Instructions:${NC}"
echo -e "  1. ${RED}BACKUP${NC} the following file to secure offline storage:"
echo -e "     ${BLUE}$BACKUP_FILE${NC}"
echo ""
echo -e "  2. This file contains your ${RED}SECRET KEY${NC} - keep it safe!"
echo ""
echo -e "  3. The public key for registration is:"
echo -e "     ${GREEN}$PUBLIC_KEY${NC}"
echo ""
echo -e "  4. To register this key on-chain, run:"
echo -e "     ${BLUE}node scripts/v26-register-asf-keys.js --public-key $PUBLIC_KEY${NC}"
echo ""
echo -e "  5. After registration, restart your validator node to load the key"
echo ""

echo -e "${YELLOW}Next Steps:${NC}"
echo -e "  1. Backup the key file: $BACKUP_FILE"
echo -e "  2. Register the public key on-chain"
echo -e "  3. Verify registration with: node scripts/v26-verify-asf-keys.js"
echo -e "  4. Restart validator node"
echo ""
