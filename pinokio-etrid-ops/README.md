# Etrid Blockchain Operations Center - Pinokio Setup

This directory contains a complete Pinokio-based operations center for managing your Etrid mainnet infrastructure across multiple cloud providers.

## Features

- **Multi-Cloud SSH Manager**: Connect to VMs across AWS, GCP, Azure, DigitalOcean from one interface
- **Node Monitoring Dashboard**: Real-time status of all Etrid nodes (FlareChain + 12 PBC chains)
- **Chain Maintenance Tools**: Automated health checks, log analysis, and maintenance tasks
- **Remote Browser Access**: Access from anywhere via Cloudflare tunneling
- **Custom UI**: Etrid-themed interface optimized for blockchain operations
- **Claude Code Integration**: Seamless CLI workflow integration

## Architecture

```
~/pinokio/api/etrid/
├── index.js              # Main Etrid API
├── ssh-manager.js        # Multi-cloud SSH connection handler
├── node-monitor.js       # Node status monitoring
├── chain-health.js       # Health check automation
└── claude-integration.js # Claude Code CLI integration

~/pinokio/
├── etrid-dashboard/      # Custom web UI
│   ├── install.json      # Installation script
│   ├── start.json        # Launch script
│   └── ui/               # Custom HTML/CSS/JS
└── etrid-scripts/        # Automation scripts
    ├── health-check.json
    ├── log-aggregator.json
    └── update-nodes.json
```

## Installation Steps

### 1. Install Pinokio

Download from: https://github.com/pinokiocomputer/pinokio/releases

```bash
# Download latest release for your platform
# Linux example:
wget https://github.com/pinokiocomputer/pinokio/releases/download/[version]/Pinokio-[version].AppImage
chmod +x Pinokio-[version].AppImage
./Pinokio-[version].AppImage
```

### 2. Install Etrid Operations Center

```bash
# Copy files to Pinokio directory
cp -r pinokio-etrid-ops/api/etrid ~/pinokio/api/
cp -r pinokio-etrid-ops/dashboard ~/pinokio/etrid-dashboard
cp -r pinokio-etrid-ops/scripts ~/pinokio/etrid-scripts
```

### 3. Configure Cloud Credentials

Edit `~/pinokio/api/etrid/config.json` with your cloud provider credentials:

```json
{
  "clouds": {
    "aws": {
      "credentials": "~/.aws/credentials",
      "nodes": [...]
    },
    "gcp": {
      "keyfile": "~/.gcp/key.json",
      "nodes": [...]
    }
  }
}
```

### 4. Enable Remote Access

1. Launch Pinokio
2. Click "Local Share" at top right
3. Choose "Internet" (Cloudflare tunnel) or "WiFi" (local network)
4. Set a strong passcode
5. Save the URL - access from any browser!

## Usage

### Start Dashboard

In Pinokio:
1. Go to "Discover" → Local → "etrid-dashboard"
2. Click "Install" (first time only)
3. Click "Start"
4. Access at `http://localhost:8080` or your remote URL

### SSH into Nodes

```javascript
// Via Pinokio script
{
  "run": [{
    "method": "etrid.ssh.connect",
    "params": {
      "node": "flarechain-validator-1",
      "cloud": "aws"
    }
  }]
}
```

### Monitor All Nodes

```javascript
{
  "run": [{
    "method": "etrid.monitor.status",
    "params": {
      "chains": "all"
    }
  }]
}
```

### Health Check All Chains

```bash
# In Pinokio, run: etrid-scripts/health-check.json
```

## Claude Code Integration

The operations center integrates with Claude Code for:

- **Automated Debugging**: Claude analyzes error logs from all nodes
- **Smart Maintenance**: AI-suggested optimizations and fixes
- **Documentation**: Auto-generate runbooks from operations
- **Code Deployment**: Review and deploy updates across all chains

### Example Workflow:

1. Dashboard detects issue on BTC-PBC node
2. Click "Debug with Claude"
3. Claude Code analyzes logs, suggests fix
4. Review and apply fix across affected nodes
5. Monitor deployment in real-time

## Node Configuration

### FlareChain Mainnet

```json
{
  "name": "flarechain-mainnet",
  "type": "validator",
  "cloud": "aws",
  "ssh": {
    "host": "flarechain.example.com",
    "user": "ubuntu",
    "key": "~/.ssh/flarechain.pem"
  },
  "ports": {
    "rpc": 9944,
    "ws": 9945,
    "p2p": 30333
  }
}
```

### PBC Chains (12 chains)

Each PBC chain has similar configuration:
- BTC-PBC, ETH-PBC, SOL-PBC, BNB-PBC, ADA-PBC, DOGE-PBC
- LINK-PBC, MATIC-PBC, SC-USDT-PBC, TRX-PBC, XLM-PBC, XRP-PBC

## Security

- All SSH keys stored locally, encrypted
- Passcode protection on remote access
- No cloud provider credentials in browser
- Audit logs for all operations
- Read-only mode available for monitoring

## Troubleshooting

### Can't connect to remote Pinokio
- Check firewall allows Cloudflare IPs
- Verify passcode is correct
- Try WiFi sharing first for testing

### SSH connection fails
- Verify cloud credentials in config.json
- Check SSH keys have correct permissions (600)
- Test manual SSH connection first

### Node monitoring not updating
- Check node RPC ports are accessible
- Verify WebSocket connections enabled
- Review logs in `~/pinokio/logs/etrid/`

## Next Steps

1. Customize the dashboard UI (`~/pinokio/etrid-dashboard/ui/`)
2. Add alerts (Telegram, Discord, email)
3. Create automated backup scripts
4. Build governance proposal monitoring
5. Add performance metrics visualization

## Resources

- Pinokio Docs: https://docs.pinokio.computer/
- Etrid GitHub: https://github.com/EojEdred/Etrid
- Support: [Your contact info]
