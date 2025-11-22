# Icon & Asset Checklist

Generated: 2025-11-19

## PWA Icons ✓

### Icons Directory: `etrid-wallet/public/icons/`

- [ ] icon-72x72.png
- [ ] icon-96x96.png
- [ ] icon-128x128.png
- [ ] icon-144x144.png
- [ ] icon-152x152.png
- [ ] icon-192x192.png
- [ ] icon-384x384.png
- [ ] icon-512x512.png
- [ ] apple-touch-icon.png (180x180)
- [ ] favicon-32x32.png
- [ ] favicon-16x16.png

## iOS Splash Screens ✓

### Splash Directory: `etrid-wallet/public/splash/`

- [ ] iphone5_splash.png (640x1136)
- [ ] iphone6_splash.png (750x1334)
- [ ] iphoneplus_splash.png (1242x2208)
- [ ] iphonex_splash.png (1125x2436)
- [ ] iphonexr_splash.png (828x1792)
- [ ] iphonexsmax_splash.png (1242x2688)
- [ ] ipad_splash.png (1536x2048)
- [ ] ipadpro1_splash.png (1668x2224)
- [ ] ipadpro2_splash.png (2048x2732)

## Android Icons ✓

### Android Resources: `etrid-wallet-native/android/app/src/main/res/`

#### mipmap-mdpi/ (48x48)
- [ ] ic_launcher.png
- [ ] ic_launcher_round.png

#### mipmap-hdpi/ (72x72)
- [ ] ic_launcher.png
- [ ] ic_launcher_round.png

#### mipmap-xhdpi/ (96x96)
- [ ] ic_launcher.png
- [ ] ic_launcher_round.png

#### mipmap-xxhdpi/ (144x144)
- [ ] ic_launcher.png
- [ ] ic_launcher_round.png

#### mipmap-xxxhdpi/ (192x192)
- [ ] ic_launcher.png
- [ ] ic_launcher_round.png

## Landing Page Assets ✓

### Landing Page Public: `landing-page/public/`

- [ ] logo.png (256x256)
- [ ] logo.svg (scalable)
- [ ] logo-white.png (256x256, for dark backgrounds)
- [ ] og-image.png (1200x630, OpenGraph for social sharing)
- [ ] twitter-card.png (1200x600, Twitter card)

## Screenshots ✓

### Screenshots Directory: `etrid-wallet/public/screenshots/`

- [ ] home-screenshot.png (1170x2532)
- [ ] trading-screenshot.png (1170x2532)
- [ ] nft-screenshot.png (1170x2532)
- [ ] card-screenshot.png (1170x2532)

> **Note:** Screenshots are initially generated as placeholders. Replace with actual app screenshots before publishing.

---

## Verification Steps

### 1. PWA Icons Verification

**Build and Test:**
```bash
cd etrid-wallet
npm run build
npm start
```

**Check in Browser:**
- [ ] Favicon appears in browser tab
- [ ] Icons appear in bookmark/favorites
- [ ] Lighthouse audit shows no icon warnings

**Check manifest.json:**
```bash
# Verify icons are referenced in manifest
cat etrid-wallet/public/manifest.json | grep icons
```

**Test Installation:**
- [ ] Chrome: Click install icon in address bar
- [ ] Edge: Check "App available" notification
- [ ] Safari iOS: Add to Home Screen works

### 2. iOS Splash Screen Verification

**Test on iOS Device:**
- [ ] Open Safari and navigate to app
- [ ] Tap Share button → "Add to Home Screen"
- [ ] Enter app name and tap "Add"
- [ ] Close Safari
- [ ] Tap app icon on home screen
- [ ] Verify splash screen appears with logo
- [ ] Splash should show for 1-3 seconds

**Test on Different Devices:**
- [ ] iPhone SE (small screen)
- [ ] iPhone 14 Pro (standard)
- [ ] iPhone 14 Pro Max (large)
- [ ] iPad (tablet)

### 3. Android Icons Verification

**Build APK:**
```bash
cd etrid-wallet-native
cd android
./gradlew assembleRelease
```

**Test on Android Device:**
- [ ] Install APK on device
- [ ] Icon appears in app drawer (crisp, not pixelated)
- [ ] Icon appears on home screen after adding
- [ ] Round icon displays properly (on supported launchers)
- [ ] Icon looks correct in recent apps/task switcher

**Test on Different Launchers:**
- [ ] Stock Android launcher
- [ ] Samsung One UI
- [ ] OnePlus/Oxygen OS
- [ ] Custom launchers (Nova, etc.)

### 4. Landing Page Assets Verification

**Check Files Exist:**
```bash
ls -lh landing-page/public/logo*
ls -lh landing-page/public/*-image.png
```

**Test Social Sharing:**
- [ ] Share URL on Twitter - verify card preview
- [ ] Share URL on Facebook - verify OG image
- [ ] Share URL on LinkedIn - verify preview
- [ ] Share in Slack/Discord - verify thumbnail

**Verify with Tools:**
- [ ] Twitter Card Validator: https://cards-dev.twitter.com/validator
- [ ] Facebook Debugger: https://developers.facebook.com/tools/debug/
- [ ] LinkedIn Inspector: https://www.linkedin.com/post-inspector/

### 5. Screenshot Verification

**Placeholder Check:**
- [ ] All 4 placeholders generated
- [ ] Correct dimensions (1170x2532)
- [ ] Contains Etrid branding

**Real Screenshots (After Replacement):**
- [ ] Shows actual app UI (not placeholder)
- [ ] UI elements are crisp and readable
- [ ] Represents actual features accurately
- [ ] No personal/test data visible
- [ ] Status bar time shows 9:41 (Apple standard)
- [ ] Battery icon shows full charge

---

## Quality Standards

### All Icons Must Meet:
- [ ] **Crisp rendering** - No pixelation or blur at any size
- [ ] **Consistent branding** - Purple gradient background (#1a0033 to #4a0080)
- [ ] **Proper padding** - Logo doesn't touch edges (20% padding recommended)
- [ ] **Transparent or branded background** - No white backgrounds unless intentional
- [ ] **Color consistency** - Brand colors match across all assets

### Splash Screens Must Meet:
- [ ] **Centered logo** - Logo positioned in center of screen
- [ ] **Gradient background** - Purple gradient from dark to light
- [ ] **Proper scaling** - Logo scales appropriately for screen size
- [ ] **Fast loading** - PNG optimized for quick display

### Screenshots Must Meet:
- [ ] **Real UI** - Actual app interface, not placeholders
- [ ] **Representative** - Shows key features accurately
- [ ] **Clean data** - No personal info or test data
- [ ] **Consistent branding** - UI matches app design
- [ ] **Professional** - No errors, loading states, or bugs visible

---

## File Size Recommendations

### Icons
- PWA icons: < 50KB each
- Android icons: < 30KB each
- Favicon: < 10KB

### Splash Screens
- iPhone splashes: < 200KB each
- iPad splashes: < 300KB each

### Landing Page
- Logo PNG: < 50KB
- Logo SVG: < 20KB
- OG image: < 300KB
- Twitter card: < 300KB

### Screenshots
- Each screenshot: < 500KB
- Store listing total: < 5MB

## Optimization Tips

**Compress PNGs:**
```bash
# Using pngquant
pngquant --quality=65-80 *.png

# Using OptiPNG
optipng -o7 *.png
```

**Optimize SVGs:**
```bash
# Using SVGO
npx svgo assets/logo.svg
```

**Batch Resize:**
```bash
# Using ImageMagick
mogrify -resize 512x512 *.png
```

---

## Integration Checklist

### PWA Integration
- [ ] Icons referenced in `manifest.json`
- [ ] Favicon linked in `index.html`
- [ ] Apple touch icon meta tag added
- [ ] Theme color matches brand

### iOS Integration
- [ ] Splash screens linked in `index.html` with media queries
- [ ] Apple meta tags configured
- [ ] Status bar style set

### Android Integration
- [ ] Icons placed in correct mipmap folders
- [ ] AndroidManifest.xml references ic_launcher
- [ ] Adaptive icon configured (optional)

### Landing Page Integration
- [ ] Logo displayed in header/navbar
- [ ] OG meta tags in HTML head
- [ ] Twitter card meta tags added
- [ ] Favicon linked

### App Store Integration
- [ ] Screenshots uploaded to App Store Connect
- [ ] Screenshots uploaded to Google Play Console
- [ ] Store listing includes all 4 screenshots
- [ ] Screenshots showcase key features

---

## Before Publishing

### Final Checks
- [ ] Run `npm run generate:assets` one final time
- [ ] Verify all file sizes are optimized
- [ ] Test PWA installation on real devices
- [ ] Test on multiple screen sizes
- [ ] Check social media previews
- [ ] Validate manifest.json
- [ ] Run Lighthouse audit (score > 90)
- [ ] Screenshots replaced with real app UI
- [ ] No placeholder assets remain

### Documentation
- [ ] Asset locations documented
- [ ] Generation process documented
- [ ] Team knows how to regenerate assets
- [ ] Brand guidelines followed

### Performance
- [ ] Total asset size < 10MB
- [ ] Icons load quickly
- [ ] Splash screens display instantly
- [ ] No layout shift from icon loading

---

## Need Help?

See `scripts/README.md` for detailed generation instructions.

Run asset generation:
```bash
npm run generate:assets
```

Verify files were created:
```bash
find . -name "icon-*.png" -o -name "*splash.png" -o -name "ic_launcher*"
```

Check file sizes:
```bash
du -sh etrid-wallet/public/icons/* etrid-wallet/public/splash/*
```
