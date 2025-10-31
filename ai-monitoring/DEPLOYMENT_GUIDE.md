# AI Dev Monitoring Deployment Guide
## Complete Setup Instructions for 24/7 VM Deployment

**Target:** Validator 1 (Gizzi) - 64.181.215.19
**Run Duration:** 24/7 on VM (not your Mac)
**Cost:** ~$56/month (optimized mode)

---

## 📋 Prerequisites

### 1. Get Claude API Key

**Step-by-Step:**

1. **Go to Anthropic Console:**
   ```
   https://console.anthropic.com/
   ```

2. **Sign in / Create Account:**
   - Use your email
   - If you already have Claude Desktop subscription, use the same email

3. **Navigate to API Keys:**
   ```
   https://console.anthropic.com/settings/keys
   ```

4. **Create New Key:**
   - Click "Create Key"
   - Name: "Etrid Validator Monitoring"
   - Copy the key (starts with `sk-ant-api03-...`)
   - **IMPORTANT:** Save this key somewhere safe! You won't be able to see it again.

5. **Add Payment Method:**
   - Go to: https://console.anthropic.com/settings/billing
   - Add credit card
   - Set up billing alerts (recommended: alert at $50, $100)

**Note:** Claude API is separate from Claude Desktop subscription
- Desktop: $20/month (what you already have)
- API: Pay per use (~$56/month optimized for this project)

---

## 🚀 Quick Deployment (Use Another Claude Instance)

**Copy the entire contents of `CLAUDE_DEPLOYMENT_PROMPT.md` to a new Claude terminal session.**

That Claude instance will:
1. SSH to the Gizzi validator VM
2. Install all dependencies
3. Deploy all Python files
4. Configure systemd service
5. Start 24/7 monitoring

**Estimated Time:** 10-15 minutes

---

## 🛠️ Manual Deployment (If You Prefer)

### Step 1: Prepare Local Files

All files are ready in `/Users/macbook/Desktop/etrid/ai-monitoring/`:
```
ai-monitoring/
├── validator_monitor.py      (200 lines)
├── ai_dev_workers.py          (250 lines)
├── orchestrator.py            (150 lines)
├── DEPLOYMENT_GUIDE.md        (this file)
└── CLAUDE_DEPLOYMENT_PROMPT.md (for automated deployment)
```

### Step 2: Copy Files to VM

```bash
# Copy validator-ips.json
scp -i ~/.ssh/gizzi-validator \
  /Users/macbook/Desktop/etrid/validator-ips.json \
  ubuntu@64.181.215.19:/tmp/

# Copy Python files
scp -i ~/.ssh/gizzi-validator \
  /Users/macbook/Desktop/etrid/ai-monitoring/*.py \
  ubuntu@64.181.215.19:/tmp/
```

### Step 3: SSH to VM and Setup

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
```

Once on VM:

```bash
# Create directory
sudo mkdir -p /opt/ai-monitoring
sudo chown ubuntu:ubuntu /opt/ai-monitoring

# Move files
mv /tmp/validator_monitor.py /opt/ai-monitoring/
mv /tmp/ai_dev_workers.py /opt/ai-monitoring/
mv /tmp/orchestrator.py /opt/ai-monitoring/
mv /tmp/validator-ips.json /opt/ai-monitoring/

# Install dependencies
sudo apt-get update
sudo apt-get install -y python3-pip
pip3 install anthropic paramiko requests

# Copy SSH key
cp ~/.ssh/authorized_keys ~/.ssh/gizzi-validator
chmod 600 ~/.ssh/gizzi-validator

# Create memory log
touch /opt/ai-monitoring/GLOBAL_MEMORY.md

# Test run
cd /opt/ai-monitoring
export ANTHROPIC_API_KEY='your-api-key-here'
python3 orchestrator.py
```

Press Ctrl+C after one cycle to verify it works.

### Step 4: Create Systemd Service

```bash
sudo tee /etc/systemd/system/ai-dev-monitoring.service > /dev/null <<'EOF'
[Unit]
Description=AI Dev Blockchain Monitoring
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/opt/ai-monitoring
Environment="ANTHROPIC_API_KEY=YOUR_API_KEY_HERE"
Environment="VALIDATOR_IPS_PATH=/opt/ai-monitoring/validator-ips.json"
Environment="SSH_KEY_PATH=/home/ubuntu/.ssh/gizzi-validator"
Environment="PROMETHEUS_URL=http://localhost:9090"
Environment="MEMORY_PATH=/opt/ai-monitoring/GLOBAL_MEMORY.md"
Environment="MONITOR_INTERVAL=300"
Environment="OPTIMIZED=true"
ExecStart=/usr/bin/python3 /opt/ai-monitoring/orchestrator.py
Restart=always
RestartSec=10
StandardOutput=append:/var/log/ai-dev-monitoring.log
StandardError=append:/var/log/ai-dev-monitoring-error.log

[Install]
WantedBy=multi-user.target
EOF
```

**Replace `YOUR_API_KEY_HERE` with your actual API key!**

### Step 5: Start Service

```bash
# Reload systemd
sudo systemctl daemon-reload

# Start service
sudo systemctl start ai-dev-monitoring

# Enable on boot
sudo systemctl enable ai-dev-monitoring

# Check status
sudo systemctl status ai-dev-monitoring

# Watch logs
sudo tail -f /var/log/ai-dev-monitoring.log
```

---

## 📊 Monitoring the System

### Check Logs

```bash
# Real-time logs
sudo journalctl -u ai-dev-monitoring -f

# Last 100 lines
sudo journalctl -u ai-dev-monitoring -n 100

# Today's logs
sudo journalctl -u ai-dev-monitoring --since today
```

### Check Memory Log

```bash
# See AI dev decisions
tail -50 /opt/ai-monitoring/GLOBAL_MEMORY.md

# Watch live
tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md
```

### Check Service Status

```bash
# Is it running?
sudo systemctl status ai-dev-monitoring

# Restart if needed
sudo systemctl restart ai-dev-monitoring

# Stop
sudo systemctl stop ai-dev-monitoring
```

---

## 🔧 Configuration Options

### Adjust Monitoring Interval

**In systemd service:**
```bash
Environment="MONITOR_INTERVAL=300"
```

Options:
- `60` = 1 minute (responsive, higher cost ~$560/month)
- `300` = 5 minutes (recommended, ~$56/month)
- `900` = 15 minutes (very low cost ~$19/month)

**Change and restart:**
```bash
sudo systemctl daemon-reload
sudo systemctl restart ai-dev-monitoring
```

### Disable Optimization (Always Call Claude)

```bash
Environment="OPTIMIZED=false"
```

This will call Claude every cycle even if no issues.
- Cost: ~$560/month
- Useful for: Getting detailed reasoning every cycle

### Use Different Prometheus Server

If Prometheus is on a different server:
```bash
Environment="PROMETHEUS_URL=http://MONITORING_SERVER_IP:9090"
```

---

## 💰 Cost Monitoring

### Check API Usage

1. Go to: https://console.anthropic.com/settings/usage
2. View daily/monthly usage
3. Set up billing alerts

### Expected Costs (Optimized Mode)

**Assumptions:**
- 5-minute intervals (288 cycles/day)
- 10% of cycles have issues (Claude called)
- 12 AI devs × ~800 tokens input, ~200 output

**Math:**
- Cycles with Claude: 288 × 0.10 = 28.8 per day
- Total AI dev calls: 28.8 × 12 = 345.6 calls/day
- Monthly calls: 345.6 × 30 = 10,368 calls

**Cost:**
- Input: 10,368 × 800 × $3/1M = $24.88
- Output: 10,368 × 200 × $15/1M = $31.10
- **Total: ~$56/month**

---

## 🚨 Troubleshooting

### Issue: Service won't start

```bash
# Check detailed error
sudo journalctl -u ai-dev-monitoring -n 50

# Common fixes:
# 1. API key not set
sudo nano /etc/systemd/system/ai-dev-monitoring.service
# Add correct API key

# 2. Missing dependencies
pip3 install anthropic paramiko requests

# 3. Wrong paths
ls -la /opt/ai-monitoring/
```

### Issue: Can't connect to validators

```bash
# Test SSH to a validator
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 "echo OK"

# If fails, check key permissions
chmod 600 ~/.ssh/gizzi-validator
```

### Issue: Prometheus not found

```bash
# Check if Prometheus is running
curl http://localhost:9090/api/v1/query?query=up

# If not running, install Prometheus first
# (See MONITORING_INFRASTRUCTURE_GUIDE.md)
```

### Issue: High API costs

```bash
# Check how often Claude is called
grep "analyze_with_claude" /var/log/ai-dev-monitoring.log | wc -l

# If too many calls:
# 1. Increase interval (5 min → 15 min)
# 2. Ensure OPTIMIZED=true
```

---

## 📞 Support

**Logs Location:**
- Service logs: `/var/log/ai-dev-monitoring.log`
- Error logs: `/var/log/ai-dev-monitoring-error.log`
- Memory log: `/opt/ai-monitoring/GLOBAL_MEMORY.md`
- Systemd journal: `journalctl -u ai-dev-monitoring`

**Common Commands:**
```bash
# Restart service
sudo systemctl restart ai-dev-monitoring

# Check logs
sudo tail -f /var/log/ai-dev-monitoring.log

# Check API calls
grep "Claude API" /var/log/ai-dev-monitoring.log | tail -20

# Check validator status
grep "healthy" /opt/ai-monitoring/GLOBAL_MEMORY.md | tail -20
```

---

## ✅ Verification Checklist

After deployment, verify:

- [ ] Service is running: `systemctl status ai-dev-monitoring`
- [ ] Logs show AI devs initializing
- [ ] First monitoring cycle completes
- [ ] GLOBAL_MEMORY.md has entries
- [ ] No errors in logs
- [ ] API key is working (no auth errors)
- [ ] Validators are being checked
- [ ] Service restarts on reboot

---

## 🎯 What Happens Now

**Every 5 minutes:**
1. Each of 12 AI devs checks their assigned validators
2. Quick health check (free)
3. If issues found → Call Claude API for analysis
4. Claude decides: restart, alert, or investigate
5. Actions executed automatically
6. Everything logged to GLOBAL_MEMORY.md

**Result:**
- Autonomous 24/7 monitoring
- AI-powered diagnostics
- Auto-restart of failed validators
- Complete audit trail
- Cost: ~$56/month

---

**Status:** Ready for deployment
**Next:** Either use automated deployment (CLAUDE_DEPLOYMENT_PROMPT.md) or follow manual steps above
