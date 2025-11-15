# Phase 4: GRANDPA Removal - Quick Start Guide

**For**: Eoj / ËTRID Development Team
**Date**: 2025-11-15
**Status**: Ready for Execution

## Quick Navigation

| Task | File | Location |
|------|------|----------|
| See what changes | `PHASE4_LIB_RS_CHANGES.md` | asf-algorithm/ |
| See dependencies to remove | `PHASE4_CARGO_CHANGES.md` | asf-algorithm/ |
| Complete overview | `PHASE4_COMPLETE_SUMMARY.md` | asf-algorithm/ |
| Rollback plan | `PHASE4_ROLLBACK_PLAN.md` | asf-algorithm/ |
| Delivery report | `PHASE4_DELIVERY_REPORT.md` | asf-algorithm/ |
| **Run migration** | `migrate_to_asf_only.sh` | asf-algorithm/ |
| **Run rollback** | `rollback_to_v106.sh` | asf-algorithm/ |

## TL;DR - What Is This?

Phase 4 removes GRANDPA finality from ËTRID FlareChain, transitioning to pure ASF consensus.

**Current**: Runtime v106 with GRANDPA + ASF
**After**: Runtime v108 with ASF only

## Files Created (8 Total)

1. `PHASE4_LIB_RS_CHANGES.md` - Runtime code changes to make
2. `PHASE4_CARGO_CHANGES.md` - Dependencies to remove
3. `PHASE4_ROLLBACK_PLAN.md` - How to rollback if needed
4. `PHASE4_COMPLETE_SUMMARY.md` - Full Phase 4 overview
5. `PHASE4_DELIVERY_REPORT.md` - Complete delivery report
6. `migrate_to_asf_only.sh` - Automated migration script
7. `rollback_to_v106.sh` - Automated rollback script
8. `mainnet_asf_only.json` - ASF-only genesis config

## 30-Second Overview

**What Gets Removed**:
- All GRANDPA code (~150 lines)
- 6 GRANDPA dependencies
- GRANDPA from SessionKeys
- GRANDPA finality gadget

**What Stays**:
- ASF consensus (Adaptive Scale of Finality)
- All validators (21 nodes)
- All account balances
- All pallets except GRANDPA

**Impact**:
- Runtime version: 106 → 108
- Binary size: -2 to -3 MB
- Breaking change: YES (all validators must upgrade)

## 2-Minute Execution

### Option 1: Automated Migration (Recommended)

```bash
cd /Users/macbook/Desktop/etrid/09-consensus/asf-algorithm
./migrate_to_asf_only.sh
```

This will:
1. Check you're on v106
2. Backup all files (timestamped)
3. Remove GRANDPA automatically
4. Update to v108
5. Verify changes
6. Optionally build & test

### Option 2: Manual Changes

If you prefer manual control, follow:
1. Read: `PHASE4_LIB_RS_CHANGES.md`
2. Read: `PHASE4_CARGO_CHANGES.md`
3. Apply changes carefully
4. Verify with verification commands

## Rollback (If Needed)

```bash
cd /Users/macbook/Desktop/etrid/09-consensus/asf-algorithm
./rollback_to_v106.sh
```

This will:
1. Find latest v106 backup
2. Restore all files
3. Rebuild runtime & node
4. Verify v106 restored

## Verification

### Before Migration
```bash
grep "spec_version:" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs | head -1
# Should show: spec_version: 106

grep -c "grandpa" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml
# Should show: 2 (or more)
```

### After Migration
```bash
grep "spec_version:" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs | head -1
# Should show: spec_version: 108

grep -c "grandpa" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml
# Should show: 0
```

## What Changed?

### Runtime (lib.rs)
- Removed GRANDPA import
- Updated SessionKeys: `{grandpa}` → `{asf}`
- Bumped version: 106 → 108
- Removed ~150 lines of GRANDPA code

### Runtime Cargo.toml
- Removed: `pallet-grandpa`
- Removed: `sp-consensus-grandpa`
- Removed: 2 std feature flags

### Node Cargo.toml
- Removed: `sc-consensus-grandpa`
- Removed: `sp-consensus-grandpa`

## Genesis Configuration

New file: `mainnet_asf_only.json`
- Location: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/presets/`
- Validators: 21 (same as current mainnet)
- NO grandpa section
- ASF configuration included
- ValidatorCommittee configured

**Verified**:
- ✓ Valid JSON format
- ✓ 21 validators present
- ✓ 0 GRANDPA references
- ✓ All balances preserved

## Build & Deploy

After migration:

```bash
# Build runtime
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
cargo build --release

# Build node
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
cargo build --release

# Generate chainspec
./target/release/flarechain-node build-spec \
  --chain mainnet_asf_only \
  --raw > chainspec-v108-asf-only.json

# Test locally
./target/release/flarechain-node --dev --tmp
```

## Common Questions

**Q: Is this a breaking change?**
A: Yes. All validators must upgrade to v108 simultaneously.

**Q: Can we rollback if needed?**
A: Yes. Run `./rollback_to_v106.sh` to restore GRANDPA.

**Q: What happens to GRANDPA keys?**
A: They're no longer needed. Validators only need ASF keys.

**Q: Does this affect existing balances?**
A: No. All account balances are preserved.

**Q: How long does migration take?**
A: Automated script: ~2 minutes. Build time: ~10-20 minutes.

**Q: What if migration fails?**
A: Run rollback script. All files are backed up with timestamps.

## Support

**Documentation**:
- Complete overview: `PHASE4_COMPLETE_SUMMARY.md`
- Rollback guide: `PHASE4_ROLLBACK_PLAN.md`
- Delivery report: `PHASE4_DELIVERY_REPORT.md`

**Scripts**:
- Migrate: `./migrate_to_asf_only.sh`
- Rollback: `./rollback_to_v106.sh`

## Checklist

Pre-Migration:
- [ ] Read `PHASE4_COMPLETE_SUMMARY.md`
- [ ] Test on local devnet
- [ ] Verify backups working
- [ ] Notify validators

Migration:
- [ ] Run `./migrate_to_asf_only.sh`
- [ ] Verify spec_version = 108
- [ ] Build runtime successfully
- [ ] Build node successfully

Post-Migration:
- [ ] Test chainspec generation
- [ ] Test node startup
- [ ] Verify ASF consensus working
- [ ] Monitor for 24 hours

## One-Line Summary

**Phase 4 removes GRANDPA, transitions to pure ASF consensus, v106 → v108.**

---

**Ready to proceed? Start with:**
```bash
cd /Users/macbook/Desktop/etrid/09-consensus/asf-algorithm
./migrate_to_asf_only.sh
```
