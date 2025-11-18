import { Request } from 'express';

// ============================================================================
// USER TYPES
// ============================================================================
export interface User {
  id: string;
  address: string;
  email?: string;
  phone?: string;
  kyc_status: 'pending' | 'verified' | 'rejected';
  kyc_level: 0 | 1 | 2;
  email_verified: boolean;
  phone_verified: boolean;
  two_factor_enabled: boolean;
  created_at: Date;
  last_login?: Date;
}

export interface CreateUserDTO {
  address: string;
  email?: string;
  phone?: string;
}

// ============================================================================
// TRANSACTION TYPES
// ============================================================================
export interface Transaction {
  id: string;
  user_id: string;
  tx_hash: string;
  block_number?: number;
  from_address: string;
  to_address: string;
  amount: string;
  asset: string;
  fee: string;
  status: 'pending' | 'confirmed' | 'failed';
  tx_type: 'transfer' | 'stake' | 'unstake' | 'vote' | 'bridge' | 'swap';
  metadata?: Record<string, any>;
  created_at: Date;
  confirmed_at?: Date;
}

export interface CreateTransactionDTO {
  user_id: string;
  tx_hash: string;
  from_address: string;
  to_address: string;
  amount: string;
  asset: string;
  fee?: string;
  tx_type: 'transfer' | 'stake' | 'unstake' | 'vote' | 'bridge' | 'swap';
  metadata?: Record<string, any>;
}

// ============================================================================
// STAKING TYPES
// ============================================================================
export interface StakingPosition {
  id: string;
  user_id: string;
  validator_address: string;
  validator_name?: string;
  amount: string;
  rewards_earned: string;
  rewards_claimed: string;
  apy?: number;
  start_date: Date;
  status: 'active' | 'unbonding' | 'withdrawn' | 'slashed';
  auto_compound: boolean;
  created_at: Date;
}

export interface Validator {
  id: string;
  address: string;
  name?: string;
  description?: string;
  commission_rate: number;
  total_stake: string;
  apy?: number;
  is_active: boolean;
  uptime_percentage?: number;
}

// ============================================================================
// GOVERNANCE TYPES
// ============================================================================
export interface GovernanceVote {
  id: string;
  user_id: string;
  proposal_id: number;
  support: boolean;
  conviction: number;
  voting_power: string;
  tx_hash?: string;
  created_at: Date;
}

export interface Proposal {
  id: number;
  proposal_hash: string;
  title: string;
  description: string;
  proposer_address: string;
  proposal_type: string;
  status: 'active' | 'passed' | 'rejected' | 'cancelled' | 'executed';
  yes_votes: string;
  no_votes: string;
  voting_starts_at: Date;
  voting_ends_at: Date;
}

// ============================================================================
// ATM TYPES
// ============================================================================
export interface ATMWithdrawal {
  id: string;
  user_id: string;
  withdrawal_code: string;
  amount_usd: number;
  amount_crypto: string;
  asset: string;
  fee: number;
  atm_partner: 'Coinme' | 'Bitcoin Depot' | 'CoinFlip';
  status: 'pending' | 'processing' | 'ready' | 'completed' | 'expired' | 'failed';
  expires_at: Date;
  created_at: Date;
}

export interface ATMLocation {
  id: string;
  partner: string;
  name: string;
  address: string;
  lat: number;
  lng: number;
  distance?: number; // In meters
  supported_assets: string[];
}

// ============================================================================
// GPU TYPES
// ============================================================================
export interface GPURental {
  id: string;
  user_id: string;
  gpu_id: string;
  gpu_name: string;
  provider: 'Vast.ai' | 'RunPod' | 'Internal';
  duration_hours: number;
  price_per_hour: string;
  total_cost: string;
  ssh_host?: string;
  ssh_port?: number;
  ssh_username?: string;
  status: 'pending' | 'provisioning' | 'active' | 'completed' | 'cancelled';
  start_time?: Date;
  end_time?: Date;
}

export interface GPUSearchResult {
  id: string;
  name: string;
  provider: string;
  vram_gb: number;
  gpu_count: number;
  cpu_cores: number;
  ram_gb: number;
  disk_gb: number;
  price_per_hour: string;
  availability: boolean;
}

// ============================================================================
// BRIDGE TYPES
// ============================================================================
export interface BridgeTransfer {
  id: string;
  user_id: string;
  from_chain: string;
  to_chain: string;
  from_address: string;
  to_address: string;
  from_asset: string;
  to_asset: string;
  amount_from: string;
  amount_to: string;
  status: 'pending' | 'confirming' | 'minting' | 'completed' | 'failed';
  created_at: Date;
}

// ============================================================================
// NOTIFICATION TYPES
// ============================================================================
export interface Notification {
  id: string;
  user_id: string;
  notification_type: string;
  title: string;
  body: string;
  data?: Record<string, any>;
  is_read: boolean;
  created_at: Date;
}

// ============================================================================
// API TYPES
// ============================================================================
export interface AuthenticatedRequest extends Request {
  user?: {
    id: string;
    address: string;
    email?: string;
  };
}

export interface PaginationParams {
  page: number;
  limit: number;
  offset: number;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
}

export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: any;
  };
  timestamp: string;
}

// ============================================================================
// BLOCKCHAIN TYPES
// ============================================================================
export interface BlockchainBlock {
  number: number;
  hash: string;
  timestamp: number;
  transactions: BlockchainTransaction[];
}

export interface BlockchainTransaction {
  hash: string;
  from: string;
  to: string;
  amount: string;
  asset: string;
  type: string;
}

export interface Balance {
  asset: string;
  free: string;
  reserved: string;
  total: string;
  locked?: string;
  price_usd?: number;
  value_usd?: number;
}

// ============================================================================
// PRICE TYPES
// ============================================================================
export interface PriceData {
  asset: string;
  price_usd: number;
  change_24h?: number;
  volume_24h?: number;
  market_cap?: number;
  last_updated: Date;
}

// ============================================================================
// ANALYTICS TYPES
// ============================================================================
export interface AnalyticsEvent {
  user_id?: string;
  event_name: string;
  event_properties?: Record<string, any>;
  user_agent?: string;
  ip_address?: string;
}

// ============================================================================
// CACHE TYPES
// ============================================================================
export interface CacheOptions {
  ttl?: number; // Time to live in seconds
  prefix?: string;
}
