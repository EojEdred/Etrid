# ËTRID Pinokio Integration Guide

Complete guide for the Pinokio-powered validator management and web UI suite integrated into etrid.org.

## 🌟 Overview

The Pinokio integration provides a unified platform for managing the ËTRID ecosystem:

- **5 Web Applications**: Accessible through etrid.org navigation
- **21 Distributed Validators**: Global infrastructure monitoring
- **AI-Powered Analytics**: Intelligent health analysis and recommendations
- **Unified API**: Real-time data access for all applications

## 🏗️ Architecture

```
ËTRID Ecosystem
├── Website (etrid.org)
│   ├── Main Landing Page (index.html)
│   ├── Lightning Landing (/lightning/)
│   └── Validator Monitor (/validator-monitor/)
│
├── Web Applications (Localhost)
│   ├── MasterChef Dashboard (port 3001)
│   ├── Validator Dashboard (port 3002)
│   ├── Watchtower Monitor (port 3003)
│   └── Wallet Web (port 3004)
│
├── Backend Services
│   ├── Validator Status API (port 3100)
│   └── AI Monitoring Service (background)
│
└── Management Tools (CLI)
    ├── Validator CLI (validator-cli.js)
    └── AI Monitor (ai-validator-monitor.js)
```

## 📦 Installation

### Prerequisites

- Node.js 18+ and npm
- SSH access to validators (optional for monitoring)
- Git

### Quick Install

```bash
# Clone the repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid

# Run the deployment script
chmod +x deployment/website/website-deployment/deploy-pinokio-suite.sh
./deployment/website/website-deployment/deploy-pinokio-suite.sh
```

### Manual Install

```bash
# 1. Install Pinokio dependencies
cd pinokio
npm install

# 2. Install API dependencies
cd ../deployment/website/website-deployment/api
npm install

# 3. Build web UIs
cd ../../..
./scripts/build-all-web-uis.sh

# 4. Configure SSH (optional)
export SSH_KEY_PATH=~/.ssh/your-validator-key
```

## 🚀 Usage

### Starting Services

#### Start All Web UIs

```bash
./scripts/start-all-web-uis.sh
```

This starts:
- Lightning Landing (port 3000)
- MasterChef Dashboard (port 3001)
- Validator Dashboard (port 3002)
- Watchtower Monitor (port 3003)
- Wallet Web (port 3004)

#### Start the API Server

```bash
cd deployment/website/website-deployment/api
npm start
```

API will be available at: `http://localhost:3100`

#### Start Continuous Monitoring

```bash
cd pinokio
npm run validator:watch
```

Runs AI monitoring every 10 minutes.

### Validator Management

#### List All Validators

```bash
cd pinokio
npm run validator:list
# or
node validator-cli.js list
```

**Output:**
```
===============================================================================
📋 ËTRID VALIDATOR NETWORK
===============================================================================

Total Validators: 21
✅ Accessible: 16
❌ Inaccessible: 5

ID  | Name                 | Region              | Role           | Status
-------------------------------------------------------------------------------
1   | Gizzi                | Azure West US       | Director       | ❌
2   | EojEdred             | Azure East US       | Director       | ❌
7   | Compiler Dev         | Azure North Europe  | Developer      | ✅
...
```

#### Check Specific Validator

```bash
node validator-cli.js status 7
```

#### Check All Validator Status

```bash
npm run validator:status
# or
node validator-cli.js status-all
```

#### Execute Remote Commands

```bash
# On specific validator
node validator-cli.js exec 7 "uptime"

# On all validators
node validator-cli.js exec-all "df -h"
```

#### View Logs

```bash
node validator-cli.js logs 7 100
```

#### Restart Validator

```bash
node validator-cli.js restart 7
```

### AI Monitoring

#### Run Once

```bash
npm run validator:monitor
# or
node ai-validator-monitor.js monitor
```

**Output:**
```
================================================================================
🤖 AI-POWERED VALIDATOR MONITORING
================================================================================

📊 Checking validators...
📊 Checking #7 Compiler Dev... 🟢 Health: 95/100
📊 Checking #8 Network Node... 🟡 Health: 72/100

================================================================================
📊 NETWORK SUMMARY
================================================================================
   Overall Health:      🟢 HEALTHY (Score: 87.3/100)
   Total Validators:    16
   Running Validators:  15/16
   Average Peer Count:  6.8

💡 AI RECOMMENDATIONS
   1. 🔴 [Service] Restart stopped validators: #14
   2. 🟡 [Network] Check connectivity for low peer count: #9

💾 Report saved to: pinokio/reports/validator-report-1234567890.json
```

#### Continuous Monitoring

```bash
npm run validator:watch
# or
node ai-validator-monitor.js continuous 10
```

Runs monitoring every 10 minutes (configurable).

## 🌐 Website Integration

### Navigation Menu

The etrid.org navigation includes a dropdown menu with all applications:

**Desktop:**
- Apps dropdown with 5 applications
- Direct links with icons

**Mobile:**
- Organized sections (Applications, Resources)
- Touch-friendly interface

### Validator Monitor Page

Access at: `https://etrid.org/validator-monitor/`

**Features:**
- Real-time validator health visualization
- Network-wide statistics
- Regional distribution charts
- AI recommendations panel
- Filter validators by health status
- Responsive design

**Data Sources:**
- Connects to API at `/api/validator-status`
- Auto-refreshes every 30 seconds
- Displays latest monitoring report

## 🔌 API Endpoints

Base URL: `http://localhost:3100`

### GET /api/validator-status

Returns current status of all validators.

**Response:**
```json
{
  "timestamp": "2025-11-08T12:34:56.789Z",
  "summary": {
    "totalValidators": 16,
    "runningValidators": 15,
    "averageHealth": 87.3,
    "averagePeerCount": 6.8,
    "criticalAlerts": 1,
    "warningAlerts": 2
  },
  "validators": [...]
}
```

### GET /api/validator-status/:id

Returns status of a specific validator.

**Example:** `/api/validator-status/7`

### GET /api/network-summary

Returns network-wide summary statistics.

### GET /api/recommendations

Returns AI-generated recommendations.

### GET /api/regions

Returns validator distribution by region.

### GET /api/health

API health check endpoint.

## 📊 Reports

### Report Location

All monitoring reports are saved to: `pinokio/reports/`

### Report Format

```json
{
  "timestamp": "2025-11-08T12:34:56.789Z",
  "summary": {
    "totalValidators": 16,
    "runningValidators": 15,
    "averageHealth": 87.3,
    "averagePeerCount": 6.8,
    "criticalAlerts": 1,
    "warningAlerts": 2
  },
  "recommendations": [
    {
      "priority": "high",
      "category": "Service",
      "message": "Restart stopped validators: #14"
    }
  ],
  "validators": [
    {
      "validator": {
        "id": 7,
        "name": "Compiler Dev",
        "region": "Azure North Europe",
        "role": "Developer/Monitor"
      },
      "metrics": {
        "isRunning": true,
        "peerCount": 8,
        "blockHeight": 12450,
        "diskUsage": 45,
        "memoryUsage": 26
      },
      "health": 95,
      "alerts": [],
      "timestamp": "2025-11-08T12:34:56.789Z"
    }
  ]
}
```

## 🔧 Configuration

### SSH Configuration

Set the SSH key path:

```bash
export SSH_KEY_PATH=~/.ssh/your-validator-key
```

Or create a `.env` file:

```bash
echo "SSH_KEY_PATH=~/.ssh/your-validator-key" > pinokio/.env
```

### Validator Configuration

Edit: `infrastructure/config/validator-ips.json`

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

### API Configuration

Edit: `deployment/website/website-deployment/api/validator-status.js`

```javascript
const PORT = process.env.VALIDATOR_API_PORT || 3100;
```

## 🛠️ Management Scripts

### Web UI Scripts

Located in `scripts/`:

```bash
# Build all web UIs
./scripts/build-all-web-uis.sh

# Start all web UIs
./scripts/start-all-web-uis.sh

# Check status
./scripts/status-web-uis.sh

# Stop all
./scripts/stop-all-web-uis.sh
```

### Deployment Script

```bash
# Full deployment
./deployment/website/website-deployment/deploy-pinokio-suite.sh
```

This script:
1. Installs all dependencies
2. Builds web UIs
3. Creates directories
4. Checks configurations
5. Generates initial report
6. Displays deployment summary

## 🐛 Troubleshooting

### SSH Connection Issues

```bash
# Test SSH connection
ssh -i ~/.ssh/your-key username@validator-ip

# Check SSH key permissions
chmod 600 ~/.ssh/your-key

# Verify SSH_KEY_PATH
echo $SSH_KEY_PATH
```

### Web UI Port Conflicts

```bash
# Check port usage
lsof -i :3000-3004

# Stop all web UIs
./scripts/stop-all-web-uis.sh

# Kill specific process
kill $(lsof -t -i:3001)
```

### API Not Starting

```bash
# Check if port is in use
lsof -i :3100

# Install dependencies
cd deployment/website/website-deployment/api
npm install

# Start with logs
npm start
```

### Monitoring Timeouts

Increase SSH timeout in `pinokio/validator-cli.js` and `ai-validator-monitor.js`:

```javascript
const SSH_TIMEOUT = 60000; // 60 seconds
```

### No Reports Generated

```bash
# Check reports directory
ls -lh pinokio/reports/

# Run monitoring manually
cd pinokio
node ai-validator-monitor.js monitor

# Check SSH configuration
echo $SSH_KEY_PATH
```

## 📈 Performance Tips

1. **Monitoring Frequency**: Adjust continuous monitoring interval based on needs
   ```bash
   node ai-validator-monitor.js continuous 5  # Every 5 minutes
   ```

2. **API Caching**: The API reads from the latest report file (no database needed)

3. **Report Cleanup**: Old reports can be deleted to save space
   ```bash
   find pinokio/reports -name "validator-report-*.json" -mtime +7 -delete
   ```

4. **Parallel Execution**: The validator CLI executes commands in parallel for better performance

## 🔐 Security Best Practices

1. **SSH Keys**: Never commit SSH keys to the repository
2. **Environment Variables**: Use `.env` files for sensitive configuration
3. **API Access**: Consider adding authentication for production deployments
4. **Firewall**: Restrict API access to trusted IPs
5. **HTTPS**: Use SSL/TLS for production deployments

## 🚀 Production Deployment

### Nginx Configuration

```nginx
# Validator Monitor
location /validator-monitor/ {
    alias /path/to/etrid/deployment/website/website-deployment/website/validator-monitor/;
    try_files $uri $uri/ /validator-monitor/index.html;
}

# API Proxy
location /api/ {
    proxy_pass http://localhost:3100/api/;
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection 'upgrade';
    proxy_set_header Host $host;
    proxy_cache_bypass $http_upgrade;
}
```

### Systemd Service

Create `/etc/systemd/system/etrid-validator-api.service`:

```ini
[Unit]
Description=ËTRID Validator Status API
After=network.target

[Service]
Type=simple
User=etrid
WorkingDirectory=/path/to/etrid/deployment/website/website-deployment/api
ExecStart=/usr/bin/node validator-status.js
Restart=on-failure
Environment=NODE_ENV=production

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable etrid-validator-api
sudo systemctl start etrid-validator-api
```

## 📚 Additional Resources

- **Pinokio README**: `pinokio/README.md`
- **Architecture Docs**: `docs/architecture.md`
- **Deployment Guide**: `docs/DEPLOYMENT_GUIDE.md`
- **Web UI Scripts**: `scripts/WEB_UI_SCRIPTS.md`
- **Validator IPs**: `infrastructure/config/validator-ips.json`

## 🤝 Contributing

For issues or improvements:
1. Create an issue in the repository
2. Submit a pull request with changes
3. Follow the coding standards

## 📄 License

MIT License - See LICENSE file for details

---

**ËTRID Foundation** - The Future of Multichain Infrastructure
