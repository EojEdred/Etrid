/**
 * Enhanced types for Etrid SDK
 */

import { SubmittableResult, SubmittableExtrinsic } from '@polkadot/api/types';
import { KeyringPair } from '@polkadot/keyring/types';

/**
 * Transaction options
 */
export interface TransactionOptions {
  /** Transaction nonce (if not provided, will be auto-detected) */
  nonce?: number;
  /** Transaction tip in smallest unit */
  tip?: bigint;
  /** Mortality period in blocks (0 for immortal) */
  mortality?: number;
  /** Era period for transaction validity */
  era?: number;
}

/**
 * Transaction result after submission
 */
export interface TransactionResult {
  /** Transaction hash */
  hash: string;
  /** Block hash where transaction was included */
  block: string;
  /** Events emitted by the transaction */
  events: any[];
  /** Whether transaction was successful */
  success: boolean;
  /** Error if transaction failed */
  error?: string;
}

/**
 * Extended balance information
 */
export interface ExtendedBalance {
  /** ETR balance */
  etr: bigint;
  /** ETD balance */
  etd: bigint;
  /** Reserved balance */
  reserved: bigint;
  /** Frozen balance */
  frozen: bigint;
  /** Total balance (etr + etd) */
  total: bigint;
  /** Available balance (total - frozen - reserved) */
  available: bigint;
}

/**
 * Validator status information
 */
export interface ValidatorStatus {
  /** Whether address is an active validator */
  isValidator: boolean;
  /** Total stake amount */
  stake: bigint;
  /** Commission rate (0-100) */
  commission: number;
  /** Whether validator is active */
  active: boolean;
  /** Number of nominators */
  nominatorCount?: number;
  /** Self-stake amount */
  selfStake?: bigint;
}

/**
 * Staking information
 */
export interface StakingInfo {
  /** Amount staked */
  staked: bigint;
  /** Validator address (if bonded) */
  validator?: string;
  /** Rewards earned */
  rewards: bigint;
  /** Unbonding period remaining (in blocks) */
  unbondingPeriod?: number;
  /** Status: bonded, unbonding, or idle */
  status: 'bonded' | 'unbonding' | 'idle';
}

/**
 * Governance proposal
 */
export interface Proposal {
  /** Proposal ID */
  id: number;
  /** Proposer address */
  proposer: string;
  /** Proposal title */
  title: string;
  /** Proposal description */
  description: string;
  /** Total votes for */
  votesFor: bigint;
  /** Total votes against */
  votesAgainst: bigint;
  /** Proposal status */
  status: 'active' | 'passed' | 'rejected' | 'executed';
  /** Block when proposal was created */
  createdAt: number;
  /** Block when voting ends */
  endsAt: number;
}

/**
 * Vote information
 */
export interface Vote {
  /** Proposal ID */
  proposalId: number;
  /** Voter address */
  voter: string;
  /** Vote direction */
  approve: boolean;
  /** Vote weight (stake) */
  weight: bigint;
  /** Block when vote was cast */
  timestamp: number;
}

/**
 * Lightning channel information
 */
export interface ChannelInfo {
  /** Channel ID */
  channelId: string;
  /** Local party address */
  local: string;
  /** Remote party address */
  remote: string;
  /** Local balance */
  localBalance: bigint;
  /** Remote balance */
  remoteBalance: bigint;
  /** Channel capacity */
  capacity: bigint;
  /** Channel status */
  status: 'opening' | 'open' | 'closing' | 'closed';
  /** Block when channel was opened */
  openedAt?: number;
  /** Number of updates */
  updateCount: number;
}

/**
 * Payment information
 */
export interface PaymentInfo {
  /** Payment hash */
  hash: string;
  /** Amount */
  amount: bigint;
  /** Fee paid */
  fee: bigint;
  /** Payment status */
  status: 'pending' | 'completed' | 'failed';
  /** Route taken */
  route?: string[];
  /** Timestamp */
  timestamp: number;
}

/**
 * Error codes
 */
export enum ErrorCode {
  // Transaction errors
  TRANSACTION_FAILED = 'TRANSACTION_FAILED',
  TRANSACTION_INVALID = 'TRANSACTION_INVALID',
  INSUFFICIENT_BALANCE = 'INSUFFICIENT_BALANCE',

  // Validation errors
  VALIDATION_ERROR = 'VALIDATION_ERROR',
  INVALID_ADDRESS = 'INVALID_ADDRESS',
  INVALID_AMOUNT = 'INVALID_AMOUNT',
  INVALID_PARAMETERS = 'INVALID_PARAMETERS',

  // Network errors
  NETWORK_ERROR = 'NETWORK_ERROR',
  CONNECTION_FAILED = 'CONNECTION_FAILED',
  RPC_ERROR = 'RPC_ERROR',

  // State errors
  NOT_CONNECTED = 'NOT_CONNECTED',
  ALREADY_CONNECTED = 'ALREADY_CONNECTED',

  // Module-specific errors
  STAKING_ERROR = 'STAKING_ERROR',
  GOVERNANCE_ERROR = 'GOVERNANCE_ERROR',
  CHANNEL_ERROR = 'CHANNEL_ERROR',

  // Unknown
  UNKNOWN_ERROR = 'UNKNOWN_ERROR',
}

/**
 * Module error information
 */
export interface ModuleError {
  /** Module index */
  index: number;
  /** Error code within module */
  error: number;
  /** Error message if available */
  message?: string;
}

/**
 * Event information
 */
export interface EventInfo {
  /** Event section (module name) */
  section: string;
  /** Event method */
  method: string;
  /** Event data */
  data: any[];
  /** Block number */
  blockNumber?: number;
  /** Transaction hash */
  txHash?: string;
}

/**
 * Block information (extended)
 */
export interface BlockInfo {
  /** Block number */
  number: number;
  /** Block hash */
  hash: string;
  /** Parent hash */
  parentHash: string;
  /** State root */
  stateRoot: string;
  /** Extrinsics root */
  extrinsicsRoot: string;
  /** Timestamp */
  timestamp: number;
  /** Number of extrinsics */
  extrinsicCount: number;
  /** Validator who produced the block */
  validator?: string;
}

/**
 * Query result wrapper
 */
export interface QueryResult<T> {
  /** Query data */
  data: T;
  /** Block number when queried */
  blockNumber: number;
  /** Block hash when queried */
  blockHash: string;
}

/**
 * Pagination options
 */
export interface PaginationOptions {
  /** Page number (0-indexed) */
  page: number;
  /** Items per page */
  limit: number;
}

/**
 * Paginated result
 */
export interface PaginatedResult<T> {
  /** Items in current page */
  items: T[];
  /** Total item count */
  total: number;
  /** Current page */
  page: number;
  /** Items per page */
  limit: number;
  /** Total pages */
  totalPages: number;
  /** Whether there is a next page */
  hasNext: boolean;
  /** Whether there is a previous page */
  hasPrevious: boolean;
}
