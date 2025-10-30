# Ëtrid 21-Validator Quick Reference Card

**Print this page and keep it handy during deployment!**

---

## 🎯 Core Stats

- **Validators Needed:** 21 (to prevent panic)
- **AI Devs Operating:** 12 (some run multiple validators)
- **Keys Per Validator:** 4 (Session, Payment, Controller, DevID)
- **Total Keys to Manage:** 84
- **Deployment Time:** 3-4 weeks
- **Annual Cost:** $25K (optimized Azure setup)
- **Annual Revenue:** ~$315K (estimated at $0.05/ËTR)

---

## 📁 Documentation Map

```
START HERE → 21_VALIDATORS_COMPLETE_DEPLOYMENT_PLAN.md (this is the master plan)
    ├─ Option Analysis → DEPLOYMENT_DECISION_MATRIX.md
    ├─ Azure Setup → AZURE_21_VALIDATOR_DEPLOYMENT.md
    ├─ Payment+DevID → VALIDATOR_AIDEVID_PAYMENT_INTEGRATION.md
    └─ Scripts:
        ├─ generate-validators-with-payment-aidevid.sh
        ├─ quick-start-21-validators.sh
        └─ monitor-validator-rewards.sh
```

---

## 🔑 Key Types Reference

| Type | Purpose | Storage | Used | Security |
|------|---------|---------|------|----------|
| **Session** | Consensus | VM keystore | Every 6s | HOT ⚠️ |
| **Payment** | Rewards | Hardware wallet | Quarterly | COLD ❄️ |
| **Controller** | Management | Azure Key Vault | Monthly | WARM 🔥 |
| **AI DevID** | Identity | Encrypted file | Periodic | WARM 🔥 |

---

## 🤖 Validator → AI DevID Mapping

| Validators | AI DevID | Count | Stake | Role |
|-----------|----------|-------|-------|------|
| 01 | governance-dev01 | 1 | 128 ËTR | Director |
| 02 | security-dev01 | 1 | 128 ËTR | Director |
| 03 | audit-dev01 | 1 | 128 ËTR | Director |
| 04-05 | consensus-dev01 | 2 | 64 ËTR | FlareNode |
| 06-07 | runtime-dev01 | 2 | 64 ËTR | FlareNode |
| 08-09 | compiler-dev01 | 2 | 64 ËTR | FlareNode |
| 10-11 | multichain-dev01 | 2 | 64 ËTR | FlareNode |
| 12 | oracle-dev01 | 1 | 64 ËTR | FlareNode |
| 13-14 | edsc-dev01 | 2 | 64 ËTR | ValidityNode |
| 15-16 | economics-dev01 | 2 | 64 ËTR | ValidityNode |
| 17-18 | ethics-dev01 | 2 | 64 ËTR | ValidityNode |
| 19-21 | docs-dev01 | 3 | 64 ËTR | ValidityNode |

**Total Stake:** 1,536 ËTR

---

## ⚡ Quick Start Commands

```bash
# 1. Generate all keys (10 minutes)
./scripts/generate-validators-with-payment-aidevid.sh

# 2. Deploy to Azure (45 minutes)
export KEYVAULT_NAME="etrid-val-keys-$(date +%s | tail -c 5)"
./scripts/quick-start-21-validators.sh

# 3. Verify deployment
ssh azureuser@validator-01-ip
sudo journalctl -u etrid-validator -f | grep "committee"

# 4. Monitor rewards
./scripts/monitor-validator-rewards.sh
```

---

## 💰 Genesis Allocations

```
Payment accounts:  21 × 1M ËTR    = 21M ËTR
Controller accounts: 21 × 100K ËTR = 2.1M ËTR
Session accounts:  21 × 10K ËTR   = 210K ËTR
─────────────────────────────────────────────
Total genesis:                      23.31M ËTR
```

---

## 🔐 Azure Key Vault Naming

```bash
# Secrets stored per validator (7 per validator × 21 = 147 secrets)

validator-01-session-seed          # Hot (consensus)
validator-01-session-phrase        # Backup
validator-01-payment-seed          # Cold (rewards)
validator-01-payment-phrase        # Backup
validator-01-payment-account       # Address
validator-01-controller-seed       # Warm (management)
validator-01-controller-account    # Address
validator-01-aidevid               # DID (did:etrid:governance-dev01)
validator-01-aidevid-pubkey        # Public key
validator-01-aura-pubkey           # For chain spec
validator-01-grandpa-pubkey        # For chain spec

# Repeat for validators 02-21...
```

---

## 🚨 Emergency Contacts

**Key Vault Name:** (Set during deployment)
```bash
echo $KEYVAULT_NAME
```

**Backup Locations:**
1. Azure Key Vault: `etrid-val-keys-XXXXX`
2. Encrypted USB: `/path/to/usb/validator-keys-complete.json.gpg`
3. Paper backup: Bank vault (safety deposit box)

**Recovery Commands:**
```bash
# Recover from USB
gpg -d /path/to/usb/validator-keys-complete.json.gpg > keys.json

# Recover from Key Vault
az keyvault secret show \
  --vault-name $KEYVAULT_NAME \
  --name "validator-01-session-seed" \
  --query value -o tsv
```

---

## 📊 Health Check Commands

```bash
# Check all validators online (run daily)
for i in {01..21}; do
  echo -n "validator-$i: "
  ssh azureuser@validator-$i-ip "systemctl is-active etrid-validator"
done

# Check committee size (should be 21)
curl -s http://validator-01-ip:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "etrid_getCommittee"}' \
  | jq '.result | length'

# Check block height (should increment every 6s)
curl -s http://validator-01-ip:9944 \
  -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
  | jq '.result.block.header.number'

# Check payment balance (validator-01)
PAYMENT_ACCOUNT=$(az keyvault secret show \
  --vault-name $KEYVAULT_NAME \
  --name "validator-01-payment-account" \
  --query value -o tsv)

curl -s http://validator-01-ip:9944 \
  -H "Content-Type: application/json" \
  -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_accountBalance\", \"params\": [\"$PAYMENT_ACCOUNT\"]}" \
  | jq '.result.free'
```

---

## 🛡️ Security Checklist

**Before Deployment:**
- [ ] Azure Key Vault created (HSM-backed)
- [ ] Key Vault access restricted (Managed Identity only)
- [ ] NSG rules configured (P2P: 30333, RPC: internal only)
- [ ] Firewall rules set (SSH from your IP only)

**After Key Generation:**
- [ ] Backup to encrypted USB (GPG)
- [ ] Store USB in fireproof safe
- [ ] Print payment phrases
- [ ] Store paper backup in bank vault
- [ ] Delete local `validator-keys-complete.json` (shred -u)

**Ongoing:**
- [ ] Weekly Key Vault audit log review
- [ ] Monthly disaster recovery test
- [ ] Quarterly security patch updates
- [ ] Annual penetration testing

---

## 📈 Performance Targets

| Metric | Target | Alert If |
|--------|--------|----------|
| Uptime | >99.5% | <99% |
| Peer count | 20+ | <10 |
| Block production | 100% of slots | <95% |
| Finality votes | 100% | <95% |
| Disk usage | <70% | >80% |
| CPU usage | <60% | >80% |
| Memory usage | <70% | >85% |

---

## 💸 Monthly Costs (Optimized)

```
Azure VMs (21 × Standard_B4ms):   $1,200/month
Storage (21 × 500GB Standard SSD): $800/month
Key Vault + networking:            $150/month
───────────────────────────────────────────
Subtotal:                          $2,150/month

Reserved instances (-30%):         -$360/month
───────────────────────────────────────────
Total:                             $1,790/month

Annual:                            ~$21,480/year
```

---

## 🎯 Deployment Milestones

- **Week 1:** Keys + Infrastructure → `generate-validators-with-payment-aidevid.sh`
- **Week 2:** VMs + Configuration → `quick-start-21-validators.sh`
- **Week 3:** Testing + AI DevID registration
- **Week 4:** Production launch 🚀

---

## ⚠️ Common Pitfalls

1. **Forgetting to backup keys** → Encrypt USB immediately
2. **Wrong chain spec** → Must include payment mappings
3. **Missing Key Vault access** → Grant Managed Identity permissions
4. **Firewall blocking P2P** → Allow port 30333
5. **Not enough genesis balance** → Fund all 3 account types
6. **Deleting keys before backup** → ALWAYS backup first
7. **Using same seed for payment+session** → Generate separately

---

## 📞 Get Help

**Discord:** #validators channel
**Email:** eoj@etrid.network
**Docs:** See master plan in `21_VALIDATORS_COMPLETE_DEPLOYMENT_PLAN.md`

---

## ✅ Success Indicators

When you see these, you're good:
```
✓ 21 VMs running (Azure portal)
✓ "Committee formed with 21 members" (validator logs)
✓ Blocks incrementing every 6 seconds (RPC query)
✓ Payment balances increasing (reward monitor)
✓ All DevIDs registered (on-chain query)
✓ Prometheus metrics healthy (Grafana dashboard)
```

---

**Print this page and pin it to your wall!**
**Questions? → `21_VALIDATORS_COMPLETE_DEPLOYMENT_PLAN.md`**
