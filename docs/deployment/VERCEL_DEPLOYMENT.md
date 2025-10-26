# Vercel Deployment Guide

This guide covers deploying Ëtrid Protocol UI applications to Vercel.

## Prerequisites

### 1. Install Vercel CLI

```bash
npm install -g vercel
# or
pnpm add -g vercel
```

### 2. Authenticate with Vercel

```bash
vercel login
```

## Deployment Methods

### Method 1: Using the Deployment Script (Recommended)

The project includes a deployment script that handles all applications:

```bash
# Deploy all apps as preview
./scripts/deploy-ui.sh

# Deploy specific app as preview
./scripts/deploy-ui.sh wallet-web
./scripts/deploy-ui.sh validator
./scripts/deploy-ui.sh watchtower

# Deploy to production
./scripts/deploy-ui.sh wallet-web --production
./scripts/deploy-ui.sh all --production
```

### Method 2: Manual Deployment

Deploy each application individually:

#### Wallet Web Application

```bash
cd apps/wallet-web/etrid-crypto-website
vercel --prod  # Production
vercel         # Preview
```

#### Validator Dashboard

```bash
cd apps/validator-dashboard
vercel --prod  # Production
vercel         # Preview
```

#### Watchtower Monitor

```bash
cd apps/watchtower-monitor
vercel --prod  # Production
vercel         # Preview
```

## Configuration Files

Each application has a `vercel.json` configuration file:

- `/apps/wallet-web/etrid-crypto-website/vercel.json`
- `/apps/validator-dashboard/vercel.json`
- `/apps/watchtower-monitor/vercel.json`

### Key Configuration Details

All applications are configured with:

- **Framework**: Next.js
- **Build Command**: `npm run build`
- **Install Command**: `npm install --legacy-peer-deps`
- **Region**: `iad1` (US East - Washington, D.C.)
- **Security Headers**: Enabled for all routes

### Environment Variables

Each deployment includes the following environment variables:

| Variable | Default Value | Description |
|----------|---------------|-------------|
| `NEXT_PUBLIC_WS_PROVIDER` | `wss://rpc.etrid.network` | WebSocket RPC endpoint |
| `NEXT_PUBLIC_NETWORK_NAME` | `Ëtrid MainNet` | Network display name |
| `NEXT_PUBLIC_CHAIN_ID` | `etrid-mainnet` | Chain identifier |
| `NODE_ENV` | `production` | Node environment |

### Application-Specific Variables

**Validator Dashboard:**
- `NEXT_PUBLIC_APP_NAME`: `Validator Dashboard`

**Watchtower Monitor:**
- `NEXT_PUBLIC_APP_NAME`: `Watchtower Monitor`
- **Max Function Duration**: 60 seconds (increased for monitoring tasks)

## Vercel Project Setup

### Initial Project Setup

For each application, you'll need to link it to a Vercel project:

```bash
cd apps/wallet-web/etrid-crypto-website
vercel link
```

Follow the prompts to:
1. Select your Vercel scope (personal or team)
2. Link to existing project or create new
3. Confirm the project settings

### Environment Variables in Vercel Dashboard

To set environment variables in the Vercel dashboard:

1. Go to your project settings
2. Navigate to "Environment Variables"
3. Add the following variables for each environment (Production, Preview, Development):

```
NEXT_PUBLIC_WS_PROVIDER=wss://rpc.etrid.network
NEXT_PUBLIC_NETWORK_NAME=Ëtrid MainNet
NEXT_PUBLIC_CHAIN_ID=etrid-mainnet
```

For testnet deployments, use:

```
NEXT_PUBLIC_WS_PROVIDER=wss://testnet-rpc.etrid.network
NEXT_PUBLIC_NETWORK_NAME=Ëtrid TestNet
NEXT_PUBLIC_CHAIN_ID=etrid-testnet
```

## Deployment Workflow

### Preview Deployments

Every git push to any branch automatically creates a preview deployment:

```bash
git add .
git commit -m "feat: add new feature"
git push origin feature-branch
```

Vercel will automatically:
- Build the application
- Deploy to a unique preview URL
- Comment the deployment URL on the PR (if using GitHub integration)

### Production Deployments

Production deployments occur when:

1. **Merging to main branch** (with Vercel GitHub integration)
2. **Manual deployment**:
   ```bash
   ./scripts/deploy-ui.sh all --production
   ```

### Rollback

To rollback to a previous deployment:

1. Go to Vercel Dashboard
2. Navigate to Deployments
3. Find the previous working deployment
4. Click "..." menu and select "Promote to Production"

Or use CLI:

```bash
vercel rollback
```

## CI/CD Integration

### GitHub Actions Example

Create `.github/workflows/deploy-vercel.yml`:

```yaml
name: Deploy to Vercel

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Install Vercel CLI
        run: npm install -g vercel

      - name: Deploy to Vercel
        env:
          VERCEL_TOKEN: ${{ secrets.VERCEL_TOKEN }}
          VERCEL_ORG_ID: ${{ secrets.VERCEL_ORG_ID }}
          VERCEL_PROJECT_ID: ${{ secrets.VERCEL_PROJECT_ID }}
        run: |
          if [ "${{ github.ref }}" = "refs/heads/main" ]; then
            ./scripts/deploy-ui.sh all --production
          else
            ./scripts/deploy-ui.sh all
          fi
```

### Required Secrets

Add these secrets to your GitHub repository:

- `VERCEL_TOKEN`: Get from Vercel dashboard → Account Settings → Tokens
- `VERCEL_ORG_ID`: Found in `.vercel/project.json` after linking
- `VERCEL_PROJECT_ID`: Found in `.vercel/project.json` after linking

## Troubleshooting

### Build Failures

**Issue**: `npm install` fails due to peer dependency conflicts

**Solution**: The configuration uses `--legacy-peer-deps` flag. Ensure `vercel.json` has:
```json
"installCommand": "npm install --legacy-peer-deps"
```

**Issue**: Build timeout

**Solution**: Increase function max duration in `vercel.json`:
```json
"functions": {
  "app/**/*.{ts,tsx}": {
    "maxDuration": 60
  }
}
```

### WebSocket Connection Issues

**Issue**: Cannot connect to WebSocket in production

**Solution**:
1. Verify `NEXT_PUBLIC_WS_PROVIDER` is set correctly
2. Check that the RPC endpoint is accessible
3. Ensure the endpoint uses `wss://` (secure WebSocket)

### Memory Issues

**Issue**: Build runs out of memory

**Solution**: Add Node.js memory flags to build command in `vercel.json`:
```json
"buildCommand": "NODE_OPTIONS='--max-old-space-size=4096' npm run build"
```

## Performance Optimization

### Edge Configuration

For better global performance, configure edge regions in `vercel.json`:

```json
"regions": ["iad1", "sfo1", "fra1"]
```

Available regions:
- `iad1`: US East (Washington, D.C.)
- `sfo1`: US West (San Francisco)
- `fra1`: Europe (Frankfurt)

### Caching Strategy

Add caching headers for static assets:

```json
"headers": [
  {
    "source": "/_next/static/(.*)",
    "headers": [
      {
        "key": "Cache-Control",
        "value": "public, max-age=31536000, immutable"
      }
    ]
  }
]
```

## Monitoring

### Vercel Analytics

Enable Vercel Analytics by installing the package:

```bash
npm install @vercel/analytics
```

Already included in wallet-web's `package.json`.

### Custom Monitoring

For custom monitoring, add API routes:

```typescript
// pages/api/health.ts
export default function handler(req, res) {
  res.status(200).json({
    status: 'healthy',
    timestamp: new Date().toISOString()
  });
}
```

## Support

For deployment issues:

1. Check Vercel deployment logs in the dashboard
2. Review build logs: `vercel logs <deployment-url>`
3. Contact team via GitHub issues

## Additional Resources

- [Vercel Documentation](https://vercel.com/docs)
- [Next.js Deployment](https://nextjs.org/docs/deployment)
- [Vercel CLI Reference](https://vercel.com/docs/cli)
