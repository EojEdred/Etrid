# Firebase Testing Guide

Comprehensive testing guide for Firebase features across all Ëtrid Wallet platforms.

## Table of Contents
- [Prerequisites](#prerequisites)
- [Web/PWA Testing](#webpwa-testing)
- [iOS Testing](#ios-testing)
- [Android Testing](#android-testing)
- [Analytics Testing](#analytics-testing)
- [Crashlytics Testing](#crashlytics-testing)
- [Integration Testing](#integration-testing)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Tools
- [ ] Firebase Console access
- [ ] Node.js and npm installed
- [ ] Xcode (for iOS testing)
- [ ] Android Studio (for Android testing)
- [ ] Physical iOS device (simulators don't support push notifications)
- [ ] Physical Android device (or emulator with Google Play Services)

### Environment Setup
- [ ] `.env.local` configured with Firebase credentials
- [ ] `GoogleService-Info.plist` in iOS project
- [ ] `google-services.json` in Android project
- [ ] Service account key downloaded (for backend testing)

---

## Web/PWA Testing

### 1. Setup

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet

# Install dependencies
npm install

# Build the app
npm run build

# Start production server (required for service workers)
npm start
```

### 2. Test Push Notifications

#### Step 1: Enable Notifications
1. Open browser: `https://localhost:3000` (or your domain)
2. Open browser DevTools (F12) → Console
3. Look for Firebase initialization messages
4. Accept notification permission when prompted
5. Copy the FCM token from console logs (starts with "FCM Token: ")

#### Step 2: Send Test Notification (Firebase Console)
1. Go to Firebase Console → Cloud Messaging
2. Click **"Send test message"**
3. Paste your FCM token in the field
4. Click **"Test"**
5. **Expected**: Notification appears in browser

#### Step 3: Test Foreground Notifications
1. Keep the app open in browser
2. Send notification from Firebase Console
3. **Expected**:
   - Notification appears at top of page
   - Console logs "Foreground message: {payload}"
   - Custom notification UI shows (if implemented)

#### Step 4: Test Background Notifications
1. Minimize browser or switch to another tab
2. Send notification from Firebase Console
3. **Expected**:
   - Browser notification appears
   - Notification shows Ëtrid Wallet icon
   - Console logs "Background message: {payload}"

#### Step 5: Test Notification Click
1. Send notification with app in background
2. Click on the notification
3. **Expected**:
   - Browser focuses on app tab
   - App navigates to relevant page (if deep link configured)

### 3. Test Analytics

```typescript
// In browser console
import { logEvent } from './lib/firebase/analytics';

// Test custom event
logEvent('test_event', { test_param: 'test_value' });

// Check Firebase Console → Analytics → DebugView
// Should see event appear within seconds
```

### 4. Debug Mode

Enable debug mode to see events in real-time:

```javascript
// Add to URL
https://localhost:3000/?debug_mode=1

// Or run in console
window['ga-disable-GA_MEASUREMENT_ID'] = false;
```

Then check: Firebase Console → Analytics → DebugView

### 5. Verification Checklist

- [ ] Firebase initializes without errors
- [ ] FCM token is generated
- [ ] Notification permission is requested
- [ ] Foreground notifications appear
- [ ] Background notifications appear
- [ ] Notification clicks work
- [ ] Analytics events are logged
- [ ] Service worker is registered
- [ ] No console errors

---

## iOS Testing

### 1. Setup

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Install dependencies
npm install

# Install iOS pods
cd ios && pod install && cd ..

# Open in Xcode
open ios/EtridWallet.xcworkspace
```

### 2. Xcode Configuration

1. Select a physical device (simulators don't support push notifications)
2. Enable signing: Select your team in "Signing & Capabilities"
3. Verify Push Notifications capability is enabled
4. Verify Background Modes → Remote notifications is enabled
5. Build and run (⌘R)

### 3. Test Push Notifications

#### Step 1: Request Permission
1. Launch app on device
2. Accept notification permission when prompted
3. Check Xcode console for FCM token
4. Copy the token (starts with "FCM Token: ")

#### Step 2: Send Test Notification
1. Go to Firebase Console → Cloud Messaging
2. Click "Send test message"
3. Paste FCM token
4. Click "Test"
5. **Expected**: Notification appears on device lock screen

#### Step 3: Test Foreground Notifications
1. Keep app open in foreground
2. Send notification from Firebase Console
3. **Expected**:
   - In-app notification appears (if implemented)
   - Or notification banner appears at top
   - Console logs notification payload

#### Step 4: Test Background Notifications
1. Put app in background (press home button)
2. Send notification from Firebase Console
3. **Expected**:
   - Notification appears on lock screen/notification center
   - Badge count updates (if configured)
   - Sound plays (if configured)

#### Step 5: Test Notification Tap
1. Send notification with app in background
2. Tap notification
3. **Expected**:
   - App opens
   - Navigates to relevant screen (if deep link configured)
   - Console logs notification open event

### 4. Test APNs Connection

```bash
# Check APNs certificate/key in Firebase Console
Firebase Console → Project Settings → Cloud Messaging → iOS app configuration

# Verify APNs key is uploaded
# Verify Key ID and Team ID are correct
```

### 5. Debug Logging

Enable Firebase debug logging:

```bash
# In Xcode, add argument to scheme:
Product → Scheme → Edit Scheme → Run → Arguments → Add:
-FIRDebugEnabled

# Or in code (AppDelegate.mm):
[FIRLogger setLoggerLevel:FIRLoggerLevelDebug];
```

### 6. Test Analytics

```typescript
// In app code
import { firebaseService } from './src/services/FirebaseService';

// Log test event
firebaseService.logEvent('test_event', { platform: 'ios' });

// Check Firebase Console → Analytics → DebugView
```

### 7. Verification Checklist

- [ ] GoogleService-Info.plist is in project
- [ ] Firebase initializes on app launch
- [ ] FCM token is generated
- [ ] APNs device token is obtained
- [ ] Notification permission is granted
- [ ] Foreground notifications work
- [ ] Background notifications work
- [ ] Notification taps open app
- [ ] Analytics events are logged
- [ ] No crashes or errors

---

## Android Testing

### 1. Setup

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Install dependencies
npm install

# Build debug APK
cd android && ./gradlew assembleDebug && cd ..

# Or run on connected device
npx react-native run-android
```

### 2. Android Studio Configuration

1. Open `android/` directory in Android Studio
2. Sync Gradle files
3. Verify `google-services.json` is in `app/` directory
4. Build project
5. Run on physical device or emulator with Google Play Services

### 3. Test Push Notifications

#### Step 1: Request Permission
1. Launch app on device
2. Accept notification permission when prompted
3. Check Logcat for FCM token
4. Copy the token (filter by "FCM Token")

```bash
# View logs
adb logcat | grep "FCM Token"
```

#### Step 2: Send Test Notification
1. Go to Firebase Console → Cloud Messaging
2. Click "Send test message"
3. Paste FCM token
4. Click "Test"
5. **Expected**: Notification appears in notification tray

#### Step 3: Test Foreground Notifications
1. Keep app open
2. Send notification from Firebase Console
3. **Expected**:
   - In-app notification appears (if implemented)
   - Or heads-up notification appears
   - Console logs notification

#### Step 4: Test Background Notifications
1. Put app in background
2. Send notification from Firebase Console
3. **Expected**:
   - Notification appears in notification tray
   - Icon shows in status bar
   - Sound/vibration (if configured)

#### Step 5: Test Notification Tap
1. Send notification with app in background
2. Tap notification
3. **Expected**:
   - App opens
   - Navigates to relevant screen
   - Logs notification open event

### 4. Test Notification Channels

```bash
# View notification channels
adb shell dumpsys notification_listener

# Test different priority levels
# High priority: heads-up notification
# Default priority: notification tray
```

### 5. Debug Logging

Enable debug mode for Analytics:

```bash
# Enable debug mode
adb shell setprop debug.firebase.analytics.app com.etrid.wallet

# Disable debug mode
adb shell setprop debug.firebase.analytics.app .none.
```

### 6. Test Analytics

```typescript
// In app code
import { firebaseService } from './src/services/FirebaseService';

// Log test event
firebaseService.logEvent('test_event', { platform: 'android' });

// Check Firebase Console → Analytics → DebugView
```

### 7. Verification Checklist

- [ ] google-services.json is in app directory
- [ ] Firebase initializes on app launch
- [ ] FCM token is generated
- [ ] Notification permission is granted
- [ ] Foreground notifications work
- [ ] Background notifications work
- [ ] Notification taps open app
- [ ] Notification channels configured
- [ ] Analytics events are logged
- [ ] No crashes or errors

---

## Analytics Testing

### 1. Enable Debug Mode

#### Web
```javascript
// Add to URL
https://localhost:3000/?debug_mode=1
```

#### iOS
```bash
# In Xcode scheme arguments
-FIRDebugEnabled
```

#### Android
```bash
adb shell setprop debug.firebase.analytics.app com.etrid.wallet
```

### 2. Test Events

```typescript
// Test predefined events
import { AnalyticsEvents, logEvent } from './lib/firebase/analytics';

// Wallet created
logEvent(AnalyticsEvents.WALLET_CREATED, { method: 'new' });

// Transaction sent
logEvent(AnalyticsEvents.TRANSACTION_SENT, {
  amount: 100,
  currency: 'ETRID'
});

// Screen view
logEvent(AnalyticsEvents.SCREEN_VIEW, {
  screen_name: 'Dashboard'
});
```

### 3. Verify in Firebase Console

1. Go to: Firebase Console → Analytics → DebugView
2. Select your device from dropdown
3. **Expected**: Events appear in real-time
4. Click event to see parameters

### 4. Check Dashboard (24-hour delay)

1. Go to: Firebase Console → Analytics → Dashboard
2. Wait 24 hours for data to appear
3. Verify events in:
   - Events tab
   - User properties
   - Audiences

### 5. Custom Conversions

1. Go to: Firebase Console → Analytics → Events
2. Mark events as conversions:
   - `wallet_created`
   - `transaction_sent`
   - `nft_purchased`

---

## Crashlytics Testing

### 1. Test Crash Reporting (iOS)

```typescript
// In app code
import crashlytics from '@react-native-firebase/crashlytics';

// Force a crash (testing only!)
crashlytics().crash();

// Or log non-fatal error
try {
  throw new Error('Test error');
} catch (error) {
  crashlytics().recordError(error);
}
```

### 2. Test Crash Reporting (Android)

```typescript
// Same as iOS
import crashlytics from '@react-native-firebase/crashlytics';

crashlytics().crash(); // Force crash
```

### 3. Verify Crash Reports

1. Trigger crash in app
2. Restart app (crashes are sent on next launch)
3. Wait 2-5 minutes
4. Go to: Firebase Console → Crashlytics
5. **Expected**: Crash report appears with stack trace

### 4. Test Custom Logs

```typescript
// Add custom logs before crash
crashlytics().log('User clicked button');
crashlytics().log('Navigation to screen X');

// Set user identifier
crashlytics().setUserId('user-123');

// Set custom attributes
crashlytics().setAttribute('user_type', 'premium');
crashlytics().setAttribute('feature_flag', 'enabled');

// Trigger crash
crashlytics().crash();
```

### 5. Verification Checklist

- [ ] Fatal crashes are reported
- [ ] Non-fatal errors are logged
- [ ] Stack traces are complete
- [ ] Custom logs appear in report
- [ ] User identifiers are set
- [ ] Custom attributes are recorded
- [ ] Debug symbols uploaded (iOS)

---

## Integration Testing

### End-to-End Notification Flow

1. **User registers** → FCM token generated
2. **Token sent to backend** → Stored in database
3. **Event occurs** → Backend sends notification
4. **User receives notification** → FCM delivers
5. **User taps notification** → App opens to relevant screen
6. **Analytics logged** → Event recorded

### Test Scenarios

#### Scenario 1: New Transaction Notification
1. Send transaction from another wallet
2. Backend detects transaction
3. Backend sends notification to user's FCM token
4. Verify notification appears
5. Tap notification
6. App opens to transaction details
7. Analytics logs "notification_opened"

#### Scenario 2: Multiple Devices
1. Register two devices with same user account
2. Send notification to both tokens
3. Verify both devices receive notification
4. Test notification preferences (opt-out on one device)

#### Scenario 3: Token Refresh
1. Trigger token refresh (reinstall app)
2. New token sent to backend
3. Backend updates token in database
4. Send notification to new token
5. Verify notification arrives

---

## Troubleshooting

### Web/PWA Issues

**Problem**: Service worker not registering
```javascript
// Check service worker status
navigator.serviceWorker.getRegistrations().then(console.log);

// Manually register
navigator.serviceWorker.register('/firebase-messaging-sw.js');
```

**Problem**: FCM token not generated
```javascript
// Check messaging support
if ('serviceWorker' in navigator && 'PushManager' in window) {
  console.log('Push notifications supported');
} else {
  console.log('Push notifications NOT supported');
}
```

**Problem**: Notifications blocked
```javascript
// Check permission
console.log('Notification permission:', Notification.permission);

// Request permission
Notification.requestPermission().then(console.log);
```

### iOS Issues

**Problem**: No FCM token
- Check: Push Notifications capability enabled
- Check: App has notification permission
- Check: APNs key uploaded to Firebase
- Check: Running on physical device (not simulator)

**Problem**: Notifications not appearing
```bash
# Check APNs connection in logs
# Look for: "APNs device token received"
# Look for: "FCM token received"
```

**Problem**: Build errors
```bash
# Clean build
cd ios
rm -rf Pods Podfile.lock
pod install --repo-update
cd ..

# Clean Xcode
Product → Clean Build Folder (Shift+Cmd+K)
```

### Android Issues

**Problem**: No FCM token
- Check: Google Play Services installed on device
- Check: google-services.json in correct location
- Check: Package name matches Firebase Console

**Problem**: Build errors
```bash
# Clean build
cd android
./gradlew clean
cd ..

# Invalidate Android Studio caches
File → Invalidate Caches / Restart
```

**Problem**: Notifications not appearing
```bash
# Check notification permission
adb shell dumpsys notification_listener

# Check app has permission
adb shell dumpsys package com.etrid.wallet | grep permission
```

### Analytics Issues

**Problem**: Events not appearing in DebugView
- Check: Debug mode enabled
- Check: Device selected in dropdown
- Check: Network connection
- Wait: Events can take 30-60 seconds

**Problem**: Events not in Dashboard
- Wait: 24-hour delay for dashboard
- Check: Events not filtered out
- Check: App version matches

### Common Issues (All Platforms)

**Problem**: "Permission denied" errors
- Solution: Re-request notification permission
- Solution: Check device notification settings

**Problem**: Old token still used
- Solution: Clear app data and re-generate token
- Solution: Update backend with new token

**Problem**: Firebase not initialized
- Solution: Check environment variables
- Solution: Check Firebase config files
- Solution: Check initialization order

---

## Testing Best Practices

1. **Test on Real Devices**: Always test notifications on physical devices
2. **Test All States**: Foreground, background, killed
3. **Test Network Conditions**: Offline, slow connection
4. **Test Edge Cases**: Token refresh, permission revoked, app reinstall
5. **Monitor Logs**: Always check logs for errors
6. **Use DebugView**: Real-time event monitoring
7. **Document Issues**: Keep track of problems and solutions

---

## Performance Testing

### Metrics to Monitor

- **App startup time**: Firebase init should be fast
- **Battery usage**: Background listeners
- **Network usage**: Event logging frequency
- **Memory usage**: No leaks from Firebase SDK

### Tools

- **Web**: Chrome DevTools Performance tab
- **iOS**: Xcode Instruments
- **Android**: Android Profiler

---

## Security Testing

### Tests to Perform

1. **API Key Restrictions**: Verify keys work only for your app
2. **Token Security**: Tokens should not be exposed in logs (production)
3. **Data Privacy**: No PII in analytics events
4. **Permissions**: Only request when needed

---

## Automated Testing

### Example Test Cases

```typescript
// Jest test for Analytics
import { logEvent } from './lib/firebase/analytics';

test('should log wallet_created event', () => {
  const spy = jest.spyOn(console, 'log');
  logEvent('wallet_created', { method: 'new' });
  expect(spy).toHaveBeenCalled();
});
```

```typescript
// E2E test for notifications
describe('Push Notifications', () => {
  it('should request permission', async () => {
    await device.launchApp({ permissions: { notifications: 'YES' } });
    // Test notification flow
  });
});
```

---

## Conclusion

This guide covers comprehensive testing for all Firebase features across all platforms. Follow each section systematically to ensure proper Firebase integration.

For issues not covered here, check:
- Firebase Console logs
- Platform-specific documentation
- Stack Overflow
- Firebase Support

**Last Updated**: 2025-11-19
