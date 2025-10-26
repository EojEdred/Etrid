# Ëtrid Protocol - Vercel Deployment Checklist

**Use this checklist to ensure successful deployment of all UI applications**

---

## Pre-Deployment Checklist

### Prerequisites
- [x] Vercel CLI installed (v48.5.0)
- [ ] Vercel account created/accessible
- [ ] GitHub repository access
- [ ] Domain registrar access (for custom domains)
- [ ] RPC endpoint accessible (wss://rpc.etrid.network)

### Build Verification
- [x] Wallet Web build passing
- [x] Validator Dashboard build passing
- [ ] Watchtower Monitor build fixed and passing
- [x] All vercel.json files valid
- [x] Environment variables configured

### Documentation Review
- [x] Deployment guide created
- [x] Deployment script ready
- [x] Architecture documented
- [x] Quick start guide available

---

## Deployment Checklist

### Step 1: Authenticate with Vercel

```bash
vercel login
```

- [ ] Choose authentication method (GitHub/GitLab/Email)
- [ ] Complete browser authorization
- [ ] Verify login: `vercel whoami`
- [ ] Note your Vercel username/org: ___________________

---

### Step 2: Deploy Wallet Web (Production)

```bash
cd /Users/macbook/Desktop/etrid/apps/wallet-web/etrid-crypto-website
vercel --prod
```

**During deployment:**
- [ ] Confirm project setup
- [ ] Enter project name: `etrid-wallet-web`
- [ ] Confirm directory: `./`
- [ ] Accept default settings (or customize)
- [ ] Wait for build to complete
- [ ] Note deployment URL: ___________________________________
- [ ] Save production URL: ___________________________________

**Post-deployment tests:**
- [ ] Visit deployment URL
- [ ] Page loads without errors
- [ ] Check browser console (no critical errors)
- [ ] Test wallet connection
- [ ] Verify balance display
- [ ] Test swap interface
- [ ] Check governance page
- [ ] Verify WebSocket connection: Open DevTools → Network → WS tab
- [ ] RPC connection established: wss://rpc.etrid.network
- [ ] No CORS errors
- [ ] Performance acceptable (< 3s page load)

**Screenshots:**
- [ ] Take screenshot of home page
- [ ] Take screenshot of swap interface
- [ ] Take screenshot of governance page
- [ ] Save to: `/Users/macbook/Desktop/etrid/docs/deployment/screenshots/wallet-web/`

---

### Step 3: Deploy Validator Dashboard (Staging)

```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
vercel
```

**During deployment:**
- [ ] Confirm project setup
- [ ] Enter project name: `etrid-validator-dashboard`
- [ ] Confirm directory: `./`
- [ ] Accept default settings (or customize)
- [ ] Wait for build to complete
- [ ] Note deployment URL: ___________________________________
- [ ] Save preview URL: ___________________________________

**Post-deployment tests:**
- [ ] Visit deployment URL
- [ ] Page loads without errors
- [ ] Check browser console (no critical errors)
- [ ] Validator stats display correctly
- [ ] Nominator list loads
- [ ] Commission settings accessible
- [ ] Reward history visible
- [ ] Performance charts render
- [ ] Verify WebSocket connection
- [ ] RPC connection established: wss://rpc.etrid.network
- [ ] No CORS errors
- [ ] Performance acceptable

**Screenshots:**
- [ ] Take screenshot of dashboard
- [ ] Take screenshot of nominator list
- [ ] Take screenshot of rewards page
- [ ] Save to: `/Users/macbook/Desktop/etrid/docs/deployment/screenshots/validator-dashboard/`

---

### Step 4: Fix Watchtower Monitor (If Not Done)

**Choose one fix:**

**Option A: Migrate to Tailwind v4 (Recommended)**
- [ ] Update `src/app/globals.css`:
  ```css
  @import "tailwindcss";
  ```
- [ ] Remove all `@apply` directives
- [ ] Use utility classes directly in components
- [ ] Test build: `npm run build`
- [ ] Verify build passes

**Option B: Downgrade to Tailwind v3 (Temporary)**
```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
npm remove @tailwindcss/postcss tailwindcss
npm install --save-dev tailwindcss@^3 autoprefixer
rm -rf .next node_modules/.cache
npm run build
```
- [ ] Execute downgrade commands
- [ ] Verify build passes
- [ ] Note: Plan upgrade to v4 later

---

### Step 5: Deploy Watchtower Monitor (Staging)

```bash
cd /Users/macbook/Desktop/etrid/apps/watchtower-monitor
vercel
```

**During deployment:**
- [ ] Confirm project setup
- [ ] Enter project name: `etrid-watchtower-monitor`
- [ ] Confirm directory: `./`
- [ ] Accept default settings (or customize)
- [ ] Wait for build to complete
- [ ] Note deployment URL: ___________________________________
- [ ] Save preview URL: ___________________________________

**Post-deployment tests:**
- [ ] Visit deployment URL
- [ ] Page loads without errors
- [ ] Check browser console (no critical errors)
- [ ] Channel monitoring displays
- [ ] Fraud detection functional
- [ ] Alerts/notifications working
- [ ] Real-time updates functioning
- [ ] Verify WebSocket connection
- [ ] RPC connection established: wss://rpc.etrid.network
- [ ] No CORS errors
- [ ] Performance acceptable

**Screenshots:**
- [ ] Take screenshot of monitor dashboard
- [ ] Take screenshot of channel list
- [ ] Take screenshot of alerts page
- [ ] Save to: `/Users/macbook/Desktop/etrid/docs/deployment/screenshots/watchtower-monitor/`

---

## Post-Deployment Checklist

### Deployment URLs
Record all deployment URLs here:

**Production:**
- Wallet Web: _______________________________________________
  - Vercel: https://etrid-wallet-web.vercel.app
  - Custom: https://___________________

**Staging:**
- Validator Dashboard: _____________________________________
  - Vercel: https://etrid-validator-dashboard-_______.vercel.app
  - Custom: https://___________________

- Watchtower Monitor: ______________________________________
  - Vercel: https://etrid-watchtower-monitor-_______.vercel.app
  - Custom: https://___________________

### Environment Variables Verification
- [ ] NEXT_PUBLIC_WS_PROVIDER set correctly
- [ ] NEXT_PUBLIC_NETWORK_NAME set correctly
- [ ] NEXT_PUBLIC_CHAIN_ID set correctly
- [ ] NODE_ENV set to production
- [ ] All app-specific variables configured

### Security Verification
- [ ] HTTPS enabled (automatic with Vercel)
- [ ] SSL certificate valid
- [ ] Security headers present:
  - [ ] X-Content-Type-Options: nosniff
  - [ ] X-Frame-Options: DENY
  - [ ] X-XSS-Protection: 1; mode=block
  - [ ] Referrer-Policy: strict-origin-when-cross-origin
  - [ ] Permissions-Policy configured
- [ ] No sensitive data exposed in client-side code
- [ ] API keys/secrets not committed to repo

### Performance Verification
- [ ] All pages load in < 3 seconds
- [ ] Core Web Vitals acceptable:
  - [ ] LCP (Largest Contentful Paint) < 2.5s
  - [ ] FID (First Input Delay) < 100ms
  - [ ] CLS (Cumulative Layout Shift) < 0.1
- [ ] Images optimized
- [ ] JavaScript bundle size reasonable
- [ ] No memory leaks detected

### Functionality Testing
- [ ] All routes accessible
- [ ] All forms submitting correctly
- [ ] All buttons/interactions working
- [ ] WebSocket connections stable
- [ ] Data fetching successful
- [ ] Error handling working
- [ ] Loading states displaying

---

## Optional: Custom Domain Configuration

### Domain Setup
- [ ] Purchase/access domain: etrid.network
- [ ] Add domains in Vercel:
  ```bash
  vercel domains add wallet.etrid.network --project=etrid-wallet-web
  vercel domains add validators.etrid.network --project=etrid-validator-dashboard
  vercel domains add watchtower.etrid.network --project=etrid-watchtower-monitor
  ```

### DNS Configuration
Configure these DNS records at your registrar:

**Wallet Web:**
- [ ] Type: CNAME
- [ ] Name: wallet (or @ for root)
- [ ] Value: cname.vercel-dns.com
- [ ] TTL: 3600

**Validator Dashboard:**
- [ ] Type: CNAME
- [ ] Name: validators
- [ ] Value: cname.vercel-dns.com
- [ ] TTL: 3600

**Watchtower Monitor:**
- [ ] Type: CNAME
- [ ] Name: watchtower
- [ ] Value: cname.vercel-dns.com
- [ ] TTL: 3600

### DNS Verification
- [ ] Wait for DNS propagation (up to 48 hours, usually minutes)
- [ ] Verify DNS: `dig wallet.etrid.network CNAME`
- [ ] Check SSL certificate provisioned
- [ ] Test HTTPS on custom domains
- [ ] Verify redirects (HTTP → HTTPS)

---

## Optional: Analytics and Monitoring

### Enable Vercel Analytics
- [ ] Go to Vercel Dashboard
- [ ] Navigate to each project
- [ ] Enable Vercel Analytics
- [ ] Configure data retention
- [ ] Set up alerts

### External Monitoring (Optional)
- [ ] Set up Sentry for error tracking
- [ ] Configure Datadog/New Relic for APM
- [ ] Enable LogRocket for session replay
- [ ] Set up Uptime monitoring (Pingdom, UptimeRobot)
- [ ] Configure alerting (email, Slack, PagerDuty)

---

## Optional: CI/CD Setup

### GitHub Actions Integration
- [ ] Create `.github/workflows/deploy.yml`
- [ ] Add Vercel secrets to GitHub:
  - [ ] VERCEL_TOKEN
  - [ ] VERCEL_ORG_ID
  - [ ] VERCEL_PROJECT_ID (for each project)
- [ ] Configure deployment triggers:
  - [ ] Push to main → Production deploy
  - [ ] Pull request → Preview deploy
  - [ ] Push to develop → Staging deploy
- [ ] Test workflow with dummy commit

### Deployment Automation
- [ ] Auto-deploy on git push
- [ ] Preview deployments for PRs
- [ ] Automatic PR comments with preview URLs
- [ ] Deployment status checks
- [ ] Rollback capability

---

## Documentation and Communication

### Update Documentation
- [ ] Update README.md with deployment URLs
- [ ] Create user guide with production links
- [ ] Document deployment process
- [ ] Create troubleshooting guide
- [ ] Update API documentation

### Team Communication
- [ ] Announce deployments to team
- [ ] Share deployment URLs
- [ ] Provide access credentials (if applicable)
- [ ] Schedule demo/walkthrough
- [ ] Collect feedback

### User Communication (if applicable)
- [ ] Announce launch to users
- [ ] Send email notification
- [ ] Update social media
- [ ] Create launch blog post
- [ ] Update marketing materials

---

## Troubleshooting

### Common Issues Encountered
- [ ] Issue: ________________________
  - [ ] Solution: ________________________
- [ ] Issue: ________________________
  - [ ] Solution: ________________________
- [ ] Issue: ________________________
  - [ ] Solution: ________________________

### Rollback Plan
If deployment fails or issues arise:

1. **Immediate Rollback:**
   ```bash
   vercel rollback [deployment-url]
   ```
   - [ ] Identify failing deployment
   - [ ] Execute rollback command
   - [ ] Verify previous version restored
   - [ ] Notify users of temporary rollback

2. **Fix and Redeploy:**
   - [ ] Identify root cause
   - [ ] Fix locally and test
   - [ ] Commit fix to git
   - [ ] Redeploy: `vercel --prod`
   - [ ] Verify fix in production

---

## Sign-Off

### Deployment Completed By
- **Name:** ______________________
- **Date:** ______________________
- **Time:** ______________________

### Deployment Verified By
- **Name:** ______________________
- **Date:** ______________________
- **Time:** ______________________

### Issues/Notes
_____________________________________________________________________
_____________________________________________________________________
_____________________________________________________________________
_____________________________________________________________________

---

## Final Status

- [ ] All applications deployed successfully
- [ ] All tests passed
- [ ] All URLs documented
- [ ] Custom domains configured (if applicable)
- [ ] Analytics enabled
- [ ] Documentation updated
- [ ] Team notified
- [ ] Deployment considered SUCCESSFUL

---

**Checklist Version:** 1.0
**Date Created:** 2025-10-22
**Last Updated:** 2025-10-22

---

## Quick Reference

**Deployment Commands:**
```bash
# Authenticate
vercel login

# Deploy production
vercel --prod

# Deploy staging
vercel

# Check status
vercel ls

# View logs
vercel logs [deployment-url]

# Rollback
vercel rollback [deployment-url]
```

**Documentation:**
- Comprehensive Guide: `/Users/macbook/Desktop/etrid/docs/VERCEL_DEPLOYMENT_GUIDE.md`
- Quick Start: `/Users/macbook/Desktop/etrid/DEPLOYMENT_QUICK_START.md`
- Full Report: `/Users/macbook/Desktop/etrid/VERCEL_DEPLOYMENT_REPORT.md`
- Architecture: `/Users/macbook/Desktop/etrid/DEPLOYMENT_ARCHITECTURE.md`
- This Checklist: `/Users/macbook/Desktop/etrid/DEPLOYMENT_CHECKLIST.md`
