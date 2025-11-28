# TestFlight Beta Testing Setup

## Prerequisites

- Apple Developer Account ($99/year)
- Xcode 15+
- Physical iOS device for testing
- App Store Connect access
- Valid provisioning profiles and certificates

## Step 1: App Store Connect Setup

1. Go to https://appstoreconnect.apple.com
2. Click "My Apps" → "+" → "New App"
3. Fill in details:
   - Platform: iOS
   - Name: Ëtrid Wallet
   - Primary Language: English (U.S.)
   - Bundle ID: com.etrid.wallet
   - SKU: etrid-wallet-ios
   - User Access: Full Access

## Step 2: Build Archive

```bash
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet-native

# Install dependencies
npm install

# Install iOS dependencies
cd ios && pod install && cd ..

# Open in Xcode
open ios/EtridWallet.xcworkspace
```

### In Xcode:

1. Select "Any iOS Device" as target
2. Product → Archive
3. Wait for archive to complete (5-10 minutes)
4. Click "Distribute App"
5. Select "App Store Connect"
6. Select distribution certificate
7. Upload (may take 10-30 minutes)

### Alternative: Command Line Build

```bash
# Build archive from command line
xcodebuild -workspace ios/EtridWallet.xcworkspace \
  -scheme EtridWallet \
  -configuration Release \
  -archivePath build/EtridWallet.xcarchive \
  archive

# Upload to App Store Connect
xcodebuild -exportArchive \
  -archivePath build/EtridWallet.xcarchive \
  -exportOptionsPlist ExportOptions.plist \
  -exportPath build/

# Upload IPA
xcrun altool --upload-app \
  --type ios \
  --file build/EtridWallet.ipa \
  --username "your@email.com" \
  --password "@keychain:AC_PASSWORD"
```

## Step 3: TestFlight Configuration

1. Go to App Store Connect → TestFlight
2. Wait for build to process (10-60 minutes)
3. Add Beta Test Information:

### Beta App Name
```
Ëtrid Wallet Beta
```

### Beta App Description
```
Test the complete DeFi wallet with crypto debit card, trading, NFTs, and more.

This is a beta version - expect bugs and frequent updates.

Features to test:
- Wallet creation & secure backup
- Send/receive transactions
- AU Bloccard (crypto debit card)
- Trading & DeFi features
- NFT gallery & marketplace
- Biometric authentication
- Multi-chain support

Please report any issues you find!
```

### Contact Information
- Feedback Email: beta@etrid.com
- Marketing URL: https://wallet.etrid.com
- Privacy Policy URL: https://wallet.etrid.com/privacy

### What to Test (Update for each build)
```
Version 1.0.0 Beta 1 - Initial Release

Focus areas for this build:
1. Wallet creation flow
2. Biometric authentication (Face ID/Touch ID)
3. Send/receive functionality
4. Transaction history
5. UI/UX feedback
6. Performance and stability

Known issues:
- Push notifications may be delayed
- Some animations need polish
- NFT images may load slowly

Please test on:
- iOS 15, 16, 17
- iPhone (all models from iPhone X+)
- iPad (optional)
- Various network conditions (WiFi, 4G, 5G, airplane mode)

Critical test scenarios:
- Create new wallet and backup seed phrase
- Import existing wallet
- Send small test transaction
- Receive transaction
- Test biometric unlock
- Test app in background/foreground switching
```

## Step 4: Add Beta Testers

### Internal Testing (25 users max)

Internal testing is instant - no review needed!

1. Go to TestFlight → Internal Testing
2. Create Group: "Internal Team"
3. Add team members:
   - By email (must have App Store Connect access)
   - Or select existing users
4. Testers receive invite email immediately
5. Build available instantly after upload completes

**Internal Test Group Examples:**
- Core Team (developers, designers)
- QA Team
- Leadership
- Close advisors

### External Testing (10,000 users max)

External testing requires Apple review (1-2 business days).

1. Go to TestFlight → External Testing
2. Create Group: "Public Beta"
3. Add testers:
   - Manually by email (CSV upload supported), OR
   - Generate Public Link (anyone can join)
4. Complete Beta App Review information
5. Submit for Beta App Review
6. Wait for approval (typically 24-48 hours)
7. Once approved, testers can install

**External Test Group Examples:**
- Community Beta (Discord members)
- Early Supporters
- Public Beta (via public link)
- Partner Testing

### Beta App Review Checklist

Required information for external testing:

- [ ] Contact information (email, phone)
- [ ] Beta app description
- [ ] What to test notes
- [ ] Test account credentials (if login required)
- [ ] Special instructions (if any)
- [ ] Export compliance information
- [ ] Age rating

## Step 5: Public Link Setup

**Best for: Open beta testing with unlimited sign-ups**

1. Create public link: TestFlight → External Testing → Enable Public Link
2. You'll get a link like: `https://testflight.apple.com/join/ABC12DEF`
3. Share on:
   - Twitter/X
   - Discord
   - Landing page (https://wallet.etrid.com/beta)
   - Email newsletter
   - Reddit
   - Telegram

### Public Link Landing Page

```html
<!-- Add to landing page -->
<div class="testflight-cta">
  <h2>Test Ëtrid Wallet on iOS</h2>
  <p>Join 10,000 beta testers</p>
  <a href="https://testflight.apple.com/join/ABC12DEF"
     class="btn-primary">
    Join TestFlight Beta
  </a>
  <p class="small">Requires iOS 15 or later</p>
</div>
```

## Beta Testing Best Practices

### What to Test - Complete Checklist

#### Core Wallet Features
- [ ] Wallet creation (new wallet)
- [ ] Import wallet (12/24 word seed phrase)
- [ ] Import wallet (private key)
- [ ] Backup seed phrase flow
- [ ] Biometric authentication (Face ID/Touch ID)
- [ ] PIN code setup
- [ ] Auto-lock functionality

#### Transaction Features
- [ ] Send transaction (small amount)
- [ ] Send transaction (max amount)
- [ ] Receive transaction
- [ ] QR code generation
- [ ] QR code scanning
- [ ] Transaction history
- [ ] Transaction details
- [ ] Pending transactions
- [ ] Failed transaction handling

#### AU Bloccard Features
- [ ] Card application flow
- [ ] KYC verification
- [ ] Card activation
- [ ] Virtual card display
- [ ] Top-up from crypto
- [ ] Transaction notifications
- [ ] Spending limits
- [ ] Card freeze/unfreeze

#### Trading Features
- [ ] Token swap
- [ ] Liquidity provision
- [ ] Staking
- [ ] Price charts
- [ ] Market data
- [ ] Trade history

#### NFT Features
- [ ] NFT gallery view
- [ ] NFT details
- [ ] NFT transfer
- [ ] NFT marketplace browse
- [ ] NFT purchase

#### App Performance
- [ ] App launch time (cold start)
- [ ] App launch time (warm start)
- [ ] Screen transitions
- [ ] Animations smoothness
- [ ] Background/foreground switching
- [ ] Memory usage
- [ ] Battery usage
- [ ] Network performance (slow connection)
- [ ] Offline mode behavior

#### Push Notifications
- [ ] Transaction received
- [ ] Transaction confirmed
- [ ] Security alerts
- [ ] Price alerts
- [ ] Card transaction alerts

#### Device Compatibility
- [ ] iPhone SE (small screen)
- [ ] iPhone 14/15 (standard)
- [ ] iPhone 14/15 Pro Max (large)
- [ ] iPad (if supported)
- [ ] iOS 15
- [ ] iOS 16
- [ ] iOS 17

### Feedback Collection Methods

#### 1. TestFlight Built-in Feedback
- Screenshots
- Crash reports
- In-app feedback

#### 2. In-App Feedback Button
```
Settings → "Send Feedback"
or
Shake device → "Report Bug"
```

#### 3. Email
```
beta@etrid.com
Include:
- iOS version
- Device model
- Build number
- Steps to reproduce
- Screenshots/screen recording
```

#### 4. Discord Channel
```
#beta-testing channel
Real-time discussion
Quick bug reports
Feature requests
```

#### 5. Weekly Survey
```
Every Friday:
- What worked well?
- What didn't work?
- Most confusing feature?
- Feature requests?
- Overall rating (1-10)
```

### Update Cadence

#### Phase 1: Rapid Iteration (Week 1-2)
- Daily builds
- Small internal team only
- Focus on critical bugs
- Fast feedback loops

#### Phase 2: Expansion (Week 3-4)
- 2-3 builds per week
- Expand to external beta
- Feature refinements
- Performance optimization

#### Phase 3: Stabilization (Week 5-6)
- Weekly builds
- Bug fixes only
- Polish and refinement
- Prepare for production

#### Phase 4: Pre-Production (Week 7+)
- Bi-weekly builds
- Release candidates
- Final testing
- App Store submission prep

### Metrics to Track

#### Quality Metrics
- Crash rate (target: <1%)
- Crash-free users (target: >99%)
- ANR rate (target: <0.1%)
- Memory warnings
- Battery drain

#### Engagement Metrics
- Session length (target: >2 min)
- Sessions per day
- Feature usage rates
- Retention (D1, D7, D30)
- Churn rate

#### Feedback Metrics
- Feedback volume
- Bug reports per user
- Feature requests
- Average rating
- NPS score

#### Performance Metrics
- App launch time (target: <2s)
- Transaction completion time
- API response time
- Screen render time

### Communication Plan

#### Build Notifications
Every new build, send email with:
- Build number and version
- What's new
- What to test
- Known issues
- ETA for next build

#### Weekly Status Update
Every Monday:
- Last week's progress
- Bugs fixed
- New features
- This week's focus
- Call for specific testing

#### Bug Triage
Daily review:
- P0: Critical (blocks usage) - fix immediately
- P1: High (major feature broken) - fix within 24h
- P2: Medium (minor issue) - fix within week
- P3: Low (polish) - backlog

### TestFlight Limits

Be aware of Apple's limits:

| Limit Type | Value |
|------------|-------|
| Internal testers | 25 users |
| External testers | 10,000 users |
| Test groups | No limit |
| Builds per app | 100 (90-day rolling) |
| Build expiration | 90 days |
| Testing duration | No limit |
| Public link | 1 per group |

## Troubleshooting

### Build Processing Stuck
- Wait 60 minutes (normal processing time)
- Check email for rejection notice
- Verify export compliance settings
- Contact Apple Developer Support

### Testers Not Receiving Invites
- Check spam folder
- Verify email address is correct
- Resend invite from TestFlight
- Check if user's Apple ID is correct region

### Public Link Not Working
- Ensure build is approved for external testing
- Check if 10,000 tester limit reached
- Verify link is enabled in settings
- Test link in incognito/private browser

### Crashes on Specific Devices
- Check Xcode Organizer for crash logs
- Symbolicate crash reports
- Test on physical device
- Review device-specific code paths

## Security Considerations

### Beta Build Security
- Do NOT use production API keys
- Use separate backend environment
- Implement beta-only logging
- Add watermarks to screenshots
- Disable sensitive features if needed

### Tester Verification
- Verify tester identities for internal testing
- Use invitation codes for external testing
- Monitor for suspicious activity
- Revoke access if needed

### Data Protection
- Encrypt all sensitive data
- Use TestFlight entitlements correctly
- Test data isolation from production
- Clear test data before production

## Preparing for Production

### Final Checklist Before App Store Submission

- [ ] All critical bugs fixed
- [ ] Crash rate <0.5%
- [ ] Performance metrics met
- [ ] All features complete
- [ ] Legal review complete
- [ ] Privacy policy updated
- [ ] Terms of service updated
- [ ] App Store screenshots ready
- [ ] App Store description written
- [ ] Keywords optimized
- [ ] Support URL active
- [ ] Privacy nutrition label complete
- [ ] Age rating confirmed
- [ ] Export compliance declared
- [ ] Final TestFlight review passed

### Transition from TestFlight to Production

1. Create production build (no beta flags)
2. Internal testing (2-3 days)
3. External testing final group (1 week)
4. Prepare App Store submission
5. Submit for App Review
6. Wait for approval (1-7 days typically)
7. Release to App Store

### Post-Launch Beta Testing

Continue TestFlight for:
- Early access to new features
- Testing major updates
- Collecting power user feedback
- Building community

## Resources

- [TestFlight Documentation](https://developer.apple.com/testflight/)
- [App Store Connect Help](https://help.apple.com/app-store-connect/)
- [iOS App Distribution Guide](https://developer.apple.com/distribute/)
- [Beta Testing Best Practices](https://developer.apple.com/testflight/testers/)

## Support

Questions? Contact:
- Email: dev@etrid.com
- Discord: #ios-development
- Internal Wiki: https://wiki.etrid.com/testflight
