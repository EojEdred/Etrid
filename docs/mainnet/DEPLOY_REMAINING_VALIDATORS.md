# Deploy Remaining Validators - Reach BFT Threshold

**Goal:** Deploy 7 more validators to reach **15 active validators** for GRANDPA supermajority and full BFT

**Current Status:**
- Active Validators: ~8-9
- Target: 15 validators minimum
- Need: 7 more validators

---

## Current Network Bootnodes

```bash
# Use these bootnodes for all new validator deployments:
BOOTNODES="/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/129.80.122.34/tcp/30333/p2p/12D3KooWGjGCJzexrJct6nGCSDnj7vaJtMohpagFUPBhPgpZqvpd"
```

---

## Priority Validators to Deploy (7 validators)

Deploy these validators in priority order to reach 15 active:

1. **validator-3** (Director) - Account: `5DLWfsK2jUGX5A6SZUqP...`
2. **validator-4** (Director) - Account: `5HRMNRrTr6ahy5TPzC3Y...`
3. **validator-6** (FlareNode) - Account: `5Hb2ySKHArSwzoAY9JHs...`
4. **validator-7** (FlareNode) - Account: `5CvjTcBhW1Vy3GUA5Gwp...`
5. **validator-8** (FlareNode) - Account: `5GEn5LgTjEo6bBevEdL3...`
6. **validator-9** (FlareNode) - Account: `5EtWzCvcDMkjhpjbn51Q...`
7. **validator-10** (FlareNode) - Account: `5GNeSkpUXSJNcoKQ6NPy...`

---

## Deployment Steps for Each Validator

### Step 1: Prepare Server/VM

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Create validator user
sudo useradd -m -s /bin/bash validator
sudo usermod -aG sudo validator

# Create data directory
sudo mkdir -p /var/lib/flarechain
sudo chown validator:validator /var/lib/flarechain
```

### Step 2: Upload Files

```bash
# From your local machine:
scp /Users/macbook/Desktop/etrid/target/release/flarechain-node validator@SERVER_IP:/tmp/
scp /Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json validator@SERVER_IP:/tmp/

# On server:
ssh validator@SERVER_IP
sudo mv /tmp/flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node
sudo mv /tmp/chainspec-mainnet-raw-FIXED.json /var/lib/flarechain/
```

### Step 3: Insert Session Keys

Extract the session keys for the specific validator from `validator-keys-complete.json`, then:

```bash
# AURA Key (sr25519)
/usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/flarechain \
  --chain /var/lib/flarechain/chainspec-mainnet-raw-FIXED.json \
  --key-type aura \
  --scheme sr25519 \
  --suri "VALIDATOR_MNEMONIC_PHRASE"

# GRANDPA Key (ed25519)
/usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/flarechain \
  --chain /var/lib/flarechain/chainspec-mainnet-raw-FIXED.json \
  --key-type gran \
  --scheme ed25519 \
  --suri "VALIDATOR_MNEMONIC_PHRASE"

# ASF Key (sr25519, key-type: asfk)
/usr/local/bin/flarechain-node key insert \
  --base-path /var/lib/flarechain \
  --chain /var/lib/flarechain/chainspec-mainnet-raw-FIXED.json \
  --key-type asfk \
  --scheme sr25519 \
  --suri "VALIDATOR_MNEMONIC_PHRASE"

# Verify keys inserted
ls -la /var/lib/flarechain/chains/flarechain_mainnet/keystore/
# Should see 3 files (aura, gran, asfk)
```

### Step 4: Generate Network Key

```bash
# Create network directory
sudo mkdir -p /var/lib/flarechain/chains/flarechain_mainnet/network

# Generate network key
/usr/local/bin/flarechain-node key generate-node-key \
  --file /var/lib/flarechain/chains/flarechain_mainnet/network/secret_ed25519

# Set permissions
sudo chown -R validator:validator /var/lib/flarechain
```

### Step 5: Create Systemd Service

Replace placeholders:
- `VALIDATOR_NAME` - e.g., "Validator-3"
- `SERVER_PUBLIC_IP` - e.g., "1.2.3.4"

```bash
sudo tee /etc/systemd/system/flarechain-validator.service > /dev/null << 'EOF'
[Unit]
Description=FlareChain Validator Node - VALIDATOR_NAME
Documentation=https://docs.etrid.com
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=validator
Group=validator
WorkingDirectory=/var/lib/flarechain

ExecStart=/usr/local/bin/flarechain-node \
  --chain=/var/lib/flarechain/chainspec-mainnet-raw-FIXED.json \
  --base-path=/var/lib/flarechain \
  --validator \
  --name="VALIDATOR_NAME" \
  --port=30333 \
  --rpc-port=9944 \
  --prometheus-port=9615 \
  --prometheus-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe \
  --rpc-external \
  --unsafe-rpc-external \
  --public-addr=/ip4/SERVER_PUBLIC_IP/tcp/30333 \
  --bootnodes=/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm,/ip4/129.80.122.34/tcp/30333/p2p/12D3KooWGjGCJzexrJct6nGCSDnj7vaJtMohpagFUPBhPgpZqvpd

LimitNOFILE=65536
LimitNPROC=4096

Restart=always
RestartSec=10
StartLimitInterval=600
StartLimitBurst=5

StandardOutput=journal
StandardError=journal
SyslogIdentifier=flarechain-validator

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
sudo systemctl daemon-reload
sudo systemctl enable flarechain-validator
```

### Step 6: Start Validator

```bash
# Start service
sudo systemctl start flarechain-validator

# Check status
sudo systemctl status flarechain-validator

# Watch logs
sudo journalctl -u flarechain-validator -f
```

### Step 7: Verify Connection

```bash
# Check peers (should see 9+ peers after joining)
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944 | jq

# Check sync status
curl -s -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' \
  http://localhost:9944 | jq

# Get Peer ID
sudo journalctl -u flarechain-validator -n 100 | grep "Local node identity"
```

### Step 8: Add to Bootnode List (Optional)

After validator is stable, add its peer info to the bootnode list for future deployments:

```bash
# Format: /ip4/SERVER_IP/tcp/30333/p2p/PEER_ID
```

---

## Firewall Configuration

**Required Ports:**
```bash
# Substrate P2P
sudo ufw allow 30333/tcp

# DETR P2P (ASF Finality)
sudo ufw allow 30334/tcp
sudo ufw allow 30334/udp

# RPC (if external access needed)
sudo ufw allow 9944/tcp

# Prometheus metrics (if monitoring)
sudo ufw allow 9615/tcp

# Enable firewall
sudo ufw enable
```

---

## Monitoring After Each Deployment

After each validator deployment, check network health:

```bash
# On any active validator (e.g., AuditDev):
curl -s http://129.80.122.34:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' | jq

# Expected output after each deployment:
# {"peers": 9, "isSyncing": false, "shouldHavePeers": true}  # After 1st new validator
# {"peers": 10, "isSyncing": false, "shouldHavePeers": true} # After 2nd new validator
# ... etc
```

**Watch for BFT Threshold (15 validators):**
```bash
# On any validator, watch logs for GRANDPA finality messages:
sudo journalctl -u flarechain-validator -f | grep -i "finalized\|grandpa"

# Should see increased finality activity once 15 validators are active
```

---

## Session Keys Reference

Extract from `/Users/macbook/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json`:

**Validator 3:**
```json
"phrase": "... mnemonic phrase for validator 3 ..."
"accountId": "5DLWfsK2jUGX5A6SZUqP..."
```

**Validator 4:**
```json
"phrase": "... mnemonic phrase for validator 4 ..."
"accountId": "5HRMNRrTr6ahy5TPzC3Y..."
```

*(Continue for validators 6-10)*

---

## Expected Timeline

**Per Validator:**
- Setup: 5-10 minutes
- Sync: 5-15 minutes (with good peers)
- Total: ~15-25 minutes per validator

**For 7 Validators:**
- Sequential deployment: ~2-3 hours
- Parallel deployment (recommended): ~30-45 minutes

---

## Success Criteria

After deploying 7 validators (reaching 15 total):

✅ **Network Health:**
- Peer count: 15+ on each validator
- Syncing: false on all validators
- Best block: Same across all validators

✅ **GRANDPA Finality:**
- Finality lag: 2-4 blocks (optimal)
- Finalized block progress: Continuous
- Authority set: 21 validators (15 active minimum)

✅ **ASF Committee:**
- Committee size: 21
- PPFA rounds active
- No DETR P2P errors

✅ **Byzantine Fault Tolerance:**
- Can tolerate up to 5 faulty validators (15 - 2/3 = 10 minimum honest)
- GRANDPA supermajority achieved (2/3+1 of 21 = 15)

---

## Troubleshooting

**Issue: Validator not connecting to network**
```bash
# Check network key exists
ls /var/lib/flarechain/chains/flarechain_mainnet/network/secret_ed25519

# Verify bootnodes in service config
sudo systemctl cat flarechain-validator | grep bootnodes

# Check firewall
sudo ufw status
```

**Issue: Validator syncing slowly**
```bash
# Check peer count
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' | jq '.result | length'

# If low peers, verify bootnodes and public-addr flag
```

**Issue: Session keys not found**
```bash
# List keystore
ls -la /var/lib/flarechain/chains/flarechain_mainnet/keystore/

# Should have 3 files (aura, gran, asfk)
# If missing, re-insert keys using Step 3
```

---

## Next Steps After 15 Validators Active

1. **Monitor Network Stability** - Run for 24 hours
2. **Deploy Remaining 6 Validators** - Reach full 21-validator set
3. **Enable Governance** - Activate DAO treasury and multisig
4. **Public Announcement** - Network ready for mainnet operations

---

**Created:** 2025-11-03
**Current Network:** Ëtrid FlareChain Mainnet
**Genesis:** `0xca40bbf4f8367f63ea110afd54cf5fd38c44df100f9454b62135bfc09df74da8`
