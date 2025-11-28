# Firebase Android Setup Guide

Complete guide for integrating Firebase into the Ëtrid Wallet Android app.

## Prerequisites

- Android Studio Arctic Fox or later
- Android SDK 23 or higher
- Physical Android device or emulator with Google Play Services
- Firebase project created with Android app added

## Step 1: Download Configuration File

1. Go to Firebase Console → Project Settings
2. Select your Android app (Package name: `com.etrid.wallet`)
3. Download `google-services.json`
4. Save to: `/home/user/Etrid/apps/wallet-mobile/etrid-wallet-native/android/app/google-services.json`

**Important**: Ensure the file is in the `app/` directory, not in the `android/` root directory.

## Step 2: Verify Gradle Configuration

The Gradle files have already been configured with Firebase dependencies.

### Project-level build.gradle

Located at: `android/build.gradle`

```gradle
buildscript {
    dependencies {
        classpath("com.android.tools.build:gradle:7.4.2")
        classpath("com.facebook.react:react-native-gradle-plugin")
        classpath("com.google.gms:google-services:4.4.0")
        classpath("com.google.firebase:firebase-crashlytics-gradle:2.9.9")
    }
}
```

### App-level build.gradle

Located at: `android/app/build.gradle`

```gradle
apply plugin: "com.android.application"
apply plugin: "com.facebook.react"
apply plugin: 'com.google.gms.google-services'
apply plugin: 'com.google.firebase.crashlytics'

// ...

dependencies {
    // Firebase
    implementation platform('com.google.firebase:firebase-bom:32.7.0')
    implementation 'com.google.firebase:firebase-messaging'
    implementation 'com.google.firebase:firebase-analytics'
    implementation 'com.google.firebase:firebase-crashlytics'
}
```

## Step 3: Configure AndroidManifest.xml

Add notification metadata to `android/app/src/main/AndroidManifest.xml`:

```xml
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <!-- Add permissions -->
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />

    <application
        android:name=".MainApplication"
        android:label="@string/app_name"
        android:icon="@mipmap/ic_launcher"
        android:roundIcon="@mipmap/ic_launcher_round"
        android:allowBackup="false"
        android:theme="@style/AppTheme">

        <!-- Firebase Cloud Messaging Service -->
        <service
            android:name="com.google.firebase.messaging.FirebaseMessagingService"
            android:exported="false">
            <intent-filter>
                <action android:name="com.google.firebase.MESSAGING_EVENT" />
            </intent-filter>
        </service>

        <!-- Default notification icon (optional) -->
        <meta-data
            android:name="com.google.firebase.messaging.default_notification_icon"
            android:resource="@drawable/ic_notification" />

        <!-- Default notification color (optional) -->
        <meta-data
            android:name="com.google.firebase.messaging.default_notification_color"
            android:resource="@color/notification_color" />

        <!-- Default notification channel ID (Android 8.0+) -->
        <meta-data
            android:name="com.google.firebase.messaging.default_notification_channel_id"
            android:value="@string/default_notification_channel_id" />

        <!-- Existing activity configuration -->
        <activity
            android:name=".MainActivity"
            android:label="@string/app_name"
            android:configChanges="keyboard|keyboardHidden|orientation|screenLayout|screenSize|smallestScreenSize|uiMode"
            android:launchMode="singleTask"
            android:windowSoftInputMode="adjustResize"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
```

## Step 4: Configure Notification Resources (Optional)

### Create Notification Icon

1. Create notification icon drawable:
   - Location: `android/app/src/main/res/drawable/ic_notification.xml`
   - Use Android Studio's Asset Studio for best results
   - Should be white/transparent for compatibility

Example `ic_notification.xml`:
```xml
<vector xmlns:android="http://schemas.android.com/apk/res/android"
    android:width="24dp"
    android:height="24dp"
    android:viewportWidth="24"
    android:viewportHeight="24">
    <path
        android:fillColor="#FFFFFF"
        android:pathData="M12,2C6.48,2 2,6.48 2,12s4.48,10 10,10 10,-4.48 10,-10S17.52,2 12,2zM12,20c-4.41,0 -8,-3.59 -8,-8s3.59,-8 8,-8 8,3.59 8,8 -3.59,8 -8,8z"/>
</vector>
```

### Configure Colors

Add to `android/app/src/main/res/values/colors.xml`:
```xml
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="notification_color">#2563EB</color>
</resources>
```

### Configure Strings

Add to `android/app/src/main/res/values/strings.xml`:
```xml
<resources>
    <string name="app_name">Ëtrid Wallet</string>
    <string name="default_notification_channel_id">etrid_wallet_default</string>
    <string name="default_notification_channel_name">Ëtrid Wallet Notifications</string>
</resources>
```

## Step 5: Initialize Firebase in MainActivity (Optional)

Firebase is auto-initialized by React Native Firebase. If you need custom initialization:

Located at: `android/app/src/main/java/com/etrid/wallet/MainActivity.java`

```java
package com.etrid.wallet;

import com.facebook.react.ReactActivity;
import com.facebook.react.ReactActivityDelegate;
import com.facebook.react.defaults.DefaultNewArchitectureEntryPoint;
import com.facebook.react.defaults.DefaultReactActivityDelegate;

public class MainActivity extends ReactActivity {
  @Override
  protected String getMainComponentName() {
    return "EtridWallet";
  }

  @Override
  protected ReactActivityDelegate createReactActivityDelegate() {
    return new DefaultReactActivityDelegate(
        this,
        getMainComponentName(),
        DefaultNewArchitectureEntryPoint.getFabricEnabled()
    );
  }
}
```

## Step 6: Create Notification Channels (Android 8.0+)

Create notification channels in JavaScript (recommended) or native code.

### JavaScript Implementation (in App.tsx or index.js):

```typescript
import { Platform } from 'react-native';
import notifee from '@notifee/react-native';

async function createNotificationChannels() {
  if (Platform.OS === 'android') {
    await notifee.createChannel({
      id: 'etrid_wallet_default',
      name: 'Ëtrid Wallet Notifications',
      importance: 4, // HIGH
      sound: 'default',
      vibration: true,
    });

    await notifee.createChannel({
      id: 'transactions',
      name: 'Transaction Notifications',
      importance: 4, // HIGH
      sound: 'default',
    });

    await notifee.createChannel({
      id: 'security',
      name: 'Security Alerts',
      importance: 5, // MAX
      sound: 'default',
      vibration: true,
    });
  }
}
```

## Step 7: Build and Test

### Clean and Build

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Clean previous builds
cd android && ./gradlew clean && cd ..

# Build debug APK
cd android && ./gradlew assembleDebug && cd ..

# Or run on device/emulator
npx react-native run-android
```

### Install on Device

```bash
# Via React Native CLI (device connected via USB)
npx react-native run-android

# Or manually install APK
adb install android/app/build/outputs/apk/debug/app-debug.apk
```

### Test Notifications

1. Launch app on device/emulator
2. Accept notification permission when prompted
3. Check Logcat for FCM token:
   ```bash
   adb logcat | grep "FCM Token"
   ```
4. Copy the token
5. Go to Firebase Console → Cloud Messaging
6. Click "Send test message"
7. Paste the token
8. Click "Test"
9. Verify notification appears

### Test Foreground Notifications

1. Keep app open
2. Send test notification from Firebase Console
3. Should see in-app notification or alert

### Test Background Notifications

1. Put app in background (press home button)
2. Send test notification from Firebase Console
3. Should see notification in notification tray

### Test Notification Tap

1. Send notification with app in background
2. Tap notification in tray
3. App should open and handle notification data

### Test Analytics

```bash
# Enable debug mode
adb shell setprop debug.firebase.analytics.app com.etrid.wallet

# In app, log events
import { firebaseService } from './src/services/FirebaseService';
firebaseService.logEvent('test_event', { platform: 'android' });

# Check Firebase Console → Analytics → DebugView

# Disable debug mode
adb shell setprop debug.firebase.analytics.app .none.
```

### Test Crashlytics

```typescript
import { firebaseService } from './src/services/FirebaseService';

// Test error reporting (development only)
firebaseService.testCrashReporting();

// Force crash (for testing)
// firebaseService.forceCrash();
// Note: Restart app to send crash report
```

## Step 8: Debug Configuration

### View Logcat Logs

```bash
# All logs
adb logcat

# Firebase only
adb logcat | grep -i firebase

# FCM only
adb logcat | grep -i "FCM"

# Crashlytics only
adb logcat | grep -i crashlytics

# Clear logs
adb logcat -c
```

### Enable Verbose Logging

Add to `android/app/src/main/res/values/strings.xml`:
```xml
<string name="firebase_database_logging_enabled">true</string>
<string name="google_analytics_adid_collection_enabled">false</string>
```

## Step 9: Proguard Configuration (for Release Builds)

Add to `android/app/proguard-rules.pro`:

```proguard
# Firebase
-keepattributes Signature
-keepattributes *Annotation*
-keepattributes EnclosingMethod
-keepattributes InnerClasses

# Firebase Messaging
-keep class com.google.firebase.messaging.** { *; }
-keep class com.google.android.gms.** { *; }

# Firebase Crashlytics
-keepattributes SourceFile,LineNumberTable
-keep public class * extends java.lang.Exception
-keep class com.google.firebase.crashlytics.** { *; }
-dontwarn com.google.firebase.crashlytics.**

# React Native Firebase
-keep class io.invertase.firebase.** { *; }
-dontwarn io.invertase.firebase.**
```

## Step 10: Production Setup

### Generate Release Keystore

```bash
cd android/app

keytool -genkeypair -v -storetype PKCS12 \
  -keystore etrid-wallet-release.keystore \
  -alias etrid-wallet \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000 \
  -storepass YOUR_PASSWORD \
  -keypass YOUR_PASSWORD
```

### Configure Release Signing

Create `android/gradle.properties` (or add to existing):
```properties
ETRID_RELEASE_STORE_FILE=etrid-wallet-release.keystore
ETRID_RELEASE_KEY_ALIAS=etrid-wallet
ETRID_RELEASE_STORE_PASSWORD=YOUR_PASSWORD
ETRID_RELEASE_KEY_PASSWORD=YOUR_PASSWORD
```

Update `android/app/build.gradle`:
```gradle
signingConfigs {
    release {
        if (project.hasProperty('ETRID_RELEASE_STORE_FILE')) {
            storeFile file(ETRID_RELEASE_STORE_FILE)
            storePassword ETRID_RELEASE_STORE_PASSWORD
            keyAlias ETRID_RELEASE_KEY_ALIAS
            keyPassword ETRID_RELEASE_KEY_PASSWORD
        }
    }
}

buildTypes {
    release {
        signingConfig signingConfigs.release
        minifyEnabled enableProguardInReleaseBuilds
        proguardFiles getDefaultProguardFile("proguard-android.txt"), "proguard-rules.pro"
    }
}
```

### Build Release APK

```bash
cd android

# Clean build
./gradlew clean

# Build release APK
./gradlew assembleRelease

# APK will be at:
# android/app/build/outputs/apk/release/app-release.apk
```

### Build App Bundle (for Play Store)

```bash
cd android

# Build App Bundle
./gradlew bundleRelease

# AAB will be at:
# android/app/build/outputs/bundle/release/app-release.aab
```

## Troubleshooting

### Issue: "google-services.json not found"

**Solution**:
```bash
# Verify file location
ls -la android/app/google-services.json

# File should exist at this exact path
cp /path/to/google-services.json android/app/
```

### Issue: Build fails with "Duplicate class" error

**Solution**:
```bash
cd android
./gradlew clean
cd ..
rm -rf node_modules
npm install
cd android && ./gradlew assembleDebug
```

### Issue: No FCM token

**Solutions**:
1. Verify Google Play Services is installed
2. Check internet connection
3. Verify `google-services.json` is in correct location
4. Check package name matches Firebase Console

### Issue: Notifications not appearing

**Solutions**:
1. Check device notification settings
2. Ensure notification permission is granted
3. Verify notification channel is created (Android 8.0+)
4. Check Logcat for errors
5. Test on different device

### Issue: Crashlytics not reporting

**Solutions**:
1. Restart app (crashes sent on next launch)
2. Wait 2-5 minutes for reports to appear
3. Verify internet connection
4. Check Firebase Console for errors
5. Ensure Crashlytics is enabled in build.gradle

### Issue: Build fails with "Failed to resolve: firebase-*"

**Solution**:
```bash
# Update Gradle
cd android
./gradlew wrapper --gradle-version=7.6

# Sync and rebuild
./gradlew clean build --refresh-dependencies
```

## Next Steps

1. ✅ Android Firebase setup complete
2. 📱 Test all notification scenarios
3. 📊 Verify analytics in Firebase Console
4. 🐛 Test crash reporting
5. 🔄 Integrate with backend for token management
6. 🚀 Build and deploy to Play Store

## Resources

- [Firebase Android Documentation](https://firebase.google.com/docs/android/setup)
- [React Native Firebase](https://rnfirebase.io/)
- [Android Notifications Guide](https://developer.android.com/develop/ui/views/notifications)
- [Firebase Cloud Messaging Android](https://firebase.google.com/docs/cloud-messaging/android/client)

## Support

For Android-specific Firebase issues:
1. Check Logcat logs
2. Review Firebase Console logs
3. Test on multiple devices
4. Check Google Play Services status
