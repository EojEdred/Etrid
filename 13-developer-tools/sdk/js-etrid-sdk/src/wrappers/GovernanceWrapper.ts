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
}
