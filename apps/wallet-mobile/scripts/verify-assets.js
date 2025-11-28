const fs = require('fs').promises;
const path = require('path');

const REQUIRED_ASSETS = {
  pwaIcons: {
    dir: 'etrid-wallet/public/icons',
    files: [
      'icon-72x72.png',
      'icon-96x96.png',
      'icon-128x128.png',
      'icon-144x144.png',
      'icon-152x152.png',
      'icon-192x192.png',
      'icon-384x384.png',
      'icon-512x512.png',
      'apple-touch-icon.png',
      'favicon-32x32.png',
      'favicon-16x16.png'
    ]
  },
  iosSplash: {
    dir: 'etrid-wallet/public/splash',
    files: [
      'iphone5_splash.png',
      'iphone6_splash.png',
      'iphoneplus_splash.png',
      'iphonex_splash.png',
      'iphonexr_splash.png',
      'iphonexsmax_splash.png',
      'ipad_splash.png',
      'ipadpro1_splash.png',
      'ipadpro2_splash.png'
    ]
  },
  screenshots: {
    dir: 'etrid-wallet/public/screenshots',
    files: [
      'home-screenshot.png',
      'trading-screenshot.png',
      'nft-screenshot.png',
      'card-screenshot.png'
    ]
  },
  landingPage: {
    dir: 'landing-page/public',
    files: [
      'logo.png',
      'logo.svg',
      'og-image.png',
      'twitter-card.png'
    ],
    optional: true
  },
  androidIcons: {
    dir: 'etrid-wallet-native/android/app/src/main/res',
    files: [
      'mipmap-mdpi/ic_launcher.png',
      'mipmap-mdpi/ic_launcher_round.png',
      'mipmap-hdpi/ic_launcher.png',
      'mipmap-hdpi/ic_launcher_round.png',
      'mipmap-xhdpi/ic_launcher.png',
      'mipmap-xhdpi/ic_launcher_round.png',
      'mipmap-xxhdpi/ic_launcher.png',
      'mipmap-xxhdpi/ic_launcher_round.png',
      'mipmap-xxxhdpi/ic_launcher.png',
      'mipmap-xxxhdpi/ic_launcher_round.png'
    ],
    optional: true
  }
};

async function verifyAssets() {
  console.log('🔍 Verifying Etrid Wallet assets...\n');

  let totalFiles = 0;
  let existingFiles = 0;
  let missingFiles = 0;
  const missing = [];

  for (const [category, config] of Object.entries(REQUIRED_ASSETS)) {
    const categoryName = category.replace(/([A-Z])/g, ' $1').trim();
    console.log(`\n📁 ${categoryName}:`);

    const baseDir = path.join(__dirname, '..', config.dir);

    // Check if directory exists
    try {
      await fs.access(baseDir);
    } catch {
      if (config.optional) {
        console.log(`  ⚠️  Directory not found (optional): ${config.dir}`);
        continue;
      } else {
        console.log(`  ❌ Directory missing: ${config.dir}`);
        missing.push({ category: categoryName, file: config.dir, reason: 'Directory not found' });
        continue;
      }
    }

    let categoryExists = 0;
    let categoryMissing = 0;

    for (const file of config.files) {
      totalFiles++;
      const filePath = path.join(baseDir, file);

      try {
        const stats = await fs.stat(filePath);
        const sizeKB = (stats.size / 1024).toFixed(2);
        console.log(`  ✅ ${file} (${sizeKB} KB)`);
        existingFiles++;
        categoryExists++;
      } catch {
        console.log(`  ❌ ${file}`);
        missingFiles++;
        categoryMissing++;
        missing.push({ category: categoryName, file, path: filePath });
      }
    }

    console.log(`  → ${categoryExists}/${config.files.length} files present`);
  }

  // Summary
  console.log('\n' + '='.repeat(60));
  console.log('📊 VERIFICATION SUMMARY');
  console.log('='.repeat(60));
  console.log(`✅ Existing files: ${existingFiles}`);
  console.log(`❌ Missing files:  ${missingFiles}`);
  console.log(`📈 Total expected: ${totalFiles}`);
  console.log(`📊 Completion:     ${((existingFiles / totalFiles) * 100).toFixed(1)}%`);

  if (missingFiles > 0) {
    console.log('\n⚠️  MISSING ASSETS:');
    console.log('='.repeat(60));

    for (const { category, file, path: filePath, reason } of missing) {
      console.log(`\n${category}:`);
      console.log(`  File: ${file}`);
      if (filePath) console.log(`  Path: ${filePath}`);
      if (reason) console.log(`  Reason: ${reason}`);
    }

    console.log('\n💡 To generate missing assets:');
    console.log('   npm run generate:assets');

    process.exit(1);
  } else {
    console.log('\n✅ All required assets are present!');
    console.log('\n📋 Next steps:');
    console.log('  1. Replace screenshot placeholders with real app screenshots');
    console.log('  2. Optimize images: npm run optimize:images');
    console.log('  3. Build and test PWA: cd etrid-wallet && npm run build');
    console.log('  4. Test app installation on real devices');

    process.exit(0);
  }
}

verifyAssets().catch(error => {
  console.error('\n❌ Verification error:', error);
  process.exit(1);
});
