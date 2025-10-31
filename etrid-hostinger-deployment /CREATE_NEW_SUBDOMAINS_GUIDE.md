# 🚀 CREATE NEW SUBDOMAINS ON HOSTINGER

**Issue:** Explorer, Bridge, and Faucet show "having trouble finding site"
**Reason:** These subdomains haven't been created yet on Hostinger

---

## ⚠️ IMPORTANT

The following 6 subdomains are **NEW** and need to be created in Hostinger before uploading files:

1. **explorer.etrid.org** - Block explorer
2. **bridge.etrid.org** - Cross-chain bridge
3. **faucet.etrid.org** - Testnet faucet
4. **status.etrid.org** - Network status
5. **forum.etrid.org** - Governance forum
6. **blog.etrid.org** - News blog

---

## 📋 STEP-BY-STEP INSTRUCTIONS

### For Each New Subdomain:

#### Step 1: Create the Subdomain in Hostinger

1. Log into **Hostinger Control Panel**
2. Go to **"Domains"** section
3. Click **"Subdomains"**
4. Click **"Create Subdomain"** button
5. In the subdomain field, enter: `explorer` (or bridge, faucet, etc.)
6. Make sure the parent domain is: `etrid.org`
7. Click **"Create"**
8. Wait 2-3 minutes for DNS propagation

#### Step 2: Upload Files to the New Subdomain

1. Go to **File Manager** in Hostinger
2. Navigate to the newly created subdomain directory
   - It will be something like: `public_html/explorer/` or similar
3. Upload the corresponding deployment zip:
   - For explorer.etrid.org → upload `explorer-deploy.zip`
   - For bridge.etrid.org → upload `bridge-deploy.zip`
   - For faucet.etrid.org → upload `faucet-deploy.zip`
4. **Extract the zip file**
5. Delete the zip file after extraction
6. Verify these files are present:
   - ✅ `index.html`
   - ✅ `.htaccess`

#### Step 3: Test the Subdomain

1. Open browser
2. Go to the subdomain URL (e.g., `https://explorer.etrid.org`)
3. Should load the page successfully
4. If still shows error, wait another 5-10 minutes for DNS propagation

---

## 🗂️ COMPLETE SUBDOMAIN CHECKLIST

### Existing Subdomains (Already Created)
- ✅ docs.etrid.org
- ✅ wallet.etrid.org
- ✅ validator.etrid.org
- ✅ masterchef.etrid.org
- ✅ watchtower.etrid.org
- ✅ telemetry.etrid.org
- ✅ governance.etrid.org

### New Subdomains (Need to Create)
- ⏳ **explorer.etrid.org** → `explorer-deploy.zip` (2.9 KB)
- ⏳ **bridge.etrid.org** → `bridge-deploy.zip` (2.6 KB)
- ⏳ **faucet.etrid.org** → `faucet-deploy.zip` (2.6 KB)
- ⏳ **status.etrid.org** → `status-deploy.zip` (1.7 KB)
- ⏳ **forum.etrid.org** → `forum-deploy.zip` (1.9 KB)
- ⏳ **blog.etrid.org** → `blog-deploy.zip` (2.3 KB)

---

## 📦 DEPLOYMENT PACKAGE LOCATIONS

All packages are in: `/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /apps/`

```
apps/
├── explorer-deploy.zip (2.9 KB)
├── bridge-deploy.zip (2.6 KB)
├── faucet-deploy.zip (2.6 KB)
├── status-deploy.zip (1.7 KB)
├── forum-deploy.zip (1.9 KB)
└── blog-deploy.zip (2.3 KB)
```

---

## 🔍 TROUBLESHOOTING

### Subdomain Still Shows Error After Creation

**If you see "can't find site" or DNS errors:**

1. **Wait for DNS propagation** (5-30 minutes)
2. **Clear browser cache** (Ctrl+F5 or Cmd+Shift+R)
3. **Check subdomain was created correctly** in Hostinger panel
4. **Verify files were uploaded** to correct directory
5. **Check .htaccess file exists** in subdomain root

### Subdomain Shows Empty Page

1. Check `index.html` was extracted to root directory
2. Verify `.htaccess` file is present
3. Check file permissions (should be 644 for files, 755 for directories)

### Subdomain Shows 403 Forbidden

1. Add/update `.htaccess` file:
```apache
RewriteEngine On
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteRule ^(.*)$ index.html [L]
```

2. Check index.html file permissions (should be 644)

---

## ⚡ QUICK COMMANDS FOR ALL 6 NEW SUBDOMAINS

In Hostinger control panel, create these subdomains:

1. Create subdomain: `explorer`
2. Create subdomain: `bridge`
3. Create subdomain: `faucet`
4. Create subdomain: `status`
5. Create subdomain: `forum`
6. Create subdomain: `blog`

Then upload and extract the corresponding `-deploy.zip` files.

---

## ✅ VERIFICATION CHECKLIST

After creating each subdomain and uploading files:

- [ ] Subdomain created in Hostinger panel
- [ ] Deployment zip uploaded to subdomain directory
- [ ] Zip file extracted
- [ ] index.html file present in root
- [ ] .htaccess file present in root
- [ ] Subdomain URL loads successfully
- [ ] Page displays correct content (not 404/403 error)

---

## 📞 NEED HELP?

If you continue having issues:

1. **Check Hostinger documentation** on creating subdomains
2. **Contact Hostinger support** - they can verify DNS is configured correctly
3. **Verify your plan supports unlimited subdomains**
4. **Check domain DNS settings** are pointing to Hostinger nameservers

---

**Note:** The files are ready to deploy. You just need to create the subdomains in Hostinger first, then upload the files!
