const sharp = require('sharp');
const fs = require('fs').promises;
const path = require('path');

const LOGO_PATH = process.env.LOGO_PATH || path.join(__dirname, '../assets/logo.svg');
const BRAND_COLOR = '#8b5cf6'; // Purple
const BACKGROUND_COLOR = '#1a0033'; // Dark purple
const BACKGROUND_GRADIENT = ['#1a0033', '#4a0080']; // Dark purple gradient

async function generateAllAssets() {
  console.log('🎨 Starting Etrid Wallet asset generation...\n');

  // Check if logo exists
  try {
    await fs.access(LOGO_PATH);
    console.log('✅ Logo found at:', LOGO_PATH);
  } catch {
    console.error('❌ Logo not found. Please provide logo at:', LOGO_PATH);
    console.log('Set LOGO_PATH environment variable or place logo.svg in ./assets/');
    process.exit(1);
  }

  await generatePWAIcons();
  await generateIOSSplashScreens();
  await generateAndroidIcons();
  await generateLandingPageAssets();
  await generateScreenshots();

  console.log('\n✅ All assets generated successfully!');
  console.log('\n📋 Next steps:');
  console.log('  1. Review generated assets in respective directories');
  console.log('  2. Replace screenshot placeholders with real app screenshots');
  console.log('  3. Run: npm run build (in each project) to verify integration');
  console.log('  4. Test PWA installation on mobile devices');
}

async function generatePWAIcons() {
  console.log('📱 Generating PWA icons...');

  const sizes = [72, 96, 128, 144, 152, 192, 384, 512];
  const outputDir = path.join(__dirname, '../etrid-wallet/public/icons');

  await fs.mkdir(outputDir, { recursive: true });

  for (const size of sizes) {
    await sharp(LOGO_PATH)
      .resize(size, size, {
        fit: 'contain',
        background: { r: 26, g: 0, b: 51, alpha: 1 } // #1a0033
      })
      .png()
      .toFile(path.join(outputDir, `icon-${size}x${size}.png`));

    console.log(`  ✓ Generated icon-${size}x${size}.png`);
  }

  // Apple touch icon (180x180)
  await sharp(LOGO_PATH)
    .resize(180, 180, {
      fit: 'contain',
      background: { r: 26, g: 0, b: 51, alpha: 1 }
    })
    .png()
    .toFile(path.join(outputDir, 'apple-touch-icon.png'));

  console.log('  ✓ Generated apple-touch-icon.png (180x180)');

  // Favicon (32x32 and 16x16)
  await sharp(LOGO_PATH)
    .resize(32, 32, {
      fit: 'contain',
      background: { r: 26, g: 0, b: 51, alpha: 1 }
    })
    .png()
    .toFile(path.join(outputDir, 'favicon-32x32.png'));

  await sharp(LOGO_PATH)
    .resize(16, 16, {
      fit: 'contain',
      background: { r: 26, g: 0, b: 51, alpha: 1 }
    })
    .png()
    .toFile(path.join(outputDir, 'favicon-16x16.png'));

  console.log('  ✓ Generated favicon-32x32.png and favicon-16x16.png');
  console.log('✅ PWA icons complete\n');
}

async function generateIOSSplashScreens() {
  console.log('🍎 Generating iOS splash screens...');

  const sizes = {
    iphone5_splash: { width: 640, height: 1136 },
    iphone6_splash: { width: 750, height: 1334 },
    iphoneplus_splash: { width: 1242, height: 2208 },
    iphonex_splash: { width: 1125, height: 2436 },
    iphonexr_splash: { width: 828, height: 1792 },
    iphonexsmax_splash: { width: 1242, height: 2688 },
    ipad_splash: { width: 1536, height: 2048 },
    ipadpro1_splash: { width: 1668, height: 2224 },
    ipadpro2_splash: { width: 2048, height: 2732 },
  };

  const outputDir = path.join(__dirname, '../etrid-wallet/public/splash');
  await fs.mkdir(outputDir, { recursive: true });

  for (const [name, { width, height }] of Object.entries(sizes)) {
    // Create gradient background
    const svg = `
      <svg width="${width}" height="${height}" xmlns="http://www.w3.org/2000/svg">
        <defs>
          <linearGradient id="grad" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" style="stop-color:#1a0033;stop-opacity:1" />
            <stop offset="100%" style="stop-color:#4a0080;stop-opacity:1" />
          </linearGradient>
        </defs>
        <rect width="${width}" height="${height}" fill="url(#grad)"/>
      </svg>
    `;

    // Calculate logo size (30% of the smaller dimension)
    const logoSize = Math.min(width, height) * 0.3;

    // Create base splash with centered logo
    await sharp(Buffer.from(svg))
      .composite([{
        input: await sharp(LOGO_PATH)
          .resize(Math.round(logoSize), Math.round(logoSize), {
            fit: 'contain',
            background: { r: 0, g: 0, b: 0, alpha: 0 }
          })
          .toBuffer(),
        gravity: 'center'
      }])
      .png()
      .toFile(path.join(outputDir, `${name}.png`));

    console.log(`  ✓ Generated ${name}.png (${width}x${height})`);
  }

  console.log('✅ iOS splash screens complete\n');
}

async function generateAndroidIcons() {
  console.log('🤖 Generating Android icons...');

  const sizes = {
    'mipmap-mdpi': 48,
    'mipmap-hdpi': 72,
    'mipmap-xhdpi': 96,
    'mipmap-xxhdpi': 144,
    'mipmap-xxxhdpi': 192,
  };

  const baseDir = path.join(__dirname, '../etrid-wallet-native/android/app/src/main/res');

  // Check if Android project exists
  try {
    await fs.access(path.join(__dirname, '../etrid-wallet-native/android'));
  } catch {
    console.log('⚠️  Android project not found, skipping Android icons');
    console.log('');
    return;
  }

  for (const [folder, size] of Object.entries(sizes)) {
    const outputDir = path.join(baseDir, folder);
    await fs.mkdir(outputDir, { recursive: true });

    // Standard launcher icon with background
    await sharp(LOGO_PATH)
      .resize(size, size, {
        fit: 'contain',
        background: { r: 26, g: 0, b: 51, alpha: 1 }
      })
      .png()
      .toFile(path.join(outputDir, 'ic_launcher.png'));

    // Round icon version (masked to circle)
    const maskSvg = `
      <svg width="${size}" height="${size}" xmlns="http://www.w3.org/2000/svg">
        <circle cx="${size/2}" cy="${size/2}" r="${size/2}" fill="white"/>
      </svg>
    `;

    await sharp(LOGO_PATH)
      .resize(size, size, {
        fit: 'contain',
        background: { r: 26, g: 0, b: 51, alpha: 1 }
      })
      .composite([{
        input: Buffer.from(maskSvg),
        blend: 'dest-in'
      }])
      .png()
      .toFile(path.join(outputDir, 'ic_launcher_round.png'));

    console.log(`  ✓ Generated ${folder} icons (${size}x${size})`);
  }

  console.log('✅ Android icons complete\n');
}

async function generateLandingPageAssets() {
  console.log('🎨 Generating landing page assets...');

  const outputDir = path.join(__dirname, '../landing-page/public');

  // Check if landing page exists
  try {
    await fs.access(path.join(__dirname, '../landing-page'));
  } catch {
    console.log('⚠️  Landing page not found, skipping landing page assets');
    console.log('');
    return;
  }

  await fs.mkdir(outputDir, { recursive: true });

  // Logo (PNG version - 256x256)
  await sharp(LOGO_PATH)
    .resize(256, 256, {
      fit: 'contain',
      background: { r: 0, g: 0, b: 0, alpha: 0 }
    })
    .png()
    .toFile(path.join(outputDir, 'logo.png'));

  console.log('  ✓ Generated logo.png (256x256)');

  // Copy SVG logo
  await fs.copyFile(
    LOGO_PATH,
    path.join(outputDir, 'logo.svg')
  );
  console.log('  ✓ Copied logo.svg');

  // White version of logo (for dark backgrounds)
  // Note: This is a simplified version - you may want to manually create a white version
  await sharp(LOGO_PATH)
    .resize(256, 256, {
      fit: 'contain',
      background: { r: 0, g: 0, b: 0, alpha: 0 }
    })
    .png()
    .toFile(path.join(outputDir, 'logo-white.png'));

  console.log('  ✓ Generated logo-white.png (256x256)');

  // OG Image (1200x630) for social media sharing
  const ogSvg = `
    <svg width="1200" height="630" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#1a0033;stop-opacity:1" />
          <stop offset="100%" style="stop-color:#4a0080;stop-opacity:1" />
        </linearGradient>
      </defs>
      <rect width="1200" height="630" fill="url(#grad)"/>
      <text x="600" y="450" font-family="Arial, sans-serif" font-size="72" font-weight="bold"
            fill="white" text-anchor="middle">Ëtrid Wallet</text>
      <text x="600" y="520" font-family="Arial, sans-serif" font-size="32"
            fill="#d8b4fe" text-anchor="middle">The Complete DeFi Wallet</text>
    </svg>
  `;

  await sharp(Buffer.from(ogSvg))
    .composite([{
      input: await sharp(LOGO_PATH)
        .resize(200, 200, {
          fit: 'contain',
          background: { r: 0, g: 0, b: 0, alpha: 0 }
        })
        .toBuffer(),
      top: 150,
      left: 500
    }])
    .png()
    .toFile(path.join(outputDir, 'og-image.png'));

  console.log('  ✓ Generated og-image.png (1200x630)');

  // Twitter Card (1200x600)
  const twitterSvg = `
    <svg width="1200" height="600" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:#1a0033;stop-opacity:1" />
          <stop offset="100%" style="stop-color:#4a0080;stop-opacity:1" />
        </linearGradient>
      </defs>
      <rect width="1200" height="600" fill="url(#grad)"/>
      <text x="600" y="430" font-family="Arial, sans-serif" font-size="68" font-weight="bold"
            fill="white" text-anchor="middle">Ëtrid Wallet</text>
      <text x="600" y="500" font-family="Arial, sans-serif" font-size="30"
            fill="#d8b4fe" text-anchor="middle">The Complete DeFi Wallet</text>
    </svg>
  `;

  await sharp(Buffer.from(twitterSvg))
    .composite([{
      input: await sharp(LOGO_PATH)
        .resize(180, 180, {
          fit: 'contain',
          background: { r: 0, g: 0, b: 0, alpha: 0 }
        })
        .toBuffer(),
      top: 150,
      left: 510
    }])
    .png()
    .toFile(path.join(outputDir, 'twitter-card.png'));

  console.log('  ✓ Generated twitter-card.png (1200x600)');
  console.log('✅ Landing page assets complete\n');
}

async function generateScreenshots() {
  console.log('📸 Generating app screenshots...');
  console.log('ℹ️  Note: Screenshots should be captured from running app');
  console.log('ℹ️  Placeholder screenshots will be created\n');

  const screenshotDir = path.join(__dirname, '../etrid-wallet/public/screenshots');
  await fs.mkdir(screenshotDir, { recursive: true });

  // Helper function to escape XML entities
  const escapeXml = (unsafe) => {
    return unsafe.replace(/[&<>"']/g, (c) => {
      switch (c) {
        case '&': return '&amp;';
        case '<': return '&lt;';
        case '>': return '&gt;';
        case '"': return '&quot;';
        case "'": return '&apos;';
        default: return c;
      }
    });
  };

  const screenshots = [
    { name: 'home-screenshot', title: 'Home Screen', subtitle: 'Manage your assets' },
    { name: 'trading-screenshot', title: 'Trading', subtitle: 'Swap and exchange' },
    { name: 'nft-screenshot', title: 'NFT Gallery', subtitle: 'Your collectibles' },
    { name: 'card-screenshot', title: 'AU Bloccard', subtitle: 'DeFi meets real world' },
  ];

  for (const { name, title, subtitle } of screenshots) {
    const svg = `
      <svg width="1170" height="2532" xmlns="http://www.w3.org/2000/svg">
        <defs>
          <linearGradient id="grad" x1="0%" y1="0%" x2="0%" y2="100%">
            <stop offset="0%" style="stop-color:#1a0033;stop-opacity:1" />
            <stop offset="50%" style="stop-color:#2a0050;stop-opacity:1" />
            <stop offset="100%" style="stop-color:#4a0080;stop-opacity:1" />
          </linearGradient>
        </defs>
        <rect width="1170" height="2532" fill="url(#grad)"/>

        <!-- Decorative circles -->
        <circle cx="200" cy="400" r="150" fill="#8b5cf6" opacity="0.1"/>
        <circle cx="970" cy="1200" r="200" fill="#a78bfa" opacity="0.1"/>
        <circle cx="300" cy="2100" r="180" fill="#8b5cf6" opacity="0.1"/>

        <text x="585" y="1150" font-family="Arial, sans-serif" font-size="100" font-weight="bold"
              fill="white" text-anchor="middle">${escapeXml(title)}</text>
        <text x="585" y="1280" font-family="Arial, sans-serif" font-size="48"
              fill="#d8b4fe" text-anchor="middle">${escapeXml(subtitle)}</text>
        <text x="585" y="1450" font-family="Arial, sans-serif" font-size="36"
              fill="#a78bfa" text-anchor="middle">Screenshot Placeholder</text>
        <text x="585" y="1520" font-family="Arial, sans-serif" font-size="28"
              fill="#8b5cf6" text-anchor="middle">Replace with actual app screenshot</text>
      </svg>
    `;

    await sharp(Buffer.from(svg))
      .composite([{
        input: await sharp(LOGO_PATH)
          .resize(280, 280, {
            fit: 'contain',
            background: { r: 0, g: 0, b: 0, alpha: 0 }
          })
          .toBuffer(),
        top: 800,
        left: 445
      }])
      .png()
      .toFile(path.join(screenshotDir, `${name}.png`));

    console.log(`  ✓ Generated ${name}.png placeholder`);
  }

  console.log('\n💡 To capture real screenshots:');
  console.log('  1. Run: npm run dev (in etrid-wallet directory)');
  console.log('  2. Open Chrome DevTools (F12) → Device Mode');
  console.log('  3. Set to iPhone 14 Pro Max (1170x2532)');
  console.log('  4. Navigate to each screen and screenshot');
  console.log('  5. Save to public/screenshots/ with same filenames');
  console.log('✅ Screenshot placeholders complete\n');
}

// Run the generator
generateAllAssets().catch(error => {
  console.error('\n❌ Error generating assets:', error);
  process.exit(1);
});
