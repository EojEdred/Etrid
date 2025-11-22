# 🚀 Quick Start - Build Your Mobile Apps

Simple, automated build process for iOS and Android.

---

## ⚡ Super Quick Start

### Android (Linux/Windows/Mac)

```bash
cd /path/to/Etrid/apps/wallet-mobile/etrid-wallet-native

# Run the automated build script
./build-android.sh
```

The script will:
1. ✅ Check your environment (Node.js, Java, Android SDK)
2. ✅ Install npm dependencies
3. ✅ Set up Gradle wrapper
4. ✅ Clean previous builds
5. ✅ Give you options: Debug APK, Release APK, or AAB for Play Store
6. ✅ Build your app!

**Output:** `android/app/build/outputs/apk/release/app-release.apk`

### iOS (macOS only)

```bash
cd /path/to/Etrid/apps/wallet-mobile/etrid-wallet-native

# Run the automated build script
./build-ios.sh
```

The script will:
1. ✅ Check your environment (Xcode, Node.js, CocoaPods)
2. ✅ Install npm dependencies
3. ✅ Install iOS pods
4. ✅ Give you options: Simulator, Device, or App Store Archive
5. ✅ Build your app!

**Output:** `build/EtridWallet.xcarchive` (ready for App Store)

---

## 📋 Prerequisites

### Android

- **Node.js 18+** (you have v22 ✅)
- **Java JDK 17+** (you have JDK 21 ✅)
- **Android Studio** (download from https://developer.android.com/studio)
- **Gradle** (you have 8.14.3 ✅)

Set environment variables:
```bash
export ANDROID_HOME=$HOME/Android/Sdk
export PATH=$PATH:$ANDROID_HOME/platform-tools
```

### iOS

- **macOS** (required - no workarounds)
- **Xcode 15+** (from App Store)
- **CocoaPods** (auto-installed by script)
- **Apple Developer Account** (for device/store builds)

---

## 🎯 Build Options

### Android

When you run `./build-android.sh`, you'll see:

```
Select build type:
1) Debug APK (for testing)
2) Release APK (production, unsigned)
3) Release APK (production, signed)
4) Release AAB (for Play Store)
```

**Recommendation:**
- Start with **Option 1** (Debug) to test
- Use **Option 3** (Signed Release) for distribution
- Use **Option 4** (AAB) for Google Play Store

### iOS

When you run `./build-ios.sh`, you'll see:

```
Select build type:
1) Debug (for Simulator)
2) Debug (for Physical Device)
3) Release Archive (for App Store/TestFlight)
4) Open in Xcode (manual build)
```

**Recommendation:**
- Start with **Option 1** (Simulator) to test
- Use **Option 3** (Archive) for App Store submission
- Use **Option 4** (Xcode) if you prefer manual control

---

## 🔧 Troubleshooting

### "Command not found" errors

**Android:**
```bash
# Install Gradle
sudo apt install gradle  # Linux
brew install gradle      # Mac

# Verify installation
gradle --version
```

**iOS:**
```bash
# Install CocoaPods
sudo gem install cocoapods

# Verify installation
pod --version
```

### Network issues

If build fails with "Could not resolve dependency":

```bash
# Check internet connection
ping google.com

# Clear Gradle cache (Android)
rm -rf ~/.gradle/caches

# Clear CocoaPods cache (iOS)
pod cache clean --all
```

### "ANDROID_HOME not set"

```bash
# Find your Android SDK path (usually one of these):
# Linux: $HOME/Android/Sdk
# Mac: $HOME/Library/Android/sdk

# Set environment variable
export ANDROID_HOME=$HOME/Android/Sdk
export PATH=$PATH:$ANDROID_HOME/platform-tools

# Make permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export ANDROID_HOME=$HOME/Android/Sdk' >> ~/.bashrc
echo 'export PATH=$PATH:$ANDROID_HOME/platform-tools' >> ~/.bashrc
source ~/.bashrc
```

### Build fails with "SDK not found"

**Android:**
1. Open Android Studio
2. Go to: Tools → SDK Manager
3. Install: Android SDK 34, Build Tools 34.0.0
4. Click "Apply"

**iOS:**
1. Open Xcode
2. Xcode → Settings → Locations
3. Select Command Line Tools version

---

## 📦 What You Get

### Android

| Build Type | Output | Size | Use Case |
|------------|--------|------|----------|
| Debug APK | `app-debug.apk` | ~80 MB | Testing on devices/emulators |
| Release APK (unsigned) | `app-release-unsigned.apk` | ~50 MB | Cannot be installed (needs signing) |
| Release APK (signed) | `app-release.apk` | ~50 MB | Direct distribution to users |
| Release AAB | `app-release.aab` | ~30 MB | Google Play Store upload |

### iOS

| Build Type | Output | Size | Use Case |
|------------|--------|------|----------|
| Debug | In Xcode | ~100 MB | Testing on simulator/device |
| Archive | `.xcarchive` folder | ~80 MB | App Store/TestFlight submission |
| IPA | `EtridWallet.ipa` | ~60 MB | Enterprise distribution |

---

## 🚀 Next Steps After Building

### Android

**Option A: Install directly**
```bash
# Connect Android device via USB
adb devices  # Verify device is connected
adb install android/app/build/outputs/apk/release/app-release.apk
```

**Option B: Upload to Google Play**
1. Go to https://play.google.com/console
2. Create new app
3. Upload the AAB file
4. Fill in store listing
5. Submit for review

### iOS

**Option A: TestFlight (Beta Testing)**
1. Open Xcode → Window → Organizer
2. Select your archive
3. Click "Distribute App"
4. Choose "App Store Connect"
5. Select "Upload"
6. Go to https://appstoreconnect.apple.com
7. Add to TestFlight
8. Invite testers

**Option B: App Store**
1. Upload via TestFlight process above
2. In App Store Connect, create new App Store version
3. Select the build
4. Fill in metadata (screenshots, description, etc.)
5. Submit for review

---

## ⏱️ Expected Build Times

| Platform | Build Type | First Build | Incremental |
|----------|-----------|-------------|-------------|
| Android | Debug | 5-10 min | 30 sec |
| Android | Release | 10-15 min | 1-2 min |
| iOS | Debug | 10-15 min | 1-2 min |
| iOS | Release | 15-20 min | 2-3 min |

---

## 🆘 Still Having Issues?

### Check the detailed guide

See `HOW_TO_BUILD.md` for comprehensive troubleshooting and manual build steps.

### Common fixes

```bash
# Reset everything (Android)
cd android
./gradlew clean
cd ..
rm -rf node_modules android/app/build
npm install --legacy-peer-deps

# Reset everything (iOS)
cd ios
xcodebuild clean
pod cache clean --all
pod deintegrate
pod install
cd ..
rm -rf node_modules ~/Library/Developer/Xcode/DerivedData
npm install --legacy-peer-deps
```

### Need internet to build

Both Android and iOS builds require internet access to download dependencies. If you're behind a firewall or proxy:

```bash
# Set proxy for npm
npm config set proxy http://proxy.company.com:8080
npm config set https-proxy http://proxy.company.com:8080

# Set proxy for Gradle (add to ~/.gradle/gradle.properties)
systemProp.http.proxyHost=proxy.company.com
systemProp.http.proxyPort=8080
systemProp.https.proxyHost=proxy.company.com
systemProp.https.proxyPort=8080
```

---

## 🎉 Success!

Once built successfully, you'll have production-ready mobile apps for your Ëtrid wallet!

**Android APK:** Ready to distribute
**iOS IPA:** Ready for App Store

Questions? Check `HOW_TO_BUILD.md` or `BUILD_INSTRUCTIONS.md`
