# Quick Start: Adding a New Contabo Validator to FlareChain Mainnet

## TL;DR - One Command Deployment

```bash
cd ~/Desktop/etrid
./scripts/deploy-new-contabo-validator.sh <number> <ip> "<name>"
```

**Example:**
```bash
./scripts/deploy-new-contabo-validator.sh 26 157.173.200.100 "stlouis-vn05"
```

---

## What The Script Does

The automation script handles everything:

1. ✅ **Opens port 30333** (fixes the Contabo firewall issue)
2. ✅ **Installs iptables-persistent** (survives reboots)
3. ✅ **Creates directory structure**
4. ✅ **Deploys binary and chainspec** from existing validator
5. ✅ **Generates unique session keys** (AURA, GRANDPA, ASF)
6. ✅ **Generates network key**
7. ✅ **Creates systemd service** with correct bootnode and --public-addr
8. ✅ **Starts the validator**

---

## After Running The Script

### 1. Save Session Keys

The script will output the session keys. **Copy them immediately** and add to:

`~/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md`

### 2. Monitor The Validator

```bash
# Watch logs in real-time
ssh -i ~/.ssh/contabo-validators root@<VM_IP> 'journalctl -u flarechain-validator -f'

# Check peer count (after 30 seconds)
ssh -i ~/.ssh/contabo-validators root@<VM_IP> 'journalctl -u flarechain-validator -n 5 | grep peers'
```

**Expected Output:**
```
- 0-5 mins:    0-3 peers  (discovering network)
- 5-30 mins:   5-10 peers (connecting via bootnodes)
- 30+ mins:    10-20 peers (full P2P mesh)
```

### 3. Verify Syncing

```bash
ssh -i ~/.ssh/contabo-validators root@<VM_IP> 'journalctl -u flarechain-validator -n 10 | grep Syncing'
```

Should show: `Syncing X bps, target=#XXXXXX`

---

## Manual Deployment (If Script Fails)

See the full checklist: `docs/mainnet/CONTABO_VM_SETUP_CHECKLIST.md`

---

## Common Issues

### Issue: 0 Peers After 5 Minutes

**Diagnosis:**
```bash
ssh -i ~/.ssh/contabo-validators root@<VM_IP> 'sudo iptables -L INPUT -n | grep 30333'
```

**If no output:** Firewall is blocking port 30333

**Fix:**
```bash
ssh -i ~/.ssh/contabo-validators root@<VM_IP> 'sudo iptables -I INPUT 1 -p tcp --dport 30333 -j ACCEPT && DEBIAN_FRONTEND=noninteractive apt-get install -y iptables-persistent && netfilter-persistent save && systemctl restart flarechain-validator'
```

### Issue: NetworkKeyNotFound

**Fix:**
```bash
ssh -i ~/.ssh/contabo-validators root@<VM_IP> 'NETWORK_KEY=$(openssl rand -hex 32) && mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network && echo -n "$NETWORK_KEY" > /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519 && chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519 && systemctl restart flarechain-validator'
```

---

## Critical Reminder

**⚠️ CONTABO VMS DEFAULT TO IPTABLES POLICY DROP**

This means:
- **Port 30333 MUST be explicitly opened** on every new VM
- **Use iptables-persistent** to survive reboots
- **Always run the deployment script** or manually open the port

Without this, the validator will have 0 peers and never sync.

---

## Files Reference

- **Deployment Script:** `~/Desktop/etrid/scripts/deploy-new-contabo-validator.sh`
- **Setup Checklist:** `~/Desktop/etrid/docs/mainnet/CONTABO_VM_SETUP_CHECKLIST.md`
- **Master Secrets File:** `~/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md`
- **Infrastructure Docs:** `~/Desktop/etrid/docs/mainnet/CURRENT_VALIDATOR_INFRASTRUCTURE.md`

---

**Last Updated:** November 9, 2025
**Context:** Firewall issue discovered on all 20 Contabo validators - port 30333 blocked by default
