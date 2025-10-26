# Ëtrid Protocol - Vercel Deployment Quick Start

**Quick reference for deploying all UI apps to Vercel**

---

## 1. Authenticate (First Time Only)

```bash
vercel login
```

Choose GitHub/GitLab/Bitbucket/Email and authorize in browser.

---

## 2. Deploy Applications

### Option A: Using Deployment Script (Recommended)

```bash
# Deploy all apps to staging
./scripts/deploy-ui.sh

# Deploy all to production
./scripts/deploy-ui.sh all --production

# Deploy specific app
./scripts/deploy-ui.sh wallet-web --production
```

### Option B: Manual Deployment

**Wallet Web (Production):**
```bash
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
vercel --prod
```

**Validator Dashboard (Staging):**
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
vercel
```

**Watchtower Monitor (Staging - Fix Required First):**
```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
# Fix Tailwind CSS issue first (see main report)
vercel
```

---

## 3. Verify Deployments

Visit the URLs provided by Vercel after deployment and test:
- Page loads
- RPC connection (WebSocket to wss://rpc.etrid.network)
- Core functionality

---

## Troubleshooting

**Build failed for Watchtower Monitor?**
```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
npm remove @tailwindcss/postcss tailwindcss
npm install --save-dev tailwindcss@^3 autoprefixer
rm -rf .next
npm run build
vercel
```

---

## Full Documentation

- **Comprehensive Guide:** `/Users/macbook/Desktop/etrid/docs/VERCEL_DEPLOYMENT_GUIDE.md`
- **Detailed Report:** `/Users/macbook/Desktop/etrid/VERCEL_DEPLOYMENT_REPORT.md`
- **Deployment Script:** `/Users/macbook/Desktop/etrid/scripts/deploy-ui.sh`

---

## Quick Status

| App | Status | Command |
|-----|--------|---------|
| Wallet Web | ✅ Ready | `vercel --prod` |
| Validator Dashboard | ✅ Ready | `vercel` |
| Watchtower Monitor | ⚠️ Fix First | See troubleshooting |

---

**That's it!** Your apps will be live on Vercel in minutes.
