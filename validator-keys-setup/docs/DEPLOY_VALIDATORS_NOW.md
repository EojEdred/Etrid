# Deploy Validators - Final Instructions

## Current Status

✅ **PPFA Authorization Fix** - Applied to code (stores authorizations during block import)
✅ **Local Testnet Genesis Fix** - Code modified to use `local_testnet_config()` instead of empty chainspec
✅ **MIN_COMMITTEE_SIZE** - Still 4 (correct for production 21 validators)
✅ **Binaries Built** - Fixed binary exists on both VMs

## Problem: CLI Design Conflict

The multichain CLI has a design flaw - TWO `--chain` parameters defined:
1. Custom chain type selector (flare, btc-pbc, etc.)
2. Substrate's chainspec file parameter

This causes: `error: the argument '--chain <CHAIN>' cannot be used multiple times`

## Solution: Use Build-Spec Generated Chainspec

Since the binary CAN'T load chainspec files directly due to CLI conflict, use build-spec to generate the genesis:

### On VM2:

```bash
ssh etrid-validator-02@172.177.44.73

# 1. Generate raw chainspec using dev mode (has proper runtime WASM)
cd ~/etrid-build
source ~/.cargo/env
./target/release/etrid build-spec --raw --chain dev > ~/etrid-validator/testnet.json

# Verify it has runtime (should be ~1.8MB)
ls -lh ~/etrid-validator/testnet.json

# 2. Start Bob using the generated chainspec file path
cd ~/etrid-validator
pkill -f etrid || true
rm -rf /tmp/bob

./target/release/etrid \
  --chain testnet.json \
  --validator \
  --bob \
  --base-path /tmp/bob \
  --port 30333 \
  --rpc-port 9945 \
  --unsafe-rpc-external \
  --rpc-methods=unsafe \
  --rpc-cors all \
  --log ppfa=debug,asf=debug \
  > bob.log 2>&1 &

# 3. Get Bob's peer ID
sleep 5
grep "Local node identity" bob.log
# Copy the peer ID (e.g., 12D3KooW...)
```

### Copy Chainspec to VM1:

```bash
# From VM2, copy to VM1's shared location
scp ~/etrid-validator/testnet.json etrid-validator-01@172.16.0.5:~/etrid-validator/
# OR copy via your local machine if internal SSH not configured
```

### On VM1:

```bash
ssh etrid-validator-01@172.16.0.5

cd ~/etrid-validator
pkill -f etrid || true
rm -rf /tmp/alice

# Use the SAME chainspec file from VM2
./etrid \
  --chain testnet.json \
  --validator \
  --alice \
  --base-path /tmp/alice \
  --port 30333 \
  --rpc-port 9945 \
  --unsafe-rpc-external \
  --rpc-methods=unsafe \
  --rpc-cors all \
  --bootnodes /ip4/172.16.0.4/tcp/30333/p2p/<BOB_PEER_ID> \
  --log ppfa=debug,asf=debug \
  > alice.log 2>&1 &
```

## Verification

### Check logs:
```bash
# On VM2
tail -f ~/etrid-validator/bob.log | grep -E "imported|PPFA|✅|❌"

# On VM1
tail -f ~/etrid-validator/alice.log | grep -E "imported|PPFA|✅|❌"
```

### Verify same chain (block hashes must match):
```bash
# On both VMs, run:
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlockHash", "params":[10]}' \
  http://localhost:9945
```

## Expected Results

✅ Both validators show "Ëtrid FlareChain Development" chain
✅ Block production alternates between Alice and Bob
✅ Both import each other's blocks
✅ PPFA authorization logs appear: `📝 Recording PPFA authorization`
✅ Block hashes match at same height
✅ No "No PPFA authorization found" errors
✅ GRANDPA finalization working

## If Still Getting Errors

The root issue is the CLI design - it needs to be fixed to not have conflicting `--chain` parameters.

**Quick workaround:** Use `--dev` mode which bypasses the chain selector entirely, BUT this creates separate genesis per invocation (will fork). So you'd need to:
1. Start Alice with --dev first
2. Export Alice's chainspec
3. Use that exported spec for both validators

## Network Issues

If Azure network is unstable (connection resets during uploads):
- Wait for Azure Front Door recovery (check Azure status)
- Build directly on each VM instead of transferring binaries
- Use smaller compressed transfers

## Files Ready

- **Local binary**: `/Users/macbook/Desktop/etrid/target/release/etrid` (59MB, with fixes)
- **VM2 binary**: `~/etrid-build/target/release/etrid` (69MB, with local testnet fix)
- **Next fix needed**: Resolve CLI --chain parameter conflict in production code
