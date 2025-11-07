# ËTRID DEVNET Test Keys Deployment - Complete

## Executive Summary

Complete toolkit created for configuring Substrate standard test keys on all 16 DEVNET nodes and restarting them with proper validator account configuration.

**Problem Solved:** Nodes were crashing due to missing/incorrect validator keys. Solution uses built-in Substrate development accounts (alice, bob, charlie, dave, eve, ferdie) which are standard for DEVNET testing.

**Status:** READY FOR DEPLOYMENT

---

## Deliverables

### 1. Main Deployment Script
**File:** `/Users/macbook/Desktop/etrid/deploy-devnet-test-keys.sh` (14KB)

Deploys test keys to all 16 nodes in parallel:
- Kills existing node processes
- Purges chain databases
- Creates systemd services with proper dev account flags
- Starts all 16 nodes
- Verifies nodes are running and producing blocks
- Generates deployment report with test key mapping

**Run:** `./deploy-devnet-test-keys.sh`

**Duration:** 3-5 minutes

### 2. Verification Script
**File:** `/Users/macbook/Desktop/etrid/verify-devnet-nodes.sh` (11KB)

Checks node status and performs node-specific operations:
- Check status of all nodes
- Monitor continuously (watch mode)
- View logs for specific nodes
- Restart specific nodes
- Reset/purge specific nodes
- Verify consensus health (block height variance)

**Run:** `./verify-devnet-nodes.sh [status|logs|restart|reset|watch]`

### 3. Management Script
**File:** `/Users/macbook/Desktop/etrid/manage-devnet-nodes.sh` (6.7KB)

Bulk operations on all nodes:
- Start/stop all nodes
- Restart all nodes (sequential or parallel)
- Purge all data
- Check disk usage
- SSH to individual nodes
- Tail logs for individual nodes

**Run:** `./manage-devnet-nodes.sh [start|stop|restart|restart-fast|status|purge|disk|logs|ssh]`

### 4. Comprehensive Documentation
**Files:** 
- `DEVNET_DEPLOYMENT_GUIDE.md` - Detailed deployment guide with troubleshooting
- `DEVNET_TOOLS_README.md` - Complete toolkit documentation
- `DEVNET_DEPLOYMENT_SUMMARY.md` - This file

---

## Quick Start

### Step 1: Initial Deployment (3-5 minutes)
```bash
cd /Users/macbook/Desktop/etrid
./deploy-devnet-test-keys.sh
```

**Expected Output:**
```
═══════════════════════════════════════════
ËTRID DEVNET TEST KEYS DEPLOYMENT
16-Node Substrate Dev Account Configuration
═══════════════════════════════════════════

[INFO] Checking SSH key...
[✓] SSH key found
[INFO] [Node 0] Starting deployment for Node0-Alice-Primary...
[✓] [Node 0] Deployment completed
[✓] [Node 1] Deployment completed
...
[✓] [Node 15] Deployment completed

DEPLOYMENT SUMMARY
─────────────────────────────────────────
Total Deployed:  16/16
Total Failed:    0/16

✓ DEVNET test key configuration complete!
```

### Step 2: Wait & Verify (2 minutes)
```bash
sleep 60
./verify-devnet-nodes.sh status
```

**Expected Output:**
```
═══════════════════════════════════════════
CHECKING DEVNET NODE STATUS
═══════════════════════════════════════════

✓ Node 0 (alice) - RUNNING - Block: 42
✓ Node 1 (bob) - RUNNING - Block: 41
✓ Node 2 (charlie) - RUNNING - Block: 42
...
Running: 16/16
Stopped: 0/16
Unreachable: 0/16

✓ Consensus healthy - block height variance: 1 blocks
```

### Step 3: Monitor (Continuous)
```bash
./verify-devnet-nodes.sh watch
```

This will update every 10 seconds showing status and block heights.

---

## Node Configuration

### 16 Nodes with Assigned Accounts

| Node | SSH Target | Account | Purpose |
|------|-----------|---------|---------|
| 0 | multichain-dev01@68.219.230.63 | alice | Primary Multichain |
| 1 | compiler-dev01@98.71.91.84 | bob | Compiler Monitoring |
| 2 | compiler-dev01@4.180.59.25 | charlie | Compiler Primary |
| 3 | consensus-dev01@20.224.104.239 | dave | Consensus Dev |
| 4 | multichain-dev01@98.71.219.106 | eve | Multichain Secondary |
| 5 | runtime-dev01@108.142.205.177 | ferdie | Runtime Primary |
| 6 | runtime-dev01@4.180.238.67 | alice | Runtime Secondary |
| 7 | audit-dev01@51.142.203.160 | bob | Audit Dev |
| 8 | flarenode15@172.166.164.19 | charlie | Economics Primary |
| 9 | flarenode16@172.166.187.180 | dave | Economics Secondary |
| 10 | flarenode17@172.166.210.244 | eve | Ethics Primary |
| 11 | oracle-dev01@172.167.8.217 | ferdie | Oracle Dev |
| 12 | flarenode18@4.251.115.186 | alice | Ethics Secondary |
| 13 | flarenode19@52.143.191.232 | bob | Docs Primary |
| 14 | flarenode20@4.211.206.210 | charlie | Docs Secondary |
| 15 | flarenode21@4.178.181.122 | dave | Docs Tertiary |

---

## Substrate Development Accounts

Each node uses one of these standard Substrate development accounts:

```
Account   Description
────────  ──────────────────────────────────
alice     Primary validator account
bob       Secondary validator account
charlie   Tertiary validator account
dave      Quaternary validator account
eve       Quinary validator account
ferdie    Senary validator account
```

These are well-known development accounts included in Substrate with standard seed phrases.

---

## System Configuration

### Systemd Service
- **Service Name:** `flarechain-devnet`
- **Service File:** `/etc/systemd/system/flarechain-devnet.service`
- **Binary Path:** `/opt/flarechain/flarechain-node`
- **Data Directory:** `/data/flarechain`

### Network Ports
- **P2P Port:** 30333 (peer-to-peer networking)
- **RPC Port:** 9944 (JSON-RPC endpoint)
- **WebSocket:** 9945 (WebSocket endpoint)
- **Prometheus:** 9615 (metrics endpoint)

### Key Startup Flags
```
--dev                           # Development mode
--alice/--bob/--charlie/etc.   # Account flag
--base-path /data/flarechain   # Data directory
--name node<N>                 # Node name
--rpc-external                 # Allow external RPC
--rpc-cors all                 # Allow all origins
--prometheus-external          # Allow external metrics
```

---

## Testing & Validation

### Success Criteria (After Deployment)

1. **All nodes running**
   ```bash
   ./verify-devnet-nodes.sh status
   # Should show all 16 nodes as "RUNNING"
   ```

2. **Block production active**
   ```bash
   # Block numbers increase every 6-12 seconds
   # Run twice with 15 second gap to verify
   ./verify-devnet-nodes.sh status
   sleep 15
   ./verify-devnet-nodes.sh status
   ```

3. **Consensus synchronized**
   ```bash
   # All block heights within 5 blocks of each other
   # Status output shows "Consensus healthy"
   ./verify-devnet-nodes.sh status
   ```

4. **RPC endpoints responsive**
   ```bash
   curl -s http://68.219.230.63:9944 \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
     | jq '.result.number'
   ```

5. **No errors in logs**
   ```bash
   ./verify-devnet-nodes.sh logs 0
   # Should show block production, no ERROR entries
   ```

---

## Common Operations

### Monitor All Nodes (Continuous)
```bash
./verify-devnet-nodes.sh watch
```

### Check Status (Single Query)
```bash
./verify-devnet-nodes.sh status
```

### View Logs for Node 0
```bash
./verify-devnet-nodes.sh logs 0
```

### Restart Node 3
```bash
./verify-devnet-nodes.sh restart 3
```

### Reset Node 5 (Purge + Restart)
```bash
./verify-devnet-nodes.sh reset 5
```

### Restart All Nodes (Parallel)
```bash
./manage-devnet-nodes.sh restart-fast
```

### Stop All Nodes
```bash
./manage-devnet-nodes.sh stop
```

### Start All Nodes
```bash
./manage-devnet-nodes.sh start
```

### SSH to Node 3
```bash
./manage-devnet-nodes.sh ssh 3
```

### Check Disk Usage
```bash
./manage-devnet-nodes.sh disk
```

---

## Troubleshooting

### Issue: SSH Connection Timeouts
**Solution:** Check SSH key and firewall
```bash
# Verify SSH key exists
ls -la ~/.ssh/etrid_vm1

# Test SSH manually
ssh -i ~/.ssh/etrid_vm1 -v multichain-dev01@68.219.230.63
```

### Issue: Nodes Not Starting
**Solution:** Check logs and reset if needed
```bash
./verify-devnet-nodes.sh logs 0
./verify-devnet-nodes.sh reset 0
sleep 60
./verify-devnet-nodes.sh status
```

### Issue: Nodes Not Syncing
**Solution:** Wait longer or reset slowest node
```bash
sleep 120  # Wait 2 minutes
./verify-devnet-nodes.sh status

# If still diverged, reset slowest
./verify-devnet-nodes.sh reset 5
```

### Issue: RPC Not Responding
**Solution:** Restart the node
```bash
./verify-devnet-nodes.sh restart 0
sleep 10
./verify-devnet-nodes.sh logs 0
```

---

## Performance Expectations

### Block Production
- Block time: ~6 seconds per block
- Block finalization: ~6 seconds (1 block)
- Full consensus: ~12 seconds per block

### Network Metrics
- P2P discovery: ~5-10 seconds
- Block propagation: ~1-2 seconds
- RPC response: <100ms

### Resource Usage
- Memory per node: 500MB-1GB
- CPU per node: <10% during normal operation
- Disk per node: ~5GB per 1000 blocks

---

## File Locations

### Deployment Tools
```
/Users/macbook/Desktop/etrid/deploy-devnet-test-keys.sh     (Main deployment)
/Users/macbook/Desktop/etrid/verify-devnet-nodes.sh          (Verification)
/Users/macbook/Desktop/etrid/manage-devnet-nodes.sh          (Management)
```

### Documentation
```
/Users/macbook/Desktop/etrid/DEVNET_DEPLOYMENT_GUIDE.md      (Detailed guide)
/Users/macbook/Desktop/etrid/DEVNET_TOOLS_README.md          (Toolkit docs)
/Users/macbook/Desktop/etrid/DEVNET_DEPLOYMENT_SUMMARY.md    (This file)
```

### SSH Key
```
~/.ssh/etrid_vm1  (Used for all 16 nodes)
```

### Node Configuration (on each node)
```
Binary:              /opt/flarechain/flarechain-node
Data:               /data/flarechain
Systemd Service:    /etc/systemd/system/flarechain-devnet.service
Logs:               journalctl -u flarechain-devnet
```

---

## Environment Variables

### SSH Key Location
```bash
export SSH_KEY=/path/to/custom/key
./deploy-devnet-test-keys.sh
```

### Connection Timeout
```bash
export TIMEOUT=60
./verify-devnet-nodes.sh status
```

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                  Deployment Control Machine                  │
│              (Your Local macOS Development PC)               │
│  - deploy-devnet-test-keys.sh  (Main deployment script)     │
│  - verify-devnet-nodes.sh      (Status verification)        │
│  - manage-devnet-nodes.sh      (Bulk operations)            │
│  - SSH Key: ~/.ssh/etrid_vm1   (Authentication)             │
└────────────────┬────────────────────────────────────────────┘
                 │
    ┌────────────┼────────────────────────────────┐
    │            │                                │
    ▼            ▼                                ▼
┌────────────┐ ┌────────────┐                ┌─────────────┐
│   Azure    │ │   Azure    │  ┌─────────┐  │  Flarenode  │
│ (Public)   │ │ (Public)   │  │ Oracle  │  │  (Private)  │
│ Nodes 0-7  │ │ Nodes 1-7  │  │Dev Node │  │ Nodes 8-15  │
└────────────┘ └────────────┘  └─────────┘  └─────────────┘
     │              │               │              │
     │              │               │              │
     └──────────────┴───────────────┴──────────────┘
              │
              ▼
    ┌──────────────────────────┐
    │  Flarechain DEVNET       │
    │  16-Node Test Network    │
    │  - Consensus Running     │
    │  - Blocks Finalizing     │
    │  - RPC Available         │
    │  - Metrics Exposed       │
    └──────────────────────────┘
```

---

## Next Steps

1. **Deploy:** Run `./deploy-devnet-test-keys.sh`
2. **Wait:** 60 seconds for startup
3. **Verify:** Run `./verify-devnet-nodes.sh status`
4. **Monitor:** Run `./verify-devnet-nodes.sh watch`
5. **Test:** Query RPC endpoints or run test transactions
6. **Document:** Note any custom configurations or observations

---

## Support Resources

- **Substrate Documentation:** https://docs.substrate.io/
- **Polkadot.js:** https://polkadot.js.org/
- **RPC Methods:** See Substrate RPC spec
- **Logs Location:** `journalctl -u flarechain-devnet -f` on each node

---

## Deployment Timeline

| Phase | Duration | Task |
|-------|----------|------|
| Pre-deploy | 1 min | Review configuration |
| Deployment | 3-5 min | Run deployment script |
| Startup | 2 min | Nodes starting |
| Verification | 1 min | Check status |
| Stabilization | 2 min | Consensus establishing |
| **Total** | **~10 min** | **Ready for use** |

---

## Rollback Procedure

If issues occur:

1. **Stop all nodes:**
   ```bash
   ./manage-devnet-nodes.sh stop
   ```

2. **Reset all data:**
   ```bash
   ./manage-devnet-nodes.sh purge
   ```

3. **Redeploy from scratch:**
   ```bash
   ./deploy-devnet-test-keys.sh
   ```

---

## Success Indicators

After successful deployment, you should see:

✓ All 16 nodes showing "RUNNING" status
✓ Block heights increasing every 6-12 seconds
✓ All nodes within 5 blocks of each other
✓ RPC endpoints responding to queries
✓ No ERROR entries in logs
✓ Prometheus metrics available on port 9615
✓ Peer connections established between nodes

---

## Notes

- **Development Mode:** All nodes run in `--dev` mode (no real consensus needed)
- **Account Cycling:** Accounts are assigned and may cycle (alice → bob → alice)
- **No Staking Required:** Development mode doesn't require actual staking
- **Reset Safe:** Can reset individual nodes without affecting others
- **Parallel Deploy:** All 16 nodes deployed simultaneously for speed

---

**Created:** November 1, 2025
**Status:** READY FOR DEPLOYMENT
**Scripts Location:** `/Users/macbook/Desktop/etrid/`
**Documentation:** Complete (3 files)

