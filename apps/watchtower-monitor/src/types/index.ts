export interface Channel {
  id: string;
  channelId: string;
  node1: string;
  node2: string;
  capacity: number;
  balance1: number;
  balance2: number;
  status: 'active' | 'inactive' | 'disputed' | 'closed';
  lastUpdate: Date;
  commitmentNumber: number;
  watchtowerActive: boolean;
}

export interface FraudAlert {
  id: string;
  channelId: string;
  type: 'old_state_broadcast' | 'double_spend' | 'invalid_signature' | 'unauthorized_close';
  severity: 'low' | 'medium' | 'high' | 'critical';
  timestamp: Date;
  description: string;
  evidenceHash: string;
  resolved: boolean;
  penaltyAmount?: number;
  reportedBy?: string;
}

export interface ReputationMetrics {
  watchtowerId: string;
  totalChannelsMonitored: number;
  fraudDetections: number;
  falsePositives: number;
  uptime: number;
  responseTime: number; // milliseconds
  successfulInterventions: number;
  score: number; // 0-100
  rank: number;
  earnings: number;
}

export interface Subscription {
  id: string;
  channelId: string;
  subscribedAt: Date;
  expiresAt: Date;
  fee: number;
  autoRenew: boolean;
  tier: 'basic' | 'premium' | 'enterprise';
  features: string[];
}

export interface Earnings {
  totalEarned: number;
  pendingRewards: number;
  lastPayout: Date;
  earningsHistory: EarningEntry[];
}

export interface EarningEntry {
  id: string;
  timestamp: Date;
  amount: number;
  type: 'subscription_fee' | 'fraud_detection_reward' | 'uptime_bonus';
  channelId?: string;
  description: string;
}

export interface MonitoringStats {
  activeChannels: number;
  totalMonitored: number;
  fraudsDetected: number;
  uptime: number;
  lastCheck: Date;
  averageResponseTime: number;
}

export interface WebSocketMessage {
  type: 'channel_update' | 'fraud_alert' | 'subscription_update' | 'earnings_update';
  payload: any;
  timestamp: Date;
}

export interface WatchtowerConfig {
  nodeEndpoint: string;
  wsEndpoint: string;
  pollingInterval: number;
  alertThresholds: {
    responseTime: number;
    balanceDeviation: number;
  };
  notifications: {
    email: boolean;
    push: boolean;
    webhook?: string;
  };
}
