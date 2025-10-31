#!/bin/bash
# Configure Bootnodes for Mainnet
# Extracts bootnode peer IDs and creates public bootnode list

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "╔════════════════════════════════════════════════════════════╗"
echo "║           Configure Mainnet Bootnodes                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Which validators should be bootnodes?
# Typically: 2-3 well-connected, reliable validators
echo "Recommended bootnodes:"
echo "  1. Validator 1 (Gizzi - AI Overseer)"
echo "  2. Validator 2 (EojEdred - Founder)"
echo "  3. Validator 3 (backup bootnode)"
echo ""

# Extract network keys for bootnodes
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Extracting Bootnode Peer IDs${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if validator keys exist
VALIDATOR_KEYS="validator-keys-setup/generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json"

if [ ! -f "$VALIDATOR_KEYS" ]; then
    echo -e "${RED}❌ Error: validator-keys-complete.json not found${NC}"
    exit 1
fi

# Extract network keys for first 3 validators
BOOTNODE_INFO=()

for i in 1 2 3; do
    NET_KEY=$(jq -r ".validators[$((i-1))].networkKeys.secretKey" "$VALIDATOR_KEYS")

    if [ -z "$NET_KEY" ] || [ "$NET_KEY" = "null" ]; then
        echo -e "${RED}❌ Error: Network key not found for Validator $i${NC}"
        continue
    fi

    # Generate peer ID from network key
    # This requires the flarechain-node binary
    if [ -f "target/release/flarechain-node" ]; then
        PEER_ID=$(echo "$NET_KEY" | ./target/release/flarechain-node key inspect-node-key --network libp2p-ed25519 2>/dev/null || echo "UNKNOWN")

        if [ "$PEER_ID" != "UNKNOWN" ]; then
            VALIDATOR_NAME=$(jq -r ".validators[$((i-1))].name" "$VALIDATOR_KEYS")
            echo -e "${GREEN}Validator $i ($VALIDATOR_NAME):${NC}"
            echo "  Peer ID: $PEER_ID"
            BOOTNODE_INFO+=("$i|$VALIDATOR_NAME|$PEER_ID")
        fi
    else
        echo -e "${YELLOW}⚠️  Binary not found, using network key directly${NC}"
        echo "  Network Key: $NET_KEY"
    fi
    echo ""
done

# Now we need the public IP addresses
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Enter Public IP Addresses for Bootnodes${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

BOOTNODE_ADDRS=()

for entry in "${BOOTNODE_INFO[@]}"; do
    IFS='|' read -r num name peer_id <<< "$entry"

    read -p "Public IP for Validator $num ($name): " ip_addr

    if [ -n "$ip_addr" ] && [ -n "$peer_id" ]; then
        multiaddr="/ip4/$ip_addr/tcp/30333/p2p/$peer_id"
        BOOTNODE_ADDRS+=("$multiaddr")
        echo -e "${GREEN}✅ Added: $multiaddr${NC}"
    fi
    echo ""
done

# Create public bootnodes.txt
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Creating Public Bootnode Configuration${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

cat > BOOTNODES.md << 'EOF'
# Ëtrid FlareChain Mainnet Bootnodes

**Network:** FlareChain Mainnet
**Chain ID:** flarechain_mainnet
**P2P Port:** 30333

---

## Official Bootnodes

Connect to these bootnodes to join the Ëtrid FlareChain mainnet:

EOF

for i in "${!BOOTNODE_ADDRS[@]}"; do
    echo "\`\`\`" >> BOOTNODES.md
    echo "${BOOTNODE_ADDRS[$i]}" >> BOOTNODES.md
    echo "\`\`\`" >> BOOTNODES.md
    echo "" >> BOOTNODES.md
done

cat >> BOOTNODES.md << 'EOF'

---

## How to Use

### Starting a Node with Bootnodes

```bash
./flarechain-node \
  --chain flarechain-mainnet-raw.json \
  --base-path /var/lib/etrid \
  --name "My Node" \
  --bootnodes BOOTNODE_MULTIADDR_HERE
```

### Multiple Bootnodes

```bash
./flarechain-node \
  --chain flarechain-mainnet-raw.json \
  --base-path /var/lib/etrid \
  --name "My Node" \
  --bootnodes BOOTNODE_1 \
  --bootnodes BOOTNODE_2 \
  --bootnodes BOOTNODE_3
```

---

## Adding Bootnodes to Chain Spec

Bootnodes are embedded in `flarechain-mainnet-raw.json` chain spec.

To add more bootnodes manually, edit the chain spec before generating the raw version:

```json
{
  "bootNodes": [
    "/ip4/IP_ADDRESS/tcp/30333/p2p/PEER_ID"
  ]
}
```

---

## Telemetry

Mainnet validators report to:
- **Polkadot Telemetry:** wss://telemetry.polkadot.io/submit/

View network stats at: https://telemetry.polkadot.io/

---

## RPC Endpoints

Public RPC endpoints (if available):

- wss://rpc.etrid.io
- https://rpc.etrid.io

**Note:** Run your own node for production applications.

---

**Need Help?**

- Website: https://etrid.io
- Docs: https://docs.etrid.io
- GitHub: https://github.com/EojEdred/Etrid
EOF

echo -e "${GREEN}✅ Created BOOTNODES.md${NC}"
echo ""

# Create machine-readable bootnodes.txt
cat > bootnodes.txt << 'EOF'
# Ëtrid FlareChain Mainnet Bootnodes
# Format: /ip4/IP/tcp/PORT/p2p/PEER_ID
EOF

for addr in "${BOOTNODE_ADDRS[@]}"; do
    echo "$addr" >> bootnodes.txt
done

echo -e "${GREEN}✅ Created bootnodes.txt${NC}"
echo ""

# Create JSON format for APIs
cat > bootnodes.json << 'EOF'
{
  "network": "FlareChain Mainnet",
  "chainId": "flarechain_mainnet",
  "bootnodes": [
EOF

for i in "${!BOOTNODE_ADDRS[@]}"; do
    if [ $i -eq $((${#BOOTNODE_ADDRS[@]} - 1)) ]; then
        echo "    \"${BOOTNODE_ADDRS[$i]}\"" >> bootnodes.json
    else
        echo "    \"${BOOTNODE_ADDRS[$i]}\"," >> bootnodes.json
    fi
done

cat >> bootnodes.json << 'EOF'
  ],
  "rpcEndpoints": [
    "wss://rpc.etrid.io",
    "https://rpc.etrid.io"
  ],
  "telemetry": "wss://telemetry.polkadot.io/submit/"
}
EOF

echo -e "${GREEN}✅ Created bootnodes.json${NC}"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}Bootnode Configuration Complete!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "📄 Files created:"
echo "  - BOOTNODES.md (human-readable documentation)"
echo "  - bootnodes.txt (machine-readable list)"
echo "  - bootnodes.json (API format)"
echo ""

echo "📋 Next steps:"
echo "  1. Add BOOTNODES.md to your website/docs"
echo "  2. Commit bootnodes.txt to git repo"
echo "  3. Update chain spec with bootnode addresses"
echo "  4. Rebuild mainnet binary with bootnodes included"
echo ""

echo "⚠️  Make sure these files are PUBLIC:"
echo "  - Add to README.md"
echo "  - Publish on website"
echo "  - Include in node setup docs"
echo ""
