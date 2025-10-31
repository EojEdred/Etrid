# How to Upload ËTRID Apps to Hostinger

## Quick Answer

Your apps in the `apps/` folder are **source code** (like ingredients), not ready-to-serve websites (like a cooked meal). You need to **build** them first.

---

## Problem

- **What you have:** Next.js/React apps (source code in `apps/` folder)
- **What Hostinger needs:** Built HTML/CSS/JS files
- **Why drag-and-drop won't work:** Hostinger can't run your build process

---

## Solution: Two Options

### Option 1: Use Vercel (EASIEST - Recommended)

Vercel is made by the Next.js team and handles everything automatically:

**Steps:**
```bash
# 1. Install Vercel CLI
npm install -g vercel

# 2. Deploy Validator Dashboard
cd apps/validator-dashboard
vercel --prod

# 3. Vercel will give you a URL like: validator-dashboard.vercel.app
# 4. Point your domain validator.etrid.org to that URL in Hostinger DNS
```

**Repeat for each app:**
- Watchtower Monitor
- MasterChef Dashboard
- Wallet Web
- Governance UI

**Pros:**
- Automatic builds
- Free SSL
- Auto-updates when you push code
- No server management

---

### Option 2: Build Manually & Upload to Hostinger

If you want everything on Hostinger:

#### Step 1: Build Each App

```bash
# Build Validator Dashboard
cd apps/validator-dashboard
npm install
npm run build
# Creates .next/standalone folder

# Build Watchtower Monitor
cd ../watchtower-monitor
npm install
npm run build
# Creates .next/standalone folder

# Build MasterChef Dashboard
cd ../masterchef-dashboard
npm install
npm run build
# Creates .next/standalone folder
```

#### Step 2: Export to Static Files (if possible)

Some Next.js apps can export to static HTML:

```bash
# In each app directory
npm run build
npx next export
# Creates 'out/' folder with static files
```

**Problem:** Not all Next.js features work as static exports (like API routes, server-side rendering).

#### Step 3: Upload to Hostinger

**What to upload:** The `out/` folder contents (if you exported) OR use Node.js hosting

**Hostinger File Manager:**
1. Log into Hostinger
2. Go to File Manager
3. Navigate to `public_html/`
4. Upload contents of `out/` folder

---

## Best Solution for You

Since you have **5 Next.js apps**, here's what I recommend:

### Main Website (etrid.org)
→ Create a simple landing page HTML (I'll create this for you)
→ Upload directly to Hostinger

### Web Apps (subdomains)
→ Use Vercel for all apps
→ Point DNS from Hostinger to Vercel

**Why?**
- Next.js apps work better on Vercel
- Hostinger shared hosting can't run Node.js apps easily
- Vercel handles all the complexity
- You get automatic SSL, builds, and deployments

---

## Quick Start: Main Landing Page

I'll create a simple main website for etrid.org that you can upload directly to Hostinger.

**Location:** `/Users/macbook/Desktop/etrid/website/`

**Files to upload:**
```
website/
├── index.html          ← Drag this to Hostinger
├── css/
│   └── styles.css     ← Drag this folder
├── js/
│   └── main.js        ← Drag this folder
└── images/            ← Drag this folder
```

---

## Hostinger Upload Steps (for main website)

1. **Log into Hostinger**
   - Go to hpanel.hostinger.com
   - Find your etrid.org domain

2. **Open File Manager**
   - Click "File Manager"
   - Navigate to `public_html/` (or your domain folder)

3. **Upload Files**
   - Delete existing `index.html` if present
   - Drag the new `index.html` into the folder
   - Drag `css/`, `js/`, `images/` folders

4. **Test**
   - Visit https://etrid.org
   - Should see your new website

---

## For the Web Apps (Validator, Watchtower, etc.)

### Method A: Vercel (Recommended)

```bash
# Install Vercel CLI once
npm install -g vercel

# Deploy each app
cd apps/validator-dashboard && vercel --prod
cd ../watchtower-monitor && vercel --prod
cd ../masterchef-dashboard && vercel --prod
cd ../wallet-web && vercel --prod
cd ../governance-ui/etrid-snapshot && vercel --prod
```

### Method B: Build & Use Hostinger Node.js

If Hostinger has Node.js hosting:

```bash
# Build the app
cd apps/validator-dashboard
npm install
npm run build

# Upload these files via FTP/SFTP:
- .next/
- public/
- package.json
- node_modules/ (or run npm install on server)

# On Hostinger, run:
npm start
```

---

## DNS Configuration

### If using Vercel for apps:

In Hostinger DNS settings:

```
A       @                    → [Hostinger IP]          # Main site
CNAME   validator.etrid.org  → validator-xxx.vercel.app
CNAME   watchtower.etrid.org → watchtower-xxx.vercel.app
CNAME   masterchef.etrid.org → masterchef-xxx.vercel.app
CNAME   wallet.etrid.org     → wallet-xxx.vercel.app
CNAME   gov.etrid.org        → governance-xxx.vercel.app
```

---

## Summary

**For etrid.org (main landing page):**
✅ I'll create a simple HTML website you can drag-and-drop to Hostinger

**For web apps (validator.etrid.org, watchtower.etrid.org, etc.):**
✅ Use Vercel - it's designed for Next.js apps
✅ Much easier than Hostinger for dynamic apps
✅ Free tier is generous

**Next step:** Let me create the main landing page HTML for you!

---

**Questions?**
- **"Can I host everything on Hostinger?"**
  - Main site: Yes (simple HTML)
  - Apps: Possible with Node.js hosting, but Vercel is easier

- **"Why not drag the apps/ folder directly?"**
  - Those are source files, not built websites
  - Like trying to serve raw ingredients instead of cooked food

- **"How much does Vercel cost?"**
  - Free for personal/hobby projects
  - $20/month per member for Pro features (not needed initially)
