# Chainspec Deployment - Simple Summary

## TL;DR

**The chainspec is just a 2MB JSON file. Copy it to each VM. That's it.**

---

## Quick Answer to Your Question

### ✅ Can it be rebuilt from any terminal with SSH access?

**YES** - But you don't need to rebuild it!

**Option 1: Just copy the file** (RECOMMENDED - 2 minutes)
```bash
scp chainspec-mainnet-raw-FIXED.json user@vm:/var/lib/flarechain/
```

**Option 2: Rebuild on VM** (NOT RECOMMENDED - 20 minutes per VM)
```bash
# Requires: Rust toolchain, repo clone, cargo build
# Total: ~2GB download + 20 min build time
```

---

## Deployment Across 4 Terminals

### Method 1: One command per terminal (Simple)

**Terminal 1: Oracle Cloud VMs**
```bash
for VM in 64.181.215.19 129.80.122.34; do
  scp chainspec-mainnet-raw-FIXED.json ubuntu@$VM:/tmp/ &
  scp flarechain-node ubuntu@$VM:/tmp/ &
done
wait
```

**Terminal 2: Azure VMs**
```bash
for VM in 52.252.142.146 [other_azure_ips]; do
  scp chainspec-mainnet-raw-FIXED.json azureuser@$VM:/tmp/ &
  scp flarechain-node azureuser@$VM:/tmp/ &
done
wait
```

**Terminal 3: AWS VMs**
```bash
for VM in [aws_ips]; do
  scp chainspec-mainnet-raw-FIXED.json ec2-user@$VM:/tmp/ &
  scp flarechain-node ec2-user@$VM:/tmp/ &
done
wait
```

**Terminal 4: Local/Other VMs**
```bash
# EojEdred (your machine)
cp chainspec-mainnet-raw-FIXED.json /var/lib/flarechain/
```

---

### Method 2: Automated script (Even simpler!)

```bash
# Edit deploy-all.sh with your VM IPs
nano docs/mainnet/deploy-all.sh

# Run it (deploys to all 21 VMs in parallel)
./docs/mainnet/deploy-all.sh
```

**Time:** ~2-3 minutes total

---

## What Gets Deployed?

**2 files per VM:**
1. `flarechain-node` (58MB binary)
2. `chainspec-mainnet-raw-FIXED.json` (2MB JSON)

**Locations:**
- Binary: `/usr/local/bin/flarechain-node`
- Chainspec: `/var/lib/flarechain/chainspec-mainnet-raw.json`

---

## Key Points

✅ **All 21 validators use the SAME chainspec file**
- No rebuilding needed
- No customization per validator
- Genesis hash will be identical: `0xca40bbf4f8367f63ea110afd54cf5fd38c44df100f9454b62135bfc09df74da8`

✅ **Validator identity comes from session keys (inserted AFTER node starts)**
- Chainspec defines the network
- Session keys define which validator you are

✅ **Deployment is a simple file copy**
- SCP the 2 files to each VM
- Start the node
- Insert session keys
- Done!

---

## After Deployment

### Start a node:
```bash
ssh ubuntu@64.181.215.19

sudo /usr/local/bin/flarechain-node \
  --base-path /var/lib/flarechain \
  --chain /var/lib/flarechain/chainspec-mainnet-raw.json \
  --name "Gizzi" \
  --validator \
  --port 30333 \
  --rpc-port 9933
```

### Insert session keys:
```bash
# From your local machine
curl -X POST http://64.181.215.19:9933 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "author_insertKey",
    "params": ["gran", "ill easily diesel mixture...", "0xYOUR_GRANDPA_KEY"],
    "id": 1
  }'
```

---

## Verification

Check chainspec is identical on all VMs:

```bash
# On each VM
sha256sum /var/lib/flarechain/chainspec-mainnet-raw.json
```

**All should output the same hash!**

---

## Files Ready for Deployment

Located at:
- `/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json`
- `/Users/macbook/Desktop/etrid/target/release/flarechain-node`

---

**For detailed step-by-step:** See `PARALLEL_DEPLOYMENT.md`
**For node startup commands:** See `QUICK_START.md`
