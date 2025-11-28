# Asset Generation Guide

## Prerequisites

Install dependencies:
```bash
cd /home/user/Etrid/apps/wallet-mobile
npm install sharp --save-dev
```

Or if you prefer pwa-asset-generator (alternative method):
```bash
npm install pwa-asset-generator --save-dev
```

## Quick Start

1. Ensure logo exists at `./assets/logo.svg` (already created!)
2. Run the generator:
```bash
npm run generate:assets
```

## What Gets Generated

### PWA Icons (10 files)
- icon-72x72.png through icon-512x512.png
- apple-touch-icon.png (180x180)
- favicon-32x32.png and favicon-16x16.png

Location: `etrid-wallet/public/icons/`

### iOS Splash Screens (9 files)
- Support for all iPhone and iPad sizes
- iPhone 5 through iPhone 14 Pro Max
- iPad and iPad Pro variants

Location: `etrid-wallet/public/splash/`

### Android Icons (10 files)
- 5 density variants (mdpi through xxxhdpi)
- Both standard and round icon versions

Location: `etrid-wallet-native/android/app/src/main/res/mipmap-*/`

### Landing Page Assets (5 files)
- logo.png (256x256)
- logo.svg (scalable)
- logo-white.png (256x256)
- og-image.png (1200x630) - For social media
- twitter-card.png (1200x600) - For Twitter

Location: `landing-page/public/`

### Screenshots (4 files)
- Placeholder screenshots for app stores
- iPhone 14 Pro Max size (1170x2532)

Location: `etrid-wallet/public/screenshots/`

## Individual Commands

If you need to regenerate specific asset types:

```bash
# All assets (recommended)
npm run generate:assets

# Using pwa-asset-generator (alternative)
npm run generate:pwa
npm run generate:splash
```

## Manual Generation with pwa-asset-generator

If the automated script doesn't work, use pwa-asset-generator:

```bash
# PWA Icons
npx pwa-asset-generator assets/logo.svg etrid-wallet/public/icons \
  --icon-only \
  --padding "20%" \
  --background "#1a0033"

# iOS Splash Screens
npx pwa-asset-generator assets/logo.svg etrid-wallet/public/splash \
  --splash-only \
  --background "#1a0033"
```

## Customization

### Change Logo
Replace `assets/logo.svg` with your logo and re-run the generator.

### Change Background Color
Edit `scripts/generate-assets.js`:
```javascript
const BACKGROUND_COLOR = '#1a0033'; // Change this
const BACKGROUND_GRADIENT = ['#1a0033', '#4a0080']; // And this
```

### Change Brand Color
Edit the gradient colors in `assets/logo.svg`.

## Output Locations

```
apps/wallet-mobile/
├── etrid-wallet/public/
│   ├── icons/          ← PWA icons
│   ├── splash/         ← iOS splash screens
│   └── screenshots/    ← App screenshots
├── etrid-wallet-native/android/app/src/main/res/
│   ├── mipmap-mdpi/    ← Android icons (48x48)
│   ├── mipmap-hdpi/    ← Android icons (72x72)
│   ├── mipmap-xhdpi/   ← Android icons (96x96)
│   ├── mipmap-xxhdpi/  ← Android icons (144x144)
│   └── mipmap-xxxhdpi/ ← Android icons (192x192)
└── landing-page/public/
    ├── logo.png        ← Landing page assets
    ├── logo.svg
    ├── logo-white.png
    ├── og-image.png
    └── twitter-card.png
```

## Replacing Screenshot Placeholders

The generator creates placeholder screenshots. Replace them with real ones:

1. **Start the app:**
   ```bash
   cd etrid-wallet
   npm run dev
   ```

2. **Open Chrome DevTools:**
   - Press F12
   - Click device toolbar icon (mobile view)
   - Select "iPhone 14 Pro Max" or set custom: 1170x2532

3. **Navigate and capture:**
   - Navigate to each screen (Home, Trading, NFT, Card)
   - Take screenshots
   - Save with same filenames to `public/screenshots/`

4. **Required screenshots:**
   - home-screenshot.png - Dashboard/wallet view
   - trading-screenshot.png - Swap/exchange interface
   - nft-screenshot.png - NFT gallery view
   - card-screenshot.png - AU Bloccard feature

## Verification

### Test PWA Icons
```bash
cd etrid-wallet
npm run build
# Run Lighthouse audit in Chrome DevTools
```

### Test iOS Splash
1. Open Safari on iPhone
2. Navigate to your app URL
3. Tap Share → Add to Home Screen
4. Close and reopen from home screen
5. Verify splash screen appears

### Test Android Icons
1. Build APK or AAB
2. Install on Android device
3. Check app drawer and home screen
4. Verify icon looks crisp

## Quality Checklist

- [ ] All icons are crisp (no pixelation)
- [ ] Consistent branding (purple gradient)
- [ ] Proper padding (logo not touching edges)
- [ ] Splash screens show logo centered
- [ ] Android round icons properly masked
- [ ] Screenshots replaced with real app images
- [ ] OG images display correctly on social media
- [ ] Favicons appear in browser tabs

## Troubleshooting

### "sharp" installation fails
```bash
# Try with legacy peer deps
npm install sharp --save-dev --legacy-peer-deps

# Or use pre-built binaries
npm install --platform=linux --arch=x64 sharp
```

### "Logo not found" error
Ensure logo exists at:
```bash
ls -la assets/logo.svg
```

Or set custom path:
```bash
LOGO_PATH=/custom/path/logo.svg npm run generate:assets
```

### Icons look stretched
- Check logo SVG viewBox and dimensions
- Ensure logo is square or has transparent padding
- Adjust fit parameter in generate-assets.js

### Need different sizes
Edit the `sizes` object in `generate-assets.js`:
```javascript
const sizes = [72, 96, 128, 144, 152, 192, 384, 512, 1024]; // Add 1024
```

## Support

For issues or questions:
1. Check this README
2. Review `scripts/generate-assets.js` comments
3. Consult Sharp documentation: https://sharp.pixelplumbingltd.com/
4. Check ICON_CHECKLIST.md for verification steps
