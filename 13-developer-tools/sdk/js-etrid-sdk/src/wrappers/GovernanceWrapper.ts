/**
 * Type-safe wrapper for Governance pallet
 */

import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { Proposal, Vote, TransactionResult, PaginatedResult } from '../types/enhanced';
import { TransactionBuilder } from '../builders/TransactionBuilder';
import { NotConnectedError, GovernanceError } from '../errors/EtridErrors';

/**
 * Governance wrapper for governance operations
 */
export class GovernanceWrapper {
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
   * Get all active proposals
   */
  async getActiveProposals(): Promise<Proposal[]> {
    this.ensureConnected();

    try {
      // This is a simplified implementation
      // Actual implementation would query the governance pallet
      const proposals = await this.api.query.governance.proposals.entries();

      return proposals.map(([key, value]: [any, any]) => {
        const proposalId = key.args[0].toNumber();
        const proposal = value.unwrap();

        return {
          id: proposalId,
          proposer: proposal.proposer.toString(),
          title: proposal.title.toString(),
          description: proposal.description.toString(),
          votesFor: proposal.votesFor.toBigInt(),
          votesAgainst: proposal.votesAgainst.toBigInt(),
          status: proposal.status.toString().toLowerCase() as any,
          createdAt: proposal.createdAt.toNumber(),
          endsAt: proposal.endsAt.toNumber(),
        };
      });
    } catch (error) {
      throw new GovernanceError('Failed to get active proposals', { error });
    }
  }

  /**
   * Get proposal by ID
   */
  async getProposal(proposalId: number): Promise<Proposal | null> {
    this.ensureConnected();

    try {
      const proposal = await this.api.query.governance.proposals(proposalId);

      if (proposal.isNone) {
        return null;
      }

      const unwrapped = proposal.unwrap();

      return {
        id: proposalId,
        proposer: unwrapped.proposer.toString(),
        title: unwrapped.title.toString(),
        description: unwrapped.description.toString(),
        votesFor: unwrapped.votesFor.toBigInt(),
        votesAgainst: unwrapped.votesAgainst.toBigInt(),
        status: unwrapped.status.toString().toLowerCase() as any,
        createdAt: unwrapped.createdAt.toNumber(),
        endsAt: unwrapped.endsAt.toNumber(),
      };
    } catch (error) {
      throw new GovernanceError('Failed to get proposal', { proposalId, error });
    }
  }

  /**
   * Get vote for a proposal
   */
  async getVote(proposalId: number, voter: string): Promise<Vote | null> {
    this.ensureConnected();

    try {
      const vote = await this.api.query.governance.votes(proposalId, voter);

      if (vote.isNone) {
        return null;
      }

      const unwrapped = vote.unwrap();

      return {
        proposalId,
        voter,
        approve: unwrapped.approve.isTrue,
        weight: unwrapped.weight.toBigInt(),
        timestamp: unwrapped.timestamp.toNumber(),
      };
    } catch (error) {
      throw new GovernanceError('Failed to get vote', { proposalId, voter, error });
    }
  }

  /**
   * Get all votes for a proposal
   */
  async getProposalVotes(proposalId: number): Promise<Vote[]> {
    this.ensureConnected();

    try {
      const votes = await this.api.query.governance.votes.entries(proposalId);

      return votes.map(([key, value]: [any, any]) => {
        const voter = key.args[1].toString();
        const vote = value.unwrap();

        return {
          proposalId,
          voter,
          approve: vote.approve.isTrue,
          weight: vote.weight.toBigInt(),
          timestamp: vote.timestamp.toNumber(),
        };
      });
    } catch (error) {
      throw new GovernanceError('Failed to get proposal votes', { proposalId, error });
    }
  }

  /**
   * Get proposals with pagination
   */
  async getProposals(
    page: number = 0,
    limit: number = 10
  ): Promise<PaginatedResult<Proposal>> {
    this.ensureConnected();

    const allProposals = await this.getActiveProposals();
    const total = allProposals.length;
    const totalPages = Math.ceil(total / limit);

    const start = page * limit;
    const end = start + limit;
    const items = allProposals.slice(start, end);

    return {
      items,
      total,
      page,
      limit,
      totalPages,
      hasNext: page < totalPages - 1,
      hasPrevious: page > 0,
    };
  }

  /**
   * Create a new proposal
   */
  async createProposal(
    from: KeyringPair,
    title: string,
    description: string,
    call: any
  ): Promise<TransactionResult> {
    this.ensureConnected();

    if (!title || title.length === 0) {
      throw new GovernanceError('Proposal title cannot be empty');
    }

    if (!description || description.length === 0) {
      throw new GovernanceError('Proposal description cannot be empty');
    }

    return new TransactionBuilder(this.api)
      .propose(title, description, call)
      .submit(from);
  }

  /**
   * Vote on a proposal
   */
  async vote(
    from: KeyringPair,
    proposalId: number,
    approve: boolean,
    stake: bigint
  ): Promise<TransactionResult> {
    this.ensureConnected();

    // Check if proposal exists
    const proposal = await this.getProposal(proposalId);
    if (!proposal) {
      throw new GovernanceError('Proposal not found', { proposalId });
    }

    // Check if proposal is still active
    if (proposal.status !== 'active') {
      throw new GovernanceError('Proposal is not active', { proposalId, status: proposal.status });
    }

    return new TransactionBuilder(this.api)
      .vote(proposalId, approve, stake)
      .submit(from);
  }

  /**
   * Execute a passed proposal
   */
  async executeProposal(
    from: KeyringPair,
    proposalId: number
  ): Promise<TransactionResult> {
    this.ensureConnected();

    const proposal = await this.getProposal(proposalId);
    if (!proposal) {
      throw new GovernanceError('Proposal not found', { proposalId });
    }

    if (proposal.status !== 'passed') {
      throw new GovernanceError('Proposal has not passed', { proposalId, status: proposal.status });
    }

    const tx = this.api.tx.governance.execute(proposalId);
    const builder = new TransactionBuilder(this.api);
    (builder as any).extrinsic = tx;

    return builder.submit(from);
  }

  /**
   * Cancel a proposal (proposer only)
   */
  async cancelProposal(
    from: KeyringPair,
    proposalId: number
  ): Promise<TransactionResult> {
    this.ensureConnected();

    const proposal = await this.getProposal(proposalId);
    if (!proposal) {
      throw new GovernanceError('Proposal not found', { proposalId });
    }

    if (proposal.proposer !== from.address) {
      throw new GovernanceError('Only proposer can cancel proposal', { proposalId });
    }

    const tx = this.api.tx.governance.cancel(proposalId);
    const builder = new TransactionBuilder(this.api);
    (builder as any).extrinsic = tx;

    return builder.submit(from);
  }

  /**
   * Get voting period
   */
  async getVotingPeriod(): Promise<number> {
    this.ensureConnected();

    try {
      const votingPeriod = await this.api.consts.governance.votingPeriod;
      return votingPeriod.toNumber();
    } catch (error) {
      // Default to 7 days in blocks (assuming 6s blocks)
      return 100800; // 7 * 24 * 60 * 10
    }
  }

  /**
   * Get minimum proposal stake
   */
  async getMinimumProposalStake(): Promise<bigint> {
    this.ensureConnected();

    try {
      const minStake = await this.api.consts.governance.minimumProposalStake;
      return minStake.toBigInt();
    } catch (error) {
      // Default minimum
      return 1000000000000000000n; // 1 ETR
    }
  }

  /**
   * Get proposal count
   */
  async getProposalCount(): Promise<number> {
    this.ensureConnected();

    try {
      const count = await this.api.query.governance.proposalCount();
      return count.toNumber();
    } catch (error) {
      throw new GovernanceError('Failed to get proposal count', { error });
    }
  }

  /**
   * Get voter's voting power
   */
  async getVotingPower(address: string): Promise<bigint> {
    this.ensureConnected();

    try {
      // Voting power typically equals staked balance
      const balance = await this.api.query.system.account(address);
      return balance.data.free.toBigInt();
    } catch (error) {
      throw new GovernanceError('Failed to get voting power', { address, error });
    }
  }

  /**
   * Check if address has voted on proposal
   */
  async hasVoted(proposalId: number, address: string): Promise<boolean> {
    this.ensureConnected();

    const vote = await this.getVote(proposalId, address);
    return vote !== null;
  }

  /**
   * Get proposal results
   */
  async getProposalResults(proposalId: number): Promise<{
    votesFor: bigint;
    votesAgainst: bigint;
    totalVotes: bigint;
    participationRate: number;
    approved: boolean;
  }> {
    this.ensureConnected();

    const proposal = await this.getProposal(proposalId);
    if (!proposal) {
      throw new GovernanceError('Proposal not found', { proposalId });
    }

    const totalVotes = proposal.votesFor + proposal.votesAgainst;
    const totalIssuance = await this.api.query.balances.totalIssuance();
    const participationRate = totalVotes > 0n
      ? Number((totalVotes * 10000n) / totalIssuance.toBigInt()) / 100
      : 0;

    // Simple majority voting
    const approved = proposal.votesFor > proposal.votesAgainst;

    return {
      votesFor: proposal.votesFor,
      votesAgainst: proposal.votesAgainst,
      totalVotes,
      participationRate,
      approved,
    };
  }

  /**
   * Get proposal history with filters
   */
  async getProposalHistory(options?: {
    proposer?: string;
    status?: 'active' | 'passed' | 'rejected' | 'executed' | 'cancelled';
    fromBlock?: number;
    toBlock?: number;
    limit?: number;
  }): Promise<Proposal[]> {
    this.ensureConnected();

    try {
      const allProposals = await this.getActiveProposals();
      let filtered = allProposals;

      // Filter by proposer
      if (options?.proposer) {
        filtered = filtered.filter(p => p.proposer === options.proposer);
      }

      // Filter by status
      if (options?.status) {
        filtered = filtered.filter(p => p.status === options.status);
      }

      // Filter by time range
      if (options?.fromBlock) {
        filtered = filtered.filter(p => p.createdAt >= options.fromBlock!);
      }
      if (options?.toBlock) {
        filtered = filtered.filter(p => p.createdAt <= options.toBlock!);
      }

      // Apply limit
      if (options?.limit) {
        filtered = filtered.slice(0, options.limit);
      }

      return filtered;
    } catch (error) {
      throw new GovernanceError('Failed to get proposal history', { options, error });
    }
  }

  /**
   * Get voting delegations for an address
   */
  async getDelegations(address: string): Promise<Array<{
    delegate: string;
    weight: bigint;
    expiresAt?: number;
  }>> {
    this.ensureConnected();

    try {
      const delegations = await this.api.query.governance.delegations(address);

      if (delegations.isNone) {
        return [];
      }

      const unwrapped = delegations.unwrap();
      return unwrapped.map((delegation: any) => ({
        delegate: delegation.delegate.toString(),
        weight: delegation.weight.toBigInt(),
        expiresAt: delegation.expiresAt?.toNumber(),
      }));
    } catch (error) {
      throw new GovernanceError('Failed to get delegations', { address, error });
    }
  }

  /**
   * Get voting statistics for an address
   */
  async getVotingStatistics(address: string): Promise<{
    totalVotes: number;
    votesFor: number;
    votesAgainst: number;
    participationRate: number;
    lastVoteAt?: number;
    activeStreak: number;
  }> {
    this.ensureConnected();

    try {
      // Query all votes by this address
      const allVotes = await this.api.query.governance.voterHistory(address);

      if (allVotes.isNone) {
        return {
          totalVotes: 0,
          votesFor: 0,
          votesAgainst: 0,
          participationRate: 0,
          activeStreak: 0,
        };
      }

      const votes = allVotes.unwrap();
      const totalVotes = votes.length;
      const votesFor = votes.filter((v: any) => v.approve.isTrue).length;
      const votesAgainst = totalVotes - votesFor;

      // Calculate participation rate
      const totalProposals = await this.getProposalCount();
      const participationRate = totalProposals > 0
        ? (totalVotes / totalProposals) * 100
        : 0;

      // Get last vote timestamp
      const lastVote = votes[votes.length - 1];
      const lastVoteAt = lastVote?.timestamp.toNumber();

      // Calculate active streak (consecutive proposals voted on)
      let activeStreak = 0;
      for (let i = votes.length - 1; i >= 0; i--) {
        const expectedProposalId = await this.getProposalCount() - (votes.length - 1 - i);
        if (votes[i].proposalId.toNumber() === expectedProposalId) {
          activeStreak++;
        } else {
          break;
        }
      }

      return {
        totalVotes,
        votesFor,
        votesAgainst,
        participationRate,
        lastVoteAt,
        activeStreak,
      };
    } catch (error) {
      throw new GovernanceError('Failed to get voting statistics', { address, error });
    }
  }

  /**
   * Get proposal lifecycle timeline
   */
  async getProposalTimeline(proposalId: number): Promise<Array<{
    event: 'created' | 'voted' | 'passed' | 'rejected' | 'executed' | 'cancelled';
    timestamp: number;
    blockNumber: number;
    actor?: string;
    details?: any;
  }>> {
    this.ensureConnected();

    try {
      const proposal = await this.getProposal(proposalId);
      if (!proposal) {
        throw new GovernanceError('Proposal not found', { proposalId });
      }

      const timeline: Array<{
        event: 'created' | 'voted' | 'passed' | 'rejected' | 'executed' | 'cancelled';
        timestamp: number;
        blockNumber: number;
        actor?: string;
        details?: any;
      }> = [];

      // Add creation event
      timeline.push({
        event: 'created',
        timestamp: proposal.createdAt,
        blockNumber: proposal.createdAt,
        actor: proposal.proposer,
      });

      // Get all votes
      const votes = await this.getProposalVotes(proposalId);
      votes.forEach(vote => {
        timeline.push({
          event: 'voted',
          timestamp: vote.timestamp,
          blockNumber: vote.timestamp,
          actor: vote.voter,
          details: {
            approve: vote.approve,
            weight: vote.weight.toString(),
          },
        });
      });

      // Add status change events
      if (proposal.status === 'passed') {
        timeline.push({
          event: 'passed',
          timestamp: proposal.endsAt,
          blockNumber: proposal.endsAt,
        });
      } else if (proposal.status === 'rejected') {
        timeline.push({
          event: 'rejected',
          timestamp: proposal.endsAt,
          blockNumber: proposal.endsAt,
        });
      } else if (proposal.status === 'executed') {
        timeline.push({
          event: 'executed',
          timestamp: proposal.endsAt,
          blockNumber: proposal.endsAt,
        });
      } else if (proposal.status === 'cancelled') {
        timeline.push({
          event: 'cancelled',
          timestamp: proposal.endsAt,
          blockNumber: proposal.endsAt,
          actor: proposal.proposer,
        });
      }

      // Sort by timestamp
      timeline.sort((a, b) => a.timestamp - b.timestamp);

      return timeline;
    } catch (error) {
      throw new GovernanceError('Failed to get proposal timeline', { proposalId, error });
    }
  }

  /**
   * Estimate proposal outcome based on current votes
   */
  async estimateProposalOutcome(proposalId: number): Promise<{
    currentResult: 'passing' | 'failing' | 'tied';
    confidence: number;
    projectedVotesFor: bigint;
    projectedVotesAgainst: bigint;
    estimatedEndVotes: bigint;
    timeRemaining: number;
    requiredVotesToPass?: bigint;
    requiredVotesToFail?: bigint;
  }> {
    this.ensureConnected();

    try {
      const proposal = await this.getProposal(proposalId);
      if (!proposal) {
        throw new GovernanceError('Proposal not found', { proposalId });
      }

      const results = await this.getProposalResults(proposalId);
      const currentTime = Date.now();
      const timeRemaining = Math.max(0, proposal.endsAt - currentTime);

      // Determine current result
      let currentResult: 'passing' | 'failing' | 'tied';
      if (results.votesFor > results.votesAgainst) {
        currentResult = 'passing';
      } else if (results.votesFor < results.votesAgainst) {
        currentResult = 'failing';
      } else {
        currentResult = 'tied';
      }

      // Calculate confidence based on vote margin and time remaining
      const voteDifference = results.votesFor > results.votesAgainst
        ? results.votesFor - results.votesAgainst
        : results.votesAgainst - results.votesFor;

      const totalIssuance = await this.api.query.balances.totalIssuance();
      const voteMargin = Number((voteDifference * 10000n) / totalIssuance.toBigInt()) / 100;
      const timeProgress = (proposal.endsAt - proposal.createdAt - timeRemaining) /
                          (proposal.endsAt - proposal.createdAt);

      // Confidence increases with vote margin and time progress
      const confidence = Math.min(100, (voteMargin * 50) + (timeProgress * 50));

      // Project votes assuming linear growth
      const timePassed = (proposal.endsAt - proposal.createdAt) - timeRemaining;
      const voteRate = timePassed > 0 ? Number(results.totalVotes) / timePassed : 0;
      const estimatedAdditionalVotes = BigInt(Math.floor(voteRate * timeRemaining));

      const forRatio = results.totalVotes > 0n
        ? Number(results.votesFor) / Number(results.totalVotes)
        : 0.5;

      const projectedVotesFor = results.votesFor + BigInt(Math.floor(Number(estimatedAdditionalVotes) * forRatio));
      const projectedVotesAgainst = results.votesAgainst + (estimatedAdditionalVotes - BigInt(Math.floor(Number(estimatedAdditionalVotes) * forRatio)));

      // Calculate votes required to change outcome
      const requiredVotesToPass = currentResult === 'failing'
        ? (results.votesAgainst - results.votesFor) + 1n
        : undefined;

      const requiredVotesToFail = currentResult === 'passing'
        ? (results.votesFor - results.votesAgainst) + 1n
        : undefined;

      return {
        currentResult,
        confidence,
        projectedVotesFor,
        projectedVotesAgainst,
        estimatedEndVotes: projectedVotesFor + projectedVotesAgainst,
        timeRemaining,
        requiredVotesToPass,
        requiredVotesToFail,
      };
    } catch (error) {
      throw new GovernanceError('Failed to estimate proposal outcome', { proposalId, error });
    }
  }

  /**
   * Delegate voting power to another address
   */
  async delegateVotes(
    from: KeyringPair,
    delegate: string,
    weight: bigint,
    expiresAt?: number
  ): Promise<TransactionResult> {
    this.ensureConnected();

    const tx = this.api.tx.governance.delegate(delegate, weight, expiresAt);
    const builder = new TransactionBuilder(this.api);
    (builder as any).extrinsic = tx;

    return builder.submit(from);
  }

  /**
   * Undelegate voting power
   */
  async undelegateVotes(
    from: KeyringPair,
    delegate: string
  ): Promise<TransactionResult> {
    this.ensureConnected();

    const tx = this.api.tx.governance.undelegate(delegate);
    const builder = new TransactionBuilder(this.api);
    (builder as any).extrinsic = tx;

    return builder.submit(from);
  }

  /**
   * Get governance network statistics
   */
  async getGovernanceStats(): Promise<{
    totalProposals: number;
    activeProposals: number;
    passedProposals: number;
    rejectedProposals: number;
    executedProposals: number;
    averageParticipation: number;
    totalVoters: number;
  }> {
    this.ensureConnected();

    try {
      const allProposals = await this.getActiveProposals();
      const totalProposals = allProposals.length;

      const activeProposals = allProposals.filter(p => p.status === 'active').length;
      const passedProposals = allProposals.filter(p => p.status === 'passed').length;
      const rejectedProposals = allProposals.filter(p => p.status === 'rejected').length;
      const executedProposals = allProposals.filter(p => p.status === 'executed').length;

      // Calculate average participation
      let totalParticipation = 0;
      for (const proposal of allProposals) {
        const results = await this.getProposalResults(proposal.id);
        totalParticipation += results.participationRate;
      }
      const averageParticipation = totalProposals > 0
        ? totalParticipation / totalProposals
        : 0;

      // Get unique voters count
      const votersSet = new Set<string>();
      for (const proposal of allProposals) {
        const votes = await this.getProposalVotes(proposal.id);
        votes.forEach(vote => votersSet.add(vote.voter));
      }
      const totalVoters = votersSet.size;

      return {
        totalProposals,
        activeProposals,
        passedProposals,
        rejectedProposals,
        executedProposals,
        averageParticipation,
        totalVoters,
      };
    } catch (error) {
      throw new GovernanceError('Failed to get governance stats', { error });
    }
  }
}
