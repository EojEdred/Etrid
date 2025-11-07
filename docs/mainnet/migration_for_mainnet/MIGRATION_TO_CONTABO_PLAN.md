# Ëtrid FlareChain - Migration from Azure to Contabo

**Date:** November 7, 2025
**Status:** 🔄 URGENT - Azure subscription locked, migration required
**Target:** Contabo.com or alternative cloud provider

---

## 🚨 Current Situation

### Problem
- **Azure Subscription:** LOCKED/DISABLED due to payment issue
- **Impact:** All 17 Azure VMs are stopped/deallocated
- **Validators Affected:** 16 out of 21 validators offline
- **Network Status:** Cannot achieve consensus (need 15/21 minimum)

### What's Still Running
✅ **Oracle Cloud (2 VMs):**
- V1-Gizzi: 64.181.215.19
- V3-Audit: 129.80.122.34

✅ **Azure Subscription 2 (3 VMs):** *(May also be affected - needs verification)*
- V0B-EojEdred: 20.69.26.209
- V1-Governance: 20.186.91.207
- V2-Security: 52.252.142.146

❌ **Azure Subscription 1 (16 VMs):** OFFLINE
- All West Europe VMs (5)
- All North Europe VMs (2)
- All UK South VMs (5)
- All France Central VMs (4)

---

## ✅ YES - Migration to Contabo is Possible

### Good News
1. **IP addresses DON'T need to migrate** - FlareChain uses dynamic peer discovery
2. **Validator identity is in session keys** - not tied to IP addresses
3. **Blockchain state is portable** - can copy data directories
4. **Configuration is simple** - just update `--public-addr` flag

### What You CAN'T Migrate
❌ Azure public IP addresses (owned by Azure)
❌ Azure-specific configurations
❌ Azure NSG rules

### What You CAN Migrate
✅ **Validator session keys** (stored in secrets folder)
✅ **Blockchain data** (can sync from scratch or copy)
✅ **Node binary** (`flarechain-node`)
✅ **Chainspec file** (`chainspec-mainnet-raw.json`)
✅ **Validator configuration** (systemd service files)

---

## 📋 Migration Strategy

### Option A: Quick Migration (Minimal Downtime)
**Timeline:** 2-4 hours
**Approach:** Deploy new VMs, sync from scratch

**Steps:**
1. Provision 16 VMs on Contabo (or any provider)
2. Install node binary and chainspec on each
3. Copy session keys to appropriate VMs
4. Start validators with updated `--public-addr` flags
5. Validators will sync from existing network

**Pros:**
- Fast deployment
- No data transfer needed
- Clean installation

**Cons:**
- Validators need to sync blockchain history
- Takes 30-60 minutes per validator to fully sync

---

### Option B: Full Migration with Data (Longer but Complete)
**Timeline:** 4-8 hours
**Approach:** Copy blockchain data from Azure (if accessible)

**Steps:**
1. If Azure VMs can be temporarily started:
   - Start VMs long enough to copy blockchain data
   - Use rsync/scp to transfer `.etrid/validator` directories
   - Shut down Azure VMs
2. Provision 16 VMs on Contabo
3. Deploy copied blockchain data
4. Update configurations with new IPs
5. Start validators

**Pros:**
- No blockchain sync needed
- Immediate full node operation

**Cons:**
- Requires Azure VMs to be accessible
- Large data transfer (10-50 GB per validator)
- Depends on Azure subscription being temporarily unlocked

---

## 🖥️ Contabo Server Requirements

### Per Validator VM Specs

**Minimum (for testing):**
- 2 vCPU cores
- 4 GB RAM
- 100 GB SSD storage
- 1 public IPv4 address
- 1 Gbps network

**Recommended (for production):**
- 4 vCPU cores
- 8 GB RAM
- 200 GB NVMe SSD
- 1 public IPv4 address
- 1+ Gbps network

### Contabo VPS Options

**VPS S (€5.50/month):**
- 4 vCPU cores
- 6 GB RAM
- 100 GB NVMe
- ⚠️ Might be tight for storage

**VPS M (€10.50/month):**
- 6 vCPU cores
- 12 GB RAM
- 200 GB NVMe
- ✅ **RECOMMENDED**

**VPS L (€18.50/month):**
- 8 vCPU cores
- 24 GB RAM
- 400 GB NVMe
- ✅ Good for high-traffic validators

### Cost Calculation

**16 validators × €10.50/month = €168/month (~ $180/month)**

Compare to Azure (approximately $300-500/month for 16 VMs)
**Savings: ~$200-300/month**

---

## 🔧 Technical Migration Details

### What Changes in Configuration

**OLD (Azure VM):**
```bash
--public-addr /ip4/20.224.104.239/tcp/30333
```

**NEW (Contabo VM):**
```bash
--public-addr /ip4/NEW_CONTABO_IP/tcp/30333
```

**That's it!** Everything else stays the same.

### Firewall Requirements on Contabo

```bash
# Required ports
sudo ufw allow 22/tcp      # SSH
sudo ufw allow 30333/tcp   # P2P networking
sudo ufw allow 9944/tcp    # RPC (optional, can restrict to monitoring server)
sudo ufw allow 9615/tcp    # Prometheus metrics (optional)
sudo ufw enable
```

### Session Keys DON'T Change

Your validator identities are cryptographic keys, NOT IP addresses:
- **AURA keys** (block production)
- **GRANDPA keys** (finality)
- **ASF keys** (committee consensus)

These stay the same regardless of IP address.

---

## 📝 Step-by-Step Migration Guide

### Phase 1: Provision New Infrastructure (30 minutes)

1. **Create Contabo Account**
   - Sign up at contabo.com
   - Add payment method

2. **Order 16 VPS instances**
   - Select VPS M (or preferred tier)
   - Choose regions (distribute for geographic diversity)
   - Select Ubuntu 22.04 LTS
   - Note down all assigned IP addresses

3. **Initial Setup on Each VM**
   ```bash
   # Update system
   sudo apt update && sudo apt upgrade -y

   # Install dependencies
   sudo apt install -y curl wget git build-essential ufw

   # Configure firewall
   sudo ufw allow 22/tcp
   sudo ufw allow 30333/tcp
   sudo ufw allow 9944/tcp
   sudo ufw enable

   # Create user (if needed)
   sudo useradd -m -s /bin/bash etrid
   ```

---

### Phase 2: Deploy Validator Software (1-2 hours)

1. **Copy Node Binary to Each VM**
   ```bash
   # On your local machine
   scp /Users/macbook/Desktop/etrid/target/release/flarechain-node \
       root@NEW_IP:/usr/local/bin/

   # On remote VM
   sudo chmod +x /usr/local/bin/flarechain-node
   ```

2. **Copy Chainspec File**
   ```bash
   scp /Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json \
       root@NEW_IP:/home/etrid/
   ```

3. **Copy Validator Session Keys**
   ```bash
   # Each validator has specific keys
   # Check /Users/macbook/Desktop/etrid/secrets/validator-keys/

   # Copy appropriate key files to each VM
   scp validator-X-keys/* root@NEW_IP:/home/etrid/.etrid/keys/
   ```

4. **Create Systemd Service**
   ```bash
   # Use template from VALIDATOR_SERVICE_TEMPLATE.md
   # Update --public-addr with NEW IP
   sudo nano /etc/systemd/system/flarechain-validator.service

   sudo systemctl daemon-reload
   sudo systemctl enable flarechain-validator
   ```

---

### Phase 3: Start Validators (30 minutes)

**Important:** Start in stages, not all at once

**Bootstrap Validators First (if not already running):**
1. Oracle Cloud validators (already deployed)
2. Azure Sub 2 validators (if accessible)

**Then start Contabo validators:**
1. Start 5 validators, wait for sync
2. Monitor peer connections
3. Start next 5 validators
4. Start final 6 validators

**Commands on each VM:**
```bash
sudo systemctl start flarechain-validator
sudo journalctl -u flarechain-validator -f
```

**Monitor for:**
- Peer connections: Should reach 8-15 peers
- Block sync: "Syncing" → "Idle" when caught up
- Finality: Blocks being finalized

---

### Phase 4: Update Network Configuration (15 minutes)

**Update Bootnode List** (if needed):
The existing bootnodes should still work:
- 20.69.26.209 (EojEdred - Azure Sub 2)
- 20.186.91.207 (Governance - Azure Sub 2)
- 52.252.142.146 (Security - Azure Sub 2)
- 64.181.215.19 (Gizzi - Oracle)
- 129.80.122.34 (Audit - Oracle)

If Azure Sub 2 is also down, designate new bootnodes from Contabo VMs.

---

## 🚀 Quick Start: Minimal Migration

If you need validators online ASAP:

```bash
# On each new Contabo VM (after provisioning)

# 1. Install node
wget YOUR_BINARY_URL -O /usr/local/bin/flarechain-node
chmod +x /usr/local/bin/flarechain-node

# 2. Get chainspec
wget YOUR_CHAINSPEC_URL -O /home/etrid/chainspec.json

# 3. Create service
cat > /etc/systemd/system/flarechain-validator.service <<EOF
[Unit]
Description=Ëtrid FlareChain Validator
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/flarechain-node \\
  --chain /home/etrid/chainspec.json \\
  --base-path /home/etrid/.etrid \\
  --validator \\
  --name "Validator-NAME" \\
  --public-addr /ip4/THIS_VM_IP/tcp/30333 \\
  --bootnodes /ip4/64.181.215.19/tcp/30333/p2p/PEER_ID \\
  --rpc-cors all \\
  --port 30333 \\
  --rpc-port 9944
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# 4. Start
systemctl daemon-reload
systemctl enable flarechain-validator
systemctl start flarechain-validator

# 5. Monitor
journalctl -u flarechain-validator -f
```

---

## 🔍 Verification Checklist

After migration, verify:

- [ ] All 16 Contabo VMs are running
- [ ] Firewall rules allow port 30333
- [ ] Validators are syncing/synced
- [ ] Peer count is healthy (8-15 peers)
- [ ] RPC endpoints responding
- [ ] Block production active
- [ ] Finality working (GRANDPA)
- [ ] No errors in logs

**Health Check:**
```bash
bash /Users/macbook/Desktop/etrid/docs/mainnet/check-validators-simple.sh
```

---

## 💰 Cost Comparison

### Azure (Current - LOCKED)
- **16 VMs:** ~$300-500/month
- **Status:** OFFLINE due to payment

### Contabo (Recommended)
- **16 VMs (VPS M):** €168/month (~$180/month)
- **Savings:** $150-320/month
- **Better:** Fixed pricing, no surprise bills

### Alternative Providers

**Hetzner:**
- CX31: €7.59/month × 16 = €121/month (~$130/month)
- Great performance, German/US datacenters

**DigitalOcean:**
- Basic Droplet: $24/month × 16 = $384/month
- More expensive but good support

**Vultr:**
- Regular Performance: $12/month × 16 = $192/month
- Good global coverage

---

## ⚠️ Important Notes

### IP Address Changes are NORMAL in Blockchain
- Validators identify by cryptographic keys, not IPs
- Peer discovery is dynamic (Kademlia DHT)
- Other validators will find new IPs automatically
- Just need to update `--public-addr` flag

### Session Keys are Portable
- Keys stored in: `/Users/macbook/Desktop/etrid/secrets/validator-keys/`
- Copy to new VMs: same validator identity
- Network recognizes validator by public key

### No Chainspec Changes Needed
- Genesis hash stays the same
- Validator list unchanged
- Just IP addresses update (transparent to protocol)

---

## 🎯 Recommended Action Plan

### Immediate (Today):
1. Sign up for Contabo account
2. Order 16 VPS M instances
3. Note all assigned IP addresses
4. Begin software deployment

### Within 24 Hours:
1. Deploy node binaries to all VMs
2. Copy chainspec and session keys
3. Configure systemd services
4. Start validators in phases

### Within 48 Hours:
1. Verify all validators synced
2. Confirm network consensus
3. Monitor for 24 hours
4. Document new infrastructure

---

## 📞 Support Resources

**Deployment Files:**
- Node binary: `/Users/macbook/Desktop/etrid/target/release/flarechain-node`
- Chainspec: `/Users/macbook/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json`
- Session keys: `/Users/macbook/Desktop/etrid/secrets/validator-keys/`
- Service template: `/Users/macbook/Desktop/etrid/docs/mainnet/VALIDATOR_SERVICE_TEMPLATE.md`

**Documentation:**
- Validator config: `VALIDATOR_FINAL_CONFIG.md`
- System service: `SYSTEMD_SERVICE_GUIDE.md`

---

## ✅ Conclusion

**YES, migration to Contabo is completely feasible.**

**Advantages:**
- ✅ Lower cost ($180 vs $300-500/month)
- ✅ No IP address migration needed
- ✅ Session keys are portable
- ✅ 2-4 hour migration timeline
- ✅ Better pricing stability

**Next Steps:**
1. Create Contabo account
2. Provision 16 VPS instances
3. Deploy using guides above
4. Start validators in phases

The blockchain will continue working with new IPs - peer discovery handles everything automatically!

---

**Status:** 🚀 READY TO MIGRATE
**Estimated Time:** 4-6 hours total
**Estimated Cost:** €168/month (~$180/month)

