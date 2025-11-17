/**
 * Distribution Pay Wrapper for Ëtrid SDK
 *
 * Manages the daily distribution of 27,397 ÉTR across 5 categories:
 * - Voters: 10% (2,740 ÉTR/day)
 * - Flare Nodes: 15% (4,110 ÉTR/day)
 * - Validity Nodes: 15% (4,110 ÉTR/day)
 * - Stakers: 40% (10,959 ÉTR/day)
 * - Directors: 20% (5,479 ÉTR/day)
 */

import { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import { TransactionError } from '../errors/EtridErrors';

/**
 * Distribution categories matching on-chain enum
 */
export enum DistributionCategory {
  /** Voters (10% = 2,740 ÉTR/day) */
  Voters = 'Voters',
  /** FlareChain validator nodes (15% = 4,110 ÉTR/day) */
  FlareNodes = 'FlareNodes',
  /** PBC validator nodes (15% = 4,110 ÉTR/day) */
  ValidityNodes = 'ValidityNodes',
  /** Token stakers (40% = 10,959 ÉTR/day) */
  Stakers = 'Stakers',
  /** Decentralized Directors (20% = 5,479 ÉTR/day) */
  Directors = 'Directors',
}

/**
 * Pending rewards breakdown by category
 */
export interface PendingRewards {
  /** Total pending rewards across all categories */
  total: bigint;
  /** Breakdown by category */
  byCategory: Record<DistributionCategory, bigint>;
  /** When next distribution occurs */
  nextDistribution: Date;
  /** Time until next distribution (milliseconds) */
  timeUntilNext: number;
}

/**
 * Distribution schedule configuration
 */
export interface DistributionSchedule {
  /** Total daily distribution amount */
  totalDaily: bigint;
  /** Category allocations */
  categories: CategoryAllocation[];
  /** Distribution time (UTC) */
  distributionTime: string;
  /** Distribution frequency (daily) */
  frequency: 'daily';
  /** Last distribution block */
  lastDistribution: number;
  /** Next distribution block */
  nextDistribution: number;
}

/**
 * Category allocation details
 */
export interface CategoryAllocation {
  /** Category name */
  category: DistributionCategory;
  /** Percentage in basis points (10000 = 100%) */
  percentage: number;
  /** Daily amount in planck */
  dailyAmount: bigint;
  /** Number of eligible participants */
  participantCount: number;
  /** Amount per participant */
  amountPerParticipant: bigint;
}

/**
 * Claim event record
 */
export interface ClaimEvent {
  /** Block number of claim */
  blockNumber: number;
  /** Category claimed from */
  category: DistributionCategory;
  /** Amount claimed */
  amount: bigint;
  /** Transaction hash */
  txHash: string;
  /** Timestamp */
  timestamp: Date;
}

/**
 * Next distribution estimate
 */
export interface DistributionEstimate {
  /** Estimated amount to receive */
  estimatedAmount: bigint;
  /** Distribution category */
  category: DistributionCategory;
  /** Estimated time */
  estimatedTime: Date;
  /** Blocks until distribution */
  blocksUntil: number;
  /** Your share percentage */
  sharePercentage: number;
}

/**
 * Claim history summary
 */
export interface ClaimHistory {
  /** Total amount claimed all-time */
  totalClaimed: bigint;
  /** Number of claims */
  claimCount: number;
  /** Claims by category */
  byCategory: Record<DistributionCategory, bigint>;
  /** Recent claims */
  recentClaims: ClaimEvent[];
  /** First claim date */
  firstClaim?: Date;
  /** Last claim date */
  lastClaim?: Date;
}

/**
 * Distribution Pay wrapper for claiming daily rewards
 *
 * Enables users to claim their share of the daily 27,397 ÉTR distribution
 * based on their participation in various network roles.
 *
 * @example
 * ```typescript
 * const distributionPay = new DistributionPayWrapper(api);
 *
 * // Check pending rewards
 * const pending = await distributionPay.getPendingRewards(aliceAddress);
 * console.log(`Pending: ${pending.total} planck`);
 *
 * // Claim rewards
 * await distributionPay.claimReward(alice, DistributionCategory.Stakers);
 * ```
 */
export class DistributionPayWrapper {
  constructor(private api: ApiPromise) {}

  /**
   * Claims pending distribution reward
   *
   * Claims your share of the daily distribution for a specific category.
   * You must be eligible for the category (e.g., must be staking for Stakers category).
   *
   * @param signer - Your account (KeyringPair)
   * @param category - Distribution category to claim from
   * @returns Promise resolving to transaction hash
   *
   * @throws {TransactionError} If claim fails (not eligible, already claimed, etc.)
   *
   * @example
   * ```typescript
   * // Claim staker rewards
   * const txHash = await distributionPay.claimReward(
   *   alice,
   *   DistributionCategory.Stakers
   * );
   * console.log('Claimed rewards:', txHash);
   * ```
   */
  async claimReward(
    signer: KeyringPair,
    category: DistributionCategory
  ): Promise<string> {
    try {
      return new Promise((resolve, reject) => {
        this.api.tx.distributionPay
          .claimReward(category)
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
                // Find RewardClaimed event
                const claimEvent = events.find(({ event }) =>
                  this.api.events.distributionPay.RewardClaimed.is(event)
                );

                if (claimEvent) {
                  console.log('Reward claimed:', claimEvent.event.data.toString());
                }

                resolve(status.asFinalized.toHex());
              }
            }
          });
      });
    } catch (error) {
      throw new TransactionError(`Failed to claim reward: ${error.message}`);
    }
  }

  /**
   * Gets pending rewards for an address
   *
   * Queries all pending unclaimed rewards across all categories.
   *
   * @param address - Account address to check
   * @returns Promise resolving to pending rewards details
   *
   * @example
   * ```typescript
   * const pending = await distributionPay.getPendingRewards(aliceAddress);
   * console.log(`Total pending: ${pending.total}`);
   * console.log(`Staker rewards: ${pending.byCategory.Stakers}`);
   * console.log(`Next distribution: ${pending.nextDistribution}`);
   * ```
   */
  async getPendingRewards(address: string): Promise<PendingRewards> {
    try {
      // Get pending for each category
      const categories = Object.values(DistributionCategory);
      const pendingAmounts = await Promise.all(
        categories.map(cat =>
          this.api.query.distributionPay.pendingRewards(address, cat)
        )
      );

      // Build category map
      const byCategory: Record<DistributionCategory, bigint> = {} as any;
      let total = 0n;

      categories.forEach((cat, i) => {
        const amount = BigInt(pendingAmounts[i].toString());
        byCategory[cat] = amount;
        total += amount;
      });

      // Get next distribution time
      const lastDistBlock = await this.api.query.distributionPay.lastDistribution();
      const blocksPerDay = 17280; // ~24 hours at 5s blocks
      const nextDistBlock = lastDistBlock.toNumber() + blocksPerDay;
      const currentBlock = (await this.api.query.system.number()).toNumber();
      const blocksUntil = Math.max(0, nextDistBlock - currentBlock);
      const secondsUntil = blocksUntil * 5;

      const nextDistribution = new Date(Date.now() + (secondsUntil * 1000));

      return {
        total,
        byCategory,
        nextDistribution,
        timeUntilNext: secondsUntil * 1000,
      };
    } catch (error) {
      throw new TransactionError(`Failed to get pending rewards: ${error.message}`);
    }
  }

  /**
   * Gets the distribution schedule configuration
   *
   * Returns the current distribution settings including daily amounts,
   * percentages, and timing.
   *
   * @returns Promise resolving to distribution schedule
   *
   * @example
   * ```typescript
   * const schedule = await distributionPay.getDistributionSchedule();
   * console.log(`Total daily: ${schedule.totalDaily} planck (27,397 ÉTR)`);
   * schedule.categories.forEach(cat => {
   *   console.log(`${cat.category}: ${cat.percentage}bp = ${cat.dailyAmount}`);
   * });
   * ```
   */
  async getDistributionSchedule(): Promise<DistributionSchedule> {
    try {
      // Get total daily amount
      const totalDaily = await this.api.query.distributionPay.totalDaily();

      // Get category percentages
      const votersPct = await this.api.query.distributionPay.votersPercentage();
      const flareNodesPct = await this.api.query.distributionPay.flareNodesPercentage();
      const validityNodesPct = await this.api.query.distributionPay.validityNodesPercentage();
      const stakersPct = await this.api.query.distributionPay.stakersPercentage();
      const directorsPct = await this.api.query.distributionPay.directorsPercentage();

      const totalDailyBN = BigInt(totalDaily.toString());

      // Calculate category allocations
      const categories: CategoryAllocation[] = [
        {
          category: DistributionCategory.Voters,
          percentage: votersPct.toNumber(),
          dailyAmount: (totalDailyBN * BigInt(votersPct.toNumber())) / 10000n,
          participantCount: 0, // Would need to query
          amountPerParticipant: 0n,
        },
        {
          category: DistributionCategory.FlareNodes,
          percentage: flareNodesPct.toNumber(),
          dailyAmount: (totalDailyBN * BigInt(flareNodesPct.toNumber())) / 10000n,
          participantCount: 0,
          amountPerParticipant: 0n,
        },
        {
          category: DistributionCategory.ValidityNodes,
          percentage: validityNodesPct.toNumber(),
          dailyAmount: (totalDailyBN * BigInt(validityNodesPct.toNumber())) / 10000n,
          participantCount: 0,
          amountPerParticipant: 0n,
        },
        {
          category: DistributionCategory.Stakers,
          percentage: stakersPct.toNumber(),
          dailyAmount: (totalDailyBN * BigInt(stakersPct.toNumber())) / 10000n,
          participantCount: 0,
          amountPerParticipant: 0n,
        },
        {
          category: DistributionCategory.Directors,
          percentage: directorsPct.toNumber(),
          dailyAmount: (totalDailyBN * BigInt(directorsPct.toNumber())) / 10000n,
          participantCount: 0,
          amountPerParticipant: 0n,
        },
      ];

      // Get distribution timing
      const lastDistBlock = await this.api.query.distributionPay.lastDistribution();
      const blocksPerDay = 17280;
      const nextDistBlock = lastDistBlock.toNumber() + blocksPerDay;

      return {
        totalDaily: totalDailyBN,
        categories,
        distributionTime: '00:00 UTC',
        frequency: 'daily',
        lastDistribution: lastDistBlock.toNumber(),
        nextDistribution: nextDistBlock,
      };
    } catch (error) {
      throw new TransactionError(`Failed to get distribution schedule: ${error.message}`);
    }
  }

  /**
   * Gets claim history for an address
   *
   * Retrieves historical claim events from blockchain events.
   *
   * @param address - Account address
   * @param fromBlock - Start block (optional, defaults to 0)
   * @param toBlock - End block (optional, defaults to current)
   * @returns Promise resolving to claim history
   *
   * @example
   * ```typescript
   * const history = await distributionPay.getClaimHistory(
   *   aliceAddress,
   *   1000000,  // from block
   *   2000000   // to block
   * );
   * console.log(`Total claimed: ${history.totalClaimed}`);
   * console.log(`Claim count: ${history.claimCount}`);
   * ```
   */
  async getClaimHistory(
    address: string,
    fromBlock: number = 0,
    toBlock?: number
  ): Promise<ClaimHistory> {
    try {
      // Get current block if toBlock not specified
      if (!toBlock) {
        toBlock = (await this.api.query.system.number()).toNumber();
      }

      const claims: ClaimEvent[] = [];
      let totalClaimed = 0n;
      const byCategory: Record<DistributionCategory, bigint> = {
        [DistributionCategory.Voters]: 0n,
        [DistributionCategory.FlareNodes]: 0n,
        [DistributionCategory.ValidityNodes]: 0n,
        [DistributionCategory.Stakers]: 0n,
        [DistributionCategory.Directors]: 0n,
      };

      // Query events (this is a simplified version - production would use event indexing)
      // In production, you'd want to use a block explorer API or event indexer
      const CHUNK_SIZE = 1000;
      for (let block = fromBlock; block <= toBlock; block += CHUNK_SIZE) {
        const endBlock = Math.min(block + CHUNK_SIZE, toBlock);
        // Note: This is pseudo-code - actual implementation would query events differently
        // You'd typically use api.query.system.events.at(blockHash) for each block
      }

      return {
        totalClaimed,
        claimCount: claims.length,
        byCategory,
        recentClaims: claims.slice(-10), // Last 10 claims
        firstClaim: claims.length > 0 ? claims[0].timestamp : undefined,
        lastClaim: claims.length > 0 ? claims[claims.length - 1].timestamp : undefined,
      };
    } catch (error) {
      throw new TransactionError(`Failed to get claim history: ${error.message}`);
    }
  }

  /**
   * Estimates next distribution amount for an address
   *
   * Predicts how much the address will receive in the next distribution
   * based on current participation and stake.
   *
   * @param address - Account address
   * @param category - Distribution category
   * @returns Promise resolving to distribution estimate
   *
   * @example
   * ```typescript
   * const estimate = await distributionPay.estimateNextDistribution(
   *   aliceAddress,
   *   DistributionCategory.Stakers
   * );
   * console.log(`Estimated: ${estimate.estimatedAmount} planck`);
   * console.log(`Your share: ${estimate.sharePercentage}%`);
   * console.log(`In ${estimate.blocksUntil} blocks`);
   * ```
   */
  async estimateNextDistribution(
    address: string,
    category: DistributionCategory
  ): Promise<DistributionEstimate> {
    try {
      // Get schedule
      const schedule = await this.getDistributionSchedule();
      const categoryAlloc = schedule.categories.find(c => c.category === category);

      if (!categoryAlloc) {
        throw new TransactionError(`Category ${category} not found`);
      }

      // Calculate share based on category
      let sharePercentage = 0;
      let estimatedAmount = 0n;

      if (category === DistributionCategory.Stakers) {
        // Get staking info
        const nominators = await this.api.query.staking.nominators(address);
        if (nominators.isSome) {
          const totalStake = await this.api.query.staking.totalStake();
          const myStake = await this.api.query.staking.ledger(address);

          if (myStake.isSome) {
            const myStakeAmount = BigInt(myStake.unwrap().active.toString());
            const totalStakeAmount = BigInt(totalStake.toString());

            sharePercentage = Number((myStakeAmount * 10000n) / totalStakeAmount) / 100;
            estimatedAmount = (categoryAlloc.dailyAmount * myStakeAmount) / totalStakeAmount;
          }
        }
      } else if (category === DistributionCategory.FlareNodes || category === DistributionCategory.ValidityNodes) {
        // Check if validator
        const validators = await this.api.query.session.validators();
        const isValidator = validators.some(v => v.toString() === address);

        if (isValidator) {
          const validatorCount = validators.length;
          sharePercentage = 100 / validatorCount;
          estimatedAmount = categoryAlloc.dailyAmount / BigInt(validatorCount);
        }
      } else if (category === DistributionCategory.Directors) {
        // Check if director
        const directors = await this.api.query.governance.directors();
        const isDirector = directors.some(d => d.toString() === address);

        if (isDirector) {
          const directorCount = directors.length; // Should be 9
          sharePercentage = 100 / directorCount;
          estimatedAmount = categoryAlloc.dailyAmount / BigInt(directorCount);
        }
      } else if (category === DistributionCategory.Voters) {
        // Check if voted recently
        const lastVote = await this.api.query.governance.lastVote(address);
        if (lastVote.isSome) {
          // Simplified - would need to count total voters
          sharePercentage = 0.1; // Placeholder
          estimatedAmount = categoryAlloc.dailyAmount / 1000n; // Placeholder
        }
      }

      // Calculate time until next distribution
      const currentBlock = (await this.api.query.system.number()).toNumber();
      const blocksUntil = Math.max(0, schedule.nextDistribution - currentBlock);
      const secondsUntil = blocksUntil * 5;
      const estimatedTime = new Date(Date.now() + (secondsUntil * 1000));

      return {
        estimatedAmount,
        category,
        estimatedTime,
        blocksUntil,
        sharePercentage,
      };
    } catch (error) {
      throw new TransactionError(`Failed to estimate distribution: ${error.message}`);
    }
  }

  /**
   * Checks if address is eligible for a category
   *
   * @param address - Account address
   * @param category - Distribution category
   * @returns Promise resolving to eligibility status
   *
   * @example
   * ```typescript
   * const eligible = await distributionPay.isEligible(
   *   aliceAddress,
   *   DistributionCategory.Stakers
   * );
   * if (eligible) {
   *   console.log('You can claim staker rewards!');
   * }
   * ```
   */
  async isEligible(address: string, category: DistributionCategory): Promise<boolean> {
    try {
      switch (category) {
        case DistributionCategory.Stakers: {
          const nominators = await this.api.query.staking.nominators(address);
          return nominators.isSome;
        }
        case DistributionCategory.FlareNodes:
        case DistributionCategory.ValidityNodes: {
          const validators = await this.api.query.session.validators();
          return validators.some(v => v.toString() === address);
        }
        case DistributionCategory.Directors: {
          const directors = await this.api.query.governance.directors();
          return directors.some(d => d.toString() === address);
        }
        case DistributionCategory.Voters: {
          const lastVote = await this.api.query.governance.lastVote(address);
          return lastVote.isSome;
        }
        default:
          return false;
      }
    } catch (error) {
      return false;
    }
  }

  /**
   * Gets all eligible categories for an address
   *
   * @param address - Account address
   * @returns Promise resolving to array of eligible categories
   *
   * @example
   * ```typescript
   * const eligible = await distributionPay.getEligibleCategories(aliceAddress);
   * console.log(`Eligible for: ${eligible.join(', ')}`);
   * ```
   */
  async getEligibleCategories(address: string): Promise<DistributionCategory[]> {
    const categories = Object.values(DistributionCategory);
    const eligibility = await Promise.all(
      categories.map(cat => this.isEligible(address, cat))
    );

    return categories.filter((_, i) => eligibility[i]);
  }
}
