/**
 * Advanced Alert System - Type Definitions
 */

export type AlertSeverity = "info" | "warning" | "critical";
export type AlertChannel = "telegram" | "discord" | "email" | "slack" | "pagerduty" | "console";
export type AlertCondition = "gt" | "lt" | "eq" | "gte" | "lte" | "pct_change";

export interface AlertRule {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  network: "mainnet" | "testnet" | "all";

  // Condition
  metric: string; // e.g., "masterchef_balance", "total_tvl", "pool_apr"
  condition: AlertCondition;
  threshold: number;
  comparisonPeriod?: number; // For pct_change (in hours)

  // Severity
  severity: AlertSeverity;

  // Channels to notify
  channels: AlertChannel[];

  // Cooldown (prevent spam)
  cooldownMinutes: number;
  lastTriggered?: string;

  // Escalation
  escalateAfterMinutes?: number;
  escalationChannels?: AlertChannel[];
}

export interface AlertNotification {
  id: string;
  ruleId: string;
  ruleName: string;
  timestamp: string;
  network: string;
  severity: AlertSeverity;
  message: string;
  details: Record<string, any>;
  channels: AlertChannel[];
  sent: boolean;
  sentAt?: string;
  error?: string;
}

export interface ChannelConfig {
  telegram?: {
    enabled: boolean;
    botToken: string;
    chatIds: number[];
  };
  discord?: {
    enabled: boolean;
    webhookUrl: string;
  };
  email?: {
    enabled: boolean;
    smtpHost: string;
    smtpPort: number;
    username: string;
    password: string;
    from: string;
    to: string[];
  };
  slack?: {
    enabled: boolean;
    webhookUrl: string;
  };
  pagerduty?: {
    enabled: boolean;
    apiKey: string;
    serviceId: string;
  };
}

export interface AlertMetrics {
  // Current metrics
  masterchef_balance: number;
  total_tvl: number;
  total_pools: number;
  reward_per_block: number;
  days_remaining: number;
  is_paused: boolean;

  // Pool-specific
  pool_tvl?: Record<number, number>;
  pool_apr?: Record<number, number>;
  pool_staked?: Record<number, number>;

  // Prices
  bnb_price?: number;
  etr_price?: number;

  // Health
  data_freshness_hours: number;
}

export interface AlertEvaluationResult {
  triggered: boolean;
  rule: AlertRule;
  currentValue: number;
  thresholdValue: number;
  message: string;
  details: Record<string, any>;
}
