# Etrid Operations Center - Quick Start Guide

Get up and running in 5 minutes! 🚀

## Overview

You've just created a complete blockchain operations center using Pinokio's AI browser platform. Here's what you can do:

✅ Monitor all Etrid nodes (FlareChain + 12 PBCs) from one dashboard
✅ SSH into any VM across AWS, GCP, Azure, DigitalOcean from one interface
✅ Access from any browser, anywhere in the world
✅ AI-powered log analysis and debugging with Claude Code
✅ Automated health checks and maintenance
✅ Real-time status updates via WebSockets

## Installation

### Option 1: One-Click Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/EojEdred/Etrid/main/pinokio-etrid-ops/install.sh | bash
```

### Option 2: Manual Install

```bash
# 1. Install Pinokio
# Download from: https://github.com/pinokiocomputer/pinokio/releases

# 2. Copy files
cp -r pinokio-etrid-ops/api/etrid ~/pinokio/api/
cp -r pinokio-etrid-ops/dashboard ~/pinokio/etrid-dashboard
cp -r pinokio-etrid-ops/scripts ~/pinokio/etrid-scripts

# 3. Install dependencies
cd ~/pinokio/api/etrid && npm install
cd ~/pinokio/etrid-dashboard && npm install
```

## Configuration

### Step 1: Add Your Nodes

Edit `~/pinokio/api/etrid/config.json`:

```json
{
  "chains": {
    "flarechain": {
      "nodes": [
        {
          "name": "flarechain-validator-1",
          "type": "validator",
          "cloud": "aws",
          "ip": "YOUR_NODE_IP",
          "sshUser": "ubuntu",
          "sshKey": "~/.ssh/your-key.pem",
          "wsPort": 9945,
          "rpcPort": 9944
        }
      ]
    },
    "pbcs": [
      {
        "name": "BTC-PBC",
        "nodes": [
          {
            "name": "btc-collator-1",
            "ip": "YOUR_BTC_NODE_IP",
            "cloud": "gcp",
            "sshUser": "etrid",
            "sshKey": "~/.ssh/gcp-key"
          }
        ]
      }
      // Add more PBC nodes...
    ]
  }
}
```

### Step 2: Set Up SSH Keys

Make sure your SSH keys are in place:

```bash
# AWS
chmod 600 ~/.ssh/aws-etrid.pem

# GCP
chmod 600 ~/.ssh/gcp-etrid

# Azure
chmod 600 ~/.ssh/azure-etrid

# DigitalOcean
chmod 600 ~/.ssh/do-etrid
```

## Launch Dashboard

### In Pinokio:

1. Open Pinokio application
2. Go to **Discover** → **Local**
3. Find "etrid-dashboard"
4. Click **Install** (first time only)
5. Click **Start**
6. Dashboard opens at `http://localhost:8080`

### Via Command Line:

```bash
cd ~/pinokio/etrid-dashboard
npm start
```

## Enable Remote Access

### WiFi Sharing (Local Network):

1. In Pinokio, click **Share** icon (top-right)
2. Select **WiFi**
3. Scan QR code on mobile devices
4. Or use the local IP URL shown

### Internet Sharing (Global Access):

1. In Pinokio, click **Share** icon
2. Select **Internet**
3. Set a strong passcode
4. Save the Cloudflare URL provided
5. Access from anywhere!

**Your URL**: `https://random-name-12345.trycloudflare.com`

## Using the Dashboard

### Status Tab

- View all nodes at a glance
- See online/offline/syncing status
- Check block heights and peer counts
- Real-time updates every 30 seconds

### Health Tab

- Click "Run Health Check"
- See critical issues and warnings
- Get recommendations for fixes
- Enable auto-fix for common problems

### Logs Tab

- Select chain and timeframe
- Click "Fetch Logs"
- Or "Analyze with AI" for Claude Code analysis
- Get actionable insights

### Nodes Tab

- List all configured nodes
- View node details
- SSH directly into nodes
- Execute commands across multiple nodes

## Common Operations

### Check Status of All Chains

```javascript
// In Pinokio, run: scripts/status-all.json
// Or via API:
{
  "method": "etrid.status",
  "params": { "chains": "all" }
}
```

### SSH into a Node

```javascript
{
  "method": "etrid.connect",
  "params": {
    "node": "flarechain-validator-1",
    "cloud": "aws"
  }
}
```

### Run Command on Multiple Nodes

```javascript
{
  "method": "etrid.exec",
  "params": {
    "nodes": ["validator-1", "validator-2"],
    "command": "systemctl status substrate",
    "parallel": true
  }
}
```

### Health Check with Auto-Fix

```javascript
{
  "method": "etrid.healthcheck",
  "params": {
    "chains": "all",
    "fix": true
  }
}
```

### Analyze Logs with AI

```javascript
{
  "method": "etrid.logs",
  "params": {
    "chains": "all",
    "since": "1h",
    "analyze": true
  }
}
```

## Automation Scripts

Pre-built scripts in `~/pinokio/etrid-scripts/`:

- `status-all.json` - Check all node status
- `status-flarechain.json` - Check FlareChain only
- `health-check.json` - Comprehensive health check
- `health-check-autofix.json` - Health check with fixes
- `logs-all.json` - Fetch all logs
- `logs-analyze.json` - Fetch and analyze with AI
- `list-nodes.json` - List all configured nodes
- `update-chains.json` - Update node software

Run any script in Pinokio's script runner!

## Troubleshooting

### Dashboard won't start

```bash
# Check if port 8080 is in use
lsof -i :8080

# Kill the process or change port
export PORT=8081
npm start
```

### Can't connect to nodes

```bash
# Test SSH manually
ssh -i ~/.ssh/your-key.pem user@node-ip

# Check firewall
# Check SSH key permissions (should be 600)
```

### Remote access not working

1. Check internet connection
2. Verify Pinokio is running
3. Try disabling and re-enabling sharing
4. Check Cloudflare status

### Node status shows "offline"

1. Check node is actually running
2. Verify WebSocket port (9945) is open
3. Check firewall rules
4. Test RPC endpoint manually:
   ```bash
   wscat -c ws://node-ip:9945
   ```

## Security Best Practices

✅ **Use strong passcodes** (16+ characters)
✅ **SSH keys only** (no password authentication)
✅ **Firewall rules** (whitelist IPs when possible)
✅ **Regular updates** (keep dashboard and nodes updated)
✅ **Audit logs** (review access logs regularly)
✅ **2FA** on cloud provider accounts
✅ **Backup** your configuration

## Performance Tips

- **Reduce refresh rate** if dashboard is slow
- **Filter nodes** when you have many nodes
- **Use local access** when on same network
- **Limit log history** for faster loading
- **Close unused tabs** to save resources

## Advanced Usage

### Custom Pinokio Scripts

Create your own automation:

```json
{
  "title": "My Custom Task",
  "run": [
    {
      "method": "etrid.status",
      "params": { "chains": "flarechain" }
    },
    {
      "method": "etrid.healthcheck",
      "params": { "chains": "flarechain", "fix": true }
    }
  ]
}
```

### Integration with Other Tools

```bash
# Export metrics to Prometheus
curl http://localhost:8080/api/status > /var/prometheus/etrid.prom

# Send alerts to Telegram
# Edit config.json and add Telegram credentials
```

### Claude Code Integration

The dashboard integrates with Claude Code CLI:

```bash
# Ask Claude about your infrastructure
etrid.claude.ask "Why is validator-1 missing blocks?"

# Get optimization suggestions
etrid.claude.optimizeConfig "flarechain"

# Generate runbooks
etrid.claude.generateRunbook ["Update nodes", "Restart services"]
```

## Next Steps

1. ✅ Configure all your nodes
2. ✅ Set up alerts (Telegram, Discord, email)
3. ✅ Create backup schedules
4. ✅ Document your runbooks
5. ✅ Train team members
6. ✅ Set up monitoring dashboards
7. ✅ Create automation scripts

## Resources

- **Full Documentation**: `README.md`
- **Remote Access Guide**: `REMOTE_ACCESS_SETUP.md`
- **Validator Package**: `VALIDATOR_PACKAGE.md`
- **Pinokio Docs**: https://docs.pinokio.computer/
- **Etrid GitHub**: https://github.com/EojEdred/Etrid

## Support

- **Issues**: GitHub Issues
- **Discord**: [Your Discord link]
- **Telegram**: [Your Telegram link]
- **Email**: support@etrid.io

## Summary

You now have a production-ready operations center that:

- Monitors all your Etrid nodes
- Provides remote access from anywhere
- Automates maintenance tasks
- Integrates with AI for debugging
- Scales from 1 to 1000+ nodes
- Works across all cloud providers

**Start monitoring your mainnet like a pro! 🚀**
