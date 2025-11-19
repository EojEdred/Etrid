# Build Instructions - Ëtrid Wallet Native

Complete guide to build iOS and Android apps from scratch.

## Prerequisites Setup

### macOS (for iOS development)

1. **Install Xcode**
   ```bash
   # Download from App Store or:
   xcode-select --install
   ```

2. **Install CocoaPods**
   ```bash
   sudo gem install cocoapods
   ```

3. **Install Node.js & npm**
   ```bash
   # Using Homebrew
   brew install node@18
   ```

### Linux/Windows (for Android development)

1. **Install Node.js**
   ```bash
   # Ubuntu/Debian
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs

   # Windows: Download from nodejs.org
   ```

2. **Install Android Studio**
   - Download from https://developer.android.com/studio
   - Install Android SDK (API 33)
   - Set up Android emulator

3. **Configure environment variables**
   ```bash
   # Add to ~/.bashrc or ~/.zshrc
   export ANDROID_HOME=$HOME/Android/Sdk
   export PATH=$PATH:$ANDROID_HOME/emulator
   export PATH=$PATH:$ANDROID_HOME/tools
   export PATH=$PATH:$ANDROID_HOME/tools/bin
   export PATH=$PATH:$ANDROID_HOME/platform-tools
   ```

## Initial Setup

### 1. Clone and Install

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Install Node modules
npm install

# Install iOS dependencies (macOS only)
cd ios && pod install && cd ..
```

### 2. Configure Firebase (for Push Notifications)

#### iOS
1. Create project in Firebase Console
2. Download `GoogleService-Info.plist`
3. Place in `ios/EtridWallet/`
4. Add to Xcode project

#### Android
1. Download `google-services.json`
2. Place in `android/app/`

### 3. Configure Deep Linking

#### iOS
1. Open `ios/EtridWallet.xcworkspace` in Xcode
2. Go to Signing & Capabilities
3. Add Associated Domains capability
4. Add domain: `applinks:wallet.etrid.io`

#### Android
1. Already configured in `AndroidManifest.xml`
2. Verify intent filters for deep links

## Development Build

### iOS Development

```bash
# Start Metro bundler
npm start

# In new terminal, run iOS
npm run ios

# Or run on specific simulator
npx react-native run-ios --simulator="iPhone 14 Pro"

# Run on physical device
npx react-native run-ios --device "Your iPhone"
```

**Troubleshooting iOS:**

```bash
# Clean build
cd ios
xcodebuild clean
pod cache clean --all
pod install
cd ..

# Reset Metro cache
npm start -- --reset-cache

# Clean watchman
watchman watch-del-all
```

### Android Development

```bash
# Start Metro bundler
npm start

# In new terminal, run Android
npm run android

# Run on specific device
adb devices  # List devices
npx react-native run-android --deviceId=<device-id>

# Reverse port for debugging
adb reverse tcp:8081 tcp:8081
```

**Troubleshooting Android:**

```bash
# Clean build
cd android
./gradlew clean
cd ..

# Clear cache
rm -rf android/app/build
rm -rf ~/.gradle/caches

# Rebuild
npm run android
```

## Production Build

### iOS Production Build

#### Step 1: Configure Signing

1. Open `ios/EtridWallet.xcworkspace` in Xcode
2. Select `EtridWallet` target
3. Go to **Signing & Capabilities**
4. Select your Team
5. Update Bundle Identifier: `com.etrid.wallet`
6. Enable automatic signing or configure manual signing

#### Step 2: Update Version

Edit `ios/EtridWallet/Info.plist`:
```xml
<key>CFBundleShortVersionString</key>
<string>1.0.0</string>
<key>CFBundleVersion</key>
<string>1</string>
```

#### Step 3: Build Archive

**Option A: Using Xcode**
1. Select "Any iOS Device (arm64)" as target
2. Product → Archive
3. Wait for archive to complete
4. Click "Distribute App"
5. Select "App Store Connect"
6. Upload to TestFlight/App Store

**Option B: Using Command Line**
```bash
# Build for App Store
xcodebuild -workspace ios/EtridWallet.xcworkspace \
  -scheme EtridWallet \
  -configuration Release \
  -archivePath build/EtridWallet.xcarchive \
  archive

# Export IPA
xcodebuild -exportArchive \
  -archivePath build/EtridWallet.xcarchive \
  -exportOptionsPlist ios/ExportOptions.plist \
  -exportPath build/
```

#### Step 4: Upload to App Store

```bash
# Using Transporter app or:
xcrun altool --upload-app \
  -f build/EtridWallet.ipa \
  -t ios \
  -u your-apple-id@email.com \
  -p your-app-specific-password
```

### Android Production Build

#### Step 1: Generate Release Keystore

```bash
cd android/app

# Generate keystore (first time only)
keytool -genkeypair -v \
  -storetype PKCS12 \
  -keystore etrid-release.keystore \
  -alias etrid-wallet \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000

# Enter password and details when prompted
```

**Keep this keystore safe! You'll need it for all future updates.**

#### Step 2: Configure Signing

Create `android/gradle.properties` (add these lines):
```properties
ETRID_UPLOAD_STORE_FILE=etrid-release.keystore
ETRID_UPLOAD_STORE_PASSWORD=your_keystore_password
ETRID_UPLOAD_KEY_ALIAS=etrid-wallet
ETRID_UPLOAD_KEY_PASSWORD=your_key_password
```

Update `android/app/build.gradle`:
```gradle
android {
    signingConfigs {
        release {
            if (project.hasProperty('ETRID_UPLOAD_STORE_FILE')) {
                storeFile file(ETRID_UPLOAD_STORE_FILE)
                storePassword ETRID_UPLOAD_STORE_PASSWORD
                keyAlias ETRID_UPLOAD_KEY_ALIAS
                keyPassword ETRID_UPLOAD_KEY_PASSWORD
            }
        }
    }
    buildTypes {
        release {
            signingConfig signingConfigs.release
            // ... rest of config
        }
    }
}
```

#### Step 3: Update Version

Edit `android/app/build.gradle`:
```gradle
defaultConfig {
    versionCode 1
    versionName "1.0.0"
}
```

#### Step 4: Build APK/AAB

```bash
cd android

# Build APK (for testing)
./gradlew assembleRelease

# Output: android/app/build/outputs/apk/release/app-release.apk

# Build AAB (for Play Store)
./gradlew bundleRelease

# Output: android/app/build/outputs/bundle/release/app-release.aab
```

#### Step 5: Upload to Google Play

1. Go to Google Play Console
2. Create app (if first time)
3. Go to Production → Create new release
4. Upload `app-release.aab`
5. Fill in release notes
6. Review and rollout

**Or using command line:**
```bash
# Install fastlane
gem install fastlane

# Configure and deploy
fastlane supply --aab android/app/build/outputs/bundle/release/app-release.aab
```

## Build Variants

### Debug Build (Development)
```bash
# iOS
npx react-native run-ios --configuration Debug

# Android
cd android && ./gradlew assembleDebug
```

### Release Build (Production)
```bash
# iOS
npx react-native run-ios --configuration Release

# Android
cd android && ./gradlew assembleRelease
```

### Staging Build (Optional)
Create additional build variants in:
- `ios/EtridWallet.xcodeproj` (add Staging scheme)
- `android/app/build.gradle` (add staging buildType)

## Code Signing

### iOS Code Signing

**Automatic Signing (Recommended for development):**
- Xcode manages certificates automatically
- Requires Apple Developer account

**Manual Signing (For CI/CD):**
```bash
# List identities
security find-identity -v -p codesigning

# Sign with specific identity
codesign -s "iPhone Distribution: Your Company" \
  build/EtridWallet.app
```

### Android Code Signing

**Debug Signing:**
- Uses default debug.keystore
- Located in `~/.android/debug.keystore`

**Release Signing:**
- Uses your generated release keystore
- Never commit to git!
- Store securely (1Password, AWS Secrets Manager, etc.)

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build iOS & Android

on:
  push:
    branches: [main]

jobs:
  build-ios:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm install
      - run: cd ios && pod install
      - run: xcodebuild -workspace ios/EtridWallet.xcworkspace ...

  build-android:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm install
      - run: cd android && ./gradlew assembleRelease
```

## Testing Builds

### Internal Testing

**iOS - TestFlight:**
1. Upload build to App Store Connect
2. Add internal testers
3. Distribute build
4. Testers install via TestFlight app

**Android - Internal Testing:**
1. Upload AAB to Play Console
2. Create internal testing track
3. Add tester emails
4. Share download link

### Beta Testing

**iOS - TestFlight External:**
- Up to 10,000 external testers
- Requires App Review for first build

**Android - Open/Closed Testing:**
- Create beta track in Play Console
- Share opt-in URL with testers

## Optimization Checklist

Before releasing:

- [ ] Enable ProGuard/R8 (Android)
- [ ] Enable Hermes (both platforms)
- [ ] Optimize images and assets
- [ ] Remove console.log statements
- [ ] Test on low-end devices
- [ ] Test offline functionality
- [ ] Verify app size (<50MB ideal)
- [ ] Test on different screen sizes
- [ ] Verify deep links work
- [ ] Test push notifications
- [ ] Security audit completed

## Release Checklist

- [ ] Version numbers updated
- [ ] Release notes written
- [ ] Screenshots prepared (App Store/Play Store)
- [ ] App icons finalized
- [ ] Privacy policy updated
- [ ] Terms of service updated
- [ ] Crashlytics/Sentry configured
- [ ] Analytics configured
- [ ] Code signing configured
- [ ] All tests passing
- [ ] No debug code in production
- [ ] API keys secured (not in code)
- [ ] Backup signing keys

## Common Issues

### iOS Build Fails

```bash
# "Command PhaseScriptExecution failed"
cd ios
pod deintegrate && pod install

# "Module not found"
npm start -- --reset-cache
rm -rf ios/build

# CocoaPods issues
pod cache clean --all
pod repo update
```

### Android Build Fails

```bash
# "Task :app:processReleaseResources FAILED"
cd android && ./gradlew clean

# "Duplicate class found"
./gradlew app:dependencies
# Resolve version conflicts in build.gradle

# Out of memory
export GRADLE_OPTS="-Xmx4096m -XX:MaxPermSize=512m"
```

### Metro Bundler Issues

```bash
# Port already in use
lsof -ti:8081 | xargs kill -9

# Cache issues
rm -rf $TMPDIR/metro-*
rm -rf $TMPDIR/haste-*
npm start -- --reset-cache
```

## Support

For build issues, consult:
- React Native Docs: https://reactnative.dev
- Etrid Discord: https://discord.gg/etrid
- Stack Overflow: Tag `react-native`
