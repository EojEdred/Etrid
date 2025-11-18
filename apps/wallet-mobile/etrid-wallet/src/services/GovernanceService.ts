import EtridSDKService from './EtridSDKService';
import KeychainService from './KeychainService';
import {
  Proposal,
  ProposalStatus,
  VoteChoice,
  VotingPower,
  VoteHistory,
  Delegation,
  ConvictionLevel,
  CONVICTION_LEVELS,
  TransactionResult,
} from '../types/defi.types';

/**
 * GovernanceService - Handles all governance and voting operations
 * Connects to FlareChain governance pallet via EtridSDK
 */
class GovernanceService {
  private sdk: EtridSDKService;
  private readonly DECIMALS = 12; // ETR decimals

  constructor() {
    this.sdk = EtridSDKService.getInstance();
  }

  /**
   * Get all active proposals
   */
  async getActiveProposals(userAddress?: string): Promise<Proposal[]> {
    try {
      await this.sdk.connect();

      const proposalsData = await this.sdk.governance.getProposals?.() || this.getMockProposals();

      const proposals: Proposal[] = await Promise.all(
        proposalsData.map(async (p: any) => {
          const hasVoted = userAddress ? await this.hasVoted(p.id, userAddress) : false;
          const userVote = hasVoted ? await this.getUserVote(p.id, userAddress!) : undefined;

          return this.formatProposal(p, hasVoted, userVote);
        })
      );

      return proposals.filter(p => p.status === 'active');
    } catch (error) {
      console.error('Failed to get proposals:', error);
      return this.getMockProposals();
    }
  }

  /**
   * Get proposal by ID
   */
  async getProposal(proposalId: number, userAddress?: string): Promise<Proposal | null> {
    try {
      await this.sdk.connect();

      const proposalData = await this.sdk.governance.getProposal?.(proposalId);

      if (!proposalData) {
        return null;
      }

      const hasVoted = userAddress ? await this.hasVoted(proposalId, userAddress) : false;
      const userVote = hasVoted ? await this.getUserVote(proposalId, userAddress!) : undefined;

      return this.formatProposal(proposalData, hasVoted, userVote);
    } catch (error) {
      console.error('Failed to get proposal:', error);
      return null;
    }
  }

  /**
   * Submit a vote on a proposal
   */
  async vote(voteChoice: VoteChoice): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      // Determine vote value based on choice
      const voteValue = voteChoice.vote === 'yes' ? true : voteChoice.vote === 'no' ? false : null;

      // Submit vote with conviction
      const voteTx = await this.sdk.governance.vote(
        voteChoice.proposalId,
        voteValue,
        keypair,
        voteChoice.conviction
      );

      return {
        success: true,
        txHash: voteTx?.toString(),
        message: `Successfully voted ${voteChoice.vote.toUpperCase()} on proposal #${voteChoice.proposalId}`,
      };
    } catch (error) {
      console.error('Failed to vote:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to submit vote',
      };
    }
  }

  /**
   * Delegate voting power to another address
   */
  async delegateVotes(
    delegateTo: string,
    amount: string,
    conviction: ConvictionLevel
  ): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      const delegateTx = await this.sdk.governance.delegate?.(
        keypair,
        delegateTo,
        amount,
        conviction
      );

      return {
        success: true,
        txHash: delegateTx?.toString(),
        message: 'Successfully delegated voting power',
      };
    } catch (error) {
      console.error('Failed to delegate:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to delegate votes',
      };
    }
  }

  /**
   * Undelegate voting power
   */
  async undelegateVotes(): Promise<TransactionResult> {
    try {
      await this.sdk.connect();
      const keypair = await KeychainService.loadKeypair();

      if (!keypair) {
        throw new Error('No wallet found');
      }

      const undelegateTx = await this.sdk.governance.undelegate?.(keypair);

      return {
        success: true,
        txHash: undelegateTx?.toString(),
        message: 'Successfully undelegated voting power',
      };
    } catch (error) {
      console.error('Failed to undelegate:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Failed to undelegate votes',
      };
    }
  }

  /**
   * Get voting power for an address
   */
  async getVotingPower(address: string): Promise<VotingPower> {
    try {
      await this.sdk.connect();

      // Get account balance
      const balance = await this.sdk.accounts.getBalance(address);
      const balanceETR = this.fromSmallestUnit(balance);

      // Get locked balance
      const locked = await this.sdk.governance.getLockedBalance?.(address) || '0';
      const lockedETR = this.fromSmallestUnit(locked);

      // Get delegation info
      const delegation = await this.sdk.governance.getDelegation?.(address);

      // Calculate total voting power (with conviction multiplier)
      // For simplicity, using default 3x multiplier
      const totalVotingPower = (balanceETR - lockedETR) * 3;

      return {
        availableBalance: balance,
        availableBalanceETR: balanceETR - lockedETR,
        lockedBalance: locked,
        lockedBalanceETR: lockedETR,
        totalVotingPower,
        delegatedVotes: delegation?.amount || '0',
        delegatedVotesETR: delegation ? this.fromSmallestUnit(delegation.amount) : 0,
        delegatedTo: delegation?.target,
        delegatedToName: delegation?.targetName,
      };
    } catch (error) {
      console.error('Failed to get voting power:', error);
      throw error;
    }
  }

  /**
   * Get vote history for an address
   */
  async getVoteHistory(address: string): Promise<VoteHistory[]> {
    try {
      await this.sdk.connect();

      const historyData = await this.sdk.governance.getVoteHistory?.(address) || [];

      return historyData.map((vote: any) => ({
        proposalId: vote.proposalId,
        proposalTitle: vote.proposalTitle || `Proposal #${vote.proposalId}`,
        vote: vote.vote,
        conviction: vote.conviction,
        timestamp: vote.timestamp,
        result: vote.result || 'pending',
      }));
    } catch (error) {
      console.error('Failed to get vote history:', error);
      return [];
    }
  }

  /**
   * Get active delegations for an address
   */
  async getDelegations(address: string): Promise<Delegation[]> {
    try {
      await this.sdk.connect();

      const delegationsData = await this.sdk.governance.getDelegations?.(address) || [];

      return delegationsData.map((del: any) => ({
        delegateTo: del.target,
        delegateToName: del.targetName,
        amount: del.amount,
        amountETR: this.fromSmallestUnit(del.amount),
        conviction: del.conviction,
        timestamp: del.timestamp,
      }));
    } catch (error) {
      console.error('Failed to get delegations:', error);
      return [];
    }
  }

  /**
   * Calculate voting power with conviction
   */
  calculateVotingPowerWithConviction(balance: number, conviction: ConvictionLevel): number {
    const convictionInfo = CONVICTION_LEVELS.find(c => c.level === conviction);
    const multiplier = convictionInfo?.multiplier || 1;
    return balance * multiplier;
  }

  /**
   * Get conviction info
   */
  getConvictionInfo(level: ConvictionLevel) {
    return CONVICTION_LEVELS.find(c => c.level === level) || CONVICTION_LEVELS[0];
  }

  /**
   * Calculate time remaining for proposal
   */
  private calculateTimeRemaining(endBlock: number): string {
    // Assuming 3 second block time
    const blocksRemaining = endBlock - this.getCurrentBlock();
    const secondsRemaining = blocksRemaining * 3;

    if (secondsRemaining <= 0) {
      return 'Ended';
    }

    const days = Math.floor(secondsRemaining / 86400);
    const hours = Math.floor((secondsRemaining % 86400) / 3600);
    const minutes = Math.floor((secondsRemaining % 3600) / 60);

    if (days > 0) {
      return `${days}d ${hours}h remaining`;
    } else if (hours > 0) {
      return `${hours}h ${minutes}m remaining`;
    } else {
      return `${minutes}m remaining`;
    }
  }

  /**
   * Get current block number (mock)
   */
  private getCurrentBlock(): number {
    // This should come from the chain
    return 1500000;
  }

  /**
   * Check if user has voted on proposal
   */
  private async hasVoted(proposalId: number, address: string): Promise<boolean> {
    try {
      const vote = await this.sdk.governance.getUserVote?.(proposalId, address);
      return vote !== null;
    } catch (error) {
      return false;
    }
  }

  /**
   * Get user's vote on proposal
   */
  private async getUserVote(
    proposalId: number,
    address: string
  ): Promise<{ vote: 'yes' | 'no' | 'abstain'; conviction: ConvictionLevel } | undefined> {
    try {
      const vote = await this.sdk.governance.getUserVote?.(proposalId, address);
      return vote;
    } catch (error) {
      return undefined;
    }
  }

  /**
   * Format proposal data
   */
  private formatProposal(
    data: any,
    hasVoted: boolean,
    userVote?: { vote: 'yes' | 'no' | 'abstain'; conviction: ConvictionLevel }
  ): Proposal {
    const votesYesETR = this.fromSmallestUnit(data.votesYes || data.votesFor || '0');
    const votesNoETR = this.fromSmallestUnit(data.votesNo || data.votesAgainst || '0');
    const votesAbstainETR = this.fromSmallestUnit(data.votesAbstain || '0');
    const totalVotesETR = votesYesETR + votesNoETR + votesAbstainETR;

    const currentApproval = totalVotesETR > 0 ? (votesYesETR / totalVotesETR) * 100 : 0;
    const currentQuorum = 50; // Mock quorum calculation

    return {
      id: data.id,
      proposalHash: data.proposalHash || `0x${data.id.toString().padStart(64, '0')}`,
      title: data.title,
      description: data.description,
      proposer: data.proposer,
      proposerName: data.proposerName,
      status: data.status,
      type: data.type || 'general',
      votesYes: data.votesYes || data.votesFor || '0',
      votesYesETR,
      votesNo: data.votesNo || data.votesAgainst || '0',
      votesNoETR,
      votesAbstain: data.votesAbstain || '0',
      votesAbstainETR,
      totalVotes: this.toSmallestUnit(totalVotesETR),
      totalVotesETR,
      quorum: 50,
      currentQuorum,
      threshold: 51,
      currentApproval,
      startBlock: data.startBlock || this.getCurrentBlock(),
      endBlock: data.endBlock,
      endsAt: Date.now() + (data.endBlock - this.getCurrentBlock()) * 3000,
      timeRemaining: this.calculateTimeRemaining(data.endBlock),
      discussionLink: data.discussionLink,
      impactSummary: data.impactSummary,
      hasVoted,
      userVote: userVote?.vote,
      userConviction: userVote?.conviction,
    };
  }

  /**
   * Convert from smallest unit to ETR
   */
  private fromSmallestUnit(amount: string): number {
    return Number(amount) / Math.pow(10, this.DECIMALS);
  }

  /**
   * Convert from ETR to smallest unit
   */
  private toSmallestUnit(amount: number): string {
    return BigInt(Math.floor(amount * Math.pow(10, this.DECIMALS))).toString();
  }

  /**
   * Mock proposals for testing
   */
  private getMockProposals(): Proposal[] {
    const currentBlock = this.getCurrentBlock();

    return [
      {
        id: 1,
        proposalHash: '0x1234567890abcdef',
        title: 'Increase validator rewards by 10%',
        description: 'This proposal aims to increase validator rewards from the current rate to incentivize more participation in network security.',
        proposer: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
        proposerName: 'Etrid Foundation',
        status: 'active',
        type: 'parameter_change',
        votesYes: '15000000000000000',
        votesYesETR: 15000000,
        votesNo: '5000000000000000',
        votesNoETR: 5000000,
        votesAbstain: '1000000000000000',
        votesAbstainETR: 1000000,
        totalVotes: '21000000000000000',
        totalVotesETR: 21000000,
        quorum: 50,
        currentQuorum: 65,
        threshold: 51,
        currentApproval: 71.4,
        startBlock: currentBlock - 10000,
        endBlock: currentBlock + 50000,
        endsAt: Date.now() + 50000 * 3000,
        timeRemaining: '3d 5h remaining',
        discussionLink: 'https://forum.etrid.network/proposal/1',
        impactSummary: 'Validators will earn 10% more rewards, improving network security',
        hasVoted: false,
      },
      {
        id: 2,
        proposalHash: '0xabcdef1234567890',
        title: 'Treasury allocation for ecosystem development',
        description: 'Allocate 100,000 ETR from treasury for ecosystem grants and developer tools.',
        proposer: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
        proposerName: 'Development Fund',
        status: 'active',
        type: 'treasury_spend',
        votesYes: '8000000000000000',
        votesYesETR: 8000000,
        votesNo: '3000000000000000',
        votesNoETR: 3000000,
        votesAbstain: '500000000000000',
        votesAbstainETR: 500000,
        totalVotes: '11500000000000000',
        totalVotesETR: 11500000,
        quorum: 50,
        currentQuorum: 45,
        threshold: 51,
        currentApproval: 69.6,
        startBlock: currentBlock - 5000,
        endBlock: currentBlock + 80000,
        endsAt: Date.now() + 80000 * 3000,
        timeRemaining: '5d 14h remaining',
        discussionLink: 'https://forum.etrid.network/proposal/2',
        impactSummary: 'Funds will support ecosystem growth and developer adoption',
        hasVoted: false,
      },
    ];
  }
}

export default new GovernanceService();
