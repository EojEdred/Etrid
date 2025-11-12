# PBC Build & Deployment - Quick Reference Card

## 🚀 ONE-COMMAND DEPLOYMENT

```bash
cd ~/Desktop/etrid && ./deploy-all-pbcs-master.sh all
```

**This deploys all 12 PBC collators to all VMs (ARM + x86_64)**

---

## 📋 What Gets Built

12 PBC Collators (ETH excluded - uses Frontier):
- btc-pbc-collator, doge-pbc-collator, sol-pbc-collator
- xlm-pbc-collator, xrp-pbc-collator, bnb-pbc-collator
- trx-pbc-collator, ada-pbc-collator, link-pbc-collator
- matic-pbc-collator, sc-usdt-pbc-collator, edsc-pbc-collator

---

## 🏗️ Architecture Strategy

| VM Group | Strategy | Location |
|----------|----------|----------|
| **ARM** (d1, d5) | Build on each VM | `~/Desktop/etrid/target/release/` |
| **x86_64** (13+ Contabo) | Build once, distribute | `~/pbc-binaries/` |

---

## ⚡ Quick Commands

### Deploy to All VMs
```bash
./deploy-all-pbcs-master.sh all
```

### Deploy ARM Only
```bash
./deploy-all-pbcs-master.sh arm
```

### Deploy Contabo Only
```bash
./deploy-all-pbcs-master.sh contabo
```

### Verify All Deployments
```bash
./deploy-all-pbcs-master.sh verify
```

### Custom Primary Build VM
```bash
PRIMARY_BUILD_VM=etrid-val-01 ./deploy-all-pbcs-master.sh contabo
```

---

## 🔍 Verification

### Check Binaries on ARM VMs
```bash
ssh d1 'ls ~/Desktop/etrid/target/release/*-pbc-collator | wc -l'
ssh d5 'ls ~/Desktop/etrid/target/release/*-pbc-collator | wc -l'
```
**Expected: 12 each**

### Check Binaries on Contabo VMs
```bash
ssh etrid-mainnet 'ls ~/pbc-binaries/*-pbc-collator | wc -l'
```
**Expected: 12**

---

## 🛠️ Manual Build (if needed)

### On ARM VMs (d1, d5)
```bash
ssh d1
cd ~/Desktop/etrid
./build-all-pbcs-expert.sh sequential
```

### On Contabo (Primary)
```bash
ssh etrid-mainnet
cd ~/Desktop/etrid
./build-all-pbcs-expert.sh parallel  # if RAM ≥ 24GB
./build-all-pbcs-expert.sh sequential  # if RAM < 24GB
```

---

## 📊 Build Times

| VM Type | Build Mode | Time |
|---------|------------|------|
| ARM (d1, d5) | Sequential | 60-90 min |
| Contabo (24GB+) | Parallel | 30-45 min |
| Contabo (< 24GB) | Sequential | 45-60 min |

**Distribution:** 5-10 minutes

---

## 🐛 Troubleshooting

### Build Failed
```bash
# Check logs
tail -100 /tmp/*-pbc-collator-build.log

# Retry single PBC
cargo build --release -p btc-pbc-collator
```

### Cannot Connect to VM
```bash
# Test SSH
ssh -v vm-name

# Check SSH config
cat ~/.ssh/config | grep -A 5 "vm-name"
```

### Out of Disk Space
```bash
# Clean old builds
cd ~/Desktop/etrid
cargo clean
```

### Distribution Failed
```bash
# Ensure SSH keys are set up between VMs
ssh primary-vm "ssh target-vm 'echo OK'"
```

---

## 📁 Binary Locations

### ARM VMs
```
~/Desktop/etrid/target/release/*-pbc-collator
```

### Contabo VMs
```
~/pbc-binaries/*-pbc-collator
```

---

## 🔄 Update & Rebuild

```bash
cd ~/Desktop/etrid
git pull origin main
./deploy-all-pbcs-master.sh all
```

---

## 📞 Quick Checks

### Check if VM is Ready
```bash
ssh vm-name 'rustc --version && cargo --version && echo OK'
```

### Check Disk Space
```bash
ssh vm-name 'df -h | grep -E "(Filesystem|/$)"'
```

### Check RAM
```bash
ssh vm-name 'free -h | grep Mem'
```

---

## 🎯 Files You Need

1. `build-all-pbcs-expert.sh` - Core build engine
2. `deploy-pbcs-arm-vms.sh` - ARM deployment
3. `deploy-pbcs-contabo-vms.sh` - Contabo deployment
4. `deploy-all-pbcs-master.sh` - Master orchestrator ⭐

**All scripts are in:** `/Users/macbook/Desktop/etrid/`

---

## 🎨 Color Legend in Scripts

- **BLUE** - Info messages
- **GREEN** - Success
- **YELLOW** - Warnings
- **RED** - Errors
- **MAGENTA** - Section headers

---

## ⏱️ Typical Deployment Timeline

1. **Start deployment:** `./deploy-all-pbcs-master.sh all`
2. **ARM builds start:** d1 and d5 building concurrently
3. **Contabo build starts:** Building on primary VM
4. **Contabo distribution:** Rsync to all other VMs
5. **Verification:** Check all VMs have 12 binaries
6. **Done:** ~60-90 minutes total

---

## 🔐 SSH Config Example

Add to `~/.ssh/config`:
```
Host d1
    HostName <d1-ip>
    User ubuntu
    IdentityFile ~/.ssh/oracle-cloud-key

Host d5
    HostName <d5-ip>
    User ubuntu
    IdentityFile ~/.ssh/oracle-cloud-key

Host etrid-mainnet
    HostName <contabo-ip>
    User ubuntu
    IdentityFile ~/.ssh/id_rsa
```

---

## 🎓 Environment Variables

```bash
# Set primary build VM
export PRIMARY_BUILD_VM=etrid-val-01

# Set build mode
export BUILD_MODE=release  # or debug

# Set parallel jobs
export PARALLEL_JOBS=8

# Set ETRID root (if not default)
export ETRID_ROOT=~/Desktop/etrid
```

---

## ✅ Pre-flight Checklist

Before deployment:
- [ ] All VMs are accessible via SSH
- [ ] Git repo is up to date: `git pull`
- [ ] Sufficient disk space: 15GB+ free
- [ ] Rust toolchain installed on all VMs
- [ ] Scripts are executable: `chmod +x *.sh`

---

## 📖 Full Documentation

See: `PBC_BUILD_DEPLOYMENT_GUIDE.md`

---

**Remember:** The master script handles everything!

```bash
./deploy-all-pbcs-master.sh all
```

That's it! ✨
