# Start ËTRID Validator Nodes - Quick Guide

## Current Situation
✅ Port 9944 is open on both Azure VMs
❌ No blockchain node software is running
📊 Connection test shows: "Connection timeout" or "ECONNREFUSED"

## What You Need

You have the validator startup scripts from earlier in this session:
- `/var/lib/etrid/bootstrap-validator.sh` - Key generation
- `/var/lib/etrid/start-validator.sh` - Node startup
- Both VMs: Alice (20.186.91.207) and Bob (172.177.44.73)

## Quick Start: Get Nodes Running Now

### Option 1: Use Existing Scripts (Recommended)

SSH into each VM and check if scripts exist:

```bash
# Connect to VM #1 (Alice)
ssh azureuser@20.186.91.207

# Check if scripts exist
ls -la /var/lib/etrid/

# If scripts exist, bootstrap and start:
cd /var/lib/etrid
sudo ./bootstrap-validator.sh alice
sudo ./start-validator.sh alice

# If scripts DON'T exist, see Option 2 below
```

Then repeat for VM #2 (Bob):
```bash
ssh azureuser@172.177.44.73
cd /var/lib/etrid
sudo ./bootstrap-validator.sh bob
sudo ./start-validator.sh bob
```

### Option 2: Manual Node Startup (If Scripts Missing)

If the scripts aren't on the VMs, you need to either:

**A) Upload the scripts** from earlier in this session:
```bash
# From your local machine
scp /path/to/bootstrap-validator.sh azureuser@20.186.91.207:/tmp/
scp /path/to/start-validator.sh azureuser@20.186.91.207:/tmp/

# Then SSH and move them
ssh azureuser@20.186.91.207
sudo mkdir -p /var/lib/etrid
sudo mv /tmp/*.sh /var/lib/etrid/
sudo chmod +x /var/lib/etrid/*.sh
cd /var/lib/etrid
sudo ./bootstrap-validator.sh alice
sudo ./start-validator.sh alice
```

**B) Run the node binary directly** (if already compiled):
```bash
# Find the etrid binary
which etrid
# or
find /usr/local/bin -name "etrid*"
# or
find ~ -name "etrid" -type f 2>/dev/null

# Once found, start it:
./etrid \
  --base-path /var/lib/etrid/data \
  --chain mainnet \
  --name "Alice" \
  --validator \
  --rpc-cors all \
  --rpc-external \
  --ws-external \
  --rpc-methods Unsafe \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944
```

**C) Use Docker** (if etrid image exists):
```bash
docker run -d \
  --name etrid-validator \
  -p 30333:30333 \
  -p 9933:9933 \
  -p 9944:9944 \
  -v /var/lib/etrid:/data \
  etrid/node:latest \
  --base-path /data \
  --chain mainnet \
  --name "Alice" \
  --validator \
  --ws-external \
  --rpc-external \
  --rpc-cors all
```

## Verification Checklist

After starting, verify everything is working:

```bash
# 1. Check if process is running
ps aux | grep etrid

# 2. Check if port 9944 is listening
sudo netstat -tulpn | grep 9944
# Should show: tcp 0.0.0.0:9944 LISTEN

# 3. Check logs
sudo journalctl -u etrid-node -f
# or if running manually:
tail -f /var/lib/etrid/node.log

# 4. Test locally from the VM
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params":[]}' \
     http://localhost:9944

# 5. Re-run connection test from your browser
open /Users/macbook/Desktop/etrid/test-node-connection.html
```

## Common Issues & Fixes

### Issue: "etrid: command not found"
**Solution**: Node binary isn't compiled yet
```bash
# Need to build from source first
cd /path/to/etrid/source
cargo build --release
sudo cp target/release/etrid /usr/local/bin/
```

### Issue: "Permission denied"
**Solution**: Need sudo
```bash
sudo chmod +x /var/lib/etrid/*.sh
sudo chown -R azureuser:azureuser /var/lib/etrid/
```

### Issue: Port 9944 already in use
**Solution**: Kill existing process
```bash
sudo lsof -ti:9944 | xargs sudo kill -9
```

### Issue: Chain won't sync / no peers
**Solution**: Add bootnode addresses
```bash
# Need at least one peer
# Use the other VM as bootnode after getting its peer ID
```

## Alternative: Deploy Without Blockchain (Works Now)

If you want to deploy everything **right now** without waiting for nodes:

✅ **Governance portal already works in preview mode**
✅ **Whitepaper viewer works standalone**
✅ **All other apps work locally**

Just upload to Hostinger as-is:
```bash
cd /Users/macbook/Desktop/etrid/hostinger-upload
zip -r governance-preview.zip governance-standalone/
# Upload and extract to /public_html/governance/
```

The governance portal will show:
- ✅ Countdown to Consensus Day 2026
- ✅ Preview statistics
- ✅ Full UI/UX
- 📝 Banner: "Preview Mode - Mainnet launching Q1 2026"

## Where Are We?

**Current Stage**: Pre-Mainnet
**Target**: Q1 2026 Mainnet Launch
**What Works Now**: All UI/UX, countdown timers, preview data
**What Needs Blockchain**: Live voting, real proposals, wallet connections

You can deploy everything now and switch to blockchain mode later when nodes are fully set up!

---

**Next Step**: Either start the nodes OR deploy in preview mode (both are valid!)
