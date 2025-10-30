# Git Security Guide - What NOT to Upload

**Date:** October 29, 2025
**CRITICAL:** This guide explains which files contain sensitive information and must NEVER be committed to git

---

## 🚨 CRITICAL - DO NOT COMMIT THESE

### 1. **validator-keys-setup/generated-keys/** ⚠️ EXTREMELY SENSITIVE

**Contains:**
- ✅ **PROTECTED** - Added to .gitignore
- 82 UNENCRYPTED private keys
- Sudo keys (control over entire blockchain)
- Payment account keys (receive validator rewards)
- Session keys (AURA, GRANDPA, ASF)
- Network keys (P2P identity)

**Files inside:**
```
validator-keys-complete.json          🚨 UNENCRYPTED PRIVATE KEYS
validator-keys-complete.json.enc      🔒 Encrypted (still don't upload)
sudo-key.json                         🚨 SUDO KEY - CONTROLS BLOCKCHAIN
sudo-key.json.enc                     🔒 Encrypted sudo key
bootnode-info.txt                     ⚠️  Contains network topology
ssh-keys-backup.tar.gz.enc            🔒 SSH keys backup
```

**Impact if leaked:**
- ❌ Attacker can impersonate validators
- ❌ Attacker can steal validator rewards
- ❌ Attacker can control blockchain via sudo key
- ❌ Complete network compromise

**Status:** ✅ **PROTECTED by .gitignore**

---

### 2. **validator-inventory.txt** ⚠️ OPERATIONAL SECURITY

**Contains:**
- IP addresses of all 21 validators
- Server names and locations
- Provider information
- Network topology

**Impact if leaked:**
- ❌ Attackers know exactly where to target
- ❌ DDoS attack surface exposed
- ❌ Physical location information revealed

**Status:** ✅ **PROTECTED by .gitignore**

---

### 3. **API Tokens and Credentials** ⚠️ INFRASTRUCTURE ACCESS

**Patterns protected:**
```
*-token.txt
*-credentials.json
*-secrets.json
.env (without .example)
```

**Contains:**
- Hetzner API tokens
- Vultr API keys
- DigitalOcean tokens
- Backblaze B2 credentials
- SSH private keys

**Impact if leaked:**
- ❌ Attacker can create/destroy VMs
- ❌ Attacker can access all servers
- ❌ Attacker can delete backups
- ❌ Infrastructure takeover

**Status:** ✅ **PROTECTED by .gitignore**

---

## ✅ SAFE TO COMMIT

### 1. **validator-keys-setup/docs/** ✅ DOCUMENTATION ONLY

**Safe files:**
- START_HERE_VALIDATOR_DEPLOYMENT.md
- VALIDATOR_KEYS_GENERATED_SUMMARY.md (doesn't contain actual keys)
- 21_VALIDATORS_COMPLETE_DEPLOYMENT_PLAN.md
- VALIDATOR_QUICKSTART.md
- All other .md documentation files

**Why safe:**
- No actual private keys
- General deployment instructions
- Architecture explanations
- Best practices

**Recommendation:** ✅ Safe to commit if you want to share deployment guides

---

### 2. **validator-keys-setup/scripts/** ✅ TOOLS ONLY

**Safe files:**
- generate-validators-gizzi-eoj-bootstrap.sh
- bootstrap-validator.sh
- start-validator.sh
- All other .sh script files

**Why safe:**
- Scripts to GENERATE keys (not the keys themselves)
- Startup scripts
- No hardcoded credentials
- Reusable tools

**Recommendation:** ✅ Safe to commit (helps others set up validators)

---

### 3. **validator-deployment-kit/** ✅ INFRASTRUCTURE AUTOMATION

**Safe files:**
- All documentation in docs/
- All scripts in scripts/
- README.md

**Why safe:**
- No actual keys or credentials
- Infrastructure automation scripts
- Provider comparison documentation
- Storage management tools

**Recommendation:** ✅ Safe to commit (valuable for community)

**Note:** Generated `validator-inventory.txt` files will be ignored

---

## 🔒 Current .gitignore Protection

### Protected Patterns

```gitignore
# Generated validator keys
validator-keys-setup/generated-keys/
**/generated-keys*/
**/generated-keys-*/

# Key files
*-keys-complete.json
*-keys-*.json
validator-keys*.json
sudo-key*.json

# Encrypted files
*.enc
*.gpg
*.asc
*.key
*.pem

# Backups with keys
*-backup*.txt
*-backup*.json
ssh-keys-backup*

# Inventory files
validator-inventory.txt
*-inventory.txt

# Credentials
*-token.txt
*-credentials.json
*-secrets.json
.env
```

---

## ✅ Verification Checklist

Before committing, run these checks:

### 1. Check what would be committed:
```bash
git status
```

**Safe output:**
```
On branch main
Untracked files:
  validator-keys-setup/docs/
  validator-keys-setup/scripts/
  validator-keys-setup/README.md
  validator-deployment-kit/
```

**UNSAFE output (DO NOT COMMIT):**
```
Untracked files:
  validator-keys-setup/generated-keys/  ❌ STOP!
  validator-keys-complete.json          ❌ STOP!
  sudo-key.json                         ❌ STOP!
  validator-inventory.txt               ❌ STOP!
```

### 2. Test .gitignore rules:
```bash
# Test if sensitive file is ignored (should return match)
git check-ignore -v validator-keys-setup/generated-keys/validator-keys-complete.json

# Expected output:
# .gitignore:66:**/generated-keys*/  validator-keys-setup/generated-keys/...
```

### 3. Check for accidentally committed secrets:
```bash
# Search staged files for potential secrets
git diff --cached | grep -i "private\|secret\|password\|token\|key"

# Should return nothing or only documentation references
```

### 4. Verify generated-keys is excluded:
```bash
git ls-files validator-keys-setup/generated-keys/

# Should return: (empty - nothing staged)
```

---

## 🚨 If You Accidentally Committed Secrets

### Immediate Actions:

**1. DO NOT PUSH to remote repository**
```bash
# If you haven't pushed yet
git reset --soft HEAD~1
```

**2. If already pushed to private repo:**
```bash
# Rotate ALL keys immediately
cd validator-keys-setup/scripts
./generate-validators-gizzi-eoj-bootstrap.sh

# This generates new keys
```

**3. If pushed to PUBLIC repo:**
```bash
# EMERGENCY - Contact team immediately
# All keys are compromised
# Must rotate everything:
# - Validator session keys
# - Payment accounts
# - Sudo keys
# - SSH keys
# - API tokens
```

**4. Remove from git history:**
```bash
# Use git-filter-repo or BFG Repo-Cleaner
# This is advanced - seek help if needed
git filter-repo --path validator-keys-setup/generated-keys --invert-paths
```

**5. Rotate compromised credentials:**
- Regenerate all validator keys
- Create new API tokens
- Change SSH keys on all VMs
- Update sudo multisig
- Redeploy all validators with new keys

---

## 📋 Safe Git Workflow

### Recommended Approach:

**Option 1: Commit only docs and scripts (RECOMMENDED)**
```bash
# Add only safe directories
git add validator-keys-setup/docs/
git add validator-keys-setup/scripts/
git add validator-keys-setup/README.md
git add validator-deployment-kit/

# Verify nothing sensitive
git status

# Commit
git commit -m "Add validator deployment system and documentation"
```

**Option 2: Keep everything local (SAFEST)**
```bash
# Don't commit validator folders at all
# Keep them only on your local machine
# Use encrypted backups instead

# Back up to encrypted drive
tar czf validator-keys-$(date +%Y%m%d).tar.gz validator-keys-setup/
gpg -c validator-keys-$(date +%Y%m%d).tar.gz
rm validator-keys-$(date +%Y%m%d).tar.gz

# Store encrypted file safely offline
```

**Option 3: Private repo with restricted access**
```bash
# If using GitHub private repo
# Still exclude generated-keys/ (defense in depth)
# Limit repo access to only trusted team members

git remote add origin git@github.com:private-org/etrid-validators.git
git push -u origin main
```

---

## 🔐 Security Best Practices

### 1. Key Storage

**DO:**
- ✅ Keep keys encrypted at rest
- ✅ Store backups offline (USB drive, hardware security module)
- ✅ Use strong passphrases for GPG encryption
- ✅ Limit file permissions: `chmod 600 validator-keys-complete.json`
- ✅ Store in encrypted home directory or encrypted volume

**DON'T:**
- ❌ Upload to cloud storage (Dropbox, Google Drive, etc.)
- ❌ Email keys
- ❌ Commit to git (even private repos)
- ❌ Share via Slack/Discord/messaging apps
- ❌ Screenshot keys
- ❌ Print keys

### 2. Access Control

**DO:**
- ✅ Use separate keys per validator (we already do this)
- ✅ Use 2-of-2 multisig for sudo (we already do this)
- ✅ Encrypt SSH keys
- ✅ Use SSH key passphrases
- ✅ Rotate keys periodically (annually)

**DON'T:**
- ❌ Share root SSH access
- ❌ Use the same key across multiple validators
- ❌ Store keys on validators themselves (only session keys)

### 3. Backup Strategy

**DO:**
- ✅ Create encrypted backups: `validator-keys-complete.json.enc`
- ✅ Store backups in multiple secure locations
- ✅ Test backup recovery procedures
- ✅ Document backup locations securely
- ✅ Use hardware security modules for sudo keys

**DON'T:**
- ❌ Rely on single backup location
- ❌ Store backups in same location as originals
- ❌ Forget backup passwords/passphrases

---

## 📊 Sensitivity Matrix

| File/Folder | Sensitivity | .gitignore | Safe to Commit? |
|-------------|-------------|-----------|-----------------|
| **validator-keys-setup/generated-keys/** | 🔴 CRITICAL | ✅ Yes | ❌ NEVER |
| validator-keys-complete.json | 🔴 CRITICAL | ✅ Yes | ❌ NEVER |
| sudo-key.json | 🔴 CRITICAL | ✅ Yes | ❌ NEVER |
| *.enc files | 🟡 HIGH | ✅ Yes | ⚠️  Not recommended |
| validator-inventory.txt | 🟡 HIGH | ✅ Yes | ⚠️  Not recommended |
| API tokens | 🔴 CRITICAL | ✅ Yes | ❌ NEVER |
| validator-keys-setup/docs/ | 🟢 LOW | ❌ No | ✅ YES |
| validator-keys-setup/scripts/ | 🟢 LOW | ❌ No | ✅ YES |
| validator-deployment-kit/ | 🟢 LOW | ❌ No | ✅ YES |

---

## 🎯 Summary

### ❌ NEVER COMMIT:
1. `validator-keys-setup/generated-keys/` - Contains all private keys
2. Any `*-keys-*.json` files
3. `sudo-key.json` files
4. `validator-inventory.txt` with IPs
5. API tokens or credentials
6. SSH private keys

### ✅ SAFE TO COMMIT:
1. `validator-keys-setup/docs/` - Documentation only
2. `validator-keys-setup/scripts/` - Key generation tools
3. `validator-deployment-kit/` - Infrastructure automation
4. `README.md` files
5. Shell scripts without hardcoded credentials

### 🔒 PROTECTION STATUS:
- ✅ All sensitive patterns added to `.gitignore`
- ✅ Tested and verified working
- ✅ Multiple layers of protection
- ✅ Safe to proceed with commits (of non-sensitive files)

---

## ❓ Questions to Ask Yourself

Before every commit:

1. **Does this file contain private keys?** → Don't commit
2. **Does this file contain passwords/tokens?** → Don't commit
3. **Does this file contain IP addresses?** → Don't commit
4. **Is this file encrypted?** → Still don't commit (defense in depth)
5. **Is this just documentation or scripts?** → Safe to commit
6. **Would I be okay with this file being public?** → If no, don't commit

**When in doubt, DON'T commit. Better safe than sorry!**

---

## 📞 Emergency Contacts

If keys are compromised:

1. **Stop all validators immediately**
2. **Revoke compromised API tokens**
3. **Regenerate all keys**
4. **Update chain spec with new keys**
5. **Redeploy validators**
6. **Monitor for unauthorized access**

**Recovery time:** 2-4 hours with this kit
**All tools provided in validator-keys-setup/scripts/**

---

**Status:** ✅ Your repository is now protected against accidental key leaks!

**Next steps:**
1. Review this guide
2. Verify .gitignore is working: `git check-ignore -v validator-keys-setup/generated-keys/`
3. Commit only safe files: `git add validator-keys-setup/docs/ validator-keys-setup/scripts/`
4. Keep generated-keys/ backed up offline and encrypted
