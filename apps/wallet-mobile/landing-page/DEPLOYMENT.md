# Deployment Guide

This guide covers deploying the Ëtrid Mobile Wallet Landing Page to various platforms.

## Prerequisites

- Node.js 18 or later
- npm 9 or later
- Git repository
- Domain name configured (wallet.etrid.com)

## Quick Deployment Options

### 1. Vercel (Recommended)

Vercel is the recommended platform for Next.js applications.

#### Steps:

1. **Install Vercel CLI**
   ```bash
   npm install -g vercel
   ```

2. **Login to Vercel**
   ```bash
   vercel login
   ```

3. **Deploy**
   ```bash
   cd /home/user/Etrid/apps/wallet-mobile/landing-page
   vercel deploy --prod
   ```

4. **Configure Domain**
   - Go to Vercel Dashboard
   - Select your project
   - Go to Settings > Domains
   - Add `wallet.etrid.com`
   - Update DNS records as instructed

#### Environment Variables:
Add in Vercel Dashboard > Settings > Environment Variables:
```
NEXT_PUBLIC_SITE_URL=https://wallet.etrid.com
```

### 2. Netlify

#### Steps:

1. **Build the project**
   ```bash
   npm run build
   ```

2. **Install Netlify CLI**
   ```bash
   npm install -g netlify-cli
   ```

3. **Login and Deploy**
   ```bash
   netlify login
   netlify init
   netlify deploy --prod
   ```

#### netlify.toml Configuration:
```toml
[build]
  command = "npm run build"
  publish = ".next"

[[plugins]]
  package = "@netlify/plugin-nextjs"
```

### 3. Docker

#### Dockerfile:
```dockerfile
FROM node:18-alpine AS deps
WORKDIR /app
COPY package*.json ./
RUN npm ci

FROM node:18-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN npm run build

FROM node:18-alpine AS runner
WORKDIR /app
ENV NODE_ENV production
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static
USER nextjs
EXPOSE 3001
ENV PORT 3001
CMD ["node", "server.js"]
```

#### Build and Run:
```bash
docker build -t etrid-landing .
docker run -p 3001:3001 -e NEXT_PUBLIC_SITE_URL=https://wallet.etrid.com etrid-landing
```

### 4. AWS (S3 + CloudFront)

#### Steps:

1. **Build static export**
   ```bash
   npm run build
   ```

2. **Upload to S3**
   ```bash
   aws s3 sync .next/static s3://wallet.etrid.com/static
   aws s3 sync out s3://wallet.etrid.com/
   ```

3. **Configure CloudFront**
   - Create distribution
   - Set origin to S3 bucket
   - Configure SSL certificate
   - Set CNAME to wallet.etrid.com

### 5. Self-Hosted (PM2)

#### Steps:

1. **Install PM2**
   ```bash
   npm install -g pm2
   ```

2. **Build the project**
   ```bash
   npm run build
   ```

3. **Start with PM2**
   ```bash
   pm2 start npm --name "etrid-landing" -- start
   pm2 save
   pm2 startup
   ```

4. **Configure Nginx reverse proxy**
   ```nginx
   server {
       listen 80;
       server_name wallet.etrid.com;

       location / {
           proxy_pass http://localhost:3001;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection 'upgrade';
           proxy_set_header Host $host;
           proxy_cache_bypass $http_upgrade;
       }
   }
   ```

## DNS Configuration

Point your domain to the deployment:

```
A     wallet.etrid.com    -> [Your Server IP or CDN IP]
CNAME www.wallet.etrid.com -> wallet.etrid.com
```

## SSL Certificate

### Using Certbot (Let's Encrypt):
```bash
sudo certbot --nginx -d wallet.etrid.com -d www.wallet.etrid.com
```

## Environment Variables

### Production Variables:
```env
NEXT_PUBLIC_SITE_URL=https://wallet.etrid.com
NODE_ENV=production
```

## Post-Deployment Checklist

- [ ] Verify all pages load correctly
- [ ] Test responsive design on mobile/tablet
- [ ] Check SSL certificate
- [ ] Verify sitemap.xml is accessible
- [ ] Check robots.txt
- [ ] Test all CTA buttons and links
- [ ] Verify PWA manifest
- [ ] Test download links
- [ ] Check console for errors
- [ ] Run Lighthouse audit (aim for 90+ scores)
- [ ] Submit sitemap to Google Search Console
- [ ] Set up analytics (if needed)
- [ ] Configure error monitoring (Sentry, etc.)

## Performance Optimization

### CDN Configuration:
- Enable caching for static assets
- Set cache headers
- Enable compression (gzip/brotli)

### Monitoring:
- Set up uptime monitoring
- Configure performance monitoring
- Set up error tracking

## Rollback Plan

If issues occur:

### Vercel:
```bash
vercel rollback
```

### Docker:
```bash
docker pull etrid-landing:previous
docker run -p 3001:3001 etrid-landing:previous
```

### PM2:
```bash
pm2 stop etrid-landing
# Deploy previous version
pm2 restart etrid-landing
```

## Continuous Deployment

### GitHub Actions Example:
```yaml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: 18
      - run: npm ci
      - run: npm run build
      - uses: amondnet/vercel-action@v20
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.ORG_ID }}
          vercel-project-id: ${{ secrets.PROJECT_ID }}
          vercel-args: '--prod'
```

## Support

For deployment issues:
- Check Next.js deployment docs: https://nextjs.org/docs/deployment
- Vercel support: https://vercel.com/support
- Contact: hello@etrid.com
