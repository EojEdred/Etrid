# Ëtrid Mobile Wallet

Complete DeFi wallet solution with PWA, React Native, and landing page.

## 🎨 Asset Generation

This project includes comprehensive asset generation for all platforms.

### Quick Start

```bash
# Install dependencies
npm install

# Generate all icons, splash screens, and assets
npm run generate:assets

# Verify all assets were created
npm run verify:assets
```

### What Gets Generated

- ✅ **PWA Icons** - 11 sizes (72px to 512px)
- ✅ **iOS Splash Screens** - 9 device sizes
- ✅ **Android Icons** - 5 densities (mdpi to xxxhdpi)
- ✅ **Landing Page Assets** - Logo, OG images, Twitter cards
- ✅ **Screenshots** - 4 app store screenshots (placeholders)

### Asset Locations

```
apps/wallet-mobile/
├── assets/
│   └── logo.svg                    # Master logo (source)
├── scripts/
│   ├── generate-assets.js          # Asset generator
│   └── verify-assets.js            # Asset verifier
├── etrid-wallet/public/
│   ├── icons/                      # PWA icons
│   ├── splash/                     # iOS splash screens
│   └── screenshots/                # App screenshots
├── etrid-wallet-native/android/app/src/main/res/
│   └── mipmap-*/                   # Android icons
└── landing-page/public/
    ├── logo.svg, logo.png          # Landing page assets
    ├── og-image.png                # Social media preview
    └── twitter-card.png            # Twitter preview
```

## 📚 Documentation

- **[ICON_CHECKLIST.md](./ICON_CHECKLIST.md)** - Complete asset checklist with verification steps
- **[ASSET_LOCATIONS.md](./ASSET_LOCATIONS.md)** - Detailed map of all asset locations
- **[scripts/README.md](./scripts/README.md)** - Asset generation guide

## 🚀 Projects

### PWA Wallet (etrid-wallet/)
Progressive Web App wallet with Next.js

```bash
cd etrid-wallet
npm install
npm run dev
```

### React Native App (etrid-wallet-native/)
Native iOS and Android app

```bash
cd etrid-wallet-native
npm install

# iOS
npx pod-install
npm run ios

# Android
npm run android
```

### Landing Page (landing-page/)
Marketing website

```bash
cd landing-page
npm install
npm run dev
```

## 📱 Available Scripts

### Asset Generation

```bash
npm run generate:assets    # Generate all assets
npm run verify:assets      # Verify assets exist
npm run generate:pwa       # PWA icons only (alternative)
npm run generate:splash    # iOS splash only (alternative)
npm run optimize:images    # Optimize PNG file sizes
```

### Development

```bash
# Start all projects (from each directory)
npm run dev                # Development server
npm run build              # Production build
npm run start              # Start production server
```

## 🔧 Requirements

- Node.js 18+ and npm/yarn
- For React Native: Xcode (iOS), Android Studio (Android)

## 🎯 Features

### PWA Wallet
- ✅ Multi-chain support (Polkadot, Ethereum, Solana)
- ✅ Token swaps and trading
- ✅ NFT gallery
- ✅ AU Bloccard integration
- ✅ Offline-first with service worker
- ✅ Install as native app

### React Native
- ✅ Native iOS and Android apps
- ✅ Biometric authentication
- ✅ Push notifications
- ✅ Deep linking

### Landing Page
- ✅ Responsive design
- ✅ Feature showcase
- ✅ Download links
- ✅ SEO optimized

## 🎨 Branding

### Colors
- **Primary Purple**: `#8b5cf6`
- **Light Purple**: `#a78bfa`
- **Dark Purple**: `#1a0033`
- **Gradient**: `#1a0033` → `#4a0080`

### Logo
Master logo: `assets/logo.svg`

All assets generated from this source.

## 📸 Screenshots

Screenshots are initially generated as placeholders. Replace with real app screenshots:

1. Start app: `cd etrid-wallet && npm run dev`
2. Open Chrome DevTools (F12) → Device Mode
3. Set to iPhone 14 Pro Max (1170x2532)
4. Navigate to each screen and screenshot
5. Save to `etrid-wallet/public/screenshots/`

Required screenshots:
- `home-screenshot.png` - Dashboard/wallet view
- `trading-screenshot.png` - Swap interface
- `nft-screenshot.png` - NFT gallery
- `card-screenshot.png` - AU Bloccard

## 🧪 Testing

### PWA Installation
1. Build: `cd etrid-wallet && npm run build && npm start`
2. Open in Chrome
3. Click install icon in address bar
4. Verify icon and splash screen

### Asset Quality
```bash
# Check all assets exist
npm run verify:assets

# Check file sizes
du -sh etrid-wallet/public/icons/*
du -sh etrid-wallet/public/splash/*

# Verify manifest references icons
cat etrid-wallet/public/manifest.json | grep icons
```

### Lighthouse Audit
1. Open app in Chrome
2. DevTools → Lighthouse tab
3. Run audit (should score 90+)
4. Check PWA checklist

## 📦 Deployment

### PWA
```bash
cd etrid-wallet
npm run build
# Deploy 'out' directory to hosting
```

### React Native
```bash
cd etrid-wallet-native

# iOS
npx react-native run-ios --configuration Release

# Android
cd android && ./gradlew assembleRelease
```

### Landing Page
```bash
cd landing-page
npm run build
# Deploy to Vercel, Netlify, etc.
```

## 🐛 Troubleshooting

### "sharp" installation fails
```bash
npm install sharp --save-dev --legacy-peer-deps
```

### Icons not showing
- Clear browser cache
- Verify manifest.json references
- Check icon paths are correct
- Restart development server

### Android build fails
```bash
cd etrid-wallet-native/android
./gradlew clean
cd .. && npm run android
```

### iOS build fails
```bash
cd etrid-wallet-native/ios
pod deintegrate && pod install
cd .. && npm run ios
```

## 📄 License

MIT

## 🤝 Contributing

1. Fork the repository
2. Create feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open pull request

## 📞 Support

For issues or questions:
- Check documentation in `/docs`
- Review ICON_CHECKLIST.md for asset issues
- See scripts/README.md for generation help

---

**Made with 💜 by the Ëtrid Team**