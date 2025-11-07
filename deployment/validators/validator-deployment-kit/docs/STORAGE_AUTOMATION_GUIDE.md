# Storage Automation Guide for Ëtrid Validators

**Date:** October 29, 2025
**Purpose:** Complete guide for managing hybrid storage across 21 validators

---

## Overview

This guide covers the automated storage management system for your 21-validator network. With the hybrid storage strategy, you'll:

- **Year 1:** Use included VPS storage with pruning = **$675/month**
- **Year 2:** Add block storage selectively = **$875/month**
- **Always:** Backup to Backblaze B2 = **$5/month**

**Total savings vs Azure:** $1,225/month = **$14,700/year**

---

## Storage Automation Scripts

### 1. `monitor-validator-storage.sh` - Storage Monitoring

**Purpose:** Check storage usage across all 21 validators and alert when thresholds are exceeded

**Usage:**
```bash
cd /Users/macbook/Desktop/etrid/scripts
./monitor-validator-storage.sh [inventory-file]
```

**Features:**
- ✅ Checks all validators via SSH
- ✅ Shows usage per validator in table format
- ✅ Alerts at 70% (warning) and 85% (critical)
- ✅ Identifies which validators need storage expansion
- ✅ Shows unreachable validators

**Example output:**
```
Validator            IP              Used       Total      Usage %    Status
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
validator-01         65.108.1.1      45G        360G       13%        ✓ OK
validator-02         65.108.1.2      52G        360G       15%        ✓ OK
validator-04         157.90.1.1      280G       360G       78%        ⚠ WARNING - Consider attaching storage
validator-05         157.90.1.2      320G       360G       89%        ✗ CRITICAL - Attach storage NOW!
...
```

**Recommended:** Run weekly or set up as cron job:
```bash
# Add to crontab
0 9 * * 1 /path/to/monitor-validator-storage.sh > /var/log/storage-monitor.log 2>&1
```

---

### 2. `attach-block-storage.sh` - Attach Block Storage

**Purpose:** Interactive helper to attach and configure block storage to a validator

**Usage:**
```bash
./attach-block-storage.sh <validator-name> <provider>
```

**Examples:**
```bash
# Attach 500GB volume to validator-04 on Hetzner
./attach-block-storage.sh validator-04 hetzner

# Attach block storage to validator-15 on Vultr
./attach-block-storage.sh validator-15 vultr

# Attach volume to validator-18 on DigitalOcean
./attach-block-storage.sh validator-18 digitalocean
```

**What it does:**
1. Shows provider-specific instructions and costs
2. Guides you through creating volume via CLI
3. SSH into validator VM
4. Formats and mounts the storage
5. Adds to `/etc/fstab` for persistence
6. Verifies configuration

**Costs per provider:**
- **Hetzner:** $25/month per 500GB volume
- **Vultr:** $50/month per 500GB block storage
- **DigitalOcean:** $50/month per 500GB volume

**When to use:**
- Storage usage > 300GB (approaching 360GB limit)
- Monitor script shows CRITICAL alert
- Validator runs out of disk space

---

### 3. `auto-tier-storage.sh` - Automatic Data Tiering

**Purpose:** Move old blockchain data from local NVMe to attached block storage

**Usage:**
```bash
# Run on validator VM (not locally)
ssh root@validator-ip
./auto-tier-storage.sh
```

**What it does:**
1. Checks local storage usage
2. If > 75% full, moves files older than 90 days to archive
3. Keeps hot data on fast local NVMe
4. Moves cold data to cheaper block storage
5. Maintains blockchain integrity

**Configuration:**
```bash
LOCAL_PATH="/var/lib/etrid"              # Blockchain data
ARCHIVE_PATH="/mnt/blockchain-archive"   # Attached storage
LOCAL_THRESHOLD=75                       # Trigger at 75% full
DAYS_OLD=90                              # Move files > 90 days old
```

**Safety features:**
- ✅ Checks if archive storage exists
- ✅ Prompts before stopping validator
- ✅ Confirms before moving data
- ✅ Verifies validator restart

**Recommended:** Run monthly or automate:
```bash
# Add to validator VM crontab
0 3 1 * * /path/to/auto-tier-storage.sh > /var/log/etrid-tiering.log 2>&1
```

---

### 4. `backup-to-b2.sh` - Backup to Backblaze B2

**Purpose:** Backup all 21 validators to Backblaze B2 object storage

**Usage:**
```bash
./backup-to-b2.sh [inventory-file]
```

**Setup (first time only):**

1. **Create Backblaze account:**
   - Go to: https://www.backblaze.com/b2/sign-up.html
   - Free tier: 10GB storage, 1GB/day download

2. **Install B2 CLI:**
   ```bash
   brew install b2-tools
   # Or: pip install b2
   ```

3. **Get API keys:**
   - Go to: https://secure.backblaze.com/app_keys.htm
   - Create "New Application Key"
   - Copy Key ID and Application Key

4. **Authenticate:**
   ```bash
   b2 authorize-account <keyID> <applicationKey>
   ```

**What it does:**
1. Connects to each validator via SSH
2. Creates compressed archive of blockchain data
3. Uploads to B2 bucket
4. Deletes backups older than 7 days
5. Shows backup summary

**Features:**
- ✅ Automatic compression (tar.gz)
- ✅ Parallel backups (can run concurrently)
- ✅ Automatic retention (keeps last 7 days)
- ✅ Cost-effective ($5/month for all 21 validators)

**Cost calculation:**
- 21 validators × 150GB each = 3.15TB
- Keep 7 days only = ~500GB average
- Backblaze: $5/TB/month = **$3-5/month total**

**Recommended:** Run daily:
```bash
# Add to crontab
0 2 * * * /path/to/backup-to-b2.sh > /var/log/etrid-backup.log 2>&1
```

---

## Complete Storage Workflow

### Year 1: Initial Setup (Months 1-12)

**Goal:** Use included storage with pruning, no extra costs

**Steps:**

1. **Deploy validators with pruning:**
   ```bash
   # Pruning already configured in deploy-validator-software.sh
   --pruning 256
   --state-pruning archive-canonical
   ```

2. **Monitor storage weekly:**
   ```bash
   ./monitor-validator-storage.sh
   ```

3. **Setup automated backups:**
   ```bash
   # Configure B2
   b2 authorize-account <keyID> <applicationKey>

   # Test backup
   ./backup-to-b2.sh

   # Add to crontab
   crontab -e
   # Add: 0 2 * * * /path/to/backup-to-b2.sh
   ```

**Expected storage usage:**
- Month 1: 30GB
- Month 6: 80GB
- Month 12: 150GB

**No extra storage needed!** All fits in 360GB included.

---

### Year 2: Selective Storage Expansion (Months 13-24)

**Goal:** Attach volumes only to validators approaching limits

**When to attach:**
- Storage monitor shows > 300GB used
- Storage usage > 80%
- Less than 60GB free

**Process:**

1. **Identify validators needing storage:**
   ```bash
   ./monitor-validator-storage.sh
   # Look for WARNING or CRITICAL alerts
   ```

2. **Attach storage to specific validators:**
   ```bash
   # Example: validator-04 needs storage
   ./attach-block-storage.sh validator-04 hetzner

   # Example: validator-15 needs storage
   ./attach-block-storage.sh validator-15 vultr
   ```

3. **Tier old data to archive:**
   ```bash
   ssh root@validator-04
   ./auto-tier-storage.sh
   ```

4. **Monitor new usage:**
   ```bash
   ./monitor-validator-storage.sh
   # Verify validator-04 now shows lower usage
   ```

**Expected cost increase:**
- ~8 validators will need storage
- Hetzner: 8 × $25 = $200/month
- **Total Year 2:** $675 + $200 = $875/month

---

## Provider-Specific Storage Pricing

### Hetzner (Recommended - Cheapest)

**Included Storage:**
- CPX31: 360GB NVMe (sufficient for Year 1)
- AX41 bare metal: 1TB NVMe (no expansion needed)

**Block Storage (Volumes):**
| Size | Monthly Cost | IOPS |
|------|--------------|------|
| 10GB | $0.50 | 1,000 |
| 100GB | $5 | 10,000 |
| 500GB | **$25** | 10,000 |
| 1TB | $50 | 10,000 |

**How to attach:**
```bash
hcloud volume create --name validator-XX-archive --size 500 --location fsn1
hcloud volume attach validator-XX-archive validator-XX
```

---

### Vultr

**Included Storage:**
- High Frequency VPS: 180GB NVMe

**Block Storage:**
- $0.10/GB/month
- 500GB = **$50/month**
- NVMe-backed, high IOPS

**How to attach:**
```bash
vultr-cli block-storage create --label validator-XX-archive --size 500 --region ewr
# Attach via web console
```

---

### DigitalOcean

**Included Storage:**
- Premium AMD: 100GB SSD

**Volumes:**
- $0.10/GB/month
- 500GB = **$50/month**
- SSD, 7,500 IOPS

**How to attach:**
```bash
doctl compute volume create validator-XX-archive --size 500GiB --region nyc3
doctl compute volume-action attach validator-XX-archive <droplet-id>
```

---

## Backup and Recovery

### Backup Strategy

**Tier 3 (Object Storage) - Long-term backups:**

1. **Daily automated backups:**
   ```bash
   0 2 * * * /path/to/backup-to-b2.sh
   ```

2. **What gets backed up:**
   - Blockchain database: `/var/lib/etrid/chains/mainnet/db`
   - Validator keys: Already encrypted and stored separately

3. **Retention policy:**
   - Daily backups: Keep 7 days
   - Weekly backups: Keep 4 weeks (manual)
   - Monthly backups: Keep 12 months (manual)

4. **Storage required:**
   - 7 days × 21 validators × 150GB = ~3TB
   - With compression: ~500GB
   - **Cost:** $3-5/month

### Recovery Process

**Scenario 1: Single validator failure**

```bash
# 1. List available backups
b2 ls etrid-validator-backups | grep validator-04

# 2. Download latest backup
b2 download-file-by-name etrid-validator-backups \
  validator-04-20251029-020000.tar.gz \
  /tmp/validator-04-backup.tar.gz

# 3. Extract to validator
scp /tmp/validator-04-backup.tar.gz root@validator-04:/tmp/
ssh root@validator-04
systemctl stop etrid-validator
tar xzf /tmp/validator-04-backup.tar.gz -C /
systemctl start etrid-validator
```

**Scenario 2: Complete infrastructure loss**

1. Re-deploy infrastructure using `deploy-hybrid-multi-provider.sh`
2. Wait for VMs to provision
3. Restore from B2 backups to all validators
4. Restart validators in order (bootstraps first)
5. Verify committee formation

**Recovery Time Objective (RTO):** 2-4 hours
**Recovery Point Objective (RPO):** 24 hours (daily backups)

---

## Monitoring and Alerts

### Storage Monitoring Dashboard

**Weekly manual check:**
```bash
./monitor-validator-storage.sh > /var/log/storage-report-$(date +%Y%m%d).log
cat /var/log/storage-report-$(date +%Y%m%d).log
```

**Automated monitoring (recommended):**

Create `/usr/local/bin/storage-alert.sh`:
```bash
#!/bin/bash
REPORT=$(/path/to/monitor-validator-storage.sh)

# Check for CRITICAL alerts
if echo "$REPORT" | grep -q "CRITICAL"; then
    # Send alert (email, Slack, Discord, etc.)
    echo "$REPORT" | mail -s "CRITICAL: Validator storage alert" your-email@example.com
fi
```

Add to crontab:
```bash
0 9 * * * /usr/local/bin/storage-alert.sh
```

### Key Metrics to Track

| Metric | Warning | Critical | Action |
|--------|---------|----------|--------|
| Storage usage | > 70% | > 85% | Attach volume |
| Free space | < 100GB | < 50GB | Immediate action |
| Backup age | > 48h | > 72h | Check backup script |
| Growth rate | > 30GB/mo | > 50GB/mo | Review pruning |

---

## Cost Summary by Scenario

### Scenario 1: Optimal (Following this guide)

**Year 1:**
```
21 validators with pruning
- Hetzner: 13 validators × included storage = $0
- Vultr: 4 validators × included storage = $0
- DigitalOcean: 3 validators × included storage = $0
- Akash: 1 validator × included storage = $0
- Backups: Backblaze B2 = $3/month

Total extra storage: $3/month
Total with VPS: $675/month
```

**Year 2:**
```
8 validators need expansion
- Hetzner volumes: 8 × $25 = $200/month
- Backups: Backblaze B2 = $5/month

Total extra storage: $205/month
Total with VPS: $875/month
```

### Scenario 2: No Optimization (No pruning, all archive mode)

**Year 1:**
```
21 validators with archive mode (1TB each)
- All need block storage immediately
- Hetzner: 13 × $50 = $650/month
- Vultr: 4 × $50 = $200/month
- DigitalOcean: 3 × $50 = $150/month

Total extra storage: $1,000/month
Total with VPS: $1,675/month
```

**Savings with optimization:** $1,000/month = **$12,000/year**

### Scenario 3: Azure Comparison

**Azure (original plan):**
```
21 validators on Azure
- 21 × $100/month = $2,100/month
- Includes managed disks (500GB each)

Total: $2,100/month
```

**Savings vs Azure:**
- Year 1: $2,100 - $675 = **$1,425/month** = **$17,100/year**
- Year 2: $2,100 - $875 = **$1,225/month** = **$14,700/year**

---

## Best Practices

### Storage Management

1. **Always use pruning mode** for validators (not archive mode)
2. **Monitor weekly** with `monitor-validator-storage.sh`
3. **Attach storage proactively** at 70% usage, don't wait for 85%
4. **Use Hetzner volumes** when possible (cheapest at $25/mo)
5. **Only run 1 archive node** (not on a validator) if historical RPC access is needed

### Backup Management

1. **Daily backups** to Backblaze B2
2. **Test recovery** quarterly
3. **Keep 7-day retention** (balance cost vs safety)
4. **Monitor backup job** logs
5. **Verify backup integrity** monthly

### Cost Optimization

1. **Start with pruning** - saves $1,000/month vs archive mode
2. **Use included storage** - saves $200/month in Year 1
3. **Attach selectively** - only 8/21 validators need volumes by Year 2
4. **Use Hetzner** for volume-needing validators ($25 vs $50)
5. **Optimize backups** - 7-day retention vs 30-day saves 75%

---

## Troubleshooting

### Issue 1: "Monitor script shows validator unreachable"

**Cause:** SSH connection failed or validator offline

**Solution:**
```bash
# Verify VM is running
hcloud server list | grep validator-XX

# Test SSH manually
ssh -v root@validator-ip

# Check firewall
# Add SSH key if missing
```

### Issue 2: "Auto-tier script fails to move files"

**Cause:** No attached storage or wrong path

**Solution:**
```bash
# Verify archive storage exists
ssh root@validator-ip
df -h | grep archive

# If missing, attach storage first
./attach-block-storage.sh validator-XX hetzner
```

### Issue 3: "Backup script fails with B2 error"

**Cause:** Not authenticated or bucket doesn't exist

**Solution:**
```bash
# Check authentication
b2 account info

# Re-authenticate
b2 authorize-account <keyID> <applicationKey>

# Create bucket
b2 create-bucket etrid-validator-backups allPrivate
```

### Issue 4: "Validator runs out of space despite pruning"

**Cause:** High transaction volume or pruning not configured

**Solution:**
```bash
# Check pruning is enabled
ssh root@validator-ip
ps aux | grep flarechain-node | grep pruning

# If not, update systemd service
systemctl edit etrid-validator
# Add: --pruning 256 --state-pruning archive-canonical

systemctl restart etrid-validator
```

---

## Quick Reference

### Daily Operations

```bash
# Check all validators (weekly)
./monitor-validator-storage.sh

# Backup all validators (automated daily)
./backup-to-b2.sh
```

### When Storage Warning Appears

```bash
# 1. Check which validator
./monitor-validator-storage.sh | grep WARNING

# 2. Attach storage
./attach-block-storage.sh validator-XX hetzner

# 3. Tier old data
ssh root@validator-xx ./auto-tier-storage.sh

# 4. Verify
./monitor-validator-storage.sh | grep validator-XX
```

### Emergency Recovery

```bash
# 1. List backups
b2 ls etrid-validator-backups | grep validator-XX

# 2. Download latest
b2 download-file-by-name etrid-validator-backups <file> /tmp/

# 3. Restore
scp /tmp/backup.tar.gz root@validator-xx:/tmp/
ssh root@validator-xx "systemctl stop etrid-validator && tar xzf /tmp/backup.tar.gz -C / && systemctl start etrid-validator"
```

---

## Summary

**You now have:**

✅ **4 automation scripts:**
- `monitor-validator-storage.sh` - Weekly monitoring
- `attach-block-storage.sh` - Guided storage expansion
- `auto-tier-storage.sh` - Automatic data tiering
- `backup-to-b2.sh` - Daily backups to B2

✅ **Optimized storage strategy:**
- Year 1: $675/month (no extra storage)
- Year 2: $875/month (selective expansion)
- Backups: $3-5/month (Backblaze B2)

✅ **Massive cost savings:**
- vs Azure: Save $1,225-1,425/month
- vs No optimization: Save $1,000/month
- **Total: $14,700-17,100/year saved**

✅ **Production-ready:**
- Automated monitoring
- Guided expansion process
- Daily backups with retention
- Recovery procedures documented

**Next steps:**
1. Run `./monitor-validator-storage.sh` to establish baseline
2. Setup daily B2 backups with `./backup-to-b2.sh`
3. Review storage weekly
4. Attach volumes in Year 2 as needed

**Questions? See the main documentation:**
- HYBRID_STORAGE_STRATEGY.md - Complete strategy
- DEPLOYMENT_OPTIONS_SUMMARY.md - Quick start
- HYBRID_DEPLOYMENT_GUIDE.md - Full deployment guide
