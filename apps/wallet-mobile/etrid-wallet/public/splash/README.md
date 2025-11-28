# PWA Splash Screens

This directory should contain splash screens for iOS devices when launching the PWA.

## Required Splash Screens

### iPhone Devices
- `iphone5_splash.png` - 640×1136px (iPhone SE, 5, 5S, 5C)
- `iphone6_splash.png` - 750×1334px (iPhone 6, 7, 8, SE 2020)
- `iphoneplus_splash.png` - 1242×2208px (iPhone 6+, 7+, 8+)
- `iphonex_splash.png` - 1125×2436px (iPhone X, XS, 11 Pro)
- `iphonexr_splash.png` - 828×1792px (iPhone XR, 11)
- `iphonexsmax_splash.png` - 1242×2688px (iPhone XS Max, 11 Pro Max)

### iPad Devices
- `ipad_splash.png` - 1536×2048px (iPad Mini, Air)
- `ipadpro1_splash.png` - 1668×2224px (iPad Pro 10.5")
- `ipadpro2_splash.png` - 2048×2732px (iPad Pro 12.9")

## How to Generate Splash Screens

### Option 1: Using PWA Asset Generator (Recommended)

```bash
# Install the tool
npm install -g pwa-asset-generator

# Generate all splash screens from a single source image
pwa-asset-generator path/to/logo.png public/splash \
  --splash-only \
  --background "#1a0033" \
  --opaque false \
  --quality 90
```

### Option 2: Using Figma/Photoshop Template

Create a template with:
1. Background color: `#1a0033` (dark purple)
2. Centered logo
3. Gradient overlay (optional): `from-[#1a0033] to-[#4a0080]`
4. Export at each required size

### Option 3: Using Online Tools

1. **[PWA Builder](https://www.pwabuilder.com/)** - Generate splash screens
2. **[AppScope](https://appsco.pe/developer/splash-screens)** - iOS splash screen generator

## Design Guidelines

### Layout
```
┌─────────────────────┐
│                     │
│                     │
│                     │
│       [LOGO]        │
│      Ëtrid          │
│                     │
│                     │
│                     │
└─────────────────────┘
```

### Colors
- **Background:** `#1a0033` (solid) or gradient `from-[#1a0033] to-[#4a0080]`
- **Logo:** White or `#8b5cf6` (purple)
- **Text:** White (optional app name below logo)

### Typography (Optional)
If including app name:
- **Font:** Inter or system font
- **Size:** 32-48px
- **Weight:** 600 (semibold)
- **Color:** White with slight opacity

### Composition
- Keep logo centered vertically and horizontally
- Logo should be ~30% of screen height
- Add subtle gradient for depth (optional)
- Ensure safe zones (especially for notched devices)

## File Format

- **PNG** format
- **RGB** color mode (not CMYK)
- Optimized file size (< 100KB per file)
- Quality: 90%

## Responsive Considerations

### Portrait Orientation
All splash screens are portrait (taller than wide)

### Safe Zones
- Top: Account for status bar and notch
- Bottom: Account for home indicator
- Sides: Minimum 20px padding

### Device-Specific Notes
- **iPhone X and newer:** Account for notch at top
- **iPhone with home button:** Account for status bar only
- **iPad:** Consider both portrait and landscape (if needed)

## Testing

Test splash screens on:
1. Real iOS devices (if possible)
2. iOS Simulator
3. Browser DevTools (responsive mode)

Check:
- Logo is centered
- Colors match brand
- No distortion or pixelation
- Loads quickly (file size optimized)

## Advanced: Animated Splash

For advanced implementation, consider:
- SVG animations
- Lottie animations
- CSS animations in the offline page

This requires custom implementation beyond the basic splash screens.

## Fallback

If no splash screen is provided:
- iOS shows a white screen
- Android uses the theme color from manifest

Always provide at least the most common sizes:
- iPhone 6/7/8 (most common)
- iPhone X (notched devices)
- iPad (tablet users)

## Current Status

⚠️ **Splash screens need to be generated**

Replace this README with actual splash screen files following the specifications above.

## Resources

- [iOS Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)
- [PWA Splash Screen Guide](https://web.dev/splash-screen/)
- [Apple PWA Documentation](https://developer.apple.com/library/archive/documentation/AppleApplications/Reference/SafariWebContent/ConfiguringWebApplications/ConfiguringWebApplications.html)
