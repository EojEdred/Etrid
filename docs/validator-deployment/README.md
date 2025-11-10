# FlareChain Validator Deployment Hub

**Complete guide for deploying and managing FlareChain mainnet validators**

This directory contains all documentation, scripts, and troubleshooting guides needed for anyone to join the FlareChain mainnet as a validator with minimal manual intervention.

---

## 📋 Quick Navigation

### For New Validators (START HERE)
- **[Quick Start Guide](./guides/QUICK_START_NEW_VALIDATOR.md)** - One-command deployment
- **[Complete Setup Checklist](./guides/COMPLETE_SETUP_CHECKLIST.md)** - Manual step-by-step guide
- **[Onboarding Guide](./guides/VALIDATOR_ONBOARDING.md)** - What you need to know before starting

### By Cloud Provider
- **[Contabo](./vm-providers/CONTABO_SETUP.md)** - Seattle, New York, St. Louis datacenters
- **[Oracle Cloud](./vm-providers/ORACLE_CLOUD_SETUP.md)** - Free tier available
- **[Azure](./vm-providers/AZURE_SETUP.md)** - Enterprise-grade
- **[DigitalOcean](./vm-providers/DIGITALOCEAN_SETUP.md)** - Developer-friendly
- **[AWS](./vm-providers/AWS_SETUP.md)** - Maximum flexibility
- **[Any Linux VPS](./vm-providers/GENERIC_LINUX_SETUP.md)** - Universal instructions

### Build From Source
- **[Linux Build Guide](./build-instructions/BUILD_LINUX.md)** - Ubuntu, Debian, CentOS, Arch
- **[macOS Build Guide](./build-instructions/BUILD_MACOS.md)** - Intel and Apple Silicon
- **[Windows WSL Build Guide](./build-instructions/BUILD_WINDOWS_WSL.md)** - Windows Subsystem for Linux
- **[Docker Build](./build-instructions/BUILD_DOCKER.md)** - Containerized builds

### Troubleshooting
- **[Common Issues](./troubleshooting/COMMON_ISSUES.md)** - Quick fixes for frequent problems
- **[Firewall Issues](./troubleshooting/FIREWALL_TROUBLESHOOTING.md)** - Port 30333 connectivity
- **[Peer Discovery](./troubleshooting/PEER_DISCOVERY.md)** - Low peer count issues
- **[Session Keys](./troubleshooting/SESSION_KEY_ISSUES.md)** - Key generation and management
- **[Sync Issues](./troubleshooting/SYNC_TROUBLESHOOTING.md)** - Block sync problems
- **[Performance Tuning](./troubleshooting/PERFORMANCE_TUNING.md)** - Optimization guide

### Automation Scripts
- **[deploy-new-validator.sh](./scripts/deploy-new-validator.sh)** - Automated deployment
- **[open-firewall.sh](./scripts/open-firewall.sh)** - Fix firewall issues
- **[generate-keys.sh](./scripts/generate-keys.sh)** - Session key generation
- **[health-check.sh](./scripts/health-check.sh)** - Validator health monitoring
- **[peer-report.sh](./scripts/peer-report.sh)** - Network connectivity report

---

## 🚀 Fastest Path to Deployment

### Option 1: Automated (Recommended)

**For Contabo VMs:**
```bash
cd ~/Desktop/etrid
./docs/validator-deployment/scripts/deploy-new-validator.sh <number> <ip> "<name>"
```

**For Other Providers:**
```bash
cd ~/Desktop/etrid
./docs/validator-deployment/scripts/deploy-new-validator.sh <number> <ip> "<name>" --provider <oracle|azure|digitalocean|aws>
```

### Option 2: Manual Deployment

Follow the [Complete Setup Checklist](./guides/COMPLETE_SETUP_CHECKLIST.md)

---

## 📊 Current Network Status

**Mainnet Genesis:** `0xca40...4da8`
**Total Validators:** 25 (9 Directors + 16 Validity Nodes)
**Bootnodes:**
- Gizzi: `/ip4/64.181.215.19/tcp/30333/p2p/12D3KooWPyfp2DECPKTmJ1AhxB6midHnp7wYTP15vBAxbTewxaq1`
- Val-6: `/ip4/85.239.239.194/tcp/30333/p2p/12D3KooWSrYpSQ6SiDR3uduqbiepyfVp8xmaC8mzY6RmU29MEHGv`

**Network Requirements:**
- Port 30333 (P2P) - **CRITICAL: Must be open**
- Port 9615 (Prometheus metrics) - Optional
- Port 9944 (WebSocket RPC) - Optional
- Port 9933 (HTTP RPC) - Optional

---

## ⚠️ Critical Information

### Firewall Configuration is MANDATORY

**All validators MUST have port 30333 open for incoming connections.**

Different cloud providers have different firewall defaults:

| Provider | Default Policy | Action Required |
|----------|---------------|-----------------|
| **Contabo** | `DROP` (blocks all) | ✅ **MUST open port 30333** |
| **Oracle Cloud** | `ACCEPT` (allows most) | ✅ Check Network Security Groups |
| **Azure** | `ACCEPT` (allows most) | ✅ Check Network Security Groups |
| **DigitalOcean** | `ACCEPT` (allows most) | ✅ Check Cloud Firewalls |
| **AWS** | `ACCEPT` (allows most) | ✅ Check Security Groups |

**Without port 30333 open, your validator will have 0 peers and will never sync.**

See [Firewall Troubleshooting Guide](./troubleshooting/FIREWALL_TROUBLESHOOTING.md) for details.

---

## 🔑 Session Keys

Every validator requires **unique session keys**:

- **AURA** (sr25519) - Block production
- **GRANDPA** (ed25519) - Finality voting
- **ASF** (sr25519) - Consensus participation

**NEVER reuse session keys from another validator.**

The automated deployment script generates unique keys automatically. For manual deployment, see [Session Key Guide](./troubleshooting/SESSION_KEY_ISSUES.md).

---

## 📝 Post-Deployment Checklist

After deployment, verify:

1. ✅ **Service is running:** `systemctl status flarechain-validator`
2. ✅ **Port 30333 is open:** `sudo iptables -L INPUT -n | grep 30333`
3. ✅ **Peers are connecting:** Wait 30 seconds, check logs for peer count
4. ✅ **Syncing to mainnet:** Check logs for "Syncing" messages with correct genesis
5. ✅ **Session keys saved:** Add keys to master secrets file

Expected peer count progression:
- **0-5 mins:** 0-3 peers (discovering network)
- **5-30 mins:** 5-10 peers (connecting to bootnodes)
- **30+ mins:** 10-20 peers (full P2P mesh)

---

## 🆘 Need Help?

1. Check [Common Issues](./troubleshooting/COMMON_ISSUES.md)
2. Check provider-specific guide for your cloud platform
3. Run the [health check script](./scripts/health-check.sh)
4. Review validator logs: `journalctl -u flarechain-validator -f`

---

## 📚 Additional Resources

- **[Infrastructure Overview](../mainnet/CURRENT_VALIDATOR_INFRASTRUCTURE.md)** - Current network topology
- **[Port Requirements](../mainnet/PORT_REQUIREMENTS.md)** - Detailed port documentation
- **[Monitoring Guide](../mainnet/VALIDATOR_MONITORING_INTEGRATION.md)** - Set up monitoring
- **[Cost Analysis](../mainnet/INFRASTRUCTURE_COST_ANALYSIS.md)** - Budget planning

---

## 🔄 Keeping This Documentation Updated

This documentation is maintained alongside the FlareChain mainnet. Last updated: **November 9, 2025**

If you discover issues not covered here, please contribute updates to help future validators.

---

**Ready to deploy? Start with the [Quick Start Guide](./guides/QUICK_START_NEW_VALIDATOR.md)**
