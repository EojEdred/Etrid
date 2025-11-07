# Phase 3: Start Validators

**Duration:** 1-2 hours
**Goal:** Launch all validators and achieve network consensus

---

## Overview

In this phase, we'll:
1. Start validators in batches (not all at once)
2. Monitor sync progress
3. Verify peer connections
4. Achieve 15/21 consensus threshold
5. Confirm block production

**Why batches?** Starting in stages helps:
- Identify issues early
- Monitor sync progress better
- Ensure stable network formation

---

## Step 1: Update Bootnode Information (10 minutes)

First, we need bootnode peer IDs for the `--bootnodes` flag.

### Get Peer IDs from Running Bootnodes

If you have access to Oracle/Azure Sub2 validators:

```bash
# SSH into a bootnode
ssh -i ~/.ssh/gizzi-validator gizziio@64.181.215.19

# Get peer ID
/usr/local/bin/flarechain-node key inspect-node-key --file /path/to/node-key

# Or from logs
sudo journalctl -u flarechain-validator | grep "Local node identity"

exit
```

### Use Existing Bootnode Addresses

From previous deployment, these bootnodes should work:

```
/ip4/64.181.215.19/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm
/ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb
```

**Update all systemd service files with these bootnode addresses.**

✅ **Checkpoint:** Bootnode information ready

---

## Step 2: Start First Batch - 5 Validators (15 minutes)

Start the first 5 validators:

```bash
# VM01 (Validator 6)
ssh -i ~/.ssh/contabo-validators root@VM01_IP "systemctl start flarechain-validator"

# VM02 (Validator 7)
ssh -i ~/.ssh/contabo-validators root@VM02_IP "systemctl start flarechain-validator"

# VM03 (Validator 8)
ssh -i ~/.ssh/contabo-validators root@VM03_IP "systemctl start flarechain-validator"

# VM04 (Validator 9)
ssh -i ~/.ssh/contabo-validators root@VM04_IP "systemctl start flarechain-validator"

# VM05 (Validator 10)
ssh -i ~/.ssh/contabo-validators root@VM05_IP "systemctl start flarechain-validator"
```

### Monitor First Validator

```bash
ssh -i ~/.ssh/contabo-validators root@VM01_IP

# Watch logs
journalctl -u flarechain-validator -f

# Look for:
# - "Local node identity is: 12D3KooW..."
# - "Discovered new external address"
# - "Syncing, target=#XXXX"
# - Peer count increasing

# Press Ctrl+C to exit

# Check status
systemctl status flarechain-validator

# Should show "active (running)"

exit
```

**What to expect:**
- Initial: "Syncing" messages
- Peer count: Will start at 0-1, should reach 3-5 within 2 minutes
- Block height: Will start catching up
- Sync time: 30-60 minutes to fully sync

✅ **Checkpoint:** First 5 validators started and syncing

---

## Step 3: Wait and Monitor (20 minutes)

Give the first batch time to stabilize:

```bash
# Check all 5 validators
for ip in VM01_IP VM02_IP VM03_IP VM04_IP VM05_IP; do
    echo "=== Checking $ip ==="
    ssh -i ~/.ssh/contabo-validators root@$ip \
        "systemctl is-active flarechain-validator && \
         journalctl -u flarechain-validator -n 10 --no-pager | grep -E 'Syncing|Idle|peers'"
    echo ""
done
```

**Monitor for:**
- ✅ All 5 services "active"
- ✅ Peer count 3-8 per validator
- ✅ Block height increasing
- ✅ No repeated errors

**Common initial messages (NORMAL):**
```
Syncing 5.2 bps, target=#12345 (7 peers)
Idle (8 peers), best: #12350 (0x1234...)
```

**If you see issues:**
- Check firewall: `ufw status`
- Verify bootnode addresses in service file
- Check logs: `journalctl -u flarechain-validator -n 100`

✅ **Checkpoint:** First batch stable and syncing

---

## Step 4: Start Second Batch - 5 Validators (15 minutes)

```bash
# VM06 (Validator 11)
ssh -i ~/.ssh/contabo-validators root@VM06_IP "systemctl start flarechain-validator"

# VM07 (Validator 12)
ssh -i ~/.ssh/contabo-validators root@VM07_IP "systemctl start flarechain-validator"

# VM08 (Validator 13)
ssh -i ~/.ssh/contabo-validators root@VM08_IP "systemctl start flarechain-validator"

# VM09 (Validator 14)
ssh -i ~/.ssh/contabo-validators root@VM09_IP "systemctl start flarechain-validator"

# VM10 (Validator 15)
ssh -i ~/.ssh/contabo-validators root@VM10_IP "systemctl start flarechain-validator"
```

**Monitor:**
```bash
# Check batch 2
for ip in VM06_IP VM07_IP VM08_IP VM09_IP VM10_IP; do
    echo "=== Checking $ip ==="
    ssh -i ~/.ssh/contabo-validators root@$ip \
        "systemctl status flarechain-validator --no-pager | head -10"
    echo ""
done
```

**You should now have 10 validators online!**

✅ **Checkpoint:** Second batch started and syncing

---

## Step 5: Start Third Batch - 6 Validators (15 minutes)

Final batch:

```bash
# VM11 (Validator 16)
ssh -i ~/.ssh/contabo-validators root@VM11_IP "systemctl start flarechain-validator"

# VM12 (Validator 17)
ssh -i ~/.ssh/contabo-validators root@VM12_IP "systemctl start flarechain-validator"

# VM13 (Validator 18)
ssh -i ~/.ssh/contabo-validators root@VM13_IP "systemctl start flarechain-validator"

# VM14 (Validator 19)
ssh -i ~/.ssh/contabo-validators root@VM14_IP "systemctl start flarechain-validator"

# VM15 (Validator 20)
ssh -i ~/.ssh/contabo-validators root@VM15_IP "systemctl start flarechain-validator"

# VM16 (Validator 21)
ssh -i ~/.ssh/contabo-validators root@VM16_IP "systemctl start flarechain-validator"
```

**You now have all 16 Contabo validators online!**

✅ **Checkpoint:** All validators started

---

## Step 6: Wait for Full Sync (30-60 minutes)

Validators need time to sync the blockchain:

### Check Sync Status

```bash
# Create quick check script
cat > check-all-sync.sh <<'EOF'
#!/bin/bash
VMS=(VM01_IP VM02_IP VM03_IP VM04_IP VM05_IP VM06_IP VM07_IP VM08_IP VM09_IP VM10_IP VM11_IP VM12_IP VM13_IP VM14_IP VM15_IP VM16_IP)

for ip in "${VMS[@]}"; do
    echo "=== $ip ==="
    ssh -i ~/.ssh/contabo-validators root@$ip \
        "journalctl -u flarechain-validator -n 1 --no-pager | grep -oE '(Syncing|Idle|peers).*' || echo 'No recent activity'"
    echo ""
done
EOF

chmod +x check-all-sync.sh
nano check-all-sync.sh  # Add your IPs
```

**Run periodically:**
```bash
./check-all-sync.sh
```

**What to look for:**
- **"Syncing X bps, target=#YYYY"** = Still catching up
- **"Idle (N peers)"** = Fully synced! ✅
- **Peer count 8-15** = Healthy

**Sync timeline:**
- 0-10 min: Initial sync, catching up fast
- 10-30 min: Syncing last blocks
- 30-60 min: Most validators fully synced

✅ **Checkpoint:** Validators syncing to current block

---

## Step 7: Verify Network Consensus (10 minutes)

Once most validators show "Idle", check consensus:

### Run Health Check

```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet
bash check-validators-simple.sh
```

**Expected output:**
```
Total Validators: 21
RPC Responding: 16-18
Network Status: ✓ HEALTHY (Supermajority online)
```

### Check Block Production

```bash
# Query any validator for current block
curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
    http://VM01_IP:9944 | python3 -m json.tool

# Should show current block number
```

### Check Finality

```bash
# Check finalized block
curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
    http://VM01_IP:9944 | python3 -m json.tool

# Finalized block should be close to latest block (within 2-5 blocks)
```

✅ **Checkpoint:** Network producing and finalizing blocks

---

## Step 8: Monitor for Stability (15 minutes)

Let the network run for 15 minutes to ensure stability:

```bash
# Watch block production
watch -n 10 'curl -s -X POST -H "Content-Type: application/json" \
    -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[],\"id\":1}" \
    http://VM01_IP:9944 | python3 -c "import sys,json; h=json.load(sys.stdin); print(\"Block:\", int(h[\"result\"][\"number\"], 16))"'

# Press Ctrl+C when satisfied
```

**What to verify:**
- Block number increasing every 6 seconds
- No validators dropping offline
- Peer counts stable (8-15 per validator)
- No errors in logs

✅ **Checkpoint:** Network stable and healthy

---

## Phase 3 Complete! ✅

Your Contabo validators are now:

- [x] All 16 validators started
- [x] Synced to current blockchain height
- [x] Connected to peer network (8-15 peers each)
- [x] Producing blocks (if in active set)
- [x] Finalizing blocks (GRANDPA consensus working)

**Network Status:** 🟢 ONLINE

With Oracle validators (Phase 5), you'll have 18/21 total!

---

## Next Step

**Open:** `04_PHASE_4_Verify_Network.md`

That guide will walk you through:
- Comprehensive network health checks
- Transaction testing
- Performance verification
- Documentation of new setup

---

## Troubleshooting

### "Validator won't start"
```bash
# Check service status
systemctl status flarechain-validator

# Check logs
journalctl -u flarechain-validator -n 50

# Common issues:
# - Missing chainspec file
# - Binary not executable
# - Port already in use
```

### "No peers connecting"
```bash
# Check firewall
ufw status

# Should allow port 30333
ufw allow 30333/tcp

# Check bootnode addresses in service file
cat /etc/systemd/system/flarechain-validator.service | grep bootnodes

# Restart if needed
systemctl restart flarechain-validator
```

### "Stuck syncing"
- This is normal, give it time (30-60 minutes)
- Check peers: should have 3-8 peers minimum
- Verify bootnode reachable: `nc -zv 64.181.215.19 30333`

### "Validator crashed"
```bash
# Check logs for errors
journalctl -u flarechain-validator -n 100

# Restart
systemctl restart flarechain-validator

# If persistent, check:
# - Disk space: df -h
# - Memory: free -h
# - CPU: top
```

---

**Phase 3 Duration:** ~1-2 hours (mostly waiting for sync)
**Status:** Network operational! 🚀

