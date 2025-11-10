# FlareChain Validator - Quick Start Guide

**Time to Deploy:** 15-30 minutes
**Skill Level:** Intermediate
**Cost:** $6-50/month depending on provider

---

## Overview

This guide will help you deploy a FlareChain validator from scratch in under 30 minutes.

**What You'll Need:**
- Cloud VM (Contabo, Oracle, Azure, DigitalOcean, or AWS)
- SSH access to the VM
- Basic Linux command line knowledge
- 15-30 minutes of time

---

## Option 1: One-Command Deployment (Fastest)

### For Contabo VMs:

```bash
# From your local machine
cd ~/Desktop/etrid
./docs/deployment-master/scripts/deploy-new-contabo-validator.sh <number> <ip> "<name>"

# Example:
./docs/deployment-master/scripts/deploy-new-contabo-validator.sh 26 157.173.200.100 "validator-name"
```

**That's it!** The script will:
- ✅ Open port 30333 (firewall)
- ✅ Deploy binary and chainspec
- ✅ Generate all session keys
- ✅ Create systemd service
- ✅ Start your validator

**Estimated time:** 10-15 minutes

---

## Option 2: Step-by-Step Deployment (All Providers)

### Step 1: Choose Your Cloud Provider (5 minutes)

| Provider | Cost/Month | Setup Difficulty | Free Tier? |
|----------|------------|------------------|------------|
| Contabo | $6-12 | Easy | No |
| Oracle Cloud | $0 (free tier) | Medium | Yes (2 VMs free forever) |
| DigitalOcean | $12-24 | Easy | $200 credit for 60 days |
| Azure | $30-60 | Medium | $200 credit for 30 days |
| AWS | $20-50 | Hard | 750 hours/month for 12 months |

**Recommended for beginners:** Oracle Cloud (free) or Contabo (cheapest paid)

**VM Specs Required:**
- 2+ vCPU
- 4GB RAM minimum
- 100GB storage
- Ubuntu 22.04 or 24.04

---

### Step 2: Configure Cloud Firewall (3 minutes)

**Open port 30333 in your cloud provider's firewall:**

#### Contabo:
```bash
# SSH to your VM first
ssh root@YOUR_VM_IP

# Then run:
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent
netfilter-persistent save
```

#### Oracle Cloud:
1. Oracle Console → Networking → Virtual Cloud Networks
2. Select your VCN → Security Lists → Default Security List
3. Add Ingress Rule: TCP port 30333 from 0.0.0.0/0

#### Azure:
1. Azure Portal → Virtual machines → Your VM
2. Networking → Create port rule → Inbound
3. Destination port: 30333, Protocol: TCP, Action: Allow

#### DigitalOcean:
1. DigitalOcean → Networking → Firewalls
2. Create Firewall → Inbound Rule
3. Type: Custom, Protocol: TCP, Port: 30333

#### AWS:
1. EC2 Console → Security Groups → Your SG
2. Inbound rules → Add rule
3. Type: Custom TCP, Port: 30333, Source: 0.0.0.0/0

**See detailed instructions:** [Cloud Provider Setup Master](../cloud-providers/CLOUD_PROVIDER_SETUP_MASTER.md)

---

### Step 3: Install FlareChain Binary (5 minutes)

**SSH to your VM:**
```bash
ssh root@YOUR_VM_IP  # or ubuntu@, azureuser@, etc.
```

**Option A: Download Pre-built Binary (Fastest)**
```bash
# Download latest release
wget https://github.com/etrid/flarechain/releases/latest/download/flarechain-node

# Install
sudo mv flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node

# Verify
flarechain-node --version
```

**Option B: Build from Source (45-60 minutes)**

See: [Linux Build Guide](../build-guides/BUILD_GUIDE_LINUX.md) or [macOS Build Guide](../build-guides/BUILD_GUIDE_MACOS.md)

---

### Step 4: Setup Validator (10 minutes)

```bash
# 1. Create directories
sudo mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
sudo mkdir -p /var/lib/etrid/chains/flarechain_mainnet/keystore

# 2. Download chainspec
wget https://raw.githubusercontent.com/etrid/flarechain/main/chainspec-mainnet-raw-FIXED.json \
  -O /var/lib/etrid/chainspec-mainnet-raw-FIXED.json

# 3. Generate network key
NETWORK_KEY=$(openssl rand -hex 32)
echo -n "$NETWORK_KEY" | sudo tee /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
sudo chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519

# 4. Generate AURA key (sr25519)
/usr/local/bin/flarechain-node key generate --scheme sr25519 --output-type json > /tmp/aura_key.json
AURA_SECRET=$(cat /tmp/aura_key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)

/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type aura \
    --scheme sr25519 \
    --suri "$AURA_SECRET"

# 5. Generate GRANDPA key (ed25519)
/usr/local/bin/flarechain-node key generate --scheme ed25519 --output-type json > /tmp/gran_key.json
GRAN_SECRET=$(cat /tmp/gran_key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)

/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type gran \
    --scheme ed25519 \
    --suri "$GRAN_SECRET"

# 6. Generate ASF key (uses same as AURA)
/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type asfk \
    --scheme sr25519 \
    --suri "$AURA_SECRET"

# 7. IMPORTANT: Save your keys!
echo "VALIDATOR Session Keys - $(date)" | sudo tee /root/validator_keys.txt
echo "AURA/ASF: $AURA_SECRET" | sudo tee -a /root/validator_keys.txt
echo "GRANDPA: $GRAN_SECRET" | sudo tee -a /root/validator_keys.txt
cat /root/validator_keys.txt

# ⚠️ BACK UP THESE KEYS IMMEDIATELY! ⚠️
# Copy /root/validator_keys.txt to a secure location
```

---

### Step 5: Create Systemd Service (3 minutes)

```bash
# Get your public IP
MY_IP=$(curl -s ifconfig.me)
echo "Your public IP: $MY_IP"

# Create systemd service
sudo bash -c "cat > /etc/systemd/system/flarechain-validator.service << 'EOFSERVICE'
[Unit]
Description=FlareChain Validator Node
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
    --name \"MyValidator\" \
    --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/12D3KooWPyfp2DECPKTmJ1AhxB6midHnp7wYTP15vBAxbTewxaq1 \
    --bootnodes /ip4/85.239.239.194/tcp/30333/p2p/12D3KooWSrYpSQ6SiDR3uduqbiepyfVp8xmaC8mzY6RmU29MEHGv \
    --public-addr \"/ip4/MY_IP_PLACEHOLDER/tcp/30333\" \
    --port 30333 \
    --prometheus-port 9615 \
    --prometheus-external

Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOFSERVICE"

# Replace placeholder with your IP
sudo sed -i "s/MY_IP_PLACEHOLDER/$MY_IP/g" /etc/systemd/system/flarechain-validator.service

# Replace validator name (optional)
sudo sed -i 's/MyValidator/YourValidatorName/g' /etc/systemd/system/flarechain-validator.service
```

---

### Step 6: Start Validator (1 minute)

```bash
# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable flarechain-validator
sudo systemctl start flarechain-validator

# Check status
sudo systemctl status flarechain-validator

# Watch logs
sudo journalctl -u flarechain-validator -f
```

**Press Ctrl+C to stop watching logs**

---

### Step 7: Verify Everything Works (5 minutes)

```bash
# 1. Check service is running
sudo systemctl is-active flarechain-validator
# Should show: active

# 2. Check peer count (wait 1-2 minutes after start)
sudo journalctl -u flarechain-validator -n 10 | grep peers
# Should show: "X peers" where X >= 5

# 3. Check sync status
sudo journalctl -u flarechain-validator -n 10 | grep Syncing
# Should show syncing messages with increasing block numbers

# 4. Verify port is open
sudo netstat -tlnp | grep 30333
# Should show: tcp6  0  0 :::30333  LISTEN

# 5. Check for errors
sudo journalctl -u flarechain-validator -n 50 | grep -i error
# Should show minimal or no errors
```

**Expected Behavior:**

| Time | Peers | Status |
|------|-------|--------|
| 0-5 mins | 0-3 | Discovering network |
| 5-30 mins | 5-10 | Connecting to bootnodes |
| 30+ mins | 10-20 | Full mesh, syncing blocks |

---

## Troubleshooting

### 0 Peers / Not Connecting

**Most common cause:** Firewall blocking port 30333

```bash
# Check firewall
sudo iptables -L INPUT -n | grep 30333

# If not showing, add rule
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo netfilter-persistent save

# Restart validator
sudo systemctl restart flarechain-validator
```

**Also check:** Cloud provider firewall (see Step 2)

### Service Won't Start

```bash
# Check detailed error
sudo journalctl -u flarechain-validator -n 50 --no-pager

# Common fixes:
# 1. Binary not executable
sudo chmod +x /usr/local/bin/flarechain-node

# 2. Missing chainspec
wget https://raw.githubusercontent.com/etrid/flarechain/main/chainspec-mainnet-raw-FIXED.json \
  -O /var/lib/etrid/chainspec-mainnet-raw-FIXED.json

# 3. Port already in use
sudo netstat -tlnp | grep 30333
# Kill process if found
```

### Not Syncing Blocks

```bash
# 1. Check peer count first
sudo journalctl -u flarechain-validator -n 5 | grep peers

# 2. If peers > 5 but not syncing, restart
sudo systemctl restart flarechain-validator

# 3. Verify genesis hash
grep '"genesis":' /var/lib/etrid/chainspec-mainnet-raw-FIXED.json | grep "0xca40"
```

**See full troubleshooting guide:** [Deployment Troubleshooting](../troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md)

---

## Next Steps After Deployment

### 1. Back Up Your Keys (CRITICAL)

```bash
# Copy keys to your local machine
scp root@YOUR_VM_IP:/root/validator_keys.txt ~/validator-keys-backup.txt

# Store securely:
# - Password manager
# - Encrypted USB drive
# - Hardware wallet
# - Multiple secure locations
```

### 2. Monitor Your Validator

```bash
# Create monitoring script
cat > ~/check-validator.sh << 'EOF'
#!/bin/bash
echo "=== Validator Status ==="
ssh root@YOUR_VM_IP "systemctl is-active flarechain-validator && \
  journalctl -u flarechain-validator -n 3 --no-pager | grep peers"
EOF

chmod +x ~/check-validator.sh

# Run it
./check-validator.sh
```

### 3. Set Up Prometheus/Grafana (Optional)

Monitor metrics at `http://YOUR_VM_IP:9615/metrics`

See: [Monitoring Guide](../../MONITORING_GUIDE.md)

### 4. Fund Your Validator Account

Your validator needs a small amount of ETR tokens for:
- Submitting validator registration transaction
- Paying transaction fees

**How to get ETR:**
- Faucet: https://faucet.etrid.org
- Community: Discord #validator-support
- Purchase: [Coming soon]

### 5. Register as Validator (On-Chain)

```bash
# Once you have ETR tokens, submit registration via:
# - Governance UI: https://governance.etrid.org
# - Polkadot.js: https://polkadot.js.org/apps/?rpc=wss://rpc.etrid.org

# You'll need:
# - Your validator account address
# - AURA public key (from validator_keys.txt)
# - GRANDPA public key (from validator_keys.txt)
```

### 6. Join the Community

- **Discord:** https://discord.gg/etrid
- **Telegram:** https://t.me/etrid
- **GitHub:** https://github.com/etrid/flarechain

---

## Checklist

**Before going live:**

- [ ] Validator synced to latest block
- [ ] Peer count >= 10
- [ ] Session keys backed up securely
- [ ] Firewall configured correctly
- [ ] Monitoring setup (optional but recommended)
- [ ] Validator registered on-chain
- [ ] Joined community channels
- [ ] Documented your setup

**Ongoing maintenance:**

- [ ] Check validator health daily
- [ ] Keep system packages updated
- [ ] Monitor disk space
- [ ] Watch for alerts/notifications
- [ ] Participate in governance
- [ ] Stay updated with releases

---

## Cost Breakdown

### Monthly Costs (Estimated)

| Provider | VM Cost | Bandwidth | Storage | Total |
|----------|---------|-----------|---------|-------|
| Contabo VPS-S | $6.99 | Included | Included | ~$7/month |
| Oracle Cloud Free | $0 | $0 | $0 | **$0/month** |
| DigitalOcean Basic | $12 | Included | $1/10GB | ~$13/month |
| Azure B2s | $30 | $0.05/GB | $4/64GB | ~$35/month |
| AWS t3.small | $15 | $0.09/GB | $8/100GB | ~$25/month |

**Best value:**
- Oracle Cloud (free tier) - $0/month
- Contabo VPS-S - $7/month

**Most reliable:**
- Azure or AWS (higher cost but better SLA)

---

## Support

**Need help?**

1. Check [Troubleshooting Guide](../troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md)
2. Search Discord #validator-support
3. Create GitHub issue: https://github.com/etrid/flarechain/issues
4. Email: support@etrid.org

**Include when asking for help:**
- Cloud provider and VM specs
- Output of: `flarechain-node --version`
- Output of: `sudo journalctl -u flarechain-validator -n 50`
- Steps you've already tried

---

## FAQ

**Q: How long does it take to sync?**
A: Initial sync takes 2-6 hours depending on network speed and VM performance.

**Q: Can I run multiple validators on one VM?**
A: Not recommended. Each validator should have dedicated resources.

**Q: What happens if my validator goes offline?**
A: You may miss block production rewards and finality participation. Ensure high uptime (>95%).

**Q: Can I change my validator name later?**
A: Yes, edit the systemd service file and restart.

**Q: How do I update FlareChain?**
A: Download new binary, stop service, replace binary, restart service.

**Q: Do I need a static IP?**
A: Recommended but not required. If IP changes, update `--public-addr` in systemd service.

---

**Congratulations! You're now running a FlareChain validator! 🎉**

**Last Updated:** November 9, 2025
**Maintainer:** FlareChain Core Team
