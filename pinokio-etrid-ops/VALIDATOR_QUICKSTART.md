# Etrid Validator - Quick Start Guide

Welcome! This guide will help you get your Etrid validator up and running in minutes.

## 📦 What's Included

- **Etrid Validator Node** - Your blockchain validator
- **Operations Center** - Beautiful web dashboard to monitor your validator
- **Auto-Discovery** - Automatically detects and configures your validator
- **SSH Terminal** - Access your validator from your browser or Terminus app

---

## 🚀 One-Command Start

```bash
./start-validator.sh
```

That's it! This single command will:
1. Start your validator node
2. Launch the Operations Center
3. Auto-detect your validator
4. Display access URLs

---

## 🌐 Access Your Dashboard

Once started, open your browser to:

**Local Access:**
```
http://localhost:8080
```

**Remote Access (Tailscale):**
```
http://YOUR-TAILSCALE-IP:8080
```

---

## 🔐 First-Time Setup

### 1. Create Your Account

On first launch, you'll see the registration page:

1. Enter your name and email
2. Choose a strong password
3. Click "Create Account"

**That's it!** Your validator will be automatically detected and configured.

### 2. What Happens Automatically

- ✅ Detects your validator on `ws://localhost:9944`
- ✅ Reads chain info (Etrid Mainnet/Testnet)
- ✅ Configures monitoring and alerts
- ✅ Sets up SSH access
- ✅ Logs you into the dashboard

You'll see a confirmation like:
```
🎉 Welcome aboard!

Your validator has been automatically detected and configured:
Etrid Mainnet v1.0.0

Logging you in...
```

---

## 📊 Dashboard Overview

### Hero Status
Big, clear display showing if your validator is online or offline.

### Key Metrics
- **Block Height** - Current blockchain height
- **Peers** - Number of connected peers
- **Uptime** - Validator uptime percentage
- **Blocks Authored** - Blocks you've produced

### Quick Actions
- **💻 Web Terminal** - SSH into your validator from the browser
- **🚀 Open in Terminus** - Launch native Terminus app
- **📄 View Logs** - Stream validator logs in real-time
- **🏥 Health Check** - Run diagnostics

---

## 🔧 Configuration

### Environment Variables

Edit `.env` file to customize:

```bash
# SSH Configuration
SSH_KEY_PATH=~/.ssh/id_rsa    # Path to your SSH key
SSH_DEFAULT_USER=ubuntu        # Your SSH username

# Tailscale (for remote access)
USE_TAILSCALE=true
TAILSCALE_DOMAIN=your-tailnet.ts.net

# Alerts (optional)
TELEGRAM_BOT_TOKEN=your-bot-token
DISCORD_WEBHOOK_URL=your-webhook-url
```

---

## 🌍 Remote Access with Tailscale

### 1. Install Tailscale (if not already)

```bash
# Ubuntu/Debian
curl -fsSL https://tailscale.com/install.sh | sh

# Start Tailscale
sudo tailscale up
```

### 2. Access from Anywhere

Once Tailscale is running:

```bash
# Get your Tailscale IP
tailscale ip -4

# Access dashboard from anywhere on your tailnet
http://YOUR-TAILSCALE-IP:8080
```

### 3. SSH from Any Device

From your laptop, phone, or tablet (with Tailscale):
- Click "Web Terminal" for browser-based SSH
- Click "Open in Terminus" for native app

---

## 🛑 Stop Your Validator

```bash
./stop-validator.sh
```

This gracefully shuts down:
- Validator node
- Operations Center

---

## 📁 Directory Structure

```
etrid-validator/
├── etrid-node              # Validator binary
├── start-validator.sh      # Start everything
├── stop-validator.sh       # Stop everything
├── etrid-ops/              # Operations Center
│   ├── dashboard/          # Web UI
│   └── api/                # Backend
├── data/                   # Blockchain data
├── logs/                   # Validator & ops logs
└── .env                    # Your configuration
```

---

## 📝 Logs

### View Logs

**Validator logs:**
```bash
tail -f logs/validator.log
```

**Operations Center logs:**
```bash
tail -f logs/ops-center.log
```

### Log Locations
- Validator: `logs/validator.log`
- Ops Center: `logs/ops-center.log`
- Blockchain data: `data/`

---

## 🔐 Security Best Practices

1. **Change Default SSH Port** (optional)
   ```bash
   # In your node config
   ssh_port: 2222  # Instead of 22
   ```

2. **Use Tailscale** for remote access (recommended)
   - End-to-end encrypted
   - No port forwarding needed
   - Works across NAT/firewalls

3. **Backup Your Keys**
   ```bash
   # Your validator keys are in:
   data/chains/etrid/keystore/

   # BACK THESE UP SECURELY!
   ```

4. **Monitor Regularly**
   - Check your dashboard daily
   - Set up Telegram/Discord alerts
   - Review logs for errors

---

## 🆘 Troubleshooting

### Validator Not Detected

**Problem:** "No validator configured" message

**Solutions:**
1. Check validator is running:
   ```bash
   ps aux | grep etrid-node
   ```

2. Check validator is listening:
   ```bash
   curl -H "Content-Type: application/json" \
        -d '{"id":1,"jsonrpc":"2.0","method":"system_chain"}' \
        http://localhost:9933
   ```

3. Restart everything:
   ```bash
   ./stop-validator.sh
   ./start-validator.sh
   ```

### Can't Access Dashboard

**Problem:** Can't reach `http://localhost:8080`

**Solutions:**
1. Check ops center is running:
   ```bash
   ps aux | grep server.js
   ```

2. Check logs:
   ```bash
   tail -f logs/ops-center.log
   ```

3. Check port is not in use:
   ```bash
   lsof -i :8080
   ```

### SSH Terminal Not Working

**Problem:** Can't connect via web terminal

**Solutions:**
1. Check SSH key path in `.env`:
   ```bash
   SSH_KEY_PATH=~/.ssh/id_rsa
   ```

2. Verify key permissions:
   ```bash
   chmod 600 ~/.ssh/id_rsa
   ```

3. Test SSH manually:
   ```bash
   ssh localhost
   ```

---

## 📞 Support

- **Documentation:** [https://docs.etrid.org](https://docs.etrid.org)
- **Discord:** [https://discord.gg/etrid](https://discord.gg/etrid)
- **GitHub:** [https://github.com/etrid/etrid](https://github.com/etrid/etrid)

---

## 🎯 Next Steps

1. ✅ **Monitor Your Validator** - Check dashboard daily
2. ✅ **Set Up Alerts** - Configure Telegram/Discord in Settings
3. ✅ **Backup Keys** - Secure your validator keys
4. ✅ **Join Community** - Connect with other validators on Discord
5. ✅ **Stay Updated** - Follow Etrid announcements

---

**Happy Validating! 🔥**
