# Oracle Cloud VMs - Current Status & Handoff

**Date:** November 3, 2025
**Session Summary:** Both Oracle Cloud VMs prepared for Ëtrid FlareChain Mainnet Bootstrap

---

## SSH Access Details

### VM 1: Gizzi Validator
- **IP:** 64.181.215.19
- **User:** ubuntu
- **SSH Key:** `~/.ssh/gizzi-validator`
- **SSH Command:**
  ```bash
  ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
  ```

### VM 2: RPC Node
- **IP:** 163.192.125.23
- **User:** ubuntu
- **SSH Key:** `~/.ssh/gizzi-validator` (same key works for both)
- **SSH Command:**
  ```bash
  ssh -i ~/.ssh/gizzi-validator ubuntu@163.192.125.23
  ```

### SSH Config Entry
Both VMs are configured in `~/.ssh/config`:
```
Host gizzi-validator
    HostName 64.181.215.19
    User ubuntu
    IdentityFile ~/.ssh/gizzi-validator
    StrictHostKeyChecking no
    ServerAliveInterval 60
    ServerAliveCountMax 3
```

---

## Current Status (Both VMs)

### ✅ Completed Tasks

1. **Latest Code Deployed**
   - Commit: `f328b1c2` - "fix: Add mainnet_config() and deprecate outdated flarechain_config()"
   - Binary version: `etrid 0.1.0`
   - Location: `/usr/local/bin/etrid`
   - Size: 75MB (Linux x86-64 ELF)

2. **Chain Data Purged**
   - `/var/lib/etrid/chains/` - Empty (4KB)
   - Fresh start ready for mainnet genesis

3. **Session Keys Pre-Loaded** (Gizzi Validator ONLY)
   - Location: `/var/lib/etrid/chains/flarechain_mainnet/keystore/`
   - Keys stored: 3 (AURA, GRANDPA, ASFK)
   - Mnemonic: "ill easily diesel mixture urge gauge health kitchen brother uniform come equip"
   - ⚠️ **RPC node does NOT have keys (not a validator)**

4. **Services**
   - Gizzi Validator: `etrid-validator.service` - **STOPPED**
   - RPC Node: No systemd service / **STOPPED**

### 📦 Binary Details

```bash
# Verify binary on both VMs:
/usr/local/bin/etrid --version
# Output: etrid 0.1.0

# Check binary type:
file /usr/local/bin/etrid
# Output: ELF 64-bit LSB pie executable, x86-64

# Source location (Gizzi only):
/home/ubuntu/etrid/target/release/etrid
```

### 🔑 Keystore Verification (Gizzi Validator)

```bash
# Check keys:
ls -la /var/lib/etrid/chains/flarechain_mainnet/keystore/
# Should show 3 files:
# - 6173666b... (asfk - ASF Consensus key)
# - 6175726... (aura - Block production key)
# - 6772616... (gran - GRANDPA finality key)
```

---

## What We're Waiting On

### 🚀 NEXT STEP: Bootstrap to Eoj

**Current State:**
- ✅ Both VMs ready with latest binary (commit f328b1c2)
- ✅ Chain data purged clean
- ✅ Session keys pre-loaded on Gizzi validator
- ✅ Both nodes stopped, awaiting start command

**Waiting for:** Eoj's command to start bootstrap process

**What Bootstrap Means:**
1. Start the Gizzi validator node FIRST (will produce genesis block)
2. Once Gizzi is producing blocks, start RPC node to sync
3. **After nodes running:** Insert session keys via RPC call (this makes Gizzi become an active validator)

---

## Critical Files & Locations

### Chainspec Files
```bash
# Raw chainspec (both VMs):
/home/ubuntu/chainspec-mainnet-raw.json  # 1.8MB, has 21 validators

# Also available at:
/tmp/chainspec-mainnet-raw.json

# Source repository:
/home/ubuntu/etrid/05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json
```

### Build Logs (Gizzi Validator)
```bash
/tmp/build-chainspec-fix.log  # Latest build (39.73s)
/tmp/build-latest.log
/tmp/build-etrid-asf-fix.log
```

### Repository Location
```bash
/home/ubuntu/etrid/  # Full repo clone on Gizzi validator
# Latest commit: f328b1c2
```

---

## Key Configuration Details

### Genesis Configuration
- **Chain ID:** flarechain_mainnet
- **Chain Name:** Ëtrid FlareChain Mainnet
- **Chain Type:** Live
- **Committee Size:** 21 validators
- **Slot Duration:** 6000ms (6 seconds)
- **Epoch Duration:** 2400 blocks
- **Token Decimals:** 12 (CRITICAL - was 18 in old config)

### Consensus
- **Block Production:** ASF PPFA (Adaptive Stake-weighted Finality - Probabilistic Proactive Finality Algorithm)
- **Finality:** Hybrid (ASF + GRANDPA)

### Validator Identity (Gizzi)
- **Account:** 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
- **Role:** Director (peer_type 2) - Validator #1 of 21
- **Stake:** 128,000 ETR (128000000000000000000000 in smallest units)

---

## Bootstrap Procedure (When Ready)

### Step 1: Start Gizzi Validator
```bash
# On Gizzi validator (64.181.215.19):
sudo systemctl start etrid-validator

# Check logs:
sudo journalctl -u etrid-validator -f

# Should see:
# - "✅ ASF FlareChain node started successfully"
# - "Committee Size: 21"
# - "🔨 Authored block #1" (genesis block)
```

### Step 2: Start RPC Node
```bash
# On RPC node (163.192.125.23):
/usr/local/bin/etrid \
  --chain /home/ubuntu/chainspec-mainnet-raw.json \
  --base-path /var/lib/etrid \
  --name "Etrid-RPC-Node-Mainnet" \
  --rpc-port 9944 \
  --unsafe-rpc-external \
  --rpc-cors all \
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp

# Should sync from Gizzi validator
```

### Step 3: Insert Session Keys (After Node Running)
```bash
# On local machine or Gizzi:
curl -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"author_rotateKeys",
    "params":[],
    "id":1
  }' \
  http://64.181.215.19:9944

# This will return the session keys that need to be set on-chain
```

---

## Quick Verification Commands

### Check Node Status
```bash
# Gizzi:
sudo systemctl status etrid-validator

# RPC:
ps aux | grep etrid
```

### Check Chain Data
```bash
# Both VMs:
du -sh /var/lib/etrid/chains/*
# Should show 4KB (empty) right now
```

### Check Binary Version
```bash
# Both VMs:
/usr/local/bin/etrid --version
# Output: etrid 0.1.0
```

### Check Latest Commit
```bash
# Gizzi only (has full repo):
cd /home/ubuntu/etrid && git log -1 --oneline
# Output: f328b1c2 fix: Add mainnet_config() and deprecate outdated flarechain_config()
```

---

## Important Notes

1. **⚠️ DO NOT START NODES YET** - Waiting for Eoj's bootstrap command
2. **RPC Node** does not need session keys (not a validator)
3. **Session keys are already in keystore** on Gizzi - but still need to call `author_rotateKeys` after node starts
4. **Genesis hash** will be calculated when first node starts - must match across all 21 validators
5. **Chainspec file** is identical on both VMs (1.8MB)

---

## Troubleshooting

### If SSH connection fails:
```bash
# Check key permissions:
chmod 600 ~/.ssh/gizzi-validator

# Test connection:
ssh -v -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
```

### If binary missing:
```bash
# Rebuild on Gizzi:
cd /home/ubuntu/etrid
source ~/.cargo/env
cargo build --release -p etrid
sudo cp target/release/etrid /usr/local/bin/
```

### If keys missing:
```bash
# Re-insert keys on Gizzi:
/usr/local/bin/etrid key insert \
  --base-path /var/lib/etrid \
  --chain /home/ubuntu/chainspec-mainnet-raw.json \
  --scheme Sr25519 \
  --suri "ill easily diesel mixture urge gauge health kitchen brother uniform come equip" \
  --key-type aura

# (repeat for gran and asfk)
```

---

## Contact Info

- **Project:** Ëtrid FlareChain Mainnet
- **Network:** 21-validator PoA network
- **Bootstrap Lead:** Eoj
- **Status:** Ready for bootstrap, awaiting command

---

**🎯 BOTTOM LINE:** Both Oracle Cloud VMs are fully prepared and waiting for Eoj's command to begin mainnet bootstrap. Session keys are pre-loaded, chain data is clean, and latest binary with chainspec fix is deployed.
