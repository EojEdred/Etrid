# FlareChain Validator - Deployment Master Directory

**Complete deployment, onboarding, and troubleshooting resources for FlareChain validators**

---

## 📁 Directory Structure

```
deployment-master/
├── README.md                           # This file
├── cloud-providers/                    # Cloud provider setup guides
│   └── CLOUD_PROVIDER_SETUP_MASTER.md  # All providers: Contabo, Oracle, Azure, DO, AWS
├── build-guides/                       # Platform-specific build instructions
│   ├── BUILD_GUIDE_LINUX.md            # Ubuntu, Debian, CentOS, Fedora, Arch
│   └── BUILD_GUIDE_MACOS.md            # macOS (Intel & Apple Silicon)
├── scripts/                            # Deployment automation scripts
│   ├── deploy-new-contabo-validator.sh # One-command Contabo deployment
│   ├── deploy-systemd-services.sh      # Systemd service installer
│   └── check-all-validators-health.sh  # Health check for all validators
├── troubleshooting/                    # Comprehensive troubleshooting
│   └── DEPLOYMENT_TROUBLESHOOTING.md   # Solutions for common issues
├── onboarding/                         # Quick-start guides
│   └── VALIDATOR_QUICK_START.md        # 15-minute validator deployment
└── templates/                          # Configuration templates
    └── (systemd service templates, etc.)
```

---

## 🚀 Quick Start

### New Validator - Choose Your Path:

#### Path 1: One-Command Deployment (Contabo Only)
**Time:** 10-15 minutes

```bash
cd ~/Desktop/etrid
./docs/deployment-master/scripts/deploy-new-contabo-validator.sh <number> <ip> "<name>"
```

#### Path 2: Manual Deployment (All Providers)
**Time:** 15-30 minutes

1. Read: [Validator Quick Start](onboarding/VALIDATOR_QUICK_START.md)
2. Follow: [Cloud Provider Setup](cloud-providers/CLOUD_PROVIDER_SETUP_MASTER.md)
3. Deploy and verify

#### Path 3: Build from Source
**Time:** 45-90 minutes

1. Choose platform: [Linux](build-guides/BUILD_GUIDE_LINUX.md) or [macOS](build-guides/BUILD_GUIDE_MACOS.md)
2. Build FlareChain binary
3. Follow deployment steps

---

## 📖 Documentation Guide

### For First-Time Validators

**Start here:** [Validator Quick Start](onboarding/VALIDATOR_QUICK_START.md)

This guide covers:
- ✅ Choosing a cloud provider
- ✅ Configuring firewalls
- ✅ Installing FlareChain
- ✅ Generating session keys
- ✅ Starting your validator
- ✅ Verifying everything works

**Estimated time:** 15-30 minutes (using pre-built binary)

---

### For Cloud Provider Setup

**Go to:** [Cloud Provider Setup Master](cloud-providers/CLOUD_PROVIDER_SETUP_MASTER.md)

Detailed guides for:
- **Contabo** - Cheapest option ($6-12/month)
  - ⚠️ **CRITICAL:** Default firewall blocks all ports
  - Quick fix included
- **Oracle Cloud** - Free tier available (2 VMs free forever)
  - Dual firewall (NSG + instance iptables)
- **Azure** - Enterprise-grade reliability
  - Network Security Group configuration
- **DigitalOcean** - Easy setup
  - Cloud Firewall + UFW
- **AWS** - Industry standard
  - Security Group configuration

---

### For Building from Source

#### Linux Build
**Go to:** [Linux Build Guide](build-guides/BUILD_GUIDE_LINUX.md)

Supports:
- Ubuntu 22.04 / 24.04
- Debian 11 / 12
- CentOS 8+
- Fedora 38+
- Arch Linux

**Build time:** 30-60 minutes

#### macOS Build
**Go to:** [macOS Build Guide](build-guides/BUILD_GUIDE_MACOS.md)

Supports:
- macOS 12 (Monterey)
- macOS 13 (Ventura)
- macOS 14 (Sonoma)
- macOS 15 (Sequoia)
- Intel (x86_64) and Apple Silicon (arm64)

**Build time:** 20-45 minutes

---

### When Things Go Wrong

**Go to:** [Deployment Troubleshooting](troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md)

Common issues solved:
- ❌ 0 peers / low peer count → Firewall blocking port 30333
- ❌ NetworkKeyNotFound error → Missing network key
- ❌ Service won't start → Binary/chainspec issues
- ❌ Not syncing blocks → Wrong genesis hash
- ❌ High CPU/memory usage → Resource optimization
- ❌ Cloud-specific issues → Provider-specific fixes

---

## 🛠️ Deployment Scripts

All scripts are located in: `scripts/`

### deploy-new-contabo-validator.sh
**Purpose:** One-command deployment for Contabo VMs

**Usage:**
```bash
./scripts/deploy-new-contabo-validator.sh <validator_number> <ip_address> "<validator_name>"

# Example:
./scripts/deploy-new-contabo-validator.sh 26 157.173.200.100 "validator-seattle-01"
```

**What it does:**
1. Opens port 30333 (fixes Contabo firewall issue)
2. Installs iptables-persistent
3. Creates directory structure
4. Deploys binary and chainspec
5. Generates unique session keys (AURA, GRANDPA, ASF)
6. Generates network key
7. Creates and starts systemd service

**Time:** 10-15 minutes

---

### check-all-validators-health.sh
**Purpose:** Health check for all deployed validators

**Usage:**
```bash
./scripts/check-all-validators-health.sh
```

**Output:**
- Service status
- Peer count
- Sync status
- Recent errors
- Network configuration

---

## 🔧 Troubleshooting Quick Reference

### Issue: 0 Peers

**Diagnosis:**
```bash
sudo iptables -L INPUT -n | grep 30333
sudo netstat -tlnp | grep 30333
```

**Fix:**
```bash
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo netfilter-persistent save
sudo systemctl restart flarechain-validator
```

---

### Issue: Service Won't Start

**Diagnosis:**
```bash
sudo systemctl status flarechain-validator
sudo journalctl -u flarechain-validator -n 50 --no-pager
```

**Common fixes:**
```bash
# Binary not executable
sudo chmod +x /usr/local/bin/flarechain-node

# Missing chainspec
wget https://raw.githubusercontent.com/etrid/flarechain/main/chainspec-mainnet-raw-FIXED.json \
  -O /var/lib/etrid/chainspec-mainnet-raw-FIXED.json
```

---

### Issue: Not Syncing

**Check genesis hash:**
```bash
grep '"genesis":' /var/lib/etrid/chainspec-mainnet-raw-FIXED.json | grep "0xca40"
```

**Reset if wrong:**
```bash
sudo systemctl stop flarechain-validator
sudo rm -rf /var/lib/etrid/chains/flarechain_mainnet/db
wget https://raw.githubusercontent.com/etrid/flarechain/main/chainspec-mainnet-raw-FIXED.json \
  -O /var/lib/etrid/chainspec-mainnet-raw-FIXED.json
sudo systemctl start flarechain-validator
```

**Full troubleshooting:** [Deployment Troubleshooting Guide](troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md)

---

## 📊 Cloud Provider Comparison

| Provider | Cost/Month | Difficulty | Free Tier | Firewall Type | Recommended For |
|----------|------------|------------|-----------|---------------|-----------------|
| **Contabo** | $6-12 | Easy | No | iptables (policy DROP) | Budget validators |
| **Oracle Cloud** | $0 | Medium | 2 VMs free | NSG + iptables | Testing, hobbyists |
| **DigitalOcean** | $12-24 | Easy | $200/60d | Cloud Firewall + UFW | Beginners |
| **Azure** | $30-60 | Medium | $200/30d | NSG | Enterprise |
| **AWS** | $20-50 | Hard | 750h/12mo | Security Groups | Advanced users |

**Best for new validators:**
- **Free:** Oracle Cloud
- **Paid:** Contabo (lowest cost)

---

## ⚙️ System Requirements

### Minimum Requirements
- **CPU:** 2 cores
- **RAM:** 4GB
- **Storage:** 100GB SSD
- **Network:** 100 Mbps
- **OS:** Ubuntu 22.04/24.04 or macOS 12+

### Recommended Requirements
- **CPU:** 4+ cores
- **RAM:** 8GB+
- **Storage:** 200GB+ NVMe SSD
- **Network:** 1 Gbps
- **OS:** Ubuntu 24.04 LTS

### For Production Validators
- **CPU:** 8+ cores
- **RAM:** 16GB+
- **Storage:** 500GB+ NVMe SSD
- **Network:** 1 Gbps with DDoS protection
- **Backup:** Automated backups of session keys
- **Monitoring:** Prometheus + Grafana

---

## 🔐 Security Best Practices

1. **SSH Key Only**
   - Disable password authentication
   - Use strong SSH keys (ed25519 or RSA 4096)

2. **Firewall Configuration**
   - Only open required ports: 30333 (required), 9615 (optional)
   - Close RPC ports (9933, 9944) to public

3. **Session Keys**
   - Back up to multiple secure locations
   - Never share session key mnemonics
   - Use hardware security modules (HSM) for production

4. **System Updates**
   - Keep OS packages updated
   - Monitor security advisories
   - Plan maintenance windows

5. **Monitoring**
   - Set up alerting for downtime
   - Monitor resource usage
   - Track peer connections

6. **DDoS Protection**
   - Use cloud provider's DDoS mitigation
   - Consider Cloudflare or similar services
   - Rate limit connections

---

## 📦 What's Included

### Documentation
- ✅ Cloud provider setup (Contabo, Oracle, Azure, DigitalOcean, AWS)
- ✅ Build guides (Linux, macOS)
- ✅ Comprehensive troubleshooting
- ✅ Quick-start onboarding
- ✅ Security best practices

### Scripts
- ✅ One-command Contabo deployment
- ✅ Systemd service installer
- ✅ Health check script
- ✅ (More scripts in development)

### Templates
- ✅ Systemd service templates
- ✅ Configuration examples
- ✅ (More templates coming soon)

---

## 🎯 Common Use Cases

### Scenario 1: Deploy First Validator (Free)
**Goal:** Test FlareChain without spending money

**Steps:**
1. Create Oracle Cloud account (free tier)
2. Provision VM.Standard.E2.1.Micro (free)
3. Follow: [Validator Quick Start](onboarding/VALIDATOR_QUICK_START.md)
4. Configure Oracle NSG for port 30333

**Cost:** $0/month (free tier)

---

### Scenario 2: Deploy Production Validator
**Goal:** Run reliable validator for mainnet

**Recommendation:** Azure B2s or AWS t3.small

**Steps:**
1. Provision VM with recommended specs
2. Configure cloud firewall
3. Build from source: [Linux Build Guide](build-guides/BUILD_GUIDE_LINUX.md)
4. Set up monitoring (Prometheus + Grafana)
5. Configure backups
6. Test failover procedures

**Cost:** $30-50/month

---

### Scenario 3: Deploy 10+ Validators (Fleet)
**Goal:** Run validator fleet across multiple regions

**Recommendation:** Contabo (cost) or DigitalOcean (ease)

**Steps:**
1. Use automation script for deployment
2. Deploy across multiple regions (US, EU, Asia)
3. Set up centralized monitoring
4. Automate updates and maintenance
5. Configure peer-to-peer mesh

**Cost:** $60-120/month (10 validators on Contabo)

---

## 🆘 Getting Help

### Self-Service Resources
1. **Troubleshooting Guide:** [DEPLOYMENT_TROUBLESHOOTING.md](troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md)
2. **FAQ:** See [Validator Quick Start - FAQ](onboarding/VALIDATOR_QUICK_START.md#faq)
3. **GitHub Issues:** Search existing issues

### Community Support
- **Discord:** https://discord.gg/etrid (#validator-support)
- **Telegram:** https://t.me/etrid
- **Forum:** https://forum.etrid.org

### Official Support
- **GitHub Issues:** https://github.com/etrid/flarechain/issues
- **Email:** support@etrid.org

**When asking for help, include:**
- Cloud provider and VM specs
- Operating system (`cat /etc/os-release`)
- FlareChain version (`flarechain-node --version`)
- Recent logs (`journalctl -u flarechain-validator -n 100`)
- Steps already tried

---

## 📅 Maintenance Schedule

### Daily
- Check validator status
- Monitor peer count
- Review error logs

### Weekly
- Check disk space
- Review performance metrics
- Verify backups

### Monthly
- Update system packages
- Review security advisories
- Update FlareChain if new release
- Test disaster recovery

### Quarterly
- Audit security configuration
- Review costs and optimize
- Update documentation
- Plan capacity upgrades

---

## 🔄 Update Procedures

### Update FlareChain Binary

```bash
# 1. Download new binary
wget https://github.com/etrid/flarechain/releases/download/vX.Y.Z/flarechain-node \
  -O /tmp/flarechain-node-new

# 2. Verify binary
/tmp/flarechain-node-new --version

# 3. Stop validator
sudo systemctl stop flarechain-validator

# 4. Backup old binary
sudo cp /usr/local/bin/flarechain-node /usr/local/bin/flarechain-node.backup

# 5. Install new binary
sudo mv /tmp/flarechain-node-new /usr/local/bin/flarechain-node
sudo chmod +x /usr/local/bin/flarechain-node

# 6. Start validator
sudo systemctl start flarechain-validator

# 7. Verify
sudo journalctl -u flarechain-validator -f
```

---

## 📈 Performance Monitoring

### Key Metrics to Monitor

- **Peer Count:** Should be >= 10
- **Block Height:** Should increase continuously
- **Finality:** Blocks should finalize
- **CPU Usage:** < 80% average
- **Memory Usage:** < 80%
- **Disk Usage:** < 80%
- **Network I/O:** Consistent traffic

### Monitoring Tools

- **Prometheus:** Metrics collection (port 9615)
- **Grafana:** Visualization dashboards
- **Alertmanager:** Alert notifications
- **Uptime monitors:** External monitoring

See: [Monitoring Guide](../MONITORING_GUIDE.md) (coming soon)

---

## 🎓 Learning Resources

- **FlareChain Docs:** https://docs.etrid.org
- **Substrate Docs:** https://docs.substrate.io
- **Polkadot Wiki:** https://wiki.polkadot.network
- **Rust Book:** https://doc.rust-lang.org/book/

---

## 🤝 Contributing

Found an issue or want to improve these docs?

1. Fork: https://github.com/etrid/flarechain
2. Create branch: `git checkout -b improve-deployment-docs`
3. Make changes
4. Submit PR

Or open an issue: https://github.com/etrid/flarechain/issues/new

---

## 📜 License

All documentation in this directory is licensed under CC BY 4.0.

Scripts are licensed under the same license as FlareChain (GPL-3.0).

---

**Last Updated:** November 9, 2025
**Maintainer:** FlareChain Core Team
**Contributors:** Community contributors welcome!

---

**Ready to deploy your validator?**

Start here: [Validator Quick Start](onboarding/VALIDATOR_QUICK_START.md)

**Questions?**

Join us on Discord: https://discord.gg/etrid
