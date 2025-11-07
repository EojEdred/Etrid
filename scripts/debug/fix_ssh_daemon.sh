#!/bin/bash

# Fix SSH Daemon on Azure VMs
# This script starts and enables SSH daemon on VMs where it's not running

set -e

echo "========================================="
echo "SSH Daemon Fix Script"
echo "========================================="
echo ""

# List of VMs to fix (update this based on diagnostic results)
declare -a VMS=(
  "etrid-compiler-dev-secondary:ETRID-VALIDATORS-EU-NORTH"
  "etrid-multichain-dev-primary:ETRID-VALIDATORS-EU-NORTH"
  "etrid-compiler-dev-primary:ETRID-VALIDATORS-EU-WEST"
  "etrid-consensus-dev-secondary:ETRID-VALIDATORS-EU-WEST"
  "etrid-multichain-dev-secondary:ETRID-VALIDATORS-EU-WEST"
  "etrid-runtime-dev-primary:ETRID-VALIDATORS-EU-WEST"
  "etrid-runtime-dev-secondary:ETRID-VALIDATORS-EU-WEST"
  "etrid-flarenode-15:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-flarenode-16:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-flarenode-17:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-oracle-dev:ETRID-VALIDATORS-UK-SOUTH"
  "etrid-flarenode-18:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-19:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-20:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-21:ETRID-VALIDATORS-EU-FR"
  "etrid-flarenode-22:ETRID-VALIDATORS-EU-FR"
)

# Function to fix SSH daemon
fix_ssh_daemon() {
  local VM_NAME=$1
  local RG=$2

  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Fixing SSH on: $VM_NAME ($RG)"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

  RESULT=$(az vm run-command invoke \
    -g "$RG" \
    -n "$VM_NAME" \
    --command-id RunShellScript \
    --scripts "
      echo '=== Starting SSH Daemon ==='

      # Try different SSH service names
      systemctl start sshd 2>&1 || systemctl start ssh 2>&1 || service ssh start 2>&1

      echo '=== Enabling SSH Daemon on Boot ==='
      systemctl enable sshd 2>&1 || systemctl enable ssh 2>&1 || chkconfig sshd on 2>&1

      echo ''
      echo '=== SSH Daemon Status ==='
      systemctl status sshd 2>&1 || systemctl status ssh 2>&1 || service ssh status 2>&1

      echo ''
      echo '=== Verifying Port 22 is Listening ==='
      netstat -tuln | grep ':22' || ss -tuln | grep ':22'
    " \
    --query "value[0].message" \
    -o tsv 2>&1)

  echo "$RESULT"

  if echo "$RESULT" | grep -q "Active: active (running)"; then
    echo "✅ SSH daemon fixed successfully!"
  else
    echo "⚠️  SSH daemon may still have issues. Review output above."
  fi

  echo ""
  echo ""
}

# Confirmation prompt
echo "This script will attempt to start and enable SSH daemon on the following VMs:"
echo ""
for vm_info in "${VMS[@]}"; do
  IFS=':' read -r VM_NAME RG <<< "$vm_info"
  echo "  - $VM_NAME ($RG)"
done
echo ""
read -p "Continue? (y/n): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
  echo "Aborted."
  exit 0
fi

echo ""
echo "Starting SSH daemon fix process..."
echo ""

for vm_info in "${VMS[@]}"; do
  IFS=':' read -r VM_NAME RG <<< "$vm_info"
  fix_ssh_daemon "$VM_NAME" "$RG"

  # Small delay to avoid throttling
  sleep 2
done

echo "========================================="
echo "SSH Daemon Fix Complete"
echo "========================================="
echo ""
echo "Test SSH connections with:"
echo "ssh azureuser@<PUBLIC_IP>"
