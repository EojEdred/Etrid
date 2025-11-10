# FlareChain Validator - Quick Reference Card

**📍 Location:** `/Users/macbook/Desktop/etrid/docs/deployment-master/`

---

## 🚀 Deploy New Validator (3 Options)

### Option 1: One-Command (Contabo - Fastest)
```bash
cd ~/Desktop/etrid
./docs/deployment-master/scripts/deploy-new-contabo-validator.sh <num> <ip> "<name>"
```
⏱️ **Time:** 10-15 minutes

---

### Option 2: Step-by-Step (All Providers)
```bash
cat docs/deployment-master/onboarding/VALIDATOR_QUICK_START.md
```
⏱️ **Time:** 15-30 minutes

---

### Option 3: Build from Source
```bash
# Linux
cat docs/deployment-master/build-guides/BUILD_GUIDE_LINUX.md

# macOS
cat docs/deployment-master/build-guides/BUILD_GUIDE_MACOS.md
```
⏱️ **Time:** 45-90 minutes

---

## 🔧 Common Fixes

### Fix: 0 Peers
```bash
sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT
sudo netfilter-persistent save
sudo systemctl restart flarechain-validator
```

### Fix: Service Won't Start
```bash
# Check logs
sudo journalctl -u flarechain-validator -n 50

# Make binary executable
sudo chmod +x /usr/local/bin/flarechain-node
```

### Fix: Wrong Genesis
```bash
sudo systemctl stop flarechain-validator
wget https://raw.githubusercontent.com/etrid/flarechain/main/chainspec-mainnet-raw-FIXED.json \
  -O /var/lib/etrid/chainspec-mainnet-raw-FIXED.json
sudo rm -rf /var/lib/etrid/chains/flarechain_mainnet/db
sudo systemctl start flarechain-validator
```

---

## 📊 Health Check

```bash
# Quick check
sudo systemctl status flarechain-validator
sudo journalctl -u flarechain-validator -n 5 | grep peers

# Full health check
~/Desktop/etrid/docs/deployment-master/scripts/check-all-validators-health.sh
```

---

## 📖 Documentation Navigation

| What You Need | File |
|---------------|------|
| **Main hub** | [README.md](README.md) |
| **Quick start** | [onboarding/VALIDATOR_QUICK_START.md](onboarding/VALIDATOR_QUICK_START.md) |
| **Cloud setup** | [cloud-providers/CLOUD_PROVIDER_SETUP_MASTER.md](cloud-providers/CLOUD_PROVIDER_SETUP_MASTER.md) |
| **Linux build** | [build-guides/BUILD_GUIDE_LINUX.md](build-guides/BUILD_GUIDE_LINUX.md) |
| **macOS build** | [build-guides/BUILD_GUIDE_MACOS.md](build-guides/BUILD_GUIDE_MACOS.md) |
| **Troubleshooting** | [troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md](troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md) |

---

## 💰 Cloud Provider Costs

| Provider | Cost/Month | Free Tier |
|----------|------------|-----------|
| Contabo | $6-12 | No |
| Oracle Cloud | $0 | 2 VMs free |
| DigitalOcean | $12-24 | $200/60d |
| Azure | $30-60 | $200/30d |
| AWS | $20-50 | 750h/12mo |

---

## 🔑 Required Ports

- **30333** - P2P networking (required)
- **9615** - Prometheus metrics (optional)
- **9944** - WebSocket RPC (optional, close to public)

---

## ⚙️ System Requirements

**Minimum:**
- 2 vCPU, 4GB RAM, 100GB SSD

**Recommended:**
- 4 vCPU, 8GB RAM, 200GB NVMe

---

## 🆘 Support

- **Troubleshooting:** [DEPLOYMENT_TROUBLESHOOTING.md](troubleshooting/DEPLOYMENT_TROUBLESHOOTING.md)
- **Discord:** #validator-support
- **GitHub:** https://github.com/etrid/flarechain/issues
- **Email:** support@etrid.org

---

**Last Updated:** November 9, 2025
