# Firebase iOS Setup Guide

Complete guide for integrating Firebase into the Ëtrid Wallet iOS app.

## Prerequisites

- Xcode 14.0 or later
- CocoaPods installed
- Physical iOS device (push notifications don't work on simulator)
- Apple Developer account
- Firebase project created with iOS app added

## Step 1: Download Configuration File

1. Go to Firebase Console → Project Settings
2. Select your iOS app (Bundle ID: `com.etrid.wallet`)
3. Download `GoogleService-Info.plist`
4. Save to: `/home/user/Etrid/apps/wallet-mobile/etrid-wallet-native/ios/EtridWallet/`

## Step 2: Add Configuration File to Xcode

1. Open Xcode workspace:
   ```bash
   cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native
   open ios/EtridWallet.xcworkspace
   ```

2. Drag `GoogleService-Info.plist` into Xcode project navigator
3. Ensure "Copy items if needed" is checked
4. Ensure "Add to targets: EtridWallet" is checked
5. Click "Finish"

## Step 3: Install Firebase Pods

The Podfile has already been configured with Firebase dependencies.

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native/ios

# Install pods
pod install

# If you encounter issues, try:
pod install --repo-update

# Or clean install:
rm -rf Pods Podfile.lock
pod install
```

## Step 4: Configure AppDelegate

### Update AppDelegate.mm

Add Firebase import and initialization:

```objc
// At the top of the file, add Firebase import
#import <Firebase.h>

// In the didFinishLaunchingWithOptions method, add Firebase configuration
- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions
{
  // Initialize Firebase (add at the beginning of the method)
  [FIRApp configure];

  // Existing code...
  self.moduleName = @"EtridWallet";
  self.initialProps = @{};

  return [super application:application didFinishLaunchingWithOptions:launchOptions];
}
```

### Add Push Notification Handlers

```objc
// Handle remote notifications
- (void)application:(UIApplication *)application
    didReceiveRemoteNotification:(NSDictionary *)userInfo
          fetchCompletionHandler:(void (^)(UIBackgroundFetchResult))completionHandler
{
  [[FIRMessaging messaging] appDidReceiveMessage:userInfo];
  completionHandler(UIBackgroundFetchResultNewData);
}

// Register for remote notifications
- (void)application:(UIApplication *)application
    didRegisterForRemoteNotificationsWithDeviceToken:(NSData *)deviceToken
{
  [FIRMessaging messaging].APNSToken = deviceToken;
}

// Handle notification failure
- (void)application:(UIApplication *)application
    didFailToRegisterForRemoteNotificationsWithError:(NSError *)error
{
  NSLog(@"Failed to register for remote notifications: %@", error);
}
```

### Complete AppDelegate.mm Example

```objc
#import "AppDelegate.h"
#import <React/RCTBundleURLProvider.h>
#import <Firebase.h>

@implementation AppDelegate

- (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions
{
  // Initialize Firebase
  [FIRApp configure];

  self.moduleName = @"EtridWallet";
  self.initialProps = @{};

  return [super application:application didFinishLaunchingWithOptions:launchOptions];
}

- (NSURL *)sourceURLForBridge:(RCTBridge *)bridge
{
#if DEBUG
  return [[RCTBundleURLProvider sharedSettings] jsBundleURLForBundleRoot:@"index"];
#else
  return [[NSBundle mainBundle] URLForResource:@"main" withExtension:@"jsbundle"];
#endif
}

// Handle remote notifications
- (void)application:(UIApplication *)application
    didReceiveRemoteNotification:(NSDictionary *)userInfo
          fetchCompletionHandler:(void (^)(UIBackgroundFetchResult))completionHandler
{
  [[FIRMessaging messaging] appDidReceiveMessage:userInfo];
  completionHandler(UIBackgroundFetchResultNewData);
}

// Register for remote notifications
- (void)application:(UIApplication *)application
    didRegisterForRemoteNotificationsWithDeviceToken:(NSData *)deviceToken
{
  [FIRMessaging messaging].APNSToken = deviceToken;
}

// Handle notification failure
- (void)application:(UIApplication *)application
    didFailToRegisterForRemoteNotificationsWithError:(NSError *)error
{
  NSLog(@"Failed to register for remote notifications: %@", error);
}

@end
```

## Step 5: Enable Capabilities in Xcode

1. Open Xcode → Select "EtridWallet" target
2. Go to "Signing & Capabilities" tab
3. Click "+ Capability" button
4. Add **Push Notifications**
5. Add **Background Modes**
6. Enable:
   - ✅ Remote notifications
   - ✅ Background fetch (optional)

## Step 6: Configure APNs in Firebase Console

### Create APNs Authentication Key

1. Go to [Apple Developer Portal](https://developer.apple.com/account)
2. Navigate to: Certificates, Identifiers & Profiles → Keys
3. Click "+" to create new key
4. Name it: "Ëtrid Wallet APNs Key"
5. Enable: **Apple Push Notifications service (APNs)**
6. Click "Continue" → "Register"
7. Download the key file (.p8)
8. **Important**: Save the **Key ID** (shown on download page)
9. **Important**: Note your **Team ID** (in top right of Apple Developer Portal)

### Upload APNs Key to Firebase

1. Go to Firebase Console → Project Settings
2. Select your iOS app
3. Scroll to "Cloud Messaging" section
4. Under "APNs Authentication Key", click "Upload"
5. Upload the .p8 file you downloaded
6. Enter **Key ID**
7. Enter **Team ID**
8. Click "Upload"

## Step 7: Update Info.plist

Add the following to `ios/EtridWallet/Info.plist`:

```xml
<key>UIBackgroundModes</key>
<array>
  <string>remote-notification</string>
</array>

<!-- Optional: For local notifications -->
<key>NSUserNotificationAlertStyle</key>
<string>alert</string>
```

## Step 8: Build and Test

### Build the App

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Clean build (optional but recommended)
npx react-native clean

# Build and run on device
npx react-native run-ios --device "Your Device Name"

# Or build in Xcode
open ios/EtridWallet.xcworkspace
# Select your device
# Click Run (⌘R)
```

### Test Notifications

1. Launch app on physical device
2. Accept notification permission when prompted
3. Check Xcode console for FCM token
4. Copy the token (starts with "FCM Token: ")
5. Go to Firebase Console → Cloud Messaging
6. Click "Send test message"
7. Paste the token
8. Click "Test"
9. Verify notification appears

### Test Foreground Notifications

1. Keep app open
2. Send test notification from Firebase Console
3. Should see alert dialog with notification

### Test Background Notifications

1. Put app in background (press home button)
2. Send test notification from Firebase Console
3. Should see notification on lock screen/notification center

### Test Analytics

```typescript
import { firebaseService } from './src/services/FirebaseService';

// Log test event
firebaseService.logEvent('test_event', { platform: 'ios' });

// Check Firebase Console → Analytics → DebugView
```

### Test Crashlytics

```typescript
import { firebaseService } from './src/services/FirebaseService';

// Test error reporting (development only)
firebaseService.testCrashReporting();

// Force crash (for testing)
// firebaseService.forceCrash();
```

## Step 9: Enable Debug Logging (Optional)

Add debug argument to Xcode scheme:

1. In Xcode: Product → Scheme → Edit Scheme
2. Select "Run" on left
3. Go to "Arguments" tab
4. Under "Arguments Passed On Launch", add:
   ```
   -FIRDebugEnabled
   ```
5. Close and rebuild

Now you'll see detailed Firebase logs in Xcode console.

## Step 10: Production Setup

### Disable Debug Logging

Remove `-FIRDebugEnabled` from scheme arguments before releasing.

### Upload Debug Symbols to Firebase

For better crash reports:

1. In Xcode, add a Run Script phase:
   - Select "EtridWallet" target
   - Go to "Build Phases"
   - Click "+" → "New Run Script Phase"
   - Paste:
     ```bash
     "${PODS_ROOT}/FirebaseCrashlytics/run"
     ```
   - Add input files:
     ```
     ${DWARF_DSYM_FOLDER_PATH}/${DWARF_DSYM_FILE_NAME}/Contents/Resources/DWARF/${TARGET_NAME}
     ${BUILT_PRODUCTS_DIR}/${INFOPLIST_PATH}
     ```

2. Build your app (Archive for production)
3. Debug symbols will be automatically uploaded

## Troubleshooting

### Issue: "No FCM token"

**Solutions**:
1. Ensure you're running on a physical device (not simulator)
2. Check notification permission is granted
3. Verify APNs key is uploaded to Firebase Console
4. Check Xcode console for errors
5. Try deleting app and reinstalling

### Issue: "Duplicate symbols" build error

**Solution**:
```bash
cd ios
rm -rf Pods Podfile.lock
pod install --repo-update
```

### Issue: Notifications not appearing

**Solutions**:
1. Check device notification settings (Settings → Notifications → Ëtrid Wallet)
2. Verify APNs key is correctly configured
3. Check Firebase Console for delivery errors
4. Ensure app has notification permission
5. Try on different device

### Issue: "Firebase not initialized"

**Solution**:
1. Verify `GoogleService-Info.plist` is in project
2. Check file is added to target
3. Verify `[FIRApp configure];` is called in AppDelegate

### Issue: Crashlytics not reporting

**Solutions**:
1. Crashes are sent on next app launch (restart app)
2. Wait 2-5 minutes for reports to appear
3. Verify Crashlytics is enabled in Firebase Console
4. Check that debug symbols are uploaded

### Issue: Build fails with "module 'Firebase' not found"

**Solution**:
```bash
cd ios
pod deintegrate
pod install
```

## Next Steps

1. ✅ iOS Firebase setup complete
2. 📱 Test all notification scenarios
3. 📊 Verify analytics in Firebase Console
4. 🐛 Test crash reporting
5. 🚀 Continue with Android setup
6. 🔄 Integrate with backend for token management

## Resources

- [Firebase iOS Documentation](https://firebase.google.com/docs/ios/setup)
- [React Native Firebase](https://rnfirebase.io/)
- [Apple Push Notifications](https://developer.apple.com/documentation/usernotifications)
- [Firebase Cloud Messaging iOS](https://firebase.google.com/docs/cloud-messaging/ios/client)

## Support

For iOS-specific Firebase issues:
1. Check Xcode console logs
2. Review Firebase Console logs
3. Test on multiple devices
4. Check Apple Developer Portal for APNs issues
