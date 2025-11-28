# Firebase Environment Variables Guide

Complete guide for managing Firebase environment variables across all Ëtrid Wallet platforms.

## Table of Contents
- [PWA/Web Environment Variables](#pwaweb-environment-variables)
- [React Native Environment Variables](#react-native-environment-variables)
- [Backend/Server Environment Variables](#backendserver-environment-variables)
- [Security Best Practices](#security-best-practices)
- [Platform-Specific Setup](#platform-specific-setup)

---

## PWA/Web Environment Variables

### Location
- Development: `/home/user/Etrid/apps/wallet-mobile/etrid-wallet/.env.local`
- Example template: `/home/user/Etrid/apps/wallet-mobile/etrid-wallet/.env.local.example`

### Required Variables

```bash
# Firebase Web SDK Configuration
NEXT_PUBLIC_FIREBASE_API_KEY=AIzaSyBxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=etrid-wallet.firebaseapp.com
NEXT_PUBLIC_FIREBASE_PROJECT_ID=etrid-wallet
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=etrid-wallet.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=123456789012
NEXT_PUBLIC_FIREBASE_APP_ID=1:123456789012:web:abcdef1234567890
NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=G-XXXXXXXXXX

# Firebase Cloud Messaging VAPID Key
NEXT_PUBLIC_FIREBASE_VAPID_KEY=BKJhXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### How to Get These Values

#### 1. Firebase Web Configuration

1. Go to [Firebase Console](https://console.firebase.google.com)
2. Select your project
3. Click **Settings** (gear icon) → **Project settings**
4. Scroll to **Your apps** section
5. Click on your Web app (or add one if not exists)
6. Copy the config object values

#### 2. VAPID Key

1. In Firebase Console → **Project settings**
2. Go to **Cloud Messaging** tab
3. Scroll to **Web configuration**
4. Under **Web Push certificates**, click **Generate key pair**
5. Copy the key (starts with "B...")

### Optional Variables

```bash
# Enable Firebase Analytics debug mode (development only)
NEXT_PUBLIC_FIREBASE_ANALYTICS_DEBUG=true

# API URL (if using custom backend)
NEXT_PUBLIC_API_URL=http://localhost:3001
```

### Setup Instructions

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet

# Copy example file
cp .env.local.example .env.local

# Edit with your values
nano .env.local
# or
vim .env.local

# Verify .env.local is in .gitignore
cat .gitignore | grep .env.local
```

### Verification

```bash
# Start development server
npm run dev

# Check if variables are loaded (open browser console)
# You should see Firebase initialization messages
```

---

## React Native Environment Variables

React Native Firebase uses **native configuration files** instead of environment variables for Firebase setup.

### iOS Configuration

**File**: `ios/EtridWallet/GoogleService-Info.plist`

This file contains:
- Firebase API Key
- Project ID
- App ID
- Google App ID
- Bundle ID
- Storage Bucket
- Database URL
- Messaging Sender ID

**Setup**:
1. Download from Firebase Console → iOS app settings
2. Place in `ios/EtridWallet/` directory
3. Add to Xcode project

### Android Configuration

**File**: `android/app/google-services.json`

This file contains:
- Project info
- Client info
- API keys
- OAuth client IDs

**Setup**:
1. Download from Firebase Console → Android app settings
2. Place in `android/app/` directory
3. Gradle will automatically process it

### Environment Variables (Optional)

For additional configuration, you can use `.env` files with React Native:

```bash
# Install react-native-config (if needed)
npm install react-native-config

# Create .env file
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native
touch .env
```

**Example `.env`**:
```bash
# API Configuration
API_URL=https://api.etrid.com

# Feature Flags
ENABLE_BIOMETRIC=true
ENABLE_NOTIFICATIONS=true

# Environment
ENVIRONMENT=development
```

**Access in code**:
```typescript
import Config from 'react-native-config';

const apiUrl = Config.API_URL;
```

---

## Backend/Server Environment Variables

For sending push notifications and managing Firebase from your backend.

### Required Variables

```bash
# Firebase Admin SDK
FIREBASE_SERVICE_ACCOUNT_KEY=/path/to/serviceAccountKey.json

# Or use individual credentials
FIREBASE_ADMIN_PROJECT_ID=etrid-wallet
FIREBASE_ADMIN_CLIENT_EMAIL=firebase-adminsdk-xxxxx@etrid-wallet.iam.gserviceaccount.com
FIREBASE_ADMIN_PRIVATE_KEY="-----BEGIN PRIVATE KEY-----\nXXXXXXXXXXXXXXXX\n-----END PRIVATE KEY-----\n"
```

### How to Get Service Account Key

1. Go to Firebase Console → **Project settings**
2. Go to **Service accounts** tab
3. Click **Generate new private key**
4. Download the JSON file
5. Store securely (NEVER commit to Git!)

### Setup Instructions

#### Option 1: Using JSON File

```bash
# Store service account key securely
mkdir -p /config/firebase
cp ~/Downloads/serviceAccountKey.json /config/firebase/

# Set environment variable
echo 'FIREBASE_SERVICE_ACCOUNT_KEY=/config/firebase/serviceAccountKey.json' >> .env

# Add to .gitignore
echo '/config/firebase/*.json' >> .gitignore
```

#### Option 2: Using Individual Credentials

```bash
# Extract values from serviceAccountKey.json
cat serviceAccountKey.json | jq -r '.project_id'
cat serviceAccountKey.json | jq -r '.client_email'
cat serviceAccountKey.json | jq -r '.private_key'

# Add to .env
echo 'FIREBASE_ADMIN_PROJECT_ID=your-project-id' >> .env
echo 'FIREBASE_ADMIN_CLIENT_EMAIL=your-service-account-email' >> .env
echo 'FIREBASE_ADMIN_PRIVATE_KEY="your-private-key"' >> .env
```

### Example Backend Code

```typescript
import admin from 'firebase-admin';

// Initialize Firebase Admin
const serviceAccount = require(process.env.FIREBASE_SERVICE_ACCOUNT_KEY);

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount),
  projectId: process.env.FIREBASE_ADMIN_PROJECT_ID,
});

// Send notification
async function sendNotification(token: string, message: any) {
  await admin.messaging().send({
    token,
    notification: message.notification,
    data: message.data,
  });
}
```

---

## Security Best Practices

### 1. Never Commit Sensitive Values

**Files to add to `.gitignore`**:
```
# Environment variables
.env.local
.env.*.local
.env.production.local

# Firebase configuration
**/serviceAccountKey.json
**/GoogleService-Info.plist
**/google-services.json

# Keep only example files
!.env.example
!.env.local.example
```

### 2. Restrict API Keys

#### Web API Key
1. Go to [Google Cloud Console](https://console.cloud.google.com)
2. Navigate to **APIs & Services** → **Credentials**
3. Find your Browser key
4. Click to edit
5. Add **Application restrictions**:
   - Select "HTTP referrers"
   - Add authorized domains:
     ```
     localhost:3000/*
     https://yourdomain.com/*
     https://*.vercel.app/*
     ```
6. Add **API restrictions**:
   - Select "Restrict key"
   - Enable only:
     - Firebase Cloud Messaging API
     - Firebase Installations API
     - Google Analytics Data API
     - Identity Toolkit API (if using Auth)

#### iOS API Key
1. Restriction type: **iOS apps**
2. Add bundle ID: `com.etrid.wallet`

#### Android API Key
1. Restriction type: **Android apps**
2. Add package name: `com.etrid.wallet`
3. Add SHA-1 certificate fingerprint

### 3. Use Different Credentials Per Environment

```bash
# Development
.env.local           # Local development
.env.development     # Dev server

# Staging
.env.staging         # Staging environment

# Production
.env.production      # Production environment
```

### 4. Secret Management

#### For Vercel (PWA)

```bash
# Add environment variables in Vercel dashboard
# Or use Vercel CLI
vercel env add NEXT_PUBLIC_FIREBASE_API_KEY
vercel env add NEXT_PUBLIC_FIREBASE_PROJECT_ID
# ... etc
```

#### For AWS/Other Cloud Providers

Use their secret management services:
- AWS Secrets Manager
- AWS Systems Manager Parameter Store
- Azure Key Vault
- Google Secret Manager

### 5. Rotate Keys Regularly

1. Generate new Firebase VAPID keys periodically
2. Update service account keys annually
3. Rotate API keys if compromised
4. Monitor usage in Firebase Console

---

## Platform-Specific Setup

### PWA/Web

1. Create `.env.local`:
   ```bash
   cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
   cp .env.local.example .env.local
   ```

2. Fill in Firebase credentials

3. Update service worker:
   ```bash
   # Edit public/firebase-messaging-sw.js
   # Replace placeholder values with actual Firebase config
   nano public/firebase-messaging-sw.js
   ```

4. Test locally:
   ```bash
   npm run build
   npm start
   ```

### React Native iOS

1. Download `GoogleService-Info.plist`

2. Add to Xcode:
   ```bash
   # Open Xcode
   open ios/EtridWallet.xcworkspace

   # Drag GoogleService-Info.plist into project
   # Ensure "Copy items if needed" is checked
   ```

3. Verify in Xcode:
   - File is in project navigator
   - Added to target: EtridWallet

### React Native Android

1. Download `google-services.json`

2. Place in correct location:
   ```bash
   cp ~/Downloads/google-services.json android/app/
   ```

3. Verify:
   ```bash
   ls -la android/app/google-services.json
   ```

4. Build and test:
   ```bash
   cd android
   ./gradlew assembleDebug
   ```

---

## Troubleshooting

### Issue: "Firebase not configured" error

**Web**:
```bash
# Check if .env.local exists
ls -la .env.local

# Verify variables are loaded
npm run dev
# Check browser console for Firebase init messages
```

**iOS**:
```bash
# Verify file exists
ls -la ios/EtridWallet/GoogleService-Info.plist

# Check in Xcode project navigator
open ios/EtridWallet.xcworkspace
```

**Android**:
```bash
# Verify file exists
ls -la android/app/google-services.json

# Clean and rebuild
cd android && ./gradlew clean && ./gradlew assembleDebug
```

### Issue: API key restrictions blocking requests

**Solution**:
1. Check error message in console
2. Go to Google Cloud Console → Credentials
3. Edit API key restrictions
4. Add missing domain/app to allowed list

### Issue: Service worker can't access environment variables

**Solution**:
Service workers can't access environment variables directly. You must hardcode the config in `firebase-messaging-sw.js` or inject it during build:

```javascript
// In next.config.js
const nextConfig = {
  webpack: (config, { isServer }) => {
    if (!isServer) {
      // Replace placeholders in service worker
      config.plugins.push(
        new webpack.DefinePlugin({
          'process.env.NEXT_PUBLIC_FIREBASE_API_KEY': JSON.stringify(
            process.env.NEXT_PUBLIC_FIREBASE_API_KEY
          ),
          // ... other variables
        })
      );
    }
    return config;
  },
};
```

---

## Environment Variable Checklist

### Web/PWA
- [ ] `.env.local` created
- [ ] All `NEXT_PUBLIC_FIREBASE_*` variables filled
- [ ] VAPID key added
- [ ] `.env.local` in `.gitignore`
- [ ] Service worker updated with config
- [ ] Tested in browser

### iOS
- [ ] `GoogleService-Info.plist` downloaded
- [ ] File added to Xcode project
- [ ] File in target membership
- [ ] Tested on physical device

### Android
- [ ] `google-services.json` downloaded
- [ ] File placed in `android/app/`
- [ ] Build successful
- [ ] Tested on device

### Backend
- [ ] Service account key downloaded
- [ ] Key stored securely
- [ ] Path added to environment variable
- [ ] Key file in `.gitignore`
- [ ] Tested notification sending

---

## Resources

- [Firebase Environment Configuration](https://firebase.google.com/docs/projects/learn-more#config-files-objects)
- [Next.js Environment Variables](https://nextjs.org/docs/basic-features/environment-variables)
- [React Native Config](https://github.com/luggit/react-native-config)
- [Google Cloud API Key Best Practices](https://cloud.google.com/docs/authentication/api-keys)

---

**Last Updated**: 2025-11-19
