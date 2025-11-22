# Vercel Environment Variables Setup

## PWA (wallet.etrid.com)

Navigate to: Vercel Dashboard → etrid-wallet → Settings → Environment Variables

### Production Variables
```
NEXT_PUBLIC_FIREBASE_API_KEY=your_production_api_key
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=etrid-wallet.firebaseapp.com
NEXT_PUBLIC_FIREBASE_PROJECT_ID=etrid-wallet
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=etrid-wallet.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=123456789012
NEXT_PUBLIC_FIREBASE_APP_ID=1:123456789012:web:abcdef
NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=G-XXXXXXXXXX
NEXT_PUBLIC_FIREBASE_VAPID_KEY=your_vapid_key_here
NEXT_PUBLIC_APP_URL=https://wallet.etrid.com
NEXT_PUBLIC_API_URL=https://api.etrid.com
```

### Preview/Development Variables
Same as above but with different Firebase project or values

## Landing Page (www.wallet.etrid.com)

```
NEXT_PUBLIC_APP_URL=https://wallet.etrid.com
NEXT_PUBLIC_TWITTER=@etrid
NEXT_PUBLIC_GITHUB=etrid
NEXT_PUBLIC_EMAIL=hello@etrid.com
```

## How to Add Variables

1. Go to Vercel Dashboard
2. Select project (etrid-wallet or etrid-wallet-landing)
3. Settings → Environment Variables
4. Add each variable:
   - Key: Variable name
   - Value: Variable value
   - Environment: Production, Preview, Development (check all)
5. Click "Save"
6. Redeploy to apply changes

## Local Development Setup

### PWA (.env.local)

Create `/home/user/Etrid/apps/wallet-mobile/etrid-wallet/.env.local`:

```bash
NEXT_PUBLIC_FIREBASE_API_KEY=your_dev_api_key
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=etrid-wallet-dev.firebaseapp.com
NEXT_PUBLIC_FIREBASE_PROJECT_ID=etrid-wallet-dev
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=etrid-wallet-dev.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=123456789012
NEXT_PUBLIC_FIREBASE_APP_ID=1:123456789012:web:devid
NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=G-DEVXXXXXXX
NEXT_PUBLIC_FIREBASE_VAPID_KEY=your_dev_vapid_key
NEXT_PUBLIC_APP_URL=http://localhost:3000
NEXT_PUBLIC_API_URL=http://localhost:8000
```

### Landing Page (.env.local)

Create `/home/user/Etrid/apps/wallet-mobile/landing-page/.env.local`:

```bash
NEXT_PUBLIC_APP_URL=http://localhost:3000
NEXT_PUBLIC_TWITTER=@etrid
NEXT_PUBLIC_GITHUB=etrid
NEXT_PUBLIC_EMAIL=hello@etrid.com
```

## Security Notes

- Never commit `.env.local` files to git (already in .gitignore)
- Use different Firebase projects for dev/staging/production
- Rotate API keys regularly
- Use Vercel's built-in secret management
- Enable Firebase App Check for production

## Vercel CLI Setup

```bash
# Install Vercel CLI
npm i -g vercel

# Login to Vercel
vercel login

# Link projects
cd apps/wallet-mobile/etrid-wallet
vercel link

cd ../landing-page
vercel link

# Pull environment variables from Vercel
vercel env pull .env.local
```

## Troubleshooting

### Build Fails with Missing Environment Variables

1. Check that all required variables are set in Vercel Dashboard
2. Verify variable names match exactly (case-sensitive)
3. Ensure variables are enabled for the correct environment (Production/Preview/Development)
4. Trigger a new deployment after adding variables

### Environment Variables Not Updating

1. Environment variables are cached during build
2. Trigger a new deployment to pick up changes
3. For local development, restart the dev server

### Firebase Not Working

1. Verify all Firebase credentials are correct
2. Check Firebase project settings match environment
3. Ensure Firebase APIs are enabled in Google Cloud Console
4. Check Firebase App Check if enabled
