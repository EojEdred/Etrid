# Migration Guide: Polkadot.js → Ëtrid SDK

## Why Migrate?

The Ëtrid SDK provides:
- ✅ **Type-safe wrappers** for all Ëtrid pallets
- ✅ **Better DX** with simpler, more intuitive APIs
- ✅ **Built-in error handling** with custom error types
- ✅ **Comprehensive documentation** and examples
- ✅ **No need to understand SCALE codec** or Substrate internals

## Quick Comparison

### Connecting to Node

**Polkadot.js**:
```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider: wsProvider });
```

**Ëtrid SDK**:
```typescript
import { EtridClient } from '@etrid/sdk';

const client = new EtridClient('ws://127.0.0.1:9944');
```

### Querying Balance

**Polkadot.js**:
```typescript
const { data: { free, reserved } } = await api.query.system.account(address);
console.log('Free:', free.toHuman());
```

**Ëtrid SDK**:
```typescript
import { AccountsWrapper } from '@etrid/sdk';

const accounts = new AccountsWrapper(client.api);
const balance = await accounts.getBalance(address);
console.log('Free:', balance.free / 10n**18n, 'ÉTR');
```

### Sending Transfer

**Polkadot.js**:
```typescript
const transfer = api.tx.balances.transfer(dest, value);
const hash = await transfer.signAndSend(keypair);
```

**Ëtrid SDK**:
```typescript
const txHash = await accounts.transfer(keypair, dest, value);
```

### Staking

**Polkadot.js**:
```typescript
const bond = api.tx.staking.bond(controller, value, payee);
await bond.signAndSend(keypair);

const nominate = api.tx.staking.nominate(validators);
await nominate.signAndSend(keypair);
```

**Ëtrid SDK**:
```typescript
import { StakingWrapper } from '@etrid/sdk';

const staking = new StakingWrapper(client.api);
await staking.bond(keypair, validator, value);
await staking.nominate(keypair, validators);
```

## Complete Migration Examples

### Example 1: Balance Operations

**Before (Polkadot.js)**:
```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

const provider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider });

// Get balance
const account = await api.query.system.account(address);
const free = account.data.free.toBigInt();
const reserved = account.data.reserved.toBigInt();

// Transfer
const tx = api.tx.balances.transfer(dest, amount);
await new Promise((resolve, reject) => {
  tx.signAndSend(keypair, ({ status }) => {
    if (status.isInBlock) {
      resolve(status.asInBlock.toString());
    }
  });
});
```

**After (Ëtrid SDK)**:
```typescript
import { EtridClient, AccountsWrapper } from '@etrid/sdk';

const client = new EtridClient('ws://127.0.0.1:9944');
const accounts = new AccountsWrapper(client.api);

// Get balance
const balance = await accounts.getBalance(address);
console.log('Free:', balance.free);
console.log('Reserved:', balance.reserved);

// Transfer
const txHash = await accounts.transfer(keypair, dest, amount);
console.log('TX:', txHash);
```

### Example 2: Staking Workflow

**Before (Polkadot.js)**:
```typescript
// Get validators
const validators = await api.query.session.validators();

// Get staking info
const ledger = await api.query.staking.ledger(address);
const active = ledger.unwrap().active.toBigInt();

// Bond
const bondTx = api.tx.staking.bond(controller, amount, 'Staked');
await bondTx.signAndSend(keypair);

// Nominate
const nominateTx = api.tx.staking.nominate(validators.slice(0, 16));
await nominateTx.signAndSend(keypair);
```

**After (Ëtrid SDK)**:
```typescript
import { StakingWrapper } from '@etrid/sdk';

const staking = new StakingWrapper(client.api);

// Get validators
const validators = await staking.getValidators();

// Get staking info
const info = await staking.getStakingInfo(address);
console.log('Staked:', info.staked);

// Bond and nominate
await staking.bond(keypair, validators[0], amount);
await staking.nominate(keypair, validators.slice(0, 16));
```

### Example 3: Governance

**Before (Polkadot.js)**:
```typescript
// Get proposals
const proposals = await api.query.democracy.publicProps();

// Vote
const voteTx = api.tx.democracy.vote(refIndex, {
  Standard: { vote: { aye: true, conviction: 1 }, balance: amount }
});
await voteTx.signAndSend(keypair);
```

**After (Ëtrid SDK)**:
```typescript
import { GovernanceWrapper } from '@etrid/sdk';

const governance = new GovernanceWrapper(client.api);

// Get proposals
const proposals = await governance.getActiveProposals();

// Vote
await governance.vote(keypair, proposalId, true, amount);
```

## Type Differences

### BigInt Handling

**Polkadot.js**:
```typescript
const balance = account.data.free;
console.log(balance.toHuman());      // "1.2345 Unit"
console.log(balance.toBigInt());     // 1234500000000000000n
console.log(balance.toNumber());     // May lose precision!
```

**Ëtrid SDK**:
```typescript
const balance = await accounts.getBalance(address);
console.log(balance.free);           // bigint natively
console.log(balance.free / 10n**18n); // Convert to ÉTR
```

### Error Handling

**Polkadot.js**:
```typescript
try {
  await tx.signAndSend(keypair);
} catch (error) {
  // Generic error, hard to handle specifically
  console.error('Transaction failed:', error.message);
}
```

**Ëtrid SDK**:
```typescript
import { 
  NotConnectedError, 
  InsufficientBalanceError,
  TransactionError 
} from '@etrid/sdk';

try {
  await accounts.transfer(keypair, dest, amount);
} catch (error) {
  if (error instanceof NotConnectedError) {
    console.error('Reconnect to node');
  } else if (error instanceof InsufficientBalanceError) {
    console.error('Not enough balance');
  } else if (error instanceof TransactionError) {
    console.error('TX failed:', error.message);
  }
}
```

## Advanced Features

### Subscriptions

**Polkadot.js**:
```typescript
const unsub = await api.query.system.account(address, (account) => {
  console.log('Balance changed:', account.data.free.toHuman());
});
```

**Ëtrid SDK**:
```typescript
const unsub = await accounts.subscribeToBalance(address, (balance) => {
  console.log('Balance changed:', balance.free / 10n**18n, 'ÉTR');
});
```

### Batch Transactions

**Polkadot.js**:
```typescript
const calls = [
  api.tx.balances.transfer(dest1, amount1),
  api.tx.balances.transfer(dest2, amount2),
];
const batch = api.tx.utility.batch(calls);
await batch.signAndSend(keypair);
```

**Ëtrid SDK**:
```typescript
import { TransactionBuilder } from '@etrid/sdk';

const builder = new TransactionBuilder(client.api);
const result = await builder
  .transfer(dest1, amount1)
  .transfer(dest2, amount2)
  .batch()
  .submit(keypair);
```

## Coexistence Strategy

You can use both SDKs together during migration:

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { EtridClient, StakingWrapper } from '@etrid/sdk';

// Share the connection
const provider = new WsProvider('ws://127.0.0.1:9944');
const polkadotApi = await ApiPromise.create({ provider });

// Use Ëtrid SDK with existing API
const staking = new StakingWrapper(polkadotApi);
await staking.getValidators();

// Or use Ëtrid client and access underlying API
const client = new EtridClient('ws://127.0.0.1:9944');
const rawApi = client.api;  // Access Polkadot.js API if needed
```

## Migration Checklist

- [ ] Replace `@polkadot/api` imports with `@etrid/sdk`
- [ ] Convert `ApiPromise.create()` to `new EtridClient()`
- [ ] Replace direct pallet calls with wrapper methods
- [ ] Update balance handling to use BigInt
- [ ] Improve error handling with typed errors
- [ ] Update type annotations
- [ ] Test thoroughly
- [ ] Update documentation

## Gradual Migration Path

1. **Start with new code**: Use Ëtrid SDK for all new features
2. **Migrate critical paths**: Convert high-value operations first
3. **Convert batch operations**: Update transaction builders
4. **Replace subscriptions**: Migrate event listeners
5. **Final cleanup**: Remove Polkadot.js dependency

---

**Questions?** Join our [Discord](https://discord.gg/etrid) for migration support!
