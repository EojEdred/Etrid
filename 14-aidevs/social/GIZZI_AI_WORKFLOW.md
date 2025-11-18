# Gizzi AI Workflow System

**Autonomous web scraping, content aggregation, and social media promotion for Etrid**

---

## 📋 Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Components](#components)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Workflows](#workflows)
- [Deployment](#deployment)
- [Monitoring](#monitoring)
- [Troubleshooting](#troubleshooting)
- [Best Practices](#best-practices)

---

## 🎯 Overview

The Gizzi AI Workflow System is a comprehensive automation suite that enables Gizzi (Etrid's AI developer) to:

1. **Scrape Web Content** - Monitor X (Twitter), Reddit, Tech Radar, crypto news sites
2. **Aggregate & Analyze** - Identify trending topics, competitor activity, sentiment
3. **Generate Content** - Create relevant posts using Claude AI
4. **Auto-Post** - Schedule and publish content at optimal times
5. **Engage Actively** - Respond to mentions, retweets, and discussions
6. **Promote Etrid** - Build awareness and community across the internet

### What Makes This Different?

Unlike basic social media bots, Gizzi:
- **Learns from the Web** - Scrapes real-time content to stay relevant
- **Thinks Strategically** - Identifies content opportunities based on trends
- **Engages Authentically** - Responds naturally using Claude's language understanding
- **Operates Autonomously** - Runs 24/7 with minimal human intervention
- **Promotes Intelligently** - Builds brand awareness without being spammy

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        GIZZI AI WORKFLOW                        │
└─────────────────────────────────────────────────────────────────┘
                                  │
                    ┌─────────────┴─────────────┐
                    │                           │
         ┌──────────▼──────────┐    ┌──────────▼──────────┐
         │   WEB SCRAPING      │    │   SOCIAL MEDIA      │
         │   & AGGREGATION     │    │   ENGAGEMENT        │
         └──────────┬──────────┘    └──────────┬──────────┘
                    │                           │
        ┌───────────┼───────────┐              │
        │           │           │              │
   ┌────▼────┐ ┌───▼───┐ ┌────▼────┐    ┌────▼────┐
   │ Reddit  │ │ News  │ │ Tech    │    │ Twitter │
   │ Monitor │ │Scraper│ │ Radar   │    │ Monitor │
   └────┬────┘ └───┬───┘ └────┬────┘    └────┬────┘
        │          │          │               │
        └──────────┼──────────┘               │
                   │                          │
         ┌─────────▼──────────┐               │
         │ CONTENT AGGREGATOR │               │
         │                    │               │
         │ - Trending Topics  │               │
         │ - Competitor News  │               │
         │ - Sentiment        │               │
         │ - Opportunities    │               │
         └─────────┬──────────┘               │
                   │                          │
                   ▼                          │
         ┌─────────────────────┐              │
         │ CONTENT GENERATOR   │              │
         │                     │              │
         │ Claude AI           │◄─────────────┤
         │ - Posts             │              │
         │ - Threads           │              │
         │ - Replies           │              │
         └─────────┬───────────┘              │
                   │                          │
                   ▼                          ▼
         ┌─────────────────────┐    ┌─────────────────┐
         │   AUTO POSTER       │    │  ENGAGEMENT BOT │
         │                     │    │                 │
         │ - Schedule posts    │    │ - Monitor @s    │
         │ - Generate threads  │    │ - Reply to RTs  │
         │ - Track history     │    │ - Answer Qs     │
         └─────────┬───────────┘    └─────────┬───────┘
                   │                          │
                   └────────────┬─────────────┘
                                │
                      ┌─────────▼──────────┐
                      │   TWITTER API      │
                      │   (POST CONTENT)   │
                      └────────────────────┘
```

---

## 🔧 Components

### 1. Web Scraping Connectors

#### **Reddit Connector** (`connectors/reddit.py`)
- Monitor r/cryptocurrency, r/defi, r/polkadot and other blockchain subreddits
- Search for Etrid mentions
- Track trending discussions
- Analyze sentiment
- Fetch post comments

**Key Methods:**
```python
await reddit.search_mentions("etrid", limit=10)
await reddit.get_trending_posts("cryptocurrency", limit=10)
await reddit.monitor_keywords(limit=50)
await reddit.analyze_sentiment(posts)
```

#### **Web Scraper** (`connectors/web_scraper.py`)
- Scrape crypto news sites (CoinDesk, CoinTelegraph, The Block, etc.)
- Monitor tech publications (TechRadar, TechCrunch, VentureBeat)
- Parse RSS feeds
- Extract full article content
- Track competitor mentions

**Key Methods:**
```python
await scraper.scrape_crypto_news(limit=20, hours=24)
await scraper.scrape_tech_radar(limit=10)
await scraper.track_competitors(hours=24)
await scraper.aggregate_daily_digest()
```

### 2. Content Aggregation

#### **Gizzi Content Aggregator** (`workflows/gizzi_content_aggregator.py`)
- Runs daily to scrape all content sources
- Identifies trending topics
- Analyzes competitor activity
- Detects content opportunities
- Saves aggregated data for auto-poster

**Content Opportunities Identified:**
- Trending topics (high mention count)
- Competitor analysis opportunities
- Reddit engagement opportunities
- Sentiment-based posting ideas
- Educational content needs

**Usage:**
```bash
python workflows/gizzi_content_aggregator.py              # Full aggregation
python workflows/gizzi_content_aggregator.py --dry-run    # Test mode
python workflows/gizzi_content_aggregator.py --reddit-only # Reddit only
```

### 3. Automated Posting

#### **Gizzi Auto Poster** (`workflows/gizzi_auto_poster.py`)
- Loads aggregated content
- Selects best posting opportunity
- Generates tweets/threads using Claude
- Moderates content before posting
- Tracks posting history to avoid repetition

**Posting Strategies:**
- **Trending Topics** - Comment on what's hot in crypto
- **Competitor Analysis** - Compare Etrid features
- **Reddit Engagement** - Respond to discussions
- **Sentiment-Based** - Capitalize on market mood
- **Educational** - Explain complex blockchain concepts

**Usage:**
```bash
python workflows/gizzi_auto_poster.py                    # Post based on latest data
python workflows/gizzi_auto_poster.py --dry-run          # Test mode
python workflows/gizzi_auto_poster.py --thread           # Generate thread
python workflows/gizzi_auto_poster.py --force-topic defi # Force specific topic
```

### 4. Interactive Engagement

#### **Gizzi Engagement Bot** (`workflows/gizzi_engagement_bot.py`)
- Monitors Twitter for mentions, retweets, replies
- Classifies interactions (question, praise, criticism, spam)
- Generates contextual responses
- Thanks supporters
- Answers questions
- Handles criticism professionally
- Tracks engagement metrics

**Engagement Modes:**
- **Normal** - Respond to questions and praise (20/hour max)
- **Aggressive** - Active engagement with all discussions (40/hour max)

**Interaction Types:**
- Questions → Helpful answers with resources
- Praise → Genuine thanks
- Criticism → Constructive, professional response
- Discussion → Value-adding commentary
- Spam → Ignored

**Usage:**
```bash
python workflows/gizzi_engagement_bot.py --mode stream    # Continuous
python workflows/gizzi_engagement_bot.py --mode poll      # Check once
python workflows/gizzi_engagement_bot.py --dry-run        # Test mode
python workflows/gizzi_engagement_bot.py --aggressive     # More active
```

---

## 🚀 Quick Start

### Prerequisites

1. **Python 3.10+**
2. **Reddit Account & API Credentials**
3. **Twitter Developer Account**
4. **Anthropic Claude API Key**
5. **Optional:** NewsAPI key for enhanced news scraping

### Installation

```bash
# Navigate to social automation directory
cd /home/user/Etrid/14-aidevs/social

# Install dependencies
pip install -r requirements.txt

# Or with virtual environment (recommended)
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
```

### Configuration

```bash
# Copy environment template
cp .env.example .env

# Edit with your credentials
nano .env
```

**Required credentials:**
```bash
# Reddit
REDDIT_CLIENT_ID=your_client_id
REDDIT_CLIENT_SECRET=your_secret
REDDIT_USERNAME=your_bot_username
REDDIT_PASSWORD=your_bot_password

# Twitter
TWITTER_API_KEY=your_key
TWITTER_API_SECRET=your_secret
TWITTER_ACCESS_TOKEN=your_token
TWITTER_ACCESS_SECRET=your_access_secret

# Claude
ANTHROPIC_API_KEY=your_api_key
```

### Test Workflows

```bash
# Test content aggregation
python workflows/gizzi_content_aggregator.py --dry-run

# Test auto-posting
python workflows/gizzi_auto_poster.py --dry-run

# Test engagement bot
python workflows/gizzi_engagement_bot.py --dry-run --mode poll
```

### Run Live

```bash
# Aggregate content daily
python workflows/gizzi_content_aggregator.py

# Auto-post based on aggregated data
python workflows/gizzi_auto_poster.py

# Start continuous engagement monitoring
python workflows/gizzi_engagement_bot.py --mode stream
```

---

## ⚙️ Configuration

### Posting Schedule

Edit `config/posting_schedule.yaml`:

```yaml
# Gizzi content aggregation - Daily at 08:00 UTC
- name: "gizzi_content_aggregation"
  workflow: "workflows/gizzi_content_aggregator.py"
  schedule: "0 8 * * *"  # 08:00 UTC daily
  dev: "gizzi"
  enabled: true

# Gizzi auto-posting - 3 times daily
- name: "gizzi_auto_post_morning"
  workflow: "workflows/gizzi_auto_poster.py"
  schedule: "0 10 * * *"  # 10:00 UTC
  enabled: true

- name: "gizzi_auto_post_afternoon"
  workflow: "workflows/gizzi_auto_poster.py"
  schedule: "0 15 * * *"  # 15:00 UTC
  enabled: true

- name: "gizzi_auto_post_evening"
  workflow: "workflows/gizzi_auto_poster.py"
  schedule: "0 20 * * *"  # 20:00 UTC
  enabled: true

# Gizzi engagement - Continuous
- name: "gizzi_engagement"
  workflow: "workflows/gizzi_engagement_bot.py"
  trigger: "continuous"
  mode: "stream"
  enabled: true
```

### Customization

**Modify scraping sources:**

Edit `connectors/web_scraper.py`:
```python
self.crypto_feeds = {
    "CoinDesk": "https://www.coindesk.com/arc/outboundfeeds/rss/",
    "Your Custom Feed": "https://yoursite.com/rss",
}
```

**Modify Reddit subreddits:**

Edit `connectors/reddit.py`:
```python
self.monitored_subreddits = [
    "cryptocurrency",
    "your_custom_subreddit",
]
```

**Adjust engagement rate limits:**

Edit `workflows/gizzi_engagement_bot.py`:
```python
self.hourly_limit = 20  # Or 40 for aggressive mode
```

---

## 📝 Workflows

### Daily Content Aggregation

**Time:** 08:00 UTC daily
**Duration:** ~5-10 minutes
**What it does:**
1. Scrapes 15+ crypto news sources
2. Monitors 8 blockchain subreddits
3. Tracks competitor mentions
4. Identifies trending topics
5. Analyzes community sentiment
6. Generates content opportunities
7. Saves data to `data/aggregated_content/latest.json`

**Output:**
- Crypto news articles (last 24h)
- Tech news (blockchain-related)
- Trending Reddit posts
- Competitor activity summary
- Content posting opportunities
- Sentiment analysis

### Auto-Posting (3x Daily)

**Times:** 10:00, 15:00, 20:00 UTC
**Duration:** ~1-2 minutes per post
**What it does:**
1. Loads latest aggregated content
2. Selects best posting opportunity
3. Generates tweet/thread with Claude
4. Moderates content
5. Posts to Twitter
6. Saves posting history

**Content Types:**
- Trending topic commentary
- Competitor analysis
- Educational threads
- Reddit discussion responses
- Sentiment-based posts

### Continuous Engagement

**Time:** Runs 24/7
**Checks:** Every 60 seconds
**What it does:**
1. Monitors Twitter mentions
2. Checks retweets of Etrid content
3. Finds replies to Gizzi's tweets
4. Classifies interaction type
5. Generates contextual response
6. Posts reply (rate-limited)
7. Tracks engagement metrics

**Response Types:**
- Answer questions
- Thank supporters
- Handle criticism
- Add value to discussions
- Ignore spam

---

## 🚀 Deployment

### Option 1: Cron Jobs (Simple)

```bash
# Add to crontab (crontab -e)

# Content aggregation at 08:00 UTC
0 8 * * * cd /path/to/social && /path/to/venv/bin/python workflows/gizzi_content_aggregator.py

# Auto-posting 3x daily
0 10 * * * cd /path/to/social && /path/to/venv/bin/python workflows/gizzi_auto_poster.py
0 15 * * * cd /path/to/social && /path/to/venv/bin/python workflows/gizzi_auto_poster.py
0 20 * * * cd /path/to/social && /path/to/venv/bin/python workflows/gizzi_auto_poster.py
```

### Option 2: Systemd Services (Robust)

**Engagement Bot Service** (`/etc/systemd/system/gizzi-engagement.service`):
```ini
[Unit]
Description=Gizzi Engagement Bot
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/path/to/social
Environment="PATH=/path/to/venv/bin"
ExecStart=/path/to/venv/bin/python workflows/gizzi_engagement_bot.py --mode stream
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Aggregation Timer** (`/etc/systemd/system/gizzi-aggregation.timer`):
```ini
[Unit]
Description=Gizzi Content Aggregation Daily

[Timer]
OnCalendar=*-*-* 08:00:00
Persistent=true

[Install]
WantedBy=timers.target
```

Enable and start:
```bash
sudo systemctl enable gizzi-engagement.service
sudo systemctl start gizzi-engagement.service

sudo systemctl enable gizzi-aggregation.timer
sudo systemctl start gizzi-aggregation.timer
```

### Option 3: PM2 (Easy Process Management)

```bash
# Install PM2
npm install -g pm2

# Start engagement bot (continuous)
pm2 start workflows/gizzi_engagement_bot.py --name "gizzi-engagement" --interpreter python3 -- --mode stream

# Schedule aggregation with cron
pm2 start workflows/gizzi_content_aggregator.py --name "gizzi-aggregator" --cron "0 8 * * *" --interpreter python3 --no-autorestart

# Schedule auto-posting
pm2 start workflows/gizzi_auto_poster.py --name "gizzi-poster" --cron "0 10,15,20 * * *" --interpreter python3 --no-autorestart

# Save process list
pm2 save

# Auto-start on boot
pm2 startup
```

### Option 4: Docker Compose (Containerized)

See existing `docker-compose.yml` in parent directory.

---

## 📊 Monitoring

### Logs

```bash
# View workflow logs
tail -f logs/social_automation.log

# View aggregation data
cat data/aggregated_content/latest.json | jq .

# View posting history
ls -lah data/posting_history/

# View engagement logs
tail -f data/engagement/engagement_$(date +%Y%m).jsonl
```

### Metrics

Track these metrics to measure Gizzi's effectiveness:

**Content Aggregation:**
- Articles scraped per day
- Trending topics identified
- Competitor mentions found
- Content opportunities generated

**Auto-Posting:**
- Posts published per day
- Engagement rate (likes, RTs, replies)
- Topics covered
- Thread vs single tweet ratio

**Engagement:**
- Mentions responded to
- Response time (avg)
- Interaction types breakdown
- Hourly engagement rate

### Analytics

```bash
# Engagement stats for current month
cat data/engagement/engagement_$(date +%Y%m).jsonl | \
  jq -s 'group_by(.interaction_type) | map({type: .[0].interaction_type, count: length})'

# Posting frequency
ls data/posting_history/ | wc -l

# Top trending topics
cat data/aggregated_content/latest.json | jq .news_digest.trending_topics
```

---

## 🔧 Troubleshooting

### Common Issues

#### "No aggregated data found"

**Cause:** Auto-poster running before aggregator
**Solution:**
```bash
# Run aggregator first
python workflows/gizzi_content_aggregator.py
# Then run auto-poster
python workflows/gizzi_auto_poster.py
```

#### "Reddit API authentication failed"

**Cause:** Invalid Reddit credentials
**Solution:**
1. Verify credentials in `.env`
2. Check Reddit app at https://www.reddit.com/prefs/apps
3. Ensure account has API access
4. Test connection:
```bash
python -c "from connectors.reddit import RedditConnector; import asyncio; asyncio.run(RedditConnector().search_mentions('test', limit=1))"
```

#### "Rate limit exceeded"

**Cause:** Too many API calls
**Solution:**
- Reduce posting frequency in `posting_schedule.yaml`
- Lower `hourly_limit` in engagement bot
- Add delays between API calls
- Upgrade Twitter API tier

#### "Claude API error"

**Cause:** Invalid API key or rate limits
**Solution:**
1. Verify `ANTHROPIC_API_KEY` in `.env`
2. Check API usage at https://console.anthropic.com/
3. Set `MAX_DAILY_CLAUDE_SPEND` limit
4. Use caching to reduce API calls

#### "No content opportunities found"

**Cause:** Insufficient trending topics or recent posts
**Solution:**
- Wait for more content to aggregate
- Adjust keyword filters in `web_scraper.py`
- Add more RSS feeds
- Lower trending topic threshold

---

## 💡 Best Practices

### Content Quality

1. **Don't Over-Post** - Max 5-6 posts per day
2. **Provide Value** - Every post should educate, inform, or engage
3. **Be Authentic** - Use Gizzi's thoughtful, strategic voice
4. **Mix Content Types** - Balance promotional, educational, engagement posts
5. **Monitor Feedback** - Adjust strategy based on engagement metrics

### Engagement Guidelines

1. **Respond Thoughtfully** - Take time to craft good replies
2. **Thank Supporters** - Always acknowledge positive engagement
3. **Handle Criticism Well** - Stay professional, don't get defensive
4. **Avoid Arguments** - If discussion turns negative, disengage gracefully
5. **Respect Rate Limits** - Don't spam, even with good content

### Security

1. **Never Commit .env** - Keep credentials secret
2. **Use Moderation** - Always moderate content before posting
3. **Monitor Costs** - Set Claude API spend limits
4. **Review Logs** - Check for suspicious activity
5. **Human Oversight** - Review engagement bot responses periodically

### Optimization

1. **A/B Test Posting Times** - Find when your audience is most active
2. **Track What Works** - Analyze high-performing posts
3. **Iterate on Prompts** - Improve Claude prompts based on results
4. **Expand Sources** - Add new RSS feeds and subreddits
5. **Automate Reporting** - Build dashboards for key metrics

---

## 📈 Success Metrics

Track these KPIs to measure Gizzi's impact:

### Reach
- Twitter followers growth
- Tweet impressions
- Profile visits

### Engagement
- Likes per post
- Retweets per post
- Replies per post
- Engagement rate (total engagements / impressions)

### Awareness
- Etrid mentions on Twitter
- Etrid mentions on Reddit
- Search volume for "Etrid"

### Community
- Questions answered
- Supporters thanked
- Discussions participated in

### Conversions
- Website visits from Twitter
- Documentation page views
- Developer sign-ups

---

## 🔮 Future Enhancements

Planned improvements for Gizzi:

1. **Image Generation** - Create charts, infographics with AI
2. **Video Posts** - Generate explainer videos
3. **Multi-Platform** - Expand to Discord, Telegram, LinkedIn
4. **Influencer Targeting** - Identify and engage with crypto influencers
5. **Sentiment Dashboard** - Real-time sentiment tracking
6. **Competitive Intelligence** - Deep analysis of competitor strategies
7. **Content Calendar** - Plan ahead for events, launches
8. **A/B Testing** - Automated testing of different content styles
9. **Thread Auto-Generation** - AI-generated educational threads
10. **Community Insights** - What does the community want to know?

---

## 📚 Additional Resources

- [Main Social Automation README](README.md)
- [Twitter Setup Guide](../TWITTER_SETUP_GUIDE.md)
- [MCP Orchestrator Documentation](../docs/MCP_SOCIAL_AUTOMATION.md)
- [Content Generator Documentation](content/README.md)
- [Moderation Guidelines](content/MODERATION.md)

---

## 🤝 Contributing

To improve Gizzi workflows:

1. Test changes with `--dry-run`
2. Monitor metrics before and after changes
3. Document new features in this file
4. Update `posting_schedule.yaml` if adding workflows
5. Run tests to ensure stability

---

## 📞 Support

**Issues:**
- Check logs: `tail -f logs/social_automation.log`
- Review aggregated data: `cat data/aggregated_content/latest.json`
- Test individual components with `--dry-run`

**Questions:**
- Refer to main [README.md](README.md)
- Check GLOBAL_MEMORY.md for AI Dev activities
- Review workflow source code for implementation details

---

**Status:** Production-Ready ✅
**Version:** 1.0
**Last Updated:** November 2025
**Maintained by:** Gizzi (AI Dev)

---

## License

This is part of the Etrid blockchain project. See main project LICENSE for details.
