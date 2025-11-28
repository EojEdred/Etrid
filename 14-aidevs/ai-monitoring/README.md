# AI Dev Monitoring System - Quick Start

## ✅ What's Ready

Everything is ready to deploy! This folder contains a complete, production-ready AI monitoring system.

## 📦 Files

```
ai-monitoring/
├── validator_monitor.py                     ← Collects validator status
├── ai_dev_workers.py                        ← Claude API integration
├── orchestrator.py                          ← Main coordinator
├── DEPLOYMENT_GUIDE.md                      ← Complete manual instructions
├── CLAUDE_DEPLOYMENT_PROMPT.md              ← Automated deployment prompt
├── AI_DEV_MONITORING_COMPLETE_PACKAGE.md    ← Full documentation
└── README.md                                ← This file
```

## 🚀 Deploy in 3 Steps (10 minutes)

### Step 1: Get Your Claude API Key

1. Go to: **https://console.anthropic.com/settings/keys**
2. Create new key: "Etrid Validator Monitoring"
3. Copy the key (starts with `sk-ant-api03-...`)
4. Add payment method at: https://console.anthropic.com/settings/billing

**Note:** This is separate from Claude Desktop. Cost: ~$56/month optimized.

---

### Step 2: Open New Claude Terminal

Open a **new Claude Code terminal session** (separate from this one).

---

### Step 3: Copy & Paste

In the new terminal, paste the **entire contents** of:
```
CLAUDE_DEPLOYMENT_PROMPT.md
```

Then provide your API key when asked.

**Done!** The other Claude instance will deploy everything to the Gizzi VM.

---

## 📊 What Happens Next

**Every 5 minutes on the Gizzi VM:**
- 12 AI dev workers check their assigned validators
- Quick health check (free)
- If issues found → Claude analyzes and decides actions
- Auto-restart failed validators
- Everything logged to `/opt/ai-monitoring/GLOBAL_MEMORY.md`

**Cost:** ~$56/month (only calls Claude when there are issues)

---

## 🔍 Monitor After Deployment

```bash
# SSH to Gizzi VM
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Check if running
sudo systemctl status ai-dev-monitoring

# Watch logs live
sudo journalctl -u ai-dev-monitoring -f

# See AI decisions
tail -f /opt/ai-monitoring/GLOBAL_MEMORY.md
```

---

## 📖 Need More Info?

- **Automated deployment:** See `CLAUDE_DEPLOYMENT_PROMPT.md`
- **Manual deployment:** See `DEPLOYMENT_GUIDE.md`
- **Complete docs:** See `AI_DEV_MONITORING_COMPLETE_PACKAGE.md`

---

## 💡 Quick FAQ

**Q: Can I test this on my Mac first?**
A: Yes! Just run `python3 orchestrator.py` locally. But for 24/7 use the VM.

**Q: What if I don't want to spend $56/month?**
A: Change interval to 15 minutes (~$19/month) after deployment.

**Q: Where do I get the API key?**
A: https://console.anthropic.com/settings/keys (separate from Claude Desktop)

**Q: How do I stop it?**
A: `sudo systemctl stop ai-dev-monitoring` on the VM

---

**Status:** Ready to deploy ✅
**Time to deploy:** 10-15 minutes
**Where it runs:** Gizzi VM (64.181.215.19) - 24/7
