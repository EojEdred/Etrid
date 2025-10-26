# Video Tutorial 01: Getting Started with Etrid

**Duration:** 5 minutes
**Target Audience:** Blockchain beginners, new Etrid users
**Prerequisites:** None

---

## Script Overview

This tutorial introduces viewers to Etrid, guides them through wallet installation, account creation, and their first transaction.

---

## Time Markers & Script

### 00:00 - 00:30 | Introduction & Hook

**NARRATION:**
"Welcome to Etrid - the next-generation multichain blockchain platform. In this tutorial, you'll learn what makes Etrid unique, create your first account, and send your first transaction. By the end of these 5 minutes, you'll be ready to start your blockchain journey. Let's dive in!"

**VISUAL CUES:**
- Animated Etrid logo reveal
- Show montage: FlareChain visualization, cross-chain transactions, governance voting
- Transition to presenter or screenshare

**KEY POINTS TO EMPHASIZE:**
- Beginner-friendly tutorial
- Complete walkthrough from zero to first transaction
- No prior blockchain experience needed

---

### 00:30 - 01:30 | What is Etrid and Why Use It?

**NARRATION:**
"Etrid is a multichain blockchain platform that solves three major problems in blockchain today. First, it provides true interoperability through 13 Partition Burst Chains - or PBCs - that connect Bitcoin, Ethereum, Solana, and 10 other major blockchains. Second, it offers lightning-fast transactions through our Layer 2 Lightning-Bloc payment channels with instant finality. Third, it's truly democratic with on-chain governance through our annual Consensus Day event, where YOU vote on network upgrades and fiscal policy.

What makes Etrid special? We use Adaptive Stake Finality - a novel consensus mechanism that combines proof of stake with coinage tracking to prevent centralization. Blocks are finalized in just 15 seconds, and you can stake your ETR tokens to earn rewards while helping secure the network."

**VISUAL CUES:**
- Architecture diagram: FlareChain relay + 13 PBCs
- Animation showing transaction flow: User → Lightning-Bloc → FlareChain → Finality
- Governance dashboard screenshot
- ASF consensus visualization (stake + time = voting power)
- Comparison chart: Etrid vs. other chains (speed, interoperability, governance)

**KEY POINTS TO EMPHASIZE:**
- Three unique value propositions: interoperability, speed, democracy
- 13 PBC connections to major chains
- 15-second finality
- User-controlled governance
- Adaptive Stake Finality prevents centralization

**COMMON MISTAKES TO MENTION:**
- "Don't confuse PBCs with traditional sidechains - they're specialized bridges with dedicated collators"
- "Consensus Day is annual, not continuous - it's a special governance event"

---

### 01:30 - 02:30 | Installing the Wallet

**NARRATION:**
"Let's get you set up with the Etrid wallet. You have two options: the web wallet for quick access, or the mobile wallet for on-the-go transactions. Today, we'll use the web wallet.

Head to wallet.etrid.io - that's W-A-L-L-E-T dot E-T-R-I-D dot I-O. Click 'Launch Wallet' and you'll see the main interface. Notice you can connect using the Polkadot.js browser extension - if you already have it installed, that's great! If not, we'll create a wallet directly in the interface.

The web wallet supports multiple chains out of the box. You'll see FlareChain - that's our main relay chain - and all 13 PBCs listed on the left sidebar. You can switch between them with a single click."

**VISUAL CUES:**
- Screen recording: Browser navigation to wallet.etrid.io
- Highlight "Launch Wallet" button with cursor
- Show wallet interface with labeled components:
  - Chain selector (left sidebar)
  - Account dropdown (top right)
  - Balance display (center)
  - Send/Receive buttons
- Show Polkadot.js extension popup (if installed)
- Pointer annotations on key UI elements

**DEMO STEPS:**
1. Open browser (Chrome or Firefox recommended)
2. Navigate to wallet.etrid.io
3. Click "Launch Wallet"
4. Wait for WebSocket connection (show connection indicator)
5. Point out chain selector dropdown
6. Point out "Create Account" button

**KEY POINTS TO EMPHASIZE:**
- Web wallet requires no installation - runs in browser
- Mobile wallet available on iOS and Android
- Polkadot.js extension provides added security
- Multi-chain support built-in from day one

**COMMON MISTAKES TO MENTION:**
- "Make sure you're on the official wallet.etrid.io domain - check for HTTPS"
- "Bookmark the correct URL to avoid phishing sites"

---

### 02:30 - 03:45 | Creating Your First Account

**NARRATION:**
"Now let's create your account. Click 'Create Account' in the top right corner. You'll see a 12-word recovery phrase - this is CRITICAL. Write these words down on paper in the exact order shown. Never screenshot this or save it digitally. Anyone with these words can access your funds.

Let's write them down now. Pause the video if you need more time. Got them? Great. Check the box that says 'I have saved my recovery phrase safely' and click 'Continue'.

Now create a strong password. This encrypts your account on this device only. You'll need your recovery phrase to restore your account on other devices, but you'll need this password every time you use this browser.

Give your account a memorable name - like 'My Main Account' - and click 'Create Account'. Congratulations! You now have an Etrid address. It starts with '5' and looks something like this. This is your public address - you can share this to receive funds."

**VISUAL CUES:**
- Screen recording: Full account creation flow
- Zoom in on recovery phrase display
- Show example of writing recovery phrase on paper (props: hand, pen, paper)
- Blur actual recovery phrase for security
- Highlight "I have saved my recovery phrase" checkbox
- Show password strength indicator
- Show account name input field
- Highlight final Etrid address (5...)
- Show copy address button with tooltip

**DEMO STEPS:**
1. Click "Create Account" button
2. Display recovery phrase screen
3. Show writing down phrase on paper (demonstration)
4. Check confirmation checkbox
5. Click "Continue"
6. Enter password (use password manager demo)
7. Confirm password
8. Enter account name: "Tutorial Account"
9. Click "Create Account"
10. Show success message
11. Display account address
12. Click "Copy Address" icon

**CODE TO DISPLAY:**
```
Recovery Phrase Example (BLURRED):
1. ████  2. ████  3. ████  4. ████
5. ████  6. ████  7. ████  8. ████
9. ████ 10. ████ 11. ████ 12. ████

Your Etrid Address:
5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

**KEY POINTS TO EMPHASIZE:**
- Recovery phrase = your funds. Lose it = lose access forever
- Write on paper, store in safe location
- Password is for browser encryption only
- Recovery phrase works on any device
- Address starting with "5" is your public identifier
- You can create multiple accounts

**COMMON MISTAKES TO MENTION:**
- "Never share your recovery phrase - not even with Etrid support"
- "Don't store recovery phrase in cloud storage, emails, or photos"
- "Password managers are great for browser password, but keep recovery phrase offline"
- "Use a strong password with letters, numbers, and symbols"

---

### 03:45 - 04:30 | Receiving Test Tokens

**NARRATION:**
"Before you can send a transaction, you need some ETR tokens. For testing, we have a faucet that gives you free testnet tokens.

Copy your address by clicking the copy icon. Now open a new tab and go to faucet.etrid.io. Paste your address in the input field and click 'Request Tokens'. You'll receive 100 test ETR tokens - enough to explore all of Etrid's features.

Wait about 15 seconds for the transaction to finalize. Now refresh your wallet. See that? Your balance updated! You now have 100 ETR tokens ready to use."

**VISUAL CUES:**
- Screen recording: Switching between wallet and faucet tabs
- Show address copy animation
- Navigate to faucet.etrid.io
- Show faucet interface with input field
- Show "Request Tokens" button click
- Show loading animation
- Show success message: "100 ETR sent to your address"
- Switch back to wallet tab
- Show refresh button click
- Highlight balance changing from 0 to 100 ETR
- Show transaction history updating

**DEMO STEPS:**
1. In wallet, click copy icon next to address
2. Open new browser tab
3. Navigate to faucet.etrid.io
4. Paste address in input field
5. Click "Request Tokens"
6. Wait for success message (show 15-second timer)
7. Return to wallet tab
8. Click refresh or wait for auto-update
9. Show balance: 100.0000 ETR
10. Show transaction in history panel

**KEY POINTS TO EMPHASIZE:**
- Faucet is for testnet only - mainnet requires purchasing or earning ETR
- Transactions finalize in ~15 seconds
- Balance shows 4 decimal places precision
- Transaction history shows all activity
- Faucet limited to once per address per day

**COMMON MISTAKES TO MENTION:**
- "Make sure you're connected to testnet, not mainnet"
- "Wait for finality - don't panic if balance doesn't update instantly"
- "Faucet has rate limits - if it fails, try again in a few minutes"

---

### 04:30 - 05:15 | Making Your First Transaction

**NARRATION:**
"Now for the moment you've been waiting for - your first transaction! Click the 'Send' button. You'll need a destination address. For testing, you can send tokens back to the faucet or create a second account and send to yourself.

Let's send 10 ETR back to the faucet. Paste the faucet address here. Enter the amount: 10. See the transaction fee preview? On FlareChain, fees are incredibly low - typically less than 0.01 ETR.

Review everything carefully - blockchain transactions are irreversible. Satisfied? Click 'Sign and Send'. Enter your password to unlock your account. And... sent! See the transaction hash? Click it to view the details. You'll see the confirmation count increasing: 1 of 3, 2 of 3, and finally 3 of 3 confirmations. Finalized in just 15 seconds!

Your balance is now updated: 90 ETR minus the small fee. Check your transaction history - there's your first transaction, immortalized on the blockchain. Congratulations!"

**VISUAL CUES:**
- Screen recording: Complete send transaction flow
- Click "Send" button
- Show send transaction form with labeled fields:
  - Recipient address
  - Amount
  - Asset selector (ETR)
  - Fee estimate
- Show transaction review modal
- Show password unlock dialog
- Show transaction submitted confirmation
- Show block explorer link (clickable)
- Show finality progress: 1/3 → 2/3 → 3/3 blocks
- Show updated balance
- Show transaction in history with "Finalized" status

**DEMO STEPS:**
1. Click "Send" button
2. Paste recipient address (faucet or second account)
3. Enter amount: 10
4. Select asset: ETR (if multiple options)
5. Review fee: ~0.0001 ETR
6. Click "Next"
7. Review transaction details modal
8. Click "Sign and Send"
9. Enter password
10. Click "Confirm"
11. Show transaction submitted message
12. Click transaction hash link
13. Show block explorer with confirmations
14. Return to wallet
15. Show updated balance: ~89.9999 ETR
16. Show transaction history

**CODE TO DISPLAY:**
```
Transaction Details:
From: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
To:   5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc
Amount: 10.0000 ETR
Fee: 0.0001 ETR
Total: 10.0001 ETR

Transaction Hash:
0x8f4a9c2b1e7d6f5a3c8e9b0d2a1f4e7c6b5d8a3f9e2c1b4a7d6f5e8c9b3a2d1f

Status: Finalized (3/3 blocks)
Block Number: 42,857
Timestamp: 2025-10-22 14:32:15 UTC
```

**KEY POINTS TO EMPHASIZE:**
- Always double-check recipient address before sending
- Blockchain transactions are IRREVERSIBLE
- Low fees on Etrid (< 0.01 ETR typical)
- 15-second finality (3 block confirmations)
- Transaction history is permanent and public
- Block explorer provides detailed information

**COMMON MISTAKES TO MENTION:**
- "Never send to an exchange deposit address without checking it supports Etrid"
- "Don't confuse testnet and mainnet addresses"
- "Always leave some ETR for future transaction fees"
- "Copy-paste addresses carefully - one wrong character = lost funds"

---

### 05:15 - 05:30 | Where to Learn More & Outro

**NARRATION:**
"Amazing work! You've just created your first Etrid account and made your first transaction. You're now part of the Etrid ecosystem.

Ready to go deeper? Visit docs.etrid.io for comprehensive guides. Want to earn rewards? Check out our next tutorial on staking and becoming a nominator. Interested in running a validator? We have a complete setup guide for that too.

Join our community on Discord and Telegram - links in the description below. Welcome to Etrid, where the free and open decentralized democracy of stakeholders awaits. See you in the next tutorial!"

**VISUAL CUES:**
- Show success screen with checkmarks:
  - ✅ Account created
  - ✅ Tokens received
  - ✅ First transaction sent
- Display resource links as text overlay:
  - 📚 docs.etrid.io
  - 💬 discord.gg/etrid
  - 📱 t.me/EtridOfficial
  - 🐦 @EtridMultichain
- Show next tutorial thumbnails:
  - "02: Running a Validator"
  - "03: Staking as a Nominator"
  - "04: Deploying Smart Contracts"
  - "05: Building DApps"
- Animated Etrid logo with tagline
- Subscribe/Like/Comment call-to-action

**KEY POINTS TO EMPHASIZE:**
- Tutorial series continues with advanced topics
- Community support available
- Documentation is comprehensive
- Multiple ways to participate in ecosystem

---

## Production Notes

### Visual Assets Needed

**Static Graphics:**
1. Etrid logo (animated reveal version)
2. Architecture diagram (FlareChain + 13 PBCs)
3. ASF consensus visualization
4. Comparison chart: Etrid vs. competitors
5. Recovery phrase security infographic
6. Transaction finality animation (1/3 → 2/3 → 3/3)
7. Resource links end card
8. Tutorial series thumbnails

**Screen Recordings:**
1. Wallet website navigation (wallet.etrid.io)
2. Account creation full flow
3. Faucet interaction (faucet.etrid.io)
4. Send transaction complete flow
5. Block explorer view

**Props:**
1. Paper and pen (for recovery phrase demo)
2. Safe or secure location (to emphasize storage)

### Demo Requirements

**Environment:**
- Etrid testnet node running and accessible
- Web wallet deployed at wallet.etrid.io
- Faucet deployed at faucet.etrid.io
- Block explorer deployed
- Browser with Polkadot.js extension (optional demo)

**Test Accounts:**
- Pre-created faucet account with ETR balance
- Clean browser state (no existing accounts)
- Second test account for receive demo (optional)

**Network:**
- Stable internet connection
- WebSocket connection to testnet node
- No firewall blocking ports 9944/9945

### Editing Notes

**Pacing:**
- Keep introduction punchy (30 seconds max)
- Slow down during recovery phrase section (emphasize importance)
- Add 3-5 second pauses for viewers to follow along
- Use on-screen timers for waiting periods (15s finality)

**Graphics:**
- Add animated highlights/circles on clickable elements
- Use zoom-in for important UI elements
- Add tooltips/labels for unfamiliar terms
- Maintain consistent branding colors

**Audio:**
- Clear voiceover with enthusiasm
- Background music: subtle, non-distracting
- Sound effects for success actions (subtle)
- Silence or pause during "write this down" moments

**Accessibility:**
- Closed captions for all narration
- High contrast for screen recordings
- Readable text size (minimum 18pt on diagrams)
- Audio descriptions for visual elements

---

## Target Metrics

**Engagement Goals:**
- Watch time: 70%+ (viewers complete tutorial)
- Click-through rate: 15%+ to next tutorial
- Faucet usage: 30%+ of viewers request tokens
- Community join rate: 10%+ join Discord/Telegram

**Educational Goals:**
- 95%+ understand what Etrid is
- 90%+ successfully create account
- 85%+ successfully receive faucet tokens
- 80%+ successfully send first transaction

---

**Tutorial Complete**
Next: 02-running-validator.md
