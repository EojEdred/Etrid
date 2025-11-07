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

🚀 **Massive Scalability** - **171,000+ TPS** across three layers (FlareChain + 14 PBCs + Lightning Bloc)

⚡ **Instant Transactions** - Lightning Bloc Layer 2 provides 100ms transaction speed with zero fees

🔗 **Multi-Layer Architecture** - Guaranteed finality in 56 minutes from Lightning to FlareChain via PBC checkpoints

🗳️ **Democratic Governance** - Annual Consensus Day where stakeholders vote on network upgrades and fiscal policy

💰 **DeFi & Staking** - Earn 8-15% APY by staking ÉTR to secure the network

🔐 **Enterprise-Grade Security** - Multi-layer fraud proofs, GRANDPA finality, and audited codebase

---

## 🔥 The Flame Architecture

**ËTRID's three layers work like a natural flame** - hottest (blue) at the center, expanding outward through orange to yellow:

```
       ⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡
     ⚡⚡ Lightning Bloc (Layer 3) ⚡⚡         Yellow Outer Flame
   ⚡⚡   100,000+ TPS | Instant   ⚡⚡         1,200-1,800°F
     ⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡⚡
        🔶🔶🔶🔶🔶🔶🔶🔶🔶🔶🔶
      🔶🔶 PBCs (Layer 2) 🔶🔶               Orange Middle Flame
    🔶🔶 70,000 TPS | 13 Chains 🔶🔶           1,800-2,100°F
      🔶🔶🔶🔶🔶🔶🔶🔶🔶🔶🔶
         🔵🔵🔵🔵🔵🔵🔵
       🔵🔵 FlareChain 🔵🔵                   Blue Core (Hottest)
     🔵🔵 1,000 TPS | 24s 🔵🔵                 2,600°F
       🔵🔵🔵🔵🔵🔵🔵

  External Chains: BTC • ETH • SOL • XRP • BNB • TRX • ADA • MATIC • LINK • DOGE • XLM
```

**Why This Works:**

| Layer | Color | Analogy | Purpose | Performance |
|-------|-------|---------|---------|-------------|
| **FlareChain (L1)** | 🔵 Blue | Hottest part of flame (complete combustion) | Maximum security, absolute finality | 1,000 TPS, 24s GRANDPA |
| **PBCs (L2)** | 🔶 Orange | Middle flame (most energy output) | High throughput, specialized chains | 70,000 TPS, 51min checkpoints |
| **Lightning (L3)** | ⚡ Yellow | Outer flame (fastest, furthest reach) | Instant transactions, zero friction | 100,000+ TPS, 100ms latency |

**Total Network Capacity:** 🔥 **171,000+ TPS** 🔥

[View Interactive 3D Visualization →](architecture-viz) | [Technical Deep Dive →](architecture.md#multi-layer-state-synchronization)

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

### 🎯 Three-Layer Architecture

**171,000+ TPS** across optimized layers:

**Layer 1: FlareChain (Relay Chain)**
- 1,000 TPS baseline capacity
- GRANDPA finality in 24 seconds
- Stores PBC state checkpoints (32 bytes each)
- Coordinated by Directors 1-5

**Layer 2: Partition Burst Chains (14 PBCs)**
- 5,000 TPS per chain = 70,000 TPS total
- Specialized runtimes for each asset type
- Checkpoints to FlareChain every 256 blocks (~51 min)
- Validators 6-21 rotate across PBCs

**Layer 3: Lightning Bloc (Payment Channels)**
- 100,000+ TPS off-chain capacity
- Instant finality (~100ms)
- Zero fees for off-chain transactions
- Batch settlements to PBCs every 5 minutes

**Result:** Instant user experience with guaranteed finality

[Learn about multi-layer state sync →](architecture.md#multi-layer-state-synchronization)

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
- ✅ Mobile wallet (iOS & Android)
- ✅ Hardware wallet support (Ledger, Trezor)
- ✅ PBC integrations complete
- ✅ Lightning-Bloc mainnet live

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
