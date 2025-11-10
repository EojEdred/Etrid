# Contabo VM Setup Checklist for FlareChain Mainnet Validators

**⚠️ CRITICAL:** All Contabo VMs have `iptables policy DROP` by default, which blocks all incoming connections unless explicitly allowed.

## Pre-Deployment Checklist

### 1. Firewall Configuration (MUST DO FIRST)

```bash
# Open port 30333 for FlareChain P2P
sudo iptables -I INPUT 1 -p tcp --dport 30333 -m comment --comment "FlareChain P2P" -j ACCEPT

# Install iptables-persistent to save rules across reboots
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent

# Save the rules
netfilter-persistent save
```

**Verify it worked:**
```bash
sudo iptables -L INPUT -n | grep 30333
# Should show: ACCEPT tcp -- 0.0.0.0/0 0.0.0.0/0 tcp dpt:30333
```

### 2. Install Dependencies

```bash
# Update system
apt-get update && apt-get upgrade -y

# Install required packages
apt-get install -y curl wget git build-essential libssl-dev pkg-config
```

### 3. Create Directory Structure

```bash
mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
mkdir -p /var/lib/etrid/chains/flarechain_mainnet/keystore
mkdir -p /usr/local/bin
```

### 4. Deploy Binaries

```bash
# Copy from existing validator or build fresh
# flarechain-node should go to /usr/local/bin/
chmod +x /usr/local/bin/flarechain-node
```

### 5. Deploy Chainspec

```bash
# Download from existing validator
scp -i ~/.ssh/contabo-validators root@85.239.239.194:/var/lib/etrid/chainspec-mainnet-raw-FIXED.json /var/lib/etrid/

# Verify genesis hash
grep -A 5 '"genesis":' /var/lib/etrid/chainspec-mainnet-raw-FIXED.json
# Should contain: 0xca40...4da8
```

### 6. Generate Network Key

```bash
# Generate Ed25519 network key
NETWORK_KEY=$(openssl rand -hex 32)
echo -n "$NETWORK_KEY" > /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
```

### 7. Generate Session Keys

```bash
# Generate AURA key (sr25519)
/usr/local/bin/flarechain-node key generate --scheme sr25519 --output-type json > /tmp/aura_key.json
AURA_SECRET=$(cat /tmp/aura_key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)

/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type aura \
    --scheme sr25519 \
    --suri "$AURA_SECRET"

# Generate GRANDPA key (ed25519)
/usr/local/bin/flarechain-node key generate --scheme ed25519 --output-type json > /tmp/gran_key.json
GRAN_SECRET=$(cat /tmp/gran_key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)

/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type gran \
    --scheme ed25519 \
    --suri "$GRAN_SECRET"

# Generate ASF key (uses same as AURA)
/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type asfk \
    --scheme sr25519 \
    --suri "$AURA_SECRET"

# Save keys for documentation
echo "VALIDATOR-XX" > /tmp/validator_keys.txt
echo "AURA/ASF: $AURA_SECRET" >> /tmp/validator_keys.txt
echo "GRANDPA: $GRAN_SECRET" >> /tmp/validator_keys.txt
cat /tmp/validator_keys.txt
```

**⚠️ CRITICAL:** Save these mnemonics to the master secrets file!

### 8. Create Systemd Service

```bash
cat > /etc/systemd/system/flarechain-validator.service << 'EOFSERVICE'
[Unit]
Description=FlareChain Validator Node (Validator-XX)
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=root
WorkingDirectory=/var/lib/etrid
ExecStart=/usr/local/bin/flarechain-node \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --validator \
    --name "Validator-XX" \
    --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/12D3KooWPyfp2DECPKTmJ1AhxB6midHnp7wYTP15vBAxbTewxaq1 \
    --bootnodes /ip4/85.239.239.194/tcp/30333/p2p/12D3KooWSrYpSQ6SiDR3uduqbiepyfVp8xmaC8mzY6RmU29MEHGv \
    --public-addr "/ip4/YOUR_VM_IP/tcp/30333" \
    --port 30333 \
    --prometheus-port 9615 \
    --prometheus-external

Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOFSERVICE

# Replace placeholders
sed -i "s/Validator-XX/Validator-$VALIDATOR_NUM/g" /etc/systemd/system/flarechain-validator.service
sed -i "s/YOUR_VM_IP/$VM_IP/g" /etc/systemd/system/flarechain-validator.service

# Enable and start service
systemctl daemon-reload
systemctl enable flarechain-validator
systemctl start flarechain-validator
```

### 9. Verify Everything Works

```bash
# Check service status
systemctl status flarechain-validator

# Check logs
journalctl -u flarechain-validator -f

# Verify peer connections (wait 30 seconds after start)
journalctl -u flarechain-validator -n 5 | grep peers
# Should show: "X peers" where X > 5

# Verify syncing
journalctl -u flarechain-validator -n 5 | grep Syncing
```

### 10. Document Keys

Add the validator information to:
`/Users/macbook/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md`

---

## Common Issues and Solutions

### Issue: 0 peers or low peer count

**Cause:** Firewall blocking port 30333

**Fix:**
```bash
# Check if port is open
sudo iptables -L INPUT -n | grep 30333

# If not, add rule
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo netfilter-persistent save
```

### Issue: NetworkKeyNotFound error

**Cause:** Missing network key

**Fix:**
```bash
NETWORK_KEY=$(openssl rand -hex 32)
mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
echo -n "$NETWORK_KEY" > /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
systemctl restart flarechain-validator
```

### Issue: Wrong genesis hash / not syncing to mainnet

**Cause:** Wrong chainspec file

**Fix:**
```bash
# Download correct chainspec
scp -i ~/.ssh/contabo-validators root@85.239.239.194:/var/lib/etrid/chainspec-mainnet-raw-FIXED.json /var/lib/etrid/

# Verify genesis
grep '"genesis":' /var/lib/etrid/chainspec-mainnet-raw-FIXED.json
# Should show: 0xca40...4da8

# Clear database and restart
systemctl stop flarechain-validator
rm -rf /var/lib/etrid/chains/flarechain_mainnet/db
systemctl start flarechain-validator
```

---

## Automation Script

For quick deployment, use this script:

```bash
#!/bin/bash
# deploy-new-contabo-validator.sh

VALIDATOR_NUM=$1
VM_IP=$2

if [ -z "$VALIDATOR_NUM" ] || [ -z "$VM_IP" ]; then
    echo "Usage: $0 <validator_number> <vm_ip>"
    exit 1
fi

echo "Deploying Validator-$VALIDATOR_NUM on $VM_IP..."

ssh -i ~/.ssh/contabo-validators root@$VM_IP bash << 'ENDSETUP'
    # 1. FIREWALL (CRITICAL!)
    sudo iptables -I INPUT 1 -p tcp --dport 30333 -m comment --comment "FlareChain P2P" -j ACCEPT
    DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent
    netfilter-persistent save
    
    # 2. Create directories
    mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
    mkdir -p /var/lib/etrid/chains/flarechain_mainnet/keystore
    
    echo "✅ Setup complete. Ready for binary and chainspec deployment."
ENDSETUP
```

**Save to:** `/Users/macbook/Desktop/etrid/scripts/deploy-new-contabo-validator.sh`

---

## Post-Deployment Monitoring

Monitor new validator for 24 hours:

```bash
# Check peer count every 5 minutes
watch -n 300 'ssh -i ~/.ssh/contabo-validators root@VM_IP "journalctl -u flarechain-validator -n 1 | grep peers"'

# Verify block sync progress
ssh -i ~/.ssh/contabo-validators root@VM_IP 'journalctl -u flarechain-validator -n 20 | grep Syncing'
```

Expected behavior:
- **0-5 mins:** 0-3 peers (discovering network)
- **5-30 mins:** 5-10 peers (connecting to bootnodes)
- **30+ mins:** 10-20 peers (full P2P mesh established)

---

**Created:** November 9, 2025  
**Last Updated:** November 9, 2025  
**Issue Context:** All Contabo VMs discovered with blocked port 30333 due to default `iptables policy DROP`
