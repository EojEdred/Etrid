# 🚨 Advanced Alert System

Enterprise-grade multi-channel alert system for proactive MasterChef monitoring.

**Get notified instantly across Telegram, Discord, Email, Slack, and PagerDuty when issues arise!**

---

## 🎯 What This Does

The Advanced Alert System provides:

✅ **Custom Alert Rules** - Define your own conditions and thresholds
✅ **Multi-Channel Notifications** - Send to 5+ channels simultaneously
✅ **Smart Alerting** - Cooldowns, escalations, and deduplication
✅ **Historical Comparisons** - Alert on percentage changes over time
✅ **Escalation Rules** - Auto-escalate unacknowledged critical alerts
✅ **No False Positives** - Intelligent cooldown periods prevent spam

---

## 🚀 Quick Start

### 1. Configure Channels

Add to your `.env` file:

```bash
# Default Network
DEFAULT_NETWORK=mainnet

# Alert Check Interval (5 minutes)
ALERT_CHECK_INTERVAL=300000

# Telegram (required if using Telegram channel)
TELEGRAM_BOT_TOKEN=your-bot-token
TELEGRAM_ADMIN_IDS=123456789,987654321

# Discord (optional)
DISCORD_WEBHOOK_URL=https://discord.com/api/webhooks/...

# Email (optional)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=alerts@yourdomain.com
SMTP_PASSWORD=your-app-password
SMTP_FROM=alerts@yourdomain.com
ALERT_EMAIL_TO=admin@yourdomain.com,team@yourdomain.com

# Slack (optional)
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/...

# PagerDuty (optional)
PAGERDUTY_API_KEY=your-pagerduty-integration-key
PAGERDUTY_SERVICE_ID=your-service-id
```

### 2. Install Dependencies

```bash
cd 05-multichain/bridge/adapters/bsc
npm install
```

This installs:
- `uuid` - For generating alert IDs
- `nodemailer` - For email notifications

### 3. Ensure Database Has Metrics

```bash
npm run collect-metrics:mainnet
```

### 4. Start Alert Monitor

```bash
npm run alert-monitor
```

**Output:**
```
🚀 ADVANCED ALERT MONITOR STARTED

   Network: mainnet
   Check Interval: 5 minutes

   Configured Channels:
     ✅ Telegram
     ✅ Discord
     ✅ Email

   Active Rules: 7
     - Low MasterChef Balance (warning)
     - Critical MasterChef Balance (critical)
     - TVL Drop (warning)
     - Low Days Remaining (warning)
     - Critical Days Remaining (critical)
     - Contract Paused (critical)
     - Stale Data (warning)

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🔍 Running alert check for mainnet...
✅ No alerts triggered
```

---

## 📋 Default Alert Rules

The system comes with 7 predefined rules:

### 1. Low MasterChef Balance
- **Condition**: Balance < 1M ÉTR
- **Severity**: Warning
- **Channels**: Telegram, Discord
- **Cooldown**: 60 minutes

### 2. Critical MasterChef Balance
- **Condition**: Balance < 500K ÉTR
- **Severity**: Critical
- **Channels**: Telegram, Discord, Email, PagerDuty
- **Cooldown**: 30 minutes
- **Escalation**: PagerDuty + Email after 60 min

### 3. TVL Drop
- **Condition**: TVL drops > 10% in 24 hours
- **Severity**: Warning
- **Channels**: Telegram, Discord
- **Cooldown**: 120 minutes

### 4. Low Days Remaining
- **Condition**: < 30 days of rewards left
- **Severity**: Warning
- **Channels**: Telegram, Email
- **Cooldown**: 24 hours

### 5. Critical Days Remaining
- **Condition**: < 7 days of rewards left
- **Severity**: Critical
- **Channels**: Telegram, Discord, Email, PagerDuty
- **Cooldown**: 6 hours

### 6. Contract Paused
- **Condition**: MasterChef is paused
- **Severity**: Critical
- **Channels**: All channels
- **Cooldown**: 15 minutes
- **Escalation**: PagerDuty after 30 min

### 7. Stale Data
- **Condition**: Metrics not updated in 2+ hours
- **Severity**: Warning
- **Channels**: Telegram, Discord
- **Cooldown**: 60 minutes

---

## 🎨 Custom Alert Rules

### Add Custom Rules via Environment Variable

```bash
# In .env
ALERT_RULES_JSON='[
  {
    "id": "custom-tvl-high",
    "name": "High TVL Milestone",
    "description": "Alert when TVL exceeds $2M",
    "enabled": true,
    "network": "mainnet",
    "metric": "total_tvl",
    "condition": "gt",
    "threshold": 2000000,
    "severity": "info",
    "channels": ["telegram", "slack"],
    "cooldownMinutes": 1440
  }
]'
```

### Rule Configuration Options

```typescript
{
  id: string;                  // Unique identifier
  name: string;               // Display name
  description: string;        // Description
  enabled: boolean;           // Enable/disable
  network: "mainnet" | "testnet" | "all";

  // Condition
  metric: string;             // Metric to monitor
  condition: "gt" | "lt" | "eq" | "gte" | "lte" | "pct_change";
  threshold: number;          // Threshold value
  comparisonPeriod?: number;  // Hours (for pct_change)

  // Severity
  severity: "info" | "warning" | "critical";

  // Notification
  channels: Array<"telegram" | "discord" | "email" | "slack" | "pagerduty" | "console">;
  cooldownMinutes: number;    // Prevent duplicate alerts

  // Escalation (optional)
  escalateAfterMinutes?: number;
  escalationChannels?: Array<...>;
}
```

### Available Metrics

**MasterChef:**
- `masterchef_balance` - ÉTR balance
- `reward_per_block` - Emission rate
- `days_remaining` - Days until depleted
- `is_paused` - Paused state (0 or 1)
- `total_pools` - Number of pools

**TVL & APR:**
- `total_tvl` - Total value locked (USD)
- `pool_tvl.{poolId}` - Specific pool TVL
- `pool_apr.{poolId}` - Specific pool APR
- `pool_staked.{poolId}` - Specific pool staked amount

**Prices:**
- `bnb_price` - BNB price in USD
- `etr_price` - ÉTR price in USD

**System:**
- `data_freshness_hours` - Hours since last update

### Condition Types

**Standard Comparisons:**
- `gt` - Greater than
- `lt` - Less than
- `gte` - Greater than or equal
- `lte` - Less than or equal
- `eq` - Equal to

**Percentage Change:**
- `pct_change` - Percentage change over time
  - Example: Alert if TVL changes > 10% in 24 hours
  - Requires `comparisonPeriod` (in hours)

---

## 💬 Alert Examples

### Telegram Notification

```
🟡 Low MasterChef Balance

Network: mainnet
Severity: WARNING

Low MasterChef Balance: masterchef_balance is 987,654.32 (lt 1,000,000)

10/24/2025, 2:15:32 PM
```

### Discord Embed

```
🟡 Low MasterChef Balance

Low MasterChef Balance: masterchef_balance is 987,654.32 (lt 1,000,000)

Network: mainnet
Severity: WARNING

Details:
{
  "metric": "masterchef_balance",
  "current": 987654.32,
  "threshold": 1000000
}

MasterChef Alert System
```

### Email

```
Subject: 🟡 WARNING: Low MasterChef Balance

[Formatted HTML email with header, content, and details]
```

### PagerDuty Incident

```
Summary: Critical MasterChef Balance: masterchef_balance is 456,789.12 (lt 500,000)
Severity: critical
Source: mainnet
Custom Details: { metric: "masterchef_balance", current: 456789.12, ... }
```

---

## 🔄 Running as Service

### Using PM2 (Recommended)

```bash
# Install PM2
npm install -g pm2

# Start alert monitor
pm2 start npm --name "alert-monitor" -- run alert-monitor

# Save PM2 config
pm2 save

# Setup auto-restart on reboot
pm2 startup

# Monitor
pm2 logs alert-monitor
pm2 status

# Restart
pm2 restart alert-monitor
```

### Using systemd (Linux)

```bash
# Create service file
sudo nano /etc/systemd/system/alert-monitor.service
```

```ini
[Unit]
Description=MasterChef Alert Monitor
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/path/to/etrid/05-multichain/bridge/adapters/bsc
ExecStart=/usr/bin/npm run alert-monitor
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start
sudo systemctl enable alert-monitor
sudo systemctl start alert-monitor

# Check status
sudo systemctl status alert-monitor

# View logs
sudo journalctl -u alert-monitor -f
```

### Using Docker

```dockerfile
FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --production

COPY . .

CMD ["npm", "run", "alert-monitor"]
```

```bash
# Build
docker build -t alert-monitor .

# Run
docker run -d \
  --name alert-monitor \
  --restart unless-stopped \
  --env-file .env \
  alert-monitor

# Logs
docker logs -f alert-monitor
```

---

## 🔔 Channel Setup Guides

### Discord

1. Open Discord server settings
2. Go to **Integrations** → **Webhooks**
3. Click **New Webhook**
4. Name it "MasterChef Alerts"
5. Copy webhook URL
6. Add to `.env` as `DISCORD_WEBHOOK_URL`

### Email (Gmail)

1. Enable 2FA on your Gmail account
2. Go to **Security** → **App Passwords**
3. Generate app password for "Mail"
4. Add to `.env`:
   ```bash
   SMTP_HOST=smtp.gmail.com
   SMTP_PORT=587
   SMTP_USERNAME=your-email@gmail.com
   SMTP_PASSWORD=your-16-char-app-password
   SMTP_FROM=your-email@gmail.com
   ALERT_EMAIL_TO=recipient@gmail.com
   ```

### Slack

1. Go to https://api.slack.com/messaging/webhooks
2. Click **Create your Slack app**
3. Choose **From scratch**
4. Enable **Incoming Webhooks**
5. Add webhook to workspace
6. Copy webhook URL
7. Add to `.env` as `SLACK_WEBHOOK_URL`

### PagerDuty

1. Go to **Configuration** → **Services**
2. Select service or create new one
3. Add **Integration** → **Events API V2**
4. Copy **Integration Key**
5. Add to `.env`:
   ```bash
   PAGERDUTY_API_KEY=your-integration-key
   PAGERDUTY_SERVICE_ID=your-service-id
   ```

---

## 🎛️ Advanced Features

### Cooldown Periods

Prevents alert spam:

```typescript
{
  cooldownMinutes: 60  // Won't trigger again for 60 min
}
```

**How it works:**
1. Alert triggers
2. Notification sent
3. Rule marked with `lastTriggered` timestamp
4. Rule skipped for next 60 minutes
5. After cooldown, can trigger again

### Escalations

Auto-escalate unacknowledged critical alerts:

```typescript
{
  severity: "critical",
  channels: ["telegram", "discord"],
  escalateAfterMinutes: 60,
  escalationChannels: ["pagerduty", "email"]
}
```

**How it works:**
1. Critical alert triggers → Sent to Telegram + Discord
2. After 60 minutes, if not acknowledged
3. Escalation sent to PagerDuty + Email
4. Continues until acknowledged

### Percentage Change Alerts

Alert on relative changes:

```typescript
{
  metric: "total_tvl",
  condition: "pct_change",
  threshold: 10,           // 10% change
  comparisonPeriod: 24     // In 24 hours
}
```

Triggers if TVL changes ±10% in 24 hours.

### Pool-Specific Alerts

Monitor individual pools:

```typescript
{
  metric: "pool_tvl.0",  // Pool 0 TVL
  condition: "lt",
  threshold: 100000      // < $100K
}

{
  metric: "pool_apr.1",  // Pool 1 APR
  condition: "pct_change",
  threshold: 20,         // 20% change
  comparisonPeriod: 6    // In 6 hours
}
```

---

## 📊 Monitoring the Monitor

### Check Status

```bash
npm run alert-monitor -- --status
```

**Output:**
```
📊 ALERT MONITOR STATUS

   Running: Yes
   Network: mainnet
   Check Interval: 5 minutes

   Total Rules: 7
   Enabled Rules: 7

   Database:
     Alerts: 42
     Metrics Snapshots: 1,234
```

### View Alert History

```sql
sqlite3 database/masterchef.db

-- Recent alerts
SELECT * FROM alerts ORDER BY timestamp DESC LIMIT 10;

-- Unacknowledged alerts
SELECT * FROM alerts WHERE acknowledged = FALSE;

-- Critical alerts in last 24h
SELECT * FROM alerts
WHERE severity = 'critical'
  AND timestamp >= datetime('now', '-1 day')
ORDER BY timestamp DESC;
```

### Monitor Logs

```bash
# PM2 logs
pm2 logs alert-monitor

# Systemd logs
sudo journalctl -u alert-monitor -f

# Docker logs
docker logs -f alert-monitor
```

---

## 🐛 Troubleshooting

### No alerts being sent

**Check monitor is running:**
```bash
pm2 status alert-monitor
# or
ps aux | grep alert-monitor
```

**Check channels are configured:**
```bash
# Verify .env has required variables
cat .env | grep -E "TELEGRAM|DISCORD|SMTP"
```

**Test notification manually:**
```typescript
// In alert-system/test.ts
const channels = new AlertChannels(config);
await channels.send(testNotification);
```

### Alerts triggering too often

**Increase cooldown:**
```typescript
{
  cooldownMinutes: 120  // 2 hours instead of 60
}
```

### Missing percentage change alerts

**Ensure historical data exists:**
```bash
# Check metrics snapshots
sqlite3 database/masterchef.db "SELECT COUNT(*) FROM metrics_snapshots;"

# Need at least 2+ snapshots for comparison
npm run collect-metrics:mainnet
```

### Email not sending

**Check SMTP credentials:**
```bash
# Test with telnet
telnet smtp.gmail.com 587

# Verify app password (not regular password)
# Gmail requires app-specific passwords with 2FA
```

### Discord webhook fails

**Verify webhook URL:**
```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{"content":"Test"}' \
  YOUR_DISCORD_WEBHOOK_URL

# Should return 204 No Content
```

---

## 🎓 Best Practices

### 1. Start Conservative

Begin with higher thresholds and longer cooldowns:

```typescript
{
  threshold: 500000,      // Higher threshold
  cooldownMinutes: 120    // Longer cooldown
}
```

Then tune based on your tolerance for alerts.

### 2. Use Escalations for Critical Issues

```typescript
{
  severity: "critical",
  channels: ["telegram"],
  escalateAfterMinutes: 30,
  escalationChannels: ["pagerduty"]
}
```

Start with low-friction channels, escalate to high-friction.

### 3. Match Channels to Severity

- **Info**: Console, Slack
- **Warning**: Telegram, Discord
- **Critical**: Email, PagerDuty

### 4. Monitor During Business Hours First

Test alerts during business hours before enabling 24/7:

```bash
# Run manually during day
npm run alert-monitor

# Once confident, enable service
pm2 start npm --name alert-monitor -- run alert-monitor
```

### 5. Keep Rules Simple

One condition per rule is easier to understand and debug:

```typescript
// Good
{ metric: "masterchef_balance", condition: "lt", threshold: 1000000 }

// Bad (hard to reason about)
{ metric: "masterchef_balance / total_tvl", condition: "lt", threshold: 0.5 }
```

---

## 📚 Integration with Other Systems

### Combine with Metrics Collection

```bash
# Cron job to collect metrics hourly
0 * * * * cd /path/to/bsc && npm run collect-metrics:mainnet

# PM2 to run alert monitor continuously
pm2 start npm --name alert-monitor -- run alert-monitor
```

### Integrate with Telegram Bot

The alert monitor can use the same Telegram bot:

```bash
# .env
TELEGRAM_BOT_TOKEN=same-bot-token
TELEGRAM_ADMIN_IDS=same-admin-ids
```

Alerts sent to admin chat IDs automatically.

### Export to External Monitoring

```bash
# Export alerts to JSON
sqlite3 database/masterchef.db "SELECT * FROM alerts" | jq

# Send to external system
curl -X POST https://your-monitoring.com/alerts \
  -d "$(sqlite3 database/masterchef.db 'SELECT * FROM alerts WHERE acknowledged = FALSE')"
```

---

## 🎉 Summary

✅ **7 predefined rules** covering common issues
✅ **5 notification channels** (Telegram, Discord, Email, Slack, PagerDuty)
✅ **Smart alerting** with cooldowns and escalations
✅ **Custom rules** via JSON configuration
✅ **Percentage change** detection
✅ **Pool-specific** monitoring
✅ **Enterprise-ready** with PM2/systemd/Docker
✅ **Zero cost** (except PagerDuty if used)

**Setup Time**: 10 minutes
**Maintenance**: Zero (runs continuously)
**Peace of Mind**: Priceless

---

## 🚀 Next Steps

1. **Configure channels** in `.env`
2. **Start monitor**: `npm run alert-monitor`
3. **Test alerts** by manually triggering conditions
4. **Tune rules** based on your needs
5. **Set up service** for 24/7 monitoring

**Related Features:**
- Historical Data Tracking (built)
- Telegram Bot (built)
- Backup & Recovery (coming next)
- Analytics API (coming next)

---

**Questions?** Check the main [SCRIPTS_README.md](SCRIPTS_README.md) or reach out!

**Ready for proactive monitoring?** Run `npm run alert-monitor` to start! 🚀
