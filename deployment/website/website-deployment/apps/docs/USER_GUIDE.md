# Etrid User Guide

**Welcome to Etrid!** This guide will help you get started with the Etrid blockchain platform, from creating your first wallet to participating in governance.

**Version:** 1.0.0
**Last Updated:** October 22, 2025
**Target Audience:** Blockchain beginners, crypto users new to Etrid

---

## Table of Contents

1. [Introduction to Etrid](#introduction-to-etrid)
2. [Getting Started](#getting-started)
3. [Wallet Setup](#wallet-setup)
4. [Account Security](#account-security)
5. [Getting ETR Tokens](#getting-etr-tokens)
6. [Sending and Receiving Transactions](#sending-and-receiving-transactions)
7. [Staking and Earning Rewards](#staking-and-earning-rewards)
8. [Governance Participation](#governance-participation)
9. [Using Cross-Chain Features](#using-cross-chain-features)
10. [Common Tasks](#common-tasks)
11. [Troubleshooting](#troubleshooting)
12. [FAQ](#faq)

---

## Introduction to Etrid

### What is Etrid?

Etrid is a next-generation multichain blockchain platform that connects 13 major blockchains, enabling seamless cross-chain transactions and truly decentralized governance.

**Key Features:**

- **🌉 Cross-Chain Connectivity**: Bridge assets between Bitcoin, Ethereum, Solana, Cardano, and 10 other major chains
- **⚡ Lightning-Fast Transactions**: 15-second finality with Layer 2 Lightning-Bloc payment channels for instant payments
- **🗳️ Democratic Governance**: Annual Consensus Day where stakeholders vote on network upgrades and fiscal policy
- **💰 Stablecoin System**: EDSC (Etrid Digital Standard Currency) backed by real-world reserves
- **🔐 Self-Sovereign Identity**: OpenDID and AIDID for human and AI identity management
- **🌱 Eco-Friendly**: Energy-efficient Ascending Scale of Finality (ASF) consensus mechanism

### How Etrid Works

```
┌─────────────────────────────────────────────────────────────────┐
│                        Etrid Ecosystem                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    FlareChain (Relay)                      │  │
│  │  - Main blockchain with 15-second finality                 │  │
│  │  - Secured by 21 validators using ASF consensus            │  │
│  │  - Processes cross-chain messages and governance           │  │
│  └───────────────────────────────────────────────────────────┘  │
│                              ▲                                    │
│                              │                                    │
│  ┌───────────────────────────┴───────────────────────────────┐  │
│  │         13 Partition Burst Chains (PBCs)                   │  │
│  │  BTC | ETH | SOL | ADA | DOT | BNB | AVAX | MATIC | ...   │  │
│  │  - Bridge assets from major blockchains                    │  │
│  │  - Maintain light clients for each chain                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              Lightning-Bloc (Layer 2)                      │  │
│  │  - Instant payments via payment channels                   │  │
│  │  - Watchtower network for security                         │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Why Use Etrid?

**For Everyday Users:**
- Send cross-chain payments without using multiple wallets or exchanges
- Earn passive income by staking your ETR tokens
- Vote on network decisions that affect your investments
- Access DeFi applications across multiple blockchains

**For Developers:**
- Build DApps that work across 13 blockchains
- Deploy smart contracts with reentrancy protection
- Use powerful SDKs in JavaScript, Python, and Rust
- Access comprehensive APIs and documentation

**For Validators:**
- Earn block rewards and transaction fees
- Participate in a fair, energy-efficient consensus mechanism
- Join a growing ecosystem with strong community support

---

## Getting Started

### Prerequisites

Before you begin, you'll need:

1. **A modern web browser** (Chrome, Firefox, Edge, or Brave)
2. **An internet connection**
3. **15 minutes** to complete the setup
4. **Paper and pen** to write down your recovery phrase

**No downloads required!** The Etrid web wallet runs entirely in your browser.

### Quick Start Checklist

Follow this checklist to get started:

- [ ] Create your Etrid wallet account
- [ ] Write down and safely store your recovery phrase
- [ ] Get test ETR tokens from the faucet (testnet)
- [ ] Send your first transaction
- [ ] Nominate a validator to start earning rewards
- [ ] Join the Etrid community (Discord/Telegram)

**Estimated time:** 15-20 minutes

---

## Wallet Setup

### Choosing a Wallet

Etrid offers three wallet options:

| Wallet Type | Best For | Features | Download |
|-------------|----------|----------|----------|
| **Web Wallet** | Quick access, beginners | No installation, works on any device | [wallet.etrid.io](https://wallet.etrid.io) |
| **Browser Extension** | Daily users, DApp interaction | Polkadot.js compatible, secure | [Chrome Web Store](https://chrome.google.com/webstore) |
| **Mobile Wallet** | On-the-go transactions | iOS/Android, QR code scanning | App Store / Google Play |

**Recommendation for beginners:** Start with the **Web Wallet** for easiest setup.

### Creating Your First Account (Web Wallet)

#### Step 1: Access the Web Wallet

1. Open your web browser
2. Navigate to **[wallet.etrid.io](https://wallet.etrid.io)**
3. Verify the URL is correct (check for HTTPS and correct spelling)
4. Click **"Launch Wallet"**

**Security Tip:** Bookmark the official URL to avoid phishing sites.

#### Step 2: Create a New Account

1. Click **"Create Account"** in the top right corner
2. You'll see a **12-word recovery phrase** displayed

**Example (DO NOT use this phrase):**
```
witch collapse practice feed shame open despair creek road again ice least
```

#### Step 3: Save Your Recovery Phrase

**⚠️ CRITICAL: This is the most important step!**

Your recovery phrase is the ONLY way to restore your account if:
- You lose access to your device
- Your browser cache is cleared
- You want to access your account on another device

**How to save it safely:**

1. **Write it down on paper** in the exact order shown (1-12)
2. **Double-check** each word for accuracy
3. **Store it securely** in a safe, fireproof location
4. **Make a backup copy** and store it separately
5. **NEVER** take a screenshot or save digitally
6. **NEVER** share it with anyone (not even Etrid support)

**Who controls the recovery phrase controls the funds. There is NO password reset!**

#### Step 4: Verify and Continue

1. Check the box: **"I have saved my recovery phrase safely"**
2. Click **"Continue"**
3. You may be asked to verify by selecting words in order

#### Step 5: Set a Password

1. Create a strong password (minimum 8 characters)
2. Use a mix of uppercase, lowercase, numbers, and symbols
3. Confirm your password

**Good password example:** `M0on$Rocket!2025`
**Bad password example:** `password123`

**Note:** This password encrypts your account on THIS device only. You'll still need your recovery phrase to restore on other devices.

#### Step 6: Name Your Account

1. Give your account a memorable name (e.g., "My Main Account")
2. Click **"Create Account"**

**Success!** You now have an Etrid account with an address like:
```
5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

### Understanding Your Account

**Your Etrid Address:**
- Starts with "5" (for FlareChain accounts)
- 48 characters long
- This is your PUBLIC identifier - safe to share
- Think of it like your email address or bank account number

**Your Recovery Phrase:**
- 12 words in a specific order
- This is your PRIVATE key - never share
- Think of it like your password or PIN

**Account Address vs. Recovery Phrase:**

| Account Address | Recovery Phrase |
|-----------------|-----------------|
| ✅ Share to receive funds | ❌ NEVER share |
| ✅ Post publicly | ❌ Keep completely private |
| ✅ Use on exchanges/forms | ❌ Store offline only |

### Creating Additional Accounts

You can create multiple accounts for different purposes:

1. Click the account dropdown (top right)
2. Click **"+ Add Account"**
3. Follow the same creation process

**Common account setups:**
- **Main Account:** Long-term holdings, staking
- **Trading Account:** Buying/selling, exchanges
- **DApp Account:** Interacting with decentralized apps
- **Savings Account:** Cold storage, rarely accessed

---

## Account Security

### The Golden Rules of Account Security

1. **NEVER share your recovery phrase** with anyone
2. **NEVER enter your recovery phrase** on websites (except official wallet restoration)
3. **ALWAYS verify URLs** before entering sensitive information
4. **ALWAYS use strong passwords**
5. **CONSIDER hardware wallets** for large amounts

### Recovery Phrase Security Best Practices

#### ✅ DO:

- Write recovery phrase on paper with pen/pencil
- Store in a fireproof safe or safety deposit box
- Make multiple copies stored in different locations
- Use metal backup solutions (fire/water resistant)
- Tell a trusted person WHERE it's stored (not the phrase itself)
- Consider using Shamir's Secret Sharing for high-value accounts

#### ❌ DON'T:

- Take screenshots or photos
- Store in cloud services (Google Drive, Dropbox, iCloud)
- Send via email, text, or messaging apps
- Store in password managers (debatable - if you do, encrypt separately)
- Write it on devices connected to internet
- Laminate paper (prevents emergency destruction if compromised)

### Password Security

**For your wallet password:**

1. **Use a password manager** (1Password, Bitwarden, KeePass)
2. **Enable 2FA** on your password manager
3. **Use unique passwords** for each account
4. **Make passwords long** (12+ characters better than complex 8-character)

**Good password patterns:**
- Passphrase: `CorrectHorseBatteryStaple` (easy to remember, hard to crack)
- Random: `xK9#mP2$vL8@nQ5!` (use password manager to generate)

### Recognizing Phishing Attempts

**Common phishing tactics:**

1. **Fake wallet websites** with similar URLs:
   - ❌ `wallet-etrid.io` (dash instead of dot)
   - ❌ `etrid-wallet.com` (wrong domain)
   - ✅ `wallet.etrid.io` (correct official URL)

2. **Recovery phrase requests:**
   - ❌ "Support" asking for your recovery phrase
   - ❌ Emails asking you to "verify" your phrase
   - ❌ Pop-ups requesting recovery phrase entry

3. **Urgent messages:**
   - "Your account will be closed unless you act now"
   - "Verify your wallet within 24 hours"
   - "Claim your airdrop by entering your recovery phrase"

**When in doubt:**
- Check official Etrid social media channels
- Ask in official Discord/Telegram
- Visit official website directly (don't click email links)

### Social Engineering Protection

**Common social engineering attacks:**

1. **Impersonation:**
   - Fake "Etrid Support" on Discord/Telegram
   - DMs offering help with "account issues"
   - Lookalike social media accounts

2. **Too good to be true:**
   - "Send 1 ETR, get 2 ETR back" scams
   - Fake airdrops requiring private key entry
   - Guaranteed returns promises

3. **Urgency tactics:**
   - "Limited time offer"
   - "Act now or lose access"
   - "Your account has been flagged"

**Protection:**
- Etrid support will NEVER DM you first
- Etrid will NEVER ask for your recovery phrase
- There are NO "giveaways" requiring private key entry

### Device Security

**Keep your devices secure:**

1. **Operating System:**
   - Keep OS updated with latest security patches
   - Use antivirus software on Windows
   - Enable firewall

2. **Browser:**
   - Keep browser updated
   - Use ad blockers to prevent malicious ads
   - Clear cache regularly
   - Use browser extensions cautiously

3. **Network:**
   - Avoid public Wi-Fi for wallet access
   - Use VPN when on untrusted networks
   - Enable router firewall

### Emergency Procedures

**If you suspect compromise:**

1. **Immediately transfer funds** to a new account
2. **Create new recovery phrase** (new account)
3. **Report the incident** to Etrid team
4. **Analyze what happened** to prevent future compromise

**If you lose your recovery phrase:**

Unfortunately, **there is NO way to recover your account** without the recovery phrase. Your funds will be permanently inaccessible. This is why backup is critical.

**If you lose your password (but have recovery phrase):**

1. Click **"Restore Account"** in wallet
2. Enter your 12-word recovery phrase
3. Set a new password
4. Your account is restored with all funds intact

---

## Getting ETR Tokens

### Understanding ETR Token

**ETR (Etrid)** is the native token of the Etrid blockchain:

- **Symbol:** ETR
- **Decimals:** 12 (1 ETR = 1,000,000,000,000 units)
- **Total Supply:** 100,000,000 ETR (100 million)
- **Use Cases:**
  - Pay transaction fees
  - Stake to earn rewards
  - Participate in governance (voting power)
  - Nominate validators

### Testnet vs. Mainnet

**Testnet:**
- Free test tokens from faucet
- For learning and testing
- No real-world value
- Reset periodically

**Mainnet:**
- Real ETR tokens with value
- Purchase from exchanges
- Production environment
- Permanent blockchain

**Always start on testnet** to learn before using real funds!

### Getting Testnet Tokens (Free)

#### Step 1: Copy Your Address

1. Open your wallet at [wallet.etrid.io](https://wallet.etrid.io)
2. Ensure you're connected to **Testnet** (check top bar)
3. Click the **copy icon** next to your address

#### Step 2: Request Tokens from Faucet

1. Open a new tab and go to **[faucet.etrid.io](https://faucet.etrid.io)**
2. Paste your address in the input field
3. Complete the CAPTCHA (if present)
4. Click **"Request Tokens"**

You'll receive **100 test ETR** within 15-30 seconds.

**Faucet limits:**
- 100 ETR per request
- Once per address per 24 hours
- Testnet only

#### Step 3: Verify Receipt

1. Return to your wallet tab
2. Click the **refresh icon** or wait for auto-update
3. Your balance should now show: `100.0000 ETR`

### Getting Mainnet Tokens (Real Value)

**Warning:** Only purchase mainnet ETR after you're comfortable using testnet!

#### Method 1: Centralized Exchanges

**Steps:**
1. Create account on supported exchange
2. Complete KYC verification
3. Deposit fiat currency (USD, EUR, etc.)
4. Buy ETR tokens
5. Withdraw to your Etrid wallet address

**Supported Exchanges:**
- Binance
- Coinbase
- Kraken
- OKX
- (Check official Etrid website for current list)

**Important:** Always test with a small amount first!

#### Method 2: Decentralized Exchanges (DEX)

**Steps:**
1. Connect wallet to Etrid DEX
2. Swap other crypto (BTC, ETH, USDC) for ETR
3. Tokens appear directly in your wallet

**Popular DEXs:**
- EtridSwap (native DEX)
- Polkadot DEXs with Etrid support
- Cross-chain DEX aggregators

#### Method 3: Peer-to-Peer (P2P)

**Steps:**
1. Find trusted seller on P2P platform
2. Negotiate price and payment method
3. Use escrow service for protection
4. Complete transaction

**P2P Platforms:**
- LocalCryptos
- Bisq
- HodlHodl

**Security:** Always use escrow and verify seller reputation!

### Checking Your Balance

**In Wallet:**
- Balance displayed prominently on dashboard
- Separate balances for each chain (FlareChain, PBC-BTC, etc.)
- Click chain selector to switch between chains

**Balance Format:**
```
Total Balance: 100.0000 ETR

Available: 90.0000 ETR (spendable)
Staked: 10.0000 ETR (locked, earning rewards)
Reserved: 0.0000 ETR (held for governance deposits)
```

**Types of balances:**
- **Available:** Freely transferable
- **Staked:** Locked for staking (withdrawable with unbonding period)
- **Reserved:** Held for specific purposes (refundable)

---

## Sending and Receiving Transactions

### Receiving ETR Tokens

**Receiving is simple - just share your address!**

#### Step 1: Find Your Address

1. Open your wallet
2. Your address is displayed prominently
3. Click the **copy icon** to copy address

#### Step 2: Share Your Address

**Safe ways to share:**
- Send via email/text to sender
- Generate QR code (mobile wallet)
- Post in invoice/request

**Address format:**
```
5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

**Tips:**
- Always double-check copied address matches displayed address
- Don't confuse addresses between different chains
- You can reuse the same address multiple times

#### Step 3: Wait for Transaction

- Transactions appear in wallet within seconds
- Full finality in ~15 seconds (3 block confirmations)
- You'll see balance update automatically

### Sending ETR Tokens

#### Step 1: Prepare to Send

Before sending, verify:
- [ ] You have sufficient balance + fees
- [ ] Recipient address is correct
- [ ] You're on the correct network (testnet vs mainnet)

#### Step 2: Initiate Transaction

1. Click **"Send"** button in wallet
2. Fill in the form:

**Transaction Form:**
```
┌─────────────────────────────────────────────────┐
│ Send ETR                                         │
├─────────────────────────────────────────────────┤
│ Recipient Address:                               │
│ [5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ...] │
│                                                  │
│ Amount:                                          │
│ [10.0000] ETR                                    │
│                                                  │
│ Transaction Fee (estimated):                     │
│ 0.0001 ETR                                       │
│                                                  │
│ Total: 10.0001 ETR                              │
│                                                  │
│ [ Cancel ]              [ Review Transaction ] │
└─────────────────────────────────────────────────┘
```

**Field explanations:**
- **Recipient Address:** Destination wallet address (starts with "5")
- **Amount:** How much ETR to send
- **Fee:** Automatically calculated based on network load

#### Step 3: Review Transaction

1. Click **"Review Transaction"**
2. Double-check all details:
   - Recipient address (first 6 and last 6 characters minimum)
   - Amount to send
   - Transaction fee
   - Total deducted from your account

**Review Screen:**
```
┌─────────────────────────────────────────────────┐
│ Transaction Review                               │
├─────────────────────────────────────────────────┤
│ From:                                            │
│ Your Account (5Grwva...KutQY)                   │
│                                                  │
│ To:                                              │
│ 5HpG9w...iQ7qUs                                 │
│                                                  │
│ Amount: 10.0000 ETR                             │
│ Fee:     0.0001 ETR                             │
│ ─────────────────────                           │
│ Total:  10.0001 ETR                             │
│                                                  │
│ ⚠️  Blockchain transactions are irreversible!   │
│    Double-check all details before confirming.  │
│                                                  │
│ [ Back ]                  [ Sign and Send ]    │
└─────────────────────────────────────────────────┘
```

#### Step 4: Sign and Send

1. Click **"Sign and Send"**
2. Enter your wallet password
3. Click **"Confirm"**

**Transaction submitted!** You'll see a confirmation:

```
✅ Transaction Sent Successfully!

Transaction Hash:
0x8f4a9c2b1e7d6f5a3c8e9b0d2a1f4e7c6b5d8a3f9e2c1b4a7d6f5e8c9b3a2d1f

Status: Pending (0/3 confirmations)

[ View in Explorer ]  [ Close ]
```

#### Step 5: Track Confirmation

Watch the status update:
- **Pending:** 0/3 confirmations (in mempool)
- **Confirming:** 1/3, 2/3 confirmations (in blocks)
- **Finalized:** 3/3 confirmations (irreversible)

**Timeline:**
- 0-5 seconds: Pending
- 5-10 seconds: 1st confirmation
- 10-15 seconds: 2nd confirmation
- 15-20 seconds: Final confirmation

### Understanding Transaction Fees

**Fee calculation factors:**
1. **Transaction size** (bytes of data)
2. **Network congestion** (higher demand = higher fees)
3. **Priority** (optional: pay more for faster processing)

**Typical fees on Etrid:**
- Simple transfer: 0.0001 - 0.001 ETR (~$0.001 - $0.01)
- Complex transaction (smart contract): 0.001 - 0.01 ETR
- During high congestion: 0.01 - 0.1 ETR

**Fee optimization:**
- Transact during off-peak hours for lower fees
- Batch multiple transactions when possible
- Use Lightning-Bloc for micro-payments (minimal fees)

### Transaction History

**View all your transactions:**

1. Click **"Transactions"** tab in wallet
2. See complete history with:
   - Date/time
   - Type (sent/received)
   - Amount
   - Status (pending/finalized)
   - Transaction hash

**Filter options:**
- By date range
- By type (sent/received/all)
- By status
- By chain

**Export options:**
- Download as CSV for accounting
- Export to tax software
- Print receipt

---

## Staking and Earning Rewards

### What is Staking?

**Staking** means locking your ETR tokens to support the network and earn rewards.

**Benefits:**
- **Earn passive income:** 8-15% annual rewards (APY varies)
- **Secure the network:** Help validate transactions
- **Governance voting power:** Staked tokens count for voting
- **Support decentralization:** Choose validators you trust

**How it works:**

```
┌─────────────────────────────────────────────────────────────────┐
│                      Etrid Staking Flow                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  1. You nominate validators                                      │
│     (choose up to 16 validators you trust)                       │
│                          │                                        │
│                          ▼                                        │
│  2. Your stake is allocated                                      │
│     (algorithm distributes to elected validators)                │
│                          │                                        │
│                          ▼                                        │
│  3. Validators produce blocks                                    │
│     (earn block rewards + fees)                                  │
│                          │                                        │
│                          ▼                                        │
│  4. Rewards distributed                                          │
│     (you receive portion based on stake)                         │
│                          │                                        │
│                          ▼                                        │
│  5. Rewards auto-compound                                        │
│     (or withdraw to available balance)                           │
└─────────────────────────────────────────────────────────────────┘
```

### Staking Requirements

**Minimum stake:** 1 ETR (for nominators)
**Maximum nominations:** 16 validators
**Unbonding period:** 28 days (period to unstake)
**Rewards distribution:** Every era (24 hours)

### How to Stake (Nominating)

#### Step 1: Navigate to Staking

1. Open wallet at [wallet.etrid.io](https://wallet.etrid.io)
2. Click **"Staking"** in the navigation menu
3. You'll see the staking dashboard:

```
┌─────────────────────────────────────────────────────────────────┐
│ Staking Dashboard                                                │
├─────────────────────────────────────────────────────────────────┤
│ Your Staking Overview:                                           │
│                                                                   │
│ Available Balance:    90.0000 ETR                               │
│ Currently Staked:      0.0000 ETR                               │
│ Pending Rewards:       0.0000 ETR                               │
│ Estimated APY:        12.5%                                      │
│                                                                   │
│ [ Start Nominating ]                                            │
└─────────────────────────────────────────────────────────────────┘
```

#### Step 2: Choose Validators

1. Click **"Start Nominating"** or **"Nominate"**
2. You'll see a list of active validators:

**Validator List:**
```
┌────────────────────────────────────────────────────────────────────┐
│ Select Validators to Nominate (max 16)                             │
├────────────────────────────────────────────────────────────────────┤
│ Search: [_________]   Filters: [Commission ▼] [Performance ▼]     │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│ ☐ Validator Alpha                         Commission: 5%          │
│   Stake: 1,234,567 ETR | Nominators: 456 | Uptime: 99.9%         │
│   Identity: ✓ Verified | Location: Germany                        │
│                                                                     │
│ ☐ Etrid Foundation Node                   Commission: 0%          │
│   Stake: 2,000,000 ETR | Nominators: 892 | Uptime: 100%          │
│   Identity: ✓ Verified | Location: Switzerland                    │
│                                                                     │
│ ☐ StakingRewards.com                      Commission: 3%          │
│   Stake: 987,654 ETR | Nominators: 234 | Uptime: 99.8%           │
│   Identity: ✓ Verified | Location: USA                            │
│                                                                     │
│ [Show More Validators...]                                          │
│                                                                     │
│ Selected: 0/16                         [ Next ]                   │
└────────────────────────────────────────────────────────────────────┘
```

**How to choose validators:**

**Look for:**
- ✅ Low commission (5% or less)
- ✅ High uptime (99%+)
- ✅ Verified identity
- ✅ Reasonable stake (not too high = centralization risk)
- ✅ Active community presence
- ✅ Regular updates and transparency

**Avoid:**
- ❌ 100% commission (you earn nothing)
- ❌ Very low uptime (<95%)
- ❌ Unknown validators with no identity
- ❌ Validators with excessive stake (over-concentration)

**Diversification strategy:**
- Select 8-16 validators (not just 1)
- Mix of large and small validators
- Different geographical locations
- Different operators (avoid same entity)

**Example selection:**
1. Etrid Foundation Node (0% commission, trusted)
2. Validator Alpha (5% commission, high uptime)
3. StakingRewards.com (3% commission, established)
4-16. Mix of community validators with good metrics

#### Step 3: Set Stake Amount

1. After selecting validators, click **"Next"**
2. Enter amount to stake:

```
┌─────────────────────────────────────────────────┐
│ Set Stake Amount                                 │
├─────────────────────────────────────────────────┤
│ Available Balance: 90.0000 ETR                  │
│                                                  │
│ Amount to Stake:                                 │
│ [50.0000] ETR                                    │
│                                                  │
│ [ Max ]  (leave some for fees!)                │
│                                                  │
│ Keep Available: 40.0000 ETR                     │
│                                                  │
│ Estimated Annual Rewards:                        │
│ 6.25 ETR (~12.5% APY)                           │
│                                                  │
│ ⓘ Unbonding period: 28 days                     │
│                                                  │
│ [ Back ]                    [ Review ]          │
└─────────────────────────────────────────────────┘
```

**How much to stake:**
- Leave at least 1-5 ETR available for transaction fees
- Consider future transaction needs
- You can always stake more later
- Unstaking takes 28 days (plan accordingly)

**Example allocation:**
- Total balance: 90 ETR
- Stake: 50 ETR (earn rewards)
- Keep available: 40 ETR (for fees, transactions, flexibility)

#### Step 4: Review and Confirm

1. Click **"Review"**
2. Verify all details:

```
┌─────────────────────────────────────────────────┐
│ Confirm Nomination                               │
├─────────────────────────────────────────────────┤
│ Stake Amount: 50.0000 ETR                       │
│                                                  │
│ Nominated Validators (3):                       │
│ • Etrid Foundation Node                         │
│ • Validator Alpha                               │
│ • StakingRewards.com                            │
│                                                  │
│ Estimated APY: 12.5%                            │
│ Estimated Monthly Rewards: 0.52 ETR             │
│ Estimated Annual Rewards: 6.25 ETR              │
│                                                  │
│ Unbonding Period: 28 days                       │
│                                                  │
│ Transaction Fee: 0.001 ETR                      │
│                                                  │
│ ⚠️  Your ETR will be locked for staking.        │
│    To unstake, you must wait 28 days.          │
│                                                  │
│ [ Back ]              [ Sign and Confirm ]     │
└─────────────────────────────────────────────────┘
```

3. Click **"Sign and Confirm"**
4. Enter your password
5. Wait for transaction confirmation

**Success!** You're now staking and will start earning rewards.

### Managing Your Stake

**View staking status:**

```
┌─────────────────────────────────────────────────────────────────┐
│ Your Active Staking                                              │
├─────────────────────────────────────────────────────────────────┤
│ Total Staked: 50.0000 ETR                                       │
│ Active Since: October 22, 2025                                   │
│                                                                   │
│ Nominated Validators: 3                                          │
│ ✓ Etrid Foundation Node (active)                                │
│ ✓ Validator Alpha (active)                                      │
│ ✓ StakingRewards.com (active)                                   │
│                                                                   │
│ Rewards Earned (this era): 0.0143 ETR                           │
│ Total Rewards Earned: 0.0143 ETR                                │
│ Current APY: 12.5%                                               │
│                                                                   │
│ [ Stake More ]  [ Change Validators ]  [ Unstake ]             │
└─────────────────────────────────────────────────────────────────┘
```

**Common actions:**

**Claim Rewards:**
- Rewards auto-compound by default
- Or click "Claim Rewards" to move to available balance
- No penalty for claiming

**Stake More:**
- Click "Stake More"
- Add additional ETR to existing stake
- Takes effect next era

**Change Validators:**
- Click "Change Validators"
- Select new validators (keeps same stake amount)
- Useful if validator performance degrades

**Unstake (Unbond):**
- Click "Unstake"
- Enter amount to unbond
- Wait 28 days for unbonding period
- After 28 days, click "Withdraw Unbonded" to receive ETR

### Understanding Rewards

**Reward calculation:**

```
Your Rewards = (Your Stake / Total Stake) × (Block Rewards + Fees) × (1 - Validator Commission)
```

**Factors affecting rewards:**
1. **Your stake amount:** More stake = more rewards
2. **Validator performance:** Uptime matters (offline = no rewards)
3. **Validator commission:** Lower commission = more for you
4. **Network participation:** Total staked affects reward rate
5. **Era points:** Validators with more era points earn more

**Reward distribution:**
- Paid every **era** (24 hours)
- Automatically added to your staked balance (compounding)
- Visible in "Pending Rewards" before distribution

**Example calculation:**

```
Your stake: 50 ETR
Total network stake: 10,000,000 ETR
Your share: 0.0005%

Era rewards pool: 1,000 ETR
Your validator's share: 50 ETR (5%)
Validator commission: 5%

Your rewards = 50 ETR × 0.0005% × 95% = 0.02375 ETR per era
Annual rewards ≈ 0.02375 × 365 = 8.66875 ETR
APY = 8.66875 / 50 = 17.3%
```

**Tax considerations:**
- Staking rewards may be taxable in your jurisdiction
- Keep records of rewards earned
- Consult tax professional for guidance
- Export transaction history for tax software

### Advanced Staking: Running a Validator

**Want to run your own validator?**

See the [Operator Guide](OPERATOR_GUIDE.md) for detailed instructions.

**Requirements:**
- Minimum stake: 64 ETR (Validity Node) or 20,000 ETR (full validator)
- Dedicated server (24/7 uptime)
- Technical expertise
- Time commitment for maintenance

**Benefits:**
- Higher rewards (earn commission from nominators)
- Full control over operations
- Contribute directly to network security
- Governance influence

---

## Governance Participation

### Why Participate in Governance?

Etrid is a **democratic blockchain** where stakeholders control the network's future.

**What you can vote on:**
- Network upgrades and parameter changes
- Treasury spending proposals
- Emergency interventions
- Fiscal policy (on Consensus Day)

**Your voting power** = your staked ETR tokens

### Types of Governance

#### 1. On-Chain Governance (Continuous)

**Proposals:**
- Technical upgrades
- Parameter changes
- Treasury allocations

**How to vote:**

1. Navigate to **"Governance"** in wallet
2. See active proposals:

```
┌─────────────────────────────────────────────────────────────────┐
│ Active Proposals                                                 │
├─────────────────────────────────────────────────────────────────┤
│ Proposal #42: Increase Block Size to 5MB                        │
│ Proposed by: TechnicalCommittee                                 │
│ Status: Voting (5 days remaining)                               │
│                                                                   │
│ Description:                                                     │
│ This proposal increases the maximum block size from 3MB to 5MB  │
│ to accommodate growing transaction volume while maintaining     │
│ decentralization.                                                │
│                                                                   │
│ Current Tally:                                                   │
│ AYE: 45,234,567 ETR (67%)                                       │
│ NAY: 22,345,123 ETR (33%)                                       │
│                                                                   │
│ Your Voting Power: 50 ETR                                        │
│                                                                   │
│ [ Vote AYE ]  [ Vote NAY ]  [ View Discussion ]                │
└─────────────────────────────────────────────────────────────────┘
```

3. Read proposal details and discussion
4. Click **"Vote AYE"** (support) or **"Vote NAY"** (oppose)
5. Optionally, set conviction multiplier (lock tokens longer for more voting power)
6. Confirm transaction

**Conviction voting:**
- Lock tokens for longer = more voting power
- Conviction 0x: No lock, 1x voting power
- Conviction 1x: Lock 1 week, 1x voting power
- Conviction 2x: Lock 2 weeks, 2x voting power
- ...
- Conviction 6x: Lock 32 weeks, 6x voting power

**Example:**
- You have 50 ETR staked
- Choose conviction 2x (lock 2 weeks)
- Your vote counts as 100 ETR voting power

#### 2. Consensus Day (Annual Event)

**What is Consensus Day?**

An annual event (February 14th) where stakeholders vote on:
- Fiscal policy (inflation rate, fee distribution)
- Major network upgrades
- Strategic direction
- Community initiatives

**How to participate:**

1. **Before Consensus Day:**
   - Review proposals published 30 days prior
   - Join community discussions
   - Research implications of each proposal

2. **On Consensus Day:**
   - Open wallet and navigate to "Consensus Day Voting"
   - Vote on each proposal
   - Votes weighted by staked ETR

3. **After Consensus Day:**
   - Results announced within 24 hours
   - Approved changes implemented in next network upgrade

**Example Consensus Day ballot:**

```
┌─────────────────────────────────────────────────────────────────┐
│ Consensus Day 2026 Ballot                                        │
├─────────────────────────────────────────────────────────────────┤
│ Your Voting Power: 50 ETR                                        │
│                                                                   │
│ Proposal 1: Set Inflation Rate for 2026                         │
│ ○ Option A: 5% inflation (more rewards, more supply)           │
│ ○ Option B: 3% inflation (balanced approach)                   │
│ ● Option C: 2% inflation (conservative, lower rewards)         │
│                                                                   │
│ Proposal 2: Treasury Allocation                                  │
│ ○ 60% Development, 30% Marketing, 10% Community                │
│ ● 50% Development, 30% Marketing, 20% Community                │
│ ○ 70% Development, 20% Marketing, 10% Community                │
│                                                                   │
│ Proposal 3: Enable EVM Compatibility on FlareChain              │
│ ● YES (support Ethereum smart contracts)                        │
│ ○ NO (keep Wasm only)                                           │
│                                                                   │
│ [ Submit Ballot ]                                               │
└─────────────────────────────────────────────────────────────────┘
```

#### 3. Treasury Proposals

**The Etrid Treasury** holds funds for ecosystem development.

**Anyone can submit proposals:**
1. Navigate to "Treasury" in governance section
2. Click "Submit Proposal"
3. Describe project and requested funding
4. Pay deposit (refundable if approved, forfeited if rejected)

**Proposal evaluation:**
- Council reviews proposals
- Community discusses merits
- Token holders vote
- Approved proposals receive funding

**Example proposal:**
```
Proposal: Build Etrid Mobile Wallet for iOS
Requested: 10,000 ETR
Proposer: MobileDev Team
Status: Under Review

Description:
We propose to build a native iOS mobile wallet with the following features:
- Send/receive transactions
- QR code scanning
- Biometric authentication
- Staking interface
- DApp browser

Team: 3 full-time developers with proven track record
Timeline: 3 months
Deliverables: Open-source code, App Store release
```

### Voting Best Practices

**Before voting:**
- ✅ Read full proposal and discussion
- ✅ Understand implications of YES/NO votes
- ✅ Research proposer's background
- ✅ Consider technical feasibility
- ✅ Think about long-term consequences

**When voting:**
- ✅ Vote your genuine conviction
- ✅ Use conviction multiplier for proposals you strongly support
- ✅ Don't blindly follow others
- ✅ Participate in discussions

**After voting:**
- ✅ Track proposal outcome
- ✅ Hold implementers accountable
- ✅ Provide feedback on execution

---

## Using Cross-Chain Features

### What is Cross-Chain?

Etrid connects 13 major blockchains through **Partition Burst Chains (PBCs)**, allowing you to:
- Bridge assets between chains
- Use Bitcoin, Ethereum, Solana, etc., within Etrid
- Access DeFi across multiple ecosystems

**Supported chains:**
1. Bitcoin (BTC)
2. Ethereum (ETH)
3. Solana (SOL)
4. Cardano (ADA)
5. Polkadot (DOT)
6. Binance Smart Chain (BNB)
7. Avalanche (AVAX)
8. Polygon (MATIC)
9. Cosmos (ATOM)
10. Algorand (ALGO)
11. Tezos (XTZ)
12. Near Protocol (NEAR)
13. EDSC (Etrid Digital Standard Currency)

### Bridging Assets to Etrid

**Example: Bridge ETH from Ethereum to Etrid**

#### Step 1: Navigate to Bridge

1. Go to [bridge.etrid.io](https://bridge.etrid.io) or click "Bridge" in wallet
2. Select chains:
   - **From:** Ethereum
   - **To:** Etrid (PBC-ETH)

#### Step 2: Connect Source Wallet

1. Click "Connect Ethereum Wallet"
2. Connect MetaMask or other Ethereum wallet
3. Approve connection

#### Step 3: Initiate Bridge

```
┌─────────────────────────────────────────────────┐
│ Bridge Assets                                    │
├─────────────────────────────────────────────────┤
│ From: Ethereum Mainnet                          │
│ Wallet: 0x742d35Cc6634C0532925a3b844Bc9e7595f0│
│                                                  │
│ To: Etrid PBC-ETH                               │
│ Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNe...  │
│                                                  │
│ Asset: ETH                                       │
│ Amount: [1.0] ETH                                │
│                                                  │
│ Bridge Fee: 0.001 ETH (~$2.50)                  │
│ Estimated Time: 15-30 minutes                    │
│                                                  │
│ You will receive: 0.999 ETH on Etrid PBC-ETH   │
│                                                  │
│ [ Review Bridge Transaction ]                   │
└─────────────────────────────────────────────────┘
```

1. Enter amount to bridge
2. Click "Review Bridge Transaction"
3. Verify details carefully
4. Click "Approve" to authorize spending (Ethereum)
5. Click "Bridge" to execute transaction

#### Step 4: Wait for Confirmations

**Bridge process:**
1. **Ethereum confirmation:** 12 blocks (~3 minutes)
2. **Relay to Etrid:** Light client verification (~2 minutes)
3. **PBC minting:** Issue wrapped ETH on PBC-ETH (~1 minute)
4. **Finalization:** 3 Etrid blocks (~15 seconds)

**Total time:** 15-30 minutes

**Track status:**
- Click "Track Transaction" link
- See real-time progress updates
- Receive notification when complete

#### Step 5: Use Bridged Assets

- Bridged ETH appears in your Etrid wallet on PBC-ETH chain
- Use for DeFi, trading, staking, or gas fees on PBC
- Bridge back to Ethereum anytime (same process in reverse)

### Bridging Assets from Etrid

**Example: Bridge ETH back to Ethereum**

1. Navigate to bridge interface
2. Select:
   - **From:** Etrid (PBC-ETH)
   - **To:** Ethereum Mainnet
3. Enter amount and destination address
4. Pay bridge fee (in ETR or ETH)
5. Wait for multi-sig custodian approval (~10 minutes)
6. Receive ETH on Ethereum Mainnet

**Security:** Bridges use multi-signature custodians and light client verification for maximum security.

### Using EDSC Stablecoin

**EDSC (Etrid Digital Standard Currency)** is a USD-pegged stablecoin backed by real-world reserves.

**Use cases:**
- Stable store of value (not volatile like ETR)
- Trading pair on DEXs
- DeFi collateral
- Remittances and payments

**How to get EDSC:**

1. **Mint EDSC** (deposit collateral):
   - Navigate to "EDSC" section in wallet
   - Deposit accepted assets (BTC, ETH, stablecoins)
   - Receive EDSC at 1:1 USD value

2. **Buy on DEX:**
   - Use EtridSwap or other DEX
   - Trade ETR or other tokens for EDSC

3. **Receive from others:**
   - Accept EDSC payments
   - Works like any other token

**Redeeming EDSC:**
- Burn EDSC to receive underlying collateral
- 1 EDSC always redeemable for $1 worth of reserves
- Small redemption fee (0.1%)

---

## Common Tasks

### Exporting Account

**Use case:** Move account to another device or wallet app.

**Steps:**
1. Go to wallet settings
2. Click "Export Account"
3. Enter password
4. You'll see your recovery phrase or JSON file
5. Save securely (never screenshot!)

**Export formats:**
- **Recovery Phrase:** 12 words (compatible with any wallet)
- **JSON Keyfile:** Encrypted file (requires password)

### Importing Account

**Use case:** Restore existing account or import from another wallet.

**Method 1: From Recovery Phrase**

1. Open wallet
2. Click "Import Account"
3. Select "From Seed Phrase"
4. Enter your 12-word recovery phrase
5. Set new password
6. Name your account
7. Account restored with all funds

**Method 2: From JSON Keyfile**

1. Open wallet
2. Click "Import Account"
3. Select "From JSON File"
4. Upload JSON file
5. Enter file password
6. Account imported

### Creating Multi-Signature Accounts

**Multi-sig** accounts require multiple signatures to execute transactions (enhanced security).

**Use cases:**
- Joint accounts (couples, business partners)
- DAO treasuries
- Cold storage for large amounts
- Corporate accounts with multiple approvers

**How to create:**

1. Navigate to "Accounts" → "Multi-Sig"
2. Click "Create Multi-Sig"
3. Add signatories (2-20 addresses)
4. Set threshold (e.g., 2-of-3, 3-of-5)
5. Create multi-sig account

**Example:**
```
Multi-Sig Configuration:
Signatories:
1. Alice (5GrwvaEF...)
2. Bob (5HpG9w8E...)
3. Carol (5FHneW46...)

Threshold: 2 of 3
(any 2 signatures required to execute transactions)
```

**Using multi-sig:**
1. One signer initiates transaction
2. Transaction status: "Pending (1/2 approvals)"
3. Other signers approve via wallet
4. After threshold reached, transaction executes

### Managing Multiple Accounts

**Switch between accounts:**

1. Click account dropdown (top right)
2. Select account from list
3. Wallet switches to selected account

**Rename account:**
1. Click account dropdown
2. Click "Edit" icon next to account name
3. Enter new name
4. Save

**Hide account:**
1. Go to settings → "Accounts"
2. Click "Hide" next to account
3. Account hidden from dropdown (still accessible in settings)

### Connecting to DApps

**Etrid DApps** can request access to your wallet.

**Connection flow:**

1. Visit DApp website (e.g., etridswap.io)
2. Click "Connect Wallet"
3. Wallet shows permission request:

```
┌─────────────────────────────────────────────────┐
│ Connection Request                               │
├─────────────────────────────────────────────────┤
│ EtridSwap wants to connect                      │
│ https://etridswap.io                            │
│                                                  │
│ This site is requesting access to:              │
│ • View your account address                     │
│ • Request transaction signatures                │
│                                                  │
│ This site will NOT be able to:                  │
│ • Access your private keys                      │
│ • Send transactions without your approval       │
│                                                  │
│ Account to connect:                              │
│ ○ Main Account (5Grwva...)                     │
│ ○ Trading Account (5HpG9w...)                  │
│                                                  │
│ [ Reject ]                    [ Connect ]       │
└─────────────────────────────────────────────────┘
```

4. Select account to connect
5. Click "Connect"

**When DApp requests transaction:**
- Wallet shows transaction details
- You must approve each transaction
- Never approve suspicious transactions

**Disconnect DApp:**
1. Go to wallet settings → "Connected Sites"
2. Find DApp in list
3. Click "Disconnect"

---

## Troubleshooting

### Common Issues and Solutions

#### Issue: Wallet won't load / connection error

**Symptoms:**
- Wallet shows "Connecting..."
- Error message "WebSocket connection failed"

**Solutions:**

1. **Check internet connection:**
   - Verify you're online
   - Try refreshing the page

2. **Check network status:**
   - Visit [status.etrid.io](https://status.etrid.io)
   - Verify FlareChain is operational

3. **Try different node:**
   - Settings → "Network"
   - Select different RPC endpoint
   - Click "Connect"

4. **Clear browser cache:**
   - Ctrl+Shift+Delete (Chrome/Firefox)
   - Select "Cached images and files"
   - Clear and refresh wallet

5. **Try different browser:**
   - Test in Chrome, Firefox, or Brave
   - Ensure JavaScript is enabled

#### Issue: Transaction stuck in "Pending"

**Symptoms:**
- Transaction shows 0/3 confirmations for > 5 minutes
- Status doesn't update

**Solutions:**

1. **Wait longer:**
   - Network may be congested
   - Can take up to 10 minutes in rare cases

2. **Check transaction hash:**
   - Copy transaction hash
   - View in block explorer
   - See actual status

3. **Verify sufficient fee:**
   - Low fee may delay transaction
   - Future transactions: increase fee

4. **Check nonce conflict:**
   - Multiple simultaneous transactions may conflict
   - Wait for earlier transaction to confirm

5. **Contact support:**
   - If stuck > 1 hour, report to Discord
   - Provide transaction hash

#### Issue: Incorrect balance showing

**Symptoms:**
- Balance doesn't match expected amount
- Recent transaction not reflected

**Solutions:**

1. **Refresh wallet:**
   - Click refresh icon
   - Or hard refresh (Ctrl+Shift+R)

2. **Check transaction history:**
   - Verify all transactions processed
   - Look for unexpected outgoing transactions

3. **Verify correct network:**
   - Ensure connected to correct network (testnet vs mainnet)
   - Check chain selector

4. **Check staked/reserved balance:**
   - Available balance excludes staked amounts
   - Click "Show breakdown" to see allocation

5. **Resync wallet:**
   - Settings → "Advanced"
   - Click "Resync Wallet"
   - Wait for completion

#### Issue: Can't unstake / unbonding stuck

**Symptoms:**
- "Unstake" button greyed out
- Unbonding period not progressing

**Solutions:**

1. **Check unbonding period:**
   - 28 days required for unbonding
   - View progress in staking dashboard

2. **Wait for era change:**
   - Unbonding starts at next era boundary
   - Eras last 24 hours

3. **Verify unbonding status:**
   - Check if unbonding already initiated
   - View in "Unbonding" section

4. **Withdraw after completion:**
   - After 28 days, click "Withdraw Unbonded"
   - Funds return to available balance

#### Issue: Recovery phrase not working

**Symptoms:**
- "Invalid recovery phrase" error
- Restores wrong account

**Solutions:**

1. **Check word spelling:**
   - Every word must be exact
   - Use BIP39 word list for reference
   - Common mistakes: "woman" vs "women", "through" vs "tough"

2. **Verify word order:**
   - Order matters - must be exact
   - Double-check against written backup

3. **Try different derivation path:**
   - Some wallets use different paths
   - Advanced settings → "Derivation Path"

4. **Check for extra spaces:**
   - Remove leading/trailing spaces
   - Single space between words

5. **Contact support:**
   - If verified correct, may be wallet software issue
   - Provide account address (not recovery phrase!)

#### Issue: High transaction fees

**Symptoms:**
- Fees higher than expected
- "Insufficient funds for fee" error

**Solutions:**

1. **Check network congestion:**
   - High demand = higher fees
   - Wait for off-peak hours

2. **Simplify transaction:**
   - Remove unnecessary data/memo
   - Reduce complexity

3. **Ensure sufficient balance:**
   - Keep extra ETR for fees
   - Minimum 0.01 ETR recommended

4. **Use Lightning-Bloc:**
   - For frequent small payments
   - Much lower fees

### Getting Help

**Official support channels:**

1. **Documentation:**
   - [docs.etrid.io](https://docs.etrid.io) - comprehensive guides
   - [API Reference](API_REFERENCE.md) - technical documentation
   - [Developer Guide](DEVELOPER_GUIDE.md) - building on Etrid

2. **Community:**
   - Discord: [discord.gg/etrid](https://discord.gg/etrid)
   - Telegram: [t.me/EtridOfficial](https://t.me/EtridOfficial)
   - Reddit: [r/EtridBlockchain](https://reddit.com/r/EtridBlockchain)

3. **Social Media:**
   - Twitter: [@EtridMultichain](https://twitter.com/EtridMultichain)
   - YouTube: [Etrid Tutorials](https://youtube.com/etrid)
   - Blog: [blog.etrid.io](https://blog.etrid.io)

4. **Support Tickets:**
   - Email: support@etrid.io
   - Include: wallet address, transaction hash, screenshots
   - Response time: 24-48 hours

**Before asking for help:**
- ✅ Search documentation and FAQ
- ✅ Check Discord/Telegram for similar issues
- ✅ Gather relevant information (transaction hash, error messages)
- ✅ Describe steps you've already tried

**NEVER share:**
- ❌ Your recovery phrase
- ❌ Your private keys
- ❌ Your wallet password
- ❌ Screenshots of recovery phrase

---

## FAQ

### General Questions

**Q: What is Etrid?**

A: Etrid is a multichain blockchain platform that connects 13 major blockchains, enabling cross-chain transactions, DeFi, and decentralized governance with 15-second finality.

**Q: How is Etrid different from other blockchains?**

A: Etrid offers unique features including 13 cross-chain bridges (PBCs), annual Consensus Day governance, Lightning-Bloc Layer 2 payments, and Ascending Scale of Finality consensus.

**Q: Is Etrid a fork of Polkadot?**

A: Etrid is built using Substrate (same framework as Polkadot) but is a completely independent network with unique consensus, governance, and multichain architecture.

**Q: What's the relationship between ETR and EDSC?**

A: ETR is the native token for staking, fees, and governance. EDSC is a USD-pegged stablecoin backed by reserves. Both exist on the Etrid blockchain.

### Account & Security

**Q: I lost my recovery phrase. Can I recover my account?**

A: Unfortunately, no. Without the recovery phrase, your account cannot be recovered. This is a fundamental security feature - no one can reset or recover it for you.

**Q: Can I change my recovery phrase?**

A: No. The recovery phrase is cryptographically derived from your account. To change it, you must create a new account and transfer funds.

**Q: Is my password the same as my recovery phrase?**

A: No! Your password encrypts the account locally in your browser. Your recovery phrase is the master key that works on any device. Both are important but serve different purposes.

**Q: Someone knows my account address. Are my funds at risk?**

A: No. Your account address is public and safe to share. Only your recovery phrase can control funds.

### Transactions

**Q: How long do transactions take?**

A: Transactions appear in ~5 seconds and achieve finality in ~15 seconds (3 block confirmations).

**Q: Can I cancel a transaction?**

A: Once submitted, transactions cannot be canceled. They're either confirmed or rejected (if invalid). Always double-check before sending!

**Q: What happens if I send to the wrong address?**

A: Blockchain transactions are irreversible. Funds sent to the wrong address are lost unless you can contact the recipient and request a return.

**Q: Why is my transaction fee so high?**

A: Fees increase during network congestion. Typical fees are 0.0001-0.001 ETR. If fees seem excessive, try waiting for off-peak hours.

### Staking

**Q: What's the minimum amount to stake?**

A: You can nominate validators with as little as 1 ETR. To run your own validator, you need 64 ETR minimum (or 20,000 ETR for full validator).

**Q: How often are staking rewards paid?**

A: Rewards are distributed every era (24 hours) and automatically added to your staked balance (compounding).

**Q: Can I unstake anytime?**

A: You can initiate unstaking anytime, but there's a 28-day unbonding period before funds become available.

**Q: What happens if my validator goes offline?**

A: You won't earn rewards for the period they're offline. If they're slashed for misbehavior, you may lose a small portion of stake. This is why diversifying across multiple validators is important.

**Q: Do rewards compound automatically?**

A: Yes! By default, rewards are added to your staked balance and earn additional rewards (compound interest).

### Governance

**Q: How much voting power do I have?**

A: Your voting power equals your staked ETR. You can multiply this using conviction voting (lock tokens longer for more power).

**Q: When is Consensus Day?**

A: Consensus Day occurs annually on February 14th. It's a 24-hour voting period for major fiscal and governance decisions.

**Q: Do I have to vote?**

A: No, voting is optional. However, participating in governance helps shape the network's future and is encouraged.

**Q: Can I delegate my vote to someone else?**

A: Yes! You can delegate voting power to a representative you trust. This is useful if you don't have time to research every proposal.

### Cross-Chain

**Q: Which chains does Etrid support?**

A: Etrid supports 13 chains: Bitcoin, Ethereum, Solana, Cardano, Polkadot, Binance Smart Chain, Avalanche, Polygon, Cosmos, Algorand, Tezos, Near, and EDSC.

**Q: How long does bridging take?**

A: Bridge times vary by source chain. Ethereum bridges take ~15-30 minutes. Faster chains like Solana take ~5-10 minutes.

**Q: Are bridges safe?**

A: Etrid bridges use multi-signature custodians and light client verification for security. While no bridge is 100% risk-free, Etrid's architecture minimizes risks.

**Q: Can I bridge any token?**

A: Currently, you can bridge native tokens (BTC, ETH, SOL, etc.) and major stablecoins. Support for additional tokens is being added.

### Technical

**Q: What consensus does Etrid use?**

A: Ascending Scale of Finality (ASF) - a novel mechanism that combines proof of stake with coinage tracking to prevent centralization.

**Q: How many transactions per second can Etrid handle?**

A: FlareChain handles ~1,000 TPS. With Lightning-Bloc Layer 2, throughput scales to ~100,000 TPS.

**Q: Is Etrid compatible with Ethereum?**

A: Etrid has a PBC for Ethereum bridging. Future updates may add EVM compatibility for running Ethereum smart contracts directly.

**Q: Can I run a node without being a validator?**

A: Yes! You can run a full node to sync the blockchain, query data, and submit transactions without staking or validating.

---

## Conclusion

Congratulations on completing the Etrid User Guide! You now have the knowledge to:

- ✅ Create and secure your wallet
- ✅ Send and receive transactions
- ✅ Stake tokens and earn rewards
- ✅ Participate in governance
- ✅ Use cross-chain features
- ✅ Troubleshoot common issues

### Next Steps

**Continue Learning:**
- 📖 Read the [Developer Guide](DEVELOPER_GUIDE.md) to build on Etrid
- 📖 Read the [Operator Guide](OPERATOR_GUIDE.md) to run a validator
- 🎥 Watch video tutorials at [youtube.com/etrid](https://youtube.com/etrid)

**Join the Community:**
- 💬 Discord: [discord.gg/etrid](https://discord.gg/etrid)
- 💬 Telegram: [t.me/EtridOfficial](https://t.me/EtridOfficial)
- 🐦 Twitter: [@EtridMultichain](https://twitter.com/EtridMultichain)

**Stay Updated:**
- 📧 Newsletter: [etrid.io/newsletter](https://etrid.io/newsletter)
- 📝 Blog: [blog.etrid.io](https://blog.etrid.io)
- 📊 Network Stats: [stats.etrid.io](https://stats.etrid.io)

### We'd Love Your Feedback!

Help us improve this guide:
- 📧 Email: docs@etrid.io
- 💡 Suggest edits: [github.com/etrid/docs](https://github.com/etrid/docs)
- ⭐ Rate this guide: [feedback.etrid.io](https://feedback.etrid.io)

---

**Welcome to the Etrid ecosystem!**

*The free and open decentralized democracy of stakeholders awaits.*

---

**Document Information**

- **Version:** 1.0.0
- **Last Updated:** October 22, 2025
- **License:** CC BY-SA 4.0
- **Maintained by:** Etrid Foundation
- **Contributors:** Etrid Community

**Found an error?** Submit an issue at [github.com/etrid/docs/issues](https://github.com/etrid/docs/issues)
