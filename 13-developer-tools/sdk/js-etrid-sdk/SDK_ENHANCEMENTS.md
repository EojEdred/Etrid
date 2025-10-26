# Etrid TypeScript SDK Enhancements

## Overview

This document describes the comprehensive enhancements made to the Etrid TypeScript SDK, providing developers with a powerful, type-safe, and user-friendly interface for interacting with the Etrid blockchain.

## Table of Contents

- [Transaction Builder](#transaction-builder)
- [Type-Safe Wrappers](#type-safe-wrappers)
- [Error Handling](#error-handling)
- [Utility Formatters](#utility-formatters)
- [Testing](#testing)
- [Examples](#examples)

---

## Transaction Builder

The `TransactionBuilder` class provides a fluent API for constructing and submitting blockchain transactions.

### Features

- **Fluent API**: Chain multiple operations for clean, readable code
- **Type Safety**: Compile-time validation of transaction parameters
- **Flexible Options**: Configure nonce, tip, mortality, and more
- **Multiple Submission Modes**: Submit and wait for finalization or just inclusion
- **Fee Estimation**: Calculate transaction fees before submission
- **Dry Run Support**: Test transactions without submitting

### Basic Usage

```typescript
import { TransactionBuilder } from '@etrid/sdk';

// Simple transfer
const result = await new TransactionBuilder(api)
  .transfer(recipientAddress, 1000000000000000000n) // 1 ETR
  .submit(signer);

console.log(`Transaction hash: ${result.hash}`);
console.log(`Block: ${result.block}`);
```

### Advanced Usage

```typescript
// Transfer with options
const result = await new TransactionBuilder(api)
  .transferKeepAlive(recipientAddress, amount)
  .withTip(1000000n)              // Add tip for priority
  .withMortality(128)             // Expire after 128 blocks
  .withNonce(5)                   // Custom nonce
  .submit(signer);

// Estimate fees before sending
const builder = new TransactionBuilder(api)
  .transfer(recipientAddress, amount);

const fees = await builder.estimateFees(signer.address);
console.log(`Estimated fees: ${formatFee(fees)}`);

// Dry run to test
const { success, error } = await builder.dryRun(signer.address);
if (!success) {
  console.error(`Transaction would fail: ${error}`);
}
```

### Staking Operations

```typescript
// Bond tokens to a validator
await new TransactionBuilder(api)
  .stake(validatorAddress, 10000000000000000000n) // 10 ETR
  .submit(signer);

// Add more stake
await new TransactionBuilder(api)
  .bondAdditional(5000000000000000000n) // 5 ETR
  .submit(signer);

// Unbond tokens
await new TransactionBuilder(api)
  .unbond(3000000000000000000n) // 3 ETR
  .submit(signer);

// Nominate validators
await new TransactionBuilder(api)
  .nominate([validator1, validator2, validator3])
  .submit(signer);
```

### Governance Operations

```typescript
// Vote on a proposal
await new TransactionBuilder(api)
  .vote(proposalId, true, 5000000000000000000n) // Vote yes with 5 ETR stake
  .submit(signer);

// Create a proposal
const call = api.tx.system.remark('Upgrade runtime');
await new TransactionBuilder(api)
  .propose('Runtime Upgrade', 'Proposal to upgrade to v2.0', call)
  .submit(signer);
```

### Lightning Channel Operations

```typescript
// Open a payment channel
await new TransactionBuilder(api)
  .openChannel(counterpartyAddress, 1000000000000000000n) // 1 ETR capacity
  .submit(signer);

// Send payment through channel
await new TransactionBuilder(api)
  .channelPayment(channelId, 100000000000000000n) // 0.1 ETR
  .submit(signer);

// Close channel
await new TransactionBuilder(api)
  .closeChannel(channelId)
  .submit(signer);
```

### Batch Transactions

```typescript
// Execute multiple operations atomically
const calls = [
  api.tx.balances.transfer(recipient1, amount1),
  api.tx.balances.transfer(recipient2, amount2),
  api.tx.balances.transfer(recipient3, amount3),
];

// All succeed or all fail
await new TransactionBuilder(api)
  .batchAll(calls)
  .submit(signer);
```

---

## Type-Safe Wrappers

The SDK provides high-level wrappers for common blockchain operations with full TypeScript type safety.

### AccountsWrapper

Comprehensive account and balance management.

```typescript
import { AccountsWrapper } from '@etrid/sdk';

const accounts = new AccountsWrapper(api);

// Get full balance information
const balance = await accounts.getBalance(address);
console.log(`ETR: ${formatETR(balance.etr)}`);
console.log(`ETD: ${formatETD(balance.etd)}`);
console.log(`Available: ${formatETR(balance.available)}`);
console.log(`Reserved: ${formatETR(balance.reserved)}`);
console.log(`Frozen: ${formatETR(balance.frozen)}`);

// Quick balance queries
const freeBalance = await accounts.getFreeBalance(address);
const totalBalance = await accounts.getTotalBalance(address);

// Check account existence
const exists = await accounts.accountExists(address);

// Get account info
const info = await accounts.getAccountInfo(address);
console.log(`Nonce: ${info.nonce}`);
console.log(`Consumers: ${info.consumers}`);
console.log(`Providers: ${info.providers}`);

// Subscribe to balance changes
const unsubscribe = await accounts.subscribeBalance(address, (balance) => {
  console.log(`New balance: ${formatETR(balance.total)}`);
});

// Transfer operations
await accounts.transfer(signer, recipientAddress, amount);
await accounts.transferKeepAlive(signer, recipientAddress, amount);
await accounts.transferAll(signer, recipientAddress);
```

### StakingWrapper

Complete staking functionality with validator support.

```typescript
import { StakingWrapper } from '@etrid/sdk';

const staking = new StakingWrapper(api);

// Get all validators
const validators = await staking.getValidators();
console.log(`Active validators: ${validators.length}`);

// Get validator details
const details = await staking.getValidatorDetails(validatorAddress);
console.log(`Total stake: ${formatETR(details.totalStake)}`);
console.log(`Nominators: ${details.nominators}`);
console.log(`Commission: ${formatPercentage(details.commission)}`);

// Get staking info for account
const info = await staking.getStakingInfo(myAddress);
console.log(`Staked: ${formatETR(info.staked)}`);
console.log(`Status: ${info.status}`);
console.log(`Validator: ${info.validator}`);

// Get current era
const era = await staking.getCurrentEra();
console.log(`Current era: ${era}`);

// Get minimum stake requirement
const minStake = await staking.getMinimumStake();
console.log(`Minimum stake: ${formatETR(minStake)}`);

// Estimate staking rewards
const rewards = await staking.estimateRewards(10000000000000000000n); // 10 ETR
console.log(`Daily: ${formatETR(rewards.daily)}`);
console.log(`Monthly: ${formatETR(rewards.monthly)}`);
console.log(`Yearly: ${formatETR(rewards.yearly)}`);
console.log(`APY: ${formatAPY(rewards.apy)}`);

// Staking operations
await staking.bond(signer, validatorAddress, amount);
await staking.bondAdditional(signer, amount);
await staking.unbond(signer, amount);
await staking.nominate(signer, [validator1, validator2]);
await staking.withdrawUnbonded(signer);
await staking.chill(signer);

// Get rewards for specific era
const eraRewards = await staking.getRewards(address, era);
```

### GovernanceWrapper

Full governance and proposal management.

```typescript
import { GovernanceWrapper } from '@etrid/sdk';

const governance = new GovernanceWrapper(api);

// Get all active proposals
const proposals = await governance.getActiveProposals();
for (const proposal of proposals) {
  console.log(`#${proposal.id}: ${proposal.title}`);
  console.log(`  For: ${formatETR(proposal.votesFor)}`);
  console.log(`  Against: ${formatETR(proposal.votesAgainst)}`);
  console.log(`  Status: ${proposal.status}`);
}

// Get specific proposal
const proposal = await governance.getProposal(proposalId);
if (proposal) {
  console.log(`Proposer: ${shortenAddress(proposal.proposer)}`);
  console.log(`Description: ${proposal.description}`);
}

// Get proposals with pagination
const result = await governance.getProposals(0, 10);
console.log(`Page ${result.page + 1} of ${result.totalPages}`);
console.log(`Total proposals: ${result.total}`);

// Get proposal votes
const votes = await governance.getProposalVotes(proposalId);
for (const vote of votes) {
  console.log(`${shortenAddress(vote.voter)}: ${vote.approve ? 'Yes' : 'No'} (${formatETR(vote.weight)})`);
}

// Check if voted
const hasVoted = await governance.hasVoted(proposalId, myAddress);

// Get voting power
const power = await governance.getVotingPower(myAddress);
console.log(`Voting power: ${formatETR(power)}`);

// Get proposal results
const results = await governance.getProposalResults(proposalId);
console.log(`For: ${formatETR(results.votesFor)}`);
console.log(`Against: ${formatETR(results.votesAgainst)}`);
console.log(`Participation: ${formatPercentage(results.participationRate)}`);
console.log(`Approved: ${results.approved}`);

// Governance operations
await governance.createProposal(signer, title, description, call);
await governance.vote(signer, proposalId, approve, stake);
await governance.executeProposal(signer, proposalId);
await governance.cancelProposal(signer, proposalId);

// Get governance parameters
const votingPeriod = await governance.getVotingPeriod();
const minStake = await governance.getMinimumProposalStake();
```

---

## Error Handling

Comprehensive error handling with specific error types and helpful messages.

### Error Types

```typescript
import {
  EtridError,              // Base error class
  TransactionError,        // Transaction failures
  ValidationError,         // Input validation errors
  NetworkError,            // Network/connection errors
  InsufficientBalanceError, // Insufficient funds
  InvalidAddressError,     // Invalid address format
  InvalidAmountError,      // Invalid amount (negative, zero)
  StakingError,            // Staking operation errors
  GovernanceError,         // Governance operation errors
  ChannelError,            // Lightning channel errors
  NotConnectedError,       // API not connected
  ErrorHelpers,            // Helper utilities
} from '@etrid/sdk';
```

### Error Handling Examples

```typescript
try {
  await accounts.transfer(signer, recipientAddress, amount);
} catch (error) {
  if (error instanceof InsufficientBalanceError) {
    console.error(`Insufficient balance. Short by: ${formatETR(error.getShortage())}`);
    console.error(`Required: ${formatETR(error.required)}`);
    console.error(`Available: ${formatETR(error.available)}`);
  } else if (error instanceof InvalidAddressError) {
    console.error(`Invalid address: ${error.address}`);
  } else if (error instanceof TransactionError) {
    console.error(`Transaction failed: ${error.getUserMessage()}`);

    // Check specific error conditions
    if (error.isInsufficientBalance()) {
      console.error('Insufficient balance for transaction');
    }

    // Get module error details
    const moduleError = error.getModuleError();
    if (moduleError) {
      console.error(`Module ${moduleError.index}, Error ${moduleError.error}`);
    }
  } else if (error instanceof NotConnectedError) {
    console.error('Not connected to blockchain');
  }
}

// Error helpers
if (ErrorHelpers.isRetryable(error)) {
  console.log('This error is retryable');
  // Implement retry logic
}

const category = ErrorHelpers.getCategory(error);
console.log(`Error category: ${category}`);

// Wrap unknown errors
const wrappedError = ErrorHelpers.wrap(error, 'Custom message');
```

### Error Codes

All errors include error codes for programmatic handling:

```typescript
enum ErrorCode {
  // Transaction errors
  TRANSACTION_FAILED = 'TRANSACTION_FAILED',
  TRANSACTION_INVALID = 'TRANSACTION_INVALID',
  INSUFFICIENT_BALANCE = 'INSUFFICIENT_BALANCE',

  // Validation errors
  VALIDATION_ERROR = 'VALIDATION_ERROR',
  INVALID_ADDRESS = 'INVALID_ADDRESS',
  INVALID_AMOUNT = 'INVALID_AMOUNT',
  INVALID_PARAMETERS = 'INVALID_PARAMETERS',

  // Network errors
  NETWORK_ERROR = 'NETWORK_ERROR',
  CONNECTION_FAILED = 'CONNECTION_FAILED',
  RPC_ERROR = 'RPC_ERROR',

  // State errors
  NOT_CONNECTED = 'NOT_CONNECTED',
  ALREADY_CONNECTED = 'ALREADY_CONNECTED',

  // Module-specific errors
  STAKING_ERROR = 'STAKING_ERROR',
  GOVERNANCE_ERROR = 'GOVERNANCE_ERROR',
  CHANNEL_ERROR = 'CHANNEL_ERROR',

  // Unknown
  UNKNOWN_ERROR = 'UNKNOWN_ERROR',
}
```

---

## Utility Formatters

Comprehensive formatting utilities for blockchain data.

### Balance Formatting

```typescript
import {
  formatBalance,
  formatETR,
  formatETD,
  parseBalance,
  formatFee,
} from '@etrid/sdk/utils/formatters';

// Format with default options
formatBalance(1500000000000000000n);
// "1.500000000000000000 ETR"

// Format with custom decimals
formatBalance(1500000000000000000n, { decimals: 2 });
// "1.50 ETR"

// Compact notation
formatBalance(1500000000000000000n, { compact: true });
// "1.5 ETR"

// Custom symbol
formatBalance(1000000000000000000n, { symbol: 'TEST' });
// "1.000000000000000000 TEST"

// Currency-specific formatters
formatETR(1500000000000000000n);  // "1.5 ETR"
formatETD(2000000000000000000n);  // "2 ETD"

// Parse balance string
parseBalance("1.5 ETR");  // 1500000000000000000n
parseBalance("1,000.5");  // 1000500000000000000n

// Format transaction fee
formatFee(1000000000000000n);  // "0.001 ETR"
```

### Address Formatting

```typescript
import {
  formatAddress,
  shortenAddress,
  validateAndFormatAddress,
} from '@etrid/sdk/utils/formatters';

// Format with custom prefix
const formatted = formatAddress(address, 42);

// Shorten for display
shortenAddress("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
// "5Grwv...utQY"

// Custom lengths
shortenAddress(address, 8, 8);
// "5GrwvaEF...GKutQY"

// Validate and format
const result = validateAndFormatAddress(address);
if (result.valid) {
  console.log(`Formatted: ${result.formatted}`);
} else {
  console.error(`Error: ${result.error}`);
}
```

### Time and Block Formatting

```typescript
import {
  formatTimestamp,
  formatDuration,
  formatBlockTime,
  formatBlockNumber,
  formatRelativeTime,
} from '@etrid/sdk/utils/formatters';

// Format timestamp
formatTimestamp(1640000000000);
// "2021-12-20 12:26:40"

formatTimestamp(1640000000000, false);
// "2021-12-20"

// Format duration from blocks (6s block time)
formatDuration(600);   // "1 hour"
formatDuration(14400); // "1 day"
formatDuration(100);   // "10 minutes"

// Block time
formatBlockTime(100);  // "10 minutes"

// Block number with separators
formatBlockNumber(1234567);  // "1,234,567"

// Relative time
formatRelativeTime(Date.now() - 120000);  // "2 minutes ago"
formatRelativeTime(Date.now() - 86400000); // "1 day ago"
```

### Hash and Number Formatting

```typescript
import {
  formatHash,
  formatCompact,
  parseCompact,
  formatPercentage,
  formatAPY,
} from '@etrid/sdk/utils/formatters';

// Format hash
formatHash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef");
// "0x12345678...90abcdef"

// Compact notation
formatCompact(1500);       // "1.5K"
formatCompact(2500000);    // "2.5M"
formatCompact(3500000000); // "3.5B"

// Parse compact
parseCompact("1.5K");  // 1500
parseCompact("2M");    // 2000000

// Percentage
formatPercentage(15.5);     // "15.50%"
formatPercentage(15.567, 3); // "15.567%"

// APY
formatAPY(12.5);  // "12.50%"
```

---

## Testing

The SDK includes comprehensive test suites covering all functionality.

### Test Structure

```
tests/
├── TransactionBuilder.test.ts   # Builder pattern tests
├── Wrappers.test.ts             # Wrapper integration tests
├── Errors.test.ts               # Error handling tests
└── Formatters.test.ts           # Formatter utility tests
```

### Running Tests

```bash
# Install dependencies
npm install

# Run all tests
npm test

# Run specific test file
npm test TransactionBuilder.test.ts

# Run with coverage
npm test -- --coverage

# Watch mode
npm test -- --watch
```

### Test Coverage

- **Transaction Builder**: 25+ tests covering all builder methods
- **Wrappers**: 30+ tests for accounts, staking, and governance
- **Error Handling**: 35+ tests for all error types
- **Formatters**: 40+ tests for all formatting functions

**Total: 130+ test cases**

---

## Examples

### Complete Application Example

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import {
  TransactionBuilder,
  AccountsWrapper,
  StakingWrapper,
  GovernanceWrapper,
  formatETR,
  formatPercentage,
  shortenAddress,
} from '@etrid/sdk';

async function main() {
  // Connect to blockchain
  const provider = new WsProvider('wss://rpc.etrid.io');
  const api = await ApiPromise.create({ provider });

  // Initialize keyring
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromMnemonic('your mnemonic here');

  // Initialize wrappers
  const accounts = new AccountsWrapper(api);
  const staking = new StakingWrapper(api);
  const governance = new GovernanceWrapper(api);

  try {
    // Get account balance
    const balance = await accounts.getBalance(alice.address);
    console.log(`\n=== Account Balance ===`);
    console.log(`Address: ${shortenAddress(alice.address)}`);
    console.log(`Total: ${formatETR(balance.total)}`);
    console.log(`Available: ${formatETR(balance.available)}`);
    console.log(`Staked: ${formatETR(balance.reserved)}`);

    // Get staking info
    const stakingInfo = await staking.getStakingInfo(alice.address);
    console.log(`\n=== Staking Info ===`);
    console.log(`Staked: ${formatETR(stakingInfo.staked)}`);
    console.log(`Status: ${stakingInfo.status}`);
    if (stakingInfo.validator) {
      console.log(`Validator: ${shortenAddress(stakingInfo.validator)}`);
    }

    // Get validators
    const validators = await staking.getValidators();
    console.log(`\n=== Validators ===`);
    console.log(`Total validators: ${validators.length}`);

    if (validators.length > 0) {
      const validatorDetails = await staking.getValidatorDetails(validators[0]);
      console.log(`\nTop Validator:`);
      console.log(`  Address: ${shortenAddress(validators[0])}`);
      console.log(`  Total Stake: ${formatETR(validatorDetails.totalStake)}`);
      console.log(`  Nominators: ${validatorDetails.nominators}`);
      console.log(`  Commission: ${formatPercentage(validatorDetails.commission)}`);
    }

    // Get governance proposals
    const proposals = await governance.getActiveProposals();
    console.log(`\n=== Active Proposals ===`);
    console.log(`Total: ${proposals.length}`);

    for (const proposal of proposals.slice(0, 3)) {
      console.log(`\n#${proposal.id}: ${proposal.title}`);
      console.log(`  Proposer: ${shortenAddress(proposal.proposer)}`);
      console.log(`  For: ${formatETR(proposal.votesFor)}`);
      console.log(`  Against: ${formatETR(proposal.votesAgainst)}`);
      console.log(`  Status: ${proposal.status}`);
    }

    // Perform a transfer
    console.log(`\n=== Transfer ===`);
    const recipientAddress = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
    const transferAmount = 100000000000000000n; // 0.1 ETR

    // Estimate fees first
    const builder = new TransactionBuilder(api)
      .transferKeepAlive(recipientAddress, transferAmount);

    const fees = await builder.estimateFees(alice.address);
    console.log(`Transfer amount: ${formatETR(transferAmount)}`);
    console.log(`Estimated fees: ${formatETR(fees)}`);

    // Submit transaction
    const result = await builder.submit(alice);
    console.log(`Transaction hash: ${result.hash}`);
    console.log(`Block: ${result.block}`);
    console.log(`Success: ${result.success}`);

  } catch (error) {
    console.error('Error:', error);

    // Handle specific errors
    if (error instanceof InsufficientBalanceError) {
      console.error(`Insufficient balance. Short by: ${formatETR(error.getShortage())}`);
    } else if (error instanceof TransactionError) {
      console.error(`Transaction failed: ${error.getUserMessage()}`);
    }
  } finally {
    await api.disconnect();
  }
}

main().catch(console.error);
```

### Staking Example

```typescript
async function stakingExample() {
  const api = await ApiPromise.create({ provider: new WsProvider('wss://rpc.etrid.io') });
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = keyring.addFromMnemonic('your mnemonic');

  const staking = new StakingWrapper(api);

  // Get all validators
  const validators = await staking.getValidators();
  console.log(`Available validators: ${validators.length}`);

  // Get top validator by stake
  let topValidator = validators[0];
  let maxStake = 0n;

  for (const validator of validators) {
    const details = await staking.getValidatorDetails(validator);
    if (details.totalStake > maxStake) {
      maxStake = details.totalStake;
      topValidator = validator;
    }
  }

  console.log(`Top validator: ${shortenAddress(topValidator)}`);
  console.log(`Total stake: ${formatETR(maxStake)}`);

  // Bond tokens
  const bondAmount = 10000000000000000000n; // 10 ETR
  await staking.bond(signer, topValidator, bondAmount);
  console.log(`Bonded ${formatETR(bondAmount)} to ${shortenAddress(topValidator)}`);

  // Estimate rewards
  const rewards = await staking.estimateRewards(bondAmount);
  console.log(`\nEstimated Rewards:`);
  console.log(`  Daily: ${formatETR(rewards.daily)}`);
  console.log(`  Monthly: ${formatETR(rewards.monthly)}`);
  console.log(`  Yearly: ${formatETR(rewards.yearly)}`);
  console.log(`  APY: ${formatAPY(rewards.apy)}`);

  await api.disconnect();
}
```

### Governance Example

```typescript
async function governanceExample() {
  const api = await ApiPromise.create({ provider: new WsProvider('wss://rpc.etrid.io') });
  const keyring = new Keyring({ type: 'sr25519' });
  const signer = keyring.addFromMnemonic('your mnemonic');

  const governance = new GovernanceWrapper(api);

  // Get all proposals with pagination
  let page = 0;
  let hasMore = true;

  while (hasMore) {
    const result = await governance.getProposals(page, 10);

    console.log(`\nPage ${page + 1} of ${result.totalPages}`);

    for (const proposal of result.items) {
      console.log(`\n#${proposal.id}: ${proposal.title}`);
      console.log(`  ${proposal.description}`);
      console.log(`  Proposer: ${shortenAddress(proposal.proposer)}`);
      console.log(`  For: ${formatETR(proposal.votesFor)}`);
      console.log(`  Against: ${formatETR(proposal.votesAgainst)}`);

      // Get detailed results
      const results = await governance.getProposalResults(proposal.id);
      console.log(`  Participation: ${formatPercentage(results.participationRate)}`);
      console.log(`  Approved: ${results.approved ? 'Yes' : 'No'}`);

      // Check if I voted
      const hasVoted = await governance.hasVoted(proposal.id, signer.address);
      console.log(`  My vote: ${hasVoted ? 'Voted' : 'Not voted'}`);
    }

    hasMore = result.hasNext;
    page++;
  }

  // Vote on active proposal
  const activeProposals = await governance.getActiveProposals();
  if (activeProposals.length > 0) {
    const proposal = activeProposals[0];
    const voteStake = 5000000000000000000n; // 5 ETR

    await governance.vote(signer, proposal.id, true, voteStake);
    console.log(`\nVoted YES on proposal #${proposal.id} with ${formatETR(voteStake)}`);
  }

  await api.disconnect();
}
```

---

## API Reference Summary

### TransactionBuilder Methods

- `transfer(to, amount)` - Create transfer transaction
- `transferKeepAlive(to, amount)` - Transfer with existential deposit check
- `stake(validator, amount)` - Bond tokens for staking
- `bondAdditional(amount)` - Add more stake
- `unbond(amount)` - Unbond tokens
- `nominate(validators)` - Nominate validators
- `vote(proposalId, approve, stake)` - Vote on proposal
- `propose(title, description, call)` - Create proposal
- `openChannel(counterparty, balance)` - Open lightning channel
- `closeChannel(channelId)` - Close lightning channel
- `channelPayment(channelId, amount)` - Send channel payment
- `batch(calls)` - Batch multiple calls
- `batchAll(calls)` - Batch with atomic execution
- `withNonce(nonce)` - Set transaction nonce
- `withTip(tip)` - Set transaction tip
- `withMortality(blocks)` - Set mortality period
- `immortal()` - Make transaction immortal
- `estimateFees(address)` - Estimate transaction fees
- `dryRun(address)` - Test transaction without submitting
- `submit(signer)` - Submit and wait for finalization
- `submitAndWaitForInclusion(signer)` - Submit and wait for inclusion

### AccountsWrapper Methods

- `getBalance(address)` - Get full balance details
- `getFreeBalance(address)` - Get spendable balance
- `getReservedBalance(address)` - Get reserved balance
- `getTotalBalance(address)` - Get total balance
- `accountExists(address)` - Check if account exists
- `getExistentialDeposit()` - Get existential deposit amount
- `getNonce(address)` - Get account nonce
- `getAccountInfo(address)` - Get comprehensive account info
- `transfer(from, to, amount)` - Transfer tokens
- `transferKeepAlive(from, to, amount)` - Safe transfer
- `transferAll(from, to)` - Transfer all balance
- `subscribeBalance(address, callback)` - Subscribe to balance changes

### StakingWrapper Methods

- `getValidators()` - Get all validators
- `getValidatorStatus(address)` - Get validator status
- `getValidatorDetails(address)` - Get validator details
- `getStakingInfo(address)` - Get account staking info
- `getCurrentEra()` - Get current era
- `getMinimumStake()` - Get minimum stake amount
- `getUnbondingPeriod()` - Get unbonding period
- `estimateRewards(amount)` - Estimate staking rewards
- `getRewards(address, era)` - Get rewards for era
- `bond(from, validator, amount)` - Bond tokens
- `bondAdditional(from, amount)` - Add more stake
- `unbond(from, amount)` - Unbond tokens
- `nominate(from, validators)` - Nominate validators
- `withdrawUnbonded(from)` - Withdraw unbonded tokens
- `chill(from)` - Stop nominating/validating

### GovernanceWrapper Methods

- `getActiveProposals()` - Get all active proposals
- `getProposal(proposalId)` - Get specific proposal
- `getProposals(page, limit)` - Get proposals with pagination
- `getProposalVotes(proposalId)` - Get all votes for proposal
- `getVote(proposalId, voter)` - Get specific vote
- `hasVoted(proposalId, address)` - Check if voted
- `getProposalResults(proposalId)` - Get proposal results
- `getProposalCount()` - Get total proposal count
- `getVotingPeriod()` - Get voting period in blocks
- `getMinimumProposalStake()` - Get minimum stake for proposals
- `getVotingPower(address)` - Get voting power
- `createProposal(from, title, description, call)` - Create proposal
- `vote(from, proposalId, approve, stake)` - Vote on proposal
- `executeProposal(from, proposalId)` - Execute passed proposal
- `cancelProposal(from, proposalId)` - Cancel proposal

---

## Conclusion

The Etrid TypeScript SDK provides a comprehensive, type-safe, and developer-friendly interface for blockchain interactions. With fluent APIs, comprehensive error handling, and extensive testing, developers can build robust applications with confidence.

For more information, visit the [Etrid Documentation](https://docs.etrid.io) or join our [Discord Community](https://discord.gg/etrid).
