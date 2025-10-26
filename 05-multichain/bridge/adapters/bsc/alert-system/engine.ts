import {
  AlertRule,
  AlertMetrics,
  AlertEvaluationResult,
  AlertNotification,
  AlertChannel,
} from "./types";
import { v4 as uuidv4 } from "uuid";

/**
 * Alert Rules Engine
 *
 * Evaluates metrics against alert rules and determines if alerts should be triggered
 */

export class AlertEngine {
  private rules: Map<string, AlertRule> = new Map();
  private historicalMetrics: AlertMetrics[] = [];

  constructor(rules: AlertRule[] = []) {
    rules.forEach((rule) => this.addRule(rule));
  }

  /**
   * Add or update a rule
   */
  addRule(rule: AlertRule) {
    this.rules.set(rule.id, rule);
  }

  /**
   * Remove a rule
   */
  removeRule(ruleId: string) {
    this.rules.delete(ruleId);
  }

  /**
   * Get all rules
   */
  getRules(): AlertRule[] {
    return Array.from(this.rules.values());
  }

  /**
   * Get enabled rules for a network
   */
  getEnabledRules(network: "mainnet" | "testnet"): AlertRule[] {
    return Array.from(this.rules.values()).filter(
      (rule) => rule.enabled && (rule.network === network || rule.network === "all")
    );
  }

  /**
   * Store historical metrics for comparison
   */
  recordMetrics(metrics: AlertMetrics) {
    this.historicalMetrics.push(metrics);

    // Keep only last 30 days
    const thirtyDaysAgo = Date.now() - 30 * 24 * 60 * 60 * 1000;
    this.historicalMetrics = this.historicalMetrics.filter(
      (m) => new Date(m.timestamp).getTime() > thirtyDaysAgo
    );
  }

  /**
   * Evaluate all rules against current metrics
   */
  evaluate(network: "mainnet" | "testnet", metrics: AlertMetrics): AlertEvaluationResult[] {
    const results: AlertEvaluationResult[] = [];
    const rules = this.getEnabledRules(network);

    for (const rule of rules) {
      // Check cooldown
      if (this.isInCooldown(rule)) {
        continue;
      }

      const result = this.evaluateRule(rule, metrics);
      if (result.triggered) {
        results.push(result);
      }
    }

    return results;
  }

  /**
   * Evaluate a single rule
   */
  private evaluateRule(rule: AlertRule, metrics: AlertMetrics): AlertEvaluationResult {
    const currentValue = this.getMetricValue(rule.metric, metrics);

    let triggered = false;
    let thresholdValue = rule.threshold;
    let message = "";
    let details: Record<string, any> = {
      metric: rule.metric,
      current: currentValue,
      threshold: rule.threshold,
    };

    // Handle percentage change conditions
    if (rule.condition === "pct_change") {
      const hoursAgo = rule.comparisonPeriod || 24;
      const previousValue = this.getHistoricalMetricValue(rule.metric, hoursAgo);

      if (previousValue !== null) {
        const pctChange = ((currentValue - previousValue) / previousValue) * 100;
        triggered = Math.abs(pctChange) >= rule.threshold;

        message = `${rule.name}: ${rule.metric} changed by ${pctChange.toFixed(2)}% in ${hoursAgo}h (${previousValue.toFixed(2)} → ${currentValue.toFixed(2)})`;

        details = {
          ...details,
          previous: previousValue,
          pct_change: pctChange,
          period_hours: hoursAgo,
        };
      }
    } else {
      // Standard comparisons
      switch (rule.condition) {
        case "gt":
          triggered = currentValue > rule.threshold;
          break;
        case "lt":
          triggered = currentValue < rule.threshold;
          break;
        case "gte":
          triggered = currentValue >= rule.threshold;
          break;
        case "lte":
          triggered = currentValue <= rule.threshold;
          break;
        case "eq":
          triggered = currentValue === rule.threshold;
          break;
      }

      if (triggered) {
        message = `${rule.name}: ${rule.metric} is ${currentValue.toFixed(2)} (${rule.condition} ${rule.threshold})`;
      }
    }

    return {
      triggered,
      rule,
      currentValue,
      thresholdValue,
      message,
      details,
    };
  }

  /**
   * Check if rule is in cooldown period
   */
  private isInCooldown(rule: AlertRule): boolean {
    if (!rule.lastTriggered) {
      return false;
    }

    const lastTriggeredTime = new Date(rule.lastTriggered).getTime();
    const cooldownMs = rule.cooldownMinutes * 60 * 1000;

    return Date.now() - lastTriggeredTime < cooldownMs;
  }

  /**
   * Get metric value from AlertMetrics object
   */
  private getMetricValue(metric: string, metrics: AlertMetrics): number {
    // Handle nested metrics (e.g., "pool_tvl.0")
    if (metric.includes(".")) {
      const [key, subkey] = metric.split(".");
      const obj = (metrics as any)[key];
      return obj?.[subkey] ?? 0;
    }

    return (metrics as any)[metric] ?? 0;
  }

  /**
   * Get historical metric value from N hours ago
   */
  private getHistoricalMetricValue(metric: string, hoursAgo: number): number | null {
    const targetTime = Date.now() - hoursAgo * 60 * 60 * 1000;

    // Find closest historical metric
    let closest: AlertMetrics | null = null;
    let closestDiff = Infinity;

    for (const m of this.historicalMetrics) {
      const time = new Date(m.timestamp).getTime();
      const diff = Math.abs(time - targetTime);

      if (diff < closestDiff) {
        closestDiff = diff;
        closest = m;
      }
    }

    if (!closest) {
      return null;
    }

    return this.getMetricValue(metric, closest);
  }

  /**
   * Create alert notification from evaluation result
   */
  createNotification(
    network: "mainnet" | "testnet",
    result: AlertEvaluationResult
  ): AlertNotification {
    const notification: AlertNotification = {
      id: uuidv4(),
      ruleId: result.rule.id,
      ruleName: result.rule.name,
      timestamp: new Date().toISOString(),
      network,
      severity: result.rule.severity,
      message: result.message,
      details: result.details,
      channels: result.rule.channels,
      sent: false,
    };

    return notification;
  }

  /**
   * Mark rule as triggered (for cooldown)
   */
  markRuleTriggered(ruleId: string) {
    const rule = this.rules.get(ruleId);
    if (rule) {
      rule.lastTriggered = new Date().toISOString();
      this.rules.set(ruleId, rule);
    }
  }

  /**
   * Check for escalations (alerts that haven't been acknowledged after X minutes)
   */
  checkEscalations(
    unacknowledgedAlerts: Array<{ ruleId: string; triggeredAt: string }>
  ): AlertNotification[] {
    const escalations: AlertNotification[] = [];

    for (const alert of unacknowledgedAlerts) {
      const rule = this.rules.get(alert.ruleId);

      if (!rule || !rule.escalateAfterMinutes || !rule.escalationChannels) {
        continue;
      }

      const triggeredTime = new Date(alert.triggeredAt).getTime();
      const escalationTime = rule.escalateAfterMinutes * 60 * 1000;

      if (Date.now() - triggeredTime >= escalationTime) {
        // Create escalation notification
        escalations.push({
          id: uuidv4(),
          ruleId: rule.id,
          ruleName: `[ESCALATED] ${rule.name}`,
          timestamp: new Date().toISOString(),
          network: rule.network === "all" ? "mainnet" : rule.network,
          severity: "critical",
          message: `ESCALATION: ${rule.name} has not been acknowledged for ${rule.escalateAfterMinutes} minutes`,
          details: { originalAlert: alert },
          channels: rule.escalationChannels,
          sent: false,
        });
      }
    }

    return escalations;
  }
}

/**
 * Predefined alert rules
 */
export const defaultRules: AlertRule[] = [
  {
    id: "low-balance",
    name: "Low MasterChef Balance",
    description: "MasterChef balance is below 1M ÉTR",
    enabled: true,
    network: "all",
    metric: "masterchef_balance",
    condition: "lt",
    threshold: 1000000,
    severity: "warning",
    channels: ["telegram", "discord"],
    cooldownMinutes: 60,
  },
  {
    id: "critical-balance",
    name: "Critical MasterChef Balance",
    description: "MasterChef balance is critically low",
    enabled: true,
    network: "all",
    metric: "masterchef_balance",
    condition: "lt",
    threshold: 500000,
    severity: "critical",
    channels: ["telegram", "discord", "email", "pagerduty"],
    cooldownMinutes: 30,
    escalateAfterMinutes: 60,
    escalationChannels: ["pagerduty", "email"],
  },
  {
    id: "tvl-drop",
    name: "TVL Drop",
    description: "Total TVL dropped more than 10% in 24 hours",
    enabled: true,
    network: "mainnet",
    metric: "total_tvl",
    condition: "pct_change",
    threshold: 10,
    comparisonPeriod: 24,
    severity: "warning",
    channels: ["telegram", "discord"],
    cooldownMinutes: 120,
  },
  {
    id: "days-remaining-low",
    name: "Low Days Remaining",
    description: "Less than 30 days of rewards remaining",
    enabled: true,
    network: "all",
    metric: "days_remaining",
    condition: "lt",
    threshold: 30,
    severity: "warning",
    channels: ["telegram", "email"],
    cooldownMinutes: 1440, // 24 hours
  },
  {
    id: "days-remaining-critical",
    name: "Critical Days Remaining",
    description: "Less than 7 days of rewards remaining",
    enabled: true,
    network: "all",
    metric: "days_remaining",
    condition: "lt",
    threshold: 7,
    severity: "critical",
    channels: ["telegram", "discord", "email", "pagerduty"],
    cooldownMinutes: 360, // 6 hours
  },
  {
    id: "contract-paused",
    name: "Contract Paused",
    description: "MasterChef contract has been paused",
    enabled: true,
    network: "all",
    metric: "is_paused",
    condition: "eq",
    threshold: 1,
    severity: "critical",
    channels: ["telegram", "discord", "email", "pagerduty", "slack"],
    cooldownMinutes: 15,
    escalateAfterMinutes: 30,
    escalationChannels: ["pagerduty"],
  },
  {
    id: "stale-data",
    name: "Stale Data",
    description: "Metrics haven't been updated in over 2 hours",
    enabled: true,
    network: "all",
    metric: "data_freshness_hours",
    condition: "gt",
    threshold: 2,
    severity: "warning",
    channels: ["telegram", "discord"],
    cooldownMinutes: 60,
  },
];
