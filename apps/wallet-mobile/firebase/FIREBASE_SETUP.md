# Firebase Setup Guide for Ëtrid Wallet

Complete guide for setting up Firebase for push notifications, analytics, and crash reporting across all platforms (PWA, iOS, Android).

## Project Creation

1. Go to https://console.firebase.google.com
2. Click "Add Project"
3. Enter project name: **etrid-wallet**
4. Enable Google Analytics: **Yes**
5. Choose or create a Google Analytics account
6. Click "Create Project"

## Apps to Add

### Web App (PWA)

1. In Firebase Console, click **"Add app"** → **Web** (</> icon)
2. App nickname: `Ëtrid Wallet PWA`
3. ✅ Enable Firebase Hosting: **Yes**
4. Click "Register app"
5. Copy the Firebase configuration object
6. Save the config to your `.env.local` file (see Environment Setup)
7. Click "Continue to console"

### iOS App

1. In Firebase Console, click **"Add app"** → **iOS** (Apple icon)
2. Bundle ID: `com.etrid.wallet`
3. App nickname: `Ëtrid Wallet iOS`
4. App Store ID: (optional, leave blank for now)
5. Click "Register app"
6. **Download GoogleService-Info.plist**
7. Save file to: `etrid-wallet-native/ios/EtridWallet/GoogleService-Info.plist`
8. Follow iOS-specific setup instructions below
9. Click "Continue to console"

### Android App

1. In Firebase Console, click **"Add app"** → **Android** (Android icon)
2. Package name: `com.etrid.wallet`
3. App nickname: `Ëtrid Wallet Android`
4. Debug signing certificate SHA-1: (optional, for testing)
5. Click "Register app"
6. **Download google-services.json**
7. Save file to: `etrid-wallet-native/android/app/google-services.json`
8. Follow Android-specific setup instructions below
9. Click "Continue to console"

## Enable Firebase Services

### 1. Cloud Messaging (Push Notifications)

#### Web Configuration
1. Navigate to: **Project Settings** → **Cloud Messaging** tab
2. Scroll to **Web configuration**
3. Under **Web Push certificates**, click **"Generate key pair"**
4. Copy the **VAPID key** (starts with "B...")
5. Save to `.env.local` as `NEXT_PUBLIC_FIREBASE_VAPID_KEY`

#### Server Configuration
1. In the same **Cloud Messaging** tab
2. Copy **Server key** (under Cloud Messaging API - Legacy)
3. Save securely for backend notification sending
4. **Important**: Never commit this key to version control

#### Enable Cloud Messaging API
1. Click the link to enable **Cloud Messaging API** in Google Cloud Console
2. Select your project
3. Click "Enable"

### 2. Analytics

1. Navigate to: **Build** → **Analytics**
2. Analytics should be enabled by default (from project creation)
3. Configure data collection settings:
   - ✅ Enable Google Analytics
   - ✅ Enable DebugView (for testing)
4. Configure events (optional):
   - Set up custom events
   - Configure conversion events

#### Enable Analytics in Google Cloud
1. Go to Google Cloud Console
2. Enable **Google Analytics Data API**
3. This allows programmatic access to analytics data

### 3. Crashlytics (React Native Only)

1. Navigate to: **Build** → **Crashlytics**
2. Click **"Enable Crashlytics"**
3. Follow platform-specific setup:
   - iOS: Follow Xcode integration steps
   - Android: Follow Gradle integration steps
4. Upload debug symbols for better crash reports

### 4. Authentication (Optional - for future features)

1. Navigate to: **Build** → **Authentication**
2. Click **"Get started"**
3. Enable sign-in providers:
   - ✅ Email/Password
   - ✅ Google
   - ✅ Apple (for iOS)
   - ✅ Anonymous (for testing)
4. Configure OAuth redirect domains
5. Set up email templates

## Security Rules

### Firestore Security Rules (if using Firestore)

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    // User data
    match /users/{userId} {
      allow read, write: if request.auth != null && request.auth.uid == userId;
    }

    // Public data (read-only)
    match /public/{document=**} {
      allow read: if true;
      allow write: if false;
    }
  }
}
```

### Storage Security Rules (if using Storage)

```javascript
rules_version = '2';
service firebase.storage {
  match /b/{bucket}/o {
    match /users/{userId}/{allPaths=**} {
      allow read, write: if request.auth != null && request.auth.uid == userId;
    }
  }
}
```

## API Keys and Permissions

### Restrict API Keys (Recommended for Production)

1. Go to: **Google Cloud Console** → **APIs & Services** → **Credentials**
2. Find your API keys (Browser key, Android key, iOS key)
3. Click each key to edit restrictions:

#### Browser Key (Web)
- **Application restrictions**: HTTP referrers
- Add your domains:
  - `localhost:3000/*` (development)
  - `https://yourdomain.com/*` (production)
  - `https://*.vercel.app/*` (if using Vercel)

#### Android Key
- **Application restrictions**: Android apps
- Add package name: `com.etrid.wallet`
- Add SHA-1 certificate fingerprint

#### iOS Key
- **Application restrictions**: iOS apps
- Add bundle ID: `com.etrid.wallet`

### API Restrictions
For each key, restrict to only necessary APIs:
- ✅ Firebase Cloud Messaging API
- ✅ Firebase Installations API
- ✅ Google Analytics Data API
- ✅ Identity Toolkit API (if using Auth)

## Service Account Setup (Backend)

For sending notifications from your backend:

1. Go to: **Project Settings** → **Service Accounts**
2. Click **"Generate new private key"**
3. Download the JSON file
4. **IMPORTANT**: Keep this file secure, never commit to Git!
5. Store in secure location: `/config/serviceAccountKey.json`
6. Add to `.gitignore`: `**/serviceAccountKey.json`

## Testing Setup

### Enable Debug Mode

#### Web
Add query parameter to URL:
```
https://localhost:3000/?debug_mode=1
```

#### iOS
In Xcode, add argument to scheme:
```
-FIRDebugEnabled
```

#### Android
Use ADB command:
```bash
adb shell setprop debug.firebase.analytics.app com.etrid.wallet
```

### Test Cloud Messaging

1. Go to: **Cloud Messaging** → **Send test message**
2. Enter your FCM token (from console logs)
3. Compose test notification
4. Send and verify delivery

## Environment Variables Summary

After completing setup, you should have these values:

**Web/PWA (.env.local)**:
```bash
NEXT_PUBLIC_FIREBASE_API_KEY=AIza...
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=etrid-wallet.firebaseapp.com
NEXT_PUBLIC_FIREBASE_PROJECT_ID=etrid-wallet
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=etrid-wallet.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=123456789
NEXT_PUBLIC_FIREBASE_APP_ID=1:123456789:web:abc123
NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=G-XXXXXXXXXX
NEXT_PUBLIC_FIREBASE_VAPID_KEY=BKJh...
```

**Backend**:
```bash
FIREBASE_SERVICE_ACCOUNT_KEY=/path/to/serviceAccountKey.json
FIREBASE_PROJECT_ID=etrid-wallet
```

## Next Steps

1. ✅ Complete Firebase Console setup
2. ✅ Download all configuration files
3. ✅ Set up environment variables
4. 📱 Follow platform-specific setup guides:
   - PWA: See `FIREBASE_PWA_SETUP.md`
   - iOS: See `FIREBASE_IOS_SETUP.md`
   - Android: See `FIREBASE_ANDROID_SETUP.md`
5. 🧪 Test each platform
6. 🚀 Deploy and monitor

## Troubleshooting

### Common Issues

**Issue**: "Firebase app named '[DEFAULT]' already exists"
- **Solution**: Check for duplicate Firebase initialization

**Issue**: "Messaging: We are unable to register the default service worker"
- **Solution**: Ensure `firebase-messaging-sw.js` is in `/public` directory

**Issue**: "Permission denied" for notifications
- **Solution**: Check browser/device notification settings

**Issue**: iOS notifications not working
- **Solution**: Enable Push Notifications capability in Xcode

**Issue**: Android build fails with "google-services.json not found"
- **Solution**: Ensure file is in `android/app/` directory

## Resources

- [Firebase Documentation](https://firebase.google.com/docs)
- [FCM Documentation](https://firebase.google.com/docs/cloud-messaging)
- [Firebase Console](https://console.firebase.google.com)
- [Google Cloud Console](https://console.cloud.google.com)

## Support

For issues specific to Ëtrid Wallet Firebase setup:
1. Check the troubleshooting section
2. Review platform-specific guides
3. Check Firebase Console logs
4. Review implementation in codebase
