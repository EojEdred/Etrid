# Ëtrid FlareChain - PBC Deployment Guide
## Activating Validators 6-21 as Validity Nodes

**Date:** 2025-11-03
**Status:** 🎯 READY TO DEPLOY
**Action:** Deploy PBCs and assign validators

---

## Executive Summary

**GREAT NEWS:** Your PBC infrastructure is already built!

**14 PBC Chains Available:**
1. btc-pbc (Bitcoin Bridge)
2. eth-pbc (Ethereum Bridge)
3. sol-pbc (Solana Bridge)
4. xrp-pbc (Ripple Bridge)
5. bnb-pbc (Binance Chain Bridge)
6. trx-pbc (Tron Bridge)
7. edsc-pbc (ËDSC Stablecoin) 🌟
8. ada-pbc (Cardano Bridge)
9. matic-pbc (Polygon Bridge)
10. link-pbc (Chainlink Bridge)
11. sc-usdt-pbc (USDT Stablecoin)
12. doge-pbc (Dogecoin Bridge)
13. xlm-pbc (Stellar Bridge)

**16 Validators Available:**
- Validators 6-21 waiting to be assigned

**Deployment Strategy:**
- Each PBC needs 8 validators (PPFA rotation)
- Start with 2 PBCs: **edsc-pbc** and **btc-pbc**
- Use all 16 validators (8 per PBC)

---

## PBC Architecture

### What is a PBC?

**PBC = Partition Burst Chain** (Polkadot terminology: "Parachain")

```
┌─────────────────────────────────────────────────────────┐
│  FLARECHAIN (Relay Chain / Main Chain)                  │
├─────────────────────────────────────────────────────────┤
│  Directors 1-5 (Flare Nodes)                            │
│  - Coordinate PBC checkpoints                           │
│  - Finalize cross-chain messages                        │
│  - Process governance votes                             │
└───────┬──────────────────────────────────────┬──────────┘
        │                                      │
        ▼                                      ▼
┌───────────────────┐              ┌───────────────────┐
│  EDSC-PBC         │              │  BTC-PBC          │
├───────────────────┤              ├───────────────────┤
│  Validators 6-13  │              │  Validators 14-21 │
│  (8 Validity Nodes│              │  (8 Validity Nodes)
│                   │              │                   │
│  - ËDSC minting   │              │  - BTC deposits   │
│  - ËDSC redemption│              │  - BTC withdrawals│
│  - Oracle pricing │              │  - Light client   │
│  - Collateral mgmt│              │  - Bridge state   │
└───────────────────┘              └───────────────────┘
```

### PBC Consensus: PPFA (Partition Proof of Authority)

**From Ivory Paper Section 9.1 (Lines 765-771):**
> "Rotation: 8 validators per PBC, rotate every 256 blocks"

**How PPFA Works:**
```
Epoch = 256 blocks (~51 minutes at 12s/block)

Block Production Schedule:
Block 1-32:   Validator 6
Block 33-64:  Validator 7
Block 65-96:  Validator 8
Block 97-128: Validator 9
... etc for all 8 validators ...

After 256 blocks: Checkpoint to FlareChain
Then: Rotate validator set if needed
```

**Benefits:**
- Predictable block production
- Each validator gets equal opportunity
- No competition for slots (deterministic)
- Fair reward distribution

---

## Deployment Plan

### Phase 1: Build PBC Collators

**Status:** PBC code exists, needs compilation

```bash
cd /Users/macbook/Desktop/etrid

# Build EDSC-PBC collator
cargo build --release -p edsc-pbc-collator

# Build BTC-PBC collator
cargo build --release -p btc-pbc-collator

# Expected output:
# target/release/edsc-pbc-collator
# target/release/btc-pbc-collator
```

**Build Time:** ~30-60 minutes per collator

**Result:** Two binary executables ready to deploy

---

### Phase 2: Generate PBC Chainspecs

**Purpose:** Define PBC genesis configuration

```bash
# Generate EDSC-PBC chainspec
./target/release/edsc-pbc-collator build-spec \
    --chain dev \
    --raw \
    --disable-default-bootnode \
    > chainspec-edsc-pbc-raw.json

# Edit chainspec to configure:
# 1. Initial validators (validators 6-13)
# 2. Initial endowments
# 3. Session keys
# 4. Relay chain connection (FlareChain)

# Generate BTC-PBC chainspec
./target/release/btc-pbc-collator build-spec \
    --chain dev \
    --raw \
    --disable-default-bootnode \
    > chainspec-btc-pbc-raw.json

# Edit for validators 14-21
```

**Key Configuration:**
```json
{
  "name": "ËDSC PBC",
  "id": "edsc-pbc",
  "chainType": "Live",
  "bootNodes": [
    "/ip4/20.224.104.239/tcp/30335/p2p/PEER_ID_VAL6",
    "/ip4/51.142.203.160/tcp/30335/p2p/PEER_ID_VAL7"
  ],
  "genesis": {
    "runtime": {
      "session": {
        "keys": [
          ["VALIDATOR_6_ACCOUNT", "SESSION_KEYS_VAL6"],
          ["VALIDATOR_7_ACCOUNT", "SESSION_KEYS_VAL7"],
          ... (8 total validators)
        ]
      },
      "parachainInfo": {
        "parachainId": 2000  // EDSC-PBC ID
      }
    }
  }
}
```

---

### Phase 3: Deploy PBC Collators to Validators

**For each validator, deploy corresponding PBC collator**

#### EDSC-PBC Deployment (Validators 6-13)

```bash
# Example: Deploy to Validator 6
ssh -i ~/.ssh/etrid_vm1 audit-dev01@20.224.104.239

# Upload collator binary
scp -i ~/.ssh/etrid_vm1 \
    target/release/edsc-pbc-collator \
    audit-dev01@20.224.104.239:/home/audit-dev01/

# Upload chainspec
scp -i ~/.ssh/etrid_vm1 \
    chainspec-edsc-pbc-raw.json \
    audit-dev01@20.224.104.239:/home/audit-dev01/

# Make binary executable
chmod +x /home/audit-dev01/edsc-pbc-collator

# Create systemd service
sudo tee /etc/systemd/system/edsc-pbc-collator.service << 'EOF'
[Unit]
Description=ËDSC PBC Collator Node
After=network.target

[Service]
Type=simple
User=audit-dev01
WorkingDirectory=/home/audit-dev01
ExecStart=/home/audit-dev01/edsc-pbc-collator \
    --collator \
    --chain /home/audit-dev01/chainspec-edsc-pbc-raw.json \
    --base-path /home/audit-dev01/.etrid/edsc-pbc \
    --port 30335 \
    --rpc-port 9945 \
    --ws-port 9946 \
    --prometheus-port 9616 \
    --name "Validator-6-EDSC" \
    --public-addr /ip4/20.224.104.239/tcp/30335 \
    -- \
    --chain /home/audit-dev01/chainspec.json \
    --port 30333 \
    --rpc-port 9944 \
    --execution wasm

Restart=always
RestartSec=10
LimitNOFILE=10000

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable edsc-pbc-collator
sudo systemctl start edsc-pbc-collator

# Verify
sudo systemctl status edsc-pbc-collator
sudo journalctl -u edsc-pbc-collator -f
```

**Important Flags:**
- `--collator`: Run as PBC collator
- `--chain chainspec-edsc-pbc-raw.json`: PBC chainspec
- `--port 30335`: PBC p2p port (different from FlareChain's 30333)
- `--rpc-port 9945`: PBC RPC port (different from FlareChain's 9944)
- `--`: Separator for relay chain args
- Second `--chain chainspec.json`: FlareChain chainspec (for relay connection)
- Second `--port 30333`: FlareChain connection port

**Repeat for all validators 6-13**

---

#### BTC-PBC Deployment (Validators 14-21)

Same process, but:
- Use `btc-pbc-collator` binary
- Use `chainspec-btc-pbc-raw.json`
- Different port set (30336, 9947, 9948)
- Service name: `btc-pbc-collator`

---

### Phase 4: Generate PBC Session Keys

**For each validator, generate session keys for their PBC**

```bash
# On Validator 6 (EDSC-PBC)
ssh -i ~/.ssh/etrid_vm1 audit-dev01@20.224.104.239

# Generate session keys
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"author_rotateKeys",
    "params":[]
}' http://localhost:9945

# Output: "0x1234abcd5678..." (hex-encoded session keys)
# Save this output - you'll need it for Phase 5
```

**Repeat for all 16 validators**

**Save output in a file:**
```bash
# Create keys mapping file
cat > pbc_session_keys.txt << EOF
Validator 6:  0x1234abcd...
Validator 7:  0x5678efgh...
Validator 8:  0x9012ijkl...
... (16 total)
EOF
```

---

### Phase 5: Insert PBC Session Keys

**Method 1: Via RPC (Recommended)**

```bash
# For Validator 6 (EDSC-PBC)
SESSION_KEYS="0x1234abcd..."  # From Phase 4 output

curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"author_insertKey\",
    \"params\":[
        \"aura\",
        \"your_seed_phrase_here\",
        \"$SESSION_KEYS\"
    ]
}" http://20.224.104.239:9945

# Insert GRANDPA key
curl -H "Content-Type: application/json" -d "{
    \"id\":1,
    \"jsonrpc\":\"2.0\",
    \"method\":\"author_insertKey\",
    \"params\":[
        \"gran\",
        \"your_seed_phrase_here\",
        \"$GRANDPA_KEY\"
    ]
}" http://20.224.104.239:9945
```

**Method 2: Pre-insert in Chainspec**

Already done if you configured session keys in Phase 2 chainspec.

---

### Phase 6: Register PBCs on FlareChain

**PBCs must be registered on FlareChain relay chain**

```bash
# Connect to FlareChain (Directors 1-5)
# Submit sudo extrinsic to register PBC

# For EDSC-PBC (ID: 2000)
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"author_submitExtrinsic",
    "params":[
        "0x..."  // Hex-encoded registrar.register extrinsic
    ]
}' http://20.69.26.209:9944

# Registration includes:
# - ParaId: 2000 (EDSC-PBC)
# - Genesis head: First PBC block hash
# - Validation code: WASM runtime
```

**Alternatively via Polkadot.js Apps:**
1. Connect to FlareChain RPC
2. Developer → Sudo → registrar.register
3. Submit with Director keys

---

### Phase 7: Start Block Production

**After registration, PBC collators should start producing blocks**

```bash
# Monitor Validator 6 logs
ssh -i ~/.ssh/etrid_vm1 audit-dev01@20.224.104.239
sudo journalctl -u edsc-pbc-collator -f

# Expected log output:
# ✓ Initialized PBC collator
# ✓ Connected to relay chain
# ✓ Imported relay chain block #12500
# ✓ Starting collation for block #1
# 🔨 Prepared block for proposing at 1
# ✓ Block finalized: #1 (0x1234...)
```

**Success indicators:**
- "Prepared block for proposing"
- "Block finalized"
- No errors about relay chain connection
- Checkpoint blocks submitted to FlareChain every 256 blocks

---

## Validator Assignment Strategy

### Option A: Two PBCs (Recommended for Mainnet)

**EDSC-PBC (Validators 6-13):**
- Most critical: ËDSC stablecoin operations
- High transaction volume expected
- Needs dedicated validator set
- 8 validators rotating via PPFA

**BTC-PBC (Validators 14-21):**
- Second most important: Bitcoin bridge
- High value transactions
- Needs strong security
- 8 validators rotating via PPFA

**Advantages:**
- Clear separation of duties
- Each PBC gets full 8-validator set
- Optimal security and decentralization
- Easier to monitor and troubleshoot

---

### Option B: Four PBCs (Maximum Utilization)

**EDSC-PBC (Validators 6-9):**
- 4 validators (reduced from 8)
- Still secure (4/4 = 100% BFT within PBC)

**BTC-PBC (Validators 10-13):**
- 4 validators

**ETH-PBC (Validators 14-17):**
- 4 validators
- Ethereum bridge operations

**SOL-PBC (Validators 18-21):**
- 4 validators
- Solana bridge operations

**Advantages:**
- 4 PBCs active simultaneously
- More bridges operational
- Higher total network throughput

**Disadvantages:**
- Lower validator count per PBC
- Reduced redundancy
- More complex management

---

### Recommendation: Start with Option A

1. **Week 1-2:** Deploy EDSC-PBC and BTC-PBC only
2. **Week 3:** Monitor performance, stability, rewards
3. **Week 4:** If stable, add ETH-PBC and SOL-PBC
4. **Month 2:** Expand to remaining PBCs as more validators join

---

## Verification & Testing

### Check PBC Registration

```bash
# Query FlareChain for registered PBCs
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"state_call",
    "params":["ParachainHost_parachains", "0x"],
    "id":1
}' http://20.69.26.209:9944

# Expected output:
# {"result": "0x..."}  // Hex-encoded list: [2000, 2001]
# Decoded: ParaId 2000 (EDSC-PBC), ParaId 2001 (BTC-PBC)
```

---

### Check PBC Block Production

```bash
# Query EDSC-PBC for current block
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"chain_getHeader",
    "params":[],
    "id":1
}' http://20.224.104.239:9945

# Expected output:
# {
#   "result": {
#     "number": "0x64",  // Block #100
#     "parentHash": "0x...",
#     "stateRoot": "0x...",
#     ...
#   }
# }
```

---

### Check Validator Participation

```bash
# Check session validators on EDSC-PBC
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"state_call",
    "params":["SessionApi_validators", "0x"],
    "id":1
}' http://20.224.104.239:9945

# Should return 8 validator accounts (validators 6-13)
```

---

### Check Checkpoints to FlareChain

```bash
# Every 256 blocks, PBC submits checkpoint to FlareChain
# Check FlareChain for PBC checkpoints

curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"chain_getBlock",
    "params":[],
    "id":1
}' http://20.69.26.209:9944 | jq '.result.block.extrinsics[]' | grep -i para

# Look for extrinsics from PBC collators
```

---

## Troubleshooting

### PBC Not Producing Blocks

**Symptoms:**
- Validator 6 running but logs show "Waiting for relay chain"
- No "Prepared block" messages

**Solutions:**
```bash
# 1. Check relay chain connection
sudo journalctl -u edsc-pbc-collator | grep -i "relay"

# Should see:
# ✓ Connected to relay chain
# ✓ Imported relay chain block #12345

# 2. Verify PBC is registered
# Query FlareChain registrar (see verification section)

# 3. Check if session keys are inserted
curl -H "Content-Type: application/json" -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method":"author_hasSessionKeys",
    "params":["0x..."],
    "id":1
}' http://localhost:9945

# Should return: {"result": true}
```

---

### Relay Chain Connection Failed

**Symptoms:**
- "Could not connect to relay chain"
- "RPC error: Connection refused"

**Solutions:**
```bash
# 1. Ensure FlareChain node is running
pgrep -a flarechain-node

# 2. Check FlareChain RPC is accessible
curl -H "Content-Type: application/json" -d '{
    "method":"system_health"
}' http://localhost:9944

# 3. Update PBC collator command with correct relay chain RPC
# In systemd service:
#   --relay-chain-rpc-url ws://localhost:9944

# 4. Restart PBC collator
sudo systemctl restart edsc-pbc-collator
```

---

### No Rewards Appearing

**Symptoms:**
- PBC producing blocks but validators not receiving rewards

**Cause:** Distribution Pay not configured for PBC validators yet

**Solution:**
```bash
# Rewards configured on FlareChain
# Check if Distribution Pay includes W% for Validity Nodes
# From Ivory Paper: W% of annual mint goes to PBC validators

# Query FlareChain for Validity Node rewards schedule
# May need governance proposal to activate rewards
```

---

## Security Considerations

### Key Management

**Each validator needs:**
1. **FlareChain session keys** (if they're also a Director)
2. **PBC session keys** (for their assigned PBC)
3. **Different seed phrases** for each

**Best practice:**
```bash
# Generate separate seed phrases
# Validator 6 FlareChain: "deer camera upper..."
# Validator 6 EDSC-PBC:   "ocean bridge mountain..." (DIFFERENT!)

# Store in secure vault (not same file)
```

---

### Firewall Rules

**For PBC collators, open:**
```bash
# PBC p2p port (30335 for EDSC, 30336 for BTC)
sudo ufw allow 30335/tcp

# PBC RPC port (only if public RPC needed)
# sudo ufw allow 9945/tcp  # Be cautious!

# FlareChain p2p port (for relay connection)
sudo ufw allow 30333/tcp
```

---

### Monitoring

**Monitor both FlareChain and PBC:**
```bash
# FlareChain node status
systemctl status flarechain-validator

# EDSC-PBC collator status
systemctl status edsc-pbc-collator

# Combined monitoring script
#!/bin/bash
echo "=== FlareChain ==="
curl -s http://localhost:9944 -d '{"method":"system_health"}' | jq .

echo "=== EDSC-PBC ==="
curl -s http://localhost:9945 -d '{"method":"system_health"}' | jq .
```

---

## Deployment Checklist

### Pre-Deployment

- [ ] Confirm 16 validators (6-21) available and accessible via SSH
- [ ] Verify FlareChain (relay chain) is running and synced
- [ ] Backup all validator data
- [ ] Document current FlareChain session keys (don't lose Director keys!)
- [ ] Allocate disk space (~100GB per PBC)

### Build Phase

- [ ] Build edsc-pbc-collator binary
- [ ] Build btc-pbc-collator binary
- [ ] Test binaries locally (--dev mode)
- [ ] Generate PBC chainspecs
- [ ] Configure initial validator set in chainspecs

### Deployment Phase (Per Validator)

- [ ] Upload PBC collator binary
- [ ] Upload PBC chainspec
- [ ] Create systemd service
- [ ] Generate PBC session keys
- [ ] Insert PBC session keys
- [ ] Start PBC collator service
- [ ] Verify logs show "Connected to relay chain"

### Registration Phase

- [ ] Register EDSC-PBC on FlareChain (ParaId 2000)
- [ ] Register BTC-PBC on FlareChain (ParaId 2001)
- [ ] Verify registration via RPC query
- [ ] Wait for PBC to start producing blocks

### Verification Phase

- [ ] All 8 EDSC-PBC validators producing blocks
- [ ] All 8 BTC-PBC validators producing blocks
- [ ] Checkpoints submitting to FlareChain every 256 blocks
- [ ] No errors in validator logs
- [ ] RPC endpoints responding
- [ ] Monitor for 24-48 hours

---

## Timeline Estimate

| Phase | Duration | Cumulative |
|-------|----------|------------|
| **Build collators** | 1-2 hours | 2 hours |
| **Generate chainspecs** | 1 hour | 3 hours |
| **Deploy to 16 validators** | 4 hours | 7 hours |
| **Generate session keys** | 1 hour | 8 hours |
| **Insert session keys** | 1 hour | 9 hours |
| **Register PBCs on FlareChain** | 1 hour | 10 hours |
| **Verify block production** | 2 hours | 12 hours |
| **Monitor stability** | 24-48 hours | 2-3 days |

**Total:** 2-3 days from start to stable production

---

## Next Steps

**Immediate (Today):**
1. ✅ Read this guide
2. ✅ Decide on validator assignment (Option A or B)
3. ✅ Start building PBC collators

**Tomorrow:**
1. Generate PBC chainspecs
2. Deploy to first 2 validators as test
3. Verify block production

**This Week:**
1. Deploy to all 16 validators
2. Register PBCs on FlareChain
3. Monitor for stability

**Next Week:**
1. Verify rewards distribution
2. Monitor performance metrics
3. Plan for additional PBCs (ETH, SOL, etc.)

---

## Support Resources

**Code Locations:**
- PBC Runtimes: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains/`
- PBC Collators: `/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-node/`
- Bridge Protocols: `/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/`

**Deployment Scripts (TBD):**
- `deploy-pbc.sh`: Automated deployment script
- `verify-pbc.sh`: Health check script
- `assign-validators.sh`: Validator assignment automation

---

**Status:** Ready to deploy PBCs and activate Validity Nodes

**Next Action:** Build PBC collators with `cargo build --release -p edsc-pbc-collator`
