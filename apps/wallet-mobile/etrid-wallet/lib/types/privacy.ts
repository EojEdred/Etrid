/**
 * Privacy Types
 * Defines types for privacy features
 */

export interface PrivacySettings {
  userId: string;
  privacyLevel: 'low' | 'medium' | 'high';

  // Stealth addresses
  stealthAddressesEnabled: boolean;
  autoGenerateNewAddress: boolean;

  // Coin mixing
  mixingEnabled: boolean;
  defaultMixingRounds: number;
  autoMixThreshold: string; // Auto-mix when balance exceeds this

  // Metadata scrubbing
  metadataScrubbing: boolean;
  torRoutingEnabled: boolean;
  randomizeTimings: boolean;
  decoyTransactions: boolean;

  updatedAt: Date;
}

export interface StealthAddress {
  id: string;
  userId: string;
  address: string;
  publicKey: string;
  status: 'unused' | 'used' | 'expired';
  generatedAt: Date;
  usedAt?: Date;
  expiresAt?: Date;
  label?: string;
  linkedTransaction?: string;
}

export interface MixingSession {
  id: string;
  userId: string;
  amount: string;
  rounds: number;
  currentRound: number;
  status: 'pending' | 'mixing' | 'completed' | 'failed';
  startedAt: Date;
  completedAt?: Date;
  estimatedCompletion?: Date;
  participants: number;
  fee: string;
  privacyScoreIncrease: number;
}

export interface MixingStatus {
  sessionId: string;
  progress: number; // 0-100
  currentRound: number;
  totalRounds: number;
  participantsInRound: number;
  estimatedTimeRemaining: number; // seconds
  status: 'waiting' | 'mixing' | 'complete' | 'error';
  message?: string;
}

export interface PrivacyScore {
  overall: number; // 0-100
  breakdown: {
    addressReuse: number; // Lower is better
    mixingLevel: number;
    metadataProtection: number;
    networkPrivacy: number;
  };
  riskLevel: 'low' | 'medium' | 'high';
  recommendations: PrivacyRecommendation[];
}

export interface PrivacyRecommendation {
  id: string;
  title: string;
  description: string;
  impact: number;
  priority: 'low' | 'medium' | 'high';
  implemented: boolean;
}

export interface TransactionMetadata {
  ipAddress?: string;
  timestamp: Date;
  userAgent?: string;
  location?: string;
  scrubbed: boolean;
}

export interface MixingRound {
  roundNumber: number;
  participants: string[];
  status: 'forming' | 'active' | 'complete';
  startedAt?: Date;
  completedAt?: Date;
}

export interface PrivacyLevelConfig {
  name: string;
  description: string;
  features: {
    stealthAddresses: boolean;
    mixing: boolean;
    mixingRounds: number;
    torRouting: boolean;
    metadataScrubbing: boolean;
    decoyTransactions: boolean;
  };
  speed: 'fast' | 'medium' | 'slow';
  cost: 'low' | 'medium' | 'high';
  privacyRating: number; // 0-100
}

export const PRIVACY_LEVELS: Record<string, PrivacyLevelConfig> = {
  low: {
    name: 'Low Privacy',
    description: 'Normal blockchain transparency, fast and cheap',
    features: {
      stealthAddresses: false,
      mixing: false,
      mixingRounds: 0,
      torRouting: false,
      metadataScrubbing: false,
      decoyTransactions: false,
    },
    speed: 'fast',
    cost: 'low',
    privacyRating: 20,
  },
  medium: {
    name: 'Medium Privacy',
    description: 'Balanced privacy with reasonable costs',
    features: {
      stealthAddresses: true,
      mixing: true,
      mixingRounds: 3,
      torRouting: false,
      metadataScrubbing: true,
      decoyTransactions: false,
    },
    speed: 'medium',
    cost: 'medium',
    privacyRating: 60,
  },
  high: {
    name: 'High Privacy',
    description: 'Maximum privacy, slower and more expensive',
    features: {
      stealthAddresses: true,
      mixing: true,
      mixingRounds: 10,
      torRouting: true,
      metadataScrubbing: true,
      decoyTransactions: true,
    },
    speed: 'slow',
    cost: 'high',
    privacyRating: 95,
  },
};

export const MIXING_FEE_PERCENT = 0.5; // 0.5% per round
