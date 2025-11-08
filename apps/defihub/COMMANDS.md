# 🏦 ËTRID DeFi Hub - Complete Command Reference

## 📋 Table of Contents
- [Web UI Management](#web-ui-management)
- [Validator Management](#validator-management)
- [API Server](#api-server)
- [Configuration](#configuration)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)

---

## 🌐 Web UI Management

### Start All Web UIs
```bash
cd /Users/macbook/Desktop/etrid
./scripts/start-all-web-uis.sh
```

**What it does:**
- Starts all 5 web applications as background processes
- Creates PID files in `/tmp/etrid-*.pid`
- Creates log files in `/tmp/etrid-*.log`
- Checks ports and dependencies automatically

**Applications started:**
- Lightning Landing (port 3000)
- MasterChef Dashboard (port 3001)
- Validator Dashboard (port 3002)
- Watchtower Monitor (port 3003)
- Wallet Web (port 3004)

### Stop All Web UIs
```bash
cd /Users/macbook/Desktop/etrid
./scripts/stop-all-web-uis.sh
```

**What it does:**
- Gracefully stops all running applications
- Cleans up PID files
- Optionally deletes log files

### Check Status
```bash
cd /Users/macbook/Desktop/etrid
./scripts/status-web-uis.sh
```

**What it does:**
- Shows which applications are running
- Displays PID and port information
- Shows URLs to access each application

### Build Production Bundles
```bash
cd /Users/macbook/Desktop/etrid
./scripts/build-all-web-uis.sh
```

**What it does:**
- Installs dependencies for all apps
- Builds production-ready bundles
- Shows build output locations

---

## 🛡️ Validator Management

### List All Validators
```bash
cd /Users/macbook/Desktop/etrid/pinokio
npm run validator:list
```
**OR**
```bash
node validator-cli.js list
```

**Output:**
```
================================================================================
📋 ËTRID VALIDATOR NETWORK
================================================================================

Total Validators: 21
✅ Accessible: 16
❌ Inaccessible: 5

ID  | Name                 | Region              | Role           | Status
--------------------------------------------------------------------------------
1   | Gizzi                | Azure West US       | Director       | ❌
2   | EojEdred             | Azure East US       | Director       | ❌
7   | Compiler Dev         | Azure North Europe  | Developer      | ✅
...
```

### Check Specific Validator Status
```bash
cd /Users/macbook/Desktop/etrid/pinokio
node validator-cli.js status 7
```

**What it shows:**
- Service status (running/stopped)
- Peer count
- Disk and memory usage
- Uptime
- Recent logs

### Check All Validator Statuses
```bash
cd /Users/macbook/Desktop/etrid/pinokio
npm run validator:status
```
**OR**
```bash
node validator-cli.js status-all
```

**Output:**
```
📊 Getting status for 16 validators...

🟢 #7 Compiler Dev         - Azure North Europe
🟡 #8 Network Node         - Azure West Europe
🔴 #14 Stopped Validator   - Connection failed
...

✅ Running: 15 | 🔴 Stopped: 1 | ❌ Failed: 0
```

### Execute Command on Validator
```bash
cd /Users/macbook/Desktop/etrid/pinokio
node validator-cli.js exec 7 "uptime"
```

**Common commands:**
```bash
# Check uptime
node validator-cli.js exec 7 "uptime"

# Check disk space
node validator-cli.js exec 7 "df -h"

# Check memory
node validator-cli.js exec 7 "free -h"

# View running processes
node validator-cli.js exec 7 "ps aux | grep etrid"

# Check network
node validator-cli.js exec 7 "netstat -an | grep ESTABLISHED"
```

### Execute on All Validators
```bash
cd /Users/macbook/Desktop/etrid/pinokio
node validator-cli.js exec-all "df -h"
```

**Use cases:**
```bash
# Check disk space on all validators
node validator-cli.js exec-all "df -h /"

# Check uptime on all
node validator-cli.js exec-all "uptime"

# Update system (be careful!)
node validator-cli.js exec-all "sudo apt update"
```

### View Validator Logs
```bash
cd /Users/macbook/Desktop/etrid/pinokio
node validator-cli.js logs 7 100
```

**Parameters:**
- First number: Validator ID
- Second number: Number of lines (default: 50)

### Restart Validator
```bash
cd /Users/macbook/Desktop/etrid/pinokio
node validator-cli.js restart 7
```

**What it does:**
- Restarts the validator service
- Waits 5 seconds
- Checks status after restart

---

## 🤖 AI Monitoring

### Run Monitoring Once
```bash
cd /Users/macbook/Desktop/etrid/pinokio
npm run validator:monitor
```
**OR**
```bash
node ai-validator-monitor.js monitor
```

**Output:**
```
================================================================================
🤖 AI-POWERED VALIDATOR MONITORING
================================================================================

⏳ Collecting metrics from 16 validators...

📊 Checking #7 Compiler Dev... 🟢 Health: 95/100
📊 Checking #8 Network Node... 🟡 Health: 72/100
...

================================================================================
📊 NETWORK SUMMARY
================================================================================
   Overall Health:      🟢 HEALTHY (Score: 87.3/100)
   Total Validators:    16
   Running Validators:  15/16
   Average Peer Count:  6.8

💡 AI RECOMMENDATIONS
   1. 🔴 [Service] Restart stopped validators: #14
   2. 🟡 [Network] Check connectivity: #9

💾 Report saved to: pinokio/reports/validator-report-1234567890.json
```

### Continuous Monitoring
```bash
cd /Users/macbook/Desktop/etrid/pinokio
npm run validator:watch
```
**OR**
```bash
node ai-validator-monitor.js continuous 10
```

**Parameters:**
- Number = interval in minutes (default: 10)

**What it does:**
- Runs monitoring every X minutes
- Saves reports to `pinokio/reports/`
- Continues until you press Ctrl+C

### Custom Monitoring Interval
```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Every 5 minutes
node ai-validator-monitor.js continuous 5

# Every hour
node ai-validator-monitor.js continuous 60

# Every 30 minutes
node ai-validator-monitor.js continuous 30
```

---

## 🔌 API Server

### Start API Server
```bash
cd /Users/macbook/Desktop/etrid/deployment/website/website-deployment/api
npm install  # First time only
npm start
```

**What it does:**
- Starts Express server on port 3100
- Serves validator status data
- Provides REST API endpoints

### API Endpoints

**Base URL:** `http://localhost:3100`

```bash
# Health check
curl http://localhost:3100/api/health

# All validators
curl http://localhost:3100/api/validator-status

# Specific validator
curl http://localhost:3100/api/validator-status/7

# Network summary
curl http://localhost:3100/api/network-summary

# AI recommendations
curl http://localhost:3100/api/recommendations

# Regional distribution
curl http://localhost:3100/api/regions
```

### API with jq (Pretty Print)
```bash
# Pretty print validator status
curl -s http://localhost:3100/api/validator-status | jq .

# Get just the summary
curl -s http://localhost:3100/api/network-summary | jq '.summary'

# Get validator #7 info
curl -s http://localhost:3100/api/validator-status/7 | jq .
```

---

## ⚙️ Configuration

### Edit Validator List
```bash
# Using nano
nano /Users/macbook/Desktop/etrid/infrastructure/config/validator-ips.json

# Using vim
vim /Users/macbook/Desktop/etrid/infrastructure/config/validator-ips.json

# Using VS Code
code /Users/macbook/Desktop/etrid/infrastructure/config/validator-ips.json
```

### Validate Configuration
```bash
cd /Users/macbook/Desktop/etrid

# Check JSON syntax
cat infrastructure/config/validator-ips.json | jq .

# Pretty print
cat infrastructure/config/validator-ips.json | jq . > temp.json && mv temp.json infrastructure/config/validator-ips.json
```

### Configuration Structure
```json
{
  "validators": [
    {
      "id": 1,
      "name": "Validator Name",
      "region": "Azure West US",
      "role": "Director/Bootstrap",
      "ip": "20.xxx.xxx.xxx",
      "sshUser": "username",
      "accessible": true
    }
  ]
}
```

### Add New Validator
```bash
# 1. Edit config
nano infrastructure/config/validator-ips.json

# 2. Add new entry:
{
  "id": 22,
  "name": "New Validator",
  "region": "Your Region",
  "role": "Network Node",
  "ip": "xxx.xxx.xxx.xxx",
  "sshUser": "admin",
  "accessible": true
}

# 3. Validate
cat infrastructure/config/validator-ips.json | jq .

# 4. Test connection
cd pinokio
node validator-cli.js status 22
```

### Set SSH Key
```bash
# Temporary (current session)
export SSH_KEY_PATH=~/.ssh/your-validator-key

# Permanent (add to ~/.zshrc or ~/.bashrc)
echo 'export SSH_KEY_PATH=~/.ssh/your-validator-key' >> ~/.zshrc
source ~/.zshrc

# Verify
echo $SSH_KEY_PATH
```

---

## 📊 Monitoring Reports

### View Latest Report
```bash
cd /Users/macbook/Desktop/etrid/pinokio

# List reports
ls -lh reports/

# View latest (pretty)
ls -t reports/validator-report-*.json | head -1 | xargs cat | jq .

# View summary only
ls -t reports/validator-report-*.json | head -1 | xargs cat | jq '.summary'
```

### Report Structure
```json
{
  "timestamp": "2025-11-08T12:34:56.789Z",
  "summary": {
    "totalValidators": 16,
    "runningValidators": 15,
    "averageHealth": 87.3,
    "averagePeerCount": 6.8,
    "criticalAlerts": 1,
    "warningAlerts": 2
  },
  "recommendations": [...],
  "validators": [...]
}
```

### Clean Old Reports
```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Delete reports older than 7 days
find reports/ -name "validator-report-*.json" -mtime +7 -delete

# Keep only last 10 reports
ls -t reports/validator-report-*.json | tail -n +11 | xargs rm

# Delete all reports
rm reports/validator-report-*.json
```

---

## 🐛 Troubleshooting Commands

### Check What's Using a Port
```bash
# Check specific port
lsof -i :3000

# Check all ËTRID ports
lsof -i :3000-3004

# Check API port
lsof -i :3100
```

### Kill Process on Port
```bash
# Kill process on port 3000
kill $(lsof -t -i:3000)

# Force kill
kill -9 $(lsof -t -i:3000)
```

### View Logs
```bash
# Real-time logs for all apps
tail -f /tmp/etrid-*.log

# Specific app
tail -f /tmp/etrid-lightning-landing.log

# Last 100 lines
tail -100 /tmp/etrid-masterchef-dashboard.log

# Follow with grep
tail -f /tmp/etrid-validator-dashboard.log | grep ERROR
```

### Check Node/NPM Versions
```bash
node --version
npm --version

# Should be Node 18+ and npm 9+
```

### Clear Node Modules
```bash
cd /Users/macbook/Desktop/etrid/apps/lightning-landing

# Remove and reinstall
rm -rf node_modules package-lock.json
npm install

# For wallet-web
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
rm -rf node_modules package-lock.json
npm install --legacy-peer-deps
```

### Check SSH Connection
```bash
# Test SSH to validator
ssh -i ~/.ssh/your-key username@validator-ip

# Test with verbose output
ssh -v -i ~/.ssh/your-key username@validator-ip

# Check key permissions
ls -la ~/.ssh/your-key
# Should be: -rw------- (600)

# Fix permissions
chmod 600 ~/.ssh/your-key
```

### Network Diagnostics
```bash
# Check if etrid.org is accessible
curl -I https://etrid.org

# Check DeFi Hub
curl -I https://etrid.org/defi-hub/

# Test API (if local)
curl -I http://localhost:3100/api/health

# DNS lookup
nslookup etrid.org

# Ping test
ping -c 4 etrid.org
```

---

## 🔄 Common Workflows

### Daily Startup
```bash
cd /Users/macbook/Desktop/etrid

# 1. Start web UIs
./scripts/start-all-web-uis.sh

# 2. Start API (new terminal)
cd deployment/website/website-deployment/api && npm start

# 3. Start monitoring (new terminal)
cd pinokio && npm run validator:watch

# 4. Check everything is running
./scripts/status-web-uis.sh
```

### Daily Shutdown
```bash
cd /Users/macbook/Desktop/etrid

# 1. Stop web UIs
./scripts/stop-all-web-uis.sh

# 2. Stop API (Ctrl+C in API terminal)

# 3. Stop monitoring (Ctrl+C in monitoring terminal)
```

### Check Validator Health
```bash
cd /Users/macbook/Desktop/etrid/pinokio

# Quick check
npm run validator:status

# Detailed check
npm run validator:monitor

# Specific validator
node validator-cli.js status 7
```

### Deploy Updates to Website
```bash
cd /Users/macbook/Desktop/etrid/deployment/website/website-deployment

# Make your changes to website/ files, then:
python3 upload-defi-hub.py
```

---

## 📚 Quick Reference Card

| Task | Command |
|------|---------|
| **Start all** | `./scripts/start-all-web-uis.sh` |
| **Stop all** | `./scripts/stop-all-web-uis.sh` |
| **Check status** | `./scripts/status-web-uis.sh` |
| **List validators** | `cd pinokio && npm run validator:list` |
| **Monitor once** | `cd pinokio && npm run validator:monitor` |
| **Monitor continuous** | `cd pinokio && npm run validator:watch` |
| **Start API** | `cd api && npm start` |
| **Edit validators** | `nano infrastructure/config/validator-ips.json` |
| **View logs** | `tail -f /tmp/etrid-*.log` |
| **Check port** | `lsof -i :3000` |

---

## 🆘 Emergency Commands

### Everything is broken - Fresh start
```bash
cd /Users/macbook/Desktop/etrid

# 1. Stop everything
./scripts/stop-all-web-uis.sh
killall node  # Nuclear option

# 2. Clean up
rm -f /tmp/etrid-*.pid
rm -f /tmp/etrid-*.log

# 3. Rebuild
./scripts/build-all-web-uis.sh

# 4. Restart
./scripts/start-all-web-uis.sh
```

### Validator monitoring not working
```bash
cd /Users/macbook/Desktop/etrid/pinokio

# 1. Check SSH key
echo $SSH_KEY_PATH
ls -la $SSH_KEY_PATH

# 2. Fix permissions
chmod 600 $SSH_KEY_PATH

# 3. Test connection
ssh -i $SSH_KEY_PATH username@validator-ip

# 4. Verify config
cat ../infrastructure/config/validator-ips.json | jq .
```

---

**For more help, see:**
- `DEFIHUB_COMPLETE.md` - Full feature guide
- `DEFIHUB_USECASES.md` - Real-world examples
- `pinokio/README.md` - Technical documentation
