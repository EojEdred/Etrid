# Getting Started with Ëtrid

Welcome to Ëtrid! This guide will help you get started with the Ëtrid blockchain, whether you're a user, developer, or validator.

---

## Table of Contents

1. [What is Ëtrid?](#what-is-ëtrid)
2. [Quick Start for Users](#quick-start-for-users)
3. [Quick Start for Developers](#quick-start-for-developers)
4. [Quick Start for Validators](#quick-start-for-validators)
5. [Next Steps](#next-steps)

---

## What is Ëtrid?

Ëtrid is a next-generation blockchain platform that combines:

- **Multi-chain architecture** with 12+ bridge integrations
- **AI-powered agents** (E³20 protocol) for intelligent automation
- **Decentralized governance** via DAO treasury and community voting
- **DeFi integrations** with BSC, Solana, and Ethereum
- **Secure consensus** using FODDoS ASF (Fork-Optimized Distributed Denial of Service Attack-Secure Framework)

**Key Features:**
- ÉTR token (native currency)
- EDSC stablecoin (algorithmic stable asset)
- Treasury governance (875M ÉTR for community)
- Cross-chain bridges (BTC, ETH, SOL, DOGE, and more)
- WebAssembly smart contracts

---

## Quick Start for Users

### 1. Get a Wallet

**Option A: Browser Extension (Recommended)**

Install [Polkadot.js Extension](https://polkadot.js.org/extension/):
- Chrome: https://chrome.google.com/webstore
- Firefox: https://addons.mozilla.org/firefox

**Option B: Mobile Wallet**

- [SubWallet](https://subwallet.app/) (iOS & Android)
- [Talisman](https://talisman.xyz/) (Browser + Mobile)

### 2. Create an Account

1. Open Polkadot.js Extension
2. Click "+" → "Create new account"
3. **Save your seed phrase securely** (12-24 words)
4. Name your account (e.g., "My Ëtrid Account")
5. Set a password

**⚠️ IMPORTANT:** Never share your seed phrase. Anyone with your seed phrase can access your funds.

### 3. Connect to Ëtrid

Visit the Ëtrid Web Interface:
- **Mainnet:** https://app.etrid.org
- **Testnet:** https://testnet.etrid.org

Or use Polkadot.js Apps:
1. Go to https://polkadot.js.org/apps
2. Click top-left dropdown
3. Select "Development" → "Custom endpoint"
4. Enter: `wss://rpc.etrid.org`
5. Click "Switch"

### 4. Get ÉTR Tokens

**Mainnet:**
- Buy on exchanges (see [Exchange Listings](./EXCHANGE_LISTING.md))
- Swap on DEXes:
  - PancakeSwap (BSC): [Link]
  - Raydium (Solana): [Link]
  - Uniswap (Ethereum): [Link]

**Testnet:**
- Use faucet: https://faucet.etrid.org
- Request tokens on Discord: #testnet-faucet

### 5. Make Your First Transaction

1. Go to "Accounts" tab
2. Click "Send" next to your account
3. Enter recipient address
4. Enter amount (e.g., `1` ÉTR)
5. Click "Make Transfer"
6. Sign transaction in wallet popup

**Transaction fees:** ~0.001 ÉTR per transaction

---

## Quick Start for Developers

### 1. Prerequisites

Install required tools:

```bash
# Rust (for runtime development)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown

# Node.js (for dApp development)
# Install from: https://nodejs.org (v18+ recommended)

# Git
git --version  # Should be installed
```

### 2. Clone the Repository

```bash
git clone https://github.com/etrid/Etrid.git
cd Etrid
```

### 3. Build the Runtime

```bash
# Full build (takes 10-15 minutes first time)
cargo build --release --locked

# Check build succeeded
./target/release/etrid --version
```

### 4. Run a Local Development Node

```bash
# Start local single-node testnet
./target/release/etrid --dev --tmp

# The node will start producing blocks
# RPC endpoints:
#   - HTTP: http://localhost:9933
#   - WebSocket: ws://localhost:9944
#   - Prometheus: http://localhost:9615
```

### 5. Connect to Your Local Node

Visit https://polkadot.js.org/apps:
1. Click top-left dropdown
2. Select "Development" → "Local Node"
3. You should see blocks being produced

### 6. Example: Transfer ÉTR

```javascript
// Using @polkadot/api
const { ApiPromise, WsProvider } = require('@polkadot/api');
const { Keyring } = require('@polkadot/keyring');

async function main() {
  // Connect to local node
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Create keyring
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');

  // Transfer 1 ETR to Bob
  const transfer = api.tx.balances.transfer(bob.address, 1000000000000);
  const hash = await transfer.signAndSend(alice);

  console.log('Transfer sent with hash', hash.toHex());
}

main().catch(console.error);
```

### 7. Write Your First Smart Contract

Ëtrid supports WebAssembly smart contracts (etwasm-vm):

```bash
# Install smart contract tools
cargo install cargo-contract

# Create new contract
cargo contract new hello_etrid
cd hello_etrid

# Build contract
cargo contract build

# Deploy to local node
cargo contract instantiate --suri //Alice
```

### 8. Explore Example dApps

Check out example applications:
- **Governance Dashboard:** `apps/governance-ui/`
- **Validator Dashboard:** `apps/validator-dashboard/`
- **Wallet Web:** `apps/wallet-web/`

```bash
# Run governance UI locally
cd apps/governance-ui
npm install
npm run dev
# Visit: http://localhost:3000
```

### 9. Read the Documentation

- **Architecture:** [docs/architecture.md](./architecture.md)
- **API Reference:** [docs/API_REFERENCE.md](./API_REFERENCE.md)
- **Developer Guide:** [docs/DEVELOPER_GUIDE.md](./DEVELOPER_GUIDE.md)
- **Pallet Docs:** `cargo doc --open`

---

## Quick Start for Validators

### 1. Server Requirements

**Minimum:**
- CPU: 4 cores
- RAM: 8 GB
- Storage: 200 GB SSD
- Network: 100 Mbps

**Recommended (Mainnet):**
- CPU: 8+ cores
- RAM: 16+ GB
- Storage: 500 GB NVMe SSD
- Network: 1 Gbps

### 2. Install Ëtrid Node

```bash
# Build from source
git clone https://github.com/etrid/Etrid.git
cd Etrid
cargo build --release --locked

# Or download pre-built binary
wget https://github.com/etrid/Etrid/releases/latest/download/etrid-linux-x86_64
chmod +x etrid-linux-x86_64
sudo mv etrid-linux-x86_64 /usr/local/bin/etrid
```

### 3. Generate Session Keys

```bash
# Start node
./target/release/etrid --chain flarechain-mainnet --validator --name "My Validator"

# In another terminal, generate keys
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
  http://localhost:9933
```

Save the returned session keys (starts with `0x...`).

### 4. Bond ÉTR Tokens

Minimum stake: **100,000 ÉTR** (100M ÉTR with 12 decimals)

1. Go to https://polkadot.js.org/apps (connected to Ëtrid)
2. Navigate to "Network" → "Staking" → "Account actions"
3. Click "+ Validator"
4. Select your stash account
5. Enter amount: `100000` ÉTR
6. Select controller account
7. Submit transaction

### 5. Set Session Keys

1. Stay on "Account actions" page
2. Click "Session Key" next to your validator
3. Paste session keys from step 3
4. Submit transaction

### 6. Start Validating

```bash
# Start validator node (use systemd for production)
./target/release/etrid \
  --chain flarechain-mainnet \
  --validator \
  --name "My Validator" \
  --base-path /data/etrid \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --prometheus-port 9615 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'
```

### 7. Monitor Your Validator

**On-chain monitoring:**
- Polkadot.js Apps: "Network" → "Staking" → "Waiting"
- Check if your validator appears in "Waiting" or "Active" tab

**Server monitoring:**
- Prometheus: http://your-server:9615/metrics
- Set up Grafana dashboard (see [MONITORING_GUIDE.md](./MONITORING_GUIDE.md))

**Telemetry:**
- https://telemetry.polkadot.io
- Search for your validator name

### 8. Maintenance

**Update node:**
```bash
cd Etrid
git pull origin main
cargo build --release --locked
sudo systemctl restart etrid-validator
```

**Backup session keys:**
```bash
# Backup keys directory
tar -czf etrid-keys-backup-$(date +%Y%m%d).tar.gz /data/etrid/chains/flarechain/keystore
```

**Monitor rewards:**
```bash
# Check rewards
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "query", "params": ["staking.ledger", "YOUR_STASH_ADDRESS"]}' \
  http://localhost:9933
```

---

## Next Steps

### For Users

- [ ] Join the community:
  - Discord: https://discord.gg/etrid
  - Telegram: https://t.me/EtridOfficial
  - Twitter: https://twitter.com/etrid_network
- [ ] Explore the block explorer: https://explorer.etrid.org
- [ ] Participate in governance: https://forum.etrid.org
- [ ] Check out ecosystem projects: https://etrid.org/ecosystem

### For Developers

- [ ] Read the [Developer Guide](./DEVELOPER_GUIDE.md)
- [ ] Explore the [API Reference](./API_REFERENCE.md)
- [ ] Check out [Example dApps](../apps/)
- [ ] Join developer chat on Discord: #developers
- [ ] Apply for grants: https://forum.etrid.org/c/treasury-proposals
- [ ] Contribute to the codebase: [CONTRIBUTING.md](../CONTRIBUTING.md)

### For Validators

- [ ] Read the [Operator Guide](./OPERATOR_GUIDE.md)
- [ ] Set up monitoring: [MONITORING_GUIDE.md](./MONITORING_GUIDE.md)
- [ ] Join validator chat on Discord: #validators
- [ ] Register on validator leaderboard
- [ ] Set up backup infrastructure
- [ ] Configure alerting

---

## Getting Help

### Documentation

- **Main docs:** https://docs.etrid.org
- **GitHub Wiki:** https://github.com/etrid/Etrid/wiki
- **API Docs:** https://docs.etrid.org/api

### Community Support

- **Discord:** https://discord.gg/etrid (Fastest response)
- **Telegram:** https://t.me/EtridOfficial
- **Forum:** https://forum.etrid.org
- **Email:** support@etrid.org

### Technical Support

- **GitHub Issues:** https://github.com/etrid/Etrid/issues
- **Stack Overflow:** Tag with `etrid` or `substrate`
- **Validator support:** validators@etrid.org

### Security Issues

**Do NOT post security vulnerabilities publicly.**

Email: security@etrid.org

PGP key: https://etrid.org/security.asc

---

## FAQ

**Q: What is the difference between ÉTR and EDSC?**

A: ÉTR is the native token used for transactions, staking, and governance. EDSC is an algorithmic stablecoin pegged to $1 USD, backed by collateral and maintained by oracles.

**Q: Can I use MetaMask with Ëtrid?**

A: Not directly. Ëtrid uses Substrate (Polkadot SDK), which requires Polkadot.js or compatible wallets. However, you can bridge ÉTR to Ethereum and use it with MetaMask.

**Q: What are the transaction fees?**

A: ~0.001 ÉTR per simple transfer. Smart contract calls vary based on complexity.

**Q: How long does a transaction take?**

A: Block time is 6 seconds. Transactions are included in the next block and finalized within 12-18 seconds (2-3 blocks).

**Q: Can I run a validator on Windows?**

A: Not recommended for mainnet. Use Linux (Ubuntu 20.04+ recommended) for production validators.

**Q: How do I bridge tokens from Ethereum to Ëtrid?**

A: See [Bridge Guide](./BRIDGE_GUIDE.md) for step-by-step instructions.

**Q: Where can I see the roadmap?**

A: [LIVING_ROADMAP.md](../LIVING_ROADMAP.md) and [docs/roadmaps/](./roadmaps/)

---

## Welcome to Ëtrid!

We're excited to have you join our community. Whether you're here to use the platform, build applications, or secure the network as a validator, we're here to support you every step of the way.

**Let's build the future of decentralized finance together!**

---

*Last updated: October 25, 2025*
*Ëtrid Version: v0.1.0-alpha*
*Runtime Version: 103*
