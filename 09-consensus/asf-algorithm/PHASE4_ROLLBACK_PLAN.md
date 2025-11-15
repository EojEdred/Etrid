# Phase 4: GRANDPA Removal - Rollback Plan

**Document Status**: READY
**Created**: 2025-11-15
**Runtime Versions**: v108 (ASF-only) → v106 (GRANDPA+ASF)

## Overview

This document provides complete rollback procedures if Phase 4 (GRANDPA removal) needs to be reverted for any reason.

## When to Rollback

Consider rolling back if:
- Runtime compilation fails after migration
- Node fails to start with v108 runtime
- Critical consensus issues appear in production
- Network fails to produce or finalize blocks
- Session key management breaks

## Backup Locations

All backups are automatically created by the migration script in:
```
/Users/macbook/Desktop/etrid/09-consensus/asf-algorithm/phase4-backups/
```

### Backup Files Created

1. `lib.rs.v106.backup-TIMESTAMP` - Runtime source before migration
2. `runtime-Cargo.toml.v106.backup-TIMESTAMP` - Runtime dependencies before migration
3. `node-Cargo.toml.v106.backup-TIMESTAMP` - Node dependencies before migration

**TIMESTAMP Format**: `YYYYMMDD-HHMMSS` (e.g., `20251115-143022`)

## Rollback Procedures

### Method 1: Automated Rollback (Recommended)

Use the automated rollback script:

```bash
#!/bin/bash
# Quick rollback to v106

BACKUP_DIR="/Users/macbook/Desktop/etrid/09-consensus/asf-algorithm/phase4-backups"
RUNTIME_DIR="/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime"
NODE_DIR="/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node"

# Find latest backups (adjust timestamp if needed)
TIMESTAMP="20251115-143022"  # Replace with your backup timestamp

# Restore files
cp "$BACKUP_DIR/lib.rs.v106.backup-$TIMESTAMP" "$RUNTIME_DIR/src/lib.rs"
cp "$BACKUP_DIR/runtime-Cargo.toml.v106.backup-$TIMESTAMP" "$RUNTIME_DIR/Cargo.toml"
cp "$BACKUP_DIR/node-Cargo.toml.v106.backup-$TIMESTAMP" "$NODE_DIR/Cargo.toml"

# Verify restoration
grep "spec_version:" "$RUNTIME_DIR/src/lib.rs" | head -1
# Should show: spec_version: 106

# Rebuild
cd "$RUNTIME_DIR"
cargo clean
cargo build --release

cd "$NODE_DIR"
cargo clean
cargo build --release

echo "Rollback complete! Runtime v106 restored with GRANDPA."
```

Save as: `/Users/macbook/Desktop/etrid/09-consensus/asf-algorithm/rollback_to_v106.sh`

### Method 2: Manual Rollback

If automated rollback fails, manually restore each change:

#### Step 1: Restore lib.rs

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src
cp /path/to/backup/lib.rs.v106.backup-TIMESTAMP lib.rs
```

Verify:
```bash
grep "spec_version:" lib.rs | head -1
# Expected: spec_version: 106
```

#### Step 2: Restore Runtime Cargo.toml

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
cp /path/to/backup/runtime-Cargo.toml.v106.backup-TIMESTAMP Cargo.toml
```

Verify GRANDPA dependencies exist:
```bash
grep "pallet-grandpa" Cargo.toml
# Expected: pallet-grandpa = { git = "https://github.com/paritytech/polkadot-sdk", ...
```

#### Step 3: Restore Node Cargo.toml

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
cp /path/to/backup/node-Cargo.toml.v106.backup-TIMESTAMP Cargo.toml
```

Verify:
```bash
grep "sc-consensus-grandpa" Cargo.toml
# Expected: sc-consensus-grandpa = { git = ...
```

#### Step 4: Clean Build

```bash
# Clean all build artifacts
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
cargo clean

cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
cargo clean

# Rebuild runtime
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
cargo build --release

# Rebuild node
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
cargo build --release
```

#### Step 5: Regenerate Chainspec

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
./target/release/flarechain-node build-spec --chain flarechain_mainnet_asf > chainspec-v106.json
```

Verify GRANDPA section exists:
```bash
grep "grandpa" chainspec-v106.json
# Should show grandpa authorities
```

### Method 3: Git Revert (If Committed)

If changes were committed to git:

```bash
cd /Users/macbook/Desktop/etrid

# Find the commit hash for Phase 4 migration
git log --oneline --grep="Phase 4" -n 5

# Revert the commit (replace HASH with actual commit hash)
git revert HASH

# Or reset to before migration (DANGEROUS - loses all changes)
git reset --hard HEAD~1  # Only if Phase 4 was the last commit
```

## Verification After Rollback

### 1. Runtime Version Check

```bash
grep "spec_version:" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs | head -1
# Expected: spec_version: 106
```

### 2. GRANDPA Dependencies Present

```bash
# Runtime Cargo.toml
grep -c "grandpa" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml
# Expected: 2 (pallet-grandpa, sp-consensus-grandpa)

# Node Cargo.toml
grep -c "grandpa" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/Cargo.toml
# Expected: 2 (sc-consensus-grandpa, sp-consensus-grandpa)
```

### 3. SessionKeys Includes GRANDPA

```bash
grep -A 3 "impl_opaque_keys!" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs
# Expected to show:
#   pub struct SessionKeys {
#       pub grandpa: Grandpa,
#   }
```

### 4. Compilation Test

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
cargo check --release
# Expected: Success

cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
cargo check --release
# Expected: Success
```

### 5. Runtime API Test

Check that GRANDPA runtime API exists:
```bash
grep -A 10 "impl sp_consensus_grandpa::GrandpaApi" /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs
# Expected: Full implementation visible
```

## Production Rollback Procedure

If rollback is needed in production (validators already upgraded to v108):

### Emergency Rollback Steps

1. **Stop All Validators**
   ```bash
   # On each validator node
   sudo systemctl stop flarechain-validator
   ```

2. **Restore v106 Binary**
   ```bash
   # Build v106 binary from backup source
   cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
   # (After restoring source files as above)
   cargo build --release

   # Copy to validator nodes
   scp target/release/flarechain-node user@validator:/usr/local/bin/
   ```

3. **Restore v106 Chainspec**
   ```bash
   # On each validator
   sudo cp /var/lib/flarechain/chainspec-v106-backup.json /var/lib/flarechain/chainspec.json
   ```

4. **Clear Database (If Incompatible)**
   ```bash
   # WARNING: This loses all v108 chain state
   sudo rm -rf /var/lib/flarechain/chains/etrid_mainnet/db
   ```

5. **Restart Validators**
   ```bash
   sudo systemctl start flarechain-validator
   ```

6. **Monitor Consensus**
   ```bash
   # Check GRANDPA is finalizing blocks
   journalctl -u flarechain-validator -f | grep -i "grandpa\|finalized"
   ```

## Data Preservation

### What Gets Preserved During Rollback

- Account balances (if database not cleared)
- Staking positions
- Governance proposals
- Treasury funds
- All on-chain data (if database not cleared)

### What Gets Lost (If Database Cleared)

- All v108 blocks produced
- Any transactions in v108 blocks
- Session changes made under v108

**IMPORTANT**: Only clear database as absolute last resort.

## Post-Rollback Actions

After successful rollback to v106:

1. **Investigate Root Cause**
   - Review build logs
   - Check error messages
   - Identify what went wrong

2. **Document Issues**
   - Create incident report
   - Note all errors encountered
   - Document recovery steps taken

3. **Fix Issues**
   - Address problems in Phase 4 changes
   - Test fixes locally
   - Prepare improved migration

4. **Plan Re-Migration**
   - Set new timeline
   - Add additional testing
   - Coordinate with validators

## Prevention Measures

To avoid needing rollback:

1. **Thorough Testing**
   - Test migration on local devnet
   - Test on public testnet (Ember)
   - Verify with multiple validators

2. **Staged Rollout**
   - Deploy to testnet first
   - Monitor for 24-48 hours
   - Deploy to mainnet only after validation

3. **Backup Everything**
   - Source code backups
   - Binary backups
   - Database snapshots
   - Chainspec backups

4. **Clear Communication**
   - Notify validators in advance
   - Provide rollback instructions
   - Maintain communication channel during migration

## Emergency Contacts

**Migration Issues**:
- Phase 4 Lead: [Contact Info]
- Runtime Engineer: [Contact Info]
- DevOps: [Contact Info]

**Rollback Authorization**:
- Technical Director: [Contact Info]
- Project Lead: [Contact Info]

## Rollback Checklist

- [ ] Identify rollback trigger (compilation fail, consensus issue, etc.)
- [ ] Notify team and validators
- [ ] Locate backup files (verify timestamp)
- [ ] Stop affected validator nodes (if production)
- [ ] Restore lib.rs from backup
- [ ] Restore runtime Cargo.toml from backup
- [ ] Restore node Cargo.toml from backup
- [ ] Verify runtime version shows 106
- [ ] Verify GRANDPA dependencies present
- [ ] Clean build artifacts
- [ ] Rebuild runtime (cargo build --release)
- [ ] Rebuild node (cargo build --release)
- [ ] Test compilation succeeded
- [ ] Regenerate chainspec (if needed)
- [ ] Deploy restored binary to validators (if production)
- [ ] Restart validator nodes
- [ ] Monitor GRANDPA finalization
- [ ] Verify network health
- [ ] Document incident
- [ ] Plan improved migration

## Conclusion

This rollback plan ensures ËTRID can safely revert Phase 4 changes if needed. Always test rollback procedures in development/testnet before production deployment.

**Remember**: Prevention is better than rollback. Thorough testing is key!
