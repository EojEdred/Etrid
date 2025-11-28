import TelegramBot from "node-telegram-bot-api";
import { ethers } from "ethers";
import { config, validateConfig } from "./config";
import { getLatestMetrics, getLatestPools, getActiveAlerts, acknowledgeAlert } from "../scripts/lib/database";
import { getBNBPriceUSD, getTokenPriceUSD } from "../scripts/lib/priceFeeds";

/**
 * MasterChef Telegram Bot
 *
 * Interactive bot for monitoring MasterChef metrics
 *
 * Commands:
 *  /start - Welcome message and help
 *  /help - Show all commands
 *  /tvl [network] - Show Total Value Locked
 *  /apr [network] [poolId] - Show APR for pool
 *  /pools [network] - List all pools
 *  /balance [network] - Show MasterChef balance
 *  /health [network] - Health check
 *  /alerts - Show active alerts
 *  /prices - Show BNB and ÉTR prices
 *  /stats [network] - Show all statistics
 *
 * Admin Commands:
 *  /ack <alertId> - Acknowledge alert
 *  /broadcast <message> - Send message to all users
 */

export class MasterChefBot {
  private bot: TelegramBot;
  private subscribedUsers: Set<number> = new Set();

  constructor() {
    const validation = validateConfig();
    if (!validation.valid) {
      console.error("❌ Configuration errors:");
      validation.errors.forEach((err) => console.error(`   - ${err}`));
      throw new Error("Invalid bot configuration");
    }

    this.bot = new TelegramBot(config.botToken, { polling: true });
    this.setupCommands();
    this.startAlertMonitoring();
  }

  private setupCommands() {
    // Welcome message
    this.bot.onText(/\/start/, (msg) => this.handleStart(msg));

    // Help
    this.bot.onText(/\/help/, (msg) => this.handleHelp(msg));

    // TVL
    this.bot.onText(/\/tvl(?:\s+(\w+))?/, (msg, match) => this.handleTVL(msg, match?.[1]));

    // APR
    this.bot.onText(/\/apr(?:\s+(\w+))?(?:\s+(\d+))?/, (msg, match) =>
      this.handleAPR(msg, match?.[1], match?.[2])
    );

    // Pools
    this.bot.onText(/\/pools(?:\s+(\w+))?/, (msg, match) => this.handlePools(msg, match?.[1]));

    // Balance
    this.bot.onText(/\/balance(?:\s+(\w+))?/, (msg, match) => this.handleBalance(msg, match?.[1]));

    // Health
    this.bot.onText(/\/health(?:\s+(\w+))?/, (msg, match) => this.handleHealth(msg, match?.[1]));

    // Alerts
    this.bot.onText(/\/alerts/, (msg) => this.handleAlerts(msg));

    // Prices
    this.bot.onText(/\/prices/, (msg) => this.handlePrices(msg));

    // Stats
    this.bot.onText(/\/stats(?:\s+(\w+))?/, (msg, match) => this.handleStats(msg, match?.[1]));

    // Admin: Acknowledge alert
    this.bot.onText(/\/ack\s+(\d+)/, (msg, match) => this.handleAcknowledge(msg, match?.[1]));

    // Admin: Broadcast
    this.bot.onText(/\/broadcast\s+(.+)/, (msg, match) => this.handleBroadcast(msg, match?.[1]));

    // Subscribe to alerts
    this.bot.onText(/\/subscribe/, (msg) => this.handleSubscribe(msg));

    // Unsubscribe from alerts
    this.bot.onText(/\/unsubscribe/, (msg) => this.handleUnsubscribe(msg));

    console.log("✅ Telegram bot commands registered");
  }

  private async handleStart(msg: TelegramBot.Message) {
    const chatId = msg.chat.id;

    const message = `
🤖 *Welcome to MasterChef Monitor Bot!*

I can help you monitor your MasterChef LP rewards program on Binance Smart Chain.

*Available Commands:*
/help - Show all commands
/tvl - Show Total Value Locked
/pools - List all pools
/balance - MasterChef balance
/health - System health check
/alerts - Active alerts
/prices - Token prices
/stats - All statistics

/subscribe - Get alert notifications
/unsubscribe - Stop alert notifications

_Use /help for detailed command usage._
    `.trim();

    this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
  }

  private async handleHelp(msg: TelegramBot.Message) {
    const chatId = msg.chat.id;
    const isAdmin = config.adminIds.includes(msg.from?.id || 0);

    let message = `
📖 *MasterChef Bot Commands*

*Monitoring:*
/tvl [network] - Total Value Locked
/apr [network] [poolId] - APR for specific pool
/pools [network] - List all pools with stats
/balance [network] - MasterChef ÉTR balance
/health [network] - Health check
/prices - BNB and ÉTR prices
/stats [network] - Complete statistics

*Alerts:*
/alerts - Show active alerts
/subscribe - Get alert notifications
/unsubscribe - Stop notifications

*Parameters:*
- network: mainnet or testnet (default: ${config.defaultNetwork})
- poolId: Pool index (0, 1, 2, etc.)

*Examples:*
/tvl mainnet
/apr mainnet 0
/pools testnet
    `.trim();

    if (isAdmin) {
      message += `\n\n*Admin Commands:*\n/ack <alertId> - Acknowledge alert\n/broadcast <msg> - Send to all subscribers`;
    }

    this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
  }

  private async handleTVL(msg: TelegramBot.Message, network?: string) {
    const chatId = msg.chat.id;
    const net = network || config.defaultNetwork;

    try {
      const metrics = getLatestMetrics(net);

      if (!metrics) {
        this.bot.sendMessage(chatId, `❌ No metrics found for ${net}`);
        return;
      }

      let message = `📊 *Total Value Locked*\n\n`;
      message += `*Network:* ${net === "mainnet" ? "BSC Mainnet" : "BSC Testnet"}\n`;
      message += `*Block:* ${metrics.block_number.toLocaleString()}\n`;

      if (metrics.total_tvl_usd) {
        message += `\n💰 *Total TVL:* $${metrics.total_tvl_usd.toLocaleString()}\n`;
        message += `*Total Staked:* ${parseFloat(metrics.total_staked_lp).toLocaleString()} LP\n`;
      } else {
        message += `\n*Total Staked:* ${parseFloat(metrics.total_staked_lp).toLocaleString()} LP\n`;
        message += `_TVL in USD not available on testnet_\n`;
      }

      message += `\n*Last Updated:* ${new Date(metrics.timestamp).toLocaleString()}`;

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handleAPR(msg: TelegramBot.Message, network?: string, poolIdStr?: string) {
    const chatId = msg.chat.id;
    const net = network || config.defaultNetwork;

    try {
      const pools = getLatestPools(net);

      if (pools.length === 0) {
        this.bot.sendMessage(chatId, `❌ No pools found for ${net}`);
        return;
      }

      // If no pool ID, show all
      if (!poolIdStr) {
        let message = `📈 *APR by Pool*\n\n*Network:* ${net}\n\n`;

        for (const pool of pools) {
          message += `*Pool ${pool.pool_id}:* ${pool.lp_symbol}\n`;
          if (pool.apr_percent) {
            message += `  APR: ${pool.apr_percent.toFixed(2)}%\n`;
          } else {
            message += `  APR: N/A (testnet)\n`;
          }
        }

        this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
        return;
      }

      // Show specific pool
      const poolId = parseInt(poolIdStr);
      const pool = pools.find((p) => p.pool_id === poolId);

      if (!pool) {
        this.bot.sendMessage(chatId, `❌ Pool ${poolId} not found`);
        return;
      }

      let message = `📈 *Pool ${poolId} APR*\n\n`;
      message += `*Symbol:* ${pool.lp_symbol}\n`;
      message += `*Name:* ${pool.lp_name}\n`;

      if (pool.apr_percent) {
        message += `\n💹 *APR:* ${pool.apr_percent.toFixed(2)}%\n`;
        message += `*TVL:* $${pool.tvl_usd?.toLocaleString()}\n`;
      } else {
        message += `\n_APR not available on testnet_\n`;
      }

      message += `\n*Staked:* ${parseFloat(pool.total_staked).toLocaleString()} LP\n`;
      message += `*Reward Share:* ${pool.reward_share.toFixed(1)}%\n`;
      message += `*Daily Rewards:* ${parseFloat(pool.daily_rewards).toFixed(2)} ÉTR\n`;
      message += `*Monthly Rewards:* ${parseFloat(pool.monthly_rewards).toFixed(2)} ÉTR`;

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handlePools(msg: TelegramBot.Message, network?: string) {
    const chatId = msg.chat.id;
    const net = network || config.defaultNetwork;

    try {
      const pools = getLatestPools(net);

      if (pools.length === 0) {
        this.bot.sendMessage(chatId, `❌ No pools found for ${net}`);
        return;
      }

      let message = `🏊 *Liquidity Pools*\n\n*Network:* ${net}\n*Total Pools:* ${pools.length}\n\n`;

      for (const pool of pools) {
        message += `*Pool ${pool.pool_id}:* ${pool.lp_symbol}\n`;
        message += `  Staked: ${parseFloat(pool.total_staked).toLocaleString()} LP\n`;

        if (pool.tvl_usd && pool.apr_percent) {
          message += `  TVL: $${pool.tvl_usd.toLocaleString()}\n`;
          message += `  APR: ${pool.apr_percent.toFixed(2)}%\n`;
        }

        message += `  Share: ${pool.reward_share.toFixed(1)}%\n\n`;
      }

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handleBalance(msg: TelegramBot.Message, network?: string) {
    const chatId = msg.chat.id;
    const net = network || config.defaultNetwork;

    try {
      const metrics = getLatestMetrics(net);

      if (!metrics) {
        this.bot.sendMessage(chatId, `❌ No metrics found for ${net}`);
        return;
      }

      const balance = parseFloat(metrics.masterchef_balance);
      const rewardPerBlock = parseFloat(metrics.reward_per_block);
      const daysRemaining = metrics.days_remaining;

      let message = `💰 *MasterChef Balance*\n\n`;
      message += `*Network:* ${net}\n`;
      message += `*Balance:* ${balance.toLocaleString()} ÉTR\n`;
      message += `*Emission:* ${rewardPerBlock.toFixed(2)} ÉTR/block\n`;
      message += `*Days Remaining:* ${daysRemaining} days\n`;

      if (daysRemaining < 30) {
        message += `\n⚠️ *WARNING:* Less than 30 days of rewards remaining!`;
      } else if (daysRemaining < 60) {
        message += `\n⚡ Running low - consider topping up soon`;
      }

      message += `\n\n*Last Updated:* ${new Date(metrics.timestamp).toLocaleString()}`;

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handleHealth(msg: TelegramBot.Message, network?: string) {
    const chatId = msg.chat.id;
    const net = network || config.defaultNetwork;

    try {
      const metrics = getLatestMetrics(net);
      const pools = getLatestPools(net);

      if (!metrics) {
        this.bot.sendMessage(chatId, `❌ No metrics found for ${net}`);
        return;
      }

      let message = `🏥 *Health Check*\n\n*Network:* ${net}\n\n`;

      // Check if paused
      if (metrics.is_paused) {
        message += `🔴 *Status:* PAUSED\n`;
      } else {
        message += `🟢 *Status:* Active\n`;
      }

      // Check balance
      const balance = parseFloat(metrics.masterchef_balance);
      if (balance < config.criticalBalanceThreshold) {
        message += `🔴 *Balance:* CRITICAL (${balance.toLocaleString()} ÉTR)\n`;
      } else if (balance < config.lowBalanceThreshold) {
        message += `🟡 *Balance:* LOW (${balance.toLocaleString()} ÉTR)\n`;
      } else {
        message += `🟢 *Balance:* OK (${balance.toLocaleString()} ÉTR)\n`;
      }

      // Check pools
      message += `🟢 *Pools:* ${pools.length} active\n`;

      // Check recent data
      const lastUpdate = new Date(metrics.timestamp);
      const hoursSinceUpdate = (Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60);

      if (hoursSinceUpdate > 2) {
        message += `🟡 *Data:* Stale (${Math.floor(hoursSinceUpdate)}h ago)\n`;
      } else {
        message += `🟢 *Data:* Fresh\n`;
      }

      message += `\n*Last Updated:* ${lastUpdate.toLocaleString()}`;

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handleAlerts(msg: TelegramBot.Message) {
    const chatId = msg.chat.id;

    try {
      const alerts = getActiveAlerts();

      if (alerts.length === 0) {
        this.bot.sendMessage(chatId, "✅ No active alerts!");
        return;
      }

      let message = `🚨 *Active Alerts* (${alerts.length})\n\n`;

      for (const alert of alerts) {
        const icon = alert.severity === "critical" ? "🔴" : alert.severity === "warning" ? "🟡" : "ℹ️";

        message += `${icon} *Alert #${alert.id}*\n`;
        message += `*Network:* ${alert.network}\n`;
        message += `*Type:* ${alert.alert_type}\n`;
        message += `*Message:* ${alert.message}\n`;
        message += `*Time:* ${new Date(alert.timestamp).toLocaleString()}\n\n`;
      }

      message += `_Use /ack <id> to acknowledge (admin only)_`;

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handlePrices(msg: TelegramBot.Message) {
    const chatId = msg.chat.id;

    try {
      this.bot.sendMessage(chatId, "⏳ Fetching prices...");

      const provider = new ethers.JsonRpcProvider(config.bscMainnetRpc);

      const bnbPrice = await getBNBPriceUSD(provider);
      const etrPrice = await getTokenPriceUSD(config.etrTokenMainnet, provider);

      let message = `💵 *Current Prices*\n\n`;
      message += `*BNB:* $${bnbPrice.toFixed(2)}\n`;
      message += `*ÉTR:* $${etrPrice.toFixed(6)}\n`;

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error fetching prices: ${error.message}`);
    }
  }

  private async handleStats(msg: TelegramBot.Message, network?: string) {
    const chatId = msg.chat.id;
    const net = network || config.defaultNetwork;

    try {
      const metrics = getLatestMetrics(net);
      const pools = getLatestPools(net);

      if (!metrics) {
        this.bot.sendMessage(chatId, `❌ No metrics found for ${net}`);
        return;
      }

      let message = `📊 *Complete Statistics*\n\n`;
      message += `*Network:* ${net === "mainnet" ? "BSC Mainnet" : "BSC Testnet"}\n`;
      message += `*Block:* ${metrics.block_number.toLocaleString()}\n\n`;

      message += `*MasterChef:*\n`;
      message += `  Balance: ${parseFloat(metrics.masterchef_balance).toLocaleString()} ÉTR\n`;
      message += `  Emission: ${parseFloat(metrics.reward_per_block).toFixed(2)} ÉTR/block\n`;
      message += `  Days Left: ${metrics.days_remaining} days\n`;
      message += `  Status: ${metrics.is_paused ? "Paused" : "Active"}\n\n`;

      if (metrics.total_tvl_usd) {
        message += `*TVL:*\n`;
        message += `  Total: $${metrics.total_tvl_usd.toLocaleString()}\n`;
        message += `  Staked: ${parseFloat(metrics.total_staked_lp).toLocaleString()} LP\n\n`;
      }

      message += `*Pools:* ${pools.length} active\n\n`;

      if (metrics.bnb_price && metrics.etr_price) {
        message += `*Prices:*\n`;
        message += `  BNB: $${metrics.bnb_price.toFixed(2)}\n`;
        message += `  ÉTR: $${metrics.etr_price.toFixed(6)}\n\n`;
      }

      message += `*Updated:* ${new Date(metrics.timestamp).toLocaleString()}`;

      this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handleSubscribe(msg: TelegramBot.Message) {
    const chatId = msg.chat.id;

    this.subscribedUsers.add(chatId);
    this.bot.sendMessage(chatId, "✅ Subscribed to alerts! You'll receive notifications when issues are detected.");
  }

  private async handleUnsubscribe(msg: TelegramBot.Message) {
    const chatId = msg.chat.id;

    this.subscribedUsers.delete(chatId);
    this.bot.sendMessage(chatId, "✅ Unsubscribed from alerts.");
  }

  private async handleAcknowledge(msg: TelegramBot.Message, alertIdStr?: string) {
    const chatId = msg.chat.id;
    const userId = msg.from?.id || 0;

    // Check admin
    if (!config.adminIds.includes(userId)) {
      this.bot.sendMessage(chatId, "❌ Admin only command");
      return;
    }

    if (!alertIdStr) {
      this.bot.sendMessage(chatId, "❌ Usage: /ack <alertId>");
      return;
    }

    try {
      const alertId = parseInt(alertIdStr);
      const username = msg.from?.username || msg.from?.first_name || "Unknown";

      acknowledgeAlert(alertId, username);

      this.bot.sendMessage(chatId, `✅ Alert #${alertId} acknowledged by ${username}`);
    } catch (error: any) {
      this.bot.sendMessage(chatId, `❌ Error: ${error.message}`);
    }
  }

  private async handleBroadcast(msg: TelegramBot.Message, message?: string) {
    const chatId = msg.chat.id;
    const userId = msg.from?.id || 0;

    // Check admin
    if (!config.adminIds.includes(userId)) {
      this.bot.sendMessage(chatId, "❌ Admin only command");
      return;
    }

    if (!message) {
      this.bot.sendMessage(chatId, "❌ Usage: /broadcast <message>");
      return;
    }

    let sent = 0;
    for (const subscribedChatId of this.subscribedUsers) {
      try {
        await this.bot.sendMessage(subscribedChatId, `📢 *Broadcast*\n\n${message}`, {
          parse_mode: "Markdown",
        });
        sent++;
      } catch (error) {
        console.error(`Failed to send to ${subscribedChatId}:`, error);
      }
    }

    this.bot.sendMessage(chatId, `✅ Broadcast sent to ${sent} subscriber(s)`);
  }

  private startAlertMonitoring() {
    setInterval(() => {
      this.checkAndSendAlerts();
    }, config.alertCheckInterval);

    console.log(
      `✅ Alert monitoring started (checking every ${config.alertCheckInterval / 1000 / 60} minutes)`
    );
  }

  private async checkAndSendAlerts() {
    try {
      const alerts = getActiveAlerts();

      if (alerts.length === 0) return;

      // Send to all subscribed users
      for (const chatId of this.subscribedUsers) {
        for (const alert of alerts) {
          const icon = alert.severity === "critical" ? "🔴" : alert.severity === "warning" ? "🟡" : "ℹ️";

          const message = `
${icon} *New Alert*

*Network:* ${alert.network}
*Severity:* ${alert.severity.toUpperCase()}
*Type:* ${alert.alert_type}

${alert.message}

_Use /alerts to see all active alerts_
          `.trim();

          try {
            await this.bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
          } catch (error) {
            console.error(`Failed to send alert to ${chatId}:`, error);
          }
        }
      }
    } catch (error) {
      console.error("Error checking alerts:", error);
    }
  }

  public start() {
    console.log("\n🤖 MasterChef Telegram Bot Started\n");
    console.log(`   Bot Token: ${config.botToken.substring(0, 10)}...`);
    console.log(`   Admins: ${config.adminIds.join(", ")}`);
    console.log(`   Default Network: ${config.defaultNetwork}`);
    console.log();
  }

  public stop() {
    this.bot.stopPolling();
    console.log("\n🛑 Telegram bot stopped\n");
  }
}
