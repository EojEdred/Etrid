/**
 * Social Features Type Definitions
 * Comprehensive TypeScript types for username system, contacts, bill splitting, and social recovery
 */

// ============================================================================
// USERNAME SYSTEM
// ============================================================================

export interface UsernameRegistration {
  id: string;
  username: string;
  address: string;
  expiresAt: Date;
  createdAt: Date;
  transactionHash?: string;
}

export interface UsernameAvailability {
  available: boolean;
  username: string;
  suggestions?: string[];
}

export interface UsernamePricing {
  username: string;
  length: number;
  price: number; // in ÉTR
  tier: 'premium' | 'standard' | 'basic';
}

export interface UsernameResolution {
  username: string;
  address: string;
  avatar?: string;
  metadata?: Record<string, any>;
}

// ============================================================================
// CONTACTS
// ============================================================================

export interface Contact {
  id: string;
  userId: string;
  contactAddress: string;
  contactUsername?: string;
  nickname?: string;
  avatar?: string;
  notes?: string;
  isFavorite: boolean;
  tags: string[];
  createdAt: Date;
  updatedAt: Date;
  lastTransactionAt?: Date;
}

export interface ContactInput {
  contactAddress?: string;
  contactUsername?: string;
  nickname?: string;
  notes?: string;
  tags?: string[];
}

export interface ContactGroup {
  id: string;
  name: string;
  description?: string;
  contactIds: string[];
  createdAt: Date;
}

// ============================================================================
// BILL SPLITTING
// ============================================================================

export type BillSplitType = 'equal' | 'custom' | 'percentage';
export type BillSplitStatus = 'pending' | 'partial' | 'completed' | 'cancelled';
export type ParticipantStatus = 'pending' | 'paid' | 'declined';

export interface BillSplitParticipant {
  id: string;
  userId?: string;
  address: string;
  username?: string;
  amountOwed: number;
  amountPaid: number;
  status: ParticipantStatus;
  paidAt?: Date;
  transactionHash?: string;
}

export interface BillSplit {
  id: string;
  creatorId: string;
  creatorAddress: string;
  name: string;
  description?: string;
  totalAmount: number;
  splitType: BillSplitType;
  status: BillSplitStatus;
  participants: BillSplitParticipant[];
  createdAt: Date;
  updatedAt: Date;
  completedAt?: Date;
  notes?: string;
}

export interface BillSplitInput {
  name: string;
  description?: string;
  totalAmount: number;
  splitType: BillSplitType;
  participants: {
    address: string;
    username?: string;
    amountOwed?: number; // Required for custom split
    percentage?: number; // Required for percentage split
  }[];
  notes?: string;
}

export interface BillSplitSummary {
  total: number;
  pending: number;
  completed: number;
  totalOwed: number;
  totalReceivable: number;
}

// ============================================================================
// SOCIAL RECOVERY
// ============================================================================

export type GuardianStatus = 'pending' | 'active' | 'declined' | 'removed';
export type RecoveryStatus = 'initiated' | 'pending_approval' | 'approved' | 'completed' | 'cancelled' | 'failed';

export interface Guardian {
  id: string;
  walletAddress: string;
  guardianAddress: string;
  guardianUsername?: string;
  status: GuardianStatus;
  addedAt: Date;
  activatedAt?: Date;
  invitationSentAt?: Date;
  lastReminderAt?: Date;
}

export interface SocialRecoveryConfig {
  walletAddress: string;
  threshold: number;
  guardians: Guardian[];
  createdAt: Date;
  updatedAt: Date;
}

export interface RecoveryApproval {
  guardianId: string;
  guardianAddress: string;
  signature: string;
  approvedAt: Date;
  transactionHash?: string;
}

export interface RecoveryProcess {
  id: string;
  walletAddress: string;
  newDeviceId: string;
  newAddress?: string;
  status: RecoveryStatus;
  requiredApprovals: number;
  approvals: RecoveryApproval[];
  initiatedAt: Date;
  completesAt?: Date; // Time delay for security (48 hours)
  completedAt?: Date;
  cancelledAt?: Date;
  failedReason?: string;
}

export interface RecoveryInitiation {
  walletAddress: string;
  newDeviceId: string;
  newAddress?: string;
}

// ============================================================================
// SOCIAL FEED
// ============================================================================

export type ActivityType =
  | 'transaction_sent'
  | 'transaction_received'
  | 'username_registered'
  | 'milestone_reached'
  | 'staking_reward'
  | 'governance_vote'
  | 'bill_split_created'
  | 'bill_split_paid'
  | 'guardian_added'
  | 'recovery_initiated';

export interface SocialActivity {
  id: string;
  userId: string;
  userAddress: string;
  username?: string;
  activityType: ActivityType;
  title: string;
  description: string;
  amount?: number;
  metadata?: Record<string, any>;
  createdAt: Date;
  isPublic: boolean;
  likes: number;
  comments: number;
}

export interface ActivityFeedFilter {
  type?: ActivityType[];
  friendsOnly?: boolean;
  dateFrom?: Date;
  dateTo?: Date;
  limit?: number;
  offset?: number;
}

export interface Milestone {
  type: 'balance' | 'transactions' | 'staking' | 'governance';
  threshold: number;
  title: string;
  description: string;
  icon: string;
}

// ============================================================================
// USER PROFILE
// ============================================================================

export interface SocialProfile {
  address: string;
  username?: string;
  avatar?: string;
  bio?: string;
  displayName?: string;
  socialLinks?: {
    twitter?: string;
    github?: string;
    website?: string;
  };
  privacy: {
    showBalance: boolean;
    showTransactions: boolean;
    showActivity: boolean;
  };
  createdAt: Date;
  updatedAt: Date;
}

// ============================================================================
// API RESPONSES
// ============================================================================

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: any;
  };
  timestamp: Date;
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    total: number;
    page: number;
    pageSize: number;
    totalPages: number;
  };
}

// ============================================================================
// NOTIFICATIONS
// ============================================================================

export type NotificationType =
  | 'bill_split_request'
  | 'bill_split_reminder'
  | 'bill_split_paid'
  | 'guardian_request'
  | 'recovery_approval_needed'
  | 'recovery_approved'
  | 'username_expiring'
  | 'contact_added';

export interface Notification {
  id: string;
  userId: string;
  type: NotificationType;
  title: string;
  message: string;
  data?: Record<string, any>;
  read: boolean;
  createdAt: Date;
  expiresAt?: Date;
}

// ============================================================================
// ERRORS
// ============================================================================

export class SocialFeatureError extends Error {
  constructor(
    message: string,
    public code: string,
    public details?: any
  ) {
    super(message);
    this.name = 'SocialFeatureError';
  }
}

export class UsernameError extends SocialFeatureError {
  constructor(message: string, code: string, details?: any) {
    super(message, code, details);
    this.name = 'UsernameError';
  }
}

export class BillSplitError extends SocialFeatureError {
  constructor(message: string, code: string, details?: any) {
    super(message, code, details);
    this.name = 'BillSplitError';
  }
}

export class RecoveryError extends SocialFeatureError {
  constructor(message: string, code: string, details?: any) {
    super(message, code, details);
    this.name = 'RecoveryError';
  }
}
