# Asset Locations Reference

Complete directory map for all generated icons, splash screens, and assets.

---

## PWA Assets (etrid-wallet/)

```
etrid-wallet/public/
├── icons/
│   ├── icon-72x72.png           # PWA icon (small)
│   ├── icon-96x96.png           # PWA icon
│   ├── icon-128x128.png         # PWA icon
│   ├── icon-144x144.png         # PWA icon
│   ├── icon-152x152.png         # PWA icon (iPad)
│   ├── icon-192x192.png         # PWA icon (recommended minimum)
│   ├── icon-384x384.png         # PWA icon
│   ├── icon-512x512.png         # PWA icon (recommended for splash)
│   ├── apple-touch-icon.png     # iOS home screen (180x180)
│   ├── favicon-32x32.png        # Browser tab icon
│   └── favicon-16x16.png        # Browser tab icon (small)
├── splash/
│   ├── iphone5_splash.png       # iPhone 5/SE (640x1136)
│   ├── iphone6_splash.png       # iPhone 6/7/8 (750x1334)
│   ├── iphoneplus_splash.png    # iPhone 6/7/8 Plus (1242x2208)
│   ├── iphonex_splash.png       # iPhone X/XS/11 Pro (1125x2436)
│   ├── iphonexr_splash.png      # iPhone XR/11 (828x1792)
│   ├── iphonexsmax_splash.png   # iPhone XS Max/11 Pro Max (1242x2688)
│   ├── ipad_splash.png          # iPad 9.7" (1536x2048)
│   ├── ipadpro1_splash.png      # iPad Pro 10.5" (1668x2224)
│   └── ipadpro2_splash.png      # iPad Pro 12.9" (2048x2732)
└── screenshots/
    ├── home-screenshot.png      # Home/Dashboard (1170x2532)
    ├── trading-screenshot.png   # Trading interface (1170x2532)
    ├── nft-screenshot.png       # NFT gallery (1170x2532)
    └── card-screenshot.png      # AU Bloccard feature (1170x2532)
```

**Referenced in:**
- `etrid-wallet/public/manifest.json` (PWA icons)
- `etrid-wallet/app/layout.tsx` or `etrid-wallet/pages/_document.tsx` (favicons, apple-touch-icon)
- `etrid-wallet/README.md` (screenshots for documentation)

---

## React Native Android (etrid-wallet-native/)

```
etrid-wallet-native/android/app/src/main/res/
├── mipmap-mdpi/
│   ├── ic_launcher.png          # 48x48 - Low density
│   └── ic_launcher_round.png    # 48x48 - Round variant
├── mipmap-hdpi/
│   ├── ic_launcher.png          # 72x72 - High density
│   └── ic_launcher_round.png    # 72x72 - Round variant
├── mipmap-xhdpi/
│   ├── ic_launcher.png          # 96x96 - Extra high density
│   └── ic_launcher_round.png    # 96x96 - Round variant
├── mipmap-xxhdpi/
│   ├── ic_launcher.png          # 144x144 - Extra extra high density
│   └── ic_launcher_round.png    # 144x144 - Round variant
└── mipmap-xxxhdpi/
    ├── ic_launcher.png          # 192x192 - Extra extra extra high density
    └── ic_launcher_round.png    # 192x192 - Round variant
```

**Referenced in:**
- `etrid-wallet-native/android/app/src/main/AndroidManifest.xml` (android:icon attribute)
- `etrid-wallet-native/android/app/src/main/res/values/strings.xml` (app_name)

**Density Mapping:**
- mdpi: ~160 dpi (1x)
- hdpi: ~240 dpi (1.5x)
- xhdpi: ~320 dpi (2x)
- xxhdpi: ~480 dpi (3x)
- xxxhdpi: ~640 dpi (4x)

---

## React Native iOS (etrid-wallet-native/)

```
etrid-wallet-native/ios/EtridWallet/Images.xcassets/AppIcon.appiconset/
├── icon-20@2x.png               # 40x40 - iPhone Notification
├── icon-20@3x.png               # 60x60 - iPhone Notification
├── icon-29@2x.png               # 58x58 - iPhone Settings
├── icon-29@3x.png               # 87x87 - iPhone Settings
├── icon-40@2x.png               # 80x80 - iPhone Spotlight
├── icon-40@3x.png               # 120x120 - iPhone Spotlight
├── icon-60@2x.png               # 120x120 - iPhone App
├── icon-60@3x.png               # 180x180 - iPhone App
├── icon-20.png                  # 20x20 - iPad Notifications
├── icon-20@2x-ipad.png          # 40x40 - iPad Notifications
├── icon-29.png                  # 29x29 - iPad Settings
├── icon-29@2x-ipad.png          # 58x58 - iPad Settings
├── icon-40.png                  # 40x40 - iPad Spotlight
├── icon-40@2x-ipad.png          # 80x80 - iPad Spotlight
├── icon-76.png                  # 76x76 - iPad App
├── icon-76@2x.png               # 152x152 - iPad App
├── icon-83.5@2x.png             # 167x167 - iPad Pro
└── icon-1024.png                # 1024x1024 - App Store
```

**Referenced in:**
- `etrid-wallet-native/ios/EtridWallet/Images.xcassets/AppIcon.appiconset/Contents.json`
- Auto-managed by Xcode

**Note:** iOS icons require manual generation or use of Xcode asset catalog.

---

## Landing Page (landing-page/)

```
landing-page/public/
├── logo.png                     # 256x256 - Standard logo
├── logo.svg                     # Scalable - SVG logo (preferred)
├── logo-white.png               # 256x256 - White variant for dark backgrounds
├── og-image.png                 # 1200x630 - OpenGraph social sharing
├── twitter-card.png             # 1200x600 - Twitter card image
└── favicon.ico                  # 32x32 - Browser favicon (optional)
```

**Referenced in:**
- `landing-page/app/layout.tsx` or `landing-page/pages/_document.tsx` (meta tags)
- `landing-page/components/Header.tsx` or `landing-page/components/Navbar.tsx` (logo display)

**Meta Tag Example:**
```html
<meta property="og:image" content="/og-image.png" />
<meta name="twitter:card" content="summary_large_image" />
<meta name="twitter:image" content="/twitter-card.png" />
```

---

## Source Assets (wallet-mobile/)

```
wallet-mobile/
├── assets/
│   ├── logo.svg                 # Master logo file (source of truth)
│   ├── logo.png                 # PNG export (optional)
│   └── README.md                # Asset usage guidelines
└── scripts/
    ├── generate-assets.js       # Main asset generator script
    └── README.md                # Generation documentation
```

**Master Files:**
- `assets/logo.svg` - Primary source, edit this for logo changes
- All other assets are generated from this file

---

## File Size Budget

### PWA Icons
```
icon-72x72.png      : < 10 KB
icon-96x96.png      : < 15 KB
icon-128x128.png    : < 20 KB
icon-144x144.png    : < 25 KB
icon-152x152.png    : < 25 KB
icon-192x192.png    : < 30 KB
icon-384x384.png    : < 50 KB
icon-512x512.png    : < 75 KB
apple-touch-icon.png: < 30 KB
favicon-*.png       : < 5 KB each
Total PWA icons     : < 300 KB
```

### iOS Splash Screens
```
iphone5_splash.png     : < 150 KB
iphone6_splash.png     : < 180 KB
iphoneplus_splash.png  : < 250 KB
iphonex_splash.png     : < 250 KB
iphonexr_splash.png    : < 200 KB
iphonexsmax_splash.png : < 280 KB
ipad_splash.png        : < 280 KB
ipadpro1_splash.png    : < 300 KB
ipadpro2_splash.png    : < 350 KB
Total splash screens   : < 2 MB
```

### Android Icons
```
Each ic_launcher.png       : < 15 KB
Each ic_launcher_round.png : < 15 KB
Total Android icons        : < 150 KB
```

### Landing Page Assets
```
logo.svg         : < 10 KB
logo.png         : < 30 KB
logo-white.png   : < 30 KB
og-image.png     : < 200 KB
twitter-card.png : < 200 KB
Total landing    : < 500 KB
```

### Screenshots
```
Each screenshot   : < 400 KB
Total screenshots : < 2 MB
```

**Grand Total Budget: < 5 MB**

---

## Generation Commands

### Generate All Assets
```bash
cd /home/user/Etrid/apps/wallet-mobile
npm run generate:assets
```

### Check Generated Files
```bash
# Count PWA icons
ls etrid-wallet/public/icons/*.png | wc -l

# Count splash screens
ls etrid-wallet/public/splash/*.png | wc -l

# Count Android icons
find etrid-wallet-native/android/app/src/main/res -name "ic_launcher*" | wc -l

# Check total size
du -sh etrid-wallet/public/icons
du -sh etrid-wallet/public/splash
```

### Verify All Assets Exist
```bash
# PWA icons (should return 11)
ls -1 etrid-wallet/public/icons/ | grep -E '\.(png|ico)$' | wc -l

# iOS splash screens (should return 9)
ls -1 etrid-wallet/public/splash/ | grep splash.png | wc -l

# Android icons (should return 10)
find etrid-wallet-native/android/app/src/main/res -name "ic_launcher*.png" | wc -l

# Landing page assets (should return 5+)
ls -1 landing-page/public/ | grep -E '(logo|og-|twitter-)' | wc -l

# Screenshots (should return 4)
ls -1 etrid-wallet/public/screenshots/ | grep screenshot.png | wc -l
```

---

## Platform-Specific Notes

### PWA (Progressive Web App)
- Icons must be referenced in `manifest.json`
- Apple touch icon needs separate meta tag
- Splash screens use media queries for device matching
- Service worker caches icons for offline use

### Android
- System automatically selects appropriate density
- Round icons used by launchers that support them
- Adaptive icons require separate XML configuration
- Foreground/background layers for dynamic theming

### iOS
- Requires asset catalog (Images.xcassets)
- Multiple sizes for different UI contexts
- App Store requires 1024x1024 icon
- No transparency allowed in app icons

### Landing Page
- OG images crucial for social media sharing
- Twitter requires specific aspect ratio
- Favicons should include multiple sizes
- SVG preferred for responsive design

---

## Integration Examples

### manifest.json (PWA)
```json
{
  "name": "Ëtrid Wallet",
  "icons": [
    { "src": "/icons/icon-72x72.png", "sizes": "72x72", "type": "image/png" },
    { "src": "/icons/icon-96x96.png", "sizes": "96x96", "type": "image/png" },
    { "src": "/icons/icon-128x128.png", "sizes": "128x128", "type": "image/png" },
    { "src": "/icons/icon-144x144.png", "sizes": "144x144", "type": "image/png" },
    { "src": "/icons/icon-152x152.png", "sizes": "152x152", "type": "image/png" },
    { "src": "/icons/icon-192x192.png", "sizes": "192x192", "type": "image/png" },
    { "src": "/icons/icon-384x384.png", "sizes": "384x384", "type": "image/png" },
    { "src": "/icons/icon-512x512.png", "sizes": "512x512", "type": "image/png" }
  ]
}
```

### layout.tsx or _document.tsx (Next.js)
```tsx
<head>
  {/* Favicons */}
  <link rel="icon" type="image/png" sizes="32x32" href="/icons/favicon-32x32.png" />
  <link rel="icon" type="image/png" sizes="16x16" href="/icons/favicon-16x16.png" />

  {/* Apple Touch Icon */}
  <link rel="apple-touch-icon" href="/icons/apple-touch-icon.png" />

  {/* iOS Splash Screens */}
  <link rel="apple-touch-startup-image" href="/splash/iphonex_splash.png"
        media="(device-width: 375px) and (device-height: 812px) and (-webkit-device-pixel-ratio: 3)" />

  {/* OpenGraph */}
  <meta property="og:image" content="/og-image.png" />

  {/* Twitter Card */}
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:image" content="/twitter-card.png" />
</head>
```

### AndroidManifest.xml
```xml
<application
    android:icon="@mipmap/ic_launcher"
    android:roundIcon="@mipmap/ic_launcher_round"
    android:label="@string/app_name">
</application>
```

---

## Maintenance

### When to Regenerate Assets

1. **Logo changes** - Any modification to brand identity
2. **New devices** - Apple releases new iPhone/iPad sizes
3. **Platform updates** - New Android density or iOS requirements
4. **Feature updates** - New screenshots for app stores
5. **Rebranding** - Complete visual identity refresh

### Asset Update Workflow

```bash
# 1. Update master logo
vi assets/logo.svg

# 2. Regenerate all assets
npm run generate:assets

# 3. Verify generation
npm run verify:assets  # (if you create this script)

# 4. Replace screenshots
# ... capture new screenshots ...

# 5. Commit changes
git add etrid-wallet/public/icons/
git add etrid-wallet/public/splash/
git add etrid-wallet-native/android/app/src/main/res/mipmap-*/
git add landing-page/public/
git commit -m "chore: regenerate assets with updated logo"
```

### Version Control

**Include in Git:**
- ✅ Source logo: `assets/logo.svg`
- ✅ Generation script: `scripts/generate-assets.js`
- ✅ Documentation: `*.md`

**Consider .gitignore (if regenerating on deploy):**
- ❓ `etrid-wallet/public/icons/*.png`
- ❓ `etrid-wallet/public/splash/*.png`
- ❓ Android mipmap folders

**Best Practice:**
Commit generated assets so deployments don't require asset generation step.

---

## Troubleshooting

### Missing Icons
```bash
# Check if directories exist
ls -la etrid-wallet/public/icons/
ls -la etrid-wallet/public/splash/

# Regenerate missing icons
npm run generate:assets
```

### Wrong Icon Displayed
- Clear browser cache
- Check manifest.json references
- Verify icon path is correct
- Check service worker isn't caching old icon

### Large File Sizes
```bash
# Compress PNGs
npx pngquant --quality=65-80 etrid-wallet/public/icons/*.png

# Optimize all images
npx imagemin etrid-wallet/public/**/*.png --out-dir=etrid-wallet/public/
```

---

## Additional Resources

- [PWA Icons Guidelines](https://web.dev/add-manifest/)
- [iOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/app-icons)
- [Android Icon Design](https://developer.android.com/guide/practices/ui_guidelines/icon_design_launcher)
- [OpenGraph Protocol](https://ogp.me/)
- [Twitter Cards](https://developer.twitter.com/en/docs/twitter-for-websites/cards/overview/abouts-cards)
