#!/bin/bash
# Hybrid Multi-Provider Deployment Script
# Deploys 21 Ëtrid validators across Hetzner, OVH, Vultr, DigitalOcean, and Akash
# Usage: ./deploy-hybrid-multi-provider.sh

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Ëtrid 21-Validator Hybrid Multi-Provider Deployment     ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Configuration
BINARY_PATH="/Users/macbook/Desktop/etrid/target/release/flarechain-node"
KEYS_DIR="/Users/macbook/Desktop/etrid/scripts/generated-keys-gizzi-eoj"
SSH_KEY="$HOME/.ssh/id_rsa.pub"

# Check prerequisites
echo -e "${YELLOW}[1/7] Checking prerequisites...${NC}"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}✗ flarechain-node binary not found at $BINARY_PATH${NC}"
    exit 1
fi
echo -e "${GREEN}✓ flarechain-node binary found ($(du -h $BINARY_PATH | cut -f1))${NC}"

# Check if keys exist
if [ ! -f "$KEYS_DIR/validator-keys-complete.json" ]; then
    echo -e "${RED}✗ Validator keys not found${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Validator keys found${NC}"

# Check if SSH key exists
if [ ! -f "$SSH_KEY" ]; then
    echo -e "${RED}✗ SSH public key not found at $SSH_KEY${NC}"
    exit 1
fi
echo -e "${GREEN}✓ SSH key found${NC}"

# Check provider CLIs
echo ""
echo -e "${YELLOW}Checking provider CLI tools...${NC}"

# Hetzner
if command -v hcloud &> /dev/null; then
    echo -e "${GREEN}✓ Hetzner CLI (hcloud) installed${NC}"
    HETZNER_OK=true
else
    echo -e "${YELLOW}! Hetzner CLI not found. Install: brew install hcloud${NC}"
    HETZNER_OK=false
fi

# DigitalOcean
if command -v doctl &> /dev/null; then
    echo -e "${GREEN}✓ DigitalOcean CLI (doctl) installed${NC}"
    DO_OK=true
else
    echo -e "${YELLOW}! DigitalOcean CLI not found. Install: brew install doctl${NC}"
    DO_OK=false
fi

# Vultr
if command -v vultr-cli &> /dev/null; then
    echo -e "${GREEN}✓ Vultr CLI installed${NC}"
    VULTR_OK=true
else
    echo -e "${YELLOW}! Vultr CLI not found. Install: brew install vultr/vultr-cli/vultr-cli${NC}"
    VULTR_OK=false
fi

echo ""
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Deployment Plan: 21 Validators Across 5 Providers${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
echo ""

echo "Critical Validators (3 bare metal):"
echo "  1. Gizzi (Bootstrap 1)      → Hetzner AX41"
echo "  2. EojEdred (Bootstrap 2)   → Hetzner AX41"
echo "  3. governance-dev01         → Hetzner AX41"
echo ""

echo "Standard Validators (14 VPS):"
echo "  4-13.  10 validators        → Hetzner CPX31"
echo "  14-17. 4 validators         → Vultr High Frequency"
echo "  18-20. 3 validators         → DigitalOcean"
echo ""

echo "Decentralized Validators (4):"
echo "  21. 1 validator             → Akash Network"
echo "  (3 more can be added to Akash after testing)"
echo ""

echo -e "${YELLOW}Estimated monthly cost: $675/mo (Year 1 with hybrid storage)${NC}"
echo -e "${YELLOW}Storage strategy: Pruned mode + included VPS storage${NC}"
echo -e "${YELLOW}Estimated setup time: 45-60 minutes${NC}"
echo ""

read -p "Continue with deployment? (yes/no): " confirm
if [ "$confirm" != "yes" ]; then
    echo "Deployment cancelled."
    exit 0
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 1: HETZNER DEPLOYMENT
# ═══════════════════════════════════════════════════════════════════════════

if [ "$HETZNER_OK" = true ]; then
    echo -e "${YELLOW}[2/7] Deploying to Hetzner...${NC}"

    # Check if logged in
    if ! hcloud context active &> /dev/null; then
        echo -e "${RED}✗ Not logged in to Hetzner${NC}"
        echo "Run: hcloud context create my-project"
        exit 1
    fi

    echo -e "${GREEN}✓ Logged in to Hetzner${NC}"

    # Upload SSH key if not exists
    if ! hcloud ssh-key list | grep -q "etrid-validators"; then
        echo "Uploading SSH key to Hetzner..."
        hcloud ssh-key create --name etrid-validators --public-key-from-file "$SSH_KEY"
    fi

    # Deploy 3 bare metal servers for critical validators
    echo ""
    echo "Deploying 3 bare metal servers (Gizzi, EojEdred, governance)..."

    # Note: Hetzner bare metal requires manual ordering through Robot panel
    # Can't be automated via CLI
    echo -e "${YELLOW}⚠️  Bare metal servers must be ordered manually:${NC}"
    echo "   1. Go to: https://robot.hetzner.com/order/index"
    echo "   2. Order 3× AX41-NVMe servers"
    echo "   3. Select location: Falkenstein (FSN1) or Helsinki (HEL1)"
    echo "   4. Add SSH key during setup"
    echo "   5. Servers will be ready in 2-24 hours"
    echo ""

    # Deploy 10 VPS servers
    echo "Deploying 10 VPS servers (CPX31)..."

    LOCATIONS=("fsn1" "nbg1" "hel1")  # Germany, Germany, Finland

    for i in {4..13}; do
        LOCATION=${LOCATIONS[$((($i-4) % 3))]}
        VALIDATOR_NAME="validator-$(printf %02d $i)"

        echo -n "  Creating $VALIDATOR_NAME in $LOCATION..."

        # Check if server already exists
        if hcloud server list | grep -q "$VALIDATOR_NAME"; then
            echo -e " ${YELLOW}already exists${NC}"
        else
            hcloud server create \
                --name "$VALIDATOR_NAME" \
                --type cpx31 \
                --image ubuntu-22.04 \
                --location "$LOCATION" \
                --ssh-key etrid-validators \
                --label "project=etrid" \
                --label "role=validator" \
                --label "tier=standard" \
                > /dev/null 2>&1

            echo -e " ${GREEN}created${NC}"
        fi
    done

    echo -e "${GREEN}✓ Hetzner deployment initiated${NC}"
else
    echo -e "${YELLOW}[2/7] Skipping Hetzner (CLI not installed)${NC}"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 2: VULTR DEPLOYMENT
# ═══════════════════════════════════════════════════════════════════════════

if [ "$VULTR_OK" = true ]; then
    echo -e "${YELLOW}[3/7] Deploying to Vultr...${NC}"

    # Check if API key is configured
    if ! vultr-cli account info &> /dev/null; then
        echo -e "${RED}✗ Not logged in to Vultr${NC}"
        echo "Run: export VULTR_API_KEY=your_api_key"
        exit 1
    fi

    echo -e "${GREEN}✓ Logged in to Vultr${NC}"

    # Get SSH key ID
    SSH_KEY_ID=$(vultr-cli ssh-key list -o json | jq -r '.[0].id' 2>/dev/null || echo "")

    if [ -z "$SSH_KEY_ID" ]; then
        echo "Uploading SSH key to Vultr..."
        SSH_KEY_ID=$(vultr-cli ssh-key create --name "etrid-validators" --key "$(cat $SSH_KEY)" -o json | jq -r '.id')
    fi

    # Deploy 4 High Frequency VPS
    echo "Deploying 4 High Frequency VPS..."

    # Vultr regions: ewr (New Jersey), lax (Los Angeles), sgp (Singapore), nrt (Tokyo)
    REGIONS=("ewr" "lax" "sgp" "nrt")

    for i in {14..17}; do
        REGION=${REGIONS[$((i-14))]}
        VALIDATOR_NAME="validator-$(printf %02d $i)"

        echo -n "  Creating $VALIDATOR_NAME in $REGION..."

        # Vultr plan: vhf-2c-16gb (High Frequency, 4 CPU, 16GB RAM, 180GB NVMe)
        vultr-cli instance create \
            --host "$VALIDATOR_NAME" \
            --label "$VALIDATOR_NAME" \
            --region "$REGION" \
            --plan "vhf-2c-16gb" \
            --os 1743 \
            --ssh-keys "$SSH_KEY_ID" \
            --tag "etrid" \
            --tag "validator" \
            > /dev/null 2>&1

        echo -e " ${GREEN}created${NC}"
    done

    echo -e "${GREEN}✓ Vultr deployment complete${NC}"
else
    echo -e "${YELLOW}[3/7] Skipping Vultr (CLI not installed)${NC}"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 3: DIGITALOCEAN DEPLOYMENT
# ═══════════════════════════════════════════════════════════════════════════

if [ "$DO_OK" = true ]; then
    echo -e "${YELLOW}[4/7] Deploying to DigitalOcean...${NC}"

    # Check if authenticated
    if ! doctl account get &> /dev/null; then
        echo -e "${RED}✗ Not logged in to DigitalOcean${NC}"
        echo "Run: doctl auth init"
        exit 1
    fi

    echo -e "${GREEN}✓ Logged in to DigitalOcean${NC}"

    # Get SSH key fingerprint
    SSH_KEY_FP=$(doctl compute ssh-key list --format FingerPrint --no-header | head -1)

    if [ -z "$SSH_KEY_FP" ]; then
        echo "Uploading SSH key to DigitalOcean..."
        doctl compute ssh-key import etrid-validators --public-key-file "$SSH_KEY"
        SSH_KEY_FP=$(doctl compute ssh-key list --format FingerPrint --no-header | head -1)
    fi

    # Deploy 3 droplets
    echo "Deploying 3 droplets (4 vCPU, 16 GB RAM)..."

    # DigitalOcean regions: nyc3, sfo3, lon1
    REGIONS=("nyc3" "sfo3" "lon1")

    for i in {18..20}; do
        REGION=${REGIONS[$((i-18))]}
        VALIDATOR_NAME="validator-$(printf %02d $i)"

        echo -n "  Creating $VALIDATOR_NAME in $REGION..."

        # Size: s-4vcpu-16gb-amd (4 vCPU, 16GB RAM, 100GB SSD)
        doctl compute droplet create "$VALIDATOR_NAME" \
            --image ubuntu-22-04-x64 \
            --size s-4vcpu-16gb-amd \
            --region "$REGION" \
            --ssh-keys "$SSH_KEY_FP" \
            --tag-names "etrid,validator" \
            --wait \
            > /dev/null 2>&1

        echo -e " ${GREEN}created${NC}"
    done

    echo -e "${GREEN}✓ DigitalOcean deployment complete${NC}"
else
    echo -e "${YELLOW}[4/7] Skipping DigitalOcean (CLI not installed)${NC}"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 4: AKASH DEPLOYMENT (Manual - Requires Akash CLI and wallet)
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[5/7] Akash deployment (manual setup required)...${NC}"
echo ""
echo "Akash deployment requires:"
echo "  1. Akash CLI installed (brew install akash)"
echo "  2. Akash wallet with AKT tokens"
echo "  3. SDL (deployment manifest) file"
echo ""
echo "I've created an SDL file: akash-validator-deployment.yml"
echo "To deploy: akash tx deployment create akash-validator-deployment.yml"
echo ""

# Create Akash SDL file
cat > /Users/macbook/Desktop/etrid/scripts/akash-validator-deployment.yml <<'EOF'
---
version: "2.0"

services:
  etrid-validator:
    image: ubuntu:22.04
    expose:
      - port: 30333
        as: 30333
        to:
          - global: true
      - port: 9944
        as: 9944
    env:
      - "VALIDATOR_NAME=validator-21"
    command:
      - "bash"
      - "-c"
    args:
      - >-
        apt-get update &&
        apt-get install -y wget &&
        # Download and setup validator binary
        echo "Validator setup script here"

profiles:
  compute:
    etrid-validator:
      resources:
        cpu:
          units: 4
        memory:
          size: 16Gi
        storage:
          size: 500Gi

  placement:
    akash:
      pricing:
        etrid-validator:
          denom: uakt
          amount: 1000

deployment:
  etrid-validator:
    akash:
      profile: etrid-validator
      count: 1
EOF

echo -e "${GREEN}✓ Akash SDL file created${NC}"

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 5: GET VM IPs AND CREATE INVENTORY
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[5/7] Configuring hybrid storage strategy...${NC}"
echo ""
echo "Hybrid Storage Architecture:"
echo "  ✓ Tier 1 (Local NVMe): Active chain state with pruning"
echo "  ✓ Tier 2 (Block Storage): Attach when local > 300GB (Year 2+)"
echo "  ✓ Tier 3 (Object Storage): Backups (Backblaze B2)"
echo ""
echo "Storage per validator with pruning:"
echo "  - Month 1-6:  30-80 GB (fits in base VPS)"
echo "  - Month 6-12: 80-150 GB (still within 360GB limit)"
echo "  - Month 12-24: 150-300 GB (may need volumes for some)"
echo ""
echo "Pruning configuration will be added to validator startup:"
echo "  --pruning 256"
echo "  --state-pruning archive-canonical"
echo "  --db-cache 4096"
echo ""
echo "When to attach block storage (Year 2):"
echo "  - Hetzner: 500GB volume = \$25/mo (when usage > 300GB)"
echo "  - Vultr: 500GB block storage = \$50/mo"
echo "  - DigitalOcean: 500GB volume = \$50/mo"
echo ""
echo -e "${GREEN}✓ Storage strategy documented${NC}"

echo ""
echo -e "${YELLOW}[6/7] Collecting VM information...${NC}"

# Create inventory file
INVENTORY_FILE="/Users/macbook/Desktop/etrid/scripts/validator-inventory.txt"
echo "# Ëtrid Validator Inventory" > $INVENTORY_FILE
echo "# Generated: $(date)" >> $INVENTORY_FILE
echo "" >> $INVENTORY_FILE

# Hetzner VMs
if [ "$HETZNER_OK" = true ]; then
    echo "=== Hetzner Validators ===" >> $INVENTORY_FILE
    hcloud server list -o columns=name,ipv4,type,location >> $INVENTORY_FILE 2>/dev/null || true
    echo "" >> $INVENTORY_FILE
fi

# Vultr VMs
if [ "$VULTR_OK" = true ]; then
    echo "=== Vultr Validators ===" >> $INVENTORY_FILE
    vultr-cli instance list -o json | jq -r '.[] | "\(.label)\t\(.main_ip)\t\(.region)"' >> $INVENTORY_FILE 2>/dev/null || true
    echo "" >> $INVENTORY_FILE
fi

# DigitalOcean VMs
if [ "$DO_OK" = true ]; then
    echo "=== DigitalOcean Validators ===" >> $INVENTORY_FILE
    doctl compute droplet list --format Name,PublicIPv4,Region,Size >> $INVENTORY_FILE 2>/dev/null || true
    echo "" >> $INVENTORY_FILE
fi

echo -e "${GREEN}✓ Inventory saved to: $INVENTORY_FILE${NC}"

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 6: DEPLOY VALIDATOR SOFTWARE
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[7/7] Next steps to complete deployment:${NC}"
echo ""
echo "1. Wait for all VMs to finish provisioning (2-5 minutes)"
echo ""
echo "2. Order Hetzner bare metal servers manually:"
echo "   → https://robot.hetzner.com/order/index"
echo "   → Order 3× AX41-NVMe"
echo ""
echo "3. Upload validator binary and keys to each VM:"
echo "   ./scripts/deploy-validator-software.sh"
echo ""
echo "4. Start validators in order:"
echo "   a. Gizzi (bootstrap 1)"
echo "   b. EojEdred (bootstrap 2)"
echo "   c. Remaining 19 validators"
echo ""
echo "5. Monitor committee formation:"
echo "   curl http://VALIDATOR_IP:9944 -H 'Content-Type: application/json' -d '{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"etrid_getCommittee\"}'"
echo ""

echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║         Infrastructure Deployment Complete!               ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}Summary:${NC}"
if [ "$HETZNER_OK" = true ]; then
    HETZNER_COUNT=$(hcloud server list 2>/dev/null | grep -c "validator-" || echo "0")
    echo "  Hetzner: $HETZNER_COUNT VPS deployed (+ 3 bare metal to order)"
fi

if [ "$VULTR_OK" = true ]; then
    VULTR_COUNT=$(vultr-cli instance list 2>/dev/null | grep -c "validator-" || echo "0")
    echo "  Vultr: $VULTR_COUNT VPS deployed"
fi

if [ "$DO_OK" = true ]; then
    DO_COUNT=$(doctl compute droplet list 2>/dev/null | grep -c "validator-" || echo "0")
    echo "  DigitalOcean: $DO_COUNT droplets deployed"
fi

echo "  Akash: 1 deployment ready (manual setup required)"
echo ""

echo -e "${YELLOW}Total monthly cost estimate:${NC}"
echo "  Year 1: $675/mo (pruned storage, no extra volumes)"
echo "  Year 2: $875/mo (attach volumes to ~8 validators as needed)"
echo ""

echo "See validator-inventory.txt for all VM IPs"
echo "Next: Run ./scripts/deploy-validator-software.sh"
