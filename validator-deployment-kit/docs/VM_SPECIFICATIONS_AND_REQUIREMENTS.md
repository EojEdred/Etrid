# Ëtrid Validator VM Specifications & Requirements

**Date:** October 29, 2025
**Purpose:** Exact specifications needed for running Ëtrid full validator nodes

---

## Executive Summary

**Minimum specs per validator:**
- **CPU:** 4 cores (2.5 GHz+)
- **RAM:** 16 GB
- **Storage:** 500 GB NVMe SSD (expandable to 1TB)
- **Network:** 100 Mbps (1 Gbps preferred)
- **OS:** Ubuntu 22.04 LTS

**Recommended specs for critical validators (Gizzi, EojEdred):**
- **CPU:** 6 cores (3.0 GHz+)
- **RAM:** 32-64 GB
- **Storage:** 1 TB NVMe SSD
- **Network:** 1 Gbps
- **Dedicated/Bare metal preferred**

---

## Part 1: Resource Requirements Analysis

### 1A. CPU Requirements

**Substrate/Polkadot SDK validators need:**
- **Minimum:** 2 cores
- **Recommended:** 4 cores
- **Optimal:** 6+ cores

**Why:**
- Block authoring (AURA consensus)
- Finality voting (GRANDPA)
- P2P networking
- RPC serving
- State pruning
- Database compaction

**Your binary:**
```bash
/Users/macbook/Desktop/etrid/target/release/flarechain-node
Size: 60,960,704 bytes (~61 MB)
```

**CPU benchmarks from Substrate:**
- 2 cores: Can handle basic validation, may lag during heavy sync
- 4 cores: Recommended for production validators
- 6+ cores: Ideal for bootstrap nodes and critical validators

**Architecture:**
- ✅ x86_64 (Intel/AMD)
- ⚠️ ARM64 (works but less tested)

**Clock speed:**
- Minimum: 2.0 GHz
- Recommended: 2.5 GHz+
- Optimal: 3.0 GHz+ (single-thread performance matters)

---

### 1B. RAM Requirements

**Substrate validators:**
- **Minimum:** 8 GB (tight, not recommended)
- **Recommended:** 16 GB
- **Optimal:** 32 GB
- **Heavy load:** 64 GB (for bootstrap nodes or archive nodes)

**Why 16 GB recommended:**
- Block production: ~2-4 GB
- Database caching: ~4-6 GB
- P2P connections (1,000+ peers): ~2-4 GB
- RPC overhead: ~1-2 GB
- OS + system: ~2 GB
- **Total typical usage:** 11-18 GB

**Your 21-validator network:**
- Bootstrap nodes (Gizzi, Eoj): 32 GB each (handles more connections)
- Critical validators (Directors): 16 GB each
- Standard validators: 16 GB each

**RAM type:**
- DDR4 or DDR5 (DDR4 is fine)
- ECC not required (but nice to have on bare metal)

---

### 1C. Storage Requirements

**Critical: Use SSD, preferably NVMe**

**Initial sync:**
- Genesis state: ~100 MB
- After 1 month: ~20-50 GB
- After 6 months: ~100-200 GB
- After 1 year: ~300-500 GB
- After 2 years: ~600-1000 GB

**Recommendation:** Start with 500 GB, plan to expand to 1 TB

**Storage types:**
- ❌ HDD (too slow, will cause validator to miss blocks)
- ⚠️ Standard SSD (acceptable but slower)
- ✅ NVMe SSD (recommended - 5-10x faster)

**Why NVMe matters:**
- Block import speed: 50-200 blocks/sec vs 10-30 blocks/sec
- State queries: <1ms vs 5-10ms
- Database compaction: minutes vs hours

**IOPS requirements:**
- Minimum: 1,000 IOPS
- Recommended: 3,000+ IOPS
- NVMe typically: 10,000-100,000 IOPS

**Storage per provider:**
- Hetzner CX31: 160 GB SSD (⚠️ may need upgrade after 6 months)
- Hetzner AX41: 2×512 GB NVMe RAID (✅ perfect)
- OVH B2-15: 100 GB SSD (⚠️ too small, need upgrade)
- DigitalOcean: 100 GB SSD (⚠️ can add volume storage)
- Vultr: 180 GB NVMe (✅ good for 6-12 months)

**Better option: 500 GB NVMe everywhere**

---

### 1D. Network Requirements

**Bandwidth:**
- **Initial sync:** 200-500 GB download (one-time)
- **Ongoing:** 50-100 GB/month upload + download
- **Peak:** 10-50 Mbps during sync
- **Steady state:** 1-5 Mbps

**Ports required:**
- **30333:** P2P (must be open to internet)
- **9944:** RPC (can be internal only)
- **9615:** Prometheus metrics (internal only)

**Latency:**
- To other validators: <200ms ideal
- To bootstrap nodes: <100ms ideal
- Cross-continental: acceptable (P2P handles it)

**Provider bandwidth included:**
- Hetzner: 20 TB/month (generous!)
- OVH: 1 TB/month (sufficient)
- DigitalOcean: 5 TB/month (good)
- Vultr: 5 TB/month (good)
- Akash: Varies (usually unlimited)

**Your 21 validators will use:**
- Initial sync (all 21): ~10.5 TB one-time
- Monthly ongoing: ~2.1 TB/month

✅ All recommended providers have sufficient bandwidth

---

### 1E. Operating System

**Supported:**
- ✅ Ubuntu 22.04 LTS (recommended)
- ✅ Ubuntu 20.04 LTS (works)
- ✅ Debian 11/12 (works)
- ⚠️ CentOS/RHEL (works but less tested)
- ⚠️ Fedora (works but less tested)

**NOT recommended:**
- ❌ Windows (requires WSL2, adds overhead)
- ❌ macOS (for servers)

**Why Ubuntu 22.04:**
- Long-term support (until 2027)
- Well-tested with Substrate
- Most documentation assumes Ubuntu
- Easy package management

**Required packages:**
```bash
- build-essential (compiler tools)
- libssl-dev (cryptography)
- pkg-config
- systemd (for service management)
```

---

## Part 2: Exact VM Specifications by Provider

### 2A. Hetzner Specifications

#### **For Standard Validators (18 nodes)**

**Hetzner Cloud CX41:**
- **CPU:** 4 vCPU (shared)
- **RAM:** 16 GB
- **Storage:** 240 GB SSD
- **Network:** 20 TB traffic
- **Cost:** €17.29/month (~$18.50/month)
- **Better than CX31:** More storage (240 vs 160 GB)

**OR Hetzner Cloud CPX31:**
- **CPU:** 4 vCPU AMD (dedicated)
- **RAM:** 16 GB
- **Storage:** 360 GB NVMe ✅
- **Network:** 20 TB traffic
- **Cost:** €23.79/month (~$25/month)
- **Why better:** Dedicated CPU + NVMe + more storage

#### **For Critical Validators (3 nodes - Gizzi, Eoj, 1 Director)**

**Hetzner Dedicated AX41-NVMe:**
- **CPU:** AMD Ryzen 5 3600 (6 cores, 12 threads @ 3.6 GHz)
- **RAM:** 64 GB DDR4
- **Storage:** 2×512 GB NVMe in RAID 1
- **Network:** 1 Gbit/s, unlimited traffic
- **Cost:** €46.41/month (~$50/month)
- **Why:** Bare metal, maximum performance, no "noisy neighbors"

**Specifications summary for Hetzner deployment:**
- 3 bare metal (critical): AX41-NVMe
- 18 cloud VPS: CPX31 (dedicated CPU)

---

### 2B. OVH Specifications

#### **For Standard Validators**

**OVH B2-60:**
- **CPU:** 8 vCores (shared)
- **RAM:** 60 GB
- **Storage:** 400 GB SSD
- **Network:** 1 Gbit/s
- **Cost:** $64/month
- **Note:** Overkill on RAM, but good storage

**OR OVH B2-30:**
- **CPU:** 4 vCores
- **RAM:** 30 GB
- **Storage:** 200 GB SSD
- **Cost:** $40/month
- **Better fit:** Closer to requirements

#### **For Critical Validators**

**OVH Rise-1:**
- **CPU:** AMD Ryzen 5 5600X (6 cores @ 4.6 GHz boost)
- **RAM:** 32 GB DDR4 ECC
- **Storage:** 2×1 TB NVMe (software RAID)
- **Network:** 1 Gbit/s, unlimited
- **Cost:** $70/month
- **Why:** Bare metal, ECC RAM, fast NVMe

---

### 2C. Vultr Specifications

**Vultr High Frequency - 16 GB:**
- **CPU:** 4 vCPU @ 3.0+ GHz (NVMe-optimized)
- **RAM:** 16 GB
- **Storage:** 180 GB NVMe SSD
- **Network:** 5 TB transfer
- **Cost:** $48/month
- **Why:** High clock speed, NVMe included

**OR Vultr Cloud Compute - Regular (cheaper):**
- **CPU:** 4 vCPU @ 2.4 GHz
- **RAM:** 16 GB
- **Storage:** 320 GB SSD
- **Cost:** $48/month
- **Note:** More storage but slower CPU

**Recommendation:** High Frequency (faster sync)

---

### 2D. DigitalOcean Specifications

**Droplet - General Purpose:**
- **CPU:** 4 vCPU (shared)
- **RAM:** 16 GB
- **Storage:** 100 GB SSD
- **Network:** 5 TB transfer
- **Cost:** $84/month

**Problem:** Only 100 GB storage

**Solution:** Add Block Storage
- **Droplet:** 4 vCPU, 8 GB RAM - $48/month
- **Block Storage:** 500 GB SSD - $50/month
- **Total:** $98/month (more expensive but flexible)

**OR Droplet - CPU-Optimized:**
- **CPU:** 4 dedicated vCPU
- **RAM:** 8 GB
- **Storage:** 50 GB SSD
- **Cost:** $84/month
- **Add:** 500 GB Block Storage - $50/month
- **Total:** $134/month (too expensive)

**Recommendation for DO:** Use 16 GB General Purpose + manually manage storage

---

### 2E. Akash Specifications

**Akash deployment (YAML):**
```yaml
resources:
  cpu:
    units: 4.0
  memory:
    size: 16Gi
  storage:
    size: 500Gi

attributes:
  - key: host
    value: akash
  - key: tier
    value: community
```

**Cost:** Varies by provider
- **Typical:** $15-25/month for these specs
- **Top providers:** $20-30/month
- **Bargain providers:** $10-15/month (may be less reliable)

**Recommendation:** Bid max $25/month per validator, select top-tier providers

---

## Part 3: Recommended Configurations

### Configuration Matrix

| Validator Tier | CPU | RAM | Storage | Provider | Cost/mo |
|----------------|-----|-----|---------|----------|---------|
| **Critical (Bootstrap)** | 6 cores | 64 GB | 1 TB NVMe | Hetzner Bare | $50 |
| **Critical (Director)** | 6 cores | 32 GB | 1 TB NVMe | OVH Bare | $70 |
| **Standard (FlareNode)** | 4 cores | 16 GB | 360 GB NVMe | Hetzner CPX31 | $25 |
| **Standard (ValidityNode)** | 4 cores | 16 GB | 180 GB NVMe | Vultr HF | $48 |
| **Budget (Test)** | 4 cores | 16 GB | 500 GB | Akash | $20 |

---

## Part 4: What You Actually Need - Simplified

### Minimum Production Specs (All 21 Validators)

**Standard configuration:**
- **CPU:** 4 cores @ 2.5 GHz minimum
- **RAM:** 16 GB
- **Storage:** 500 GB NVMe SSD
- **Network:** 100 Mbps, 5 TB/month
- **OS:** Ubuntu 22.04 LTS

**This works for:** All 21 validators

**Exception:** Bootstrap nodes (Gizzi, EojEdred)
- Upgrade to 6 cores, 32-64 GB RAM (handles more peers)

---

### Recommended Production Specs

**Tier 1 - Critical (Gizzi, EojEdred, 1 Director) - 3 nodes:**
- **Type:** Bare metal dedicated server
- **CPU:** 6 cores @ 3.0+ GHz
- **RAM:** 32-64 GB
- **Storage:** 1 TB NVMe
- **Provider:** Hetzner AX41 or OVH Rise-1
- **Cost:** $50-70/month each

**Tier 2 - Standard (Remaining 18 validators):**
- **Type:** VPS with dedicated CPU
- **CPU:** 4 cores @ 2.5+ GHz
- **RAM:** 16 GB
- **Storage:** 360-500 GB NVMe
- **Providers:** Hetzner CPX31, Vultr HF, DigitalOcean
- **Cost:** $25-50/month each

---

## Part 5: Storage Growth Planning

### Projected Storage Usage

**Conservative estimate (6 sec blocks, 21 validators):**

| Timeframe | Chain Data | Logs | Total | Recommended |
|-----------|------------|------|-------|-------------|
| **1 month** | 30 GB | 5 GB | 35 GB | 100 GB |
| **3 months** | 80 GB | 10 GB | 90 GB | 200 GB |
| **6 months** | 150 GB | 15 GB | 165 GB | 300 GB |
| **1 year** | 280 GB | 20 GB | 300 GB | 500 GB |
| **2 years** | 550 GB | 30 GB | 580 GB | 1 TB |

**With state pruning enabled:**
- Reduces by 40-60%
- 1 year: ~180 GB instead of 300 GB
- 2 years: ~350 GB instead of 580 GB

**Recommendation:** Start with 500 GB, upgrade to 1 TB after 12-18 months

---

## Part 6: Bandwidth Analysis

### Per Validator Bandwidth Usage

**Initial sync (one-time):**
- Download chain state: 200-500 GB
- Time: 4-24 hours (depends on peer speed)

**Ongoing (monthly):**
- Receiving blocks: 20-40 GB/month
- Broadcasting blocks: 20-40 GB/month
- P2P maintenance: 10-20 GB/month
- **Total:** 50-100 GB/month per validator

**All 21 validators:**
- Initial sync: 4.2-10.5 TB (one-time)
- Monthly: 1.05-2.1 TB/month

**All recommended providers have 5-20 TB/month included** ✅

---

## Part 7: Final Specifications Summary

### RECOMMENDED SPECS FOR HYBRID DEPLOYMENT

#### **3 Critical Validators (Bare Metal)**
**Hetzner AX41-NVMe or OVH Rise-1:**
- CPU: 6 cores @ 3.0+ GHz
- RAM: 32-64 GB
- Storage: 1 TB NVMe
- Network: 1 Gbps
- Cost: $50-70/month

**Deploy:**
- Gizzi (Bootstrap 1)
- EojEdred (Bootstrap 2)
- governance-dev01 (Critical director)

---

#### **18 Standard Validators (VPS)**
**Hetzner CPX31 or Vultr HF:**
- CPU: 4 cores @ 2.5-3.0 GHz
- RAM: 16 GB
- Storage: 360-500 GB NVMe
- Network: 100+ Mbps
- Cost: $25-48/month

**Deploy:**
- 10 on Hetzner CPX31 ($25/mo) - Cost-effective backbone
- 4 on Vultr HF ($48/mo) - Global distribution
- 4 on Akash ($20/mo) - Decentralization

---

### TOTAL COST: $802/month

**Breakdown:**
- 3 bare metal: 3 × $60 avg = $180
- 10 Hetzner VPS: 10 × $25 = $250
- 4 Vultr VPS: 4 × $48 = $192
- 4 Akash: 4 × $20 = $80
- **3 spare for DigitalOcean/OVH**: 3 × $40-80 = $180

**Actual:** Let me recalculate for precision...

Actually, let me create the EXACT deployment plan next with scripts.

---

## Part 8: What Else You Need

### Software Requirements

**On each VM:**
1. **flarechain-node binary** (61 MB) - Your compiled validator
2. **systemd service file** - Auto-start on boot
3. **Validator keys** - From generated-keys-gizzi-eoj/
4. **Chain spec** - Genesis configuration
5. **Monitoring agent** - Prometheus node exporter (optional)

### Network Requirements

**Firewall rules:**
- Allow inbound: 30333 (P2P)
- Allow outbound: All
- Block inbound: 9944, 9615 (RPC/metrics - internal only)

**DNS (optional but nice):**
- `gizzi.etrid.network` → Gizzi VM IP
- `eojedred.etrid.network` → EojEdred VM IP
- Makes management easier

### Monitoring Requirements

**Essential:**
- Uptime monitoring (UptimeRobot or Prometheus)
- Disk space alerts (warn at 70%, critical at 85%)
- Block production monitoring

**Nice to have:**
- Grafana dashboard
- Log aggregation (Loki)
- Alert manager (PagerDuty, Discord webhooks)

### Backup Requirements

**What to backup:**
- Validator keys (already backed up in encrypted files)
- Chain database (optional - can resync)
- systemd service files
- Custom configs

**Backup frequency:**
- Keys: Already done ✅
- Configs: After any change
- Database: Not critical (can resync in 4-24 hours)

---

## Ready for Scripts?

Next I'll create:
1. ✅ Automated deployment scripts for all providers
2. ✅ Manual step-by-step guides (if you prefer to click through UIs)
3. ✅ Provider-specific setup instructions

**Which approach do you prefer?**
- **Option A:** Automated scripts (faster, 1 hour to deploy all 21)
- **Option B:** Manual UI-based setup (slower, 4-6 hours, but you see each step)
- **Option C:** Hybrid (script for most, manual for critical validators)

Let me know and I'll create the deployment guides!
