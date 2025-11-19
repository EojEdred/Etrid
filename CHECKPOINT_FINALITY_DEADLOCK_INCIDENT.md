# Checkpoint BFT Finality Deadlock Incident Report

**Date**: November 19, 2025
**Severity**: CRITICAL - Chain Halt
**Duration**: ~8 minutes (13:33 - 13:41 CET)
**Status**: RESOLVED

## Executive Summary

The checkpoint BFT finality implementation caused a **complete chain halt** due to a deadlock in the GRANDPA finality gadget. All 20 deployed validators experienced the same deadlock, stopping block production at #106190. An emergency rollback restored chain operation.

## Timeline

| Time (CET) | Event |
|------------|-------|
| 12:59 | Initial deployment of checkpoint finality to some validators |
| 13:33:23 | First "Finality gadget locked for >3s" warning appears |
| 13:33:29 | **Last block produced (#106190) - CHAIN HALTS** |
| 13:33:29+ | Repeated deadlock warnings, no new blocks |
| 13:39-13:40 | Emergency rollback executed on all 20 validators |
| 13:41 | **Chain resumes** - blocks #106195, #106196, #106197 produced |
| 13:41+ | Normal operation restored |

## Root Cause

**The checkpoint BFT finality gadget is causing a deadlock with GRANDPA finality.**

### Evidence from Logs

```
Nov 19 13:33:23 flarechain-node[2448143]: ⚠️ Finality gadget locked for >3s - skipping block #106182 (possible deadlock)
Nov 19 13:33:26 flarechain-node[2448143]: ⚠️ Finality gadget locked for >3s - skipping block #106182 (possible deadlock)
Nov 19 13:33:29 flarechain-node[2448143]: ⚠️ Finality gadget locked for >3s - skipping block #106182 (possible deadlock)
Nov 19 13:33:32 flarechain-node[2448143]: ⚠️ Finality gadget locked for >3s - skipping block #106182 (possible deadlock)
... (continued repeatedly)
```

### Technical Analysis

1. **Concurrent Finality Gadgets**: Both GRANDPA and Checkpoint BFT are attempting to finalize blocks
2. **Lock Contention**: The checkpoint finality code is holding locks that GRANDPA needs
3. **Cascading Failure**: GRANDPA cannot finalize → AURA cannot author blocks → Chain halts
4. **All Validators Affected**: Every validator with the new binary experienced the same deadlock

## Impact

- **Block Production**: Stopped completely at #106190 for ~8 minutes
- **Network**: 20/22 validators affected (91%)
- **Downtime**: ~8 minutes of complete chain halt
- **Data Loss**: None - chain resumed from last finalized block
- **User Impact**: All transactions stopped during incident

## Resolution

### Emergency Rollback

Executed rollback script to restore old binaries on all 20 validators:
- 16 Contabo validators (100.95.0.72, 100.86.111.37, etc.)
- 4 Oracle Directors (100.102.128.51, 100.71.242.104, 100.74.84.28, 100.89.102.75)

### Recovery Verification

- ✅ Block production resumed at #106195
- ✅ GRANDPA finalization working
- ✅ AURA authoring active
- ✅ No deadlock warnings
- ✅ 18 peers connected
- ✅ Chain progressing normally

## Critical Bug to Fix

**Location**: Checkpoint BFT finality implementation
**File**: `05-multichain/flare-chain/consensus/etrid-finality-gadget/src/lib.rs` (and related files)

### Problem

The checkpoint finality gadget is holding locks for >3 seconds, causing GRANDPA to detect a deadlock. This suggests:

1. **Blocking I/O in critical section** - The checkpoint code may be doing network I/O or database writes while holding finality locks
2. **Missing async/await** - Synchronous operations in async context
3. **Lock ordering issue** - Checkpoint and GRANDPA acquiring locks in different order
4. **Shared state contention** - Both gadgets competing for the same blockchain state locks

### Investigation Needed

```rust
// Check these areas in etrid-finality-gadget:
1. Lock acquisition in `run_checkpoint_worker()`
2. Shared state access in `CheckpointFinalityGadget`
3. Integration with GRANDPA in `flarechain-node/src/service.rs`
4. Finality notifications handling
```

### Potential Fix Approaches

#### Option 1: Decouple Finality Mechanisms
```rust
// Don't hold locks while waiting for checkpoint signatures
// Use channels for async communication between GRANDPA and Checkpoint
```

#### Option 2: Checkpoint-Only Finality
```rust
// Disable GRANDPA when checkpoint finality is active
// Use only one finality mechanism at a time
```

#### Option 3: Non-Blocking Checkpoint Collection
```rust
// Collect checkpoint signatures asynchronously
// Don't block finality while waiting for quorum
```

## Rollback Artifacts

### Scripts Created
- `/tmp/emergency_rollback.sh` - Emergency rollback script
- `/tmp/emergency_rollback.log` - Rollback execution log

### Binary Backups
All validators have backup binaries:
- `/usr/local/bin/flarechain-node.backup` - Pre-checkpoint binary
- `/usr/local/bin/flarechain-node.backup-pre-fix` - Directors backup
- `/usr/local/bin/flarechain-node` - Current (rolled back) binary

## Lessons Learned

1. **Testnet Testing Insufficient**: The checkpoint finality was not adequately tested with real network load
2. **Gradual Rollout Critical**: Should have deployed to 1-2 validators first, monitored for 1 hour
3. **Deadlock Detection**: Need better monitoring for finality gadget lock times
4. **Rollback Plan Essential**: Having backup binaries saved hours of recovery time

## Next Steps

### Before Redeployment

- [ ] Fix the deadlock bug in checkpoint finality implementation
- [ ] Add lock timeout monitoring and alerts
- [ ] Test checkpoint + GRANDPA interaction on testnet for 24 hours
- [ ] Deploy to 1 validator, monitor for 1 hour
- [ ] Deploy to 3 validators, monitor for 2 hours
- [ ] If stable, proceed with full deployment

### Monitoring Improvements

- [ ] Add Prometheus metric: `grandpa_lock_hold_time_seconds`
- [ ] Add alert: Finality gadget locked >1s
- [ ] Add alert: No new blocks for >30 seconds
- [ ] Dashboard: Finality gadget status

### Code Review Checklist

- [ ] Review all lock acquisitions in checkpoint finality
- [ ] Verify no blocking I/O in critical sections
- [ ] Check lock ordering between GRANDPA and Checkpoint
- [ ] Add unit tests for concurrent finality
- [ ] Add integration test: GRANDPA + Checkpoint running together

## Validators Affected

### Contabo Validators (16)
1. 100.95.0.72 (Validator-6)
2. 100.86.111.37 (Validator-7)
3. 100.125.147.88 (Validator-8)
4. 100.80.84.82 (Validator-9)
5. 100.109.252.56 (Validator-10)
6. 100.117.43.53 (Validator-11)
7. 100.88.104.58 (Validator-12)
8. 100.70.73.10 (Validator-13)
9. 100.68.185.50 (Validator-14)
10. 100.71.127.127 (Validator-15)
11. 100.93.43.18 (Validator-16)
12. 100.124.117.73 (Validator-17)
13. 100.74.204.23 (Validator-18)
14. 100.125.251.60 (Validator-19)
15. 100.114.244.62 (Validator-20)
16. 100.113.226.111 (Validator-21)

### Oracle Directors (4)
1. 100.102.128.51 (Director-3)
2. 100.71.242.104 (Director-4)
3. 100.74.84.28 (Director-5)
4. 100.89.102.75 (Director-6)

### Not Deployed (2)
1. 100.122.19.7 (Gizzi) - SSH timeout
2. 100.126.54.89 (AuditDev) - SSH timeout

## References

- Incident log: `/tmp/checkpoint-fix-deployment.log`
- Rollback log: `/tmp/emergency_rollback.log`
- Validator logs: `ssh root@100.95.0.72 'journalctl -u flarechain-validator --since "13:33:20" --until "13:34:00"'`

---

**Report compiled by**: Claude Code
**Last updated**: 2025-11-19 13:42 CET
