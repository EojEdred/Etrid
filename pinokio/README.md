# ÉTRID Pinokio Tools

Complete Pinokio integration for ÉTRID blockchain ecosystem, providing:
- 🌐 **Web UI Suite**: 5 Next.js applications for blockchain interaction
- 🔧 **Validator Management**: Remote CLI access to 21 distributed validators
- 🤖 **AI-Powered Monitoring**: Intelligent health analysis and recommendations

## Overview

This directory contains the Pinokio configuration and tools for managing the ÉTRID blockchain infrastructure through a unified interface.

## Architecture

```
pinokio/
├── index.js                    # Main Pinokio configuration (Web UIs + Validators)
├── validator-cli.js            # Remote validator CLI manager
├── ai-validator-monitor.js     # AI-powered monitoring and analysis
├── package.json                # Node.js package configuration
└── README.md                   # This file

Connected to:
├── apps/                       # 5 Web UI applications
├── scripts/                    # Management scripts
├── infrastructure/config/      # Validator configuration (21 VMs)
└── reports/                    # AI monitoring reports (auto-generated)
```

## Features

### 1. Web UI Suite (5 Applications)

All web UIs are accessible through Pinokio's interface:

| Application | Port | Description |
|------------|------|-------------|
| Lightning Landing | 3000 | Main landing page with network stats |
| MasterChef Dashboard | 3001 | LP rewards and staking |
| Validator Dashboard | 3002 | Real-time validator monitoring |
| Watchtower Monitor | 3003 | Lightning-Bloc channel monitoring |
| Wallet Web | 3004 | Full crypto wallet with multi-chain support |

### 2. Validator Management (21 Distributed VMs)

Manage validators across multiple cloud providers:
- **16 Accessible Validators** via SSH
- **Azure**: 15 validators across global regions
- **Oracle Cloud**: 1 validator
- **Roles**: Directors, Developers, Nodes

### 3. AI-Powered Monitoring

Intelligent monitoring with:
- Real-time health scoring (0-100)
- Automatic alert detection
- Smart recommendations
- Network-wide analysis
- JSON report generation

## Quick Start

### Using Pinokio UI

1. **Install**:
   - Open Pinokio
   - Navigate to ÉTRID Complete Suite
   - Click "Install"
   - Wait for all components to build

2. **Run**:
   - Click "Run" to start everything
   - Web UIs start on ports 3000-3004
   - AI monitor checks validators every 10 minutes

3. **Access**:
   - Use the menu to access web UIs
   - Check monitoring reports in `reports/` directory

### Using CLI

```bash
cd pinokio

# Validator Management
node validator-cli.js list                    # List all 21 validators
node validator-cli.js status                  # Check all accessible validators
node validator-cli.js status 7                # Check specific validator
node validator-cli.js exec 7 "uptime"         # Execute command on validator #7
node validator-cli.js exec-all "df -h"        # Execute on all accessible validators
node validator-cli.js logs 7 100              # View last 100 log lines
node validator-cli.js restart 7               # Restart validator service

# AI Monitoring
node ai-validator-monitor.js monitor          # Run single check
node ai-validator-monitor.js continuous 5     # Continuous (every 5 min)

# Web UIs (via npm scripts)
npm run web:build                             # Build all web UIs
npm run web:start                             # Start all web UIs
npm run web:status                            # Check web UI status
npm run web:stop                              # Stop all web UIs

# Quick monitoring
npm run validator:list                        # List validators
npm run validator:status                      # Check all statuses
npm run validator:monitor                     # Run AI monitoring
npm run validator:watch                       # Continuous monitoring
```

## Validator CLI Examples

### List All Validators
```bash
$ node validator-cli.js list

📋 ÉTRID Validators:

✅ # 6: Runtime Dev         - 20.224.104.239   (Azure West Europe)
✅ # 7: Compiler Dev        - 98.71.91.84      (Azure North Europe)
✅ # 8: Network Dev         - 20.169.114.25    (Azure France Central)
...

✅ Accessible: 16/21
```

### Check Specific Validator
```bash
$ node validator-cli.js status 7

================================================================================
Validator #7: Compiler Dev
================================================================================
IP:          98.71.91.84
Region:      Azure North Europe
Role:        Developer/MonitoringServer

Service:     🟢 RUNNING
Uptime:      Mon 2025-11-08 03:15:42 UTC

Peers:       Idle (8 peers)
Block:       #12456

Disk:        45% used
Memory:      2.1Gi / 8.0Gi (26%)
================================================================================
```

### Execute Commands Remotely
```bash
# Check disk space on validator #7
$ node validator-cli.js exec 7 "df -h /var/lib/etrid"

# Check on all validators
$ node validator-cli.js exec-all "systemctl is-active etrid-validator"

# View real-time logs
$ node validator-cli.js logs 7 50
```

## AI Monitoring Report

### Sample Output

```bash
$ node ai-validator-monitor.js monitor

🔍 Gathering validator data...

====================================================================================================
🤖 AI-POWERED VALIDATOR MONITORING REPORT
====================================================================================================

📊 NETWORK SUMMARY
   Overall Health:      🟢 HEALTHY (Score: 87.3/100)
   Total Validators:    16
   Running Validators:  15/16
   Average Peer Count:  6.8
   Timestamp:           2025-11-08 05:30:15

🚨 CRITICAL ALERTS (1)
   🔴 1 validator(s) are not running

💡 NETWORK-WIDE RECOMMENDATIONS
   1. Restart stopped validators: #14

📋 VALIDATOR DETAILS

🟢 Validator #6: Runtime Dev (Health: 95/100)
   Status: 🟢 Running | Peers: 8 | Block: #12450
   Disk: 42% | Memory: 24.5%

🟡 Validator #7: Compiler Dev (Health: 75/100)
   Status: 🟢 Running | Peers: 3 | Block: #12450
   Disk: 45% | Memory: 26.3%
   Alerts:
      ⚠️  LOW PEER COUNT: 3 peers (minimum recommended: 3)
   Recommendations:
      💡 Check network connectivity and firewall rules for port 30333

🔴 Validator #14: Perf Dev (Health: 50/100)
   Status: 🔴 Stopped | Peers: Unknown | Block: Unknown
   Disk: 38% | Memory: 15.2%
   Alerts:
      🔴 CRITICAL: Validator service is not running
   Recommendations:
      💡 Restart the validator service immediately: systemctl restart etrid-validator

...

====================================================================================================

📄 Report saved to: ./reports/validator-report-2025-11-08T05-30-15-000Z.json
```

## SSH Configuration

### Prerequisites

1. **SSH Key**: Ensure you have SSH access to validators
   ```bash
   export SSH_KEY_PATH=~/.ssh/your-validator-key
   ```

2. **Network Access**: VMs must be accessible from your location
   - Check `infrastructure/config/validator-ips.json`
   - Accessible validators have `"accessible": true`

3. **SSH User**: Each validator has a specific SSH user
   - Example: `runtime-dev01@20.224.104.239`

### Security

- All SSH connections use `-o StrictHostKeyChecking=no` for automation
- Never commit SSH keys to the repository
- Use environment variables for sensitive paths
- Rotate SSH keys regularly

## Monitoring Reports

AI monitoring generates JSON reports in `reports/`:

```
reports/
├── validator-report-2025-11-08T05-30-15-000Z.json
├── validator-report-2025-11-08T05-40-15-000Z.json
└── validator-report-2025-11-08T05-50-15-000Z.json
```

### Report Structure

```json
{
  "timestamp": "2025-11-08T05:30:15.000Z",
  "summary": {
    "overall_health": "🟢 HEALTHY",
    "total_validators": 16,
    "running_validators": 15,
    "average_peers": "6.8",
    "average_health_score": "87.3"
  },
  "critical_alerts": [...],
  "network_recommendations": [...],
  "validator_details": [...]
}
```

## Integration with Pinokio

### Menu Structure

```
ÉTRID Complete Suite
├── 🌐 Web UIs
│   ├── Lightning Landing (http://localhost:3000)
│   ├── MasterChef Dashboard (http://localhost:3001)
│   ├── Validator Dashboard (http://localhost:3002)
│   ├── Watchtower Monitor (http://localhost:3003)
│   └── Wallet Web (http://localhost:3004)
├── 🔧 Validator Tools
│   ├── List Validators
│   ├── Monitor All Validators
│   └── Continuous Monitoring
├── 📊 Reports (file://./reports)
└── 📖 Documentation
```

### Lifecycle

1. **Install**: Builds all 5 web UIs + sets up validator tools
2. **Run**: Starts web UIs + continuous AI monitoring (10 min intervals)
3. **Stop**: Gracefully stops all services

## Troubleshooting

### SSH Connection Issues

```bash
# Test SSH connection manually
ssh -i ~/.ssh/your-key runtime-dev01@20.224.104.239

# Check validator configuration
cat ../infrastructure/config/validator-ips.json | jq '.validators[] | select(.accessible==true)'
```

### Validator Not Responding

```bash
# Check if validator is accessible
node validator-cli.js status <number>

# Try restarting
node validator-cli.js restart <number>

# View recent logs
node validator-cli.js logs <number> 100
```

### Web UI Build Failures

```bash
# Rebuild specific app
cd ../apps/lightning-landing
npm install
npm run build

# Or use the build script
cd ../scripts
./build-all-web-uis.sh
```

## Advanced Usage

### Custom Monitoring Thresholds

Edit `ai-validator-monitor.js`:

```javascript
this.alertThresholds = {
  minPeers: 3,          // Minimum peer count
  maxBlockLag: 10,      // Maximum block difference
  minDiskSpace: 10,     // Minimum disk space (GB)
  maxMemoryUsage: 90,   // Maximum memory usage (%)
};
```

### Adding New Validators

1. Update `infrastructure/config/validator-ips.json`
2. Add SSH access credentials
3. Set `"accessible": true`
4. Run `node validator-cli.js list` to verify

### Custom Commands

```bash
# Create a wrapper script
#!/bin/bash
# my-validator-task.sh

export SSH_KEY_PATH=~/.ssh/my-key

# Check all validators
node pinokio/validator-cli.js status

# Restart any stopped validators
# ... custom logic ...
```

## Requirements

- **Node.js**: >= 18.0.0
- **SSH Access**: Private key for validators
- **Ports**: 3000-3004 available
- **Disk Space**: ~2GB for all builds

## Support

- **Documentation**: See main `PINOKIO_README.md`
- **Web UI Docs**: `scripts/WEB_UI_SCRIPTS.md`
- **Issues**: GitHub repository issues

## License

MIT License - See repository root for details

---

**Made with ❤️ by the ÉTRID Development Team**

For more information, visit the main project documentation.
