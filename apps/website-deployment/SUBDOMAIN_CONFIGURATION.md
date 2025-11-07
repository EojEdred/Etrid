# Hostinger Subdomain Configuration Guide

**All files have been uploaded successfully!** ✅

Now you need to configure subdomains in Hostinger so that:
- `masterchef.etrid.org` → Points to `/public_html/masterchef/`
- `bridge.etrid.org` → Points to `/public_html/bridge/`
- `explorer.etrid.org` → Points to `/public_html/explorer/`
- `forum.etrid.org` → Points to `/public_html/forum/`
- `telemetry.etrid.org` → Points to `/public_html/telemetry/`
- `wallet.etrid.org` → Points to `/public_html/wallet/` 🆕
- `validator.etrid.org` → Points to `/public_html/validator/` 🆕

---

## Step-by-Step: Configure Subdomains in Hostinger

### Step 1: Login to Hostinger

1. Go to: https://hostinger.com
2. Click "Login" (top right)
3. Enter your email and password
4. Click "Log In"

### Step 2: Access hPanel

1. After login, you'll see your hosting accounts
2. Click on the **"hPanel"** button for `etrid.org`
3. This opens your hosting control panel

### Step 3: Navigate to Subdomains

1. In hPanel, look for **"Domains"** section (left sidebar or main dashboard)
2. Click on **"Subdomains"** (or **"Manage Domains"** → **"Subdomains"**)
3. You should see a list of existing subdomains (if any)

### Step 4: Create Each Subdomain

For **EACH subdomain**, repeat these steps:

#### Example: Creating `masterchef.etrid.org`

1. Click **"Create Subdomain"** button
2. Fill in the form:
   - **Subdomain:** `masterchef` (just the prefix, not the full domain)
   - **Domain:** `etrid.org` (should be auto-selected)
   - **Document Root:** `/domains/etrid.org/public_html/masterchef`
3. Click **"Create"** button
4. Wait for confirmation message

#### Repeat for All Subdomains:

| Subdomain | Document Root |
|-----------|---------------|
| `masterchef` | `/domains/etrid.org/public_html/masterchef` |
| `bridge` | `/domains/etrid.org/public_html/bridge` |
| `explorer` | `/domains/etrid.org/public_html/explorer` |
| `forum` | `/domains/etrid.org/public_html/forum` |
| `telemetry` | `/domains/etrid.org/public_html/telemetry` |
| `wallet` | `/domains/etrid.org/public_html/wallet` |
| `validator` | `/domains/etrid.org/public_html/validator` |

---

## Step 5: Wait for DNS Propagation

After creating all subdomains:

1. **Wait 5-60 minutes** for DNS changes to propagate
2. DNS propagation time varies:
   - Local: 5-10 minutes
   - Worldwide: Up to 24 hours (usually 30-60 minutes)

### Check DNS Status

You can check if DNS has propagated:

```bash
# Check if subdomain resolves
nslookup masterchef.etrid.org

# Check DNS record
dig masterchef.etrid.org
```

Expected result: Should show same IP as `etrid.org`

---

## Step 6: Test Each Subdomain

Once DNS has propagated, test each subdomain:

| URL | Expected Result |
|-----|-----------------|
| https://masterchef.etrid.org | Yield farming dashboard loads |
| https://bridge.etrid.org | Bridge interface loads |
| https://explorer.etrid.org | Block explorer loads |
| https://forum.etrid.org | Forum categories load |
| https://telemetry.etrid.org | Validator telemetry loads |
| https://wallet.etrid.org | Wallet app loads |
| https://validator.etrid.org | Validator dashboard loads |

---

## Troubleshooting

### Problem: Subdomain shows "404 Not Found"

**Solution:**
1. Check Document Root path is correct
2. Make sure files exist at that path
3. Check file permissions (should be 644 for files, 755 for directories)

### Problem: Subdomain shows "This site can't be reached"

**Solution:**
- DNS hasn't propagated yet → Wait longer
- Subdomain not created → Check hPanel → Subdomains
- Try clearing DNS cache: `ipconfig /flushdns` (Windows) or `sudo dscacheutil -flushcache` (Mac)

### Problem: Subdomain shows wrong content

**Solution:**
- Document Root path is wrong → Edit subdomain in hPanel
- Files uploaded to wrong location → Check FTP paths

### Problem: Can't find "Subdomains" in hPanel

**Solution:**
- Look for: "Domains" → "Subdomains"
- Or try: "Websites" → Select etrid.org → "Subdomains"
- Or search for "subdomain" in hPanel search bar (top right)

---

## Alternative: Edit Existing Subdomains

If subdomains already exist but point to wrong locations:

1. Go to hPanel → Subdomains
2. Find the subdomain in the list
3. Click **"Manage"** or **"Edit"** button
4. Update the **Document Root** path
5. Click **"Save"** or **"Update"**
6. Wait for changes to apply (1-5 minutes)

---

## Quick Reference: What's Uploaded Where

All files are in: `/domains/etrid.org/public_html/`

```
public_html/
├── index.html (main website)
├── masterchef/
│   └── index.html (29,258 bytes)
├── bridge/
│   └── index.html (14,114 bytes)
├── explorer/
│   └── index.html (28,721 bytes)
├── forum/
│   └── index.html (18,011 bytes)
├── telemetry/
│   ├── index.html (14,815 bytes)
│   └── app-telemetry-feed.js (14,973 bytes)
├── wallet/
│   └── index.html (19,813 bytes) 🆕
└── validator/
    └── index.html (19,993 bytes) 🆕
```

---

## Temporary Workaround

While waiting for DNS or if subdomains don't work, users can access via:

- https://etrid.org/masterchef/
- https://etrid.org/bridge/
- https://etrid.org/explorer/
- https://etrid.org/forum/
- https://etrid.org/telemetry/
- https://etrid.org/wallet/
- https://etrid.org/validator/

These work immediately without subdomain configuration!

---

## After Configuration

Once all subdomains are configured and working, you'll have:

✅ **8 Fully Functional Apps:**

1. **etrid.org** - Main website (MetaMask wallet)
2. **masterchef.etrid.org** - Yield farming (Polkadot.js wallet)
3. **bridge.etrid.org** - Cross-chain bridge (Polkadot.js wallet)
4. **explorer.etrid.org** - Block explorer (no wallet needed)
5. **forum.etrid.org** - Community forum (demo)
6. **telemetry.etrid.org** - Validator monitoring
7. **wallet.etrid.org** - Wallet app (Polkadot.js) 🆕
8. **validator.etrid.org** - Validator dashboard (Polkadot.js) 🆕

All apps have wallet connections where applicable!

---

## Need Help?

### Hostinger Support:
- **Live Chat:** Available in hPanel (bottom right)
- **Email:** support@hostinger.com
- **Phone:** Available in your hPanel under "Support"

Tell them: *"I need to configure subdomains for my domain etrid.org to point to different directories in my public_html folder."*

### DNS Check Tools:
- https://dnschecker.org/ (check DNS propagation worldwide)
- https://www.whatsmydns.net/ (check from multiple locations)

---

## Summary

**What You Need to Do:**

1. Login to Hostinger → hPanel
2. Go to Subdomains section
3. Create 7 subdomains (see table above)
4. Wait 30-60 minutes for DNS
5. Test all URLs

**Estimated Time:** 10 minutes + DNS propagation time

**Difficulty:** Easy (point-and-click in web interface)

---

**Once done, all ËTRID apps will be live and accessible via their subdomains with full wallet integration!** 🎉
