#!/bin/bash
# Insert validator keys via RPC

SSH_KEY="$HOME/.ssh/contabo-validators"

# The 21 validator IDs - these need to be full 64-char hex keys
# For now, let's use the IDs as seeds and generate proper sr25519 keys
VALIDATOR_IDS=(
  "00400f479b47d741752f1d01344ef714"
  "2627aa12b4ab2d8d6e82c259b186efb3"
  "2c339e81f2a1fc80ae67c3bda3ecade0"
  "325362702873fdeaf94eb07f8f2a9659"
  "36edba289334c21d8c78b81d56dba974"
  "384a80f6b1c16fd5f8df53458f9f6ec5"
  "44f5ed22b0372d4822bcd0c3a0cad74a"
  "4a2320a52c89db6e72fa445bf1f774a2"
  "58716581b09066395ef75cead565526f"
  "7cefa78d24e90d0e0823afce3cbb57f0"
  "7eb293d7da884a6d359abce6756326b6"
  "9e270842ee6d0cc5d4634760717fb2fe"
  "9e33e587928d3f3fd9bed5d407350889"
  "b89f96a7d5dcff24aec4fee55507d243"
  "be9fdd4416eff9375461618f1e2bd244"
  "d27ae8bc2d7b32cfd6e1a301a4d9931e"
  "dc2e6eabc3d02e01f26bbf2bf8810c56"
  "ecec5b1247d1276260758b159add80c7"
  "f29e4e1cfc2867fcda12ac9b190bea01"
  "f44ee1c6da7cf209998874f2fa612e75"
  "fe14bf4fd7b9cb683697114b9b60dc5a"
)

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

echo "Inserting validator keys via RPC..."
echo ""

for i in "${!VMS[@]}"; do
    VM="${VMS[$i]}"
    VALIDATOR_ID="${VALIDATOR_IDS[$i]}"

    # Pad to full 64 hex chars (32 bytes)
    FULL_KEY="0x${VALIDATOR_ID}168e6f0646beffd77d69d39bad76b47a"

    echo "VM $((i+1)): $VM -> Inserting key for validator $VALIDATOR_ID"

    # Insert key via RPC using author_insertKey
    # Key type "asf " for ASF consensus
    ssh -i "$SSH_KEY" -o StrictHostKeyChecking=no root@$VM "curl -H 'Content-Type: application/json' -d '{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_insertKey\", \"params\":[\"asf \",\"//ValidatorNode${i}\",\"${FULL_KEY}\"]}' http://localhost:9944 2>/dev/null" > /dev/null

    echo "✅ $VM key inserted"
done

echo ""
echo "All keys inserted via RPC!"
