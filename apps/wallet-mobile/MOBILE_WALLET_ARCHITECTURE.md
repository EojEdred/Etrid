# Ëtrid Mobile DeFi Wallet - Complete Architecture

**Project Name**: Ëtrid Wallet (Mobile)
**Vision**: The world's first comprehensive crypto bank account with ATM access, cold storage integration, and full DeFi features
**Target Platforms**: iOS 15+, Android 11+
**Date**: November 18, 2025

---

## 🎯 Vision Statement

Create a **robust mobile DeFi wallet** that functions like a traditional bank account but for crypto, featuring:
- Bank-like user experience (checking/savings)
- Crypto ATM access for cash withdrawals
- Ledger/DEGN/cold storage wallets as "debit cards"
- Direct staking and governance voting
- All financial features wrapped into one comprehensive app

**Design Philosophy**: Hide blockchain complexity, show familiar banking concepts.

---

## 🏗️ Technical Stack

### Frontend (Mobile)
**Choice**: **React Native** (Expo framework)

**Why React Native**:
- Single codebase for iOS + Android
- JavaScript SDK already complete (13 wrappers)
- Fast development with Expo
- Great UI libraries (React Native Paper, NativeBase)
- Hardware wallet support via Bluetooth/USB
- Biometric authentication built-in
- Push notifications via Expo

**Alternatives Considered**:
- Flutter: Would require Dart bindings for SDK
- Native (Swift/Kotlin): 2x development time

**Key Dependencies**:
```json
{
  "dependencies": {
    "@etrid/sdk": "^1.0.0",
    "@polkadot/api": "^10.0.0",
    "react-native": "^0.72.0",
    "expo": "~49.0.0",
    "react-native-paper": "^5.10.0",
    "@react-navigation/native": "^6.1.0",
    "@react-navigation/bottom-tabs": "^6.5.0",
    "@react-navigation/stack": "^6.3.0",
    "react-native-biometrics": "^3.0.1",
    "react-native-ble-plx": "^3.0.3",
    "react-native-qrcode-svg": "^6.2.0",
    "react-native-camera": "^4.2.1",
    "react-native-chart-kit": "^6.12.0",
    "react-native-push-notification": "^8.1.1",
    "react-native-secure-storage": "^3.0.0",
    "bignumber.js": "^9.1.0",
    "ethers": "^6.7.0",
    "web3": "^4.0.0"
  }
}
```

### Backend Services
**Architecture**: Microservices on Node.js

**Services**:
1. **API Gateway** (Express.js + GraphQL)
   - User authentication (JWT)
   - Rate limiting
   - API versioning

2. **Blockchain Indexer** (PostgreSQL + Redis)
   - Transaction history
   - Balance caching
   - Price feed caching
   - Notification triggers

3. **ATM Integration Service**
   - Partner API connections (Coinme, Bitcoin Depot, CoinFlip)
   - Location services
   - Cash-out requests
   - Fee calculations

4. **Notification Service**
   - Push notifications (Expo Push)
   - Email alerts
   - SMS 2FA

5. **Analytics Service**
   - Portfolio tracking
   - Performance metrics
   - User insights

**Database**:
- **PostgreSQL**: User accounts, transaction history, settings
- **Redis**: Session cache, price cache, rate limiting
- **IPFS**: Document storage (KYC, receipts)

**Deployment**:
- **Cloud**: AWS (primary), Azure (backup)
- **Containers**: Docker + Kubernetes
- **CDN**: CloudFlare for API acceleration
- **Monitoring**: Datadog, Sentry

---

## 📱 Feature Breakdown

### 1. **Home Dashboard** 💰
**SDK Wrappers Used**: AccountsWrapper, OracleWrapper, StakingWrapper

**Features**:
- Total balance in USD (auto-converted)
- Quick actions: Send, Receive, Swap, Stake
- Recent transactions (last 10)
- Asset breakdown chart (pie chart)
- Price alerts (e.g., "BTC +5.2% today")

**UI Components**:
```typescript
<Dashboard>
  <BalanceCard totalUSD={balanceUSD} />
  <QuickActions
    onSend={() => navigate('Send')}
    onReceive={() => navigate('Receive')}
    onSwap={() => navigate('Swap')}
    onStake={() => navigate('Stake')}
  />
  <AssetList assets={portfolio} />
  <TransactionHistory recent={transactions} />
</Dashboard>
```

**SDK Integration**:
```typescript
import { AccountsWrapper, OracleWrapper } from '@etrid/sdk';

// Get total balance
const accounts = new AccountsWrapper(api);
const oracle = new OracleWrapper(api);

const balance = await accounts.getBalance(address);
const etrPrice = await oracle.getPrice('ETR/USD');
const balanceUSD = (balance / 1e18) * etrPrice;
```

---

### 2. **Bank Accounts** (Checking/Savings) 🏦
**SDK Wrappers Used**: ReserveVaultWrapper, StakingWrapper

**Features**:
- **Checking Account**: Daily spending wallet
  - Instant transfers
  - Low/zero fees
  - No minimum balance
  - ATM access enabled

- **Savings Account**: High-yield DeFi vault
  - 15% APY (via ReserveVault)
  - Auto-compound interest
  - No lock-up period
  - Health factor monitoring

**UI Design**:
```
┌─────────────────────────────────┐
│ 💳 Checking Account             │
│ $12,450.32                      │
│ Available for spending          │
│ [Send Money] [Add Funds]        │
└─────────────────────────────────┘

┌─────────────────────────────────┐
│ 📈 Savings Account              │
│ $50,000.00                      │
│ Earning 15% APY                 │
│ +$20.55 today                   │
│ Health: ✅ Excellent (210%)     │
│ [Deposit] [Withdraw]            │
└─────────────────────────────────┘
```

**SDK Integration**:
```typescript
import { ReserveVaultWrapper } from '@etrid/sdk';

// Create savings vault
const vault = new ReserveVaultWrapper(api);
const savingsVault = await vault.createVault(keypair, 'SAVINGS');

// Deposit to savings (80% of amount as collateral)
await vault.depositCollateral(
  keypair,
  savingsVault.id,
  depositAmount * 0.8
);

// Borrow 50% to earn yield elsewhere
const borrowAmount = (depositAmount * 0.8) * 0.5;
await vault.borrow(keypair, savingsVault.id, borrowAmount);

// Show health factor
const health = await vault.getHealthFactor(savingsVault.id);
// Display: "Health: ✅ Excellent (210%)"
```

---

### 3. **Send & Receive** 💸
**SDK Wrappers Used**: AccountsWrapper, LightningBlocWrapper

**Features**:
- QR code scanning for addresses
- Contact book integration
- Lightning-fast payments (500K TPS via Lightning-Bloc)
- Multi-asset support (13 chains)
- Bill splitting
- Recurring payments
- Payment requests

**UI Flow**:
```
Send Money:
1. Enter amount: [$100] or [0.5 ETR]
2. Select recipient: [Contact] [QR Code] [Address]
3. Choose speed: [Instant (L3)] [Fast (L2)] [Standard (L1)]
4. Review: "Sending $100 to Alice, Fee: $0.01"
5. Authenticate: [Face ID] [Fingerprint] [PIN]
6. Confirm: "✅ Sent!"
```

**SDK Integration**:
```typescript
import { LightningBlocWrapper } from '@etrid/sdk';

// Open Lightning-Bloc channel for instant payments
const lightning = new LightningBlocWrapper(api);
const channel = await lightning.openChannel(
  keypair,
  recipientAddress,
  channelCapacity
);

// Send instant payment (500K TPS, <1s finality)
await lightning.sendPayment(
  channel.channelId,
  amount,
  recipientAddress
);

// User sees: "✅ Sent $100 to Alice (0.3s)"
```

---

### 4. **Crypto ATM Integration** 🏧
**SDK Wrappers Used**: BridgeWrapper, OracleWrapper, AccountsWrapper

**Architecture**:

```
User Wallet → Backend API → ATM Partner API → Physical ATM
                    ↓
              Blockchain TX → Confirm → Dispense Cash
```

**Partners**:
- **Coinme**: 40,000+ ATMs (Coinstar kiosks)
- **Bitcoin Depot**: 7,000+ ATMs
- **CoinFlip**: 4,500+ ATMs
- **Athena Bitcoin**: 1,500+ ATMs

**Features**:
- **Find ATMs Near Me**: GPS-based map view
- **Cash Withdrawal**: Convert ÉTR/BTC/ETH → USD cash
- **Withdrawal Limits**: $500-$3,000 per day (partner dependent)
- **Fees**: 7-12% (industry standard)
- **No Account Needed**: Just QR code + PIN

**UI Design**:
```
┌─────────────────────────────────┐
│ 🏧 ATM Locator                  │
│                                 │
│ [Map View]                      │
│  📍 Your Location               │
│  🏧 Coinme ATM - 0.3 mi        │
│  🏧 Bitcoin Depot - 0.8 mi     │
│                                 │
│ [List View]                     │
│ ┌─────────────────────────────┐ │
│ │ 🏧 Coinme ATM               │ │
│ │ 123 Main St                 │ │
│ │ 0.3 mi · Open 24/7          │ │
│ │ Fee: 8% · Limit: $1,000/day │ │
│ │ [Get Directions] [Withdraw] │ │
│ └─────────────────────────────┘ │
└─────────────────────────────────┘

Withdrawal Flow:
1. Select amount: [$100] [$200] [$500] [Custom]
2. Select asset: [ÉTR] [BTC] [ETH]
3. Convert: "0.5 ÉTR = $100 (fee: $8)"
4. Generate code: "Your code: 8472-3951"
5. Go to ATM: "Enter code at Coinme ATM"
6. Receive cash: "✅ Withdrawal complete"
```

**SDK Integration**:
```typescript
import { BridgeWrapper, OracleWrapper } from '@etrid/sdk';

// Step 1: User initiates cash withdrawal
const withdrawalAmount = 100; // $100 USD
const asset = 'ETR';

// Step 2: Get conversion rate
const oracle = new OracleWrapper(api);
const etrPrice = await oracle.getPrice('ETR/USD');
const etrAmount = withdrawalAmount / etrPrice;

// Step 3: Bridge to partner's wallet (if needed)
const bridge = new BridgeWrapper(api);
if (asset !== 'ETR') {
  await bridge.bridge(keypair, asset, 'ETR', etrAmount);
}

// Step 4: Submit withdrawal request to backend
const withdrawalCode = await apiGateway.createATMWithdrawal({
  user: address,
  amount: withdrawalAmount,
  asset: asset,
  atmPartner: 'Coinme',
  atmLocation: atmId
});

// Step 5: User sees code: "8472-3951"
// Step 6: User enters code at ATM
// Step 7: ATM partner API confirms blockchain TX
// Step 8: ATM dispenses cash
```

**Backend API Endpoints**:
```typescript
// POST /api/v1/atm/withdraw
{
  "user": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  "amount": 100,
  "currency": "USD",
  "asset": "ETR",
  "atmPartner": "Coinme",
  "atmLocationId": "coinme_12345"
}

// Response:
{
  "withdrawalCode": "8472-3951",
  "expiresAt": "2025-11-18T15:30:00Z",
  "fee": 8.00,
  "total": 108.00,
  "txHash": "0xabc123..."
}
```

---

### 5. **Cold Storage "Debit Card"** 💳
**SDK Wrappers Used**: LedgerHardwareWrapper

**Concept**: Ledger/DEGN as the physical "debit card" for high-value transactions

**Features**:
- Bluetooth/NFC connection to Ledger Nano X
- On-device transaction approval
- BIP44 path: m/44'/354'/0'/0/0
- Multi-account support (checking/savings)
- Spend limits (require Ledger for >$500)

**UI Flow**:
```
┌─────────────────────────────────┐
│ 💳 Connected Devices             │
│                                 │
│ ✅ Ledger Nano X                │
│    Battery: 85%                 │
│    Accounts: 3                  │
│    [Manage] [Disconnect]        │
│                                 │
│ ⏹ DEGN Wallet                   │
│    Not connected                │
│    [Connect via Bluetooth]      │
└─────────────────────────────────┘

Transaction Flow (>$500):
1. User initiates: "Send $1,000 to Bob"
2. App detects: "Amount requires Ledger"
3. Prompt: "Connect your Ledger Nano X"
4. Bluetooth pairs
5. Ledger screen: "Approve transaction?"
6. User presses button on Ledger
7. App: "✅ Transaction signed and sent"
```

**SDK Integration**:
```typescript
import { LedgerHardwareWrapper, AccountsWrapper } from '@etrid/sdk';

// Connect to Ledger via Bluetooth
const ledger = new LedgerHardwareWrapper(api);
const device = await ledger.connectLedger('bluetooth');

// Get Ledger account address
const ledgerAccount = await ledger.getAddress(device, 0); // Account 0

// High-value transaction (>$500)
const amount = 1000; // $1,000
if (amount > 500) {
  // Require Ledger signing
  const accounts = new AccountsWrapper(api);
  const tx = await accounts.transfer(
    ledgerAccount.address,
    recipientAddress,
    amount
  );

  // Sign with Ledger (user presses button on device)
  const signature = await ledger.signTransaction(device, tx);

  // Submit signed transaction
  await api.tx.send(signature);

  // User sees: "✅ Transaction signed with Ledger and sent"
} else {
  // Standard signing (app keychain)
  await accounts.transfer(keypair, recipientAddress, amount);
}
```

**Security Model**:
- **<$100**: Biometric only (Face ID/Fingerprint)
- **$100-$500**: Biometric + PIN
- **>$500**: Ledger/DEGN required
- **>$5,000**: Ledger + 2FA (SMS/Email)

---

### 6. **Staking & Earning** 📈
**SDK Wrappers Used**: StakingWrapper, DistributionPayWrapper

**Features**:
- One-tap staking (auto-select best validator)
- Flexible/fixed staking terms
- Auto-compound rewards
- Unstaking countdown (28-day unbonding)
- Estimated APY: 10-15%
- Daily distribution rewards (27,397 ÉTR/day)

**UI Design**:
```
┌─────────────────────────────────┐
│ 📈 Staking Dashboard            │
│                                 │
│ Total Staked: 10,000 ÉTR        │
│ Current APY: 12.5%              │
│ Daily Rewards: 3.42 ÉTR         │
│ Total Earned: 1,250 ÉTR         │
│                                 │
│ [Stake More] [Unstake]          │
│                                 │
│ ┌─────────────────────────────┐ │
│ │ Validator: FlareNode-01     │ │
│ │ Your Stake: 10,000 ÉTR      │ │
│ │ Commission: 5%              │ │
│ │ Status: ✅ Active           │ │
│ │ [View Details]              │ │
│ └─────────────────────────────┘ │
└─────────────────────────────────┘

Staking Flow:
1. Amount: "How much to stake? [5,000 ÉTR]"
2. Duration: [Flexible] [1 month] [3 months] [1 year]
3. Validator: [Auto-select] [Choose manually]
4. Rewards: [Auto-compound] [Send to wallet]
5. Review: "Stake 5,000 ÉTR at 12% APY"
6. Confirm: [Face ID]
7. Done: "✅ Staking 5,000 ÉTR"
```

**SDK Integration**:
```typescript
import { StakingWrapper, DistributionPayWrapper } from '@etrid/sdk';

// Get staking info
const staking = new StakingWrapper(api);
const stakingInfo = await staking.getStakingInfo(address);

// Display dashboard
console.log(`Total Staked: ${stakingInfo.total / 1e18} ÉTR`);
console.log(`Current APY: ${stakingInfo.apy}%`);

// Stake (auto-select best validator)
const networkStats = await staking.getNetworkStats();
const bestValidator = networkStats.validators
  .sort((a, b) => b.apy - a.apy)[0]; // Highest APY

await staking.bond(keypair, stakeAmount);
await staking.nominate(keypair, [bestValidator.address]);

// Auto-compound (re-stake rewards daily)
setInterval(async () => {
  const rewards = await staking.getPendingRewards(address);
  if (rewards > 0) {
    await staking.bond(keypair, rewards); // Re-stake
  }
}, 86400000); // Every 24 hours

// Distribution Pay rewards (from 5 categories)
const distribution = new DistributionPayWrapper(api);
const categories = [
  'validators', 'developers', 'marketing',
  'governance', 'reserve'
];

for (const category of categories) {
  const reward = await distribution.calculateCategoryReward(category);
  console.log(`${category}: ${reward / 1e18} ÉTR/day`);
}
```

---

### 7. **Governance & Voting** 🗳️
**SDK Wrappers Used**: GovernanceWrapper

**Features**:
- View active proposals
- Vote with conviction (1x-6x multiplier)
- Delegate voting power
- Proposal history
- Push notifications for new proposals
- Estimated outcome predictions

**UI Design**:
```
┌─────────────────────────────────┐
│ 🗳️ Governance                   │
│                                 │
│ Active Proposals (3)            │
│                                 │
│ ┌─────────────────────────────┐ │
│ │ 🔥 HOT                      │ │
│ │ Proposal #127               │ │
│ │ "Increase validator rewards"│ │
│ │                             │ │
│ │ 👍 65% YES (12,500 votes)   │ │
│ │ 👎 35% NO (6,500 votes)     │ │
│ │                             │ │
│ │ Ends in: 2 days 14h         │ │
│ │ Your Vote: Not voted        │ │
│ │                             │ │
│ │ [Vote YES] [Vote NO]        │ │
│ │ [Read Details]              │ │
│ └─────────────────────────────┘ │
└─────────────────────────────────┘

Voting Flow:
1. Select proposal: "Proposal #127"
2. Read details: [Full text, discussion]
3. Choose vote: [YES] [NO] [ABSTAIN]
4. Conviction: [1x] [2x] [3x] [4x] [5x] [6x]
   "Lock tokens for longer = more voting power"
5. Review: "Voting YES with 3x conviction (90-day lock)"
6. Confirm: [Face ID]
7. Done: "✅ Vote recorded"
```

**SDK Integration**:
```typescript
import { GovernanceWrapper } from '@etrid/sdk';

// Get active proposals
const governance = new GovernanceWrapper(api);
const proposals = await governance.getActiveProposals();

// Display proposal
for (const proposal of proposals) {
  console.log(`Proposal #${proposal.id}: ${proposal.title}`);
  console.log(`YES: ${proposal.yesVotes} | NO: ${proposal.noVotes}`);
  console.log(`Ends: ${proposal.endDate}`);
}

// Vote on proposal (with 3x conviction = 90-day lock)
await governance.vote(
  keypair,
  proposalId,
  true, // support = YES
  3     // conviction = 3x (90-day lock)
);

// Delegate voting power (to trusted validator)
await governance.delegateVotes(
  keypair,
  validatorAddress,
  delegationAmount
);

// Push notification when new proposal
governance.subscribeToProposals((newProposal) => {
  sendPushNotification({
    title: "New Proposal",
    body: `Proposal #${newProposal.id}: ${newProposal.title}`,
    data: { proposalId: newProposal.id }
  });
});
```

---

### 8. **Cross-Chain Swaps** 🔄
**SDK Wrappers Used**: BridgeWrapper, OracleWrapper

**Features**:
- Swap between 13 chains (BTC, ETH, SOL, XRP, BNB, TRX, ADA, MATIC, DOGE, LTC, XLM, LINK, USDT)
- Real-time exchange rates
- Slippage protection
- Bridge fee transparency
- Transaction status tracking

**UI Design**:
```
┌─────────────────────────────────┐
│ 🔄 Swap                         │
│                                 │
│ From:                           │
│ [🟠 BTC] [0.1]                  │
│ ≈ $6,500                        │
│                                 │
│ ⬇️                               │
│                                 │
│ To:                             │
│ [🟣 ETH] [2.85]                 │
│ ≈ $6,435                        │
│                                 │
│ Rate: 1 BTC = 28.5 ETH          │
│ Fee: $10 (0.15%)                │
│ Time: ~15 minutes               │
│                                 │
│ [Swap Now]                      │
└─────────────────────────────────┘

Swap Flow:
1. Select source: [BTC] Amount: [0.1]
2. Select destination: [ETH]
3. Review rate: "1 BTC = 28.5 ETH"
4. Confirm: "Swap 0.1 BTC for 2.85 ETH?"
5. Slippage: [0.5%] [1%] [2%] [Custom]
6. Execute: [Face ID]
7. Track: "⏳ Bridging... (Step 2/3)"
8. Done: "✅ Received 2.85 ETH"
```

**SDK Integration**:
```typescript
import { BridgeWrapper, OracleWrapper } from '@etrid/sdk';

// Get supported chains
const bridge = new BridgeWrapper(api);
const chains = await bridge.getSupportedChains();
// ['BTC', 'ETH', 'SOL', 'XRP', 'BNB', 'TRX', 'ADA', 'MATIC', 'DOGE', 'LTC', 'XLM', 'LINK', 'USDT']

// Get exchange rate
const oracle = new OracleWrapper(api);
const btcEth = await oracle.getPrice('BTC/ETH');

// Execute bridge
const bridgeTx = await bridge.bridge(
  keypair,
  'BTC',        // source chain
  'ETH',        // destination chain
  0.1 * 1e8,    // amount (BTC has 8 decimals)
  ethAddress,   // destination address
  { slippage: 0.5 } // 0.5% slippage tolerance
);

// Track status
const status = await bridge.getTransferStatus(bridgeTx.transferId);
console.log(`Status: ${status.stage}/3 - ${status.message}`);
// "Status: 2/3 - Confirming on destination chain"
```

---

### 9. **Portfolio Tracker** 📊
**SDK Wrappers Used**: AccountsWrapper, OracleWrapper, BridgeWrapper

**Features**:
- Total net worth (all assets)
- Asset allocation chart
- 24h/7d/30d/1y performance
- Profit/loss calculations
- Transaction history with filters
- CSV export for taxes

**UI Design**:
```
┌─────────────────────────────────┐
│ 📊 Portfolio                    │
│                                 │
│ Total Value: $125,450.32        │
│ 24h Change: +$1,250.15 (+1.0%) │
│                                 │
│ [Chart: 7 days]                 │
│  📈 ──────/───────               │
│                                 │
│ Assets (8):                     │
│ ┌─────────────────────────────┐ │
│ │ ÉTR  50,000  $65,000  52%   │ │
│ │ BTC  0.5     $32,500  26%   │ │
│ │ ETH  10      $22,750  18%   │ │
│ │ SOL  100     $4,500   4%    │ │
│ │ USDT 700     $700     <1%   │ │
│ └─────────────────────────────┘ │
│                                 │
│ [Add Asset] [Export]            │
└─────────────────────────────────┘
```

**SDK Integration**:
```typescript
import { AccountsWrapper, OracleWrapper } from '@etrid/sdk';

// Get all balances
const accounts = new AccountsWrapper(api);
const balances = await accounts.getAllBalances(address);

// Get prices for all assets
const oracle = new OracleWrapper(api);
const portfolio = await Promise.all(
  balances.map(async (asset) => {
    const price = await oracle.getPrice(`${asset.symbol}/USD`);
    return {
      symbol: asset.symbol,
      amount: asset.balance / 1e18,
      valueUSD: (asset.balance / 1e18) * price,
      price: price
    };
  })
);

// Calculate totals
const totalValue = portfolio.reduce((sum, asset) => sum + asset.valueUSD, 0);

// Get 24h change
const yesterday = Date.now() - 86400000;
const portfolioYesterday = await getPortfolioValue(yesterday);
const change24h = totalValue - portfolioYesterday;
const changePercent = (change24h / portfolioYesterday) * 100;

// Display
console.log(`Total Value: $${totalValue.toFixed(2)}`);
console.log(`24h Change: $${change24h.toFixed(2)} (${changePercent.toFixed(2)}%)`);
```

---

### 10. **Security & Settings** 🔐
**SDK Wrappers Used**: LedgerHardwareWrapper, AccountsWrapper

**Features**:
- Biometric authentication (Face ID/Fingerprint)
- PIN backup
- Transaction limits
- Address whitelist
- 2FA (SMS/Email/TOTP)
- Recovery phrase backup (12/24 words)
- Auto-lock timer
- Privacy mode (hide balances)

**UI Design**:
```
┌─────────────────────────────────┐
│ ⚙️ Settings                      │
│                                 │
│ Security                        │
│ • Face ID: ✅ Enabled           │
│ • PIN: ✅ Set                   │
│ • 2FA: ⏹ Disabled               │
│ • Auto-lock: 5 minutes          │
│                                 │
│ Transaction Limits              │
│ • Daily: $5,000                 │
│ • Per Transaction: $1,000       │
│ • Require Ledger: >$500         │
│                                 │
│ Privacy                         │
│ • Hide Balances: ⏹ OFF          │
│ • Screenshot Protection: ✅ ON  │
│                                 │
│ Backup                          │
│ • Recovery Phrase: [View]       │
│ • Last Backup: 2 days ago       │
│                                 │
│ [Manage] [Advanced]             │
└─────────────────────────────────┘
```

---

## 🏛️ Backend Architecture

### System Diagram
```
┌─────────────────────────────────────────────────────────────┐
│                     Mobile App (React Native)                │
│  [Home] [Accounts] [Send] [ATM] [Stake] [Governance]        │
└─────────────────┬───────────────────────────────────────────┘
                  │ HTTPS + WebSocket
                  ▼
┌─────────────────────────────────────────────────────────────┐
│                    API Gateway (Express.js)                  │
│  • Authentication (JWT)                                      │
│  • Rate Limiting (Redis)                                     │
│  • GraphQL API                                               │
└─────┬───────────────────────────────────────────────────────┘
      │
      ├─────────────────┬─────────────────┬─────────────────┐
      ▼                 ▼                 ▼                 ▼
┌───────────┐   ┌───────────────┐   ┌───────────┐   ┌──────────────┐
│ Blockchain│   │ ATM Service   │   │ Notification│   │ Analytics    │
│ Indexer   │   │               │   │ Service     │   │ Service      │
│           │   │ • Coinme      │   │             │   │              │
│ PostgreSQL│   │ • Bitcoin     │   │ • Expo Push │   │ • Portfolio  │
│ + Redis   │   │   Depot       │   │ • Email     │   │   Tracking   │
│           │   │ • CoinFlip    │   │ • SMS       │   │ • Metrics    │
└─────┬─────┘   └───────┬───────┘   └─────┬───────┘   └──────┬───────┘
      │                 │                   │                  │
      └─────────────────┴───────────────────┴──────────────────┘
                                │
                                ▼
                    ┌───────────────────────┐
                    │  FlareChain RPC Node  │
                    │  (Substrate)          │
                    └───────────────────────┘
```

### Database Schema (PostgreSQL)

**Users Table**:
```sql
CREATE TABLE users (
  id UUID PRIMARY KEY,
  address VARCHAR(48) UNIQUE NOT NULL, -- SS58 address
  email VARCHAR(255) UNIQUE,
  phone VARCHAR(20),
  kyc_status VARCHAR(20) DEFAULT 'pending', -- pending, verified, rejected
  kyc_level INT DEFAULT 0, -- 0=none, 1=basic, 2=full
  created_at TIMESTAMP DEFAULT NOW(),
  last_login TIMESTAMP
);

CREATE INDEX idx_users_address ON users(address);
```

**Transactions Table**:
```sql
CREATE TABLE transactions (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  tx_hash VARCHAR(66) NOT NULL,
  block_number BIGINT,
  from_address VARCHAR(48),
  to_address VARCHAR(48),
  amount NUMERIC(36, 18), -- 18 decimals
  asset VARCHAR(10), -- ETR, BTC, ETH, etc.
  fee NUMERIC(36, 18),
  status VARCHAR(20), -- pending, confirmed, failed
  tx_type VARCHAR(30), -- transfer, stake, vote, bridge, etc.
  metadata JSONB, -- Extra data
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_tx_user ON transactions(user_id);
CREATE INDEX idx_tx_hash ON transactions(tx_hash);
CREATE INDEX idx_tx_status ON transactions(status);
```

**Staking Table**:
```sql
CREATE TABLE staking_positions (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  validator_address VARCHAR(48),
  amount NUMERIC(36, 18),
  rewards_earned NUMERIC(36, 18) DEFAULT 0,
  apy DECIMAL(5, 2),
  start_date TIMESTAMP DEFAULT NOW(),
  end_date TIMESTAMP, -- NULL = flexible
  status VARCHAR(20), -- active, unbonding, withdrawn
  auto_compound BOOLEAN DEFAULT true
);

CREATE INDEX idx_staking_user ON staking_positions(user_id);
```

**ATM Withdrawals Table**:
```sql
CREATE TABLE atm_withdrawals (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  withdrawal_code VARCHAR(20) UNIQUE NOT NULL,
  amount_usd DECIMAL(10, 2),
  amount_crypto NUMERIC(36, 18),
  asset VARCHAR(10),
  fee DECIMAL(10, 2),
  atm_partner VARCHAR(50), -- Coinme, Bitcoin Depot, etc.
  atm_location_id VARCHAR(100),
  tx_hash VARCHAR(66),
  status VARCHAR(20), -- pending, completed, expired, failed
  expires_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW(),
  completed_at TIMESTAMP
);

CREATE INDEX idx_atm_code ON atm_withdrawals(withdrawal_code);
CREATE INDEX idx_atm_status ON atm_withdrawals(status);
```

**Governance Votes Table**:
```sql
CREATE TABLE governance_votes (
  id UUID PRIMARY KEY,
  user_id UUID REFERENCES users(id),
  proposal_id INT NOT NULL,
  support BOOLEAN, -- true=YES, false=NO
  conviction INT DEFAULT 0, -- 0-6
  voting_power NUMERIC(36, 18),
  tx_hash VARCHAR(66),
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_votes_proposal ON governance_votes(proposal_id);
CREATE INDEX idx_votes_user ON governance_votes(user_id);
```

### API Endpoints

**Authentication**:
```typescript
POST /api/v1/auth/login
POST /api/v1/auth/refresh
POST /api/v1/auth/logout
POST /api/v1/auth/verify-2fa
```

**Accounts**:
```typescript
GET /api/v1/accounts/:address/balance
GET /api/v1/accounts/:address/portfolio
GET /api/v1/accounts/:address/transactions?page=1&limit=50
POST /api/v1/accounts/:address/transfer
```

**Staking**:
```typescript
GET /api/v1/staking/:address/positions
GET /api/v1/staking/:address/rewards
POST /api/v1/staking/:address/stake
POST /api/v1/staking/:address/unstake
GET /api/v1/staking/validators?sort=apy
```

**Governance**:
```typescript
GET /api/v1/governance/proposals?status=active
GET /api/v1/governance/proposals/:id
POST /api/v1/governance/proposals/:id/vote
GET /api/v1/governance/:address/votes
```

**ATM**:
```typescript
GET /api/v1/atm/locations?lat=37.7749&lng=-122.4194&radius=10
POST /api/v1/atm/withdraw
GET /api/v1/atm/withdrawals/:code/status
POST /api/v1/atm/withdrawals/:code/complete
```

**Bridge**:
```typescript
GET /api/v1/bridge/chains
GET /api/v1/bridge/rate?from=BTC&to=ETH
POST /api/v1/bridge/transfer
GET /api/v1/bridge/transfers/:id/status
```

---

## 🎨 UI/UX Design Principles

### Design System
**Brand Colors**:
- Primary: `#6C5CE7` (Purple - Ëtrid brand)
- Secondary: `#00B894` (Green - success, positive)
- Accent: `#FD79A8` (Pink - highlights)
- Background: `#FFFFFF` (Light), `#1E1E1E` (Dark)
- Text: `#2D3436` (Dark), `#DFE6E9` (Light)

**Typography**:
- Headings: Inter Bold (24-32px)
- Body: Inter Regular (14-16px)
- Monospace: JetBrains Mono (for addresses, amounts)

**Components**:
- React Native Paper (Material Design)
- Custom components for crypto-specific UI
- Lottie animations for loading states
- Haptic feedback for actions

### UX Principles

1. **Progressive Disclosure**: Show simple view by default, advanced options behind "Advanced" button
2. **Familiar Metaphors**: Use banking terms (checking/savings) instead of blockchain jargon
3. **Zero Blockchain Knowledge Required**: Hide addresses, hashes, gas fees from average users
4. **Instant Feedback**: Show optimistic UI updates, confirm in background
5. **Error Prevention**: Validate inputs, show warnings before irreversible actions
6. **Accessibility**: Support screen readers, high contrast mode, large text

---

## 🔐 Security Architecture

### Authentication Flow
```
1. App Launch
   ↓
2. Check Biometric: [Face ID] [Fingerprint]
   ↓
3. If failed → PIN Entry (6 digits)
   ↓
4. Decrypt Keychain (stores keypair)
   ↓
5. Load User Session
```

### Key Storage
- **iOS**: Keychain with Secure Enclave
- **Android**: Keystore with TEE (Trusted Execution Environment)
- **Encryption**: AES-256-GCM
- **Biometric**: Face ID (iOS), Fingerprint (Android)

### Transaction Security Levels

| Amount | Security Required | Timeout |
|--------|------------------|---------|
| <$100 | Biometric | 15 min |
| $100-$500 | Biometric + PIN | 5 min |
| $500-$5,000 | Ledger/DEGN | Immediate |
| >$5,000 | Ledger + 2FA | Immediate |

### Attack Mitigation

**Phishing Protection**:
- Address book verification (show contact name, not address)
- Domain verification for dApps
- Warning for new/unknown addresses

**Device Compromise**:
- Remote wipe capability
- Session invalidation
- Geofencing alerts

**Network Attacks**:
- Certificate pinning
- TLS 1.3+
- WebSocket encryption

---

## 📱 Screen Flows

### 1. Onboarding Flow
```
┌──────────────┐
│ 1. Welcome   │
│ "Your crypto │
│  bank"       │
│ [Get Started]│
└──────┬───────┘
       ▼
┌──────────────┐
│ 2. Create    │
│    Account   │
│ • New wallet │
│ • Import     │
└──────┬───────┘
       ▼
┌──────────────┐
│ 3. Backup    │
│    Phrase    │
│ "Write down  │
│  12 words"   │
└──────┬───────┘
       ▼
┌──────────────┐
│ 4. Verify    │
│    Phrase    │
│ "Select word │
│  #3, #7, #11"│
└──────┬───────┘
       ▼
┌──────────────┐
│ 5. Biometric │
│ "Enable      │
│  Face ID?"   │
│ [Yes] [Skip] │
└──────┬───────┘
       ▼
┌──────────────┐
│ 6. Dashboard │
│ (Main app)   │
└──────────────┘
```

### 2. Send Money Flow
```
Home → Send → Enter Amount → Select Recipient → Choose Speed → Review → Authenticate → Confirm
```

### 3. ATM Withdrawal Flow
```
Home → ATM → Find Location → Select Amount → Select Asset → Generate Code → Show QR → Track Status → Complete
```

### 4. Staking Flow
```
Home → Staking → View Validators → Enter Amount → Select Duration → Review APY → Confirm → Track Rewards
```

### 5. Governance Flow
```
Home → Governance → View Proposals → Read Details → Choose Vote → Set Conviction → Confirm → Track Vote
```

---

## 🚀 Implementation Roadmap

### Phase 1: MVP (8 weeks)
**Goal**: Core wallet functionality

**Week 1-2: Foundation**
- [ ] Initialize React Native + Expo project
- [ ] Set up folder structure
- [ ] Integrate @etrid/sdk
- [ ] Create design system (colors, typography, components)
- [ ] Implement authentication (biometric + PIN)
- [ ] Set up secure key storage (Keychain/Keystore)

**Week 3-4: Core Features**
- [ ] Home dashboard with balance
- [ ] Send/receive with QR codes
- [ ] Transaction history
- [ ] Contact management
- [ ] Portfolio tracker
- [ ] Price feeds (OracleWrapper)

**Week 5-6: Banking Features**
- [ ] Checking account UI
- [ ] Savings account (ReserveVaultWrapper)
- [ ] Cross-chain swaps (BridgeWrapper)
- [ ] Asset breakdown charts

**Week 7-8: Testing & Polish**
- [ ] Unit tests (Jest)
- [ ] E2E tests (Detox)
- [ ] UI polish
- [ ] Performance optimization
- [ ] Beta testing (TestFlight/Play Console)

**Deliverable**: Functional wallet app with send/receive, swaps, basic DeFi

---

### Phase 2: DeFi Features (6 weeks)
**Goal**: Staking, governance, advanced features

**Week 9-10: Staking**
- [ ] Staking dashboard (StakingWrapper)
- [ ] Validator selection
- [ ] Flexible/fixed staking
- [ ] Auto-compound rewards
- [ ] Unstaking flow

**Week 11-12: Governance**
- [ ] Proposal list (GovernanceWrapper)
- [ ] Proposal details with discussion
- [ ] Voting with conviction
- [ ] Vote delegation
- [ ] Push notifications for new proposals

**Week 13-14: Advanced Features**
- [ ] Lightning-Bloc channels (LightningBlocWrapper)
- [ ] Distribution Pay rewards (DistributionPayWrapper)
- [ ] AI DID integration (AIDidWrapper)
- [ ] ETWASM contract interaction (EtwasmVMWrapper)

**Deliverable**: Full DeFi features, governance, instant payments

---

### Phase 3: ATM & Cold Storage (4 weeks)
**Goal**: Cash access and hardware wallet integration

**Week 15-16: ATM Integration**
- [ ] ATM location map (Google Maps API)
- [ ] Partner API integration (Coinme)
- [ ] Withdrawal code generation
- [ ] QR code display
- [ ] Status tracking
- [ ] Fee calculations

**Week 17-18: Cold Storage**
- [ ] Ledger Nano X Bluetooth (LedgerHardwareWrapper)
- [ ] Transaction signing flow
- [ ] Multi-account support
- [ ] Security level enforcement
- [ ] DEGN wallet integration (if available)

**Deliverable**: ATM cash access, hardware wallet security

---

### Phase 4: Enterprise & Polish (4 weeks)
**Goal**: Enterprise features, final polish, launch

**Week 19-20: Enterprise**
- [ ] Hyperledger bridge (HyperledgerBridgeWrapper)
- [ ] GPU marketplace (GPURegistryWrapper, GPUNFTWrapper)
- [ ] ETH PBC integration (ETHPBCPrecompileWrapper)
- [ ] KYC/AML compliance
- [ ] Multi-language support

**Week 21-22: Launch Prep**
- [ ] Security audit (third-party)
- [ ] Performance optimization
- [ ] Analytics integration (Mixpanel)
- [ ] Customer support (Intercom)
- [ ] Marketing materials
- [ ] App Store submission (iOS)
- [ ] Play Store submission (Android)

**Deliverable**: Production-ready app, approved for app stores

---

### Total Timeline: **22 weeks (5.5 months)**

---

## 💰 Business Model

### Revenue Streams

1. **Transaction Fees** (0.1-0.5%)
   - Send/receive: 0.1%
   - Cross-chain swaps: 0.3%
   - Bridge transfers: 0.5%

2. **ATM Fees** (7-12%)
   - Partner revenue share: 30-50%
   - Estimated: $3-$6 per $100 withdrawal

3. **Staking Commission** (5-10%)
   - Take 5% of staking rewards as platform fee
   - Validator partnership: 50/50 split

4. **Premium Features** ($9.99/month)
   - Higher transaction limits
   - Priority customer support
   - Advanced analytics
   - Tax reporting tools

5. **Institutional Services** (Custom pricing)
   - Multi-signature wallets
   - Team accounts
   - API access
   - White-label solution

### Competitive Analysis

| Feature | Ëtrid Wallet | MetaMask | Trust Wallet | Coinbase Wallet |
|---------|--------------|----------|--------------|-----------------|
| Mobile Native | ✅ | ✅ | ✅ | ✅ |
| Bank-like UI | ✅ | ❌ | ⏹ | ⏹ |
| ATM Access | ✅ | ❌ | ❌ | ⏹ (Coinbase ATM) |
| Staking | ✅ | ⏹ | ✅ | ✅ |
| Governance | ✅ | ❌ | ❌ | ❌ |
| Hardware Wallet | ✅ (Ledger) | ✅ (Ledger) | ⏹ | ❌ |
| Cross-Chain | ✅ (13 chains) | ⏹ | ✅ | ⏹ |
| DeFi Vaults | ✅ | ❌ | ⏹ | ⏹ |
| GPU Marketplace | ✅ | ❌ | ❌ | ❌ |
| Lightning Payments | ✅ (500K TPS) | ❌ | ❌ | ❌ |

**Unique Value Proposition**: Only wallet that combines banking UX + DeFi features + ATM access + hardware wallet security + multi-chain support.

---

## 📊 Success Metrics

### Key Performance Indicators (KPIs)

**User Growth**:
- MAU (Monthly Active Users): Target 100K in Year 1
- DAU/MAU Ratio: Target 30%
- User Retention: Target 60% (30-day)

**Engagement**:
- Transactions per user per month: Target 10
- Staking participation: Target 40% of users
- Governance participation: Target 15% of users

**Revenue**:
- ARPU (Average Revenue Per User): Target $5/month
- Transaction volume: Target $50M/month
- ATM withdrawals: Target $1M/month

**Technical**:
- App crash rate: <1%
- API response time: <200ms (p95)
- Transaction success rate: >99%

---

## 🔮 Future Features (Post-Launch)

### Year 1 Enhancements
1. **Fiat On/Ramp**: Buy crypto with credit card (Stripe, Ramp Network)
2. **Recurring Payments**: Auto-pay bills, subscriptions
3. **Social Features**: Send money via username, social recovery
4. **Tax Reporting**: Auto-generate tax forms (1099, 8949)
5. **Savings Goals**: Set targets, auto-save towards goals
6. **Loan Marketplace**: P2P lending, collateralized loans

### Year 2 Innovations
1. **Crypto Credit Card**: Visa/Mastercard backed by crypto balance
2. **Investment Products**: Index funds, yield farming strategies
3. **Insurance**: Wallet insurance, smart contract coverage
4. **Business Accounts**: Invoicing, payroll, accounting integrations
5. **Web3 Integration**: dApp browser, NFT gallery, metaverse wallets
6. **AI Assistant**: Natural language transactions ("Send $100 to Alice")

---

## 🎯 Conclusion

This architecture provides a **comprehensive roadmap** for building the Ëtrid Mobile DeFi Wallet - a revolutionary app that combines:

✅ **Bank-like UX** (checking/savings accounts)
✅ **Crypto ATM access** (cash withdrawals at 50K+ locations)
✅ **Cold storage integration** (Ledger/DEGN as "debit cards")
✅ **Full DeFi features** (staking, governance, swaps)
✅ **Cross-chain support** (13 blockchains)
✅ **Lightning-fast payments** (500K TPS)
✅ **Enterprise features** (Hyperledger, GPU marketplace)

**Total Development Time**: 22 weeks (5.5 months)
**Tech Stack**: React Native + Expo + @etrid/sdk
**Target Launch**: Q2 2026

**Next Steps**:
1. Initialize React Native project
2. Integrate Ëtrid SDK
3. Build MVP (Home, Send, Receive, Portfolio)
4. Beta testing with early users
5. Iterate based on feedback

**This wallet will redefine what a crypto wallet can be - not just a tool for enthusiasts, but a complete financial solution for everyone.** 🚀

---

**Document Version**: 1.0
**Last Updated**: November 18, 2025
**Author**: Ëtrid Development Team
**Status**: Architecture Complete, Ready for Implementation
