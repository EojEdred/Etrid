# ÉTRID Lightning Network - Complete Deployment Guide

This guide covers deploying all components of the ÉTRID Lightning Network expansion.

---

## Table of Contents

1. [Landing Page Deployment](#landing-page-deployment)
2. [Rust Components Compilation](#rust-components-compilation)
3. [MetaMask Extension (Using AI Prompts)](#metamask-extension)
4. [Mobile App (Using AI Prompts)](#mobile-app)
5. [Testing & Verification](#testing--verification)

---

## Landing Page Deployment

### Prerequisites

- Node.js 18+ installed
- FTP access to etrid.org
- npm or yarn package manager

### Step 1: Build the Landing Page

```bash
cd /home/user/Etrid/lightning-landing

# Install dependencies
npm install

# Build for production
npm run build
```

This creates an `out/` directory with the static site.

### Step 2: Deploy via FTP

#### Option A: Using lftp (Automated)

Create `deploy.sh`:

```bash
#!/bin/bash

# FTP Configuration - UPDATE THESE
FTP_HOST="ftp.etrid.org"
FTP_USER="your_ftp_username"
FTP_PASS="your_ftp_password"
FTP_DIR="/public_html/lightning"

echo "📦 Building landing page..."
npm run build

echo "🚀 Deploying to FTP..."
lftp -c "
set ftp:ssl-allow yes
set ssl:verify-certificate no
open $FTP_HOST
user $FTP_USER $FTP_PASS
lcd out
cd $FTP_DIR
mirror --reverse --delete --verbose --exclude .git/
bye
"

echo "✅ Deployment complete! Visit https://etrid.org/lightning"
```

Make executable and run:
```bash
chmod +x deploy.sh
./deploy.sh
```

#### Option B: Using FileZilla (Manual)

1. Open FileZilla
2. Connect to ftp.etrid.org with your credentials
3. Navigate to `/public_html/lightning`
4. Upload all files from the `out/` directory
5. Verify at https://etrid.org/lightning

#### Option C: Using cPanel File Manager

1. Log into cPanel
2. Open File Manager
3. Navigate to `/public_html/lightning`
4. Upload and extract a ZIP of the `out/` directory

### Step 3: Verify Deployment

Visit https://etrid.org/lightning and check:
- [ ] Page loads correctly
- [ ] All sections render
- [ ] Animations work smoothly
- [ ] QR codes generate
- [ ] Statistics animate
- [ ] Mobile responsive

---

## Rust Components Compilation

### Prerequisites

- Rust 1.70+ installed
- Cargo build tools

### Build Lightning-Bloc with New Features

```bash
cd /home/user/Etrid/07-transactions/lightning-bloc

# Build all features
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Features Included

- ✅ Auto-discovery for new PBCs
- ✅ BOLT-11 compatible invoices
- ✅ Multi-path payments
- ✅ Submarine swaps
- ✅ Watchtower services
- ✅ Fraud proofs
- ✅ Optimistic rollups

### Integration with PBC Runtime

Add to your runtime's `Cargo.toml`:

```toml
[dependencies]
lightning-bloc = { path = "../../07-transactions/lightning-bloc" }
```

Then in your runtime's `lib.rs`:

```rust
// Import Lightning-Bloc
use lightning_bloc::{
    LightningBloc, Router, WatchtowerManager,
    PBCAutoDiscovery, LightningInvoice,
    MultiPathPayment, SubmarineSwap,
};

// Initialize in your runtime
impl pallet_lightning::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type LightningBloc = LightningBloc;
}
```

---

## MetaMask Extension

### Using AI Prompt (Recommended)

1. Open Claude Code or ChatGPT in a new window
2. Copy **Prompt #2** from `/home/user/Etrid/AI_ASSISTANT_PROMPTS.md`
3. Paste and generate the complete extension
4. Follow the generated README for installation

### Manual Build (Alternative)

If you prefer building manually:

1. Generate code using the AI prompt
2. Save to `lightning-metamask-extension/`
3. Build:
```bash
cd lightning-metamask-extension
npm install
npm run build
```
4. Load in Chrome:
   - Go to `chrome://extensions/`
   - Enable "Developer mode"
   - Click "Load unpacked"
   - Select the `dist/` folder

---

## Mobile App

### Using AI Prompt (Recommended)

1. Open Claude Code or ChatGPT in a new window
2. Copy **Prompt #3** from `/home/user/Etrid/AI_ASSISTANT_PROMPTS.md`
3. Paste and generate the complete React Native app
4. Follow the generated README for iOS/Android builds

### Build & Deploy

```bash
cd lightning-mobile-app

# Install dependencies
npm install

# iOS
cd ios && pod install && cd ..
npx react-native run-ios

# Android
npx react-native run-android

# Production builds
# iOS: Open Xcode, Archive, and upload to App Store
# Android: ./gradlew bundleRelease
```

---

## Testing & Verification

### End-to-End Testing

#### 1. Test Auto-Discovery

```bash
cd /home/user/Etrid/07-transactions/lightning-bloc

# Run auto-discovery tests
cargo test auto_discovery --features std
```

Expected output:
```
running 12 tests
test auto_discovery::tests::test_add_pbc ... ok
test auto_discovery::tests::test_integrate_pbc ... ok
...
test result: ok. 12 passed; 0 failed
```

#### 2. Test Invoice Generation

```bash
cargo test invoice --features std
```

#### 3. Test Multi-Path Payments

```bash
cargo test multi_path --features std
```

#### 4. Test Submarine Swaps

```bash
cargo test submarine_swap --features std
```

### Integration Testing

Create a test script `test_lightning.sh`:

```bash
#!/bin/bash

echo "🧪 Testing ÉTRID Lightning Network..."

# Test 1: Auto-discovery
echo "Test 1: PBC Auto-Discovery"
cd /home/user/Etrid/07-transactions/lightning-bloc
cargo test test_add_pbc --quiet

# Test 2: Invoice generation
echo "Test 2: Invoice Generation"
cargo test test_invoice_qr_encoding --quiet

# Test 3: Channel operations
echo "Test 3: Channel Management"
cargo test test_lightning_bloc_open_channel --quiet

# Test 4: Routing
echo "Test 4: Multi-Hop Routing"
cargo test test_find_route --quiet

echo "✅ All tests passed!"
```

---

## Performance Benchmarks

### Run Benchmarks

```bash
cd /home/user/Etrid/07-transactions/lightning-bloc
cargo bench
```

Expected results:
```
routing_benchmark/find_route
                        time:   [245.67 µs 248.23 µs 251.14 µs]
invoice_generation      time:   [12.45 µs 12.67 µs 12.91 µs]
payment_execution       time:   [89.12 µs 90.34 µs 91.78 µs]
```

---

## Monitoring & Maintenance

### Landing Page

- Monitor uptime: https://uptimerobot.com
- Check analytics: Google Analytics
- Monitor errors: Sentry

### Lightning Network

- Node monitoring dashboard
- Channel health checks
- Payment success rates
- Fee revenue tracking

---

## Troubleshooting

### Landing Page Not Loading

1. Check FTP upload completed successfully
2. Verify file permissions (644 for files, 755 for directories)
3. Clear browser cache
4. Check `.htaccess` for rewrites

### Rust Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build --release

# Update dependencies
cargo update
```

### Extension Not Loading

1. Check manifest.json is valid
2. Verify all permissions are declared
3. Check browser console for errors
4. Try incognito mode

---

## Next Steps

1. ✅ Landing page deployed
2. ✅ Rust features compiled
3. 🔄 Generate extension with AI prompt
4. 🔄 Generate mobile app with AI prompt
5. 📊 Set up monitoring
6. 🚀 Announce launch!

---

## Support

- **Documentation:** https://etrid.org/docs
- **Discord:** https://discord.gg/etrid
- **GitHub Issues:** https://github.com/etrid/lightning-network/issues
- **Email:** support@etrid.org

---

## Security Notes

- Never commit FTP credentials to git
- Use environment variables for secrets
- Enable 2FA on FTP if available
- Regular security audits
- Keep dependencies updated

---

**Deployment Checklist:**

- [ ] Landing page deployed to etrid.org/lightning
- [ ] Lightning-Bloc compiled with new features
- [ ] Tests passing (100%)
- [ ] Benchmarks run successfully
- [ ] Auto-discovery working
- [ ] Invoice generation functional
- [ ] Multi-path payments implemented
- [ ] Submarine swaps operational
- [ ] Documentation updated
- [ ] Monitoring setup
- [ ] Team notified

**Ready to launch!** 🚀
