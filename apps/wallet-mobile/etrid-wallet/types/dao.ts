/**
 * DAO Type Definitions
 */

export interface DAO {
  id: string;
  name: string;
  description: string;
  logoUrl?: string;
  governance: DAOGovernance;
  treasuryAddress: string;
  memberCount: number;
  activeProposalCount: number;
  treasuryValue: string; // USD value
  createdAt: Date;
  userRole?: 'owner' | 'member' | 'voter';
}

export interface DAODetail extends DAO {
  members: DAOMember[];
  proposals: Proposal[];
  treasury: Treasury;
  stats: DAOStats;
}

export interface DAOGovernance {
  votingPeriod: number; // days
  quorum: number; // percentage (e.g., 20 = 20%)
  proposalThreshold: number; // min tokens to propose
  executionDelay: number; // days after passing
  votingStrategy: 'token-weighted' | 'one-person-one-vote' | 'quadratic';
  membershipType: 'open' | 'token-gated' | 'nft-gated' | 'invite-only';
  tokenAddress?: string; // for token-gated
  nftAddress?: string; // for nft-gated
}

export interface DAOMember {
  id: string;
  userId: string;
  address: string;
  username?: string;
  avatarUrl?: string;
  role: 'owner' | 'member' | 'voter';
  votingPower: number;
  joinedAt: Date;
  proposalsCreated: number;
  votesCast: number;
}

export interface Proposal {
  id: string;
  daoId: string;
  proposer: DAOMember;
  title: string;
  description: string;
  type: ProposalType;
  status: ProposalStatus;
  votesFor: number;
  votesAgainst: number;
  votesAbstain: number;
  totalVotes: number;
  quorumReached: boolean;
  createdAt: Date;
  votingStartsAt: Date;
  votingEndsAt: Date;
  executedAt?: Date;
  executionData?: any;
  discussionUrl?: string;
}

export type ProposalType =
  | 'governance'
  | 'treasury'
  | 'membership'
  | 'custom';

export type ProposalStatus =
  | 'pending'
  | 'active'
  | 'passed'
  | 'rejected'
  | 'executed'
  | 'cancelled';

export interface ProposalInput {
  title: string;
  description: string;
  type: ProposalType;
  executionData?: any;
}

export type VoteType = 'for' | 'against' | 'abstain';

export interface Vote {
  id: string;
  proposalId: string;
  voterId: string;
  voter: DAOMember;
  vote: VoteType;
  weight: number;
  votedAt: Date;
  reason?: string;
}

export interface Treasury {
  daoId: string;
  totalValue: string; // USD
  assets: TreasuryAsset[];
  transactions: TreasuryTransaction[];
  analytics: TreasuryAnalytics;
}

export interface TreasuryAsset {
  asset: string;
  symbol: string;
  amount: string;
  valueUsd: string;
  percentage: number;
  logoUrl?: string;
}

export interface TreasuryTransaction {
  id: string;
  type: 'inflow' | 'outflow';
  asset: string;
  amount: string;
  valueUsd: string;
  from?: string;
  to?: string;
  description?: string;
  proposalId?: string;
  timestamp: Date;
  txHash: string;
}

export interface TreasuryAnalytics {
  totalInflows: string; // USD
  totalOutflows: string; // USD
  netChange: string; // USD
  changePercentage: number;
  period: '24h' | '7d' | '30d' | 'all';
  inflowsOverTime: TimeSeriesData[];
  outflowsOverTime: TimeSeriesData[];
}

export interface TimeSeriesData {
  timestamp: Date;
  value: number;
}

export interface SpendProposal {
  recipient: string;
  asset: string;
  amount: string;
  reason: string;
}

export interface DAOParams {
  name: string;
  description: string;
  logoUrl?: string;
  governance: DAOGovernance;
  initialMembers?: string[];
  initialTreasuryAmount?: string;
}

export interface DAOStats {
  totalProposals: number;
  passedProposals: number;
  rejectedProposals: number;
  averageParticipation: number; // percentage
  treasuryGrowth: number; // percentage
  activeMembers: number;
}

export type ProposalFilter = 'active' | 'passed' | 'rejected' | 'all';

export interface VoteBreakdown {
  for: {
    count: number;
    percentage: number;
    voters: DAOMember[];
  };
  against: {
    count: number;
    percentage: number;
    voters: DAOMember[];
  };
  abstain: {
    count: number;
    percentage: number;
    voters: DAOMember[];
  };
  quorumProgress: number; // percentage
  quorumRequired: number; // percentage
}
