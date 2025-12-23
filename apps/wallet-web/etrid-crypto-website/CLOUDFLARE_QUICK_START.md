# Cloudflare Pages - Quick Start Guide

## 1. Install Dependencies
```bash
npm install
```

## 2. Test Build
```bash
npm run build
```
This creates the `out` directory with static files.

## 3. Setup GitHub Secrets

Add these to GitHub repo settings (Settings > Secrets > Actions):

- **CLOUDFLARE_API_TOKEN**: Get from https://dash.cloudflare.com/profile/api-tokens
  - Permission needed: `Cloudflare Pages - Edit`

- **CLOUDFLARE_ACCOUNT_ID**: Find in Cloudflare dashboard URL

## 4. Deploy

### Automatic (via GitHub Actions)
```bash
git add .
git commit -m "Deploy to Cloudflare Pages"
git push origin main
```
Deploys automatically! Check GitHub Actions tab for status.

### Manual (via CLI)
```bash
npx wrangler login
npm run deploy:cloudflare
```

## 5. Access Your Site

- **Production**: https://etrid-wallet.pages.dev
- **Preview**: https://[branch].etrid-wallet.pages.dev

## Common Commands

```bash
# Local development
npm run dev

# Build for production
npm run build

# Deploy to Cloudflare
npm run deploy:cloudflare

# Preview locally with Cloudflare
npm run preview:cloudflare
```

## Custom Domain

1. Go to Cloudflare Pages dashboard
2. Select `etrid-wallet` project
3. Add domain: `wallet.etrid.io`
4. Update DNS as instructed

## Need Help?

- Full guide: `CLOUDFLARE_DEPLOY.md`
- Setup summary: `CLOUDFLARE_SETUP_SUMMARY.md`
- Cloudflare docs: https://developers.cloudflare.com/pages/

## Rollback

Dashboard: Pages > etrid-wallet > Deployments > Rollback

CLI: `npx wrangler pages deployment list --project-name=etrid-wallet`

---
That's it! Your ËTRID Wallet is ready for Cloudflare Pages.
