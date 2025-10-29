# Setting Up Governance as Separate Subdomain

## Current Problem

- `wallet.etrid.org` has governance as a sub-route (`/governance`)
- You want `governance.etrid.org` as a completely separate subdomain
- Wallet has dead links pointing to `/governance` instead of external subdomain

## Solution: Two Options

### **Option 1: Keep Governance Under Wallet (RECOMMENDED)**

**Current Structure:**
```
wallet.etrid.org/              → Wallet homepage
wallet.etrid.org/governance    → Governance interface
wallet.etrid.org/swap          → Swap interface
```

**This is the CORRECT architecture because:**
1. ✅ All wallet-related functions in one place
2. ✅ Single SSLcertificate
3. ✅ Easier navigation
4. ✅ No broken links
5. ✅ Already built and working

**To make this work on Hostinger:**

1. Upload `wallet.zip` to `/public_html/wallet/`
2. Extract - it will create:
   ```
   /public_html/wallet/
   ├── index.html
   ├── .htaccess
   ├── governance/
   │   └── index.html
   └── swap/
       └── index.html
   ```

3. Test URLs:
   - https://wallet.etrid.org → Works
   - https://wallet.etrid.org/governance → Works
   - https://wallet.etrid.org/swap → Works

**THIS IS ALREADY SET UP CORRECTLY!**

---

### **Option 2: Separate Governance Subdomain** ⚠️

**New Structure:**
```
wallet.etrid.org       → Wallet only
governance.etrid.org   → Governance interface (separate)
swap.etrid.org         → Swap interface (separate)
```

**If you want this, you need:**

1. **Create governance subdomain in Hostinger:**
   - Subdomain: `governance`
   - Document root: `/public_html/governance/`

2. **Create swap subdomain:**
   - Subdomain: `swap`
   - Document root: `/public_html/swap/`

3. **Create standalone packages** (I can do this)

4. **Update ALL links across site:**
   - Main site → Change governance links to `https://governance.etrid.org`
   - Wallet → Change governance button to `https://governance.etrid.org`
   - Wallet → Change swap button to `https://swap.etrid.org`

---

## Which Option Do You Want?

### If Option 1 (RECOMMENDED):
- ✅ Nothing to change
- ✅ Just upload wallet.zip as-is
- ✅ Structure already correct

### If Option 2:
- I'll create 3 separate packages:
  - `wallet-standalone.zip` (no governance/swap)
  - `governance-standalone.zip`
  - `swap-standalone.zip`
- Update all links everywhere
- Create 2 new subdomains

---

## Current Issues

### 1. Ivory Papers Not Showing

**Problem:** You uploaded website.zip but `/whitepaper/` doesn't show

**Possible Causes:**
1. Didn't extract the zip
2. Extracted to wrong location
3. Permission issues
4. Missing index.html

**Solution:**

Check in Hostinger File Manager:
```
/public_html/whitepaper/
├── index.html           ← Must exist
├── ivory-paper.md
├── ivory-paper-vol1-conceptual.md
├── ivory-paper-vol2-technical.md
└── ivory-paper-vol3-governance.md
```

**Steps:**
1. Navigate to `/public_html/`
2. Delete old `whitepaper.html` if it exists
3. Check if `whitepaper/` folder exists
4. If not, upload `website.zip` again
5. Extract it
6. Set permissions: 755 for folders, 644 for files
7. Test: https://etrid.org/whitepaper/

---

## Quick Decision Guide

**Tell me what you want:**

**A)** Keep governance/swap under wallet ✅ EASIEST
   - URL: `wallet.etrid.org/governance`
   - No changes needed

**B)** Separate governance subdomain 🔨 MORE WORK
   - URL: `governance.etrid.org`
   - Requires updates everywhere

---

## If You Choose Option B (Separate Subdomains)

I will create:

### 1. governance.zip
- Standalone governance interface
- Maps to: `/public_html/governance/`
- URL: `https://governance.etrid.org`

### 2. swap.zip
- Standalone swap interface
- Maps to: `/public_html/swap/`
- URL: `https://swap.etrid.org`

### 3. wallet-updated.zip
- Wallet without governance/swap pages
- Updated links point to external subdomains
- Maps to: `/public_html/wallet/`
- URL: `https://wallet.etrid.org`

### 4. website-updated.zip
- Main site with updated governance links
- Links point to `governance.etrid.org`
- Maps to: `/public_html/`

---

## Recommendation

**KEEP CURRENT STRUCTURE (Option A)**

Why:
- ✅ It's the standard architecture (like Uniswap, Aave, etc.)
- ✅ wallet.etrid.org/governance is professional and clear
- ✅ Everything wallet-related in one place
- ✅ No broken links to fix
- ✅ Easier for users to navigate

**Only use Option B if you have a specific reason like:**
- Different teams managing governance vs wallet
- Want to deploy them independently
- Security isolation requirements

---

## Let me know which option you want!
