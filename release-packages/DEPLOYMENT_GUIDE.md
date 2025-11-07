# FlareChain Validator Deployment Guide

Complete guide for deploying FlareChain binaries to your 21 validator VMs.

## Quick Reference

- **Your VM**: 98.71.91.84 (Linux x86_64 - Ubuntu 22.04)
- **Binary to use**: `linux-x86_64/flarechain-node`
- **Total validators**: 21 (Gizzi + EojEdred + 19 others)

## Pre-Deployment Checklist

- [ ] Linux binary compiled (`linux-x86_64/flarechain-node`)
- [ ] macOS binary backed up (`macos-arm64/flarechain-node`)
- [ ] Session keys loaded (`source ../secrets/.env.mainnet`)
- [ ] VM access verified (SSH to all 21 VMs)
- [ ] Firewall rules configured (ports 9944, 30333)

## Step 1: Verify Binary

```bash
cd /Users/macbook/Desktop/etrid/release-packages

# Check Linux binary exists and is correct architecture
ls -lh linux-x86_64/flarechain-node
file linux-x86_64/flarechain-node
# Expected: ELF 64-bit LSB executable, x86-64

# Check macOS binary (for local testing)
ls -lh macos-arm64/flarechain-node
file macos-arm64/flarechain-node
# Expected: Mach-O 64-bit executable arm64
```

## Step 2: Load Environment Variables

```bash
# Load all session keys and configuration
cd /Users/macbook/Desktop/etrid
source secrets/.env.mainnet

# Verify keys are loaded
echo "Gizzi Session Seed: ${GIZZI_SESSION_SEED:0:20}..."
echo "Eoj Session Seed: ${EOJ_SESSION_SEED:0:20}..."
echo "Binary location: $FLARECHAIN_BINARY"
```

## Step 3: Transfer Binary to First VM

**Test on your VM first (98.71.91.84):**

```bash
# Transfer Linux binary
scp release-packages/linux-x86_64/flarechain-node ubuntu@98.71.91.84:~/

# SSH to VM and verify
ssh ubuntu@98.71.91.84 << 'EOF'
  # Make executable
  chmod +x ~/flarechain-node

  # Verify architecture
  file ~/flarechain-node

  # Check version
  ./flarechain-node --version

  # Test it runs
  ./flarechain-node --help | head -20
EOF
```

## Step 4: Deploy to All 21 VMs

**If you have a list of validator IPs:**

```bash
# Create validator list file
cat > /tmp/validator-ips.txt << 'EOF'
98.71.91.84  # Validator 01 (Gizzi or Eoj)
# Add your other 20 validator IPs here
# Format: IP_ADDRESS  # Comment
EOF

# Deploy to all validators
while read ip comment; do
  [[ $ip =~ ^# ]] && continue  # Skip comment lines
  echo "=== Deploying to $ip $comment ==="

  # Transfer binary
  scp release-packages/linux-x86_64/flarechain-node ubuntu@$ip:~/

  # Make executable and verify
  ssh ubuntu@$ip "chmod +x ~/flarechain-node && ./flarechain-node --version"
done < /tmp/validator-ips.txt
```

## Step 5: Insert Session Keys on Each VM

**For Validator 1 (Gizzi):**

```bash
ssh ubuntu@98.71.91.84 << EOF
  # Start node in background
  nohup ./flarechain-node \\
    --validator \\
    --chain flarechain \\
    --name "Gizzi-AI-Overseer" \\
    --rpc-port 9944 \\
    --port 30333 \\
    --rpc-cors all \\
    --rpc-external \\
    > node.log 2>&1 &

  # Wait for node to start
  sleep 10

  # Insert AURA key
  curl -H "Content-Type: application/json" \\
    -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["aura","${GIZZI_SESSION_SEED}","${GIZZI_AURA_KEY}"]}' \\
    http://localhost:9944

  # Insert GRANDPA key
  curl -H "Content-Type: application/json" \\
    -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran","${GIZZI_SESSION_SEED}","${GIZZI_GRANDPA_KEY}"]}' \\
    http://localhost:9944

  # Insert ASF key
  curl -H "Content-Type: application/json" \\
    -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["asf!","${GIZZI_SESSION_SEED}","${GIZZI_ASF_KEY}"]}' \\
    http://localhost:9944

  # Verify keys inserted
  curl -H "Content-Type: application/json" \\
    -d '{"id":1, "jsonrpc":"2.0", "method":"author_hasSessionKeys", "params":["${GIZZI_SESSION_SEED}"]}' \\
    http://localhost:9944
EOF
```

**For Validator 2 (EojEdred):**

```bash
ssh ubuntu@VALIDATOR_02_IP << EOF
  # Start node
  nohup ./flarechain-node \\
    --validator \\
    --chain flarechain \\
    --name "EojEdred-Founder" \\
    --rpc-port 9944 \\
    --port 30333 \\
    --bootnodes /ip4/98.71.91.84/tcp/30333/p2p/GIZZI_PEER_ID \\
    > node.log 2>&1 &

  sleep 10

  # Insert keys (replace with EOJ keys)
  curl -H "Content-Type: application/json" \\
    -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["aura","${EOJ_SESSION_SEED}","${EOJ_AURA_KEY}"]}' \\
    http://localhost:9944

  curl -H "Content-Type: application/json" \\
    -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["gran","${EOJ_SESSION_SEED}","${EOJ_GRANDPA_KEY}"]}' \\
    http://localhost:9944

  curl -H "Content-Type: application/json" \\
    -d '{"id":1, "jsonrpc":"2.0", "method":"author_insertKey", "params":["asf!","${EOJ_SESSION_SEED}","${EOJ_ASF_KEY}"]}' \\
    http://localhost:9944
EOF
```

## Step 6: Monitor Validators

```bash
# Check node status on each VM
ssh ubuntu@98.71.91.84 "tail -f ~/node.log"

# Check if node is syncing
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
  http://98.71.91.84:9944

# Check connected peers
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
  http://98.71.91.84:9944

# Check block production
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"chain_getHeader"}' \
  http://98.71.91.84:9944
```

## Step 7: Start Remaining Validators (3-21)

Once Gizzi and EojEdred are running and producing blocks:

```bash
# For validators 3-21, use this template
for i in {03..21}; do
  echo "=== Starting Validator $i ==="
  ssh ubuntu@VALIDATOR_${i}_IP << 'EOF'
    nohup ./flarechain-node \
      --validator \
      --chain flarechain \
      --name "Validator-${i}" \
      --rpc-port 9944 \
      --port 30333 \
      --bootnodes /ip4/98.71.91.84/tcp/30333/p2p/GIZZI_PEER_ID \
      --bootnodes /ip4/VALIDATOR_02_IP/tcp/30333/p2p/EOJ_PEER_ID \
      > node.log 2>&1 &
EOF
done
```

## Troubleshooting

### Binary doesn't execute on Linux VM

```bash
# Check architecture
file flarechain-node
# Must be: ELF 64-bit LSB executable, x86-64

# Check if you accidentally transferred macOS binary
ssh ubuntu@98.71.91.84 "file ~/flarechain-node"
```

### "Permission denied"

```bash
ssh ubuntu@98.71.91.84 "chmod +x ~/flarechain-node"
```

### Node won't start

```bash
# Check logs
ssh ubuntu@98.71.91.84 "tail -100 ~/node.log"

# Check if port is already in use
ssh ubuntu@98.71.91.84 "netstat -tulpn | grep 9944"

# Kill existing process
ssh ubuntu@98.71.91.84 "pkill flarechain-node"
```

### Can't insert session keys

```bash
# Check if RPC is listening
ssh ubuntu@98.71.91.84 "netstat -tulpn | grep 9944"

# Check if node is running
ssh ubuntu@98.71.91.84 "ps aux | grep flarechain-node"

# Try local curl first
ssh ubuntu@98.71.91.84 << 'EOF'
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_health"}' \
  http://localhost:9944
EOF
```

### Nodes not connecting to each other

```bash
# Check firewall allows port 30333
ssh ubuntu@98.71.91.84 "sudo ufw status"

# Open ports if needed
ssh ubuntu@98.71.91.84 << 'EOF'
  sudo ufw allow 9944/tcp   # RPC
  sudo ufw allow 30333/tcp  # P2P
  sudo ufw reload
EOF

# Get peer ID from running node
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_localPeerId"}' \
  http://98.71.91.84:9944
```

## Systemd Service (Optional)

Create a systemd service for automatic restarts:

```bash
ssh ubuntu@98.71.91.84 << 'EOF'
sudo tee /etc/systemd/system/flarechain.service > /dev/null << 'SERVICE'
[Unit]
Description=FlareChain Validator Node
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu
ExecStart=/home/ubuntu/flarechain-node \
  --validator \
  --chain flarechain \
  --name "Gizzi-AI-Overseer" \
  --rpc-port 9944 \
  --port 30333 \
  --rpc-cors all
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
SERVICE

sudo systemctl daemon-reload
sudo systemctl enable flarechain
sudo systemctl start flarechain
sudo systemctl status flarechain
EOF
```

## Post-Deployment Verification

```bash
# Check all validators are running
for i in {01..21}; do
  echo "=== Validator $i ==="
  ssh ubuntu@VALIDATOR_${i}_IP "./flarechain-node --version"
done

# Check all validators are connected
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
  http://98.71.91.84:9944 | jq '.result | length'
# Should show ~20 (connected to other validators)

# Check block production
for i in {1..10}; do
  echo "Block $i:"
  curl -s -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method":"chain_getHeader"}' \
    http://98.71.91.84:9944 | jq '.result.number'
  sleep 6  # ASF produces blocks every 6 seconds
done
```

## Network is LIVE when:

- ✅ All 21 validators are running
- ✅ All validators have inserted session keys
- ✅ Validators are connected to each other (20 peers each)
- ✅ New blocks are being produced every 6 seconds
- ✅ Block finality is progressing (GRANDPA)

## Next Steps After Mainnet Launch

1. Deploy PBC contracts to Base, Arbitrum, BSC, Polygon
2. Enable cross-chain bridges
3. Deploy governance UI
4. Open validator registration for public
5. List ETR on exchanges

---

**Need Help?**
- Check `secrets/.env.mainnet` for all configuration values
- Review `../CLEANUP_SUMMARY.md` for key locations
- See `README.md` for binary details

🔥 **LET'S LAUNCH!** 🔥
