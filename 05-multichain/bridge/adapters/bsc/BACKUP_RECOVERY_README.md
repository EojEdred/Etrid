# 💾 Backup & Recovery System

Enterprise backup and disaster recovery for your MasterChef deployment.

**Never lose your contract state, deployment artifacts, or historical data again!**

---

## 🎯 What Gets Backed Up

✅ **Contract Artifacts** - Compiled contracts, deployment records, ABIs
✅ **On-Chain State** - Current MasterChef configuration and pool data
✅ **Configuration Files** - .env, hardhat.config, package.json, workflows
✅ **Database** - All historical metrics and event logs
✅ **Compression** - Automatic tar.gz compression
✅ **Encryption** - Optional AES-256 encryption

---

## 🚀 Quick Start

### Create Full Backup

```bash
npm run backup:full
```

**Output:**
```
🔄 CREATING FULL BACKUP

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📝 Backing up contract artifacts...
   ✅ Contract artifacts backed up
⛓️  Backing up on-chain state...
   ✅ Mainnet state backed up
   ✅ Testnet state backed up
⚙️  Backing up configuration...
   ✅ Configuration backed up
💾 Backing up database...
   ✅ Database backed up
🗜️  Compressing backup...
   ✅ Compressed to full_2025-10-24_1729789012345.tar.gz

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ FULL BACKUP COMPLETE

   Location: backups/full_2025-10-24_1729789012345.tar.gz
   Size: 2.45 MB
   Checksum: a1b2c3d4e5f6...
```

### Restore from Backup

```bash
npm run backup:restore backups/full_2025-10-24_1729789012345.tar.gz
```

---

## ⚙️ Configuration

### Environment Variables

```bash
# Enable encryption (optional)
BACKUP_ENCRYPT=true
BACKUP_ENCRYPTION_KEY=your-secret-key-min-32-chars

# Max backups to keep (default: 30)
MAX_BACKUPS=30

# Contract addresses (for state backup)
ETR_TOKEN_ADDRESS_MAINNET=0x...
MASTERCHEF_ADDRESS_MAINNET=0x...
```

### Enable Encryption

```bash
# Generate secure key
openssl rand -base64 32

# Add to .env
BACKUP_ENCRYPT=true
BACKUP_ENCRYPTION_KEY=your-generated-key
```

---

## 📦 Backup Components

### 1. Contract Artifacts
- Compiled contracts (`artifacts/`)
- Deployment records (`deployments/`)
- Source code (`contracts/`)

### 2. On-Chain State
**Mainnet:**
- MasterChef configuration
- All pool data (LP tokens, allocPoints, staked amounts)
- Ownership
- Pause state
- ETR token balances

**Testnet:**
- Same as mainnet

### 3. Configuration Files
- `.env.example`
- `hardhat.config.ts`
- `package.json`
- `tsconfig.json`
- `.github/workflows/`

### 4. Database
- SQLite database (`masterchef.db`)
- SQL dump for portability

---

## 🔄 Automated Backups

### Daily Cron Job

```bash
# Edit crontab
crontab -e

# Add daily backup at 2 AM
0 2 * * * cd /path/to/etrid/05-multichain/bridge/adapters/bsc && npm run backup:full >> /var/log/backup.log 2>&1
```

### Using PM2

```bash
# Install PM2 cron module
pm2 install pm2-auto-cron

# Schedule daily backup
pm2 start ecosystem.config.js
```

**ecosystem.config.js:**
```javascript
module.exports = {
  apps: [{
    name: 'backup',
    script: 'npm',
    args: 'run backup:full',
    cron_restart: '0 2 * * *',
    autorestart: false
  }]
};
```

---

## 🔐 Security Best Practices

### 1. Encrypt Sensitive Backups

```bash
BACKUP_ENCRYPT=true
BACKUP_ENCRYPTION_KEY=min-32-character-secret-key
```

### 2. Store Backups Offsite

```bash
# After backup, copy to S3
aws s3 cp backups/full_*.tar.gz s3://your-bucket/masterchef-backups/

# Or rsync to remote server
rsync -avz backups/ user@backup-server:/backups/masterchef/
```

### 3. Verify Backups Regularly

```bash
# Test restore to temp directory
mkdir test-restore
cd test-restore
npm run backup:restore ../backups/full_latest.tar.gz
```

### 4. Protect Encryption Key

```bash
# Store key in secrets manager (AWS, Vault, etc.)
# Never commit to git
echo "BACKUP_ENCRYPTION_KEY" >> .gitignore
```

---

## 📊 Backup Management

### List Backups

```bash
ls -lh backups/
```

### Check Backup Size

```bash
du -sh backups/
```

### Verify Checksum

```bash
sha256sum backups/full_2025-10-24_*.tar.gz
# Compare with checksum in database
```

### Delete Old Backups

```bash
# Manually (keeps last 10)
ls -t backups/full_*.tar.gz | tail -n +11 | xargs rm

# Automatic cleanup (configured via MAX_BACKUPS)
```

---

## 🔧 Advanced Usage

### Backup Only Specific Components

Modify `backup.ts` to export individual methods:

```typescript
const backup = new BackupSystem();

// Contracts only
await backup.backupContracts(targetPath);

// State only
await backup.backupState(targetPath);

// Config only
await backup.backupConfig(targetPath);

// Database only
await backup.backupDatabase(targetPath);
```

### Remote Backup Storage

```bash
#!/bin/bash
# backup-to-s3.sh

# Create backup
npm run backup:full

# Find latest backup
LATEST=$(ls -t backups/full_*.tar.gz | head -1)

# Upload to S3
aws s3 cp "$LATEST" s3://your-bucket/masterchef-backups/

# Optional: Delete local after upload
rm "$LATEST"
```

---

## 🛠️ Troubleshooting

### "Database locked" error

```bash
# Close all connections
lsof database/masterchef.db | grep node | awk '{print $2}' | xargs kill

# Retry backup
npm run backup:full
```

### Encryption key error

```bash
# Verify key is set
echo $BACKUP_ENCRYPTION_KEY

# Regenerate if needed
openssl rand -base64 32
```

### Restore fails

```bash
# Check backup integrity
tar -tzf backups/full_*.tar.gz

# Try manual extract
mkdir temp-backup
tar -xzf backups/full_*.tar.gz -C temp-backup
```

---

## 📚 Summary

✅ **Full backups** in one command
✅ **Automatic compression** (tar.gz)
✅ **Optional encryption** (AES-256)
✅ **On-chain state** preservation
✅ **Automatic cleanup** of old backups
✅ **One-click restore**
✅ **Cron-ready** for automation

**Cost**: $0 (only storage)
**Setup Time**: 2 minutes
**Recovery Time**: < 5 minutes

---

**Ready to protect your deployment?** Run `npm run backup:full` now! 💾
