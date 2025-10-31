# 🤖 How to Make AI Monitoring Automation LIVE

**Purpose:** This guide shows you how to activate the autonomous AI monitoring system that will monitor all 21 validators 24/7.

---

## 📋 Quick Summary

The AI monitoring system has 3 tiers that automatically escalate issues:

```
Tier 1: Ollama (Local) → Quick checks every 5 min (FREE)
           ↓ Issues?
Tier 2: GPT-4 (API) → Technical analysis ($0.02/call)
           ↓ Critical?
Tier 3: Claude (API) → Final decisions ($0.05/call)
```

**Cost:** ~$35-45/month (optimized to only call APIs when needed)

---

## ✅ Prerequisites (Already Done)

- [x] Ollama deployed and running
- [x] GPT-4 API key configured
- [x] Claude API key configured
- [x] Monitoring server operational
- [x] Prometheus scraping validators

---

## 🚀 Activation Steps (3 Options)

### Option 1: One-Command Activation (Easiest)

**Just run this:**
```bash
cd ~/Desktop/etrid
./ACTIVATE_AI_MONITORING.sh
```

**What it does:**
1. Copies AI monitoring code to server
2. Installs Python dependencies
3. Creates GLOBAL_MEMORY.md file
4. Starts AI orchestrator in background
5. Verifies everything is running

**Time:** ~2-3 minutes

---

### Option 2: Manual Step-by-Step

If you want to do it manually or understand each step:

#### Step 1: Copy AI Code to Server
```bash
scp -i ~/.ssh/gizzi-validator -r \
  ~/Desktop/etrid/ai-monitoring \
  compiler-dev01@98.71.91.84:/tmp/

ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "sudo mkdir -p /opt/ai-monitoring && \
   sudo cp -r /tmp/ai-monitoring/* /opt/ai-monitoring/"
```

#### Step 2: Install Dependencies
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "sudo apt update && \
   sudo apt install -y python3-pip && \
   pip3 install anthropic openai requests python-dotenv"
```

#### Step 3: Create GLOBAL_MEMORY File
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "sudo mkdir -p /opt/ai-monitoring && \
   sudo touch /opt/ai-monitoring/GLOBAL_MEMORY.md && \
   sudo chmod 666 /opt/ai-monitoring/GLOBAL_MEMORY.md"
```

#### Step 4: Start AI Monitoring
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "cd /opt/ai-monitoring && \
   nohup python3 orchestrator.py > /var/log/ai-monitoring.log 2>&1 &"
```

#### Step 5: Verify It's Running
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "ps aux | grep orchestrator"

# Should show python3 orchestrator.py running
```

---

### Option 3: Systemd Service (Production - Most Reliable)

For automatic restart on reboot and proper service management:

#### Create Service File
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84

sudo tee /etc/systemd/system/ai-monitoring.service > /dev/null <<'EOF'
[Unit]
Description=Ëtrid AI Monitoring System
After=network.target prometheus.service ollama.service

[Service]
Type=simple
User=root
WorkingDirectory=/opt/ai-monitoring
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
ExecStart=/usr/bin/python3 /opt/ai-monitoring/orchestrator.py
Restart=always
RestartSec=10
StandardOutput=append:/var/log/ai-monitoring.log
StandardError=append:/var/log/ai-monitoring.log

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable ai-monitoring
sudo systemctl start ai-monitoring

# Check status
sudo systemctl status ai-monitoring
```

#### Service Management Commands
```bash
# Start
sudo systemctl start ai-monitoring

# Stop
sudo systemctl stop ai-monitoring

# Restart
sudo systemctl restart ai-monitoring

# Status
sudo systemctl status ai-monitoring

# View logs
journalctl -u ai-monitoring -f
```

---

## 📊 How to Monitor the AI System

### Watch AI Decisions in Real-Time
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md"
```

**What you'll see:**
- Every monitoring cycle (every 5 minutes)
- Which AI tier was used (Ollama, GPT-4, or Claude)
- What issues were detected
- What actions were taken
- Cost tracking

**Example output:**
```
## [2025-10-31 17:00 UTC] consensus-dev01
**AI Tier:** Tier 1 (Ollama)
**Decision:** all_healthy
**Status:** LOW
**Tags:** #monitoring #validator-health #healthy

**Summary:** All 2 validators healthy

**Validator Status:**
- ✅ Validator #6 (consensus-dev01): 12 peers, block 45829
- ✅ Validator #7 (runtime-dev01): 15 peers, block 45829

**Cost This Cycle:** Ollama=1, GPT-4=0, Claude=0
```

### View System Logs
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "tail -f /var/log/ai-monitoring.log"
```

### Check If System Is Running
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "ps aux | grep orchestrator"
```

---

## 🎯 What Happens When Activated

### Automatic Monitoring Cycle (Every 5 Minutes)

1. **Tier 1 (Ollama)** checks all validators
   - If all healthy → Logs to GLOBAL_MEMORY, done ✅
   - If issues detected → Escalates to Tier 2

2. **Tier 2 (GPT-4)** analyzes the issue
   - Performs technical diagnosis
   - Recommends actions
   - If critical → Escalates to Tier 3
   - If not critical → Executes actions, done ✅

3. **Tier 3 (Claude)** makes final decision
   - Reviews all data
   - Makes executive decision
   - Coordinates across AI devs
   - Approves/executes actions ✅

### Automatic Actions

The AI can automatically:
- ✅ Restart offline validators
- ✅ Alert on low peer counts
- ✅ Investigate finalization lag
- ✅ Coordinate network-wide issues
- ✅ Log all decisions for audit

**Safety:** All actions are logged. You can review GLOBAL_MEMORY anytime.

---

## 💰 Cost Optimization

### How It Saves Money

**Without Optimization:**
- Every check calls Claude: $0.05 × 288 calls/day = $14.40/day = $432/month ❌

**With 3-Tier Optimization:**
- 90% of checks: Ollama (FREE)
- 9% need GPT-4: ~26 calls/day × $0.02 = $0.52/day
- 1% need Claude: ~3 calls/day × $0.05 = $0.15/day
- **Total: ~$20/month** ✅ (95% savings!)

### Current Settings

In `.env` file:
```
OPTIMIZED_MODE=true      # Only call expensive AIs when needed
MONITORING_INTERVAL=300  # Check every 5 minutes
```

### To Change Settings

Edit on server:
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84

sudo nano /opt/ai-monitoring/.env

# Change MONITORING_INTERVAL to adjust frequency:
# 300 = 5 minutes (recommended)
# 600 = 10 minutes (lower cost)
# 60 = 1 minute (higher cost, more responsive)

# Restart monitoring
sudo systemctl restart ai-monitoring
```

---

## 🔧 Troubleshooting

### AI Monitoring Not Starting

**Check Python dependencies:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "python3 -c 'import anthropic, openai, requests; print(\"OK\")'"
```

If error, reinstall:
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "pip3 install --user anthropic openai requests python-dotenv"
```

**Check API keys:**
```bash
cat ~/Desktop/etrid/ai-monitoring/.env
# Verify all API keys are present and valid
```

**Check logs for errors:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "cat /var/log/ai-monitoring.log"
```

### Ollama Not Responding

**Check if Ollama is running:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "systemctl status ollama"
```

**Test Ollama:**
```bash
curl http://98.71.91.84:11434/api/generate -d '{
  "model": "llama3.2:latest",
  "prompt": "test",
  "stream": false
}'
```

**Restart if needed:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "sudo systemctl restart ollama"
```

### No Decisions Appearing in GLOBAL_MEMORY

**Check if file exists:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "ls -la /opt/ai-monitoring/GLOBAL_MEMORY.md"
```

**Check permissions:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "sudo chmod 666 /opt/ai-monitoring/GLOBAL_MEMORY.md"
```

**Check if orchestrator is running:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "ps aux | grep orchestrator"
```

---

## 📈 Success Indicators

### You'll know it's working when:

✅ **Process is running:**
```bash
ps aux | grep orchestrator
# Shows: python3 orchestrator.py
```

✅ **GLOBAL_MEMORY is updating:**
```bash
ls -lh /opt/ai-monitoring/GLOBAL_MEMORY.md
# Shows: file size increasing
```

✅ **New entries every 5 minutes:**
```bash
tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md
# Shows: New entries with timestamps
```

✅ **Validators being checked:**
```bash
grep "Validator #" /opt/ai-monitoring/GLOBAL_MEMORY.md | tail -20
# Shows: All your validators listed
```

---

## 🎉 What to Expect

### First 24 Hours

- ✅ AI learns normal validator behavior
- ✅ Mostly Tier 1 (Ollama) calls (FREE)
- ✅ Occasional Tier 2 (GPT-4) for diagnostics
- ✅ Rare Tier 3 (Claude) for critical decisions

### Ongoing Operation

- **Normal day:** ~100% Tier 1 (Ollama), cost = $0
- **Minor issues:** ~90% Tier 1, ~10% Tier 2, cost = ~$1-2
- **Major incident:** ~70% Tier 1, ~20% Tier 2, ~10% Tier 3, cost = ~$5-10

**Average monthly cost:** $35-45

---

## 🛑 How to Stop/Pause Monitoring

### Temporary Stop
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "pkill -f orchestrator.py"
```

### Permanent Stop (If using systemd)
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "sudo systemctl stop ai-monitoring && \
   sudo systemctl disable ai-monitoring"
```

### Restart
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "cd /opt/ai-monitoring && \
   nohup python3 orchestrator.py > /var/log/ai-monitoring.log 2>&1 &"

# Or if using systemd:
sudo systemctl start ai-monitoring
```

---

## 📚 Additional Resources

**Documentation:**
- Full deployment report: `~/Desktop/etrid/FINAL_DEPLOYMENT_REPORT.md`
- Credentials & info: `~/Desktop/etrid/CRITICAL_INFO_AND_CREDENTIALS.txt`
- Configuration summary: `~/Desktop/etrid/CONFIGURATION_COMPLETE.md`

**Key Files:**
- AI configuration: `~/Desktop/etrid/ai-monitoring/.env`
- Activation script: `~/Desktop/etrid/ACTIVATE_AI_MONITORING.sh`
- Validator keys: `~/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json`

**Monitoring URLs:**
- Grafana: http://98.71.91.84:3000 (admin / G1zzi!Pwr2025$)
- Prometheus: http://98.71.91.84:9090
- Ollama API: http://98.71.91.84:11434

---

## 🚀 Ready to Activate?

**Run this command:**
```bash
cd ~/Desktop/etrid
./ACTIVATE_AI_MONITORING.sh
```

Then watch it work:
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84 \
  "tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md"
```

**That's it! Your autonomous AI monitoring is now live.** 🎉

---

*Last Updated: October 31, 2025*
*Status: Ready for activation*
