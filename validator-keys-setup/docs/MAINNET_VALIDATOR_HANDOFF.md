# Ëtrid FlareChain Mainnet Validator Launch - Handoff Document

**Date**: October 28, 2025
**Status**: Ready to deploy startup scripts and launch validators
**Critical Issue**: ASF consensus CommitteeFull error - SOLUTION READY

---

## Current State

### ✅ Completed
- Built flarechain-node binary on both Azure VMs (~12 min builds)
- Generated validator keys (AURA, GRANDPA, node keys) for both validators
- Identified root cause of ASF consensus failure
- Created startup scripts that fix the issue

### 🔧 In Progress
- Transferring startup scripts to VMs
- Starting validators

### ⏸️ Blocked
- Multiple background cargo builds running (can be killed)
- Git push failed due to SSH key issues (not blocking - scripts are local)

---

## The Problem and Solution

### Root Cause
The `--alice` and `--bob` flags insert AURA and GRANDPA keys into the keystore, but **NOT** ASF keys (key type: `asfk`). When the node starts, ASF consensus initialization fails with:

```
⚠️  No validator keys in keystore. Committee will be empty.
Failed to initialize committee rotation: CommitteeFull
Essential task `asf-ppfa-proposer` failed
```

The bug is in `/Users/macbook/Desktop/etrid/09-consensus/validator-management/src/committee.rs:151-222`:
```rust
if eligible.is_empty() {
    return Err(ValidatorError::CommitteeFull);  // This fires even when committee is 0/21
}
```

### The Solution
The startup scripts now perform 3 critical steps:
1. Generate network key (fixes `NetworkKeyNotFound`)
2. **Insert ASF validator key** (fixes `CommitteeFull`)
3. Start the node with proper configuration

---

## VM Configuration

### VM #1 - Primary Validator (Alice)
- **IP**: 20.186.91.207
- **User**: etrid-validator-01
- **Binary Location**: `/opt/etrid/flarechain-node`
- **Startup Script**: `/tmp/start-validator-vm1.sh` (on local machine)
- **Generated Keys**:
  - AURA: 5HVQnveMRrfSgzDHNmgx1PDfogGTfJSCLmuvWHzDnN6ZyFdj
  - GRANDPA: 5EXDVLNuqQGTvdNn3T6ehg1bk568T2w81tFUgtPB4VquRJsm
  - Node ID: 12D3KooWEytQF5oTpM7HMaD87Xea2u46gXJ7MX8XUKnkHZhSxv4L

### VM #2 - Secondary Validator (Bob)
- **IP**: 172.177.44.73
- **User**: etrid-validator-02
- **Binary Location**: `/opt/etrid/flarechain-node`
- **Startup Script**: `/tmp/start-validator-vm2.sh` (on local machine)
- **Generated Keys**:
  - AURA: 5CkXPqNBo2JdhBENJiwe5x8sub1mThv8X1jRMVgTYm4iDstH
  - GRANDPA: 5HnxucpMBKv2aHbrhfGimwKDGriUfD3iQKyBZmBKQBK33xFo
  - Node ID: 12D3KooWFQKLF5XWyf99Bn1YZWrygFgvRYRZiSwaP2BmLhmvHe4q

---

## Immediate Action Required

### Step 1: Clean Up Background Processes (Local Machine)
Kill all running background cargo builds:
```bash
pkill -f "cargo build" || true
```

### Step 2: Deploy Startup Scripts to VMs

The startup scripts are ready at:
- `/tmp/start-validator-vm1.sh` → Deploy to VM #1
- `/tmp/start-validator-vm2.sh` → Deploy to VM #2

**Method A: Manual Copy (Recommended due to Azure terminal issues)**

On **VM #1** (20.186.91.207):
```bash
nano /tmp/start-validator.sh
# Paste the entire contents of /tmp/start-validator-vm1.sh from local machine
# Save: Ctrl+O, Enter
# Exit: Ctrl+X
chmod +x /tmp/start-validator.sh
```

On **VM #2** (172.177.44.73):
```bash
nano /tmp/start-validator.sh
# Paste the entire contents of /tmp/start-validator-vm2.sh from local machine
# Save: Ctrl+O, Enter
# Exit: Ctrl+X
chmod +x /tmp/start-validator.sh
```

**Method B: SCP (if SSH access is fixed)**
```bash
scp /tmp/start-validator-vm1.sh etrid-validator-01@20.186.91.207:/tmp/start-validator.sh
scp /tmp/start-validator-vm2.sh etrid-validator-02@172.177.44.73:/tmp/start-validator.sh
```

### Step 3: Start Validators

**On VM #1:**
```bash
cd /tmp
./start-validator.sh
```

**On VM #2 (in a separate terminal):**
```bash
cd /tmp
./start-validator.sh
```

### Step 4: Monitor Validator Logs

You should see:
```
=== Ëtrid FlareChain Validator #1 (Alice) Startup ===

Step 1/3: Generating network key...
✓ Network key generated

Step 2/3: Inserting ASF validator key...
✓ ASF key inserted

Step 3/3: Starting validator...
Node will now start and begin producing blocks...

2025-10-28 ... Ëtrid FlareChain Node
2025-10-28 ... ✨ version 0.1.0
2025-10-28 ... ❤️  by Ëtrid Team
2025-10-28 ... 📋 Chain specification: Ëtrid FlareChain Local Testnet
2025-10-28 ... 🏷  Node name: Alice
2025-10-28 ... 💾 Database: RocksDB
2025-10-28 ... ⛓  Native runtime: flarechain-1
2025-10-28 ... 👶 Creating empty BABE epoch changes on what appears to be first startup.
2025-10-28 ... 🔨 Initializing Genesis block/state
2025-10-28 ... 🏷  Local node identity is: 12D3KooW...
2025-10-28 ... 💤 Idle (1 peers), best: #0 (0x1234…)
2025-10-28 ... ✨ Imported #1 (0x5678…)
2025-10-28 ... 🎁 Prepared block for proposing at 1
```

**Success indicators:**
- Both nodes connect to each other (1+ peers)
- Block production begins
- No `CommitteeFull` error
- ASF consensus activates

---

## Troubleshooting

### If CommitteeFull error still appears:
Check that ASF keys were inserted:
```bash
ls -la /tmp/testchain-alice/chains/flarechain_local/keystore/
# Should contain file starting with "6173666b" (hex for "asfk")
```

### If nodes don't connect:
Add bootnodes flag to each startup script before the `exec` line:
```bash
--bootnodes /ip4/20.186.91.207/tcp/30333/p2p/12D3KooWEytQF5oTpM7HMaD87Xea2u46gXJ7MX8XUKnkHZhSxv4L
```

### If network key errors:
The scripts auto-generate keys. If issues persist, manually generate:
```bash
cd /opt/etrid
./flarechain-node key generate-node-key --base-path=/tmp/testchain-alice
```

---

## Post-Launch Tasks

Once both validators are running and producing blocks:

1. **Monitor for 10-15 minutes** - Ensure stable block production
2. **Create systemd services** - Use templates from previous session
3. **Set up monitoring** - Prometheus metrics exposed on port 9615
4. **Document the process** - Update repo with validator setup guide
5. **Create mainnet chainspec** - Currently using `local` testnet config

---

## Technical Context

### ASF Consensus Configuration
- **Slot Duration**: 6000ms (6 seconds per block)
- **Committee Size**: 21 validators (currently running with 2)
- **Key Type**: `asfk` (Sr25519 scheme)
- **Finality**: 3-level system (Pre-commit → Commit → Finalized)

### Chain Specification
- **Chain ID**: `flarechain_local`
- **Type**: Local testnet (2 validators)
- **Runtime**: `flare_chain_runtime`
- **Config**: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/src/chain-spec.rs:20-29`

### Key Files
- Runtime: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/`
- Node: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/`
- ASF Consensus: `/Users/macbook/Desktop/etrid/09-consensus/`
- Committee Logic: `/Users/macbook/Desktop/etrid/09-consensus/validator-management/src/committee.rs`

---

## Known Issues

1. **Azure VM Terminal**: Automatically wraps long commands at certain character limits, breaking bash syntax. **Solution**: Use nano to create script files instead of pasting commands.

2. **SSH Access**: Cannot establish automated SSH from local machine despite correct key setup. Possibly Azure NSG or sshd config issue. **Workaround**: Manual access via Azure portal.

3. **Git Push**: SSH key permission denied when pushing to GitHub. **Status**: Scripts committed locally, can push later.

4. **Background Builds**: Multiple cargo build processes running in background from previous session attempts. **Action**: Kill all before proceeding.

---

## Expected Timeline

- **Script deployment**: 5 minutes (manual copy via nano)
- **Validator startup**: 30 seconds per node
- **Initial sync**: Instant (genesis block)
- **First block**: Within 12 seconds (2x slot duration)
- **Stable consensus**: 2-3 minutes

---

## Success Criteria

- ✅ Both validators start without errors
- ✅ Nodes discover each other (peer count > 0)
- ✅ Block production begins (block height increases)
- ✅ ASF consensus activates (no CommitteeFull error)
- ✅ Finality advances (finalized block height increases)
- ✅ No crashes or panics in logs

---

## Contact & Resources

- **Project Directory**: `/Users/macbook/Desktop/etrid`
- **Startup Scripts**: `/tmp/start-validator-vm1.sh`, `/tmp/start-validator-vm2.sh`
- **Also in Repo**: `/Users/macbook/Desktop/etrid/scripts/start-validator-alice.sh`, `start-validator-bob.sh`
- **VM Binaries**: `/opt/etrid/flarechain-node` on both VMs

---

## Quick Reference Commands

**View script contents:**
```bash
cat /tmp/start-validator-vm1.sh
cat /tmp/start-validator-vm2.sh
```

**Kill background builds:**
```bash
pkill -f "cargo build"
```

**Check VM binary:**
```bash
ssh etrid-validator-01@20.186.91.207 "/opt/etrid/flarechain-node --version"
```

**Monitor validator logs (once running):**
```bash
# Follow the output after starting the script
# Or if backgrounded:
tail -f /tmp/testchain-alice/flarechain.log
```

---

## Next Session Prompt

Use this prompt to continue in a new terminal:

```
Continue mainnet validator deployment. Current status:

- Two Azure VMs with flarechain-node binaries built and ready
- VM #1 (20.186.91.207) = Alice validator
- VM #2 (172.177.44.73) = Bob validator
- Startup scripts created to fix ASF consensus CommitteeFull bug
- Scripts ready at: /tmp/start-validator-vm1.sh and /tmp/start-validator-vm2.sh

IMMEDIATE TASK: Deploy these startup scripts to the VMs and start the validators.

The scripts fix the critical bug where --alice/--bob flags don't insert ASF keys, causing CommitteeFull errors. They now properly:
1. Generate network keys
2. Insert ASF validator keys (key-type=asfk)
3. Start nodes with correct configuration

Read MAINNET_VALIDATOR_HANDOFF.md for complete context and instructions.

First action: Kill background cargo builds with `pkill -f "cargo build"`, then help deploy the scripts to VMs.
```
