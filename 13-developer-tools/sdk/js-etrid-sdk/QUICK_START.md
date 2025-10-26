# Etrid SDK Quick Start Guide

Get started with the Etrid TypeScript SDK in minutes!

## Installation

```bash
npm install @etrid/sdk @polkadot/api
```

## Basic Setup

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import {
  TransactionBuilder,
  AccountsWrapper,
  StakingWrapper,
  GovernanceWrapper,
  formatETR,
  shortenAddress,
} from '@etrid/sdk';

// Connect to blockchain
const provider = new WsProvider('wss://rpc.etrid.io');
const api = await ApiPromise.create({ provider });

// Setup account
const keyring = new Keyring({ type: 'sr25519' });
const alice = keyring.addFromMnemonic('your mnemonic here');
```

## Common Operations

### 1. Check Balance

```typescript
const accounts = new AccountsWrapper(api);
const balance = await accounts.getBalance(alice.address);

console.log(`Total: ${formatETR(balance.total)}`);
console.log(`Available: ${formatETR(balance.available)}`);
console.log(`Staked: ${formatETR(balance.reserved)}`);
```

### 2. Send Transfer

```typescript
const result = await new TransactionBuilder(api)
  .transferKeepAlive(recipientAddress, 1000000000000000000n) // 1 ETR
  .submit(alice);

console.log(`Success: ${result.success}`);
console.log(`Hash: ${result.hash}`);
```

### 3. Stake Tokens

```typescript
const staking = new StakingWrapper(api);

// Get validators
const validators = await staking.getValidators();

// Stake to validator
await staking.bond(alice, validators[0], 10000000000000000000n); // 10 ETR
```

### 4. Vote on Proposal

```typescript
const governance = new GovernanceWrapper(api);

// Get proposals
const proposals = await governance.getActiveProposals();

// Vote on first proposal
await governance.vote(
  alice,
  proposals[0].id,
  true,                      // approve
  5000000000000000000n       // 5 ETR stake
);
```

## Advanced Features

### Fee Estimation

```typescript
const builder = new TransactionBuilder(api)
  .transfer(recipientAddress, amount);

const fees = await builder.estimateFees(alice.address);
console.log(`Estimated fees: ${formatETR(fees)}`);
```

### Dry Run (Test Transaction)

```typescript
const builder = new TransactionBuilder(api)
  .transfer(recipientAddress, amount);

const { success, error } = await builder.dryRun(alice.address);
if (!success) {
  console.error(`Would fail: ${error}`);
}
```

### Batch Transactions

```typescript
const calls = [
  api.tx.balances.transfer(recipient1, amount1),
  api.tx.balances.transfer(recipient2, amount2),
  api.tx.balances.transfer(recipient3, amount3),
];

await new TransactionBuilder(api)
  .batchAll(calls)
  .submit(alice);
```

### Subscribe to Balance Changes

```typescript
const accounts = new AccountsWrapper(api);

const unsubscribe = await accounts.subscribeBalance(
  alice.address,
  (balance) => {
    console.log(`New balance: ${formatETR(balance.total)}`);
  }
);

// Later: unsubscribe()
```

## Error Handling

```typescript
import {
  InsufficientBalanceError,
  InvalidAddressError,
  TransactionError,
} from '@etrid/sdk';

try {
  await accounts.transfer(alice, recipientAddress, amount);
} catch (error) {
  if (error instanceof InsufficientBalanceError) {
    console.error(`Short by: ${formatETR(error.getShortage())}`);
  } else if (error instanceof InvalidAddressError) {
    console.error(`Invalid address: ${error.address}`);
  } else if (error instanceof TransactionError) {
    console.error(`Transaction failed: ${error.getUserMessage()}`);
  }
}
```

## Formatters

```typescript
import {
  formatETR,
  formatETD,
  formatPercentage,
  formatAPY,
  shortenAddress,
  formatTimestamp,
  formatDuration,
  formatCompact,
} from '@etrid/sdk';

// Balance
formatETR(1500000000000000000n);  // "1.5 ETR"

// Address
shortenAddress("5Grwv...utQY");   // "5Grwv...utQY"

// Time
formatDuration(600);               // "1 hour"

// Numbers
formatCompact(1500000);            // "1.5M"
formatPercentage(15.5);            // "15.50%"
formatAPY(12.5);                   // "12.50%"
```

## Examples

### Complete Transfer Example

```typescript
async function transferExample() {
  const api = await ApiPromise.create({
    provider: new WsProvider('wss://rpc.etrid.io')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromMnemonic('your mnemonic');
  const accounts = new AccountsWrapper(api);

  // Check balance
  const balance = await accounts.getBalance(alice.address);
  console.log(`Available: ${formatETR(balance.available)}`);

  // Build transaction
  const amount = 1000000000000000000n; // 1 ETR
  const builder = new TransactionBuilder(api)
    .transferKeepAlive(recipientAddress, amount);

  // Estimate fees
  const fees = await builder.estimateFees(alice.address);
  console.log(`Fees: ${formatETR(fees)}`);

  // Submit
  const result = await builder.submit(alice);
  console.log(`Success: ${result.success}`);
  console.log(`Hash: ${result.hash}`);

  await api.disconnect();
}
```

### Complete Staking Example

```typescript
async function stakingExample() {
  const api = await ApiPromise.create({
    provider: new WsProvider('wss://rpc.etrid.io')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromMnemonic('your mnemonic');
  const staking = new StakingWrapper(api);

  // Get validators
  const validators = await staking.getValidators();
  console.log(`${validators.length} validators available`);

  // Get validator details
  const details = await staking.getValidatorDetails(validators[0]);
  console.log(`Validator: ${shortenAddress(validators[0])}`);
  console.log(`Total stake: ${formatETR(details.totalStake)}`);
  console.log(`Commission: ${formatPercentage(details.commission)}`);

  // Estimate rewards
  const amount = 10000000000000000000n; // 10 ETR
  const rewards = await staking.estimateRewards(amount);
  console.log(`Yearly rewards: ${formatETR(rewards.yearly)}`);
  console.log(`APY: ${formatAPY(rewards.apy)}`);

  // Bond tokens
  await staking.bond(alice, validators[0], amount);
  console.log(`Staked ${formatETR(amount)}`);

  await api.disconnect();
}
```

### Complete Governance Example

```typescript
async function governanceExample() {
  const api = await ApiPromise.create({
    provider: new WsProvider('wss://rpc.etrid.io')
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromMnemonic('your mnemonic');
  const governance = new GovernanceWrapper(api);

  // Get active proposals
  const proposals = await governance.getActiveProposals();
  console.log(`${proposals.length} active proposals`);

  for (const proposal of proposals) {
    console.log(`\n#${proposal.id}: ${proposal.title}`);
    console.log(`Proposer: ${shortenAddress(proposal.proposer)}`);
    console.log(`For: ${formatETR(proposal.votesFor)}`);
    console.log(`Against: ${formatETR(proposal.votesAgainst)}`);

    // Get results
    const results = await governance.getProposalResults(proposal.id);
    console.log(`Participation: ${formatPercentage(results.participationRate)}`);
    console.log(`Approved: ${results.approved ? 'Yes' : 'No'}`);
  }

  // Vote on first proposal
  if (proposals.length > 0) {
    const voteStake = 5000000000000000000n; // 5 ETR
    await governance.vote(alice, proposals[0].id, true, voteStake);
    console.log(`\nVoted YES with ${formatETR(voteStake)}`);
  }

  await api.disconnect();
}
```

## Resources

- **Full Documentation**: See `SDK_ENHANCEMENTS.md`
- **API Reference**: See inline TypeScript documentation
- **Tests**: See `tests/` directory for more examples
- **Etrid Docs**: https://docs.etrid.io
- **Discord**: https://discord.gg/etrid

## Common Patterns

### Pattern 1: Check Before Transfer

```typescript
const balance = await accounts.getBalance(alice.address);
const amount = 1000000000000000000n;

if (balance.available >= amount) {
  await new TransactionBuilder(api)
    .transferKeepAlive(recipient, amount)
    .submit(alice);
}
```

### Pattern 2: Estimate Then Submit

```typescript
const builder = new TransactionBuilder(api)
  .transfer(recipient, amount);

const fees = await builder.estimateFees(alice.address);
console.log(`This will cost ${formatETR(fees)} in fees`);

const result = await builder.submit(alice);
```

### Pattern 3: Dry Run Before Submit

```typescript
const builder = new TransactionBuilder(api)
  .transfer(recipient, amount);

const { success, error } = await builder.dryRun(alice.address);

if (success) {
  await builder.submit(alice);
} else {
  console.error(`Would fail: ${error}`);
}
```

### Pattern 4: Transaction with Options

```typescript
await new TransactionBuilder(api)
  .transfer(recipient, amount)
  .withTip(1000000n)       // Priority tip
  .withMortality(128)      // Expire after 128 blocks
  .submit(alice);
```

### Pattern 5: Comprehensive Error Handling

```typescript
import { ErrorHelpers } from '@etrid/sdk';

try {
  await operation();
} catch (error) {
  console.error(`Error: ${error.message}`);
  console.log(`Category: ${ErrorHelpers.getCategory(error)}`);

  if (ErrorHelpers.isRetryable(error)) {
    console.log('Will retry...');
  }
}
```

## Tips

1. **Always use `transferKeepAlive`** instead of `transfer` to prevent account reaping
2. **Estimate fees** before submitting large transactions
3. **Use dry run** to test complex transactions
4. **Handle errors** appropriately with specific error types
5. **Format balances** for display using `formatETR()` or `formatETD()`
6. **Subscribe to changes** for real-time updates
7. **Check connection** before operations
8. **Validate addresses** before sending transactions

## Next Steps

1. Read the full documentation in `SDK_ENHANCEMENTS.md`
2. Explore the test files for more examples
3. Check out the type definitions for all available options
4. Join the Etrid Discord for support

Happy coding! 🚀
