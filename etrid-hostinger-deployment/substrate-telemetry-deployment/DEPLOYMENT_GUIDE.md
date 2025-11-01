# 🚀 ËTRID Substrate Telemetry Integration Guide

Complete guide to deploy Substrate Telemetry Server and integrate all 21 validators with the ËTRID website.

---

## 📋 Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Prerequisites](#prerequisites)
3. [Phase 1: Deploy Telemetry Server](#phase-1-deploy-telemetry-server)
4. [Phase 2: Configure DNS](#phase-2-configure-dns)
5. [Phase 3: Set Up SSL](#phase-3-set-up-ssl)
6. [Phase 4: Configure Validators](#phase-4-configure-validators)
7. [Phase 5: Update Website](#phase-5-update-website)
8. [Phase 6: Testing](#phase-6-testing)
9. [Troubleshooting](#troubleshooting)

---

## Architecture Overview

```
┌─────────────────────────────────────────┐
│  Telemetry Server (telemetry.etrid.org) │
│  - Backend: Receives validator data     │
│  - Frontend: Web UI                     │
│  - Shard: WebSocket handler             │
└─────────────────────────────────────────┘
            ▲                    │
            │ WSS /submit/       │ WSS /feed/
            │                    ▼
    ┌───────┴─────────┐    ┌──────────────┐
    │  21 Validators  │    │   Website    │
    │  Report data    │    │   etrid.org  │
    └─────────────────┘    └──────────────┘
```

**Data Flow:**
1. Each validator connects to `wss://telemetry.etrid.org/submit/`
2. Validators send real-time metrics every 5 seconds
3. Telemetry server aggregates and stores data
4. Website connects to `wss://telemetry.etrid.org/feed/`
5. Website displays ALL 21 validators with real-time data

---

## Prerequisites

- Ubuntu server (98.71.91.84 recommended - already has monitoring)
- Root or sudo access
- Domain DNS access (to create `telemetry.etrid.org`)
- SSH access to all 21 validators

---

## Phase 1: Deploy Telemetry Server

### Step 1.1: Transfer Files to Server

```bash
# On your local machine
cd /Users/macbook/Desktop/etrid/substrate-telemetry-deployment
scp -i ~/.ssh/gizzi-validator *.yml *.conf *.sh ubuntu@98.71.91.84:~/
```

### Step 1.2: Run Deployment Script

```bash
# SSH to server
ssh -i ~/.ssh/gizzi-validator ubuntu@98.71.91.84

# Run deployment
chmod +x deploy-telemetry.sh
./deploy-telemetry.sh
```

**Expected Output:**
```
✅ Docker installed
✅ Docker Compose installed
✅ Nginx installed
✅ Certbot installed
🚀 Starting Substrate Telemetry containers...
✅ Telemetry Server Deployed!
```

### Step 1.3: Verify Deployment

```bash
# Check containers are running
docker-compose -f /opt/substrate-telemetry/docker-compose.yml ps

# Should show:
# telemetry-backend   Up
# telemetry-frontend  Up
# telemetry-shard     Up

# Check logs
docker-compose -f /opt/substrate-telemetry/docker-compose.yml logs -f
```

---

## Phase 2: Configure DNS

### Step 2.1: Get Server IP

```bash
curl ifconfig.me
# Should return: 98.71.91.84
```

### Step 2.2: Create DNS A Record

**In your DNS provider (Hostinger, Cloudflare, etc.):**

| Type | Name | Value | TTL |
|------|------|-------|-----|
| A | telemetry | 98.71.91.84 | 300 |

Or fully qualified:
| Type | Name | Value | TTL |
|------|------|-------|-----|
| A | telemetry.etrid.org | 98.71.91.84 | 300 |

### Step 2.3: Verify DNS Propagation

```bash
# Wait 5-10 minutes, then test:
nslookup telemetry.etrid.org

# Should return: 98.71.91.84

# Or use:
dig telemetry.etrid.org +short
```

**⚠️ IMPORTANT:** Do NOT proceed until DNS is resolving correctly!

---

## Phase 3: Set Up SSL

### Step 3.1: Get SSL Certificate

```bash
# SSH to server
ssh -i ~/.ssh/gizzi-validator ubuntu@98.71.91.84

# Stop Nginx temporarily
sudo systemctl stop nginx

# Get certificate
sudo certbot certonly --standalone -d telemetry.etrid.org

# Follow prompts:
# - Enter email: your@email.com
# - Agree to terms: Y
# - Share email: N (optional)
```

**Expected Output:**
```
Successfully received certificate.
Certificate is saved at: /etc/letsencrypt/live/telemetry.etrid.org/fullchain.pem
Key is saved at:         /etc/letsencrypt/live/telemetry.etrid.org/privkey.pem
```

### Step 3.2: Configure Nginx

```bash
# Copy Nginx config
sudo cp ~/nginx-telemetry.conf /etc/nginx/sites-available/telemetry

# Remove default site
sudo rm -f /etc/nginx/sites-enabled/default

# Enable telemetry site
sudo ln -s /etc/nginx/sites-available/telemetry /etc/nginx/sites-enabled/

# Test config
sudo nginx -t

# Reload Nginx
sudo systemctl start nginx
sudo systemctl reload nginx
```

### Step 3.3: Verify SSL

```bash
# Test HTTPS
curl -I https://telemetry.etrid.org

# Should return: HTTP/2 200

# Test WebSocket
wscat -c wss://telemetry.etrid.org/feed/
# (Install wscat if needed: npm install -g wscat)
```

---

## Phase 4: Configure Validators

### Step 4.1: Update Validator Systemd Services

For EACH of the 21 validators, update the systemd service to include telemetry reporting.

**Method A: Manual (for each validator):**

```bash
# SSH to validator
ssh -i ~/.ssh/gizzi-validator ubuntu@VALIDATOR_IP

# Stop validator
sudo systemctl stop flarechain

# Edit service file
sudo nano /etc/systemd/system/flarechain.service

# Add telemetry flag to ExecStart line:
--telemetry-url 'wss://telemetry.etrid.org/submit/ 0'

# Full example:
ExecStart=/usr/local/bin/flarechain-node \
  --base-path /var/lib/flarechain \
  --chain flare \
  --validator \
  --name "Validator-01" \
  --telemetry-url 'wss://telemetry.etrid.org/submit/ 0' \
  --prometheus-external

# Save and exit (Ctrl+X, Y, Enter)

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl start flarechain

# Verify
sudo journalctl -u flarechain -f | grep telemetry
```

**Method B: Automated Script:**

```bash
# Edit configure-validators.sh with all validator IPs
nano configure-validators.sh

# Add all 21 validators:
configure_validator "Validator-01" "98.71.91.84"
configure_validator "Validator-02" "IP_ADDRESS"
configure_validator "Validator-03" "129.80.122.34"
# ... etc for all 21

# Run script
./configure-validators.sh
```

### Step 4.2: Verify Validators Appear

```bash
# Check telemetry logs
ssh ubuntu@98.71.91.84
docker-compose -f /opt/substrate-telemetry/docker-compose.yml logs -f shard

# Should see:
# "New node connected: Validator-01"
# "New node connected: Validator-02"
# etc.
```

**Or visit:** https://telemetry.etrid.org

You should see all validators appearing in the web UI!

---

## Phase 5: Update Website

### Step 5.1: Update Telemetry App

Replace the telemetry app.js with the new Substrate Telemetry integration:

```bash
# Copy new integration
cp telemetry-feed-integration.js /Users/macbook/Desktop/etrid/etrid-hostinger-deployment/apps/telemetry/app.js
```

### Step 5.2: Deploy to Website

```bash
cd /Users/macbook/Desktop/etrid/etrid-hostinger-deployment

# Upload via FTP
python3 << 'PYEOF'
import ftplib

ftp = ftplib.FTP()
ftp.connect('157.173.214.206', 21)
ftp.login('u724092535', 'Fullashit13!')

with open('apps/telemetry/app.js', 'rb') as f:
    ftp.storbinary('STOR /domains/etrid.org/public_html/telemetry/app.js', f)

ftp.quit()
print("✅ Telemetry app updated!")
PYEOF
```

### Step 5.3: Test Website Integration

Visit: https://etrid.org/telemetry/

**You should see:**
- ✅ All 21 validators listed
- ✅ Real-time block heights
- ✅ Real peer counts
- ✅ Geographic map with all validator locations
- ✅ Network statistics aggregated from all nodes

---

## Phase 6: Testing

### Test Checklist

**Telemetry Server:**
- [ ] `https://telemetry.etrid.org` loads
- [ ] Web UI shows "FlareChain" network
- [ ] All 21 validators appear in the list
- [ ] Validators show "online" status
- [ ] Block heights update in real-time

**Validators:**
- [ ] All 21 validators running successfully
- [ ] Logs show "Connected to telemetry"
- [ ] No errors in systemd logs
- [ ] Validators producing blocks

**Website:**
- [ ] `https://etrid.org/telemetry/` loads
- [ ] Shows all 21 validators
- [ ] Real-time updates every 5 seconds
- [ ] Map shows validator locations
- [ ] Network stats accurate

---

## Troubleshooting

### Problem: Telemetry server won't start

```bash
# Check container logs
docker-compose -f /opt/substrate-telemetry/docker-compose.yml logs

# Restart containers
docker-compose -f /opt/substrate-telemetry/docker-compose.yml restart

# Check ports
sudo netstat -tlnp | grep -E '8000|8001|3001'
```

### Problem: Validators not appearing

```bash
# Check validator logs
ssh ubuntu@VALIDATOR_IP
sudo journalctl -u flarechain -f | grep telemetry

# Should see:
# "Connected to telemetry: wss://telemetry.etrid.org/submit/"

# If not connecting, check:
# 1. DNS resolves: dig telemetry.etrid.org
# 2. SSL certificate: curl -I https://telemetry.etrid.org
# 3. Firewall allows outbound WSS: sudo ufw status
```

### Problem: Website shows empty/offline

```bash
# Open browser console (F12)
# Check for errors

# Common issues:
# 1. Mixed content: Access via HTTP instead of HTTPS
# 2. WebSocket blocked: Check browser security settings
# 3. CORS issue: Check Nginx CORS headers
```

### Problem: DNS not resolving

```bash
# Clear DNS cache (on your machine)
sudo dscacheutil -flushcache
sudo killall -HUP mDNSResponder

# Check DNS provider
# - Verify A record created correctly
# - Check TTL (should be 300 or lower for testing)
# - Wait up to 1 hour for full propagation
```

### Problem: SSL certificate fails

```bash
# Common causes:
# 1. DNS not resolved yet - wait longer
# 2. Port 80 blocked - check firewall: sudo ufw allow 80
# 3. Nginx already running - stop it first: sudo systemctl stop nginx

# Check certbot logs
sudo tail -f /var/log/letsencrypt/letsencrypt.log
```

---

## 📊 Success Metrics

**Before Substrate Telemetry:**
- ❌ Website shows only 1 validator
- ❌ No real-time updates from all nodes
- ❌ Manual aggregation required
- ❌ Can't see individual validator metrics

**After Substrate Telemetry:**
- ✅ All 21 validators visible
- ✅ Real-time data from every node
- ✅ Automatic aggregation
- ✅ Individual metrics per validator
- ✅ Geographic distribution map
- ✅ Network-wide statistics
- ✅ Professional monitoring UI

---

## 🔗 Quick Reference

### URLs
- **Telemetry UI:** https://telemetry.etrid.org
- **Telemetry Feed (WebSocket):** wss://telemetry.etrid.org/feed/
- **Validator Submit (WebSocket):** wss://telemetry.etrid.org/submit/
- **Website Telemetry:** https://etrid.org/telemetry/

### Important Files
- **Docker Compose:** `/opt/substrate-telemetry/docker-compose.yml`
- **Nginx Config:** `/etc/nginx/sites-available/telemetry`
- **SSL Cert:** `/etc/letsencrypt/live/telemetry.etrid.org/`
- **Validator Service:** `/etc/systemd/system/flarechain.service`

### Commands
```bash
# Restart telemetry
docker-compose -f /opt/substrate-telemetry/docker-compose.yml restart

# View logs
docker-compose -f /opt/substrate-telemetry/docker-compose.yml logs -f

# Restart validator
sudo systemctl restart flarechain

# View validator logs
sudo journalctl -u flarechain -f

# Reload Nginx
sudo systemctl reload nginx
```

---

## 📝 Next Steps After Deployment

1. **Monitor for 24 hours** - Ensure all validators stay connected
2. **Set up alerts** - Monitor for validators going offline
3. **Document validator IPs** - Create inventory of all 21 nodes
4. **Create backup** - Snapshot telemetry server configuration
5. **Update documentation** - Add telemetry URLs to operator guides

---

**Deployment created:** November 1, 2025  
**Last updated:** November 1, 2025
