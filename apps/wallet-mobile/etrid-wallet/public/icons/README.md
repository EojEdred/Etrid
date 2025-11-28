# PWA Icons

This directory should contain app icons in various sizes for the Progressive Web App.

## Required Icons

### Main App Icons
- `icon-72x72.png` - Small icon
- `icon-96x96.png` - Medium icon
- `icon-128x128.png` - Standard icon
- `icon-144x144.png` - Standard icon
- `icon-152x152.png` - iOS icon
- `icon-192x192.png` - Android icon (minimum)
- `icon-384x384.png` - Large icon
- `icon-512x512.png` - Android icon (recommended)

### Apple-Specific Icons
- `apple-touch-icon.png` - 180x180px iOS home screen icon

### Notification Icons
- `badge-72x72.png` - 72x72px monochrome icon for notification badge

### Shortcut Icons (96x96 each)
- `send-96x96.png` - Send money shortcut
- `scan-96x96.png` - Scan QR shortcut
- `card-96x96.png` - AU Bloccard shortcut

## How to Generate Icons

### Option 1: Using PWA Asset Generator (Recommended)

```bash
# Install the tool
npm install -g pwa-asset-generator

# Generate all icons from a single source image
pwa-asset-generator path/to/logo.png public/icons \
  --icon-only \
  --padding "20%" \
  --background "#1a0033" \
  --favicon
```

### Option 2: Using Online Tools

1. **[PWA Builder](https://www.pwabuilder.com/)** - Upload manifest.json
2. **[RealFaviconGenerator](https://realfavicongenerator.net/)** - Generate all sizes
3. **[Favicon.io](https://favicon.io/)** - Simple icon generator

### Option 3: Manual Creation

Use image editing software (Photoshop, Figma, etc.) to create each size manually:

1. Start with a 512x512 source image
2. Use the Ëtrid logo
3. Add 20% padding around the logo
4. Background color: `#1a0033` (dark purple)
5. Logo color: `#8b5cf6` (purple) and white
6. Export each required size

## Icon Design Guidelines

### Colors
- **Background:** `#1a0033` (dark purple from brand)
- **Primary:** `#8b5cf6` (purple accent)
- **Secondary:** `#ffffff` (white)

### Composition
- Keep the logo centered
- Add 20% padding (safe zone)
- Ensure the icon works at small sizes (72x72)
- Test on both light and dark backgrounds

### Maskable Icons
All icons should work as maskable icons:
- Keep important content in the center 80%
- Outer 20% may be cropped on some devices
- Test with [Maskable.app](https://maskable.app/)

### File Format
- **PNG** format with transparency
- **sRGB** color space
- Optimized file size (use tools like TinyPNG)

## Verification

After generating icons, verify:
1. All required sizes are present
2. Icons look good at all sizes
3. Icons work on light/dark backgrounds
4. File sizes are optimized (< 50KB each)
5. Transparency is preserved where needed

## Current Status

⚠️ **Icons need to be generated**

Replace this README with actual icon files following the specifications above.

## Resources

- [PWA Icon Guidelines](https://web.dev/add-manifest/#icons)
- [Maskable Icons](https://web.dev/maskable-icon/)
- [Android Adaptive Icons](https://developer.android.com/guide/practices/ui_guidelines/icon_design_adaptive)
