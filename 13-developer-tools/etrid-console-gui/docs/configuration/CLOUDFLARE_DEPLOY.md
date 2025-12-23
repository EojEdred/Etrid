# Cloudflare Pages Deployment Guide - ËTRID Wallet

This guide covers deploying the ËTRID Wallet Web App to Cloudflare Pages.

## Prerequisites

- Node.js 20+ installed
- Cloudflare account with Pages enabled
- Wrangler CLI (installed globally or via npx)

## Quick Start - Automatic Deployment

The project is configured for automatic deployment via GitHub Actions. Every push to the `main` branch will trigger a deployment.

### Required GitHub Secrets

Set these secrets in your GitHub repository settings (Settings > Secrets and variables > Actions):

1. **CLOUDFLARE_API_TOKEN**: Your Cloudflare API token with Pages permissions
   - Create at: https://dash.cloudflare.com/profile/api-tokens
   - Required permissions: `Cloudflare Pages - Edit`

2. **CLOUDFLARE_ACCOUNT_ID**: Your Cloudflare account ID
   - Find at: https://dash.cloudflare.com/ (in the URL or account settings)

## Manual Deployment Steps

### 1. Install Wrangler CLI

```bash
# Global installation
npm install -g wrangler

# Or use npx (no installation needed)
npx wrangler --version
```

### 2. Login to Cloudflare

```bash
wrangler login
```

This will open a browser window for authentication.

### 3. Build the Application

```bash
npm install
npm run build
```

This creates a static export in the `out` directory.

### 4. Deploy to Cloudflare Pages

```bash
# First deployment (creates the project)
wrangler pages deploy out --project-name=etrid-wallet

# Subsequent deployments
wrangler pages deploy out --project-name=etrid-wallet --branch=main
```

### 5. Verify Deployment

After deployment, Wrangler will output the deployment URL:
- Production: `https://etrid-wallet.pages.dev`
- Preview: `https://[commit-hash].etrid-wallet.pages.dev`

## Configuration Files

### wrangler.toml

Main configuration file for Cloudflare Pages deployment:
- Project name: `etrid-wallet`
- Output directory: `.vercel/output/static`
- Environment variables
- Security headers

### next.config.mjs

Updated with:
- `output: 'export'` - Enables static export for Cloudflare Pages
- Image optimization disabled (required for static export)

### .github/workflows/cloudflare-deploy.yml

Automated deployment workflow that:
- Triggers on push to main branch
- Builds the Next.js application
- Deploys to Cloudflare Pages using the Pages Action

## Environment Variables

Set these in Cloudflare Pages dashboard (or in `wrangler.toml` for non-secrets):

### Required
- `NEXT_PUBLIC_APP_NAME`: "Ëtrid Wallet"
- `NEXT_PUBLIC_NETWORK_NAME`: "Ëtrid MainNet"
- `NODE_ENV`: "production"

### Optional (if using wallet connectors)
- `NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID`: Your WalletConnect project ID
- `NEXT_PUBLIC_RPC_URL`: Custom RPC endpoint
- `NEXT_PUBLIC_CHAIN_ID`: Network chain ID

## Custom Domain Setup

1. Go to Cloudflare Pages dashboard
2. Select your `etrid-wallet` project
3. Navigate to **Custom domains**
4. Add your domain: `wallet.etrid.io`
5. Update DNS records as instructed

## Troubleshooting

### Build Fails

If the build fails, check:
- All dependencies are installed: `npm ci`
- TypeScript errors (currently ignored in config)
- Environment variables are set correctly

### Deployment Fails

Common issues:
- **API token invalid**: Regenerate token with correct permissions
- **Account ID wrong**: Verify account ID in Cloudflare dashboard
- **Project name conflict**: Use a unique project name

### Runtime Errors

- Check browser console for errors
- Verify environment variables are set in Cloudflare Pages dashboard
- Check Cloudflare Pages Function Logs

## Performance Optimization

Cloudflare Pages provides:
- Global CDN distribution
- Automatic HTTPS
- HTTP/2 and HTTP/3 support
- Edge caching
- DDoS protection

## Monitoring

Monitor your deployment:
- Cloudflare Pages dashboard: Analytics and logs
- Real User Monitoring (RUM) available in Cloudflare dashboard
- Function logs for debugging

## Rollback

To rollback to a previous deployment:

```bash
# List deployments
wrangler pages deployment list --project-name=etrid-wallet

# Promote a specific deployment to production
wrangler pages deployment promote [deployment-id] --project-name=etrid-wallet
```

Or use the Cloudflare Pages dashboard to rollback visually.

## Development vs Production

### Development Preview
```bash
# Preview deployments are automatic for all branches
git checkout -b feature/new-feature
git push origin feature/new-feature
# Preview URL: https://feature-new-feature.etrid-wallet.pages.dev
```

### Production
```bash
# Production deployments from main branch only
git checkout main
git push origin main
# Production URL: https://etrid-wallet.pages.dev
```

## Additional Resources

- [Cloudflare Pages Documentation](https://developers.cloudflare.com/pages/)
- [Wrangler CLI Documentation](https://developers.cloudflare.com/workers/wrangler/)
- [Next.js on Cloudflare Pages](https://developers.cloudflare.com/pages/framework-guides/nextjs/)
- [ËTRID Documentation](https://docs.etrid.io)

## Support

For issues specific to ËTRID Wallet deployment:
- GitHub Issues: https://github.com/etrid/wallet-web/issues
- ËTRID Discord: [Your Discord link]
- Documentation: https://docs.etrid.io
