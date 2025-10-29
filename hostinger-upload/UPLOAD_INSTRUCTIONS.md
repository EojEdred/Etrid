# ËTRID Hostinger Upload Instructions

## ✅ Fixed Zip Files - Ready to Upload!

All zip files have been recreated to extract properly without nested folders.

---

## 📦 Files Ready for Upload

| Zip File | Size | Destination | Purpose |
|----------|------|-------------|---------|
| `website.zip` | 67 KB | `/public_html/` | Main etrid.org site (with governance) |
| `validator.zip` | 630 KB | `/public_html/validator/` | Validator dashboard |
| `watchtower.zip` | 731 KB | `/public_html/watchtower/` | Watchtower monitor |
| `masterchef.zip` | 310 KB | `/public_html/masterchef/` | MasterChef dashboard |
| `wallet.zip` | 924 KB | `/public_html/wallet/` | Wallet with governance UI |

---

## 🚀 Step-by-Step Upload Process

### **Option 1: Using Hostinger File Manager** (Easiest)

#### **1. Upload Main Website (etrid.org)**

1. Log into **hpanel.hostinger.com**
2. Click on your **etrid.org** domain
3. Click **"File Manager"** in the left sidebar
4. Navigate to `/public_html/`
5. Upload `website.zip`:
   - Click **"Upload"** button (top right)
   - Select `website.zip` from your computer
   - Wait for upload to complete
6. Right-click on `website.zip` → **"Extract"**
7. After extraction, you should see these files in `/public_html/`:
   ```
   index.html
   css/
   js/
   images/
   robots.txt
   sitemap.xml
   .htaccess
   ```
8. Delete `website.zip` (cleanup)
9. **Test**: Visit https://etrid.org

---

#### **2. Upload Validator Dashboard**

1. In File Manager, navigate to `/public_html/validator/`
2. Upload `validator.zip`
3. Right-click → **"Extract"**
4. After extraction, you should see:
   ```
   index.html
   _next/
   404.html
   favicon.ico
   performance.html
   nominators.html
   rewards.html
   settings.html
   ```
5. Delete `validator.zip` (cleanup)
6. **Test**: Visit https://validator.etrid.org

---

#### **3. Upload Watchtower Monitor**

1. Navigate to `/public_html/watchtower/`
2. Upload `watchtower.zip`
3. Right-click → **"Extract"**
4. After extraction, you should see:
   ```
   index.html
   _next/
   Monitor.html
   Reports.html
   Settings.html
   404.html
   ```
5. Delete `watchtower.zip`
6. **Test**: Visit https://watchtower.etrid.org

---

#### **4. Upload MasterChef Dashboard**

1. Navigate to `/public_html/masterchef/`
2. Upload `masterchef.zip`
3. Right-click → **"Extract"**
4. After extraction, you should see:
   ```
   index.html
   _next/
   404.html
   404/
   ```
5. Delete `masterchef.zip`
6. **Test**: Visit https://masterchef.etrid.org

---

#### **5. Upload Wallet (with Governance)**

1. Navigate to `/public_html/wallet/`
2. Upload `wallet.zip`
3. Right-click → **"Extract"**
4. After extraction, you should see:
   ```
   index.html
   _next/
   governance.html     ← Important! Governance page
   swap.html
   404.html
   placeholder-*.* (images)
   ```
5. Delete `wallet.zip`
6. **Test**:
   - Visit https://wallet.etrid.org
   - Visit https://wallet.etrid.org/governance ← **Governance UI!**

---

## ✅ Verification Checklist

After uploading all files, test each URL:

```bash
✅ https://etrid.org                        # Main site
✅ https://etrid.org#governance             # Governance section
✅ https://validator.etrid.org              # Validator dashboard
✅ https://watchtower.etrid.org             # Watchtower monitor
✅ https://masterchef.etrid.org             # MasterChef dashboard
✅ https://wallet.etrid.org                 # Wallet homepage
✅ https://wallet.etrid.org/governance      # Governance voting interface
```

---

## 🔧 Troubleshooting

### **Issue: Pages showing "404 Not Found"**

**Cause**: Files not extracted properly or missing `index.html`

**Solution**:
1. Check that `index.html` exists in the subdomain root folder
2. If not, re-extract the zip file
3. Make sure you're extracting in the correct folder

---

### **Issue: Blank white page or loading forever**

**Cause**: JavaScript files not loading correctly

**Solution**:
1. Check browser console for errors (F12 → Console tab)
2. Make sure `_next/` folder exists alongside `index.html`
3. Clear browser cache and reload (Ctrl+Shift+R)

---

### **Issue: Governance page not found**

**Cause**: Wallet not uploaded or governance.html missing

**Solution**:
1. Check that `/public_html/wallet/governance.html` exists
2. Re-upload `wallet.zip` if needed
3. Make sure URL is `https://wallet.etrid.org/governance` (not `gov.etrid.org`)

---

### **Issue: Styles look broken or missing**

**Cause**: CSS files not loading

**Solution**:
1. For main site: Check that `/public_html/css/styles.css` exists
2. For apps: Check that `_next/static/css/` folder exists
3. Check file permissions (should be 644 for files, 755 for folders)
4. Clear browser cache

---

## 📝 Optional: Setup gov.etrid.org Redirect

If you want `gov.etrid.org` to redirect to the wallet governance page:

1. Navigate to `/public_html/gov/`
2. Create/edit `.htaccess` file
3. Add this content:
   ```apache
   RewriteEngine On
   RewriteRule ^(.*)$ https://wallet.etrid.org/governance [R=301,L]
   ```
4. Save the file
5. Test: Visit https://gov.etrid.org (should redirect)

---

## 🔐 SSL Certificate Check

Make sure SSL is enabled for all subdomains:

1. In Hostinger panel → **SSL**
2. Check these domains have SSL:
   - etrid.org
   - validator.etrid.org
   - watchtower.etrid.org
   - masterchef.etrid.org
   - wallet.etrid.org
3. If missing, click **"Install SSL"** for each one
4. Wait 5-15 minutes for SSL to activate

---

## 🎉 You're Done!

Once all URLs are working:

1. **Main site** shows governance section ✅
2. **Wallet governance** page works ✅
3. **All subdomains** are accessible ✅
4. **SSL certificates** are active (https://) ✅

Your ËTRID ecosystem is now live with integrated governance!

---

## 📞 Need Help?

- **File structure issues**: Check the `GOVERNANCE_INTEGRATION_COMPLETE.md` guide
- **Hostinger support**: https://support.hostinger.com
- **Governance details**: See runtime pallets in `/10-foundation/governance/`

**All zip files are ready in:**
`/Users/macbook/Desktop/etrid/hostinger-upload/`
