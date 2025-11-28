# Google Play Internal Testing Setup

## Prerequisites

- Google Play Developer Account ($25 one-time fee)
- Android Studio Arctic Fox or later
- Physical Android device or emulator
- Valid app signing key

## Step 1: Google Play Console Setup

1. Go to https://play.google.com/console
2. Click "Create app"
3. Fill in details:
   - App name: **Ëtrid Wallet**
   - Default language: **English (United States)**
   - App or game: **App**
   - Free or paid: **Free**
4. Accept declarations:
   - Developer Program Policies
   - US export laws
5. Click "Create app"

## Step 2: Complete App Information

### Store Presence → Main Store Listing

Required before any testing track:

```
App name: Ëtrid Wallet
Short description (80 chars):
Complete DeFi wallet with crypto debit card, trading, and NFTs

Full description (4000 chars):
Ëtrid Wallet is your gateway to decentralized finance. Send, receive, and manage your crypto assets with ease.

Features:
• Multi-chain wallet (Ethereum, Polygon, BSC, and more)
• AU Bloccard - Crypto debit card for everyday spending
• Built-in DEX trading with best price routing
• NFT gallery and marketplace
• DeFi dashboard (staking, lending, yield farming)
• Biometric security (fingerprint, face unlock)
• Hardware wallet support
• dApp browser with Web3 integration

Security First:
• Non-custodial (you control your keys)
• Encrypted local storage
• Biometric authentication
• Transaction signing confirmation
• Anti-phishing protection

AU Bloccard:
Convert crypto to fiat instantly and spend anywhere Visa is accepted.
• Virtual and physical card options
• Instant top-ups from your wallet
• Cashback rewards in crypto
• Global acceptance

Trading & DeFi:
• Swap tokens at best rates
• Provide liquidity and earn fees
• Stake tokens for rewards
• Access yield farming opportunities
• Real-time price charts and alerts

Join thousands of users managing their crypto with Ëtrid Wallet.
```

### App Category
```
Category: Finance
Tags: cryptocurrency, wallet, defi, blockchain, bitcoin
```

### Contact Details
```
Email: support@etrid.com
Phone: +1-XXX-XXX-XXXX (optional)
Website: https://wallet.etrid.com
Privacy Policy: https://wallet.etrid.com/privacy
```

### Store Listing Assets

Upload required:
- App icon (512x512 PNG)
- Feature graphic (1024x500 PNG)
- Phone screenshots (2-8 images, 16:9 or 9:16)
- 7-inch tablet screenshots (optional)
- 10-inch tablet screenshots (optional)
- Promo video (optional, YouTube URL)

## Step 3: App Content Settings

### Privacy Policy
```
URL: https://wallet.etrid.com/privacy
```

### App Access
```
( ) All functionality is available without restrictions
(•) Some functionality requires an account or additional access
```

### Ads
```
(•) No, my app does not contain ads
```

### Content Rating
Complete questionnaire (Finance app, no user-generated content)

### Target Audience
```
Age groups: 18 and over (financial app)
```

### Data Safety
Declare data collection:
- Personal info: Email address, name (optional)
- Financial info: Payment info, purchase history
- Location: Approximate location (for compliance)
- App activity: App interactions
- Device or other IDs: Device ID

Security practices:
- [x] Data is encrypted in transit
- [x] Data is encrypted at rest
- [x] Users can request data deletion
- [ ] Data is not shared with third parties
- [x] Complies with Play Families Policy

## Step 4: Build App Bundle (AAB)

### Generate Upload Keystore (First Time Only)

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native/android

# Generate keystore
keytool -genkeypair -v -storetype PKCS12 \
  -keystore app/etrid-release.keystore \
  -alias etrid-wallet \
  -keyalg RSA \
  -keysize 2048 \
  -validity 10000

# You'll be prompted for:
# - Keystore password (save securely!)
# - Key password (same as keystore password recommended)
# - First and last name: Etrid Technologies
# - Organizational unit: Mobile Development
# - Organization: Etrid
# - City: Melbourne
# - State: VIC
# - Country code: AU
```

**CRITICAL: Backup your keystore!**
```bash
# Store in multiple secure locations:
# 1. Password manager (1Password, LastPass)
# 2. Encrypted cloud storage
# 3. Offline secure storage
#
# If you lose this keystore, you cannot update your app!
```

### Configure Gradle for Release Signing

Create `android/keystore.properties`:
```properties
storeFile=etrid-release.keystore
storePassword=YOUR_KEYSTORE_PASSWORD
keyAlias=etrid-wallet
keyPassword=YOUR_KEY_PASSWORD
```

Add to `android/.gitignore`:
```
keystore.properties
*.keystore
*.jks
```

Update `android/app/build.gradle`:
```gradle
android {
    ...

    // Load keystore
    def keystorePropertiesFile = rootProject.file("keystore.properties")
    def keystoreProperties = new Properties()
    if (keystorePropertiesFile.exists()) {
        keystoreProperties.load(new FileInputStream(keystorePropertiesFile))
    }

    signingConfigs {
        release {
            if (keystorePropertiesFile.exists()) {
                storeFile file(keystoreProperties['storeFile'])
                storePassword keystoreProperties['storePassword']
                keyAlias keystoreProperties['keyAlias']
                keyPassword keystoreProperties['keyPassword']
            }
        }
    }

    buildTypes {
        release {
            signingConfig signingConfigs.release
            minifyEnabled true
            shrinkResources true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }
    }
}
```

### Build the App Bundle

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native/android

# Clean previous builds
./gradlew clean

# Build release AAB
./gradlew bundleRelease

# Output location:
# android/app/build/outputs/bundle/release/app-release.aab
```

### Verify AAB

```bash
# Check AAB size (should be <150MB)
ls -lh app/build/outputs/bundle/release/app-release.aab

# Verify signing
jarsigner -verify -verbose -certs app/build/outputs/bundle/release/app-release.aab
```

## Step 5: Internal Testing Track

**Best for: Team testing (up to 100 testers)**

### Create Internal Testing Release

1. Go to Play Console → **Testing** → **Internal testing**
2. Click **Create new release**
3. Upload AAB: `app-release.aab`
4. Release name: **1.0.0-beta.1**
5. Release notes:

```
Version 1.0.0 Beta 1 - Initial Release

New Features:
• Complete wallet functionality (send, receive, manage crypto)
• AU Bloccard crypto debit card integration
• Multi-chain support (Ethereum, Polygon, BSC)
• Built-in DEX trading
• NFT gallery and marketplace
• DeFi dashboard (staking, lending, yield)
• Biometric authentication
• Hardware wallet support
• dApp browser with Web3 integration

Please Test:
• Wallet creation and import
• Send/receive transactions
• AU Bloccard application flow
• Trading features
• NFT viewing and transfers
• App performance and stability
• Biometric authentication

Known Issues:
• Push notifications may have delays
• Some animations need optimization
• NFT images may load slowly on slower connections

Feedback:
Report bugs to beta@etrid.com or via in-app feedback button.
Include device model, Android version, and steps to reproduce.

Target Devices:
• Android 8.0 (API 26) and above
• 4GB RAM minimum recommended
```

6. Click **Save** → **Review release** → **Start rollout to Internal testing**

### Add Internal Testers

1. Go to **Internal testing** → **Testers** tab
2. Create email list:
   - Click **Create email list**
   - Name: "Internal Team"
   - Add emails (one per line or comma-separated):
   ```
   developer1@etrid.com
   developer2@etrid.com
   qa@etrid.com
   designer@etrid.com
   pm@etrid.com
   ```
3. Click **Save**
4. Copy the **Opt-in URL**
5. Share with testers

### Opt-in URL Format
```
https://play.google.com/apps/internaltest/4701234567890123456
```

### Internal Testing Workflow

1. Developer uploads new AAB
2. Release available **immediately** (no review)
3. Testers install/update from Play Store
4. Testing happens
5. Feedback collected
6. Next build uploaded (repeat)

**Update frequency: Daily or multiple times per day**

## Step 6: Closed Testing Track

**Best for: Broader beta testing (unlimited testers)**

### Create Closed Testing Release

1. Go to Play Console → **Testing** → **Closed testing**
2. Create track: "Beta"
3. Click **Create new release**
4. Upload AAB (same as internal testing)
5. Release name: **1.0.0-beta.2**
6. Release notes (similar to internal testing)
7. Click **Save** → **Review release**
8. **Submit for review** (requires Google review, 24-48 hours)

### Add Closed Testers

Option 1: Email Lists (Managed)
```
1. Create email list: "Beta Testers"
2. Add up to 1000 emails per list
3. Multiple lists supported
4. Controlled access
```

Option 2: Open to Anyone with Link
```
1. Enable "open closed testing"
2. Generate shareable link
3. Anyone with link can join
4. Up to 100,000 testers
```

### Closed Testing Opt-in URL
```
https://play.google.com/apps/testing/com.etrid.wallet
```

### Rollout Percentage

Closed testing supports staged rollouts:
```
10% → 25% → 50% → 100%
```

Benefits:
- Catch issues early with small user group
- Gradually expand exposure
- Monitor metrics at each stage
- Halt rollout if issues detected

## Step 7: Open Testing Track (Optional)

**Best for: Public beta (visible in Play Store)**

### Create Open Testing Release

1. Go to Play Console → **Testing** → **Open testing**
2. Upload release (same process as closed testing)
3. **Visible in Play Store** with "Early Access" badge
4. Anyone can join (no email list needed)
5. Requires Google review (24-48 hours)

### Benefits of Open Testing
- Discover app via Play Store search
- Public reviews and ratings (separate from production)
- Larger user base
- Marketing opportunity

### Considerations
- More public exposure
- Can't control who joins
- Reviews are visible
- Harder to manage feedback

## Step 8: Production Track (When Ready)

### Staged Production Rollout

```
1. Internal testing → 100% (immediate)
2. Closed testing → 50% → 100% (2-3 days)
3. Open testing → 50% → 100% (1 week)
4. Production → 5% → 10% → 25% → 50% → 100% (2-4 weeks)
```

### Production Release Checklist

- [ ] All critical bugs fixed
- [ ] Crash rate <1%
- [ ] ANR rate <0.47%
- [ ] All features complete and tested
- [ ] Legal compliance verified
- [ ] Privacy policy updated
- [ ] Store listing complete
- [ ] Screenshots and graphics ready
- [ ] Pre-launch report reviewed (Play Console)
- [ ] App signing by Google Play enabled (recommended)

## Testing Instructions for Beta Testers

### How to Join Beta

**Email Template to Send Testers:**

```
Subject: You're invited to test Ëtrid Wallet for Android

Hi [Name],

You've been invited to test the Ëtrid Wallet beta for Android!

How to Install:

1. Click this link on your Android device:
   [OPT_IN_URL]

2. Accept the invitation

3. Open Google Play Store

4. Search "Ëtrid Wallet" or visit:
   https://play.google.com/store/apps/details?id=com.etrid.wallet

5. Install (you'll see "Internal test" or "Beta" label)

What to Test:

✓ Wallet creation & backup
✓ Send/receive crypto
✓ AU Bloccard features
✓ Trading & swaps
✓ NFT gallery
✓ Biometric authentication
✓ App performance

How to Report Issues:

1. In-app: Settings → "Report Bug"
2. Email: beta@etrid.com
3. Discord: #android-beta

Include:
- Device model (e.g., Samsung Galaxy S23)
- Android version (e.g., Android 13)
- Steps to reproduce
- Screenshots or screen recording

Expected Issues:

This is beta software - please be patient with:
- Occasional crashes
- UI polish needed
- Feature gaps
- Performance issues

Your feedback helps us build a better wallet!

Thanks,
Ëtrid Team
```

### In-App Beta Badge

Add visual indicator that users are on beta:

```kotlin
// Add to MainActivity or App.kt
if (BuildConfig.BUILD_TYPE == "beta") {
    // Show beta badge in UI
    showBetaBadge()
    // Enable additional logging
    enableDebugLogging()
    // Show feedback button
    showFeedbackButton()
}
```

## Release Management

### Version Numbering

```gradle
// android/app/build.gradle
android {
    defaultConfig {
        // Version code: integer that increases with each release
        versionCode 1     // Increment: 1, 2, 3, 4, ...

        // Version name: human-readable version
        versionName "1.0.0-beta.1"

        // Format: MAJOR.MINOR.PATCH-beta.BUILD
        // Examples:
        // 1.0.0-beta.1
        // 1.0.0-beta.2
        // 1.0.0-rc.1 (release candidate)
        // 1.0.0 (production)
    }
}
```

### Build Variants

```gradle
android {
    flavorDimensions "environment"

    productFlavors {
        dev {
            dimension "environment"
            applicationIdSuffix ".dev"
            versionNameSuffix "-dev"
        }

        beta {
            dimension "environment"
            applicationIdSuffix ".beta"
            versionNameSuffix "-beta"
        }

        prod {
            dimension "environment"
        }
    }
}
```

Build commands:
```bash
# Development build
./gradlew assembleDevDebug

# Beta build
./gradlew bundleBetaRelease

# Production build
./gradlew bundleProdRelease
```

### Rollout Strategy

| Stage | Percentage | Duration | Criteria to Advance |
|-------|-----------|----------|---------------------|
| Internal | 100% | 1-2 days | No critical bugs |
| Closed (10%) | 10% | 2-3 days | Crash rate <2% |
| Closed (25%) | 25% | 2-3 days | Crash rate <1.5% |
| Closed (50%) | 50% | 3-5 days | Crash rate <1% |
| Closed (100%) | 100% | 1 week | All metrics good |
| Production (5%) | 5% | 2-3 days | Monitor closely |
| Production (10%) | 10% | 2-3 days | No issues |
| Production (25%) | 25% | 3-5 days | Stable |
| Production (50%) | 50% | 5-7 days | Metrics good |
| Production (100%) | 100% | - | Full release |

### Halting a Rollout

If issues detected:
1. **Halt rollout immediately** (Play Console → Pause rollout)
2. No new users get the update
3. Existing users keep their version
4. Fix the issue
5. Upload new build
6. Resume or start fresh rollout

## Metrics to Monitor

### Quality Metrics

**Android Vitals (Play Console)**
- Crash rate: <1% (target: <0.5%)
- ANR rate: <0.47% (target: <0.3%)
- Crash-free users: >99%
- Excessive wakeups: <10 per hour
- Stuck wake locks: <1 per hour

**Performance**
- App startup time: <2 seconds (cold start)
- Frame rendering: >95% frames <16ms (60 FPS)
- Battery drain: Minimal background usage

### Engagement Metrics

- Daily active users (DAU)
- Monthly active users (MAU)
- DAU/MAU ratio (stickiness)
- Session length (target: >2 minutes)
- Sessions per day
- Retention (D1, D7, D30)

### Acquisition Metrics

- Total installs
- Install rate (store listing visits → installs)
- Uninstall rate (target: <5%)
- Organic vs. acquired installs

### Feedback Metrics

- Crash reports
- ANR reports
- User reviews
- In-app feedback submissions
- Support tickets

## Automation Scripts

### Automated Build & Upload

```bash
#!/bin/bash
# scripts/deploy-android-beta.sh

set -e  # Exit on error

echo "🚀 Starting Android Beta Deployment"

# Navigate to project
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native/android

# Clean previous builds
echo "🧹 Cleaning previous builds..."
./gradlew clean

# Increment version code
echo "📈 Incrementing version..."
# (Add version increment logic here)

# Build release AAB
echo "🔨 Building release AAB..."
./gradlew bundleRelease

# Verify build
AAB_PATH="app/build/outputs/bundle/release/app-release.aab"
if [ ! -f "$AAB_PATH" ]; then
    echo "❌ Build failed - AAB not found"
    exit 1
fi

echo "✅ Build successful: $AAB_PATH"
echo "📦 Size: $(du -h $AAB_PATH | cut -f1)"

# Upload to Play Console (requires Play Console API setup)
echo "📤 Uploading to Play Console..."
# fastlane supply --track internal --aab "$AAB_PATH"

echo "🎉 Beta deployment complete!"
```

Make executable:
```bash
chmod +x scripts/deploy-android-beta.sh
```

### Fastlane Setup (Recommended)

Install Fastlane:
```bash
gem install fastlane
cd android
fastlane init
```

Create `android/fastlane/Fastfile`:
```ruby
platform :android do
  desc "Deploy to Internal Testing"
  lane :internal do
    gradle(task: "clean bundleRelease")
    upload_to_play_store(
      track: 'internal',
      aab: 'app/build/outputs/bundle/release/app-release.aab',
      skip_upload_metadata: true,
      skip_upload_images: true,
      skip_upload_screenshots: true
    )
  end

  desc "Deploy to Closed Testing"
  lane :beta do
    gradle(task: "clean bundleRelease")
    upload_to_play_store(
      track: 'beta',
      aab: 'app/build/outputs/bundle/release/app-release.aab',
      rollout: '0.1'  # 10% rollout
    )
  end

  desc "Deploy to Production"
  lane :production do
    gradle(task: "clean bundleRelease")
    upload_to_play_store(
      track: 'production',
      aab: 'app/build/outputs/bundle/release/app-release.aab',
      rollout: '0.05'  # 5% rollout
    )
  end
end
```

Usage:
```bash
fastlane android internal
fastlane android beta
fastlane android production
```

## Troubleshooting

### Build Fails

**Issue: "Keystore not found"**
```bash
# Verify keystore exists
ls -la android/app/etrid-release.keystore

# Verify keystore.properties
cat android/keystore.properties
```

**Issue: "Execution failed for task ':app:signReleaseBundle'"**
```bash
# Wrong password - check keystore.properties
# Or keystore corrupted - restore from backup
```

### Upload Fails

**Issue: "Version code already exists"**
```bash
# Increment versionCode in build.gradle
# Each upload must have higher versionCode
```

**Issue: "APK or AAB file is invalid"**
```bash
# Verify signing
jarsigner -verify app/build/outputs/bundle/release/app-release.aab

# Check file integrity
ls -lh app/build/outputs/bundle/release/app-release.aab
```

### Review Rejected

**Issue: "Missing privacy policy"**
```bash
# Add privacy policy URL in Play Console
# Store Presence → Privacy Policy
```

**Issue: "Content rating incomplete"**
```bash
# Complete content rating questionnaire
# Policy → App content → Content rating
```

### Testers Can't Install

**Issue: "App not available in your country"**
```bash
# Check country restrictions
# Play Console → Production → Countries/regions
```

**Issue: "Your device isn't compatible"**
```bash
# Check minSdkVersion (should be 21-26)
# Check device compatibility in Play Console
```

## Security Best Practices

### Keystore Security

- [ ] Never commit keystore to git
- [ ] Store keystore password in secure vault
- [ ] Use different keystores for dev/beta/prod
- [ ] Backup keystore to multiple secure locations
- [ ] Restrict keystore access to authorized personnel

### App Security

- [ ] Enable ProGuard/R8 for release builds
- [ ] Implement certificate pinning
- [ ] Use App Signing by Google Play
- [ ] Enable Play Integrity API
- [ ] Implement root detection
- [ ] Encrypt sensitive data
- [ ] Use Android Keystore for crypto keys

### API Security

- [ ] Use separate API endpoints for beta
- [ ] Implement rate limiting
- [ ] Use API key rotation
- [ ] Enable additional logging for beta
- [ ] Monitor for suspicious activity

## Resources

- [Google Play Console](https://play.google.com/console)
- [Android Distribution Guide](https://developer.android.com/distribute)
- [Testing Tracks Documentation](https://support.google.com/googleplay/android-developer/answer/9845334)
- [Android Vitals](https://developer.android.com/distribute/best-practices/develop/android-vitals)
- [Fastlane Documentation](https://docs.fastlane.tools/getting-started/android/setup/)

## Support

Questions? Contact:
- Email: dev@etrid.com
- Discord: #android-development
- Internal Wiki: https://wiki.etrid.com/android-testing
