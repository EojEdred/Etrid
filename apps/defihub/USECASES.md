# 🏦 ËTRID DeFi Hub - Real-World Use Cases

## 📋 Table of Contents
- [Daily Operations](#daily-operations)
- [Validator Management](#validator-management)
- [Development Workflows](#development-workflows)
- [Monitoring & Alerts](#monitoring--alerts)
- [Troubleshooting](#troubleshooting)
- [Advanced Scenarios](#advanced-scenarios)

---

## 👨‍💼 Daily Operations

### Use Case 1: Morning Validator Check
**Scenario:** You want to check all your validators are running properly each morning.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Quick status check
npm run validator:status

# If any issues, run full monitoring
npm run validator:monitor
```

**Expected Output:**
```
🟢 #7 Compiler Dev - Azure North Europe
🟢 #8 Network Node - Azure West Europe
🔴 #14 Validator X - Connection failed

✅ Running: 15 | 🔴 Stopped: 1
```

**Action Items:**
- If you see 🔴, investigate that validator
- Check the AI recommendations
- Restart if needed: `node validator-cli.js restart 14`

---

### Use Case 2: Starting Work Day
**Scenario:** You're starting your work day and need all tools running.

```bash
cd /Users/macbook/Desktop/etrid

# Terminal 1: Start web UIs
./scripts/start-all-web-uis.sh

# Terminal 2: Start API
cd deployment/website/website-deployment/api
npm start

# Terminal 3: Start monitoring
cd pinokio
npm run validator:watch

# Terminal 4: Check everything
./scripts/status-web-uis.sh
```

**Access your tools:**
- DeFi Hub: https://etrid.org/defi-hub/
- Validator Dashboard: http://localhost:3002
- MasterChef: http://localhost:3001
- All others: Check status output

---

### Use Case 3: End of Day Shutdown
**Scenario:** Clean shutdown of all services before going home.

```bash
cd /Users/macbook/Desktop/etrid

# Stop web UIs
./scripts/stop-all-web-uis.sh

# In other terminals, Ctrl+C to stop:
# - API server
# - Monitoring

# Verify everything stopped
./scripts/status-web-uis.sh
```

---

## 🛡️ Validator Management

### Use Case 4: Adding a New Validator
**Scenario:** You've deployed a new validator and need to add it to monitoring.

**Step 1: Get validator info**
```bash
# SSH into new validator
ssh admin@new-validator-ip

# Get details
hostname
cat /etc/etrid/validator.conf
```

**Step 2: Add to config**
```bash
cd /Users/macbook/Desktop/etrid

# Edit configuration
nano infrastructure/config/validator-ips.json

# Add new entry:
{
  "id": 22,
  "name": "New Production Validator",
  "region": "Azure Southeast Asia",
  "role": "Network Node",
  "ip": "20.xxx.xxx.xxx",
  "sshUser": "admin",
  "accessible": true
}

# Save and validate
cat infrastructure/config/validator-ips.json | jq .
```

**Step 3: Test connection**
```bash
cd pinokio

# Test SSH
node validator-cli.js status 22

# Add to monitoring
npm run validator:monitor
```

**Step 4: Verify in DeFi Hub**
- Go to https://etrid.org/defi-hub/
- Click "Refresh"
- New validator should appear

---

### Use Case 5: Checking Disk Space Across All Validators
**Scenario:** You want to see which validators are running low on disk space.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Check all validators
node validator-cli.js exec-all "df -h / | tail -1"
```

**Output:**
```
🚀 Executing on 16 validators...

✅ #7 Compiler Dev
/dev/sda1  100G  45G  55G  45%  /

✅ #8 Network Node
/dev/sda1  100G  78G  22G  78%  /

⚠️  #8 has high disk usage (78%)
```

**Action:** Free up space on high-usage validators
```bash
# Connect to validator #8
ssh admin@validator-8-ip

# Clean up
sudo apt autoclean
sudo apt autoremove
sudo journalctl --vacuum-time=7d

# Verify
df -h
```

---

### Use Case 6: Checking Memory Usage
**Scenario:** A validator is slow, you suspect memory issues.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Check specific validator
node validator-cli.js exec 7 "free -h && top -bn1 | head -20"
```

**Analysis:**
```
Output shows:
              total        used        free
Mem:          16Gi        14Gi        2Gi

# This validator is using 87% memory - might need attention
```

**Solutions:**
```bash
# Restart validator service to free memory
node validator-cli.js restart 7

# Or SSH in and investigate
ssh admin@validator-7-ip
systemctl status etrid-validator
journalctl -u etrid-validator -n 100
```

---

### Use Case 7: Updating Validator Software
**Scenario:** You need to update the validator software on all nodes.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# First, check current versions
node validator-cli.js exec-all "etrid-validator --version"

# Update one validator first (test)
node validator-cli.js exec 7 "sudo systemctl stop etrid-validator && sudo apt update && sudo apt install -y etrid-validator && sudo systemctl start etrid-validator"

# Verify it worked
node validator-cli.js status 7

# If successful, update all others
# Do in batches to maintain network health
node validator-cli.js exec-all "sudo systemctl stop etrid-validator && sudo apt update && sudo apt install -y etrid-validator && sudo systemctl start etrid-validator"
```

**Monitor during update:**
```bash
# In another terminal, watch status
watch -n 30 'cd /Users/macbook/Desktop/etrid/pinokio && npm run validator:status'
```

---

## 💻 Development Workflows

### Use Case 8: Testing a New Feature
**Scenario:** You're developing a new feature and need to test it.

```bash
# 1. Start dev environment
cd /Users/macbook/Desktop/etrid
./scripts/start-all-web-uis.sh

# 2. Make your changes in the code
# Edit apps/your-app/...

# 3. Dev server auto-reloads, test at:
# http://localhost:3000 (or appropriate port)

# 4. Check logs for errors
tail -f /tmp/etrid-your-app.log

# 5. When done, stop
./scripts/stop-all-web-uis.sh
```

---

### Use Case 9: Building for Production
**Scenario:** You're ready to deploy your changes to production.

```bash
cd /Users/macbook/Desktop/etrid

# 1. Build all apps
./scripts/build-all-web-uis.sh

# 2. Test builds locally
cd apps/lightning-landing
npm run start  # Runs production build

# 3. If good, deploy
cd ../../deployment/website/website-deployment
python3 upload-defi-hub.py
```

---

### Use Case 10: Debugging an Application
**Scenario:** Validator Dashboard isn't loading properly.

```bash
cd /Users/macbook/Desktop/etrid

# 1. Check if it's running
./scripts/status-web-uis.sh

# 2. View logs
tail -50 /tmp/etrid-validator-dashboard.log

# 3. Check port
lsof -i :3002

# 4. Restart just this app
# Stop
kill $(lsof -t -i:3002)

# Start
cd apps/validator-dashboard
PORT=3002 npm run dev &

# 5. Check logs again
tail -f /tmp/etrid-validator-dashboard.log
```

---

## 📊 Monitoring & Alerts

### Use Case 11: Setting Up 24/7 Monitoring
**Scenario:** You want continuous monitoring that runs even when you're away.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Start monitoring in background
nohup node ai-validator-monitor.js continuous 10 > /tmp/monitoring.log 2>&1 &

# Save the PID
echo $! > /tmp/monitoring.pid

# Check it's running
tail -f /tmp/monitoring.log

# To stop later
kill $(cat /tmp/monitoring.pid)
```

**Or use a systemd service (Linux):**
```bash
# Create service file
sudo nano /etc/systemd/system/etrid-monitoring.service

# Add:
[Unit]
Description=ETRID Validator Monitoring
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/Users/macbook/Desktop/etrid/pinokio
ExecStart=/usr/bin/node ai-validator-monitor.js continuous 10
Restart=on-failure

[Install]
WantedBy=multi-user.target

# Enable and start
sudo systemctl enable etrid-monitoring
sudo systemctl start etrid-monitoring

# Check status
sudo systemctl status etrid-monitoring
```

---

### Use Case 12: Getting Alerts for Issues
**Scenario:** You want to be notified when a validator goes down.

**Create a monitoring script:**
```bash
cd /Users/macbook/Desktop/etrid

# Create alert script
cat > scripts/alert-on-issues.sh << 'EOF'
#!/bin/bash
cd /Users/macbook/Desktop/etrid/pinokio

# Run monitoring
node ai-validator-monitor.js monitor > /tmp/monitor-result.txt 2>&1

# Check for critical issues
if grep -q "CRITICAL" /tmp/monitor-result.txt; then
    # Send email (requires mail configured)
    cat /tmp/monitor-result.txt | mail -s "ETRID Validator Alert!" your@email.com

    # Or use curl to send to Slack/Discord
    # curl -X POST https://hooks.slack.com/... -d "payload={\"text\":\"Validator alert!\"}"

    echo "Alert sent!"
fi
EOF

chmod +x scripts/alert-on-issues.sh

# Run every 10 minutes with cron
# crontab -e
# Add: */10 * * * * /Users/macbook/Desktop/etrid/scripts/alert-on-issues.sh
```

---

### Use Case 13: Generating Weekly Reports
**Scenario:** You want a weekly summary of validator health.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Generate report
cat > generate-weekly-report.sh << 'EOF'
#!/bin/bash
REPORT_DIR="/Users/macbook/Desktop/etrid/weekly-reports"
mkdir -p $REPORT_DIR

# Get all reports from last 7 days
cat reports/validator-report-*.json | \
  jq -s '. | map(select(.timestamp > (now - 604800))) | {
    period: "Last 7 days",
    average_health: (map(.summary.averageHealth) | add / length),
    total_alerts: (map(.summary.criticalAlerts + .summary.warningAlerts) | add),
    uptime_pct: ((map(select(.summary.runningValidators > 0)) | length) / length * 100)
  }' > "$REPORT_DIR/weekly-$(date +%Y%m%d).json"

echo "Report generated!"
EOF

chmod +x generate-weekly-report.sh
./generate-weekly-report.sh
```

---

## 🔧 Troubleshooting

### Use Case 14: A Validator Stopped Responding
**Scenario:** Validator #7 stopped responding in monitoring.

**Step 1: Check connection**
```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Try to connect
ssh -i $SSH_KEY_PATH compiler-dev01@20.224.104.239

# If SSH works, check service
systemctl status etrid-validator
```

**Step 2: Check monitoring**
```bash
# Force check
node validator-cli.js status 7
```

**Step 3: Restart if needed**
```bash
# Via CLI
node validator-cli.js restart 7

# Or via SSH
ssh validator-7
sudo systemctl restart etrid-validator
sudo systemctl status etrid-validator
```

**Step 4: Verify in DeFi Hub**
- Go to https://etrid.org/defi-hub/
- Click "Refresh"
- Validator should show 🟢

---

### Use Case 15: Port Conflicts
**Scenario:** Can't start web UIs because ports are in use.

```bash
# Find what's using the ports
lsof -i :3000-3004

# Kill specific process
kill $(lsof -t -i:3001)

# Or kill all node processes (nuclear option)
killall node

# Clean up PID files
rm /tmp/etrid-*.pid

# Try starting again
./scripts/start-all-web-uis.sh
```

---

### Use Case 16: API Not Returning Data
**Scenario:** DeFi Hub shows no validator data.

**Check 1: Is API running?**
```bash
curl http://localhost:3100/api/health
```

**Check 2: Are there reports?**
```bash
ls -lh /Users/macbook/Desktop/etrid/pinokio/reports/
```

**Check 3: Generate new report**
```bash
cd /Users/macbook/Desktop/etrid/pinokio
npm run validator:monitor
```

**Check 4: Test API again**
```bash
curl http://localhost:3100/api/validator-status | jq .
```

---

## 🚀 Advanced Scenarios

### Use Case 17: Batch Operations on Validators
**Scenario:** You need to run multiple commands on all validators.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Create script
cat > batch-operations.sh << 'EOF'
#!/bin/bash

# Update system
node validator-cli.js exec-all "sudo apt update"

# Upgrade packages
node validator-cli.js exec-all "sudo apt upgrade -y"

# Restart validator service
node validator-cli.js exec-all "sudo systemctl restart etrid-validator"

# Check status
node validator-cli.js status-all
EOF

chmod +x batch-operations.sh
./batch-operations.sh
```

---

### Use Case 18: Custom Health Checks
**Scenario:** You want to check specific metrics not in default monitoring.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Check network latency
node validator-cli.js exec-all "ping -c 3 8.8.8.8 | tail -1"

# Check validator sync status
node validator-cli.js exec-all "etrid-validator info | grep 'Block Height'"

# Check certificate expiration
node validator-cli.js exec-all "openssl x509 -in /etc/etrid/cert.pem -noout -dates"
```

---

### Use Case 19: Emergency Validator Replacement
**Scenario:** Validator #8 hardware failed, you need to replace it.

**Step 1: Deploy new validator**
```bash
# SSH into new server
ssh new-validator-ip

# Install validator software
sudo apt install etrid-validator

# Configure
sudo nano /etc/etrid/validator.conf
```

**Step 2: Update config**
```bash
cd /Users/macbook/Desktop/etrid

# Edit config
nano infrastructure/config/validator-ips.json

# Update validator #8 IP
{
  "id": 8,
  "name": "Network Node (Replacement)",
  "region": "Azure West Europe",
  "role": "Network Node",
  "ip": "NEW-IP-HERE",
  "sshUser": "admin",
  "accessible": true
}
```

**Step 3: Test**
```bash
cd pinokio
node validator-cli.js status 8
```

**Step 4: Monitor**
```bash
# Run monitoring to verify
npm run validator:monitor

# Check in DeFi Hub
# https://etrid.org/defi-hub/
```

---

### Use Case 20: Performance Optimization
**Scenario:** You want to identify slow validators.

```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Check response times
for i in {7..21}; do
  echo "Validator #$i:"
  time node validator-cli.js exec $i "echo 'pong'"
done

# Check resource usage
node validator-cli.js exec-all "top -bn1 | head -5"

# Generate full report
npm run validator:monitor

# Analyze reports
cd reports
cat validator-report-*.json | jq '.validators[] | select(.health < 80) | {id: .validator.id, name: .validator.name, health: .health, alerts: .alerts}'
```

---

## 📝 Summary

This guide covered:
- ✅ Daily operations and workflows
- ✅ Validator management tasks
- ✅ Development and debugging
- ✅ Monitoring and alerting
- ✅ Troubleshooting scenarios
- ✅ Advanced use cases

**For complete command reference, see:** `DEFIHUB_COMMANDS.md`

**For full feature guide, see:** `DEFIHUB_COMPLETE.md`
