# Quick Start Guide - Etrid Wallet Assets

## 🚀 Instant Setup (30 seconds)

```bash
cd /home/user/Etrid/apps/wallet-mobile

# 1. Install dependencies (if not done)
npm install

# 2. Generate all assets
npm run generate:assets

# 3. Verify everything was created
npm run verify:assets
```

**That's it!** All 38 icons and assets are now ready.

---

## 📁 What Was Created?

```
✅ 11 PWA icons       → etrid-wallet/public/icons/
✅ 9 iOS splash       → etrid-wallet/public/splash/
✅ 10 Android icons   → etrid-wallet-native/android/.../res/mipmap-*/
✅ 4 Landing assets   → landing-page/public/
✅ 4 Screenshots      → etrid-wallet/public/screenshots/
```

**Total**: 38 files, ~700 KB

---

## 🎯 Most Common Tasks

### Regenerate All Assets

```bash
npm run generate:assets
```

### Check What's Missing

```bash
npm run verify:assets
```

### Update the Logo

1. Edit: `assets/logo.svg`
2. Run: `npm run generate:assets`
3. Done!

### Replace Screenshot Placeholders

```bash
# 1. Start app
cd etrid-wallet
npm run dev

# 2. Open Chrome DevTools (F12) → Device Mode
# 3. Set to iPhone 14 Pro Max (1170x2532)
# 4. Screenshot each screen
# 5. Save to: public/screenshots/
```

---

## 📱 Integration (3 steps)

### Step 1: PWA Manifest

Add to `etrid-wallet/public/manifest.json`:

```json
{
  "icons": [
    { "src": "/icons/icon-192x192.png", "sizes": "192x192", "type": "image/png" },
    { "src": "/icons/icon-512x512.png", "sizes": "512x512", "type": "image/png" }
  ]
}
```

### Step 2: HTML Head

Add to `etrid-wallet/app/layout.tsx`:

```tsx
<link rel="icon" href="/icons/favicon-32x32.png" />
<link rel="apple-touch-icon" href="/icons/apple-touch-icon.png" />
<meta property="og:image" content="/og-image.png" />
```

### Step 3: Test It

```bash
cd etrid-wallet
npm run build
npm start

# Open http://localhost:3000
# Try "Install App" in Chrome
```

---

## 🧪 Quick Tests

### PWA Installation Test

1. Build: `cd etrid-wallet && npm run build && npm start`
2. Open Chrome → http://localhost:3000
3. Click install icon in address bar
4. Verify icon and name

### Lighthouse Audit

1. Open app in Chrome
2. DevTools (F12) → Lighthouse tab
3. Run audit
4. Should score 90+

### Icon Quality Check

```bash
# View all icons
ls -lh etrid-wallet/public/icons/

# Check file sizes
du -sh etrid-wallet/public/icons/*
du -sh etrid-wallet/public/splash/*

# Total size
du -sh etrid-wallet/public/icons etrid-wallet/public/splash
```

---

## 📦 File Locations Reference

| Asset Type | Location |
|------------|----------|
| **PWA Icons** | `etrid-wallet/public/icons/` |
| **iOS Splash** | `etrid-wallet/public/splash/` |
| **Android Icons** | `etrid-wallet-native/android/app/src/main/res/mipmap-*/` |
| **Landing Page** | `landing-page/public/` |
| **Screenshots** | `etrid-wallet/public/screenshots/` |
| **Master Logo** | `assets/logo.svg` |

---

## 🎨 Brand Colors

```css
Primary:    #8b5cf6  /* Purple */
Light:      #a78bfa  /* Light Purple */
Dark:       #1a0033  /* Dark Purple */
Gradient:   #1a0033 → #4a0080
```

---

## ⚡ Troubleshooting

### "sharp" won't install

```bash
npm install sharp --legacy-peer-deps
```

### Icons not showing

- Clear browser cache
- Check manifest.json
- Restart dev server

### Need to change colors

Edit `scripts/generate-assets.js`:
- Line 6: `BRAND_COLOR`
- Line 7: `BACKGROUND_COLOR`
- Line 8: `BACKGROUND_GRADIENT`

---

## 📚 Full Documentation

- **[ASSET_GENERATION_SUMMARY.md](./ASSET_GENERATION_SUMMARY.md)** - Complete generation report
- **[ICON_CHECKLIST.md](./ICON_CHECKLIST.md)** - Verification checklist
- **[ASSET_LOCATIONS.md](./ASSET_LOCATIONS.md)** - Detailed file map
- **[scripts/README.md](./scripts/README.md)** - Generation guide

---

## 💡 Pro Tips

1. **Regenerate anytime** - Just run `npm run generate:assets`
2. **All from one source** - Edit `assets/logo.svg` to update all icons
3. **Optimize if needed** - Run `npm run optimize:images`
4. **Test on real devices** - PWA works best on actual phones
5. **Check file sizes** - Keep total under 1 MB

---

## ✅ Quick Checklist

Before deploying:

- [ ] All 38 assets generated (`npm run verify:assets`)
- [ ] Screenshots replaced with real app UI
- [ ] Icons added to manifest.json
- [ ] Favicon linked in HTML
- [ ] Social media meta tags added
- [ ] Lighthouse score > 90
- [ ] Tested on real device

---

**Need help?** Check the documentation files or review the generation script at `scripts/generate-assets.js`.

**Ready to deploy?** All assets are production-ready!
