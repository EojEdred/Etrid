/**
 * Security Types
 * Defines types for enhanced security features
 */

export interface SecuritySettings {
  userId: string;
  // Whitelist settings
  whitelistEnabled: boolean;
  whitelistBypassAmount: string; // Max amount that bypasses whitelist

  // Timelock settings
  timelockEnabled: boolean;
  timelockHours: number; // Delay before withdrawal
  timelockExceptions: string[]; // Addresses exempt from timelock

  // Spending limits
  dailyLimit: string;
  weeklyLimit: string;
  monthlyLimit: string;
  perTransactionLimit: string;

  // Panic mode
  panicModeActive: boolean;
  duressPin?: string;

  // Guardians
  guardians: string[];
  guardianThreshold: number; // How many guardians needed to unlock

  // 2FA
  twoFactorEnabled: boolean;
  biometricsEnabled: boolean;

  updatedAt: Date;
}

export interface WhitelistedAddress {
  id: string;
  address: string;
  label?: string;
  addedAt: Date;
  addedBy: string;
}

export interface TimelockedTransaction {
  id: string;
  from: string;
  to: string;
  amount: string;
  unlockAt: Date;
  createdAt: Date;
  status: 'pending' | 'unlocked' | 'cancelled' | 'executed';
  canCancel: boolean;
}

export interface SpendingLimit {
  period: 'daily' | 'weekly' | 'monthly';
  limit: string;
  spent: string;
  remaining: string;
  resetsAt: Date;
}

export interface SecurityEvent {
  id: string;
  userId: string;
  type: 'whitelist_add' | 'whitelist_remove' | 'timelock_set' | 'panic_activated' |
        'panic_deactivated' | 'limit_exceeded' | 'guardian_added' | 'suspicious_transaction';
  description: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  timestamp: Date;
  metadata?: Record<string, any>;
}

export interface SecurityScore {
  overall: number; // 0-100
  breakdown: {
    biometrics: number;
    twoFactor: number;
    whitelist: number;
    timelock: number;
    spendingLimits: number;
    guardians: number;
  };
  recommendations: SecurityRecommendation[];
}

export interface SecurityRecommendation {
  id: string;
  title: string;
  description: string;
  impact: number; // Points gained if implemented
  priority: 'low' | 'medium' | 'high';
  implemented: boolean;
}

export interface GuardianApproval {
  guardian: string;
  approved: boolean;
  timestamp: Date;
}

export interface PanicModeStatus {
  active: boolean;
  activatedAt?: Date;
  activatedBy?: string;
  reason?: string;
  guardiansNotified: boolean;
  guardianApprovals: GuardianApproval[];
  canDeactivate: boolean;
}

export const DEFAULT_SECURITY_SETTINGS: Partial<SecuritySettings> = {
  whitelistEnabled: false,
  whitelistBypassAmount: '100', // $100
  timelockEnabled: false,
  timelockHours: 48,
  timelockExceptions: [],
  dailyLimit: '10000',
  weeklyLimit: '50000',
  monthlyLimit: '200000',
  perTransactionLimit: '10000',
  panicModeActive: false,
  guardians: [],
  guardianThreshold: 2,
  twoFactorEnabled: false,
  biometricsEnabled: false,
};
