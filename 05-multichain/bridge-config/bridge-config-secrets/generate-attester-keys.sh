#!/bin/bash
#
# Generate Attester Keys for ËTRID Bridge
#
# This script generates ECDSA key pairs for bridge attesters using subkey.
# The keys are compatible with both Substrate and EVM environments.
#
# Usage:
#   ./generate-attester-keys.sh [number_of_attesters]
#
# Example:
#   ./generate-attester-keys.sh 5
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default number of attesters
NUM_ATTESTERS=${1:-5}

# Output directory
OUTPUT_DIR="$HOME/.etrid/attester-keys"
GENESIS_OUTPUT="./attester-genesis-generated.json"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   ËTRID Bridge Attester Key Generation                     ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if subkey is installed
if ! command -v subkey &> /dev/null; then
    echo -e "${RED}Error: subkey is not installed!${NC}"
    echo -e "${YELLOW}Install it with:${NC}"
    echo "  cargo install --force subkey --git https://github.com/paritytech/substrate"
    exit 1
fi

echo -e "${GREEN}✓ subkey found${NC}"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"
chmod 700 "$OUTPUT_DIR"

echo -e "${YELLOW}Generating $NUM_ATTESTERS attester key pairs...${NC}"
echo ""

# Array to store attester data
declare -a ATTESTERS

# Generate keys
for i in $(seq 1 $NUM_ATTESTERS); do
    echo -e "${BLUE}Generating Attester-$i...${NC}"

    # Generate ECDSA key pair
    KEY_OUTPUT=$(subkey generate --scheme ecdsa --output-type json 2>/dev/null)

    # Extract values
    SECRET_PHRASE=$(echo "$KEY_OUTPUT" | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)
    PUBLIC_KEY=$(echo "$KEY_OUTPUT" | grep -o '"publicKey":"[^"]*"' | cut -d'"' -f4)
    SS58_ADDRESS=$(echo "$KEY_OUTPUT" | grep -o '"ss58Address":"[^"]*"' | cut -d'"' -f4)

    # Save to files
    ATTESTER_DIR="$OUTPUT_DIR/attester-$i"
    mkdir -p "$ATTESTER_DIR"
    chmod 700 "$ATTESTER_DIR"

    # Save secret phrase
    echo "$SECRET_PHRASE" > "$ATTESTER_DIR/secret-phrase.txt"
    chmod 400 "$ATTESTER_DIR/secret-phrase.txt"

    # Save public key
    echo "$PUBLIC_KEY" > "$ATTESTER_DIR/public-key.txt"
    chmod 644 "$ATTESTER_DIR/public-key.txt"

    # Save SS58 address
    echo "$SS58_ADDRESS" > "$ATTESTER_DIR/ss58-address.txt"
    chmod 644 "$ATTESTER_DIR/ss58-address.txt"

    # Save full JSON
    echo "$KEY_OUTPUT" > "$ATTESTER_DIR/full-keypair.json"
    chmod 400 "$ATTESTER_DIR/full-keypair.json"

    # Store for genesis config
    ATTESTERS[$i]="$PUBLIC_KEY"

    echo -e "  ${GREEN}✓${NC} Public Key: $PUBLIC_KEY"
    echo -e "  ${GREEN}✓${NC} Address: $SS58_ADDRESS"
    echo -e "  ${GREEN}✓${NC} Saved to: $ATTESTER_DIR"
    echo ""
done

# Generate genesis configuration JSON
echo -e "${YELLOW}Generating genesis configuration...${NC}"

cat > "$GENESIS_OUTPUT" << EOF
{
  "attesters": [
EOF

ATTESTER_NAMES=("Alpha" "Beta" "Gamma" "Delta" "Epsilon" "Zeta" "Eta" "Theta" "Iota" "Kappa")

for i in $(seq 1 $NUM_ATTESTERS); do
    NAME="Attester-${ATTESTER_NAMES[$((i-1))]}"
    PUBLIC_KEY="${ATTESTERS[$i]}"

    cat >> "$GENESIS_OUTPUT" << EOF
    {
      "id": $i,
      "name": "$NAME",
      "public_key": "$PUBLIC_KEY",
      "enabled": true,
      "description": "Attester node $i - Auto-generated",
      "operator": "ETRID Network",
      "endpoint": "https://attester-$i.etrid.network"
    }
EOF

    if [ $i -lt $NUM_ATTESTERS ]; then
        echo "," >> "$GENESIS_OUTPUT"
    else
        echo "" >> "$GENESIS_OUTPUT"
    fi
done

cat >> "$GENESIS_OUTPUT" << EOF
  ],
  "threshold_config": {
    "min_signatures": $((NUM_ATTESTERS * 3 / 5)),
    "total_attesters": $NUM_ATTESTERS,
    "description": "$(($NUM_ATTESTERS * 3 / 5))-of-$NUM_ATTESTERS threshold signature scheme (M-of-N)",
    "byzantine_fault_tolerance": "Can tolerate up to $((NUM_ATTESTERS - (NUM_ATTESTERS * 3 / 5))) faulty or malicious attesters"
  },
  "security_parameters": {
    "attestation_max_age_blocks": 1000,
    "signature_type": "ECDSA",
    "hash_algorithm": "Blake2-256",
    "replay_protection": "nonce-based"
  },
  "generation_info": {
    "generated_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
    "tool": "generate-attester-keys.sh",
    "num_attesters": $NUM_ATTESTERS
  }
}
EOF

echo -e "${GREEN}✓ Genesis configuration saved to: $GENESIS_OUTPUT${NC}"
echo ""

# Print summary
echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Generation Complete                                      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Generated $NUM_ATTESTERS attester key pairs${NC}"
echo ""
echo -e "${YELLOW}Key Storage:${NC}"
echo -e "  Location: $OUTPUT_DIR"
echo -e "  Permissions: 700 (owner only)"
echo ""
echo -e "${YELLOW}Genesis Config:${NC}"
echo -e "  File: $GENESIS_OUTPUT"
echo -e "  Threshold: $(($NUM_ATTESTERS * 3 / 5))-of-$NUM_ATTESTERS"
echo ""
echo -e "${RED}⚠️  SECURITY WARNINGS:${NC}"
echo -e "${RED}  1. NEVER commit secret phrases to git!${NC}"
echo -e "${RED}  2. Store secret phrases in a secure password manager${NC}"
echo -e "${RED}  3. Consider using HSM for production key storage${NC}"
echo -e "${RED}  4. Back up keys to multiple secure locations${NC}"
echo -e "${RED}  5. Share public keys only - never share secret phrases${NC}"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo -e "  1. Review generated keys in: $OUTPUT_DIR"
echo -e "  2. Back up secret phrases securely"
echo -e "  3. Use $GENESIS_OUTPUT for chain genesis configuration"
echo -e "  4. Register attesters on-chain using public keys"
echo -e "  5. Configure threshold via governance"
echo ""
echo -e "${GREEN}Attester Public Keys:${NC}"
for i in $(seq 1 $NUM_ATTESTERS); do
    echo -e "  Attester-$i: ${ATTESTERS[$i]}"
done
echo ""

# Create a quick reference file
cat > "$OUTPUT_DIR/README.txt" << EOF
ËTRID Bridge Attester Keys
==========================

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Number of Attesters: $NUM_ATTESTERS
Threshold: $(($NUM_ATTESTERS * 3 / 5))-of-$NUM_ATTESTERS

Directory Structure:
--------------------
attester-1/
  ├── secret-phrase.txt       (KEEP SECRET!)
  ├── public-key.txt          (Share this)
  ├── ss58-address.txt        (Share this)
  └── full-keypair.json       (KEEP SECRET!)

attester-2/
  ...

Security Reminders:
-------------------
1. NEVER share secret-phrase.txt or full-keypair.json
2. NEVER commit these files to version control
3. Store backups in multiple secure locations
4. Use hardware security modules (HSM) for production
5. Rotate keys regularly (every 6-12 months)

Public Keys (safe to share):
-----------------------------
EOF

for i in $(seq 1 $NUM_ATTESTERS); do
    echo "Attester-$i: ${ATTESTERS[$i]}" >> "$OUTPUT_DIR/README.txt"
done

echo -e "${GREEN}✓ Quick reference saved to: $OUTPUT_DIR/README.txt${NC}"
echo ""
echo -e "${BLUE}Done!${NC}"
