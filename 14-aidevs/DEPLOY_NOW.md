# 🚀 Deploy Social Automation NOW - Quick Start

**Ready to make AI Devs autonomous? Here's how to deploy in 30 minutes.**

---

## ⚡ Prerequisites (5 minutes)

You need these three things:

### 1. Twitter Developer Account
- [ ] Go to [developer.twitter.com](https://developer.twitter.com/)
- [ ] Create developer account (free)
- [ ] Create new app
- [ ] Generate API keys:
  - API Key
  - API Secret
  - Access Token
  - Access Token Secret

### 2. Claude API Key
- [ ] Go to [console.anthropic.com](https://console.anthropic.com/)
- [ ] Sign up (first $5 free)
- [ ] Create API key
- [ ] Budget: ~$100-150/month for automation

### 3. Blockchain Access
**Option A (Easy):** Use public RPC
- No setup needed
- URL: `wss://rpc.etrid.network` (or testnet)

**Option B (Advanced):** Run local node
```bash
cd /path/to/flarechain
./target/release/node-template --dev
# URL: ws://127.0.0.1:9944
```

---

## 📦 Installation (10 minutes)

### Step 1: Navigate to Social Directory
```bash
cd /Users/macbook/Desktop/etrid/ai-devs/social
```

### Step 2: Install Python Dependencies
```bash
# With virtual environment (recommended)
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt

# Or without venv
pip3 install -r requirements.txt
```

### Step 3: Configure Environment
```bash
# Copy template
cp .env.example .env

# Edit with your credentials
nano .env
# Or use any text editor: code .env, vim .env, etc.
```

**Add these to `.env`:**
```bash
# Blockchain
BLOCKCHAIN_WS_URL=wss://rpc.etrid.network  # Or ws://127.0.0.1:9944

# Twitter (from step 1 above)
TWITTER_API_KEY=your_api_key_here
TWITTER_API_SECRET=your_api_secret_here
TWITTER_ACCESS_TOKEN=your_access_token_here
TWITTER_ACCESS_SECRET=your_access_secret_here
TWITTER_USERNAME=EtridAI_Devs

# Claude (from step 2 above)
ANTHROPIC_API_KEY=your_anthropic_api_key_here

# Development settings (for testing)
DRY_RUN=false  # Set to true for testing without posting
DEBUG=true
```

Save and exit (Ctrl+X, Y, Enter in nano)

---

## 🧪 Test (5 minutes)

### Test 1: Daily Stats Workflow
```bash
python workflows/daily_stats.py --dry-run
```

**Expected output:**
```
============================================================
  Daily Stats Workflow - Oracle Dev
============================================================

✅ Connected to blockchain: wss://rpc.etrid.network
📊 Fetching 24h blockchain stats...
✅ Stats fetched: {...}
🤖 Generating tweet with Claude...
✅ Tweet generated:
   📊 Ëtrid Daily Stats (Oct 24):
   • Blocks: 14,400 (+0.2%)
   ...

🧪 DRY RUN - Would post:
   [Full tweet with signature]

============================================================
  ✅ Daily Stats Workflow Complete!
============================================================
```

If you see this, everything works! 🎉

### Test 2: Twitter Mentions
```bash
python workflows/twitter_mentions.py --dry-run --mode poll
```

### Test 3: Weekly Summary
```bash
python workflows/weekly_summary.py --dry-run
```

---

## 🎬 Go Live (10 minutes)

### Option 1: Post First Stats Manually

```bash
# Remove DRY_RUN from .env (or set to false)
nano .env
# Set: DRY_RUN=false

# Post your first automated daily stats!
python workflows/daily_stats.py
```

This will:
1. Fetch 24h blockchain stats
2. Generate tweet using Claude (Oracle Dev voice)
3. Verify accuracy
4. Moderate content
5. Post to Twitter with "—Oracle Dev" signature
6. Log to GLOBAL_MEMORY.md

**Check Twitter** - You should see your first automated post! 🎉

---

### Option 2: Start Continuous Monitoring

```bash
# Start mention monitoring (responds to community questions 24/7)
python workflows/twitter_mentions.py --mode stream

# This will run continuously, press Ctrl+C to stop
```

**What this does:**
- Monitors @EtridAI_Devs mentions every 60 seconds
- Routes questions to appropriate dev (edsc-dev, governance-dev, etc.)
- Generates contextual responses using Claude
- Posts replies automatically
- Logs all interactions

---

### Option 3: Full Autonomous Setup (Production)

**For Linux/Mac with systemd:**

Create service file: `/etc/systemd/system/etrid-social.service`
```ini
[Unit]
Description=Etrid AI Devs - Social Automation
After=network.target

[Service]
Type=simple
User=yourusername
WorkingDirectory=/Users/macbook/Desktop/etrid/ai-devs/social
Environment="PATH=/path/to/venv/bin"
ExecStart=/path/to/venv/bin/python workflows/twitter_mentions.py --mode stream
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable:
```bash
sudo systemctl enable etrid-social
sudo systemctl start etrid-social
sudo systemctl status etrid-social
```

**For Mac/Linux with cron:**

Add to crontab (`crontab -e`):
```cron
# Daily stats at 12:00 UTC
0 12 * * * cd /Users/macbook/Desktop/etrid/ai-devs/social && /path/to/venv/bin/python workflows/daily_stats.py

# Weekly summary on Sunday at 18:00 UTC
0 18 * * 0 cd /Users/macbook/Desktop/etrid/ai-devs/social && /path/to/venv/bin/python workflows/weekly_summary.py
```

**For any OS with PM2 (Node.js process manager):**

```bash
# Install PM2
npm install -g pm2

# Start continuous workflows
pm2 start workflows/twitter_mentions.py \
  --name "etrid-mentions" \
  --interpreter python3 \
  -- --mode stream

pm2 start workflows/governance_monitor.py \
  --name "etrid-governance" \
  --interpreter python3 \
  -- --mode stream

# Save and auto-start on reboot
pm2 save
pm2 startup
```

---

## 📊 Monitor

### View Logs
```bash
tail -f logs/social_automation.log
```

### Check What's Running
```bash
# With PM2
pm2 list
pm2 logs etrid-mentions

# With systemd
sudo systemctl status etrid-social
sudo journalctl -u etrid-social -f
```

### Check AI Dev Activity
```bash
cat ../memory/GLOBAL_MEMORY.md | tail -50
```

---

## 🎯 What Happens Next

### Immediate (First Hour)
- ✅ First daily stats posted
- ✅ System monitoring mentions
- ✅ Ready to respond to community

### First Day
- ✅ Community questions answered automatically
- ✅ Any new governance proposals announced
- ✅ Audit alerts if security events detected

### First Week
- ✅ 7 daily stats posts
- ✅ 10-20 community responses
- ✅ 1 weekly summary thread (Sunday)
- ✅ Governance/audit posts (as events occur)

### First Month
- ✅ Fully autonomous operation
- ✅ 30+ daily stats
- ✅ 100+ community interactions
- ✅ 4 weekly summaries
- ✅ Recognized AI Dev brand

---

## 🛑 Troubleshooting

### "Failed to connect to blockchain"
```bash
# Check node is running
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9944

# Or use public RPC in .env
BLOCKCHAIN_WS_URL=wss://rpc.etrid.network
```

### "Twitter API authentication failed"
- Verify credentials in `.env`
- Check Twitter Developer Portal app status
- Regenerate tokens if needed

### "Claude API error"
- Check API key is correct
- Verify you have credits ($5 free tier)
- Check rate limits in Anthropic console

### "ModuleNotFoundError: No module named 'X'"
```bash
# Reinstall dependencies
pip install -r requirements.txt --upgrade
```

### "Permission denied"
```bash
# Make workflows executable
chmod +x workflows/*.py
```

---

## 💰 Costs

### Monthly Operating Budget

| Service | Cost |
|---------|------|
| Twitter API (Free tier) | $0 |
| Claude API | $100-150 |
| Blockchain RPC (public) | $0 |
| **Total** | **$100-150/month** |

**Cost optimization:**
- Start with `MAX_DAILY_CLAUDE_SPEND=5` in .env
- Monitor usage in Anthropic console
- Adjust posting frequency if needed

---

## 🎉 Success Checklist

- [ ] Python dependencies installed
- [ ] `.env` configured with credentials
- [ ] Test workflows run successfully with `--dry-run`
- [ ] First daily stats posted to Twitter
- [ ] Mention monitoring active
- [ ] Logs showing activity
- [ ] GLOBAL_MEMORY.md updating

**All checked?** Congratulations! Your AI Devs are now autonomous! 🚀

---

## 📞 Need Help?

1. **Check README**: `social/README.md` has detailed troubleshooting
2. **Review logs**: `logs/social_automation.log`
3. **Check GLOBAL_MEMORY**: `memory/GLOBAL_MEMORY.md`
4. **Test with dry-run**: All workflows support `--dry-run` flag

---

## 🔥 The Bottom Line

You're **3 commands** away from autonomous AI Devs:

```bash
# 1. Install
pip install -r requirements.txt

# 2. Configure
cp .env.example .env && nano .env

# 3. Deploy
python workflows/daily_stats.py
python workflows/twitter_mentions.py --mode stream
```

That's it. **Your AI Devs are now posting and responding autonomously.** 🎊

---

**Estimated time to deploy:** 30 minutes
**Estimated time to value:** Immediate (first post within 5 minutes)
**Monthly cost:** $100-150
**Result:** 24/7 autonomous Ëtrid social presence

---

*Let's make the AI Devs autonomous.* 🚀
