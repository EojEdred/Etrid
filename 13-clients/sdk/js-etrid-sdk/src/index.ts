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

export const VERSION = '0.1.0';
