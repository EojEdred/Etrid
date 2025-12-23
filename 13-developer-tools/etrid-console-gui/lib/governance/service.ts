/**
 * ETRID Governance Service
 *
 * Complete governance functionality for the ETRID web wallet
 * Connects to the real chain at wss://rpc.etrid.org
 *
 * *** UPDATED TO FETCH REAL DATA FROM DEMOCRACY PALLET ***
 * - Uses api.query.democracy.publicProps() for pending proposals
 * - Uses api.query.democracy.referendumInfoOf() for active/finished referendums
 * - Uses api.query.democracy.votingOf() for vote information
 * - Falls back gracefully if democracy pallet is not available
 */

import { ApiPromise } from '@polkadot/api';
import type { Option, Vec, u32 } from '@polkadot/types';
import type { AccountId, BalanceOf, Hash } from '@polkadot/types/interfaces';
import type { ITuple } from '@polkadot/types/types';
import { primearcCoreChainApi } from '../api/primearc-core-chain';
import {
  ProposalCategory,
  ProposalStatus,
  VoteType,
  GovernanceErrorCode,
  ConvictionLevel,
  getConvictionConfig,
  calculateVotingPower,
} from './types';
import type {
  Proposal,
  ProposalFilters,
  CreateProposalParams,
  CastVoteParams,
  VoteRecord,
  VotingStats,
  VotingPower,
  VotingPowerBreakdown,
  DelegateParams,
  Delegation,
  DelegationStats,
  GovernanceStats,
  CategoryStats,
  TransactionResult,
  GovernanceError,
  PaginatedResponse,
  PaginationParams,
} from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// GOVERNANCE SERVICE CLASS
// ═══════════════════════════════════════════════════════════════════════════════

export class GovernanceService {
  private api: ApiPromise | null = null;
  private connectionPromise: Promise<ApiPromise> | null = null;

  /**
   * Ensure connection to ETRID chain
   */
  private async ensureConnection(): Promise<ApiPromise> {
    if (this.api && this.api.isConnected) {
      return this.api;
    }

    // Reuse existing connection attempt if in progress
    if (this.connectionPromise) {
      return this.connectionPromise;
    }

    this.connectionPromise = primearcCoreChainApi.connectToPrimearcCoreChain();

    try {
      this.api = await this.connectionPromise;
      console.log('✅ Connected to ETRID governance pallet');
      return this.api;
    } catch (error) {
      this.connectionPromise = null;
      throw this.createError(
        GovernanceErrorCode.NotConnected,
        'Failed to connect to ETRID chain',
        error
      );
    }
  }

  /**
   * Create a standardized error
   */
  private createError(
    code: GovernanceErrorCode,
    message: string,
    details?: any
  ): GovernanceError {
    return { code, message, details };
  }

  /**
   * Parse proposal category from chain data
   */
  private parseCategory(categoryData: any): ProposalCategory {
    if (typeof categoryData === 'string') {
      return categoryData as ProposalCategory;
    }

    // Handle enum-style data from chain
    if (categoryData.isInflationRate) return ProposalCategory.InflationRate;
    if (categoryData.isParameterChange) return ProposalCategory.ParameterChange;
    if (categoryData.isBudgetAllocation) return ProposalCategory.BudgetAllocation;
    if (categoryData.isProtocolUpgrade) return ProposalCategory.ProtocolUpgrade;
    if (categoryData.isDirectorElection) return ProposalCategory.DirectorElection;
    if (categoryData.isEmergencyAction) return ProposalCategory.EmergencyAction;

    return ProposalCategory.ParameterChange;
  }

  /**
   * Parse proposal status from chain data
   */
  private parseStatus(statusData: any): ProposalStatus {
    if (typeof statusData === 'string') {
      return statusData as ProposalStatus;
    }

    // Handle enum-style data from chain
    if (statusData.isPending) return ProposalStatus.Pending;
    if (statusData.isActive) return ProposalStatus.Active;
    if (statusData.isPassed) return ProposalStatus.Passed;
    if (statusData.isRejected) return ProposalStatus.Rejected;
    if (statusData.isExecuted) return ProposalStatus.Executed;
    if (statusData.isCancelled) return ProposalStatus.Cancelled;

    return ProposalStatus.Pending;
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // PROPOSAL QUERIES
  // ═══════════════════════════════════════════════════════════════════════════

  /**
   * Get all proposals with optional filters
   * Fetches from democracy.publicProps() and democracy.referendumInfoOf()
   */
  async getProposals(filters?: ProposalFilters): Promise<Proposal[]> {
    try {
      const api = await this.ensureConnection();

      const proposals: Proposal[] = [];

      // Check if democracy pallet exists
      if (!api.query.democracy) {
        console.warn('Democracy pallet not found on chain');
        return [];
      }

      // Fetch public proposals (not yet referendums)
      try {
        const publicProps = (await api.query.democracy.publicProps()) as any;

        for (const [index, hash, proposer] of publicProps) {
          const depositInfo = (await api.query.democracy.depositOf(index.toNumber())) as any;
          const deposit = depositInfo.isSome ? depositInfo.unwrap()[1].toString() : '0';

          proposals.push({
            id: index.toNumber(),
            proposer: proposer.toString(),
            title: `Proposal #${index.toNumber()}`,
            description: `Democracy proposal ${hash.toHex()}`,
            category: ProposalCategory.ParameterChange,
            status: ProposalStatus.Pending,
            createdAt: 0,
            votingDeadline: 0,
            votesFor: '0',
            votesAgainst: '0',
            votesAbstain: '0',
            totalVotingPower: deposit,
            approved: false,
            executed: false,
          });
        }
      } catch (err) {
        console.error('Error fetching public proposals:', err);
      }

      // Fetch active and recent referendums
      try {
        const refCount = (await api.query.democracy.referendumCount()) as any;
        const count = refCount.toNumber();
        const currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();

        // Query last 50 referendums or all if less
        const startIndex = Math.max(0, count - 50);

        for (let i = startIndex; i < count; i++) {
          const refInfo = (await api.query.democracy.referendumInfoOf(i)) as any;

          if (refInfo.isNone) continue;

          const info = refInfo.unwrap();

          if (info.isOngoing) {
                        const ongoing = info.asOngoing;
                        const end = (ongoing.end as any).toNumber();
                        const isPassed = currentBlock >= end;
            proposals.push({
              id: i + 10000, // Offset to distinguish from public proposals
              proposer: 'Democracy',
              title: `Referendum #${i}`,
              description: `Active referendum ${ongoing.proposalHash.toHex()}`,
              category: ProposalCategory.ParameterChange,
              status: isPassed ? ProposalStatus.Passed : ProposalStatus.Active,
              createdAt: 0,
              votingDeadline: end,
              votesFor: ongoing.tally.ayes.toString(),
              votesAgainst: ongoing.tally.nays.toString(),
              votesAbstain: '0',
              totalVotingPower: ongoing.tally.turnout.toString(),
              approved: isPassed,
              executed: false,
            });
          } else if (info.isFinished) {
            const finished = info.asFinished;

            proposals.push({
              id: i + 10000,
              proposer: 'Democracy',
              title: `Referendum #${i}`,
              description: `Finished referendum`,
              category: ProposalCategory.ParameterChange,
              status: finished.approved.isTrue ? ProposalStatus.Passed : ProposalStatus.Rejected,
              createdAt: 0,
              votingDeadline: 0,
              votesFor: '0',
              votesAgainst: '0',
              votesAbstain: '0',
              totalVotingPower: '0',
              approved: finished.approved.isTrue,
              executed: true,
              executedAt: finished.end.toNumber(),
            });
          }
        }
      } catch (err) {
        console.error('Error fetching referendums:', err);
      }

      // Apply filters
      let filtered = proposals;
      if (filters?.categories) {
        filtered = filtered.filter(p => filters.categories!.includes(p.category));
      }
      if (filters?.statuses) {
        filtered = filtered.filter(p => filters.statuses!.includes(p.status));
      }
      if (filters?.proposer) {
        filtered = filtered.filter(p => p.proposer === filters.proposer);
      }

      return filtered;
    } catch (error) {
      console.error('Failed to fetch proposals:', error);
      return [];
    }
  }

  /**
   * Get a specific proposal by ID
   * Handles both public proposals (<10000) and referendums (>=10000)
   */
  async getProposal(proposalId: number): Promise<Proposal | null> {
    try {
      const api = await this.ensureConnection();

      if (!api.query.democracy) {
        console.warn('Democracy pallet not found');
        return null;
      }

      // If ID >= 10000, it's a referendum
      if (proposalId >= 10000) {
        const refIndex = proposalId - 10000;
        const refInfo = (await api.query.democracy.referendumInfoOf(refIndex)) as any;

        if (refInfo.isNone) {
          return null;
        }

        const info = refInfo.unwrap();
        const currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();

        if (info.isOngoing) {
                      const ongoing = info.asOngoing;
                      const end = (ongoing.end as any).toNumber();
                      const isPassed = currentBlock >= end;
          return {
            id: proposalId,
            proposer: 'Democracy',
            title: `Referendum #${refIndex}`,
            description: `Active referendum ${ongoing.proposalHash.toHex()}`,
            category: ProposalCategory.ParameterChange,
            status: isPassed ? ProposalStatus.Passed : ProposalStatus.Active,
            createdAt: 0,
            votingDeadline: end,
            votesFor: ongoing.tally.ayes.toString(),
            votesAgainst: ongoing.tally.nays.toString(),
            votesAbstain: '0',
            totalVotingPower: ongoing.tally.turnout.toString(),
            approved: isPassed,
            executed: false,
          };
        } else if (info.isFinished) {
          const finished = info.asFinished;

          return {
            id: proposalId,
            proposer: 'Democracy',
            title: `Referendum #${refIndex}`,
            description: `Finished referendum`,
            category: ProposalCategory.ParameterChange,
            status: finished.approved.isTrue ? ProposalStatus.Passed : ProposalStatus.Rejected,
            createdAt: 0,
            votingDeadline: 0,
            votesFor: '0',
            votesAgainst: '0',
            votesAbstain: '0',
            totalVotingPower: '0',
            approved: finished.approved.isTrue,
            executed: true,
            executedAt: finished.end.toNumber(),
          };
        }
      } else {
        // It's a public proposal
        const publicProps = (await api.query.democracy.publicProps()) as any;
        const proposal = publicProps.find(([index]: any) => index.toNumber() === proposalId);

        if (!proposal) {
          return null;
        }

        const [index, hash, proposer] = proposal;
        const depositInfo = (await api.query.democracy.depositOf(index.toNumber())) as any;
        const deposit = depositInfo.isSome ? depositInfo.unwrap()[1].toString() : '0';

        return {
          id: proposalId,
          proposer: proposer.toString(),
          title: `Proposal #${proposalId}`,
          description: `Democracy proposal ${hash.toHex()}`,
          category: ProposalCategory.ParameterChange,
          status: ProposalStatus.Pending,
          createdAt: 0,
          votingDeadline: 0,
          votesFor: '0',
          votesAgainst: '0',
          votesAbstain: '0',
          totalVotingPower: deposit,
          approved: false,
          executed: false,
        };
      }

      return null;
    } catch (error) {
      console.error(`Failed to fetch proposal ${proposalId}:`, error);
      return null;
    }
  }

  /**
   * Get active proposals
   */
  async getActiveProposals(): Promise<Proposal[]> {
    return this.getProposals({
      statuses: [ProposalStatus.Active],
    });
  }

  /**
   * Get proposals by category
   */
  async getProposalsByCategory(category: ProposalCategory): Promise<Proposal[]> {
    return this.getProposals({
      categories: [category],
    });
  }

  /**
   * Get proposals with pagination
   */
  async getProposalsPaginated(
    params: PaginationParams,
    filters?: ProposalFilters
  ): Promise<PaginatedResponse<Proposal>> {
    const allProposals = await this.getProposals(filters);
    const start = (params.page - 1) * params.pageSize;
    const end = start + params.pageSize;
    const paginatedData = allProposals.slice(start, end);

    return {
      data: paginatedData,
      total: allProposals.length,
      page: params.page,
      pageSize: params.pageSize,
      hasMore: end < allProposals.length,
    };
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // PROPOSAL SUBMISSION
  // ═══════════════════════════════════════════════════════════════════════════

  /**
   * Submit a new governance proposal
   */
  async submitProposal(
    params: CreateProposalParams,
    signer: any
  ): Promise<TransactionResult> {
    try {
      const api = await this.ensureConnection();

      // Encode category
      const categoryEncoded = this.encodeCategoryForChain(params.category);

      // Create proposal extrinsic
      const tx = api.tx.governance.submitProposal(
        params.title,
        params.description,
        categoryEncoded,
        params.metadata || {}
      );

      // Sign and send
      return await this.signAndSend(tx, signer);
    } catch (error: any) {
      return {
        success: false,
        error: this.createError(
          GovernanceErrorCode.TransactionFailed,
          'Failed to submit proposal',
          error
        ),
      };
    }
  }

  /**
   * Encode proposal category for chain submission
   */
  private encodeCategoryForChain(category: ProposalCategory): any {
    const categoryMap: Record<ProposalCategory, number> = {
      [ProposalCategory.InflationRate]: 0,
      [ProposalCategory.ParameterChange]: 1,
      [ProposalCategory.BudgetAllocation]: 2,
      [ProposalCategory.ProtocolUpgrade]: 3,
      [ProposalCategory.DirectorElection]: 4,
      [ProposalCategory.EmergencyAction]: 5,
    };

    return categoryMap[category];
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // VOTING
  // ═══════════════════════════════════════════════════════════════════════════

  /**
   * Cast a vote on a proposal with conviction
   * Uses democracy.vote() for referendums
   */
  async castVote(params: CastVoteParams, signer: any): Promise<TransactionResult> {
    try {
      const api = await this.ensureConnection();

      if (!api.tx.democracy) {
        return {
          success: false,
          error: this.createError(
            GovernanceErrorCode.NotConnected,
            'Democracy pallet not available on chain'
          ),
        };
      }

      // Referendum IDs are offset by 10000
      const refIndex = params.proposalId >= 10000 ? params.proposalId - 10000 : params.proposalId;

      // Create vote object for democracy pallet
      const vote = {
        Standard: {
          vote: {
            aye: params.voteType === VoteType.Aye,
            conviction: params.conviction || ConvictionLevel.None,
          },
          balance: params.balance || '0',
        },
      };

      const tx = api.tx.democracy.vote(refIndex, vote);

      return await this.signAndSend(tx, signer);
    } catch (error: any) {
      return {
        success: false,
        error: this.createError(
          GovernanceErrorCode.TransactionFailed,
          'Failed to cast vote',
          error
        ),
      };
    }
  }

  /**
   * Encode vote type for chain submission
   */
  private encodeVoteType(voteType: VoteType): any {
    const voteMap: Record<VoteType, number> = {
      [VoteType.Aye]: 0,
      [VoteType.Nay]: 1,
      [VoteType.Abstain]: 2,
    };

    return voteMap[voteType];
  }

  /**
   * Get votes for a proposal/referendum
   * Queries democracy.votingOf() to find all votes
   */
  async getVotes(proposalId: number): Promise<VoteRecord[]> {
    try {
      const api = await this.ensureConnection();

      if (!api.query.democracy) {
        return [];
      }

      const refIndex = proposalId >= 10000 ? proposalId - 10000 : proposalId;
      const votes: VoteRecord[] = [];

      // Query all voting info entries
      const votingInfo = (await api.query.democracy.votingOf.entries()) as any;

      for (const [key, value] of votingInfo) {
        const voter = key.args[0].toString();

        if (value.isDirect) {
          const direct = value.asDirect;
          const voterVotes = direct.votes;

          // Find vote for this specific referendum
          for (const [voteRefIndex, voteData] of (voterVotes as any)) {
            if (voteRefIndex.toNumber() === refIndex) {
              if (voteData.isStandard) {
                const standard = voteData.asStandard;
                const isAye = standard.vote.isAye;
                const conviction = standard.vote.conviction.toNumber();
                const balance = standard.balance.toString();

                votes.push({
                  voter,
                  proposalId,
                  voteType: isAye ? VoteType.Aye : VoteType.Nay,
                  conviction: conviction as ConvictionLevel,
                  balance,
                  votingPower: balance, // Simple calculation, could apply conviction multiplier
                  timestamp: 0,
                });
              }
            }
          }
        }
      }

      return votes;
    } catch (error) {
      console.error(`Failed to fetch votes for proposal ${proposalId}:`, error);
      return [];
    }
  }

  /**
   * Parse vote type from chain data
   */
  private parseVoteType(voteData: any): VoteType {
    if (typeof voteData === 'string') {
      return voteData as VoteType;
    }

    if (voteData.isAye || voteData === 0) return VoteType.Aye;
    if (voteData.isNay || voteData === 1) return VoteType.Nay;
    if (voteData.isAbstain || voteData === 2) return VoteType.Abstain;

    return VoteType.Abstain;
  }

  /**
   * Get voting statistics for a proposal
   */
  async getVotingStats(proposalId: number): Promise<VotingStats> {
    const votes = await this.getVotes(proposalId);

    const ayeVotes = votes.filter(v => v.voteType === VoteType.Aye);
    const nayVotes = votes.filter(v => v.voteType === VoteType.Nay);
    const abstainVotes = votes.filter(v => v.voteType === VoteType.Abstain);

    const ayePower = ayeVotes.reduce((sum, v) => sum + parseFloat(v.votingPower), 0);
    const nayPower = nayVotes.reduce((sum, v) => sum + parseFloat(v.votingPower), 0);
    const abstainPower = abstainVotes.reduce((sum, v) => sum + parseFloat(v.votingPower), 0);
    const totalPower = ayePower + nayPower + abstainPower;

    return {
      proposalId,
      totalVotes: votes.length,
      totalVotingPower: totalPower.toString(),
      ayeVotes: ayeVotes.length,
      ayePower: ayePower.toString(),
      nayVotes: nayVotes.length,
      nayPower: nayPower.toString(),
      abstainVotes: abstainVotes.length,
      abstainPower: abstainPower.toString(),
      turnoutPercent: 0, // Calculate based on circulating supply
      passingPercent: totalPower > 0 ? (ayePower / totalPower) * 100 : 0,
    };
  }

  /**
   * Check if an account has voted on a proposal/referendum
   */
  async hasVoted(proposalId: number, account: string): Promise<boolean> {
    try {
      const api = await this.ensureConnection();

      if (!api.query.democracy) {
        return false;
      }

      const refIndex = proposalId >= 10000 ? proposalId - 10000 : proposalId;
      const votingInfo = (await api.query.democracy.votingOf(account)) as any;

      if (votingInfo.isDirect) {
        const direct = votingInfo.asDirect;
        const votes = direct.votes;

        // Check if this account has voted on this referendum
        return votes.some(([voteRefIndex]: any) => voteRefIndex.toNumber() === refIndex);
      }

      return false;
    } catch (error) {
      return false;
    }
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // VOTING POWER
  // ═══════════════════════════════════════════════════════════════════════════

  /**
   * Get voting power for an account
   */
  async getVotingPower(account: string): Promise<VotingPower> {
    try {
      const api = await this.ensureConnection();

      // Get account balance info
      const accountInfo = (await api.query.system.account(account)) as any;
      const balanceData = accountInfo.data;

      // Get staking info
      const stakingLedger = (await api.query.staking.ledger(account)) as any;
      const stakedBalance = stakingLedger.isSome
        ? stakingLedger.unwrap().active.toString()
        : '0';

      // Get lock information
      const locks = (await api.query.balances.locks(account)) as any;
      const governanceLock = locks.find((lock: any) =>
        lock.id.toHuman() === 'govrnce' || lock.id.toHuman() === 'democracy'
      );

      const lockedBalance = governanceLock?.amount.toString() || '0';
      const lockedUntil = governanceLock?.until?.toNumber();

      // Get active votes and delegations
      const votesCount = await this.getActiveVotesCount(account);
      const delegationsCount = await this.getActiveDelegationsCount(account);

      // Calculate voting power
      const baseVotingPower = stakedBalance;
      const delegatedPower = await this.getDelegatedVotingPower(account);
      const totalPower = (parseFloat(baseVotingPower) + parseFloat(delegatedPower)).toString();

      return {
        address: account,
        stakedBalance,
        lockedBalance,
        lockedUntil,
        lockPeriodDays: lockedUntil ? this.calculateLockPeriodDays(lockedUntil) : 0,
        baseVotingPower,
        delegatedVotingPower: delegatedPower,
        totalVotingPower: totalPower,
        canVote: parseFloat(totalPower) > 0,
        canDelegate: parseFloat(stakedBalance) > 0,
        participationHistory: 0, // Can be tracked separately
        activeVotes: votesCount,
        activeDelegations: delegationsCount,
      };
    } catch (error) {
      console.error(`Failed to get voting power for ${account}:`, error);

      return {
        address: account,
        stakedBalance: '0',
        lockedBalance: '0',
        baseVotingPower: '0',
        delegatedVotingPower: '0',
        totalVotingPower: '0',
        canVote: false,
        canDelegate: false,
        participationHistory: 0,
        activeVotes: 0,
        activeDelegations: 0,
      };
    }
  }

  /**
   * Calculate voting power breakdown with conviction
   */
  calculateVotingPowerBreakdown(
    balance: string,
    conviction: ConvictionLevel
  ): VotingPowerBreakdown {
    const config = getConvictionConfig(conviction);
    const calculatedPower = calculateVotingPower(balance, conviction);

    const unlockDate = conviction !== ConvictionLevel.None
      ? new Date(Date.now() + config.lockPeriodDays * 24 * 60 * 60 * 1000)
      : undefined;

    return {
      baseBalance: balance,
      conviction,
      multiplier: config.multiplier,
      calculatedPower,
      lockPeriodDays: config.lockPeriodDays,
      unlockDate,
    };
  }

  private calculateLockPeriodDays(unlockBlock: number): number {
    // Assuming 6 second block time
    const currentBlock = 0; // Get from chain
    const blocksRemaining = Math.max(0, unlockBlock - currentBlock);
    const secondsRemaining = blocksRemaining * 6;
    return Math.ceil(secondsRemaining / (24 * 60 * 60));
  }

  private async getActiveVotesCount(account: string): Promise<number> {
    try {
      const api = await this.ensureConnection();
      const votes = await api.query.governance.voterVotes(account);
      return votes.toJSON() ? (votes.toJSON() as any[]).length : 0;
    } catch {
      return 0;
    }
  }

  private async getActiveDelegationsCount(account: string): Promise<number> {
    try {
      const api = await this.ensureConnection();
      const delegations = (await api.query.governance.delegations(account)) as any;
      return delegations.isSome ? 1 : 0;
    } catch {
      return 0;
    }
  }

  private async getDelegatedVotingPower(account: string): Promise<string> {
    try {
      const api = await this.ensureConnection();
      const delegatedPower = await api.query.governance.receivedDelegations(account);
      return delegatedPower.toString();
    } catch {
      return '0';
    }
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // DELEGATION
  // ═══════════════════════════════════════════════════════════════════════════

  /**
   * Delegate voting power to another account
   */
  async delegate(params: DelegateParams, signer: any): Promise<TransactionResult> {
    try {
      const api = await this.ensureConnection();

      // Encode tracks if specified
      const tracks = params.tracks
        ? params.tracks.map(cat => this.encodeCategoryForChain(cat))
        : null;

      const tx = api.tx.governance.delegate(
        params.target,
        params.conviction,
        params.balance || null,
        tracks
      );

      return await this.signAndSend(tx, signer);
    } catch (error: any) {
      return {
        success: false,
        error: this.createError(
          GovernanceErrorCode.TransactionFailed,
          'Failed to delegate voting power',
          error
        ),
      };
    }
  }

  /**
   * Undelegate voting power
   */
  async undelegate(signer: any): Promise<TransactionResult> {
    try {
      const api = await this.ensureConnection();
      const tx = api.tx.governance.undelegate();

      return await this.signAndSend(tx, signer);
    } catch (error: any) {
      return {
        success: false,
        error: this.createError(
          GovernanceErrorCode.TransactionFailed,
          'Failed to undelegate voting power',
          error
        ),
      };
    }
  }

  /**
   * Get delegation for an account
   */
  async getDelegation(account: string): Promise<Delegation | null> {
    try {
      const api = await this.ensureConnection();
      const delegationData = (await api.query.governance.delegations(account)) as any;

      if (delegationData.isNone) {
        return null;
      }

      const delegation = delegationData.unwrap().toJSON() as any;

      return {
        delegator: account,
        target: delegation.target,
        conviction: delegation.conviction || ConvictionLevel.None,
        balance: delegation.balance || '0',
        votingPower: delegation.votingPower || '0',
        tracks: delegation.tracks || [],
        createdAt: delegation.createdAt || 0,
      };
    } catch (error) {
      console.error(`Failed to get delegation for ${account}:`, error);
      return null;
    }
  }

  /**
   * Get delegation statistics for an account
   */
  async getDelegationStats(account: string): Promise<DelegationStats> {
    try {
      const api = await this.ensureConnection();

      // Get outgoing delegation
      const outgoing = await this.getDelegation(account);

      // Get incoming delegations
      const incoming = await api.query.governance.receivedDelegations(account);
      const incomingData = incoming.toJSON() as any;

      return {
        totalDelegated: outgoing?.balance || '0',
        totalReceived: incomingData?.totalBalance || '0',
        activeDelegations: outgoing ? 1 : 0,
        receivedDelegations: incomingData?.count || 0,
        delegatedVotingPower: outgoing?.votingPower || '0',
        receivedVotingPower: incomingData?.totalVotingPower || '0',
      };
    } catch (error) {
      console.error(`Failed to get delegation stats for ${account}:`, error);

      return {
        totalDelegated: '0',
        totalReceived: '0',
        activeDelegations: 0,
        receivedDelegations: 0,
        delegatedVotingPower: '0',
        receivedVotingPower: '0',
      };
    }
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // GOVERNANCE STATISTICS
  // ═══════════════════════════════════════════════════════════════════════════

  /**
   * Get overall governance statistics
   * Fetches from democracy pallet and calculates based on real data
   */
  async getGovernanceStats(): Promise<GovernanceStats> {
    try {
      const api = await this.ensureConnection();

      // Get all proposals
      const proposals = await this.getProposals();

      const activeProposals = proposals.filter(p => p.status === ProposalStatus.Active);
      const passedProposals = proposals.filter(p => p.status === ProposalStatus.Passed);
      const rejectedProposals = proposals.filter(p => p.status === ProposalStatus.Rejected);
      const executedProposals = proposals.filter(p => p.status === ProposalStatus.Executed);

      // Calculate unique voters from all referendums
      const uniqueVotersSet = new Set<string>();
      let totalVotes = 0;

      if (api.query.democracy) {
        try {
          const refCount = (await api.query.democracy.referendumCount()) as any;
          const count = refCount.toNumber();

          // Sample last 20 referendums for voter stats
          const startIndex = Math.max(0, count - 20);
          for (let i = startIndex; i < count; i++) {
            try {
              const votingInfo = (await api.query.democracy.votingOf.entries()) as any;
              for (const [key, value] of votingInfo) {
                const address = key.args[0].toString();
                if (value.isDirect) {
                  const votes = value.asDirect.votes;
                  for (const [refIndex] of votes) {
                    if (refIndex.toNumber() === i) {
                      uniqueVotersSet.add(address);
                      totalVotes++;
                    }
                  }
                }
              }
            } catch (err) {
              // Skip if error querying specific referendum
            }
          }
        } catch (err) {
          console.error('Error calculating voter stats:', err);
        }
      }

      // Get treasury and supply
      let treasuryBalance = '0';
      try {
        if (api.query.treasury?.pot) {
          const pot = await api.query.treasury.pot();
          treasuryBalance = pot.toString();
        }
      } catch (err) {
        console.error('Error fetching treasury balance:', err);
      }

      let totalIssuance = '0';
      try {
        const issuance = await api.query.balances.totalIssuance();
        totalIssuance = issuance.toString();
      } catch (err) {
        console.error('Error fetching total issuance:', err);
      }

      return {
        totalProposals: proposals.length,
        activeProposals: activeProposals.length,
        passedProposals: passedProposals.length,
        rejectedProposals: rejectedProposals.length,
        executedProposals: executedProposals.length,
        totalVotes,
        uniqueVoters: uniqueVotersSet.size,
        totalVotingPower: '0',
        participationRate: 0,
        totalDelegations: 0,
        delegatedVotingPower: '0',
        treasuryBalance,
        totalDeposits: '0',
        circulatingSupply: totalIssuance,
        stakedSupply: '0',
        inflationRate: 0,
      };
    } catch (error) {
      console.error('Failed to get governance stats:', error);

      return {
        totalProposals: 0,
        activeProposals: 0,
        passedProposals: 0,
        rejectedProposals: 0,
        executedProposals: 0,
        totalVotes: 0,
        uniqueVoters: 0,
        totalVotingPower: '0',
        participationRate: 0,
        totalDelegations: 0,
        delegatedVotingPower: '0',
        treasuryBalance: '0',
        totalDeposits: '0',
        circulatingSupply: '0',
        stakedSupply: '0',
        inflationRate: 0,
      };
    }
  }

  /**
   * Get statistics for a specific category
   */
  async getCategoryStats(category: ProposalCategory): Promise<CategoryStats> {
    const proposals = await this.getProposalsByCategory(category);
    const activeProposals = proposals.filter(p => p.status === ProposalStatus.Active);
    const passedProposals = proposals.filter(p => p.status === ProposalStatus.Passed);

    const passRate = proposals.length > 0
      ? (passedProposals.length / proposals.length) * 100
      : 0;

    return {
      category,
      totalProposals: proposals.length,
      activeProposals: activeProposals.length,
      passRate,
      averageVotes: 0, // Calculate from vote records
      averageTurnout: 0,
    };
  }

  // ═══════════════════════════════════════════════════════════════════════════
  // TRANSACTION UTILITIES
  // ═══════════════════════════════════════════════════════════════════════════

  /**
   * Sign and send transaction
   */
  private async signAndSend(tx: any, signer: any): Promise<TransactionResult> {
    const api = await this.ensureConnection();

    return new Promise((resolve) => {
      tx.signAndSend(signer, ({ status, dispatchError, txHash }: any) => {
        if (dispatchError) {
          if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            resolve({
              success: false,
              error: this.createError(
                GovernanceErrorCode.TransactionFailed,
                `${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`,
                decoded
              ),
            });
          } else {
            resolve({
              success: false,
              error: this.createError(
                GovernanceErrorCode.TransactionFailed,
                dispatchError.toString()
              ),
            });
          }
        } else if (status.isInBlock) {
          resolve({
            success: true,
            txHash: txHash.toString(),
            blockHash: status.asInBlock.toString(),
          });
        } else if (status.isFinalized) {
          resolve({
            success: true,
            txHash: txHash.toString(),
            blockHash: status.asFinalized.toString(),
          });
        }
      }).catch((error: any) => {
        resolve({
          success: false,
          error: this.createError(
            GovernanceErrorCode.TransactionFailed,
            'Transaction failed',
            error
          ),
        });
      });
    });
  }

  /**
   * Disconnect from chain
   */
  async disconnect(): Promise<void> {
    if (this.api) {
      await this.api.disconnect();
      this.api = null;
      this.connectionPromise = null;
      console.log('❌ Disconnected from ETRID governance');
    }
  }
}

// ═══════════════════════════════════════════════════════════════════════════════
// SINGLETON INSTANCE
// ═══════════════════════════════════════════════════════════════════════════════

export const governanceService = new GovernanceService();

// ═══════════════════════════════════════════════════════════════════════════════
// CONVENIENCE EXPORTS
// ═══════════════════════════════════════════════════════════════════════════════

export default governanceService;
