# Ëtrid Wallet - React Native Mobile App

Production-ready iOS and Android mobile applications for the Ëtrid blockchain ecosystem.

## Features

### Native Mobile Features
- **Biometric Authentication** - Face ID, Touch ID, Fingerprint
- **Secure Storage** - iOS Keychain, Android Keystore
- **Camera/QR Scanner** - Scan addresses and payment requests
- **Push Notifications** - Real-time transaction alerts
- **NFC Support** - Hardware wallet integration (Ledger)
- **Deep Linking** - Universal links and app schemes

### DeFi Features
- Multi-chain wallet (FlareChain, EDSC-PBC)
- Send/Receive tokens (ÉTR, EDSC)
- NFT Gallery & Marketplace
- Trading & Swaps
- Lending & Borrowing
- AU Bloccard - Over-collateralized debit card
- Savings Goals with auto-save
- Fiat On/Off Ramps

## Project Structure

```
etrid-wallet-native/
├── android/                 # Android native code
│   ├── app/
│   │   ├── src/main/
│   │   │   ├── AndroidManifest.xml
│   │   │   └── java/
│   │   └── build.gradle
│   └── build.gradle
├── ios/                     # iOS native code
│   ├── EtridWallet/
│   │   └── Info.plist
│   └── Podfile
├── src/
│   ├── navigation/          # React Navigation setup
│   │   ├── RootNavigator.tsx
│   │   ├── HomeNavigator.tsx
│   │   ├── SocialNavigator.tsx
│   │   ├── NFTNavigator.tsx
│   │   ├── TradeNavigator.tsx
│   │   └── MoreNavigator.tsx
│   ├── screens/             # Screen components
│   │   ├── HomeScreen.tsx
│   │   ├── SendScreen.tsx
│   │   ├── ReceiveScreen.tsx
│   │   ├── NFTGalleryScreen.tsx
│   │   ├── TradingScreen.tsx
│   │   ├── MoreScreen.tsx
│   │   └── ... (14+ screens)
│   ├── components/          # Reusable components
│   │   └── QRScanner.tsx
│   ├── services/            # Native services
│   │   ├── BiometricService.ts
│   │   ├── SecureStorageService.ts
│   │   ├── PushNotificationService.ts
│   │   └── NFCService.ts
│   ├── hooks/               # Custom React hooks
│   ├── types/               # TypeScript types
│   ├── theme/               # Theme configuration
│   │   ├── colors.ts
│   │   └── index.ts
│   └── utils/               # Utility functions
├── package.json
├── tsconfig.json
├── babel.config.js
├── metro.config.js
└── app.json
```

## Getting Started

### Prerequisites

- Node.js >= 18.0.0
- npm >= 9.0.0
- React Native CLI
- Xcode (for iOS development)
- Android Studio (for Android development)
- CocoaPods (for iOS dependencies)

### Installation

1. **Install dependencies**
   ```bash
   cd etrid-wallet-native
   npm install
   ```

2. **Install iOS dependencies**
   ```bash
   npm run pod-install
   # or
   cd ios && pod install && cd ..
   ```

### Running the App

#### iOS
```bash
# Run on iOS simulator
npm run ios

# Run on specific device
npm run ios -- --device "iPhone 14 Pro"

# Run on physical device
npm run ios -- --device "Your iPhone Name"
```

#### Android
```bash
# Run on Android emulator/device
npm run android

# Run on specific device
npm run android -- --deviceId=<device-id>
```

### Development

```bash
# Start Metro bundler
npm start

# Clear cache and start
npm start -- --reset-cache

# Run tests
npm test

# Lint code
npm run lint
```

## Building for Production

### iOS

1. **Configure signing**
   - Open `ios/EtridWallet.xcworkspace` in Xcode
   - Select your team in Signing & Capabilities
   - Update Bundle Identifier if needed

2. **Build**
   ```bash
   npm run build:ios
   ```

3. **Archive and upload to App Store**
   - Product → Archive in Xcode
   - Upload to App Store Connect

### Android

1. **Generate release keystore**
   ```bash
   keytool -genkeypair -v -storetype PKCS12 -keystore etrid-release.keystore \
     -alias etrid-wallet -keyalg RSA -keysize 2048 -validity 10000
   ```

2. **Configure signing**
   - Add keystore credentials to `android/gradle.properties`
   - Update `android/app/build.gradle` with release signing config

3. **Build**
   ```bash
   npm run build:android
   ```

4. **Output**: `android/app/build/outputs/apk/release/app-release.apk`

## Platform-Specific Configuration

### iOS Permissions (Info.plist)

```xml
<key>NSCameraUsageDescription</key>
<string>We need camera access to scan QR codes</string>

<key>NSFaceIDUsageDescription</key>
<string>We use Face ID to secure your wallet</string>

<key>NSPhotoLibraryUsageDescription</key>
<string>We need access to save QR codes</string>

<key>NFCReaderUsageDescription</key>
<string>We use NFC to connect with hardware wallets</string>
```

### Android Permissions (AndroidManifest.xml)

```xml
<uses-permission android:name="android.permission.CAMERA" />
<uses-permission android:name="android.permission.USE_BIOMETRIC" />
<uses-permission android:name="android.permission.NFC" />
<uses-permission android:name="android.permission.INTERNET" />
```

## Native Features Integration

### Biometric Authentication

```typescript
import {biometricService} from '@/services/BiometricService';

// Check availability
const {available, biometryType} = await biometricService.isBiometricAvailable();

// Authenticate
const success = await biometricService.authenticate('Authenticate to send');
```

### Secure Storage

```typescript
import {secureStorage} from '@/services/SecureStorageService';

// Store sensitive data
await secureStorage.setMnemonic('your seed phrase');

// Retrieve
const mnemonic = await secureStorage.getMnemonic();
```

### QR Scanner

```typescript
import QRScanner from '@/components/QRScanner';

<QRScanner
  onScan={(data) => console.log('Scanned:', data)}
  onClose={() => setShowScanner(false)}
/>
```

### Push Notifications

```typescript
import {pushNotificationService} from '@/services/PushNotificationService';

// Initialize
await pushNotificationService.initialize();

// Send local notification
pushNotificationService.sendLocalNotification(
  'Transaction Confirmed',
  'Your transaction has been confirmed on-chain'
);
```

## Key Differences from Web Version

| Feature | Web (Next.js) | Mobile (React Native) |
|---------|--------------|----------------------|
| Navigation | Next.js Router | React Navigation |
| Styling | Tailwind CSS | StyleSheet API |
| Storage | localStorage | AsyncStorage + Keychain |
| Camera | WebRTC | react-native-camera |
| Biometrics | Web Authentication API | Native Biometrics |
| Push Notifications | Service Workers | Firebase Cloud Messaging |
| Deep Links | URL routing | React Navigation Linking |

## Testing

### Unit Tests
```bash
npm test
```

### E2E Tests (Coming Soon)
```bash
# Detox for iOS/Android
npm run e2e:ios
npm run e2e:android
```

### Manual Testing Checklist

- [ ] Biometric authentication works
- [ ] QR code scanning works
- [ ] Send/Receive transactions
- [ ] NFT gallery displays correctly
- [ ] Trading interface functional
- [ ] Push notifications received
- [ ] Deep links work
- [ ] App works offline (cached data)
- [ ] Secure storage persists after restart

## Troubleshooting

### iOS

**Pod install fails**
```bash
cd ios
pod cache clean --all
pod deintegrate
pod install
```

**Build fails**
```bash
# Clean build
cd ios
xcodebuild clean
rm -rf ~/Library/Developer/Xcode/DerivedData
```

### Android

**Gradle build fails**
```bash
cd android
./gradlew clean
cd ..
npm run android
```

**Metro bundler issues**
```bash
npm start -- --reset-cache
```

## Performance Optimization

- **Hermes** - JavaScript engine for faster startup (enabled)
- **Code Splitting** - React Navigation lazy loading
- **Image Optimization** - react-native-fast-image
- **Memoization** - React.memo, useMemo, useCallback
- **List Performance** - FlatList with proper keys

## Security Best Practices

1. **Never commit sensitive data** (.env, keystores, credentials)
2. **Use Keychain/Keystore** for private keys and mnemonics
3. **Implement biometric authentication** for transactions
4. **Validate all inputs** before sending to blockchain
5. **Use SSL pinning** for API calls (production)
6. **Enable ProGuard** for Android release builds
7. **Obfuscate JavaScript** for iOS/Android releases

## License

Copyright © 2024 Ëtrid Foundation

## Support

- Documentation: https://docs.etrid.io
- Discord: https://discord.gg/etrid
- Twitter: @EtridNetwork
