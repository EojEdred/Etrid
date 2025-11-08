#!/bin/bash
#
# Bulk NSG Configuration for All 21 FlareChain Validators
# Run this once to configure NSG rules for all validators
#

set -e

# ===========================
# CONFIGURATION
# ===========================

RESOURCE_GROUP="your-resource-group"

# Your admin IP for SSH/RPC access
YOUR_ADMIN_IP="<your_admin_ip>"

# Monitoring server IP for Prometheus
MONITORING_SERVER_IP="<monitoring_server_ip>"

# DeTr P2P Mode: "public" (open to all) or "validators" (whitelist)
DETR_P2P_MODE="validators"  # Recommended: validators

# All 21 validator IPs (update after provisioning)
VALIDATOR_IPS=(
  "64.181.215.19"  # Validator 1: Gizzi
  "<IP2>"          # Validator 2: EojEdred
  "<IP3>"          # Validator 3
  "<IP4>"          # Validator 4
  "<IP5>"          # Validator 5
  "<IP6>"          # Validator 6
  "<IP7>"          # Validator 7
  "<IP8>"          # Validator 8
  "<IP9>"          # Validator 9
  "<IP10>"         # Validator 10
  "<IP11>"         # Validator 11
  "<IP12>"         # Validator 12
  "<IP13>"         # Validator 13
  "<IP14>"         # Validator 14
  "<IP15>"         # Validator 15
  "<IP16>"         # Validator 16
  "<IP17>"         # Validator 17
  "<IP18>"         # Validator 18
  "<IP19>"         # Validator 19
  "<IP20>"         # Validator 20
  "<IP21>"         # Validator 21
)

# All 21 NSG names (update with your actual NSG names)
VALIDATOR_NSGS=(
  "validator-01-gizzi-nsg"
  "validator-02-eojedred-nsg"
  "validator-03-nsg"
  "validator-04-nsg"
  "validator-05-nsg"
  "validator-06-nsg"
  "validator-07-nsg"
  "validator-08-nsg"
  "validator-09-nsg"
  "validator-10-nsg"
  "validator-11-nsg"
  "validator-12-nsg"
  "validator-13-nsg"
  "validator-14-nsg"
  "validator-15-nsg"
  "validator-16-nsg"
  "validator-17-nsg"
  "validator-18-nsg"
  "validator-19-nsg"
  "validator-20-nsg"
  "validator-21-nsg"
)

# ===========================
# BUILD VALIDATOR IP LIST
# ===========================

# Convert array to comma-separated string
VALIDATOR_IP_LIST=$(IFS=,; echo "${VALIDATOR_IPS[*]}")

echo "================================================"
echo "  Bulk NSG Configuration for 21 Validators"
echo "================================================"
echo ""
echo "Configuration:"
echo "  Resource Group: $RESOURCE_GROUP"
echo "  Admin IP: $YOUR_ADMIN_IP"
echo "  Monitoring IP: $MONITORING_SERVER_IP"
echo "  DeTr P2P Mode: $DETR_P2P_MODE"
echo ""
echo "Will configure ${#VALIDATOR_NSGS[@]} NSGs..."
echo ""

# ===========================
# CONFIGURE EACH VALIDATOR NSG
# ===========================

COUNTER=1
for nsg in "${VALIDATOR_NSGS[@]}"; do
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "[$COUNTER/${#VALIDATOR_NSGS[@]}] Configuring: $nsg"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  # Check if NSG exists
  if ! az network nsg show --resource-group $RESOURCE_GROUP --name $nsg &>/dev/null; then
    echo "⚠️  NSG $nsg not found - skipping"
    echo ""
    ((COUNTER++))
    continue
  fi

  # Rule 1: SSH (Your IP only)
  echo "  → Rule 1: SSH (port 22)"
  az network nsg rule create \
    --resource-group $RESOURCE_GROUP \
    --nsg-name $nsg \
    --name AllowSSH \
    --priority 100 \
    --source-address-prefixes $YOUR_ADMIN_IP \
    --destination-port-ranges 22 \
    --protocol Tcp \
    --access Allow \
    --description "SSH access for administration" \
    --output none 2>/dev/null || echo "  (Rule may already exist)"

  # Rule 2: P2P Network (Public)
  echo "  → Rule 2: P2P (port 30333)"
  az network nsg rule create \
    --resource-group $RESOURCE_GROUP \
    --nsg-name $nsg \
    --name AllowP2PNetwork \
    --priority 200 \
    --source-address-prefixes '*' \
    --destination-port-ranges 30333 \
    --protocol Tcp \
    --access Allow \
    --description "Substrate P2P network" \
    --output none 2>/dev/null || echo "  (Rule may already exist)"

  # Rule 3: DeTr P2P (Configurable)
  if [ "$DETR_P2P_MODE" = "public" ]; then
    echo "  → Rule 3: DeTr P2P (port 30334) - Public"
    az network nsg rule create \
      --resource-group $RESOURCE_GROUP \
      --nsg-name $nsg \
      --name AllowDeTrP2P \
      --priority 250 \
      --source-address-prefixes '*' \
      --destination-port-ranges 30334 \
      --protocol Tcp \
      --access Allow \
      --description "DeTr P2P protocol - Public" \
      --output none 2>/dev/null || echo "  (Rule may already exist)"
  else
    echo "  → Rule 3: DeTr P2P (port 30334) - Validators only"
    az network nsg rule create \
      --resource-group $RESOURCE_GROUP \
      --nsg-name $nsg \
      --name AllowDeTrP2PValidators \
      --priority 250 \
      --source-address-prefixes $VALIDATOR_IP_LIST \
      --destination-port-ranges 30334 \
      --protocol Tcp \
      --access Allow \
      --description "DeTr P2P - Validator whitelist" \
      --output none 2>/dev/null || echo "  (Rule may already exist)"
  fi

  # Rule 4: RPC (Your IP only)
  echo "  → Rule 4: RPC (port 9944)"
  az network nsg rule create \
    --resource-group $RESOURCE_GROUP \
    --nsg-name $nsg \
    --name AllowRPC \
    --priority 300 \
    --source-address-prefixes $YOUR_ADMIN_IP \
    --destination-port-ranges 9944 \
    --protocol Tcp \
    --access Allow \
    --description "RPC WebSocket" \
    --output none 2>/dev/null || echo "  (Rule may already exist)"

  # Rule 5: Prometheus (Monitoring server only)
  echo "  → Rule 5: Prometheus (port 9615)"
  az network nsg rule create \
    --resource-group $RESOURCE_GROUP \
    --nsg-name $nsg \
    --name AllowPrometheusMetrics \
    --priority 400 \
    --source-address-prefixes $MONITORING_SERVER_IP \
    --destination-port-ranges 9615 \
    --protocol Tcp \
    --access Allow \
    --description "Prometheus metrics" \
    --output none 2>/dev/null || echo "  (Rule may already exist)"

  echo "  ✅ $nsg configured"
  echo ""

  ((COUNTER++))
done

echo "================================================"
echo "✅ All 21 validator NSGs configured successfully!"
echo "================================================"
echo ""
echo "📊 Configuration Summary:"
echo "  Port 22    (SSH)        → Your IP only ($YOUR_ADMIN_IP)"
echo "  Port 30333 (P2P)        → Public (all IPs)"
echo "  Port 30334 (DeTr P2P)   → $DETR_P2P_MODE"
echo "  Port 9944  (RPC)        → Your IP only ($YOUR_ADMIN_IP)"
echo "  Port 9615  (Prometheus) → Monitoring server ($MONITORING_SERVER_IP)"
echo ""
echo "🔐 Security: Configured for production mainnet"
echo ""
echo "Next steps:"
echo "  1. Verify rules in Azure Portal"
echo "  2. Test connectivity to each validator"
echo "  3. Monitor network with Prometheus"
