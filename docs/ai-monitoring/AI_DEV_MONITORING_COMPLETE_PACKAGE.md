# AI Dev Monitoring - Complete Implementation Package
## Everything You Need to Deploy 24/7 Autonomous Validator Monitoring

**Date:** 2025-10-31
**Status:** Ready to Deploy
**Location:** `/Users/macbook/Desktop/etrid/ai-monitoring/`

---

## 📦 Package Contents

### Python Files (Production-Ready)
```
ai-monitoring/
├── validator_monitor.py          ✅ 200 lines - Collects validator status
├── ai_dev_workers.py              ✅ 250 lines - Claude API integration
├── orchestrator.py                ✅ 150 lines - Main coordinator
├── DEPLOYMENT_GUIDE.md            ✅ Complete deployment instructions
└── CLAUDE_DEPLOYMENT_PROMPT.md   ✅ Automated deployment via Claude
```

### Configuration
```
validator-ips.json                 ✅ 21 validators mapped to 12 AI devs
```

---

## 🎯 What This Does

**12 AI Dev Workers** (using Claude API) monitor **21 validators** autonomously:

| AI Dev | Validators | What They Monitor |
|--------|------------|-------------------|
| governance-dev01 | 1 | Bootnode, block production, peers |
| security-dev01 | 2 | Bootnode, security metrics |
| audit-dev01 | 3 | Bootnode, transaction validation |
| consensus-dev01 | 4-5 | PPFA rotation, finalization |
| runtime-dev01 | 6-7 | Runtime performance, upgrades |
| compiler-dev01 | 8-9 | WASM compilation, execution |
| multichain-dev01 | 10-11 | Cross-chain state, bridges |
| oracle-dev01 | 12 | Price feeds, reserve ratios |
| edsc-dev01 | 13-14 | Economic validity |
| economics-dev01 | 15-16 | Token economics |
| ethics-dev01 | 17-18 | Transaction fairness |
| docs-dev01 | 19-21 | Network documentation |

**Every 5 minutes:**
1. Check all assigned validators (Prometheus + RPC + SSH)
2. Quick health analysis (free)
3. If issues → Call Claude API for intelligent diagnosis
4. Execute actions (restart, alert, investigate)
5. Log everything to GLOBAL_MEMORY.md

---

## 💰 Cost Breakdown

### Optimized Mode (Recommended)
- **Interval:** 5 minutes
- **Claude calls:** Only when issues detected (~10% of cycles)
- **Monthly cost:** ~$56

### Aggressive Mode
- **Interval:** 1 minute
- **Claude calls:** Every cycle
- **Monthly cost:** ~$560

### Conservative Mode
- **Interval:** 15 minutes
- **Claude calls:** Only when issues detected
- **Monthly cost:** ~$19

**Recommendation:** Start with Optimized Mode (5 min, optimized)

---

## 🚀 Two Ways to Deploy

### Option A: Automated (Recommended - 10 minutes)

**Step 1:** Get Claude API Key
1. Go to: https://console.anthropic.com/settings/keys
2. Create new key named "Etrid Validator Monitoring"
3. Copy the key (starts with `sk-ant-api03-...`)
4. Add payment method: https://console.anthropic.com/settings/billing

**Step 2:** Open a new Claude terminal session and paste the entire contents of:
```
CLAUDE_DEPLOYMENT_PROMPT.md
```

**Step 3:** Provide your API key when asked

**Done!** The other Claude instance will:
- Copy all files to Gizzi VM
- Install dependencies
- Create systemd service
- Start 24/7 monitoring
- Verify everything works

---

### Option B: Manual (30 minutes)

Follow the complete step-by-step instructions in:
```
DEPLOYMENT_GUIDE.md
```

Includes:
- How to get API key
- File copying commands
- SSH setup
- Systemd service creation
- Verification steps
- Troubleshooting guide

---

## 📊 How to Monitor the System

### Check if it's running
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
sudo systemctl status ai-dev-monitoring
```

### Watch real-time logs
```bash
sudo journalctl -u ai-dev-monitoring -f
```

**Expected output:**
```
AI DEV ORCHESTRATOR - Monitoring Cycle
[governance-dev01] Starting monitoring cycle...
[governance-dev01] All 1 validators healthy
[consensus-dev01] Starting monitoring cycle...
[consensus-dev01] All 2 validators healthy
...
```

### See AI dev decisions
```bash
tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md
```

**Example entry:**
```markdown
## [2025-10-31 14:30:00] consensus-dev01

**Summary:** All validators healthy

**Validators:** #4, #5

**Health:** ✅ Healthy

**Status:**
- ✅ Validator #4: 18 peers, block 123456
- ✅ Validator #5: 20 peers, block 123456

**Reasoning:** Both validators producing blocks with healthy peer counts...
```

### Check API usage/costs
https://console.anthropic.com/settings/usage

---

## 🔧 Configuration

### Change monitoring interval
```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
sudo nano /etc/systemd/system/ai-dev-monitoring.service
```

Find:
```
Environment="MONITOR_INTERVAL=300"
```

Change to:
- `60` = 1 minute
- `300` = 5 minutes (default)
- `900` = 15 minutes

Then:
```bash
sudo systemctl daemon-reload
sudo systemctl restart ai-dev-monitoring
```

### Disable optimization (always call Claude)
```bash
Environment="OPTIMIZED=false"
```

Costs 10x more but provides reasoning every cycle.

---

## 🎯 Architecture Summary

```
┌─────────────────────────────────────────────┐
│     Validator 1 (Gizzi) - 64.181.215.19    │
│                                              │
│  ┌────────────────────────────────────────┐ │
│  │  AI Dev Orchestrator (Python)          │ │
│  │  - Running as systemd service 24/7     │ │
│  │  - Every 5 minutes                      │ │
│  │                                         │ │
│  │  ┌──────────┐  ┌──────────┐           │ │
│  │  │ Worker 1 │  │ Worker 2 │  ... (12x)│ │
│  │  └──────────┘  └──────────┘           │ │
│  │       ↓              ↓                  │ │
│  │  Claude API (Anthropic)                │ │
│  └────────────────────────────────────────┘ │
│                 ↓ ↓ ↓                       │
│  ┌────────────────────────────────────────┐ │
│  │ ValidatorMonitor                       │ │
│  │ - SSH to all validators                │ │
│  │ - Query Prometheus                     │ │
│  │ - Check RPC                            │ │
│  │ - Restart if needed                    │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
                 ↓ ↓ ↓
┌─────────────────────────────────────────────┐
│        21 Validators (All VMs)              │
│  - Prometheus metrics (port 9615)           │
│  - RPC (port 9944)                          │
│  - SSH (port 22)                            │
└─────────────────────────────────────────────┘
```

---

## ✨ Key Features

### Intelligent Decision Making
- Each AI dev has personality and role context
- Claude analyzes complex failure patterns
- Decides: restart vs alert vs investigate vs escalate
- All reasoning logged

### Cost Optimized
- Only calls Claude API when issues detected
- Simple health checks are free
- 90% cost savings vs calling every cycle

### Autonomous Actions
- Auto-restart failed validators
- Alert on persistent issues
- Escalate to governance-dev01 when multiple validators affected
- Complete audit trail in GLOBAL_MEMORY.md

### Production Ready
- Systemd service (restarts on failure)
- Comprehensive logging
- SSH with proper username detection
- Error handling and retries

---

## 📋 Deployment Checklist

### Before Deployment
- [ ] Get Claude API key from console.anthropic.com
- [ ] Add payment method to Anthropic account
- [ ] Set billing alert at $50 and $100
- [ ] Verify SSH access to Gizzi VM (64.181.215.19)
- [ ] Confirm files are in `/Users/macbook/Desktop/etrid/ai-monitoring/`

### During Deployment
- [ ] Files copied to VM
- [ ] Dependencies installed (anthropic, paramiko, requests)
- [ ] Directory created (/opt/ai-monitoring/)
- [ ] SSH key configured (~/.ssh/gizzi-validator)
- [ ] Systemd service created
- [ ] API key set in service file
- [ ] Service started

### After Deployment
- [ ] Service is running (systemctl status)
- [ ] Logs show AI devs initializing
- [ ] First monitoring cycle completes
- [ ] GLOBAL_MEMORY.md has entries
- [ ] No auth errors in logs
- [ ] Service enabled for reboot

---

## 🚨 Common Issues & Solutions

### Issue: "ANTHROPIC_API_KEY not set"
**Solution:** Edit systemd service file and add correct API key
```bash
sudo nano /etc/systemd/system/ai-dev-monitoring.service
```

### Issue: "Permission denied (publickey)"
**Solution:** Check SSH key permissions
```bash
chmod 600 ~/.ssh/gizzi-validator
```

### Issue: "Connection refused" to Prometheus
**Solution:** Prometheus not running yet (OK, will work once Prometheus is deployed)

### Issue: High API costs
**Solution:**
1. Ensure OPTIMIZED=true
2. Increase interval to 15 minutes
3. Check how often Claude is called in logs

---

## 📞 Support Commands

```bash
# Check service status
sudo systemctl status ai-dev-monitoring

# Watch logs
sudo journalctl -u ai-dev-monitoring -f

# See AI decisions
tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md

# Restart service
sudo systemctl restart ai-dev-monitoring

# Stop service
sudo systemctl stop ai-dev-monitoring

# Check API calls count
grep "Claude API" /var/log/ai-dev-monitoring.log | wc -l
```

---

## 🎯 Next Steps

### Immediate (Today)
1. **Get Claude API key** from console.anthropic.com
2. **Deploy using Option A** (automated via Claude)
3. **Verify it's running** (check logs)
4. **Set billing alerts** ($50, $100)

### Short Term (This Week)
1. Deploy Prometheus for metrics collection
2. Verify all 21 validators accessible
3. Monitor API costs daily
4. Review GLOBAL_MEMORY.md entries

### Long Term (This Month)
1. Add Discord/email alerts
2. Create Grafana dashboard for AI dev activity
3. Implement cross-dev coordination
4. Add predictive analytics

---

## 💡 FAQ

**Q: Do I need Claude Desktop subscription?**
A: No, Claude API is separate. You need to sign up at console.anthropic.com and add a payment method.

**Q: Can I run this on my Mac?**
A: You can test it, but for 24/7 production use the VM (Gizzi validator).

**Q: What if I can't afford $56/month?**
A: Use 15-minute intervals (~$19/month) or wait for issues and monitor manually.

**Q: Do all 12 workers share one API key?**
A: Yes, all 12 workers use the same API key from your one Anthropic account.

**Q: What if Prometheus isn't running?**
A: The service will keep running but won't be able to get metrics. Deploy Prometheus when ready.

**Q: Can I see what Claude is thinking?**
A: Yes! Set OPTIMIZED=false to get reasoning every cycle (costs 10x more).

**Q: How do I stop it?**
A: `sudo systemctl stop ai-dev-monitoring`

---

## ✅ Success Criteria

After deployment, you should see:
- ✅ Service running 24/7
- ✅ All 12 AI devs initialized
- ✅ Monitoring cycles every 5 minutes
- ✅ GLOBAL_MEMORY.md accumulating entries
- ✅ Auto-restart working (if validator goes down)
- ✅ API costs within budget (~$56/month)
- ✅ No authentication errors
- ✅ Complete audit trail

---

## 📂 File Locations Reference

**Local (Your Mac):**
- `/Users/macbook/Desktop/etrid/ai-monitoring/` - All implementation files
- `/Users/macbook/Desktop/etrid/validator-ips.json` - Validator mapping
- `/Users/macbook/.ssh/gizzi-validator` - SSH key

**Remote (Gizzi VM):**
- `/opt/ai-monitoring/` - Application directory
- `/opt/ai-monitoring/GLOBAL_MEMORY.md` - AI decision log
- `/etc/systemd/system/ai-dev-monitoring.service` - Service config
- `/var/log/ai-dev-monitoring.log` - Service logs
- `/home/ubuntu/.ssh/gizzi-validator` - SSH key

**Online:**
- https://console.anthropic.com/settings/keys - API keys
- https://console.anthropic.com/settings/usage - Usage monitoring
- https://console.anthropic.com/settings/billing - Billing

---

**Status:** Complete and ready to deploy
**Estimated Setup Time:** 10-15 minutes (automated) or 30 minutes (manual)
**Expected Monthly Cost:** ~$56 (optimized mode)

---

**Ready to deploy? Copy `CLAUDE_DEPLOYMENT_PROMPT.md` to a new Claude terminal session!**
