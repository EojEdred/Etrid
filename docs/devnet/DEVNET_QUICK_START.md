# ËTRID DEVNET - Quick Start Guide

## Deploy DEVNET Test Keys in 3 Steps

### Step 1: Deploy (3-5 minutes)
```bash
cd /Users/macbook/Desktop/etrid
./deploy-devnet-test-keys.sh
```

### Step 2: Wait
```bash
sleep 60
```

### Step 3: Verify
```bash
./verify-devnet-nodes.sh status
```

**Done!** All 16 nodes should now be running with test keys.

---

## Command Quick Reference

### Monitoring
```bash
./verify-devnet-nodes.sh status      # Check all nodes
./verify-devnet-nodes.sh watch       # Continuous monitoring
./verify-devnet-nodes.sh logs 0      # View Node 0 logs
```

### Node Operations
```bash
./verify-devnet-nodes.sh restart 3   # Restart Node 3
./verify-devnet-nodes.sh reset 5     # Reset Node 5
```

### Bulk Operations
```bash
./manage-devnet-nodes.sh start       # Start all nodes
./manage-devnet-nodes.sh stop        # Stop all nodes
./manage-devnet-nodes.sh restart-fast # Restart all (parallel)
./manage-devnet-nodes.sh ssh 0       # SSH to Node 0
```

---

## Node IPs & Accounts

| Node | IP Address | Account |
|------|-----------|---------|
| 0 | 68.219.230.63 | alice |
| 1 | 98.71.91.84 | bob |
| 2 | 4.180.59.25 | charlie |
| 3 | 20.224.104.239 | dave |
| 4 | 98.71.219.106 | eve |
| 5 | 108.142.205.177 | ferdie |
| 6 | 4.180.238.67 | alice |
| 7 | 51.142.203.160 | bob |
| 8 | 172.166.164.19 | charlie |
| 9 | 172.166.187.180 | dave |
| 10 | 172.166.210.244 | eve |
| 11 | 172.167.8.217 | ferdie |
| 12 | 4.251.115.186 | alice |
| 13 | 52.143.191.232 | bob |
| 14 | 4.211.206.210 | charlie |
| 15 | 4.178.181.122 | dave |

---

## Access Nodes

### RPC Endpoint
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

---

## Common Issues

| Issue | Solution |
|-------|----------|
| SSH timeout | Check firewall, test SSH key |
| Node won't start | View logs: `verify-devnet-nodes.sh logs 0` |
| Nodes not syncing | Wait 2 min, then reset: `verify-devnet-nodes.sh reset 0` |
| RPC not responding | Restart node: `verify-devnet-nodes.sh restart 0` |

---

## Success Indicators

- All 16 nodes showing "RUNNING"
- Block numbers increasing every 6-12 seconds
- Block heights within 5 blocks of each other
- RPC endpoints responding
- No ERROR entries in logs

---

## File Locations

```
Deployment Tools:
  /Users/macbook/Desktop/etrid/deploy-devnet-test-keys.sh
  /Users/macbook/Desktop/etrid/verify-devnet-nodes.sh
  /Users/macbook/Desktop/etrid/manage-devnet-nodes.sh

Documentation:
  /Users/macbook/Desktop/etrid/DEVNET_DEPLOYMENT_GUIDE.md
  /Users/macbook/Desktop/etrid/DEVNET_TOOLS_README.md
  /Users/macbook/Desktop/etrid/DEVNET_DEPLOYMENT_SUMMARY.md

SSH Key:
  ~/.ssh/etrid_vm1
```

---

**First time?** Read `DEVNET_DEPLOYMENT_GUIDE.md` for detailed instructions.

**Need help?** Check `DEVNET_TOOLS_README.md` for comprehensive documentation.
