# Quick Deploy Reference

## One-Command Deployments

### Deploy All Apps (Preview)
```bash
./scripts/deploy-ui.sh
```

### Deploy All Apps (Production)
```bash
./scripts/deploy-ui.sh all --production
```

### Deploy Individual Apps

| App | Preview | Production |
|-----|---------|------------|
| Wallet Web | `./scripts/deploy-ui.sh wallet-web` | `./scripts/deploy-ui.sh wallet-web --production` |
| Validator Dashboard | `./scripts/deploy-ui.sh validator` | `./scripts/deploy-ui.sh validator --production` |
| Watchtower Monitor | `./scripts/deploy-ui.sh watchtower` | `./scripts/deploy-ui.sh watchtower --production` |

## First-Time Setup

```bash
# 1. Install Vercel CLI
npm install -g vercel

# 2. Authenticate
vercel login

# 3. Link each project (run from each app directory)
cd apps/wallet-web/etrid-crypto-website && vercel link
cd apps/validator-dashboard && vercel link
cd apps/watchtower-monitor && vercel link
```

## Environment Variables

All apps use these environment variables (configured in `vercel.json`):

```bash
NEXT_PUBLIC_WS_PROVIDER=wss://rpc.etrid.network
NEXT_PUBLIC_NETWORK_NAME=Ëtrid MainNet
NEXT_PUBLIC_CHAIN_ID=etrid-mainnet
NODE_ENV=production
```

### Override for TestNet

To deploy to testnet, temporarily update `vercel.json`:

```json
"NEXT_PUBLIC_WS_PROVIDER": "wss://testnet-rpc.etrid.network",
"NEXT_PUBLIC_NETWORK_NAME": "Ëtrid TestNet",
"NEXT_PUBLIC_CHAIN_ID": "etrid-testnet"
```

## Application Details

### Wallet Web
- **Path**: `/apps/wallet-web/etrid-crypto-website`
- **Port**: 3000 (dev)
- **Features**: Polkadot wallet, token management, DeFi interface

### Validator Dashboard
- **Path**: `/apps/validator-dashboard`
- **Port**: 3002 (dev)
- **Features**: Validator monitoring, staking management, performance metrics

### Watchtower Monitor
- **Path**: `/apps/watchtower-monitor`
- **Port**: 3003 (dev)
- **Features**: Lightning-Bloc channel monitoring, fraud detection
- **Note**: Uses 60s function timeout (vs 30s for other apps)

## Troubleshooting

### Build Fails
```bash
# All configs use --legacy-peer-deps, but if issues persist:
cd apps/[app-name]
npm install --legacy-peer-deps
npm run build  # Test build locally
```

### Not Authenticated
```bash
vercel login
```

### Check Deployment Status
```bash
vercel ls
```

### View Logs
```bash
vercel logs [deployment-url]
```

### Rollback Production
```bash
cd apps/[app-name]
vercel rollback
```

## Deployment URLs

After deployment, Vercel provides:

- **Preview**: `https://[project]-[hash].vercel.app`
- **Production**: `https://[project].vercel.app` (or custom domain)

Set custom domains in Vercel Dashboard → Project Settings → Domains

## Recommended Domains

- **Wallet Web**: `wallet.etrid.network` or `app.etrid.network`
- **Validator Dashboard**: `validators.etrid.network`
- **Watchtower Monitor**: `watchtower.etrid.network`

## Security Headers

All deployments include:
- X-Content-Type-Options: nosniff
- X-Frame-Options: DENY
- X-XSS-Protection: 1; mode=block
- Referrer-Policy: strict-origin-when-cross-origin
- Permissions-Policy: camera=(), microphone=(), geolocation=()

## Performance

- **Region**: US East (iad1)
- **Framework**: Next.js 15.2.4 (wallet-web, watchtower) / 14.0.4 (validator)
- **Build Cache**: Enabled
- **Function Timeout**: 30s (wallet-web, validator), 60s (watchtower)

## Support Links

- Full Documentation: `/docs/deployment/VERCEL_DEPLOYMENT.md`
- Vercel Dashboard: https://vercel.com/dashboard
- Vercel Docs: https://vercel.com/docs
