# FlareChain Validator - Cloud Provider Setup Master Guide

**Version:** 1.0
**Last Updated:** November 9, 2025
**Supported Providers:** Contabo, Oracle Cloud, Azure, DigitalOcean, AWS

---

## Table of Contents

1. [Quick Reference](#quick-reference)
2. [Contabo Setup](#contabo-setup)
3. [Oracle Cloud Setup](#oracle-cloud-setup)
4. [Azure Setup](#azure-setup)
5. [DigitalOcean Setup](#digitalocean-setup)
6. [AWS Setup](#aws-setup)
7. [Common Configuration](#common-configuration)
8. [Troubleshooting](#troubleshooting)

---

## Quick Reference

| Provider | Default Firewall | Port Opening Method | SSH Key Required | Cost (est./month) |
|----------|------------------|---------------------|------------------|-------------------|
| Contabo | iptables DROP | Manual iptables | Yes | $6-12 |
| Oracle Cloud | NSG closed | Security List rules | Yes | Free tier available |
| Azure | NSG closed | Network Security Group | Yes | $30-60 |
| DigitalOcean | UFW disabled | UFW or cloud firewall | Yes | $12-24 |
| AWS | Security Group closed | Security Group rules | Yes | $20-50 |

**Required Ports for All Providers:**
- `30333` - P2P networking (TCP)
- `9615` - Prometheus metrics (TCP, optional)
- `9944` - WebSocket RPC (TCP, optional)
- `9933` - HTTP RPC (TCP, optional)

---

## Contabo Setup

### 🔴 CRITICAL: Firewall Configuration

**⚠️ All Contabo VMs have `iptables policy DROP` by default.** You MUST open port 30333 FIRST before anything else will work.

### Prerequisites

- VPS plan: VPS-S or higher (2 vCPU, 4GB RAM minimum)
- Ubuntu 22.04 or 24.04
- Root SSH access
- SSH key configured

### Step 1: Open Firewall (DO THIS FIRST!)

```bash
# SSH to your Contabo VM
ssh root@YOUR_CONTABO_IP

# Open port 30333 for FlareChain P2P
sudo iptables -I INPUT 1 -p tcp --dport 30333 -m comment --comment "FlareChain P2P" -j ACCEPT

# Install iptables-persistent to save rules across reboots
DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent

# Save the rules permanently
netfilter-persistent save

# Verify it worked
sudo iptables -L INPUT -n | grep 30333
# Should show: ACCEPT tcp -- 0.0.0.0/0 0.0.0.0/0 tcp dpt:30333
```

### Step 2: System Updates

```bash
apt-get update && apt-get upgrade -y
apt-get install -y curl wget git build-essential libssl-dev pkg-config
```

### Step 3: Configure Optional Ports

```bash
# Prometheus metrics (optional)
sudo iptables -I INPUT 1 -p tcp --dport 9615 -m comment --comment "Prometheus" -j ACCEPT

# WebSocket RPC (if needed)
sudo iptables -I INPUT 1 -p tcp --dport 9944 -m comment --comment "WS RPC" -j ACCEPT

# HTTP RPC (if needed)
sudo iptables -I INPUT 1 -p tcp --dport 9933 -m comment --comment "HTTP RPC" -j ACCEPT

# Save rules
netfilter-persistent save
```

### Common Contabo Issues

❌ **0 peers / low peer count**
- **Cause:** Firewall blocking port 30333
- **Fix:** Run Step 1 above

❌ **NetworkKeyNotFound error**
- **Cause:** Missing network key file
- **Fix:** See [Common Configuration](#common-configuration) section

---

## Oracle Cloud Setup

### Prerequisites

- Oracle Cloud account (free tier available)
- Compute instance: VM.Standard.E2.1.Micro or higher
- Ubuntu 22.04 or 24.04
- SSH key pair added to instance

### Step 1: Configure Network Security Group (NSG)

**Method A: Via Oracle Cloud Console**

1. Log into Oracle Cloud Console
2. Navigate to: **Networking → Virtual Cloud Networks**
3. Select your VCN → **Security Lists** → **Default Security List**
4. Click **Add Ingress Rules**

Add these rules:

| Source CIDR | Protocol | Destination Port | Description |
|-------------|----------|------------------|-------------|
| 0.0.0.0/0 | TCP | 30333 | FlareChain P2P |
| 0.0.0.0/0 | TCP | 9615 | Prometheus (optional) |
| 0.0.0.0/0 | TCP | 9944 | WebSocket RPC (optional) |

5. Click **Add Ingress Rules**

**Method B: Via OCI CLI**

```bash
# Install OCI CLI
bash -c "$(curl -L https://raw.githubusercontent.com/oracle/oci-cli/master/scripts/install/install.sh)"

# Configure
oci setup config

# Add ingress rule for port 30333
oci network security-list update \
  --security-list-id <YOUR_SECURITY_LIST_OCID> \
  --ingress-security-rules '[
    {
      "source": "0.0.0.0/0",
      "protocol": "6",
      "tcpOptions": {
        "destinationPortRange": {
          "min": 30333,
          "max": 30333
        }
      }
    }
  ]'
```

### Step 2: Configure Ubuntu Firewall (Instance-Level)

```bash
# SSH to Oracle Cloud instance
ssh -i ~/.ssh/oracle_key ubuntu@YOUR_ORACLE_IP

# Oracle Cloud often uses iptables - ensure port 30333 is open
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo netfilter-persistent save
```

### Step 3: Verify Connectivity

```bash
# From your local machine, test port 30333
nc -zv YOUR_ORACLE_IP 30333

# Should show: Connection to YOUR_ORACLE_IP 30333 port [tcp/*] succeeded!
```

### Common Oracle Cloud Issues

❌ **Firewall still blocking after NSG changes**
- **Cause:** Instance-level iptables also blocking
- **Fix:** Run Step 2 above

❌ **NSG rules not applying**
- **Cause:** Wrong Security List attached to subnet
- **Fix:** Verify Security List is attached to the subnet your instance is in

---

## Azure Setup

### Prerequisites

- Azure subscription
- VM: Standard_B2s or higher (2 vCPU, 4GB RAM)
- Ubuntu 22.04 or 24.04
- SSH key configured

### Step 1: Configure Network Security Group (NSG)

**Method A: Via Azure Portal**

1. Log into Azure Portal
2. Navigate to: **Virtual machines** → Select your VM
3. Click **Networking** → **Network settings**
4. Click **Create port rule** → **Inbound port rule**

Add rule:
- **Source:** Any
- **Source port ranges:** *
- **Destination:** Any
- **Service:** Custom
- **Destination port ranges:** 30333
- **Protocol:** TCP
- **Action:** Allow
- **Priority:** 1000
- **Name:** FlareChain-P2P

5. Click **Add**

Repeat for optional ports (9615, 9944, 9933)

**Method B: Via Azure CLI**

```bash
# Install Azure CLI
curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash

# Login
az login

# Add NSG rule
az network nsg rule create \
  --resource-group YOUR_RESOURCE_GROUP \
  --nsg-name YOUR_NSG_NAME \
  --name FlareChain-P2P \
  --protocol tcp \
  --priority 1000 \
  --destination-port-range 30333 \
  --access allow \
  --direction inbound
```

### Step 2: Verify on VM

```bash
# SSH to Azure VM
ssh -i ~/.ssh/azure_key azureuser@YOUR_AZURE_IP

# No additional firewall config needed - Azure VMs typically don't have UFW enabled
# But verify just in case
sudo ufw status
# Should show: Status: inactive

# If active, allow port 30333
sudo ufw allow 30333/tcp
```

### Common Azure Issues

❌ **NSG rule created but port still blocked**
- **Cause:** Rule priority too low (lower numbers = higher priority)
- **Fix:** Set priority to 1000 or lower

❌ **Multiple NSGs applied**
- **Cause:** NSG on subnet AND NIC
- **Fix:** Check both NSGs and ensure port 30333 is allowed in both

---

## DigitalOcean Setup

### Prerequisites

- DigitalOcean account
- Droplet: Basic plan with 2GB RAM minimum
- Ubuntu 22.04 or 24.04
- SSH key configured

### Step 1: Configure Cloud Firewall

**Method A: Via DigitalOcean Console**

1. Log into DigitalOcean
2. Navigate to: **Networking** → **Firewalls**
3. Click **Create Firewall**
4. Add **Inbound Rule:**
   - **Type:** Custom
   - **Protocol:** TCP
   - **Port Range:** 30333
   - **Sources:** All IPv4, All IPv6
5. Apply to your Droplet
6. Click **Create Firewall**

**Method B: Via doctl CLI**

```bash
# Install doctl
snap install doctl
doctl auth init

# Create firewall
doctl compute firewall create \
  --name flarechain-validator \
  --inbound-rules "protocol:tcp,ports:30333,address:0.0.0.0/0,address:::/0" \
  --droplet-ids YOUR_DROPLET_ID
```

### Step 2: Configure UFW (if enabled)

```bash
# SSH to DigitalOcean Droplet
ssh -i ~/.ssh/do_key root@YOUR_DO_IP

# Check UFW status
sudo ufw status

# If enabled, allow port 30333
sudo ufw allow 30333/tcp
sudo ufw reload
```

### Common DigitalOcean Issues

❌ **Cloud Firewall not applying**
- **Cause:** Firewall not attached to Droplet
- **Fix:** In Firewalls section, ensure your Droplet is listed under "Droplets"

---

## AWS Setup

### Prerequisites

- AWS account
- EC2 instance: t3.small or higher
- Ubuntu 22.04 or 24.04 AMI
- SSH key pair created

### Step 1: Configure Security Group

**Method A: Via AWS Console**

1. Log into AWS Console
2. Navigate to: **EC2** → **Security Groups**
3. Select your instance's security group
4. Click **Inbound rules** → **Edit inbound rules**
5. Click **Add rule**
   - **Type:** Custom TCP
   - **Port range:** 30333
   - **Source:** 0.0.0.0/0
   - **Description:** FlareChain P2P
6. Click **Save rules**

**Method B: Via AWS CLI**

```bash
# Install AWS CLI
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install

# Configure
aws configure

# Add security group rule
aws ec2 authorize-security-group-ingress \
  --group-id YOUR_SECURITY_GROUP_ID \
  --protocol tcp \
  --port 30333 \
  --cidr 0.0.0.0/0
```

### Step 2: Verify on EC2 Instance

```bash
# SSH to EC2 instance
ssh -i ~/.ssh/aws_key.pem ubuntu@YOUR_AWS_IP

# AWS instances typically don't have UFW enabled
sudo ufw status
# Should show: Status: inactive
```

### Common AWS Issues

❌ **Security Group rule not working**
- **Cause:** Wrong security group attached to instance
- **Fix:** Verify instance's security group: `EC2 → Instances → Select instance → Security tab`

---

## Common Configuration

After configuring your cloud provider's firewall, follow these steps on the VM:

### 1. Create Directory Structure

```bash
sudo mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
sudo mkdir -p /var/lib/etrid/chains/flarechain_mainnet/keystore
sudo mkdir -p /usr/local/bin
```

### 2. Deploy FlareChain Binary

**Option A: Download from GitHub releases**
```bash
wget https://github.com/etrid/flarechain/releases/latest/download/flarechain-node
sudo mv flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node
```

**Option B: Copy from existing validator**
```bash
# From your local machine
scp -i ~/.ssh/your_key /path/to/flarechain-node root@YOUR_VM_IP:/usr/local/bin/
ssh -i ~/.ssh/your_key root@YOUR_VM_IP "chmod +x /usr/local/bin/flarechain-node"
```

**Option C: Build from source** (see [Build Guides](../build-guides/))

### 3. Deploy Chainspec

```bash
# Download mainnet chainspec
wget https://raw.githubusercontent.com/etrid/flarechain/main/chainspec-mainnet-raw-FIXED.json -O /var/lib/etrid/chainspec-mainnet-raw-FIXED.json

# Verify genesis hash
grep '"genesis":' /var/lib/etrid/chainspec-mainnet-raw-FIXED.json | grep "0xca40"
```

### 4. Generate Network Key

```bash
# Generate Ed25519 network key
NETWORK_KEY=$(openssl rand -hex 32)
echo -n "$NETWORK_KEY" | sudo tee /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
sudo chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
```

### 5. Generate Session Keys

```bash
# AURA key (sr25519)
/usr/local/bin/flarechain-node key generate --scheme sr25519 --output-type json > /tmp/aura_key.json
AURA_SECRET=$(cat /tmp/aura_key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)

/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type aura \
    --scheme sr25519 \
    --suri "$AURA_SECRET"

# GRANDPA key (ed25519)
/usr/local/bin/flarechain-node key generate --scheme ed25519 --output-type json > /tmp/gran_key.json
GRAN_SECRET=$(cat /tmp/gran_key.json | grep -o '"secretPhrase":"[^"]*"' | cut -d'"' -f4)

/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type gran \
    --scheme ed25519 \
    --suri "$GRAN_SECRET"

# ASF key (uses same as AURA)
/usr/local/bin/flarechain-node key insert \
    --base-path /var/lib/etrid \
    --chain /var/lib/etrid/chainspec-mainnet-raw-FIXED.json \
    --key-type asfk \
    --scheme sr25519 \
    --suri "$AURA_SECRET"

# Save keys for backup
echo "VALIDATOR-XX Session Keys" | sudo tee /root/validator_keys.txt
echo "AURA/ASF: $AURA_SECRET" | sudo tee -a /root/validator_keys.txt
echo "GRANDPA: $GRAN_SECRET" | sudo tee -a /root/validator_keys.txt
```

**⚠️ CRITICAL:** Back up these mnemonics securely!

### 6. Create Systemd Service

```bash
sudo bash -c 'cat > /etc/systemd/system/flarechain-validator.service << "EOFSERVICE"
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
    --name "VALIDATOR_NAME_HERE" \
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
EOFSERVICE'

# Update placeholders
sudo sed -i "s/VALIDATOR_NAME_HERE/Validator-YourName/g" /etc/systemd/system/flarechain-validator.service
sudo sed -i "s/YOUR_VM_IP/$(curl -s ifconfig.me)/g" /etc/systemd/system/flarechain-validator.service
```

### 7. Start Validator

```bash
sudo systemctl daemon-reload
sudo systemctl enable flarechain-validator
sudo systemctl start flarechain-validator
```

### 8. Verify

```bash
# Check service status
sudo systemctl status flarechain-validator

# Watch logs
sudo journalctl -u flarechain-validator -f

# Check peer count (wait 1-2 minutes)
sudo journalctl -u flarechain-validator -n 20 | grep peers

# Expected: "X peers" where X >= 5
```

---

## Troubleshooting

### Universal Issues (All Providers)

#### 0 Peers / Low Peer Count

**Symptoms:**
- Logs show "0 peers" or "1-2 peers"
- Not syncing blocks

**Diagnosis:**
```bash
# Check if port 30333 is listening
sudo netstat -tlnp | grep 30333

# Check firewall rules
sudo iptables -L -n | grep 30333  # Linux
sudo ufw status                    # Ubuntu with UFW
```

**Fix:**
1. Verify cloud provider firewall allows port 30333 (see provider-specific sections above)
2. Verify instance-level firewall allows port 30333
3. Restart validator: `sudo systemctl restart flarechain-validator`
4. Wait 2-5 minutes and check again

#### NetworkKeyNotFound Error

**Symptoms:**
- Logs show "NetworkKeyNotFound"
- Validator fails to start

**Fix:**
```bash
# Generate network key
NETWORK_KEY=$(openssl rand -hex 32)
sudo mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
echo -n "$NETWORK_KEY" | sudo tee /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
sudo chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519

# Restart
sudo systemctl restart flarechain-validator
```

#### Wrong Genesis Hash

**Symptoms:**
- Syncing to different chain
- Genesis hash not `0xca40...4da8`

**Fix:**
```bash
# Stop validator
sudo systemctl stop flarechain-validator

# Download correct chainspec
wget https://raw.githubusercontent.com/etrid/flarechain/main/chainspec-mainnet-raw-FIXED.json -O /var/lib/etrid/chainspec-mainnet-raw-FIXED.json

# Clear database
sudo rm -rf /var/lib/etrid/chains/flarechain_mainnet/db

# Start validator
sudo systemctl start flarechain-validator
```

#### Service Won't Start

**Diagnosis:**
```bash
# Check service status
sudo systemctl status flarechain-validator

# Check detailed logs
sudo journalctl -u flarechain-validator -n 50 --no-pager
```

**Common causes:**
- Binary not executable: `sudo chmod +x /usr/local/bin/flarechain-node`
- Missing chainspec: Download as shown in [Common Configuration](#common-configuration)
- Wrong file paths in systemd service
- Port already in use: `sudo netstat -tlnp | grep 30333`

---

## Monitoring

### Check Validator Health

```bash
# Peer count
sudo journalctl -u flarechain-validator -n 5 | grep peers

# Sync status
sudo journalctl -u flarechain-validator -n 5 | grep Syncing

# Block height
sudo journalctl -u flarechain-validator -n 10 | grep "Imported"
```

### Expected Peer Count Timeline

- **0-5 mins:** 0-3 peers (network discovery)
- **5-30 mins:** 5-10 peers (connecting to bootnodes)
- **30+ mins:** 10-20 peers (full P2P mesh established)

### Prometheus Metrics (if enabled)

```bash
# Access metrics endpoint
curl http://YOUR_VM_IP:9615/metrics

# Check specific metrics
curl -s http://YOUR_VM_IP:9615/metrics | grep substrate_sub_libp2p_peers_count
```

---

## Provider-Specific Cost Optimization

### Contabo
- Use VPS-S ($6/month) for single validator
- Disable backups if not needed
- Annual payment for discount

### Oracle Cloud
- Use Always Free tier: VM.Standard.E2.1.Micro
- 2x free instances per account
- Block storage: 200GB free

### Azure
- Use B-series (B2s) for cost efficiency
- Reserved instances for 1-year commitment
- Auto-shutdown during low activity (if not validating)

### DigitalOcean
- Basic Droplet: $12/month minimum
- Use Spaces for backups instead of snapshots
- Enable monitoring (free)

### AWS
- Use t3.small with Reserved Instance pricing
- Spot instances NOT recommended (can be terminated)
- Free tier: 750 hours/month for 12 months

---

## Security Best Practices

1. **SSH Key Only** - Disable password authentication
2. **Firewall** - Only open required ports (30333, 9615)
3. **Updates** - Keep system packages updated
4. **Backups** - Back up session keys and network key
5. **Monitoring** - Set up alerts for validator downtime
6. **DDoS Protection** - Use cloud provider's DDoS mitigation
7. **Rate Limiting** - Configure connection limits in FlareChain

---

## Next Steps

After deployment:

1. ✅ Verify validator is syncing and has peers
2. 📊 Set up monitoring (Prometheus + Grafana)
3. 🔐 Back up session keys securely
4. 📝 Document your validator in network map
5. 💰 Fund validator account for transactions
6. 🎯 Submit validator registration transaction
7. 🚀 Start validating!

---

## Support

- **Documentation:** https://etrid.org/docs
- **Discord:** https://discord.gg/etrid
- **GitHub Issues:** https://github.com/etrid/flarechain/issues

---

**Last Updated:** November 9, 2025
**Maintainer:** FlareChain Core Team
