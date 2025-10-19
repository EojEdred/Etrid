# Ëtrid Quick Start Guide

Get Ëtrid multi-node network running in **under 5 minutes**.

---

## ⚡ Quick Commands

### Test Multi-Node Network (Recommended)
```bash
cd /Users/macbook/Desktop/etrid
./scripts/run_multi_validator_test.sh
```
**Result:** 3 validators running (Alice, Bob, Charlie)

### Build All Nodes
```bash
./scripts/build_all_nodes.sh
```
**Time:** ~15-20 minutes
**Output:** 1 FlareChain + 12 PBC collators

### Generate Chain Specs
```bash
./scripts/generate_chain_specs.sh
```
**Output:** Chain specs in `chain-specs/` directory

---

## 🔍 Check Status

### Are Nodes Running?
```bash
ps aux | grep flarechain-node | grep -v grep
```

### View Live Logs
```bash
# Alice's activity
tail -f .validator-test/logs/alice.log | grep -E 'Imported|Authored|peers'

# All validators
tail -f .validator-test/logs/*.log
```

### Query Node via RPC
```bash
# System health
curl -s -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
     http://localhost:9944 | jq

# Latest block
curl -s -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
     http://localhost:9944 | jq '.result.number'

# Peer count
curl -s -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
     http://localhost:9944 | jq '.result | length'
```

---

## 🛑 Stop Network

```bash
pkill -f flarechain-node
```

---

## 📊 Node Endpoints

### FlareChain Validators
- **Alice:**   http://localhost:9944 (Port 30333)
- **Bob:**     http://localhost:9945 (Port 30334)
- **Charlie:** http://localhost:9946 (Port 30335)

### PBC Collators (when running full testnet)
- **BTC:**  http://localhost:8000 (Port 40000)
- **ETH:**  http://localhost:8001 (Port 40001)
- **DOGE:** http://localhost:8002 (Port 40002)

---

## 🚀 What's Running?

When you start `run_multi_validator_test.sh`, you get:

✅ **3 FlareChain validator nodes**
- Alice (Validator 1) with predefined network key
- Bob (Validator 2) with predefined network key
- Charlie (Validator 3) with predefined network key

✅ **ASF Consensus active**
- PPFA block production (~6 second blocks)
- Hybrid finality (ASF + GRANDPA)
- Validator management

✅ **RPC interfaces**
- Full JSON-RPC access on each node
- Can query blockchain state
- Can submit extrinsics

---

## 📚 Documentation

- **Setup Guide:** `MULTI_NODE_TESTING.md`
- **Session Report:** `MULTI_NODE_SUCCESS_REPORT.md`
- **Security Guide:** `NETWORK_KEYS_SECURITY_GUIDE.md`
- **Full Summary:** `SESSION_SUMMARY.md`

---

## ❓ Troubleshooting

### Nodes won't start
```bash
# Check if ports are in use
lsof -i :9944

# Kill existing processes
pkill -f flarechain-node

# Try again
./scripts/run_multi_validator_test.sh
```

### Can't connect to RPC
```bash
# Verify node is running
ps aux | grep flarechain-node

# Check logs for errors
tail -f .validator-test/logs/alice.log
```

### Nodes have 0 peers
**This is expected!** Currently nodes use separate genesis blocks. This is fine for testing individual node functionality. To enable peering, all nodes must use the same chain spec (see `MULTI_NODE_TESTING.md` for details).

---

## 🎯 Next Steps

1. **Test individual nodes** ✅ (This guide)
2. **Fix peer connectivity** - Use shared chain spec
3. **Build with full WASM** - Remove SKIP_WASM_BUILD
4. **Test bridge functionality** - Cross-chain operations

---

## 🔑 Quick Reference

| Action | Command |
|--------|---------|
| Start test network | `./scripts/run_multi_validator_test.sh` |
| Stop all nodes | `pkill -f flarechain-node` |
| View Alice logs | `tail -f .validator-test/logs/alice.log` |
| Check Alice RPC | `curl -d '{"method":"system_health"}' localhost:9944` |
| Count running nodes | `ps aux \| grep flarechain \| wc -l` |
| Build all binaries | `./scripts/build_all_nodes.sh` |

---

**Ready to start?**
```bash
./scripts/run_multi_validator_test.sh
```

Watch the logs, query the RPC, and see Ëtrid come alive! 🚀
