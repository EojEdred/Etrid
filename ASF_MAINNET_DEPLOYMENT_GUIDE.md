# FlareChain Pure ASF Mainnet Deployment Guide

## 🔴 CRITICAL ISSUE IDENTIFIED

**Problem:** All 20 validators are RUNNING but IDLE (no block production)

**Root Cause:** Validators are using the **WRONG binary and chainspec**:
- ❌ Using: `flarechain-node` binary with `--chain dev`
- ❌ Result: Dev chainspec has NO ASF authorities configured
- ❌ Result: No block production, stuck at genesis block #0

## ✅ SOLUTION

**Correct Configuration:**
- ✅ Binary: `etrid` (unified binary with mainnet support)
- ✅ Chainspec: `flarechain_production_raw.json` (Pure ASF with 21 validators)
- ✅ Result: ASF consensus active, block production starts

## 📋 What Was Generated

### 1. ASF Mainnet Raw Chainspec
**File:** `flarechain_production_raw.json`

**Configuration:**
- **Chain Name:** Ëtrid FlareChain Mainnet (Pure ASF)
- **Chain ID:** flarechain_mainnet_v1
- **Chain Type:** Live
- **Consensus Mode:** pure_asf
- **Block Production:** PPFA
- **Finality:** ASF

**Validators:** 21 total
- 9 DecentralizedDirector (128 ETR stake each)
- 12 ValidityNode (128 ETR stake each)

### 2. Deployment Script
**File:** `deploy_asf_mainnet.sh`

**What it does:**
1. Uploads new `etrid` binary to all 20 validators
2. Uploads ASF raw chainspec
3. Stops old `flarechain-node` processes
4. Generates network keys (if needed)
5. Creates/updates systemd service
6. Starts validators with correct configuration
7. Verifies successful startup

## 🚀 DEPLOYMENT STEPS

### Step 1: Verify Prerequisites

```bash
# Check binary exists
ls -lh ./target/release/etrid
# Should show: 56M (built Nov 16 18:03)

# Check chainspec exists
ls -lh ./flarechain_production_raw.json
# Should show: 205 lines

# Check deployment script
ls -lh ./deploy_asf_mainnet.sh
# Should show: 8.4K, executable
```

### Step 2: Test on Single Validator (RECOMMENDED)

```bash
# Deploy to just val-1 first
ssh ubuntu@146.190.136.56 << 'EOF'
    # Stop old node
    sudo systemctl stop flarechain-node
    sudo pkill -9 flarechain-node

    # Upload files manually (or use scp)
    # scp ./target/release/etrid ubuntu@146.190.136.56:/tmp/
    # scp ./flarechain_production_raw.json ubuntu@146.190.136.56:/tmp/

    # Install
    sudo mv /tmp/etrid /usr/local/bin/etrid
    sudo chmod +x /usr/local/bin/etrid
    sudo mkdir -p /etc/etrid
    sudo mv /tmp/flarechain_production_raw.json /etc/etrid/

    # Generate node key
    openssl rand -hex 32 | sudo tee /etc/etrid/node-key.secret

    # Start manually to test
    sudo /usr/local/bin/etrid \
      --chain /etc/etrid/flarechain_production_raw.json \
      --validator \
      --base-path /var/lib/etrid \
      --node-key $(cat /etc/etrid/node-key.secret) \
      --name "FlareChain-Val-1-Test"
EOF

# Monitor logs
ssh ubuntu@146.190.136.56 'sudo journalctl -u etrid-validator -f'
```

**What to look for:**
- ✅ Genesis block initialized
- ✅ ASF consensus started
- ✅ PPFA proposer initialized
- ✅ "ASF FlareChain node started successfully"
- ✅ Network identity created
- ✅ DETR P2P network started

### Step 3: Deploy to All Validators

```bash
# Run deployment script
./deploy_asf_mainnet.sh

# When prompted, type: yes

# Script will:
# - Deploy to all 20 validators in sequence
# - Show progress for each validator
# - Display final summary
```

### Step 4: Verify Block Production

```bash
# Check first validator (should start producing blocks)
curl -s -X POST http://146.190.136.56:9944 -H "Content-Type: application/json" \
  -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' | jq

# Expected response:
# {
#   "jsonrpc": "2.0",
#   "result": {
#     "isSyncing": false,
#     "peers": 19,
#     "shouldHavePeers": true
#   },
#   "id": 1
# }

# Check block number (should be increasing)
curl -s -X POST http://146.190.136.56:9944 -H "Content-Type: application/json" \
  -d '{"id":1,"jsonrpc":"2.0","method":"chain_getHeader","params":[]}' | jq '.result.number'

# Expected: Should show increasing block numbers
```

### Step 5: Monitor Network Health

```bash
# Check all validator logs
for i in {1..20}; do
    echo "=== Validator $i ==="
    ssh ubuntu@val-$i 'sudo journalctl -u etrid-validator -n 20 --no-pager' | grep -E "(block|finalized|ASF|PPFA)"
done

# Check peer connectivity
for i in {1..20}; do
    echo "Validator $i:"
    curl -s -X POST http://val-$i:9944 -H "Content-Type: application/json" \
      -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' | jq '.result.peers'
done
```

## 🔧 TROUBLESHOOTING

### Issue: Validator won't start

**Check logs:**
```bash
ssh ubuntu@val-1 'sudo journalctl -u etrid-validator -n 100'
```

**Common issues:**
1. **NetworkKeyNotFound**: Node key not generated
   ```bash
   ssh ubuntu@val-1 'ls -la /etc/etrid/node-key.secret'
   # If missing: openssl rand -hex 32 | sudo tee /etc/etrid/node-key.secret
   ```

2. **Chainspec not found**: File not uploaded
   ```bash
   ssh ubuntu@val-1 'ls -la /etc/etrid/flarechain_production_raw.json'
   ```

3. **Binary not found**: Not installed correctly
   ```bash
   ssh ubuntu@val-1 'which etrid'
   ssh ubuntu@val-1 '/usr/local/bin/etrid --version'
   ```

### Issue: No block production

**Verify ASF authorities:**
```bash
curl -s -X POST http://val-1:9944 -H "Content-Type: application/json" \
  -d '{"id":1,"jsonrpc":"2.0","method":"state_getStorage","params":["0x..."]}' | jq
```

**Check if validator has keys in keystore:**
```bash
ssh ubuntu@val-1 'ls -la /var/lib/etrid/chains/flarechain_mainnet_v1/keystore/'
```

**Insert ASF keys if needed:**
```bash
# Generate ASF keys for validator
ssh ubuntu@val-1 '/usr/local/bin/etrid key generate --scheme sr25519'

# Insert into keystore
ssh ubuntu@val-1 '/usr/local/bin/etrid key insert \
  --base-path /var/lib/etrid \
  --chain /etc/etrid/flarechain_production_raw.json \
  --scheme sr25519 \
  --suri "YOUR_SEED_PHRASE_HERE" \
  --key-type asf'
```

### Issue: Validators not connecting to each other

**Check boot nodes:**
```bash
# The chainspec should have boot nodes configured
# If empty, add manually to systemd service:
--bootnodes /ip4/146.190.136.56/tcp/30333/p2p/PEER_ID_HERE
```

**Get peer ID:**
```bash
ssh ubuntu@val-1 'sudo journalctl -u etrid-validator | grep "Local node identity"'
```

## 📊 SUCCESS CRITERIA

After deployment, you should see:

✅ All 20 validators running (systemctl status etrid-validator)
✅ Block production active (blocks increasing every ~6 seconds)
✅ Validators connected to each other (19 peers each)
✅ ASF finality working (blocks being finalized)
✅ No error messages in logs
✅ Prometheus metrics available on port 9615

## 🎯 NEXT STEPS AFTER DEPLOYMENT

1. **Monitor for 24 hours** - Ensure stability
2. **Set up alerting** - Prometheus + Grafana
3. **Document node keys** - Backup securely
4. **Add boot nodes** - Improve peer discovery
5. **Enable telemetry** - Send to monitoring server

## 📞 SUPPORT

If issues persist:
1. Check validator logs: `sudo journalctl -u etrid-validator -f`
2. Verify chainspec is correct: `cat /etc/etrid/flarechain_production_raw.json | jq`
3. Test local node: `./target/release/etrid --chain flarechain_production_raw.json --validator --tmp --node-key=$(openssl rand -hex 32)`

---

**Generated:** 2025-11-16
**Version:** v1.0
**Status:** Ready for Production Deployment
