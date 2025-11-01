# 🚀 ËTRID Quick Reference - Deployment Guide

## 📋 Everything You Need to Deploy

**Last Updated:** October 31, 2025
**Main Domain:** etrid.org
**Twitter:** @gizzi_io

---

## ✅ What's Ready to Deploy

### 1. Main Website with Updates
- ✅ Ivory papers viewer (5 documents including Protocol Charter)
- ✅ Network monitoring page with Grafana
- ✅ Updated homepage navigation
- ✅ All Twitter links changed to @gizzi_io
- ✅ Validator leaderboard + participation pages

### 2. Seven Apps (Next.js Static Exports)
- ✅ Wallet (`/apps/wallet/` - 947 KB)
- ✅ Validator Dashboard (`/apps/validator/` - 880 KB)
- ✅ Explorer (`/apps/explorer/` - 3 KB)
- ✅ MasterChef (`/apps/masterchef/` - 534 KB)
- ✅ Telemetry (`/apps/telemetry/` - 224 KB)
- ✅ Governance (`/apps/governance/` - 14 KB)
- ✅ Bridge (`/apps/bridge/` - 3 KB)

---

## 🌐 Subdomains to Create (Priority Order)

### 🔴 PHASE 1: Essential (Setup First)

```
1. www.etrid.org           → Redirect to etrid.org
2. wallet.etrid.org        → public_html/wallet/
3. explorer.etrid.org      → public_html/explorer/
4. docs.etrid.org          → Documentation (external or /docs/)
```

**Why:** Core user-facing services everyone needs.

---

### 🟡 PHASE 2: Infrastructure (Setup Second)

```
5. validators.etrid.org    → public_html/validators/
6. api.etrid.org           → FlareChain RPC server (98.71.91.84:9944)
7. telemetry.etrid.org     → Grafana (98.71.91.84:3000) or /network/
```

**Why:** Validator infrastructure and developer tools.

---

### 🟢 PHASE 3: DeFi & Ecosystem (Setup Third)

```
8. bridge.etrid.org        → public_html/bridge/
9. governance.etrid.org    → public_html/governance/
10. vote.etrid.org         → Redirect to governance.etrid.org
```

**Why:** Ecosystem growth features.

---

### ⚪ PHASE 4: Optional (Setup Later)

```
11. forum.etrid.org        → Community forum server
12. blog.etrid.org         → public_html/blog/
13. status.etrid.org       → Status page service
14. faucet.etrid.org       → Testnet faucet app
15. grafana.etrid.org      → Direct Grafana access
```

**Why:** Nice-to-have community features.

---

## 📊 Subdomain Setup Quick Steps

### For Apps on Same Server (Hostinger):

1. **Go to Subdomains:**
   - Login to hpanel.hostinger.com
   - Domains → etrid.org → Subdomains

2. **Create Subdomain:**
   - Subdomain: `wallet`
   - Document Root: `public_html/wallet`
   - Click "Create"

3. **Upload Files:**
   ```bash
   python3 upload-all-apps.py
   # Select the app to upload
   ```

4. **Enable SSL:**
   - Domains → etrid.org → SSL
   - Select subdomain → Install SSL

5. **Verify:**
   - Visit https://wallet.etrid.org
   - Should load the app

### For External Services (RPC, Grafana):

1. **Go to DNS Zone Editor:**
   - Domains → etrid.org → DNS Zone Editor

2. **Add A Record:**
   - Type: `A`
   - Name: `api`
   - Points to: `98.71.91.84`
   - TTL: `3600`

3. **Verify:**
   - `nslookup api.etrid.org`
   - Should return the IP

---

## 🚀 Deployment Scripts (In Order)

### 1. Deploy Ivory Papers + Protocol Charter (6 minutes)
```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "
python3 upload-ivory-papers-ftp.py
```

**Uploads:**
- viewer-standalone.html (451 KB) with 5 documents
- check-file-integrity.html

**Verify at:** https://etrid.org/whitepaper/viewer-standalone.html

---

### 2. Deploy Network Monitoring + Updated Homepage (5 minutes)
```bash
python3 upload-monitoring-page.py
```

**Uploads:**
- network/index.html (Grafana embed)
- index.html (updated Twitter links)

**Verify at:**
- https://etrid.org/network/
- https://etrid.org/

---

### 3. Deploy All Apps (15 minutes)
```bash
python3 upload-all-apps.py
# Select "1. All apps"
```

**Uploads:**
- wallet/ (947 KB)
- validator/ (880 KB)
- explorer/ (3 KB)
- masterchef/ (534 KB)
- telemetry/ (224 KB)
- governance/ (14 KB)
- bridge/ (3 KB)

**Verify at:**
- https://etrid.org/wallet/
- https://etrid.org/validator/
- https://etrid.org/explorer/
- (etc.)

---

## 🐦 Twitter Links (All Updated)

All Twitter links across the site now point to:

### ✅ @gizzi_io
**URL:** https://twitter.com/gizzi_io

**Updated in 28 files including:**
- Homepage
- Blog
- Developers
- Learn
- All feature pages
- All whitepaper pages
- Validator pages

---

## 📝 Post-Deployment Checklist

### After Uploading Files:

- [ ] **Clear browser cache** (Cmd+Shift+R or Ctrl+Shift+R)
- [ ] **Test main site** - https://etrid.org
- [ ] **Test ivory papers** - https://etrid.org/whitepaper/viewer-standalone.html
  - [ ] All 5 documents load (Complete, Vol I, II, III, Charter)
- [ ] **Test network monitoring** - https://etrid.org/network/
  - [ ] Grafana dashboard visible
- [ ] **Test apps** - https://etrid.org/wallet/, /explorer/, /validator/
  - [ ] All load without errors
- [ ] **Check Twitter links** - Click social icons
  - [ ] Should go to https://twitter.com/gizzi_io

---

### After Creating Subdomains:

- [ ] **www.etrid.org** → Redirects to etrid.org
- [ ] **wallet.etrid.org** → Loads wallet app
- [ ] **explorer.etrid.org** → Loads explorer
- [ ] **validators.etrid.org** → Loads leaderboard
- [ ] **api.etrid.org** → RPC endpoint responds
- [ ] **telemetry.etrid.org** → Shows monitoring
- [ ] **All have SSL** → Padlock icon visible

---

## 🔐 SSL Certificates

**Hostinger provides free SSL for all subdomains.**

**Enable for each subdomain:**
1. Domains → etrid.org → SSL
2. Select subdomain
3. Click "Install SSL"
4. Wait 5-10 minutes

**Force HTTPS (optional .htaccess):**
```apache
RewriteEngine On
RewriteCond %{HTTPS} off
RewriteRule ^(.*)$ https://%{HTTP_HOST}%{REQUEST_URI} [L,R=301]
```

---

## 📧 Recommended Email Addresses

Set up in Hostinger → Email Accounts:

```
hello@etrid.org          - General inquiries
support@etrid.org        - User support
validators@etrid.org     - Validator program
noreply@etrid.org        - Automated emails
security@etrid.org       - Security reports
press@etrid.org          - Press inquiries
```

---

## 🌐 DNS Propagation Time

After creating subdomains:
- **Hostinger:** 5-30 minutes (usually ~15 min)
- **External DNS:** Up to 48 hours (usually ~1 hour)

**Check propagation:**
```bash
nslookup wallet.etrid.org
# or visit: https://dnschecker.org
```

---

## 🎯 Quick Commands Reference

### Upload Everything at Once:
```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "

# 1. Ivory papers + charter
python3 upload-ivory-papers-ftp.py

# 2. Network monitoring + homepage
python3 upload-monitoring-page.py

# 3. All apps
python3 upload-all-apps.py
```

**Total time:** ~25 minutes
**You'll need:** FTP credentials (host, username, password)

---

### Check What Files Changed:
```bash
# Show Twitter link changes
grep -r "twitter.com/gizzi_io" website/ --include="*.html" | wc -l

# Show ivory paper file size
ls -lh website/whitepaper/viewer-standalone.html

# Show apps available
ls -d apps/*/
```

---

## 📊 File Sizes Summary

| Component | Size | Upload Time |
|-----------|------|-------------|
| Ivory Papers Viewer | 451 KB | ~6 min |
| Network Monitoring | 15 KB | ~1 min |
| Homepage | 20 KB | ~1 min |
| Wallet App | 947 KB | ~4 min |
| Validator App | 880 KB | ~3 min |
| Explorer App | 3 KB | ~1 min |
| MasterChef App | 534 KB | ~2 min |
| Telemetry App | 224 KB | ~1 min |
| Governance App | 14 KB | ~1 min |
| Bridge App | 3 KB | ~1 min |
| **TOTAL** | **~3.1 MB** | **~21 min** |

---

## 🔄 Future Updates

### To Update Ivory Papers:
```bash
# After editing files in /etrid/docs/specifications/
python3 update-ivory-papers-viewer.py
python3 upload-ivory-papers-ftp.py
```

### To Update Network Monitoring:
```bash
# After editing /website/network/index.html
python3 upload-monitoring-page.py
```

### To Update Apps:
```bash
# After rebuilding app in /apps/[app-name]/
python3 upload-all-apps.py
# Select specific app to re-upload
```

---

## 📚 Documentation References

**All guides in deployment folder:**
- `COMPLETE_WEBSITE_DEPLOYMENT_SUMMARY.md` - Full deployment guide
- `SUBDOMAIN_SETUP_GUIDE.md` - Detailed subdomain instructions
- `TWITTER_LINKS_UPDATED.md` - Twitter link update summary
- `IVORY_PAPERS_UPDATED.md` - Ivory papers changes
- `MONITORING_UPDATED.md` - Network monitoring details
- `APPS_INTEGRATION_GUIDE.md` - Apps deployment guide
- `VALIDATOR_PAGE_COMPARISON.md` - Validator page analysis

---

## 🎉 Summary

### ✅ Ready to Deploy:
- **Main Website:** Updated with Twitter → @gizzi_io
- **Ivory Papers:** 5 documents (Complete, Vol I-III, Charter)
- **Network Monitoring:** Live Grafana dashboard
- **7 Apps:** Wallet, validator, explorer, etc.

### 🌐 Subdomains to Create:
- **Phase 1 (Essential):** www, wallet, explorer, docs
- **Phase 2 (Infrastructure):** validators, api, telemetry
- **Phase 3 (Ecosystem):** bridge, governance, vote
- **Phase 4 (Optional):** forum, blog, status, faucet

### 🐦 Social Media:
- **Twitter:** https://twitter.com/gizzi_io (updated in all files)

### ⏱️ Deployment Time:
- **File Upload:** ~25 minutes (automated scripts)
- **Subdomain Setup:** ~30-60 minutes (manual in Hostinger)
- **Total:** ~1-2 hours for complete deployment

---

**Everything is ready! Run the 3 upload scripts and create subdomains in Hostinger.** 🚀

**Your ËTRID website ecosystem will be fully deployed and professional!** 🌐
