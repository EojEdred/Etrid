#!/bin/bash
# Insert correct validator keys from the original secrets

SSH_KEY="$HOME/.ssh/contabo-validators"
KEYS_FILE="/Users/macbook/Desktop/etrid/secrets/mainnet/validator-keys-complete.json"

VMS=(
  vmi2896906
  vmi2896907
  vmi2896908
  vmi2896909
  vmi2896910
  vmi2896911
  vmi2896914
  vmi2896915
  vmi2896916
  vmi2896917
  vmi2896918
  vmi2896921
  vmi2896922
  vmi2896923
  vmi2896924
  vmi2896925
  vmi2897381
  vmi2897382
  vmi2897383
  vmi2897384
)

echo "Inserting correct validator keys from secrets..."
echo ""

SUCCESS=0
FAILED=0

# Extract all validator data (20 VMs = validators 1-20)
for i in "${!VMS[@]}"; do
    VM="${VMS[$i]}"
    VALIDATOR_INDEX=$((i + 1))

    # Extract session seed and ASF key from JSON
    SESSION_SEED=$(jq -r ".validators[$i].sessionKeys.seed" "$KEYS_FILE")
    ASF_KEY=$(jq -r ".validators[$i].sessionKeys.asfKey" "$KEYS_FILE")

    if [ "$SESSION_SEED" = "null" ] || [ "$ASF_KEY" = "null" ]; then
        echo "⚠️  VM $((i+1)): $VM -> No key data found for validator $VALIDATOR_INDEX"
        FAILED=$((FAILED + 1))
        continue
    fi

    echo "VM $((i+1)): $VM -> Validator $VALIDATOR_INDEX (ASF: ${ASF_KEY:0:18}...)"

    # Insert key via RPC using the actual session seed
    RESPONSE=$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no root@$VM 'curl -s -H "Content-Type: application/json" -d '"'"'{"id":1, "jsonrpc":"2.0", "method": "author_insertKey", "params":["asf ","'"$SESSION_SEED"'","'"$ASF_KEY"'"]}'"'"' http://localhost:9944' 2>/dev/null)

    # Check if successful
    if echo "$RESPONSE" | grep -q '"result":null'; then
        echo "✅ $VM key inserted successfully"
        SUCCESS=$((SUCCESS + 1))
    else
        echo "⚠️  $VM response: $RESPONSE"
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo "Summary: $SUCCESS successful, $FAILED failed"
echo ""
echo "Waiting 5 seconds for keys to be recognized..."
sleep 5
