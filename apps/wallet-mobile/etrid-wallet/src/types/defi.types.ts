/**
 * DeFi Types - Type definitions for Staking, Governance, and Lightning-Bloc
 * Ëtrid Mobile DeFi Wallet - Phase 2
 */

// ============================================================================
// Staking Types
// ============================================================================

export interface StakingInfo {
  totalStaked: string; // Amount staked in smallest unit (12 decimals)
  totalStakedETR: number; // Human-readable amount
  totalStakedUSD: number;
  currentAPY: number; // Annual Percentage Yield (e.g., 12.5 for 12.5%)
  dailyRewards: string; // Daily rewards in smallest unit
  dailyRewardsETR: number;
  totalEarned: string; // All-time earned rewards
  totalEarnedETR: number;
  totalEarnedUSD: number;
  activeValidators: ValidatorStake[];
  unbondingPeriod: number; // Days
  unbondingPositions: UnbondingPosition[];
  rewardsHistory: RewardHistory[];
}

export interface ValidatorStake {
  validatorAddress: string;
  validatorName: string;
  stakedAmount: string;
  stakedAmountETR: number;
  commission: number; // Percentage (e.g., 5 for 5%)
  apy: number;
  status: 'active' | 'waiting' | 'inactive';
  uptime: number; // Percentage (e.g., 99.5 for 99.5%)
  rank?: number;
}

export interface Validator {
  address: string;
  name: string;
  commission: number;
  apy: number;
  uptime: number;
  totalStake: string;
  totalStakeETR: number;
  nominators: number;
  status: 'active' | 'waiting' | 'inactive';
  identity?: {
    display?: string;
    email?: string;
    website?: string;
    twitter?: string;
  };
}

export interface UnbondingPosition {
  amount: string;
  amountETR: number;
  unbondingAt: number; // Timestamp
  remainingDays: number;
  remainingHours: number;
  status: 'unbonding' | 'ready';
}

export interface RewardHistory {
  timestamp: number;
  amount: string;
  amountETR: number;
  amountUSD: number;
  validator: string;
  type: 'staking_reward' | 'compound';
}

export interface StakeOptions {
  amount: string; // Amount to stake in smallest unit
  validatorAddress?: string; // Optional, auto-select if not provided
  duration?: 'flexible' | '1month' | '3months' | '1year';
  autoCompound: boolean; // Auto-compound rewards or send to wallet
}

export interface StakingEstimate {
  dailyReward: number;
  monthlyReward: number;
  yearlyReward: number;
  effectiveAPY: number;
}

// ============================================================================
// Governance Types
// ============================================================================

export interface Proposal {
  id: number;
  proposalHash: string;
  title: string;
  description: string;
  proposer: string;
  proposerName?: string;
  status: ProposalStatus;
  type: ProposalType;
  votesYes: string; // In smallest unit
  votesYesETR: number;
  votesNo: string;
  votesNoETR: number;
  votesAbstain: string;
  votesAbstainETR: number;
  totalVotes: string;
  totalVotesETR: number;
  quorum: number; // Required percentage
  currentQuorum: number; // Current percentage
  threshold: number; // Required percentage for passage
  currentApproval: number; // Current approval percentage
  startBlock: number;
  endBlock: number;
  endsAt: number; // Timestamp
  timeRemaining: string; // Human-readable (e.g., "3 days 5 hours")
  discussionLink?: string;
  impactSummary?: string;
  hasVoted: boolean;
  userVote?: 'yes' | 'no' | 'abstain';
  userConviction?: number;
}

export type ProposalStatus =
  | 'active'
  | 'passed'
  | 'failed'
  | 'executed'
  | 'cancelled';

export type ProposalType =
  | 'runtime_upgrade'
  | 'treasury_spend'
  | 'parameter_change'
  | 'council_election'
  | 'general';

export interface VoteChoice {
  proposalId: number;
  vote: 'yes' | 'no' | 'abstain';
  conviction: ConvictionLevel;
  amount?: string; // Optional: amount of tokens to lock
}

export type ConvictionLevel = 0 | 1 | 2 | 3 | 4 | 5 | 6;

export interface ConvictionInfo {
  level: ConvictionLevel;
  multiplier: number; // Voting power multiplier (1x, 2x, 3x, etc.)
  lockDays: number; // Number of days tokens are locked
  description: string;
  recommended: boolean;
}

export const CONVICTION_LEVELS: ConvictionInfo[] = [
  { level: 0, multiplier: 0.1, lockDays: 0, description: 'No lock', recommended: false },
  { level: 1, multiplier: 1, lockDays: 7, description: '7 days lock', recommended: false },
  { level: 2, multiplier: 2, lockDays: 30, description: '30 days lock', recommended: false },
  { level: 3, multiplier: 3, lockDays: 90, description: '90 days lock', recommended: true },
  { level: 4, multiplier: 4, lockDays: 180, description: '180 days lock', recommended: false },
  { level: 5, multiplier: 5, lockDays: 365, description: '365 days lock', recommended: false },
  { level: 6, multiplier: 6, lockDays: 730, description: '730 days lock', recommended: false },
];

export interface VotingPower {
  availableBalance: string;
  availableBalanceETR: number;
  lockedBalance: string;
  lockedBalanceETR: number;
  totalVotingPower: number; // With conviction multiplier
  delegatedVotes: string;
  delegatedVotesETR: number;
  delegatedTo?: string;
  delegatedToName?: string;
}

export interface VoteHistory {
  proposalId: number;
  proposalTitle: string;
  vote: 'yes' | 'no' | 'abstain';
  conviction: ConvictionLevel;
  timestamp: number;
  result: 'passed' | 'failed' | 'pending';
}

export interface Delegation {
  delegateTo: string;
  delegateToName?: string;
  amount: string;
  amountETR: number;
  conviction: ConvictionLevel;
  timestamp: number;
}

// ============================================================================
// Lightning-Bloc Types (Instant Payments)
// ============================================================================

export interface LightningChannel {
  id: string;
  channelId: string;
  counterparty: string;
  counterpartyName?: string;
  capacity: string; // Total channel capacity
  capacityETR: number;
  localBalance: string; // Your balance
  localBalanceETR: number;
  remoteBalance: string; // Counterparty balance
  remoteBalanceETR: number;
  status: ChannelStatus;
  openedAt: number;
  closedAt?: number;
  txHash: string;
  isActive: boolean;
}

export type ChannelStatus =
  | 'opening'
  | 'active'
  | 'closing'
  | 'closed'
  | 'force_closed';

export interface OpenChannelRequest {
  counterparty: string;
  capacity: string; // Amount to lock in channel
  feeRate?: number; // Optional: custom fee rate
}

export interface LightningPayment {
  id: string;
  channelId: string;
  amount: string;
  amountETR: number;
  recipient: string;
  recipientName?: string;
  timestamp: number;
  status: 'pending' | 'completed' | 'failed';
  txHash?: string;
  fee: string;
  feeETR: number;
  type: 'sent' | 'received';
}

export interface LightningStats {
  totalChannels: number;
  activeChannels: number;
  totalCapacity: string;
  totalCapacityETR: number;
  totalSent: string;
  totalSentETR: number;
  totalReceived: string;
  totalReceivedETR: number;
  averageFee: number; // In percentage
}

// ============================================================================
// Common Types
// ============================================================================

export interface TransactionResult {
  success: boolean;
  txHash?: string;
  blockHash?: string;
  blockNumber?: number;
  error?: string;
  message?: string;
}

export interface LoadingState {
  loading: boolean;
  error?: string;
  lastUpdated?: number;
}

export interface RefreshOptions {
  force?: boolean; // Force refresh even if recently updated
  silent?: boolean; // Don't show loading state
}
