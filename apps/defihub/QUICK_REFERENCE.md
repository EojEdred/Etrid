# 🏦 ËTRID DeFi Hub - Quick Reference

## 🚀 Access

**URL:** `https://etrid.org/defi-hub/`

**From Main Site:**
1. Go to `etrid.org`
2. Click "Apps" dropdown
3. Select "🏦 DeFi Hub"

## ⚙️ Edit Validators

### Quick Edit
1. Open DeFi Hub
2. Click "⚙️ Configure" button
3. Follow modal instructions

### File Location
```
/Users/macbook/Desktop/etrid/infrastructure/config/validator-ips.json
```

### Edit Command
```bash
nano infrastructure/config/validator-ips.json
```

### Validator Structure
```json
{
  "id": 1,
  "name": "Validator Name",
  "region": "Region",
  "role": "Role",
  "ip": "IP Address",
  "sshUser": "username",
  "accessible": true
}
```

## 🎛️ Operations Center Links

| App | Port | URL |
|-----|------|-----|
| Lightning Landing | 3000 | http://localhost:3000 |
| MasterChef | 3001 | http://localhost:3001 |
| Validator Dashboard | 3002 | http://localhost:3002 |
| Watchtower | 3003 | http://localhost:3003 |
| Wallet Web | 3004 | http://localhost:3004 |
| API Server | 3100 | http://localhost:3100 |
| Telemetry | Live | https://etrid.org/telemetry |
| Docs | Live | https://docs.etrid.org |

## 🔧 Start Services

```bash
# All web UIs
./scripts/start-all-web-uis.sh

# API
cd deployment/website/website-deployment/api && npm start

# Monitoring
cd pinokio && npm run validator:watch
```

## 📊 Validator Commands

```bash
cd pinokio

# List validators
npm run validator:list

# Check status
npm run validator:status

# AI monitoring
npm run validator:monitor

# Execute command
node validator-cli.js exec 7 "uptime"

# View logs
node validator-cli.js logs 7 100
```

## 🛠️ Stop Services

```bash
./scripts/stop-all-web-uis.sh
```

## 📁 Key Files

```
Desktop/etrid/
├── infrastructure/config/validator-ips.json  # Edit here!
├── deployment/website/.../defi-hub/          # DeFi Hub
├── pinokio/                                  # Management tools
└── scripts/                                  # Helper scripts
```

## 🐛 Troubleshooting

```bash
# Check status
./scripts/status-web-uis.sh

# Validate config
cat infrastructure/config/validator-ips.json | jq .

# Check ports
lsof -i :3000-3004

# Restart services
./scripts/stop-all-web-uis.sh
./scripts/start-all-web-uis.sh
```

## 📚 Documentation

- **Complete Guide**: `DEFIHUB_COMPLETE.md`
- **Pinokio Docs**: `pinokio/README.md`
- **Integration**: `docs/PINOKIO_INTEGRATION.md`

---

**Quick Access:** `https://etrid.org/defi-hub/`
