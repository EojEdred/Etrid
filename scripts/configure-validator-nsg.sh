#!/bin/bash
#
# Complete NSG Configuration for FlareChain Validator
# Configures all required ports with proper security
#

set -e

# ===========================
# CONFIGURATION - UPDATE THESE
# ===========================

RESOURCE_GROUP="your-resource-group"
VALIDATOR_NSG="validator-nsg"  # Change per validator

# Security: Your admin IP for SSH/RPC access
YOUR_ADMIN_IP="<your_admin_ip>"

# Monitoring: Prometheus server IP
MONITORING_SERVER_IP="<monitoring_server_ip>"

# Whitelisting: All 21 validator IPs (comma-separated)
# Update this after all VMs are provisioned
VALIDATOR_IPS="64.181.215.19,<IP2>,<IP3>,<IP4>,<IP5>,<IP6>,<IP7>,<IP8>,<IP9>,<IP10>,<IP11>,<IP12>,<IP13>,<IP14>,<IP15>,<IP16>,<IP17>,<IP18>,<IP19>,<IP20>,<IP21>"

# DeTr P2P Access Mode
# Options: "public" (open to all) or "validators" (whitelist only)
DETR_P2P_MODE="public"  # Change to "validators" for whitelist

# ===========================
# NSG RULES CREATION
# ===========================

echo "🔧 Configuring NSG: $VALIDATOR_NSG"
echo ""

# Rule 1: SSH Access (Your IP only)
echo "📌 Rule 1: SSH (port 22) - Admin access only"
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $VALIDATOR_NSG \
  --name AllowSSH \
  --priority 100 \
  --source-address-prefixes $YOUR_ADMIN_IP \
  --destination-port-ranges 22 \
  --protocol Tcp \
  --access Allow \
  --description "SSH access for administration"

# Rule 2: P2P Network - Port 30333 (Public - Required)
echo "📌 Rule 2: P2P (port 30333) - Public access"
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $VALIDATOR_NSG \
  --name AllowP2PNetwork \
  --priority 200 \
  --source-address-prefixes '*' \
  --destination-port-ranges 30333 \
  --protocol Tcp \
  --access Allow \
  --description "Substrate P2P network (libp2p)"

# Rule 3: DeTr P2P - Port 30334 (Configurable)
if [ "$DETR_P2P_MODE" = "public" ]; then
  echo "📌 Rule 3: DeTr P2P (port 30334) - Public access"
  az network nsg rule create \
    --resource-group $RESOURCE_GROUP \
    --nsg-name $VALIDATOR_NSG \
    --name AllowDeTrP2P \
    --priority 250 \
    --source-address-prefixes '*' \
    --destination-port-ranges 30334 \
    --protocol Tcp \
    --access Allow \
    --description "DeTr P2P protocol - Public access"
else
  echo "📌 Rule 3: DeTr P2P (port 30334) - Validator whitelist only"
  az network nsg rule create \
    --resource-group $RESOURCE_GROUP \
    --nsg-name $VALIDATOR_NSG \
    --name AllowDeTrP2PValidators \
    --priority 250 \
    --source-address-prefixes $VALIDATOR_IPS \
    --destination-port-ranges 30334 \
    --protocol Tcp \
    --access Allow \
    --description "DeTr P2P protocol - Validators only"
fi

# Rule 4: RPC WebSocket - Port 9944 (Admin access only - Optional)
echo "📌 Rule 4: RPC (port 9944) - Admin access only"
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $VALIDATOR_NSG \
  --name AllowRPC \
  --priority 300 \
  --source-address-prefixes $YOUR_ADMIN_IP \
  --destination-port-ranges 9944 \
  --protocol Tcp \
  --access Allow \
  --description "RPC WebSocket for monitoring"

# Rule 5: Prometheus Metrics - Port 9615 (Monitoring server only)
echo "📌 Rule 5: Prometheus (port 9615) - Monitoring server only"
az network nsg rule create \
  --resource-group $RESOURCE_GROUP \
  --nsg-name $VALIDATOR_NSG \
  --name AllowPrometheusMetrics \
  --priority 400 \
  --source-address-prefixes $MONITORING_SERVER_IP \
  --destination-port-ranges 9615 \
  --protocol Tcp \
  --access Allow \
  --description "Prometheus metrics scraping"

echo ""
echo "✅ NSG configuration complete for $VALIDATOR_NSG"
echo ""
echo "📊 Summary of Rules:"
echo "  Port 22    (SSH)        → Your IP only"
echo "  Port 30333 (P2P)        → Public"
echo "  Port 30334 (DeTr P2P)   → $DETR_P2P_MODE"
echo "  Port 9944  (RPC)        → Your IP only"
echo "  Port 9615  (Prometheus) → Monitoring server only"
echo ""
echo "🔒 Security Status: Configured"
