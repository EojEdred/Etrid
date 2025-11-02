# Ëtrid FlareChain Mainnet - Validator Documentation

**Last Updated:** November 2, 2025
**Status:** ✅ Production Verified

---

## Overview

This directory contains complete documentation for running Ëtrid FlareChain validators. All configurations have been tested on live mainnet validators and are known to work correctly.

---

## 📚 Documentation Files

### 🚀 **HOW_TO_RUN_A_VALIDATOR.md** - START HERE

**Complete guide for anyone wanting to run a validator**

This is the primary guide for setting up and running your own Ëtrid FlareChain validator node, whether joining the mainnet or creating your own testnet.

**Covers:**
- Building the validator binary from source
- Generating and managing session keys
- Network configuration and firewall setup
- Creating systemd service
- Troubleshooting common issues
- Security best practices

**Who it's for:**
- Developers wanting to join the mainnet
- Anyone setting up their own testnet
- Operators learning to run validators

**Time required:** 30-60 minutes

---

### 🔧 **VALIDATOR_SERVICE_TEMPLATE.md** - Technical Reference

**Detailed systemd service templates and flag reference**

Technical documentation with working configurations from live mainnet validators.

**Covers:**
- Systemd service templates for different platforms
- Complete flag reference and explanations
- Working configurations from production validators
- Common issues and their solutions
- Platform-specific differences (Azure vs Oracle Cloud)

**Who it's for:**
- Operators customizing their setup
- Developers debugging configuration issues
- Anyone needing detailed flag documentation

---

### 📋 **chainspec-mainnet-raw.json** - Mainnet Chainspec

**The official Ëtrid FlareChain mainnet chainspec**

This is the chainspec file used by all mainnet validators.

**Details:**
- Chain: "Ëtrid FlareChain Mainnet"
- Genesis Hash: `0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4`
- Chain Type: "Live"

**Usage:**
```bash
# Download
wget https://raw.githubusercontent.com/EojEdred/Etrid/main/docs/mainnet/chainspec-mainnet-raw.json

# Or copy from this repo
cp docs/mainnet/chainspec-mainnet-raw.json /etc/etrid/chainspec.json
```

---

### 📖 **Other Documentation**

- **VALIDATOR_QUICKSTART.md** - Original quick start guide
- **VALIDATOR_FIREWALL_RULES.md** - Detailed firewall configuration
- **GENESIS_SETUP_GUIDE.md** - Genesis block configuration
- **VALIDATOR_MONITORING_INTEGRATION.md** - Monitoring setup

---

## Quick Start

### For New Validators

**1. Read the main guide:**
```bash
cat docs/mainnet/HOW_TO_RUN_A_VALIDATOR.md
```

**2. Follow the steps:**
- Build the binary
- Generate session keys
- Configure your server
- Start the validator

**3. Verify it's working:**
```bash
sudo journalctl -u etrid-validator -f
```

---

## Critical Configuration Requirements

These are **non-negotiable** and will cause failures if missed:

### ✅ 1. `--public-addr` Flag (CRITICAL)

```bash
--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333
```

**Why it matters:** Without this flag, your validator cannot be discovered by other peers. You'll only connect to 1 peer (the bootnode) instead of 3+.

**How to verify:**
```bash
sudo systemctl cat etrid-validator | grep public-addr
```

---

### ✅ 2. Correct Chainspec

**For mainnet:**
```bash
--chain=/etc/etrid/chainspec.json
```

Must be the file from `docs/mainnet/chainspec-mainnet-raw.json`

**Verify:**
```bash
head -5 /etc/etrid/chainspec.json
# Should show: "name": "Ëtrid FlareChain Mainnet"
```

---

### ✅ 3. Unique Node Key

Each validator needs a unique 64-character hex node key for network identity.

**Generate:**
```bash
openssl rand -hex 32
```

**Never reuse** node keys between validators.

---

### ✅ 4. Session Keys Inserted

Three keys required:
- **AURA** (sr25519) - Block authoring - key type: `aura`
- **GRANDPA** (ed25519) - Finality voting - key type: `gran`
- **ASF** (sr25519) - Attestation - key type: `asfk`

**Verify:**
```bash
ls ~/.local/share/etrid-validator/chains/*/keystore/
# Should show 3 files
```

---

### ✅ 5. Port 30333 Open

```bash
# Verify firewall
sudo ufw status | grep 30333

# Should show: 30333/tcp ALLOW Anywhere
```

---

## What We Fixed (Nov 2, 2025)

### Before (Broken):
```bash
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "validator-name" \
  --chain=/path/to/chainspec.json \
  --node-key abc123... \
  --bootnodes /ip4/... \
  --port 30333

# RESULT: Only 1 peer connected ❌
```

### After (Fixed):
```bash
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "validator-name" \
  --chain=/path/to/chainspec.json \
  --node-key abc123... \
  --public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \  # ← ADDED THIS
  --bootnodes /ip4/... \
  --port 30333

# RESULT: 3-20 peers connected ✅
```

**The missing `--public-addr` flag was preventing peer discovery.**

---

## Verification After Bootstrap

After starting your validator, verify within 60 seconds:

```bash
# 1. Service is running
sudo systemctl is-active etrid-validator
# Expected: active

# 2. Chain name (if joining mainnet)
sudo journalctl -u etrid-validator --no-pager | grep "Chain specification"
# Expected: Chain specification: Ëtrid FlareChain Mainnet

# 3. Peer count
sudo journalctl -u etrid-validator -n 5 | grep "Idle"
# Expected: 💤 Idle (3+ peers) for mainnet, (1+ peers) for testnet

# 4. Block sync
sudo journalctl -u etrid-validator -n 10 | grep "Imported"
# Expected: Multiple "Imported #XXXX" messages
```

---

## Current Mainnet Status

**As of November 2, 2025:**

- ✅ Mainnet is live and operational
- ✅ Multiple validators running
- ✅ Producing blocks consistently
- ✅ All validators synchronized
- ✅ ASF PPFA + GRANDPA consensus working

**Genesis Hash:** `0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4`

**Bootnode:**
```
/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

---

## Troubleshooting

### Only 1 Peer Connected
**Cause:** Missing `--public-addr` flag
**Fix:** Add `--public-addr /ip4/YOUR_IP/tcp/30333` to service file

### Wrong Chain (Shows "Local Testnet")
**Cause:** Using wrong chainspec
**Fix:** Use `chainspec-mainnet-raw.json` from this directory

### Service Won't Start
**Cause:** Various (keys, permissions, paths)
**Fix:** Check detailed logs: `sudo journalctl -u etrid-validator -n 100`

### Can't Connect to Bootnode
**Cause:** Firewall blocking port 30333
**Fix:** `sudo ufw allow 30333/tcp`

See **HOW_TO_RUN_A_VALIDATOR.md** for complete troubleshooting guide.

---

## Mainnet Network Details

### Connection Information

**Bootnode Multiaddr:**
```
/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

**Required in your service file:**
```bash
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

### Chainspec Verification

```bash
# Verify you have the correct chainspec
head -10 /etc/etrid/chainspec.json | grep -E "(name|id|chainType)"

# Expected output:
#   "name": "Ëtrid FlareChain Mainnet",
#   "id": "flarechain_mainnet",
#   "chainType": "Live",
```

---

## Support & Community

### Getting Help

1. **Documentation First:** Read HOW_TO_RUN_A_VALIDATOR.md thoroughly
2. **Check Logs:** `sudo journalctl -u etrid-validator -n 100`
3. **Verify Configuration:** Compare with templates in this directory
4. **GitHub Issues:** https://github.com/EojEdred/Etrid/issues

### Reporting Issues

When reporting an issue, include:
- Your validator setup (OS, specs)
- Service configuration (`sudo systemctl cat etrid-validator`)
- Recent logs (`sudo journalctl -u etrid-validator -n 100`)
- Steps to reproduce

---

## Success Criteria

Your validator is working correctly when:

- ✅ Service status is "active (running)"
- ✅ Logs show correct chain name
- ✅ Connected to 3+ peers (mainnet) or 1+ (testnet)
- ✅ Importing blocks (`best: #XXXX` increasing)
- ✅ No errors in logs
- ✅ Resource usage is stable

---

## Document History

**November 2, 2025:**
- Added HOW_TO_RUN_A_VALIDATOR.md (complete public guide)
- Updated README_BOOTSTRAP.md for public use
- Removed internal deployment references
- All configurations tested on live validators

**Previous Updates:**
- Added VALIDATOR_SERVICE_TEMPLATE.md
- Added chainspec-mainnet-raw.json
- Documented --public-addr flag requirement

---

## Files Checksum Reference

### chainspec-mainnet-raw.json
```bash
# Verify file integrity
sha256sum docs/mainnet/chainspec-mainnet-raw.json

# Or simply check the header:
head -5 docs/mainnet/chainspec-mainnet-raw.json
# Should show: "name": "Ëtrid FlareChain Mainnet"
```

---

## Additional Resources

- **Main Repository:** https://github.com/EojEdred/Etrid
- **Documentation:** `docs/` directory
- **Release Packages:** `release-packages/` directory
- **Scripts:** `scripts/` directory

---

**Maintained By:** Ëtrid Development Team
**Repository:** https://github.com/EojEdred/Etrid
**License:** See LICENSE file
**Status:** ✅ Production Ready
