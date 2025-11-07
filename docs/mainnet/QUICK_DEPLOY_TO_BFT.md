# Quick Deploy to BFT Threshold - 15 Validators

**Current Status:** 8-9 active validators
**Target:** 15 active validators (GRANDPA supermajority)
**Need:** 7 more validators
**Estimated Time:** 30-45 minutes (parallel deployment)

---

## 🎯 Goal: Reach BFT Byzantine Fault Tolerance

**Why 15 validators?**
- GRANDPA requires **2/3+1 supermajority** = 15 of 21 validators
- Full Byzantine Fault Tolerance (tolerate up to 5 faulty validators)
- ASF committee achieves optimal 3-level consensus
- Network achieves production-grade security

---

## 📋 Prerequisites

1. **7 servers/VMs ready** with:
   - Ubuntu 20.04/22.04
   - 2+ CPU cores
   - 4+ GB RAM
   - 50+ GB storage
   - Public IP addresses
   - SSH access configured

2. **Files ready:**
   - ✅ Binary: `/Users/macbook/Desktop/etrid/target/release/flarechain-node`
   - ✅ Chainspec: `/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw-FIXED.json`
   - ✅ Keys: `/Users/macbook/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json`

3. **Current bootnodes:**
```
/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm
/ip4/129.80.122.34/tcp/30333/p2p/12D3KooWGjGCJzexrJct6nGCSDnj7vaJtMohpagFUPBhPgpZqvpd
```

---

## 🚀 Automated Deployment (Recommended)

### Option A: Deploy Single Validator

```bash
cd /Users/macbook/Desktop/etrid/scripts

# Deploy validator #3 to IP 1.2.3.4
./deploy-validator.sh 3 1.2.3.4 "Validator-3"

# Deploy validator #4 to IP 5.6.7.8
./deploy-validator.sh 4 5.6.7.8 "Validator-4"

# Continue for validators 6-10...
```

**Script does everything:**
1. ✅ Uploads binary and chainspec
2. ✅ Extracts and inserts session keys (AURA, GRANDPA, ASF)
3. ✅ Generates network key
4. ✅ Creates systemd service
5. ✅ Configures firewall
6. ✅ Starts validator
7. ✅ Displays peer ID and bootnode string

### Option B: Deploy Multiple Validators in Parallel

Create a deployment manifest file:

```bash
cat > /tmp/validator-deployment-manifest.txt << 'EOF'
# Format: validator_index server_ip validator_name
3 1.2.3.4 Validator-3
4 5.6.7.8 Validator-4
6 9.10.11.12 Validator-6
7 13.14.15.16 Validator-7
8 17.18.19.20 Validator-8
9 21.22.23.24 Validator-9
10 25.26.27.28 Validator-10
EOF
```

Run parallel deployment:

```bash
cd /Users/macbook/Desktop/etrid/scripts

# Deploy all validators in parallel
while read idx ip name; do
  [[ "$idx" =~ ^# ]] && continue  # Skip comments
  echo "Deploying $name to $ip..."
  ./deploy-validator.sh "$idx" "$ip" "$name" &
done < /tmp/validator-deployment-manifest.txt

# Wait for all deployments to complete
wait

echo "All validators deployed!"
```

---

## 📊 Monitor Deployment Progress

### Check Network Peer Count

```bash
# On AuditDev validator
curl -s http://129.80.122.34:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' | jq

# Expected progression:
# {"peers": 9, ...}  ← After 1st validator
# {"peers": 10, ...} ← After 2nd validator
# {"peers": 11, ...} ← After 3rd validator
# ...
# {"peers": 15, ...} ← TARGET REACHED! 🎉
```

### Watch for BFT Achievement

```bash
# Monitor finality on any validator
ssh ubuntu@129.80.122.34 "sudo journalctl -u flarechain-validator -f | grep -i 'finalized\|supermajority\|authority'"

# Look for:
# ✅ "Finalized #XXXX" messages with consistent 2-4 block lag
# ✅ GRANDPA authority set with 15+ active validators
# ✅ No finality stalls or warnings
```

### Real-time Network Dashboard

```bash
# Create a monitoring script
cat > /tmp/monitor-network.sh << 'EOSCRIPT'
#!/bin/bash
while true; do
  clear
  echo "═══════════════════════════════════════════════════════"
  echo "  FlareChain Network Status - $(date '+%H:%M:%S')"
  echo "═══════════════════════════════════════════════════════"

  HEALTH=$(curl -s http://129.80.122.34:9944 -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"system_health"}')
  PEERS=$(echo $HEALTH | jq -r '.result.peers')
  SYNCING=$(echo $HEALTH | jq -r '.result.isSyncing')

  SYNC=$(curl -s http://129.80.122.34:9944 -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"system_syncState"}')
  BEST=$(echo $SYNC | jq -r '.result.currentBlock')

  FINALIZED=$(curl -s http://129.80.122.34:9944 -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"chain_getFinalizedHead"}')
  FIN_HASH=$(echo $FINALIZED | jq -r '.result')

  FIN_HEADER=$(curl -s http://129.80.122.34:9944 -H "Content-Type: application/json" -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"$FIN_HASH\"]}")
  FIN_NUM=$(echo $FIN_HEADER | jq -r '.result.number' | xargs printf "%d")

  LAG=$((BEST - FIN_NUM))

  echo ""
  echo "Peers:           $PEERS / 21 validators"
  echo "Best Block:      #$BEST"
  echo "Finalized:       #$FIN_NUM"
  echo "Finality Lag:    $LAG blocks"
  echo "Syncing:         $SYNCING"
  echo ""

  if [ "$PEERS" -ge 15 ]; then
    echo "🎉 BFT THRESHOLD REACHED! ($PEERS/21 validators)"
    echo "✅ GRANDPA supermajority active"
  else
    NEEDED=$((15 - PEERS))
    echo "⚠️  Need $NEEDED more validators for BFT threshold"
  fi

  echo ""
  echo "Press Ctrl+C to exit..."
  sleep 5
done
EOSCRIPT

chmod +x /tmp/monitor-network.sh
/tmp/monitor-network.sh
```

---

## 🎯 Success Indicators

### When You Reach 15 Validators:

**Network Health:**
```json
{
  "peers": 15,
  "isSyncing": false,
  "shouldHavePeers": true
}
```

**GRANDPA Finality:**
- Consistent 2-4 block lag
- No finality stalls
- Continuous finalization messages in logs

**Validator Logs Show:**
```
💤 Idle (15 peers), best: #XXXX (0xabcd...), finalized #YYYY (0xef12...), ⬇ 45.2kiB/s ⬆ 38.1kiB/s
✨ Imported #XXXX (0x1234...)
🎁 Prepared block for proposing at XXXX...
✅ Finalized block #YYYY
```

**ASF Committee:**
```
🌐 ASF Committee size: 21
🔒 PPFA round 1 complete
🔗 DETR P2P network: 15 peers connected
```

---

## 📈 Deployment Priority Order

Deploy in this order for optimal network growth:

1. **validator-3** (Director) - High stake, high priority
2. **validator-4** (Director) - High stake, high priority
3. **validator-6** (FlareNode) - Core network validator
4. **validator-7** (FlareNode) - Core network validator
5. **validator-8** (FlareNode) - Core network validator
6. **validator-9** (FlareNode) - Core network validator
7. **validator-10** (FlareNode) - Core network validator

**This reaches 15 total validators (including 8 already active)**

---

## 🔍 Post-Deployment Verification

### 1. Check All Validators Connected

```bash
# On any validator, list all peers
curl -s http://129.80.122.34:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' | jq '.result | length'

# Should show 15+ peers
```

### 2. Verify Session Keys Active

```bash
# On each deployed validator
ssh ubuntu@SERVER_IP "ls -la /home/ubuntu/flarechain-data/chains/flarechain_mainnet/keystore/"

# Should show 3 files (aura, gran, asfk)
```

### 3. Confirm Finality Working

```bash
# Watch finality on main validator
ssh ubuntu@129.80.122.34 "sudo journalctl -u flarechain-validator -n 50 | grep -i finalized"

# Should see recent finalization messages with 2-4 block lag
```

### 4. Check Byzantine Tolerance

With 15 validators:
- **Maximum faulty:** 5 validators (15 - 2/3 × 15 = 5)
- **Minimum honest:** 10 validators
- **Security level:** Production-grade BFT
- **Supermajority:** 2/3+1 = 10+ validators needed for finality

---

## ⚠️ Troubleshooting

**Validator not connecting?**
```bash
# Check service status
ssh ubuntu@SERVER_IP "sudo systemctl status flarechain-validator"

# Check logs for errors
ssh ubuntu@SERVER_IP "sudo journalctl -u flarechain-validator -n 100"

# Common issues:
# - Firewall blocking ports 30333/30334
# - Session keys not inserted correctly
# - Network key missing
# - Incorrect bootnodes
```

**Low peer count?**
```bash
# Verify bootnodes in service
ssh ubuntu@SERVER_IP "sudo systemctl cat flarechain-validator | grep bootnodes"

# Ensure --public-addr is set correctly
ssh ubuntu@SERVER_IP "sudo systemctl cat flarechain-validator | grep public-addr"

# Restart if needed
ssh ubuntu@SERVER_IP "sudo systemctl restart flarechain-validator"
```

**Session key warnings?**
```bash
# Re-insert session keys
ssh ubuntu@SERVER_IP

# Use validator's mnemonic phrase (from validator-keys-complete.json)
/usr/local/bin/flarechain-node key insert \
  --base-path /home/ubuntu/flarechain-data \
  --chain /home/ubuntu/chainspec-mainnet-raw-FIXED.json \
  --key-type aura \
  --scheme sr25519 \
  --suri "mnemonic phrase here"

# Repeat for gran (ed25519) and asfk (sr25519)
```

---

## 🎉 After Reaching 15 Validators

**Congratulations! Your network has achieved BFT threshold!**

### Next Steps:

1. **Monitor for 24 hours** - Ensure stability
2. **Deploy remaining 6 validators** - Reach full 21-validator set
3. **Enable governance** - Activate DAO treasury
4. **Public announcement** - Mainnet is production-ready
5. **Launch user services** - Wallets, block explorers, etc.

### Network Capabilities Unlocked:

✅ **Byzantine Fault Tolerance** - Tolerate up to 5 faulty validators
✅ **GRANDPA Supermajority** - Fast, secure finality
✅ **ASF 3-Level Consensus** - Enhanced security via PPFA
✅ **Production Security** - Ready for mainnet operations
✅ **Governance Ready** - Can process on-chain proposals

---

## 📞 Need Help?

**Quick Reference:**
- Deployment script: `/Users/macbook/Desktop/etrid/scripts/deploy-validator.sh`
- Full guide: `/Users/macbook/Desktop/etrid/docs/mainnet/DEPLOY_REMAINING_VALIDATORS.md`
- Keys file: `/Users/macbook/Desktop/etrid/mainnet-deployment-package/validator-keys-complete.json`
- Bootnode list: `/tmp/flarechain-bootnode-list.txt`

**Common Commands:**
```bash
# Deploy single validator
./deploy-validator.sh <index> <ip> "<name>"

# Check network health
curl -s http://129.80.122.34:9944 -d '{"id":1,"jsonrpc":"2.0","method":"system_health"}' | jq

# Monitor logs
ssh ubuntu@SERVER_IP "sudo journalctl -u flarechain-validator -f"
```

---

**Ready to deploy? Let's reach BFT threshold! 🚀**

Genesis Hash: `0xca40bbf4f8367f63ea110afd54cf5fd38c44df100f9454b62135bfc09df74da8`
