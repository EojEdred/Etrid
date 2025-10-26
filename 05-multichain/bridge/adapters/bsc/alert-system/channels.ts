import TelegramBot from "node-telegram-bot-api";
import { AlertNotification, ChannelConfig } from "./types";

/**
 * Multi-Channel Alert Sender
 *
 * Sends alerts to Telegram, Discord, Email, Slack, and PagerDuty
 */

export class AlertChannels {
  private config: ChannelConfig;

  constructor(config: ChannelConfig) {
    this.config = config;
  }

  /**
   * Send notification to all configured channels
   */
  async send(notification: AlertNotification): Promise<void> {
    const results: Array<Promise<void>> = [];

    for (const channel of notification.channels) {
      switch (channel) {
        case "telegram":
          if (this.config.telegram?.enabled) {
            results.push(this.sendToTelegram(notification));
          }
          break;

        case "discord":
          if (this.config.discord?.enabled) {
            results.push(this.sendToDiscord(notification));
          }
          break;

        case "email":
          if (this.config.email?.enabled) {
            results.push(this.sendToEmail(notification));
          }
          break;

        case "slack":
          if (this.config.slack?.enabled) {
            results.push(this.sendToSlack(notification));
          }
          break;

        case "pagerduty":
          if (this.config.pagerduty?.enabled) {
            results.push(this.sendToPagerDuty(notification));
          }
          break;

        case "console":
          this.sendToConsole(notification);
          break;
      }
    }

    await Promise.allSettled(results);
  }

  /**
   * Send to Telegram
   */
  private async sendToTelegram(notification: AlertNotification): Promise<void> {
    if (!this.config.telegram) return;

    const bot = new TelegramBot(this.config.telegram.botToken);
    const icon = this.getSeverityIcon(notification.severity);

    const message = `
${icon} *${notification.ruleName}*

*Network:* ${notification.network}
*Severity:* ${notification.severity.toUpperCase()}

${notification.message}

_${new Date(notification.timestamp).toLocaleString()}_
    `.trim();

    for (const chatId of this.config.telegram.chatIds) {
      try {
        await bot.sendMessage(chatId, message, { parse_mode: "Markdown" });
      } catch (error) {
        console.error(`Failed to send to Telegram chat ${chatId}:`, error);
      }
    }
  }

  /**
   * Send to Discord
   */
  private async sendToDiscord(notification: AlertNotification): Promise<void> {
    if (!this.config.discord) return;

    const color = this.getSeverityColor(notification.severity);
    const icon = this.getSeverityIcon(notification.severity);

    const payload = {
      embeds: [
        {
          title: `${icon} ${notification.ruleName}`,
          description: notification.message,
          color: color,
          fields: [
            {
              name: "Network",
              value: notification.network,
              inline: true,
            },
            {
              name: "Severity",
              value: notification.severity.toUpperCase(),
              inline: true,
            },
            {
              name: "Details",
              value: `\`\`\`json\n${JSON.stringify(notification.details, null, 2)}\n\`\`\``,
            },
          ],
          timestamp: notification.timestamp,
          footer: {
            text: "MasterChef Alert System",
          },
        },
      ],
    };

    try {
      const response = await fetch(this.config.discord.webhookUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(`Discord returned ${response.status}`);
      }
    } catch (error) {
      console.error("Failed to send to Discord:", error);
      throw error;
    }
  }

  /**
   * Send to Email
   */
  private async sendToEmail(notification: AlertNotification): Promise<void> {
    if (!this.config.email) return;

    const nodemailer = require("nodemailer");

    const transporter = nodemailer.createTransport({
      host: this.config.email.smtpHost,
      port: this.config.email.smtpPort,
      secure: this.config.email.smtpPort === 465,
      auth: {
        user: this.config.email.username,
        pass: this.config.email.password,
      },
    });

    const icon = this.getSeverityIcon(notification.severity);
    const subject = `${icon} ${notification.severity.toUpperCase()}: ${notification.ruleName}`;

    const html = `
<!DOCTYPE html>
<html>
<head>
  <style>
    body { font-family: Arial, sans-serif; }
    .header { background-color: ${this.getSeverityColorHex(notification.severity)}; color: white; padding: 20px; }
    .content { padding: 20px; }
    .details { background-color: #f5f5f5; padding: 15px; margin-top: 20px; }
    .footer { color: #666; font-size: 12px; margin-top: 30px; }
  </style>
</head>
<body>
  <div class="header">
    <h1>${icon} ${notification.ruleName}</h1>
  </div>
  <div class="content">
    <p><strong>Network:</strong> ${notification.network}</p>
    <p><strong>Severity:</strong> ${notification.severity.toUpperCase()}</p>
    <p><strong>Message:</strong></p>
    <p>${notification.message}</p>

    <div class="details">
      <h3>Details</h3>
      <pre>${JSON.stringify(notification.details, null, 2)}</pre>
    </div>

    <div class="footer">
      <p>Triggered at: ${new Date(notification.timestamp).toLocaleString()}</p>
      <p>MasterChef Alert System</p>
    </div>
  </div>
</body>
</html>
    `.trim();

    try {
      await transporter.sendMail({
        from: this.config.email.from,
        to: this.config.email.to.join(", "),
        subject,
        html,
      });
    } catch (error) {
      console.error("Failed to send email:", error);
      throw error;
    }
  }

  /**
   * Send to Slack
   */
  private async sendToSlack(notification: AlertNotification): Promise<void> {
    if (!this.config.slack) return;

    const icon = this.getSeverityIcon(notification.severity);
    const color = this.getSeverityColorHex(notification.severity);

    const payload = {
      attachments: [
        {
          color: color,
          title: `${icon} ${notification.ruleName}`,
          text: notification.message,
          fields: [
            {
              title: "Network",
              value: notification.network,
              short: true,
            },
            {
              title: "Severity",
              value: notification.severity.toUpperCase(),
              short: true,
            },
            {
              title: "Details",
              value: `\`\`\`${JSON.stringify(notification.details, null, 2)}\`\`\``,
            },
          ],
          footer: "MasterChef Alert System",
          ts: Math.floor(new Date(notification.timestamp).getTime() / 1000),
        },
      ],
    };

    try {
      const response = await fetch(this.config.slack.webhookUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(`Slack returned ${response.status}`);
      }
    } catch (error) {
      console.error("Failed to send to Slack:", error);
      throw error;
    }
  }

  /**
   * Send to PagerDuty
   */
  private async sendToPagerDuty(notification: AlertNotification): Promise<void> {
    if (!this.config.pagerduty) return;

    const payload = {
      routing_key: this.config.pagerduty.apiKey,
      event_action: "trigger",
      dedup_key: `${notification.ruleId}_${notification.network}`,
      payload: {
        summary: `${notification.ruleName}: ${notification.message}`,
        severity: notification.severity === "critical" ? "critical" : "warning",
        source: notification.network,
        timestamp: notification.timestamp,
        custom_details: notification.details,
      },
    };

    try {
      const response = await fetch("https://events.pagerduty.com/v2/enqueue", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });

      if (!response.ok) {
        throw new Error(`PagerDuty returned ${response.status}`);
      }
    } catch (error) {
      console.error("Failed to send to PagerDuty:", error);
      throw error;
    }
  }

  /**
   * Send to console
   */
  private sendToConsole(notification: AlertNotification): void {
    const icon = this.getSeverityIcon(notification.severity);
    console.log(`\n${icon} ${notification.ruleName}`);
    console.log(`Network: ${notification.network}`);
    console.log(`Severity: ${notification.severity.toUpperCase()}`);
    console.log(`Message: ${notification.message}`);
    console.log(`Details:`, notification.details);
    console.log(`Time: ${new Date(notification.timestamp).toLocaleString()}\n`);
  }

  /**
   * Get severity icon
   */
  private getSeverityIcon(severity: string): string {
    switch (severity) {
      case "critical":
        return "🔴";
      case "warning":
        return "🟡";
      default:
        return "ℹ️";
    }
  }

  /**
   * Get severity color (Discord/Slack decimal)
   */
  private getSeverityColor(severity: string): number {
    switch (severity) {
      case "critical":
        return 0xff0000; // Red
      case "warning":
        return 0xffaa00; // Orange
      default:
        return 0x0099ff; // Blue
    }
  }

  /**
   * Get severity color (hex string for email)
   */
  private getSeverityColorHex(severity: string): string {
    switch (severity) {
      case "critical":
        return "#ff0000";
      case "warning":
        return "#ffaa00";
      default:
        return "#0099ff";
    }
  }
}
