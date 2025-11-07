# Ëtrid FlareChain - Final Validator Configuration

**Date:** November 3, 2025
**Status:** ✅ **PRODUCTION READY**

---

## 🎯 Critical Lessons Learned

### The `--public-addr` Discovery

**Problem Identified:** Validators were NOT advertising their public addresses, preventing peer discovery.

**What Was Happening:**
- ✅ All validators listening on `0.0.0.0:30333`
- ✅ All validators connected to bootnode (1 peer)
- ❌ Validators NOT advertising their public addresses
- ❌ Bootnode couldn't tell other validators how to reach them
- ❌ Result: Validators only connected TO bootnode, not to each other

**Solution:** Add `--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333` flag

**Result After Fix:**
- VM1: 1 → 11 peers ✅
- VM2: 1 → 13 peers ✅
- VM3: 1 → 9 peers ✅
- Network: Full mesh peer discovery working!

---

## 💡 Key Insights for Substrate Networks

### 1. `--public-addr` is CRITICAL for Peer Discovery
**Without it:** Validators can't advertise how to reach them
- Validator binds to `0.0.0.0:30333` (listens on all interfaces)
- BUT doesn't tell other peers which external address to use
- Other validators can't discover and connect
- Result: Star topology (all→bootnode) instead of mesh

**With it:** Full peer discovery and mesh networking
- Validator advertises: `/ip4/YOUR_PUBLIC_IP/tcp/30333/p2p/PEER_ID`
- Bootnode shares this address with other validators
- Other validators can connect directly
- Result: Fully connected mesh network

### 2. Multiple Bootnodes = Better Resilience
**Why it matters:**
- Single bootnode = single point of failure
- Multiple bootnodes = redundant entry points
- If one bootnode is down, validators can still join network
- Recommended: 3+ bootnodes for production

**Our configuration:**
- VM1 (EojEdred) - Primary bootstrap
- VM2 (Governance) - Secondary bootstrap
- VM3 (Security) - Tertiary bootstrap
- Any validator can connect to any of these 3

### 3. Private IPs Can Work
**The trick:** Advertise public address correctly
- Validator can run on private IP (cloud internal network)
- Use `--public-addr` with public IP for advertisement
- NAT/firewall forwards port 30333 to validator
- Other validators connect to public IP, reach private validator

**Example:**
```bash
# Validator running on private IP 172.16.0.5
# But accessible via public IP 20.69.26.209
--public-addr /ip4/20.69.26.209/tcp/30333  # ✅ Advertises public
# NOT: --public-addr /ip4/172.16.0.5/tcp/30333  # ❌ Won't work
```

### 4. Substrate Peer Discovery Works Great
**Once configured properly:**
- Validators find each other automatically via Kademlia DHT
- No need for static peer lists
- Self-healing network topology
- Optimal routing discovered automatically
- Gossip protocol ensures all validators stay synchronized

**Timeline observed:**
```
0s   → Connect to bootnode (1 peer)
30s  → Discover 2-3 validators
1min → Mesh forming (3-5 peers)
2min → Healthy network (5-10 peers)
5min → Full connectivity (8-15 peers)
```

---

## ⚠️ Common Mistakes to Avoid

### ❌ Mistake 1: Omitting `--public-addr`
**Symptom:** Validator stuck at 1 peer (bootnode only)
**Fix:** Add `--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333`

### ❌ Mistake 2: Comma-separated bootnodes
**Wrong:**
```bash
--bootnodes /ip4/.../p2p/ABC,/ip4/.../p2p/XYZ  # ❌ Parse error
```
**Correct:**
```bash
--bootnodes /ip4/.../p2p/ABC \
--bootnodes /ip4/.../p2p/XYZ  # ✅ Works
```

### ❌ Mistake 3: Using private IP in `--public-addr`
**Wrong:**
```bash
--public-addr /ip4/172.16.0.5/tcp/30333  # ❌ Private IP
```
**Correct:**
```bash
--public-addr /ip4/20.69.26.209/tcp/30333  # ✅ Public IP
```

### ❌ Mistake 4: Firewall blocking port 30333
**Symptom:** No incoming connections, peer count stays low
**Fix:**
```bash
sudo ufw allow 30333/tcp
# Or cloud firewall: Allow TCP 30333 from 0.0.0.0/0
```

---

## 📦 Complete Working Configuration

### Required Flags for ALL Validators:

```bash
--chain /path/to/chainspec.json \
--base-path /path/to/data \
--validator \
--name "YourValidatorName" \
--port 30333 \
--rpc-port 9944 \
--rpc-cors all \
--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm \
--bootnodes /ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb
```

### ⚠️ **CRITICAL FLAGS:**

1. **`--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333`** ← **REQUIRED** for peer discovery
2. **`--bootnodes`** (repeat flag for each bootnode) ← **NOT** comma-separated

---

## 🔧 Systemd Service Template

```ini
[Unit]
Description=Ëtrid FlareChain Validator Node
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=YOUR_USER
Group=YOUR_USER
WorkingDirectory=/home/YOUR_USER

# Node configuration
ExecStart=/path/to/flarechain-node \
  --chain /path/to/chainspec.json \
  --base-path /path/to/.etrid/validator \
  --validator \
  --name "YourValidatorName" \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --unsafe-rpc-external \
  --public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm \
  --bootnodes /ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb \
  --bootnodes /ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6

# Restart policy
Restart=on-failure
RestartSec=10s
KillSignal=SIGTERM
TimeoutStopSec=60s

# Resource limits
LimitNOFILE=65536
LimitNPROC=4096

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=/home/YOUR_USER/.etrid

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=flarechain-validator

[Install]
WantedBy=multi-user.target
```

---

## 🌐 Bootstrap Nodes (Proven Working)

### All 3 Azure Bootnodes:

```
VM1 (EojEdred - Primary Hub):
  IP: 20.69.26.209
  Peer ID: 12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm
  Address: /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWDQhL88sDaMCbtWfFFnCAyTYgyRcQumNbhfi2tmiP7ckm
  Status: ✅ 11+ peers connected

VM2 (Governance):
  IP: 20.186.91.207
  Peer ID: 12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb
  Address: /ip4/20.186.91.207/tcp/30333/p2p/12D3KooWAsAUeDfBhoQpQ1oXrr1ADkFssUdVanj7ssSyrpCiNEyb
  Status: ✅ 13+ peers connected

VM3 (Security):
  IP: 52.252.142.146
  Peer ID: 12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6
  Address: /ip4/52.252.142.146/tcp/30333/p2p/12D3KooWBeXtrDrJTFA23b3GnZtQ6DWtCb4B4Kra8uqi2JFvhJf6
  Status: ✅ 9+ peers connected
```

---

## 🚀 Quick Start for New Validators

### Step 1: Install Binary and Chainspec

```bash
# Place binary in PATH
sudo cp flarechain-node /usr/local/bin/
sudo chmod +x /usr/local/bin/flarechain-node

# Place chainspec
mkdir -p ~/flarechain
cp chainspec-mainnet-raw-FIXED.json ~/flarechain/chainspec.json
```

### Step 2: Create Data Directory

```bash
mkdir -p ~/.etrid/validator
```

### Step 3: Generate Network Key

```bash
flarechain-node key generate-node-key \
  --base-path ~/.etrid/validator \
  --chain ~/flarechain/chainspec.json
```

### Step 4: Create Systemd Service

```bash
# Replace YOUR_PUBLIC_IP with your actual public IP
sudo nano /etc/systemd/system/flarechain-validator.service

# Paste the template from above with your values
```

### Step 5: Start Validator

```bash
sudo systemctl daemon-reload
sudo systemctl enable flarechain-validator
sudo systemctl start flarechain-validator

# Watch logs
sudo journalctl -u flarechain-validator -f
```

---

## ✅ Success Indicators

After starting, you should see:

```
✅ 🏷  Local node identity is: 12D3Koo...
✅ 🔍 Discovered new external address: /ip4/YOUR_IP/tcp/30333/p2p/12D3Koo...
✅ 💤 Idle (1 peers), best: #124...
✅ 💤 Idle (2 peers), best: #124...  ← Peer count increasing
✅ 💤 Idle (5 peers), best: #124...  ← Keep increasing
```

**Expected peer count:** 2-15 peers within 1-2 minutes

---

## 🐛 Troubleshooting

### Validator stuck at 0 peers

**Check firewall:**
```bash
sudo ufw allow 30333/tcp
sudo systemctl restart flarechain-validator
```

### Validator stuck at 1 peer

**Missing --public-addr flag!**
```bash
# Edit service file
sudo nano /etc/systemd/system/flarechain-validator.service

# Add this line:
  --public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \

# Restart
sudo systemctl daemon-reload
sudo systemctl restart flarechain-validator
```

### Wrong bootnode format error

**Error:** `multiaddr parsing error: Invalid base string`

**Cause:** Using comma-separated bootnodes instead of separate flags

**Fix:** Use separate `--bootnodes` flag for each address:
```bash
# ❌ WRONG:
--bootnodes /ip4/.../p2p/...,/ip4/.../p2p/...

# ✅ CORRECT:
--bootnodes /ip4/.../p2p/... \
--bootnodes /ip4/.../p2p/...
```

---

## 📊 Network Health Check

```bash
# Check peer count
sudo journalctl -u flarechain-validator | grep "Idle" | tail -5

# Check for address advertisement
sudo journalctl -u flarechain-validator | grep "Discovered new external"

# Check for peer connections
sudo journalctl -u flarechain-validator | grep "peers" | tail -10
```

---

## 🎯 Minimum Requirements

- ✅ Firewall port 30333 open (TCP)
- ✅ Public IP address
- ✅ `--public-addr` flag with public IP
- ✅ At least 1 bootnode configured
- ✅ Correct chainspec file (MD5: 14f67296cc7ad134abb0704cc67c5a86)

---

## 📈 Expected Network Growth

```
Time    | Peers | Status
--------|-------|------------------
0s      | 0     | Starting
10s     | 1     | Connected to bootnode
30s     | 2-3   | Discovering validators
1min    | 3-5   | Building mesh
2min    | 5-10  | Healthy network
5min    | 8-15  | Full connectivity
```

---

## 🔐 Security Notes

- ✅ Only expose port 30333 publicly
- ✅ Keep RPC ports (9933, 9944) restricted
- ✅ Use `--rpc-cors all` only if needed
- ✅ Consider removing `--unsafe-rpc-external` for production
- ✅ Enable `--prometheus-external` for monitoring

---

## 📝 Configuration Checklist

- [ ] Binary installed at `/usr/local/bin/flarechain-node`
- [ ] Chainspec at `~/flarechain/chainspec.json`
- [ ] Data directory created at `~/.etrid/validator`
- [ ] Network key generated
- [ ] Public IP address identified
- [ ] Firewall port 30333 opened
- [ ] Systemd service created with:
  - [ ] `--public-addr /ip4/YOUR_IP/tcp/30333`
  - [ ] `--bootnodes` (at least 1)
  - [ ] `--validator` flag
  - [ ] Correct chainspec path
- [ ] Service enabled and started
- [ ] Peer count increasing in logs

---

## 🎉 Success Criteria

**Your validator is working correctly when:**

1. ✅ Service status: `active (running)`
2. ✅ Peer count: 2+ peers within 2 minutes
3. ✅ Block sync: Importing blocks
4. ✅ Finality: Block numbers increasing
5. ✅ External address discovered in logs

---

---

## 📚 Troubleshooting Journey (For Future Reference)

This section documents the complete troubleshooting process that led to the working configuration above. **Save future deployers hours of debugging!**

### Phase 1: Initial Deployment (Incomplete Config)
**Configuration:**
```bash
--validator
--chain chainspec.json
--base-path ~/.etrid/validator
--port 30333
--rpc-port 9944
```

**Result:**
- ✅ Validators started successfully
- ❌ 0 peers - Complete isolation
- **Problem:** No bootnode specified

---

### Phase 2: Added Bootnode (Single Point)
**Configuration:**
```bash
# ... previous flags ...
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooW...
```

**Result:**
- ✅ VM1 (bootnode): 20+ peers (acting as hub)
- ⚠️ VM2, VM3: 1 peer each (only connected to VM1)
- ❌ Validators NOT discovering each other
- **Problem:** Validators not advertising public addresses

---

### Phase 3: Added Multiple Bootnodes (Still Limited)
**Configuration:**
```bash
# ... previous flags ...
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooW... \
--bootnodes /ip4/20.186.91.207/tcp/30333/p2p/12D3KooW... \
--bootnodes /ip4/52.252.142.146/tcp/30333/p2p/12D3KooW...
```

**Result:**
- ✅ VM1: 21 peers
- ⚠️ VM2: 2 peers (VM1 + VM3)
- ⚠️ VM3: 2 peers (VM1 + VM2)
- ❌ Still not forming full mesh
- **Problem:** Validators listening but not advertising

---

### Phase 4: Added `--public-addr` (SOLUTION!) ✅
**Configuration:**
```bash
# ... previous flags ...
--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooW... \
--bootnodes /ip4/20.186.91.207/tcp/30333/p2p/12D3KooW... \
--bootnodes /ip4/52.252.142.146/tcp/30333/p2p/12D3KooW...
```

**Result:**
- 🎉 VM1: 11+ peers (healthy mesh)
- 🎉 VM2: 13+ peers (healthy mesh)
- 🎉 VM3: 9+ peers (healthy mesh)
- ✅ Full peer discovery working
- ✅ Self-healing mesh network
- **Solution confirmed:** `--public-addr` was the missing piece!

---

## 🔬 Technical Deep Dive: Why `--public-addr` Matters

### How Substrate P2P Discovery Works:

1. **Validator starts:**
   - Binds to `0.0.0.0:30333` (listens on all interfaces)
   - Generates or loads peer ID from network key
   - Connects to configured bootnodes

2. **Bootnode connection established:**
   - Validator sends: "I'm peer 12D3KooW..."
   - Bootnode responds: "Here are other peers I know about"

3. **The critical question: "How do others reach me?"**

   **WITHOUT `--public-addr`:**
   ```
   Validator → Bootnode: "I'm 12D3KooW..., listening on 0.0.0.0:30333"
   Bootnode → Other validators: "Connect to 12D3KooW... at ???"
   Other validators: "No address to connect to!" ❌
   ```

   **WITH `--public-addr`:**
   ```
   Validator → Bootnode: "I'm 12D3KooW... at /ip4/20.69.26.209/tcp/30333"
   Bootnode → Other validators: "Connect to 12D3KooW... at 20.69.26.209:30333"
   Other validators: "Got it! Connecting..." ✅
   ```

4. **Peer discovery cascade:**
   - Each validator tells others about peers it knows
   - Kademlia DHT maintains routing table
   - Network self-organizes into efficient topology
   - No central coordination needed

### Network Topology Evolution:

```
Phase 2 (Bootnode only):           Phase 4 (With --public-addr):
       VM1 (Bootnode)                      VM1 ⟷ VM2
      /    \                                ↕  ⤬  ↕
    VM2    VM3                             VM3 ⟷ Other
    (isolated from each other)            (full mesh)
```

---

## 🎓 What We Learned About Substrate Networking

### 1. **Bind vs Advertise** - Two Different Concepts
- **Bind address** (`0.0.0.0:30333`): Where the validator LISTENS
- **Advertise address** (`--public-addr`): What it TELLS others
- Must explicitly tell Substrate what address others should use

### 2. **NAT Traversal** - Cloud Networking Reality
- Cloud VMs often have private IPs (172.x.x.x, 10.x.x.x)
- Public traffic NATted through public IP
- Validator must advertise the PUBLIC IP, not private
- Firewall/NAT must forward 30333 to validator

### 3. **Bootnode Role** - More Than Connection
- Not just "first peer to connect to"
- Acts as gossip hub for peer discovery
- Shares peer addresses with new joiners
- Network can function even if bootnode goes down (after initial discovery)

### 4. **Substrate vs Traditional P2P**
- **Bitcoin/Ethereum:** Often use hardcoded peer lists
- **Substrate:** Dynamic DHT-based discovery (like BitTorrent)
- More resilient, no single point of failure
- But requires proper address advertisement!

---

## 📖 References & Further Reading

### Substrate Documentation:
- [Configure a Bootnode](https://docs.substrate.io/tutorials/build-a-blockchain/add-trusted-nodes/#configure-a-bootnode)
- [Network Configuration](https://docs.substrate.io/reference/command-line-tools/node-template/#network-configuration)
- [libp2p in Substrate](https://docs.substrate.io/learn/networks-and-nodes/)

### Related Issues:
- Why validators may only connect to bootnode (this issue!)
- Private IP vs Public IP in cloud deployments
- Firewall configuration for P2P networks

### Commands for Debugging:
```bash
# Check what addresses validator is listening on
sudo netstat -tlnp | grep 30333

# Check what validator is advertising
sudo journalctl -u flarechain-validator | grep "Discovered new external"

# Check peer connections
sudo journalctl -u flarechain-validator | grep "Idle" | tail -20

# Test connectivity from another machine
telnet VALIDATOR_IP 30333
```

---

**Last Updated:** November 3, 2025
**Tested on:** Ubuntu 22.04 LTS (ARM64)
**Network:** Ëtrid FlareChain Mainnet
**Genesis Hash:** 0xca40bbf4f8367f63ea110afd54cf5fd38c44df100f9454b62135bfc09df74da8

---

## 🙏 Acknowledgments

Special thanks to the troubleshooting process that identified the `--public-addr` requirement - a critical insight that transformed isolated validators into a healthy mesh network. This documentation will save countless hours for future Substrate network operators.

**Key Contributor:** Root cause analysis identifying peer advertisement as the blocker

**Lesson:** Sometimes the most critical configuration is the one you don't see in basic tutorials. Always verify what your node is advertising to the network!
