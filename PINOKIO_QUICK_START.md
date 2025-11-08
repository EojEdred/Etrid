# ËTRID Pinokio - Quick Start Guide

## 🚀 One-Command Deployment

```bash
cd /Users/macbook/Desktop/etrid
./deployment/website/website-deployment/deploy-pinokio-suite.sh
```

## 📋 Essential Commands

### Start Everything

```bash
# 1. Start all web UIs
./scripts/start-all-web-uis.sh

# 2. Start API server (in new terminal)
cd deployment/website/website-deployment/api && npm start

# 3. Start continuous monitoring (in new terminal)
cd pinokio && npm run validator:watch
```

### Stop Everything

```bash
./scripts/stop-all-web-uis.sh
# API: Ctrl+C in API terminal
# Monitoring: Ctrl+C in monitoring terminal
```

## 🌐 Access Points

### Website
- **Main Site**: http://localhost:8080 or https://etrid.org
- **Validator Monitor**: https://etrid.org/validator-monitor/

### Web UIs
- **Lightning Landing**: http://localhost:3000
- **MasterChef Dashboard**: http://localhost:3001
- **Validator Dashboard**: http://localhost:3002
- **Watchtower Monitor**: http://localhost:3003
- **Wallet Web**: http://localhost:3004

### API
- **Base URL**: http://localhost:3100
- **Validator Status**: http://localhost:3100/api/validator-status
- **Network Summary**: http://localhost:3100/api/network-summary

## 🔧 Common Tasks

### List All Validators
```bash
cd pinokio
npm run validator:list
```

### Check Validator Status
```bash
cd pinokio
npm run validator:status
```

### Run AI Monitoring
```bash
cd pinokio
npm run validator:monitor
```

### Execute Command on Validator
```bash
cd pinokio
node validator-cli.js exec 7 "uptime"
```

### View Validator Logs
```bash
cd pinokio
node validator-cli.js logs 7 100
```

### Check Web UI Status
```bash
./scripts/status-web-uis.sh
```

## ⚙️ Configuration

### Set SSH Key (Required for Validator Management)
```bash
export SSH_KEY_PATH=~/.ssh/your-validator-key
chmod 600 ~/.ssh/your-validator-key
```

### Test SSH Connection
```bash
ssh -i ~/.ssh/your-key username@validator-ip
```

## 📊 View Reports

```bash
# List all reports
ls -lh pinokio/reports/

# View latest report
cat pinokio/reports/validator-report-*.json | tail -1 | jq .
```

## 🐛 Troubleshooting

### Ports Already in Use
```bash
lsof -i :3000-3004
./scripts/stop-all-web-uis.sh
```

### SSH Connection Failed
```bash
echo $SSH_KEY_PATH
chmod 600 ~/.ssh/your-validator-key
```

### API Not Starting
```bash
cd deployment/website/website-deployment/api
npm install
npm start
```

## 📚 Documentation

- **Full Guide**: docs/PINOKIO_INTEGRATION.md
- **Pinokio README**: pinokio/README.md
- **Completion Report**: PINOKIO_INTEGRATION_COMPLETE.md

## 🎯 Quick Links

| Application | URL | Description |
|------------|-----|-------------|
| Lightning Landing | http://localhost:3000 | Main landing page |
| MasterChef | http://localhost:3001 | LP rewards dashboard |
| Validator Monitor | /validator-monitor/ | Real-time monitoring |
| Watchtower | http://localhost:3003 | Channel monitoring |
| Wallet Web | http://localhost:3004 | Multi-chain wallet |
| API | http://localhost:3100 | REST API |

---

**ËTRID Foundation** - Powered by Pinokio
