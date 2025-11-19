# Ëtrid Wallet - Vercel Deployment Guide

> Complete Vercel deployment configuration for PWA and Landing Page with CI/CD, environment management, and production optimization.

## What You Get

A complete, production-ready Vercel deployment setup that includes:

- ✅ **Vercel Configuration** - Production-optimized configs for both PWA and Landing Page
- ✅ **CI/CD Pipeline** - Automated GitHub Actions workflows
- ✅ **Environment Management** - Complete environment variable setup
- ✅ **Domain Configuration** - DNS and SSL setup guides
- ✅ **Deployment Scripts** - One-click deployment automation
- ✅ **Security Headers** - Production-grade security configuration
- ✅ **Performance Optimization** - Caching, compression, and CDN setup
- ✅ **Comprehensive Documentation** - Step-by-step guides for everything

## Quick Navigation

### 🚀 Getting Started
1. **New to Vercel?** Start here: [QUICK_DEPLOY_GUIDE.md](QUICK_DEPLOY_GUIDE.md)
   - Get deployed in under 10 minutes
   - Beginner-friendly walkthrough
   - Includes troubleshooting

2. **Environment Variables**: [VERCEL_ENV_SETUP.md](VERCEL_ENV_SETUP.md)
   - All required variables listed
   - Firebase configuration
   - Local development setup

3. **GitHub Actions Setup**: [GITHUB_SECRETS_SETUP.md](GITHUB_SECRETS_SETUP.md)
   - 12 required secrets explained
   - Step-by-step configuration
   - GitHub CLI commands included

### 📋 Configuration Files

#### PWA (etrid-wallet)
- **Vercel Config**: `etrid-wallet/vercel.json`
- **Deployment Exclusions**: `etrid-wallet/.vercelignore`
- **Features**: Service workers, Firebase messaging, security headers, PWA manifest

#### Landing Page
- **Vercel Config**: `landing-page/vercel.json`
- **Deployment Exclusions**: `landing-page/.vercelignore`
- **Features**: Security headers, static asset caching, redirects

### 🔧 Deployment Tools

#### Deployment Script
- **File**: `scripts/deploy.sh`
- **Usage**: `./scripts/deploy.sh`
- **Features**: Interactive menu, preview/production modes, error handling

#### GitHub Actions
- **PWA Workflow**: `.github/workflows/deploy-pwa.yml`
- **Landing Workflow**: `.github/workflows/deploy-landing.yml`
- **Triggers**: Auto-deploy on push to main, preview on PR

### 📚 Documentation

| Document | Purpose | When to Use |
|----------|---------|-------------|
| [QUICK_DEPLOY_GUIDE.md](QUICK_DEPLOY_GUIDE.md) | 10-minute deployment | First time deploying |
| [VERCEL_ENV_SETUP.md](VERCEL_ENV_SETUP.md) | Environment variables | Setting up environment |
| [GITHUB_SECRETS_SETUP.md](GITHUB_SECRETS_SETUP.md) | CI/CD secrets | Enabling GitHub Actions |
| [DOMAIN_SETUP.md](DOMAIN_SETUP.md) | Custom domains | Configuring DNS |
| [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md) | Complete checklist | Before production launch |
| [VERCEL_DEPLOYMENT_SUMMARY.md](VERCEL_DEPLOYMENT_SUMMARY.md) | Full overview | Understanding the setup |

### 🎯 Common Tasks

#### First Time Deployment
```bash
# 1. Install Vercel CLI
npm i -g vercel

# 2. Login
vercel login

# 3. Deploy PWA
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
vercel --prod

# 4. Deploy Landing Page
cd /home/user/Etrid/apps/wallet-mobile/landing-page
vercel --prod
```

#### Using Deployment Script
```bash
cd /home/user/Etrid/apps/wallet-mobile
./scripts/deploy.sh
# Select: 3 (Both projects)
# Production: y
```

#### Setting Up CI/CD
1. Follow [GITHUB_SECRETS_SETUP.md](GITHUB_SECRETS_SETUP.md) to add all secrets
2. Push to `main` branch
3. GitHub Actions automatically deploys
4. Check Actions tab for deployment status

#### Configuring Custom Domain
1. Follow [DOMAIN_SETUP.md](DOMAIN_SETUP.md)
2. Add domain in Vercel Dashboard
3. Configure DNS records
4. Wait for SSL certificate (automatic)
5. Verify with `curl -I https://wallet.etrid.com`

## File Structure

```
/home/user/Etrid/
├── .github/
│   └── workflows/
│       ├── deploy-pwa.yml              # PWA auto-deployment
│       └── deploy-landing.yml          # Landing page auto-deployment
│
└── apps/wallet-mobile/
    ├── etrid-wallet/
    │   ├── vercel.json                 # PWA Vercel configuration
    │   └── .vercelignore               # Deployment exclusions
    │
    ├── landing-page/
    │   ├── vercel.json                 # Landing page configuration
    │   └── .vercelignore               # Deployment exclusions
    │
    ├── scripts/
    │   └── deploy.sh                   # Interactive deployment script
    │
    ├── QUICK_DEPLOY_GUIDE.md           # 10-minute deployment guide
    ├── VERCEL_ENV_SETUP.md             # Environment variables setup
    ├── GITHUB_SECRETS_SETUP.md         # GitHub Actions secrets
    ├── DOMAIN_SETUP.md                 # DNS and domain configuration
    ├── DEPLOYMENT_CHECKLIST.md         # Pre/post deployment checklist
    ├── VERCEL_DEPLOYMENT_SUMMARY.md    # Complete overview
    ├── vercel-performance.json         # Performance optimizations
    └── README_VERCEL_DEPLOYMENT.md     # This file
```

## Features Breakdown

### Security
- **XSS Protection**: Prevents cross-site scripting attacks
- **Content-Type-Options**: Prevents MIME type sniffing
- **Frame-Options**: Prevents clickjacking
- **Referrer-Policy**: Controls referrer information
- **Permissions-Policy**: Restricts browser features
- **HTTPS Only**: Automatic SSL/TLS encryption
- **Environment Variables**: Secured in Vercel Dashboard

### Performance
- **Service Worker Caching**: Offline-first PWA support
- **Static Asset Caching**: Immutable caching for icons, images, scripts
- **Image Optimization**: AVIF and WebP support
- **Code Splitting**: Automatic chunk optimization
- **CDN Distribution**: Global edge network (automatic)
- **Compression**: Automatic gzip/brotli
- **Build Caching**: Faster subsequent deployments

### PWA Specific
- **Service Worker Support**: Proper routing and cache headers
- **Web App Manifest**: Install prompts and app icons
- **Firebase Cloud Messaging**: Push notification support
- **Offline Mode**: Full offline functionality
- **Install Prompts**: Add to Home Screen
- **Background Sync**: Sync when connection restored

### CI/CD
- **Automatic Deployments**: Push to main → auto-deploy
- **Preview Deployments**: Every PR gets preview URL
- **Build Caching**: npm dependencies cached
- **Path-Based Triggers**: Only build when relevant files change
- **Environment Injection**: Secure environment variable handling
- **Status Checks**: Build status in PR

## Deployment Environments

### Preview (Development/Staging)
- **Trigger**: Push to any branch or create PR
- **URL Format**: `https://etrid-wallet-[hash]-[username].vercel.app`
- **Purpose**: Testing, review, QA
- **Environment Variables**: Uses "Preview" environment
- **Auto-Delete**: Old previews cleaned up automatically

### Production
- **Trigger**: Push to `main` branch or `vercel --prod`
- **URL**: `https://etrid-wallet.vercel.app` (or custom domain)
- **Purpose**: Live production site
- **Environment Variables**: Uses "Production" environment
- **Aliased**: Custom domain automatically pointed here

## Required Setup

### 1. Vercel Account
- Sign up: https://vercel.com/signup
- Free tier includes everything you need
- GitHub integration recommended

### 2. Environment Variables
See: [VERCEL_ENV_SETUP.md](VERCEL_ENV_SETUP.md)

**PWA (10 variables)**:
- Firebase API Key
- Firebase Auth Domain
- Firebase Project ID
- Firebase Storage Bucket
- Firebase Messaging Sender ID
- Firebase App ID
- Firebase VAPID Key
- App URL
- API URL

**Landing Page (4 variables)**:
- App URL
- Twitter handle
- GitHub username
- Email address

### 3. GitHub Secrets (for CI/CD)
See: [GITHUB_SECRETS_SETUP.md](GITHUB_SECRETS_SETUP.md)

**12 Secrets Required**:
- VERCEL_TOKEN
- VERCEL_ORG_ID
- VERCEL_PROJECT_ID
- VERCEL_TOKEN_LANDING
- VERCEL_PROJECT_ID_LANDING
- FIREBASE_API_KEY
- FIREBASE_AUTH_DOMAIN
- FIREBASE_PROJECT_ID
- FIREBASE_STORAGE_BUCKET
- FIREBASE_MESSAGING_SENDER_ID
- FIREBASE_APP_ID
- FIREBASE_VAPID_KEY

### 4. Custom Domains (Optional)
See: [DOMAIN_SETUP.md](DOMAIN_SETUP.md)

**Recommended Setup**:
- `wallet.etrid.com` → PWA
- `www.wallet.etrid.com` → Landing Page

## Step-by-Step Deployment

### Phase 1: Initial Setup (15 minutes)

1. **Install Vercel CLI**
   ```bash
   npm i -g vercel
   ```

2. **Create Vercel Account**
   - Visit https://vercel.com/signup
   - Sign up with GitHub

3. **Setup Environment Variables**
   - Follow [VERCEL_ENV_SETUP.md](VERCEL_ENV_SETUP.md)
   - Create `.env.local` files locally
   - Add variables to Vercel Dashboard

### Phase 2: First Deployment (10 minutes)

1. **Deploy PWA**
   ```bash
   cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
   npm install
   npm run build  # Test build
   vercel         # Preview deployment
   vercel --prod  # Production deployment
   ```

2. **Deploy Landing Page**
   ```bash
   cd /home/user/Etrid/apps/wallet-mobile/landing-page
   npm install
   npm run build
   vercel --prod
   ```

### Phase 3: Setup CI/CD (20 minutes)

1. **Configure GitHub Secrets**
   - Follow [GITHUB_SECRETS_SETUP.md](GITHUB_SECRETS_SETUP.md)
   - Add all 12 secrets to repository

2. **Test Workflows**
   ```bash
   cd /home/user/Etrid
   git add .
   git commit -m "chore: Setup Vercel deployment"
   git push
   ```

3. **Verify**
   - Check GitHub Actions tab
   - Verify deployments succeeded
   - Test preview URLs

### Phase 4: Custom Domains (30 minutes)

1. **Configure DNS**
   - Follow [DOMAIN_SETUP.md](DOMAIN_SETUP.md)
   - Add CNAME or A records

2. **Add to Vercel**
   - Vercel Dashboard → Domains
   - Add `wallet.etrid.com`
   - Add `www.wallet.etrid.com`

3. **Wait for SSL**
   - Automatic Let's Encrypt
   - Usually 5-10 minutes

### Phase 5: Testing (30 minutes)

Follow: [DEPLOYMENT_CHECKLIST.md](DEPLOYMENT_CHECKLIST.md)

- [ ] PWA installation on mobile
- [ ] Push notifications
- [ ] Offline mode
- [ ] All routes work
- [ ] Lighthouse score >90
- [ ] Cross-browser testing

### Phase 6: Monitoring (15 minutes)

1. **Enable Vercel Analytics**
   - Dashboard → Analytics → Enable

2. **Setup Uptime Monitoring**
   - UptimeRobot or Pingdom
   - Monitor main URLs

3. **Configure Alerts**
   - Deployment notifications
   - Error alerts

## Cost Breakdown

### Vercel Free Tier (Hobby) - $0/month
- ✅ 100 GB bandwidth/month
- ✅ Unlimited websites
- ✅ Automatic HTTPS
- ✅ Unlimited deployments
- ✅ Preview deployments
- ✅ 1 concurrent build
- ✅ Serverless functions
- ✅ Edge Network (CDN)
- ✅ Analytics (basic)

**Good for**: Personal projects, small apps, development

### Vercel Pro - $20/month per user
- ✅ 1 TB bandwidth/month
- ✅ Team collaboration
- ✅ Advanced analytics
- ✅ 12 concurrent builds
- ✅ Password protection
- ✅ Commercial use
- ✅ Priority support

**Good for**: Production apps, teams, commercial use

### When to Upgrade
- More than 100 GB bandwidth needed
- Need team collaboration
- Commercial/business use
- Need faster builds (12 concurrent vs 1)
- Want advanced analytics

### Estimated Costs by Scale
- **0-10K users**: $0/month (Free tier)
- **10K-100K users**: $0-20/month (Free or Pro)
- **100K-1M users**: $20-50/month (Pro + extra bandwidth)

## Troubleshooting

### Build Fails
**Check**:
- TypeScript errors: `npm run type-check`
- Environment variables set correctly
- Dependencies installed: `npm install`
- Build locally first: `npm run build`

**Fix**:
```bash
rm -rf .next node_modules
npm install
npm run build
```

### Deployment Fails
**Check**:
- Vercel CLI authenticated: `vercel whoami`
- Project linked: `vercel link`
- Correct directory

**Fix**:
```bash
vercel logout
vercel login
vercel link
```

### Environment Variables Not Working
**Check**:
- Variables set in Vercel Dashboard
- Correct environment selected (Production/Preview)
- Variable names exact (case-sensitive)
- Redeployed after adding variables

**Fix**:
- Redeploy: `vercel --prod --force`

### GitHub Actions Failing
**Check**:
- All 12 secrets added to repository
- Secret names exact match
- Workflows have correct paths
- Build succeeds locally

**Fix**:
- Re-add secrets
- Check workflow logs
- Test build locally first

### Domain Not Working
**Check**:
- DNS records correct
- DNS propagation (wait 5-48 hours)
- Cloudflare proxy disabled (if using)
- SSL certificate issued

**Fix**:
```bash
# Check DNS
dig wallet.etrid.com

# Check SSL
curl -I https://wallet.etrid.com
```

## Best Practices

### Security
1. ✅ Never commit `.env.local` files
2. ✅ Use Vercel Environment Variables for secrets
3. ✅ Restrict Firebase API keys to your domains
4. ✅ Enable Firebase App Check
5. ✅ Keep dependencies updated
6. ✅ Use HTTPS only (automatic on Vercel)
7. ✅ Review security headers regularly

### Performance
1. ✅ Use Next.js Image component
2. ✅ Enable image optimization
3. ✅ Implement code splitting
4. ✅ Use service worker caching
5. ✅ Monitor bundle size
6. ✅ Run Lighthouse audits regularly
7. ✅ Optimize Core Web Vitals

### Development
1. ✅ Test builds locally before deploying
2. ✅ Use preview deployments for testing
3. ✅ Review deployment logs
4. ✅ Keep documentation updated
5. ✅ Use deployment script for consistency
6. ✅ Monitor analytics and errors
7. ✅ Regular dependency updates

### Deployment
1. ✅ Always test in preview first
2. ✅ Use production deployment only for stable code
3. ✅ Review deployment checklist
4. ✅ Monitor deployments
5. ✅ Have rollback plan ready
6. ✅ Communicate deployments to team
7. ✅ Keep staging/production separate

## Support Resources

### Documentation
- **This Setup**: All `.md` files in this directory
- **Vercel Docs**: https://vercel.com/docs
- **Next.js Docs**: https://nextjs.org/docs
- **Firebase Docs**: https://firebase.google.com/docs
- **GitHub Actions**: https://docs.github.com/en/actions

### Community
- **Vercel Discord**: https://vercel.com/discord
- **Next.js Discord**: https://nextjs.org/discord
- **GitHub Discussions**: Your repository

### Support
- **Vercel Support**: support@vercel.com
- **Vercel Status**: https://www.vercel-status.com
- **Firebase Support**: https://firebase.google.com/support

## What's Next?

After successful deployment:

1. ✅ **Monitor Performance**
   - Enable Vercel Analytics
   - Set up error tracking
   - Monitor Core Web Vitals

2. ✅ **Optimize**
   - Review Lighthouse scores
   - Optimize images
   - Reduce bundle size

3. ✅ **Scale**
   - Monitor bandwidth usage
   - Plan for traffic growth
   - Consider upgrading to Pro if needed

4. ✅ **Maintain**
   - Regular dependency updates
   - Security patches
   - Performance monitoring

## Quick Reference

### Commands
```bash
# Deploy to preview
vercel

# Deploy to production
vercel --prod

# Check status
vercel ls

# View logs
vercel logs [deployment-url]

# Pull env variables
vercel env pull

# Link project
vercel link

# Remove deployment
vercel rm [deployment-url]

# Rollback
vercel rollback [deployment-url]

# Check version
vercel --version

# Get help
vercel --help
```

### Environment Variable Prefixes
- `NEXT_PUBLIC_*` - Exposed to browser (use for public configs)
- No prefix - Server-side only (use for secrets)

### URLs
- **Vercel Dashboard**: https://vercel.com/dashboard
- **Account Tokens**: https://vercel.com/account/tokens
- **Project Settings**: https://vercel.com/[username]/[project]/settings
- **Deployments**: https://vercel.com/[username]/[project]/deployments

## Conclusion

You now have a complete, production-ready Vercel deployment setup with:

- ✅ Automated CI/CD pipeline
- ✅ Production-optimized configurations
- ✅ Security headers and best practices
- ✅ Performance optimizations
- ✅ Comprehensive documentation
- ✅ Interactive deployment scripts
- ✅ Monitoring and analytics ready
- ✅ Custom domain support

**Ready to deploy?** Start with [QUICK_DEPLOY_GUIDE.md](QUICK_DEPLOY_GUIDE.md) for a 10-minute walkthrough!

---

**Questions?** Check the specific documentation files or open an issue in your repository.

**Need help?** Join Vercel Discord or check the troubleshooting sections in each guide.
