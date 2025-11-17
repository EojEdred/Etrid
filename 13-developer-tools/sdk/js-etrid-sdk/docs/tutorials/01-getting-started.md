# Tutorial 1: Getting Started with Ëtrid SDK

## Introduction

This tutorial will guide you through installing and using the Ëtrid JavaScript/TypeScript SDK for the first time.

## Prerequisites

- Node.js 16+ and npm
- TypeScript knowledge (optional but recommended)
- Basic blockchain concepts

## Installation

### Option 1: NPM (Recommended)

```bash
npm install @etrid/sdk
```

### Option 2: From Source

```bash
git clone https://github.com/etrid/etrid-protocol
cd etrid-protocol/13-developer-tools/sdk/js-etrid-sdk
npm install
npm run build
```

## Your First Connection

Let's connect to an Ëtrid node and query basic information:

```typescript
import { EtridClient } from '@etrid/sdk';

// Connect to local node
const client = new EtridClient('ws://127.0.0.1:9944');

// Or connect to testnet
// const client = new EtridClient('wss://testnet.etrid.io');

// Check connection
console.log('Connected:', client.isConnected());
console.log('Chain:', client.getChain());
console.log('Block:', await client.getBlockNumber());
```

**Output:**
```
Connected: true
Chain: Etrid
Block: 12345
```

## Creating Accounts

### Generate New Account

```typescript
import { Keyring } from '@polkadot/keyring';

const keyring = new Keyring({ type: 'sr25519' });

// Generate new account
const alice = keyring.addFromUri('//Alice');
console.log('Alice address:', alice.address);

// Generate random account
const random = keyring.addFromMnemonic(Keyring.generateMnemonic());
console.log('Random address:', random.address);
```

### Restore from Mnemonic

```typescript
const mnemonic = 'word1 word2 word3 ... word12';
const restored = keyring.addFromMnemonic(mnemonic);
console.log('Restored:', restored.address);
```

## Your First Transaction

Let's check a balance and send a simple transfer:

```typescript
import { AccountsWrapper } from '@etrid/sdk';

const accounts = new AccountsWrapper(client.api);

// Check balance
const balance = await accounts.getBalance(alice.address);
console.log('Balance:', balance.free / 10n**18n, 'ÉTR');

// Send transfer
const txHash = await accounts.transfer(
  alice,
  bob.address,
  100n * 10n**18n  // 100 ÉTR
);

console.log('Transfer sent:', txHash);
```

## Using Wrappers

Each Ëtrid pallet has a dedicated wrapper class:

```typescript
import { 
  LightningBlocWrapper,
  DistributionPayWrapper,
  StakingWrapper 
} from '@etrid/sdk';

// Initialize wrappers
const lightning = new LightningBlocWrapper(client.api);
const distribution = new DistributionPayWrapper(client.api);
const staking = new StakingWrapper(client.api);

// Use wrapper methods
const channel = await lightning.openChannel(
  alice,
  bob.address,
  1000n * 10n**18n
);

const pending = await distribution.getPendingRewards(alice.address);
const validators = await staking.getValidators();
```

## Error Handling

Always wrap SDK calls in try-catch blocks:

```typescript
import { NotConnectedError, TransactionError } from '@etrid/sdk';

try {
  const balance = await accounts.getBalance(address);
  console.log('Balance:', balance.free);
} catch (error) {
  if (error instanceof NotConnectedError) {
    console.error('Not connected to node');
  } else if (error instanceof TransactionError) {
    console.error('Transaction failed:', error.message);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

## Type Safety with TypeScript

The SDK provides full TypeScript support:

```typescript
import { 
  ValidatorStatus, 
  StakingInfo,
  TransactionResult 
} from '@etrid/sdk';

// TypeScript knows the exact return types
const status: ValidatorStatus | null = await staking.getValidatorStatus(address);
const info: StakingInfo = await staking.getStakingInfo(address);
const result: TransactionResult = await staking.bond(alice, validator, amount);

// IDE autocomplete works perfectly
status?.commission  // TypeScript knows this exists
info.staked         // And this
result.txHash       // And this
```

## Working with BigInts

Ëtrid uses 18 decimal places. Always use BigInt for amounts:

```typescript
// ✅ Correct - using BigInt
const amount = 1000n * 10n**18n;  // 1000 ÉTR

// ❌ Wrong - regular numbers lose precision
const wrong = 1000 * Math.pow(10, 18);  // Don't do this!

// Helper function
function toETR(planck: bigint): string {
  return (planck / 10n**18n).toString();
}

console.log('Amount:', toETR(amount), 'ÉTR');
```

## Next Steps

Now that you've mastered the basics, explore:

- **Tutorial 2**: Advanced Features (Lightning-Bloc, AI DID)
- **Tutorial 3**: Best Practices
- **Tutorial 4**: Common Patterns
- **Tutorial 5**: Migration from Polkadot.js

## Complete Example

```typescript
import { EtridClient, LightningBlocWrapper } from '@etrid/sdk';
import { Keyring } from '@polkadot/keyring';

async function main() {
  // 1. Connect
  const client = new EtridClient('ws://127.0.0.1:9944');
  console.log('Connected to', client.getChain());
  
  // 2. Create accounts
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');
  
  // 3. Use Lightning-Bloc
  const lightning = new LightningBlocWrapper(client.api);
  
  try {
    // Open channel
    const channel = await lightning.openChannel(
      alice,
      bob.address,
      1000n * 10n**18n
    );
    console.log('Channel opened:', channel.channelId);
    
    // Send payment
    const payment = await lightning.sendPayment(
      alice,
      channel.channelId,
      100n * 10n**18n
    );
    console.log('Payment sent:', payment.txHash);
    
    // Close channel
    const closeHash = await lightning.closeChannel(alice, channel.channelId);
    console.log('Channel closed:', closeHash);
    
  } catch (error) {
    console.error('Error:', error.message);
  }
  
  // 4. Cleanup
  client.close();
}

main().catch(console.error);
```

## Troubleshooting

### "Not connected to node"

Make sure your Ëtrid node is running:

```bash
# Start local node
./target/release/flarechain-node --dev
```

### "Insufficient balance"

Fund your account from the faucet:

```bash
# Testnet faucet
curl https://faucet.etrid.io/drip/YOUR_ADDRESS
```

### "Type errors with BigInt"

Make sure your tsconfig.json has:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020"]
  }
}
```

## Resources

- [API Documentation](https://docs.etrid.io/sdk)
- [Examples Repository](../examples)
- [Discord Community](https://discord.gg/etrid)
- [GitHub Issues](https://github.com/etrid/etrid-protocol/issues)

---

**Next**: [Tutorial 2: Advanced Features →](./02-advanced-features.md)
