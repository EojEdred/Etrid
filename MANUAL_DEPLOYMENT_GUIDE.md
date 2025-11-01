# ËTRID - Manual Deployment Guide for 21 Validators

## Quick Deployment Steps

### Phase 1: Deploy to Build VM (98.71.91.84)
This is the multichain-dev-primary VM with monitoring deployed. Deploy here first:

```bash
# SSH to build VM
ssh -i ~/.ssh/gizzi-validator ubuntu@98.71.91.84

# Clone or update repo
sudo mkdir -p /opt/etrid
sudo chown -R $(whoami):$(whoami) /opt/etrid

if [ -d "/opt/etrid/.git" ]; then
    cd /opt/etrid && git pull origin main
else
    git clone https://github.com/EojEdred/Etrid.git /opt/etrid
    cd /opt/etrid
fi

git log --oneline -3  # Verify latest commit: 31fa14bf
```

### Phase 2: Deploy to Oracle Cloud VMs

**Gizzi (V#1) - 64.181.215.19:**
```bash
ssh -i ~/.ssh/gizzi-validator gizziio@64.181.215.19
# Then run the same git clone/pull commands above
```

**Audit (V#3) - 129.80.122.34:**
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@129.80.122.34
# Then run the same git clone/pull commands above
```

### Phase 3: Deploy to Azure Special Validators

**EojEdred (V#2) - 20.69.26.209:**
```bash
ssh -i ~/.ssh/gizzi-validator azureuser@20.69.26.209
# Run git clone/pull commands
```

**Governance Dev (V#4) - 20.186.91.207:**
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@20.186.91.207
# Run git clone/pull commands
```

### Phase 4: Deploy to Azure FlareNodes Using Azure CLI

For the remaining 16 Azure VMs, use Azure CLI with run-command:

```bash
# Login to Azure
az login

# Deploy to a single VM (example)
VM_NAME="etrid-consensus-dev-secondary"
RG="ETRID-VALIDATORS-EU-WEST"

az vm run-command invoke \
  --name "$VM_NAME" \
  --resource-group "$RG" \
  --command-id RunShellScript \
  --scripts '
    sudo mkdir -p /opt/etrid
    sudo chown -R azureuser:azureuser /opt/etrid

    if [ -d "/opt/etrid/.git" ]; then
        cd /opt/etrid
        git fetch origin
        git reset --hard origin/main
        git pull origin main
    else
        git clone https://github.com/EojEdred/Etrid.git /opt/etrid
    fi

    cd /opt/etrid
    echo "Deployed commit: $(git rev-parse --short HEAD)"
  '
```

### Phase 5: Batch Deploy to All Azure VMs

```bash
# Array of all Azure VMs
declare -a AZURE_VMS=(
  "ETRID-VALIDATORS-EU-WEST:etrid-consensus-dev-secondary"
  "ETRID-VALIDATORS-EU-WEST:etrid-runtime-dev-primary"
  "ETRID-VALIDATORS-EU-WEST:etrid-compiler-dev-primary"
  "ETRID-VALIDATORS-EU-WEST:etrid-compiler-dev-secondary"
  "ETRID-VALIDATORS-EU-NORTH:etrid-multichain-dev-primary"
  "ETRID-VALIDATORS-EU-NORTH:etrid-multichain-dev-secondary"
  "ETRID-VALIDATORS-EU-WEST:etrid-oracle-dev"
  "ETRID-VALIDATORS-UK-SOUTH:etrid-edsc-dev-primary"
  "ETRID-VALIDATORS-UK-SOUTH:etrid-edsc-dev-secondary"
  "ETRID-VALIDATORS-UK-SOUTH:etrid-economics-dev-primary"
  "ETRID-VALIDATORS-UK-SOUTH:etrid-economics-dev-secondary"
  "ETRID-VALIDATORS-UK-SOUTH:etrid-ethics-dev-primary"
  "ETRID-VALIDATORS-FR-CENTRAL:etrid-ethics-dev-secondary"
  "ETRID-VALIDATORS-FR-CENTRAL:etrid-docs-dev-primary"
  "ETRID-VALIDATORS-FR-CENTRAL:etrid-docs-dev-secondary"
  "ETRID-VALIDATORS-FR-CENTRAL:etrid-docs-dev-tertiary"
)

# Loop through and deploy
for VM_INFO in "${AZURE_VMS[@]}"; do
  RG=$(echo $VM_INFO | cut -d: -f1)
  VM=$(echo $VM_INFO | cut -d: -f2)

  echo "Deploying to $VM in $RG..."

  az vm run-command invoke \
    --name "$VM" \
    --resource-group "$RG" \
    --command-id RunShellScript \
    --scripts '
      sudo mkdir -p /opt/etrid
      sudo chown -R azureuser:azureuser /opt/etrid

      if [ -d "/opt/etrid/.git" ]; then
          cd /opt/etrid
          git pull origin main
      else
          git clone https://github.com/EojEdred/Etrid.git /opt/etrid
      fi

      echo "✓ Deployed: $(cd /opt/etrid && git rev-parse --short HEAD)"
    ' | grep "✓ Deployed"

  echo ""
done
```

## Verification

Check deployment on all VMs:

```bash
# SSH method
ssh -i ~/.ssh/gizzi-validator ubuntu@VM_IP "cd /opt/etrid && git log --oneline -1"

# Azure CLI method
az vm run-command invoke \
  --name "$VM_NAME" \
  --resource-group "$RG" \
  --command-id RunShellScript \
  --scripts 'cd /opt/etrid && git log --oneline -3'
```

Expected commit: **31fa14bf** - "Update validator IP addresses"

## Summary of All 21 Validators

| # | Name | IP | Method |
|---|------|----|----|
| 1 | Gizzi | 64.181.215.19 | SSH (gizziio) |
| 2 | EojEdred | 20.69.26.209 | SSH (azureuser) |
| 3 | Audit | 129.80.122.34 | SSH (ubuntu) |
| 4 | Governance | 20.186.91.207 | SSH (ubuntu) |
| 5-21 | FlareNodes/ValidityNodes | Various | Azure CLI |

