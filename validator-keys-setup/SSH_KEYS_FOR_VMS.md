# SSH Keys for Validator VMs

**Date:** October 29, 2025
**Purpose:** SSH access keys for all 21 validator virtual machines

---

## 🔑 Gizzi Validator SSH Key

**Validator:** validator-01 (Gizzi - Bootstrap 1, AI Overseer)

**Key Location:**
- Private key: `~/.ssh/gizzi-validator`
- Public key: `~/.ssh/gizzi-validator.pub`

**Public Key (Add to VM):**
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPF8VDyQrdP96bOtjN6fGRJXZGPtNgeBPYcthQ323RVC gizzi-validator-01
```

**Usage:**
```bash
# Connect to Gizzi's VM
ssh -i ~/.ssh/gizzi-validator root@<gizzi-vm-ip>

# Copy files to Gizzi's VM
scp -i ~/.ssh/gizzi-validator file.txt root@<gizzi-vm-ip>:/path/

# Add to cloud provider (Hetzner example)
hcloud ssh-key create --name gizzi-validator --public-key-from-file ~/.ssh/gizzi-validator.pub
```

---

## 📋 Generate SSH Keys for All Validators

You can generate dedicated SSH keys for each validator for better security and access control:

```bash
# Generate SSH key for EojEdred (validator-02)
ssh-keygen -t ed25519 -C "eojedred-validator-02" -f ~/.ssh/eojedred-validator -N ""

# Generate SSH key for validator-03
ssh-keygen -t ed25519 -C "validator-03" -f ~/.ssh/validator-03 -N ""

# ... and so on for validators 04-21
```

**Or use a loop:**
```bash
# Generate keys for all standard validators
for i in {3..21}; do
  VALIDATOR_NUM=$(printf "%02d" $i)
  ssh-keygen -t ed25519 \
    -C "validator-${VALIDATOR_NUM}" \
    -f ~/.ssh/validator-${VALIDATOR_NUM} \
    -N "" -q
  echo "✅ Generated SSH key for validator-${VALIDATOR_NUM}"
done
```

---

## 🔐 SSH Key Management Best Practices

### Key Types

We use **Ed25519** keys because:
- ✅ More secure than RSA at equivalent key lengths
- ✅ Faster to generate and verify
- ✅ Smaller key size (256-bit)
- ✅ Industry standard for modern SSH

### Security Practices

**DO:**
- ✅ Use separate SSH keys per validator (already doing this for Gizzi)
- ✅ Add passphrase protection for production keys
- ✅ Store private keys securely (never commit to git)
- ✅ Regularly rotate SSH keys (annually)
- ✅ Use SSH agent for key management
- ✅ Limit key permissions: `chmod 600 ~/.ssh/gizzi-validator`

**DON'T:**
- ❌ Share SSH private keys
- ❌ Reuse same key across all validators
- ❌ Commit private keys to git (already protected by .gitignore)
- ❌ Store keys on the VMs themselves
- ❌ Use weak key types (RSA < 4096 bits, DSA)

---

## 🚀 Deploy SSH Keys to Cloud Providers

### Hetzner

```bash
# Upload Gizzi's public key
hcloud ssh-key create \
  --name gizzi-validator-01 \
  --public-key-from-file ~/.ssh/gizzi-validator.pub

# Use when creating VM
hcloud server create \
  --name gizzi-bootstrap-1 \
  --type cpx31 \
  --image ubuntu-22.04 \
  --ssh-key gizzi-validator-01
```

### Vultr

```bash
# Upload key
vultr-cli ssh-key create \
  --name "gizzi-validator-01" \
  --key "$(cat ~/.ssh/gizzi-validator.pub)"

# Use when creating VM
vultr-cli instance create \
  --host "gizzi-validator-01" \
  --ssh-keys <ssh-key-id>
```

### DigitalOcean

```bash
# Upload key
doctl compute ssh-key import gizzi-validator-01 \
  --public-key-file ~/.ssh/gizzi-validator.pub

# Use when creating droplet
doctl compute droplet create gizzi-validator-01 \
  --ssh-keys <fingerprint>
```

---

## 📝 SSH Key Inventory

| Validator | VM Name | SSH Key File | Status |
|-----------|---------|--------------|--------|
| validator-01 | gizzi-bootstrap-1 | ~/.ssh/gizzi-validator | ✅ Generated |
| validator-02 | eojedred-bootstrap-2 | ~/.ssh/eojedred-validator | ⏳ To generate |
| validator-03 | governance-validator-03 | ~/.ssh/validator-03 | ⏳ To generate |
| validator-04 | validator-04 | ~/.ssh/validator-04 | ⏳ To generate |
| ... | ... | ... | ... |
| validator-21 | validator-21 | ~/.ssh/validator-21 | ⏳ To generate |

---

## 🔄 SSH Key Backup

### Backup All SSH Keys

```bash
# Create encrypted backup of all validator SSH keys
cd ~/.ssh
tar czf validator-ssh-keys-backup.tar.gz \
  gizzi-validator \
  gizzi-validator.pub \
  eojedred-validator* \
  validator-* \
  2>/dev/null || true

# Encrypt backup
gpg -c validator-ssh-keys-backup.tar.gz

# Store securely
mv validator-ssh-keys-backup.tar.gz.gpg ~/secure-backup/
rm validator-ssh-keys-backup.tar.gz

echo "✅ SSH keys backed up and encrypted"
```

### Restore from Backup

```bash
# Decrypt backup
gpg -d validator-ssh-keys-backup.tar.gz.gpg > validator-ssh-keys-backup.tar.gz

# Extract to ~/.ssh
tar xzf validator-ssh-keys-backup.tar.gz -C ~/.ssh/

# Set correct permissions
chmod 600 ~/.ssh/gizzi-validator
chmod 644 ~/.ssh/gizzi-validator.pub

echo "✅ SSH keys restored"
```

---

## 🔧 SSH Configuration

### Create SSH Config for Easy Access

Add to `~/.ssh/config`:

```ssh-config
# Gizzi Validator (Bootstrap 1)
Host gizzi
  HostName <gizzi-vm-ip>
  User root
  IdentityFile ~/.ssh/gizzi-validator
  ServerAliveInterval 60
  ServerAliveCountMax 3

# EojEdred Validator (Bootstrap 2)
Host eojedred
  HostName <eojedred-vm-ip>
  User root
  IdentityFile ~/.ssh/eojedred-validator
  ServerAliveInterval 60
  ServerAliveCountMax 3

# Standard Validators (03-21)
Host validator-03
  HostName <validator-03-ip>
  User root
  IdentityFile ~/.ssh/validator-03
  ServerAliveInterval 60

# ... repeat for validators 04-21
```

**Then connect simply:**
```bash
ssh gizzi
ssh eojedred
ssh validator-03
```

---

## 🛡️ Security: SSH Key vs Validator Keys

**IMPORTANT: These are DIFFERENT types of keys!**

### SSH Keys (This Document)
- **Purpose:** Access VMs via SSH
- **Type:** Ed25519 or RSA
- **Used for:** Remote server access, file transfers
- **Location:** `~/.ssh/gizzi-validator`
- **Added to:** VM's `~/.ssh/authorized_keys`

### Validator Keys (In generated-keys/)
- **Purpose:** Blockchain consensus and validation
- **Types:** AURA (Sr25519), GRANDPA (Ed25519), ASF (Sr25519)
- **Used for:** Block production, finalization, rewards
- **Location:** `validator-keys-setup/generated-keys/`
- **Added to:** Validator's keystore on the VM

**Both are critical but serve completely different purposes!**

---

## ⚡ Quick Commands

### Check SSH Key

```bash
# View public key
cat ~/.ssh/gizzi-validator.pub

# View fingerprint
ssh-keygen -l -f ~/.ssh/gizzi-validator.pub

# Test connection
ssh -i ~/.ssh/gizzi-validator root@<vm-ip> "echo 'Connection successful!'"
```

### Add Key to Running VM

```bash
# If VM is already running without your key
ssh-copy-id -i ~/.ssh/gizzi-validator.pub root@<gizzi-vm-ip>

# Or manually
cat ~/.ssh/gizzi-validator.pub | ssh root@<gizzi-vm-ip> \
  "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys"
```

### Generate Key with Passphrase (More Secure)

```bash
# Generate with passphrase for production
ssh-keygen -t ed25519 \
  -C "gizzi-validator-01-production" \
  -f ~/.ssh/gizzi-validator-prod

# You'll be prompted for passphrase
# Use SSH agent to avoid typing passphrase repeatedly:
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/gizzi-validator-prod
```

---

## 📊 SSH Access Matrix

| Who | Validator | SSH Key | Access Level |
|-----|-----------|---------|--------------|
| You (Eoj) | All 21 | All keys | Full root access |
| Gizzi (AI) | validator-01 | gizzi-validator | Root (own validator) |
| Automation Scripts | All | deployment-key | Limited (deploy only) |

---

## 🆘 Troubleshooting

### "Permission denied (publickey)"

**Solutions:**
```bash
# 1. Check key file permissions
chmod 600 ~/.ssh/gizzi-validator

# 2. Verify public key is on server
ssh root@<vm-ip> "cat ~/.ssh/authorized_keys"

# 3. Use verbose mode to debug
ssh -vvv -i ~/.ssh/gizzi-validator root@<vm-ip>

# 4. Test key locally
ssh-keygen -y -f ~/.ssh/gizzi-validator
```

### "Could not open a connection to your authentication agent"

```bash
# Start SSH agent
eval "$(ssh-agent -s)"

# Add key
ssh-add ~/.ssh/gizzi-validator
```

### Lost SSH Key

```bash
# 1. Restore from encrypted backup
gpg -d validator-ssh-keys-backup.tar.gz.gpg | tar xz -C ~/.ssh/

# 2. Or regenerate and add via cloud console
# Generate new key
ssh-keygen -t ed25519 -f ~/.ssh/gizzi-validator-new

# Add via cloud provider console (Hetzner/Vultr/DO)
# Then update ~/.ssh/config
```

---

## 📖 Related Documentation

- **GIT_SECURITY_GUIDE.md** - What NOT to commit (includes SSH keys)
- **VM_SECURITY_AND_ACCESS_KEYS.md** - Comprehensive VM security (in generated-keys/)
- **VALIDATOR_QUICKSTART.md** - How to start validators after SSH access

---

## 🎯 Summary

**Gizzi's SSH Key Status:**
- ✅ Private key: `~/.ssh/gizzi-validator`
- ✅ Public key: `~/.ssh/gizzi-validator.pub`
- ✅ Ready to deploy to VM
- ✅ Protected by .gitignore (won't commit)

**Next Steps:**
1. Add public key to Gizzi's VM when deploying infrastructure
2. Test SSH connection: `ssh -i ~/.ssh/gizzi-validator root@<gizzi-vm-ip>`
3. Generate SSH keys for other validators as needed
4. Create backup of all SSH keys (encrypted)

**Remember:** SSH keys are for VM access, validator keys (AURA/GRANDPA/ASF) are separate and already generated in `generated-keys/`!
