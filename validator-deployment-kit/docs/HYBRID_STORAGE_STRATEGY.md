# Hybrid Storage Strategy for 21 Ëtrid Validators

**Date:** October 29, 2025
**Purpose:** Optimize storage costs using tiered hybrid storage

---

## Problem

**Original plan:** 500GB-1TB NVMe SSD per validator × 21 = expensive
**Better approach:** Hybrid storage with hot/warm/cold tiers

---

## Cost Comparison: Traditional vs Hybrid

### Traditional Approach (All NVMe)

**21 validators with 500GB NVMe each:**

| Provider | Storage/Validator | 21 Validators | Monthly Cost |
|----------|-------------------|---------------|--------------|
| Hetzner CPX31 | 360GB NVMe included | Free (in VPS) | $0 |
| Hetzner CPX41 | 240GB SSD included | Free (in VPS) | $0 |
| Need to upgrade | +140GB each | 21 × $? | Expensive |

**Problem:** Most providers don't offer 500GB in base VPS
**Solution:** Use smaller base VPS + attach block storage

---

### Hybrid Approach (Tiered Storage)

**Architecture per validator:**
- **Tier 1 (Local SSD):** 200GB NVMe - Active chain state
- **Tier 2 (Block Storage):** 500GB - Archive blocks
- **Tier 3 (Object Storage):** Unlimited - Backups

**Cost breakdown:**

| Component | Cost/Validator | 21 Validators | Monthly |
|-----------|----------------|---------------|---------|
| Base VPS (200GB) | Included | 21 × $0 | $0 |
| Block Storage (500GB) | $50 | 21 × $50 | $1,050 |
| Object Storage (1TB backup) | $5 | 1 × $5 | $5 |
| **TOTAL** | **$55/validator** | | **$1,055/mo** |

**Wait, that's MORE expensive!**

Let me recalculate with provider-optimized approach...

---

## Optimized Hybrid Strategy

### Strategy 1: Provider-Included Storage (CHEAPEST)

**Use VPS plans that already include enough storage:**

**Hetzner CPX31:**
- 360GB NVMe included
- Sufficient for Year 1
- 21 × €23.79 = €500/month (~$535/month)
- **No extra storage cost!**

**When storage fills up (Year 2):**
- Attach Hetzner Volume: 500GB = €23.80/month (~$25)
- Only attach to validators that need it (~10 validators)
- Extra cost: $250/month

**Year 1:** $535/month (base only)
**Year 2:** $535 + $250 = $785/month (still cheaper than all large VPS)

---

### Strategy 2: Mix Small VPS + Attached Storage

**Use smallest VPS + large attached volumes:**

**Hetzner CCX13:**
- 2 vCPU, 8GB RAM, 80GB SSD
- €9.51/month ($10/month)

**+ Hetzner Volume:**
- 500GB = €23.80/month ($25/month)

**Total per validator:** $35/month
**21 validators:** $735/month

**Savings:** $137/month vs CPX31 approach

**Problem:** Only 2 vCPU may not be enough for validators
**Better:** Use 4 vCPU VPS + attached storage selectively

---

### Strategy 3: Tiered by Validator Importance (RECOMMENDED)

**Critical validators (3) - No compromises:**
- Hetzner AX41 bare metal (1TB NVMe included)
- $50/month each = $150/month
- **Storage:** Included, no extra cost

**Standard validators (12) - Included storage:**
- Hetzner CPX31 (360GB NVMe included)
- $25/month each = $300/month
- **Storage:** Included, enough for Year 1

**Budget validators (6) - Hybrid storage:**
- Vultr 80GB VPS ($24/month) + 500GB block storage ($50/month)
- $74/month each = $444/month
- **Total with storage:** $444/month

**Total: $150 + $300 + $444 = $894/month**

**Still more than our original $802... Let's optimize further.**

---

## FINAL OPTIMIZED STRATEGY

### Smart Tiered Deployment with Pruning

**Key insight:** Validators don't need ALL historical blocks

**Substrate pruning modes:**
1. **Archive mode:** Keep all blocks (1TB+ over time) ❌ Expensive
2. **Pruned mode:** Keep only recent state (100-200GB) ✅ Perfect for validators
3. **Fast sync:** Quick startup, minimal storage ✅ Great for new validators

**Recommended setup:**

#### **All 21 Validators: Pruned Mode**

**Configure each validator:**
```bash
/usr/local/bin/flarechain-node \
  --base-path /var/lib/etrid \
  --pruning 256 \
  --state-pruning archive-canonical \
  ...
```

**Storage needed with pruning:**
- Month 1: 30GB
- Month 6: 80GB
- Month 12: 150GB
- Month 24: 250GB

**This fits in base VPS storage for 12-18 months!**

---

### Deployment Plan with Pruning

**Year 1 (Months 1-12):**

**All validators use base VPS storage only:**
- Hetzner CPX31: 360GB NVMe (plenty for pruned mode)
- 21 × $25 = $525/month
- **No extra storage needed!**

**Plus 3 bare metal:**
- Hetzner AX41: 1TB NVMe (critical validators)
- 3 × $50 = $150/month

**Year 1 Total:** $675/month ($8,100/year)

---

**Year 2 (Months 13-24):**

**Some validators approaching 360GB limit:**
- Attach block storage to ~8 validators
- Hetzner Volume: 500GB = $25/month each
- 8 × $25 = $200/month extra

**Year 2 Total:** $675 + $200 = $875/month ($10,500/year)

**Still cheaper than Azure: $2,100/month!**

---

## When to Use Hybrid Storage

### Use Attached Block Storage When:

1. **Archive node required** (1 per network for RPC)
   - Keep ALL historical blocks
   - Need 1-2TB storage
   - Use on 1 dedicated node (not a validator)

2. **Storage exceeds base plan** (Year 2+)
   - Pruned chain grows to 300-400GB
   - Base VPS only has 360GB
   - Attach 500GB volume

3. **Backup storage** (all validators)
   - Daily snapshots to object storage
   - Backblaze B2: $5/month for 1TB
   - Backup all 21 validators to single bucket

### Don't Use Attached Storage When:

- ❌ Base VPS already has enough (Year 1)
- ❌ Can upgrade VPS to larger size cheaper
- ❌ Validator can use pruning instead

---

## Provider-Specific Hybrid Storage Options

### Hetzner (Our Primary Provider)

**Base VPS Options:**

| Type | vCPU | RAM | Storage | Price | Good For |
|------|------|-----|---------|-------|----------|
| CPX21 | 3 | 8GB | 240GB NVMe | $18 | Too small |
| CPX31 | 4 | 16GB | **360GB NVMe** | $25 | **Perfect** ✅ |
| CPX41 | 8 | 32GB | 480GB NVMe | $45 | Overkill |

**Attached Volume Pricing:**
- 10GB: €0.476/month ($0.50)
- 100GB: €4.76/month ($5)
- 500GB: €23.80/month ($25)
- 1TB: €47.60/month ($50)

**Best approach:** CPX31 + attach 500GB volume when needed

---

### Vultr (Secondary Provider)

**Base VPS Options:**

| Type | vCPU | RAM | Storage | Price | Good For |
|------|------|-----|---------|-------|----------|
| Regular | 4 | 16GB | 320GB SSD | $48 | OK |
| High Freq | 4 | 16GB | **180GB NVMe** | $48 | Needs storage |

**Attached Block Storage:**
- $0.10/GB/month
- 500GB = $50/month
- NVMe-backed

**Best approach:** High Frequency + 500GB block = $98/month (expensive!)

**Better:** Use Regular plan with 320GB included

---

### DigitalOcean (Tertiary Provider)

**Base Droplet:**
- 4 vCPU, 16GB RAM
- **Only 100GB SSD** ⚠️
- $84/month

**Attached Volume:**
- $0.10/GB/month
- 500GB = $50/month
- **Total:** $134/month (too expensive!)

**Better approach:** Use Hetzner instead, or accept 100GB with aggressive pruning

---

## Practical Implementation

### Setup 1: Base VPS Only (Year 1)

**Hetzner CPX31 with pruning:**

```bash
# Launch validator with pruning
/usr/local/bin/flarechain-node \
  --base-path /var/lib/etrid \
  --chain mainnet \
  --pruning 256 \
  --state-pruning archive-canonical \
  --db-cache 4096 \
  --validator

# Monitor storage usage
df -h /var/lib/etrid

# Expected: 30-150GB in Year 1
```

**Cost:** $25/month per validator
**Storage:** 360GB available, 100-150GB used

---

### Setup 2: With Attached Block Storage (Year 2+)

**When local storage reaches 300GB:**

**1. Create and attach Hetzner volume:**

```bash
# Via Hetzner Cloud Console
hcloud volume create \
  --name validator-04-archive \
  --size 500 \
  --location fsn1

hcloud volume attach validator-04-archive validator-04
```

**2. Mount on VM:**

```bash
ssh root@validator-04

# Format volume (first time only)
mkfs.ext4 /dev/sdb

# Create mount point
mkdir -p /mnt/blockchain-archive

# Mount volume
mount /dev/sdb /mnt/blockchain-archive

# Make permanent
echo '/dev/sdb /mnt/blockchain-archive ext4 defaults,nofail 0 2' >> /etc/fstab

# Set permissions
chown -R root:root /mnt/blockchain-archive
```

**3. Move archive data to volume:**

```bash
# Stop validator
systemctl stop etrid-validator

# Move old chain data to archive
mv /var/lib/etrid/chains/mainnet/db /mnt/blockchain-archive/
ln -s /mnt/blockchain-archive/db /var/lib/etrid/chains/mainnet/db

# Restart validator
systemctl start etrid-validator
```

**Cost:** $25 (base) + $25 (volume) = $50/month

---

### Setup 3: Object Storage for Backups

**Backup all 21 validators to one bucket:**

**Backblaze B2 ($5/month for 1TB):**

```bash
# Install B2 CLI
brew install b2-tools

# Authenticate
b2 authorize-account YOUR_KEY_ID YOUR_APP_KEY

# Create bucket
b2 create-bucket etrid-validator-backups allPrivate

# Backup script (run daily)
#!/bin/bash
for i in {01..21}; do
  VALIDATOR="validator-$i"
  ssh root@$VALIDATOR_IP "tar czf - /var/lib/etrid/chains/mainnet/db" | \
  b2 upload-file --noProgress etrid-validator-backups - ${VALIDATOR}-$(date +%Y%m%d).tar.gz
done
```

**Storage needed:**
- 21 validators × 150GB each = 3.15TB
- Backblaze: $5/TB/month = $16/month
- **Retention:** Keep last 7 days only = reduce to ~500GB = $3/month

---

## Updated Cost Analysis

### Deployment Plan with Smart Storage

**Year 1 (Storage < 360GB):**

| Component | Count | Unit Cost | Total |
|-----------|-------|-----------|-------|
| Hetzner AX41 (bare metal) | 3 | $50 | $150 |
| Hetzner CPX31 (VPS) | 14 | $25 | $350 |
| Vultr Regular (320GB) | 4 | $48 | $192 |
| Backblaze B2 (backups) | 1 | $3 | $3 |
| **Year 1 Total** | **21+backup** | | **$695/mo** |

**Annual:** $8,340

---

**Year 2 (Storage > 360GB on some validators):**

| Component | Count | Unit Cost | Total |
|-----------|-------|-----------|-------|
| Hetzner AX41 | 3 | $50 | $150 |
| Hetzner CPX31 | 14 | $25 | $350 |
| Hetzner Volumes (for 8 validators) | 8 | $25 | $200 |
| Vultr Regular | 4 | $48 | $192 |
| Backblaze B2 | 1 | $5 | $5 |
| **Year 2 Total** | | | **$897/mo** |

**Annual:** $10,764

---

**vs Original Plans:**

| Plan | Year 1 | Year 2 | Notes |
|------|--------|--------|-------|
| **Hybrid Storage** | $695/mo | $897/mo | Optimized! ✅ |
| Original Multi-Provider | $802/mo | $802/mo | No growth planning |
| Azure | $2,100/mo | $2,100/mo | Expensive baseline |

**Savings vs Azure:**
- Year 1: $1,405/month = **$16,860/year**
- Year 2: $1,203/month = **$14,436/year**

---

## Automation Scripts

### Storage Monitoring Script

```bash
#!/bin/bash
# monitor-validator-storage.sh
# Check storage usage on all validators

THRESHOLD=70  # Alert when > 70% full

for i in {01..21}; do
  IP=$(grep "validator-$i" validator-ips.txt | awk '{print $2}')

  if [ ! -z "$IP" ]; then
    USAGE=$(ssh root@$IP "df -h /var/lib/etrid | tail -1 | awk '{print \$5}' | sed 's/%//'")

    echo -n "validator-$i ($IP): ${USAGE}% "

    if [ "$USAGE" -gt "$THRESHOLD" ]; then
      echo "⚠️  WARNING: Consider attaching block storage"
    else
      echo "✓ OK"
    fi
  fi
done
```

---

### Automatic Archive Tiering Script

```bash
#!/bin/bash
# auto-tier-storage.sh
# Move old blocks to attached storage when local fills up

LOCAL_PATH="/var/lib/etrid"
ARCHIVE_PATH="/mnt/blockchain-archive"
LOCAL_THRESHOLD=75  # Move data when local > 75% full

# Check if we have attached storage
if [ ! -d "$ARCHIVE_PATH" ]; then
  echo "No archive storage attached"
  exit 0
fi

# Check local storage usage
USAGE=$(df -h $LOCAL_PATH | tail -1 | awk '{print $5}' | sed 's/%//')

if [ "$USAGE" -gt "$LOCAL_THRESHOLD" ]; then
  echo "Local storage at ${USAGE}%, moving old data to archive..."

  # Find blocks older than 90 days, move to archive
  find $LOCAL_PATH/chains/mainnet/db -type f -mtime +90 \
    -exec mv {} $ARCHIVE_PATH/ \; 2>/dev/null || true

  echo "Data moved. New usage:"
  df -h $LOCAL_PATH
fi
```

---

## Recommendations

### For Ëtrid 21-Validator Network:

**Year 1 Strategy:**
1. **Use Hetzner CPX31** for all standard validators (360GB included)
2. **Enable pruning** on all validators (keeps storage < 200GB)
3. **Use bare metal** for critical validators (1TB included)
4. **Setup Backblaze B2** for daily backups ($3/month)

**Total Year 1:** $695/month

**When to add block storage (Year 2):**
- Storage usage > 300GB (approaching 360GB limit)
- Attach 500GB Hetzner Volume ($25/month per validator)
- Only attach to validators that need it (~8 of 21)

**Total Year 2:** $897/month

---

### Archive Node (Optional)

**If you need full historical RPC access:**

**Deploy 1 dedicated archive node (not a validator):**
- Hetzner CCX33: 8 vCPU, 32GB RAM, 480GB NVMe ($45/month)
- + Hetzner Volume: 2TB ($100/month)
- **Total:** $145/month

**Purpose:**
- Full blockchain history for dApps
- RPC endpoint for developers
- Historical queries

**Not needed for validators** (validators only need recent state)

---

## Summary

**Best approach for your 21 validators:**

1. **Year 1:** Use included storage + pruning = $695/month
2. **Year 2:** Add volumes selectively = $897/month
3. **Backups:** Backblaze B2 = $3-5/month
4. **Archive node:** Optional, $145/month extra

**Total cost:** $695-897/month
**vs Azure:** Save $16,000+/year

**Storage strategy:**
- ✅ Pruned mode on all validators (100-200GB)
- ✅ Use included VPS storage (Year 1)
- ✅ Attach block storage when needed (Year 2)
- ✅ Object storage for backups (always)
- ❌ Don't pay for unused storage upfront

**This is the most cost-effective approach!** 🎯
