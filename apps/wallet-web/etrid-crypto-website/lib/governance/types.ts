/**
 * ETRID Governance Types
 *
 * Comprehensive type definitions for governance operations matching the iOS app
 */

// ═══════════════════════════════════════════════════════════════════════════════
// PROPOSAL TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Proposal categories supported by ETRID governance
 */
export enum ProposalCategory {
  InflationRate = 'InflationRate',
  ParameterChange = 'ParameterChange',
  BudgetAllocation = 'BudgetAllocation',
  ProtocolUpgrade = 'ProtocolUpgrade',
  DirectorElection = 'DirectorElection',
  EmergencyAction = 'EmergencyAction',
}

/**
 * Proposal lifecycle status
 */
export enum ProposalStatus {
  Pending = 'Pending',
  Active = 'Active',
  Passed = 'Passed',
  Rejected = 'Rejected',
  Executed = 'Executed',
  Cancelled = 'Cancelled',
}

/**
 * Complete proposal data structure
 */
export interface Proposal {
  id: number;
  proposer: string;
  title: string;
  description: string;
  category: ProposalCategory;
  status: ProposalStatus;
  createdAt: number;
  votingDeadline: number;
  executionBlock?: number;

  // Voting tallies
  votesFor: string;
  votesAgainst: string;
  votesAbstain: string;
  totalVotingPower: string;

  // Execution status
  approved: boolean;
  executed: boolean;
  executedAt?: number;

  // Category-specific data
  metadata?: ProposalMetadata;
}

/**
 * Category-specific proposal metadata
 */
export interface ProposalMetadata {
  // InflationRate
  newInflationRate?: number; // basis points

  // ParameterChange
  parameter?: string;
  newValue?: string;

  // BudgetAllocation
  recipient?: string;
  amount?: string;
  purpose?: string;

  // ProtocolUpgrade
  runtimeVersion?: number;
  specVersion?: number;
  wasmHash?: string;

  // DirectorElection
  candidates?: string[];
  seats?: number;

  // EmergencyAction
  action?: string;
  justification?: string;
}

/**
 * Proposal creation parameters
 */
export interface CreateProposalParams {
  title: string;
  description: string;
  category: ProposalCategory;
  metadata?: ProposalMetadata;
  deposit?: string; // Optional explicit deposit amount
}

// ═══════════════════════════════════════════════════════════════════════════════
// VOTING TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Vote direction
 */
export enum VoteType {
  Aye = 'Aye',
  Nay = 'Nay',
  Abstain = 'Abstain',
}

/**
 * Conviction voting multipliers with lock periods
 */
export enum ConvictionLevel {
  None = 0,      // 0.1x multiplier, no lock
  Locked1x = 1,  // 1x multiplier, 7 days
  Locked2x = 2,  // 2x multiplier, 14 days
  Locked3x = 3,  // 3x multiplier, 28 days
  Locked4x = 4,  // 4x multiplier, 56 days
  Locked5x = 5,  // 5x multiplier, 112 days
  Locked6x = 6,  // 6x multiplier, 224 days
}

/**
 * Conviction multiplier configuration
 */
export interface ConvictionConfig {
  level: ConvictionLevel;
  multiplier: number;
  lockPeriodDays: number;
  description: string;
}

/**
 * Vote casting parameters
 */
export interface CastVoteParams {
  proposalId: number;
  voteType: VoteType;
  conviction: ConvictionLevel;
  balance?: string; // Optional: specific balance to vote with
}

/**
 * Individual vote record
 */
export interface VoteRecord {
  voter: string;
  proposalId: number;
  voteType: VoteType;
  conviction: ConvictionLevel;
  balance: string;
  votingPower: string; // balance * conviction multiplier
  timestamp: number;
}

/**
 * Aggregated voting statistics for a proposal
 */
export interface VotingStats {
  proposalId: number;
  totalVotes: number;
  totalVotingPower: string;
  ayeVotes: number;
  ayePower: string;
  nayVotes: number;
  nayPower: string;
  abstainVotes: number;
  abstainPower: string;
  turnoutPercent: number;
  passingPercent: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// DELEGATION TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Vote delegation parameters
 */
export interface DelegateParams {
  target: string;
  conviction: ConvictionLevel;
  balance?: string; // Optional: specific balance to delegate
  tracks?: ProposalCategory[]; // Optional: specific categories to delegate
}

/**
 * Active delegation record
 */
export interface Delegation {
  delegator: string;
  target: string;
  conviction: ConvictionLevel;
  balance: string;
  votingPower: string;
  tracks: ProposalCategory[];
  createdAt: number;
}

/**
 * Delegation statistics for an account
 */
export interface DelegationStats {
  totalDelegated: string;
  totalReceived: string;
  activeDelegations: number;
  receivedDelegations: number;
  delegatedVotingPower: string;
  receivedVotingPower: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// VOTING POWER TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Account voting power calculation
 */
export interface VotingPower {
  address: string;

  // Staked balance
  stakedBalance: string;

  // Lock information
  lockedBalance: string;
  lockedUntil?: number;
  lockPeriodDays?: number;

  // Calculated voting power
  baseVotingPower: string;
  delegatedVotingPower: string;
  totalVotingPower: string;

  // Participation
  canVote: boolean;
  canDelegate: boolean;
  participationHistory: number;

  // Active positions
  activeVotes: number;
  activeDelegations: number;
}

/**
 * Voting power calculation breakdown
 */
export interface VotingPowerBreakdown {
  baseBalance: string;
  conviction: ConvictionLevel;
  multiplier: number;
  calculatedPower: string;
  lockPeriodDays: number;
  unlockDate?: Date;
}

// ═══════════════════════════════════════════════════════════════════════════════
// GOVERNANCE STATISTICS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Overall governance metrics
 */
export interface GovernanceStats {
  // Proposals
  totalProposals: number;
  activeProposals: number;
  passedProposals: number;
  rejectedProposals: number;
  executedProposals: number;

  // Participation
  totalVotes: number;
  uniqueVoters: number;
  totalVotingPower: string;
  participationRate: number;

  // Delegation
  totalDelegations: number;
  delegatedVotingPower: string;

  // Treasury
  treasuryBalance: string;
  totalDeposits: string;

  // Network
  circulatingSupply: string;
  stakedSupply: string;
  inflationRate: number;
}

/**
 * Category-specific statistics
 */
export interface CategoryStats {
  category: ProposalCategory;
  totalProposals: number;
  activeProposals: number;
  passRate: number;
  averageVotes: number;
  averageTurnout: number;
}

// ═══════════════════════════════════════════════════════════════════════════════
// EVENT TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Governance event types
 */
export enum GovernanceEventType {
  ProposalCreated = 'ProposalCreated',
  VoteCast = 'VoteCast',
  ProposalPassed = 'ProposalPassed',
  ProposalRejected = 'ProposalRejected',
  ProposalExecuted = 'ProposalExecuted',
  ProposalCancelled = 'ProposalCancelled',
  DelegationCreated = 'DelegationCreated',
  DelegationRevoked = 'DelegationRevoked',
}

/**
 * Governance event data
 */
export interface GovernanceEvent {
  type: GovernanceEventType;
  blockNumber: number;
  timestamp: number;
  proposalId?: number;
  account?: string;
  data?: Record<string, any>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// QUERY FILTERS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Proposal query filters
 */
export interface ProposalFilters {
  categories?: ProposalCategory[];
  statuses?: ProposalStatus[];
  proposer?: string;
  minVotes?: number;
  fromBlock?: number;
  toBlock?: number;
}

/**
 * Vote query filters
 */
export interface VoteFilters {
  proposalId?: number;
  voter?: string;
  voteType?: VoteType;
  minVotingPower?: string;
}

// ═══════════════════════════════════════════════════════════════════════════════
// ERROR TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Governance-specific errors
 */
export enum GovernanceErrorCode {
  NotConnected = 'NOT_CONNECTED',
  InsufficientBalance = 'INSUFFICIENT_BALANCE',
  ProposalNotFound = 'PROPOSAL_NOT_FOUND',
  VotingClosed = 'VOTING_CLOSED',
  AlreadyVoted = 'ALREADY_VOTED',
  InvalidConviction = 'INVALID_CONVICTION',
  InvalidCategory = 'INVALID_CATEGORY',
  InsufficientDeposit = 'INSUFFICIENT_DEPOSIT',
  DelegationExists = 'DELEGATION_EXISTS',
  NoDelegation = 'NO_DELEGATION',
  Unauthorized = 'UNAUTHORIZED',
  TransactionFailed = 'TRANSACTION_FAILED',
}

/**
 * Governance error with details
 */
export interface GovernanceError {
  code: GovernanceErrorCode;
  message: string;
  details?: any;
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Transaction result
 */
export interface TransactionResult {
  success: boolean;
  txHash?: string;
  blockHash?: string;
  blockNumber?: number;
  error?: GovernanceError;
}

/**
 * Pagination parameters
 */
export interface PaginationParams {
  page: number;
  pageSize: number;
}

/**
 * Paginated response
 */
export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONVICTION LEVEL MAPPINGS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Conviction level configurations
 */
export const CONVICTION_CONFIGS: ConvictionConfig[] = [
  {
    level: ConvictionLevel.None,
    multiplier: 0.1,
    lockPeriodDays: 0,
    description: '0.1x voting power, no lock period',
  },
  {
    level: ConvictionLevel.Locked1x,
    multiplier: 1,
    lockPeriodDays: 7,
    description: '1x voting power, locked for 7 days',
  },
  {
    level: ConvictionLevel.Locked2x,
    multiplier: 2,
    lockPeriodDays: 14,
    description: '2x voting power, locked for 14 days',
  },
  {
    level: ConvictionLevel.Locked3x,
    multiplier: 3,
    lockPeriodDays: 28,
    description: '3x voting power, locked for 28 days',
  },
  {
    level: ConvictionLevel.Locked4x,
    multiplier: 4,
    lockPeriodDays: 56,
    description: '4x voting power, locked for 56 days',
  },
  {
    level: ConvictionLevel.Locked5x,
    multiplier: 5,
    lockPeriodDays: 112,
    description: '5x voting power, locked for 112 days',
  },
  {
    level: ConvictionLevel.Locked6x,
    multiplier: 6,
    lockPeriodDays: 224,
    description: '6x voting power, locked for 224 days',
  },
];

/**
 * Get conviction config by level
 */
export function getConvictionConfig(level: ConvictionLevel): ConvictionConfig {
  return CONVICTION_CONFIGS.find(c => c.level === level) || CONVICTION_CONFIGS[0];
}

/**
 * Calculate voting power with conviction
 */
export function calculateVotingPower(balance: string, conviction: ConvictionLevel): string {
  const config = getConvictionConfig(conviction);
  const balanceNum = parseFloat(balance);
  const power = balanceNum * config.multiplier;
  return power.toString();
}

/**
 * Category display names
 */
export const CATEGORY_LABELS: Record<ProposalCategory, string> = {
  [ProposalCategory.InflationRate]: 'Inflation Rate',
  [ProposalCategory.ParameterChange]: 'Parameter Change',
  [ProposalCategory.BudgetAllocation]: 'Budget Allocation',
  [ProposalCategory.ProtocolUpgrade]: 'Protocol Upgrade',
  [ProposalCategory.DirectorElection]: 'Director Election',
  [ProposalCategory.EmergencyAction]: 'Emergency Action',
};

/**
 * Category descriptions
 */
export const CATEGORY_DESCRIPTIONS: Record<ProposalCategory, string> = {
  [ProposalCategory.InflationRate]: 'Modify network inflation parameters',
  [ProposalCategory.ParameterChange]: 'Update runtime parameters',
  [ProposalCategory.BudgetAllocation]: 'Treasury fund allocation',
  [ProposalCategory.ProtocolUpgrade]: 'Runtime and protocol upgrades',
  [ProposalCategory.DirectorElection]: 'Elect network directors',
  [ProposalCategory.EmergencyAction]: 'Emergency protocol actions',
};

/**
 * Status display names
 */
export const STATUS_LABELS: Record<ProposalStatus, string> = {
  [ProposalStatus.Pending]: 'Pending',
  [ProposalStatus.Active]: 'Active',
  [ProposalStatus.Passed]: 'Passed',
  [ProposalStatus.Rejected]: 'Rejected',
  [ProposalStatus.Executed]: 'Executed',
  [ProposalStatus.Cancelled]: 'Cancelled',
};
