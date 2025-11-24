# REVERT INSTRUCTIONS - Chain Spec Regeneration
**Date:** 2025-11-22 20:09 CET
**Purpose:** Instructions to revert chain spec regeneration if needed

## Current Network State (Before Regeneration)

- **Chain:** FlareChain Production Network (Pure ASF)
- **Block Height:** ~20,650
- **Finalized:** ~20,550
- **Active Validators:** 17-20 (Contabo cluster)
- **Finality Status:** Implicit finality (ASF not working - key mismatch)
- **Oracle Cloud Status:** Isolated (0 peers)

## Issue Being Fixed

ASF finality gadget not running because validator session keys in keystores don't match hardcoded genesis authority set. Chain is using implicit finality instead of proper ASF finality.

## Backup Contents

- `chainspec-mainnet-backup.json` - Current chain spec from Genesis 1
- `current-network-status.txt` - Network health snapshot
- `validator-processes-backup.txt` - Current validator process configs

## To Revert

If the new chain spec causes issues, revert with:

### 1. Stop all validators
```bash
# On each Contabo VM
systemctl stop flarechain-validator

# On Oracle Cloud VMs
sudo systemctl stop flarechain-validator
```

### 2. Restore old chain spec
```bash
# Copy backup to each validator
scp chainspec-mainnet-backup.json root@<validator-ip>:/var/lib/etrid/chainspec-mainnet.json
```

### 3. Restart validators
```bash
systemctl start flarechain-validator
```

### 4. Verify chain continues from block ~20,650

---

**IMPORTANT:** Keep this backup until new chain spec is verified stable for 24+ hours.
