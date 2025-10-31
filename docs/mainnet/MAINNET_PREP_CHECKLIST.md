# Ëtrid FlareChain Mainnet Preparation Checklist

**Purpose:** Complete checklist for mainnet launch preparation

---

## 📋 Pre-Launch Checklist

### Phase 1: Foundation & Governance (DO THIS NOW)

- [ ] **Create Foundation Multisig (5-of-7)**
  ```bash
  ./create-foundation-multisig.sh
  # This will guide you through creating the multisig address
  ```
  - Uses the 7 foundation signers already generated
  - Generates multisig address for sudo key
  - ⚠️ **MUST DO BEFORE building mainnet binary**

- [ ] **Update Genesis Config with Multisig**
  ```bash
  # Edit flarechain_mainnet_genesis.json
  # Replace "sudo.key" value with your multisig address
  nano flarechain_mainnet_genesis.json
  ```

### Phase 2: Bootnode Configuration (DO THIS NOW)

- [ ] **Configure Bootnodes**
  ```bash
  ./configure-bootnodes.sh
  ```
  - Extracts peer IDs for Validators 1-3
  - Creates public BOOTNODES.md, bootnodes.txt, bootnodes.json
  - ⚠️ **These files MUST be public** (website, repo, docs)

- [ ] **Publish Bootnode Information**
  - [ ] Add BOOTNODES.md to GitHub repo
  - [ ] Add to website (etrid.io/mainnet/bootnodes)
  - [ ] Include in node setup documentation
  - [ ] Add to README.md

### Phase 3: Build Mainnet Binary

- [ ] **Copy Genesis Config to Runtime**
  ```bash
  cp flarechain_mainnet_genesis.json \
     05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json
  ```

- [ ] **Build Mainnet Binary**
  ```bash
  cd 05-multichain/flare-chain/node
  cargo build --release --locked
  ```
  - Takes 15-30 minutes
  - Binary: `target/release/flarechain-node`

- [ ] **Generate Raw Chain Spec**
  ```bash
  ./target/release/flarechain-node build-spec \
    --chain mainnet --raw > flarechain-mainnet-raw.json
  ```

- [ ] **Verify Chain Spec Includes Bootnodes**
  ```bash
  grep "bootNodes" flarechain-mainnet-raw.json
  # Should show your bootnode multiaddrs
  ```

### Phase 4: Prepare Deployment Package

- [ ] **Create Deployment Package**
  ```bash
  mkdir -p mainnet-deployment-package
  cp target/release/flarechain-node mainnet-deployment-package/
  cp flarechain-mainnet-raw.json mainnet-deployment-package/
  cp validator-keys-setup/generated-keys/generated-keys-gizzi-eoj/validator-keys-complete.json \
     mainnet-deployment-package/
  tar -czf mainnet-deployment.tar.gz mainnet-deployment-package/
  ```

### Phase 5: Provision All 21 Validator VMs

- [ ] **Provision VMs**
  - [ ] Validator 1 (Gizzi) - 64.181.215.19 ✅ Already provisioned
  - [ ] Validator 2 (EojEdred) - `<IP TBD>`
  - [ ] Validators 3-21 - `<IPs TBD>`

- [ ] **Create VM List**
  ```bash
  # Create validator-vms-numbered.txt
  cat > validator-vms-numbered.txt << 'EOF'
  1 ubuntu@64.181.215.19
  2 ubuntu@<validator-2-ip>
  3 ubuntu@<validator-3-ip>
  # ... (all 21 validators)
  EOF
  ```
  - ⚠️ **This file is INTERNAL only** (not public)
  - Used by deployment scripts to coordinate SSH

- [ ] **Verify SSH Access to All VMs**
  ```bash
  while read num vm; do
    echo "Testing $vm..."
    ssh -i ~/.ssh/gizzi-validator "$vm" "echo OK"
  done < validator-vms-numbered.txt
  ```

### Phase 6: Deploy to All Validators

- [ ] **Run Deployment Script**
  ```bash
  ./deploy-mainnet-to-all-validators.sh
  ```
  - Uploads binary + chain spec to all 21 VMs
  - Installs binaries
  - Inserts session keys (AURA, GRANDPA, ASF)
  - Creates systemd services
  - Takes ~10-15 minutes

- [ ] **Verify Deployment on Each Validator**
  ```bash
  # Check binary installed
  ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
    "/usr/local/bin/flarechain-node --version"

  # Check keys inserted
  ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
    "ls -la /var/lib/etrid/chains/flarechain_mainnet/keystore/"
  # Should show 3 key files
  ```

### Phase 7: Coordinated Launch

- [ ] **Coordinate Launch Time**
  - Schedule specific UTC time (e.g., "2024-11-15 18:00:00 UTC")
  - Notify all stakeholders
  - Prepare monitoring

- [ ] **Start All Validators**
  ```bash
  ./start-all-validators.sh
  # Option 1: Immediate start
  # Option 2: Scheduled start (recommended)
  ```

- [ ] **Monitor Launch**
  ```bash
  ./monitor-mainnet.sh
  # Real-time dashboard of all 21 validators
  ```

- [ ] **Verify Consensus Achieved**
  - [ ] At least 15 validators online (>2/3 for GRANDPA)
  - [ ] Blocks being produced
  - [ ] Blocks being finalized
  - [ ] Peer connections established

### Phase 8: Post-Launch Verification

- [ ] **Verify Network Health**
  ```bash
  # Check block production
  curl -s -H 'Content-Type: application/json' \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
    http://64.181.215.19:9944 | jq .

  # Check finalized block
  curl -s -H 'Content-Type: application/json' \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getFinalizedHead"}' \
    http://64.181.215.19:9944 | jq .
  ```

- [ ] **Test Governance**
  - [ ] Submit test proposal
  - [ ] Test multisig operations
  - [ ] Verify sudo access works

- [ ] **Test EDSC Functionality**
  - [ ] Oracle price feeds
  - [ ] Minting/burning operations
  - [ ] Reserve vault operations

- [ ] **Monitor for 24 Hours**
  - [ ] Check validator logs for errors
  - [ ] Monitor block production rate
  - [ ] Watch for network issues
  - [ ] Check Polkadot telemetry

---

## 🔐 Security Checklist

- [ ] **Foundation Multisig**
  - [ ] 5-of-7 threshold configured
  - [ ] All 7 signers have access to their keys
  - [ ] Test multisig execution before mainnet

- [ ] **Validator Keys**
  - [ ] All session keys backed up
  - [ ] Payment accounts secured
  - [ ] Network keys protected

- [ ] **Genesis Accounts**
  - [ ] All 54 account backups verified
  - [ ] Recovery tests passed (54/54)
  - [ ] Offline backups created

- [ ] **Infrastructure**
  - [ ] Firewalls configured (port 30333 open)
  - [ ] DDoS protection enabled
  - [ ] Monitoring alerts set up

---

## 📊 Critical Information to Publish

### Public Information (MUST publish before/at launch)

1. **Bootnodes** (BOOTNODES.md)
   - Publish on website
   - Add to GitHub repo
   - Include in docs

2. **Chain Spec**
   - `flarechain-mainnet-raw.json` (public)
   - Allows anyone to sync with mainnet

3. **Genesis Hash**
   - Calculate and publish
   - Allows verification of network identity

4. **RPC Endpoints** (if providing public access)
   - wss://rpc.etrid.io
   - https://rpc.etrid.io

5. **Explorer Links**
   - Polkadot.js Apps URL
   - Block explorer URL

### Private Information (NEVER publish)

1. **validator-vms-numbered.txt**
   - Internal coordination only
   - Contains VM addresses

2. **Session Key Seeds**
   - Keep encrypted offline
   - Only use for key recovery

3. **Genesis Account Private Keys**
   - Especially multisig signer keys
   - Treasury and fund keys

---

## 🎯 Success Criteria

Mainnet is successfully launched when:

- ✅ All 21 validators online
- ✅ Consensus achieved (>15 validators)
- ✅ Blocks producing every 6 seconds
- ✅ Blocks finalizing (GRANDPA finality)
- ✅ All validators connected (>10 peers each)
- ✅ No errors in validator logs
- ✅ Telemetry showing healthy network
- ✅ Community able to connect via bootnodes

---

## 🚨 What NOT to Do

- ❌ **Don't** build binary before creating multisig
- ❌ **Don't** keep bootnode info private
- ❌ **Don't** publish validator-vms-numbered.txt
- ❌ **Don't** publish private keys or seeds
- ❌ **Don't** start validators one-by-one (must coordinate)
- ❌ **Don't** skip backup verification
- ❌ **Don't** rush - double-check everything

---

## 📞 Emergency Contacts

If mainnet launch encounters issues:

1. Stop all validators: `ssh <validator> "sudo systemctl stop flarechain-validator"`
2. Investigate logs: `sudo journalctl -u flarechain-validator -n 100`
3. Fix issue and restart coordinated launch
4. Network can be restarted as long as genesis hash stays same

---

**Current Status:** 🔧 Pre-launch preparation

**Next Steps:**
1. Create Foundation multisig → Update genesis config
2. Configure bootnodes → Publish bootnode info
3. Build mainnet binary → Generate chain spec
4. Provision remaining VMs (20 more needed)
5. Deploy to all 21 validators
6. Coordinate launch

---

**Last Updated:** $(date)
