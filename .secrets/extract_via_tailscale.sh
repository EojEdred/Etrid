#!/bin/bash

echo "═══════════════════════════════════════════════════════════"
echo "Extracting Session Keys via Tailscale Network"
echo "═══════════════════════════════════════════════════════════"
echo ""

# Validators with Tailscale IPs
VALIDATORS=(
    "100.93.43.18:director_0_eoj"
    "100.96.84.69:director_1_gizzi"
    "100.71.127.127:director_2_audit_dev"
    "100.68.185.50:director_3_security_dev"
    "100.70.73.10:director_4_governance_dev"
    "100.88.104.58:director_5_oracle_dev"
    "100.117.43.53:director_6_consensus_dev"
    "100.109.252.56:director_7_economics_dev"
    "100.80.84.82:director_8_compiler_dev"
    "100.125.147.88:validator_15"
    "100.86.111.37:validator_16"
    "100.95.0.72:validator_17"
    "100.113.226.111:validator_18"
    "100.114.244.62:validator_20"
)

OUTPUT_FILE="session_keys_from_tailscale.json"

echo "{" > $OUTPUT_FILE
echo "  \"extracted_date\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"," >> $OUTPUT_FILE
echo "  \"method\": \"Tailscale RPC\"," >> $OUTPUT_FILE
echo "  \"validators\": {" >> $OUTPUT_FILE

FIRST=true
SUCCESS_COUNT=0
FAIL_COUNT=0

for ENTRY in "${VALIDATORS[@]}"; do
    IP="${ENTRY%%:*}"
    NAME="${ENTRY##*:}"

    echo "🔍 Querying $NAME at $IP..."

    # Query author_rotateKeys
    RESULT=$(curl -s http://$IP:9944 \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"author_rotateKeys","params":[],"id":1}' \
        --max-time 5 2>/dev/null)

    if [ ! -z "$RESULT" ]; then
        KEY=$(echo "$RESULT" | jq -r '.result // empty' 2>/dev/null)

        if [ ! -z "$KEY" ] && [ "$KEY" != "null" ] && [ "$KEY" != "0x" ]; then
            if [ "$FIRST" = false ]; then
                echo "," >> $OUTPUT_FILE
            fi
            FIRST=false

            echo "    \"$NAME\": {" >> $OUTPUT_FILE
            echo "      \"tailscale_ip\": \"$IP\"," >> $OUTPUT_FILE
            echo "      \"session_key\": \"$KEY\"," >> $OUTPUT_FILE
            echo "      \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"" >> $OUTPUT_FILE
            echo -n "    }" >> $OUTPUT_FILE

            echo "  ✅ Extracted: ${KEY:0:20}..."
            ((SUCCESS_COUNT++))
        else
            echo "  ⚠️  No valid key returned"
            ((FAIL_COUNT++))
        fi
    else
        echo "  ❌ Connection timeout"
        ((FAIL_COUNT++))
    fi
    echo ""
done

echo "" >> $OUTPUT_FILE
echo "  }," >> $OUTPUT_FILE
echo "  \"summary\": {" >> $OUTPUT_FILE
echo "    \"total\": ${#VALIDATORS[@]}," >> $OUTPUT_FILE
echo "    \"success\": $SUCCESS_COUNT," >> $OUTPUT_FILE
echo "    \"failed\": $FAIL_COUNT" >> $OUTPUT_FILE
echo "  }" >> $OUTPUT_FILE
echo "}" >> $OUTPUT_FILE

echo "═══════════════════════════════════════════════════════════"
echo "Extraction Complete!"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "📊 Results:"
echo "   ✅ Success: $SUCCESS_COUNT / ${#VALIDATORS[@]}"
echo "   ❌ Failed: $FAIL_COUNT / ${#VALIDATORS[@]}"
echo ""
echo "💾 Saved to: $OUTPUT_FILE"
echo ""

if [ $SUCCESS_COUNT -gt 0 ]; then
    echo "Session keys extracted:"
    cat $OUTPUT_FILE | jq '.validators' 2>/dev/null || cat $OUTPUT_FILE
fi
