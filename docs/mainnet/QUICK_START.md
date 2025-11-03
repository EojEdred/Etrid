# Ëtrid FlareChain Mainnet - Quick Start Guide

**Status:** ✅ READY TO DEPLOY  
**Date:** November 2, 2025

---

## Files You Need

```
/Users/macbook/Desktop/etrid/target/release/flarechain-node  (58MB binary)
/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json  (2.0MB chainspec)
/Users/macbook/Desktop/etrid/secrets/validator-keys/  (session keys for all 21 validators)
```

---

## 🚀 Launch Sequence

### Step 1: Start Bootstrap Validator (Gizzi)

**Server:** 64.181.215.19 (Oracle Cloud)

```bash
# Copy files to server
scp flarechain-node gizzi@64.181.215.19:/usr/local/bin/
scp chainspec-mainnet-raw-FIXED.json gizzi@64.181.215.19:/var/lib/flarechain/

# SSH into server
ssh gizzi@64.181.215.19

# Start node
/usr/local/bin/flarechain-node \
  --base-path /var/lib/flarechain \
  --chain /var/lib/flarechain/chainspec-mainnet-raw-FIXED.json \
  --name "Gizzi" \
  --validator \
  --rpc-cors all \
  --rpc-methods=Unsafe \
  --rpc-external \
  --ws-external \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
```

### Step 2: Insert Session Keys (Gizzi)

**Mnemonic:** `ill easily diesel mixture urge gauge health kitchen brother uniform come equip`

```bash
# AURA key
curl -H "Content-Type: application/json" -d '{
  "id":1, "jsonrpc":"2.0", "method": "author_insertKey",
  "params":["aura","ill easily diesel mixture urge gauge health kitchen brother uniform come equip","0x44f5ed22b0372d4822bcd0c3a0cad74a29ca5c7e9ee3cc50e8f59fa491620b58"]
}' http://localhost:9933

# GRANDPA key
curl -H "Content-Type: application/json" -d '{
  "id":1, "jsonrpc":"2.0", "method": "author_insertKey",
  "params":["gran","ill easily diesel mixture urge gauge health kitchen brother uniform come equip","0x00ee75f5f1fdf647006e7408c5e9c7ca98afcb2fcd9ae66503dcacaf71427a85"]
}' http://localhost:9933

# ASF key
curl -H "Content-Type: application/json" -d '{
  "id":1, "jsonrpc":"2.0", "method": "author_insertKey",
  "params":["imon","ill easily diesel mixture urge gauge health kitchen brother uniform come equip","0x44f5ed22b0372d4822bcd0c3a0cad74a29ca5c7e9ee3cc50e8f59fa491620b58"]
}' http://localhost:9933
```

### Step 3: Get Gizzi's Node ID

```bash
# Check logs for node ID (looks like: 12D3KooW...)
tail -f /var/lib/flarechain/chains/flarechain_mainnet/network/secret_ed25519
```

Or check via RPC:
```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' http://localhost:9933
```

### Step 4: Start Remaining Bootstrap Validators

Use Gizzi's node ID as bootnode:

```bash
./flarechain-node \
  --validator \
  --chain chainspec-mainnet-raw-FIXED.json \
  --name "EojEdred" \
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/<GIZZI_NODE_ID>
```

Repeat for:
- governance-dev01
- security-dev01 (52.252.142.146)
- audit-dev01 (129.80.122.34)

### Step 5: Verify Network

```bash
# Check block production
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' http://localhost:9933

# Check peer count
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9933

# Check finalization
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getFinalizedHead"}' http://localhost:9933
```

---

## ✅ Success Indicators

- [ ] All 5 bootstrap validators connected
- [ ] Blocks being produced (every 6 seconds)
- [ ] GRANDPA finality working (blocks being finalized)
- [ ] Telemetry showing on Polkadot telemetry
- [ ] No errors in logs

---

## 📊 Network Stats

- **21 Validators Total** (5 bootstrap + 16 standard)
- **Block Time:** 6 seconds
- **Total Supply:** 2.521 Billion ETR
- **SS58 Format:** 42
- **Chain ID:** flarechain_mainnet

---

## 🆘 Troubleshooting

### Node won't start
- Check chainspec path is correct
- Verify binary has execute permissions: `chmod +x flarechain-node`
- Check logs: `journalctl -u flarechain -f`

### Keys won't insert
- Ensure RPC is accessible: `--rpc-external --rpc-methods=Unsafe`
- Verify mnemonic is correct
- Check RPC port is open: `netstat -tlnp | grep 9933`

### No peers connecting
- Verify firewall allows port 30333
- Check bootnode address is correct
- Ensure DNS resolves correctly

### Blocks not finalizing
- All 21 validators must have GRANDPA keys inserted
- Check validator set: `curl ... method: "grandpa_authorities"`
- Verify at least 15 validators are online (supermajority)

---

## 📚 Full Documentation

- **Deployment Guide:** `DEPLOYMENT_READY_STATUS.md`
- **Technical Analysis:** `RAW_CHAINSPEC_ISSUE_ANALYSIS.md`
- **Session Summary:** `SESSION_SUMMARY.md`
- **Validator Keys:** `/Users/macbook/Desktop/etrid/secrets/validator-keys/`

---

## 🔐 Security Notes

- **NEVER share session key mnemonics publicly**
- Use firewall rules to restrict RPC access
- Disable `--rpc-methods=Unsafe` after key insertion
- Monitor sudo key usage (DAO Treasury)

---

**Next:** Once all 5 bootstrap validators are running and producing blocks, proceed with configuring the remaining 16 validators.

🚀 **Let's launch!**

---

**Generated:** November 2, 2025  
**Network:** Ëtrid FlareChain Mainnet
