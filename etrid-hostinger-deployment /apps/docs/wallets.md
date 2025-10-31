# ËTRID Wallets

Complete guide to choosing and using ËTRID wallets.

## Wallet Options

| Wallet Type | Best For | Platform | Download |
|-------------|----------|----------|----------|
| **Web Wallet** | Beginners, quick access | Browser | [wallet.etrid.org](https://wallet.etrid.org) |
| **Browser Extension** | Daily users, DApp interaction | Chrome, Firefox, Brave | [Chrome Web Store](https://chrome.google.com/webstore) |
| **Mobile Wallet** | On-the-go transactions | iOS, Android | App Store / Google Play |
| **Hardware Wallet** | Large holdings, max security | Ledger, Trezor | Device + desktop app |
| **CLI Wallet** | Developers, automation | Linux, macOS, Windows | [GitHub Releases](https://github.com/etrid/etrid/releases) |

---

## Web Wallet

**Best for:** Beginners and quick access

### Features
- ✅ No installation required
- ✅ Works on any device with browser
- ✅ Full functionality (send, receive, stake, governance)
- ✅ Beautiful, intuitive interface
- ⚠️ Requires internet connection
- ⚠️ Security depends on browser/device

### Getting Started

1. Visit [wallet.etrid.org](https://wallet.etrid.org)
2. Click **"Create Account"**
3. **Save your 12-word recovery phrase** (CRITICAL!)
4. Set a strong password
5. Name your account
6. Done!

### Security Tips
- Bookmark official URL to avoid phishing
- Use strong password (12+ characters)
- Enable 2FA if supported
- Clear browser cache on shared computers
- Never save recovery phrase digitally

---

## Browser Extension

**Best for:** Daily users and DApp interaction

### Features
- ✅ Quick access from browser toolbar
- ✅ Seamless DApp connection
- ✅ Compatible with Polkadot.js ecosystem
- ✅ Multiple account management
- ✅ Transaction signing for websites
- ⚠️ Must install on each browser

### Supported Extensions

#### Polkadot.js Extension
**Platforms:** Chrome, Firefox, Brave

1. Install from [polkadot.js.org/extension](https://polkadot.js.org/extension)
2. Create or import account
3. Select ËTRID network
4. Connect to [wallet.etrid.org](https://wallet.etrid.org)

#### SubWallet
**Platforms:** Chrome, Firefox, Edge

1. Install from [subwallet.app](https://subwallet.app)
2. Supports multiple chains including ËTRID
3. Built-in DApp browser
4. Portfolio tracking

---

## Mobile Wallet

**Best for:** On-the-go transactions and QR codes

### Features
- ✅ Send/receive with QR codes
- ✅ Biometric authentication (Face ID, fingerprint)
- ✅ Push notifications for transactions
- ✅ Portable, always with you
- ⚠️ Security risk if device lost/stolen

### ËTRID Mobile (iOS & Android)

**Status:** Coming Q1 2026

**Planned Features:**
- QR code scanning for addresses
- Staking interface
- Governance voting
- DApp browser
- Secure enclave integration
- Multi-account support

**Sign up for beta:** [etrid.org/mobile-beta](https://etrid.org/mobile-beta)

### Alternative: Polkadot Vault

1. Install Polkadot Vault (formerly Parity Signer)
2. Create air-gapped account
3. Sign transactions via QR codes
4. Use with web wallet for maximum security

---

## Hardware Wallets

**Best for:** Large holdings and maximum security

### Features
- ✅ Private keys never leave device
- ✅ Protection against malware
- ✅ Requires physical confirmation
- ✅ Supports multiple cryptocurrencies
- ⚠️ Costs $50-200
- ⚠️ Setup more complex

### Supported Devices

#### Ledger (Nano S, Nano X)
**Status:** Support coming Q2 2026

**How to Use:**
1. Install ËTRID app on Ledger
2. Connect to [wallet.etrid.org](https://wallet.etrid.org)
3. Select "Hardware Wallet"
4. Follow on-screen instructions
5. Confirm all transactions on device

#### Trezor (Model T)
**Status:** Support planned 2026

---

## CLI Wallet (Command Line)

**Best for:** Developers, automation, and advanced users

### Features
- ✅ Full node functionality
- ✅ Scriptable transactions
- ✅ Direct RPC access
- ✅ Maximum control
- ⚠️ Requires technical knowledge
- ⚠️ Terminal-based interface

### Installation

```bash
# Download latest release
wget https://github.com/etrid/etrid/releases/download/v1.0.0/etrid-cli-linux-x64.tar.gz

# Extract
tar -xzf etrid-cli-linux-x64.tar.gz

# Install
sudo mv etrid-cli /usr/local/bin/

# Verify
etrid-cli --version
```

### Basic Commands

```bash
# Create new account
etrid-cli account new

# List accounts
etrid-cli account list

# Check balance
etrid-cli balance <ADDRESS>

# Send transaction
etrid-cli transfer --to <ADDRESS> --amount 10 --from <YOUR_ADDRESS>

# Stake
etrid-cli staking nominate --validators <VALIDATOR_ADDRESS>

# Query blockchain
etrid-cli query <MODULE> <FUNCTION>
```

See [Developer Guide](DEVELOPER_GUIDE.md) for full CLI documentation.

---

## Multi-Signature Wallets

**Best for:** Joint accounts, DAOs, corporate treasuries

### Features
- ✅ Requires multiple approvals
- ✅ Enhanced security
- ✅ Shared control
- ✅ Customizable threshold (2-of-3, 3-of-5, etc.)

### Creating Multi-Sig

1. Open [wallet.etrid.org](https://wallet.etrid.org)
2. Navigate to **"Accounts"** → **"Multi-Sig"**
3. Click **"Create Multi-Sig"**
4. Add signatories (2-20 addresses)
5. Set threshold (e.g., 2-of-3)
6. Create account

### Using Multi-Sig

**Initiating Transaction:**
1. One signer creates transaction
2. Status: "Pending (1/2 approvals)"
3. Other signers approve via wallet
4. After threshold reached, transaction executes

**Example Configurations:**
- **2-of-3:** Personal backup (you + 2 trusted people)
- **3-of-5:** Business account (majority approval)
- **5-of-7:** DAO treasury (decentralized control)

---

## Wallet Security

### The Golden Rules

1. **NEVER share your recovery phrase** with anyone
2. **NEVER enter recovery phrase** on websites (except official wallet restore)
3. **ALWAYS verify URLs** before entering sensitive info
4. **ALWAYS use strong passwords**
5. **CONSIDER hardware wallets** for large amounts (>$10,000)

### Recovery Phrase Security

#### ✅ DO:
- Write on paper with pen/pencil
- Store in fireproof safe or safety deposit box
- Make multiple copies in different locations
- Use metal backup solutions (fire/water resistant)
- Tell trusted person WHERE it's stored (not the phrase itself)
- Consider Shamir's Secret Sharing for high-value accounts

#### ❌ DON'T:
- Take screenshots or photos
- Store in cloud services (Google Drive, Dropbox, iCloud)
- Send via email, text, or messaging apps
- Store in password managers (debatable)
- Write on devices connected to internet
- Laminate paper (prevents emergency destruction)

### Password Security

**Best Practices:**
1. Use password manager (1Password, Bitwarden, KeePass)
2. Enable 2FA on password manager
3. Use unique passwords for each account
4. Make passwords long (12+ characters)

**Good Password Examples:**
- Passphrase: `CorrectHorseBatteryStaple` (easy to remember)
- Random: `xK9#mP2$vL8@nQ5!` (use password manager)

### Recognizing Phishing

**Common Phishing Tactics:**

1. **Fake Websites**
   - ❌ `wallet-etrid.org` (dash instead of dot)
   - ❌ `etrid-wallet.com` (wrong domain)
   - ✅ `wallet.etrid.org` (correct official URL)

2. **Recovery Phrase Requests**
   - ❌ "Support" asking for recovery phrase
   - ❌ Emails requesting phrase verification
   - ❌ Pop-ups requesting phrase entry

3. **Urgency Tactics**
   - "Your account will be closed unless you act now"
   - "Verify wallet within 24 hours"
   - "Claim airdrop by entering recovery phrase"

**Protection:**
- Bookmark official URLs
- Verify on official social media
- ËTRID support will NEVER ask for recovery phrase
- No legitimate "giveaways" require private keys

---

## Wallet Comparison

| Feature | Web | Extension | Mobile | Hardware | CLI |
|---------|-----|-----------|--------|----------|-----|
| **Ease of Use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐ |
| **Security** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **DApp Support** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐ |
| **Portability** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐ |
| **Offline Use** | ❌ | ❌ | ⚠️ Limited | ✅ | ✅ |
| **Cost** | Free | Free | Free | $50-200 | Free |

---

## Importing/Exporting Accounts

### Export Account

**Use Case:** Move account to another device/wallet

1. Open wallet settings
2. Click **"Export Account"**
3. Enter password
4. Choose format:
   - **Recovery Phrase:** 12 words (universal)
   - **JSON Keyfile:** Encrypted file (requires password)
5. Save securely (never screenshot!)

### Import Account

**Method 1: Recovery Phrase**
1. Click **"Import Account"**
2. Select **"From Seed Phrase"**
3. Enter 12-word recovery phrase
4. Set new password
5. Account restored with all funds

**Method 2: JSON Keyfile**
1. Click **"Import Account"**
2. Select **"From JSON File"**
3. Upload JSON file
4. Enter file password
5. Account imported

---

## Wallet Recovery

### Lost Password (Have Recovery Phrase)

✅ **You can recover:**
1. Click **"Restore Account"**
2. Enter 12-word recovery phrase
3. Set new password
4. Full access restored

### Lost Recovery Phrase

❌ **Cannot recover:**
- Without recovery phrase, account is permanently inaccessible
- No one can reset or recover it (not even ËTRID team)
- This is why backup is CRITICAL

### Compromised Account

If you suspect compromise:
1. **Immediately transfer funds** to new account
2. **Create new recovery phrase** (new account)
3. **Report incident** to ËTRID team
4. **Analyze what happened** to prevent future compromise

---

## Wallet Recommendations by Use Case

### 💡 Beginner (Just Starting)
**Recommended:** Web Wallet
- Easiest to use
- No installation required
- Learn basics risk-free on testnet

### 💼 Daily User (Active Trading/Staking)
**Recommended:** Browser Extension
- Quick access
- DApp integration
- Secure enough for moderate amounts

### 📱 Mobile User (On-the-Go)
**Recommended:** Mobile Wallet (when available)
- Currently: Polkadot Vault + Web Wallet
- Future: ËTRID Mobile App (Q1 2026)

### 🏦 Large Holdings (>$10,000)
**Recommended:** Hardware Wallet
- Maximum security
- Worth the investment
- Peace of mind

### 👨‍💻 Developer (Building on ËTRID)
**Recommended:** CLI Wallet
- Full control
- Scriptable
- Direct RPC access

### 🏢 Business/DAO (Shared Control)
**Recommended:** Multi-Signature Wallet
- Requires multiple approvals
- Prevents single point of failure
- Audit trail

---

## Need Help?

**Resources:**
- 📖 [User Guide](USER_GUIDE.md) - Comprehensive wallet guide
- 🎥 [Video Tutorials](https://youtube.com/etrid) - Visual walkthroughs
- 💬 [Discord Community](https://discord.gg/etrid) - Ask questions

**Support:**
- Email: wallet-support@etrid.org
- Include: wallet type, issue description, screenshots (never recovery phrase!)

---

**Ready to get started?** [Create Wallet →](https://wallet.etrid.org)
