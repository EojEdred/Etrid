# Beta Testing Checklists and Templates

Comprehensive testing checklists for all features across iOS, Android, and PWA.

## Master Testing Checklist

### Pre-Release Testing

- [ ] All features from requirements implemented
- [ ] No critical bugs in backlog
- [ ] Crash rate < 1%
- [ ] All automated tests passing
- [ ] Manual smoke tests completed
- [ ] Performance benchmarks met
- [ ] Security audit completed
- [ ] Legal compliance verified
- [ ] Privacy policy updated
- [ ] Terms of service reviewed

### Platform-Specific

#### iOS
- [ ] TestFlight build uploaded
- [ ] Build processing complete
- [ ] Beta test information complete
- [ ] Internal testers invited
- [ ] External testing approved
- [ ] Public link enabled (if applicable)

#### Android
- [ ] AAB built and signed
- [ ] Uploaded to Play Console
- [ ] Internal testing track created
- [ ] Closed testing track configured
- [ ] Store listing complete
- [ ] Content rating received

#### PWA
- [ ] Deployed to beta subdomain
- [ ] Service worker functioning
- [ ] Offline mode tested
- [ ] Feature flags configured
- [ ] Analytics tracking verified
- [ ] Error monitoring active

## Feature Testing Checklists

### 1. Wallet Creation & Import

#### Test Scenarios

**New Wallet Creation**
```
Test Case: TC-WALLET-001
Title: Create new wallet with 12-word seed phrase

Prerequisites: Fresh app install
Steps:
1. Open app
2. Tap "Create New Wallet"
3. Set wallet name
4. Set PIN code (6 digits)
5. Confirm PIN code
6. View seed phrase (12 words)
7. Confirm seed phrase (select words in order)
8. Complete setup

Expected Results:
✓ Wallet created successfully
✓ Seed phrase displayed (12 words)
✓ Confirmation step requires correct word order
✓ Wallet accessible after creation
✓ Backup reminder shown

Test Data:
- PIN: 123456
- Expected seed words: 12 random words

Pass Criteria:
- All steps complete without errors
- Wallet accessible with PIN
- Seed phrase can be backed up
```

**Import Existing Wallet**
```
Test Case: TC-WALLET-002
Title: Import wallet with seed phrase

Prerequisites: None
Steps:
1. Open app
2. Tap "Import Wallet"
3. Select "Seed Phrase"
4. Enter 12 or 24 word seed phrase
5. Set wallet name
6. Set PIN code
7. Confirm PIN code
8. Complete import

Expected Results:
✓ Wallet imported successfully
✓ Correct balance displayed
✓ Transaction history loaded
✓ All tokens visible

Test Data:
- Seed phrase: [test wallet seed]
- Expected balance: 0.1 ETH
- Expected tokens: USDC, DAI

Pass Criteria:
- Wallet accessible with correct balance
- Transaction history matches blockchain
```

**Checklist:**
- [ ] Create wallet (12-word seed)
- [ ] Create wallet (24-word seed)
- [ ] Import wallet (12-word seed)
- [ ] Import wallet (24-word seed)
- [ ] Import wallet (private key)
- [ ] Import wallet (WalletConnect)
- [ ] Import wallet (hardware wallet)
- [ ] Seed phrase backup flow
- [ ] Seed phrase verification
- [ ] PIN code setup
- [ ] PIN code confirmation
- [ ] Biometric authentication setup
- [ ] Error handling (invalid seed)
- [ ] Error handling (invalid PIN)
- [ ] Cancel during setup
- [ ] App killed during setup

### 2. Send Transactions

#### Test Scenarios

**Send ETH**
```
Test Case: TC-SEND-001
Title: Send ETH to valid address

Prerequisites:
- Wallet with > 0.01 ETH
- Goerli testnet

Steps:
1. Navigate to wallet
2. Tap "Send"
3. Select ETH
4. Enter recipient address
5. Enter amount (0.001 ETH)
6. Review transaction details
7. Confirm with biometric/PIN
8. Wait for confirmation

Expected Results:
✓ Transaction broadcast successfully
✓ Transaction appears in history
✓ Balance updated correctly
✓ Transaction confirmed on-chain

Test Data:
- Recipient: 0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb
- Amount: 0.001 ETH
- Gas fee: ~0.0001 ETH

Pass Criteria:
- Transaction successful
- Correct amount sent
- Balance accurate
- Transaction visible in history
```

**Checklist:**
- [ ] Send ETH (small amount)
- [ ] Send ETH (max amount)
- [ ] Send ERC-20 token
- [ ] Send to ENS domain
- [ ] Send to invalid address (error)
- [ ] Send with insufficient balance (error)
- [ ] Send with custom gas
- [ ] Send with max priority fee
- [ ] QR code scanning
- [ ] Address book selection
- [ ] Recent recipients
- [ ] Transaction confirmation screen
- [ ] Biometric confirmation
- [ ] PIN confirmation
- [ ] Cancel transaction
- [ ] Transaction history updates
- [ ] Balance updates
- [ ] Pending transaction display
- [ ] Failed transaction handling
- [ ] Network switch during send

### 3. Receive Transactions

**Checklist:**
- [ ] Display receive address
- [ ] QR code generation
- [ ] QR code scanning
- [ ] Copy address to clipboard
- [ ] Share address (native share)
- [ ] Request specific amount
- [ ] Request with message
- [ ] Multiple receive addresses (HD wallet)
- [ ] Address format (checksum)
- [ ] Network indicator visible
- [ ] Warning for testnet address

### 4. Transaction History

**Checklist:**
- [ ] List all transactions
- [ ] Sort by date (newest first)
- [ ] Filter by type (send/receive/swap)
- [ ] Filter by token
- [ ] Search by address/hash
- [ ] Transaction details view
- [ ] Transaction status (pending/confirmed/failed)
- [ ] Block explorer link
- [ ] Transaction timestamp
- [ ] Gas fee display
- [ ] USD value at time of transaction
- [ ] Export transaction history (CSV)
- [ ] Pagination (load more)
- [ ] Pull to refresh
- [ ] Empty state (no transactions)

### 5. AU Bloccard

#### Test Scenarios

**Card Application**
```
Test Case: TC-BLOCCARD-001
Title: Complete AU Bloccard application

Prerequisites:
- Verified wallet
- Minimum balance (e.g., $100)

Steps:
1. Navigate to AU Bloccard section
2. Tap "Apply for Card"
3. Enter personal information
4. Upload ID verification
5. Complete KYC process
6. Accept terms and conditions
7. Choose card type (virtual/physical)
8. Submit application

Expected Results:
✓ Application submitted successfully
✓ Verification in progress
✓ Email confirmation received
✓ Application status trackable

Pass Criteria:
- Application accepted or properly rejected with reason
- Status updates visible
```

**Checklist:**
- [ ] View card application form
- [ ] Fill personal information
- [ ] Upload ID document
- [ ] Selfie verification
- [ ] Address verification
- [ ] KYC submission
- [ ] Application status tracking
- [ ] Virtual card creation
- [ ] Physical card order
- [ ] Card activation
- [ ] View card details
- [ ] Copy card number
- [ ] View CVV (with auth)
- [ ] Set card PIN
- [ ] Freeze/unfreeze card
- [ ] Top up from crypto
- [ ] Transaction history
- [ ] Spending limits
- [ ] Card settings
- [ ] Report lost/stolen
- [ ] Cancel card
- [ ] Push notifications for transactions

### 6. Trading & Swaps

**Checklist:**
- [ ] View token list
- [ ] Search tokens
- [ ] Select token to swap
- [ ] Enter swap amount
- [ ] View swap preview
- [ ] View price impact
- [ ] View slippage tolerance
- [ ] Adjust slippage
- [ ] View route (DEX aggregation)
- [ ] Approve token (if needed)
- [ ] Execute swap
- [ ] Swap confirmation
- [ ] Failed swap handling
- [ ] Price update during swap
- [ ] Minimum received amount
- [ ] Max slippage protection
- [ ] Swap history
- [ ] Gas estimation
- [ ] Multi-hop swaps
- [ ] Network switch for swap

### 7. NFT Gallery

**Checklist:**
- [ ] Load NFT collection
- [ ] Display NFT images
- [ ] Display NFT metadata
- [ ] View NFT details
- [ ] View NFT properties
- [ ] View collection info
- [ ] View floor price
- [ ] Transfer NFT
- [ ] List NFT for sale
- [ ] View NFT transaction history
- [ ] Filter by collection
- [ ] Search NFTs
- [ ] Sort options
- [ ] Grid/list view toggle
- [ ] Refresh collection
- [ ] Hidden NFTs feature
- [ ] Spam NFT detection
- [ ] Opensea link
- [ ] Share NFT

### 8. DeFi Dashboard

**Checklist:**
- [ ] View total portfolio value
- [ ] View token balances
- [ ] View staked positions
- [ ] View liquidity positions
- [ ] View yield farming positions
- [ ] APY/APR display
- [ ] Rewards tracking
- [ ] Claim rewards
- [ ] Stake tokens
- [ ] Unstake tokens
- [ ] Add liquidity
- [ ] Remove liquidity
- [ ] Position details
- [ ] Historical performance
- [ ] Price charts
- [ ] 24h change
- [ ] Gainers/losers
- [ ] Watchlist

### 9. Security Features

**Checklist:**
- [ ] PIN code setup
- [ ] PIN code change
- [ ] PIN code reset (with seed)
- [ ] Biometric authentication (Face ID)
- [ ] Biometric authentication (Touch ID)
- [ ] Biometric authentication (Android)
- [ ] Auto-lock timer
- [ ] Manual lock
- [ ] Lock on background
- [ ] Seed phrase backup
- [ ] Seed phrase verification
- [ ] Seed phrase export
- [ ] Private key export
- [ ] Transaction signing confirmation
- [ ] WalletConnect session approval
- [ ] dApp connection approval
- [ ] Phishing warning
- [ ] Address validation
- [ ] Network warning
- [ ] Hardware wallet connection

### 10. Settings & Preferences

**Checklist:**
- [ ] Change language
- [ ] Change currency (USD, EUR, etc.)
- [ ] Change theme (light/dark)
- [ ] Notification preferences
- [ ] Privacy settings
- [ ] Network selection
- [ ] Custom RPC endpoint
- [ ] Gas price settings
- [ ] Default slippage
- [ ] Address book management
- [ ] About section
- [ ] Version number display
- [ ] Terms of service link
- [ ] Privacy policy link
- [ ] Support/help link
- [ ] Send feedback
- [ ] Export wallet
- [ ] Delete wallet
- [ ] Logout

## Platform-Specific Testing

### iOS Testing

**Device Coverage:**
- [ ] iPhone SE (small screen)
- [ ] iPhone 14 (standard)
- [ ] iPhone 14 Pro Max (large)
- [ ] iPad (if supported)

**iOS Versions:**
- [ ] iOS 15
- [ ] iOS 16
- [ ] iOS 17

**iOS-Specific Features:**
- [ ] Face ID authentication
- [ ] Touch ID authentication
- [ ] Haptic feedback
- [ ] Native share sheet
- [ ] Wallet integration
- [ ] App clips (if applicable)
- [ ] Widgets (if applicable)
- [ ] Siri shortcuts (if applicable)
- [ ] Deep linking
- [ ] Universal links
- [ ] Background refresh
- [ ] Push notifications
- [ ] App icon badge

### Android Testing

**Device Coverage:**
- [ ] Samsung Galaxy S23
- [ ] Google Pixel 7
- [ ] OnePlus device
- [ ] Budget device (<$300)

**Android Versions:**
- [ ] Android 10
- [ ] Android 11
- [ ] Android 12
- [ ] Android 13

**Android-Specific Features:**
- [ ] Fingerprint authentication
- [ ] Face unlock
- [ ] Gesture navigation
- [ ] Native share sheet
- [ ] App shortcuts
- [ ] Widgets (if applicable)
- [ ] Deep linking
- [ ] App links
- [ ] Background services
- [ ] Push notifications
- [ ] Notification channels

### PWA Testing

**Browser Coverage:**
- [ ] Chrome (desktop)
- [ ] Chrome (mobile)
- [ ] Safari (desktop)
- [ ] Safari (iOS)
- [ ] Firefox
- [ ] Edge

**PWA Features:**
- [ ] Install prompt
- [ ] Add to home screen
- [ ] Standalone mode
- [ ] Service worker
- [ ] Offline functionality
- [ ] Cache strategy
- [ ] Push notifications (where supported)
- [ ] Background sync
- [ ] Web Share API
- [ ] Clipboard API
- [ ] Biometric auth (WebAuthn)

## Performance Testing

### Load Time Benchmarks

**Targets:**
- App startup (cold): < 2 seconds
- App startup (warm): < 1 second
- Screen transitions: < 300ms
- API calls: < 500ms (p95)
- Image loading: < 1 second

**Checklist:**
- [ ] Cold start time
- [ ] Warm start time
- [ ] Screen navigation speed
- [ ] List scrolling performance
- [ ] Image loading performance
- [ ] Animation frame rate (60fps)
- [ ] Memory usage
- [ ] Battery drain
- [ ] Network usage
- [ ] Storage usage

### Stress Testing

**Scenarios:**
- [ ] 100+ tokens in wallet
- [ ] 1000+ transactions in history
- [ ] 100+ NFTs in gallery
- [ ] Rapid screen switching
- [ ] Background/foreground switching
- [ ] Low battery mode
- [ ] Airplane mode
- [ ] Poor network (2G/3G)
- [ ] Network switching (WiFi ↔ Mobile)
- [ ] App killed and restored
- [ ] Multiple wallets

## Accessibility Testing

**Checklist:**
- [ ] Screen reader support (VoiceOver/TalkBack)
- [ ] Font scaling (small, large, extra large)
- [ ] Color contrast (WCAG AA)
- [ ] Touch target size (minimum 44x44pt)
- [ ] Keyboard navigation (web)
- [ ] Focus indicators
- [ ] Alternative text for images
- [ ] Labels for form inputs
- [ ] Error messages readable
- [ ] Success messages readable

## Security Testing

**Checklist:**
- [ ] Seed phrase never logged
- [ ] Private keys never exposed
- [ ] Secure storage (Keychain/Keystore)
- [ ] HTTPS only
- [ ] Certificate pinning
- [ ] SQL injection prevention
- [ ] XSS prevention
- [ ] CSRF protection
- [ ] Rate limiting
- [ ] Input validation
- [ ] Output encoding
- [ ] Session management
- [ ] Jailbreak/root detection
- [ ] Screenshot blocking (sensitive screens)
- [ ] Clipboard clearing (sensitive data)

## Bug Report Template

```markdown
## Bug Report

**Bug ID:** BR-[YYYYMMDD]-[###]
**Reported by:** [Name/Email]
**Date:** [YYYY-MM-DD]
**Platform:** iOS / Android / PWA
**Version:** [e.g., 1.0.0-beta.5]

### Environment
- Device: [e.g., iPhone 14 Pro]
- OS Version: [e.g., iOS 17.0]
- App Version: [e.g., 1.0.0-beta.5 (build 42)]
- Network: WiFi / 4G / 5G

### Summary
[Brief description of the bug]

### Severity
- [ ] Critical (app unusable, data loss, security issue)
- [ ] High (major feature broken)
- [ ] Medium (minor feature issue, workaround exists)
- [ ] Low (cosmetic, minor inconvenience)

### Steps to Reproduce
1. [First step]
2. [Second step]
3. [Third step]

### Expected Behavior
[What should happen]

### Actual Behavior
[What actually happens]

### Screenshots/Videos
[Attach screenshots or screen recording]

### Logs
[Attach relevant logs if available]

### Additional Context
[Any other relevant information]

### Reproducibility
- [ ] Always reproducible
- [ ] Sometimes reproducible
- [ ] Happened once

### Workaround
[Is there a workaround? If yes, describe]
```

## Feature Request Template

```markdown
## Feature Request

**FR ID:** FR-[YYYYMMDD]-[###]
**Requested by:** [Name/Email]
**Date:** [YYYY-MM-DD]

### Summary
[Brief description of the feature]

### Problem Statement
[What problem does this solve?]

### Proposed Solution
[How should it work?]

### User Story
As a [type of user], I want to [action] so that [benefit].

### Priority
- [ ] Must have (critical for success)
- [ ] Should have (important but not critical)
- [ ] Nice to have (would improve experience)

### Platform
- [ ] iOS
- [ ] Android
- [ ] PWA
- [ ] All platforms

### Mockups/Examples
[Attach mockups or examples from other apps]

### Acceptance Criteria
- [ ] [Criterion 1]
- [ ] [Criterion 2]
- [ ] [Criterion 3]

### Additional Context
[Any other relevant information]
```

## Test Cycle Report Template

```markdown
# Beta Test Cycle Report

**Cycle:** Beta [#]
**Date Range:** [Start Date] - [End Date]
**Version:** [e.g., 1.0.0-beta.5]

## Summary
[High-level summary of test cycle]

## Metrics

### Quality Metrics
- Crash rate: [X]%
- Crash-free users: [X]%
- Bug reports: [X]
  - Critical: [X]
  - High: [X]
  - Medium: [X]
  - Low: [X]

### Engagement Metrics
- Active testers: [X]
- Daily active users: [X]
- Average session length: [X] minutes
- Feedback submissions: [X]

### Platform Breakdown
- iOS testers: [X]
- Android testers: [X]
- PWA users: [X]

## Top Issues
1. [Issue 1] - [Severity] - [Status]
2. [Issue 2] - [Severity] - [Status]
3. [Issue 3] - [Severity] - [Status]

## Feature Highlights
- [Feature 1] - [Adoption rate]%
- [Feature 2] - [Adoption rate]%
- [Feature 3] - [Adoption rate]%

## Tester Feedback Summary
[Summary of qualitative feedback]

## Action Items for Next Cycle
- [ ] [Action 1]
- [ ] [Action 2]
- [ ] [Action 3]

## Go/No-Go for Next Phase
- [ ] All critical bugs resolved
- [ ] Crash rate < 1%
- [ ] Positive tester sentiment
- [ ] Performance targets met

**Decision:** GO / NO-GO

**Next Steps:**
[What happens next]
```

## Resources

- [Software Testing Best Practices](https://www.softwaretestinghelp.com/software-testing-best-practices/)
- [Mobile App Testing Checklist](https://www.testlio.com/blog/mobile-app-testing-checklist/)
- [PWA Testing Guide](https://web.dev/pwa-checklist/)

## Support

Questions? Contact:
- Email: qa@etrid.com
- Discord: #qa-testing
- Internal Wiki: https://wiki.etrid.com/testing
