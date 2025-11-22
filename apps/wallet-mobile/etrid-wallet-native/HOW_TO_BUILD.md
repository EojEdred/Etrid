# How to Build Ëtrid Mobile Wallet - iOS & Android Apps

This guide will walk you through building production-ready iOS and Android apps from the React Native codebase.

## 🎯 Quick Summary

- **Android**: Can build on Linux, Windows, or Mac
- **iOS**: Requires Mac with Xcode
- **Dependencies Updated**: Using React Native 0.76.6 (latest) ✅
- **QR Scanner Updated**: Using modern vision-camera ✅

---

## Prerequisites

### For Both Platforms

1. **Node.js 18+** (you have v22 ✅)
   ```bash
   node --version  # Should be v18 or higher
   ```

2. **Install Dependencies**
   ```bash
   cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native
   npm install
   ```

### For Android Only

3. **Java JDK 17+** (you have JDK 21 ✅)
   ```bash
   java -version  # Should show OpenJDK 17 or higher
   ```

4. **Android Studio**
   - Download from: https://developer.android.com/studio
   - Install Android SDK (API 34)
   - Set up Android emulator (optional, for testing)

5. **Environment Variables**
   Add to `~/.bashrc` or `~/.zshrc`:
   ```bash
   export ANDROID_HOME=$HOME/Android/Sdk
   export PATH=$PATH:$ANDROID_HOME/emulator
   export PATH=$PATH:$ANDROID_HOME/platform-tools
   export PATH=$PATH:$ANDROID_HOME/tools
   export PATH=$PATH:$ANDROID_HOME/tools/bin
   ```

   Then run:
   ```bash
   source ~/.bashrc  # or source ~/.zshrc
   ```

### For iOS Only

6. **macOS Required** (iOS builds only work on Mac)

7. **Xcode 15+**
   ```bash
   xcode-select --install
   # Or download full Xcode from App Store
   ```

8. **CocoaPods**
   ```bash
   sudo gem install cocoapods
   cd ios && pod install && cd ..
   ```

---

## 🤖 Building Android App

### Option 1: Build APK for Testing (Recommended First)

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Generate Gradle wrapper if needed
cd android
gradle wrapper --gradle-version=8.10.2
cd ..

# Build debug APK (for testing)
cd android && ./gradlew assembleDebug

# Your APK will be at:
# android/app/build/outputs/apk/debug/app-debug.apk
```

### Option 2: Build Release APK (Production)

1. **Generate Signing Key** (first time only)
   ```bash
   cd android/app
   keytool -genkey -v -keystore etrid-wallet.keystore -alias etrid-wallet-key -keyalg RSA -keysize 2048 -validity 10000
   # Enter password and details when prompted
   # SAVE THIS PASSWORD - you'll need it!
   ```

2. **Configure Signing**

   Create `android/gradle.properties`:
   ```properties
   MYAPP_UPLOAD_STORE_FILE=etrid-wallet.keystore
   MYAPP_UPLOAD_KEY_ALIAS=etrid-wallet-key
   MYAPP_UPLOAD_STORE_PASSWORD=your_keystore_password
   MYAPP_UPLOAD_KEY_PASSWORD=your_key_password
   ```

   Update `android/app/build.gradle`:
   ```gradle
   android {
       ...
       signingConfigs {
           release {
               if (project.hasProperty('MYAPP_UPLOAD_STORE_FILE')) {
                   storeFile file(MYAPP_UPLOAD_STORE_FILE)
                   storePassword MYAPP_UPLOAD_STORE_PASSWORD
                   keyAlias MYAPP_UPLOAD_KEY_ALIAS
                   keyPassword MYAPP_UPLOAD_KEY_PASSWORD
               }
           }
       }
       buildTypes {
           release {
               signingConfig signingConfigs.release
               minifyEnabled true
               proguardFiles getDefaultProguardFile('proguard-android.txt'), 'proguard-rules.pro'
           }
       }
   }
   ```

3. **Build Release APK**
   ```bash
   cd android && ./gradlew assembleRelease

   # Your production APK will be at:
   # android/app/build/outputs/apk/release/app-release.apk
   ```

4. **Build AAB for Play Store** (Alternative to APK)
   ```bash
   cd android && ./gradlew bundleRelease

   # Your AAB will be at:
   # android/app/build/outputs/bundle/release/app-release.aab
   ```

### Testing Android App

```bash
# Install on connected device/emulator
adb devices  # Check device is connected
adb install android/app/build/outputs/apk/debug/app-debug.apk

# Or use npm script
npm run android
```

---

## 🍎 Building iOS App

### Requires macOS

1. **Install iOS Dependencies**
   ```bash
   cd ios
   pod install
   cd ..
   ```

2. **Open in Xcode**
   ```bash
   open ios/EtridWallet.xcworkspace
   ```

3. **Configure Signing**
   - In Xcode, select `EtridWallet` target
   - Go to "Signing & Capabilities"
   - Select your Apple Developer Team
   - Update Bundle Identifier: `com.etrid.wallet` (or your custom domain)
   - Enable Automatic Signing (or configure manual signing)

4. **Build for Testing (Simulator)**
   ```bash
   # Run on iOS simulator
   npm run ios

   # Or specific simulator
   npx react-native run-ios --simulator="iPhone 15 Pro"
   ```

5. **Build for Physical Device**
   - Connect iPhone/iPad via USB
   - In Xcode, select your device as target
   - Click "Run" button (⌘R)
   - Or command line:
   ```bash
   npx react-native run-ios --device "Your iPhone"
   ```

6. **Build Production IPA for App Store**

   **Option A: Using Xcode (Easiest)**
   1. In Xcode, select "Any iOS Device (arm64)" as target
   2. Product → Archive
   3. Wait for archive to complete
   4. Click "Distribute App"
   5. Select "App Store Connect"
   6. Follow wizard to upload

   **Option B: Using Command Line**
   ```bash
   # Build archive
   xcodebuild -workspace ios/EtridWallet.xcworkspace \
     -scheme EtridWallet \
     -configuration Release \
     -archivePath build/EtridWallet.xcarchive \
     archive

   # Export IPA
   xcodebuild -exportArchive \
     -archivePath build/EtridWallet.xcarchive \
     -exportPath build \
     -exportOptionsPlist ios/ExportOptions.plist
   ```

---

## 🔧 Troubleshooting

### Android Issues

**Gradle fails to download dependencies**
```bash
# Try offline mode first
cd android && ./gradlew assembleRelease --offline

# Or clear cache
./gradlew clean
rm -rf ~/.gradle/caches
```

**Build fails with "SDK not found"**
```bash
# Make sure ANDROID_HOME is set
echo $ANDROID_HOME
# Should print /home/username/Android/Sdk

# Install SDK tools
sdkmanager "platforms;android-34" "build-tools;34.0.0"
```

**Out of memory errors**
```bash
# Edit android/gradle.properties, add:
org.gradle.jvmargs=-Xmx4096m -XX:MaxPermSize=512m
```

### iOS Issues

**CocoaPods install fails**
```bash
cd ios
pod cache clean --all
pod deintegrate
pod install
cd ..
```

**Xcode build fails**
```bash
# Clean build folder
cd ios
xcodebuild clean
rm -rf ~/Library/Developer/Xcode/DerivedData
cd ..

# Reset Metro cache
npm start -- --reset-cache
```

**Watchman errors**
```bash
watchman watch-del-all
```

---

## 📦 What You Get

### Android
- **APK**: `app-release.apk` (~50-80 MB)
  - For direct distribution (sideloading)
  - Can upload to third-party app stores

- **AAB**: `app-release.aab` (~30-50 MB)
  - Required for Google Play Store
  - Google generates optimized APKs for each device

### iOS
- **IPA**: `EtridWallet.ipa` (~60-100 MB)
  - For App Store distribution
  - For TestFlight beta testing
  - For enterprise distribution

---

## 🚀 Distribution Options

### Android

1. **Google Play Store**
   - Upload AAB file
   - Reaches billions of users
   - Requires $25 one-time developer fee

2. **Direct Distribution**
   - Share APK file directly
   - Users must enable "Install from Unknown Sources"
   - No fees

3. **Alternative Stores**
   - Amazon Appstore
   - Samsung Galaxy Store
   - F-Droid (for open source)

### iOS

1. **Apple App Store**
   - Upload IPA via Xcode or Transporter
   - Requires $99/year developer account
   - Goes through Apple review (~1-3 days)

2. **TestFlight** (Beta Testing)
   - Upload via App Store Connect
   - Up to 10,000 testers
   - No review required for internal testing

3. **Enterprise Distribution**
   - Requires Apple Enterprise account ($299/year)
   - Can distribute outside App Store
   - For organizations only

---

## ⏱️ Build Times

Expected build times:

- **Android Debug**: 2-5 minutes (first build), 30 seconds (incremental)
- **Android Release**: 5-10 minutes
- **iOS Debug**: 3-7 minutes (first build), 1 minute (incremental)
- **iOS Release**: 10-20 minutes

---

## 📱 Alternative: Use Expo (Easier)

If you want an easier build process, consider converting to Expo:

```bash
# Install Expo CLI
npm install -g expo-cli eas-cli

# Build iOS and Android in the cloud
eas build --platform all

# No Mac required for iOS builds!
# Expo's cloud build service handles everything
```

**Pros:**
- Build iOS without a Mac
- Automatic code signing
- Over-the-air updates
- Easier to maintain

**Cons:**
- Adds ~3 MB to app size
- Some native modules may not be compatible
- Requires Expo account (free tier available)

---

## 📋 Pre-Build Checklist

Before building for production:

- [ ] Update version in `package.json`
- [ ] Update version in `android/app/build.gradle` (versionCode & versionName)
- [ ] Update version in `ios/EtridWallet/Info.plist` (CFBundleShortVersionString & CFBundleVersion)
- [ ] Configure Firebase (optional - for push notifications)
- [ ] Update app icons (already done in this project ✅)
- [ ] Update splash screens (already done ✅)
- [ ] Test on real devices (iOS and Android)
- [ ] Configure deep linking URLs
- [ ] Set up crash reporting (Firebase Crashlytics)
- [ ] Review permissions in AndroidManifest.xml
- [ ] Review permissions in Info.plist
- [ ] Create privacy policy (required for both stores)
- [ ] Create terms of service

---

## 🆘 Need Help?

Common resources:

- **React Native Docs**: https://reactnative.dev/docs/environment-setup
- **Android Studio**: https://developer.android.com/studio/build
- **Xcode**: https://developer.apple.com/xcode/
- **Expo Build**: https://docs.expo.dev/build/introduction/

---

## ✅ Summary

Your React Native wallet is ready to build! The dependencies have been updated to the latest versions, and the code is production-ready.

**Next steps:**
1. Install Android Studio (for Android)
2. Set environment variables
3. Run `cd android && gradle wrapper`
4. Build with `./gradlew assembleRelease`
5. Your APK will be in `android/app/build/outputs/apk/release/`

For iOS, you'll need a Mac with Xcode installed.

Good luck with your mobile wallet launch! 🚀
