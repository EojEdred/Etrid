# Firebase Dependencies

Required dependencies for Firebase integration across all platforms.

## PWA/Web (Next.js)

### Install Firebase SDK

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet

# Install Firebase Web SDK
npm install firebase

# Or with specific version
npm install firebase@10.7.1
```

### Dependencies in package.json

```json
{
  "dependencies": {
    "firebase": "^10.7.1"
  }
}
```

### What's Included

The Firebase package includes:
- `firebase/app` - Core Firebase functionality
- `firebase/messaging` - Cloud Messaging (FCM)
- `firebase/analytics` - Analytics
- `firebase/performance` - Performance monitoring

### Import Usage

```typescript
import { initializeApp } from 'firebase/app';
import { getMessaging, getToken, onMessage } from 'firebase/messaging';
import { getAnalytics, logEvent } from 'firebase/analytics';
```

---

## React Native (iOS & Android)

### Install React Native Firebase

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Install React Native Firebase core
npm install @react-native-firebase/app

# Install messaging module
npm install @react-native-firebase/messaging

# Install analytics module
npm install @react-native-firebase/analytics

# Install crashlytics module
npm install @react-native-firebase/crashlytics
```

### Dependencies in package.json

```json
{
  "dependencies": {
    "@react-native-firebase/app": "^18.7.3",
    "@react-native-firebase/messaging": "^18.7.3",
    "@react-native-firebase/analytics": "^18.7.3",
    "@react-native-firebase/crashlytics": "^18.7.3"
  }
}
```

### iOS Setup (CocoaPods)

```bash
cd ios
pod install
cd ..
```

The Podfile already includes:
```ruby
pod 'Firebase/Core'
pod 'Firebase/Messaging'
pod 'Firebase/Analytics'
pod 'Firebase/Crashlytics'
```

### Android Setup (Gradle)

No additional steps needed. Dependencies are managed via:
- `build.gradle` (project level): Google Services plugin
- `app/build.gradle`: Firebase BOM and modules

---

## Optional Dependencies

### Web Push (Alternative to Firebase)

If using custom Web Push implementation:

```bash
npm install web-push
```

Generate VAPID keys:
```bash
npx web-push generate-vapid-keys
```

### Local Notifications (React Native)

For advanced local notification features:

```bash
npm install @notifee/react-native
```

### Analytics Debug (Web)

For enhanced analytics debugging:

```bash
npm install --save-dev @firebase/analytics
```

---

## Version Compatibility

### Current Versions (as of 2025-11-19)

| Package | Version | Notes |
|---------|---------|-------|
| `firebase` | 10.7.1 | Web SDK |
| `@react-native-firebase/app` | 18.7.3 | RN core |
| `@react-native-firebase/messaging` | 18.7.3 | RN FCM |
| `@react-native-firebase/analytics` | 18.7.3 | RN Analytics |
| `@react-native-firebase/crashlytics` | 18.7.3 | RN Crashlytics |

### Compatibility

- **React Native**: 0.70.0 or higher
- **iOS**: 13.0 or higher
- **Android**: API 23 (Android 6.0) or higher
- **Node.js**: 16.0 or higher
- **Next.js**: 13.0 or higher (for PWA)

---

## Installation Scripts

### Complete PWA Setup

```bash
#!/bin/bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet

# Install Firebase
npm install firebase@^10.7.1

# Verify installation
npm list firebase

echo "✅ Firebase installed for PWA"
```

### Complete React Native Setup

```bash
#!/bin/bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Install React Native Firebase modules
npm install @react-native-firebase/app@^18.7.3
npm install @react-native-firebase/messaging@^18.7.3
npm install @react-native-firebase/analytics@^18.7.3
npm install @react-native-firebase/crashlytics@^18.7.3

# iOS - Install pods
echo "Installing iOS pods..."
cd ios
pod install
cd ..

# Android - Clean and rebuild
echo "Cleaning Android build..."
cd android
./gradlew clean
cd ..

echo "✅ Firebase installed for React Native"
```

---

## Verification

### Verify PWA Installation

```bash
cd etrid-wallet
npm list firebase

# Should show:
# etrid-wallet@1.0.0
# └── firebase@10.7.1
```

### Verify React Native Installation

```bash
cd etrid-wallet-native
npm list | grep firebase

# Should show:
# ├── @react-native-firebase/app@18.7.3
# ├── @react-native-firebase/messaging@18.7.3
# ├── @react-native-firebase/analytics@18.7.3
# └── @react-native-firebase/crashlytics@18.7.3
```

### Verify iOS Pods

```bash
cd ios
pod list | grep Firebase

# Should show Firebase pods installed
```

### Verify Android Dependencies

```bash
cd android
./gradlew app:dependencies | grep firebase

# Should show Firebase dependencies
```

---

## Updating Dependencies

### Update PWA Firebase

```bash
cd etrid-wallet

# Check for updates
npm outdated firebase

# Update to latest
npm update firebase

# Or specific version
npm install firebase@latest
```

### Update React Native Firebase

```bash
cd etrid-wallet-native

# Check for updates
npm outdated | grep firebase

# Update all Firebase modules
npm update @react-native-firebase/app
npm update @react-native-firebase/messaging
npm update @react-native-firebase/analytics
npm update @react-native-firebase/crashlytics

# iOS - Update pods
cd ios
pod update Firebase
cd ..
```

---

## Troubleshooting

### Issue: "Cannot find module 'firebase'"

**Solution**:
```bash
cd etrid-wallet
rm -rf node_modules package-lock.json
npm install
```

### Issue: iOS build fails with "Firebase not found"

**Solution**:
```bash
cd ios
rm -rf Pods Podfile.lock
pod install --repo-update
cd ..
```

### Issue: Android build fails with Firebase errors

**Solution**:
```bash
cd android
./gradlew clean
./gradlew --stop
cd ..
rm -rf node_modules
npm install
```

### Issue: Version conflicts

**Solution**:
```bash
# Check React Native Firebase compatibility
# https://rnfirebase.io/

# Use specific compatible versions
npm install @react-native-firebase/app@18.7.3
```

---

## Production Considerations

### Bundle Size (Web)

Firebase Web SDK uses tree-shaking. Only import what you need:

```typescript
// ❌ Don't import entire Firebase
import firebase from 'firebase/app';

// ✅ Import only what you need
import { initializeApp } from 'firebase/app';
import { getMessaging } from 'firebase/messaging';
```

### Native Module Linking (React Native)

React Native Firebase uses autolinking (React Native 0.60+). No manual linking needed.

For older versions:
```bash
react-native link @react-native-firebase/app
```

---

## Summary

### PWA/Web
- **Required**: `firebase` (10.7.1+)
- **Installation**: `npm install firebase`
- **Size**: ~300KB (with tree-shaking)

### React Native
- **Required**:
  - `@react-native-firebase/app`
  - `@react-native-firebase/messaging`
  - `@react-native-firebase/analytics`
  - `@react-native-firebase/crashlytics`
- **Installation**: `npm install` + `pod install` (iOS) + Gradle (Android)
- **Auto-linking**: Yes (RN 0.60+)

### Next Steps

1. Install dependencies
2. Configure Firebase (see `FIREBASE_SETUP.md`)
3. Add configuration files
4. Test on each platform

---

**Last Updated**: 2025-11-19
**Firebase SDK Version**: 10.7.1
**React Native Firebase Version**: 18.7.3
