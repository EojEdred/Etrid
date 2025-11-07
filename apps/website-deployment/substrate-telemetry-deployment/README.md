# 🚀 ËTRID Substrate Telemetry Deployment Package

Complete deployment package to integrate all 21 ËTRID validators with Substrate Telemetry Server and display them on the website.

---

## 📦 Package Contents

| File | Description |
|------|-------------|
| `DEPLOYMENT_GUIDE.md` | **START HERE** - Complete step-by-step deployment guide |
| `docker-compose.yml` | Substrate Telemetry server containers |
| `nginx-telemetry.conf` | Nginx reverse proxy configuration |
| `deploy-telemetry.sh` | Automated deployment script for server |
| `configure-validators.sh` | Script to update all 21 validators |
| `telemetry-feed-integration.js` | Updated website telemetry app |

---

## 🎯 Quick Start

### 1. Read the Deployment Guide
```bash
cat DEPLOYMENT_GUIDE.md
```

### 2. Deploy Telemetry Server
```bash
# Transfer files to server
scp -i ~/.ssh/gizzi-validator *.yml *.conf *.sh ubuntu@98.71.91.84:~/

# SSH and run deployment
ssh -i ~/.ssh/gizzi-validator ubuntu@98.71.91.84
./deploy-telemetry.sh
```

### 3. Create DNS Record
```
telemetry.etrid.org → 98.71.91.84
```

### 4. Get SSL Certificate
```bash
sudo certbot certonly --standalone -d telemetry.etrid.org
sudo systemctl reload nginx
```

### 5. Configure Validators
Edit `configure-validators.sh` with all 21 IPs, then run:
```bash
./configure-validators.sh
```

### 6. Update Website
```bash
cp telemetry-feed-integration.js /path/to/website/apps/telemetry/app.js
# Deploy via FTP
```

---

## ✅ What This Achieves

**Current Setup (Before):**
- ❌ Website connects to only 1 validator
- ❌ Shows data from single node
- ❌ Can't see all 21 validators

**After Deployment:**
- ✅ Dedicated Substrate Telemetry Server
- ✅ All 21 validators report real-time data
- ✅ Website shows ALL validators with individual metrics
- ✅ Professional monitoring UI
- ✅ Geographic distribution map
- ✅ Network-wide statistics

---

## 🔗 Result URLs

After deployment:
- **Telemetry UI:** https://telemetry.etrid.org
- **Website Telemetry:** https://etrid.org/telemetry/
- **Explorer:** https://etrid.org/explorer/
- **Network Monitor (Grafana):** https://etrid.org/network/

---

## 📊 Architecture

```
Telemetry Server (telemetry.etrid.org)
    ↑
    │ All 21 validators report via WSS
    │
┌───┴────┬─────────┬─────────┬─────────┐
│ Val-01 │ Val-02  │ Val-03  │ ... 21  │
│ Validator IPs across network          │
└────────┴─────────┴─────────┴─────────┘
```

Website connects to telemetry feed → Shows ALL validators

---

## 📝 Prerequisites

- Ubuntu server (98.71.91.84 recommended)
- Docker & Docker Compose
- Nginx
- Certbot
- SSH access to all validators
- DNS access to create `telemetry.etrid.org`

---

## 💡 Support

**Read first:**
- `DEPLOYMENT_GUIDE.md` - Complete instructions
- Troubleshooting section in deployment guide

**Check logs:**
```bash
# Telemetry server
docker-compose -f /opt/substrate-telemetry/docker-compose.yml logs -f

# Validators
sudo journalctl -u flarechain -f | grep telemetry
```

---

**Created:** November 1, 2025  
**For:** ËTRID Blockchain Network
