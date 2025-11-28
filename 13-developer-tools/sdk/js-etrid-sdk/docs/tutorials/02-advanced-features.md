# Tutorial 2: Advanced Features

## Layer 3 Payment Channels (Lightning-Bloc)

### Opening and Managing Channels

```typescript
import { LightningBlocWrapper } from '@etrid/sdk';

const lightning = new LightningBlocWrapper(client.api);

// Open channel with 1000 ÉTR capacity
const channel = await lightning.openChannel(
  alice,
  bob.address,
  1000n * 10n**18n
);

console.log('Channel ID:', channel.channelId);
console.log('Capacity:', channel.amount / 10n**18n, 'ÉTR');

// Get channel info
const info = await lightning.getChannel(channel.channelId);
console.log('Balance:', info.balance / 10n**18n, 'ÉTR');
console.log('Nonce:', info.nonce);
console.log('Status:', info.status);
```

### Multi-hop Payments

```typescript
// Find route from Alice to Charlie through Bob
const route = await lightning.getRoute(
  alice.address,
  charlie.address,
  50n * 10n**18n
);

console.log('Route found:', route);
// ['channel-1', 'channel-2', 'channel-3']

// Send payment through route
for (const channelId of route) {
  await lightning.sendPayment(alice, channelId, amount);
}
```

### Channel Monitoring

```typescript
// Subscribe to channel events
const unsubscribe = await lightning.subscribeToChannelEvents(
  channel.channelId,
  (event) => {
    console.log('Event:', event.type);
    console.log('Data:', event.data);
  }
);

// Unsubscribe later
unsubscribe();
```

## AI Identity (AI DID)

### Registering AI Agents

```typescript
import { AIDidWrapper, AIType, ReputationTier } from '@etrid/sdk';

const aiDid = new AIDidWrapper(client.api);

// Register new AI
const registration = await aiDid.registerAI(
  alice,
  'GPT-Assistant',
  AIType.LLM,
  'https://api.myai.com',
  {
    version: '4.0',
    capabilities: ['text', 'code', 'analysis'],
    model: 'gpt-4',
    parameters: 175000000000
  }
);

console.log('AI DID:', registration.did);
console.log('Reputation:', registration.reputation);
console.log('Tier:', registration.tier);
```

### Managing AI Reputation

```typescript
// Update reputation (only owner can do this)
await aiDid.updateReputation(alice, aiDid.did, 50);

// Check current reputation
const reputation = await aiDid.getReputation(aiDid.did);
console.log('Reputation:', reputation);

// Get tier
const tier = await aiDid.getReputationTier(reputation);
console.log('Tier:', tier);
// ReputationTier.Silver (500-999)
// ReputationTier.Gold (1000-1999)
// ReputationTier.Platinum (2000+)
```

### Permission Management

```typescript
// Grant read permission to operator
await aiDid.grantPermission(
  alice,
  registration.did,
  operator.address,
  'read'
);

// Grant execute permission
await aiDid.grantPermission(
  alice,
  registration.did,
  executor.address,
  'execute'
);

// Check permission
const hasPermission = await aiDid.hasPermission(
  registration.did,
  operator.address,
  'read'
);

// Get all permissions
const permissions = await aiDid.getPermissions(registration.did);
```

## Cross-Chain Bridge

### Bridging Assets

```typescript
import { BridgeWrapper, SupportedChain } from '@etrid/sdk';

const bridge = new BridgeWrapper(client.api);

// Bridge from Ethereum to BNB
const transfer = await bridge.bridge(
  alice,
  SupportedChain.Ethereum,
  SupportedChain.BNB,
  100n * 10n**18n,
  '0x742d35Cc...'  // BNB recipient address
);

console.log('Transfer ID:', transfer.transferId);
console.log('Estimated time:', transfer.estimatedTime, 'seconds');
```

### Monitoring Transfers

```typescript
// Check transfer status
const status = await bridge.getTransferStatus(transfer.transferId);
console.log('Status:', status.status);
// 'pending' | 'confirmed' | 'completed' | 'failed'

// Subscribe to updates
const unsub = await bridge.subscribeToBridgeEvents((event) => {
  if (event.transferId === transfer.transferId) {
    console.log('Status changed:', event.status);
  }
});
```

### Bridge Analytics

```typescript
// Get supported chains
const chains = await bridge.getSupportedChains();
console.log('Supported:', chains);
// ['BTC', 'ETH', 'BNB', 'SOL', 'XRP', ...]

// Calculate fees
const fee = await bridge.getBridgeFee(
  SupportedChain.Ethereum,
  SupportedChain.BNB,
  100n * 10n**18n
);

// Get limits
const min = await bridge.getMinimumBridgeAmount(
  SupportedChain.Ethereum,
  SupportedChain.BNB
);
const max = await bridge.getMaximumBridgeAmount(
  SupportedChain.Ethereum,
  SupportedChain.BNB
);

// Bridge statistics
const stats = await bridge.getBridgeStatistics();
console.log('Total transfers:', stats.totalTransfers);
console.log('Total volume:', stats.totalVolume / 10n**18n, 'ÉTR');
console.log('Success rate:', stats.successRate, '%');
```

## ETWASM Smart Contracts

### Deploying Contracts

```typescript
import { EtwasmVMWrapper, GAS_CONSTANTS } from '@etrid/sdk';
import fs from 'fs';

const etwasm = new EtwasmVMWrapper(client.api);

// Load WASM bytecode
const wasmCode = fs.readFileSync('token.wasm');

// Deploy contract
const deployment = await etwasm.deployContract(
  alice,
  wasmCode,
  ['MyToken', 'MTK', 18],  // Constructor args
  0n,  // No value sent
  GAS_CONSTANTS.DEFAULT_GAS
);

console.log('Contract:', deployment.address);
console.log('Code hash:', deployment.codeHash);
console.log('Gas used:', deployment.gasUsed);
```

### Calling Contracts

```typescript
// Call contract method (state-changing)
const result = await etwasm.callContract(
  alice,
  deployment.address,
  'transfer',
  [bob.address, 100n * 10n**18n],
  0n,
  500_000n
);

console.log('TX hash:', result.txHash);
console.log('Gas used:', result.gasUsed);

// Query contract (read-only, no gas)
const balance = await etwasm.queryContract(
  deployment.address,
  'balanceOf',
  [alice.address]
);

console.log('Balance:', balance);
```

### Gas Estimation

```typescript
// Estimate gas before calling
const estimate = await etwasm.estimateGas(
  deployment.address,
  'transfer',
  [bob.address, 100n * 10n**18n]
);

console.log('Estimated:', estimate.estimated);
console.log('With buffer:', estimate.withBuffer);
console.log('Max possible:', estimate.maxPossible);

// Use estimated gas
const tx = await etwasm.callContract(
  alice,
  deployment.address,
  'transfer',
  [bob.address, 100n * 10n**18n],
  0n,
  estimate.withBuffer
);
```

## Price Oracles

### Querying Prices

```typescript
import { OracleWrapper } from '@etrid/sdk';

const oracle = new OracleWrapper(client.api);

// Get current price
const price = await oracle.getPrice('BTC/USD');
console.log('BTC price:', price / 10n**18n, 'USD');

// Get with metadata
const priceData = await oracle.getPriceWithMetadata('BTC/USD');
console.log('Price:', priceData.price / 10n**18n);
console.log('Confidence:', priceData.confidence, '%');
console.log('Sources:', priceData.sources);
console.log('Timestamp:', new Date(priceData.timestamp));
```

### Time-Weighted Average Price (TWAP)

```typescript
// Get 1-hour TWAP
const twap1h = await oracle.getTWAP('BTC/USD', 3600);

// Get 24-hour TWAP
const twap24h = await oracle.getTWAP('BTC/USD', 86400);

// Get 7-day TWAP
const twap7d = await oracle.getTWAP('BTC/USD', 604800);

console.log('1h TWAP:', twap1h / 10n**18n);
console.log('24h TWAP:', twap24h / 10n**18n);
console.log('7d TWAP:', twap7d / 10n**18n);
```

### Price Subscriptions

```typescript
// Subscribe to price updates
const unsubscribe = await oracle.subscribeToPriceUpdates(
  'BTC/USD',
  (price) => {
    console.log('New price:', price / 10n**18n, 'USD');
  }
);

// Subscribe to multiple pairs
const pairs = ['BTC/USD', 'ETH/USD', 'BNB/USD'];
const unsubs = await Promise.all(
  pairs.map(pair => 
    oracle.subscribeToPriceUpdates(pair, (price) => {
      console.log(`${pair}:`, price / 10n**18n);
    })
  )
);
```

## Reserve Vaults (DeFi)

### Creating Vaults

```typescript
import { ReserveVaultWrapper } from '@etrid/sdk';

const vault = new ReserveVaultWrapper(client.api);

// Create vault with collateral
const vaultData = await vault.createVault(
  alice,
  1000n * 10n**18n  // 1000 ÉTR collateral
);

console.log('Vault ID:', vaultData.vaultId);
console.log('Collateral:', vaultData.collateral / 10n**18n, 'ÉTR');
```

### Borrowing

```typescript
// Calculate max borrow
const maxBorrow = await vault.calculateBorrowLimit(1000n * 10n**18n);
console.log('Max borrow:', maxBorrow / 10n**18n, 'ÉTR');

// Borrow against collateral (up to 66% with 150% ratio)
await vault.borrow(
  alice,
  vaultData.vaultId,
  600n * 10n**18n  // Borrow 600 ÉTR
);

// Check vault health
const healthFactor = await vault.getHealthFactor(vaultData.vaultId);
console.log('Health factor:', healthFactor);
// > 1.0 = healthy
// < 1.0 = liquidatable
```

### Managing Vaults

```typescript
// Deposit more collateral
await vault.depositCollateral(
  alice,
  vaultData.vaultId,
  500n * 10n**18n
);

// Repay debt
await vault.repay(
  alice,
  vaultData.vaultId,
  200n * 10n**18n
);

// Check collateral ratio
const ratio = await vault.getCollateralRatio(vaultData.vaultId);
console.log('Collateral ratio:', ratio, '%');

// Withdraw collateral (if safe)
await vault.withdrawCollateral(
  alice,
  vaultData.vaultId,
  100n * 10n**18n
);
```

## Daily Rewards Distribution

### Claiming Rewards

```typescript
import { DistributionPayWrapper, DistributionCategory } from '@etrid/sdk';

const distribution = new DistributionPayWrapper(client.api);

// Check what you can claim
const pending = await distribution.getPendingRewards(alice.address);
console.log('Total pending:', pending.total / 10n**18n, 'ÉTR');
console.log('Stakers:', pending.byCategory.stakers / 10n**18n, 'ÉTR');
console.log('Voters:', pending.byCategory.voters / 10n**18n, 'ÉTR');

// Claim staker rewards
await distribution.claimReward(alice, DistributionCategory.Stakers);

// Claim voter rewards
await distribution.claimReward(alice, DistributionCategory.Voters);
```

### Distribution Schedule

```typescript
// Get daily distribution info
const schedule = await distribution.getDistributionSchedule();
console.log('Daily total:', schedule.totalDailyDistribution / 10n**18n, 'ÉTR');
console.log('Period:', schedule.distributionPeriod, 'seconds');

// Category allocations
console.log('Stakers:', schedule.categoryAllocations.stakers / 10n**18n, 'ÉTR');
console.log('Voters:', schedule.categoryAllocations.voters / 10n**18n, 'ÉTR');
console.log('FlareNodes:', schedule.categoryAllocations.flareNodes / 10n**18n, 'ÉTR');
```

---

**Next**: [Tutorial 3: Best Practices →](./03-best-practices.md)
