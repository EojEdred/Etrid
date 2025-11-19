# Quick Deploy Guide - Ëtrid Wallet to Vercel

Get your PWA and Landing Page deployed to Vercel in under 10 minutes!

## Prerequisites

- Node.js 18+ installed
- Git installed
- GitHub account
- Vercel account (free tier works)
- Firebase project set up

## Step 1: Install Vercel CLI (2 minutes)

```bash
# Install globally
npm i -g vercel

# Login to Vercel
vercel login
```

Follow the browser prompts to authenticate.

## Step 2: Deploy PWA (3 minutes)

```bash
# Navigate to PWA directory
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet

# Create environment file
cat > .env.local << EOF
NEXT_PUBLIC_FIREBASE_API_KEY=your_firebase_api_key
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
NEXT_PUBLIC_FIREBASE_PROJECT_ID=your-project-id
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=your-project.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=123456789
NEXT_PUBLIC_FIREBASE_APP_ID=1:123456789:web:abc123
NEXT_PUBLIC_FIREBASE_VAPID_KEY=your_vapid_key
NEXT_PUBLIC_APP_URL=http://localhost:3000
EOF

# Install dependencies
npm install

# Test build locally
npm run build

# Deploy to Vercel
vercel
```

Answer the prompts:
- Set up and deploy? `Y`
- Which scope? Select your account
- Link to existing project? `N`
- Project name? `etrid-wallet`
- Directory? `./`
- Override settings? `N`

You'll get a preview URL like: `https://etrid-wallet-xxx.vercel.app`

## Step 3: Deploy Landing Page (2 minutes)

```bash
# Navigate to Landing Page directory
cd /home/user/Etrid/apps/wallet-mobile/landing-page

# Install dependencies
npm install

# Test build locally
npm run build

# Deploy to Vercel
vercel
```

Answer the prompts:
- Project name? `etrid-wallet-landing`
- Rest: same as PWA

You'll get a preview URL like: `https://etrid-wallet-landing-xxx.vercel.app`

## Step 4: Add Environment Variables to Vercel (3 minutes)

### For PWA:

1. Go to https://vercel.com/dashboard
2. Select `etrid-wallet` project
3. Click Settings → Environment Variables
4. Add each variable from your `.env.local` file:
   - Click "Add New"
   - Key: `NEXT_PUBLIC_FIREBASE_API_KEY`
   - Value: Your API key
   - Environment: Check all (Production, Preview, Development)
   - Click "Save"
5. Repeat for all Firebase variables

### For Landing Page:

1. Select `etrid-wallet-landing` project
2. Add these variables:
   ```
   NEXT_PUBLIC_APP_URL=https://wallet.etrid.com
   NEXT_PUBLIC_TWITTER=@etrid
   NEXT_PUBLIC_GITHUB=etrid
   NEXT_PUBLIC_EMAIL=hello@etrid.com
   ```

## Step 5: Deploy to Production (1 minute)

```bash
# PWA
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
vercel --prod

# Landing Page
cd /home/user/Etrid/apps/wallet-mobile/landing-page
vercel --prod
```

You'll get production URLs like:
- PWA: `https://etrid-wallet.vercel.app`
- Landing: `https://etrid-wallet-landing.vercel.app`

## Step 6: Set Up Custom Domains (Optional)

1. Go to Vercel Dashboard → Project → Settings → Domains
2. Add your custom domain (e.g., `wallet.etrid.com`)
3. Follow DNS instructions from Vercel
4. Wait for SSL certificate (automatic)

## Step 7: Enable GitHub Auto-Deploy

1. Go to Vercel Dashboard → Project → Settings → Git
2. Click "Connect Git Repository"
3. Select your GitHub repository
4. Connect and authorize
5. Now every push to `main` will auto-deploy!

## Using the Deployment Script

Instead of manual deployment, use the provided script:

```bash
cd /home/user/Etrid/apps/wallet-mobile
./scripts/deploy.sh
```

Select:
1. Which project to deploy (PWA / Landing / Both)
2. Environment (Preview / Production)

Done! The script handles everything.

## Troubleshooting

### Build Fails

```bash
# Clear cache and rebuild
rm -rf .next node_modules
npm install
npm run build
```

### Environment Variables Not Working

- Ensure variables are set in Vercel Dashboard
- Redeploy after adding variables
- Check variable names are exact (case-sensitive)

### Domain Not Working

- Wait 5-48 hours for DNS propagation
- Verify DNS records are correct
- Check with: `dig wallet.etrid.com`

## Next Steps

- [ ] Set up GitHub Actions for auto-deployment
- [ ] Configure custom domains
- [ ] Enable Vercel Analytics
- [ ] Set up monitoring
- [ ] Test PWA installation on mobile

## Need Help?

- Vercel Documentation: https://vercel.com/docs
- Firebase Documentation: https://firebase.google.com/docs
- GitHub Actions: https://docs.github.com/en/actions

## Quick Commands Reference

```bash
# Deploy to preview
vercel

# Deploy to production
vercel --prod

# Check deployment status
vercel ls

# View logs
vercel logs

# Pull environment variables
vercel env pull

# Link project
vercel link

# Remove deployment
vercel rm <deployment-url>

# Rollback to previous deployment
vercel rollback
```

## Performance Tips

1. **Enable Image Optimization**
   - Use Next.js `<Image>` component
   - Configure domains in `next.config.js`

2. **Enable Caching**
   - Already configured in `vercel.json`
   - Verify with browser DevTools

3. **Optimize Bundle Size**
   ```bash
   npm run build
   # Check bundle size in output
   # Use next-bundle-analyzer if needed
   ```

4. **Test Performance**
   - Run Lighthouse in Chrome DevTools
   - Aim for score >90
   - Fix any issues highlighted

## Security Tips

1. **Never commit secrets**
   - `.env.local` is in `.gitignore`
   - Use Vercel environment variables

2. **Restrict Firebase API keys**
   - Go to Google Cloud Console
   - Restrict keys to your domains only

3. **Enable security headers**
   - Already configured in `vercel.json`
   - Verify with security headers checker

4. **Use HTTPS only**
   - Enabled by default on Vercel
   - Force HTTPS in Vercel settings

## Cost Estimate

### Vercel Free Tier (Hobby)
- ✅ 100 GB bandwidth/month
- ✅ Serverless functions
- ✅ SSL certificates
- ✅ 1 concurrent build
- ✅ Enough for small-medium traffic

### When to Upgrade to Pro ($20/month)
- More than 100 GB bandwidth
- Team collaboration needed
- Advanced analytics
- Longer build times
- Commercial use

### Firebase Free Tier (Spark)
- ✅ 10K document reads/day
- ✅ 20K document writes/day
- ✅ 1 GB storage
- ✅ 10 GB hosting bandwidth
- ✅ Enough for development and small apps

Total cost for small app: **$0/month** (both free tiers)
