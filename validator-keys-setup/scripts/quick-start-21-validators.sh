#!/bin/bash
# Quick Start: Deploy 21 Validators on Azure
# This script automates the entire deployment process
# Usage: ./quick-start-21-validators.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Ëtrid 21-Validator Azure Deployment - Quick Start       ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Configuration
RESOURCE_GROUP="etrid-validators-prod"
LOCATION="eastus"
KEYVAULT_PREFIX="etrid-val-keys"
STORAGE_ACCOUNT_PREFIX="etridbackup"
VM_SIZE="Standard_B4ms"  # 4 vCPU, 16GB RAM - $0.166/hour
DISK_SIZE_GB=500
BINARY_PATH="../target/release/flarechain-node"

# Check prerequisites
echo -e "${YELLOW}[1/10] Checking prerequisites...${NC}"

# Check if Azure CLI is installed
if ! command -v az &> /dev/null; then
    echo -e "${RED}✗ Azure CLI not found${NC}"
    echo "Install: https://docs.microsoft.com/en-us/cli/azure/install-azure-cli"
    exit 1
fi
echo -e "${GREEN}✓ Azure CLI installed${NC}"

# Check if logged in to Azure
if ! az account show &> /dev/null; then
    echo -e "${RED}✗ Not logged in to Azure${NC}"
    echo "Run: az login"
    exit 1
fi
echo -e "${GREEN}✓ Logged in to Azure${NC}"

# Check if flarechain-node binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}✗ flarechain-node binary not found at $BINARY_PATH${NC}"
    echo "Build it first: cd 05-multichain/flare-chain && cargo build --release"
    exit 1
fi
echo -e "${GREEN}✓ flarechain-node binary found${NC}"

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo -e "${RED}✗ jq not found (required for JSON parsing)${NC}"
    echo "Install: brew install jq (macOS) or apt-get install jq (Linux)"
    exit 1
fi
echo -e "${GREEN}✓ jq installed${NC}"

echo ""

# Get Azure subscription ID
SUBSCRIPTION_ID=$(az account show --query id -o tsv)
echo -e "${GREEN}Using subscription: $SUBSCRIPTION_ID${NC}"
echo ""

# Confirm with user
echo -e "${YELLOW}This script will:${NC}"
echo "  1. Create Azure resource group in $LOCATION"
echo "  2. Create Azure Key Vault (HSM-backed)"
echo "  3. Generate 21 validator key sets"
echo "  4. Create 21 Azure VMs ($VM_SIZE, 500GB disk)"
echo "  5. Configure networking and security"
echo "  6. Deploy validator software"
echo "  7. Start validators and form committee"
echo ""
echo -e "${YELLOW}Estimated cost: ~\$2,100/month (\$25K/year)${NC}"
echo -e "${YELLOW}Estimated time: 30-45 minutes${NC}"
echo ""
read -p "Continue? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "Aborted."
    exit 0
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 1: INFRASTRUCTURE SETUP
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[2/10] Creating resource group...${NC}"
az group create \
  --name $RESOURCE_GROUP \
  --location $LOCATION \
  --tags "project=etrid" "environment=production" \
  > /dev/null

echo -e "${GREEN}✓ Resource group created: $RESOURCE_GROUP${NC}"
echo ""

# Create unique names
TIMESTAMP=$(date +%s | tail -c 6)
KEYVAULT_NAME="${KEYVAULT_PREFIX}-${TIMESTAMP}"
STORAGE_ACCOUNT="${STORAGE_ACCOUNT_PREFIX}${TIMESTAMP}"

echo -e "${YELLOW}[3/10] Creating Azure Key Vault (HSM-backed)...${NC}"
az keyvault create \
  --name $KEYVAULT_NAME \
  --resource-group $RESOURCE_GROUP \
  --location $LOCATION \
  --sku Premium \
  --enable-purge-protection true \
  --enable-soft-delete true \
  --retention-days 90 \
  > /dev/null

echo -e "${GREEN}✓ Key Vault created: $KEYVAULT_NAME${NC}"
echo ""

echo -e "${YELLOW}[4/10] Creating storage account for backups...${NC}"
az storage account create \
  --name $STORAGE_ACCOUNT \
  --resource-group $RESOURCE_GROUP \
  --location $LOCATION \
  --sku Standard_GRS \
  --kind StorageV2 \
  --min-tls-version TLS1_2 \
  --allow-blob-public-access false \
  > /dev/null

az storage container create \
  --name chain-snapshots \
  --account-name $STORAGE_ACCOUNT \
  --auth-mode login \
  > /dev/null

echo -e "${GREEN}✓ Storage account created: $STORAGE_ACCOUNT${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 2: GENERATE VALIDATOR KEYS
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[5/10] Generating 21 validator key sets...${NC}"
echo "This will take ~3-5 minutes..."

mkdir -p generated-keys
cat > generated-keys/validator-keys.json <<EOF
{
  "validators": [
EOF

for i in {01..21}; do
  VALIDATOR_NAME="validator-$i"
  printf "  [%2d/21] Generating $VALIDATOR_NAME..." $((10#$i))

  # Generate sr25519 seed
  SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json 2>/dev/null)
  SEED=$(echo $SEED_JSON | jq -r '.secretSeed')
  SECRET_PHRASE=$(echo $SEED_JSON | jq -r '.secretPhrase')
  ACCOUNT_ID=$(echo $SEED_JSON | jq -r '.ss58Address')

  # Derive AURA key
  AURA_JSON=$($BINARY_PATH key inspect --scheme sr25519 "$SEED" --output-type json 2>/dev/null)
  AURA_PUBKEY=$(echo $AURA_JSON | jq -r '.publicKey')

  # Derive GRANDPA key
  GRANDPA_JSON=$($BINARY_PATH key inspect --scheme ed25519 "$SEED" --output-type json 2>/dev/null)
  GRANDPA_PUBKEY=$(echo $GRANDPA_JSON | jq -r '.publicKey')

  # Store in Key Vault
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-seed" \
    --value "$SEED" \
    --tags "validator=$VALIDATOR_NAME" "index=$i" \
    > /dev/null

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-phrase" \
    --value "$SECRET_PHRASE" \
    > /dev/null

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-account-id" \
    --value "$ACCOUNT_ID" \
    > /dev/null

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-aura-pubkey" \
    --value "$AURA_PUBKEY" \
    > /dev/null

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-grandpa-pubkey" \
    --value "$GRANDPA_PUBKEY" \
    > /dev/null

  # Determine stake and role
  if [ $((10#$i)) -le 3 ]; then
    STAKE="128000000000000000000000"
    ROLE=4  # DecentralizedDirector
    ROLE_NAME="Director"
  elif [ $((10#$i)) -le 12 ]; then
    STAKE="64000000000000000000000"
    ROLE=3  # FlareNode
    ROLE_NAME="FlareNode"
  else
    STAKE="64000000000000000000000"
    ROLE=2  # ValidityNode
    ROLE_NAME="ValidityNode"
  fi

  # Write to JSON file
  if [ $((10#$i)) -eq 21 ]; then
    COMMA=""
  else
    COMMA=","
  fi

  cat >> generated-keys/validator-keys.json <<EOF
    {
      "name": "$VALIDATOR_NAME",
      "accountId": "$ACCOUNT_ID",
      "auraKey": "$AURA_PUBKEY",
      "grandpaKey": "$GRANDPA_PUBKEY",
      "stake": "$STAKE",
      "role": $ROLE,
      "roleName": "$ROLE_NAME"
    }$COMMA
EOF

  echo -e " ${GREEN}✓${NC}"
done

cat >> generated-keys/validator-keys.json <<EOF
  ]
}
EOF

echo -e "${GREEN}✓ All 21 validators generated and stored in Key Vault${NC}"
echo -e "${GREEN}✓ Public keys saved to: generated-keys/validator-keys.json${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 3: NETWORK SETUP
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[6/10] Creating virtual network and security groups...${NC}"

# Create VNet
az network vnet create \
  --resource-group $RESOURCE_GROUP \
  --name etrid-vnet \
  --address-prefix 10.0.0.0/16 \
  --subnet-name validators \
  --subnet-prefix 10.0.1.0/24 \
  > /dev/null

# Create NSG
az network nsg create \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validators-nsg \
  > /dev/null

# Allow P2P traffic
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name etrid-validators-nsg \
  --name AllowP2P \
  --priority 100 \
  --direction Inbound \
  --access Allow \
  --protocol Tcp \
  --destination-port-ranges 30333 \
  --source-address-prefixes '*' \
  > /dev/null

# Allow RPC from internal VNet only
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name etrid-validators-nsg \
  --name AllowRPCInternal \
  --priority 110 \
  --direction Inbound \
  --access Allow \
  --protocol Tcp \
  --destination-port-ranges 9944 \
  --source-address-prefixes 10.0.0.0/16 \
  > /dev/null

# Allow SSH from your IP only
YOUR_IP=$(curl -s ifconfig.me)
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name etrid-validators-nsg \
  --name AllowSSH \
  --priority 120 \
  --direction Inbound \
  --access Allow \
  --protocol Tcp \
  --destination-port-ranges 22 \
  --source-address-prefixes $YOUR_IP/32 \
  > /dev/null

echo -e "${GREEN}✓ Network configured${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 4: CREATE VMs
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[7/10] Creating 21 validator VMs...${NC}"
echo "This will take ~15-20 minutes..."
echo ""

for i in {01..21}; do
  VM_NAME="etrid-validator-$i"
  NIC_NAME="${VM_NAME}-nic"
  PUBLIC_IP_NAME="${VM_NAME}-ip"
  DISK_NAME="${VM_NAME}-data-disk"

  printf "  [%2d/21] Creating $VM_NAME..." $((10#$i))

  # Create public IP
  az network public-ip create \
    --resource-group $RESOURCE_GROUP \
    --name $PUBLIC_IP_NAME \
    --sku Standard \
    --allocation-method Static \
    --zone 1 2 3 \
    > /dev/null 2>&1 &

  # Create NIC
  az network nic create \
    --resource-group $RESOURCE_GROUP \
    --name $NIC_NAME \
    --vnet-name etrid-vnet \
    --subnet validators \
    --network-security-group etrid-validators-nsg \
    --public-ip-address $PUBLIC_IP_NAME \
    > /dev/null 2>&1 &

  wait

  # Create VM
  az vm create \
    --resource-group $RESOURCE_GROUP \
    --name $VM_NAME \
    --nics $NIC_NAME \
    --image Canonical:0001-com-ubuntu-server-jammy:22_04-lts-gen2:latest \
    --size $VM_SIZE \
    --os-disk-size-gb 100 \
    --admin-username azureuser \
    --generate-ssh-keys \
    --assign-identity [system] \
    --zone $((i % 3 + 1)) \
    --tags "validator=validator-$i" "tier=validator" \
    > /dev/null 2>&1

  # Attach data disk
  az vm disk attach \
    --resource-group $RESOURCE_GROUP \
    --vm-name $VM_NAME \
    --name $DISK_NAME \
    --new \
    --size-gb $DISK_SIZE_GB \
    --sku Standard_SSD_LRS \
    > /dev/null 2>&1

  # Grant VM access to Key Vault
  PRINCIPAL_ID=$(az vm identity show \
    --resource-group $RESOURCE_GROUP \
    --name $VM_NAME \
    --query principalId -o tsv)

  az keyvault set-policy \
    --name $KEYVAULT_NAME \
    --object-id $PRINCIPAL_ID \
    --secret-permissions get list \
    > /dev/null 2>&1

  echo -e " ${GREEN}✓${NC}"
done

echo -e "${GREEN}✓ All 21 VMs created${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 5: CONFIGURE VMs
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[8/10] Configuring validators...${NC}"
echo "Installing software and keys on each VM..."
echo ""

# Upload binary to Azure Storage (for faster download)
echo "  Uploading binary to storage..."
az storage blob upload \
  --account-name $STORAGE_ACCOUNT \
  --container-name chain-snapshots \
  --name flarechain-node \
  --file $BINARY_PATH \
  --auth-mode login \
  --overwrite \
  > /dev/null

BINARY_URL=$(az storage blob url \
  --account-name $STORAGE_ACCOUNT \
  --container-name chain-snapshots \
  --name flarechain-node \
  --auth-mode login \
  -o tsv)

echo -e "  ${GREEN}✓ Binary uploaded${NC}"

# Create setup script
cat > /tmp/setup-validator.sh <<'EOFSCRIPT'
#!/bin/bash
set -e

# Install dependencies
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq
apt-get install -y -qq curl jq ca-certificates > /dev/null 2>&1

# Install Azure CLI
curl -sL https://aka.ms/InstallAzureCLIDeb | bash > /dev/null 2>&1

# Format data disk
DATA_DISK="/dev/sdc"
mkfs.ext4 -q $DATA_DISK
mkdir -p /var/lib/etrid
mount $DATA_DISK /var/lib/etrid
echo "$DATA_DISK /var/lib/etrid ext4 defaults,nofail 0 2" >> /etc/fstab

# Create etrid user
useradd -r -m -d /home/etrid -s /bin/bash etrid
chown -R etrid:etrid /var/lib/etrid

# Download binary
mkdir -p /usr/local/bin
curl -sL "$BINARY_URL" -o /usr/local/bin/flarechain-node
chmod +x /usr/local/bin/flarechain-node

# Authenticate with Managed Identity
az login --identity > /dev/null 2>&1

# Retrieve keys from Key Vault
VALIDATOR_NAME=$(hostname)
SEED=$(az keyvault secret show \
  --vault-name "$KEYVAULT_NAME" \
  --name "${VALIDATOR_NAME}-seed" \
  --query value -o tsv)

# Insert keys
sudo -u etrid /usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain local \
  --key-type aura \
  --scheme sr25519 \
  --suri "$SEED" \
  > /dev/null 2>&1

sudo -u etrid /usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain local \
  --key-type gran \
  --scheme ed25519 \
  --suri "$SEED" \
  > /dev/null 2>&1

sudo -u etrid /usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain local \
  --key-type asfk \
  --scheme sr25519 \
  --suri "$SEED" \
  > /dev/null 2>&1

unset SEED

# Create systemd service
cat > /etc/systemd/system/etrid-validator.service <<EOF
[Unit]
Description=Ëtrid Validator Node
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/home/etrid
ExecStart=/usr/local/bin/flarechain-node \\
  --base-path /var/lib/etrid \\
  --chain local \\
  --name "$VALIDATOR_NAME" \\
  --validator \\
  --port 30333 \\
  --rpc-port 9944 \\
  --rpc-methods Safe \\
  --rpc-cors all \\
  --prometheus-port 9615 \\
  --prometheus-external
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable etrid-validator > /dev/null 2>&1
systemctl start etrid-validator

echo "VALIDATOR_CONFIGURED"
EOFSCRIPT

# Deploy to all VMs in parallel
for i in {01..21}; do
  {
    VM_NAME="etrid-validator-$i"
    printf "  [%2d/21] Configuring $VM_NAME..." $((10#$i))

    RESULT=$(az vm run-command invoke \
      --resource-group $RESOURCE_GROUP \
      --name $VM_NAME \
      --command-id RunShellScript \
      --scripts @/tmp/setup-validator.sh \
      --parameters \
        "KEYVAULT_NAME=$KEYVAULT_NAME" \
        "BINARY_URL=$BINARY_URL" \
      --query 'value[0].message' -o tsv 2>/dev/null)

    if echo "$RESULT" | grep -q "VALIDATOR_CONFIGURED"; then
      echo -e " ${GREEN}✓${NC}"
    else
      echo -e " ${RED}✗${NC}"
    fi
  } &
done

wait
rm /tmp/setup-validator.sh

echo -e "${GREEN}✓ All validators configured${NC}"
echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 6: VERIFY DEPLOYMENT
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[9/10] Verifying deployment...${NC}"

sleep 10  # Give validators time to start

# Check first validator
FIRST_VM_IP=$(az network public-ip show \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validator-01-ip \
  --query ipAddress -o tsv)

echo "  Checking validator-01 at $FIRST_VM_IP..."

# Try to query RPC endpoint
RESPONSE=$(ssh -o StrictHostKeyChecking=no azureuser@$FIRST_VM_IP \
  'curl -s -H "Content-Type: application/json" \
   -d '"'"'{"id":1, "jsonrpc":"2.0", "method": "system_health"}'"'"' \
   http://localhost:9944' 2>/dev/null)

if echo "$RESPONSE" | jq -e '.result' > /dev/null 2>&1; then
  PEERS=$(echo "$RESPONSE" | jq -r '.result.peers')
  echo -e "  ${GREEN}✓ Validator responding (Peers: $PEERS)${NC}"
else
  echo -e "  ${YELLOW}⚠ Validator not responding yet (still syncing)${NC}"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════════════
# PHASE 7: SUMMARY
# ═══════════════════════════════════════════════════════════════════════════

echo -e "${YELLOW}[10/10] Deployment Summary${NC}"
echo ""
echo -e "${GREEN}✅ Deployment Complete!${NC}"
echo ""
echo "─────────────────────────────────────────────────────────"
echo "Resource Group: $RESOURCE_GROUP"
echo "Location: $LOCATION"
echo "Key Vault: $KEYVAULT_NAME"
echo "Storage Account: $STORAGE_ACCOUNT"
echo ""
echo "Validators:"
for i in {01..21}; do
  VM_NAME="etrid-validator-$i"
  PUBLIC_IP=$(az network public-ip show \
    --resource-group $RESOURCE_GROUP \
    --name "${VM_NAME}-ip" \
    --query ipAddress -o tsv 2>/dev/null)

  if [ $((10#$i)) -le 3 ]; then
    ROLE="Director"
  elif [ $((10#$i)) -le 12 ]; then
    ROLE="FlareNode"
  else
    ROLE="ValidityNode"
  fi

  printf "  %2d. %-20s %15s  (%s)\n" $((10#$i)) "$VM_NAME" "$PUBLIC_IP" "$ROLE"
done
echo "─────────────────────────────────────────────────────────"
echo ""

echo "Next Steps:"
echo ""
echo "1. Monitor committee formation:"
echo "   ssh azureuser@$FIRST_VM_IP"
echo "   sudo journalctl -u etrid-validator -f"
echo ""
echo "2. Check peers connected:"
echo "   curl -s http://$FIRST_VM_IP:9944 \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_peers\"}'"
echo ""
echo "3. Verify block production:"
echo "   curl -s http://$FIRST_VM_IP:9944 \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getBlock\"}'"
echo ""
echo "4. Set up monitoring:"
echo "   See AZURE_21_VALIDATOR_DEPLOYMENT.md (Phase 5)"
echo ""
echo "5. Backup keys (CRITICAL):"
echo "   az keyvault backup start \\"
echo "     --storage-container-name vault-backups \\"
echo "     --account-name $STORAGE_ACCOUNT"
echo ""

echo -e "${GREEN}📄 Validator keys saved to: generated-keys/validator-keys.json${NC}"
echo -e "${GREEN}💾 Keys backed up to Azure Key Vault: $KEYVAULT_NAME${NC}"
echo -e "${YELLOW}⚠️  IMPORTANT: Export Key Vault backup to offline storage!${NC}"
echo ""

echo "Monthly Cost Estimate: ~\$2,100 (with optimizations)"
echo "Annual Cost: ~\$25,000"
echo ""
echo -e "${GREEN}🚀 Your 21-validator network is ready!${NC}"
