# ËTRID HOSTINGER ARCHITECTURE - COMPLETE STRUCTURE

## 🏗️ **Complete Directory Structure**

Your Hostinger account base path:
```
/home/u724092535/domains/etrid.org/
```

### **Full File Tree:**

```
/home/u724092535/domains/etrid.org/
│
├── public_html/                           ← Main domain root (etrid.org)
│   │
│   ├── index.html                         ← Main homepage
│   ├── .htaccess                          ← Apache configuration
│   ├── robots.txt                         ← Search engine directives
│   ├── sitemap.xml                        ← Site map
│   │
│   ├── css/                               ← Stylesheets
│   │   └── styles.css
│   │
│   ├── js/                                ← JavaScript
│   │   └── main.js
│   │
│   ├── images/                            ← Images and assets
│   │   └── (logo, icons, etc.)
│   │
│   ├── whitepaper/                        ← ËTRID Ivory Papers v2.0
│   │   ├── index.html                     ← Whitepaper hub page
│   │   ├── ivory-paper.md                 ← Complete specification (43KB)
│   │   ├── ivory-paper-vol1-conceptual.md ← Volume I: Conceptual (16KB)
│   │   ├── ivory-paper-vol2-technical.md  ← Volume II: Technical (25KB)
│   │   └── ivory-paper-vol3-governance.md ← Volume III: Governance (32KB)
│   │
│   ├── validator/                         ← Validator Dashboard subdomain
│   │   ├── index.html                     ← Dashboard app entry
│   │   ├── .htaccess                      ← Subdomain Apache config
│   │   ├── _next/                         ← Next.js static files
│   │   │   ├── static/
│   │   │   └── (chunks, css, js)
│   │   └── (other Next.js exported files)
│   │
│   ├── watchtower/                        ← Network Monitor subdomain
│   │   ├── index.html                     ← Monitor app entry
│   │   ├── .htaccess                      ← Subdomain Apache config
│   │   ├── _next/                         ← Next.js static files
│   │   │   ├── static/
│   │   │   └── (chunks, css, js)
│   │   └── (other Next.js exported files)
│   │
│   ├── masterchef/                        ← Staking Dashboard subdomain
│   │   ├── index.html                     ← Staking app entry
│   │   ├── .htaccess                      ← Subdomain Apache config
│   │   ├── _next/                         ← Next.js static files
│   │   │   ├── static/
│   │   │   └── (chunks, css, js)
│   │   └── (other Next.js exported files)
│   │
│   ├── wallet/                            ← Web Wallet subdomain
│   │   ├── index.html                     ← Wallet app entry
│   │   ├── .htaccess                      ← Subdomain Apache config
│   │   ├── governance/                    ← Governance sub-route
│   │   │   └── index.html                 ← Consensus Day interface
│   │   ├── swap/                          ← Swap sub-route
│   │   │   └── index.html                 ← DEX interface
│   │   ├── _next/                         ← Next.js static files
│   │   │   ├── static/
│   │   │   └── (chunks, css, js)
│   │   └── (other Next.js exported files)
│   │
│   └── docs/                              ← Documentation Hub subdomain
│       ├── index.html                     ← Docs landing page
│       └── .htaccess                      ← Subdomain Apache config
│
└── logs/                                  ← Server logs (managed by Hostinger)
    ├── access_log
    └── error_log
```

---

## 🌐 **Subdomain → Directory Mapping**

### **In Hostinger Control Panel:**

| Subdomain | Maps To Directory | URL |
|-----------|------------------|-----|
| `etrid.org` (main) | `/home/u724092535/domains/etrid.org/public_html/` | https://etrid.org |
| `validator.etrid.org` | `/home/u724092535/domains/etrid.org/public_html/validator/` | https://validator.etrid.org |
| `watchtower.etrid.org` | `/home/u724092535/domains/etrid.org/public_html/watchtower/` | https://watchtower.etrid.org |
| `masterchef.etrid.org` | `/home/u724092535/domains/etrid.org/public_html/masterchef/` | https://masterchef.etrid.org |
| `wallet.etrid.org` | `/home/u724092535/domains/etrid.org/public_html/wallet/` | https://wallet.etrid.org |
| `docs.etrid.org` | `/home/u724092535/domains/etrid.org/public_html/docs/` | https://docs.etrid.org |

---

## 📦 **Zip Files → Extraction Locations**

### **What Goes Where:**

| Zip File | Size | Extract To | Creates |
|----------|------|-----------|---------|
| `website.zip` | ~50 KB | `/public_html/` | Main site files (index.html, css/, js/, whitepaper/, etc.) |
| `validator.zip` | 630 KB | `/public_html/validator/` | Validator dashboard files (index.html, _next/, .htaccess) |
| `watchtower.zip` | 731 KB | `/public_html/watchtower/` | Network monitor files (index.html, _next/, .htaccess) |
| `masterchef.zip` | 310 KB | `/public_html/masterchef/` | Staking dashboard files (index.html, _next/, .htaccess) |
| `wallet.zip` | 925 KB | `/public_html/wallet/` | Wallet app files (index.html, governance/, swap/, _next/, .htaccess) |
| `docs.zip` | 3 KB | `/public_html/docs/` | Documentation hub files (index.html, .htaccess) |

---

## 📋 **Detailed Upload Instructions**

### **Step 1: Main Domain (etrid.org)**

**Upload Location:** `/public_html/`

**Process:**
1. Navigate to `/public_html/` in File Manager
2. Upload `website.zip`
3. Right-click → Extract
4. Delete `website.zip` after extraction
5. Verify these files exist:
   - `/public_html/index.html`
   - `/public_html/css/styles.css`
   - `/public_html/js/main.js`
   - `/public_html/whitepaper/index.html`
   - `/public_html/.htaccess`

**Test:** https://etrid.org

---

### **Step 2: Validator Subdomain (validator.etrid.org)**

**Prerequisites:**
- Create subdomain in Hostinger: `validator.etrid.org` → `/public_html/validator/`
- Enable SSL for validator.etrid.org

**Upload Location:** `/public_html/validator/`

**Process:**
1. Create folder: `/public_html/validator/` (if doesn't exist)
2. Navigate to `/public_html/validator/`
3. Upload `validator.zip`
4. Right-click → Extract
5. Delete `validator.zip` after extraction
6. Verify these files exist:
   - `/public_html/validator/index.html`
   - `/public_html/validator/.htaccess`
   - `/public_html/validator/_next/`

**Test:** https://validator.etrid.org

---

### **Step 3: Watchtower Subdomain (watchtower.etrid.org)**

**Prerequisites:**
- Create subdomain in Hostinger: `watchtower.etrid.org` → `/public_html/watchtower/`
- Enable SSL for watchtower.etrid.org

**Upload Location:** `/public_html/watchtower/`

**Process:**
1. Create folder: `/public_html/watchtower/` (if doesn't exist)
2. Navigate to `/public_html/watchtower/`
3. Upload `watchtower.zip`
4. Right-click → Extract
5. Delete `watchtower.zip` after extraction
6. Verify these files exist:
   - `/public_html/watchtower/index.html`
   - `/public_html/watchtower/.htaccess`
   - `/public_html/watchtower/_next/`

**Test:** https://watchtower.etrid.org

---

### **Step 4: MasterChef Subdomain (masterchef.etrid.org)**

**Prerequisites:**
- Create subdomain in Hostinger: `masterchef.etrid.org` → `/public_html/masterchef/`
- Enable SSL for masterchef.etrid.org

**Upload Location:** `/public_html/masterchef/`

**Process:**
1. Create folder: `/public_html/masterchef/` (if doesn't exist)
2. Navigate to `/public_html/masterchef/`
3. Upload `masterchef.zip`
4. Right-click → Extract
5. Delete `masterchef.zip` after extraction
6. Verify these files exist:
   - `/public_html/masterchef/index.html`
   - `/public_html/masterchef/.htaccess`
   - `/public_html/masterchef/_next/`

**Test:** https://masterchef.etrid.org

---

### **Step 5: Wallet Subdomain (wallet.etrid.org)**

**Prerequisites:**
- Create subdomain in Hostinger: `wallet.etrid.org` → `/public_html/wallet/`
- Enable SSL for wallet.etrid.org

**Upload Location:** `/public_html/wallet/`

**Process:**
1. Create folder: `/public_html/wallet/` (if doesn't exist)
2. Navigate to `/public_html/wallet/`
3. Upload `wallet.zip`
4. Right-click → Extract
5. Delete `wallet.zip` after extraction
6. Verify these files exist:
   - `/public_html/wallet/index.html`
   - `/public_html/wallet/.htaccess`
   - `/public_html/wallet/governance/index.html`
   - `/public_html/wallet/swap/index.html`
   - `/public_html/wallet/_next/`

**Test:**
- https://wallet.etrid.org
- https://wallet.etrid.org/governance
- https://wallet.etrid.org/swap

---

### **Step 6: Docs Subdomain (docs.etrid.org)**

**Prerequisites:**
- Create subdomain in Hostinger: `docs.etrid.org` → `/public_html/docs/`
- Enable SSL for docs.etrid.org

**Upload Location:** `/public_html/docs/`

**Process:**
1. Create folder: `/public_html/docs/` (if doesn't exist)
2. Navigate to `/public_html/docs/`
3. Upload `docs.zip`
4. Right-click → Extract
5. Delete `docs.zip` after extraction
6. Verify these files exist:
   - `/public_html/docs/index.html`
   - `/public_html/docs/.htaccess`

**Test:** https://docs.etrid.org

---

## 🔐 **File Permissions**

**After extraction, set these permissions:**

### **Folders:**
```
chmod 755 /public_html/
chmod 755 /public_html/css/
chmod 755 /public_html/js/
chmod 755 /public_html/images/
chmod 755 /public_html/whitepaper/
chmod 755 /public_html/validator/
chmod 755 /public_html/watchtower/
chmod 755 /public_html/masterchef/
chmod 755 /public_html/wallet/
chmod 755 /public_html/wallet/governance/
chmod 755 /public_html/wallet/swap/
chmod 755 /public_html/docs/
```

### **Files:**
```
chmod 644 /public_html/index.html
chmod 644 /public_html/.htaccess
chmod 644 /public_html/css/*.css
chmod 644 /public_html/js/*.js
chmod 644 /public_html/whitepaper/*.html
chmod 644 /public_html/whitepaper/*.md
chmod 644 /public_html/validator/index.html
chmod 644 /public_html/validator/.htaccess
chmod 644 /public_html/watchtower/index.html
chmod 644 /public_html/watchtower/.htaccess
chmod 644 /public_html/masterchef/index.html
chmod 644 /public_html/masterchef/.htaccess
chmod 644 /public_html/wallet/index.html
chmod 644 /public_html/wallet/.htaccess
chmod 644 /public_html/docs/index.html
chmod 644 /public_html/docs/.htaccess
```

**Or use Hostinger File Manager:**
- Right-click folder → Change Permissions → Set to 755
- Right-click file → Change Permissions → Set to 644

---

## 🔗 **URL Structure Map**

### **Main Domain:**
```
https://etrid.org/                          → /public_html/index.html
https://etrid.org/whitepaper/               → /public_html/whitepaper/index.html
https://etrid.org/whitepaper/ivory-paper.md → /public_html/whitepaper/ivory-paper.md
https://etrid.org/#features                 → /public_html/index.html#features
https://etrid.org/#governance               → /public_html/index.html#governance
```

### **Validator Subdomain:**
```
https://validator.etrid.org/                → /public_html/validator/index.html
```

### **Watchtower Subdomain:**
```
https://watchtower.etrid.org/               → /public_html/watchtower/index.html
```

### **MasterChef Subdomain:**
```
https://masterchef.etrid.org/               → /public_html/masterchef/index.html
```

### **Wallet Subdomain:**
```
https://wallet.etrid.org/                   → /public_html/wallet/index.html
https://wallet.etrid.org/governance         → /public_html/wallet/governance/index.html
https://wallet.etrid.org/swap               → /public_html/wallet/swap/index.html
```

### **Docs Subdomain:**
```
https://docs.etrid.org/                     → /public_html/docs/index.html
```

---

## 📊 **File Size Summary**

```
Total size of all deployments: ~2.7 MB

/public_html/                    ~50 KB
├── Main site                    ~20 KB
└── whitepaper/                  ~30 KB (markdown files)

/public_html/validator/          630 KB
/public_html/watchtower/         731 KB
/public_html/masterchef/         310 KB
/public_html/wallet/             925 KB
/public_html/docs/               3 KB
```

---

## 🛠️ **Hostinger Control Panel Steps**

### **Creating Subdomains:**

1. Log into Hostinger control panel
2. Navigate to **Domains** → **Subdomains**
3. Click **Create Subdomain**
4. For each subdomain:

**Validator:**
- Subdomain: `validator`
- Parent domain: `etrid.org`
- Document root: `/public_html/validator`
- Click **Create**

**Watchtower:**
- Subdomain: `watchtower`
- Parent domain: `etrid.org`
- Document root: `/public_html/watchtower`
- Click **Create**

**MasterChef:**
- Subdomain: `masterchef`
- Parent domain: `etrid.org`
- Document root: `/public_html/masterchef`
- Click **Create**

**Wallet:**
- Subdomain: `wallet`
- Parent domain: `etrid.org`
- Document root: `/public_html/wallet`
- Click **Create**

**Docs:**
- Subdomain: `docs`
- Parent domain: `etrid.org`
- Document root: `/public_html/docs`
- Click **Create**

5. Enable SSL for each subdomain:
   - Go to **SSL** section
   - Enable SSL for each subdomain
   - Wait for SSL certificate generation (1-5 minutes)

---

## ✅ **Verification Checklist**

After completing all uploads, verify each URL loads:

### **Main Domain:**
- [ ] https://etrid.org
- [ ] https://etrid.org/whitepaper/
- [ ] https://etrid.org/#governance

### **Subdomains:**
- [ ] https://validator.etrid.org
- [ ] https://watchtower.etrid.org
- [ ] https://masterchef.etrid.org
- [ ] https://wallet.etrid.org
- [ ] https://wallet.etrid.org/governance
- [ ] https://wallet.etrid.org/swap
- [ ] https://docs.etrid.org

### **SSL Certificates:**
- [ ] All domains show padlock icon (SSL enabled)
- [ ] No certificate warnings

### **Navigation:**
- [ ] All links on main site work
- [ ] Footer links work
- [ ] No 404 errors
- [ ] All 5 app cards navigate correctly

---

## 🚨 **Common Issues & Fixes**

### **403 Forbidden Error:**
**Cause:** Wrong permissions
**Fix:**
```bash
# Folders: 755
# Files: 644
```

### **404 Not Found:**
**Cause:** Subdomain not mapped correctly
**Fix:** Check subdomain document root in Hostinger

### **Blank Page:**
**Cause:** Missing index.html
**Fix:** Verify index.html exists in correct directory

### **CSS/JS Not Loading:**
**Cause:** Permission issues or wrong paths
**Fix:** Check .htaccess and file permissions

---

## 📁 **Quick Reference**

```
Base Path: /home/u724092535/domains/etrid.org/public_html/

Main Site:        /public_html/                → etrid.org
Validator:        /public_html/validator/      → validator.etrid.org
Watchtower:       /public_html/watchtower/     → watchtower.etrid.org
MasterChef:       /public_html/masterchef/     → masterchef.etrid.org
Wallet:           /public_html/wallet/         → wallet.etrid.org
Docs:             /public_html/docs/           → docs.etrid.org
```

---

**Your complete ËTRID ecosystem architecture!** 🚀
