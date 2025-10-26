# Ëtrid Protocol - Exchange Expansion Documentation

Welcome to the documentation for Ëtrid Protocol's **Exchange Expansion & MasterChef LP Rewards** program.

> Complete, production-ready system for zero-budget multichain deployment with community LP rewards.

## What This Documentation Covers

This is a focused documentation site for the **Exchange Expansion** deployment system, including:

- ✅ **BSC Smart Contracts** (ÉTR Token + MasterChef)
- ✅ **Deployment Scripts** (30+ automated tools)
- ✅ **Monitoring Dashboard** (Real-time metrics)
- ✅ **Operational Guides** (Day-to-day management)
- ✅ **Emergency Procedures** (Incident response)

For complete protocol documentation, see: [Full Documentation Catalog](README.md)

## Quick Links

### 🚀 Get Started

- **New to Ëtrid?** → Start with [Quick Start Guide](../QUICK_START.md)
- **Ready to deploy?** → Follow [BSC Deployment Guide](../05-multichain/bridge/adapters/bsc/README_DEPLOYMENT.md)
- **Setting up operations?** → See [Scripts Reference](../05-multichain/bridge/adapters/bsc/SCRIPTS_README.md)

### 📖 Essential Guides

| Guide | Description | Time to Complete |
|-------|-------------|------------------|
| [Quick Start](../QUICK_START.md) | Deploy your first contract to testnet | 30 minutes |
| [MasterChef Setup](../05-multichain/bridge/adapters/bsc/MASTERCHEF_GUIDE.md) | Configure LP rewards program | 1 hour |
| [Monitoring Setup](../05-multichain/bridge/adapters/bsc/AUTOMATED_MONITORING_SETUP.md) | Automate health checks & alerts | 1 hour |
| [Emergency Runbook](../05-multichain/bridge/adapters/bsc/EMERGENCY_RESPONSE_RUNBOOK.md) | Critical incident procedures | Read before launch |

## System Overview

**Total Cost**: ~$11-40 (gas fees only)
**Deployment Time**: 2 weeks
**Community Rewards**: 20M ÉTR over 6 months
**Supported Chains**: BSC + Solana

### What's Included

```
📦 Complete System
├── 🔷 Smart Contracts (3 contracts, 1000+ lines)
├── 🛠️ Deployment Tools (30+ npm scripts)
├── ✅ Testing Suite (77 unit tests)
├── 📊 Monitoring Dashboard (Next.js app)
├── 🤖 CI/CD Pipeline (GitHub Actions)
├── 📈 Price Feeds (PancakeSwap integration)
└── 📚 Documentation (20+ guides, 20,000+ words)
```

## Features

### Zero-Budget Deployment

- **No Upfront Capital**: Use token emissions instead of USD
- **Low Gas Costs**: BSC ($5-20) + Solana ($0.50-3)
- **Community Incentives**: 20M ÉTR rewards attract liquidity
- **Proven Strategy**: Time-tested LP rewards model

### Production Infrastructure

- **Automated Testing**: 77 comprehensive unit tests
- **Pre-Launch Validation**: 20+ critical safety checks
- **Health Monitoring**: Automated checks every 8 hours
- **Emergency Procedures**: Complete incident runbook
- **Multi-Sig Support**: Secure admin operations

### Real-Time Monitoring

- **Live Dashboard**: Beautiful Next.js interface
- **Price Integration**: PancakeSwap + Chainlink feeds
- **Multiple Formats**: JSON, CSV, Prometheus export
- **Auto-Refresh**: Updates every 60 seconds
- **Alert System**: Discord, Telegram, Email notifications

## Documentation Structure

This site uses the sidebar on the left to navigate between sections:

- **Getting Started**: Introduction and quick start
- **BSC Deployment**: Complete deployment guides
- **Development**: Setup and testing
- **Operations**: Daily management and monitoring
- **Reference**: API docs and SDK
- **Resources**: Tutorials, FAQ, troubleshooting

## Quick Start Example

```bash
# 1. Install dependencies
cd 05-multichain/bridge/adapters/bsc
npm install

# 2. Run tests
npm test

# 3. Generate wallet
npm run generate-wallet

# 4. Get testnet BNB
# Visit: https://testnet.bnbchain.org/faucet-smart

# 5. Deploy to testnet
npm run deploy:testnet

# 6. Deploy MasterChef
npm run deploy:masterchef:testnet

# 7. Validate deployment
npm run pre-launch-check:testnet
```

## Need Help?

- **Documentation Issues**: [GitHub Issues](https://github.com/etrid/etrid-protocol/issues)
- **General Questions**: [Discord #support](https://discord.gg/etrid)
- **Emergency Support**: support@etrid.io

## License

MIT License - see [LICENSE](../LICENSE) for details

---

**Ready?** → [Quick Start Guide](../QUICK_START.md)
