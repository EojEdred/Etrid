/**
 * ETRID Governance Service
 *
 * Complete governance functionality for the ETRID web wallet
 * Connects to the real chain at wss://rpc.etrid.org
 */

import { ApiPromise } from '@polkadot/api';
import { primearcCoreChainApi } from '../api/primearc-core-chain';
import type {
  Proposal,
  ProposalCategory,
  ProposalStatus,
  ProposalFilters,
  CreateProposalParams,
  CastVoteParams,
  VoteType,
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
  GovernanceErrorCode,
  ConvictionLevel,
  PaginatedResponse,
  PaginationParams,
} from './types';
import { getConvictionConfig, calculateVotingPower } from './types';

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
   */
  async getProposals(filters?: ProposalFilters): Promise<Proposal[]> {
    try {
      const api = await this.ensureConnection();

      // Query all proposals from storage
      const proposalCount = await api.query.governance.proposalCount();
      const count = proposalCount.toNumber();

      const proposals: Proposal[] = [];

      for (let id = 0; id < count; id++) {
        const proposal = await this.getProposal(id);
        if (proposal) {
          // Apply filters
          if (filters?.categories && !filters.categories.includes(proposal.category)) {
            continue;
          }
          if (filters?.statuses && !filters.statuses.includes(proposal.status)) {
            continue;
          }
          if (filters?.proposer && proposal.proposer !== filters.proposer) {
            continue;
          }

          proposals.push(proposal);
        }
      }

      return proposals;
    } catch (error) {
      console.error('Failed to fetch proposals:', error);
      return [];
    }
  }

  /**
   * Get a specific proposal by ID
   */
  async getProposal(proposalId: number): Promise<Proposal | null> {
    try {
      const api = await this.ensureConnection();

      // Query proposal from storage
      const proposalData = await api.query.governance.proposals(proposalId);

      if (proposalData.isNone) {
        return null;
      }

      const proposal = proposalData.unwrap();
      const proposalJson = proposal.toJSON() as any;

      // Query vote tallies
      const voteTallies = await api.query.governance.voteTallies(proposalId);
      const talliesJson = voteTallies.toJSON() as any;

      return {
        id: proposalId,
        proposer: proposalJson.proposer,
        title: proposalJson.title || `Proposal #${proposalId}`,
        description: proposalJson.description || '',
        category: this.parseCategory(proposalJson.category),
        status: this.parseStatus(proposalJson.status),
        createdAt: proposalJson.createdAt || 0,
        votingDeadline: proposalJson.votingDeadline || 0,
        executionBlock: proposalJson.executionBlock,
        votesFor: talliesJson?.votesFor || '0',
        votesAgainst: talliesJson?.votesAgainst || '0',
        votesAbstain: talliesJson?.votesAbstain || '0',
        totalVotingPower: talliesJson?.totalVotingPower || '0',
        approved: proposalJson.approved || false,
        executed: proposalJson.executed || false,
        executedAt: proposalJson.executedAt,
        metadata: proposalJson.metadata,
      };
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
   */
  async castVote(params: CastVoteParams, signer: any): Promise<TransactionResult> {
    try {
      const api = await this.ensureConnection();

      // Encode vote type
      const voteEncoded = this.encodeVoteType(params.voteType);

      // Create vote extrinsic with conviction
      const tx = api.tx.governance.vote(
        params.proposalId,
        voteEncoded,
        params.conviction,
        params.balance || null
      );

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
   * Get votes for a proposal
   */
  async getVotes(proposalId: number): Promise<VoteRecord[]> {
    try {
      const api = await this.ensureConnection();

      // Query all votes for proposal
      const votes = await api.query.governance.votes.entries(proposalId);

      return votes.map(([key, value]) => {
        const voter = key.args[1].toString();
        const voteData = value.toJSON() as any;

        return {
          voter,
          proposalId,
          voteType: this.parseVoteType(voteData.voteType),
          conviction: voteData.conviction || ConvictionLevel.None,
          balance: voteData.balance || '0',
          votingPower: voteData.votingPower || '0',
          timestamp: voteData.timestamp || 0,
        };
      });
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
   * Check if an account has voted on a proposal
   */
  async hasVoted(proposalId: number, account: string): Promise<boolean> {
    try {
      const api = await this.ensureConnection();
      const vote = await api.query.governance.votes(proposalId, account);
      return vote.isSome;
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
      const accountInfo = await api.query.system.account(account);
      const balanceData = accountInfo.data;

      // Get staking info
      const stakingLedger = await api.query.staking.ledger(account);
      const stakedBalance = stakingLedger.isSome
        ? stakingLedger.unwrap().active.toString()
        : '0';

      // Get lock information
      const locks = await api.query.balances.locks(account);
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
      const delegations = await api.query.governance.delegations(account);
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
      const delegationData = await api.query.governance.delegations(account);

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
   */
  async getGovernanceStats(): Promise<GovernanceStats> {
    try {
      const api = await this.ensureConnection();

      // Get proposal counts
      const proposalCount = await api.query.governance.proposalCount();
      const proposals = await this.getProposals();

      const activeProposals = proposals.filter(p => p.status === ProposalStatus.Active);
      const passedProposals = proposals.filter(p => p.status === ProposalStatus.Passed);
      const rejectedProposals = proposals.filter(p => p.status === ProposalStatus.Rejected);
      const executedProposals = proposals.filter(p => p.status === ProposalStatus.Executed);

      // Get voting stats
      const totalVotesData = await api.query.governance.totalVotes();
      const uniqueVotersData = await api.query.governance.uniqueVoters();
      const totalVotingPowerData = await api.query.governance.totalVotingPower();

      // Get treasury and supply
      const treasuryBalance = await api.query.treasury.balance?.();
      const totalIssuance = await api.query.balances.totalIssuance();

      return {
        totalProposals: proposalCount.toNumber(),
        activeProposals: activeProposals.length,
        passedProposals: passedProposals.length,
        rejectedProposals: rejectedProposals.length,
        executedProposals: executedProposals.length,
        totalVotes: totalVotesData?.toNumber() || 0,
        uniqueVoters: uniqueVotersData?.toNumber() || 0,
        totalVotingPower: totalVotingPowerData?.toString() || '0',
        participationRate: 0, // Calculate based on eligible voters
        totalDelegations: 0,
        delegatedVotingPower: '0',
        treasuryBalance: treasuryBalance?.toString() || '0',
        totalDeposits: '0',
        circulatingSupply: totalIssuance.toString(),
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
