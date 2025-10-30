# Azure 21-Validator Network Deployment Guide

## Overview

This guide provides a complete deployment strategy for 21 production validators on Azure, integrating your multi-tier peer system with proper key management and monitoring.

---

## Prerequisites

- Azure subscription with sufficient quota (21 VMs)
- Azure CLI installed (`az --version`)
- Terraform 1.5+ (for infrastructure as code)
- Compiled `flarechain-node` binary
- Genesis chain spec with 21 validator keys

---

## Phase 1: Infrastructure Setup (Week 1)

### Step 1.1: Create Resource Group

```bash
RESOURCE_GROUP="etrid-validators-prod"
LOCATION="eastus"
SUBSCRIPTION_ID=$(az account show --query id -o tsv)

az group create \
  --name $RESOURCE_GROUP \
  --location $LOCATION \
  --tags "project=etrid" "environment=production"
```

### Step 1.2: Create Virtual Network

```bash
# Create VNet
az network vnet create \
  --resource-group $RESOURCE_GROUP \
  --name etrid-vnet \
  --address-prefix 10.0.0.0/16 \
  --subnet-name validators \
  --subnet-prefix 10.0.1.0/24

# Create NSG (Network Security Group)
az network nsg create \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validators-nsg

# Allow P2P traffic (30333)
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name etrid-validators-nsg \
  --name AllowP2P \
  --priority 100 \
  --direction Inbound \
  --access Allow \
  --protocol Tcp \
  --destination-port-ranges 30333 \
  --source-address-prefixes '*'

# Allow RPC (9944) from internal VNet only
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name etrid-validators-nsg \
  --name AllowRPCInternal \
  --priority 110 \
  --direction Inbound \
  --access Allow \
  --protocol Tcp \
  --destination-port-ranges 9944 \
  --source-address-prefixes 10.0.0.0/16

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
  --source-address-prefixes $YOUR_IP/32
```

### Step 1.3: Create Azure Key Vault (Critical for Key Management)

```bash
KEYVAULT_NAME="etrid-val-keys-$(date +%s | tail -c 5)"  # Must be globally unique

az keyvault create \
  --name $KEYVAULT_NAME \
  --resource-group $RESOURCE_GROUP \
  --location $LOCATION \
  --sku Premium \
  --enable-purge-protection true \
  --enable-soft-delete true \
  --retention-days 90 \
  --tags "project=etrid" "criticality=high"

# Enable diagnostic logging
WORKSPACE_ID=$(az monitor log-analytics workspace create \
  --resource-group $RESOURCE_GROUP \
  --workspace-name etrid-logs \
  --query id -o tsv)

az monitor diagnostic-settings create \
  --name etrid-keyvault-logs \
  --resource $KEYVAULT_NAME \
  --resource-group $RESOURCE_GROUP \
  --resource-type Microsoft.KeyVault/vaults \
  --workspace $WORKSPACE_ID \
  --logs '[{"category": "AuditEvent", "enabled": true}]'
```

### Step 1.4: Create Storage Account (Chain Data Backups)

```bash
STORAGE_ACCOUNT="etridbackup$(date +%s | tail -c 8)"

az storage account create \
  --name $STORAGE_ACCOUNT \
  --resource-group $RESOURCE_GROUP \
  --location $LOCATION \
  --sku Standard_GRS \
  --kind StorageV2 \
  --min-tls-version TLS1_2 \
  --allow-blob-public-access false

# Create blob container for chain data snapshots
az storage container create \
  --name chain-snapshots \
  --account-name $STORAGE_ACCOUNT \
  --auth-mode login
```

---

## Phase 2: Generate & Store Validator Keys (Week 1)

### Step 2.1: Generate 21 Validator Key Sets

```bash
#!/bin/bash
# scripts/generate-21-validator-keys.sh

KEYVAULT_NAME="your-keyvault-name"
BINARY_PATH="/opt/etrid/flarechain-node"

echo "Generating keys for 21 validators..."
echo "This will take ~5 minutes"
echo ""

for i in {01..21}; do
  VALIDATOR_NAME="validator-$i"

  echo "[$i/21] Generating $VALIDATOR_NAME..."

  # Generate sr25519 seed (master key)
  SEED_JSON=$($BINARY_PATH key generate --scheme sr25519 --output-type json)
  SEED=$(echo $SEED_JSON | jq -r '.secretSeed')
  SECRET_PHRASE=$(echo $SEED_JSON | jq -r '.secretPhrase')
  ACCOUNT_ID=$(echo $SEED_JSON | jq -r '.ss58Address')

  # Derive AURA key (sr25519)
  AURA_JSON=$($BINARY_PATH key inspect --scheme sr25519 "$SEED" --output-type json)
  AURA_PUBKEY=$(echo $AURA_JSON | jq -r '.publicKey')

  # Derive GRANDPA key (ed25519)
  GRANDPA_JSON=$($BINARY_PATH key inspect --scheme ed25519 "$SEED" --output-type json)
  GRANDPA_PUBKEY=$(echo $GRANDPA_JSON | jq -r '.publicKey')

  # Store secret seed in Key Vault (HSM-backed)
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-seed" \
    --value "$SEED" \
    --tags "validator=$VALIDATOR_NAME" "index=$i" "type=sr25519_seed"

  # Store secret phrase (BIP39 recovery)
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-phrase" \
    --value "$SECRET_PHRASE"

  # Store public keys (for chain spec)
  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-account-id" \
    --value "$ACCOUNT_ID"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-aura-pubkey" \
    --value "$AURA_PUBKEY"

  az keyvault secret set \
    --vault-name $KEYVAULT_NAME \
    --name "${VALIDATOR_NAME}-grandpa-pubkey" \
    --value "$GRANDPA_PUBKEY"

  # Log to file (for chain spec generation)
  cat >> validator-keys.json <<EOF
{
  "name": "$VALIDATOR_NAME",
  "accountId": "$ACCOUNT_ID",
  "auraKey": "$AURA_PUBKEY",
  "grandpaKey": "$GRANDPA_PUBKEY",
  "stake": "64000000000000000000000"
},
EOF

  echo "  ✓ Stored in Key Vault: ${VALIDATOR_NAME}-*"
done

echo ""
echo "✅ All 21 validators generated and stored in Key Vault"
echo "📄 Public keys saved to: validator-keys.json"
echo ""
echo "⚠️  CRITICAL: Export vault backup to offline storage:"
echo "    az keyvault backup start --storage-container-name vault-backups --account-name $STORAGE_ACCOUNT"
```

### Step 2.2: Create Paper Backup (Disaster Recovery)

```bash
# Generate BIP39 recovery sheets
for i in {01..21}; do
  PHRASE=$(az keyvault secret show \
    --vault-name $KEYVAULT_NAME \
    --name "validator-$i-phrase" \
    --query value -o tsv)

  echo "Validator $i: $PHRASE" >> recovery-phrases.txt
done

# Print and store in bank vault
lpr recovery-phrases.txt  # Or: cat recovery-phrases.txt
rm recovery-phrases.txt  # Delete after printing
```

---

## Phase 3: Create Chain Spec with 21 Validators (Week 1)

### Step 3.1: Update Chain Spec Template

```bash
# Edit: infrastructure/chain-specs/mainnet-raw.json
# Based on validator-keys.json from Step 2.1

cat > infrastructure/chain-specs/mainnet-21-validators.json <<'EOF'
{
  "name": "Ëtrid Mainnet",
  "id": "etrid_mainnet",
  "chainType": "Live",
  "bootNodes": [
    "/dns4/bootnode-1.etrid.io/tcp/30333/p2p/PLACEHOLDER",
    "/dns4/bootnode-2.etrid.io/tcp/30333/p2p/PLACEHOLDER"
  ],
  "telemetryEndpoints": [
    ["wss://telemetry.etrid.io/submit/", 0]
  ],
  "protocolId": "etr",
  "properties": {
    "tokenSymbol": "ËTR",
    "tokenDecimals": 18,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "validatorCommittee": {
        "validators": [
          // Paste from validator-keys.json
          // Should have 21 entries
        ]
      },
      "staking": {
        "roles": [
          // Director assignments (validators 1-3)
          {
            "account": "VALIDATOR_01_ACCOUNT_ID",
            "role": 4,
            "stake": "128000000000000000000000",
            "active": true
          },
          {
            "account": "VALIDATOR_02_ACCOUNT_ID",
            "role": 4,
            "stake": "128000000000000000000000",
            "active": true
          },
          {
            "account": "VALIDATOR_03_ACCOUNT_ID",
            "role": 4,
            "stake": "128000000000000000000000",
            "active": true
          },
          // FlareNode assignments (validators 4-12)
          {
            "account": "VALIDATOR_04_ACCOUNT_ID",
            "role": 3,
            "stake": "64000000000000000000000",
            "active": true
          },
          // ... validators 5-12 ...

          // ValidityNode assignments (validators 13-21)
          {
            "account": "VALIDATOR_13_ACCOUNT_ID",
            "role": 2,
            "stake": "64000000000000000000000",
            "active": true
          }
          // ... validators 14-21 ...
        ]
      },
      "balances": {
        "balances": [
          // Fund each validator account (for tx fees)
          ["VALIDATOR_01_ACCOUNT_ID", "1000000000000000000000000"],  // 1M ËTR
          ["VALIDATOR_02_ACCOUNT_ID", "1000000000000000000000000"],
          // ... all 21 validators ...
        ]
      }
    }
  }
}
EOF

# Convert to raw format
./flarechain-node build-spec \
  --chain infrastructure/chain-specs/mainnet-21-validators.json \
  --raw \
  > infrastructure/chain-specs/mainnet-raw.json
```

---

## Phase 4: Deploy 21 VMs (Week 2)

### Step 4.1: Create VM Creation Script

```bash
#!/bin/bash
# scripts/azure-create-validators.sh

RESOURCE_GROUP="etrid-validators-prod"
VNET_NAME="etrid-vnet"
SUBNET_NAME="validators"
NSG_NAME="etrid-validators-nsg"
KEYVAULT_NAME="your-keyvault-name"

# VM Configuration
VM_SIZE="Standard_D4s_v5"  # 4 vCPU, 16GB RAM
OS_IMAGE="Canonical:0001-com-ubuntu-server-jammy:22_04-lts-gen2:latest"
DISK_SIZE_GB=500  # 500GB for chain data

echo "Creating 21 validator VMs..."
echo "This will take ~30 minutes"
echo ""

for i in {01..21}; do
  VM_NAME="etrid-validator-$i"
  NIC_NAME="${VM_NAME}-nic"
  PUBLIC_IP_NAME="${VM_NAME}-ip"
  DISK_NAME="${VM_NAME}-data-disk"

  echo "[$i/21] Creating $VM_NAME..."

  # Create public IP (static)
  az network public-ip create \
    --resource-group $RESOURCE_GROUP \
    --name $PUBLIC_IP_NAME \
    --sku Standard \
    --allocation-method Static \
    --zone 1 2 3  # Zone-redundant

  # Create NIC
  az network nic create \
    --resource-group $RESOURCE_GROUP \
    --name $NIC_NAME \
    --vnet-name $VNET_NAME \
    --subnet $SUBNET_NAME \
    --network-security-group $NSG_NAME \
    --public-ip-address $PUBLIC_IP_NAME

  # Create VM
  az vm create \
    --resource-group $RESOURCE_GROUP \
    --name $VM_NAME \
    --nics $NIC_NAME \
    --image $OS_IMAGE \
    --size $VM_SIZE \
    --os-disk-size-gb 100 \
    --admin-username azureuser \
    --generate-ssh-keys \
    --assign-identity [system] \
    --zone $((i % 3 + 1)) \
    --tags "validator=validator-$i" "tier=validator"

  # Attach data disk for chain storage
  az vm disk attach \
    --resource-group $RESOURCE_GROUP \
    --vm-name $VM_NAME \
    --name $DISK_NAME \
    --new \
    --size-gb $DISK_SIZE_GB \
    --sku Premium_LRS

  # Get VM's managed identity principal ID
  PRINCIPAL_ID=$(az vm identity show \
    --resource-group $RESOURCE_GROUP \
    --name $VM_NAME \
    --query principalId -o tsv)

  # Grant VM access to Key Vault (only its own keys)
  az keyvault set-policy \
    --name $KEYVAULT_NAME \
    --object-id $PRINCIPAL_ID \
    --secret-permissions get list

  # Get public IP
  PUBLIC_IP=$(az network public-ip show \
    --resource-group $RESOURCE_GROUP \
    --name $PUBLIC_IP_NAME \
    --query ipAddress -o tsv)

  echo "  ✓ Created: $VM_NAME (IP: $PUBLIC_IP)"
done

echo ""
echo "✅ All 21 VMs created successfully"
```

### Step 4.2: Configure VMs (Automated Setup)

```bash
#!/bin/bash
# scripts/azure-configure-validators.sh

RESOURCE_GROUP="etrid-validators-prod"
KEYVAULT_NAME="your-keyvault-name"
BINARY_URL="https://github.com/yourusername/etrid/releases/download/v1.0.0/flarechain-node-linux-amd64"
CHAIN_SPEC_URL="https://raw.githubusercontent.com/yourusername/etrid/main/infrastructure/chain-specs/mainnet-raw.json"

# This script runs on EACH VM
cat > setup-validator.sh <<'EOFSCRIPT'
#!/bin/bash
set -e

# 1. Install dependencies
apt-get update
apt-get install -y curl jq ca-certificates gnupg lsb-release

# 2. Install Azure CLI (for Key Vault access)
curl -sL https://aka.ms/InstallAzureCLIDeb | bash

# 3. Format and mount data disk
DATA_DISK="/dev/sdc"  # Azure data disks are usually sdc
mkfs.ext4 $DATA_DISK
mkdir -p /var/lib/etrid
mount $DATA_DISK /var/lib/etrid
echo "$DATA_DISK /var/lib/etrid ext4 defaults,nofail 0 2" >> /etc/fstab

# 4. Create etrid user
useradd -r -m -d /home/etrid -s /bin/bash etrid
chown -R etrid:etrid /var/lib/etrid

# 5. Download flarechain-node binary
curl -L $BINARY_URL -o /usr/local/bin/flarechain-node
chmod +x /usr/local/bin/flarechain-node

# 6. Download chain spec
mkdir -p /etc/etrid
curl -L $CHAIN_SPEC_URL -o /etc/etrid/mainnet-raw.json

# 7. Retrieve validator keys from Key Vault
VALIDATOR_NAME=$(hostname)  # Should be "etrid-validator-XX"
echo "Retrieving keys for $VALIDATOR_NAME from Key Vault..."

# Authenticate with Managed Identity
az login --identity

# Get seed
SEED=$(az keyvault secret show \
  --vault-name $KEYVAULT_NAME \
  --name "${VALIDATOR_NAME}-seed" \
  --query value -o tsv)

# Insert keys into keystore
sudo -u etrid /usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain /etc/etrid/mainnet-raw.json \
  --key-type aura \
  --scheme sr25519 \
  --suri "$SEED"

sudo -u etrid /usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain /etc/etrid/mainnet-raw.json \
  --key-type gran \
  --scheme ed25519 \
  --suri "$SEED"

sudo -u etrid /usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/etrid \
  --chain /etc/etrid/mainnet-raw.json \
  --key-type asfk \
  --scheme sr25519 \
  --suri "$SEED"

# Clear seed from memory
unset SEED

echo "✓ Keys inserted successfully"

# 8. Create systemd service
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
  --chain /etc/etrid/mainnet-raw.json \\
  --name "$VALIDATOR_NAME" \\
  --validator \\
  --port 30333 \\
  --rpc-port 9944 \\
  --rpc-methods Safe \\
  --rpc-cors all \\
  --prometheus-port 9615 \\
  --prometheus-external \\
  --telemetry-url 'wss://telemetry.etrid.io/submit/ 0' \\
  --bootnodes /dns4/bootnode-1.etrid.io/tcp/30333/p2p/BOOTNODE1_PEER_ID \\
  --bootnodes /dns4/bootnode-2.etrid.io/tcp/30333/p2p/BOOTNODE2_PEER_ID
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

# 9. Enable and start service
systemctl daemon-reload
systemctl enable etrid-validator
systemctl start etrid-validator

echo "✅ Validator setup complete!"
echo "Check logs: journalctl -u etrid-validator -f"
EOFSCRIPT

# Deploy to all VMs
for i in {01..21}; do
  VM_NAME="etrid-validator-$i"

  echo "Configuring $VM_NAME..."

  az vm run-command invoke \
    --resource-group $RESOURCE_GROUP \
    --name $VM_NAME \
    --command-id RunShellScript \
    --scripts @setup-validator.sh \
    --parameters \
      "KEYVAULT_NAME=$KEYVAULT_NAME" \
      "BINARY_URL=$BINARY_URL" \
      "CHAIN_SPEC_URL=$CHAIN_SPEC_URL"
done
```

---

## Phase 5: Monitoring & Alerts (Week 2)

### Step 5.1: Configure Prometheus + Grafana

```bash
# Create monitoring VM
az vm create \
  --resource-group $RESOURCE_GROUP \
  --name etrid-monitoring \
  --image Ubuntu2204 \
  --size Standard_D2s_v5 \
  --admin-username azureuser \
  --generate-ssh-keys

# SSH into monitoring VM
ssh azureuser@<MONITORING_VM_IP>

# Install Prometheus
wget https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
tar xvfz prometheus-*.tar.gz
cd prometheus-2.45.0.linux-amd64

# Configure Prometheus to scrape all validators
cat > prometheus.yml <<EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'etrid-validators'
    static_configs:
      - targets:
          - '10.0.1.4:9615'   # validator-01
          - '10.0.1.5:9615'   # validator-02
          # ... all 21 validators ...
          - '10.0.1.24:9615'  # validator-21
EOF

# Start Prometheus
./prometheus --config.file=prometheus.yml

# Install Grafana
sudo apt-get install -y grafana
sudo systemctl start grafana-server

# Access Grafana at http://<MONITORING_VM_IP>:3000
```

### Step 5.2: Configure Azure Alerts

```bash
# Alert when validator goes offline
az monitor metrics alert create \
  --name validator-offline \
  --resource-group $RESOURCE_GROUP \
  --scopes $(az vm list -g $RESOURCE_GROUP --query "[].id" -o tsv) \
  --condition "avg Percentage CPU < 1" \
  --window-size 5m \
  --evaluation-frequency 1m \
  --action-group-name etrid-ops-team

# Alert when disk usage > 80%
az monitor metrics alert create \
  --name validator-disk-full \
  --resource-group $RESOURCE_GROUP \
  --scopes $(az vm list -g $RESOURCE_GROUP --query "[].id" -o tsv) \
  --condition "avg Percentage Disk Used > 80" \
  --window-size 5m \
  --action-group-name etrid-ops-team
```

---

## Phase 6: Testing & Validation (Week 3)

### Step 6.1: Verify Committee Formation

```bash
# SSH into validator-01
ssh azureuser@validator-01-ip

# Check node logs
journalctl -u etrid-validator -f | grep "committee"

# Expected output:
# "Committee formed with 21 members"
# "Current PPFA index: 0"
# "Starting consensus session"
```

### Step 6.2: Test Block Production

```bash
# Query RPC endpoint
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
  http://10.0.1.4:9944

# Expected: Block numbers incrementing every 6 seconds
```

### Step 6.3: Test PPFA Rotation

```bash
# Watch proposer rotation
for i in {0..42}; do
  PROPOSER=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "etrid_getCurrentProposer"}' \
    http://10.0.1.4:9944 | jq -r '.result')

  echo "Block $i: Proposer $PROPOSER"
  sleep 6
done

# Should see rotation: validator-01 → validator-02 → ... → validator-21 → validator-01
```

---

## Phase 7: Disaster Recovery Plan

### Backup Procedures

```bash
# Daily snapshot of chain data
for i in {01..21}; do
  VM_NAME="etrid-validator-$i"

  az snapshot create \
    --resource-group $RESOURCE_GROUP \
    --name "${VM_NAME}-snapshot-$(date +%Y%m%d)" \
    --source "/subscriptions/$SUBSCRIPTION_ID/resourceGroups/$RESOURCE_GROUP/providers/Microsoft.Compute/disks/${VM_NAME}-data-disk"
done

# Weekly Key Vault backup
az backup vault create \
  --resource-group $RESOURCE_GROUP \
  --name etrid-backup-vault \
  --location $LOCATION

az backup protection enable-for-azurefileshare \
  --vault-name etrid-backup-vault \
  --resource-group $RESOURCE_GROUP \
  --policy-name DefaultPolicy \
  --storage-account $STORAGE_ACCOUNT \
  --azure-file-share keyvault-backups
```

### Recovery Procedures

```bash
# If validator goes down:
# 1. Check if VM is running
az vm get-instance-view \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validator-XX \
  --query instanceView.statuses

# 2. Restart VM
az vm restart \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validator-XX

# 3. Check logs
az vm run-command invoke \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validator-XX \
  --command-id RunShellScript \
  --scripts "journalctl -u etrid-validator -n 100"

# 4. If keys corrupted, re-insert from Key Vault
az vm run-command invoke \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validator-XX \
  --command-id RunShellScript \
  --scripts @scripts/retrieve-keys-from-vault.sh
```

---

## Cost Optimization Tips

1. **Use Reserved Instances**: 1-year reservation = 30% savings
   ```bash
   az reservations reservation-order purchase \
     --reservation-order-id $(uuidgen) \
     --sku-name Standard_D4s_v5 \
     --location eastus \
     --quantity 21 \
     --term P1Y
   ```

2. **Auto-shutdown non-critical VMs**: Monitoring VM can shutdown at night
   ```bash
   az vm auto-shutdown \
     --resource-group $RESOURCE_GROUP \
     --name etrid-monitoring \
     --time 0200
   ```

3. **Use Standard SSD instead of Premium**: 50% cheaper, acceptable for most validators
   ```bash
   # Update disk SKU
   az disk update \
     --resource-group $RESOURCE_GROUP \
     --name etrid-validator-XX-data-disk \
     --sku StandardSSD_LRS
   ```

---

## Security Hardening

```bash
# 1. Enable Azure Defender
az security pricing create \
  --name VirtualMachines \
  --tier standard

# 2. Enable disk encryption
for i in {01..21}; do
  az vm encryption enable \
    --resource-group $RESOURCE_GROUP \
    --name etrid-validator-$i \
    --disk-encryption-keyvault $KEYVAULT_NAME
done

# 3. Enable JIT (Just-In-Time) SSH access
az security jit-policy create \
  --resource-group $RESOURCE_GROUP \
  --location $LOCATION \
  --name etrid-jit-policy \
  --vm-ids $(az vm list -g $RESOURCE_GROUP --query "[].id" -o tsv) \
  --port 22 \
  --protocol TCP \
  --max-request-duration PT3H
```

---

## Maintenance Schedule

```
Weekly:
- Review validator uptime (should be >99.5%)
- Check disk usage (alert if >70%)
- Review slashing events (should be 0)
- Update security patches

Monthly:
- Rotate non-critical secrets
- Review Key Vault access logs
- Test disaster recovery procedure
- Update monitoring dashboards

Quarterly:
- Upgrade node software (coordinated with network)
- Review and optimize costs
- Audit validator performance
- Update documentation

Annually:
- Director elections (Consensus Day)
- Major version upgrades
- Security audit
- Business continuity drill
```

---

## Troubleshooting

### Issue: Committee won't form (< 21 nodes online)

**Diagnosis:**
```bash
# Check how many peers are connected
curl -s http://10.0.1.4:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
  | jq '.result | length'
```

**Solution:**
- Ensure all 21 VMs are running
- Check NSG rules allow port 30333
- Verify bootnodes are reachable
- Check validator logs for errors

### Issue: Keys not loading

**Diagnosis:**
```bash
ssh azureuser@validator-XX-ip
sudo ls -la /var/lib/etrid/keystore/
# Should see 3 files (aura, gran, asfk)
```

**Solution:**
```bash
# Re-run key insertion
az vm run-command invoke \
  --resource-group $RESOURCE_GROUP \
  --name etrid-validator-XX \
  --command-id RunShellScript \
  --scripts @scripts/retrieve-keys-from-vault.sh
```

### Issue: High disk usage

**Solution:**
```bash
# Prune old blocks (keep last 256 blocks)
flarechain-node purge-chain \
  --base-path /var/lib/etrid \
  --chain /etc/etrid/mainnet-raw.json \
  --keep-blocks 256
```

---

## Summary

**Timeline:**
- Week 1: Infrastructure + Keys (Steps 1-3)
- Week 2: VM Deployment + Monitoring (Steps 4-5)
- Week 3: Testing + Validation (Step 6)
- Week 4: Production launch

**Total Cost:** ~$3,000-5,000/month depending on options
**Estimated Setup Time:** 40-60 hours
**Team Required:** 2-3 DevOps engineers

**Next Steps:**
1. Run `scripts/generate-21-validator-keys.sh`
2. Update chain spec with generated keys
3. Deploy infrastructure with `scripts/azure-create-validators.sh`
4. Monitor committee formation
5. Celebrate when all 21 validators are producing blocks! 🎉

---

**Questions?** Review the troubleshooting section or open an issue in the repo.
