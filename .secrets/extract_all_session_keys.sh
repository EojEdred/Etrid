#!/bin/bash

# Ëtrid - Extract All Validator Session Keys from Running Chain

set -e

echo "═══════════════════════════════════════════════════════════"
echo "Extracting Session Keys from All Validators"
echo "═══════════════════════════════════════════════════════════"
echo ""

cd ~/Desktop/etrid/.secrets

# Create output file
OUTPUT_FILE="session_keys_extracted_$(date +%Y%m%d_%H%M%S).json"
echo "{" > $OUTPUT_FILE
echo "  \"extracted_date\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"," >> $OUTPUT_FILE
echo "  \"validators\": {" >> $OUTPUT_FILE

# All validator IPs from updated secrets
VALIDATORS=(
  "80.190.82.186:validator_9"
  "85.239.239.194:validator_10"
  "85.239.239.193:validator_11"
  "85.239.239.189:validator_12"
  "85.239.239.188:validator_13"
  "154.12.250.18:validator_14"
  "129.80.122.34:validator_19"
  "157.173.200.86:director_0_eoj"
  "80.190.82.185:director_2_audit_dev"
  "80.190.82.184:director_3_security_dev"
  "80.190.82.183:director_4_governance_dev"
  "154.12.250.17:director_5_oracle_dev"
  "154.12.250.15:director_6_consensus_dev"
  "157.173.200.84:director_7_economics_dev"
  "157.173.200.81:director_8_compiler_dev"
  "157.173.200.80:validator_15"
  "158.220.83.146:validator_16"
  "154.12.249.223:validator_17"
  "154.12.249.182:validator_18"
  "85.239.239.190:validator_20"
)

FIRST=true

for ENTRY in "${VALIDATORS[@]}"; do
  IP="${ENTRY%%:*}"
  NAME="${ENTRY##*:}"

  echo "Querying $NAME at $IP..."

  # Try to get session keys via RPC
  RESULT=$(curl -s -X POST http://$IP:9944 \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"author_rotateKeys","params":[],"id":1}' \
    --max-time 5 2>/dev/null || echo "")

  if [ ! -z "$RESULT" ]; then
    # Extract the hex key from result
    KEY=$(echo $RESULT | grep -o '"result":"0x[^"]*"' | cut -d'"' -f4)

    if [ ! -z "$KEY" ] && [ "$KEY" != "0x" ]; then
      if [ "$FIRST" = false ]; then
        echo "," >> $OUTPUT_FILE
      fi
      FIRST=false

      echo "    \"$NAME\": {" >> $OUTPUT_FILE
      echo "      \"ip\": \"$IP\"," >> $OUTPUT_FILE
      echo "      \"session_key\": \"$KEY\"," >> $OUTPUT_FILE
      echo "      \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"" >> $OUTPUT_FILE
      echo -n "    }" >> $OUTPUT_FILE

      echo "  ✅ Extracted: ${KEY:0:20}..."
    else
      echo "  ⚠️  No session key returned (RPC may not be exposed)"
    fi
  else
    echo "  ❌ Connection failed (firewall or node offline)"
  fi
  echo ""
done

echo "" >> $OUTPUT_FILE
echo "  }" >> $OUTPUT_FILE
echo "}" >> $OUTPUT_FILE

echo "✅ Session keys saved to $OUTPUT_FILE"
echo ""
echo "To update VALIDATORS_MAINNET_REAL.json, review the extracted keys"
echo "and manually update the 'session_keys.asfk' field for each validator."
echo ""
