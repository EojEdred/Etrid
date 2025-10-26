/**
 * Fluent transaction builder for Etrid SDK
 */

import { ApiPromise } from '@polkadot/api';
import { SubmittableExtrinsic } from '@polkadot/api/types';
import { KeyringPair } from '@polkadot/keyring/types';
import { TransactionOptions, TransactionResult } from '../types/enhanced';
import { TransactionError, InvalidAmountError, NotConnectedError, ErrorHelpers } from '../errors/EtridErrors';

/**
 * Default transaction options
 */
const DEFAULT_OPTIONS: TransactionOptions = {
  tip: 0n,
  mortality: 64, // 64 blocks (~6.4 minutes)
};

/**
 * Transaction builder with fluent API
 */
export class TransactionBuilder {
  private extrinsic?: SubmittableExtrinsic<'promise'>;
  private options: TransactionOptions;
  private api: ApiPromise;

  /**
   * Create a new transaction builder
   */
  constructor(api: ApiPromise) {
    this.api = api;
    this.options = { ...DEFAULT_OPTIONS };
  }

  /**
   * Build a transfer transaction
   */
  transfer(to: string, amount: bigint): this {
    if (amount <= 0n) {
      throw new InvalidAmountError(amount, 'Transfer amount must be positive');
    }
    this.extrinsic = this.api.tx.balances.transfer(to, amount);
    return this;
  }

  /**
   * Build a transfer keep-alive transaction (ensures sender maintains existential deposit)
   */
  transferKeepAlive(to: string, amount: bigint): this {
    if (amount <= 0n) {
      throw new InvalidAmountError(amount, 'Transfer amount must be positive');
    }
    this.extrinsic = this.api.tx.balances.transferKeepAlive(to, amount);
    return this;
  }

  /**
   * Build a staking bond transaction
   */
  stake(validator: string, amount: bigint): this {
    if (amount <= 0n) {
      throw new InvalidAmountError(amount, 'Stake amount must be positive');
    }
    this.extrinsic = this.api.tx.staking.bond(validator, amount, 'Staked');
    return this;
  }

  /**
   * Build a bond additional transaction
   */
  bondAdditional(amount: bigint): this {
    if (amount <= 0n) {
      throw new InvalidAmountError(amount, 'Bond amount must be positive');
    }
    this.extrinsic = this.api.tx.staking.bondExtra(amount);
    return this;
  }

  /**
   * Build an unbond transaction
   */
  unbond(amount: bigint): this {
    if (amount <= 0n) {
      throw new InvalidAmountError(amount, 'Unbond amount must be positive');
    }
    this.extrinsic = this.api.tx.staking.unbond(amount);
    return this;
  }

  /**
   * Build a nominate transaction
   */
  nominate(validators: string[]): this {
    if (validators.length === 0) {
      throw new Error('Must nominate at least one validator');
    }
    this.extrinsic = this.api.tx.staking.nominate(validators);
    return this;
  }

  /**
   * Build a governance vote transaction
   */
  vote(proposalId: number, approve: boolean, stake: bigint): this {
    if (stake <= 0n) {
      throw new InvalidAmountError(stake, 'Vote stake must be positive');
    }
    this.extrinsic = this.api.tx.governance.vote(proposalId, approve, stake);
    return this;
  }

  /**
   * Build a governance proposal transaction
   */
  propose(title: string, description: string, call: any): this {
    this.extrinsic = this.api.tx.governance.propose(title, description, call);
    return this;
  }

  /**
   * Build an open channel transaction
   */
  openChannel(counterparty: string, balance: bigint): this {
    if (balance <= 0n) {
      throw new InvalidAmountError(balance, 'Channel balance must be positive');
    }
    this.extrinsic = this.api.tx.lightningBloc.openChannel(counterparty, balance);
    return this;
  }

  /**
   * Build a close channel transaction
   */
  closeChannel(channelId: string): this {
    this.extrinsic = this.api.tx.lightningBloc.closeChannel(channelId);
    return this;
  }

  /**
   * Build a channel payment transaction
   */
  channelPayment(channelId: string, amount: bigint): this {
    if (amount <= 0n) {
      throw new InvalidAmountError(amount, 'Payment amount must be positive');
    }
    this.extrinsic = this.api.tx.lightningBloc.sendPayment(channelId, amount);
    return this;
  }

  /**
   * Build a batch transaction (multiple calls)
   */
  batch(calls: SubmittableExtrinsic<'promise'>[]): this {
    this.extrinsic = this.api.tx.utility.batch(calls);
    return this;
  }

  /**
   * Build a batch transaction that stops on first error
   */
  batchAll(calls: SubmittableExtrinsic<'promise'>[]): this {
    this.extrinsic = this.api.tx.utility.batchAll(calls);
    return this;
  }

  /**
   * Set transaction nonce
   */
  withNonce(nonce: number): this {
    this.options.nonce = nonce;
    return this;
  }

  /**
   * Set transaction tip
   */
  withTip(tip: bigint): this {
    if (tip < 0n) {
      throw new InvalidAmountError(tip, 'Tip cannot be negative');
    }
    this.options.tip = tip;
    return this;
  }

  /**
   * Set transaction mortality period
   */
  withMortality(blocks: number): this {
    if (blocks < 0) {
      throw new Error('Mortality period cannot be negative');
    }
    this.options.mortality = blocks;
    return this;
  }

  /**
   * Make transaction immortal (no expiration)
   */
  immortal(): this {
    this.options.mortality = 0;
    return this;
  }

  /**
   * Set transaction era
   */
  withEra(era: number): this {
    this.options.era = era;
    return this;
  }

  /**
   * Get block hash for mortality period
   */
  private async getBlockHash(blocks: number): Promise<string> {
    const currentHeader = await this.api.rpc.chain.getHeader();
    const currentBlock = currentHeader.number.toNumber();
    const targetBlock = currentBlock - blocks;
    const hash = await this.api.rpc.chain.getBlockHash(targetBlock);
    return hash.toHex();
  }

  /**
   * Build and sign the transaction
   */
  async buildAndSign(signer: KeyringPair): Promise<SubmittableExtrinsic<'promise'>> {
    if (!this.extrinsic) {
      throw new Error('No transaction to build. Call a builder method first.');
    }

    if (!this.api.isConnected) {
      throw new NotConnectedError();
    }

    // Get nonce if not provided
    const nonce = this.options.nonce ?? await this.api.rpc.system.accountNextIndex(signer.address);

    // Build sign options
    const signOptions: any = {
      nonce,
      tip: this.options.tip,
    };

    // Add mortality if specified
    if (this.options.mortality && this.options.mortality > 0) {
      signOptions.era = this.options.mortality;
    }

    // Sign the transaction
    return this.extrinsic.sign(signer, signOptions);
  }

  /**
   * Estimate transaction fees
   */
  async estimateFees(address: string): Promise<bigint> {
    if (!this.extrinsic) {
      throw new Error('No transaction to estimate. Call a builder method first.');
    }

    const paymentInfo = await this.extrinsic.paymentInfo(address);
    return paymentInfo.partialFee.toBigInt();
  }

  /**
   * Get transaction length
   */
  getLength(): number {
    if (!this.extrinsic) {
      throw new Error('No transaction built');
    }
    return this.extrinsic.length;
  }

  /**
   * Submit the transaction
   */
  async submit(signer: KeyringPair): Promise<TransactionResult> {
    const signed = await this.buildAndSign(signer);

    return new Promise((resolve, reject) => {
      try {
        signed.send((result) => {
          // Transaction is in a block
          if (result.status.isInBlock) {
            console.log(`Transaction included in block ${result.status.asInBlock.toHex()}`);
          }

          // Transaction is finalized
          if (result.status.isFinalized) {
            const success = result.events.some(
              ({ event }) =>
                event.section === 'system' && event.method === 'ExtrinsicSuccess'
            );

            const error = result.events.find(
              ({ event }) =>
                event.section === 'system' && event.method === 'ExtrinsicFailed'
            );

            if (success) {
              resolve({
                hash: signed.hash.toHex(),
                block: result.status.asFinalized.toHex(),
                events: result.events.map((e) => e.toHuman()),
                success: true,
              });
            } else {
              const errorMessage = error ? 'Transaction failed' : 'Unknown error';
              reject(new TransactionError(errorMessage, result));
            }
          }

          // Transaction error
          if (result.isError) {
            reject(new TransactionError('Transaction failed', result));
          }

          // Dispatch error
          if (result.dispatchError) {
            reject(ErrorHelpers.fromDispatchError(result.dispatchError, result));
          }
        });
      } catch (error) {
        reject(ErrorHelpers.wrap(error, 'Failed to submit transaction'));
      }
    });
  }

  /**
   * Submit and wait for inclusion in block (not finalized)
   */
  async submitAndWaitForInclusion(signer: KeyringPair): Promise<TransactionResult> {
    const signed = await this.buildAndSign(signer);

    return new Promise((resolve, reject) => {
      try {
        signed.send((result) => {
          // Transaction is in a block
          if (result.status.isInBlock) {
            const success = result.events.some(
              ({ event }) =>
                event.section === 'system' && event.method === 'ExtrinsicSuccess'
            );

            if (success) {
              resolve({
                hash: signed.hash.toHex(),
                block: result.status.asInBlock.toHex(),
                events: result.events.map((e) => e.toHuman()),
                success: true,
              });
            } else {
              reject(new TransactionError('Transaction failed', result));
            }
          }

          // Transaction error
          if (result.isError) {
            reject(new TransactionError('Transaction failed', result));
          }

          // Dispatch error
          if (result.dispatchError) {
            reject(ErrorHelpers.fromDispatchError(result.dispatchError, result));
          }
        });
      } catch (error) {
        reject(ErrorHelpers.wrap(error, 'Failed to submit transaction'));
      }
    });
  }

  /**
   * Dry run the transaction (simulate without submitting)
   */
  async dryRun(address: string): Promise<{ success: boolean; error?: string }> {
    if (!this.extrinsic) {
      throw new Error('No transaction to dry run. Call a builder method first.');
    }

    try {
      const result = await this.api.rpc.system.dryRun(this.extrinsic.toHex(), null);

      // Check if dry run succeeded
      if (result.isOk) {
        return { success: true };
      } else {
        return {
          success: false,
          error: result.asErr.toString(),
        };
      }
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  /**
   * Clone the builder for reuse
   */
  clone(): TransactionBuilder {
    const builder = new TransactionBuilder(this.api);
    builder.options = { ...this.options };
    builder.extrinsic = this.extrinsic;
    return builder;
  }
}
