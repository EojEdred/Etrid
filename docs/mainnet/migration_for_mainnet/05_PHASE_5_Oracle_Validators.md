# Phase 5: Start Oracle Cloud Validators

**Duration:** 30 minutes (can run in parallel with other phases)
**Goal:** Get 2 Oracle validators online

---

## Overview

You have 2 Oracle Cloud validators:
1. **V1-Gizzi (64.181.215.19)** - Primary bootnode
2. **V3-Audit (129.80.122.34)** - Tertiary bootnode

These can be started anytime during or after the Contabo migration.

---

## Step 1: Access Oracle Gizzi VM (5 minutes)

### Try Different Access Methods

```bash
# Method 1: Try gizziio user
ssh -i ~/.ssh/gizzi-validator gizziio@64.181.215.19

# Method 2: Try ubuntu user
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Method 3: Try default Oracle user
ssh -i ~/.ssh/gizzi-validator opc@64.181.215.19

# Method 4: If you don't have the key, try password
ssh gizziio@64.181.215.19
```

**Once logged in:**
```bash
# Check if you're in
whoami
hostname

# Should show gizziio or similar
```

✅ **Checkpoint:** Successfully logged into Gizzi VM

---

## Step 2: Check Validator Status on Gizzi (5 minutes)

```bash
# Check if validator service exists
systemctl list-units | grep -i flare

# Possible service names:
# - flarechain-validator
# - etrid-validator
# - flarechain-node

# Check status
sudo systemctl status flarechain-validator

# If service doesn't exist, check for running process
ps aux | grep flarechain

# Check if binary exists
ls -lh /usr/local/bin/flarechain-node
ls -lh /usr/local/bin/etrid
ls -lh /usr/local/bin/etrid-validator

# Check for chainspec
find /home -name "*chainspec*.json" 2>/dev/null
find /root -name "*chainspec*.json" 2>/dev/null
```

✅ **Checkpoint:** Identified validator setup on Gizzi

---

## Step 3: Start Gizzi Validator (5 minutes)

### If Service Exists

```bash
# Start the service
sudo systemctl start flarechain-validator

# Enable auto-start
sudo systemctl enable flarechain-validator

# Check status
sudo systemctl status flarechain-validator

# Monitor logs
sudo journalctl -u flarechain-validator -f

# Look for:
# - "Local node identity"
# - "Syncing" or "Idle"
# - Peer connections

# Press Ctrl+C when satisfied
```

### If No Service (Manual Start)

```bash
# Find the binary
BINARY=$(find / -name "flarechain-node" -o -name "etrid-validator" 2>/dev/null | head -1)
echo "Binary: $BINARY"

# Find chainspec
CHAINSPEC=$(find / -name "*chainspec*.json" 2>/dev/null | head -1)
echo "Chainspec: $CHAINSPEC"

# Start manually
$BINARY \
  --chain $CHAINSPEC \
  --base-path /home/gizziio/.etrid \
  --validator \
  --name "Gizzi-Oracle-Bootnode" \
  --public-addr /ip4/64.181.215.19/tcp/30333 \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all

# Run in background with nohup
nohup $BINARY --chain $CHAINSPEC --validator --port 30333 > /tmp/validator.log 2>&1 &

# Check it's running
ps aux | grep flarechain
tail -f /tmp/validator.log
```

✅ **Checkpoint:** Gizzi validator started

---

## Step 4: Verify Gizzi is Online (3 minutes)

**From your local machine:**

```bash
# Test RPC
curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
    http://64.181.215.19:9944 | python3 -m json.tool

# Should show:
# - peers: 8-15
# - isSyncing: false (after sync complete)

# Check node exporter
curl -s http://64.181.215.19:9100/metrics | grep "node_uname_info"

# Should return system info if node exporter running
```

✅ **Checkpoint:** Gizzi responding on network

---

## Step 5: Access Oracle Audit VM (5 minutes)

```bash
# Try different methods
ssh -i ~/.ssh/gizzi-validator ubuntu@129.80.122.34
ssh -i ~/.ssh/gizzi-validator aed2020@129.80.122.34
ssh -i ~/.ssh/gizzi-validator opc@129.80.122.34

# If VM not responding:
# - May be stopped in Oracle Cloud Console
# - May have different firewall rules
# - May need to be started via Oracle CLI/Console
```

**If you can't access:**
- Log into Oracle Cloud Console (https://cloud.oracle.com)
- Check VM power state
- Start VM if stopped
- Check firewall/security list rules
- Try SSH again after 2-3 minutes

✅ **Checkpoint:** Accessed Audit VM OR identified issue

---

## Step 6: Start Audit Validator (5 minutes)

**Same process as Gizzi:**

```bash
# Check service
sudo systemctl status flarechain-validator

# Start if exists
sudo systemctl start flarechain-validator
sudo systemctl enable flarechain-validator

# Or start manually if needed
# (see Step 3 for manual start instructions)

# Monitor
sudo journalctl -u flarechain-validator -f
```

✅ **Checkpoint:** Audit validator started

---

## Step 7: Verify Both Oracle Validators (2 minutes)

**From your local machine:**

```bash
# Test Gizzi
echo "=== Gizzi (64.181.215.19) ==="
curl -s -m 5 -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
    http://64.181.215.19:9944 | python3 -c "import sys,json; h=json.load(sys.stdin); print('Peers:', h['result']['peers'], '| Syncing:', h['result']['isSyncing'])" 2>/dev/null || echo "Not responding"

echo ""

# Test Audit
echo "=== Audit (129.80.122.34) ==="
curl -s -m 5 -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
    http://129.80.122.34:9944 | python3 -c "import sys,json; h=json.load(sys.stdin); print('Peers:', h['result']['peers'], '| Syncing:', h['result']['isSyncing'])" 2>/dev/null || echo "Not responding"
```

✅ **Checkpoint:** Oracle validators online

---

## Phase 5 Complete! ✅

You should now have:

- [x] Gizzi validator online (V1)
- [x] Audit validator online (V3) OR identified access issue
- [x] 2 Oracle validators contributing to network

**Total Online:** 18 validators (16 Contabo + 2 Oracle)

---

## Final Network Status

```
Total Validators: 21
├── Contabo: 16 ✅ ONLINE
├── Oracle: 2 ✅ ONLINE (or 1 if Audit inaccessible)
└── Azure Sub 2: 0-3 ❓ UNKNOWN

Consensus: 18/21 = 85.7% ✅ EXCELLENT
Threshold: 15/21 = 71.4% ✅ EXCEEDED
```

---

## Troubleshooting Oracle VMs

### Can't SSH into VM
1. **Check VM is running** in Oracle Cloud Console
2. **Verify security list** allows port 22 from your IP
3. **Check firewall** on VM itself (`sudo ufw status`)
4. **Try different user** (ubuntu, opc, gizziio)
5. **Check SSH key** is correct (`~/.ssh/gizzi-validator`)

### Validator won't start
1. **Check binary exists** (`find / -name flarechain-node 2>/dev/null`)
2. **Check chainspec exists** (`find / -name "*chainspec*.json" 2>/dev/null`)
3. **Check logs** (`sudo journalctl -u flarechain-validator -n 100`)
4. **Start manually** to see error messages

### VM appears offline
- May be in "stopped" state in Oracle Console
- Start via Oracle Cloud Console or CLI
- Wait 2-3 minutes for boot
- Try SSH again

---

## Next Steps

With 18/21 validators online, your network is fully operational!

**Optional:**
- Check Azure Subscription 2 status (3 more validators)
- Set up monitoring dashboard
- Configure alerting
- Document final infrastructure

**Recommended:**
- Run weekly health checks
- Monitor costs
- Keep node binary updated
- Regular security patches

---

## Success! 🎉

**MIGRATION COMPLETE**

Your Ëtrid FlareChain mainnet is now running on:
- 16 Contabo validators (€168/month)
- 2 Oracle validators (free tier)
- Total cost: ~$180/month
- Savings: ~$220-320/month vs Azure

**Network Status:** 🟢 FULLY OPERATIONAL

---

**Phase 5 Duration:** ~30 minutes
**Total Migration Time:** 4-6 hours
**Status:** ✅ COMPLETE

