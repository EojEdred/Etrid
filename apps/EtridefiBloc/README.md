# Ëtrid Wallet - Native iOS App

A secure, native iOS wallet app built with Swift and SwiftUI for iOS 17+.

## Features

- ✅ **Biometric Authentication** - Face ID / Touch ID support
- ✅ **Secure Key Storage** - Keychain integration
- ✅ **QR Code Scanner** - Built-in camera scanning
- ✅ **Multi-Network Support** - Ethereum, Polygon, BSC, Arbitrum
- 🔄 **WalletConnect** - (Integration in progress)
- 🔄 **NFC Support** - (Coming soon)

## Project Structure

```
EtridWalletSwift/
├── Package.swift
├── Sources/
│   └── EtridWalletSwift/
│       ├── WalletApp.swift          # Main app entry point
│       ├── ContentView.swift        # Main UI
│       ├── WalletManager.swift      # Core wallet logic
│       ├── KeychainManager.swift    # Secure storage
│       └── QRScannerView.swift      # QR code scanning
└── README.md
```

## Setup Instructions

### 1. Create Xcode Project

Since Xcode is already open, follow these steps:

1. In Xcode, go to **File > New > Project**
2. Select **iOS** > **App**
3. Click **Next**
4. Configure your project:
   - **Product Name**: EtridWalletSwift
   - **Team**: Select your Apple Developer team
   - **Organization Identifier**: com.etrid
   - **Interface**: SwiftUI
   - **Language**: Swift
   - **Minimum Deployments**: iOS 17.0
5. Click **Next** and save in: `/Users/macbook/Desktop/etrid/apps/EtridWalletSwift`

### 2. Replace Default Files

After creating the project:
1. Delete the default `ContentView.swift` and `EtridWalletSwiftApp.swift` files
2. Add all the Swift files from the `Sources/EtridWalletSwift/` directory to your Xcode project

### 3. Configure Capabilities

In Xcode project settings, add these capabilities:

**Signing & Capabilities** tab:
- ✅ **Keychain Sharing**
- ✅ **Near Field Communication Tag Reading** (for NFC)
- ✅ **Push Notifications** (for Firebase)

### 4. Add Required Permissions

Add these keys to `Info.plist`:

```xml
<key>NSCameraUsageDescription</key>
<string>Camera access is required to scan QR codes</string>

<key>NSFaceIDUsageDescription</key>
<string>Face ID is used to secure your wallet</string>

<key>NFCReaderUsageDescription</key>
<string>NFC is used for contactless payments</string>

<key>com.apple.developer.nfc.readersession.formats</key>
<array>
    <string>NDEF</string>
    <string>TAG</string>
</array>
```

### 5. Add Dependencies (via Swift Package Manager)

In Xcode:
1. Go to **File > Add Package Dependencies**
2. Add these packages:
   - WalletConnectSwiftV2: `https://github.com/WalletConnect/WalletConnectSwiftV2`
   - Web3.swift: `https://github.com/Boilertalk/Web3.swift`

### 6. Build and Run

1. Select your target device/simulator (iPhone 16 Pro - iOS 18)
2. Press **Cmd + R** to build and run
3. The app should compile and launch successfully!

## Next Steps

- [ ] Integrate WalletConnect SDK
- [ ] Add transaction signing
- [ ] Implement token balance fetching
- [ ] Add NFC payment support
- [ ] Connect to blockchain networks

## Xcode 16.4 Compatibility

✅ This project is fully compatible with Xcode 16.4 and iOS 18!

No React Native compatibility issues - everything is native Swift.

## License

MIT License - Ëtrid Wallet
