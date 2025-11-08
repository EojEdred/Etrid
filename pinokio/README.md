# ËTRID Pinokio Integration

Complete ËTRID Web UI Suite and Distributed Validator Management with Pinokio.

## 🌟 Overview

This Pinokio integration provides a unified interface to manage:

- **5 Web UIs**: Lightning Landing, MasterChef, Validator Dashboard, Watchtower, and Wallet
- **21 Distributed Validators**: Across Azure and Oracle Cloud infrastructure
- **AI-Powered Monitoring**: Intelligent health analysis with automatic recommendations

## 📦 Components

### Web Applications

| Application | Port | Description |
|------------|------|-------------|
| **Lightning Landing** | 3000 | Main landing page with network statistics |
| **MasterChef Dashboard** | 3001 | LP rewards tracking with Ethers.js |
| **Validator Dashboard** | 3002 | Real-time validator monitoring |
| **Watchtower Monitor** | 3003 | Lightning-Bloc channel monitoring |
| **Wallet Web** | 3004 | Multi-chain crypto wallet |

### Validator Management Tools

#### `validator-cli.js` - Remote CLI Manager

Provides command-line access to all 21 validators:

```bash
# List all validators
node validator-cli.js list

# Check specific validator status
node validator-cli.js status 7

# Execute command on a validator
node validator-cli.js exec 7 "uptime"

# Execute on all accessible validators
node validator-cli.js exec-all "systemctl status etrid-validator"

# View logs
node validator-cli.js logs 7 100

# Restart validator service
node validator-cli.js restart 7

# Get status of all validators
node validator-cli.js status-all
```

#### `ai-validator-monitor.js` - AI Monitoring

Intelligent monitoring with health scoring and recommendations:

```bash
# Run monitoring once
node ai-validator-monitor.js monitor

# Continuous monitoring (every 5 minutes)
node ai-validator-monitor.js continuous 5
```

**AI Features:**
- Real-time health scoring (0-100 per validator)
- Automatic alert detection:
  - Low peer count
  - High disk usage
  - High memory usage
  - Block synchronization issues
  - Service status
- Smart recommendations for issues
- Network-wide analysis
- JSON report generation

## 🚀 Quick Start

### Using NPM Scripts

```bash
cd pinokio

# Install dependencies
npm install

# Validator Management
npm run validator:list        # List all validators
npm run validator:status      # Check all statuses
npm run validator:monitor     # Run AI monitoring once
npm run validator:watch       # Continuous AI monitoring (10 min)

# Web UI Management
npm run web:build            # Build all web UIs
npm run web:start            # Start all web UIs
npm run web:status           # Check status
npm run web:stop             # Stop all
```

### Direct CLI Access

```bash
# Validator operations
node pinokio/validator-cli.js list
node pinokio/validator-cli.js status 7
node pinokio/ai-validator-monitor.js monitor

# Web UI operations
./scripts/build-all-web-uis.sh
./scripts/start-all-web-uis.sh
./scripts/status-web-uis.sh
./scripts/stop-all-web-uis.sh
```

### Using Pinokio Platform

1. Open Pinokio
2. Install ËTRID Complete Suite
3. Click "Run" to start all services
4. Use the menu to:
   - Access web UIs (ports 3000-3004)
   - Manage validators
   - View AI monitoring reports

## 🔧 Configuration

### SSH Access

To use validator management tools, configure SSH access:

```bash
# Set your SSH key path
export SSH_KEY_PATH=~/.ssh/your-validator-key

# Or create a .env file
echo "SSH_KEY_PATH=~/.ssh/your-validator-key" > .env
```

### Validator Configuration

Validators are configured in: `infrastructure/config/validator-ips.json`

**Structure:**
```json
{
  "validators": [
    {
      "id": 7,
      "name": "Compiler Dev",
      "region": "Azure North Europe",
      "role": "Developer/Monitor",
      "ip": "20.224.104.239",
      "sshUser": "compiler-dev01",
      "accessible": true
    }
  ]
}
```

## 📊 Infrastructure Overview

### 21 Distributed Validators

| ID | Name | Region | Provider | Role | Accessible |
|----|------|--------|----------|------|------------|
| 1 | Gizzi | Azure West US | Azure | Director/Bootstrap | ❌ |
| 2 | EojEdred | Azure East US | Azure | Director/Bootstrap | ❌ |
| 6 | Runtime Dev | Azure West Europe | Azure | Developer | ✅ |
| 7 | Compiler Dev | Azure North Europe | Azure | Developer/Monitor | ✅ |
| 8-21 | Various | Global | Azure/Oracle | Developers/Nodes | ✅ (most) |

**Coverage:**
- 16 Accessible Validators (SSH enabled)
- Regions: Europe, Asia, Middle East, Africa, US
- Providers: Azure (15), Oracle Cloud (1)

## 🤖 AI Monitoring Details

### Health Scoring Algorithm

Base score: **100 points**

**Deductions:**
- Service not running: **-50 points**
- Low peer count (< 4): **-20 points**
- High disk usage (> 80%): **-15 points**
- High memory usage (> 85%): **-10 points**

### Metrics Collected

- **Service Status**: Running/stopped state
- **Peer Count**: Network connectivity
- **Block Height**: Synchronization status
- **Disk Usage**: Storage capacity
- **Memory Usage**: RAM consumption
- **Uptime**: Service availability

### Report Format

Reports are saved to: `pinokio/reports/validator-report-TIMESTAMP.json`

```json
{
  "timestamp": "2025-11-08T...",
  "summary": {
    "totalValidators": 16,
    "runningValidators": 15,
    "averageHealth": 87.3,
    "averagePeerCount": 6.8,
    "criticalAlerts": 1,
    "warningAlerts": 2
  },
  "recommendations": [...],
  "validators": [...]
}
```

## 📖 Example Output

### AI Monitoring Report

```
================================================================================
🤖 AI-POWERED VALIDATOR MONITORING REPORT
================================================================================

📊 NETWORK SUMMARY
   Overall Health:      🟢 HEALTHY (Score: 87.3/100)
   Total Validators:    16
   Running Validators:  15/16
   Average Peer Count:  6.8

🚨 CRITICAL ALERTS (1)
   🔴 1 validator(s) are not running

💡 AI RECOMMENDATIONS
   1. 🔴 [Service] Restart stopped validators: #14
   2. 🟡 [Network] Check connectivity for low peer count: #9, #12

📋 VALIDATOR DETAILS

🟢 Validator #7: Compiler Dev (Health: 95/100)
   Status: 🟢 Running | Peers: 8 | Block: #12450
   Disk: 45% | Memory: 26.3%

🟡 Validator #9: Network Node (Health: 72/100)
   Status: 🟢 Running | Peers: 2 | Block: #12448
   Disk: 62% | Memory: 31.2%
   🟡 Low peer count: 2
```

## 🛠️ Management Scripts

Located in `scripts/`:

### `build-all-web-uis.sh`
Installs dependencies and builds production bundles for all 5 web UIs.

### `start-all-web-uis.sh`
Starts all web UIs as background processes:
- Creates PID files in `/tmp/etrid-*.pid`
- Logs to `/tmp/etrid-*.log`

### `stop-all-web-uis.sh`
Gracefully stops all running web UIs and cleans up PID/log files.

### `status-web-uis.sh`
Shows running/stopped status and displays URLs for active applications.

## 🔐 Security Notes

- SSH keys should have proper permissions (`chmod 600`)
- Never commit SSH keys to the repository
- Use environment variables for sensitive configuration
- Validator IPs should be kept secure

## 🐛 Troubleshooting

### SSH Connection Issues

```bash
# Test SSH connection manually
ssh -i ~/.ssh/your-key username@validator-ip

# Check SSH key permissions
chmod 600 ~/.ssh/your-key

# Verify SSH_KEY_PATH is set
echo $SSH_KEY_PATH
```

### Web UI Port Conflicts

```bash
# Check if ports are in use
lsof -i :3000-3004

# Stop conflicting services
./scripts/stop-all-web-uis.sh
```

### Monitoring Timeouts

Increase SSH timeout in the scripts:
```javascript
const SSH_TIMEOUT = 60000; // 60 seconds
```

## 📚 Additional Resources

- **Validator IPs**: `infrastructure/config/validator-ips.json`
- **Web UI Scripts**: `scripts/WEB_UI_SCRIPTS.md`
- **Architecture**: `docs/architecture.md`
- **Deployment**: `docs/DEPLOYMENT_GUIDE.md`

## 🤝 Contributing

For issues or improvements:
1. Create an issue in the repository
2. Submit a pull request with changes
3. Follow the coding standards

## 📄 License

MIT License - See LICENSE file for details

---

**ËTRID Foundation** - The Future of Multichain Infrastructure
