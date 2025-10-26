# Ëtrid Protocol - Vercel Deployment Architecture

**Visual overview of the deployment architecture and flow**

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Ëtrid Protocol                            │
│                     Vercel Deployment Stack                      │
└─────────────────────────────────────────────────────────────────┘

                              ┌──────────┐
                              │  GitHub  │
                              │   Repo   │
                              └────┬─────┘
                                   │
                                   │ git push
                                   ▼
                         ┌─────────────────┐
                         │  Vercel Deploy  │
                         │   Auto-trigger  │
                         └────┬─────┬──────┘
                              │     │
                 ┌────────────┘     └────────────┐
                 │                               │
                 ▼                               ▼
         ┌───────────────┐             ┌───────────────┐
         │   Build Node  │             │  Edge Network │
         │   (iad1)      │             │   (Global)    │
         └───────┬───────┘             └───────┬───────┘
                 │                             │
                 │ Successful Build            │ CDN Deploy
                 ▼                             ▼
         ┌───────────────┐             ┌───────────────┐
         │  Serverless   │────────────▶│   Production  │
         │  Functions    │             │   URLs        │
         └───────────────┘             └───────────────┘
```

---

## Application Deployment Flow

```
                    ┌─────────────────────────────────┐
                    │   LOCAL DEVELOPMENT             │
                    │   /Users/macbook/Desktop/etrid  │
                    └────────────┬────────────────────┘
                                 │
                    ┌────────────┴────────────┐
                    │   vercel login          │
                    │   (Authenticate)        │
                    └────────────┬────────────┘
                                 │
        ┌────────────────────────┼────────────────────────┐
        │                        │                        │
        ▼                        ▼                        ▼
┌───────────────┐      ┌────────────────┐      ┌────────────────┐
│  WALLET WEB   │      │   VALIDATOR    │      │  WATCHTOWER    │
│               │      │   DASHBOARD    │      │    MONITOR     │
│ wallet-web/   │      │ validator-     │      │ watchtower-    │
│ etrid-crypto- │      │ dashboard/     │      │ monitor/       │
│ website/      │      │                │      │                │
│               │      │                │      │                │
│ vercel --prod │      │ vercel         │      │ vercel         │
│               │      │                │      │                │
│ ✅ READY      │      │ ✅ READY       │      │ ⚠️ FIX FIRST   │
└───────┬───────┘      └────────┬───────┘      └────────┬───────┘
        │                       │                       │
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐      ┌────────────────┐      ┌────────────────┐
│ PRODUCTION    │      │  STAGING       │      │  STAGING       │
│               │      │  (Preview)     │      │  (Preview)     │
│ wallet.etrid. │      │ validators.    │      │ watchtower.    │
│ network       │      │ etrid.network  │      │ etrid.network  │
│               │      │                │      │                │
│ https://      │      │ https://       │      │ https://       │
│ etrid-wallet- │      │ etrid-         │      │ etrid-         │
│ web.vercel    │      │ validator-     │      │ watchtower-    │
│ .app          │      │ dashboard.     │      │ monitor.       │
│               │      │ vercel.app     │      │ vercel.app     │
└───────────────┘      └────────────────┘      └────────────────┘
```

---

## Network Architecture

```
                    ┌────────────────────────────┐
                    │   Users / Validators       │
                    │   (Global)                 │
                    └──────────┬─────────────────┘
                               │
                               │ HTTPS
                               ▼
                    ┌────────────────────────────┐
                    │   Vercel Edge Network      │
                    │   (CDN + SSL)              │
                    └──────────┬─────────────────┘
                               │
                ┌──────────────┼──────────────┐
                │              │              │
                ▼              ▼              ▼
        ┌───────────┐  ┌───────────┐  ┌───────────┐
        │  Wallet   │  │ Validator │  │Watchtower │
        │   Web     │  │ Dashboard │  │  Monitor  │
        └─────┬─────┘  └─────┬─────┘  └─────┬─────┘
              │              │              │
              │              │              │
              │ WebSocket    │ WebSocket    │ WebSocket
              │ (wss://)     │ (wss://)     │ (wss://)
              │              │              │
              └──────────────┼──────────────┘
                             ▼
                  ┌────────────────────┐
                  │  Ëtrid RPC Node    │
                  │ wss://rpc.etrid    │
                  │     .network       │
                  └────────────────────┘
                             │
                             ▼
                  ┌────────────────────┐
                  │  Substrate Chain   │
                  │  (Blockchain)      │
                  └────────────────────┘
```

---

## Build and Deploy Process

```
┌─────────────────────────────────────────────────────────────┐
│ Step 1: Pre-Build Checks                                    │
├─────────────────────────────────────────────────────────────┤
│ • Verify vercel.json configuration                          │
│ • Check environment variables                               │
│ • Validate package.json scripts                             │
│ • Ensure node_modules are up to date                        │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ Step 2: Install Dependencies                                │
├─────────────────────────────────────────────────────────────┤
│ • npm install --legacy-peer-deps                            │
│ • Resolve peer dependency conflicts                         │
│ • Cache node_modules for faster builds                      │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ Step 3: Build Application                                   │
├─────────────────────────────────────────────────────────────┤
│ • npm run build                                             │
│ • Next.js compilation                                       │
│ • Type checking (TypeScript)                                │
│ • Linting (ESLint)                                          │
│ • Static page generation                                    │
│ • Bundle optimization                                       │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ Step 4: Deploy to Vercel                                    │
├─────────────────────────────────────────────────────────────┤
│ • Upload build artifacts                                    │
│ • Create serverless functions                               │
│ • Deploy to Edge Network                                    │
│ • Provision SSL certificate                                 │
│ • Update DNS (if custom domain)                             │
└─────────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│ Step 5: Post-Deploy Verification                            │
├─────────────────────────────────────────────────────────────┤
│ • Health check                                              │
│ • Verify deployment URL                                     │
│ • Test RPC connectivity                                     │
│ • Run smoke tests                                           │
└─────────────────────────────────────────────────────────────┘
```

---

## Environment Configuration

```
┌────────────────────────────────────────────────────────────┐
│                    Environment Variables                    │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  Production (Wallet Web):                                  │
│  ┌──────────────────────────────────────┐                 │
│  │ NEXT_PUBLIC_WS_PROVIDER              │                 │
│  │   = wss://rpc.etrid.network          │                 │
│  │                                       │                 │
│  │ NEXT_PUBLIC_NETWORK_NAME             │                 │
│  │   = Ëtrid MainNet                    │                 │
│  │                                       │                 │
│  │ NEXT_PUBLIC_CHAIN_ID                 │                 │
│  │   = etrid-mainnet                    │                 │
│  │                                       │                 │
│  │ NODE_ENV                              │                 │
│  │   = production                        │                 │
│  └──────────────────────────────────────┘                 │
│                                                             │
│  Staging (Validator Dashboard & Watchtower):               │
│  ┌──────────────────────────────────────┐                 │
│  │ NEXT_PUBLIC_WS_PROVIDER              │                 │
│  │   = wss://rpc.etrid.network          │                 │
│  │                                       │                 │
│  │ NEXT_PUBLIC_APP_NAME                 │                 │
│  │   = [App Specific Name]              │                 │
│  │                                       │                 │
│  │ NODE_ENV                              │                 │
│  │   = production                        │                 │
│  └──────────────────────────────────────┘                 │
└────────────────────────────────────────────────────────────┘
```

---

## Security Headers (All Apps)

```
┌────────────────────────────────────────────────────────────┐
│                     Security Headers                        │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  X-Content-Type-Options: nosniff                           │
│  X-Frame-Options: DENY                                     │
│  X-XSS-Protection: 1; mode=block                           │
│  Referrer-Policy: strict-origin-when-cross-origin          │
│  Permissions-Policy: camera=(), microphone=(), geo=()      │
│                                                             │
│  + Automatic SSL/TLS (HTTPS only)                          │
│  + HSTS (HTTP Strict Transport Security)                   │
└────────────────────────────────────────────────────────────┘
```

---

## API Routes and Rewrites

```
┌────────────────────────────────────────────────────────────┐
│                    Validator Dashboard                      │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  /api/validator/:path* ──▶ /api/validator/:path*          │
│                                                             │
│  Example:                                                   │
│  /api/validator/stats  ──▶ Serverless Function             │
│  /api/validator/list   ──▶ Serverless Function             │
│                                                             │
└────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────┐
│                    Watchtower Monitor                       │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  /api/watchtower/:path* ──▶ /api/watchtower/:path*        │
│                                                             │
│  Example:                                                   │
│  /api/watchtower/channels  ──▶ Serverless Function         │
│  /api/watchtower/alerts    ──▶ Serverless Function         │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

---

## Deployment Regions

```
                    ┌────────────────────────┐
                    │   Primary Region       │
                    │   iad1 (US East)       │
                    │   Virginia, USA        │
                    └───────────┬────────────┘
                                │
                                │ Build & Deploy
                                ▼
                    ┌────────────────────────┐
                    │   Edge Network         │
                    │   (Global CDN)         │
                    ├────────────────────────┤
                    │ • San Francisco        │
                    │ • Washington DC        │
                    │ • Amsterdam            │
                    │ • Singapore            │
                    │ • Tokyo                │
                    │ • Sydney               │
                    │ • São Paulo            │
                    │ • Mumbai               │
                    │ • Hong Kong            │
                    │ • Stockholm            │
                    └────────────────────────┘
```

---

## Function Configuration

```
┌────────────────────────────────────────────────────────────┐
│              Serverless Functions Timeout                   │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  Wallet Web & Validator Dashboard:                         │
│  ┌──────────────────────────────────────┐                 │
│  │ app/**/*.{ts,tsx}                    │                 │
│  │   maxDuration: 30 seconds            │                 │
│  │                                       │                 │
│  │ pages/api/**/*.{ts,tsx}              │                 │
│  │   maxDuration: 30 seconds            │                 │
│  └──────────────────────────────────────┘                 │
│                                                             │
│  Watchtower Monitor:                                       │
│  ┌──────────────────────────────────────┐                 │
│  │ app/**/*.{ts,tsx}                    │                 │
│  │   maxDuration: 60 seconds            │                 │
│  │                                       │                 │
│  │ pages/api/**/*.{ts,tsx}              │                 │
│  │   maxDuration: 60 seconds            │                 │
│  └──────────────────────────────────────┘                 │
└────────────────────────────────────────────────────────────┘
```

---

## Custom Domain Flow

```
┌────────────────────────────────────────────────────────────┐
│                    Custom Domain Setup                      │
└────────────────────────────────────────────────────────────┘

1. Add Domain to Vercel
   └─▶ vercel domains add wallet.etrid.network

2. Configure DNS (at Domain Registrar)
   └─▶ CNAME  wallet  →  cname.vercel-dns.com

3. Vercel Validates DNS
   └─▶ DNS propagation check (can take up to 48h)

4. SSL Certificate Provisioning
   └─▶ Automatic via Let's Encrypt

5. Domain Active
   └─▶ https://wallet.etrid.network


Example Configuration:
┌─────────────────────────────────────────────────────────┐
│ DNS Records for etrid.network                           │
├─────────────────────────────────────────────────────────┤
│                                                          │
│ Type    Name          Value                  TTL        │
│ ────    ────          ─────                  ───        │
│ CNAME   wallet        cname.vercel-dns.com   3600       │
│ CNAME   validators    cname.vercel-dns.com   3600       │
│ CNAME   watchtower    cname.vercel-dns.com   3600       │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## Monitoring and Analytics

```
┌────────────────────────────────────────────────────────────┐
│                    Vercel Analytics                         │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  Metrics Collected:                                         │
│  • Page views                                              │
│  • Unique visitors                                         │
│  • Core Web Vitals (LCP, FID, CLS)                        │
│  • Time to First Byte (TTFB)                              │
│  • Edge network latency                                    │
│  • Function execution time                                 │
│  • Cache hit ratio                                         │
│                                                             │
│  Error Tracking:                                           │
│  • Build errors                                            │
│  • Runtime errors                                          │
│  • Function timeouts                                       │
│  • Failed deployments                                      │
│                                                             │
└────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────┐
│              Optional External Integrations                 │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  • Sentry (Error tracking & performance)                   │
│  • Datadog (APM & Infrastructure monitoring)               │
│  • LogRocket (Session replay)                              │
│  • Mixpanel (Product analytics)                            │
│  • PostHog (Product analytics & feature flags)             │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

---

## CI/CD Integration (Future Enhancement)

```
┌────────────────────────────────────────────────────────────┐
│              GitHub Actions + Vercel CI/CD                  │
└────────────────────────────────────────────────────────────┘

GitHub Event            Vercel Action           Environment
─────────────           ─────────────           ───────────

Push to branch      →   Deploy Preview      →   Preview URL
  feature/*                                     (Staging)

Pull Request        →   Deploy Preview      →   Preview URL
  created/updated        + Comment on PR         (Staging)

Push to main        →   Deploy Production   →   Production URL
  (after merge)          (Auto-deploy)          (Live)

Tag created         →   Deploy Release      →   Tagged Deploy
  v*.*.*                 (Versioned)            (Archived)


Configuration:
┌─────────────────────────────────────────────────────────┐
│ .github/workflows/deploy.yml                            │
├─────────────────────────────────────────────────────────┤
│ name: Deploy to Vercel                                  │
│ on:                                                      │
│   push:                                                  │
│     branches: [main, develop]                           │
│   pull_request:                                          │
│     branches: [main]                                     │
│                                                          │
│ jobs:                                                    │
│   deploy:                                                │
│     runs-on: ubuntu-latest                              │
│     steps:                                               │
│       - uses: actions/checkout@v3                       │
│       - uses: amondnet/vercel-action@v25                │
│         with:                                            │
│           vercel-token: ${{ secrets.VERCEL_TOKEN }}     │
│           vercel-org-id: ${{ secrets.ORG_ID }}          │
│           vercel-project-id: ${{ secrets.PROJECT_ID }}  │
└─────────────────────────────────────────────────────────┘
```

---

## Architecture Summary

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Frontend** | Next.js 14/15 | React-based UI framework |
| **Styling** | Tailwind CSS | Utility-first CSS |
| **API** | Next.js API Routes | Serverless backend functions |
| **Deployment** | Vercel | Cloud platform & CDN |
| **RPC** | WebSocket | Real-time blockchain data |
| **SSL** | Let's Encrypt | Automatic HTTPS |
| **CDN** | Vercel Edge | Global content delivery |
| **Analytics** | Vercel Analytics | Performance monitoring |

---

**Document Version:** 1.0
**Last Updated:** 2025-10-22
**Architecture Type:** Serverless + Edge Computing
