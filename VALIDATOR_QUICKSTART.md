# Ëtrid FlareChain Validator - Quick Start Guide

This guide will help you set up an Ëtrid validator node in minutes.

---

## Prerequisites

- Linux server (Ubuntu 20.04+ recommended)
- 4+ CPU cores
- 8GB+ RAM
- 100GB+ SSD storage
- Open ports: 30333 (P2P), 9944 (RPC), 9615 (Metrics)

---

## One-Command Setup

### Step 1: Download and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/etrid.git
cd etrid

# Build the node (takes ~10-15 minutes)
cargo build --release -p flarechain-node

# Move binary to accessible location
sudo mkdir -p /opt/etrid
sudo cp target/release/flarechain-node /opt/etrid/
sudo chmod +x /opt/etrid/flarechain-node
```

### Step 2: Bootstrap Your Validator

```bash
# Set up environment
export FLARECHAIN_BINARY=/opt/etrid/flarechain-node
export BASE_PATH=/var/lib/etrid
export NODE_NAME="my-validator-$(hostname)"

# Run bootstrap script
sudo mkdir -p $BASE_PATH
sudo chmod +x scripts/bootstrap-validator.sh
sudo ./scripts/bootstrap-validator.sh
```

The bootstrap script will:
1. Generate network key (for P2P discovery)
2. Generate validator keys (AURA, GRANDPA, ASF)
3. Save all keys to `/var/lib/etrid/keys/`
4. Display your bootnode address

**IMPORTANT**: Back up the keys directory immediately!
```bash
sudo tar -czf validator-keys-backup.tar.gz /var/lib/etrid/keys/
```

### Step 3: Start Your Validator

```bash
# Simple start (auto-runs bootstrap if needed)
sudo ./scripts/start-validator.sh
```

Or connect to existing bootnodes:
```bash
sudo ./scripts/start-validator.sh \
  --bootnode /ip4/20.186.91.207/tcp/30333/p2p/12D3KooW...
```

---

## What Gets Created

After bootstrap, you'll have:

```
/var/lib/etrid/
├── keys/
│   ├── validator_seed         # Your secret validator key - BACKUP THIS!
│   ├── network_secret          # Your network key
│   └── validator_keys.txt      # Public keys reference
├── keystore/                   # Contains inserted keys
└── network/
    └── secret_ed25519          # Network identity
```

---

## Connecting to the Network

### For the First Node (Bootstrap Node)

After starting, you'll see:
```
Your bootnode address:
  /ip4/YOUR_PUBLIC_IP/tcp/30333/p2p/12D3KooW...
```

Share this address with other validators!

### For Additional Nodes

Start with the bootstrap node's address:
```bash
sudo ./scripts/start-validator.sh \
  --bootnode /ip4/20.186.91.207/tcp/30333/p2p/12D3KooW...
```

---

## Verifying Your Node

### Check Node Status
```bash
# View logs
journalctl -u etrid-validator -f

# Check peer connections
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_peers"}' \
  http://localhost:9944
```

### Success Indicators
- ✅ "Idle (N peers)" - Connected to network
- ✅ "Imported #123" - Syncing blocks
- ✅ "Starting consensus session" - Participating in consensus
- ✅ "Prepared block for proposing" - Producing blocks

---

## Environment Variables

Customize your setup:

```bash
# Binary location (default: ./flarechain-node)
export FLARECHAIN_BINARY=/opt/etrid/flarechain-node

# Data directory (default: /var/lib/etrid)
export BASE_PATH=/data/validator

# Chain spec (default: local)
export CHAIN=local

# Node name (default: etrid-validator-$HOSTNAME)
export NODE_NAME="my-awesome-validator"

# Network ports
export RPC_PORT=9944
export WS_PORT=9945
export P2P_PORT=30333
```

---

## Running as System Service

Create `/etc/systemd/system/etrid-validator.service`:

```ini
[Unit]
Description=Ëtrid FlareChain Validator
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/home/etrid/etrid
Environment="FLARECHAIN_BINARY=/opt/etrid/flarechain-node"
Environment="BASE_PATH=/var/lib/etrid"
Environment="NODE_NAME=my-validator"
ExecStart=/home/etrid/etrid/scripts/start-validator.sh --bootnode /ip4/20.186.91.207/tcp/30333/p2p/12D3KooW...
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-validator
sudo systemctl start etrid-validator
sudo systemctl status etrid-validator
```

---

## Security Best Practices

### 1. Firewall Configuration
```bash
# Allow P2P connections
sudo ufw allow 30333/tcp comment "Etrid P2P"

# Restrict RPC to localhost only (recommended)
sudo ufw deny 9944/tcp

# Or allow from specific IPs
sudo ufw allow from YOUR_IP to any port 9944
```

### 2. Key Backup
```bash
# Backup validator keys (DO THIS NOW!)
sudo tar -czf validator-keys-$(date +%Y%m%d).tar.gz /var/lib/etrid/keys/

# Store backup offline or in secure cloud storage
# NEVER commit keys to git!
```

### 3. Monitor Your Node
```bash
# Setup Prometheus monitoring (port 9615)
# Grafana dashboard for validator metrics
# Alert on peer count dropping to 0
```

---

## Troubleshooting

### No Peers
```bash
# Check your public IP
curl ifconfig.me

# Verify port is open
nc -zv YOUR_PUBLIC_IP 30333

# Check firewall
sudo ufw status
```

### "CommitteeFull" Error
This is fixed by the bootstrap script which inserts ASF keys. If you see this:
```bash
# Re-run bootstrap
sudo rm -rf /var/lib/etrid/keystore
sudo ./scripts/bootstrap-validator.sh
```

### "NetworkKeyNotFound" Error
```bash
# Regenerate network key
sudo rm -rf /var/lib/etrid/network
sudo /opt/etrid/flarechain-node key generate-node-key --base-path=/var/lib/etrid
```

---

## Adding Your Keys to Chain Spec

For mainnet launches, share your public keys with the chain spec maintainer:

```bash
# Your keys are in:
cat /var/lib/etrid/keys/validator_keys.txt
```

Send the AURA and GRANDPA public keys to be added to the chain spec.

---

## Updating Chain Spec with Bootnodes

Maintainers can add bootnodes to the chain spec:

```bash
./scripts/add-bootnode.sh \
  infrastructure/chain-specs/flarechain-local.json \
  /ip4/20.186.91.207/tcp/30333/p2p/12D3KooW...
```

---

## Need Help?

- **Documentation**: `/docs`
- **Issues**: https://github.com/yourusername/etrid/issues
- **Discord**: [Your Discord Link]

---

## Quick Commands Reference

```bash
# Bootstrap validator
sudo ./scripts/bootstrap-validator.sh

# Start validator
sudo ./scripts/start-validator.sh

# Start with bootnode
sudo ./scripts/start-validator.sh --bootnode /ip4/.../p2p/...

# Check peers
curl -s http://localhost:9944 -H "Content-Type: application/json" \
  -d '{"id":1,"jsonrpc":"2.0","method":"system_peers"}' | jq '.result | length'

# Backup keys
sudo tar -czf keys-backup.tar.gz /var/lib/etrid/keys/

# View logs
tail -f /var/lib/etrid/node.log

# Stop node
sudo systemctl stop etrid-validator
```

---

That's it! Your validator should now be running and participating in consensus.
