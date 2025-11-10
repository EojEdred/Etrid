# Master VM Provider Setup Guide

**Universal guide for deploying FlareChain validators on any cloud provider**

This guide covers deployment across all major cloud platforms. Choose your provider below or follow the generic Linux instructions for any VPS.

---

## 📋 Table of Contents

1. [General Requirements](#general-requirements)
2. [Provider-Specific Guides](#provider-specific-guides)
3. [Critical Firewall Configuration](#critical-firewall-configuration)
4. [Common Setup Steps](#common-setup-steps)
5. [Provider Comparison](#provider-comparison)

---

## General Requirements

### Minimum Hardware Requirements
- **CPU:** 2+ cores (4+ cores recommended for Directors)
- **RAM:** 4GB minimum (8GB+ recommended)
- **Storage:** 100GB SSD (200GB+ recommended for future growth)
- **Network:** 100 Mbps+ bandwidth
- **OS:** Ubuntu 20.04/22.04 LTS (recommended) or any modern Linux

### Required Ports
- **30333** (TCP) - **CRITICAL:** P2P networking (MUST be open)
- **9615** (TCP) - Prometheus metrics (optional)
- **9944** (TCP) - WebSocket RPC (optional, for local access)
- **9933** (TCP) - HTTP RPC (optional, for local access)

### Required Software
- `curl`, `wget`, `git`
- `build-essential` or equivalent
- `libssl-dev`, `pkg-config`
- Rust toolchain (if building from source)

---

## Provider-Specific Guides

### [Contabo](./CONTABO_SETUP.md)
- **Pros:** Extremely cost-effective ($6-8/month), good bandwidth
- **Cons:** **CRITICAL:** Default firewall blocks ALL ports (policy DROP)
- **Locations:** Seattle, New York, St. Louis, UK, Germany
- **Action Required:** ⚠️ **MUST open port 30333 immediately after provisioning**

**Quick Deploy:**
```bash
./docs/validator-deployment/scripts/deploy-new-validator.sh <num> <ip> "<name>" --provider contabo
```

[→ See Contabo-specific guide](./CONTABO_SETUP.md)

---

### [Oracle Cloud](./ORACLE_CLOUD_SETUP.md)
- **Pros:** Free tier available (2 VMs with 1GB RAM each), generous network
- **Cons:** Free tier limited resources, requires credit card
- **Locations:** Multiple worldwide
- **Action Required:** Configure Network Security Groups for port 30333

**Firewall:**
```bash
# On the VM
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo apt-get install -y iptables-persistent
sudo netfilter-persistent save

# In Oracle Cloud Console
# → Virtual Cloud Networks → Security Lists → Add Ingress Rule
# Source: 0.0.0.0/0, Protocol: TCP, Port: 30333
```

[→ See Oracle Cloud-specific guide](./ORACLE_CLOUD_SETUP.md)

---

### [Azure](./AZURE_SETUP.md)
- **Pros:** Enterprise-grade, excellent reliability, global network
- **Cons:** Higher cost (~$30-50/month), complex networking
- **Locations:** 60+ regions worldwide
- **Action Required:** Configure Network Security Group (NSG) for port 30333

**Firewall:**
```bash
# Via Azure CLI
az network nsg rule create \
  --resource-group <resource-group> \
  --nsg-name <nsg-name> \
  --name Allow-FlareChain-P2P \
  --priority 100 \
  --source-address-prefixes '*' \
  --destination-port-ranges 30333 \
  --protocol Tcp \
  --access Allow

# On the VM
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
```

[→ See Azure-specific guide](./AZURE_SETUP.md)

---

### [DigitalOcean](./DIGITALOCEAN_SETUP.md)
- **Pros:** Developer-friendly, simple interface, predictable pricing
- **Cons:** Moderate cost (~$18-24/month)
- **Locations:** 14+ datacenters worldwide
- **Action Required:** Configure Cloud Firewall or VM firewall for port 30333

**Firewall:**
```bash
# If using DigitalOcean Cloud Firewall (recommended)
# → Networking → Firewalls → Add Inbound Rule
# Type: Custom, Protocol: TCP, Port: 30333, Sources: All IPv4, All IPv6

# On the VM (if not using Cloud Firewall)
sudo ufw allow 30333/tcp
sudo ufw enable
```

[→ See DigitalOcean-specific guide](./DIGITALOCEAN_SETUP.md)

---

### [AWS (Amazon Web Services)](./AWS_SETUP.md)
- **Pros:** Maximum flexibility, global infrastructure, powerful features
- **Cons:** Complex pricing, steep learning curve
- **Locations:** 30+ regions worldwide
- **Action Required:** Configure Security Group for port 30333

**Firewall:**
```bash
# Via AWS CLI
aws ec2 authorize-security-group-ingress \
  --group-id <security-group-id> \
  --protocol tcp \
  --port 30333 \
  --cidr 0.0.0.0/0

# On the VM
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
```

[→ See AWS-specific guide](./AWS_SETUP.md)

---

### [Generic Linux VPS](./GENERIC_LINUX_SETUP.md)
**Works with:** Linode, Vultr, Hetzner, OVH, or any Linux VPS

**Basic firewall setup:**
```bash
# Using iptables
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo apt-get install -y iptables-persistent
sudo netfilter-persistent save

# OR using ufw
sudo ufw allow 30333/tcp
sudo ufw enable
```

[→ See Generic Linux guide](./GENERIC_LINUX_SETUP.md)

---

## Critical Firewall Configuration

### ⚠️ Port 30333 MUST Be Open

**All validators require port 30333 (TCP) open for incoming connections.**

Without this, your validator will:
- Have 0 peers
- Never sync blocks
- Not participate in consensus
- Waste resources running but not functioning

### Firewall Default Policies by Provider

| Provider | Default Policy | Behavior | Action Required |
|----------|---------------|----------|-----------------|
| **Contabo** | `DROP` | Blocks ALL incoming | ⚠️ **CRITICAL: Open port 30333 first** |
| **Oracle Cloud** | `ACCEPT` (VM) | Allows most | ✅ Configure NSG in console |
| **Azure** | `ACCEPT` (VM) | Allows most | ✅ Configure NSG in console |
| **AWS** | `ACCEPT` (VM) | Allows most | ✅ Configure Security Group |
| **DigitalOcean** | `ACCEPT` | Allows all | ✅ Optional: Use Cloud Firewall |
| **Linode/Vultr** | `ACCEPT` | Allows all | ✅ Configure VM firewall |

### Testing Port 30333

**From another machine:**
```bash
nc -zv <your-vm-ip> 30333
```

**Expected output if open:**
```
Connection to <ip> 30333 port [tcp/*] succeeded!
```

**Expected output if blocked:**
```
nc: connect to <ip> port 30333 (tcp) failed: Connection refused
```

---

## Common Setup Steps

### 1. Create VM

**Recommended specs:**
```
OS: Ubuntu 22.04 LTS
CPU: 4 cores
RAM: 8 GB
Storage: 200 GB SSD
```

### 2. Open Firewall (Port 30333)

**See provider-specific guide above for your platform.**

### 3. SSH into VM

```bash
ssh user@<your-vm-ip>
```

### 4. Update System

```bash
sudo apt-get update && sudo apt-get upgrade -y
```

### 5. Install Dependencies

```bash
sudo apt-get install -y curl wget git build-essential libssl-dev pkg-config
```

### 6. Deploy Using Automated Script

```bash
# On your local machine (not the VM)
cd ~/Desktop/etrid
./docs/validator-deployment/scripts/deploy-new-validator.sh <number> <ip> "<name>" --provider <provider>
```

The script will:
1. ✅ Open port 30333 (if not already open)
2. ✅ Install iptables-persistent (for persistence)
3. ✅ Create directory structure
4. ✅ Deploy binary and chainspec
5. ✅ Generate unique session keys
6. ✅ Generate network key
7. ✅ Create systemd service
8. ✅ Start validator

### 7. Verify Deployment

```bash
# Check service status
ssh user@<your-vm-ip> 'systemctl status flarechain-validator'

# Check peer count (wait 30 seconds first)
ssh user@<your-vm-ip> 'journalctl -u flarechain-validator -n 5 | grep peers'

# Monitor logs
ssh user@<your-vm-ip> 'journalctl -u flarechain-validator -f'
```

---

## Provider Comparison

### Cost Comparison (Monthly)

| Provider | 2 CPU / 4GB RAM | 4 CPU / 8GB RAM | Free Tier |
|----------|-----------------|-----------------|-----------|
| **Contabo** | $6-8 | $12-15 | No |
| **Oracle Cloud** | $0 (free tier) | $0-15 | Yes (2x 1GB VMs) |
| **DigitalOcean** | $18 | $36 | No (but $200 credit) |
| **Linode** | $18 | $36 | No (but $100 credit) |
| **Vultr** | $18 | $36 | No (but $100 credit) |
| **Azure** | $30-40 | $60-80 | Yes ($200 credit) |
| **AWS** | $25-35 | $50-70 | Yes (12 months limited) |
| **Hetzner** | €5-7 | €12-15 | No |

### Performance Comparison

| Provider | Network Speed | Reliability | Support | Best For |
|----------|--------------|-------------|---------|----------|
| **Contabo** | Very Good | Good | Email | Budget validators |
| **Oracle Cloud** | Excellent | Very Good | Good | Free tier testing |
| **DigitalOcean** | Excellent | Excellent | Excellent | Developers |
| **Linode** | Excellent | Excellent | Excellent | Simplicity |
| **Vultr** | Excellent | Very Good | Good | Global reach |
| **Azure** | Excellent | Excellent | Enterprise | Production |
| **AWS** | Excellent | Excellent | Enterprise | Production |
| **Hetzner** | Excellent | Very Good | Good | EU validators |

### Recommended Providers

**For Budget Validators:**
1. **Contabo** - Best price/performance ratio
2. **Hetzner** - European budget option
3. **Oracle Cloud** - Free tier for testing

**For Production Validators:**
1. **Azure** - Enterprise reliability
2. **AWS** - Maximum features
3. **DigitalOcean** - Balance of simplicity and power

**For Developers:**
1. **DigitalOcean** - Simple interface
2. **Linode** - Documentation and community
3. **Vultr** - Quick deployment

---

## Next Steps

1. Choose your provider from the list above
2. Follow the provider-specific setup guide
3. Use the automated deployment script
4. Verify your validator is running and peering
5. Save your session keys to the master secrets file

---

**Need help?** See [Troubleshooting Guide](../troubleshooting/COMMON_ISSUES.md)

**Last Updated:** November 9, 2025
