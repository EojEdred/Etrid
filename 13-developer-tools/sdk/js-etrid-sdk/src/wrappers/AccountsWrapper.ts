/**
 * Type-safe wrapper for Accounts pallet
 */

import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { ExtendedBalance, TransactionResult } from '../types/enhanced';
import { TransactionBuilder } from '../builders/TransactionBuilder';
import { NotConnectedError, InvalidAddressError } from '../errors/EtridErrors';
import { decodeAddress } from '@polkadot/util-crypto';

/**
 * Accounts wrapper for balance operations
 */
export class AccountsWrapper {
  constructor(private api: ApiPromise) {}

  /**
   * Ensure API is connected
   */
  private ensureConnected(): void {
    if (!this.api.isConnected) {
      throw new NotConnectedError();
    }
  }

  /**
   * Validate address format
   */
  private validateAddress(address: string): void {
    try {
      decodeAddress(address);
    } catch (error) {
      throw new InvalidAddressError(address, 'Invalid SS58 address format');
    }
  }

  /**
   * Get account balance with all details
   */
  async getBalance(address: string): Promise<ExtendedBalance> {
    this.ensureConnected();
    this.validateAddress(address);

    const account = await this.api.query.system.account(address);

    const free = account.data.free.toBigInt();
    const reserved = account.data.reserved.toBigInt();
    const frozen = account.data.frozen ? account.data.frozen.toBigInt() : 0n;

    // For Etrid, we might have separate ETR and ETD balances
    // This is a simplified version - adjust based on actual pallet structure
    const etr = free;
    const etd = 0n; // Would come from a separate query if ETD is tracked separately

    const total = etr + etd;
    const available = total - frozen - reserved;

    return {
      etr,
      etd,
      reserved,
      frozen,
      total,
      available,
    };
  }

  /**
   * Get free balance (spendable)
   */
  async getFreeBalance(address: string): Promise<bigint> {
    const balance = await this.getBalance(address);
    return balance.available;
  }

  /**
   * Get reserved balance
   */
  async getReservedBalance(address: string): Promise<bigint> {
    const balance = await this.getBalance(address);
    return balance.reserved;
  }

  /**
   * Get total balance (all types)
   */
  async getTotalBalance(address: string): Promise<bigint> {
    const balance = await this.getBalance(address);
    return balance.total;
  }

  /**
   * Check if account exists (has balance above existential deposit)
   */
  async accountExists(address: string): Promise<boolean> {
    this.ensureConnected();
    this.validateAddress(address);

    const account = await this.api.query.system.account(address);
    const existentialDeposit = this.api.consts.balances.existentialDeposit.toBigInt();

    return account.data.free.toBigInt() >= existentialDeposit;
  }

  /**
   * Get existential deposit
   */
  getExistentialDeposit(): bigint {
    this.ensureConnected();
    return this.api.consts.balances.existentialDeposit.toBigInt();
  }

  /**
   * Get account nonce
   */
  async getNonce(address: string): Promise<number> {
    this.ensureConnected();
    this.validateAddress(address);

    const nonce = await this.api.rpc.system.accountNextIndex(address);
    return nonce.toNumber();
  }

  /**
   * Transfer tokens
   */
  async transfer(
    from: KeyringPair,
    to: string,
    amount: bigint
  ): Promise<TransactionResult> {
    this.ensureConnected();
    this.validateAddress(to);

    return new TransactionBuilder(this.api)
      .transfer(to, amount)
      .submit(from);
  }

  /**
   * Transfer tokens (keep alive - ensures sender maintains existential deposit)
   */
  async transferKeepAlive(
    from: KeyringPair,
    to: string,
    amount: bigint
  ): Promise<TransactionResult> {
    this.ensureConnected();
    this.validateAddress(to);

    return new TransactionBuilder(this.api)
      .transferKeepAlive(to, amount)
      .submit(from);
  }

  /**
   * Transfer all balance (minus existential deposit)
   */
  async transferAll(
    from: KeyringPair,
    to: string
  ): Promise<TransactionResult> {
    this.ensureConnected();
    this.validateAddress(to);

    const balance = await this.getBalance(from.address);
    const existentialDeposit = this.getExistentialDeposit();
    const fees = 1000000n; // Estimate - should be calculated properly

    const amount = balance.available - existentialDeposit - fees;

    return new TransactionBuilder(this.api)
      .transfer(to, amount)
      .submit(from);
  }

  /**
   * Get account information
   */
  async getAccountInfo(address: string): Promise<{
    nonce: number;
    consumers: number;
    providers: number;
    sufficients: number;
    balance: ExtendedBalance;
  }> {
    this.ensureConnected();
    this.validateAddress(address);

    const account = await this.api.query.system.account(address);
    const balance = await this.getBalance(address);

    return {
      nonce: account.nonce.toNumber(),
      consumers: account.consumers.toNumber(),
      providers: account.providers.toNumber(),
      sufficients: account.sufficients.toNumber(),
      balance,
    };
  }

  /**
   * Subscribe to balance changes
   */
  async subscribeBalance(
    address: string,
    callback: (balance: ExtendedBalance) => void
  ): Promise<() => void> {
    this.ensureConnected();
    this.validateAddress(address);

    const unsubscribe = await this.api.query.system.account(address, (account) => {
      const free = account.data.free.toBigInt();
      const reserved = account.data.reserved.toBigInt();
      const frozen = account.data.frozen ? account.data.frozen.toBigInt() : 0n;

      const etr = free;
      const etd = 0n;
      const total = etr + etd;
      const available = total - frozen - reserved;

      callback({
        etr,
        etd,
        reserved,
        frozen,
        total,
        available,
      });
    });

    return unsubscribe;
  }
}
