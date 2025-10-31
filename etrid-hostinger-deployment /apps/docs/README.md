# ËTRID Protocol Documentation

Welcome to the official ËTRID documentation. Learn how to use, build on, and operate the ËTRID multichain blockchain platform.

<div class="hero">
  <p class="tagline">Fast, secure, and interconnected blockchain infrastructure connecting 13 major chains</p>
</div>

---

## 🚀 Quick Links

<div class="grid-container">
  <div class="grid-item">
    <h3>👤 For Users</h3>
    <ul>
      <li><a href="#/GETTING_STARTED">Getting Started Guide</a></li>
      <li><a href="#/getting-etr">Get ÉTR Tokens</a></li>
      <li><a href="#/wallets">Choose a Wallet</a></li>
      <li><a href="#/staking">Stake & Earn Rewards</a></li>
      <li><a href="#/bridges">Use Cross-Chain Bridges</a></li>
    </ul>
  </div>

  <div class="grid-item">
    <h3>👨‍💻 For Developers</h3>
    <ul>
      <li><a href="#/DEVELOPER_GUIDE">Developer Guide</a></li>
      <li><a href="#/API_REFERENCE">API Reference</a></li>
      <li><a href="#/testing">Testing Framework</a></li>
      <li><a href="https://github.com/etrid/etrid" target="_blank">GitHub Repository</a></li>
    </ul>
  </div>

  <div class="grid-item">
    <h3>⚙️ For Validators</h3>
    <ul>
      <li><a href="#/OPERATOR_GUIDE">Operator Guide</a></li>
      <li><a href="#/MONITORING_GUIDE">Monitoring Setup</a></li>
      <li><a href="#/maintenance">Maintenance Guide</a></li>
      <li><a href="#/docker">Docker Deployment</a></li>
    </ul>
  </div>
</div>

---

## What is ËTRID?

ËTRID is a next-generation blockchain platform that connects 13 major blockchains through innovative **Partition Burst Chains (PBCs)**, enabling:

✨ **Cross-Chain Interoperability** - Seamlessly bridge assets between Bitcoin, Ethereum, Solana, and 10 other major chains

⚡ **Fast Finality** - 15-second block finality with ASF (Ascending Scale of Finality) consensus

🗳️ **Democratic Governance** - Annual Consensus Day where stakeholders vote on network upgrades and fiscal policy

💰 **DeFi & Staking** - Earn 8-15% APY by staking ÉTR to secure the network

🔐 **Enterprise-Grade Security** - Audited codebase with built-in reentrancy protection

---

## Key Features

### 🌉 13 Cross-Chain Bridges

Connect to major blockchains:
- Bitcoin (BTC)
- Ethereum (ETH)
- Solana (SOL)
- Cardano (ADA)
- Polkadot (DOT)
- Binance Smart Chain (BNB)
- Avalanche (AVAX)
- Polygon (MATIC)
- And 5 more...

[Learn about bridges →](bridges.md)

---

### ⚡ Ascending Scale of Finality (ASF)

Novel consensus mechanism that:
- Achieves 15-second finality
- Prevents validator centralization
- Energy-efficient (no mining)
- Secure against long-range attacks

[Read architecture docs →](architecture.md)

---

### 🏛️ Consensus Day Governance

Annual democratic event where:
- Stakeholders vote on inflation rate
- Network upgrades are decided
- Treasury allocation determined
- Community proposals enacted

Next Consensus Day: **December 1, 2026**

[Explore governance →](specifications/governance-appendix.md)

---

### 💎 ÉTR Token Economics

**Total Supply:** 100,000,000 ÉTR

**Distribution:**
- 40% - Staking Rewards (vested over 10 years)
- 25% - Community Treasury
- 20% - Development Team (4-year vesting)
- 10% - Initial Validators
- 5% - Early Supporters

**Utility:**
- Transaction fees
- Staking rewards
- Governance voting power
- Cross-chain bridge fees

---

## Network Status

**Mainnet:** ✅ Live (Block #8,342,156)

**Testnet (Ember):** ✅ Live

**Current Stats:**
- **Validators:** 21 active
- **Total Staked:** 42.3M ÉTR (42.3%)
- **Avg Block Time:** 5 seconds
- **Finality Time:** 15 seconds
- **TPS:** ~1,000 (100k+ with Lightning-Bloc L2)

[View live stats →](https://stats.etrid.org)

---

## Getting Started

### For Users

1. **[Create a wallet](wallets.md)** - Choose between web, mobile, or hardware wallet
2. **[Get ÉTR tokens](getting-etr.md)** - From faucet (testnet) or exchanges (mainnet)
3. **[Send your first transaction](GETTING_STARTED.md#transactions)** - Transfer ÉTR to another address
4. **[Start staking](staking.md)** - Earn 8-15% APY by nominating validators

### For Developers

1. **[Set up development environment](DEVELOPER_GUIDE.md#setup)** - Install tools and dependencies
2. **[Run a local node](DEVELOPER_GUIDE.md#local-node)** - Test your dApps locally
3. **[Deploy a smart contract](DEVELOPER_GUIDE.md#smart-contracts)** - Build with ink! or Solidity
4. **[Integrate with your app](API_REFERENCE.md)** - Use JavaScript/Rust SDKs

### For Validators

1. **[Read operator guide](OPERATOR_GUIDE.md)** - Understand validator requirements
2. **[Deploy validator node](OPERATOR_GUIDE.md#deployment)** - Set up production infrastructure
3. **[Set session keys](OPERATOR_GUIDE.md#session-keys)** - Configure validator identity
4. **[Start validating](OPERATOR_GUIDE.md#start-validating)** - Join the active validator set

---

## Core Concepts

### FlareChain (Relay Chain)
- Main blockchain secured by 21 validators
- Coordinates cross-chain messages
- Processes governance transactions
- 15-second finality with ASF consensus

### Partition Burst Chains (PBCs)
- 13 specialized chains connected to FlareChain
- Each PBC bridges to an external blockchain
- Light client verification for security
- Enables cross-chain asset transfers

### Lightning-Bloc (Layer 2)
- Payment channel network for instant transactions
- Minimal fees for micropayments
- Watchtower network for security
- Up to 100,000 TPS off-chain

[Learn more about architecture →](architecture.md)

---

## Specifications

📄 **[Ivory Paper v2.0](specifications/ivory-paper.md)** - Complete technical specification

📜 **[Protocol Charter](specifications/protocol-charter.md)** - Governance rules and principles

🏛️ **[Governance Appendix](specifications/governance-appendix.md)** - Consensus Day details

---

## Community

### Join the Community

💬 **Discord:** [discord.gg/etrid](https://discord.gg/etrid)
📱 **Telegram:** [t.me/EtridOfficial](https://t.me/EtridOfficial)
🐦 **Twitter:** [@EtridMultichain](https://twitter.com/EtridMultichain)
📺 **YouTube:** [youtube.com/@etrid](https://youtube.com/@etrid)
💼 **Reddit:** [r/EtridBlockchain](https://reddit.com/r/EtridBlockchain)

[View all links →](links.md)

### Governance & Proposals

🏛️ **Governance Portal:** [governance.etrid.org](https://governance.etrid.org)
💬 **Forum:** [forum.etrid.org](https://forum.etrid.org)
📋 **Submit Proposal:** [proposal-templates.md](proposal-templates.md)

---

## Support

### Get Help

📖 **Documentation:** You're reading it!
💬 **Discord Support:** [#support channel](https://discord.gg/etrid)
📧 **Email:** support@etrid.org

### Report Issues

🐛 **Bug Reports:** [github.com/etrid/etrid/issues](https://github.com/etrid/etrid/issues)
🔒 **Security Issues:** security@etrid.org (PGP key available)

---

## Recent Updates

**October 2025**
- ✅ Mainnet launch successful
- ✅ 21 validators online
- ✅ Cross-chain bridges live (BTC, ETH, SOL, BSC, and more)
- ✅ Governance portal deployed

**Upcoming (Q1 2026)**
- 🔜 Mobile wallet launch (iOS & Android)
- 🔜 Hardware wallet support (Ledger, Trezor)
- 🔜 Additional PBC integrations
- 🔜 Lightning-Bloc mainnet beta

[View full roadmap →](https://etrid.org/roadmap)

---

## Contributing

ËTRID is open source! Contributions welcome:

📝 **Documentation:** Improve these docs
💻 **Code:** Submit PRs to GitHub
🐛 **Bug Reports:** File issues
💡 **Ideas:** Share on forum

[Contributing Guide →](https://github.com/etrid/etrid/blob/main/CONTRIBUTING.md)

---

<div class="footer-cta">
  <h2>Ready to get started?</h2>
  <p>Choose your path:</p>
  <div class="button-group">
    <a href="#/GETTING_STARTED" class="button button-primary">Start as User</a>
    <a href="#/DEVELOPER_GUIDE" class="button button-secondary">Build on ËTRID</a>
    <a href="#/OPERATOR_GUIDE" class="button button-secondary">Run a Validator</a>
  </div>
</div>

---

**Last Updated:** October 30, 2025
**Documentation Version:** 2.0
**Network Version:** v1.0.0

*Found an issue with these docs? [Report it here](https://github.com/etrid/docs/issues)*
