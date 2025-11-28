import dotenv from "dotenv";
import { AlertEngine, defaultRules } from "./engine";
import { AlertChannels } from "./channels";
import { AlertMetrics, ChannelConfig } from "./types";
import { getLatestMetrics, getLatestPools, getActiveAlerts as getDBActiveAlerts, createAlert, getDatabaseStats } from "../scripts/lib/database";

dotenv.config();

/**
 * Advanced Alert Monitor
 *
 * Continuously monitors MasterChef metrics and triggers alerts based on custom rules
 *
 * Usage:
 *   npm run alert-monitor
 */

class AlertMonitor {
  private engine: AlertEngine;
  private channels: AlertChannels;
  private network: "mainnet" | "testnet";
  private checkIntervalMs: number;
  private running: boolean = false;

  constructor(network: "mainnet" | "testnet" = "mainnet", checkIntervalMs: number = 300000) {
    this.network = network;
    this.checkIntervalMs = checkIntervalMs;

    // Initialize engine with default rules
    this.engine = new AlertEngine(defaultRules);

    // Load custom rules from environment if provided
    this.loadCustomRules();

    // Initialize channels
    this.channels = new AlertChannels(this.getChannelConfig());
  }

  /**
   * Load custom rules from ALERT_RULES_JSON env variable
   */
  private loadCustomRules() {
    const rulesJson = process.env.ALERT_RULES_JSON;
    if (rulesJson) {
      try {
        const customRules = JSON.parse(rulesJson);
        customRules.forEach((rule: any) => this.engine.addRule(rule));
        console.log(`✅ Loaded ${customRules.length} custom alert rules`);
      } catch (error) {
        console.error("❌ Failed to parse ALERT_RULES_JSON:", error);
      }
    }
  }

  /**
   * Get channel configuration from environment
   */
  private getChannelConfig(): ChannelConfig {
    return {
      telegram: {
        enabled: !!process.env.TELEGRAM_BOT_TOKEN,
        botToken: process.env.TELEGRAM_BOT_TOKEN || "",
        chatIds: process.env.TELEGRAM_ADMIN_IDS?.split(",").map((id) => parseInt(id.trim())) || [],
      },
      discord: {
        enabled: !!process.env.DISCORD_WEBHOOK_URL,
        webhookUrl: process.env.DISCORD_WEBHOOK_URL || "",
      },
      email: {
        enabled: !!(process.env.SMTP_HOST && process.env.SMTP_USERNAME),
        smtpHost: process.env.SMTP_HOST || "",
        smtpPort: parseInt(process.env.SMTP_PORT || "587"),
        username: process.env.SMTP_USERNAME || "",
        password: process.env.SMTP_PASSWORD || "",
        from: process.env.SMTP_FROM || process.env.SMTP_USERNAME || "",
        to: process.env.ALERT_EMAIL_TO?.split(",") || [],
      },
      slack: {
        enabled: !!process.env.SLACK_WEBHOOK_URL,
        webhookUrl: process.env.SLACK_WEBHOOK_URL || "",
      },
      pagerduty: {
        enabled: !!process.env.PAGERDUTY_API_KEY,
        apiKey: process.env.PAGERDUTY_API_KEY || "",
        serviceId: process.env.PAGERDUTY_SERVICE_ID || "",
      },
    };
  }

  /**
   * Collect current metrics
   */
  private async collectMetrics(): Promise<AlertMetrics> {
    const dbMetrics = getLatestMetrics(this.network);
    const pools = getLatestPools(this.network);

    if (!dbMetrics) {
      throw new Error(`No metrics found for ${this.network}`);
    }

    // Calculate data freshness
    const lastUpdate = new Date(dbMetrics.timestamp);
    const dataFreshnessHours = (Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60);

    // Build pool metrics maps
    const poolTVL: Record<number, number> = {};
    const poolAPR: Record<number, number> = {};
    const poolStaked: Record<number, number> = {};

    for (const pool of pools) {
      poolTVL[pool.pool_id] = pool.tvl_usd || 0;
      poolAPR[pool.pool_id] = pool.apr_percent || 0;
      poolStaked[pool.pool_id] = parseFloat(pool.total_staked);
    }

    const metrics: AlertMetrics = {
      masterchef_balance: parseFloat(dbMetrics.masterchef_balance),
      total_tvl: dbMetrics.total_tvl_usd || 0,
      total_pools: dbMetrics.total_pools,
      reward_per_block: parseFloat(dbMetrics.reward_per_block),
      days_remaining: dbMetrics.days_remaining,
      is_paused: dbMetrics.is_paused,
      pool_tvl: poolTVL,
      pool_apr: poolAPR,
      pool_staked: poolStaked,
      bnb_price: dbMetrics.bnb_price,
      etr_price: dbMetrics.etr_price,
      data_freshness_hours: dataFreshnessHours,
      timestamp: dbMetrics.timestamp,
    };

    return metrics;
  }

  /**
   * Run one check cycle
   */
  private async runCheck() {
    try {
      console.log(`\n🔍 Running alert check for ${this.network}...`);

      // Collect metrics
      const metrics = await this.collectMetrics();

      // Store for historical comparison
      this.engine.recordMetrics(metrics);

      // Evaluate rules
      const results = this.engine.evaluate(this.network, metrics);

      if (results.length === 0) {
        console.log("✅ No alerts triggered");
        return;
      }

      console.log(`🚨 ${results.length} alert(s) triggered:`);

      // Send notifications and store in database
      for (const result of results) {
        console.log(`   - ${result.rule.name}: ${result.message}`);

        const notification = this.engine.createNotification(this.network, result);

        // Send to channels
        try {
          await this.channels.send(notification);
          notification.sent = true;
          notification.sentAt = new Date().toISOString();
          console.log(`     ✅ Sent to: ${notification.channels.join(", ")}`);
        } catch (error: any) {
          notification.error = error.message;
          console.log(`     ❌ Failed to send: ${error.message}`);
        }

        // Store in database
        createAlert({
          network: this.network,
          severity: result.rule.severity,
          alert_type: result.rule.id,
          message: result.message,
        });

        // Mark rule as triggered (for cooldown)
        this.engine.markRuleTriggered(result.rule.id);
      }

      // Check for escalations
      await this.checkEscalations();
    } catch (error) {
      console.error("❌ Alert check failed:", error);
    }
  }

  /**
   * Check for alerts that need escalation
   */
  private async checkEscalations() {
    const activeAlerts = getDBActiveAlerts(this.network);

    if (activeAlerts.length === 0) {
      return;
    }

    const unacknowledged = activeAlerts
      .filter((alert) => !alert.acknowledged)
      .map((alert) => ({
        ruleId: alert.alert_type,
        triggeredAt: alert.timestamp,
      }));

    const escalations = this.engine.checkEscalations(unacknowledged);

    if (escalations.length > 0) {
      console.log(`⚠️  ${escalations.length} alert(s) need escalation:`);

      for (const escalation of escalations) {
        console.log(`   - ${escalation.ruleName}`);

        try {
          await this.channels.send(escalation);
          console.log(`     ✅ Escalation sent to: ${escalation.channels.join(", ")}`);
        } catch (error: any) {
          console.log(`     ❌ Failed to send escalation: ${error.message}`);
        }

        // Store escalation in database
        createAlert({
          network: this.network,
          severity: "critical",
          alert_type: "escalation",
          message: escalation.message,
        });
      }
    }
  }

  /**
   * Start monitoring
   */
  start() {
    if (this.running) {
      console.log("⚠️  Monitor is already running");
      return;
    }

    console.log("\n🚀 ADVANCED ALERT MONITOR STARTED\n");
    console.log(`   Network: ${this.network}`);
    console.log(`   Check Interval: ${this.checkIntervalMs / 1000 / 60} minutes`);

    // Show configured channels
    const config = this.getChannelConfig();
    console.log("\n   Configured Channels:");
    if (config.telegram?.enabled) console.log("     ✅ Telegram");
    if (config.discord?.enabled) console.log("     ✅ Discord");
    if (config.email?.enabled) console.log("     ✅ Email");
    if (config.slack?.enabled) console.log("     ✅ Slack");
    if (config.pagerduty?.enabled) console.log("     ✅ PagerDuty");

    // Show active rules
    const rules = this.engine.getEnabledRules(this.network);
    console.log(`\n   Active Rules: ${rules.length}`);
    rules.forEach((rule) => {
      console.log(`     - ${rule.name} (${rule.severity})`);
    });

    console.log("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    this.running = true;

    // Run immediately
    this.runCheck();

    // Schedule periodic checks
    const intervalId = setInterval(() => {
      this.runCheck();
    }, this.checkIntervalMs);

    // Handle shutdown
    process.on("SIGINT", () => {
      console.log("\n\n📦 Shutting down alert monitor...");
      clearInterval(intervalId);
      this.running = false;
      process.exit(0);
    });

    process.on("SIGTERM", () => {
      console.log("\n\n📦 Shutting down alert monitor...");
      clearInterval(intervalId);
      this.running = false;
      process.exit(0);
    });
  }

  /**
   * Show monitor status
   */
  showStatus() {
    console.log("\n📊 ALERT MONITOR STATUS\n");
    console.log(`   Running: ${this.running ? "Yes" : "No"}`);
    console.log(`   Network: ${this.network}`);
    console.log(`   Check Interval: ${this.checkIntervalMs / 1000 / 60} minutes`);

    const rules = this.engine.getRules();
    const enabledRules = this.engine.getEnabledRules(this.network);

    console.log(`\n   Total Rules: ${rules.length}`);
    console.log(`   Enabled Rules: ${enabledRules.length}`);

    const dbStats = getDatabaseStats();
    console.log(`\n   Database:`);
    console.log(`     Alerts: ${dbStats.alerts}`);
    console.log(`     Metrics Snapshots: ${dbStats.metrics_snapshots}`);

    console.log();
  }
}

// CLI Entry Point
async function main() {
  const network = (process.env.DEFAULT_NETWORK as "mainnet" | "testnet") || "mainnet";
  const checkInterval = parseInt(process.env.ALERT_CHECK_INTERVAL || "300000");

  const monitor = new AlertMonitor(network, checkInterval);

  // Check if --status flag
  if (process.argv.includes("--status")) {
    monitor.showStatus();
    process.exit(0);
  }

  monitor.start();
}

main().catch((error) => {
  console.error("\n❌ Fatal error:");
  console.error(error);
  process.exit(1);
});
