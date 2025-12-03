# Gizzi Validator Migration Plan
## Oracle Cloud → Contabo (Before Dec 25, 2025)

---

## 🎯 Objective
Migrate `gizzi-validator` (ts-val-01) from Oracle Cloud to Contabo without network disruption.

## 📋 Pre-Migration Checklist

### 1. Provision New Contabo VM
- **Specs**: Match or exceed current gizzi specs
  - CPU: 4+ cores
  - RAM: 8+ GB
  - Storage: 100+ GB SSD
  - OS: Ubuntu 22.04/24.04 LTS

### 2. Verify Current Gizzi Configuration
```bash
# Check current validator keys
ssh ts-val-01 'ls -la /var/lib/etrid/chains/primearc-mainnet/keystore/'

# Check chain data size
ssh ts-val-01 'du -sh /var/lib/etrid/chains/primearc-mainnet/'

# Verify service configuration
ssh ts-val-01 'systemctl cat etrid-validator || systemctl cat primearc-validator'

# Backup current peer ID
ssh ts-val-01 'curl -sf localhost:9944 -H "Content-Type: application/json" -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_localPeerId\",\"params\":[],\"id\":1}" | jq -r .result'
```

---

## 🔄 Migration Steps

### Phase 1: Backup (Day 1)

#### Step 1.1: Backup Session Keys
```bash
# On gizzi validator
ssh ts-val-01

# Create backup directory
sudo mkdir -p /tmp/gizzi-backup/keystore
sudo mkdir -p /tmp/gizzi-backup/config

# Backup keystore (CRITICAL - validator identity)
sudo cp -r /var/lib/etrid/chains/primearc-mainnet/keystore/* /tmp/gizzi-backup/keystore/

# Backup chain spec and config
sudo cp /var/lib/etrid/primearc-mainnet-v1.json /tmp/gizzi-backup/config/
sudo cp /etc/systemd/system/etrid-validator.service /tmp/gizzi-backup/config/ 2>/dev/null || \
sudo cp /etc/systemd/system/primearc-validator.service /tmp/gizzi-backup/config/

# Set permissions
sudo chmod -R 600 /tmp/gizzi-backup/keystore/*
sudo tar czf /tmp/gizzi-backup.tar.gz -C /tmp gizzi-backup/

# Download to local machine
exit
scp -i ~/.ssh/gizzi-validator ubuntu@100.96.84.69:/tmp/gizzi-backup.tar.gz ~/Desktop/
```

#### Step 1.2: Document Current State
```bash
# Save current configuration
ssh ts-val-01 'curl -sf localhost:9944 -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"author_rotateKeys\",\"params\":[],\"id\":1}"' \
  > ~/Desktop/gizzi-session-keys-backup.json

# Save peer list
ssh ts-val-01 'curl -sf localhost:9615/metrics | grep "substrate_sub_libp2p_peers_count"' \
  > ~/Desktop/gizzi-metrics-backup.txt
```

### Phase 2: Prepare New Contabo VM (Day 2-3)

#### Step 2.1: Initial Setup
```bash
# SSH to new Contabo VM (assume root access)
ssh root@<NEW_CONTABO_IP>

# Update system
apt update && apt upgrade -y

# Install dependencies
apt install -y curl jq tar gzip ufw

# Create validator user (matching gizzi's ubuntu user)
useradd -m -s /bin/bash ubuntu
usermod -aG sudo ubuntu
```

#### Step 2.2: Install Tailscale on New VM
```bash
# Install Tailscale
curl -fsSL https://tailscale.com/install.sh | sh

# Authenticate Tailscale (DO NOT START YET)
# We'll handle Tailscale IP reassignment later
```

#### Step 2.3: Transfer Validator Binary
```bash
# From gizzi, copy the binary
ssh ts-val-01 'which etrid-validator || which primearc-validator'
# Let's assume it's /usr/local/bin/etrid-validator

# Copy binary from gizzi to local
scp -i ~/.ssh/gizzi-validator ubuntu@100.96.84.69:/usr/local/bin/etrid-validator ~/Desktop/

# Upload to new Contabo VM
scp ~/Desktop/etrid-validator root@<NEW_CONTABO_IP>:/usr/local/bin/
ssh root@<NEW_CONTABO_IP> 'chmod +x /usr/local/bin/etrid-validator'
```

#### Step 2.4: Create Directory Structure
```bash
# On new Contabo VM
ssh root@<NEW_CONTABO_IP>

# Create data directories
mkdir -p /var/lib/etrid/chains/primearc-mainnet/keystore
chown -R ubuntu:ubuntu /var/lib/etrid/

# Create config directory
mkdir -p /etc/etrid/
```

### Phase 3: Tailscale IP Reassignment (Day 4)

**Option A: Transfer Device (Recommended)**
```bash
# 1. On OLD gizzi VM - Stop validator first
ssh ts-val-01 'sudo systemctl stop etrid-validator'

# 2. Check Tailscale device name
ssh ts-val-01 'tailscale status'
# Look for device name (e.g., "gizzi-validator" or "ts-val-01")

# 3. On NEW Contabo VM - Join with same device name
ssh root@<NEW_CONTABO_IP>
tailscale up --hostname=gizzi-validator --advertise-tags=tag:validator

# 4. In Tailscale Admin Console (https://login.tailscale.com/admin/machines)
# - Approve new device
# - Delete old gizzi device
# - New device should inherit 100.96.84.69 or get new IP

# 5. Verify new Tailscale IP
tailscale ip -4
```

**Option B: Manual IP Management**
```bash
# If Tailscale doesn't preserve 100.96.84.69:
# 1. Note new Tailscale IP (e.g., 100.x.x.x)
# 2. Update all other validators' bootnodes if hardcoded
# 3. Update DNS/documentation with new IP
```

### Phase 4: Restore Validator Keys (Day 4)

```bash
# On new Contabo VM
ssh root@<NEW_CONTABO_IP>

# Upload backup
scp ~/Desktop/gizzi-backup.tar.gz root@<NEW_CONTABO_IP>:/tmp/

# Extract keys
cd /tmp
tar xzf gizzi-backup.tar.gz

# Restore keystore (CRITICAL)
cp -r /tmp/gizzi-backup/keystore/* /var/lib/etrid/chains/primearc-mainnet/keystore/
chmod -R 600 /var/lib/etrid/chains/primearc-mainnet/keystore/*
chown -R ubuntu:ubuntu /var/lib/etrid/chains/primearc-mainnet/keystore/

# Restore chain spec
cp /tmp/gizzi-backup/config/primearc-mainnet-v1.json /var/lib/etrid/
```

### Phase 5: Configure Validator Service (Day 4)

```bash
# On new Contabo VM
ssh root@<NEW_CONTABO_IP>

# Create systemd service
cat > /etc/systemd/system/etrid-validator.service << 'EOF'
[Unit]
Description=Etrid Validator Node
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/var/lib/etrid
ExecStart=/usr/local/bin/etrid-validator \
  --base-path /var/lib/etrid \
  --chain /var/lib/etrid/primearc-mainnet-v1.json \
  --validator \
  --name gizzi-validator \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --prometheus-port 9615 \
  --prometheus-external \
  --port 30333
Restart=always
RestartSec=10
LimitNOFILE=65535

[Install]
WantedBy=multi-user.target
EOF

# Reload and enable
systemctl daemon-reload
systemctl enable etrid-validator
```

### Phase 6: Sync Chain Data (Day 5-6)

**Option A: Fast Sync (Copy from gizzi)**
```bash
# This requires significant downtime for gizzi
# On gizzi, create chain data archive
ssh ts-val-01 'sudo systemctl stop etrid-validator'
ssh ts-val-01 'cd /var/lib/etrid/chains/primearc-mainnet && tar czf /tmp/chaindata.tar.gz db'

# Download to local (WARNING: May be large)
scp -i ~/.ssh/gizzi-validator ubuntu@100.96.84.69:/tmp/chaindata.tar.gz ~/Desktop/

# Upload to new VM
scp ~/Desktop/chaindata.tar.gz root@<NEW_CONTABO_IP>:/tmp/

# Extract on new VM
ssh root@<NEW_CONTABO_IP>
cd /var/lib/etrid/chains/primearc-mainnet
tar xzf /tmp/chaindata.tar.gz
chown -R ubuntu:ubuntu db/
```

**Option B: Network Sync (Recommended - Zero Downtime)**
```bash
# Start new validator and let it sync from network
ssh root@<NEW_CONTABO_IP> 'systemctl start etrid-validator'

# Monitor sync progress
ssh root@<NEW_CONTABO_IP> 'journalctl -u etrid-validator -f | grep "best:"'
# Wait until it catches up to current block height
```

### Phase 7: Cutover (Day 7)

```bash
# 1. Verify new VM is synced
ssh root@<NEW_CONTABO_IP> \
  'curl -sf localhost:9944 -H "Content-Type: application/json" \
   -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_syncState\",\"params\":[],\"id\":1}" | jq'

# 2. Verify validator role
ssh root@<NEW_CONTABO_IP> \
  'curl -sf localhost:9944 -H "Content-Type: application/json" \
   -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_nodeRoles\",\"params\":[],\"id\":1}" | jq'
# Should show: ["Authority"]

# 3. Check peer count
ssh root@<NEW_CONTABO_IP> \
  'curl -sf localhost:9615/metrics | grep substrate_sub_libp2p_peers_count'
# Should show 15-21 peers

# 4. Verify session keys match
ssh root@<NEW_CONTABO_IP> \
  'curl -sf localhost:9944 -H "Content-Type: application/json" \
   -d "{\"jsonrpc\":\"2.0\",\"method\":\"author_hasSessionKeys\",\"params\":[\"<SESSION_KEY_FROM_BACKUP>\"],\"id\":1}" | jq'

# 5. Stop OLD gizzi validator
ssh ts-val-01 'sudo systemctl stop etrid-validator && sudo systemctl disable etrid-validator'
```

### Phase 8: Update Documentation & Access (Day 7)

```bash
# Update SSH config
vi ~/.ssh/config
# Change ts-val-01 host to new IP

# Update ETRID_MAINNET_VM_ACCESS.md
# Update any scripts referencing 100.96.84.69

# Update check-all-nodes.sh if Tailscale IP changed
```

---

## ⚠️ Critical Considerations

### 1. **Validator Keys are Sacred**
- NEVER lose `/var/lib/etrid/chains/primearc-mainnet/keystore/*`
- NEVER run two validators with same keys simultaneously (equivocation = slash)
- Keep encrypted backup of keys

### 2. **Downtime Minimization**
- Network can tolerate 6 Byzantine faults (21 - 15 = 6)
- One validator offline is acceptable for short period
- Recommended: Sync new VM first, then quick cutover

### 3. **Equivocation Risk**
**DANGER**: Running old AND new validator simultaneously = double-signing = SLASH
- Always stop old validator BEFORE starting new one with same keys
- Or sync new validator first, then do fast cutover

### 4. **Genesis Configuration**
- Current genesis has 21 validators with gizzi included
- Moving gizzi to new VM doesn't require genesis change (same keys)
- If Tailscale IP changes, other validators will autodiscover via gossip

---

## 🧪 Testing Checklist

After migration, verify:

```bash
# 1. Node is running
ssh root@<NEW_CONTABO_IP> 'systemctl status etrid-validator'

# 2. Is an Authority
ssh root@<NEW_CONTABO_IP> 'curl -sf localhost:9944 -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_nodeRoles\",\"params\":[],\"id\":1}" | jq'

# 3. Has peers
ssh root@<NEW_CONTABO_IP> 'curl -sf localhost:9615/metrics | grep substrate_sub_libp2p_peers_count'

# 4. Synced to latest block
ssh root@<NEW_CONTABO_IP> 'curl -sf localhost:9944 -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getBlock\",\"params\":[],\"id\":1}" | jq .result.block.header.number'

# 5. Appears in validator set
ssh root@<NEW_CONTABO_IP> 'curl -sf localhost:9944 -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"session_validators\",\"params\":[],\"id\":1}" | jq'

# 6. Run check-all from auditdev
ssh ts-val-02 'check-all'
# Should show gizzi/ts-val-01 as VALIDATOR
```

---

## 📅 Recommended Timeline

**Week 1 (Dec 1-7):**
- Order Contabo VM
- Backup gizzi keys and configuration
- Document current state

**Week 2 (Dec 8-14):**
- Setup new Contabo VM
- Install dependencies
- Configure Tailscale (don't start yet)

**Week 3 (Dec 15-21):**
- Transfer validator binary
- Start network sync on new VM (in background)
- Monitor sync progress

**Week 4 (Dec 22-24):**
- Final sync verification
- Perform cutover (stop old, verify new)
- Update documentation
- Monitor for 24-48 hours

**Dec 25+:**
- Oracle Cloud shutdown
- Old VM decommissioned

---

## 🆘 Rollback Plan

If migration fails:

```bash
# 1. Stop new validator immediately
ssh root@<NEW_CONTABO_IP> 'systemctl stop etrid-validator'

# 2. Restart old gizzi
ssh ts-val-01 'sudo systemctl start etrid-validator'

# 3. Verify old gizzi is validating
ssh ts-val-01 'curl -sf localhost:9944 -H "Content-Type: application/json" \
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_nodeRoles\",\"params\":[],\"id\":1}" | jq'

# 4. Investigate issue on new VM
# 5. Retry migration
```

---

## 📝 Post-Migration Updates

Files to update after successful migration:

1. `/Users/macbook/Desktop/ETRID_MAINNET_VM_ACCESS.md`
   - Update ts-val-01 IP address
   - Update SSH access details

2. `~/.ssh/config`
   - Update ts-val-01 host entry

3. `/Users/macbook/Desktop/etrid/scripts/check-all-nodes.sh`
   - Update IP in ALL_VMS array if Tailscale IP changed

4. `/Users/macbook/Desktop/etrid/PHONE_COMMANDS.md`
   - Update IP references if changed

5. Documentation
   - Update any deployment guides
   - Update monitoring dashboards

---

## 🔒 Security Notes

1. **SSH Keys**: Transfer `~/.ssh/gizzi-validator` key to new VM or generate new one
2. **Firewall**: Configure UFW on new Contabo VM
   ```bash
   ufw allow 22/tcp    # SSH
   ufw allow 30333/tcp # P2P
   ufw allow 9944/tcp  # RPC (restrict to Tailscale)
   ufw allow 9615/tcp  # Metrics (restrict to Tailscale)
   ufw enable
   ```
3. **Tailscale ACLs**: Verify validator tag permissions
4. **Keystore Backup**: Keep encrypted backup in secure location

---

## 💡 Pro Tips

1. **Sync Before Cutover**: Start syncing new VM weeks in advance (Option B in Phase 6)
2. **Monitor Both**: Keep both VMs running (old validating, new syncing) until new is ready
3. **Quick Cutover**: When ready, stop old → verify new → done (< 1 minute downtime)
4. **Peer Discovery**: Other validators will autodiscover new IP via Kademlia DHT
5. **Session Keys**: If you ever need to regenerate, use `author_rotateKeys` RPC

---

**Questions or Issues?** Check logs:
```bash
journalctl -u etrid-validator -n 100 -f
```
