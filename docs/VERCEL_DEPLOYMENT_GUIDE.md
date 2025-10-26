# Ëtrid Protocol - Vercel Deployment Guide

**Date:** 2025-10-22
**Status:** Ready for Deployment
**Prepared by:** Claude Code Deployment Agent

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Build Status](#build-status)
4. [Deployment Steps](#deployment-steps)
5. [Environment Variables](#environment-variables)
6. [Post-Deployment Verification](#post-deployment-verification)
7. [Troubleshooting](#troubleshooting)
8. [Custom Domain Configuration](#custom-domain-configuration)

---

## Overview

This guide provides step-by-step instructions for deploying all Ëtrid Protocol UI applications to Vercel.

### Applications to Deploy

| Application | Path | Status | Deployment Type |
|------------|------|--------|----------------|
| **Wallet Web** | `apps/wallet-web/etrid-crypto-website/` | ✅ Build Passing | Production |
| **Validator Dashboard** | `apps/validator-dashboard/` | ✅ Build Passing | Staging |
| **Watchtower Monitor** | `apps/watchtower-monitor/` | ⚠️ Build Issues | Staging |

---

## Prerequisites

### 1. Vercel CLI Installation

Vercel CLI has been installed globally:

```bash
npm install -g vercel
# Installed version: 48.5.0
```

### 2. Vercel Account Authentication

You need to authenticate with Vercel before deploying. Run:

```bash
vercel login
```

Choose your preferred authentication method:
- GitHub
- GitLab
- Bitbucket
- Email

The CLI will open your browser for authentication.

---

## Build Status

### ✅ Wallet Web (READY)

**Path:** `/Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website/`

**Build Command:**
```bash
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
npm run build
```

**Build Output:**
```
✓ Compiled successfully
✓ Generating static pages (6/6)
✓ Finalizing page optimization

Route (app)                                 Size  First Load JS
┌ ○ /                                    5.66 kB         118 kB
├ ○ /_not-found                            988 B         102 kB
├ ○ /governance                            17 kB         513 kB
└ ○ /swap                                 105 kB         602 kB
```

**Configuration:** `vercel.json` ✅
**Framework:** Next.js 15.2.4
**Region:** iad1 (US East)

---

### ✅ Validator Dashboard (READY)

**Path:** `/Users/macbook/Desktop/etrid/apps/validator-dashboard/`

**Build Command:**
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
npm run build
```

**Build Output:**
```
✓ Linting and checking validity of types
✓ Compiled successfully
✓ Build completed successfully
```

**Configuration:** `vercel.json` ✅
**Framework:** Next.js 14.0.4
**Region:** iad1 (US East)

---

### ⚠️ Watchtower Monitor (BUILD ISSUES)

**Path:** `/Users/macbook/Desktop/etrid/apps/watchtower-monitor/`

**Issue:** Tailwind CSS configuration incompatibility

**Error:**
```
Error: Cannot apply unknown utility class `border-border`.
Are you using CSS modules or similar and missing `@reference`?
```

**Root Cause:** The project uses Tailwind CSS v4 (`@tailwindcss/postcss`) but the CSS file uses v3 syntax.

**Fix Required:** Update `src/app/globals.css` to use Tailwind v4 syntax (see Troubleshooting section).

**Temporary Solution:** Deploy with v3 PostCSS config until the CSS is migrated to v4 syntax.

---

## Deployment Steps

### Using the Deployment Script (Recommended)

The project includes a deployment script at `scripts/deploy-ui.sh`:

```bash
# Deploy all applications (preview)
./scripts/deploy-ui.sh

# Deploy all to production
./scripts/deploy-ui.sh all --production

# Deploy specific app
./scripts/deploy-ui.sh wallet-web --production
./scripts/deploy-ui.sh validator
./scripts/deploy-ui.sh watchtower
```

### Manual Deployment

#### 1. Authenticate with Vercel

```bash
vercel login
```

#### 2. Deploy Wallet Web (Production)

```bash
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website

# Deploy to production
vercel --prod

# Follow the prompts:
# - Set up and deploy? Yes
# - Which scope? (Select your account)
# - Link to existing project? No
# - Project name? etrid-wallet-web
# - Directory? ./
# - Override settings? No
```

**Expected Output:**
```
✓ Production: https://etrid-wallet-web.vercel.app [Deployed]
```

#### 3. Deploy Validator Dashboard (Staging)

```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard

# Deploy to staging (preview)
vercel

# Follow the prompts:
# - Set up and deploy? Yes
# - Which scope? (Select your account)
# - Link to existing project? No
# - Project name? etrid-validator-dashboard
# - Directory? ./
# - Override settings? No
```

**Expected Output:**
```
✓ Preview: https://etrid-validator-dashboard-[unique-id].vercel.app [Deployed]
```

#### 4. Deploy Watchtower Monitor (Staging)

**Note:** Fix the build issues first or use the temporary fix below.

**Temporary Fix:**
```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor

# Downgrade to Tailwind v3 PostCSS (temporary)
npm install --save-dev postcss autoprefixer tailwindcss@^3

# Update postcss.config.mjs
cat > postcss.config.mjs << 'EOF'
const config = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};
export default config;
EOF

# Clear cache and rebuild
rm -rf .next node_modules/.cache
npm run build

# Deploy
vercel
```

---

## Environment Variables

All applications are pre-configured with environment variables in their `vercel.json` files:

### Wallet Web

```json
{
  "NEXT_PUBLIC_WS_PROVIDER": "wss://rpc.etrid.network",
  "NEXT_PUBLIC_NETWORK_NAME": "Ëtrid MainNet",
  "NEXT_PUBLIC_CHAIN_ID": "etrid-mainnet",
  "NODE_ENV": "production"
}
```

### Validator Dashboard

```json
{
  "NEXT_PUBLIC_WS_PROVIDER": "wss://rpc.etrid.network",
  "NEXT_PUBLIC_NETWORK_NAME": "Ëtrid MainNet",
  "NEXT_PUBLIC_CHAIN_ID": "etrid-mainnet",
  "NEXT_PUBLIC_APP_NAME": "Validator Dashboard",
  "NODE_ENV": "production"
}
```

### Watchtower Monitor

```json
{
  "NEXT_PUBLIC_WS_PROVIDER": "wss://rpc.etrid.network",
  "NEXT_PUBLIC_NETWORK_NAME": "Ëtrid MainNet",
  "NEXT_PUBLIC_CHAIN_ID": "etrid-mainnet",
  "NEXT_PUBLIC_APP_NAME": "Watchtower Monitor",
  "NODE_ENV": "production"
}
```

**Note:** These environment variables can be overridden in the Vercel Dashboard after deployment.

---

## Post-Deployment Verification

### 1. Check Deployment URLs

After each deployment, Vercel will provide URLs. Save them:

```bash
# Example deployment URLs
Wallet Web:           https://etrid-wallet-web.vercel.app
Validator Dashboard:  https://etrid-validator-dashboard.vercel.app
Watchtower Monitor:   https://etrid-watchtower-monitor.vercel.app
```

### 2. Verify Functionality

For each deployed application:

1. **Visit the URL** - Ensure the app loads
2. **Check RPC Connection** - Open browser console, verify WebSocket connection to `wss://rpc.etrid.network`
3. **Test Core Features:**
   - Wallet Web: Connect wallet, view balance
   - Validator Dashboard: View validator stats, nominator list
   - Watchtower Monitor: View channel monitoring, fraud detection

### 3. Check Build Logs

View deployment logs in the Vercel Dashboard:
```
https://vercel.com/[your-account]/[project-name]/deployments
```

### 4. Test Performance

Use Vercel Analytics to monitor:
- Page load times
- Core Web Vitals
- Error rates

---

## Troubleshooting

### Issue: "No credentials found"

**Solution:**
```bash
vercel login
```

### Issue: "Build failed" for Watchtower Monitor

**Cause:** Tailwind CSS v3 syntax with v4 package

**Solution 1: Migrate to Tailwind v4 (Recommended)**

Update `apps/watchtower-monitor/src/app/globals.css`:

```css
/* Replace old syntax */
@tailwind base;
@tailwind components;
@tailwind utilities;

/* With new syntax */
@import "tailwindcss";
```

Remove `@apply` directives and use utility classes directly in components.

**Solution 2: Use Tailwind v3 (Temporary)**

```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
npm remove @tailwindcss/postcss tailwindcss
npm install --save-dev tailwindcss@^3 autoprefixer
```

### Issue: "Invalid vercel.json options"

**Solution:** Ensure `vercel.json` follows the schema:
```bash
# Validate JSON syntax
cat vercel.json | jq .
```

### Issue: Environment variables not working

**Solution:** Set them manually in Vercel Dashboard:
1. Go to Project Settings
2. Navigate to "Environment Variables"
3. Add variables for Production/Preview/Development

### Issue: Build timeout

**Solution:** Increase function timeout in `vercel.json`:
```json
{
  "functions": {
    "app/**/*.{ts,tsx}": {
      "maxDuration": 60
    }
  }
}
```

---

## Custom Domain Configuration

### Add Custom Domains (Optional)

After deployment, you can add custom domains:

#### Via Vercel CLI

```bash
# Add domain to wallet-web
vercel domains add wallet.etrid.network --project=etrid-wallet-web

# Add domain to validator dashboard
vercel domains add validators.etrid.network --project=etrid-validator-dashboard

# Add domain to watchtower
vercel domains add watchtower.etrid.network --project=etrid-watchtower-monitor
```

#### Via Vercel Dashboard

1. Go to Project Settings > Domains
2. Click "Add Domain"
3. Enter your custom domain
4. Configure DNS records:

```
Type: CNAME
Name: wallet (or @)
Value: cname.vercel-dns.com
```

#### DNS Configuration

Add these DNS records to your domain provider:

**Wallet Web:**
```
CNAME  wallet      cname.vercel-dns.com
```

**Validator Dashboard:**
```
CNAME  validators  cname.vercel-dns.com
```

**Watchtower Monitor:**
```
CNAME  watchtower  cname.vercel-dns.com
```

### Enable HTTPS

Vercel automatically provisions SSL certificates for:
- `*.vercel.app` domains (immediate)
- Custom domains (within minutes of DNS propagation)

---

## Deployment Checklist

Use this checklist to ensure successful deployment:

### Pre-Deployment
- [x] Vercel CLI installed (v48.5.0)
- [ ] Authenticated with Vercel
- [x] Wallet Web build passing
- [x] Validator Dashboard build passing
- [ ] Watchtower Monitor build issues resolved

### Deployment
- [ ] Wallet Web deployed to production
- [ ] Validator Dashboard deployed to staging
- [ ] Watchtower Monitor deployed to staging
- [ ] All deployment URLs documented

### Post-Deployment
- [ ] All apps accessible via URLs
- [ ] RPC connections working
- [ ] Core functionality tested
- [ ] No console errors
- [ ] Performance metrics acceptable

### Optional
- [ ] Custom domains configured
- [ ] DNS records updated
- [ ] SSL certificates provisioned
- [ ] Analytics enabled
- [ ] Error tracking configured (Sentry, etc.)

---

## Quick Reference Commands

```bash
# Login
vercel login

# Deploy preview (staging)
cd [app-directory]
vercel

# Deploy production
cd [app-directory]
vercel --prod

# Check deployment status
vercel ls

# View deployment logs
vercel logs [deployment-url]

# Inspect project
vercel inspect [deployment-url]

# Remove deployment
vercel rm [deployment-name]

# List domains
vercel domains ls

# Add domain
vercel domains add [domain] --project=[project-name]
```

---

## Support and Resources

### Vercel Documentation
- [Deployment Guide](https://vercel.com/docs/deployments/overview)
- [CLI Reference](https://vercel.com/docs/cli)
- [Environment Variables](https://vercel.com/docs/environment-variables)
- [Custom Domains](https://vercel.com/docs/custom-domains)

### Ëtrid Protocol Resources
- [Architecture Docs](/docs/architecture.md)
- [Developer Guide](/docs/DEVELOPER_GUIDE.md)
- [API Reference](/docs/API_REFERENCE.md)

### Getting Help
- Vercel Support: https://vercel.com/support
- Ëtrid Protocol: [Your support channel]

---

## Next Steps

After successful deployment:

1. **Monitor Performance** - Set up Vercel Analytics
2. **Configure Monitoring** - Integrate error tracking (Sentry, Rollbar)
3. **Set Up CI/CD** - Automate deployments via GitHub Actions
4. **Security Audit** - Review environment variables, API keys
5. **Load Testing** - Test apps under load
6. **Documentation** - Update user guides with production URLs
7. **Communication** - Announce deployments to users/team

---

**Document Version:** 1.0
**Last Updated:** 2025-10-22
**Author:** Claude Code Deployment Agent
