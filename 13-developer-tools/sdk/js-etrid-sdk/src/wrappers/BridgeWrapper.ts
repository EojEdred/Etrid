/**
 * Bridge Wrapper for Ëtrid SDK
 *
 * Manages cross-chain token transfers across 13 supported blockchains
 * through Partition Burst Chains (PBCs).
 */

import { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import { TransactionError, ValidationError } from '../errors/EtridErrors';

/**
 * Supported blockchain networks
 */
export enum Chain {
  /** Bitcoin */
  BTC = 'BTC',
  /** Ethereum */
  ETH = 'ETH',
  /** Solana */
  SOL = 'SOL',
  /** Ripple */
  XRP = 'XRP',
  /** Binance Smart Chain */
  BNB = 'BNB',
  /** Tron */
  TRX = 'TRX',
  /** Cardano */
  ADA = 'ADA',
  /** Polygon */
  MATIC = 'MATIC',
  /** Chainlink */
  LINK = 'LINK',
  /** Dogecoin */
  DOGE = 'DOGE',
  /** Stellar */
  XLM = 'XLM',
  /** Tether USD */
  USDT = 'USDT',
  /** EDSC Stablecoin */
  EDSC = 'EDSC',
}

/**
 * Bridge transaction hash (can be either source or target chain)
 */
export type BridgeTxHash = string;

/**
 * Bridge transaction status
 */
export enum BridgeStatus {
  /** Transaction initiated */
  Pending = 'Pending',
  /** Awaiting confirmations */
  Confirming = 'Confirming',
  /** Confirmed on source chain */
  Confirmed = 'Confirmed',
  /** Finalized on target chain */
  Finalized = 'Finalized',
  /** Transaction failed */
  Failed = 'Failed',
  /** Transaction timed out */
  TimedOut = 'TimedOut',
}

/**
 * Bridge transaction details
 */
export interface BridgeTransaction {
  /** Bridge transaction ID */
  id: string;
  /** Source chain */
  sourceChain: Chain;
  /** Target chain */
  targetChain: Chain;
  /** Sender address (source chain format) */
  sender: string;
  /** Recipient address (target chain format) */
  recipient: string;
  /** Amount being bridged */
  amount: bigint;
  /** Current status */
  status: BridgeStatus;
  /** Source chain transaction hash */
  sourceTxHash?: string;
  /** Target chain transaction hash */
  targetTxHash?: string;
  /** Confirmations received */
  confirmations: number;
  /** Required confirmations */
  requiredConfirmations: number;
  /** Estimated completion time */
  estimatedCompletion?: Date;
  /** Bridge fee paid */
  fee: bigint;
  /** Block initiated */
  initiatedAt: number;
  /** Block completed */
  completedAt?: number;
}

/**
 * PBC (Partition Burst Chain) information
 */
export interface PBCInfo {
  /** Chain this PBC bridges to */
  chain: Chain;
  /** PBC identifier */
  pbcId: number;
  /** Collator address */
  collatorAddress: string;
  /** Total value locked in bridge */
  totalValueLocked: bigint;
  /** Number of active bridges */
  activeBridges: number;
  /** Bridge health status */
  bridgeHealth: BridgeHealth;
  /** Last checkpoint block */
  lastCheckpoint: number;
  /** Finality time (seconds) */
  finalityTime: number;
}

/**
 * Bridge health status
 */
export enum BridgeHealth {
  /** All systems operational */
  Healthy = 'Healthy',
  /** Some degradation */
  Degraded = 'Degraded',
  /** Critical issues */
  Critical = 'Critical',
  /** Bridge offline */
  Offline = 'Offline',
}

/**
 * Bridge fee breakdown
 */
export interface BridgeFee {
  /** Base bridge fee */
  baseFee: bigint;
  /** Gas fee on target chain */
  gasFee: bigint;
  /** Liquidity fee (if applicable) */
  liquidityFee: bigint;
  /** Total fee */
  totalFee: bigint;
  /** Fee as percentage of amount */
  feePercentage: number;
}

/**
 * Bridge limits
 */
export interface BridgeLimits {
  /** Minimum bridge amount */
  minAmount: bigint;
  /** Maximum bridge amount */
  maxAmount: bigint;
  /** Daily limit */
  dailyLimit: bigint;
  /** Daily amount used */
  dailyUsed: bigint;
  /** Daily remaining */
  dailyRemaining: bigint;
}

/**
 * Chain metadata
 */
export interface ChainMetadata {
  /** Chain identifier */
  chain: Chain;
  /** Chain name */
  name: string;
  /** Native token symbol */
  symbol: string;
  /** Token decimals */
  decimals: number;
  /** Block time (seconds) */
  blockTime: number;
  /** Required confirmations */
  requiredConfirmations: number;
  /** Whether chain is enabled */
  enabled: boolean;
}

/**
 * Bridge wrapper for cross-chain token transfers
 *
 * Enables seamless asset bridging across 13 major blockchains
 * using Ëtrid's Partition Burst Chain architecture.
 *
 * @example
 * ```typescript
 * const bridge = new BridgeWrapper(api);
 *
 * // Bridge ETH to BTC
 * const tx = await bridge.bridgeTokens(
 *   alice,
 *   Chain.ETH,
 *   Chain.BTC,
 *   1_000_000_000_000_000_000n,  // 1 token
 *   btcAddress
 * );
 *
 * // Check status
 * const status = await bridge.getBridgeStatus(tx.id);
 * console.log('Status:', status.status);
 * ```
 */
export class BridgeWrapper {
  constructor(private api: ApiPromise) {}

  /**
   * Bridges tokens from one chain to another
   *
   * Initiates a cross-chain token transfer through the PBC network.
   *
   * @param signer - Your account
   * @param sourceChain - Source blockchain
   * @param targetChain - Target blockchain
   * @param amount - Amount to bridge (in smallest unit)
   * @param recipient - Recipient address on target chain
   * @returns Promise resolving to bridge transaction
   *
   * @throws {ValidationError} If parameters are invalid
   * @throws {TransactionError} If bridge fails
   *
   * @example
   * ```typescript
   * // Bridge 1 ETH to BTC
   * const tx = await bridge.bridgeTokens(
   *   alice,
   *   Chain.ETH,
   *   Chain.BTC,
   *   1_000_000_000_000_000_000n,  // 1 ETH
   *   '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa'  // BTC address
   * );
   *
   * console.log('Bridge ID:', tx.id);
   * console.log('Estimated completion:', tx.estimatedCompletion);
   * ```
   */
  async bridgeTokens(
    signer: KeyringPair,
    sourceChain: Chain,
    targetChain: Chain,
    amount: bigint,
    recipient: string
  ): Promise<BridgeTransaction> {
    try {
      // Validate parameters
      if (sourceChain === targetChain) {
        throw new ValidationError('Source and target chains must be different');
      }

      if (amount <= 0n) {
        throw new ValidationError('Amount must be greater than 0');
      }

      // Check limits
      const limits = await this.getBridgeLimits(sourceChain, targetChain);
      if (amount < limits.minAmount) {
        throw new ValidationError(`Amount below minimum: ${limits.minAmount}`);
      }
      if (amount > limits.maxAmount) {
        throw new ValidationError(`Amount exceeds maximum: ${limits.maxAmount}`);
      }
      if (amount > limits.dailyRemaining) {
        throw new ValidationError(`Amount exceeds daily limit remaining: ${limits.dailyRemaining}`);
      }

      return new Promise((resolve, reject) => {
        this.api.tx.xcmBridge
          .bridgeTokens(
            sourceChain,
            targetChain,
            amount.toString(),
            recipient
          )
          .signAndSend(signer, ({ status, events, dispatchError }) => {
            if (status.isFinalized) {
              if (dispatchError) {
                if (dispatchError.isModule) {
                  const decoded = this.api.registry.findMetaError(dispatchError.asModule);
                  reject(new TransactionError(`${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`));
                } else {
                  reject(new TransactionError(dispatchError.toString()));
                }
              } else {
                // Find BridgeInitiated event
                const bridgeEvent = events.find(({ event }) =>
                  this.api.events.xcmBridge.BridgeInitiated.is(event)
                );

                if (bridgeEvent) {
                  const [bridgeId, source, target, amountData, recipientData, fee] = bridgeEvent.event.data;

                  const metadata = this.getChainMetadata(targetChain);
                  const estimatedSeconds = metadata.blockTime * metadata.requiredConfirmations;

                  resolve({
                    id: bridgeId.toString(),
                    sourceChain,
                    targetChain,
                    sender: signer.address,
                    recipient,
                    amount,
                    status: BridgeStatus.Pending,
                    confirmations: 0,
                    requiredConfirmations: metadata.requiredConfirmations,
                    estimatedCompletion: new Date(Date.now() + (estimatedSeconds * 1000)),
                    fee: BigInt(fee.toString()),
                    initiatedAt: status.asFinalized.toNumber(),
                  });
                } else {
                  reject(new TransactionError('BridgeInitiated event not found'));
                }
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to bridge tokens: ${error.message}`);
    }
  }

  /**
   * Gets bridge transaction status
   *
   * @param txHash - Bridge transaction ID or hash
   * @returns Promise resolving to transaction status
   *
   * @example
   * ```typescript
   * const status = await bridge.getBridgeStatus(bridgeId);
   *
   * console.log('Status:', status.status);
   * console.log('Confirmations:', `${status.confirmations}/${status.requiredConfirmations}`);
   *
   * if (status.status === BridgeStatus.Finalized) {
   *   console.log('Target TX:', status.targetTxHash);
   * }
   * ```
   */
  async getBridgeStatus(txHash: BridgeTxHash): Promise<BridgeTransaction> {
    try {
      const txOption = await this.api.query.xcmBridge.bridgeTransactions(txHash);

      if (txOption.isNone) {
        throw new TransactionError('Bridge transaction not found');
      }

      const tx = txOption.unwrap();

      return {
        id: txHash,
        sourceChain: this.parseChain(tx.sourceChain),
        targetChain: this.parseChain(tx.targetChain),
        sender: tx.sender.toString(),
        recipient: tx.recipient.toString(),
        amount: BigInt(tx.amount.toString()),
        status: this.parseBridgeStatus(tx.status),
        sourceTxHash: tx.sourceTxHash.isSome ? tx.sourceTxHash.unwrap().toHex() : undefined,
        targetTxHash: tx.targetTxHash.isSome ? tx.targetTxHash.unwrap().toHex() : undefined,
        confirmations: tx.confirmations.toNumber(),
        requiredConfirmations: tx.requiredConfirmations.toNumber(),
        fee: BigInt(tx.fee.toString()),
        initiatedAt: tx.initiatedAt.toNumber(),
        completedAt: tx.completedAt.isSome ? tx.completedAt.unwrap().toNumber() : undefined,
      };
    } catch (error) {
      throw new TransactionError(`Failed to get bridge status: ${error.message}`);
    }
  }

  /**
   * Gets PBC information for a chain
   *
   * @param chain - Blockchain to query
   * @returns Promise resolving to PBC details
   *
   * @example
   * ```typescript
   * const pbc = await bridge.getPBCInfo(Chain.BTC);
   * console.log('PBC ID:', pbc.pbcId);
   * console.log('TVL:', pbc.totalValueLocked);
   * console.log('Health:', pbc.bridgeHealth);
   * ```
   */
  async getPBCInfo(chain: Chain): Promise<PBCInfo> {
    try {
      const pbcOption = await this.api.query.xcmBridge.pbcInfo(chain);

      if (pbcOption.isNone) {
        throw new TransactionError(`PBC not found for chain: ${chain}`);
      }

      const pbc = pbcOption.unwrap();

      return {
        chain,
        pbcId: pbc.pbcId.toNumber(),
        collatorAddress: pbc.collator.toString(),
        totalValueLocked: BigInt(pbc.totalValueLocked.toString()),
        activeBridges: pbc.activeBridges.toNumber(),
        bridgeHealth: this.parseBridgeHealth(pbc.health),
        lastCheckpoint: pbc.lastCheckpoint.toNumber(),
        finalityTime: pbc.finalityTime.toNumber(),
      };
    } catch (error) {
      throw new TransactionError(`Failed to get PBC info: ${error.message}`);
    }
  }

  /**
   * Estimates bridge fee
   *
   * @param sourceChain - Source blockchain
   * @param targetChain - Target blockchain
   * @param amount - Amount to bridge
   * @returns Promise resolving to fee breakdown
   *
   * @example
   * ```typescript
   * const fee = await bridge.estimateBridgeFee(
   *   Chain.ETH,
   *   Chain.BTC,
   *   1_000_000_000_000_000_000n  // 1 ETH
   * );
   *
   * console.log('Total fee:', fee.totalFee);
   * console.log('Fee %:', fee.feePercentage);
   * ```
   */
  async estimateBridgeFee(
    sourceChain: Chain,
    targetChain: Chain,
    amount: bigint
  ): Promise<BridgeFee> {
    try {
      const feeData = await this.api.rpc.xcmBridge.estimateFee(
        sourceChain,
        targetChain,
        amount.toString()
      );

      const baseFee = BigInt(feeData.baseFee.toString());
      const gasFee = BigInt(feeData.gasFee.toString());
      const liquidityFee = BigInt(feeData.liquidityFee.toString());
      const totalFee = baseFee + gasFee + liquidityFee;
      const feePercentage = amount > 0n
        ? Number((totalFee * 10000n) / amount) / 100
        : 0;

      return {
        baseFee,
        gasFee,
        liquidityFee,
        totalFee,
        feePercentage,
      };
    } catch (error) {
      throw new TransactionError(`Failed to estimate fee: ${error.message}`);
    }
  }

  /**
   * Gets bridge transaction history for an address
   *
   * @param address - Account address
   * @param chain - Optional chain filter
   * @param limit - Maximum results (default: 10)
   * @returns Promise resolving to transaction history
   *
   * @example
   * ```typescript
   * const history = await bridge.getBridgeHistory(
   *   aliceAddress,
   *   Chain.ETH,  // Only ETH bridges
   *   20          // Last 20 transactions
   * );
   *
   * history.forEach(tx => {
   *   console.log(`${tx.sourceChain} → ${tx.targetChain}: ${tx.amount}`);
   * });
   * ```
   */
  async getBridgeHistory(
    address: string,
    chain?: Chain,
    limit: number = 10
  ): Promise<BridgeTransaction[]> {
    try {
      const entries = await this.api.query.xcmBridge.bridgeTransactions.entries();
      const transactions: BridgeTransaction[] = [];

      for (const [key, txOption] of entries) {
        if (txOption.isNone) continue;

        const tx = txOption.unwrap();
        const sender = tx.sender.toString();

        // Filter by address
        if (sender !== address) continue;

        // Filter by chain if specified
        if (chain) {
          const sourceChain = this.parseChain(tx.sourceChain);
          const targetChain = this.parseChain(tx.targetChain);
          if (sourceChain !== chain && targetChain !== chain) continue;
        }

        const bridgeId = key.args[0].toString();
        const bridgeTx = await this.getBridgeStatus(bridgeId);
        transactions.push(bridgeTx);

        if (transactions.length >= limit) break;
      }

      // Sort by initiated time (newest first)
      transactions.sort((a, b) => b.initiatedAt - a.initiatedAt);

      return transactions;
    } catch (error) {
      throw new TransactionError(`Failed to get bridge history: ${error.message}`);
    }
  }

  /**
   * Gets all supported chains
   *
   * @returns Promise resolving to array of supported chains
   *
   * @example
   * ```typescript
   * const chains = await bridge.getSupportedChains();
   * console.log('Supported chains:', chains.map(c => c.name).join(', '));
   * ```
   */
  async getSupportedChains(): Promise<ChainMetadata[]> {
    const chains = Object.values(Chain);
    return chains.map(chain => this.getChainMetadata(chain));
  }

  /**
   * Gets bridge limits for a chain pair
   *
   * @param sourceChain - Source blockchain
   * @param targetChain - Target blockchain
   * @returns Promise resolving to bridge limits
   *
   * @example
   * ```typescript
   * const limits = await bridge.getBridgeLimits(Chain.ETH, Chain.BTC);
   * console.log('Min:', limits.minAmount);
   * console.log('Max:', limits.maxAmount);
   * console.log('Daily remaining:', limits.dailyRemaining);
   * ```
   */
  async getBridgeLimits(
    sourceChain: Chain,
    targetChain: Chain
  ): Promise<BridgeLimits> {
    try {
      const limits = await this.api.query.xcmBridge.bridgeLimits(sourceChain, targetChain);

      return {
        minAmount: BigInt(limits.minAmount.toString()),
        maxAmount: BigInt(limits.maxAmount.toString()),
        dailyLimit: BigInt(limits.dailyLimit.toString()),
        dailyUsed: BigInt(limits.dailyUsed.toString()),
        dailyRemaining: BigInt(limits.dailyLimit.toString()) - BigInt(limits.dailyUsed.toString()),
      };
    } catch (error) {
      throw new TransactionError(`Failed to get bridge limits: ${error.message}`);
    }
  }

  /**
   * Checks if a chain is currently enabled for bridging
   *
   * @param chain - Blockchain to check
   * @returns Promise resolving to enabled status
   */
  async isChainEnabled(chain: Chain): Promise<boolean> {
    try {
      const enabled = await this.api.query.xcmBridge.enabledChains(chain);
      return enabled.isTrue;
    } catch {
      return false;
    }
  }

  /**
   * Gets chain metadata
   *
   * @param chain - Blockchain
   * @returns Chain metadata
   */
  private getChainMetadata(chain: Chain): ChainMetadata {
    const metadata: Record<Chain, Omit<ChainMetadata, 'chain'>> = {
      [Chain.BTC]: {
        name: 'Bitcoin',
        symbol: 'BTC',
        decimals: 8,
        blockTime: 600,
        requiredConfirmations: 6,
        enabled: true,
      },
      [Chain.ETH]: {
        name: 'Ethereum',
        symbol: 'ETH',
        decimals: 18,
        blockTime: 12,
        requiredConfirmations: 12,
        enabled: true,
      },
      [Chain.SOL]: {
        name: 'Solana',
        symbol: 'SOL',
        decimals: 9,
        blockTime: 0.4,
        requiredConfirmations: 32,
        enabled: true,
      },
      [Chain.XRP]: {
        name: 'Ripple',
        symbol: 'XRP',
        decimals: 6,
        blockTime: 4,
        requiredConfirmations: 1,
        enabled: true,
      },
      [Chain.BNB]: {
        name: 'Binance Smart Chain',
        symbol: 'BNB',
        decimals: 18,
        blockTime: 3,
        requiredConfirmations: 15,
        enabled: true,
      },
      [Chain.TRX]: {
        name: 'Tron',
        symbol: 'TRX',
        decimals: 6,
        blockTime: 3,
        requiredConfirmations: 19,
        enabled: true,
      },
      [Chain.ADA]: {
        name: 'Cardano',
        symbol: 'ADA',
        decimals: 6,
        blockTime: 20,
        requiredConfirmations: 15,
        enabled: true,
      },
      [Chain.MATIC]: {
        name: 'Polygon',
        symbol: 'MATIC',
        decimals: 18,
        blockTime: 2,
        requiredConfirmations: 128,
        enabled: true,
      },
      [Chain.LINK]: {
        name: 'Chainlink',
        symbol: 'LINK',
        decimals: 18,
        blockTime: 12,
        requiredConfirmations: 12,
        enabled: true,
      },
      [Chain.DOGE]: {
        name: 'Dogecoin',
        symbol: 'DOGE',
        decimals: 8,
        blockTime: 60,
        requiredConfirmations: 6,
        enabled: true,
      },
      [Chain.XLM]: {
        name: 'Stellar',
        symbol: 'XLM',
        decimals: 7,
        blockTime: 5,
        requiredConfirmations: 1,
        enabled: true,
      },
      [Chain.USDT]: {
        name: 'Tether USD',
        symbol: 'USDT',
        decimals: 6,
        blockTime: 12,
        requiredConfirmations: 12,
        enabled: true,
      },
      [Chain.EDSC]: {
        name: 'EDSC Stablecoin',
        symbol: 'EDSC',
        decimals: 18,
        blockTime: 5,
        requiredConfirmations: 3,
        enabled: true,
      },
    };

    return {
      chain,
      ...metadata[chain],
    };
  }

  private parseChain(chain: any): Chain {
    const chainStr = chain.toString();
    return Chain[chainStr as keyof typeof Chain] || Chain.ETH;
  }

  private parseBridgeStatus(status: any): BridgeStatus {
    if (status.isPending) return BridgeStatus.Pending;
    if (status.isConfirming) return BridgeStatus.Confirming;
    if (status.isConfirmed) return BridgeStatus.Confirmed;
    if (status.isFinalized) return BridgeStatus.Finalized;
    if (status.isFailed) return BridgeStatus.Failed;
    if (status.isTimedOut) return BridgeStatus.TimedOut;
    return BridgeStatus.Pending;
  }

  private parseBridgeHealth(health: any): BridgeHealth {
    if (health.isHealthy) return BridgeHealth.Healthy;
    if (health.isDegraded) return BridgeHealth.Degraded;
    if (health.isCritical) return BridgeHealth.Critical;
    if (health.isOffline) return BridgeHealth.Offline;
    return BridgeHealth.Healthy;
  }
}
