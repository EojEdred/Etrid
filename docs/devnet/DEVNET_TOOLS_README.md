# ËTRID DEVNET Deployment Tools

Complete toolkit for deploying and managing Substrate test keys on 16 DEVNET nodes.

## Tools Overview

### 1. `deploy-devnet-test-keys.sh` (Main Deployment)

**Purpose:** Initial deployment of test keys to all 16 nodes

**What it does:**
- Kills existing node processes on all nodes
- Purges chain databases (/data/flarechain)
- Creates systemd services with proper test account flags
- Starts all 16 nodes with Substrate development accounts
- Verifies nodes are running

**Usage:**
```bash
./deploy-devnet-test-keys.sh
```

**Duration:** 3-5 minutes

**Output:**
```
[INFO] Checking SSH key...
[✓] SSH key found
═══════════════════════════════════════════
DEPLOYING TO ALL 16 DEVNET NODES (PARALLEL)
═══════════════════════════════════════════

[INFO] [Node 0] Starting deployment...
[✓] [Node 0] Deployment completed
...

DEPLOYMENT SUMMARY
─────────────────────────────────────────
✓ Node 0 (alice) - SUCCESS
✓ Node 1 (bob) - SUCCESS
...
Total Deployed:  16/16
Total Failed:    0/16
```

### 2. `verify-devnet-nodes.sh` (Status & Verification)

**Purpose:** Check status of nodes and perform node-specific operations

**Usage:**
```bash
# Check status of all nodes
./verify-devnet-nodes.sh status

# Monitor continuously (updates every 10 seconds)
./verify-devnet-nodes.sh watch

# View logs for specific node
./verify-devnet-nodes.sh logs 0

# Restart specific node
./verify-devnet-nodes.sh restart 3

# Reset specific node (purge + restart)
./verify-devnet-nodes.sh reset 5

# Show help
./verify-devnet-nodes.sh help
```

**Expected Output (status):**
```
═══════════════════════════════════════════
CHECKING DEVNET NODE STATUS
═══════════════════════════════════════════

[✓] [Node 0] (alice) - RUNNING - Block: 42
[✓] [Node 1] (bob) - RUNNING - Block: 41
[✓] [Node 2] (charlie) - RUNNING - Block: 42
...

Node Status Overview:
─────────────────────────────────────────
✓ Node 0 - RUNNING (Block 42)
✓ Node 1 - RUNNING (Block 41)
✓ Node 2 - RUNNING (Block 42)
...
Running:     16/16
Stopped:     0/16
Unreachable: 0/16
─────────────────────────────────────────

✓ Consensus healthy - block height variance: 1 blocks
```

### 3. `manage-devnet-nodes.sh` (Bulk Operations)

**Purpose:** Perform bulk operations on all nodes

**Usage:**
```bash
# Start all nodes
./manage-devnet-nodes.sh start

# Stop all nodes
./manage-devnet-nodes.sh stop

# Restart all (sequential)
./manage-devnet-nodes.sh restart

# Restart all (parallel - faster)
./manage-devnet-nodes.sh restart-fast

# Check status
./manage-devnet-nodes.sh status

# Purge all data (DESTRUCTIVE)
./manage-devnet-nodes.sh purge

# Check disk usage
./manage-devnet-nodes.sh disk

# Tail logs for specific node
./manage-devnet-nodes.sh logs 0

# SSH to specific node
./manage-devnet-nodes.sh ssh 3
```

## Typical Workflows

### Initial Setup (First Time)

```bash
# 1. Deploy to all nodes
./deploy-devnet-test-keys.sh

# 2. Wait for startup
sleep 30

# 3. Verify all nodes running
./verify-devnet-nodes.sh status

# 4. Start monitoring
./verify-devnet-nodes.sh watch
```

**Expected Duration:** 5-10 minutes

### Daily Maintenance

```bash
# Check status
./verify-devnet-nodes.sh status

# If needed, restart a node
./verify-devnet-nodes.sh restart 3

# Monitor logs
./verify-devnet-nodes.sh logs 0
```

### Emergency Recovery (All Nodes Down)

```bash
# 1. Restart all nodes
./manage-devnet-nodes.sh restart-fast

# 2. Wait 60 seconds
sleep 60

# 3. Verify
./verify-devnet-nodes.sh status

# 4. If blocks not advancing, reset all
./manage-devnet-nodes.sh purge

# 5. Redeploy
./deploy-devnet-test-keys.sh
```

### Rolling Restart (Minimize Downtime)

```bash
# Restart nodes one at a time
for i in {0..15}; do
    echo "Restarting node $i..."
    ./verify-devnet-nodes.sh restart $i
    sleep 30
done

# Verify final state
./verify-devnet-nodes.sh status
```

### Testing Network Partition

```bash
# Stop minority partition (1 node)
./verify-devnet-nodes.sh restart 0

# Check others continue
./verify-devnet-nodes.sh status

# Monitor recovery
./verify-devnet-nodes.sh watch

# After 1 minute, Node 0 should rejoin
```

## Prerequisites

### SSH Key
- Location: `~/.ssh/etrid_vm1`
- Permissions: `0600`
- Used for all 16 nodes

### Network
- All nodes must be accessible via SSH
- Default timeout: 5 seconds per node
- Parallel execution: up to 4 nodes at once

### Node Configuration
- Binary path: `/opt/flarechain/flarechain-node`
- Data directory: `/data/flarechain`
- Service file: `/etc/systemd/system/flarechain-devnet.service`

## Node Specifications

### Validator Accounts
```
Node 0  = alice      (multichain-dev01 @ 68.219.230.63)
Node 1  = bob        (compiler-dev01 @ 98.71.91.84)
Node 2  = charlie    (compiler-dev01 @ 4.180.59.25)
Node 3  = dave       (consensus-dev01 @ 20.224.104.239)
Node 4  = eve        (multichain-dev01 @ 98.71.219.106)
Node 5  = ferdie     (runtime-dev01 @ 108.142.205.177)
Node 6  = alice      (runtime-dev01 @ 4.180.238.67)
Node 7  = bob        (audit-dev01 @ 51.142.203.160)
Node 8  = charlie    (flarenode15 @ 172.166.164.19)
Node 9  = dave       (flarenode16 @ 172.166.187.180)
Node 10 = eve        (flarenode17 @ 172.166.210.244)
Node 11 = ferdie     (oracle-dev01 @ 172.167.8.217)
Node 12 = alice      (flarenode18 @ 4.251.115.186)
Node 13 = bob        (flarenode19 @ 52.143.191.232)
Node 14 = charlie    (flarenode20 @ 4.211.206.210)
Node 15 = dave       (flarenode21 @ 4.178.181.122)
```

### Network Ports
```
P2P Port:        30333
RPC Port:        9944
WebSocket Port:  9945
Prometheus Port: 9615
```

## Troubleshooting

### SSH Connection Failed
```bash
# Test SSH manually
ssh -i ~/.ssh/etrid_vm1 -v user@host

# Check key permissions
ls -la ~/.ssh/etrid_vm1  # Should be 0600

# Check firewall
# Azure: Verify NSG rules allow SSH (port 22)
```

### Node Won't Start
```bash
# Check systemd status
./verify-devnet-nodes.sh logs 0

# Restart node
./verify-devnet-nodes.sh restart 0

# Reset node completely
./verify-devnet-nodes.sh reset 0
```

### Nodes Not Syncing
```bash
# Wait 2 minutes
sleep 120

# Check status
./verify-devnet-nodes.sh status

# If still diverged, reset slowest node
./verify-devnet-nodes.sh reset 5
```

### RPC Not Responding
```bash
# Check service is running
./verify-devnet-nodes.sh logs 0

# Look for "RPC server" or "listening on" messages
# If not found, service crashed during startup

# Restart
./verify-devnet-nodes.sh restart 0
```

### Out of Disk Space
```bash
# Check disk usage
./manage-devnet-nodes.sh disk

# If >80%, purge node data
./verify-devnet-nodes.sh reset 0

# Or purge all (if acceptable)
./manage-devnet-nodes.sh purge
```

## Performance Tuning

### For Faster Block Times
Edit `/etc/systemd/system/flarechain-devnet.service` on each node and add:
```
--insecure-validator-i-know-what-i-do
```

### For More Stable Consensus
Ensure all nodes have similar system resources:
- RAM: 4GB minimum
- CPU: 2+ cores
- Disk: 50GB available
- Network: <10ms latency between nodes

## Monitoring

### Real-time Status
```bash
./verify-devnet-nodes.sh watch
```

### Prometheus Metrics
```bash
curl http://<host>:9615/metrics | grep "substrate_"
```

### RPC Query
```bash
curl -s http://<host>:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
  | jq '.result.number'
```

## Advanced Usage

### Custom SSH Key
```bash
export SSH_KEY=/custom/path/to/key
./deploy-devnet-test-keys.sh
```

### Custom Timeout
```bash
export TIMEOUT=60
./verify-devnet-nodes.sh status
```

### Manual SSH Access
```bash
ssh -i ~/.ssh/etrid_vm1 multichain-dev01@68.219.230.63

# Once connected:
sudo systemctl status flarechain-devnet
journalctl -u flarechain-devnet -f
```

## Success Indicators

### Deployment Success
- ✓ All 16 nodes show "SUCCESS" in summary
- ✓ No SSH timeouts
- ✓ Services created on all nodes

### Runtime Success
- ✓ All 16 nodes showing "RUNNING" in status
- ✓ Block height numbers increasing
- ✓ All block heights within 5 blocks of each other
- ✓ RPC endpoints responding to queries
- ✓ No ERROR entries in logs

### Consensus Success
- ✓ Blocks being produced every 6-12 seconds
- ✓ Block heights synchronized across all nodes
- ✓ No "stalled" or "not finalized" messages in logs
- ✓ Peer connections showing in network stats

## Script Architecture

### deploy-devnet-test-keys.sh
```
Main flow:
1. Check SSH key
2. Deploy to all nodes in parallel
   - Kill existing processes
   - Purge chain data
   - Create systemd service
   - Start service
3. Wait for startup
4. Verify nodes producing blocks
5. Print summary with test key mapping
```

### verify-devnet-nodes.sh
```
Main flow:
1. Query each node's systemd status
2. Query RPC for block height
3. Aggregate results
4. Print status table with colors
5. Check consensus health (block variance)
```

### manage-devnet-nodes.sh
```
Main flow:
1. Parse command
2. Execute on all nodes in parallel
3. Return status for each node
```

## Logging and Debugging

### View Deployment Log
```bash
# Grep for errors during deployment
grep -i error /tmp/devnet_deploy_status.txt
```

### View Runtime Logs
```bash
# SSH to node and view service logs
ssh -i ~/.ssh/etrid_vm1 user@host
journalctl -u flarechain-devnet --no-pager | head -100
```

### Enable Debug Logging
Edit systemd service to add:
```
--log debug,runtime=trace
```

Then restart:
```bash
./verify-devnet-nodes.sh restart 0
```

## Maintenance Schedule

### Daily
- Run `./verify-devnet-nodes.sh watch` for 5 minutes
- Check for ERROR entries in logs

### Weekly
- Review disk usage: `./manage-devnet-nodes.sh disk`
- Check for stale processes

### Monthly
- Review consensus stability
- Analyze Prometheus metrics
- Consider purge and redeploy for clean state

## Support and Documentation

- **Detailed Guide:** See `DEVNET_DEPLOYMENT_GUIDE.md`
- **RPC Examples:** See node's RPC documentation
- **Substrate Docs:** https://docs.substrate.io/

---

**Tools Location:** `/Users/macbook/Desktop/etrid/`

**Last Updated:** November 1, 2025

**Support:** Check logs with `verify-devnet-nodes.sh logs <node_num>`
