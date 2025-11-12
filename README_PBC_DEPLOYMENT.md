# ËTRID PBC Deployment System

**Expert-level architecture-aware build and deployment for all Partition Burst Chain collators**

---

## 🚀 Quick Start (One Command!)

```bash
cd ~/Desktop/etrid
./deploy-all-pbcs-master.sh all
```

This will build and deploy all 12 PBC collators to all your VMs (ARM + x86_64) in the most efficient way possible.

---

## 📚 Documentation

- **[PBC_QUICK_REFERENCE.md](PBC_QUICK_REFERENCE.md)** - Start here! Quick commands and essential info
- **[PBC_BUILD_DEPLOYMENT_GUIDE.md](PBC_BUILD_DEPLOYMENT_GUIDE.md)** - Complete guide with troubleshooting

---

## 🎯 What This Does

Deploys **12 PBC collators** to your infrastructure:

1. btc-pbc-collator
2. doge-pbc-collator
3. sol-pbc-collator
4. xlm-pbc-collator
5. xrp-pbc-collator
6. bnb-pbc-collator
7. trx-pbc-collator
8. ada-pbc-collator
9. link-pbc-collator
10. matic-pbc-collator
11. sc-usdt-pbc-collator
12. edsc-pbc-collator

---

## 🏗️ Architecture-Aware Strategy

### ARM VMs (d1, d5 - Oracle Cloud)
- Build **locally on each VM** (ARM requires native compilation)
- Concurrent builds on both VMs
- Time: 60-90 minutes per VM

### x86_64 VMs (13+ Contabo/Azure)
- Build **once on primary VM**
- Distribute to all other VMs via rsync
- Time: 30-60 min build + 5-10 min distribution

---

## 📦 Scripts

| Script | Purpose | Use When |
|--------|---------|----------|
| `deploy-all-pbcs-master.sh` | **Master orchestrator** | Deploy to all VMs (recommended) |
| `build-all-pbcs-expert.sh` | Core build engine | Manual builds on specific VMs |
| `deploy-pbcs-arm-vms.sh` | ARM deployment | Deploy to d1, d5 only |
| `deploy-pbcs-contabo-vms.sh` | Contabo deployment | Deploy to x86_64 VMs only |

---

## ⚡ Common Commands

### Deploy to All VMs
```bash
./deploy-all-pbcs-master.sh all
```

### Deploy to Specific Group
```bash
./deploy-all-pbcs-master.sh arm      # ARM VMs only
./deploy-all-pbcs-master.sh contabo  # Contabo VMs only
```

### Verify Deployment
```bash
./deploy-all-pbcs-master.sh verify
```

### Interactive Mode
```bash
./deploy-all-pbcs-master.sh  # Shows menu
```

---

## 🔧 Customization

### Set Primary Build VM

Default is `etrid-mainnet`. To change:

```bash
PRIMARY_BUILD_VM=etrid-val-01 ./deploy-all-pbcs-master.sh contabo
```

### Configure Your VMs

Edit `deploy-pbcs-contabo-vms.sh` (lines 31-42):

```bash
CONTABO_VMS=(
    "etrid-val-01"
    "etrid-val-02"
    # Add your VM hostnames here
)
```

---

## ✅ Prerequisites

- [ ] SSH access to all VMs
- [ ] Git repository up to date
- [ ] Rust toolchain installed on all VMs
- [ ] 15GB+ free disk space per VM
- [ ] Scripts are executable (`chmod +x *.sh`)

---

## 🔍 Verification

After deployment, verify binaries:

```bash
# Quick check
./deploy-all-pbcs-master.sh verify

# Manual check on ARM
ssh d1 'ls -lh ~/Desktop/etrid/target/release/*-pbc-collator | wc -l'
ssh d5 'ls -lh ~/Desktop/etrid/target/release/*-pbc-collator | wc -l'

# Manual check on Contabo
ssh etrid-mainnet 'ls -lh ~/pbc-binaries/ | wc -l'
```

**Expected: 12 binaries on each VM**

---

## ⏱️ Expected Timeline

| Phase | Time |
|-------|------|
| ARM builds (d1, d5) | 60-90 min (concurrent) |
| Contabo build | 30-60 min |
| Distribution | 5-10 min |
| **Total** | **~60-90 minutes** |

---

## 🐛 Troubleshooting

### Build fails
```bash
# Check logs
tail -100 /tmp/*-pbc-collator-build.log
```

### VM not accessible
```bash
ssh -v vm-name
```

### Out of disk space
```bash
cargo clean
```

See full troubleshooting in [PBC_BUILD_DEPLOYMENT_GUIDE.md](PBC_BUILD_DEPLOYMENT_GUIDE.md)

---

## 🎨 Features

- ✅ Respects ARM vs x86_64 architecture
- ✅ Parallel builds (when RAM permits)
- ✅ Concurrent ARM VM builds
- ✅ Efficient x86_64 distribution
- ✅ Comprehensive logging
- ✅ Automatic verification
- ✅ Interactive & non-interactive modes
- ✅ Error handling & recovery

---

## 📍 Binary Locations

**ARM VMs:**
```
~/Desktop/etrid/target/release/*-pbc-collator
```

**Contabo VMs:**
```
~/pbc-binaries/*-pbc-collator
```

---

## 🎓 Advanced Usage

### Manual Build on Specific VM
```bash
ssh d1
cd ~/Desktop/etrid
./build-all-pbcs-expert.sh sequential
```

### Parallel Build (32GB+ RAM)
```bash
./build-all-pbcs-expert.sh parallel
```

### Custom Parallel Jobs
```bash
PARALLEL_JOBS=8 ./build-all-pbcs-expert.sh parallel
```

---

## 📖 Full Documentation

For complete details, examples, and advanced topics:

👉 **Read [PBC_BUILD_DEPLOYMENT_GUIDE.md](PBC_BUILD_DEPLOYMENT_GUIDE.md)**

For quick commands and reference:

👉 **Read [PBC_QUICK_REFERENCE.md](PBC_QUICK_REFERENCE.md)**

---

## 🏆 Best Practices

1. Always run `./deploy-all-pbcs-master.sh verify` after deployment
2. Test on one VM before deploying to all
3. Monitor disk space (need 15-20 GB free)
4. Keep build logs for troubleshooting
5. Update git before building: `git pull origin main`

---

## 🌟 Why This System?

Traditional approach: Build on each VM individually
- Time: 13 VMs × 60 min = **~13 hours**
- Risk: 13 separate builds to manage

**This expert system:**
- Time: **~60-90 minutes total**
- Efficiency: Build once for x86_64, distribute to 13+ VMs
- Reliability: Centralized build, verified distribution
- Architecture-aware: ARM VMs get native builds

**Result: 8-10x faster deployment with better reliability**

---

## 📞 Support

Issues? Check:
1. Build logs: `/tmp/*-pbc-collator-build.log`
2. SSH connectivity: `ssh vm-name 'echo OK'`
3. Disk space: `ssh vm-name 'df -h'`
4. [PBC_BUILD_DEPLOYMENT_GUIDE.md](PBC_BUILD_DEPLOYMENT_GUIDE.md) Troubleshooting section

---

## 🎯 Summary

**Deploy all 12 PBC collators to all VMs:**

```bash
./deploy-all-pbcs-master.sh all
```

**That's it!** The system handles everything automatically. ✨

---

**Created for ËTRID by an expert system that respects your architecture.**

Happy deploying! 🚀
