# 🌐 ËTRID Subdomain Setup Guide

## 📋 Overview

Complete subdomain configuration for the ËTRID ecosystem across all services and apps.

**Last Updated:** October 31, 2025
**Main Domain:** etrid.org

---

## 🎯 Subdomain Structure

### Core Infrastructure (Required)

| Subdomain | Purpose | Points To | Status |
|-----------|---------|-----------|--------|
| **etrid.org** | Main website | Hostinger public_html/ | ✅ Primary |
| **www.etrid.org** | WWW redirect | → etrid.org | ✅ Redirect |
| **docs.etrid.org** | Documentation | Hostinger public_html/docs/ or external | 🔴 Setup Required |
| **wallet.etrid.org** | Web3 Wallet | Hostinger public_html/wallet/ | 🔴 Setup Required |
| **explorer.etrid.org** | Blockchain Explorer | Hostinger public_html/explorer/ | 🔴 Setup Required |

### Validator & Network (High Priority)

| Subdomain | Purpose | Points To | Status |
|-----------|---------|-----------|--------|
| **validators.etrid.org** | Validator Leaderboard | Hostinger public_html/validators/ | 🔴 Setup Required |
| **telemetry.etrid.org** | Network Monitoring | Hostinger public_html/network/ or Grafana server | 🔴 Setup Required |
| **network.etrid.org** | Alternative for telemetry | Same as telemetry | 🟡 Optional |

### DeFi & Apps (Standard Priority)

| Subdomain | Purpose | Points To | Status |
|-----------|---------|-----------|--------|
| **bridge.etrid.org** | Cross-chain Bridge | Hostinger public_html/bridge/ | 🔴 Setup Required |
| **governance.etrid.org** | Governance Portal | Hostinger public_html/governance/ | 🔴 Setup Required |
| **vote.etrid.org** | Alternative for governance | → governance.etrid.org | 🟡 Optional Redirect |
| **dex.etrid.org** | Decentralized Exchange | FlareSwap deployment | 🟡 Future |
| **swap.etrid.org** | Alternative for DEX | → dex.etrid.org | 🟡 Future |

### Community & Support (Standard Priority)

| Subdomain | Purpose | Points To | Status |
|-----------|---------|-----------|--------|
| **forum.etrid.org** | Community Forum | Discourse/NodeBB server | 🔴 Setup Required |
| **blog.etrid.org** | Official Blog | Hostinger public_html/blog/ | 🟡 Optional |
| **status.etrid.org** | Network Status Page | Status page service | 🟡 Optional |

### Developer Tools (Standard Priority)

| Subdomain | Purpose | Points To | Status |
|-----------|---------|-----------|--------|
| **api.etrid.org** | RPC API Endpoint | FlareChain RPC server | 🔴 Setup Required |
| **rpc.etrid.org** | Alternative RPC | Same as api.etrid.org | 🟡 Optional |
| **testnet.etrid.org** | Testnet Resources | Testnet info/faucet | 🟡 Optional |
| **faucet.etrid.org** | Testnet Faucet | Faucet application | 🟡 Optional |

### Monitoring & Analytics (Low Priority)

| Subdomain | Purpose | Points To | Status |
|-----------|---------|-----------|--------|
| **grafana.etrid.org** | Grafana Dashboard | Grafana server (port 3000) | 🟡 Optional |
| **prometheus.etrid.org** | Prometheus Metrics | Prometheus server | 🟡 Optional |
| **stats.etrid.org** | Network Statistics | Stats dashboard | 🟡 Optional |

---

## 🚀 Deployment Phases

### Phase 1: Core Setup (Do First) - 30 minutes

**Required subdomains for basic functionality:**

```
✅ etrid.org               → Hostinger public_html/
✅ www.etrid.org           → redirect to etrid.org
🔴 wallet.etrid.org        → Hostinger public_html/wallet/
🔴 explorer.etrid.org      → Hostinger public_html/explorer/
🔴 docs.etrid.org          → Documentation (external or /docs/)
```

**Why these first:**
- Main site is the entry point
- Wallet is needed for all users
- Explorer is essential for blockchain interaction
- Docs provide support and guides

---

### Phase 2: Validator Infrastructure (Do Second) - 20 minutes

**For validator recruitment and monitoring:**

```
🔴 validators.etrid.org    → Hostinger public_html/validators/
🔴 telemetry.etrid.org     → Grafana server or /network/
🔴 api.etrid.org           → FlareChain RPC node
```

**Why these second:**
- Validators need leaderboard and program page
- Telemetry shows network health
- API enables dApp development

---

### Phase 3: DeFi & Governance (Do Third) - 15 minutes

**For ecosystem growth:**

```
🔴 bridge.etrid.org        → Hostinger public_html/bridge/
🔴 governance.etrid.org    → Hostinger public_html/governance/
🟡 dex.etrid.org           → FlareSwap (when ready)
```

**Why these third:**
- Bridge enables cross-chain functionality
- Governance engages community
- DEX drives adoption

---

### Phase 4: Community & Optional (Later) - Variable

**Nice-to-have additions:**

```
🟡 forum.etrid.org         → Community forum software
🟡 blog.etrid.org          → Official blog
🟡 status.etrid.org        → Status page
🟡 faucet.etrid.org        → Testnet faucet
```

---

## 🔧 How to Setup Subdomains on Hostinger

### Method 1: For Apps Hosted on Same Server (Recommended)

**Example: Setting up wallet.etrid.org**

1. **Login to Hostinger**
   - Go to hpanel.hostinger.com
   - Select your etrid.org hosting plan

2. **Navigate to Subdomains**
   - Click "Domains" → "etrid.org" → "Subdomains"

3. **Create Subdomain**
   - Subdomain: `wallet`
   - Document Root: `public_html/wallet`
   - Click "Create"

4. **Wait for DNS Propagation**
   - Takes 5-30 minutes
   - Check: `nslookup wallet.etrid.org`

5. **Upload App Files**
   - Upload `/apps/wallet/` contents to `public_html/wallet/`
   - Files must include `index.html`

6. **Verify**
   - Visit: https://wallet.etrid.org
   - Should load the wallet app

**Repeat for all app-based subdomains:**
- `explorer` → `public_html/explorer/`
- `validators` → `public_html/validators/`
- `bridge` → `public_html/bridge/`
- `governance` → `public_html/governance/`

---

### Method 2: For External Services (Advanced)

**Example: Setting up api.etrid.org to point to RPC server**

1. **Go to DNS Zone Editor**
   - Hostinger → Domains → etrid.org → DNS/Name Servers → DNS Zone Editor

2. **Add A Record**
   - Type: `A`
   - Name: `api`
   - Points to: `98.71.91.84` (your FlareChain node IP)
   - TTL: `3600`

3. **Or Add CNAME Record**
   - Type: `CNAME`
   - Name: `api`
   - Points to: `node1.etrid.internal`
   - TTL: `3600`

4. **For Port Forwarding (Optional)**
   - Use reverse proxy (nginx/Apache)
   - Forward api.etrid.org → localhost:9944

**Use for:**
- `api.etrid.org` → RPC server
- `telemetry.etrid.org` → Grafana server (port 3000)
- `forum.etrid.org` → Forum software server

---

### Method 3: Subdomain Redirects

**Example: Redirect vote.etrid.org → governance.etrid.org**

1. **Create Subdomain**
   - Subdomain: `vote`
   - Document Root: `public_html/vote`

2. **Add .htaccess Redirect**
   ```apache
   # In public_html/vote/.htaccess
   RewriteEngine On
   RewriteRule ^(.*)$ https://governance.etrid.org/$1 [R=301,L]
   ```

**Or use DNS CNAME:**
   - Type: `CNAME`
   - Name: `vote`
   - Points to: `governance.etrid.org`

**Use for:**
- `www.etrid.org` → `etrid.org`
- `vote.etrid.org` → `governance.etrid.org`
- `swap.etrid.org` → `dex.etrid.org`
- `network.etrid.org` → `telemetry.etrid.org`

---

## 📊 Complete Subdomain Deployment Checklist

### ✅ Phase 1: Core (Deploy Now)

- [ ] **etrid.org** - Main website
  - Upload: Main site files
  - Verify: https://etrid.org loads

- [ ] **www.etrid.org** - WWW redirect
  - Setup: CNAME or redirect to etrid.org
  - Verify: https://www.etrid.org → https://etrid.org

- [ ] **wallet.etrid.org** - Web3 Wallet
  - Document Root: `public_html/wallet/`
  - Upload: `/apps/wallet/` contents
  - Verify: https://wallet.etrid.org loads

- [ ] **explorer.etrid.org** - Blockchain Explorer
  - Document Root: `public_html/explorer/`
  - Upload: `/apps/explorer/` contents
  - Verify: https://explorer.etrid.org loads

- [ ] **docs.etrid.org** - Documentation
  - Option A: Subdomain → `public_html/docs/`
  - Option B: External docs hosting (Gitbook, Docusaurus)
  - Verify: https://docs.etrid.org loads

---

### ✅ Phase 2: Validators (Deploy Second)

- [ ] **validators.etrid.org** - Validator Leaderboard
  - Document Root: `public_html/validators/`
  - Upload: `/website/validators/` contents
  - Verify: https://validators.etrid.org shows leaderboard

- [ ] **telemetry.etrid.org** - Network Monitoring
  - Option A: Point to Grafana server (98.71.91.84:3000)
  - Option B: Subdomain → `public_html/network/`
  - Verify: https://telemetry.etrid.org shows metrics

- [ ] **api.etrid.org** - RPC Endpoint
  - A Record: Points to FlareChain RPC server IP
  - Port: 9944 (WebSocket) or 9933 (HTTP)
  - Verify: `curl https://api.etrid.org` returns RPC response

---

### ✅ Phase 3: DeFi & Governance

- [ ] **bridge.etrid.org** - Cross-chain Bridge
  - Document Root: `public_html/bridge/`
  - Upload: `/apps/bridge/` contents
  - Verify: https://bridge.etrid.org loads

- [ ] **governance.etrid.org** - Governance Portal
  - Document Root: `public_html/governance/`
  - Upload: `/apps/governance/` contents
  - Verify: https://governance.etrid.org loads

- [ ] **vote.etrid.org** - Redirect to governance
  - Redirect: → governance.etrid.org
  - Verify: https://vote.etrid.org redirects

---

### 🟡 Phase 4: Optional Enhancements

- [ ] **forum.etrid.org** - Community Forum
  - Points to: Forum software server
  - Software: Discourse, NodeBB, or Flarum
  - Verify: https://forum.etrid.org loads forum

- [ ] **blog.etrid.org** - Official Blog
  - Document Root: `public_html/blog/`
  - Upload: Blog files
  - Verify: https://blog.etrid.org loads

- [ ] **status.etrid.org** - Status Page
  - Service: Statuspage.io, cachet, etc.
  - Shows: Network uptime, validator status
  - Verify: https://status.etrid.org loads

- [ ] **faucet.etrid.org** - Testnet Faucet
  - App: Faucet application
  - Gives: Testnet ÉTR to developers
  - Verify: https://faucet.etrid.org loads

- [ ] **grafana.etrid.org** - Grafana Dashboard
  - Points to: Grafana server (port 3000)
  - Reverse proxy to hide port
  - Verify: https://grafana.etrid.org loads dashboard

---

## 🔐 SSL/HTTPS Setup

Hostinger provides **free SSL certificates** for all subdomains.

### Enable SSL for Each Subdomain:

1. **Go to SSL section**
   - Hostinger → Domains → etrid.org → SSL

2. **Install SSL for subdomain**
   - Select subdomain (e.g., wallet.etrid.org)
   - Click "Install SSL"
   - Wait 5-10 minutes for activation

3. **Force HTTPS (Optional)**
   - Add to .htaccess in subdomain root:
   ```apache
   RewriteEngine On
   RewriteCond %{HTTPS} off
   RewriteRule ^(.*)$ https://%{HTTP_HOST}%{REQUEST_URI} [L,R=301]
   ```

4. **Verify SSL**
   - Visit: https://subdomain.etrid.org
   - Should show padlock icon
   - Check certificate is valid

**SSL for All Subdomains:**
- ✅ etrid.org
- ✅ www.etrid.org
- ✅ wallet.etrid.org
- ✅ explorer.etrid.org
- ✅ validators.etrid.org
- ✅ All other subdomains

---

## 🌐 DNS Propagation

After creating subdomains, DNS propagation can take:

- **Hostinger Nameservers:** 5-30 minutes (usually ~15 min)
- **External Nameservers:** Up to 48 hours (usually ~1 hour)

### Check Propagation Status:

```bash
# Check if subdomain resolves
nslookup wallet.etrid.org

# Or use online tools
# https://dnschecker.org
# https://www.whatsmydns.net
```

### If Subdomain Not Resolving:

1. **Clear DNS Cache (Local)**
   ```bash
   # macOS
   sudo dscacheutil -flushcache; sudo killall -HUP mDNSResponder

   # Windows
   ipconfig /flushdns

   # Linux
   sudo systemd-resolve --flush-caches
   ```

2. **Check DNS Records**
   - Hostinger → DNS Zone Editor
   - Verify A/CNAME record exists

3. **Wait Longer**
   - Give it 1-2 hours
   - Check again

---

## 📝 Quick Setup Commands

### Create All App Subdomains at Once

If using Hostinger's subdomain feature, you can batch create:

**In Hostinger Panel:**
1. Go to Subdomains
2. Create these one by one:

```
wallet       → public_html/wallet
explorer     → public_html/explorer
validators   → public_html/validators
bridge       → public_html/bridge
governance   → public_html/governance
telemetry    → public_html/network
```

**Upload apps via FTP:**
```bash
# Already done if you ran upload-all-apps.py
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "
python3 upload-all-apps.py
```

---

## 🔄 Subdomain → Directory Mapping

| Subdomain | Directory | App/Content |
|-----------|-----------|-------------|
| etrid.org | public_html/ | Main website |
| wallet.etrid.org | public_html/wallet/ | Wallet app |
| explorer.etrid.org | public_html/explorer/ | Explorer app |
| validators.etrid.org | public_html/validators/ | Leaderboard + program |
| bridge.etrid.org | public_html/bridge/ | Bridge app |
| governance.etrid.org | public_html/governance/ | Governance app |
| telemetry.etrid.org | public_html/network/ | Network monitoring |

**Alternative: Use /subdomain/ instead of subdomain.domain**

If you don't want to use subdomains:
- https://etrid.org/wallet/ ✅ (works now)
- https://etrid.org/validators/ ✅ (works now)
- https://etrid.org/explorer/ ✅ (works now)

**Pros of subdomain approach:**
- Cleaner URLs
- Easier to migrate to separate servers later
- Can have separate SSL certs
- Better for SEO

**Pros of /folder/ approach:**
- No DNS setup needed
- Faster to deploy
- Already working if you uploaded apps

---

## 🎯 Recommended Subdomain Strategy

### Use Subdomains For:
- ✅ **wallet.etrid.org** - Separate service feel
- ✅ **explorer.etrid.org** - Professional appearance
- ✅ **api.etrid.org** - Different server/port
- ✅ **forum.etrid.org** - Separate software
- ✅ **docs.etrid.org** - External documentation platform

### Use Folders For:
- ✅ **etrid.org/validators/** - Part of main site
- ✅ **etrid.org/network/** - Embedded content
- ✅ **etrid.org/whitepaper/** - Part of main site
- ✅ **etrid.org/blog/** - Main site section

### Hybrid Approach (Recommended):
```
Main Site:
  https://etrid.org/                  - Homepage
  https://etrid.org/whitepaper/       - Ivory papers
  https://etrid.org/validators/       - Leaderboard
  https://etrid.org/network/          - Monitoring

Subdomains:
  https://wallet.etrid.org            - Wallet app
  https://explorer.etrid.org          - Explorer
  https://api.etrid.org               - RPC endpoint
  https://docs.etrid.org              - Documentation
  https://forum.etrid.org             - Community
```

---

## 📧 Email Subdomains (Bonus)

You can also create email subdomains:

```
noreply@etrid.org       - Automated emails
support@etrid.org       - User support
validators@etrid.org    - Validator program applications
hello@etrid.org         - General inquiries
```

**Setup in Hostinger:**
- Email → Email Accounts → Create
- Can forward to Gmail/personal email

---

## ✅ Summary

### Essential Subdomains (Do First):
1. **www.etrid.org** → redirect to etrid.org
2. **wallet.etrid.org** → public_html/wallet/
3. **explorer.etrid.org** → public_html/explorer/
4. **validators.etrid.org** → public_html/validators/
5. **docs.etrid.org** → documentation

### High Priority (Do Second):
6. **api.etrid.org** → RPC server
7. **telemetry.etrid.org** → Grafana/network monitoring
8. **bridge.etrid.org** → public_html/bridge/
9. **governance.etrid.org** → public_html/governance/

### Optional (Add Later):
10. **forum.etrid.org** → community forum
11. **blog.etrid.org** → official blog
12. **status.etrid.org** → status page
13. **faucet.etrid.org** → testnet faucet

---

## 🚀 Next Steps

1. **Create subdomains in Hostinger** (Phase 1: Core)
2. **Upload apps if not already done** (`upload-all-apps.py`)
3. **Enable SSL for each subdomain**
4. **Update navigation links** to use subdomains
5. **Test all subdomains** after DNS propagation

---

**Your ËTRID ecosystem will have professional subdomains for all services!** 🌐
