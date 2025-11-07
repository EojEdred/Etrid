# Phase 4: Verify Network Health

**Duration:** 30 minutes
**Goal:** Confirm everything is working correctly

---

## Step 1: Run Comprehensive Health Check (5 minutes)

```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet
bash check-validators-simple.sh
```

**Expected output:**
```
Total Validators: 21
RPC Responding: 16-18  (Contabo + Oracle)
Network Status: ✓ HEALTHY (Supermajority online)

Current Block: #XXXXX
Blockchain is producing and finalizing blocks.
```

✅ **Checkpoint:** Health check shows network operational

---

## Step 2: Verify All Contabo Validators (10 minutes)

Check each validator individually:

```bash
# Create verification script
cat > verify-all-validators.sh <<'EOF'
#!/bin/bash

VMS=(
    "VM01_IP:6"
    "VM02_IP:7"
    "VM03_IP:8"
    "VM04_IP:9"
    "VM05_IP:10"
    "VM06_IP:11"
    "VM07_IP:12"
    "VM08_IP:13"
    "VM09_IP:14"
    "VM10_IP:15"
    "VM11_IP:16"
    "VM12_IP:17"
    "VM13_IP:18"
    "VM14_IP:19"
    "VM15_IP:20"
    "VM16_IP:21"
)

echo "═══════════════════════════════════════════════════════"
echo "ËTRID FLARECHAIN - CONTABO VALIDATOR VERIFICATION"
echo "═══════════════════════════════════════════════════════"
echo ""

for vm in "${VMS[@]}"; do
    IFS=':' read -r ip val_num <<< "$vm"
    echo "Validator $val_num ($ip):"

    # Check RPC
    health=$(curl -s -m 5 -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
        http://$ip:9944 2>/dev/null)

    if echo "$health" | grep -q "result"; then
        peers=$(echo "$health" | python3 -c "import sys,json; print(json.load(sys.stdin)['result']['peers'])" 2>/dev/null || echo "?")
        syncing=$(echo "$health" | python3 -c "import sys,json; print(json.load(sys.stdin)['result']['isSyncing'])" 2>/dev/null || echo "?")
        echo "  ✓ RPC responding"
        echo "  Peers: $peers"
        echo "  Syncing: $syncing"
    else
        echo "  ✗ RPC not responding"
    fi
    echo ""
done

echo "═══════════════════════════════════════════════════════"
EOF

chmod +x verify-all-validators.sh
nano verify-all-validators.sh  # Add your IPs
./verify-all-validators.sh
```

**All validators should show:**
- ✓ RPC responding
- Peers: 8-15
- Syncing: false

✅ **Checkpoint:** All validators verified

---

## Step 3: Test Block Production (5 minutes)

Verify blocks are being produced:

```bash
# Get current block
BLOCK1=$(curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
    http://VM01_IP:9944 | python3 -c "import sys,json; print(int(json.load(sys.stdin)['result']['number'], 16))")

echo "Current block: #$BLOCK1"

# Wait 30 seconds
sleep 30

# Get new block
BLOCK2=$(curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
    http://VM01_IP:9944 | python3 -c "import sys,json; print(int(json.load(sys.stdin)['result']['number'], 16))")

echo "Block after 30s: #$BLOCK2"

# Calculate blocks produced
BLOCKS_PRODUCED=$((BLOCK2 - BLOCK1))
echo "Blocks produced in 30s: $BLOCKS_PRODUCED"
echo "Expected: ~5 blocks (6 second block time)"

if [ $BLOCKS_PRODUCED -ge 3 ]; then
    echo "✓ Block production is working!"
else
    echo "⚠ Block production may be slow or stalled"
fi
```

✅ **Checkpoint:** Blocks being produced

---

## Step 4: Test Finality (5 minutes)

Verify GRANDPA finality is working:

```bash
# Get finalized block
finalized=$(curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getFinalizedHead","params":[],"id":1}' \
    http://VM01_IP:9944)

echo "Finalized block hash:"
echo "$finalized" | python3 -m json.tool

# Get latest block
latest=$(curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"chain_getHeader","params":[],"id":1}' \
    http://VM01_IP:9944)

# Compare block numbers (finalized should be close to latest)
echo ""
echo "Latest block number:"
echo "$latest" | python3 -c "import sys,json; print(int(json.load(sys.stdin)['result']['number'], 16))"
```

**Finality should be within 2-10 blocks of latest.**

✅ **Checkpoint:** Finality working

---

## Step 5: Test Transaction Submission (Optional - 5 minutes)

If you have test accounts:

```bash
# Query account balance
curl -s -X POST -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","method":"system_accountNextIndex","params":["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"],"id":1}' \
    http://VM01_IP:9944 | python3 -m json.tool

# If this returns data, RPC is fully functional
```

✅ **Checkpoint:** RPC fully operational

---

## Step 6: Document New Infrastructure (10 minutes)

Create a record of your new setup:

```bash
cat > /Users/macbook/Desktop/etrid/docs/mainnet/migration_for_mainnet/CONTABO_INFRASTRUCTURE.md <<'EOF'
# Ëtrid FlareChain - Contabo Infrastructure

**Migration Date:** November 7, 2025
**Provider:** Contabo.com
**Total VMs:** 16
**Monthly Cost:** €168 (~$180)

## VM Inventory

| VM | Validator | IP Address | Region | Status |
|----|-----------|------------|--------|--------|
| VM01 | Validator 6 | ___________ | _______ | ✅ Active |
| VM02 | Validator 7 | ___________ | _______ | ✅ Active |
| VM03 | Validator 8 | ___________ | _______ | ✅ Active |
| VM04 | Validator 9 | ___________ | _______ | ✅ Active |
| VM05 | Validator 10 | ___________ | _______ | ✅ Active |
| VM06 | Validator 11 | ___________ | _______ | ✅ Active |
| VM07 | Validator 12 | ___________ | _______ | ✅ Active |
| VM08 | Validator 13 | ___________ | _______ | ✅ Active |
| VM09 | Validator 14 | ___________ | _______ | ✅ Active |
| VM10 | Validator 15 | ___________ | _______ | ✅ Active |
| VM11 | Validator 16 | ___________ | _______ | ✅ Active |
| VM12 | Validator 17 | ___________ | _______ | ✅ Active |
| VM13 | Validator 18 | ___________ | _______ | ✅ Active |
| VM14 | Validator 19 | ___________ | _______ | ✅ Active |
| VM15 | Validator 20 | ___________ | _______ | ✅ Active |
| VM16 | Validator 21 | ___________ | _______ | ✅ Active |

## Access

**SSH Key:** `~/.ssh/contabo-validators`
**User:** root
**Base Path:** `/root/.etrid/validator`
**Binary:** `/usr/local/bin/flarechain-node`
**Chainspec:** `/root/chainspec.json`

## Maintenance Commands

### Check all validators
```bash
cd /Users/macbook/Desktop/etrid/docs/mainnet
bash check-validators-simple.sh
```

### Restart a validator
```bash
ssh -i ~/.ssh/contabo-validators root@VM_IP "systemctl restart flarechain-validator"
```

### View logs
```bash
ssh -i ~/.ssh/contabo-validators root@VM_IP "journalctl -u flarechain-validator -f"
```

### Update node binary
```bash
scp -i ~/.ssh/contabo-validators \
    /path/to/new/flarechain-node \
    root@VM_IP:/usr/local/bin/
ssh -i ~/.ssh/contabo-validators root@VM_IP "systemctl restart flarechain-validator"
```

## Network Status

**Total Validators:** 21
- Contabo: 16 ✅
- Oracle Cloud: 2 ✅
- Azure Sub 2: 0-3 (status unknown)

**Consensus Threshold:** 15/21 ✅ ACHIEVED
**Block Production:** Active
**Finality:** Working

## Cost Savings

**Previous (Azure):** ~$400-500/month
**Current (Contabo):** ~$180/month
**Savings:** ~$220-320/month (45-60%)

---
**Status:** ✅ MIGRATION SUCCESSFUL
**Date:** November 7, 2025
EOF

# Fill in your actual IPs
nano /Users/macbook/Desktop/etrid/docs/mainnet/migration_for_mainnet/CONTABO_INFRASTRUCTURE.md
```

✅ **Checkpoint:** Infrastructure documented

---

## Phase 4 Complete! ✅

You have successfully:

- [x] Verified all 16 Contabo validators online
- [x] Confirmed network consensus achieved
- [x] Tested block production
- [x] Verified finality working
- [x] Documented new infrastructure

**Network Status:** 🟢 FULLY OPERATIONAL

---

## Next Step

**Open:** `05_PHASE_5_Oracle_Validators.md`

That guide will help you start the 2 Oracle Cloud validators to get to 18/21 total.

---

## Success Metrics Summary

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Validators Online | 15+ | 16+ | ✅ |
| Block Production | Active | Yes | ✅ |
| Finality | Working | Yes | ✅ |
| Peer Connections | 8-15 | 8-15 | ✅ |
| RPC Endpoints | Responding | Yes | ✅ |
| Monthly Cost | <$300 | ~$180 | ✅ |

**Migration Status:** ✅ SUCCESS

---

**Phase 4 Duration:** ~30 minutes
**Overall Migration:** COMPLETE 🎉

