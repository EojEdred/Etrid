# Fix Telemetry Subdomain Configuration

**Issue:** https://telemetry.etrid.org is not loading properly

**Files are uploaded correctly:**
- `/domains/etrid.org/public_html/telemetry/index.html` ✅
- `/domains/etrid.org/public_html/telemetry/app-telemetry-feed.js` ✅

**Problem:** Subdomain DNS/configuration in Hostinger

---

## Solution: Configure Subdomain in Hostinger

### Step 1: Login to Hostinger
1. Go to https://hostinger.com
2. Login with your credentials
3. Navigate to **hPanel** dashboard

### Step 2: Check Subdomain Configuration
1. In hPanel, find **Domains** section
2. Click on **etrid.org**
3. Look for **Subdomains** option

### Step 3: Add/Verify telemetry Subdomain

You need to create a subdomain pointing to the correct directory:

**Subdomain:** `telemetry`
**Document Root:** `/domains/etrid.org/public_html/telemetry`

#### If subdomain doesn't exist:
1. Click "Create Subdomain"
2. Enter subdomain name: `telemetry`
3. Set document root: `/domains/etrid.org/public_html/telemetry`
4. Click "Create"

#### If subdomain exists but points to wrong location:
1. Find `telemetry.etrid.org` in the list
2. Click "Manage" or "Edit"
3. Change document root to: `/domains/etrid.org/public_html/telemetry`
4. Save changes

### Step 4: DNS Propagation
- DNS changes can take **5-60 minutes** to propagate
- Clear browser cache after DNS update
- Try accessing in incognito/private mode

---

## Alternative: Check DNS Records

If subdomain configuration looks correct, check DNS records:

### Check Current DNS
```bash
# Check if subdomain resolves
nslookup telemetry.etrid.org

# Check DNS A record
dig telemetry.etrid.org
```

### Expected Result:
```
telemetry.etrid.org should point to same IP as etrid.org
```

### If DNS not resolving:
1. In hPanel → Domains → etrid.org → DNS Zone Editor
2. Add A record:
   - Type: `A`
   - Name: `telemetry`
   - Points to: `(same IP as main domain)`
   - TTL: `14400`

---

## Verify Files Are Accessible

Test direct file access:
```bash
curl https://etrid.org/telemetry/index.html
curl https://etrid.org/telemetry/app-telemetry-feed.js
```

If these work, but `https://telemetry.etrid.org` doesn't, it's definitely a subdomain configuration issue.

---

## All Other Subdomains That Need Same Configuration

Make sure these are all configured correctly:

| Subdomain | Document Root |
|-----------|---------------|
| `masterchef.etrid.org` | `/domains/etrid.org/public_html/masterchef` |
| `explorer.etrid.org` | `/domains/etrid.org/public_html/explorer` |
| `forum.etrid.org` | `/domains/etrid.org/public_html/forum` |
| `bridge.etrid.org` | `/domains/etrid.org/public_html/bridge` |
| `telemetry.etrid.org` | `/domains/etrid.org/public_html/telemetry` |
| `governance.etrid.org` | `/domains/etrid.org/public_html/governance` |

---

## Quick Test After Configuration

1. Wait 5-10 minutes after making changes
2. Clear browser cache (Ctrl+Shift+Delete)
3. Try accessing: https://telemetry.etrid.org
4. Check browser console for errors (F12)

---

## If Still Not Working

### Check .htaccess File
Create `/domains/etrid.org/public_html/telemetry/.htaccess`:
```apache
DirectoryIndex index.html
RewriteEngine On
RewriteBase /
RewriteCond %{REQUEST_FILENAME} !-f
RewriteCond %{REQUEST_FILENAME} !-d
RewriteRule ^(.*)$ index.html [L]
```

### Check File Permissions
```bash
# Should be 644 for files
chmod 644 /domains/etrid.org/public_html/telemetry/index.html
chmod 644 /domains/etrid.org/public_html/telemetry/app-telemetry-feed.js

# Should be 755 for directory
chmod 755 /domains/etrid.org/public_html/telemetry
```

---

## Contact Hostinger Support

If none of the above works, contact Hostinger support with:
- Issue: Subdomain `telemetry.etrid.org` not resolving
- Expected behavior: Should load files from `/domains/etrid.org/public_html/telemetry/`
- Error: Subdomain not loading or showing wrong content

**Hostinger Support:**
- Live Chat in hPanel
- Email: support@hostinger.com
- 24/7 available

---

## Temporary Workaround

While DNS propagates, users can access telemetry via:
- Direct path: https://etrid.org/telemetry/
- This should work immediately since files are uploaded

---

**Once configured, telemetry will display:**
- Live validator status
- ASF consensus metrics
- PPFA committee monitoring
- Block production statistics
