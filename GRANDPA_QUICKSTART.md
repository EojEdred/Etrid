# FlareChain GRANDPA Fix - Quick Start Guide

## TL;DR - Deploy in 5 Minutes

### What This Does
Fixes GRANDPA finality stuck at genesis by updating committee from 1 to 10 validators.

### Files You Need
1. **WASM Runtime**: `/Users/macbook/Desktop/etrid/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm`
2. **Deployment Script**: `/Users/macbook/Desktop/etrid/deploy-grandpa-fix.js`
3. **Verification Script**: `/Users/macbook/Desktop/etrid/verify-grandpa-upgrade.sh`

---

## Method 1: Automated Script (Easiest)

```bash
# Install dependencies
cd /Users/macbook/Desktop/etrid
npm install @polkadot/api @polkadot/keyring

# Deploy (replace sudo URI with your actual key)
node deploy-grandpa-fix.js \
  --endpoint ws://64.181.215.19:9944 \
  --sudo-uri "YOUR_SUDO_SEED_OR_URI"

# Verify
./verify-grandpa-upgrade.sh http://64.181.215.19:9933
```

**Done!** Script handles everything automatically.

---

## Method 2: Polkadot.js UI (Manual)

1. **Open**: https://polkadot.js.org/apps/
2. **Connect**: `ws://64.181.215.19:9944`
3. **Navigate**: Developer → Extrinsics
4. **Select Account**: Your sudo key
5. **Choose**: `sudo` → `sudoUncheckedWeight(call, weight)`
6. **Set Call**: `system` → `setCode(code)`
7. **Upload WASM**:
   ```
   /Users/macbook/Desktop/etrid/target/release/wbuild/flare-chain-runtime/flare_chain_runtime.compact.compressed.wasm
   ```
8. **Set Weight**:
   - refTime: `2,000,000,000`
   - proofSize: `1,048,576`
9. **Submit Transaction**
10. **Wait for block inclusion**

---

## Verify Success

```bash
# Check spec version (should be 106)
curl -s http://64.181.215.19:9933 -d \
  '{"id":1,"jsonrpc":"2.0","method":"state_getRuntimeVersion"}' | \
  jq .result.specVersion

# Run full verification
./verify-grandpa-upgrade.sh http://64.181.215.19:9933
```

---

## Expected Results

✅ **Immediate** (block N+1):
- Runtime spec_version = 106
- GRANDPA authorities updated to 10 validators

✅ **Within 1-2 blocks**:
- GRANDPA finality resumes
- Finalized block height > 0
- All validators participating

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Transaction fails | Use `sudoUncheckedWeight`, not regular `sudo` |
| spec_version still 105 | Wait 1-2 blocks for inclusion |
| GRANDPA not updated | Check logs, migration may have failed |
| Can't connect to RPC | Check endpoint: `curl http://64.181.215.19:9933/health` |

---

## Monitor Progress

```bash
# Watch finalized blocks
watch -n 2 'curl -s http://64.181.215.19:9933 -d \
  "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getFinalizedHead\"}" | jq'

# Watch validator logs
journalctl -u flarechain-validator -f | grep -i grandpa
```

---

## Rollback (if needed)

If something goes wrong, you can rollback:

```bash
# Stop validator
sudo systemctl stop flarechain-validator

# Restore from backup
sudo cp -r /backup/flarechain/* /var/lib/flarechain/

# Restart
sudo systemctl start flarechain-validator
```

---

## Need Help?

- **Full Guide**: `GRANDPA_RUNTIME_UPGRADE_DEPLOYMENT.md`
- **Summary**: `GRANDPA_FIX_SUMMARY.md`
- **Logs**: `journalctl -u flarechain-validator -f`

---

**Ready to Deploy!** 🚀

Choose Method 1 (automated) or Method 2 (manual) above.
