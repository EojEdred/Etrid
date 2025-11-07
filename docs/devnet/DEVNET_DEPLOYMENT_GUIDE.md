# ËTRID DEVNET Test Keys Deployment Guide

## Overview

This guide covers configuring Substrate standard test keys on all 16 DEVNET nodes and restarting them with proper validator accounts.

**Problem Being Solved:**
- Nodes were crashing due to missing/incorrect validator keys
- Binary works fine - it's a key configuration issue
- Need to use standard test keys suitable for DEVNET

## Deployment Architecture

### 16 DEVNET Nodes

| Node | SSH Target | Account | Location | Purpose |
|------|-----------|---------|----------|---------|
| 0 | multichain-dev01@68.219.230.63 | alice | Primary Multichain | Entry node |
| 1 | compiler-dev01@98.71.91.84 | bob | Compiler Monitoring | Compiler validation |
| 2 | compiler-dev01@4.180.59.25 | charlie | Compiler Primary | Compiler execution |
| 3 | consensus-dev01@20.224.104.239 | dave | Consensus Dev | Consensus testing |
| 4 | multichain-dev01@98.71.219.106 | eve | Multichain Secondary | Multichain sync |
| 5 | runtime-dev01@108.142.205.177 | ferdie | Runtime Primary | Runtime execution |
| 6 | runtime-dev01@4.180.238.67 | alice | Runtime Secondary | Runtime sync |
| 7 | audit-dev01@51.142.203.160 | bob | Audit Dev | Audit trail |
| 8 | flarenode15@172.166.164.19 | charlie | Economics Primary | Economic models |
| 9 | flarenode16@172.166.187.180 | dave | Economics Secondary | Economic sync |
| 10 | flarenode17@172.166.210.244 | eve | Ethics Primary | Ethics framework |
| 11 | oracle-dev01@172.167.8.217 | ferdie | Oracle Dev | Oracle integration |
| 12 | flarenode18@4.251.115.186 | alice | Ethics Secondary | Ethics enforcement |
| 13 | flarenode19@52.143.191.232 | bob | Docs Primary | Documentation |
| 14 | flarenode20@4.211.206.210 | charlie | Docs Secondary | Docs sync |
| 15 | flarenode21@4.178.181.122 | dave | Docs Tertiary | Docs backup |

### Substrate Development Accounts

These are standard Substrate/Polkadot development accounts with well-known keys:

- **alice** - Primary validator account
- **bob** - Secondary validator account
- **charlie** - Tertiary validator account
- **dave** - Quaternary validator account
- **eve** - Quinary validator account
- **ferdie** - Senary validator account

## Quick Start

### 1. Deploy Test Keys to All 16 Nodes

```bash
cd /Users/macbook/Desktop/etrid

# Deploy to all nodes (parallel)
./deploy-devnet-test-keys.sh

# This will:
# - Kill existing node processes
# - Purge chain databases
# - Create systemd services with proper test accounts
# - Start all 16 nodes
# - Generate deployment report
```

**Expected Duration:** 3-5 minutes

### 2. Verify Deployment

```bash
# Check status of all nodes
./verify-devnet-nodes.sh status

# Monitor continuously
./verify-devnet-nodes.sh watch

# Expected output:
# - All 16 nodes RUNNING
# - Block heights within 5 blocks of each other
# - RPC endpoints responding
```

**Expected Block Height Growth:**
- First block: ~10 seconds
- Subsequent blocks: ~6 seconds per block
- After 1 minute: ~10 blocks

### 3. Troubleshooting Individual Nodes

```bash
# View logs for Node 0
./verify-devnet-nodes.sh logs 0

# Restart Node 3
./verify-devnet-nodes.sh restart 3

# Reset Node 5 (purge data + restart)
./verify-devnet-nodes.sh reset 5
```

## Configuration Details

### Systemd Service

Each node runs as a systemd service: `flarechain-devnet`

**Service File Location:** `/etc/systemd/system/flarechain-devnet.service`

**Service Commands:**
```bash
# SSH to node first
ssh -i ~/.ssh/etrid_vm1 user@host

# Then on remote:
sudo systemctl status flarechain-devnet      # Check status
sudo systemctl start flarechain-devnet       # Start service
sudo systemctl stop flarechain-devnet        # Stop service
sudo systemctl restart flarechain-devnet     # Restart service
sudo systemctl enable flarechain-devnet      # Enable on boot

# View logs
journalctl -u flarechain-devnet -f           # Follow logs
journalctl -u flarechain-devnet -n 100       # Last 100 lines
```

### Node Configuration

**Binary Path:** `/opt/flarechain/flarechain-node`

**Data Directory:** `/data/flarechain`

**Key Startup Flags:**
```bash
--dev                           # Development mode (no real consensus)
--<account>                     # Account flag (alice, bob, charlie, etc.)
--base-path /data/flarechain    # Data directory
--name node<N>                  # Node name
--port 30333                    # P2P port
--rpc-port 9944                 # RPC port
--rpc-external                  # Allow external RPC
--rpc-cors all                  # Allow all RPC origins
--prometheus-external           # Allow external Prometheus
--prometheus-port 9615          # Prometheus metrics port
--validator                     # Run as validator (optional in dev mode)
--log info,runtime=debug        # Logging level
```

## Testing Scenarios

### Scenario 1: Basic Consensus Test

```bash
# 1. Deploy to all nodes
./deploy-devnet-test-keys.sh

# 2. Wait 30 seconds
sleep 30

# 3. Verify all nodes are synced
./verify-devnet-nodes.sh status

# Expected:
# - All 16 nodes running
# - Block heights within 2-3 blocks of each other
# - All RPC endpoints responding
```

### Scenario 2: Network Partition Test

```bash
# 1. Stop Node 0 (minority partition)
./verify-devnet-nodes.sh restart 0

# 2. Check if remaining 15 nodes continue
./verify-devnet-nodes.sh status

# Expected:
# - 15 nodes continue producing blocks
# - Node 0 catches up after restart
```

### Scenario 3: Node Recovery Test

```bash
# 1. Reset Node 3 (complete purge)
./verify-devnet-nodes.sh reset 3

# 2. Wait 2 minutes for sync
sleep 120

# 3. Check sync status
./verify-devnet-nodes.sh logs 3
./verify-devnet-nodes.sh status

# Expected:
# - Node 3 syncs to current block height
# - Joins network consensus
```

## Accessing Nodes

### RPC Endpoint

Each node exposes RPC on port 9944:

```bash
# Get block header
curl -s http://<host>:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
  | jq .

# Get system info
curl -s http://<host>:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_properties","params":[],"id":1}' \
  | jq .

# List peers
curl -s http://<host>:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_peers","params":[],"id":1}' \
  | jq .
```

### WebSocket Endpoint

Some clients prefer WebSocket on port 9945:

```bash
# Using wscat:
wscat -c ws://<host>:9945
```

### Prometheus Metrics

Metrics available on port 9615:

```bash
curl http://<host>:9615/metrics

# Key metrics:
# substrate_block_height - Current block height
# substrate_block_producer - Block producer metrics
# substrate_consensus_round - Consensus round info
```

## Monitoring and Observability

### SSH Key

All nodes use the same SSH key: `~/.ssh/etrid_vm1`

### Log Inspection

```bash
# SSH to a node
ssh -i ~/.ssh/etrid_vm1 user@host

# Follow service logs in real-time
journalctl -u flarechain-devnet -f

# Search for errors
journalctl -u flarechain-devnet | grep -i error

# Get summary statistics
journalctl -u flarechain-devnet --no-pager | tail -50
```

### Common Log Messages

**Good signs:**
```
Consensus round starting
Block authored
Block finalized
Peer connected
```

**Warning signs:**
```
ERROR
connection refused
Timeout
failed to connect
not responding
```

## Troubleshooting

### Problem: Nodes Not Starting

**Symptoms:**
- Service shows as "inactive"
- Systemctl status shows error

**Solution:**
```bash
# Check logs
./verify-devnet-nodes.sh logs 0

# Restart node
./verify-devnet-nodes.sh restart 0

# If still failing, reset completely
./verify-devnet-nodes.sh reset 0
```

### Problem: Nodes Not Syncing

**Symptoms:**
- RPC not responding
- Block heights diverging
- Service shows "inactive"

**Solution:**
```bash
# Check if service is running
./verify-devnet-nodes.sh status

# View logs to see errors
./verify-devnet-nodes.sh logs 3

# Reset the node
./verify-devnet-nodes.sh reset 3

# Wait for sync
sleep 60

# Verify
./verify-devnet-nodes.sh status
```

### Problem: High Block Height Variance

**Symptoms:**
- Blocks heights vary by >10 blocks
- Some nodes ahead, some lagging

**Solution:**
```bash
# This is often temporary during startup
# Wait 2 minutes and check again
sleep 120
./verify-devnet-nodes.sh status

# If persists, restart the slowest node
./verify-devnet-nodes.sh restart 5
```

### Problem: SSH Connection Timeouts

**Symptoms:**
- "SSH connection failed"
- "ConnectTimeout"

**Solution:**
```bash
# Check SSH key exists
ls -la ~/.ssh/etrid_vm1

# Test SSH manually
ssh -i ~/.ssh/etrid_vm1 -v user@host

# May indicate firewall issues on Azure
# Check Azure NSG rules allow SSH (port 22)
```

## Environment Variables

Control deployment with environment variables:

```bash
# Specify custom SSH key
SSH_KEY=/path/to/custom/key ./deploy-devnet-test-keys.sh

# Specify timeout for connections
TIMEOUT=60 ./verify-devnet-nodes.sh status
```

## Performance Expectations

### Block Production
- Block time: ~6 seconds per block
- Full consensus: ~12 seconds per block
- Finalization: ~6 seconds (1 block)

### Network Latency
- P2P discovery: ~5-10 seconds
- Block propagation: ~1-2 seconds
- RPC response: <100ms

### Resource Usage
- Memory per node: ~500MB-1GB
- CPU per node: <10% at 6s block time
- Disk per node: ~5GB after 1000 blocks

## Backup and Restoration

### Backup Node State

```bash
# SSH to node
ssh -i ~/.ssh/etrid_vm1 user@host

# Backup data directory
sudo tar czf /backup/flarechain-backup-$(date +%s).tar.gz /data/flarechain
```

### Restore Node State

```bash
# Stop service
sudo systemctl stop flarechain-devnet

# Restore backup
sudo tar xzf /backup/flarechain-backup-TIMESTAMP.tar.gz -C /

# Start service
sudo systemctl start flarechain-devnet
```

## Maintenance

### Regular Health Checks

```bash
# Daily monitoring
./verify-devnet-nodes.sh watch

# Check logs for errors
./verify-devnet-nodes.sh logs 0
./verify-devnet-nodes.sh logs 8
./verify-devnet-nodes.sh logs 15

# Verify consensus health
curl -s http://node0:9944 -d '...' | jq '.result.number'
curl -s http://node8:9944 -d '...' | jq '.result.number'
```

### Weekly Maintenance

```bash
# Check for protocol updates
cd /Users/macbook/Desktop/etrid
git pull

# Rebuild if needed
cargo build --release -p flarechain-node

# Redeploy if binary changed
./deploy-devnet-test-keys.sh
```

### Data Cleanup

```bash
# On each node, cleanup old block data
ssh -i ~/.ssh/etrid_vm1 user@host

# Check disk usage
du -sh /data/flarechain

# If >50GB, consider pruning or resetting
sudo systemctl stop flarechain-devnet
sudo rm -rf /data/flarechain/*
sudo systemctl start flarechain-devnet
```

## Success Criteria

After deployment, you should see:

1. **All nodes running**
   ```
   ✓ Node 0 (alice) - RUNNING - Block: 42
   ✓ Node 1 (bob) - RUNNING - Block: 41
   ...
   Running: 16/16
   ```

2. **Block production active**
   ```
   Block numbers increasing every 6-12 seconds
   Block height variance <5 blocks
   ```

3. **RPC responding**
   ```
   $ curl http://host:9944 -d '...' -H 'Content-Type: application/json'
   {"jsonrpc":"2.0","result":{"number":"0x2a",...},"id":1}
   ```

4. **Logs showing consensus**
   ```
   journal output shows: "Block authored", "Block finalized"
   No ERROR entries
   ```

## Next Steps

1. **Wait 2 minutes** for nodes to fully sync
2. **Run verification script** to confirm all nodes are running
3. **Monitor logs** for any errors or warnings
4. **Test RPC endpoints** to verify functionality
5. **Proceed with testing scenarios** as needed

## Support

For issues or questions:

1. Check logs: `./verify-devnet-nodes.sh logs <node_num>`
2. Restart node: `./verify-devnet-nodes.sh restart <node_num>`
3. Check full status: `./verify-devnet-nodes.sh status`
4. Review this guide's troubleshooting section

---

**Last Updated:** November 1, 2025
**Deployment Scripts:** `/Users/macbook/Desktop/etrid/deploy-devnet-test-keys.sh`, `verify-devnet-nodes.sh`
**SSH Key:** `~/.ssh/etrid_vm1`
