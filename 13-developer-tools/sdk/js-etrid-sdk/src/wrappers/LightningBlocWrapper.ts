/**
 * Lightning-Bloc Wrapper for Ëtrid SDK
 *
 * Provides high-level interface for Lightning-Bloc payment channels
 * enabling instant, zero-fee transactions at 500K+ TPS.
 */

import { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { Bytes, u128, u64, u32, Option } from '@polkadot/types';
import { ChannelError } from '../errors/EtridErrors';

/**
 * Channel ID (unique identifier for payment channel)
 */
export type ChannelId = string;

/**
 * Payment route through network
 */
export interface PaymentRoute {
  /** Path of addresses from sender to recipient */
  path: string[];
  /** Total routing fees */
  totalFee: bigint;
  /** Estimated time in milliseconds */
  estimatedTime: number;
  /** Number of hops */
  hopCount: number;
}

/**
 * Channel status enum
 */
export enum ChannelStatus {
  Open = 'Open',
  Closing = 'Closing',
  Closed = 'Closed',
  Disputed = 'Disputed',
}

/**
 * Payment channel structure
 */
export interface Channel {
  /** Unique channel ID */
  id: ChannelId;
  /** First party's address */
  partyA: string;
  /** Second party's address */
  partyB: string;
  /** Party A's balance in channel */
  balanceA: bigint;
  /** Party B's balance in channel */
  balanceB: bigint;
  /** Current nonce (increments with each update) */
  nonce: number;
  /** Block number when channel expires */
  expiresAt: number;
  /** Current channel status */
  status: ChannelStatus;
}

/**
 * Off-chain channel state
 */
export interface ChannelState {
  /** Channel ID */
  channelId: ChannelId;
  /** Party A's balance */
  balanceA: bigint;
  /** Party B's balance */
  balanceB: bigint;
  /** State nonce */
  nonce: number;
  /** Party A's signature (if signed) */
  signatureA?: Uint8Array;
  /** Party B's signature (if signed) */
  signatureB?: Uint8Array;
}

/**
 * Channel balance information
 */
export interface ChannelBalance {
  /** Your balance in channel */
  yourBalance: bigint;
  /** Counterparty's balance */
  theirBalance: bigint;
  /** Total channel capacity */
  totalCapacity: bigint;
  /** Your percentage of channel */
  yourPercentage: number;
}

/**
 * Routing fee estimate
 */
export interface RoutingFeeEstimate {
  /** Base fee */
  baseFee: bigint;
  /** Per-hop fee */
  perHopFee: bigint;
  /** Total fee for route */
  totalFee: bigint;
  /** Fee as percentage of payment */
  feePercentage: number;
}

/**
 * Channel update result
 */
export interface ChannelUpdateResult {
  /** Updated channel state */
  state: ChannelState;
  /** Whether update was successful */
  success: boolean;
  /** Error message if failed */
  error?: string;
}

/**
 * Lightning-Bloc wrapper for payment channel operations
 *
 * Enables instant, zero-fee transactions through off-chain payment channels
 * with multi-hop routing capabilities.
 *
 * @example
 * ```typescript
 * const lightningBloc = new LightningBlocWrapper(api);
 *
 * // Open a channel
 * const channelId = await lightningBloc.openChannel(
 *   bobAddress,
 *   10_000_000_000_000_000_000n,  // 10 ETR
 *   10_000_000_000_000_000_000n,  // 10 ETR
 *   28800                          // ~2 days
 * );
 *
 * // Make instant payment
 * await lightningBloc.updateChannel(
 *   channelId,
 *   1_000_000_000_000_000_000n,   // 1 ETR
 *   1,
 *   signature
 * );
 *
 * // Close channel
 * await lightningBloc.closeChannel(channelId);
 * ```
 */
export class LightningBlocWrapper {
  constructor(private api: ApiPromise) {}

  /**
   * Opens a new Lightning-Bloc payment channel
   *
   * Creates a bidirectional payment channel between you and a counterparty.
   * Both parties must deposit funds to open the channel.
   *
   * @param signer - Your account (KeyringPair)
   * @param counterparty - Counterparty's address (SS58 format)
   * @param myDeposit - Your initial deposit (in planck, 1 ETR = 10^18 planck)
   * @param theirDeposit - Required deposit from counterparty
   * @param duration - Channel duration in blocks (~5s per block)
   * @returns Promise resolving to the new channel ID
   *
   * @throws {InsufficientBalanceError} If sender balance is too low
   * @throws {InvalidAddressError} If counterparty address is invalid
   * @throws {ChannelError} If channel creation fails
   *
   * @example
   * ```typescript
   * const channelId = await lightningBloc.openChannel(
   *   alice,
   *   bobAddress,
   *   10_000_000_000_000_000_000n,  // 10 ETR
   *   10_000_000_000_000_000_000n,  // 10 ETR
   *   28800                          // ~2 days (5s blocks)
   * );
   * console.log('Channel opened:', channelId);
   * ```
   */
  async openChannel(
    signer: KeyringPair,
    counterparty: string,
    myDeposit: bigint,
    theirDeposit: bigint,
    duration: number
  ): Promise<ChannelId> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.lightningBloc
          .openChannel(counterparty, myDeposit.toString(), theirDeposit.toString(), duration)
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new ChannelError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new ChannelError(dispatchError.toString()));
                }
              } else {
                // Find ChannelOpened event
                const channelEvent = events.find(({ event }) =>
                  this.api.events.lightningBloc.ChannelOpened.is(event)
                );

                if (channelEvent) {
                  const [channelId] = channelEvent.event.data;
                  resolve(channelId.toString());
                } else {
                  reject(new ChannelError('ChannelOpened event not found'));
                }
              }
            }
          });
      });
    } catch (error) {
      throw new ChannelError(`Failed to open channel: ${error.message}`);
    }
  }

  /**
   * Closes an open Lightning-Bloc payment channel
   *
   * Settles the channel on-chain using the latest agreed state.
   * Both parties receive their final balances.
   *
   * @param signer - Your account
   * @param channelId - The channel ID to close
   * @returns Promise resolving to transaction hash
   *
   * @throws {ChannelError} If channel doesn't exist or is already closed
   *
   * @example
   * ```typescript
   * const txHash = await lightningBloc.closeChannel(alice, channelId);
   * console.log('Channel closed:', txHash);
   * ```
   */
  async closeChannel(
    signer: KeyringPair,
    channelId: ChannelId
  ): Promise<string> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.lightningBloc
          .closeChannel(channelId)
          .signAndSend(signer, ({ status, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new ChannelError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new ChannelError(dispatchError.toString()));
                }
              } else {
                resolve(status.asFinalized.toHex());
              }
            }
          });
      });
    } catch (error) {
      throw new ChannelError(`Failed to close channel: ${error.message}`);
    }
  }

  /**
   * Forces a channel closure with disputed state
   *
   * Use this if the counterparty is unresponsive or trying to cheat.
   * Requires the latest signed channel state and your signature.
   *
   * @param signer - Your account
   * @param channelId - The channel ID
   * @param latestState - Latest agreed channel state
   * @param signature - Your signature on the state
   * @returns Promise resolving to transaction hash
   *
   * @throws {ChannelError} If force close fails
   *
   * @example
   * ```typescript
   * const txHash = await lightningBloc.forceClose(
   *   alice,
   *   channelId,
   *   latestState,
   *   mySignature
   * );
   * ```
   */
  async forceClose(
    signer: KeyringPair,
    channelId: ChannelId,
    latestState: ChannelState,
    signature: Uint8Array
  ): Promise<string> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.lightningBloc
          .forceClose(
            channelId,
            latestState.nonce,
            latestState.balanceA.toString(),
            latestState.balanceB.toString(),
            signature
          )
          .signAndSend(signer, ({ status, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new ChannelError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new ChannelError(dispatchError.toString()));
                }
              } else {
                resolve(status.asFinalized.toHex());
              }
            }
          });
      });
    } catch (error) {
      throw new ChannelError(`Failed to force close channel: ${error.message}`);
    }
  }

  /**
   * Updates channel state with new balance distribution (off-chain)
   *
   * Creates a new channel state with updated balances. This is an off-chain
   * operation that doesn't require a blockchain transaction until channel close.
   *
   * @param signer - Your account
   * @param channelId - The channel ID
   * @param amount - Amount to transfer (positive = A to B, negative = B to A)
   * @param nonce - New nonce (must be > previous nonce)
   * @param theirSignature - Counterparty's signature on this state
   * @returns Promise resolving to new channel state
   *
   * @throws {ChannelError} If update fails (invalid nonce, insufficient balance, etc.)
   *
   * @example
   * ```typescript
   * // Transfer 1 ETR from A to B
   * const newState = await lightningBloc.updateChannel(
   *   alice,
   *   channelId,
   *   1_000_000_000_000_000_000n,  // 1 ETR
   *   2,  // nonce
   *   bobSignature
   * );
   * ```
   */
  async updateChannel(
    signer: KeyringPair,
    channelId: ChannelId,
    amount: bigint,
    nonce: number,
    theirSignature: Uint8Array
  ): Promise<ChannelUpdateResult> {
    try {
      // Get current channel state
      const channel = await this.getChannel(channelId);
      if (!channel) {
        throw new ChannelError('Channel not found');
      }

      // Validate nonce
      if (nonce <= channel.nonce) {
        throw new ChannelError(`Invalid nonce: must be > ${channel.nonce}`);
      }

      // Calculate new balances
      let newBalanceA: bigint;
      let newBalanceB: bigint;

      if (amount >= 0) {
        // A to B transfer
        newBalanceA = channel.balanceA - amount;
        newBalanceB = channel.balanceB + amount;
      } else {
        // B to A transfer
        newBalanceA = channel.balanceA + (-amount);
        newBalanceB = channel.balanceB - (-amount);
      }

      // Validate balances
      if (newBalanceA < 0n || newBalanceB < 0n) {
        throw new ChannelError('Insufficient channel balance');
      }

      // Create new state
      const newState: ChannelState = {
        channelId,
        balanceA: newBalanceA,
        balanceB: newBalanceB,
        nonce,
        signatureB: theirSignature,
      };

      // Sign the new state
      const stateHash = this.hashChannelState(newState);
      const mySignature = signer.sign(stateHash);
      newState.signatureA = mySignature;

      return {
        state: newState,
        success: true,
      };
    } catch (error) {
      return {
        state: null as any,
        success: false,
        error: error.message,
      };
    }
  }

  /**
   * Routes a payment through the Lightning-Bloc network
   *
   * Finds and executes a payment route from sender to recipient,
   * potentially through multiple intermediate channels (up to 20 hops).
   *
   * @param signer - Your account
   * @param recipient - Final recipient's address
   * @param amount - Amount to send
   * @param maxHops - Maximum number of hops (default: 20)
   * @returns Promise resolving to payment route details
   *
   * @throws {ChannelError} If no route found or payment fails
   *
   * @example
   * ```typescript
   * // Pay Bob 5 ETR through the network
   * const route = await lightningBloc.routePayment(
   *   alice,
   *   bobAddress,
   *   5_000_000_000_000_000_000n,  // 5 ETR
   *   10  // max 10 hops
   * );
   * console.log(`Payment routed through ${route.hopCount} hops`);
   * ```
   */
  async routePayment(
    signer: KeyringPair,
    recipient: string,
    amount: bigint,
    maxHops: number = 20
  ): Promise<PaymentRoute> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.lightningBloc
          .routePayment(recipient, amount.toString(), maxHops)
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new ChannelError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new ChannelError(dispatchError.toString()));
                }
              } else {
                // Find PaymentRouted event
                const routeEvent = events.find(({ event }) =>
                  this.api.events.lightningBloc.PaymentRouted.is(event)
                );

                if (routeEvent) {
                  const [path, totalFee, hopCount] = routeEvent.event.data;
                  resolve({
                    path: path.toJSON() as string[],
                    totalFee: BigInt(totalFee.toString()),
                    hopCount: hopCount.toNumber(),
                    estimatedTime: hopCount.toNumber() * 100, // ~100ms per hop
                  });
                } else {
                  reject(new ChannelError('PaymentRouted event not found'));
                }
              }
            }
          });
      });
    } catch (error) {
      throw new ChannelError(`Failed to route payment: ${error.message}`);
    }
  }

  /**
   * Gets channel information
   *
   * @param channelId - The channel ID
   * @returns Promise resolving to channel details or null if not found
   *
   * @example
   * ```typescript
   * const channel = await lightningBloc.getChannel(channelId);
   * console.log(`Channel balance: ${channel.balanceA} / ${channel.balanceB}`);
   * ```
   */
  async getChannel(channelId: ChannelId): Promise<Channel | null> {
    try {
      const channelOption = await this.api.query.lightningBloc.channels(channelId);

      if (channelOption.isNone) {
        return null;
      }

      const channel = channelOption.unwrap();
      return {
        id: channelId,
        partyA: channel.partyA.toString(),
        partyB: channel.partyB.toString(),
        balanceA: BigInt(channel.balanceA.toString()),
        balanceB: BigInt(channel.balanceB.toString()),
        nonce: channel.nonce.toNumber(),
        expiresAt: channel.expiresAt.toNumber(),
        status: this.parseChannelStatus(channel.status),
      };
    } catch (error) {
      throw new ChannelError(`Failed to get channel: ${error.message}`);
    }
  }

  /**
   * Gets all channels for an address
   *
   * @param address - Account address
   * @returns Promise resolving to array of channels
   *
   * @example
   * ```typescript
   * const channels = await lightningBloc.getMyChannels(aliceAddress);
   * console.log(`You have ${channels.length} channels`);
   * ```
   */
  async getMyChannels(address: string): Promise<Channel[]> {
    try {
      const channelIds = await this.api.query.lightningBloc.accountChannels(address);
      const channels: Channel[] = [];

      for (const channelId of channelIds) {
        const channel = await this.getChannel(channelId.toString());
        if (channel) {
          channels.push(channel);
        }
      }

      return channels;
    } catch (error) {
      throw new ChannelError(`Failed to get channels: ${error.message}`);
    }
  }

  /**
   * Gets channel balance information
   *
   * @param channelId - The channel ID
   * @param myAddress - Your address (to determine which side you're on)
   * @returns Promise resolving to balance information
   *
   * @example
   * ```typescript
   * const balance = await lightningBloc.getChannelBalance(channelId, aliceAddress);
   * console.log(`Your balance: ${balance.yourBalance}`);
   * console.log(`Your share: ${balance.yourPercentage}%`);
   * ```
   */
  async getChannelBalance(channelId: ChannelId, myAddress: string): Promise<ChannelBalance> {
    const channel = await this.getChannel(channelId);
    if (!channel) {
      throw new ChannelError('Channel not found');
    }

    const isPartyA = channel.partyA === myAddress;
    const yourBalance = isPartyA ? channel.balanceA : channel.balanceB;
    const theirBalance = isPartyA ? channel.balanceB : channel.balanceA;
    const totalCapacity = yourBalance + theirBalance;
    const yourPercentage = totalCapacity > 0n
      ? Number((yourBalance * 10000n) / totalCapacity) / 100
      : 0;

    return {
      yourBalance,
      theirBalance,
      totalCapacity,
      yourPercentage,
    };
  }

  /**
   * Estimates routing fees for a payment
   *
   * @param amount - Payment amount
   * @param hops - Number of hops in route
   * @returns Promise resolving to fee estimate
   *
   * @example
   * ```typescript
   * const fees = await lightningBloc.estimateRoutingFee(
   *   1_000_000_000_000_000_000n,  // 1 ETR
   *   5  // 5 hops
   * );
   * console.log(`Total fee: ${fees.totalFee}`);
   * ```
   */
  async estimateRoutingFee(amount: bigint, hops: number): Promise<RoutingFeeEstimate> {
    try {
      const baseFee = await this.api.query.lightningBloc.baseFee();
      const perHopFee = await this.api.query.lightningBloc.perHopFee();

      const baseFeeValue = BigInt(baseFee.toString());
      const perHopFeeValue = BigInt(perHopFee.toString());
      const totalFee = baseFeeValue + (perHopFeeValue * BigInt(hops));
      const feePercentage = amount > 0n
        ? Number((totalFee * 10000n) / amount) / 100
        : 0;

      return {
        baseFee: baseFeeValue,
        perHopFee: perHopFeeValue,
        totalFee,
        feePercentage,
      };
    } catch (error) {
      throw new ChannelError(`Failed to estimate routing fee: ${error.message}`);
    }
  }

  /**
   * Hashes channel state for signing
   *
   * @private
   */
  private hashChannelState(state: ChannelState): Uint8Array {
    const data = this.api.createType('(Bytes, u128, u128, u32)', [
      state.channelId,
      state.balanceA.toString(),
      state.balanceB.toString(),
      state.nonce,
    ]);
    return this.api.registry.hash(data.toU8a()).toU8a();
  }

  /**
   * Parses channel status from chain enum
   *
   * @private
   */
  private parseChannelStatus(status: any): ChannelStatus {
    if (status.isOpen) return ChannelStatus.Open;
    if (status.isClosing) return ChannelStatus.Closing;
    if (status.isClosed) return ChannelStatus.Closed;
    if (status.isDisputed) return ChannelStatus.Disputed;
    return ChannelStatus.Open; // default
  }
}
