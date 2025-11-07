# Password Manager Setup Guide for Etrid Secrets

**Date**: November 7, 2025
**Purpose**: Securely manage all cryptographic keys, passwords, and sensitive data for Etrid

---

## 🔐 Recommended Password Managers

### Top Choice: **Bitwarden** (Best for Crypto/Development)

**Why Bitwarden:**
- ✅ **Open source** - You can audit the code
- ✅ **Self-hosting option** - Full control of your data
- ✅ **Affordable** - Free for personal, $10/year for premium
- ✅ **Secure Notes** - Store private keys as secure notes
- ✅ **Attachments** - Store JSON files (keypairs.json, etc.)
- ✅ **CLI tool** - Integrate with deployment scripts
- ✅ **Multi-platform** - Desktop, mobile, browser extension
- ✅ **2FA support** - Protect with hardware key (YubiKey)
- ✅ **Audited** - Regular third-party security audits

**Pricing:**
- Free: Unlimited passwords, 2 devices
- Premium: $10/year - Attachments, emergency access, 2FA
- Families: $40/year - 6 users

**Get it:** https://bitwarden.com

---

### Alternative 1: **1Password** (User-Friendly)

**Why 1Password:**
- ✅ **Very polished UI** - Easiest to use
- ✅ **Secret References** - Link secrets across entries
- ✅ **Developer-friendly** - CLI and SSH agent integration
- ✅ **Watchtower** - Alerts for compromised passwords
- ✅ **Travel Mode** - Hide vaults when crossing borders

**Cons:**
- ❌ Not open source
- ❌ More expensive ($36-48/year)

**Pricing:**
- Individual: $3/month ($36/year)
- Families: $5/month (5 users)

**Get it:** https://1password.com

---

### Alternative 2: **KeePassXC** (Maximum Security)

**Why KeePassXC:**
- ✅ **Completely offline** - No cloud, 100% local
- ✅ **Open source** - Free forever
- ✅ **No subscription** - One-time download
- ✅ **Browser integration** - Via plugin
- ✅ **Hardware key support** - YubiKey, OnlyKey

**Cons:**
- ❌ Manual sync required (Dropbox, Google Drive, etc.)
- ❌ No official mobile app (use KeePass2Android, Strongbox)
- ❌ Less user-friendly interface

**Pricing:**
- Free forever

**Get it:** https://keepassxc.org

---

## 🗂️ What to Store in Password Manager

### Category 1: Blockchain Deployment Keys

**Create vault/folder: "Etrid - Deployment Keys"**

#### For Each Chain:
```
Title: Etrid Deployment - [Chain Name]
Username: 0x... (public address)
Password: 0x... (private key)

Notes:
Chain: [Ethereum/BSC/Polygon/Arbitrum/Base]
Deployed Contracts: [list addresses]
Explorer: [blockchain explorer URL]
Generated: [date]
Last Used: [date]

Attachments:
- deployment-receipt.json
- contract-addresses.txt
```

**Entries needed:**
1. Etrid Deployment - Ethereum
2. Etrid Deployment - BSC
3. Etrid Deployment - Polygon
4. Etrid Deployment - Arbitrum
5. Etrid Deployment - Base
6. Etrid Deployment - Solana
7. Etrid Deployment - Unified Contracts

---

### Category 2: Validator Keys

**Create vault/folder: "Etrid - Validators"**

#### For Each Validator:
```
Title: Etrid Validator [Number] - Session Keys
Username: [Validator SS58 Address]

Password/Keys stored in secure note:
{
  "validator_id": "validator_01",
  "stash_account": "5...",
  "controller_account": "5...",
  "session_keys": {
    "grandpa": "0x...",
    "babe": "0x...",
    "im_online": "0x...",
    "authority_discovery": "0x..."
  },
  "payment_account": "5..."
}

Attachments:
- validator-keys-complete.json (from secrets/mainnet/)
```

---

### Category 3: AI Agent DID Keys

**Create vault/folder: "Etrid - AI Agents"**

```
Title: AI Agent DID Keypairs
Username: [Primary AI Agent DID]

Secure Note contains:
- Full keypairs.json content (from secrets/aidevs-keys/)
- Public keys only (for reference)

Attachments:
- keypairs.json
- public_keys.json
- ai-monitoring-keypairs.json
```

---

### Category 4: Infrastructure Credentials

**Create vault/folder: "Etrid - Infrastructure"**

#### Oracle Cloud
```
Title: Oracle Cloud - Validators
Username: [OCI username/email]
Password: [password]

Notes:
User OCID: ocid1.user...
Tenancy OCID: ocid1.tenancy...
Region: us-ashburn-1
Compartment: etrid-validators

Attachments:
- oci_api_key.pem
- oci_api_key_fingerprint.txt
```

#### Azure VMs
```
Title: Azure - Validator VMs
Username: [Azure username]
Password: [password]

Notes:
Subscription ID: ...
Resource Group: etrid-validators
VM IPs: [list]

Attachments:
- azure-credentials.json
```

#### RPC Providers
```
Title: Alchemy API Keys
Username: [email]
Password: [password]

API Keys:
- Ethereum: alcht_...
- Polygon: alcht_...
- Arbitrum: alcht_...
```

---

### Category 5: DEX & Exchange Accounts

**Create vault/folder: "Etrid - DEX/CEX"**

```
Title: PancakeSwap Liquidity Pool
Notes:
Pool Address: 0x...
LP Token Address: 0x...
Initial Liquidity: 100K ETR + 0.05 BNB
Created: [date]

---

Title: Raydium Pool (Solana)
Notes:
Pool ID: ...
Token: 8XdUXcvWUYnyKg6hR5yEDFHJqhqD2CbizLURVQCXNppq
AMM: Raydium CLMM
```

---

### Category 6: GitHub & Repository Access

**Create vault/folder: "Etrid - Development"**

```
Title: GitHub - Etrid Main Repo
Username: EojEdred
Password: [GitHub PAT - Personal Access Token]

Notes:
Repo: https://github.com/EojEdred/Etrid
Token Scopes: repo, workflow, write:packages
Expires: [date]

---

Title: NPM Publishing Token
Username: [npm username]
Password: [npm token]
```

---

## 📋 Importing Existing Secrets

### Step 1: Create Master Inventory

Run this to create an inventory of all secrets:

```bash
cd /Users/macbook/Desktop/etrid

cat > secrets-inventory.txt <<'EOF'
# Etrid Secrets Inventory
# Generated: $(date)

## Mainnet Genesis & Validator Keys
EOF

ls -lh secrets/mainnet/ >> secrets-inventory.txt

cat >> secrets-inventory.txt <<'EOF'

## AI Agent Keys
EOF

ls -lh secrets/aidevs-keys/ >> secrets-inventory.txt

cat >> secrets-inventory.txt <<'EOF'

## Validator Generated Keys
EOF

find secrets/validator-keys/generated-keys/ -name "*.json" >> secrets-inventory.txt

cat >> secrets-inventory.txt <<'EOF'

## Genesis Accounts
EOF

find secrets/genesis-accounts/ -name "*.json" | head -20 >> secrets-inventory.txt

echo "Inventory created: secrets-inventory.txt"
```

### Step 2: Extract Private Keys (for import)

```bash
# Create a temporary import file (DELETE after importing!)
cat > /tmp/keys-to-import.txt <<'EOF'
=== DEPLOYMENT KEYS ===

BSC Deployment Key:
PRIVATE_KEY=0x1b4734300c70328ac73f7b7bda27fca85c11ec6cebfd56fb77f147cad5d3faed
ADDRESS=0x0eD1324C22b05B94FcFD5f76c1b3Ba8aD7df05Ed

Polygon Deployment Key:
(Same as BSC - NEED TO ROTATE!)

Solana Deployment:
WALLET=482aYVUgiqFF7Dtvw3nUy5cW6fbo4P1nZya8yibFAcGr
(Private key in Phantom wallet)

EOF

# Add validator keys
echo "=== VALIDATOR KEYS ===" >> /tmp/keys-to-import.txt
cat secrets/mainnet/validator-keys-complete.json | jq -r '.validators[] | "Validator \(.id): \(.stash_account)"' >> /tmp/keys-to-import.txt 2>/dev/null || echo "validator-keys-complete.json not found or invalid JSON"

# Add AI agent keys
echo -e "\n=== AI AGENT KEYS ===" >> /tmp/keys-to-import.txt
cat secrets/aidevs-keys/keypairs.json 2>/dev/null || echo "keypairs.json not found"

echo "Import file created: /tmp/keys-to-import.txt"
echo "⚠️  REMEMBER TO DELETE THIS FILE AFTER IMPORTING!"
```

### Step 3: Import to Password Manager

#### Using Bitwarden:

1. **Download Bitwarden CLI:**
   ```bash
   brew install bitwarden-cli
   ```

2. **Login:**
   ```bash
   bw login
   ```

3. **Create folder:**
   ```bash
   bw get template folder | jq '.name = "Etrid - Deployment Keys"' | bw encode | bw create folder
   ```

4. **Create items:**
   ```bash
   # Create a secure note template
   bw get template item | jq '.name = "Etrid BSC Deployment" | .type = 2 | .secureNote.type = 0 | .notes = "PRIVATE_KEY=0x1b47...faed\nADDRESS=0x0eD1..."' | bw encode | bw create item
   ```

5. **Sync:**
   ```bash
   bw sync
   ```

#### Using 1Password CLI:

```bash
# Install
brew install 1password-cli

# Sign in
op signin

# Create vault
op vault create "Etrid Secrets"

# Create item
op item create --category="Secure Note" \
  --title="Etrid BSC Deployment" \
  --vault="Etrid Secrets" \
  "PRIVATE_KEY=0x1b47...faed" \
  "ADDRESS=0x0eD1..."
```

#### Manual Import (Easiest):

1. Open Bitwarden/1Password desktop app
2. Create new folder/vault: "Etrid Secrets"
3. For each key, create "Secure Note" item:
   - Title: "Etrid [Chain] Deployment"
   - Copy-paste from `/tmp/keys-to-import.txt`
4. Attach JSON files as attachments

### Step 4: Verify and Cleanup

```bash
# Verify you can retrieve a key
bw list items --search "Etrid BSC" | jq '.[0].notes'

# Once verified, SECURELY DELETE the temp file
shred -u /tmp/keys-to-import.txt  # Linux
srm /tmp/keys-to-import.txt       # macOS (install with: brew install srm)
# Or manually:
rm -P /tmp/keys-to-import.txt      # macOS built-in
```

---

## 🔒 Security Best Practices

### 1. Enable 2FA on Password Manager

**Recommended: Hardware Key (YubiKey)**
- Buy: https://www.yubico.com ($25-45)
- Setup in Bitwarden: Settings → Security → Two-step Login → YubiKey

**Alternative: Authenticator App**
- Use Authy or Google Authenticator
- Backup codes stored in different location

### 2. Use Strong Master Password

**Requirements:**
- 20+ characters
- Mix of words, numbers, symbols
- NOT related to Etrid or crypto
- Memorized, never written down

**Example (don't use this):**
`correct-horse-battery-staple-8472-!purple`

**Generate one:**
```bash
# Random passphrase
openssl rand -base64 24

# Diceware (more memorable)
# Use: https://diceware.dmuth.org/
```

### 3. Backup Recovery Codes

**Important:**
- Download recovery codes from password manager
- Print on paper
- Store in physical safe or safety deposit box
- NEVER store recovery codes in the password manager itself

### 4. Regular Security Review

**Monthly:**
- [ ] Review all stored credentials
- [ ] Remove unused keys
- [ ] Update "Last Used" dates
- [ ] Check for compromised passwords (Watchtower in 1Password)

**Quarterly:**
- [ ] Rotate deployment keys
- [ ] Audit access logs
- [ ] Update emergency contacts

---

## 🚨 Emergency Access

### Setup Emergency Contact

**Bitwarden:**
Settings → Emergency Access → Add Emergency Contact

**1Password:**
Settings → Family/Team → Emergency Kit

**Who to add:**
- Trusted family member or business partner
- Lawyer or executor (for estate planning)
- Emergency wait time: 30-90 days

---

## 📱 Mobile Setup

### iOS/Android

1. **Install app:**
   - Bitwarden: https://apps.apple.com/app/bitwarden/id1137397744
   - 1Password: https://apps.apple.com/app/1password/id568903335

2. **Enable biometric unlock:**
   - Face ID / Touch ID
   - Still requires master password periodically

3. **Auto-fill setup:**
   - iOS: Settings → Passwords → AutoFill Passwords → Bitwarden
   - Android: Settings → System → Languages & input → Autofill service → Bitwarden

---

## 🔄 Integration with Development Workflow

### CLI Access During Deployment

```bash
# Install Bitwarden CLI
brew install bitwarden-cli

# Login
bw login

# Get a specific key
export PRIVATE_KEY=$(bw get notes "Etrid BSC Deployment" | grep PRIVATE_KEY | cut -d= -f2)

# Use in deployment script
echo "PRIVATE_KEY=$PRIVATE_KEY" > /tmp/.env
npm run deploy:bsc
rm /tmp/.env
```

### Secure Deployment Script

```bash
#!/bin/bash
# deploy-with-bitwarden.sh

# Ensure logged in
bw login --check || { echo "Please run: bw login"; exit 1; }

# Get the key
PRIVATE_KEY=$(bw get notes "Etrid BSC Deployment" | grep PRIVATE_KEY | cut -d= -f2)

# Temporary .env file
cat > /tmp/.env.deploy <<EOF
PRIVATE_KEY=$PRIVATE_KEY
BSC_RPC=https://bsc-dataseed.bnbchain.org
EOF

# Deploy
npm run deploy:bsc --env-file=/tmp/.env.deploy

# Cleanup
shred -u /tmp/.env.deploy
```

---

## 📦 Files to Attach in Password Manager

For each secret entry, attach the corresponding file:

### Deployment Keys Entry:
- `ethereum-key.txt`
- `deployment-receipt.json`
- `contract-addresses.json`

### Validator Entry:
- `secrets/mainnet/validator-keys-complete.json`
- `secrets/mainnet/flarechain_mainnet_genesis.json`

### AI Agents Entry:
- `secrets/aidevs-keys/keypairs.json`
- `secrets/aidevs-keys/public_keys.json`

### Oracle Cloud Entry:
- `.oci/oci_api_key.pem`
- `oci-config.txt`

---

## ✅ Migration Checklist

Complete this checklist when setting up your password manager:

- [ ] Choose password manager (Bitwarden recommended)
- [ ] Create account and set strong master password
- [ ] Enable 2FA (YubiKey or authenticator app)
- [ ] Download recovery codes and store safely
- [ ] Create folder structure (7 categories)
- [ ] Import deployment keys (5 chains + unified)
- [ ] Import validator keys (20 validators)
- [ ] Import AI agent DID keypairs
- [ ] Import infrastructure credentials (OCI, Azure)
- [ ] Attach JSON files to relevant entries
- [ ] Test retrieval of at least 3 keys
- [ ] Install mobile app and enable biometric unlock
- [ ] Setup emergency access contact
- [ ] Delete all temporary import files securely
- [ ] Document password manager in team runbook
- [ ] Schedule first security review (1 month)

---

## 🎯 Quick Start (10 Minutes)

**Fastest way to get started:**

1. **Sign up for Bitwarden** (2 min)
   - Go to https://vault.bitwarden.com/#/register
   - Create account with strong master password

2. **Install desktop app** (1 min)
   - macOS: `brew install --cask bitwarden`

3. **Create first entries** (5 min)
   - New Item → Secure Note
   - Title: "Etrid BSC Deployment"
   - Paste from `/tmp/keys-to-import.txt`
   - Save

4. **Enable 2FA** (2 min)
   - Settings → Security → Two-step Login
   - Choose Authenticator App
   - Scan QR code with phone

**Done!** Your first secrets are secured.

---

## 💡 Pro Tips

1. **Use separate entries for testnet vs mainnet keys**
2. **Tag entries**: `#mainnet`, `#testnet`, `#deprecated`
3. **Add custom fields**: `Chain ID`, `Block Explorer`, `Contract Address`
4. **Set expiration dates** for temporary keys
5. **Use password generator** for API keys (even if regenerating existing)
6. **Never email or Slack private keys** - share password manager entry instead
7. **Audit trail**: Note when/why keys were used in Notes field

---

## 📞 Support

**Bitwarden:**
- Docs: https://bitwarden.com/help/
- Community: https://community.bitwarden.com/
- Email: hello@bitwarden.com

**1Password:**
- Support: https://support.1password.com/
- Email: support@1password.com

**KeePassXC:**
- Docs: https://keepassxc.org/docs/
- GitHub: https://github.com/keepassxc/keepassxc

---

**Remember**: Your password manager is the single most important security tool you'll use. Take time to set it up properly!
