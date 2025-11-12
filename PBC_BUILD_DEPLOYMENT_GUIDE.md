# ËTRID PBC Collator Build & Deployment Guide

**Expert-Level Architecture-Aware Deployment System**

---

## Overview

This guide covers building and deploying all 12 PBC (Partition Burst Chain) collators across your VM infrastructure, respecting architecture differences between ARM and x86_64 systems.

### PBC Collators (12 Total)

1. **btc-pbc-collator** - Bitcoin
2. **doge-pbc-collator** - Dogecoin
3. **sol-pbc-collator** - Solana
4. **xlm-pbc-collator** - Stellar
5. **xrp-pbc-collator** - Ripple
6. **bnb-pbc-collator** - BNB Chain
7. **trx-pbc-collator** - Tron
8. **ada-pbc-collator** - Cardano
9. **link-pbc-collator** - Chainlink
10. **matic-pbc-collator** - Polygon
11. **sc-usdt-pbc-collator** - USDT Stablecoin
12. **edsc-pbc-collator** - ËTRID Dollar Stablecoin

> **Note:** `eth-pbc-collator` is excluded from this build system as it uses Frontier from polkadot-stable2506 and has separate build requirements.

---

## Architecture Strategy

### Group 1: ARM VMs (Oracle Cloud)
- **VMs:** d1 (Gizzi), d5 (Audit)
- **Architecture:** ARM (aarch64)
- **Strategy:** Build locally on each VM
- **Reason:** ARM binaries must be compiled natively

### Group 2: x86_64 VMs (Contabo/Azure)
- **Count:** 13+ VMs
- **Architecture:** x86_64
- **Strategy:** Build once, distribute to all
- **Primary Build VM:** Configurable (default: `etrid-mainnet`)
- **Distribution:** rsync to all other VMs

---

## Deployment Scripts

### 1. `build-all-pbcs-expert.sh`
**Core build engine** - Can be run on any VM

```bash
# Sequential build (safer, lower RAM usage)
./build-all-pbcs-expert.sh sequential

# Parallel build (faster, requires 32GB+ RAM)
./build-all-pbcs-expert.sh parallel

# Create distribution tarball
./build-all-pbcs-expert.sh distribute
```

**Features:**
- Architecture detection
- Parallel or sequential builds
- Build logging to `/tmp/*-pbc-collator-build.log`
- Binary verification
- Distribution tarball creation

### 2. `deploy-pbcs-arm-vms.sh`
**ARM VM deployment** - Builds on d1 and d5 concurrently

```bash
./deploy-pbcs-arm-vms.sh
```

**Workflow:**
1. Uploads build script to each ARM VM
2. Starts builds concurrently on d1 and d5
3. Uses sequential mode (safer for ARM)
4. Verifies completion

### 3. `deploy-pbcs-contabo-vms.sh`
**Contabo/x86_64 deployment** - Build once, distribute to all

```bash
# Deploy to all Contabo VMs
./deploy-pbcs-contabo-vms.sh deploy

# Verify deployment
./deploy-pbcs-contabo-vms.sh verify

# Use custom primary build VM
PRIMARY_BUILD_VM=etrid-val-01 ./deploy-pbcs-contabo-vms.sh deploy
```

**Workflow:**
1. Build all PBCs on primary VM (parallel if RAM ≥ 24GB)
2. Create `~/pbc-binaries/` directory with all binaries
3. Rsync binaries to all other Contabo VMs
4. Verify deployment

### 4. `deploy-all-pbcs-master.sh`
**Master orchestrator** - Deploys to all VMs

```bash
# Interactive menu (recommended)
./deploy-all-pbcs-master.sh

# Non-interactive: deploy to all VMs
./deploy-all-pbcs-master.sh all

# Deploy ARM VMs only
./deploy-all-pbcs-master.sh arm

# Deploy Contabo VMs only
./deploy-all-pbcs-master.sh contabo

# Verify all deployments
./deploy-all-pbcs-master.sh verify
```

---

## Quick Start

### Option 1: Deploy to All VMs (Recommended)

```bash
cd ~/Desktop/etrid
./deploy-all-pbcs-master.sh all
```

This will:
1. Build on d1 and d5 concurrently (ARM)
2. Build on primary Contabo VM (x86_64)
3. Distribute to all other Contabo VMs
4. Verify all deployments

**Estimated Time:**
- ARM VMs: 45-90 minutes per VM (concurrent)
- Contabo build: 30-60 minutes (depends on RAM/parallel mode)
- Distribution: 5-10 minutes
- **Total: ~60-90 minutes**

### Option 2: Interactive Mode

```bash
cd ~/Desktop/etrid
./deploy-all-pbcs-master.sh
```

Select from menu:
1. Deploy to ARM VMs only
2. Deploy to Contabo VMs only
3. Deploy to ALL VMs ← **Recommended**
4. Verify deployment status
5. Exit

### Option 3: Selective Deployment

```bash
# ARM VMs only
./deploy-all-pbcs-master.sh arm

# Contabo VMs only with custom primary
PRIMARY_BUILD_VM=etrid-val-01 ./deploy-all-pbcs-master.sh contabo
```

---

## Customization

### Configure Primary Build VM

Default is `etrid-mainnet`. To change:

```bash
export PRIMARY_BUILD_VM=etrid-val-01
./deploy-pbcs-contabo-vms.sh deploy
```

Or edit `deploy-pbcs-contabo-vms.sh` line 28:
```bash
PRIMARY_BUILD_VM="${PRIMARY_BUILD_VM:-your-vm-name}"
```

### Configure Target VMs

Edit `deploy-pbcs-contabo-vms.sh` lines 31-42:

```bash
CONTABO_VMS=(
    "etrid-val-01"
    "etrid-val-02"
    # Add your VMs here
)
```

Add your actual SSH host aliases from `~/.ssh/config`.

### Build Mode Selection

The Contabo script auto-detects RAM:
- **RAM ≥ 24GB:** Parallel mode (faster)
- **RAM < 24GB:** Sequential mode (safer)

To force a mode, edit the script or run directly:
```bash
ssh etrid-mainnet 'cd ~/Desktop/etrid && ./build-all-pbcs-expert.sh parallel'
```

---

## Verification

### Check Build Status on All VMs

```bash
./deploy-all-pbcs-master.sh verify
```

### Manual Verification

**ARM VMs:**
```bash
ssh d1 'ls -lh ~/Desktop/etrid/target/release/*-pbc-collator'
ssh d5 'ls -lh ~/Desktop/etrid/target/release/*-pbc-collator'
```

**Contabo VMs:**
```bash
for vm in etrid-val-01 etrid-val-02; do
    echo "=== $vm ==="
    ssh $vm 'ls -lh ~/pbc-binaries/*-pbc-collator 2>/dev/null | wc -l'
done
```

Expected: **12 binaries** per VM

---

## Troubleshooting

### Build Fails on ARM VM

**Issue:** Limited RAM or swap
**Solution:**
```bash
ssh d1
cd ~/Desktop/etrid
# Increase swap if needed
sudo fallocate -l 16G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Build sequentially
./build-all-pbcs-expert.sh sequential
```

### Cannot Connect to VM

**Issue:** VM not accessible
**Solution:**
1. Check SSH config: `cat ~/.ssh/config | grep -A 5 "Host vm-name"`
2. Test connection: `ssh -v vm-name`
3. Check firewall/security groups

### Rsync Distribution Fails

**Issue:** SSH key not configured between VMs
**Solution:**
```bash
# On primary build VM
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519 -N ""

# Copy to target VMs
for vm in etrid-val-01 etrid-val-02; do
    ssh-copy-id $vm
done
```

### Out of Disk Space

**Issue:** target/ directory too large
**Solution:**
```bash
# Clean build artifacts
cd ~/Desktop/etrid
cargo clean

# Or only clean release artifacts
rm -rf target/release
```

### Binary Not Found After Build

**Issue:** Build reported success but binary missing
**Solution:**
```bash
# Check build logs
tail -100 /tmp/*-pbc-collator-build.log

# Rebuild specific PBC
cargo build --release -p btc-pbc-collator
```

---

## Binary Locations

### ARM VMs (d1, d5)
```
~/Desktop/etrid/target/release/btc-pbc-collator
~/Desktop/etrid/target/release/doge-pbc-collator
# ... etc (12 total)
```

### Contabo VMs
```
~/pbc-binaries/btc-pbc-collator
~/pbc-binaries/doge-pbc-collator
# ... etc (12 total)
```

---

## Performance Optimization

### Parallel Build Jobs

Auto-detected based on CPU cores. To override:

```bash
PARALLEL_JOBS=8 ./build-all-pbcs-expert.sh parallel
```

### Build Cache

Use `sccache` for faster subsequent builds:

```bash
# Install sccache
cargo install sccache

# Configure
export RUSTC_WRAPPER=sccache
./build-all-pbcs-expert.sh parallel
```

### Incremental Builds

If you need to rebuild after code changes:

```bash
# Only rebuild changed crates
cargo build --release -p btc-pbc-collator

# Or rebuild all PBCs (faster than full clean build)
./build-all-pbcs-expert.sh sequential
```

---

## Integration with Services

### Systemd Service Example

After deployment, create services on each VM:

```bash
# Example: BTC PBC collator service
sudo tee /etc/systemd/system/btc-pbc-collator.service <<EOF
[Unit]
Description=ËTRID BTC PBC Collator
After=network.target

[Service]
Type=simple
User=ubuntu
ExecStart=/home/ubuntu/pbc-binaries/btc-pbc-collator \\
    --collator \\
    --force-authoring \\
    --chain btc-pbc \\
    --base-path /var/lib/btc-pbc \\
    --port 30333 \\
    --rpc-port 9944
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable btc-pbc-collator
sudo systemctl start btc-pbc-collator
```

---

## Architecture Comparison

| Aspect | ARM VMs (d1, d5) | x86_64 VMs (Contabo) |
|--------|------------------|----------------------|
| Build Location | On each VM | Primary VM only |
| Build Mode | Sequential | Auto (Parallel/Sequential) |
| Build Time | ~60-90 min | ~30-60 min |
| Distribution | Not needed | Rsync to all |
| Binary Size | ~150-200 MB each | ~150-200 MB each |
| Total Disk | ~2-3 GB | ~2-3 GB |

---

## Best Practices

1. **Always verify** after deployment: `./deploy-all-pbcs-master.sh verify`
2. **Test on one VM first** before full deployment
3. **Monitor disk space** - builds require 10-20 GB free
4. **Use parallel mode** on VMs with 24GB+ RAM
5. **Keep build logs** for troubleshooting
6. **Update git** before building: `git pull origin main`
7. **Clean old builds** periodically: `cargo clean`

---

## Summary

This expert-level deployment system:

✅ Respects architecture differences (ARM vs x86_64)
✅ Optimizes build time (build once for x86_64, distribute to all)
✅ Provides comprehensive verification
✅ Handles errors gracefully
✅ Supports parallel and sequential builds
✅ Fully automated deployment

**Recommended Command:**
```bash
cd ~/Desktop/etrid
./deploy-all-pbcs-master.sh all
```

This will deploy all 12 PBC collators to all VMs in the most efficient way possible.

---

## Support

For issues or questions:
1. Check build logs: `/tmp/*-pbc-collator-build.log`
2. Verify SSH connectivity: `ssh vm-name 'echo OK'`
3. Check disk space: `ssh vm-name 'df -h'`
4. Review this guide's Troubleshooting section

**Happy building! 🚀**
