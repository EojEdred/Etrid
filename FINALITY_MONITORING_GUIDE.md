# ASF Finality Monitoring & P2P Network Diagnosis

## Overview

This guide explains how to use P2P network monitoring tools to diagnose and fix ASF (Asynchronous Safe Finality) issues in the FlareChain validator network.

## The Finality Problem

**Root Cause**: ASF finality stuck at block #0 because view transitions weren't implemented

**Solution**: Deploy view transition code that enables validators to coordinate via NewView messages over the P2P network

**Key Requirement**: Need 15/22 validators (2/3 + 1 quorum) broadcasting NewView messages for finality to advance

## How P2P Monitoring Helps Finality

### 1. **Track View Transition Deployment**

As you deploy view transitions to validators, P2P monitoring lets you:
- See which validators are broadcasting NewView messages
- Count how many validators have the update (progress toward 15/22 quorum)
- Verify P2P message propagation across the network
- Detect validators that failed to deploy or restart

### 2. **Diagnose View Change Coordination**

Once deployed, monitor whether:
- NewView messages are being sent every 6 seconds
- NewView messages are being received by other validators
- Validators are achieving view change quorum (15/22)
- Views are advancing: View(1) → View(2) → View(3)
- After 3 consecutive views, finality starts advancing

### 3. **Verify Finality Advancement**

Check if finality is working by monitoring:
- Finalized block numbers increasing beyond #0
- All validators converging on the same finalized blocks
- Finality advancing at the expected rate (one block per view cycle)

## Monitoring Tools

### Quick Status Check (Fast)

```bash
/tmp/quick-finality-check.sh
```

**What it shows**:
- NewView broadcast status for 6 key validators
- Current view numbers
- Latest finalized blocks
- Quick quorum progress estimate

**When to use**: During deployment to quickly check if view transitions are working

**Runtime**: ~10-15 seconds

### Full Network Report (Comprehensive)

```bash
chmod +x /tmp/monitor-finality-network.sh
/tmp/monitor-finality-network.sh
```

**What it shows**:
- Complete status of all 22 validators
- NewView broadcast/reception counts
- View distribution across network
- Finalized block distribution
- Quorum analysis and recommendations
- P2P connectivity health

**When to use**:
- After deployment completes to verify full network status
- When debugging finality issues
- To generate comprehensive status reports

**Runtime**: ~2-3 minutes

**Output**: Saves detailed report to `~/Desktop/etrid/finality-reports/finality_report_*.txt`

### P2P Network Topology (Visual)

```bash
/tmp/map-p2p-network.sh
```

**What it shows**:
- Complete P2P connection topology
- Which validators are connected to which
- Peer relationship maps
- Network partitioning detection
- GraphViz visualization (if installed)

**When to use**:
- To understand P2P network structure
- To detect network partitions preventing message propagation
- To troubleshoot why NewView messages aren't reaching all validators

## Monitoring Workflow

### Phase 1: During Deployment

While deploying view transitions to all 22 validators:

```bash
# Check deployment progress
tail -f /tmp/view-transition-deployment.log

# Quick finality check every few minutes
watch -n 180 /tmp/quick-finality-check.sh
```

**What to look for**:
- NewView broadcast count increasing as validators deploy
- Progress toward 15/22 quorum threshold

### Phase 2: Approaching Quorum (13-15 validators deployed)

```bash
# Run full network check
/tmp/monitor-finality-network.sh
```

**What to look for**:
- NewView broadcast quorum status
- Whether validators are receiving messages from peers
- View numbers starting to advance

### Phase 3: Post-Quorum (15+ validators deployed)

```bash
# Monitor for finality advancement
/tmp/quick-finality-check.sh

# Check specific validator logs
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'sudo journalctl -u flarechain-validator -f | grep -E "NewView|View.*triggered|finalized"'
```

**What to look for**:
- "View change quorum reached" messages
- Views advancing: View(1) → View(2) → View(3)
- Finalized block numbers increasing beyond #0
- All validators converging on same finalized height

### Phase 4: Steady State Monitoring

```bash
# Periodic health checks
/tmp/quick-finality-check.sh

# Full network reports (daily/weekly)
/tmp/monitor-finality-network.sh
```

## Key Metrics to Monitor

### NewView Broadcast Count
**What it means**: Number of validators actively sending NewView messages

**Healthy**: 22/22 (all validators broadcasting)
**Quorum**: 15/22 (minimum for view changes)
**Problem**: <15 (quorum not achieved, finality stuck)

**How to check**:
```bash
/tmp/quick-finality-check.sh | grep "Broadcasting NewView"
```

### NewView Reception Count
**What it means**: Number of validators receiving NewView messages from peers

**Healthy**: 22/22 (all validators receiving)
**Problem**: <15 (P2P connectivity issues, network partition)

**How to diagnose**:
- Check P2P peer counts: `curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944`
- Map P2P topology: `/tmp/map-p2p-network.sh`
- Check for firewall issues blocking P2P ports

### View Distribution
**What it means**: Which View() validators are currently on

**Healthy**: All validators on same view
**Problem**: Validators on different views (coordination failing)

**Diagnosis**:
- Check if NewView messages are propagating
- Verify system clocks are synchronized (view timeouts are time-based)
- Look for network partitions in P2P topology

### Finalized Block Height
**What it means**: Latest block number that has reached finality

**Healthy**: Increasing every view cycle (~6 seconds)
**Stuck**: All validators at #0 (view transitions not working)
**Diverged**: Different validators at different heights (consensus issue)

**How to track**:
```bash
# Watch finality advance in real-time
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'sudo journalctl -u flarechain-validator -f | grep "finalized #"'
```

## Troubleshooting with P2P Monitoring

### Problem: Finality Stuck at #0

**Diagnosis**:
```bash
/tmp/monitor-finality-network.sh
```

**Check**:
1. **NewView broadcast count < 15**: Deploy view transitions to more validators
2. **NewView broadcasts ≥ 15 but reception < 15**: P2P connectivity issue
   - Run `/tmp/map-p2p-network.sh` to check topology
   - Verify validators have peers: Check "P2P peers" count in report
   - Check for network partitions or firewall issues
3. **Both broadcast/reception ≥ 15 but no finality**: View coordination issue
   - Check validator logs for "View change quorum reached" messages
   - Verify views are advancing (View numbers should increase)
   - Wait 3 view cycles (18 seconds) for finality to start

### Problem: Some Validators Not Broadcasting NewView

**Diagnosis**:
```bash
/tmp/monitor-finality-network.sh | grep "Broadcasting NewView"
```

**Possible causes**:
1. **Validator didn't get deployment**: Re-run deploy script for that validator
2. **Validator binary didn't restart**: Check systemctl status, restart if needed
3. **Old code still running**: Verify binary version/commit hash matches deployed code

**Fix**:
```bash
# Check specific validator
ssh -i ~/.ssh/contabo-validators root@VALIDATOR_IP \
  'systemctl status flarechain-validator && \
   /usr/local/bin/flarechain-node --version'

# Redeploy if needed
# (use individual validator deploy from /tmp/deploy-view-transition-gizzi.sh as template)
```

### Problem: NewView Messages Not Propagating

**Diagnosis**:
```bash
/tmp/map-p2p-network.sh
```

**Check for**:
1. **Low peer counts**: Validators should have 10-20 peers each
2. **Network partitions**: Some validators isolated from others
3. **Firewall blocking P2P**: Port 30333 must be open

**Fix**:
```bash
# Check P2P connectivity from validator
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'curl -s -H "Content-Type: application/json" \
   -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}" \
   http://localhost:9944'

# Should show "peers": 15+ for healthy connectivity
```

### Problem: Views Not Advancing

**Symptoms**: Validators broadcasting/receiving NewView but View() number stays the same

**Diagnosis**:
```bash
# Check view transition logs
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'sudo journalctl -u flarechain-validator --since "5 minutes ago" | \
   grep -E "View.*triggered|View change quorum"'
```

**Possible causes**:
1. **Quorum not reached**: Need 15/22 validators with matching NewView messages
2. **Timeout not triggering**: Check "View timeout triggered" appears every 6 seconds
3. **View change logic bug**: Check logs for errors in view transition handling

## Integration with Existing P2P Tools

### Combined with P2P Network Mapping

```bash
# 1. Check finality status
/tmp/quick-finality-check.sh

# 2. If issues found, map full P2P topology
/tmp/map-p2p-network.sh

# 3. Analyze connectivity between specific validators
# (from P2P_NETWORK_MAPPING_GUIDE.md)
```

### Live Monitoring Dashboard

Create a continuous monitoring loop:

```bash
#!/bin/bash
while true; do
  clear
  echo "=== ASF Finality Status ==="
  date
  echo ""
  /tmp/quick-finality-check.sh
  echo ""
  echo "Refreshing in 30 seconds..."
  sleep 30
done
```

## Expected Behavior After Successful Deployment

### Immediately After Quorum (15+ validators)

**Logs should show**:
```
🔄 View timeout triggered: transitioning to view=View(1)
✅ NewView message broadcast: view=View(1), validator=586020164
🔄 NewView received: view=View(1), from validator=123456789, count=15/22
✅ View change quorum reached! Transitioning to view=View(1)
```

### After 3 Consecutive Views (~18 seconds)

**Logs should show**:
```
🎯 finalized #1001 (hash: 0x...)
🎯 finalized #1002 (hash: 0x...)
🎯 finalized #1003 (hash: 0x...)
```

### Steady State

- Views advancing every 6 seconds: View(N) → View(N+1)
- Finality advancing every view cycle: Block N → Block N+1
- All 22 validators broadcasting NewView messages
- All 22 validators on the same view
- Finalized block heights converging across all validators

## Quick Reference Commands

```bash
# Quick status (10 seconds)
/tmp/quick-finality-check.sh

# Full network report (2-3 minutes)
/tmp/monitor-finality-network.sh

# P2P topology map (3-5 minutes)
/tmp/map-p2p-network.sh

# Watch deployment progress
tail -f /tmp/view-transition-deployment.log

# Watch finality advance (single validator)
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'sudo journalctl -u flarechain-validator -f | grep "finalized #"'

# Watch NewView messages (single validator)
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'sudo journalctl -u flarechain-validator -f | grep -E "NewView|View.*triggered"'

# Check P2P peers (single validator)
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'curl -s -H "Content-Type: application/json" \
   -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}" \
   http://localhost:9944 | grep -o "\"peers\":[0-9]*"'
```

## Summary

P2P network monitoring is **essential** for diagnosing finality issues because:

1. **View transitions depend on P2P**: NewView messages must propagate via P2P network
2. **Quorum requires connectivity**: 15/22 validators must communicate for view changes
3. **Finality requires coordination**: All validators must agree on views via P2P messages
4. **Network health = Finality health**: If P2P is broken, finality won't work

Use these tools to:
- Track view transition deployment progress
- Verify NewView message propagation
- Diagnose P2P connectivity issues
- Confirm finality advancement
- Monitor ongoing network health

**The deployment is successful when**: 15+ validators are broadcasting NewView, views are advancing, and finalized block heights are increasing beyond #0.
