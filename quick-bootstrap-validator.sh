#!/bin/bash
# Ëtrid Quick Validator Bootstrap
# Automatically reads keys from validator-keys-complete.json
# Usage: sudo ./quick-bootstrap-validator.sh <validator-index>

set -e

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║         Ëtrid Quick Validator Bootstrap                   ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo -e "${RED}⚠️  This script must be run as root or with sudo${NC}"
    echo "Usage: sudo ./quick-bootstrap-validator.sh <validator-index>"
    exit 1
fi

# Check arguments
if [ "$#" -ne 1 ]; then
    echo -e "${RED}❌ Error: Missing validator index${NC}"
    echo ""
    echo "Usage: sudo ./quick-bootstrap-validator.sh <validator-index>"
    echo ""
    echo "Example:"
    echo "  sudo ./quick-bootstrap-validator.sh 1   # Bootstrap validator #1 (Gizzi)"
    echo "  sudo ./quick-bootstrap-validator.sh 2   # Bootstrap validator #2 (EojEdred)"
    echo "  sudo ./quick-bootstrap-validator.sh 3   # Bootstrap validator #3"
    echo ""
    exit 1
fi

VALIDATOR_INDEX=$1
KEYS_FILE="validator-keys-setup/generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json"

# Check if keys file exists
if [ ! -f "$KEYS_FILE" ]; then
    echo -e "${RED}❌ Error: Keys file not found: $KEYS_FILE${NC}"
    echo "Expected location: $PWD/$KEYS_FILE"
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "Installing jq for JSON parsing..."
    apt-get update -qq
    apt-get install -y jq
fi

echo -e "${GREEN}✅ Keys file found${NC}"
echo ""

# Extract validator data
echo "📖 Reading validator #$VALIDATOR_INDEX data..."
VALIDATOR_DATA=$(jq ".validators[$((VALIDATOR_INDEX - 1))]" "$KEYS_FILE")

if [ "$VALIDATOR_DATA" = "null" ]; then
    echo -e "${RED}❌ Error: Validator #$VALIDATOR_INDEX not found in keys file${NC}"
    TOTAL_VALIDATORS=$(jq '.validators | length' "$KEYS_FILE")
    echo "Available validators: 1-$TOTAL_VALIDATORS"
    exit 1
fi

# Extract fields
VALIDATOR_NAME=$(echo "$VALIDATOR_DATA" | jq -r '.name')
SESSION_SEED=$(echo "$VALIDATOR_DATA" | jq -r '.sessionKeys.seed')
AURA_KEY=$(echo "$VALIDATOR_DATA" | jq -r '.sessionKeys.auraKey')
GRANDPA_KEY=$(echo "$VALIDATOR_DATA" | jq -r '.sessionKeys.grandpaKey')
ASF_KEY=$(echo "$VALIDATOR_DATA" | jq -r '.sessionKeys.asfKey')
NODE_KEY=$(echo "$VALIDATOR_DATA" | jq -r '.networkKey.secret // empty')

# If no network key, generate one
if [ -z "$NODE_KEY" ] || [ "$NODE_KEY" = "null" ]; then
    echo -e "${YELLOW}⚠️  No network key found, generating one...${NC}"
    NODE_KEY=$(openssl rand -hex 32)
    echo "Generated node key: $NODE_KEY"
fi

echo -e "${GREEN}✅ Validator data extracted${NC}"
echo "   Name: $VALIDATOR_NAME"
echo "   AURA: ${AURA_KEY:0:20}..."
echo "   GRANDPA: ${GRANDPA_KEY:0:20}..."
echo "   ASF: ${ASF_KEY:0:20}..."
echo ""

# Call the main bootstrap script
BOOTSTRAP_SCRIPT="./bootstrap-validator.sh"

if [ ! -f "$BOOTSTRAP_SCRIPT" ]; then
    echo -e "${RED}❌ Error: Bootstrap script not found: $BOOTSTRAP_SCRIPT${NC}"
    exit 1
fi

echo "🚀 Running bootstrap script..."
echo ""

bash "$BOOTSTRAP_SCRIPT" \
    "$VALIDATOR_NAME" \
    "$SESSION_SEED" \
    "$AURA_KEY" \
    "$GRANDPA_KEY" \
    "$ASF_KEY" \
    "$NODE_KEY"

echo ""
echo -e "${GREEN}✅ Quick bootstrap complete!${NC}"
