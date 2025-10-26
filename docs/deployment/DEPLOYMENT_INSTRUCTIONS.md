# 🚀 Ëtrid Protocol UI - Deployment Instructions

**All apps are built and ready to deploy!**

---

## ✅ Pre-Deployment Status

All three apps have been verified and are building successfully:

| Application | Status | Bundle Size | Configuration |
|-------------|--------|-------------|---------------|
| Wallet-Web | ✅ PASSING | 119 KB | Production ready |
| Watchtower Monitor | ✅ PASSING | 216 KB | Staging ready |
| Validator Dashboard | ✅ PASSING | 548 KB | Staging ready |

All `vercel.json` configuration files have been created with:
- Security headers (CSP, XSS, Frame Options)
- Environment variables
- Build commands
- Function timeouts (60s for server functions)

---

## 📋 Deployment Steps

### Step 1: Authenticate with Vercel

Open your terminal and run:
```bash
vercel login
```

Follow the browser prompt to authenticate. Once complete, verify with:
```bash
vercel whoami
```

---

### Step 2: Deploy All Apps (Automated)

Run the deployment script:
```bash
cd /Users/macbook/Desktop/etrid
./scripts/deploy-all-apps.sh
```

This will deploy all three apps in order:
1. Wallet-Web → Production
2. Watchtower Monitor → Staging
3. Validator Dashboard → Staging

**OR**

### Step 2: Deploy Apps Individually (Manual)

#### Deploy Wallet-Web (Production)
```bash
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
vercel --prod
```

Expected output:
- Deployment URL
- Production domain assignment
- Build logs

#### Deploy Watchtower Monitor (Staging)
```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
vercel
```

Expected output:
- Preview URL
- Build logs

#### Deploy Validator Dashboard (Staging)
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
vercel
```

Expected output:
- Preview URL
- Build logs

---

## 🔍 Post-Deployment Verification

### Wallet-Web Checklist:
- [ ] Homepage loads correctly
- [ ] Dark mode toggle button visible (top-right of hero)
- [ ] Dark mode switching works
- [ ] Governance page loads
- [ ] Swap page loads
- [ ] No console errors

### Watchtower Monitor Checklist:
- [ ] Dashboard loads
- [ ] WebSocket connection status shown
- [ ] Real-time updates working (or graceful fallback to demo mode)
- [ ] Settings page accessible
- [ ] Reports page accessible
- [ ] No console errors

### Validator Dashboard Checklist:
- [ ] Dashboard loads
- [ ] Attempts to connect to WebSocket (wss://rpc.etrid.network)
- [ ] Shows connection status
- [ ] Performance metrics display
- [ ] Nominators page accessible
- [ ] Rewards page accessible
- [ ] Settings page accessible
- [ ] No TypeScript errors

---

## 🔧 Environment Variables

All environment variables are configured in `vercel.json` files and will be automatically applied during deployment.

### Wallet-Web:
```bash
NEXT_PUBLIC_APP_NAME="Ëtrid Wallet"
NEXT_PUBLIC_NETWORK_NAME="Ëtrid MainNet"
NODE_ENV="production"
```

### Watchtower Monitor & Validator Dashboard:
```bash
NEXT_PUBLIC_WS_PROVIDER="wss://rpc.etrid.network"
NEXT_PUBLIC_NETWORK_NAME="Ëtrid MainNet"
NEXT_PUBLIC_CHAIN_ID="etrid-mainnet"
NODE_ENV="production"
```

---

## 🛡️ Security Headers

All apps are configured with:
- X-Content-Type-Options: nosniff
- X-Frame-Options: DENY
- X-XSS-Protection: 1; mode=block
- Referrer-Policy: strict-origin-when-cross-origin
- Permissions-Policy: camera=(), microphone=(), geolocation=()

Wallet-Web additionally includes:
- Content-Security-Policy with strict directives

---

## 🐛 Troubleshooting

### Build Fails During Deployment

**Watchtower Monitor:**
If build fails with Tailwind CSS error, run locally:
```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
rm -rf .next
npm run build
```

If successful, try deploying again.

**Validator Dashboard:**
ESLint warnings are normal and don't block deployment. If build fails:
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
npm run build
```

Check for any new TypeScript errors.

### WebSocket Connection Issues

Both Watchtower and Validator Dashboard connect to `wss://rpc.etrid.network`. If this endpoint is not yet available:

**Watchtower:** Will automatically fall back to demo mode
**Validator:** Will show connection error but UI will still load

To update the endpoint later, redeploy with updated environment variables.

### Dark Mode Not Working (Wallet-Web)

Verify the following files exist:
- `components/theme-provider.tsx`
- `components/mode-toggle.tsx`
- ThemeProvider is wrapping children in `app/layout.tsx`

---

## 📊 Monitoring Deployments

Once deployed, you can monitor your apps at:
- Vercel Dashboard: https://vercel.com/dashboard
- Build logs: Check each deployment's build logs
- Analytics: Available in Vercel dashboard

---

## 🔄 Redeploying

To redeploy after making changes:

```bash
# Production (wallet-web)
cd apps/wallet-web/etrid-crypto-website
vercel --prod

# Staging (any app)
cd apps/[app-name]
vercel
```

Or use the automated script:
```bash
./scripts/deploy-all-apps.sh
```

---

## 📝 Notes

- **First deployment** may take 5-10 minutes as Vercel optimizes the build
- **Subsequent deployments** are faster (~2-3 minutes)
- **Preview deployments** (staging) are created for non-production deployments
- **Production deployments** require the `--prod` flag

---

## ✅ Success Criteria

Deployment is successful when:
- All three apps have deployment URLs
- Apps load without errors in browser
- Dark mode toggle works (wallet-web)
- WebSocket connections attempt to connect (watchtower & validator)
- No critical console errors

---

## 🎯 Next Steps After Deployment

1. Test all deployed URLs
2. Verify functionality
3. Set up custom domains (if desired)
4. Enable Vercel Analytics (optional)
5. Set up production WebSocket endpoint (when available)
6. Monitor for any issues in first 24 hours

---

**Prepared by:** Claude Code
**Date:** October 23, 2025
**Status:** Ready for Deployment
**All Preparation Complete:** ✅

Good luck with your deployment! 🚀
