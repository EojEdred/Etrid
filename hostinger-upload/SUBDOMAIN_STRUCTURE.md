# ËTRID Comprehensive Subdomain Structure

## 🗺️ Current vs Proposed Structure

### **What You Have Now:**

```
etrid.org                     → Main landing page ✅
├── validator.etrid.org       → Validator dashboard ✅
├── watchtower.etrid.org      → Network monitoring ✅
├── masterchef.etrid.org      → Staking dashboard ✅
└── wallet.etrid.org          → Wallet homepage
    ├── /governance           → Consensus Day voting
    └── /swap                 → DEX interface
```

### **Issues:**

1. ❌ **docs.etrid.org** → Doesn't exist (links to nowhere)
2. ❌ **explorer.etrid.org** → Doesn't exist (confused with watchtower)
3. ❌ **Governance nested under wallet** → Should be top-level
4. ❌ **Wallet looks like main site** → Has its own branding/header
5. ❌ **Whitepaper links** → No whitepaper page/PDF
6. ❌ **Launch App buttons** → Unlinked

---

## ✅ **Recommended Structure**

```
etrid.org                     → Main landing/marketing site
├── docs.etrid.org            → Documentation hub
├── gov.etrid.org             → Governance (Consensus Day)
├── explorer.etrid.org        → Blockchain explorer (NEW)
├── validator.etrid.org       → Validator dashboard
├── watchtower.etrid.org      → Network monitoring/alerts
├── masterchef.etrid.org      → Staking/farming dashboard
├── wallet.etrid.org          → Web wallet
│   └── /swap                 → DEX/swap (can stay nested)
└── whitepaper.pdf            → At /whitepaper.pdf on main site
```

---

## 📋 **What Each Subdomain Should Be**

### **1. etrid.org** (Main Site)
**Purpose**: Marketing, information, links to ecosystem
**Content**:
- Hero section with mission
- Features overview
- Technology explanation
- Governance section (overview, links to gov.etrid.org)
- Apps ecosystem (cards linking to all subdomains)
- Community links
- Footer with all proper links

**Current Status**: ✅ Working (but links need fixing)

---

### **2. docs.etrid.org** (Documentation Hub)
**Purpose**: Technical documentation, guides, API references
**Content**:
- Getting started guides
- Runtime/pallet documentation
- Governance explanation
- Validator setup
- Developer API docs
- Integration guides
- FAQs

**Current Status**: ❌ Doesn't exist
**Solution**: Create simple docs page or redirect to GitHub wiki

---

### **3. gov.etrid.org** (Governance Portal)
**Purpose**: On-chain governance and Consensus Day voting
**Content**:
- Consensus Day countdown
- Active proposals list
- Voting interface
- Historical votes
- Governance documentation links
- ASF voting power calculator

**Current Status**: ⚠️ Currently at wallet.etrid.org/governance
**Solution**: Move to own subdomain (gov.etrid.org)

---

### **4. explorer.etrid.org** (Block Explorer)
**Purpose**: Blockchain data explorer
**Content**:
- Block search
- Transaction lookup
- Account/address viewer
- Extrinsic details
- Network stats

**Current Status**: ❌ Doesn't exist
**Options**:
- A) Use Polkadot.js Apps as explorer
- B) Build custom explorer (future)
- C) Rename watchtower to "Explorer & Monitor"
- D) Remove explorer links for now

---

### **5. validator.etrid.org** (Validator Dashboard)
**Purpose**: Validator node monitoring and management
**Current Status**: ✅ Working
**Keep**: As is

---

### **6. watchtower.etrid.org** (Network Monitor)
**Purpose**: Network health monitoring, alerts, reports
**Current Status**: ✅ Working
**Note**: This is NOT a block explorer (it's monitoring/alerting)

---

### **7. masterchef.etrid.org** (Staking Dashboard)
**Purpose**: Staking, farming, liquidity management
**Current Status**: ✅ Working
**Keep**: As is

---

### **8. wallet.etrid.org** (Web Wallet)
**Purpose**: Send/receive ETR, manage accounts
**Content**:
- Wallet interface
- Send/receive
- Transaction history
- Account management
- /swap (DEX nested here is OK)

**Current Status**: ⚠️ Looks like main site (needs redesign)
**Solution**: Make it wallet-focused, not governance-focused

---

## 🔧 **Quick Fixes Needed**

###1. **Main Site (etrid.org)** - Update Links

Change:
```html
<!-- FROM: -->
<a href="https://docs.etrid.org">Docs</a>
<a href="https://explorer.etrid.org">Explorer</a>
<a href="https://wallet.etrid.org/governance">Governance</a>

<!-- TO: -->
<a href="#" onclick="alert('Coming soon!')">Docs</a>
<!-- OR -->
<a href="https://github.com/EojEdred/Etrid">Docs (GitHub)</a>

<a href="https://watchtower.etrid.org">Network Monitor</a>
<!-- Remove "Explorer" or add future placeholder -->

<a href="https://gov.etrid.org">Governance</a>
<!-- OR keep wallet.etrid.org/governance if you prefer -->
```

### 2. **Create gov.etrid.org Subdomain**

**Option A**: Point to wallet governance
```
gov.etrid.org → wallet.etrid.org/governance (redirect)
```

**Option B**: Copy governance files to own subdomain
```
/public_html/gov/
├── index.html (from wallet/governance.html)
├── _next/ (copy needed assets)
└── .htaccess
```

### 3. **Create docs.etrid.org**

**Option A**: Simple static docs page
```html
<!-- Quick documentation landing page -->
<html>
<head><title>ËTRID Documentation</title></head>
<body>
  <h1>ËTRID Documentation</h1>
  <ul>
    <li><a href="https://github.com/EojEdred/Etrid">GitHub Repository</a></li>
    <li><a href="https://github.com/EojEdred/Etrid/wiki">Wiki</a></li>
    <li><a href="/governance">Governance Guide</a></li>
    <li><a href="/validator">Validator Setup</a></li>
  </ul>
</body>
</html>
```

**Option B**: Redirect to GitHub
```apache
# In /public_html/docs/.htaccess
RewriteEngine On
RewriteRule ^(.*)$ https://github.com/EojEdred/Etrid [R=301,L]
```

### 4. **Whitepaper**

Add whitepaper PDF to:
```
/public_html/whitepaper.pdf
```

Or create placeholder:
```
/public_html/whitepaper/index.html
→ "Whitepaper coming soon"
```

---

## 🎯 **Immediate Action Plan**

### **Phase 1: Quick Fixes** (Do This Now)

1. **Update main site links:**
   - Change docs.etrid.org → GitHub or "Coming Soon"
   - Change explorer.etrid.org → watchtower.etrid.org or remove
   - Keep governance → wallet.etrid.org/governance (or add gov redirect)

2. **Create simple docs page:**
   - Single HTML page at docs.etrid.org
   - Links to GitHub, wiki, guides

3. **Add whitepaper placeholder:**
   - Create whitepaper.html with "Coming Soon"

### **Phase 2: Governance Restructure** (If Desired)

1. Create gov.etrid.org subdomain
2. Copy governance.html from wallet
3. Update all links to point to gov.etrid.org
4. Simplify wallet.etrid.org to be wallet-only

### **Phase 3: Block Explorer** (Future)

1. Deploy Polkadot.js Apps as explorer
2. OR build custom explorer
3. OR rename watchtower → "Explorer & Monitor"

---

## 📝 **Subdomain Mapping for Hostinger**

### **Currently Active:**
```
validator.etrid.org    → /public_html/validator/
watchtower.etrid.org   → /public_html/watchtower/
masterchef.etrid.org   → /public_html/masterchef/
wallet.etrid.org       → /public_html/wallet/
```

### **Need to Create:**
```
docs.etrid.org         → /public_html/docs/
gov.etrid.org          → /public_html/gov/ (optional)
explorer.etrid.org     → /public_html/explorer/ (future)
```

---

## 🔗 **Updated Link Structure for Main Site**

```html
<!-- Navigation -->
<nav>
  <a href="#features">Features</a>
  <a href="#technology">Technology</a>
  <a href="#governance">Governance</a>
  <a href="#apps">Apps</a>
  <a href="/docs">Docs</a> <!-- OR https://docs.etrid.org -->
  <a href="https://github.com/EojEdred/Etrid">GitHub</a>
</nav>

<!-- Apps Section -->
<section id="apps">
  <a href="https://validator.etrid.org">Validator Dashboard</a>
  <a href="https://wallet.etrid.org">Web Wallet</a>
  <a href="https://watchtower.etrid.org">Network Monitor</a>
  <a href="https://masterchef.etrid.org">Staking Dashboard</a>
  <a href="https://gov.etrid.org">Governance Portal</a>
  <a href="/docs">Documentation</a>
</section>

<!-- Governance Section -->
<section id="governance">
  <a href="https://gov.etrid.org">View Proposals</a>
  <!-- OR -->
  <a href="https://wallet.etrid.org/governance">View Proposals</a>
</section>

<!-- Footer -->
<footer>
  <a href="/whitepaper.pdf">Whitepaper</a>
  <a href="/docs">Documentation</a>
  <a href="https://github.com/EojEdred/Etrid">GitHub</a>
</footer>
```

---

## ✅ **Recommendation**

**For Now** (Easiest):
1. Update main site links to remove broken ones
2. Point docs → GitHub temporarily
3. Keep governance at wallet.etrid.org/governance
4. Rename "explorer" mentions to "network monitor"
5. Add whitepaper placeholder

**For Production** (Best):
1. Create proper docs.etrid.org site
2. Create gov.etrid.org (move governance out of wallet)
3. Build/deploy block explorer
4. Publish whitepaper PDF
5. Clean up wallet.etrid.org to be wallet-only

---

**Which approach do you prefer?**
