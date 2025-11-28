/**
 * DAO Proposal Service
 * Handles proposal creation, voting, and execution
 */

import {
  Proposal,
  ProposalInput,
  ProposalFilter,
  Vote,
  VoteType,
  VoteBreakdown,
  DAOMember,
} from '@/types/dao';

export class DAOProposalService {
  private proposals: Map<string, Proposal> = new Map();
  private votes: Map<string, Vote[]> = new Map(); // proposalId -> votes

  /**
   * Create a new proposal
   */
  async createProposal(daoId: string, proposal: ProposalInput): Promise<Proposal> {
    // Validate proposal
    if (!proposal.title || !proposal.description) {
      throw new Error('Title and description are required');
    }

    // Get current user as proposer
    const proposer = await this.getCurrentUserAsMember(daoId);

    // Check if user has permission to propose
    // TODO: Check governance.proposalThreshold
    const canPropose = await this.checkCanPropose(daoId, proposer);
    if (!canPropose) {
      throw new Error('Insufficient voting power to create proposal');
    }

    // Get governance settings
    const governance = await this.getDAOGovernance(daoId);

    const votingStartsAt = new Date();
    const votingEndsAt = new Date(
      votingStartsAt.getTime() + governance.votingPeriod * 24 * 60 * 60 * 1000
    );

    const newProposal: Proposal = {
      id: Date.now().toString(),
      daoId,
      proposer,
      title: proposal.title,
      description: proposal.description,
      type: proposal.type,
      status: 'active',
      votesFor: 0,
      votesAgainst: 0,
      votesAbstain: 0,
      totalVotes: 0,
      quorumReached: false,
      createdAt: new Date(),
      votingStartsAt,
      votingEndsAt,
      executionData: proposal.executionData,
    };

    this.proposals.set(newProposal.id, newProposal);
    await this.saveProposals();

    // TODO: Create proposal on-chain
    // await this.createProposalOnChain(newProposal);

    return newProposal;
  }

  /**
   * Get proposals for a DAO with optional filter
   */
  async getProposals(daoId: string, filter?: ProposalFilter): Promise<Proposal[]> {
    let proposals = Array.from(this.proposals.values()).filter(
      (p) => p.daoId === daoId
    );

    // Update proposal statuses based on time
    proposals = await Promise.all(
      proposals.map((p) => this.updateProposalStatus(p))
    );

    // Apply filter
    if (filter && filter !== 'all') {
      proposals = proposals.filter((p) => p.status === filter);
    }

    // Sort by creation date (newest first)
    proposals.sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime());

    return proposals;
  }

  /**
   * Get single proposal details
   */
  async getProposal(proposalId: string): Promise<Proposal> {
    const proposal = this.proposals.get(proposalId);
    if (!proposal) {
      throw new Error('Proposal not found');
    }

    return this.updateProposalStatus(proposal);
  }

  /**
   * Vote on a proposal
   */
  async vote(
    proposalId: string,
    voteType: VoteType,
    reason?: string
  ): Promise<void> {
    const proposal = await this.getProposal(proposalId);

    // Check if proposal is active
    if (proposal.status !== 'active') {
      throw new Error('Proposal is not active');
    }

    // Check if voting period has ended
    if (new Date() > proposal.votingEndsAt) {
      throw new Error('Voting period has ended');
    }

    // Get current user as voter
    const voter = await this.getCurrentUserAsMember(proposal.daoId);

    // Check if user has already voted
    const existingVote = await this.getUserVote(proposalId, voter.userId);
    if (existingVote) {
      throw new Error('Already voted on this proposal');
    }

    // Get voting power
    const votingPower = await this.getVotingPower(proposal.daoId, voter);

    // Create vote
    const vote: Vote = {
      id: Date.now().toString(),
      proposalId,
      voterId: voter.userId,
      voter,
      vote: voteType,
      weight: votingPower,
      votedAt: new Date(),
      reason,
    };

    // Store vote
    const proposalVotes = this.votes.get(proposalId) || [];
    proposalVotes.push(vote);
    this.votes.set(proposalId, proposalVotes);

    // Update proposal vote counts
    switch (voteType) {
      case 'for':
        proposal.votesFor += votingPower;
        break;
      case 'against':
        proposal.votesAgainst += votingPower;
        break;
      case 'abstain':
        proposal.votesAbstain += votingPower;
        break;
    }

    proposal.totalVotes += votingPower;

    // Check quorum
    const governance = await this.getDAOGovernance(proposal.daoId);
    const totalVotingPower = await this.getTotalVotingPower(proposal.daoId);
    const participationRate = (proposal.totalVotes / totalVotingPower) * 100;
    proposal.quorumReached = participationRate >= governance.quorum;

    this.proposals.set(proposalId, proposal);

    await this.saveProposals();
    await this.saveVotes();

    // TODO: Submit vote on-chain
    // await this.voteOnChain(proposalId, voteType, votingPower);
  }

  /**
   * Execute a passed proposal
   */
  async executeProposal(proposalId: string): Promise<void> {
    const proposal = await this.getProposal(proposalId);

    // Check if proposal has passed
    if (proposal.status !== 'passed') {
      throw new Error('Proposal has not passed');
    }

    // Check execution delay
    const governance = await this.getDAOGovernance(proposal.daoId);
    const canExecuteAt = new Date(
      proposal.votingEndsAt.getTime() +
        governance.executionDelay * 24 * 60 * 60 * 1000
    );

    if (new Date() < canExecuteAt) {
      throw new Error('Execution delay has not passed');
    }

    // Execute proposal based on type
    switch (proposal.type) {
      case 'treasury':
        await this.executeTreasuryProposal(proposal);
        break;
      case 'governance':
        await this.executeGovernanceProposal(proposal);
        break;
      case 'membership':
        await this.executeMembershipProposal(proposal);
        break;
      case 'custom':
        await this.executeCustomProposal(proposal);
        break;
    }

    // Update proposal status
    proposal.status = 'executed';
    proposal.executedAt = new Date();
    this.proposals.set(proposalId, proposal);

    await this.saveProposals();
  }

  /**
   * Cancel a proposal (only by proposer)
   */
  async cancelProposal(proposalId: string): Promise<void> {
    const proposal = await this.getProposal(proposalId);
    const currentUserId = await this.getCurrentUserId();

    // Check if user is the proposer
    if (proposal.proposer.userId !== currentUserId) {
      throw new Error('Only proposer can cancel proposal');
    }

    // Check if proposal is still active
    if (proposal.status !== 'active' && proposal.status !== 'pending') {
      throw new Error('Cannot cancel proposal in current status');
    }

    proposal.status = 'cancelled';
    this.proposals.set(proposalId, proposal);

    await this.saveProposals();

    // TODO: Cancel proposal on-chain
    // await this.cancelProposalOnChain(proposalId);
  }

  /**
   * Get vote breakdown for a proposal
   */
  async getVoteBreakdown(proposalId: string): Promise<VoteBreakdown> {
    const proposal = await this.getProposal(proposalId);
    const votes = this.votes.get(proposalId) || [];

    const forVotes = votes.filter((v) => v.vote === 'for');
    const againstVotes = votes.filter((v) => v.vote === 'against');
    const abstainVotes = votes.filter((v) => v.vote === 'abstain');

    const total = proposal.totalVotes || 1; // Avoid division by zero

    const governance = await this.getDAOGovernance(proposal.daoId);

    return {
      for: {
        count: proposal.votesFor,
        percentage: (proposal.votesFor / total) * 100,
        voters: forVotes.map((v) => v.voter),
      },
      against: {
        count: proposal.votesAgainst,
        percentage: (proposal.votesAgainst / total) * 100,
        voters: againstVotes.map((v) => v.voter),
      },
      abstain: {
        count: proposal.votesAbstain,
        percentage: (proposal.votesAbstain / total) * 100,
        voters: abstainVotes.map((v) => v.voter),
      },
      quorumProgress: proposal.quorumReached ? 100 : (total / governance.quorum) * 100,
      quorumRequired: governance.quorum,
    };
  }

  /**
   * Get user's vote on a proposal
   */
  async getUserVote(proposalId: string, userId: string): Promise<Vote | null> {
    const votes = this.votes.get(proposalId) || [];
    return votes.find((v) => v.voterId === userId) || null;
  }

  /**
   * Update proposal status based on time and votes
   */
  private async updateProposalStatus(proposal: Proposal): Promise<Proposal> {
    if (proposal.status === 'executed' || proposal.status === 'cancelled') {
      return proposal;
    }

    const now = new Date();

    // Check if voting period has ended
    if (now > proposal.votingEndsAt && proposal.status === 'active') {
      // Determine if passed or rejected
      if (proposal.quorumReached && proposal.votesFor > proposal.votesAgainst) {
        proposal.status = 'passed';
      } else {
        proposal.status = 'rejected';
      }
      this.proposals.set(proposal.id, proposal);
      await this.saveProposals();
    }

    return proposal;
  }

  /**
   * Execute treasury proposal
   */
  private async executeTreasuryProposal(proposal: Proposal): Promise<void> {
    // TODO: Execute treasury action (transfer funds, etc.)
    console.log('Executing treasury proposal:', proposal.id);
  }

  /**
   * Execute governance proposal
   */
  private async executeGovernanceProposal(proposal: Proposal): Promise<void> {
    // TODO: Update DAO governance settings
    console.log('Executing governance proposal:', proposal.id);
  }

  /**
   * Execute membership proposal
   */
  private async executeMembershipProposal(proposal: Proposal): Promise<void> {
    // TODO: Add/remove members
    console.log('Executing membership proposal:', proposal.id);
  }

  /**
   * Execute custom proposal
   */
  private async executeCustomProposal(proposal: Proposal): Promise<void> {
    // TODO: Execute custom on-chain action
    console.log('Executing custom proposal:', proposal.id);
  }

  // Helper methods
  private async getCurrentUserId(): Promise<string> {
    // TODO: Get from auth service
    return 'user-1';
  }

  private async getCurrentUserAsMember(daoId: string): Promise<DAOMember> {
    // TODO: Get from DAO service
    return {
      id: '1',
      userId: 'user-1',
      address: '0x1234...',
      username: 'Current User',
      role: 'member',
      votingPower: 100,
      joinedAt: new Date(),
      proposalsCreated: 0,
      votesCast: 0,
    };
  }

  private async checkCanPropose(daoId: string, member: DAOMember): Promise<boolean> {
    const governance = await this.getDAOGovernance(daoId);
    return member.votingPower >= governance.proposalThreshold;
  }

  private async getDAOGovernance(daoId: string): Promise<any> {
    // TODO: Get from DAO service
    return {
      votingPeriod: 7,
      quorum: 20,
      proposalThreshold: 10,
      executionDelay: 2,
      votingStrategy: 'token-weighted',
    };
  }

  private async getVotingPower(daoId: string, member: DAOMember): Promise<number> {
    // TODO: Calculate based on governance strategy
    return member.votingPower;
  }

  private async getTotalVotingPower(daoId: string): Promise<number> {
    // TODO: Calculate total voting power for quorum
    return 1000;
  }

  // Storage methods
  private async saveProposals(): Promise<void> {
    const proposalsArray = Array.from(this.proposals.values());
    // TODO: Persist to AsyncStorage or API
    // await AsyncStorage.setItem('dao_proposals', JSON.stringify(proposalsArray));
  }

  private async saveVotes(): Promise<void> {
    const votesMap: any = {};
    this.votes.forEach((votes, proposalId) => {
      votesMap[proposalId] = votes;
    });
    // TODO: Persist to AsyncStorage or API
    // await AsyncStorage.setItem('dao_votes', JSON.stringify(votesMap));
  }
}

export const daoProposalService = new DAOProposalService();
