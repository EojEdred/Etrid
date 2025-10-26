/**
 * Ëtrid SDK for JavaScript/TypeScript
 *
 * Provides a comprehensive interface to interact with Ëtrid Protocol blockchain.
 *
 * @example
 * ```typescript
 * import { EtridClient, Account } from '@etrid/sdk';
 *
 * const client = new EtridClient('ws://localhost:9944');
 * await client.connect();
 *
 * const account = Account.fromMnemonic('word1 word2 ...');
 * const balance = await client.query.balance(account.address);
 * console.log(`Balance: ${balance.free} ETR`);
 * ```
 *
 * @packageDocumentation
 */

export { EtridClient } from './client';
export { Account } from './account';
export * from './types';

// Export builders
export { TransactionBuilder } from './builders/TransactionBuilder';

// Export wrappers
export { AccountsWrapper } from './wrappers/AccountsWrapper';
export { StakingWrapper } from './wrappers/StakingWrapper';
export { GovernanceWrapper } from './wrappers/GovernanceWrapper';

// Export errors
export {
  EtridError,
  TransactionError,
  ValidationError,
  NetworkError,
  InsufficientBalanceError,
  InvalidAddressError,
  InvalidAmountError,
  StakingError,
  GovernanceError,
  ChannelError,
  NotConnectedError,
  ErrorHelpers,
} from './errors/EtridErrors';

// Export enhanced types
export * from './types/enhanced';

// Export formatters
export * from './utils/formatters';

export const VERSION = '0.1.0';
