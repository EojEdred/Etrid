# ËTRID DEVNET Substrate Test Keys Configuration Toolkit

Complete solution for deploying standard Substrate test keys on all 16 DEVNET nodes.

## The Problem

Nodes were crashing due to missing or incorrect validator key configuration. The Flarechain binary was working correctly - it was purely a key configuration issue.

## The Solution

Use Substrate's built-in standard development accounts (alice, bob, charlie, dave, eve, ferdie) which are designed specifically for DEVNET testing and development.

## What's Included

### Executable Scripts (Ready to Run)

1. **deploy-devnet-test-keys.sh** - Main deployment script
   - Deploys to all 16 nodes in parallel (3-5 minutes)
   - Kills existing processes, purges data, creates services
   - Configures each node with a Substrate dev account
   - Verifies success with block production checks

2. **verify-devnet-nodes.sh** - Status and monitoring
   - Check status of all 16 nodes
   - Continuous monitoring (watch mode)
   - View logs, restart, or reset individual nodes
   - Verify consensus health

3. **manage-devnet-nodes.sh** - Bulk operations
   - Start/stop/restart all nodes
   - Purge all data
   - Check disk usage
   - SSH to nodes and view logs

### Documentation (Comprehensive Guides)

1. **DEVNET_QUICK_START.md** - Get started in minutes
2. **DEVNET_DEPLOYMENT_GUIDE.md** - Complete procedures with troubleshooting
3. **DEVNET_TOOLS_README.md** - Full toolkit reference
4. **DEVNET_DEPLOYMENT_SUMMARY.md** - Executive overview

## Quick Start

```bash
# Step 1: Deploy to all 16 nodes (3-5 minutes)
cd /Users/macbook/Desktop/etrid
./deploy-devnet-test-keys.sh

# Step 2: Wait for startup
sleep 60

# Step 3: Verify all nodes are running
./verify-devnet-nodes.sh status
```

Expected output: All 16 nodes running with Substrate test keys!

## Common Commands

```bash
# Monitor all nodes continuously
./verify-devnet-nodes.sh watch

# Check node logs
./verify-devnet-nodes.sh logs 0

# Restart a specific node
./verify-devnet-nodes.sh restart 3

# Reset a node (purge + restart)
./verify-devnet-nodes.sh reset 5

# Restart all nodes in parallel
./manage-devnet-nodes.sh restart-fast

# SSH to a node
./manage-devnet-nodes.sh ssh 0
```

## Node Configuration

### 16 Nodes with Assigned Accounts

| Node | Account | Host | IP Address |
|------|---------|------|-----------|
| 0 | alice | multichain-dev01 | 68.219.230.63 |
| 1 | bob | compiler-dev01 | 98.71.91.84 |
| 2 | charlie | compiler-dev01 | 4.180.59.25 |
| 3 | dave | consensus-dev01 | 20.224.104.239 |
| 4 | eve | multichain-dev01 | 98.71.219.106 |
| 5 | ferdie | runtime-dev01 | 108.142.205.177 |
| 6 | alice | runtime-dev01 | 4.180.238.67 |
| 7 | bob | audit-dev01 | 51.142.203.160 |
| 8 | charlie | flarenode15 | 172.166.164.19 |
| 9 | dave | flarenode16 | 172.166.187.180 |
| 10 | eve | flarenode17 | 172.166.210.244 |
| 11 | ferdie | oracle-dev01 | 172.167.8.217 |
| 12 | alice | flarenode18 | 4.251.115.186 |
| 13 | bob | flarenode19 | 52.143.191.232 |
| 14 | charlie | flarenode20 | 4.211.206.210 |
| 15 | dave | flarenode21 | 4.178.181.122 |

### Substrate Development Accounts

- **alice** - Primary validator account
- **bob** - Secondary validator account  
- **charlie** - Tertiary validator account
- **dave** - Quaternary validator account
- **eve** - Quinary validator account
- **ferdie** - Senary validator account

These are standard Substrate accounts with well-known test keys.

## System Details

**On Each Node:**
- Service: `flarechain-devnet` (systemd)
- Binary: `/opt/flarechain/flarechain-node`
- Data: `/data/flarechain`
- Config: `/etc/systemd/system/flarechain-devnet.service`

**Network Ports:**
- P2P: 30333 (peer-to-peer networking)
- RPC: 9944 (JSON-RPC endpoint)
- WebSocket: 9945 (WebSocket)
- Prometheus: 9615 (metrics)

**Authentication:**
- SSH Key: `~/.ssh/etrid_vm1`
- Used by all deployment scripts

## Success Indicators

After deployment, you should see:

✓ All 16 nodes showing "RUNNING"
✓ Block heights increasing every 6-12 seconds
✓ All nodes within 5 blocks of each other
✓ RPC endpoints responding
✓ No ERROR entries in logs
✓ Consensus health verified

## Accessing Nodes

### RPC Queries
```bash
curl -s http://<IP>:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
  | jq '.result.number'
```

### SSH Access
```bash
ssh -i ~/.ssh/etrid_vm1 <user>@<IP>
journalctl -u flarechain-devnet -f
```

## Troubleshooting

**SSH Connection Timeout:**
- Check SSH key: `ls -la ~/.ssh/etrid_vm1`
- Test manually: `ssh -i ~/.ssh/etrid_vm1 -v user@host`
- Verify firewall rules on Azure NSG

**Nodes Not Starting:**
- View logs: `./verify-devnet-nodes.sh logs 0`
- Restart: `./verify-devnet-nodes.sh restart 0`
- Reset: `./verify-devnet-nodes.sh reset 0`

**Nodes Not Syncing:**
- Wait 2 minutes for startup
- Check status: `./verify-devnet-nodes.sh status`
- Reset slowest node: `./verify-devnet-nodes.sh reset 5`

See **DEVNET_DEPLOYMENT_GUIDE.md** for comprehensive troubleshooting.

## Documentation Index

**Start Here:** DEVNET_QUICK_START.md (5 minute overview)

**For Deployment:** DEVNET_DEPLOYMENT_GUIDE.md (step-by-step procedures)

**For Reference:** DEVNET_TOOLS_README.md (command reference)

**For Details:** DEVNET_DEPLOYMENT_SUMMARY.md (complete specification)

## Files Included

```
deploy-devnet-test-keys.sh      (14 KB) - Main deployment
verify-devnet-nodes.sh          (11 KB) - Status & monitoring
manage-devnet-nodes.sh          (6.7 KB) - Bulk operations

DEVNET_QUICK_START.md           (2.9 KB) - Quick reference
DEVNET_DEPLOYMENT_GUIDE.md      (12 KB) - Complete guide
DEVNET_TOOLS_README.md          (10 KB) - Toolkit documentation
DEVNET_DEPLOYMENT_SUMMARY.md    (14 KB) - Executive summary

README_DEVNET_TOOLKIT.md        (This file)
```

## Performance Expectations

**Block Production:**
- Block time: ~6 seconds per block
- Block finalization: ~6 seconds
- Full consensus: ~12 seconds

**Network:**
- P2P discovery: ~5-10 seconds
- Block propagation: ~1-2 seconds
- RPC response: <100ms

**Resources (per node):**
- Memory: 500MB-1GB
- CPU: <10% at 6s block time
- Disk: ~5GB per 1000 blocks

## Getting Help

1. **Quick Commands:** See DEVNET_QUICK_START.md
2. **Procedures:** See DEVNET_DEPLOYMENT_GUIDE.md  
3. **Troubleshooting:** See DEVNET_DEPLOYMENT_GUIDE.md (Troubleshooting section)
4. **View Logs:** `./verify-devnet-nodes.sh logs <node_num>`
5. **SSH Access:** `./manage-devnet-nodes.sh ssh <node_num>`

## Ready to Deploy?

```bash
cd /Users/macbook/Desktop/etrid
./deploy-devnet-test-keys.sh
```

The entire process takes 3-5 minutes. After deployment, all 16 nodes will be running with properly configured Substrate development accounts and producing blocks.

---

**Created:** November 1, 2025  
**Status:** READY FOR DEPLOYMENT  
**Location:** /Users/macbook/Desktop/etrid/
