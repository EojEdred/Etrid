# ËTRID Mainnet VM Deployment Reference

**Last Updated:** November 12, 2025
**Total Validators:** 21
**Chain:** FlareChain Mainnet

---

## VM Architecture Groups

### Group 1: Oracle Cloud VMs (ARM Architecture)
**Count:** 2 validators
**Architecture:** ARM (different from x86_64)
**Build Strategy:** Must build locally on each VM

| VM Name | Validator ID | IP Address | SSH Priority | Architecture |
|---------|--------------|------------|--------------|--------------|
| d1      | V1-Gizzi     | TBD        | High         | ARM          |
| d5      | V3-Audit     | TBD        | High         | ARM          |

**Build Commands:**
```bash
# d1
ssh d1
cd ~/Desktop/etrid
cargo build --release --bin flarechain-node
sudo systemctl restart flarechain-validator

# d5
ssh d5
cd ~/Desktop/etrid
cargo build --release --bin flarechain-node
sudo systemctl restart flarechain-validator
```

---

### Group 2: Contabo/Azure VMs (x86_64 Architecture)
**Count:** 19 validators
**Architecture:** x86_64 (shared)
**Build Strategy:** Build once, rsync to all

#### Azure Subscription 1 - West Europe (5 VMs)
| VM Name | Validator ID | IP Address | SSH Priority | Notes |
|---------|--------------|------------|--------------|-------|
| azure-we-1 | TBD | TBD | Medium | |
| azure-we-2 | TBD | TBD | Medium | |
| azure-we-3 | TBD | TBD | Medium | |
| azure-we-4 | TBD | TBD | Medium | |
| azure-we-5 | TBD | TBD | Medium | |

#### Azure Subscription 1 - North Europe (2 VMs)
| VM Name | Validator ID | IP Address | SSH Priority | Notes |
|---------|--------------|------------|--------------|-------|
| azure-ne-1 | TBD | TBD | Medium | |
| azure-ne-2 | TBD | TBD | Medium | |

#### Azure Subscription 1 - UK South (5 VMs)
| VM Name | Validator ID | IP Address | SSH Priority | Notes |
|---------|--------------|------------|--------------|-------|
| azure-uk-1 | TBD | TBD | Medium | |
| azure-uk-2 | TBD | TBD | Medium | |
| azure-uk-3 | TBD | TBD | Medium | |
| azure-uk-4 | TBD | TBD | Medium | |
| azure-uk-5 | TBD | TBD | Medium | |

#### Azure Subscription 1 - France Central (4 VMs)
| VM Name | Validator ID | IP Address | SSH Priority | Notes |
|---------|--------------|------------|--------------|-------|
| azure-fr-1 | TBD | TBD | Medium | |
| azure-fr-2 | TBD | TBD | Medium | |
| azure-fr-3 | TBD | TBD | Medium | |
| azure-fr-4 | TBD | TBD | Medium | |

#### Azure Subscription 2 (3 VMs)
| VM Name | Validator ID | IP Address | SSH Priority | Notes |
|---------|--------------|------------|--------------|-------|
| V0B-EojEdred | TBD | TBD | High | Primary dev |
| V1-Governance | TBD | TBD | High | Governance |
| V2-Security | TBD | TBD | High | Security |

---

## SSH Access Priority

### Priority Levels

**High Priority (5 VMs):**
- d1 (Oracle)
- d5 (Oracle)
- V0B-EojEdred (Azure Sub 2)
- V1-Governance (Azure Sub 2)
- V2-Security (Azure Sub 2)

**Reason:** Critical infrastructure, different architecture (Oracle), or special roles

**Medium Priority (16 VMs):**
- All other Azure Subscription 1 VMs

**Reason:** Standard validators, same architecture, can rsync from build server

---

## Build and Deployment Strategy

### Architecture-Aware Build Process

#### For Oracle VMs (d1, d5) - ARM Architecture
```bash
#!/bin/bash
# build-oracle.sh - Run on each Oracle VM

VM_NAME=$(hostname)
echo "Building flarechain-node on $VM_NAME (ARM architecture)"

cd ~/Desktop/etrid

# Clean previous build
cargo clean

# Build release binary
cargo build --release --bin flarechain-node

# Verify binary
ls -lh target/release/flarechain-node

# Check architecture
file target/release/flarechain-node

# Install
sudo cp target/release/flarechain-node /usr/local/bin/

# Restart service
sudo systemctl restart flarechain-validator

# Check status
sudo systemctl status flarechain-validator
```

#### For Contabo/Azure VMs (x86_64 Architecture)
```bash
#!/bin/bash
# build-and-distribute-x86_64.sh - Run on ONE build server

BUILD_SERVER="V0B-EojEdred"  # Or any x86_64 VM
TARGET_VMS=(
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V1-Governance" "V2-Security"
)

echo "=== Building on $BUILD_SERVER ==="
cd ~/Desktop/etrid

# Clean and build
cargo clean
cargo build --release --bin flarechain-node

# Compress for transfer
gzip -c target/release/flarechain-node > flarechain-node-x86_64.gz

echo "=== Distributing to ${#TARGET_VMS[@]} VMs ==="
for vm in "${TARGET_VMS[@]}"; do
    echo "Deploying to $vm..."

    # Copy compressed binary
    scp flarechain-node-x86_64.gz $vm:/tmp/

    # Decompress and install on remote
    ssh $vm << 'ENDSSH'
        cd /tmp
        gunzip -f flarechain-node-x86_64.gz
        sudo cp flarechain-node-x86_64 /usr/local/bin/flarechain-node
        sudo chmod +x /usr/local/bin/flarechain-node
        sudo systemctl restart flarechain-validator
        echo "✅ $vm deployed"
ENDSSH
done

echo "=== Deployment complete ==="
```

### Verification Script
```bash
#!/bin/bash
# verify-all-nodes.sh - Check all nodes are running correct version

ALL_VMS=(
    "d1" "d5"
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V0B-EojEdred" "V1-Governance" "V2-Security"
)

echo "=== Verifying 21 Validators ==="

for vm in "${ALL_VMS[@]}"; do
    echo -n "$vm: "

    ssh $vm 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
        -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"state_getRuntimeVersion\"}" | \
        jq -r ".result.specVersion"' 2>/dev/null || echo "OFFLINE"
done
```

---

## Peer Mesh Configuration

### Full Mesh Topology

All 21 validators must peer with each other.

### Generate reserved_peers.txt

```bash
#!/bin/bash
# generate-reserved-peers.sh - Run this to create peer list

ALL_VMS=(
    "d1" "d5"
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V0B-EojEdred" "V1-Governance" "V2-Security"
)

echo "=== Collecting Peer IDs ==="

> all_peers.txt

for vm in "${ALL_VMS[@]}"; do
    echo "Getting peer ID from $vm..."

    PEER_ID=$(ssh $vm 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
        -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_localPeerId\"}" | \
        jq -r ".result"')

    # Get public IP
    IP=$(ssh $vm 'curl -s ifconfig.me')

    # Format: /ip4/IP/tcp/30333/p2p/PEER_ID
    echo "/ip4/$IP/tcp/30333/p2p/$PEER_ID" >> all_peers.txt

    echo "  $vm: $IP - $PEER_ID"
done

echo ""
echo "=== Generated all_peers.txt ==="
cat all_peers.txt
```

### Distribute reserved_peers.txt

```bash
#!/bin/bash
# distribute-peer-config.sh - Copy reserved peers to all VMs

ALL_VMS=(
    "d1" "d5"
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V0B-EojEdred" "V1-Governance" "V2-Security"
)

for vm in "${ALL_VMS[@]}"; do
    echo "Configuring $vm..."

    # Copy peer list (excluding self)
    grep -v "$vm" all_peers.txt | ssh $vm 'cat > ~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt'

    # Update systemd service to use reserved peers
    ssh $vm << 'ENDSSH'
        sudo systemctl stop flarechain-validator

        # Update service file
        sudo sed -i 's|ExecStart=.*|ExecStart=/usr/local/bin/flarechain-node \
            --validator \
            --name $(hostname) \
            --chain /etc/flarechain/mainnet.json \
            --base-path ~/.local/share/flarechain-node \
            --reserved-peers ~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt \
            --reserved-only \
            --rpc-port 9944 \
            --port 30333|' /etc/systemd/system/flarechain-validator.service

        sudo systemctl daemon-reload
        sudo systemctl start flarechain-validator
ENDSSH

    echo "✅ $vm configured"
done
```

### Verify Full Mesh

```bash
#!/bin/bash
# verify-peer-mesh.sh - Check all nodes have 20 peers

ALL_VMS=(
    "d1" "d5"
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V0B-EojEdred" "V1-Governance" "V2-Security"
)

echo "=== Verifying Peer Mesh ==="

for vm in "${ALL_VMS[@]}"; do
    PEER_COUNT=$(ssh $vm 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
        -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}" | \
        jq -r ".result.peers"' 2>/dev/null)

    if [ "$PEER_COUNT" = "20" ]; then
        echo "✅ $vm: $PEER_COUNT peers"
    else
        echo "❌ $vm: $PEER_COUNT peers (expected 20)"
    fi
done
```

---

## Keystore Management

### Generate Keys for All Validators

```bash
#!/bin/bash
# generate-all-keys.sh - Generate session keys on all VMs

ALL_VMS=(
    "d1" "d5"
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V0B-EojEdred" "V1-Governance" "V2-Security"
)

> session_keys.txt

for vm in "${ALL_VMS[@]}"; do
    echo "Generating keys on $vm..."

    SESSION_KEY=$(ssh $vm 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
        -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_rotateKeys\"}" | \
        jq -r ".result"')

    echo "$vm: $SESSION_KEY" >> session_keys.txt
    echo "  $vm: $SESSION_KEY"
done

echo ""
echo "=== Generated session_keys.txt ==="
cat session_keys.txt
```

### Verify Keystore

```bash
#!/bin/bash
# verify-keystores.sh - Check all validators have GRANDPA and AURA keys

ALL_VMS=(
    "d1" "d5"
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V0B-EojEdred" "V1-Governance" "V2-Security"
)

for vm in "${ALL_VMS[@]}"; do
    echo -n "$vm: "

    GRANDPA=$(ssh $vm 'ls ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/ | grep "^6772616e"' 2>/dev/null | wc -l)
    AURA=$(ssh $vm 'ls ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/ | grep "^61757261"' 2>/dev/null | wc -l)

    if [ "$GRANDPA" -gt 0 ] && [ "$AURA" -gt 0 ]; then
        echo "✅ GRANDPA ($GRANDPA) + AURA ($AURA)"
    else
        echo "❌ Missing keys (GRANDPA: $GRANDPA, AURA: $AURA)"
    fi
done
```

---

## Deployment Checklist

### Phase 1: Build Preparation
- [ ] Pull latest code with committee fix (commit eb9e0de1 or later)
- [ ] Verify spec_version = 105 in runtime
- [ ] Run local tests: `cargo test --workspace`
- [ ] Generate mainnet chainspec: `flarechain-node build-spec --chain mainnet > mainnet.json`

### Phase 2: Build Binaries
- [ ] Build on d1 (Oracle ARM)
- [ ] Build on d5 (Oracle ARM)
- [ ] Build on one x86_64 VM (e.g., V0B-EojEdred)
- [ ] Compress x86_64 binary: `gzip flarechain-node`
- [ ] Verify binary sizes and architecture with `file` command

### Phase 3: Distribute Binaries
- [ ] Copy to /usr/local/bin/ on both Oracle VMs
- [ ] Rsync to all 19 x86_64 VMs
- [ ] Set executable permissions on all VMs
- [ ] Verify binary version on all VMs

### Phase 4: Configure Peer Mesh
- [ ] Generate all peer IDs (run generate-reserved-peers.sh)
- [ ] Create reserved_peers.txt for each VM (excluding self)
- [ ] Update systemd services with --reserved-peers flag
- [ ] Add --reserved-only flag to prevent external connections

### Phase 5: Keystore Setup
- [ ] Generate session keys on all 21 VMs
- [ ] Verify GRANDPA + AURA keys exist in keystores
- [ ] Record all session keys for genesis config
- [ ] Backup all keystores

### Phase 6: Genesis Configuration
- [ ] Update mainnet genesis with all 21 validators
- [ ] Set correct peerType for each validator (0, 1, or 2)
- [ ] Include all session keys
- [ ] Set CommitteeSize = 21
- [ ] Distribute mainnet.json to all VMs

### Phase 7: Coordinated Start
- [ ] Stop all validators: `sudo systemctl stop flarechain-validator`
- [ ] Clear databases (if restarting chain)
- [ ] Start all validators simultaneously (use tmux/parallel ssh)
- [ ] Monitor logs for peer connections

### Phase 8: Verification
- [ ] Check peer count on all nodes (should be 20)
- [ ] Verify block production starting
- [ ] Check ASF committee size in logs (should be 21)
- [ ] Verify GRANDPA finality messages
- [ ] Monitor for errors in logs

---

## Troubleshooting

### Node Not Syncing
```bash
# Check peer count
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' | jq

# Check if reserved peers are configured
cat ~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt

# Check firewall
sudo ufw status
sudo ufw allow 30333/tcp
```

### Binary Architecture Mismatch
```bash
# Check binary architecture
file /usr/local/bin/flarechain-node

# Should show:
# - ARM: "Mach-O 64-bit executable arm64"
# - x86_64: "ELF 64-bit LSB executable, x86-64"

# If wrong architecture, rebuild on correct VM type
```

### Peer Connection Issues
```bash
# Check if node is reachable
nc -zv <VM_IP> 30333

# Check reserved peers config
systemctl cat flarechain-validator | grep reserved-peers

# Verify peer ID
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_localPeerId"}' | jq
```

### Keystore Missing
```bash
# Check keystore location
ls -la ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/

# Regenerate keys
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
    http://localhost:9944
```

---

## Quick Reference Commands

### System Status
```bash
# Service status
sudo systemctl status flarechain-validator

# View logs
journalctl -u flarechain-validator -f

# Latest block
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' | jq -r ".result.number"
```

### Node Info
```bash
# Node version
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_version"}' | jq

# Peer count
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' | jq -r ".result.peers"

# Sync status
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "system_syncState"}' | jq
```

### Chain Status
```bash
# Current block
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' | jq

# Finalized block
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getFinalizedHead"}' | jq
```

---

## Contact Information

**Primary Administrator:** Eoj Edred (eojedredbitepubkey1@proton.me)

**High Priority VMs (Direct Access):**
- d1, d5 (Oracle)
- V0B-EojEdred, V1-Governance, V2-Security (Azure Sub 2)

**Backup Access:**
- All Azure Subscription 1 VMs

---

*Last updated: November 12, 2025*
*Document version: 1.0*
