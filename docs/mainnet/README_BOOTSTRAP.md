# Ëtrid FlareChain Mainnet - Bootstrap Documentation

**Last Updated:** November 2, 2025
**Status:** ✅ Production Verified (4 validators running)

---

## Overview

This directory contains the complete, tested, and verified configuration for bootstrapping Ëtrid FlareChain mainnet validators.

All configurations have been tested on 4 live validators and are known to work correctly.

---

## Critical Files

### 1. `VALIDATOR_SERVICE_TEMPLATE.md` ⭐
**Complete validator configuration templates with all critical flags**

- Systemd service templates for Azure and Oracle Cloud
- Working configurations from all 4 live validators
- Complete flag reference
- Common issues and solutions
- **START HERE** if setting up a new validator

### 2. `BOOTSTRAP_QUICK_REFERENCE.md` ⭐
**Quick step-by-step bootstrap guide**

- Copy-paste commands for rapid deployment
- Pre-filled templates
- Verification checklist
- Troubleshooting guide
- **USE THIS** for deploying validators 5-21

### 3. `chainspec-mainnet-raw.json` ⭐
**The working mainnet chainspec**

- Genesis Hash: `0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4`
- Chain: "Ëtrid FlareChain Mainnet"
- Type: "Live"
- **REQUIRED** for all validators

### 4. Other Documentation
- `VALIDATOR_QUICKSTART.md` - Original quick start guide
- `VALIDATOR_FIREWALL_RULES.md` - Detailed firewall setup
- `GENESIS_SETUP_GUIDE.md` - Genesis configuration details
- Other reference documents

---

## Quick Start for New Validator

### For Validators 5-21:

1. Read `BOOTSTRAP_QUICK_REFERENCE.md`
2. Copy `chainspec-mainnet-raw.json` to your VM
3. Follow the step-by-step commands
4. Verify with the checklist

**Time to deploy:** ~5 minutes

---

## Critical Configuration Requirements

These are **non-negotiable** and will cause failures if missed:

### 1. ✅ `--public-addr` Flag (CRITICAL)
```bash
--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333
```
**Without this:** Validator will only connect to 1 peer (the bootnode) and others cannot discover it.

### 2. ✅ Correct Chainspec
```bash
--chain=/home/azureuser/chainspec-mainnet-raw.json
```
**Must contain:**
- Name: "Ëtrid FlareChain Mainnet"
- Genesis: `0x2fb6...84c4`
- Chain Type: "Live"

### 3. ✅ Correct Bootnode
```bash
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```
**This is VM1 (EojEdred).** Do not use old peer IDs.

### 4. ✅ Unique Node Key
Each validator needs a unique 64-character hex node key.
```
Validator 5: 0000000000000000000000000000000000000000000000000000000000000005
Validator 6: 0000000000000000000000000000000000000000000000000000000000000006
...etc
```

### 5. ✅ Session Keys Inserted
Three keys required:
- AURA (sr25519) - key type: `aura`
- GRANDPA (ed25519) - key type: `gran`
- ASF (sr25519) - key type: `asfk`

---

## What Changed (Nov 2, 2025 Fixes)

### Before (Broken):
```bash
# Missing --public-addr
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "validator-name" \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --node-key 0000...0003 \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooW... \
  --port 30333

# RESULT: Only 1 peer connected ❌
```

### After (Fixed):
```bash
# Added --public-addr flag
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "validator-name" \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --node-key 0000...0003 \
  --public-addr /ip4/20.186.91.207/tcp/30333 \  # ← ADDED THIS
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooW... \
  --port 30333

# RESULT: 20 peers connected ✅
```

**This one flag fixed the entire network connectivity issue.**

---

## Verification After Bootstrap

After starting your validator, verify these within 60 seconds:

```bash
# 1. Chain name
sudo journalctl -u etrid-validator --no-pager | grep "Chain specification"
# ✅ Expected: Chain specification: Ëtrid FlareChain Mainnet

# 2. Genesis hash
sudo journalctl -u etrid-validator --no-pager | grep "finalized #0"
# ✅ Expected: finalized #0 (0x2fb6…84c4)

# 3. Peer count
sudo journalctl -u etrid-validator -n 5 | grep "Idle"
# ✅ Expected: 💤 Idle (3+ peers)

# 4. Block sync
sudo journalctl -u etrid-validator -n 10 | grep "Imported"
# ✅ Expected: Multiple "Imported #XXXX" messages
```

---

## Current Network Status

**As of November 2, 2025:**

| Validator | IP | Status | Peers | Block |
|-----------|-----|--------|-------|-------|
| VM1 (EojEdred) | 20.69.26.209 | ✅ Running | 20 | #1800+ |
| Gizzi (Foundation) | 64.181.215.19 | ✅ Running | 3 | #1800+ |
| VM2 (governance) | 20.186.91.207 | ✅ Running | 20 | #1800+ |
| VM3 (security) | 52.252.142.146 | ✅ Running | 20 | #1800+ |

**Network Health:** ✅ All validators synchronized and producing blocks

---

## Next Steps

### To Deploy Validator 5:
1. Get validator info from `secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md`
2. Follow `BOOTSTRAP_QUICK_REFERENCE.md`
3. Use node key: `0000000000000000000000000000000000000000000000000000000000000005`

### To Deploy Validators 6-21:
Repeat for each, incrementing node key:
- Validator 6: `...0006`
- Validator 7: `...0007`
- ... etc

---

## Troubleshooting

### Only 1 Peer Connected
**Cause:** Missing `--public-addr` flag
**Fix:** Add `--public-addr /ip4/YOUR_IP/tcp/30333` to service file

### Wrong Chain (Shows "Local Testnet")
**Cause:** Using wrong chainspec
**Fix:** Use `chainspec-mainnet-raw.json` from this directory

### Can't Connect to Bootnode
**Cause:** Old bootnode peer ID
**Fix:** Use `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`

### Wrong Genesis Hash
**Cause:** Wrong chainspec
**Fix:** Verify genesis is `0x2fb6...84c4`

---

## File Checksums (for verification)

### chainspec-mainnet-raw.json
```bash
# Verify you have the correct file
head -5 chainspec-mainnet-raw.json

# Should show:
# {
#   "name": "Ëtrid FlareChain Mainnet",
#   "id": "flarechain_mainnet",
#   "chainType": "Live",
#   "bootNodes": [],
```

---

## Support

If you encounter issues not covered in the documentation:

1. Check `VALIDATOR_SERVICE_TEMPLATE.md` - Common Issues section
2. Check `BOOTSTRAP_QUICK_REFERENCE.md` - Troubleshooting section
3. Verify your configuration matches the working templates exactly
4. Review logs: `sudo journalctl -u etrid-validator -n 100`

---

## Document History

**November 2, 2025:**
- Added `VALIDATOR_SERVICE_TEMPLATE.md` with complete configurations
- Added `BOOTSTRAP_QUICK_REFERENCE.md` for quick deployment
- Added `chainspec-mainnet-raw.json` (working mainnet chainspec)
- Documented `--public-addr` flag requirement
- All configurations tested on 4 live validators

**October 31, 2025:**
- Original documentation created
- Basic quickstart guide

---

## Success Criteria

Your validator is working correctly when:

- ✅ Service is active and running
- ✅ Logs show "Ëtrid FlareChain Mainnet"
- ✅ Genesis hash is `0x2fb6...84c4`
- ✅ Connected to 3+ peers
- ✅ Importing blocks (best: #XXXX increasing)
- ✅ No errors in logs

**After these checks pass, your validator is successfully bootstrapped to mainnet.**

---

**Maintained By:** Eoj (with Claude AI assistance)
**Repository:** https://github.com/EojEdred/Etrid
**Status:** ✅ Production Ready
