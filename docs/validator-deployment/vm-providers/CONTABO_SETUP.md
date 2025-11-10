# Contabo VM Setup for FlareChain Validators

**Complete guide for deploying FlareChain validators on Contabo VPS**

Contabo is the most cost-effective option for FlareChain validators ($6-15/month), but requires **critical firewall configuration** immediately after provisioning.

---

## ⚠️ CRITICAL WARNING

**ALL Contabo VMs have `iptables policy DROP` by default, which blocks ALL incoming connections.**

**Port 30333 MUST be explicitly opened or your validator will have 0 peers and never sync.**

This is the #1 cause of validator deployment failures on Contabo.

---

## Table of Contents

1. [Why Choose Contabo](#why-choose-contabo)
2. [Prerequisites](#prerequisites)
3. [Step-by-Step Setup](#step-by-step-setup)
4. [Automated Deployment](#automated-deployment)
5. [Manual Deployment](#manual-deployment)
6. [Troubleshooting](#troubleshooting)

---

## Why Choose Contabo

### Pros
- **Extremely cost-effective:** $6-15/month for validator-grade hardware
- **Good bandwidth:** Generous network allocation
- **Multiple locations:** Seattle, New York, St. Louis, UK, Germany
- **Reliable:** 99.9% uptime SLA
- **No traffic limits:** Unmetered bandwidth

### Cons
- **Firewall blocks everything by default:** Requires immediate configuration
- **Email-only support:** Slower response times
- **Manual IP configuration:** Not as automated as cloud providers

### Recommended for
- Budget-conscious validators
- Testnet deployments
- Learning and development
- Geographic distribution (multiple cheap VMs in different regions)

---

## Prerequisites

### Account Setup
1. Create account at https://contabo.com
2. Verify email
3. Add payment method (PayPal or credit card)

### SSH Key
Generate SSH key pair if you don't have one:
```bash
ssh-keygen -t ed25519 -f ~/.ssh/contabo-validators -C "flarechain-validator"
```

---

## Step-by-Step Setup

### 1. Order VPS

**Recommended Configuration:**
- **Product:** VPS S or VPS M
- **OS:** Ubuntu 22.04 LTS
- **Location:** Choose based on your geography
  - Seattle (US West)
  - New York (US East)
  - St. Louis (US Central)
  - Nuremberg (EU)
  - UK

**Minimum specs for Validity Node:**
- 4 vCPU cores
- 8 GB RAM
- 200 GB SSD

**For Director Node:**
- 6 vCPU cores
- 16 GB RAM
- 400 GB SSD

### 2. Get VM IP Address

After provisioning (usually 24-48 hours), Contabo will email:
- VM IP address
- Root password
- SSH access details

### 3. First Login

```bash
ssh root@<your-vm-ip>
```

Enter the root password from email.

**Change root password immediately:**
```bash
passwd
```

### 4. Add Your SSH Key

```bash
# On your local machine
ssh-copy-id -i ~/.ssh/contabo-validators.pub root@<your-vm-ip>
```

Or manually:
```bash
# On the VM
mkdir -p ~/.ssh
chmod 700 ~/.ssh
echo "<paste-your-public-key-here>" >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
```

### 5. ⚠️ CRITICAL: Open Port 30333

**This MUST be done before deploying the validator.**

```bash
# SSH into the VM
ssh -i ~/.ssh/contabo-validators root@<your-vm-ip>

# Open port 30333
sudo iptables -I INPUT 1 -p tcp --dport 30333 -m comment --comment "FlareChain P2P" -j ACCEPT

# Install iptables-persistent to save rules across reboots
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent

# Save the firewall rules
netfilter-persistent save

# Verify the rule was added
sudo iptables -L INPUT -n | grep 30333
```

**Expected output:**
```
ACCEPT     tcp  --  0.0.0.0/0            0.0.0.0/0            tcp dpt:30333 /* FlareChain P2P */
```

### 6. Update System

```bash
apt-get update && apt-get upgrade -y
```

### 7. Install Dependencies

```bash
apt-get install -y curl wget git build-essential libssl-dev pkg-config
```

---

## Automated Deployment

**Easiest option - one command deployment:**

```bash
# On your local machine (NOT on the VM)
cd ~/Desktop/etrid
./docs/validator-deployment/scripts/deploy-new-validator.sh <number> <ip> "<name>"
```

**Example:**
```bash
./docs/validator-deployment/scripts/deploy-new-validator.sh 26 157.173.200.100 "stlouis-vn05"
```

### What the Script Does

1. ✅ **Opens port 30333** (fixes Contabo firewall issue)
2. ✅ **Installs iptables-persistent** (survives reboots)
3. ✅ **Creates directory structure**
4. ✅ **Deploys binary and chainspec** from existing validator
5. ✅ **Generates unique session keys** (AURA, GRANDPA, ASF)
6. ✅ **Generates network key**
7. ✅ **Creates systemd service** with correct bootnode and --public-addr
8. ✅ **Starts the validator**

### After Running the Script

1. **Save session keys** displayed by the script to:
   ```
   ~/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md
   ```

2. **Monitor the validator:**
   ```bash
   ssh -i ~/.ssh/contabo-validators root@<your-vm-ip> 'journalctl -u flarechain-validator -f'
   ```

3. **Check peer count** (after 30 seconds):
   ```bash
   ssh -i ~/.ssh/contabo-validators root@<your-vm-ip> 'journalctl -u flarechain-validator -n 5 | grep peers'
   ```

**Expected peer count:**
- 0-5 mins: 0-3 peers (discovering network)
- 5-30 mins: 5-10 peers (connecting via bootnodes)
- 30+ mins: 10-20 peers (full P2P mesh)

---

## Manual Deployment

If the automated script fails, follow these manual steps:

### 1. Open Firewall (if not done already)

```bash
sudo iptables -I INPUT 1 -p tcp --dport 30333 -m comment --comment "FlareChain P2P" -j ACCEPT
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent
netfilter-persistent save
```

### 2. Create Directory Structure

```bash
mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
mkdir -p /var/lib/etrid/chains/flarechain_mainnet/keystore
mkdir -p /usr/local/bin
```

### 3. Deploy Binary

**Option A: Copy from existing validator (fastest):**
```bash
# On your local machine
ssh -i ~/.ssh/contabo-validators root@85.239.239.194 'cat /usr/local/bin/flarechain-node' | \
    ssh -i ~/.ssh/contabo-validators root@<new-vm-ip> 'cat > /usr/local/bin/flarechain-node && chmod +x /usr/local/bin/flarechain-node'
```

**Option B: Build from source:**
See [Linux Build Guide](../build-instructions/BUILD_LINUX.md)

### 4. Deploy Chainspec

```bash
# Copy from existing validator
ssh -i ~/.ssh/contabo-validators root@85.239.239.194 'cat /var/lib/etrid/chainspec-mainnet-raw-FIXED.json' | \
    ssh -i ~/.ssh/contabo-validators root@<new-vm-ip> 'cat > /var/lib/etrid/chainspec-mainnet-raw-FIXED.json'
```

### 5. Generate Network Key

```bash
# On the VM
NETWORK_KEY=$(openssl rand -hex 32)
echo -n "$NETWORK_KEY" > /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
```

### 6. Generate Session Keys

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

# Display keys for documentation
echo "AURA/ASF: $AURA_SECRET"
echo "GRANDPA: $GRAN_SECRET"
```

**⚠️ Save these mnemonics to the master secrets file!**

### 7. Create Systemd Service

```bash
cat > /etc/systemd/system/flarechain-validator.service << 'EOF'
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
    --name "YOUR_VALIDATOR_NAME" \
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
EOF

# Replace placeholders
sed -i "s/YOUR_VALIDATOR_NAME/Validator-26/g" /etc/systemd/system/flarechain-validator.service
sed -i "s/YOUR_VM_IP/<your-vm-ip>/g" /etc/systemd/system/flarechain-validator.service
```

### 8. Start Validator

```bash
systemctl daemon-reload
systemctl enable flarechain-validator
systemctl start flarechain-validator
```

### 9. Verify

```bash
# Check status
systemctl status flarechain-validator

# Monitor logs
journalctl -u flarechain-validator -f
```

---

## Troubleshooting

### Issue: 0 Peers After 5 Minutes

**Diagnosis:**
```bash
sudo iptables -L INPUT -n | grep 30333
```

**If no output:** Firewall is blocking port 30333

**Fix:**
```bash
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent
netfilter-persistent save
systemctl restart flarechain-validator
```

### Issue: NetworkKeyNotFound

**Error:**
```
Error: NetworkKeyNotFound("/var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519")
```

**Fix:**
```bash
NETWORK_KEY=$(openssl rand -hex 32)
mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
echo -n "$NETWORK_KEY" > /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
systemctl restart flarechain-validator
```

### Issue: Wrong Genesis Hash

**Symptom:** Syncing to different chain

**Fix:**
```bash
# Download correct chainspec
scp -i ~/.ssh/contabo-validators root@85.239.239.194:/var/lib/etrid/chainspec-mainnet-raw-FIXED.json /var/lib/etrid/

# Verify genesis
grep '"genesis":' /var/lib/etrid/chainspec-mainnet-raw-FIXED.json
# Should contain: 0xca40...4da8

# Clear database and restart
systemctl stop flarechain-validator
rm -rf /var/lib/etrid/chains/flarechain_mainnet/db
systemctl start flarechain-validator
```

### Issue: Service Crashes on Startup

**Check logs:**
```bash
journalctl -u flarechain-validator -n 50 --no-pager
```

Common causes:
- Missing network key → See NetworkKeyNotFound fix above
- Corrupt database → Delete `/var/lib/etrid/chains/flarechain_mainnet/db` and restart
- Missing chainspec → Re-download from existing validator

---

## Port Testing

### Test from External Machine

```bash
nc -zv <your-contabo-vm-ip> 30333
```

**If successful:**
```
Connection to <ip> 30333 port [tcp/*] succeeded!
```

**If blocked:**
```
nc: connect to <ip> port 30333 (tcp) failed: Connection refused
```

### Test from Inside VM

```bash
# Check iptables rule exists
sudo iptables -L INPUT -n -v | grep 30333

# Check process is listening
sudo ss -tlnp | grep 30333
```

---

## Contabo-Specific Tips

### 1. Firewall Persistence

Always use `iptables-persistent` on Contabo:
```bash
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent
netfilter-persistent save
```

Without this, firewall rules are lost on reboot.

### 2. Default Policy

Contabo VMs have `iptables policy DROP` by default:
```bash
sudo iptables -L | head -3
```

Output:
```
Chain INPUT (policy DROP)
Chain FORWARD (policy DROP)
Chain OUTPUT (policy ACCEPT)
```

This means **all incoming connections are blocked unless explicitly allowed**.

### 3. Multiple Validators

If running multiple Contabo validators, consider:
- Different datacenters for geographic distribution
- Consistent naming scheme (e.g., seattle-vn01, seattle-vn02)
- Centralized monitoring to track all validators

---

## Cost Optimization

### Recommended Plans

**For Validity Nodes:**
- **VPS M:** 6 vCPU, 16GB RAM, 400GB SSD - ~$12/month
- Best value for production validators

**For Testing:**
- **VPS S:** 4 vCPU, 8GB RAM, 200GB SSD - ~$6/month
- Minimum for testnet/development

### Multi-Validator Discounts

Contabo doesn't offer volume discounts, but cost is already very low:
- 5 validators: $30-60/month
- 10 validators: $60-120/month
- 20 validators: $120-240/month

Compare to AWS/Azure: $500-1000/month for same setup.

---

## Security Recommendations

1. **Disable password authentication:**
   ```bash
   sed -i 's/PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
   systemctl restart sshd
   ```

2. **Enable automatic security updates:**
   ```bash
   apt-get install -y unattended-upgrades
   dpkg-reconfigure -plow unattended-upgrades
   ```

3. **Configure fail2ban:**
   ```bash
   apt-get install -y fail2ban
   systemctl enable fail2ban
   systemctl start fail2ban
   ```

4. **Only open required ports:**
   - 30333 (FlareChain P2P) - Required
   - 22 (SSH) - Required for management
   - 9615 (Prometheus) - Optional (if using monitoring)

---

## Next Steps

1. ✅ VM provisioned and firewall configured
2. ✅ Validator deployed and running
3. ✅ Peers connecting (8-20 peers)
4. ✅ Syncing to mainnet (genesis 0xca40...4da8)
5. 📝 Session keys saved to master secrets file

**Monitor your validator:**
```bash
ssh -i ~/.ssh/contabo-validators root@<your-vm-ip> 'journalctl -u flarechain-validator -f'
```

---

**Last Updated:** November 9, 2025
**Firewall Issue Discovered:** November 9, 2025 (all 20 Contabo VMs had port 30333 blocked)
