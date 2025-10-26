# Automated Monitoring Setup

**Complete guide to setting up automated monitoring for MasterChef LP Rewards**

Last Updated: October 24, 2025

---

## 📋 Table of Contents

- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Cron Job Setup](#cron-job-setup)
- [Alert Configuration](#alert-configuration)
- [Dashboard Integration](#dashboard-integration)
- [Log Management](#log-management)
- [Troubleshooting](#troubleshooting)

---

## 📊 Overview

This guide sets up automated monitoring that:
- ✅ Runs health checks every 8 hours
- ✅ Monitors TVL and APR hourly
- ✅ Exports metrics for dashboards
- ✅ Sends alerts when issues detected
- ✅ Logs all operations for audit trail

**Estimated setup time**: 30-60 minutes

---

## 🔧 Prerequisites

### Required
- Linux/macOS server (or WSL on Windows)
- Node.js 18+ installed
- Git repository cloned
- `.env` configured with contract addresses
- Cron access (usually default on Linux/macOS)

### Optional (for alerts)
- Discord webhook URL
- Telegram bot token
- Email SMTP credentials
- Slack webhook URL

---

## ⏰ Cron Job Setup

### Step 1: Create Monitoring Directory

```bash
# Create log directory
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc
mkdir -p logs monitoring
```

---

### Step 2: Create Monitoring Scripts

#### Health Check Script

Create `monitoring/health-check.sh`:

```bash
#!/bin/bash
# health-check.sh - Automated health monitoring

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LOG_FILE="$PROJECT_DIR/logs/health-check-$(date +%Y%m%d).log"

echo "$(date '+%Y-%m-%d %H:%M:%S') - Starting health check" >> "$LOG_FILE"

cd "$PROJECT_DIR"

# Run health check
npm run check-pool-health:mainnet >> "$LOG_FILE" 2>&1
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "$(date '+%Y-%m-%d %H:%M:%S') - CRITICAL: Health check failed!" >> "$LOG_FILE"

    # Send alert (uncomment after configuring)
    # "$SCRIPT_DIR/send-alert.sh" "CRITICAL" "Health check failed - exit code $EXIT_CODE"

    exit 1
fi

echo "$(date '+%Y-%m-%d %H:%M:%S') - Health check passed" >> "$LOG_FILE"
exit 0
```

Make executable:
```bash
chmod +x monitoring/health-check.sh
```

---

#### TVL Monitoring Script

Create `monitoring/monitor-tvl.sh`:

```bash
#!/bin/bash
# monitor-tvl.sh - TVL monitoring

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LOG_FILE="$PROJECT_DIR/logs/tvl-monitor-$(date +%Y%m%d).log"

echo "$(date '+%Y-%m-%d %H:%M:%S') - Monitoring TVL" >> "$LOG_FILE"

cd "$PROJECT_DIR"

# Run TVL monitor
npm run monitor-tvl:mainnet >> "$LOG_FILE" 2>&1

# Check for significant TVL drop (optional - requires jq)
# CURRENT_TVL=$(npm run monitor-tvl:mainnet 2>/dev/null | grep "Total LP Staked" | awk '{print $4}')
# Compare with previous and alert if dropped >50%

echo "$(date '+%Y-%m-%d %H:%M:%S') - TVL monitoring complete" >> "$LOG_FILE"
```

Make executable:
```bash
chmod +x monitoring/monitor-tvl.sh
```

---

#### Metrics Export Script

Create `monitoring/export-metrics.sh`:

```bash
#!/bin/bash
# export-metrics.sh - Export metrics for dashboards

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LOG_FILE="$PROJECT_DIR/logs/metrics-export-$(date +%Y%m%d).log"

echo "$(date '+%Y-%m-%d %H:%M:%S') - Exporting metrics" >> "$LOG_FILE"

cd "$PROJECT_DIR"

# Run metrics export
npm run export-metrics:mainnet >> "$LOG_FILE" 2>&1

# Optional: Copy to web server for dashboard display
# METRICS_FILE=$(ls -t metrics-*.json | head -1)
# scp "$METRICS_FILE" user@webserver:/var/www/metrics/latest.json

echo "$(date '+%Y-%m-%d %H:%M:%S') - Metrics export complete" >> "$LOG_FILE"
```

Make executable:
```bash
chmod +x monitoring/export-metrics.sh
```

---

#### Balance Alert Script

Create `monitoring/check-balance.sh`:

```bash
#!/bin/bash
# check-balance.sh - Alert if MasterChef balance low

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LOG_FILE="$PROJECT_DIR/logs/balance-check-$(date +%Y%m%d).log"

echo "$(date '+%Y-%m-%d %H:%M:%S') - Checking MasterChef balance" >> "$LOG_FILE"

cd "$PROJECT_DIR"

# Run health check and parse days remaining
HEALTH_OUTPUT=$(npm run check-pool-health:mainnet 2>&1)

# Extract days remaining (requires parsing JSON output)
# This is a simple version - enhance with jq for production
DAYS_REMAINING=$(echo "$HEALTH_OUTPUT" | grep -oP 'daysRemaining":\s*\K\d+' | head -1)

if [ -n "$DAYS_REMAINING" ]; then
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Days remaining: $DAYS_REMAINING" >> "$LOG_FILE"

    if [ "$DAYS_REMAINING" -lt 7 ]; then
        echo "$(date '+%Y-%m-%d %H:%M:%S') - CRITICAL: Only $DAYS_REMAINING days of rewards remaining!" >> "$LOG_FILE"
        "$SCRIPT_DIR/send-alert.sh" "CRITICAL" "MasterChef has only $DAYS_REMAINING days of ÉTR rewards remaining!"
    elif [ "$DAYS_REMAINING" -lt 30 ]; then
        echo "$(date '+%Y-%m-%d %H:%M:%S') - WARNING: Only $DAYS_REMAINING days of rewards remaining" >> "$LOG_FILE"
        "$SCRIPT_DIR/send-alert.sh" "WARNING" "MasterChef has only $DAYS_REMAINING days of ÉTR rewards remaining"
    fi
fi

echo "$(date '+%Y-%m-%d %H:%M:%S') - Balance check complete" >> "$LOG_FILE"
```

Make executable:
```bash
chmod +x monitoring/check-balance.sh
```

---

### Step 3: Configure Cron Jobs

Edit crontab:
```bash
crontab -e
```

Add the following lines:

```cron
# MasterChef Monitoring - Mainnet
# Replace /path/to/bsc with your actual path

# Health check every 8 hours (1 AM, 9 AM, 5 PM)
0 1,9,17 * * * /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/monitoring/health-check.sh

# TVL monitoring every 4 hours
0 */4 * * * /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/monitoring/monitor-tvl.sh

# Metrics export every hour
0 * * * * /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/monitoring/export-metrics.sh

# Balance check twice daily (9 AM, 9 PM)
0 9,21 * * * /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/monitoring/check-balance.sh

# Log cleanup weekly (Sunday at 2 AM) - keep 30 days
0 2 * * 0 find /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/logs -name "*.log" -mtime +30 -delete
```

Save and exit (`:wq` in vim).

---

### Step 4: Verify Cron Setup

Check cron jobs are registered:
```bash
crontab -l
```

Test a script manually:
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc
./monitoring/health-check.sh
```

Check the log:
```bash
tail -f logs/health-check-$(date +%Y%m%d).log
```

---

## 🚨 Alert Configuration

### Option 1: Discord Webhook

Create `monitoring/send-alert.sh`:

```bash
#!/bin/bash
# send-alert.sh - Send alerts via Discord/Telegram/Email

SEVERITY=$1
MESSAGE=$2

# Discord Webhook (uncomment and configure)
# DISCORD_WEBHOOK="https://discord.com/api/webhooks/YOUR_WEBHOOK_HERE"
#
# COLOR="0" # Red for critical
# if [ "$SEVERITY" == "WARNING" ]; then
#     COLOR="16776960" # Yellow
# elif [ "$SEVERITY" == "INFO" ]; then
#     COLOR="65280" # Green
# fi
#
# curl -H "Content-Type: application/json" \
#      -X POST \
#      -d "{\"embeds\": [{\"title\": \"MasterChef Alert\", \"description\": \"**$SEVERITY**: $MESSAGE\", \"color\": $COLOR}]}" \
#      "$DISCORD_WEBHOOK"

# Telegram Bot (uncomment and configure)
# TELEGRAM_BOT_TOKEN="YOUR_BOT_TOKEN"
# TELEGRAM_CHAT_ID="YOUR_CHAT_ID"
#
# curl -s -X POST "https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/sendMessage" \
#      -d chat_id="$TELEGRAM_CHAT_ID" \
#      -d text="🚨 *MasterChef Alert*%0A%0A*$SEVERITY*: $MESSAGE" \
#      -d parse_mode="Markdown"

# Email (uncomment and configure)
# SMTP_SERVER="smtp.gmail.com"
# SMTP_PORT="587"
# SMTP_USER="your-email@gmail.com"
# SMTP_PASS="your-app-password"
# TO_EMAIL="alerts@etrid.io"
#
# echo "Subject: MasterChef Alert - $SEVERITY
#
# $MESSAGE
#
# Timestamp: $(date '+%Y-%m-%d %H:%M:%S')
# Server: $(hostname)" | \
# sendmail -S "$SMTP_SERVER:$SMTP_PORT" \
#          -au"$SMTP_USER" \
#          -ap"$SMTP_PASS" \
#          "$TO_EMAIL"

# For now, just log it
echo "$(date '+%Y-%m-%d %H:%M:%S') - ALERT [$SEVERITY]: $MESSAGE"
```

Make executable:
```bash
chmod +x monitoring/send-alert.sh
```

---

### Option 2: Telegram Bot

1. **Create Telegram Bot**:
   - Message @BotFather on Telegram
   - Send `/newbot`
   - Follow prompts to get bot token

2. **Get Chat ID**:
   - Start chat with your bot
   - Visit: `https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getUpdates`
   - Find your `chat_id`

3. **Configure** in `send-alert.sh` (uncomment Telegram section)

---

### Option 3: Email Alerts

1. **Gmail App Password**:
   - Go to Google Account settings
   - Security > 2-Step Verification > App passwords
   - Generate password for "Mail"

2. **Configure** in `send-alert.sh` (uncomment Email section)

---

## 📊 Dashboard Integration

### Option 1: Simple Web Dashboard

Create `monitoring/dashboard-sync.sh`:

```bash
#!/bin/bash
# dashboard-sync.sh - Sync metrics to web dashboard

PROJECT_DIR="/Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc"
WEB_DIR="/var/www/html/etrid-dashboard"  # Adjust to your web server

# Copy latest metrics
LATEST_METRICS=$(ls -t "$PROJECT_DIR"/metrics-*.json 2>/dev/null | head -1)

if [ -f "$LATEST_METRICS" ]; then
    cp "$LATEST_METRICS" "$WEB_DIR/metrics.json"
    echo "$(date '+%Y-%m-%d %H:%M:%S') - Dashboard updated" >> "$PROJECT_DIR/logs/dashboard-sync.log"
fi
```

Then create simple `index.html` to display metrics.

---

### Option 2: Grafana Dashboard

1. **Install Prometheus Node Exporter**:

```bash
# macOS
brew install prometheus node_exporter

# Linux
wget https://github.com/prometheus/node_exporter/releases/download/v1.6.1/node_exporter-1.6.1.linux-amd64.tar.gz
tar xvfz node_exporter-*.tar.gz
cd node_exporter-*/
./node_exporter &
```

2. **Configure Prometheus** (`prometheus.yml`):

```yaml
global:
  scrape_interval: 60s

scrape_configs:
  - job_name: 'masterchef'
    static_configs:
      - targets: ['localhost:9090']
    file_sd_configs:
      - files:
          - '/Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/metrics-*.prom'
```

3. **Install Grafana**:

```bash
# macOS
brew install grafana
brew services start grafana

# Linux
sudo apt-get install -y grafana
sudo systemctl start grafana-server
```

4. **Access Grafana**:
   - Visit: http://localhost:3000
   - Login: admin / admin
   - Add Prometheus data source
   - Import dashboard (see `GRAFANA_DASHBOARD.json` below)

---

### Grafana Dashboard JSON

Create `monitoring/GRAFANA_DASHBOARD.json`:

```json
{
  "dashboard": {
    "title": "MasterChef Monitoring",
    "panels": [
      {
        "title": "Days of ÉTR Remaining",
        "type": "gauge",
        "targets": [
          {
            "expr": "masterchef_days_remaining",
            "legendFormat": "Days"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "thresholds": {
              "steps": [
                {"value": 0, "color": "red"},
                {"value": 7, "color": "yellow"},
                {"value": 30, "color": "green"}
              ]
            }
          }
        }
      },
      {
        "title": "Total Value Locked (LP Tokens)",
        "type": "graph",
        "targets": [
          {
            "expr": "pool_total_staked",
            "legendFormat": "{{lp_symbol}}"
          }
        ]
      },
      {
        "title": "Daily Rewards per Pool",
        "type": "graph",
        "targets": [
          {
            "expr": "pool_daily_rewards",
            "legendFormat": "{{lp_symbol}}"
          }
        ]
      },
      {
        "title": "MasterChef ÉTR Balance",
        "type": "graph",
        "targets": [
          {
            "expr": "masterchef_balance_etr",
            "legendFormat": "Balance"
          }
        ]
      }
    ]
  }
}
```

---

## 📝 Log Management

### Log Rotation

Create `/etc/logrotate.d/masterchef`:

```
/Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc/logs/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0644 $(whoami) $(whoami)
}
```

Apply:
```bash
sudo logrotate -f /etc/logrotate.d/masterchef
```

---

### View Logs

```bash
# Latest health check log
tail -f logs/health-check-$(date +%Y%m%d).log

# Latest TVL monitor log
tail -f logs/tvl-monitor-$(date +%Y%m%d).log

# All logs from today
tail -f logs/*-$(date +%Y%m%d).log

# Search for errors
grep -i "error\|critical\|fail" logs/*.log

# Count health checks run today
grep "Health check" logs/health-check-$(date +%Y%m%d).log | wc -l
```

---

## 🔍 Monitoring Dashboard Commands

Create `monitoring/status.sh` for quick status check:

```bash
#!/bin/bash
# status.sh - Quick monitoring status

echo "=== MasterChef Monitoring Status ==="
echo ""

# Check if cron jobs are running
echo "Cron Jobs:"
crontab -l | grep "health-check\|monitor-tvl\|export-metrics\|check-balance" | wc -l | xargs echo "  Active jobs:"

# Check recent log activity
echo ""
echo "Recent Activity:"
echo "  Health checks today: $(grep "Health check" logs/health-check-$(date +%Y%m%d).log 2>/dev/null | wc -l)"
echo "  TVL monitors today: $(grep "Monitoring TVL" logs/tvl-monitor-$(date +%Y%m%d).log 2>/dev/null | wc -l)"
echo "  Metrics exports today: $(grep "Exporting metrics" logs/metrics-export-$(date +%Y%m%d).log 2>/dev/null | wc -l)"

# Check for recent alerts
echo ""
echo "Recent Alerts:"
grep -h "ALERT" logs/*.log 2>/dev/null | tail -5 | sed 's/^/  /'

# Check disk usage
echo ""
echo "Disk Usage:"
du -sh logs/ | sed 's/^/  /'

echo ""
echo "================================="
```

Make executable and run:
```bash
chmod +x monitoring/status.sh
./monitoring/status.sh
```

---

## 🛠️ Troubleshooting

### Cron Jobs Not Running

**Check cron service**:
```bash
# macOS
sudo launchctl list | grep cron

# Linux
sudo systemctl status cron
```

**Check cron logs**:
```bash
# macOS
log show --predicate 'process == "cron"' --last 1h

# Linux
sudo tail -f /var/log/cron
```

**Test script manually**:
```bash
cd /Users/macbook/Desktop/etrid/05-multichain/bridge/adapters/bsc
./monitoring/health-check.sh
echo $?  # Should be 0 if successful
```

---

### Scripts Failing

**Check Node.js is in PATH**:
```bash
which node
which npm
```

If not found in cron, add to scripts:
```bash
export PATH="/usr/local/bin:$PATH"
```

**Check permissions**:
```bash
ls -la monitoring/*.sh
# Should all be -rwxr-xr-x
```

**Check .env is readable**:
```bash
ls -la .env
cat .env | grep ADDRESS
```

---

### Alerts Not Sending

**Test alert script**:
```bash
./monitoring/send-alert.sh "TEST" "This is a test alert"
```

**Check webhook URL**:
```bash
# Discord
curl -H "Content-Type: application/json" \
     -X POST \
     -d '{"content": "Test"}' \
     "YOUR_DISCORD_WEBHOOK"
```

**Check firewall**:
```bash
# Ensure outbound HTTPS is allowed
curl -I https://discord.com
curl -I https://api.telegram.org
```

---

### High CPU/Memory Usage

**Check running processes**:
```bash
ps aux | grep node
```

**Reduce monitoring frequency** if needed:
```bash
# Edit crontab
crontab -e

# Change from hourly to every 4 hours
# 0 * * * * → 0 */4 * * *
```

**Limit log file sizes**:
```bash
# Add to scripts
LIMIT_LOGS=true
if [ "$LIMIT_LOGS" == "true" ]; then
    tail -n 1000 "$LOG_FILE" > "$LOG_FILE.tmp"
    mv "$LOG_FILE.tmp" "$LOG_FILE"
fi
```

---

## ✅ Setup Verification Checklist

After setup, verify everything works:

- [ ] All monitoring scripts are executable
- [ ] Cron jobs are registered (`crontab -l`)
- [ ] Manual script execution works
- [ ] Logs are being created
- [ ] Alerts send successfully (test with send-alert.sh)
- [ ] Dashboard displays data (if configured)
- [ ] Log rotation works
- [ ] Disk space is monitored
- [ ] Emergency contacts are notified

---

## 📚 Related Documentation

- [Scripts README](SCRIPTS_README.md) - All script commands
- [Emergency Runbook](EMERGENCY_RESPONSE_RUNBOOK.md) - Incident response
- [MasterChef Guide](MASTERCHEF_GUIDE.md) - Contract interactions

---

**Last Updated**: October 24, 2025
**Version**: 1.0
**Status**: Production Ready

🚀 **Monitoring automation complete!**
