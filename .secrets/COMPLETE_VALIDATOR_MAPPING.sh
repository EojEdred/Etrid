#!/bin/bash

# Ëtrid Primearc Core - Validator IP & Key Mapping Script
# Completes TBD fields in VALIDATORS_MAINNET_REAL.json

set -e

echo "═══════════════════════════════════════════════════════════"
echo "Ëtrid Validator Mapping - Extracting Real Chain Data"
echo "═══════════════════════════════════════════════════════════"
echo ""

cd ~/Desktop/etrid/.secrets

# Known validator IPs from existing data
KNOWN_IPS=(
  "64.181.215.19"    # Gizzi (Director 1)
  "80.190.82.186"    # Validator 9
  "85.239.239.194"   # Validator 10
  "85.239.239.193"   # Validator 11
  "85.239.239.189"   # Validator 12
  "85.239.239.188"   # Validator 13
  "154.12.250.18"    # Validator 14
  "129.80.122.34"    # Validator 19
)

echo "Step 1: Extracting session keys from reachable validators..."
echo "─────────────────────────────────────────────────────────"
echo ""

SESSION_KEYS_FILE="validator_session_keys_extracted.json"
echo "{" > $SESSION_KEYS_FILE
echo "  \"extracted_date\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"," >> $SESSION_KEYS_FILE
echo "  \"validators\": {" >> $SESSION_KEYS_FILE

FIRST=true
for IP in "${KNOWN_IPS[@]}"; do
  echo "Querying $IP:9944..."

  # Try to get session keys via RPC (author_rotateKeys)
  RESULT=$(curl -s -X POST http://$IP:9944 \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"author_rotateKeys","params":[],"id":1}' \
    --max-time 5 2>/dev/null || echo "")

  if [ ! -z "$RESULT" ]; then
    # Extract the hex key from result
    KEY=$(echo $RESULT | grep -o '"result":"0x[^"]*"' | cut -d'"' -f4)

    if [ ! -z "$KEY" ] && [ "$KEY" != "0x" ]; then
      if [ "$FIRST" = false ]; then
        echo "," >> $SESSION_KEYS_FILE
      fi
      FIRST=false

      echo "    \"$IP\": {" >> $SESSION_KEYS_FILE
      echo "      \"session_key\": \"$KEY\"," >> $SESSION_KEYS_FILE
      echo "      \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"" >> $SESSION_KEYS_FILE
      echo -n "    }" >> $SESSION_KEYS_FILE

      echo "  ✅ Extracted: ${KEY:0:20}..."
    else
      echo "  ⚠️  No session key returned"
    fi
  else
    echo "  ❌ Connection failed"
  fi
  echo ""
done

echo "" >> $SESSION_KEYS_FILE
echo "  }" >> $SESSION_KEYS_FILE
echo "}" >> $SESSION_KEYS_FILE

echo "✅ Session keys saved to $SESSION_KEYS_FILE"
echo ""

echo "Step 2: Mapping Tailscale VMs to validators..."
echo "─────────────────────────────────────────────────────────"
echo ""

# Get Tailscale status
tailscale status > tailscale_vms.txt 2>/dev/null || echo "⚠️  Tailscale not available"

if [ -f tailscale_vms.txt ]; then
  echo "Tailscale VMs found:"
  grep -E "vmi|oracle-vm|gizzi|eoj" tailscale_vms.txt || echo "No matching VMs"
  echo ""

  # Extract Gizzi's Tailscale IP
  GIZZI_TAILSCALE=$(grep "gizzi-io-validator" tailscale_vms.txt | awk '{print $1}')
  if [ ! -z "$GIZZI_TAILSCALE" ]; then
    echo "✅ Gizzi Tailscale IP: $GIZZI_TAILSCALE"
  fi
  echo ""
fi

echo "Step 3: Mapping AI Dev VMs..."
echo "─────────────────────────────────────────────────────────"
echo ""

# Try to SSH to each oracle-vm to find which vmi they're on
AI_DEVS=(
  "audit"
  "security"
  "governance"
  "oracle"
  "consensus"
  "economics"
  "compiler"
)

echo "Attempting to resolve oracle-vm hostnames to vmi IDs..."
for DEV in "${AI_DEVS[@]}"; do
  HOSTNAME="oracle-vm-$DEV"

  # Try to resolve via Tailscale or SSH
  VMI=$(tailscale status 2>/dev/null | grep "$HOSTNAME" | awk '{print $2}')

  if [ ! -z "$VMI" ]; then
    echo "  ✅ $HOSTNAME → $VMI"
  else
    # Try alternative: check if hostname resolves
    IP=$(getent hosts $HOSTNAME 2>/dev/null | awk '{print $1}')
    if [ ! -z "$IP" ]; then
      echo "  ✅ $HOSTNAME → $IP"
    else
      echo "  ⚠️  $HOSTNAME → Not found (TBD)"
    fi
  fi
done
echo ""

echo "Step 4: Checking deployment logs for missing IPs..."
echo "─────────────────────────────────────────────────────────"
echo ""

# Check if deployment logs exist
if [ -d "../deployment-logs" ]; then
  echo "Searching deployment logs for validator IPs..."
  grep -r "Validator.*IP\|validator.*deployed" ../deployment-logs/ 2>/dev/null | head -20 || echo "No deployment logs found"
else
  echo "⚠️  No deployment-logs directory found"
fi
echo ""

echo "═══════════════════════════════════════════════════════════"
echo "Mapping Complete!"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "Generated files:"
echo "  - $SESSION_KEYS_FILE"
echo "  - tailscale_vms.txt (if available)"
echo ""
echo "Next steps:"
echo "  1. Review the extracted session keys"
echo "  2. Manually update VALIDATORS_MAINNET_REAL.json with:"
echo "     - Eoj's IP address and Tailscale hostname"
echo "     - AI Dev to vmi mappings"
echo "     - Missing public IPs for validators 15-18, 20"
echo "  3. Cross-reference with Contabo dashboard for missing IPs"
echo ""
echo "To update the JSON, use a Python script or manually edit:"
echo "  nano VALIDATORS_MAINNET_REAL.json"
echo ""
