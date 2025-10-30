# Ëtrid Validator Keys Setup & Generation

**Date:** October 29, 2025
**Status:** Complete - 82 validator keys generated
**Purpose:** Complete validator key generation and management system for Ëtrid blockchain

---

## 🚨 SECURITY WARNING - READ FIRST!

**CRITICAL:** This folder contains HIGHLY SENSITIVE cryptographic keys!

⚠️ **Before committing to git, READ:** `GIT_SECURITY_GUIDE.md`

**What's protected:**
- ❌ `generated-keys/` - Contains ALL private keys (auto-blocked by .gitignore)
- ❌ `validator-keys-complete.json` - 82 unencrypted private keys
- ❌ `sudo-key.json` - Controls entire blockchain
- ✅ `docs/` and `scripts/` - Safe to commit (no actual keys)

**All sensitive files are automatically blocked by .gitignore** ✅

---

## 📦 What's In This Folder

This folder contains all the validator key generation work, including:
- Complete validator key generation scripts
- All 82 validator keys (network, session, payment)
- Documentation for validator setup and deployment
- Bootstrap node configuration (Gizzi & EojEdred)
- Sudo key configuration (2-of-2 multisig)

---

## 📁 Folder Structure

```
validator-keys-setup/
├── README.md (this file)
├── GIT_SECURITY_GUIDE.md ⭐ IMPORTANT - Read before committing!
├── SSH_KEYS_FOR_VMS.md ⭐ SSH access keys for VMs
├── docs/ (11 documentation files)
│   ├── START_HERE_VALIDATOR_DEPLOYMENT.md ⭐ Main guide
│   ├── VALIDATOR_KEYS_GENERATED_SUMMARY.md ⭐ Key summary
│   ├── 21_VALIDATORS_COMPLETE_DEPLOYMENT_PLAN.md
│   ├── AZURE_21_VALIDATOR_DEPLOYMENT.md
│   ├── DEPLOY_VALIDATORS_NOW.md
│   ├── MAINNET_VALIDATOR_HANDOFF.md
│   ├── START_VALIDATORS_NOW.md
│   ├── UPDATED_VALIDATOR_MAPPING_GIZZI_EOJ.md
│   ├── VALIDATOR_AIDEVID_PAYMENT_INTEGRATION.md
│   ├── VALIDATOR_QUICK_REFERENCE.md
│   └── VALIDATOR_QUICKSTART.md
├── scripts/ (10 validator scripts)
│   ├── generate-validators-gizzi-eoj-bootstrap.sh ⭐ Main generator
│   ├── bootstrap-validator.sh
│   ├── generate-validators-with-payment-aidevid.sh
│   ├── one-command-validator.sh
│   ├── quick-start-21-validators.sh
│   ├── run_multi_validator_test.sh
│   ├── start-validator-alice.sh
│   ├── start-validator-bob.sh
│   ├── start-validator-optimized.sh
│   └── start-validator.sh
└── generated-keys/
    └── generated-keys-gizzi-eoj/ (82 validator keys)
        ├── validator-keys-complete.json ⭐ All keys
        ├── validator-keys-encrypted.json.gpg
        ├── sudo-key-multisig.json
        ├── sudo-backup-gizzi.txt.gpg
        ├── sudo-backup-eojedred.txt.gpg
        └── individual validator key files...
```

---

## 🔑 Generated Keys Summary

### Total Keys Generated: 82 Keys

**21 Validators (2 bootstrap + 19 standard):**

1. **Gizzi (Bootstrap 1)** - AI Overseer
2. **EojEdred (Bootstrap 2)** - Human Founder
3-21. **Standard Validators** (validator-03 through validator-21)

### Keys Per Validator (4 keys each):

1. **Network Key (Ed25519)** - P2P networking
2. **Session Keys:**
   - **AURA Key (Sr25519)** - Block production
   - **GRANDPA Key (Ed25519)** - Finalization
   - **ASF Key (Sr25519)** - Custom consensus (Attestation State Function)
3. **Payment Account (Sr25519)** - Receives staking rewards

**Total:** 21 validators × 4 keys = **84 keys**

### Additional Keys:

- **Sudo Key (2-of-2 Multisig):**
  - Gizzi's sudo key (Sr25519)
  - EojEdred's sudo key (Sr25519)
  - Combined multisig address

**Grand Total:** 82 unique keys + 1 multisig = **83 total**

---

## 🚀 Quick Start

### 1. View Generated Keys

```bash
cd generated-keys/generated-keys-gizzi-eoj

# View all keys (unencrypted - KEEP SECURE!)
cat validator-keys-complete.json | jq '.'

# View specific validator
cat validator-keys-complete.json | jq '.validators[0]'  # Gizzi
cat validator-keys-complete.json | jq '.validators[1]'  # EojEdred
```

### 2. Generate New Keys (If Needed)

```bash
cd scripts
./generate-validators-gizzi-eoj-bootstrap.sh
```

This creates:
- 21 validators with all keys
- 2 bootstrap nodes (Gizzi, EojEdred)
- Sudo 2-of-2 multisig
- Encrypted backups

### 3. Deploy Validators

See the deployment kit for infrastructure setup:
```bash
cd ../validator-deployment-kit
open README.md
```

---

## 📚 Documentation Guide

### Primary Documents

**START_HERE_VALIDATOR_DEPLOYMENT.md** ⭐
- Overview of validator architecture
- Bootstrap node explanation (Gizzi & EojEdred)
- Key types and purposes
- Security best practices

**VALIDATOR_KEYS_GENERATED_SUMMARY.md** ⭐
- Complete list of all 82 keys
- Key locations and formats
- Backup procedures
- Recovery instructions

**21_VALIDATORS_COMPLETE_DEPLOYMENT_PLAN.md**
- Detailed deployment strategy
- 21-validator network architecture
- Committee formation process
- Consensus mechanism (PPFA)

### Setup Guides

**VALIDATOR_QUICKSTART.md**
- Quick reference for starting validators
- Bootstrap process
- Common commands

**MAINNET_VALIDATOR_HANDOFF.md**
- Transitioning from testnet to mainnet
- Validator rotation procedures
- Network upgrade process

**VALIDATOR_QUICK_REFERENCE.md**
- Cheat sheet for validator operations
- Key management commands
- Troubleshooting quick fixes

### Integration Documents

**VALIDATOR_AIDEVID_PAYMENT_INTEGRATION.md**
- AI dev validator payments
- Staking reward distribution
- AIDEVID integration

**UPDATED_VALIDATOR_MAPPING_GIZZI_EOJ.md**
- Validator ID to name mapping
- Bootstrap node configuration
- Network topology

### Deployment-Specific

**AZURE_21_VALIDATOR_DEPLOYMENT.md**
- Azure-specific deployment (deprecated - use multi-provider instead)
- Cost analysis for Azure
- Why Azure was replaced

**DEPLOY_VALIDATORS_NOW.md**
**START_VALIDATORS_NOW.md**
- Quick deployment instructions
- Emergency startup procedures

---

## 🛠️ Scripts Guide

### Key Generation Scripts

**generate-validators-gizzi-eoj-bootstrap.sh** ⭐ MAIN SCRIPT
- Generates all 82 validator keys
- Creates bootstrap configuration
- Sets up 2-of-2 sudo multisig
- Creates encrypted backups

**Features:**
- Generates network, session (AURA, GRANDPA, ASF), and payment keys
- Creates chain spec with genesis validators
- Encrypts sensitive keys with GPG
- Produces human-readable summaries

**Usage:**
```bash
./generate-validators-gizzi-eoj-bootstrap.sh
```

**Output:**
- `validator-keys-complete.json` - All keys (KEEP SECURE!)
- `validator-keys-encrypted.json.gpg` - Encrypted backup
- `sudo-key-multisig.json` - Sudo multisig configuration
- Individual validator key files

**generate-validators-with-payment-aidevid.sh**
- Alternative generator with AIDEVID integration
- Includes payment account generation
- Links validators to AI dev payment system

### Validator Startup Scripts

**bootstrap-validator.sh**
- Auto-generates all keys for a new validator
- One-command bootstrap setup
- Creates network keys, session keys
- Configures systemd service

**Usage:**
```bash
./bootstrap-validator.sh
```

**start-validator.sh**
- Generic validator startup script
- Configurable parameters
- Used for standard validators

**start-validator-alice.sh**
**start-validator-bob.sh**
- Pre-configured for Alice/Bob test validators
- Useful for local testing

**start-validator-optimized.sh**
- Performance-optimized validator configuration
- Production-ready settings

**one-command-validator.sh**
- All-in-one validator setup
- Generates keys + starts node
- Quick deployment for testing

### Testing & Deployment Scripts

**quick-start-21-validators.sh**
- Deploy all 21 validators locally (testing)
- Forms committee automatically
- Useful for integration testing

**run_multi_validator_test.sh**
- Multi-validator test suite
- Verifies committee formation
- Checks consensus

---

## 🔐 Key Security & Backup

### Security Best Practices

**Encrypted Storage:**
- ✅ All keys encrypted with GPG
- ✅ Sudo keys have separate encrypted backups
- ✅ Password-protected key files

**Access Control:**
- ✅ Keys stored in secure directory (`generated-keys/`)
- ✅ Limited file permissions (600)
- ✅ Never commit unencrypted keys to git

**Backup Strategy:**
- ✅ Encrypted backups: `validator-keys-encrypted.json.gpg`
- ✅ Sudo key backups: `sudo-backup-gizzi.txt.gpg`, `sudo-backup-eojedred.txt.gpg`
- ✅ Store backups in multiple secure locations
- ✅ Test recovery procedures regularly

### Key Locations

**Primary Keys File:**
```
generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json
```

**Encrypted Backup:**
```
generated-keys/generated-keys-gizzi-eoj/validator-keys-encrypted.json.gpg
```

**Sudo Keys:**
```
generated-keys/generated-keys-gizzi-eoj/sudo-key-multisig.json
generated-keys/generated-keys-gizzi-eoj/sudo-backup-gizzi.txt.gpg
generated-keys/generated-keys-gizzi-eoj/sudo-backup-eojedred.txt.gpg
```

### Decrypting Keys

```bash
# Decrypt main backup
gpg -d validator-keys-encrypted.json.gpg > validator-keys-complete.json

# Decrypt sudo backups
gpg -d sudo-backup-gizzi.txt.gpg
gpg -d sudo-backup-eojedred.txt.gpg
```

---

## 🏗️ Validator Architecture

### Network Topology

```
Bootstrap Layer (2 nodes):
┌─────────────────────────────────────────┐
│  Gizzi (validator-01)                   │
│  - AI Overseer                          │
│  - Bootstrap 1                          │
│  - Sudo key holder (1 of 2)             │
└─────────────────────────────────────────┘
           ↓
┌─────────────────────────────────────────┐
│  EojEdred (validator-02)                │
│  - Human Founder                        │
│  - Bootstrap 2                          │
│  - Sudo key holder (2 of 2)             │
└─────────────────────────────────────────┘
           ↓
Standard Validators (19 nodes):
┌─────────────────────────────────────────┐
│  validator-03 through validator-21      │
│  - Standard validators                  │
│  - Connect to bootstrap nodes           │
│  - Participate in consensus             │
└─────────────────────────────────────────┘
```

### Key Types Explained

**Network Key (Ed25519):**
- Purpose: P2P networking, peer discovery
- Used for: Node identity, libp2p connections
- Format: `12D3KooW...` (multiaddr format)

**Session Keys:**

1. **AURA Key (Sr25519):**
   - Purpose: Block production (Authority Round)
   - Used for: Proposing blocks in assigned slots
   - Critical for: Validator rewards

2. **GRANDPA Key (Ed25519):**
   - Purpose: Block finalization
   - Used for: GRANDPA finality gadget voting
   - Critical for: Chain finality

3. **ASF Key (Sr25519):**
   - Purpose: Attestation State Function (custom)
   - Used for: Ëtrid-specific consensus
   - Critical for: PPFA mechanism

**Payment Account (Sr25519):**
- Purpose: Receive staking rewards
- Used for: Validator earnings
- Controlled by: Validator operator

---

## 🎯 Validator Deployment Workflow

### Step 1: Keys Generated ✅ COMPLETE

All 82 keys have been generated and are stored securely in:
```
generated-keys/generated-keys-gizzi-eoj/
```

### Step 2: Infrastructure Deployment

See `../validator-deployment-kit/` for:
- Multi-provider deployment (Hetzner, Vultr, DigitalOcean, Akash)
- Automated deployment scripts
- Storage management
- Cost optimization

### Step 3: Key Distribution

For each validator:
1. Upload validator binary to VM
2. Insert session keys using `key insert` command
3. Configure payment account
4. Start validator service

### Step 4: Bootstrap Process

1. **Start Gizzi (Bootstrap 1):**
   ```bash
   ssh validator-01
   systemctl start etrid-validator
   ```

2. **Start EojEdred (Bootstrap 2):**
   ```bash
   ssh validator-02
   systemctl start etrid-validator
   ```

3. **Wait for bootstrap connection**

4. **Start remaining 19 validators:**
   ```bash
   # They will discover and connect via bootstrap nodes
   for i in {03..21}; do
     ssh validator-$i systemctl start etrid-validator
   done
   ```

5. **Verify committee formation:**
   ```bash
   curl -H "Content-Type: application/json" \
     -d '{"id":1,"jsonrpc":"2.0","method":"etrid_getCommittee"}' \
     http://validator-01:9944 | jq '.result | length'
   # Should return: 21
   ```

---

## 📊 Validator Key Mapping

| Validator ID | Name | Role | Sudo Key | Payment Account |
|--------------|------|------|----------|-----------------|
| validator-01 | Gizzi | Bootstrap 1, AI Overseer | ✅ 1 of 2 | ✅ |
| validator-02 | EojEdred | Bootstrap 2, Human Founder | ✅ 2 of 2 | ✅ |
| validator-03 | governance-dev01 | Standard | ❌ | ✅ |
| validator-04 | validator-04 | Standard | ❌ | ✅ |
| ... | ... | ... | ... | ... |
| validator-21 | validator-21 | Standard | ❌ | ✅ |

**Total:** 21 validators, 2 sudo keyholders, 21 payment accounts

---

## 🔄 Key Rotation & Recovery

### When to Rotate Keys

**Session Keys (recommended annually):**
- Generate new AURA, GRANDPA, ASF keys
- Update chain spec
- Coordinate rotation across all validators

**Network Keys (only if compromised):**
- Requires re-peering
- Update bootnode configurations

**Sudo Keys (emergency only):**
- Requires 2-of-2 multisig agreement
- Use only for critical governance

### Recovery Procedures

**Lost Payment Key:**
1. Decrypt backup: `gpg -d validator-keys-encrypted.json.gpg`
2. Extract payment seed for specific validator
3. Import to new wallet
4. Update validator configuration

**Lost Session Keys:**
1. Decrypt backup
2. Re-insert keys on validator VM
3. Restart validator
4. Verify participation in consensus

**Lost Sudo Keys:**
1. Decrypt sudo backups:
   - `gpg -d sudo-backup-gizzi.txt.gpg`
   - `gpg -d sudo-backup-eojedred.txt.gpg`
2. Import to Polkadot.js wallet
3. Reconstruct multisig address
4. Test with non-critical transaction

---

## 🆘 Troubleshooting

### "Validator not participating in consensus"

**Check:**
1. Session keys inserted correctly
2. Validator is running (`systemctl status etrid-validator`)
3. Connected to bootstrap nodes
4. Block production enabled (`--validator` flag)

**Fix:**
```bash
# Re-insert session keys
./bootstrap-validator.sh
systemctl restart etrid-validator
```

### "Committee size less than 21"

**Check:**
1. All 21 validators running
2. Bootstrap nodes (Gizzi, EojEdred) started first
3. Network connectivity between validators

**Fix:**
```bash
# Restart validators in order
# Bootstrap nodes first
ssh validator-01 systemctl restart etrid-validator
ssh validator-02 systemctl restart etrid-validator

# Wait 30 seconds

# Standard validators
for i in {03..21}; do
  ssh validator-$i systemctl restart etrid-validator
  sleep 5
done
```

### "Cannot decrypt backup"

**Check:**
1. GPG key available
2. Correct passphrase
3. File not corrupted

**Fix:**
```bash
# Test GPG
gpg --version

# Import GPG key if needed
gpg --import backup-key.asc

# Decrypt with verbose output
gpg -v -d validator-keys-encrypted.json.gpg
```

---

## 📈 Network Statistics

**Genesis Configuration:**
- 21 validators (minimum committee size)
- 2 bootstrap nodes (Gizzi, EojEdred)
- 2-of-2 sudo multisig (secure governance)
- Block time: 6 seconds
- Finality: 2-3 blocks (~12-18 seconds)

**Security:**
- Byzantine fault tolerance: Up to 6 malicious validators (< 1/3)
- Liveness: Requires 14+ validators online (> 2/3)
- Finality: Requires 14+ GRANDPA votes (> 2/3)

---

## 🔗 Related Folders

**Validator Deployment Kit:** `../validator-deployment-kit/`
- Infrastructure deployment (VMs, cloud providers)
- Storage management
- Backup automation
- Cost optimization

**Main Scripts:** `../scripts/`
- Other utility scripts
- Build scripts
- Deployment automation

---

## 📝 Summary

**This folder contains:**
✅ 82 generated validator keys (network, session, payment)
✅ 2 bootstrap nodes configured (Gizzi, EojEdred)
✅ Sudo 2-of-2 multisig setup
✅ Encrypted backups of all keys
✅ Complete documentation
✅ All generation and startup scripts

**Next steps:**
1. ✅ Keys generated (COMPLETE)
2. → Deploy infrastructure (see `validator-deployment-kit/`)
3. → Distribute keys to validators
4. → Start validators and form committee
5. → Begin block production

**Status:** Ready for deployment! 🚀

---

**For deployment, see:** `../validator-deployment-kit/README.md`
**For questions, see:** `docs/START_HERE_VALIDATOR_DEPLOYMENT.md`
