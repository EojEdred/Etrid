# 🤖 MasterChef Telegram Bot

Interactive Telegram bot for monitoring your MasterChef LP rewards program.

**Get real-time metrics, alerts, and manage your MasterChef directly from Telegram!**

---

## 🎯 What This Does

The Telegram bot provides:

✅ **Real-Time Monitoring** - Check TVL, APR, balance, and pool stats instantly
✅ **Alert Notifications** - Get notified when issues are detected
✅ **Interactive Commands** - Query any metric with simple commands
✅ **Admin Controls** - Manage alerts and broadcast to subscribers
✅ **Multi-Network** - Support for mainnet and testnet
✅ **User-Friendly** - Clean interface with markdown formatting

---

## 🚀 Quick Start

### 1. Create Telegram Bot

1. **Open Telegram** and search for `@BotFather`
2. **Send** `/newbot`
3. **Follow prompts** to name your bot
4. **Copy the bot token** (looks like `123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11`)

### 2. Get Your Telegram User ID

1. **Search for** `@userinfobot` on Telegram
2. **Send** `/start`
3. **Copy your user ID** (a number like `123456789`)

### 3. Configure Environment Variables

Add to your `.env` file:

```bash
# Telegram Bot Configuration
TELEGRAM_BOT_TOKEN=123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11
TELEGRAM_ADMIN_IDS=123456789,987654321  # Comma-separated for multiple admins
DEFAULT_NETWORK=mainnet

# Alert Thresholds (optional)
LOW_BALANCE_THRESHOLD=1000000    # 1M ÉTR
CRITICAL_BALANCE_THRESHOLD=500000  # 500K ÉTR
TVL_DROP_THRESHOLD=10              # 10% drop
APR_DROP_THRESHOLD=20              # 20% drop

# Alert Check Interval (optional)
ALERT_CHECK_INTERVAL=300000  # 5 minutes in milliseconds
```

### 4. Install Dependencies

```bash
cd 05-multichain/bridge/adapters/bsc
npm install
```

This installs `node-telegram-bot-api` and other dependencies.

### 5. Ensure Database Has Data

```bash
# Collect initial metrics
npm run collect-metrics:mainnet

# Or testnet
npm run collect-metrics:testnet
```

### 6. Start the Bot

```bash
npm run telegram-bot
```

**Output:**
```
🚀 Starting MasterChef Telegram Bot...

🤖 MasterChef Telegram Bot Started

   Bot Token: 123456:ABC...
   Admins: 123456789
   Default Network: mainnet

✅ Bot is running! Send /start in Telegram to begin.

Press Ctrl+C to stop
```

### 7. Use the Bot

1. **Open Telegram**
2. **Search for your bot** by name
3. **Send** `/start`
4. **Enjoy!**

---

## 📋 Commands

### User Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/start` | Welcome message and quick help | `/start` |
| `/help` | Show all commands | `/help` |
| `/tvl [network]` | Show Total Value Locked | `/tvl mainnet` |
| `/apr [network] [poolId]` | Show APR for pool(s) | `/apr mainnet 0` |
| `/pools [network]` | List all pools with stats | `/pools mainnet` |
| `/balance [network]` | Show MasterChef balance | `/balance mainnet` |
| `/health [network]` | System health check | `/health mainnet` |
| `/alerts` | Show active alerts | `/alerts` |
| `/prices` | Show BNB and ÉTR prices | `/prices` |
| `/stats [network]` | Complete statistics | `/stats mainnet` |
| `/subscribe` | Get alert notifications | `/subscribe` |
| `/unsubscribe` | Stop alert notifications | `/unsubscribe` |

### Admin Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/ack <alertId>` | Acknowledge an alert | `/ack 42` |
| `/broadcast <message>` | Send message to all subscribers | `/broadcast Maintenance in 1 hour` |

**Parameters:**
- `network` - `mainnet` or `testnet` (defaults to `DEFAULT_NETWORK` from .env)
- `poolId` - Pool index (0, 1, 2, etc.)

---

## 💬 Usage Examples

### Check TVL

```
You: /tvl mainnet

Bot:
📊 Total Value Locked

Network: BSC Mainnet
Block: 34,567,890

💰 Total TVL: $1,234,567
Total Staked: 98,765.43 LP

Last Updated: 10/24/2025, 2:15:32 PM
```

### Check APR for Specific Pool

```
You: /apr mainnet 0

Bot:
📈 Pool 0 APR

Symbol: ÉTR-BNB LP
Name: PancakeSwap ÉTR-BNB LP

💹 APR: 142.56%
TVL: $567,890

Staked: 45,678.90 LP
Reward Share: 50.0%
Daily Rewards: 1,150.68 ÉTR
Monthly Rewards: 34,520.55 ÉTR
```

### List All Pools

```
You: /pools mainnet

Bot:
🏊 Liquidity Pools

Network: mainnet
Total Pools: 3

Pool 0: ÉTR-BNB LP
  Staked: 45,678.90 LP
  TVL: $567,890
  APR: 142.56%
  Share: 50.0%

Pool 1: ÉTR-BUSD LP
  Staked: 23,456.78 LP
  TVL: $345,678
  APR: 95.43%
  Share: 30.0%

Pool 2: ÉTR-USDT LP
  Staked: 12,345.67 LP
  TVL: $198,765
  APR: 78.21%
  Share: 20.0%
```

### Check Balance

```
You: /balance mainnet

Bot:
💰 MasterChef Balance

Network: mainnet
Balance: 18,456,789 ÉTR
Emission: 11.57 ÉTR/block
Days Remaining: 156 days

Last Updated: 10/24/2025, 2:15:32 PM
```

### Health Check

```
You: /health mainnet

Bot:
🏥 Health Check

Network: mainnet

🟢 Status: Active
🟢 Balance: OK (18,456,789 ÉTR)
🟢 Pools: 3 active
🟢 Data: Fresh

Last Updated: 10/24/2025, 2:15:32 PM
```

### View Alerts

```
You: /alerts

Bot:
🚨 Active Alerts (2)

🟡 Alert #42
Network: mainnet
Type: low_balance
Message: MasterChef balance below 1M ÉTR (987,654 remaining)
Time: 10/24/2025, 1:45:12 PM

🔴 Alert #43
Network: mainnet
Type: tvl_drop
Message: TVL dropped 12% in last hour ($1.2M → $1.05M)
Time: 10/24/2025, 2:10:05 PM

Use /ack <id> to acknowledge (admin only)
```

### Get Prices

```
You: /prices

Bot:
💵 Current Prices

BNB: $312.45
ÉTR: $0.000123
```

### Complete Statistics

```
You: /stats mainnet

Bot:
📊 Complete Statistics

Network: BSC Mainnet
Block: 34,567,890

MasterChef:
  Balance: 18,456,789 ÉTR
  Emission: 11.57 ÉTR/block
  Days Left: 156 days
  Status: Active

TVL:
  Total: $1,234,567
  Staked: 98,765.43 LP

Pools: 3 active

Prices:
  BNB: $312.45
  ÉTR: $0.000123

Updated: 10/24/2025, 2:15:32 PM
```

---

## 🔔 Alert Notifications

### Subscribe to Alerts

```
You: /subscribe

Bot: ✅ Subscribed to alerts! You'll receive notifications when issues are detected.
```

Now you'll receive real-time notifications like:

```
Bot:
🔴 New Alert

Network: mainnet
Severity: CRITICAL
Type: critical_balance

MasterChef balance critically low: 456,789 ÉTR remaining (< 30 days)

Use /alerts to see all active alerts
```

### Unsubscribe

```
You: /unsubscribe

Bot: ✅ Unsubscribed from alerts.
```

---

## 👑 Admin Features

### Acknowledge Alerts

```
Admin: /ack 42

Bot: ✅ Alert #42 acknowledged by @admin_username
```

This marks the alert as acknowledged in the database and stops it from being sent to new subscribers.

### Broadcast Messages

```
Admin: /broadcast Scheduled maintenance tonight at 11 PM UTC. Expect 10 minutes downtime.

Bot: ✅ Broadcast sent to 15 subscriber(s)
```

All subscribed users receive:

```
Bot:
📢 Broadcast

Scheduled maintenance tonight at 11 PM UTC. Expect 10 minutes downtime.
```

---

## ⚙️ Configuration

### Environment Variables

**Required:**
- `TELEGRAM_BOT_TOKEN` - Bot token from @BotFather
- `TELEGRAM_ADMIN_IDS` - Comma-separated user IDs who can use admin commands
- Contract addresses (already in your .env)

**Optional:**
- `DEFAULT_NETWORK` - Default network for commands (default: `mainnet`)
- `LOW_BALANCE_THRESHOLD` - Balance threshold for warnings (default: `1000000`)
- `CRITICAL_BALANCE_THRESHOLD` - Balance threshold for critical alerts (default: `500000`)
- `TVL_DROP_THRESHOLD` - TVL drop % for alerts (default: `10`)
- `APR_DROP_THRESHOLD` - APR drop % for alerts (default: `20`)
- `ALERT_CHECK_INTERVAL` - How often to check for alerts in ms (default: `300000` = 5 min)

### Alert Thresholds

Customize when you get notified:

```bash
# Conservative (get alerts early)
LOW_BALANCE_THRESHOLD=2000000     # Alert at 2M ÉTR
CRITICAL_BALANCE_THRESHOLD=1000000  # Critical at 1M ÉTR
TVL_DROP_THRESHOLD=5                # Alert on 5% TVL drop
APR_DROP_THRESHOLD=10               # Alert on 10% APR drop

# Relaxed (only major issues)
LOW_BALANCE_THRESHOLD=500000      # Alert at 500K ÉTR
CRITICAL_BALANCE_THRESHOLD=250000   # Critical at 250K ÉTR
TVL_DROP_THRESHOLD=20               # Alert on 20% TVL drop
APR_DROP_THRESHOLD=30               # Alert on 30% APR drop
```

---

## 🔄 Running as Service

### Using PM2 (Recommended)

```bash
# Install PM2
npm install -g pm2

# Start bot
pm2 start npm --name "masterchef-bot" -- run telegram-bot

# Save PM2 config
pm2 save

# Setup auto-restart on reboot
pm2 startup

# Monitor
pm2 logs masterchef-bot
pm2 status

# Restart
pm2 restart masterchef-bot

# Stop
pm2 stop masterchef-bot
```

### Using systemd (Linux)

```bash
# Create service file
sudo nano /etc/systemd/system/masterchef-bot.service
```

```ini
[Unit]
Description=MasterChef Telegram Bot
After=network.target

[Service]
Type=simple
User=youruser
WorkingDirectory=/path/to/etrid/05-multichain/bridge/adapters/bsc
ExecStart=/usr/bin/npm run telegram-bot
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start
sudo systemctl enable masterchef-bot
sudo systemctl start masterchef-bot

# Check status
sudo systemctl status masterchef-bot

# View logs
sudo journalctl -u masterchef-bot -f
```

### Using Docker

```dockerfile
# Dockerfile
FROM node:20-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --production

COPY . .

CMD ["npm", "run", "telegram-bot"]
```

```bash
# Build
docker build -t masterchef-bot .

# Run
docker run -d \
  --name masterchef-bot \
  --restart unless-stopped \
  --env-file .env \
  masterchef-bot

# Logs
docker logs -f masterchef-bot
```

---

## 🔒 Security Best Practices

### 1. Protect Bot Token

**Never commit** `.env` to git:

```bash
# .gitignore should include:
.env
.env.*
```

**Regenerate token** if exposed:
1. Go to @BotFather
2. Send `/revoke`
3. Select your bot
4. Get new token

### 2. Restrict Admin Access

Only add trusted user IDs to `TELEGRAM_ADMIN_IDS`:

```bash
# Single admin
TELEGRAM_ADMIN_IDS=123456789

# Multiple admins
TELEGRAM_ADMIN_IDS=123456789,987654321,555444333
```

### 3. Rate Limiting

The bot uses Telegram's built-in rate limiting, but you can add custom limits if needed.

### 4. Command Validation

All commands validate inputs and check permissions before executing.

---

## 🐛 Troubleshooting

### Bot doesn't respond

**Check if bot is running:**
```bash
pm2 status masterchef-bot
# or
ps aux | grep telegram-bot
```

**Check logs:**
```bash
pm2 logs masterchef-bot
# or
tail -f /var/log/masterchef-bot.log
```

**Verify token:**
```bash
# Test bot token
curl https://api.telegram.org/bot<YOUR_BOT_TOKEN>/getMe
```

### "No metrics found" error

**Run metrics collection:**
```bash
npm run collect-metrics:mainnet
```

**Check database:**
```bash
sqlite3 database/masterchef.db "SELECT COUNT(*) FROM metrics_snapshots;"
```

### Alerts not being sent

**Check subscription:**
```
/subscribe
```

**Check alert interval:**
```bash
# In .env
ALERT_CHECK_INTERVAL=300000  # 5 minutes
```

**Verify alerts exist:**
```
/alerts
```

### "Admin only command" error

**Verify your user ID** is in `TELEGRAM_ADMIN_IDS`:

```bash
# Get your ID from @userinfobot
# Then add to .env
TELEGRAM_ADMIN_IDS=123456789
```

---

## 📊 Integration with Other Systems

### Combine with Metrics Collection

Run both the metrics collector and bot together:

```bash
# Terminal 1: Metrics collector (cron)
npm run collect-metrics:mainnet

# Terminal 2: Telegram bot
npm run telegram-bot
```

Or use PM2:

```bash
pm2 start npm --name "metrics-collector" -- run collect-metrics:mainnet
pm2 start npm --name "telegram-bot" -- run telegram-bot
```

### Export Bot Data

The bot reads from the same database as other tools:

```bash
# Export metrics while bot is running
npm run export-metrics:mainnet

# Query history while bot is running
npm run query-history
```

---

## 🎓 Advanced Usage

### Custom Bot Commands

You can extend the bot by adding new commands in `telegram-bot/bot.ts`:

```typescript
// Add to setupCommands()
this.bot.onText(/\/mycustom/, (msg) => this.handleMyCustomCommand(msg));

// Add handler
private async handleMyCustomCommand(msg: TelegramBot.Message) {
  const chatId = msg.chat.id;
  // Your custom logic here
  this.bot.sendMessage(chatId, "Custom response!");
}
```

### Webhook Mode (for production)

Instead of polling, use webhooks for better performance:

```typescript
// In bot.ts constructor
this.bot = new TelegramBot(config.botToken, {
  webHook: {
    port: 8443,
    host: "0.0.0.0",
  },
});

this.bot.setWebHook(`https://your-domain.com:8443/bot${config.botToken}`);
```

### Multi-Language Support

Add language detection and translations:

```typescript
const messages = {
  en: { welcome: "Welcome!" },
  es: { welcome: "¡Bienvenido!" },
};

const lang = msg.from?.language_code || "en";
this.bot.sendMessage(chatId, messages[lang].welcome);
```

---

## 📈 Monitoring Bot Health

### Bot Uptime

```bash
# Using PM2
pm2 show masterchef-bot

# Check uptime
uptime=$(pm2 show masterchef-bot | grep uptime)
echo $uptime
```

### Message Statistics

Add to bot code:

```typescript
private messageCount = 0;
private commandStats: Record<string, number> = {};

// In each handler
this.messageCount++;
this.commandStats[command] = (this.commandStats[command] || 0) + 1;

// Add /botstats command
private async handleBotStats(msg: TelegramBot.Message) {
  let message = `📊 Bot Statistics\n\n`;
  message += `Total Messages: ${this.messageCount}\n`;
  message += `Subscribers: ${this.subscribedUsers.size}\n\n`;
  message += `Commands:\n`;

  for (const [cmd, count] of Object.entries(this.commandStats)) {
    message += `  /${cmd}: ${count}\n`;
  }

  this.bot.sendMessage(msg.chat.id, message);
}
```

---

## 🎉 Summary

✅ **Interactive monitoring** via Telegram
✅ **Real-time alerts** for critical issues
✅ **Admin controls** for management
✅ **Multi-network support** (mainnet/testnet)
✅ **Easy setup** with @BotFather
✅ **Production-ready** with PM2/systemd/Docker
✅ **Secure** with admin-only commands
✅ **Extensible** - add custom commands

**Cost**: $0 (Telegram bots are free!)
**Setup Time**: 5 minutes
**Value**: Priceless monitoring at your fingertips

---

## 🚀 Next Steps

1. **Create your bot** with @BotFather
2. **Configure** `.env` with bot token
3. **Start bot**: `npm run telegram-bot`
4. **Subscribe** to alerts: `/subscribe`
5. **Monitor** your MasterChef from anywhere!

**Related Features:**
- Historical Data Tracking (built)
- Advanced Alert System (coming next)
- Backup & Recovery (coming next)
- Analytics API (coming next)

---

**Questions?** Check the main [SCRIPTS_README.md](SCRIPTS_README.md) or ask in Telegram!

**Ready to monitor from Telegram?** Run `npm run telegram-bot` to start! 🚀
