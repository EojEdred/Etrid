/**
 * Alert System - Multi-channel notifications
 * Telegram, Discord, Email, Webhooks
 */

const axios = require('axios');
const nodemailer = require('nodemailer');

class AlertSystem {
  constructor(config) {
    this.config = config;
    this.alertHistory = [];
    this.rateLimits = new Map(); // Prevent alert spam

    // Initialize integrations
    this.telegram = config.alerts?.telegram?.enabled ? this.initTelegram() : null;
    this.discord = config.alerts?.discord?.enabled ? this.initDiscord() : null;
    this.email = config.alerts?.email?.enabled ? this.initEmail() : null;
  }

  initTelegram() {
    const { botToken, chatId } = this.config.alerts.telegram;
    return {
      botToken,
      chatId,
      baseUrl: `https://api.telegram.org/bot${botToken}`
    };
  }

  initDiscord() {
    const { webhookUrl } = this.config.alerts.discord;
    return { webhookUrl };
  }

  initEmail() {
    const { smtp, to } = this.config.alerts.email;
    return {
      transporter: nodemailer.createTransport(smtp),
      recipients: to
    };
  }

  /**
   * Send alert through all enabled channels
   */
  async sendAlert(alert) {
    const {
      severity = 'info', // info, warning, critical
      title,
      message,
      chain,
      node,
      details = {},
      deduplicateKey = null
    } = alert;

    // Rate limiting / deduplication
    if (deduplicateKey && this.isRecentAlert(deduplicateKey)) {
      console.log(`Alert deduplicated: ${deduplicateKey}`);
      return { deduplicated: true };
    }

    // Format alert
    const formattedAlert = this.formatAlert(alert);

    // Send to all channels
    const results = await Promise.allSettled([
      this.telegram ? this.sendTelegram(formattedAlert) : null,
      this.discord ? this.sendDiscord(formattedAlert) : null,
      this.email ? this.sendEmail(formattedAlert) : null
    ]);

    // Track alert
    this.alertHistory.push({
      ...alert,
      timestamp: Date.now(),
      sent: results.filter(r => r.status === 'fulfilled').length
    });

    if (deduplicateKey) {
      this.rateLimits.set(deduplicateKey, Date.now());
    }

    return {
      success: true,
      results: results.map((r, i) => ({
        channel: ['telegram', 'discord', 'email'][i],
        status: r.status,
        value: r.value
      }))
    };
  }

  /**
   * Send Telegram message
   */
  async sendTelegram(alert) {
    if (!this.telegram) return null;

    const emoji = this.getSeverityEmoji(alert.severity);
    let text = `${emoji} *${alert.title}*\n\n`;
    text += `${alert.message}\n\n`;

    if (alert.chain) text += `Chain: ${alert.chain}\n`;
    if (alert.node) text += `Node: ${alert.node}\n`;

    if (Object.keys(alert.details).length > 0) {
      text += `\nDetails:\n`;
      for (const [key, value] of Object.entries(alert.details)) {
        text += `• ${key}: ${value}\n`;
      }
    }

    text += `\n_${new Date().toISOString()}_`;

    try {
      const response = await axios.post(
        `${this.telegram.baseUrl}/sendMessage`,
        {
          chat_id: this.telegram.chatId,
          text,
          parse_mode: 'Markdown'
        }
      );

      return { success: true, messageId: response.data.result.message_id };
    } catch (err) {
      console.error('Telegram alert failed:', err.message);
      return { success: false, error: err.message };
    }
  }

  /**
   * Send Discord webhook
   */
  async sendDiscord(alert) {
    if (!this.discord) return null;

    const color = this.getSeverityColor(alert.severity);
    const embed = {
      title: alert.title,
      description: alert.message,
      color: parseInt(color.replace('#', ''), 16),
      fields: [],
      timestamp: new Date().toISOString()
    };

    if (alert.chain) {
      embed.fields.push({ name: 'Chain', value: alert.chain, inline: true });
    }
    if (alert.node) {
      embed.fields.push({ name: 'Node', value: alert.node, inline: true });
    }

    for (const [key, value] of Object.entries(alert.details)) {
      embed.fields.push({
        name: key,
        value: String(value),
        inline: true
      });
    }

    try {
      await axios.post(this.discord.webhookUrl, {
        embeds: [embed],
        username: 'Etrid Operations',
        avatar_url: 'https://etrid.io/logo.png' // Optional
      });

      return { success: true };
    } catch (err) {
      console.error('Discord alert failed:', err.message);
      return { success: false, error: err.message };
    }
  }

  /**
   * Send Email
   */
  async sendEmail(alert) {
    if (!this.email) return null;

    const subject = `[${alert.severity.toUpperCase()}] ${alert.title}`;

    let html = `<h2>${alert.title}</h2>`;
    html += `<p>${alert.message}</p>`;

    if (alert.chain || alert.node) {
      html += '<hr>';
      if (alert.chain) html += `<p><strong>Chain:</strong> ${alert.chain}</p>`;
      if (alert.node) html += `<p><strong>Node:</strong> ${alert.node}</p>`;
    }

    if (Object.keys(alert.details).length > 0) {
      html += '<hr><h3>Details:</h3><ul>';
      for (const [key, value] of Object.entries(alert.details)) {
        html += `<li><strong>${key}:</strong> ${value}</li>`;
      }
      html += '</ul>';
    }

    html += `<hr><p><small>${new Date().toISOString()}</small></p>`;

    try {
      await this.email.transporter.sendMail({
        from: this.config.alerts.email.smtp.user,
        to: this.email.recipients.join(', '),
        subject,
        html
      });

      return { success: true };
    } catch (err) {
      console.error('Email alert failed:', err.message);
      return { success: false, error: err.message };
    }
  }

  /**
   * Predefined alert templates
   */

  async alertNodeOffline(node, chain) {
    return await this.sendAlert({
      severity: 'critical',
      title: '🔴 Node Offline',
      message: `Node ${node} in ${chain} chain is offline and unreachable.`,
      chain,
      node,
      deduplicateKey: `node-offline-${chain}-${node}`
    });
  }

  async alertLowPeers(node, chain, peerCount, threshold) {
    return await this.sendAlert({
      severity: 'warning',
      title: '⚠️ Low Peer Count',
      message: `Node ${node} has only ${peerCount} peers (threshold: ${threshold}).`,
      chain,
      node,
      details: { peers: peerCount, threshold },
      deduplicateKey: `low-peers-${chain}-${node}`
    });
  }

  async alertHighCPU(node, chain, cpuUsage) {
    return await this.sendAlert({
      severity: 'warning',
      title: '⚠️ High CPU Usage',
      message: `Node ${node} CPU usage is at ${cpuUsage}%.`,
      chain,
      node,
      details: { cpu: `${cpuUsage}%` },
      deduplicateKey: `high-cpu-${chain}-${node}`
    });
  }

  async alertLowDiskSpace(node, chain, available, threshold) {
    return await this.sendAlert({
      severity: 'critical',
      title: '🔴 Low Disk Space',
      message: `Node ${node} has only ${this.formatBytes(available)} available (threshold: ${this.formatBytes(threshold)}).`,
      chain,
      node,
      details: {
        available: this.formatBytes(available),
        threshold: this.formatBytes(threshold)
      },
      deduplicateKey: `low-disk-${chain}-${node}`
    });
  }

  async alertBlockProduction(chain, blockNumber, expectedBlock) {
    return await this.sendAlert({
      severity: 'critical',
      title: '🔴 Block Production Issue',
      message: `${chain} is behind on block production. Current: ${blockNumber}, Expected: ${expectedBlock}.`,
      chain,
      details: {
        current: blockNumber,
        expected: expectedBlock,
        behind: expectedBlock - blockNumber
      },
      deduplicateKey: `block-production-${chain}`
    });
  }

  async alertHealthCheckFailed(chain, issues) {
    return await this.sendAlert({
      severity: 'warning',
      title: '⚠️ Health Check Failed',
      message: `Health check found ${issues.length} issue(s) on ${chain}.`,
      chain,
      details: {
        issues: issues.length,
        firstIssue: issues[0]?.message || 'Unknown'
      },
      deduplicateKey: `health-check-${chain}`
    });
  }

  async alertUpdateAvailable(chain, currentVersion, newVersion) {
    return await this.sendAlert({
      severity: 'info',
      title: '📦 Update Available',
      message: `New version ${newVersion} is available for ${chain} (current: ${currentVersion}).`,
      chain,
      details: {
        current: currentVersion,
        new: newVersion
      }
    });
  }

  async alertValidatorSlashed(validator, amount, reason) {
    return await this.sendAlert({
      severity: 'critical',
      title: '🚨 Validator Slashed',
      message: `Validator ${validator} was slashed for ${amount}. Reason: ${reason}`,
      node: validator,
      details: {
        amount,
        reason
      },
      deduplicateKey: `slashed-${validator}`
    });
  }

  async alertNetworkPartition(affectedNodes) {
    return await this.sendAlert({
      severity: 'critical',
      title: '🚨 Network Partition Detected',
      message: `Possible network partition affecting ${affectedNodes.length} nodes.`,
      details: {
        affectedNodes: affectedNodes.join(', ')
      }
    });
  }

  /**
   * Test alerts
   */
  async sendTestAlert() {
    return await this.sendAlert({
      severity: 'info',
      title: '✅ Test Alert',
      message: 'This is a test alert from Etrid Operations Center. All systems configured correctly!',
      details: {
        timestamp: new Date().toISOString(),
        channels: [
          this.telegram ? 'Telegram' : null,
          this.discord ? 'Discord' : null,
          this.email ? 'Email' : null
        ].filter(Boolean).join(', ')
      }
    });
  }

  // Helper methods

  formatAlert(alert) {
    return {
      ...alert,
      timestamp: new Date().toISOString(),
      formattedTime: new Date().toLocaleString()
    };
  }

  isRecentAlert(key) {
    const lastAlert = this.rateLimits.get(key);
    if (!lastAlert) return false;

    const timeSinceLastAlert = Date.now() - lastAlert;
    const cooldown = 15 * 60 * 1000; // 15 minutes

    return timeSinceLastAlert < cooldown;
  }

  getSeverityEmoji(severity) {
    const emojis = {
      info: 'ℹ️',
      warning: '⚠️',
      critical: '🔴'
    };
    return emojis[severity] || '📢';
  }

  getSeverityColor(severity) {
    const colors = {
      info: '#4da3ff',
      warning: '#ffaa00',
      critical: '#ff5370'
    };
    return colors[severity] || '#9fa6c0';
  }

  formatBytes(bytes) {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }

    return `${size.toFixed(2)} ${units[unitIndex]}`;
  }

  /**
   * Get alert history
   */
  getHistory(limit = 100) {
    return this.alertHistory
      .slice(-limit)
      .reverse();
  }

  /**
   * Clear rate limits (for testing)
   */
  clearRateLimits() {
    this.rateLimits.clear();
  }
}

module.exports = { AlertSystem };
