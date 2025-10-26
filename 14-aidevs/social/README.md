# AI Devs Social Automation

**Autonomous social media system for Ëtrid blockchain**

Automatically posts blockchain stats, governance updates, audit alerts, and responds to community questions across Twitter (and future: Discord, Telegram).

---

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [Workflows](#workflows)
- [Deployment](#deployment)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)

---

## 🎯 Overview

The AI Devs Social Automation system gives Ëtrid's 15 AI developers autonomous social media presence. Each dev has a unique voice and expertise, posting relevant updates and answering community questions 24/7.

**What it does:**
- **Daily Stats** - Oracle Dev posts blockchain metrics at 12:00 UTC
- **Governance** - Governance Dev announces proposals and voting results
- **Audits** - Audit Dev flags security issues and suspicious activity
- **Community QA** - Auto-routes questions to expert devs
- **Weekly Summaries** - Gizzi posts reflective week-in-review threads
- **Event Alerts** - Real-time notifications for important on-chain events

---

## ✨ Features

### Automated Posting
- **Scheduled**: Daily stats, weekly summaries, periodic reports
- **Event-Driven**: Governance proposals, validator slashing, treasury spends
- **Reactive**: Responds to mentions and community questions

### Content Quality
- **Claude-Powered**: High-quality content generation using Anthropic's Claude API
- **Dev Personas**: Each AI Dev has unique voice (oracle-dev: data-driven, audit-dev: security-focused, gizzi: reflective)
- **Moderation**: Multi-layer safety checks before posting

### Intelligence
- **Question Routing**: Automatically routes questions to appropriate dev based on keywords
- **Accuracy Verification**: Cross-checks data with blockchain before posting
- **Spam Filtering**: Detects and ignores spam/inappropriate mentions

### Monitoring
- **GLOBAL_MEMORY**: All activities logged for transparency and cross-dev coordination
- **Metrics**: Prometheus-compatible metrics for monitoring
- **Alerts**: Slack/email notifications for critical events

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────┐
│         MCP Orchestrator (Scheduler)            │
│  - Cron-like scheduling                         │
│  - Event monitoring                             │
│  - Workflow coordination                        │
└──────────────┬──────────────────────────────────┘
               │
       ┌───────┴────────┐
       │                │
       ▼                ▼
┌─────────────┐  ┌─────────────┐
│ Blockchain  │  │   Global    │
│  Monitor    │  │   Memory    │
│             │  │             │
│ - Stats     │  │ - Dev logs  │
│ - Events    │  │ - History   │
│ - Proposals │  │ - Coord     │
└──────┬──────┘  └──────┬──────┘
       │                │
       └────────┬───────┘
                │
         ┌──────▼──────┐
         │   Content   │
         │  Generator  │
         │             │
         │ - Claude    │
         │ - Personas  │
         └──────┬──────┘
                │
         ┌──────▼──────┐
         │  Content    │
         │ Moderator   │
         │             │
         │ - Safety    │
         │ - Brand     │
         └──────┬──────┘
                │
         ┌──────▼──────┐
         │   Twitter   │
         │  Connector  │
         │             │
         │ - Post      │
         │ - Reply     │
         │ - Monitor   │
         └─────────────┘
```

---

## 🚀 Quick Start

### 1. Install Dependencies

```bash
cd /Users/macbook/Desktop/etrid/ai-devs/social
pip install -r requirements.txt
```

### 2. Configure Environment

```bash
# Copy environment template
cp .env.example .env

# Edit with your credentials
nano .env
```

**Required credentials:**
- `BLOCKCHAIN_WS_URL` - FlareChain WebSocket URL
- `TWITTER_API_KEY`, `TWITTER_API_SECRET`, `TWITTER_ACCESS_TOKEN`, `TWITTER_ACCESS_SECRET`
- `ANTHROPIC_API_KEY` - Claude API key

### 3. Test Workflows

```bash
# Test daily stats (dry run)
python workflows/daily_stats.py --dry-run

# Test Twitter mentions
python workflows/twitter_mentions.py --dry-run

# Test weekly summary
python workflows/weekly_summary.py --dry-run
```

### 4. Run Live

```bash
# Post daily stats (runs once)
python workflows/daily_stats.py

# Monitor mentions continuously
python workflows/twitter_mentions.py --mode stream

# Monitor governance events
python workflows/governance_monitor.py --mode stream
```

---

## 📦 Installation

### Prerequisites

- **Python 3.10+**
- **FlareChain Node** (or access to public RPC)
- **Twitter Developer Account** (Free tier sufficient)
- **Anthropic API Key** (Claude access)

### Step-by-Step Installation

#### 1. Install Python Dependencies

```bash
cd social/
pip install -r requirements.txt

# Or with virtual environment (recommended)
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

#### 2. Set Up Twitter API

1. Create Twitter account [@EtridAI_Devs](https://twitter.com/signup)
2. Apply for Developer Account at [developer.twitter.com](https://developer.twitter.com/)
3. Create new app in Developer Portal
4. Generate API keys:
   - API Key and Secret
   - Access Token and Secret
5. Save credentials to `.env`

#### 3. Get Claude API Key

1. Sign up at [console.anthropic.com](https://console.anthropic.com/)
2. Create API key
3. Add to `.env` as `ANTHROPIC_API_KEY`

#### 4. Configure Blockchain Connection

**Option A: Local Node (Development)**
```bash
# Start FlareChain node
cd /path/to/flarechain
./target/release/node-template --dev

# In .env:
BLOCKCHAIN_WS_URL=ws://127.0.0.1:9944
```

**Option B: Public RPC (Production)**
```bash
# In .env:
BLOCKCHAIN_WS_URL=wss://rpc.etrid.network
```

#### 5. Test Connection

```bash
python workflows/daily_stats.py --dry-run
```

If successful, you should see:
```
✅ Connected to blockchain: ws://127.0.0.1:9944
📊 Fetching 24h blockchain stats...
✅ Stats fetched: {...}
🤖 Generating tweet with Claude...
✅ Tweet generated
```

---

## ⚙️ Configuration

### Environment Variables

See `.env.example` for complete list. Key settings:

**Blockchain:**
```bash
BLOCKCHAIN_WS_URL=ws://127.0.0.1:9944
BLOCKCHAIN_NETWORK=development
```

**Twitter:**
```bash
TWITTER_API_KEY=your_key
TWITTER_API_SECRET=your_secret
TWITTER_ACCESS_TOKEN=your_token
TWITTER_ACCESS_SECRET=your_access_secret
TWITTER_USERNAME=EtridAI_Devs
```

**Claude:**
```bash
ANTHROPIC_API_KEY=your_api_key
CLAUDE_MODEL=claude-sonnet-4-5-20250929
```

**Moderation:**
```bash
MODERATION_STRICTNESS=normal  # normal, strict, paranoid
AUTO_APPROVE_THRESHOLD=0.8    # 0-1 confidence score
REQUIRE_HUMAN_APPROVAL=false  # true for manual review
```

**Rate Limiting:**
```bash
MAX_POSTS_PER_DAY=50
MIN_POST_INTERVAL=300  # 5 minutes
MAX_RESPONSES_PER_HOUR=20
```

**Development:**
```bash
DRY_RUN=false          # Set true to test without posting
USE_MOCK_DATA=false    # Set true to test without blockchain
DEBUG=true             # Verbose logging
```

### Posting Schedule

Edit `config/posting_schedule.yaml` to customize:

**Scheduled Posts:**
```yaml
scheduled:
  - name: "daily_stats"
    schedule: "0 12 * * *"  # Daily at 12:00 UTC
    dev: "oracle-dev"
    enabled: true
```

**Event-Driven Posts:**
```yaml
event_driven:
  - name: "governance_proposals"
    trigger: "on_chain_event"
    event_filter:
      pallet: "Democracy"
      method: "Proposed"
    dev: "governance-dev"
    enabled: true
```

---

## 📝 Workflows

### Available Workflows

#### 1. Daily Stats (`workflows/daily_stats.py`)
**What:** Posts 24h blockchain metrics
**When:** Daily at 12:00 UTC
**Dev:** Oracle Dev
**Usage:**
```bash
python workflows/daily_stats.py              # Post live
python workflows/daily_stats.py --dry-run    # Test mode
```

#### 2. Weekly Summary (`workflows/weekly_summary.py`)
**What:** Week-in-review thread
**When:** Sunday at 18:00 UTC
**Dev:** Gizzi
**Usage:**
```bash
python workflows/weekly_summary.py --dry-run
```

#### 3. Twitter Mentions (`workflows/twitter_mentions.py`)
**What:** Auto-responds to community questions
**When:** Continuous monitoring
**Dev:** Auto-routed based on question
**Usage:**
```bash
python workflows/twitter_mentions.py --mode stream  # Continuous
python workflows/twitter_mentions.py --mode poll    # Check once
```

#### 4. Governance Monitor (`workflows/governance_monitor.py`)
**What:** Announces proposals and voting results
**When:** Event-driven (new proposals, results)
**Dev:** Governance Dev
**Usage:**
```bash
python workflows/governance_monitor.py --mode stream
```

#### 5. Audit Monitor (`workflows/audit_monitor.py`)
**What:** Security alerts (slashing, suspicious proposals, reserve warnings)
**When:** Event-driven (security events)
**Dev:** Audit Dev
**Usage:**
```bash
python workflows/audit_monitor.py --mode stream
```

### Creating Custom Workflows

Template for new workflow:

```python
#!/usr/bin/env python3
import asyncio
import sys
from pathlib import Path

sys.path.append(str(Path(__file__).parent.parent))

from connectors.blockchain import BlockchainMonitor
from connectors.twitter import TwitterConnector
from content.generator import ContentGenerator
from content.moderation import ContentModerator

class MyWorkflow:
    def __init__(self, dry_run=False):
        self.dry_run = dry_run
        self.blockchain = BlockchainMonitor()
        self.twitter = TwitterConnector()
        self.generator = ContentGenerator()
        self.moderator = ContentModerator()

    async def run(self):
        # Your workflow logic here
        pass

async def main():
    dry_run = "--dry-run" in sys.argv
    workflow = MyWorkflow(dry_run=dry_run)
    await workflow.run()

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 🚀 Deployment

### Development Setup

```bash
# Run individual workflows manually
python workflows/daily_stats.py --dry-run

# Monitor mentions in foreground
python workflows/twitter_mentions.py --mode stream
```

### Production Deployment

#### Option 1: Systemd Services (Linux)

Create service files in `/etc/systemd/system/`:

**`etrid-daily-stats.service`:**
```ini
[Unit]
Description=Etrid AI Devs - Daily Stats
After=network.target

[Service]
Type=oneshot
User=etrid
WorkingDirectory=/path/to/ai-devs/social
Environment="PATH=/path/to/venv/bin"
ExecStart=/path/to/venv/bin/python workflows/daily_stats.py

[Install]
WantedBy=multi-user.target
```

**`etrid-daily-stats.timer`:**
```ini
[Unit]
Description=Run Etrid Daily Stats at 12:00 UTC

[Timer]
OnCalendar=*-*-* 12:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

Enable and start:
```bash
sudo systemctl enable etrid-daily-stats.timer
sudo systemctl start etrid-daily-stats.timer
```

#### Option 2: Cron Jobs

Add to crontab (`crontab -e`):

```cron
# Daily stats at 12:00 UTC
0 12 * * * cd /path/to/ai-devs/social && /path/to/venv/bin/python workflows/daily_stats.py

# Weekly summary on Sunday at 18:00 UTC
0 18 * * 0 cd /path/to/ai-devs/social && /path/to/venv/bin/python workflows/weekly_summary.py
```

#### Option 3: Docker

```dockerfile
# Dockerfile
FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["python", "workflows/daily_stats.py"]
```

Build and run:
```bash
docker build -t etrid-social-automation .
docker run -d --env-file .env etrid-social-automation
```

#### Option 4: PM2 (Node.js Process Manager)

```bash
# Install PM2
npm install -g pm2

# Start continuous workflows
pm2 start workflows/twitter_mentions.py --name "etrid-mentions" --interpreter python3 -- --mode stream
pm2 start workflows/governance_monitor.py --name "etrid-governance" --interpreter python3 -- --mode stream

# Save process list
pm2 save

# Auto-start on boot
pm2 startup
```

---

## 📊 Monitoring

### Logs

**View logs:**
```bash
tail -f logs/social_automation.log
```

**Log levels:**
- `DEBUG`: Verbose info (set `DEBUG=true` in .env)
- `INFO`: General operation
- `WARNING`: Potential issues
- `ERROR`: Failures

### Metrics

Prometheus metrics exposed on `:9090/metrics`:

- `etrid_posts_total` - Total posts by dev
- `etrid_responses_total` - Total community responses
- `etrid_moderation_rejections` - Posts rejected by moderation
- `etrid_api_calls_total` - API calls (Twitter, Claude, blockchain)
- `etrid_workflow_duration_seconds` - Workflow execution time

### Alerts

Configure Slack webhook in `.env`:
```bash
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/YOUR/WEBHOOK
```

Alerts sent for:
- Post failures
- Moderation rejections
- Critical audit events
- API rate limit warnings
- Daily summary

---

## 🔧 Troubleshooting

### Common Issues

#### 1. "Failed to connect to blockchain"

**Solution:**
```bash
# Check blockchain node is running
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944

# Or use public RPC
# In .env: BLOCKCHAIN_WS_URL=wss://rpc.etrid.network
```

#### 2. "Twitter API authentication failed"

**Solution:**
- Verify credentials in `.env`
- Check Twitter Developer Portal for app status
- Regenerate tokens if needed

#### 3. "Claude API rate limit exceeded"

**Solution:**
```bash
# Reduce posting frequency in config/posting_schedule.yaml
# Or increase API limit in Anthropic console
# Set cost limits: MAX_DAILY_CLAUDE_SPEND=10.00
```

#### 4. "Moderation rejected post"

**Solution:**
```bash
# Check moderation logs
cat logs/social_automation.log | grep "Moderation"

# Adjust strictness
# In .env: MODERATION_STRICTNESS=normal  # instead of 'strict'
```

#### 5. "No new mentions found (but there are mentions)"

**Solution:**
- Check `TWITTER_USERNAME` matches actual account
- Verify app has read permissions
- Check rate limits (Twitter API)

### Debug Mode

Enable verbose logging:
```bash
# In .env:
DEBUG=true
LOG_LEVEL=DEBUG

# Run workflow
python workflows/daily_stats.py --dry-run
```

### Testing Without Live Posting

```bash
# All workflows support --dry-run
python workflows/daily_stats.py --dry-run
python workflows/twitter_mentions.py --dry-run --mode stream

# Use mock blockchain data
# In .env: USE_MOCK_DATA=true
```

---

## 💰 Cost Estimates

### Monthly Operating Costs

| Service | Usage | Cost |
|---------|-------|------|
| Twitter API (Free) | 1,500 tweets/month | $0 |
| Claude API | ~50-150 requests/day | $100-150 |
| Blockchain Node | Public RPC or dedicated | $0-50 |
| Database (optional) | PostgreSQL/Redis | $10-20 |
| Hosting | VPS for workflows | $10-20 |
| **Total** | | **$120-240/month** |

### Cost Optimization

- Use `DRY_RUN=true` for testing
- Set `MAX_DAILY_CLAUDE_SPEND` limit
- Use mock data for development
- Monitor API usage in logs

---

## 📚 Additional Resources

- [Full Automation Plan](../MCP_SOCIAL_AUTOMATION.md) - Complete architecture and use cases
- [Deployment Summary](../COMPLETE_SUMMARY_AND_NEXT_STEPS.md) - Overview and roadmap
- [Twitter Setup Guide](../TWITTER_SETUP_GUIDE.md) - Account creation and first posts
- [Session Summaries](../) - Development history

---

## 🤝 Contributing

To add new workflows or improve existing ones:

1. Create workflow in `workflows/`
2. Add connector if needed in `connectors/`
3. Update `config/posting_schedule.yaml`
4. Test with `--dry-run`
5. Document in this README

---

## 📞 Support

- **Issues**: Log to `logs/social_automation.log`
- **Errors**: Check Slack alerts (if configured)
- **Questions**: Review GLOBAL_MEMORY.md for AI Dev activities

---

**Status:** Production-ready ✅
**Version:** 1.0
**Last Updated:** October 24, 2025
