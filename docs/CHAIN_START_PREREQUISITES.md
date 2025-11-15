# ËTRID FlareChain Mainnet Start Prerequisites

**Last Updated:** November 12, 2025
**Target:** FlareChain Mainnet Launch
**Validators:** 21 total

---

## Table of Contents

1. [Overview](#overview)
2. [Critical Prerequisites](#critical-prerequisites)
3. [Session Keys](#session-keys)
4. [Genesis Configuration](#genesis-configuration)
5. [Keystore Setup](#keystore-setup)
6. [Network Configuration](#network-configuration)
7. [Runtime Requirements](#runtime-requirements)
8. [Verification Procedures](#verification-procedures)
9. [Launch Sequence](#launch-sequence)

---

## Overview

Starting a Substrate-based blockchain with 21 validators requires precise coordination of:
- **Session keys** (AURA + GRANDPA) for all validators
- **Genesis configuration** with correct validator committee
- **Peer mesh** for full connectivity
- **Runtime** with proper committee size and consensus parameters

**Critical:** All prerequisites must be met BEFORE chain start. Missing any component will cause finality failure.

---

## Critical Prerequisites

### 1. Binary Deployment ✓

All 21 VMs must have the correct binary:
- **Oracle VMs (d1, d5):** ARM architecture binary
- **Contabo/Azure VMs (19):** x86_64 architecture binary
- **Version:** spec_version = 105 or later
- **Commit:** eb9e0de1 or later (includes DecentralizedDirector committee fix)

**Verification:**
```bash
# Check binary exists
ls -lh /usr/local/bin/flarechain-node

# Check architecture
file /usr/local/bin/flarechain-node

# Check version
/usr/local/bin/flarechain-node --version
```

### 2. Session Keys ✓

Each validator needs 2 key types in keystore:
- **AURA keys (Sr25519):** For block production
- **GRANDPA keys (Ed25519):** For finality voting

**Status:** MUST be generated and verified before genesis

### 3. Genesis Configuration ✓

Genesis must include:
- All 21 validators in `validatorCommittee.validators`
- All 21 session keys in `session.keys`
- Correct `peerType` for each validator (0, 1, or 2)
- `CommitteeSize = 21` in runtime config

### 4. Network Topology ✓

Full mesh peering required:
- Each validator connects to all other 20 validators
- `--reserved-peers` with 20 peer addresses
- `--reserved-only` to prevent external connections

### 5. Firewall Rules ✓

Ports must be open:
- **30333/tcp:** P2P networking
- **9944/tcp:** RPC (localhost only for security)
- **9615/tcp:** Prometheus metrics (optional, localhost only)

---

## Session Keys

### Understanding Session Keys

Substrate uses **session keys** to separate validator identity from block production/finality keys. This allows key rotation without changing validator accounts.

**Key Types:**
1. **AURA (Sr25519):** Authority Round block production
2. **GRANDPA (Ed25519):** GRANDPA finality gadget

**Combined Session Key Format:**
```
0x + AURA_PUBKEY (32 bytes) + GRANDPA_PUBKEY (32 bytes) = 64 bytes hex
```

### Generating Session Keys

#### Method 1: author_rotateKeys RPC (Recommended)

```bash
# Generate new keys (returns combined session key)
curl -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
    http://localhost:9944

# Response:
{
  "jsonrpc": "2.0",
  "result": "0xabcd1234...ef567890abcd1234...ef567890", # 64 bytes
  "id": 1
}
```

**What happens:**
1. Node generates new Sr25519 (AURA) and Ed25519 (GRANDPA) keypairs
2. Saves them to keystore directory
3. Returns combined public keys as hex

**Keystore location:**
```
~/.local/share/flarechain-node/chains/flare_mainnet/keystore/
├── 6772616e... (GRANDPA key - "gran" in hex)
└── 61757261... (AURA key - "aura" in hex)
```

#### Method 2: Manual Key Insertion

```bash
# Insert GRANDPA key
curl -H "Content-Type: application/json" \
    -d '{
        "id":1,
        "jsonrpc":"2.0",
        "method": "author_insertKey",
        "params": [
            "gran",
            "your secret phrase here",
            "0xYOUR_GRANDPA_PUBLIC_KEY"
        ]
    }' \
    http://localhost:9944

# Insert AURA key
curl -H "Content-Type: application/json" \
    -d '{
        "id":1,
        "jsonrpc":"2.0",
        "method": "author_insertKey",
        "params": [
            "aura",
            "your secret phrase here",
            "0xYOUR_AURA_PUBLIC_KEY"
        ]
    }' \
    http://localhost:9944
```

### Extracting Individual Keys from Session Key

```bash
# Session key format: AURA (32 bytes) + GRANDPA (32 bytes)
SESSION_KEY="0xabcd1234...full64bytes...ef567890"

# Extract AURA key (first 32 bytes = 64 hex chars)
AURA_KEY="0x${SESSION_KEY:2:64}"

# Extract GRANDPA key (last 32 bytes = 64 hex chars)
GRANDPA_KEY="0x${SESSION_KEY:66:64}"

echo "AURA:    $AURA_KEY"
echo "GRANDPA: $GRANDPA_KEY"
```

### Generate Keys for All 21 Validators

```bash
#!/bin/bash
# generate-all-session-keys.sh

VALIDATORS=(
    "d1" "d5"
    "azure-we-1" "azure-we-2" "azure-we-3" "azure-we-4" "azure-we-5"
    "azure-ne-1" "azure-ne-2"
    "azure-uk-1" "azure-uk-2" "azure-uk-3" "azure-uk-4" "azure-uk-5"
    "azure-fr-1" "azure-fr-2" "azure-fr-3" "azure-fr-4"
    "V0B-EojEdred" "V1-Governance" "V2-Security"
)

echo "=== Generating Session Keys for All Validators ==="
echo ""

> session_keys_all.json

for i in "${!VALIDATORS[@]}"; do
    VM="${VALIDATORS[$i]}"
    echo "[$((i+1))/21] Generating keys on $VM..."

    # Start node if not running
    ssh $VM "sudo systemctl start flarechain-validator" 2>/dev/null
    sleep 2

    # Generate session key
    SESSION_KEY=$(ssh $VM 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
        -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"author_rotateKeys\"}" | \
        jq -r ".result"')

    if [ -n "$SESSION_KEY" ] && [ "$SESSION_KEY" != "null" ]; then
        # Get validator account (assuming it's in a known location or env var)
        VALIDATOR_ACCOUNT=$(ssh $VM 'cat ~/.flarechain/validator_account 2>/dev/null || echo "TBD"')

        # Extract AURA and GRANDPA keys
        AURA_KEY="0x${SESSION_KEY:2:64}"
        GRANDPA_KEY="0x${SESSION_KEY:66:64}"

        # Record in JSON format
        cat >> session_keys_all.json << EOF
{
  "validator": "$VM",
  "account": "$VALIDATOR_ACCOUNT",
  "sessionKey": "$SESSION_KEY",
  "auraKey": "$AURA_KEY",
  "grandpaKey": "$GRANDPA_KEY"
},
EOF

        echo "  ✅ Session key: ${SESSION_KEY:0:20}...${SESSION_KEY: -20}"
    else
        echo "  ❌ Failed to generate key"
    fi
done

echo ""
echo "=== Session keys saved to session_keys_all.json ==="
```

### Verify Keystore Contents

```bash
#!/bin/bash
# verify-all-keystores.sh

for VM in d1 d5 azure-*; do
    echo -n "$VM: "

    # Check for GRANDPA key (hex prefix: 6772616e = "gran")
    GRANDPA_COUNT=$(ssh $VM 'ls ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/ 2>/dev/null | grep "^6772616e" | wc -l')

    # Check for AURA key (hex prefix: 61757261 = "aura")
    AURA_COUNT=$(ssh $VM 'ls ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/ 2>/dev/null | grep "^61757261" | wc -l')

    if [ "$GRANDPA_COUNT" -ge 1 ] && [ "$AURA_COUNT" -ge 1 ]; then
        echo "✅ GRANDPA ($GRANDPA_COUNT) + AURA ($AURA_COUNT)"
    else
        echo "❌ Missing keys (GRANDPA: $GRANDPA_COUNT, AURA: $AURA_COUNT)"
    fi
done
```

---

## Genesis Configuration

### Genesis Structure

Genesis configuration defines the initial state of the blockchain, including:
- Validator committee
- Session keys
- Initial balances
- Runtime parameters

**File:** `05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json`

### ValidatorCommittee Section

```json
{
  "validatorCommittee": {
    "validators": [
      {
        "accountId": "0x1234...validator1account",
        "stake": 64000000000000000000000,  // 64 ETR in smallest unit
        "peerType": 0,  // 0=ValidityNode, 1=FlareNode, 2=DecentralizedDirector
        "auraKey": "0xabcd...aura_public_key",
        "grandpaKey": "0xef01...grandpa_public_key"
      },
      // ... 20 more validators
    ]
  }
}
```

**Critical:**
- **ALL 21 validators** must be in this array
- **peerType** must match validator's role
- **stake** must be >= 64 ETR (64000000000000000000000 smallest units)
- **auraKey** and **grandpaKey** must match keystore keys

### Session Keys Section

```json
{
  "session": {
    "keys": [
      [
        "0x1234...validator1account",  // Validator account
        "0x1234...validator1account",  // Session account (usually same)
        {
          "grandpa": "0xef01...grandpa_public_key"
        }
      ],
      // ... 20 more
    ]
  }
}
```

**Note:** Session keys only include GRANDPA keys, not AURA. AURA keys are in ValidatorCommittee.

### Runtime Parameters

```json
{
  "runtime": {
    "committeeSize": 21,
    "epochLength": 2400,  // blocks
    "sessionLength": 600   // blocks
  }
}
```

### Generating Genesis from Session Keys

```bash
#!/bin/bash
# generate-genesis-config.sh
# Reads session_keys_all.json and generates genesis config

cat > genesis_validators.json << 'EOF'
{
  "validatorCommittee": {
    "validators": [
EOF

# Read session keys JSON and generate validator entries
jq -c '.[]' session_keys_all.json | while read validator; do
    ACCOUNT=$(echo $validator | jq -r '.account')
    AURA=$(echo $validator | jq -r '.auraKey')
    GRANDPA=$(echo $validator | jq -r '.grandpaKey')
    PEER_TYPE=0  # Set appropriately: 0=ValidityNode, 1=FlareNode, 2=DecentralizedDirector

    cat >> genesis_validators.json << EOF
      {
        "accountId": "$ACCOUNT",
        "stake": 64000000000000000000000,
        "peerType": $PEER_TYPE,
        "auraKey": "$AURA",
        "grandpaKey": "$GRANDPA"
      },
EOF
done

cat >> genesis_validators.json << 'EOF'
    ]
  }
}
EOF

echo "Generated genesis_validators.json"
```

### Building Chain Spec

```bash
# Generate chain spec from preset
./target/release/flarechain-node build-spec \
    --chain mainnet \
    > mainnet-spec.json

# Convert to raw format (optimized for node startup)
./target/release/flarechain-node build-spec \
    --chain mainnet-spec.json \
    --raw \
    > mainnet-raw.json

# Distribute to all validators
for VM in d1 d5 azure-*; do
    scp mainnet-raw.json $VM:/etc/flarechain/mainnet.json
done
```

---

## Keystore Setup

### Directory Structure

```
~/.local/share/flarechain-node/
└── chains/
    └── flare_mainnet/
        ├── keystore/
        │   ├── 6772616e64706131... (GRANDPA key)
        │   └── 6175726120...       (AURA key)
        ├── db/
        │   └── full/
        └── network/
            └── secret_ed25519
```

### Keystore Permissions

```bash
# Set proper permissions
chmod 700 ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/
chmod 600 ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/*
```

### Backup Keystores

```bash
#!/bin/bash
# backup-all-keystores.sh

BACKUP_DIR="./keystore_backups_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

for VM in d1 d5 azure-*; do
    echo "Backing up keystore from $VM..."
    mkdir -p "$BACKUP_DIR/$VM"

    scp -r $VM:~/.local/share/flarechain-node/chains/flare_mainnet/keystore/ \
        "$BACKUP_DIR/$VM/"
done

# Encrypt backup
tar czf keystore_backups.tar.gz "$BACKUP_DIR"
gpg -c keystore_backups.tar.gz  # Prompts for password

echo "✅ Keystores backed up to keystore_backups.tar.gz.gpg"
```

---

## Network Configuration

### Full Mesh Topology

All 21 validators must connect to each other (full mesh = 210 connections total).

**Why full mesh?**
- GRANDPA finality requires 2/3+ validators to communicate
- Network partitions can stall finality
- Direct connections reduce latency

### Reserved Peers Configuration

```bash
# systemd service configuration
ExecStart=/usr/local/bin/flarechain-node \
    --validator \
    --chain /etc/flarechain/mainnet.json \
    --name $(hostname) \
    --base-path ~/.local/share/flarechain-node \
    --reserved-peers ~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt \
    --reserved-only \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-methods Unsafe \
    --rpc-cors all
```

**Flags:**
- `--reserved-peers`: Path to file with peer multiaddresses
- `--reserved-only`: Only connect to reserved peers (no external peers)

### Firewall Configuration

```bash
# Allow P2P port
sudo ufw allow 30333/tcp

# Block RPC from external (security)
sudo ufw deny 9944/tcp

# If using nginx proxy for RPC
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Apply rules
sudo ufw enable
```

### Port Forwarding (if behind NAT)

```bash
# If validator is behind NAT, configure port forwarding:
# Router: Forward external_ip:30333 → validator_ip:30333

# Set external address in node config
--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333
```

---

## Runtime Requirements

### Runtime Version

**Current:** spec_version = 105

All validators MUST run the same runtime version. Mismatch causes consensus failure.

**Check runtime version:**
```bash
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "state_getRuntimeVersion"}' | \
    jq '.result.specVersion'
```

### Committee Size Configuration

**Runtime config:** `05-multichain/flare-chain/runtime/src/lib.rs`

```rust
parameter_types! {
    pub const CommitteeSize: u32 = 21;  // Must match validator count
}

impl pallet_validator_committee::Config for Runtime {
    type CommitteeSize = CommitteeSize;
    // ...
}
```

### ASF Parameters

```rust
pub struct ASFParams {
    pub max_committee_size: u32,  // 21
    pub slot_duration: u64,        // 6000ms = 6 seconds
    pub min_validator_stake: Balance,  // 64 ETR
    // ...
}

impl Default for ASFParams {
    fn default() -> Self {
        Self {
            max_committee_size: 21,
            slot_duration: 6000,
            min_validator_stake: 64_000_000_000_000_000_000_000,
            // ...
        }
    }
}
```

---

## Verification Procedures

### Pre-Start Checklist

Run these checks on ALL 21 validators before chain start:

#### 1. Binary Verification
```bash
# Check binary exists
[ -f /usr/local/bin/flarechain-node ] && echo "✅ Binary exists" || echo "❌ Binary missing"

# Check executable permission
[ -x /usr/local/bin/flarechain-node ] && echo "✅ Executable" || echo "❌ Not executable"

# Check architecture
file /usr/local/bin/flarechain-node | grep -q "x86-64\|arm64" && echo "✅ Architecture OK" || echo "❌ Wrong architecture"
```

#### 2. Keystore Verification
```bash
KEYSTORE=~/.local/share/flarechain-node/chains/flare_mainnet/keystore

# Check GRANDPA key
ls $KEYSTORE | grep -q "^6772616e" && echo "✅ GRANDPA key" || echo "❌ GRANDPA key missing"

# Check AURA key
ls $KEYSTORE | grep -q "^61757261" && echo "✅ AURA key" || echo "❌ AURA key missing"
```

#### 3. Genesis Configuration Verification
```bash
# Check mainnet spec exists
[ -f /etc/flarechain/mainnet.json ] && echo "✅ Chain spec exists" || echo "❌ Chain spec missing"

# Verify it's raw format
jq -r '.genesis.raw' /etc/flarechain/mainnet.json >/dev/null && echo "✅ Raw format" || echo "❌ Not raw format"
```

#### 4. Network Configuration Verification
```bash
# Check reserved peers file
[ -f ~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt ] && \
    echo "✅ Reserved peers configured" || \
    echo "❌ Reserved peers missing"

# Count peer addresses (should be 20)
PEER_COUNT=$(wc -l < ~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt)
[ "$PEER_COUNT" -eq 20 ] && echo "✅ 20 peers configured" || echo "❌ Only $PEER_COUNT peers"
```

#### 5. Firewall Verification
```bash
# Check if P2P port is open
sudo ufw status | grep "30333.*ALLOW" && echo "✅ Port 30333 open" || echo "❌ Port 30333 blocked"
```

### Automated Verification Script

```bash
#!/bin/bash
# verify-validator-ready.sh - Run on each validator

echo "=== FlareChain Validator Readiness Check ==="
echo ""

SCORE=0
MAX_SCORE=10

# 1. Binary check
if [ -x /usr/local/bin/flarechain-node ]; then
    echo "✅ Binary exists and is executable"
    SCORE=$((SCORE + 2))
else
    echo "❌ Binary missing or not executable"
fi

# 2. Architecture check
ARCH=$(file /usr/local/bin/flarechain-node | grep -o "x86-64\|arm64")
if [ -n "$ARCH" ]; then
    echo "✅ Architecture: $ARCH"
    SCORE=$((SCORE + 1))
else
    echo "❌ Unknown or wrong architecture"
fi

# 3. Keystore check
KEYSTORE=~/.local/share/flarechain-node/chains/flare_mainnet/keystore
GRANDPA_COUNT=$(ls $KEYSTORE 2>/dev/null | grep "^6772616e" | wc -l)
AURA_COUNT=$(ls $KEYSTORE 2>/dev/null | grep "^61757261" | wc -l)

if [ "$GRANDPA_COUNT" -ge 1 ] && [ "$AURA_COUNT" -ge 1 ]; then
    echo "✅ Keystore: GRANDPA ($GRANDPA_COUNT) + AURA ($AURA_COUNT)"
    SCORE=$((SCORE + 2))
else
    echo "❌ Keystore incomplete (GRANDPA: $GRANDPA_COUNT, AURA: $AURA_COUNT)"
fi

# 4. Chain spec check
if [ -f /etc/flarechain/mainnet.json ]; then
    echo "✅ Chain spec exists"
    SCORE=$((SCORE + 1))
else
    echo "❌ Chain spec missing"
fi

# 5. Reserved peers check
PEERS_FILE=~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt
if [ -f "$PEERS_FILE" ]; then
    PEER_COUNT=$(wc -l < "$PEERS_FILE")
    if [ "$PEER_COUNT" -eq 20 ]; then
        echo "✅ Reserved peers: $PEER_COUNT (correct)"
        SCORE=$((SCORE + 2))
    else
        echo "⚠️  Reserved peers: $PEER_COUNT (expected 20)"
        SCORE=$((SCORE + 1))
    fi
else
    echo "❌ Reserved peers file missing"
fi

# 6. Firewall check
if sudo ufw status 2>/dev/null | grep -q "30333.*ALLOW"; then
    echo "✅ Port 30333 open"
    SCORE=$((SCORE + 1))
else
    echo "⚠️  Port 30333 status unknown"
fi

# 7. Service check
if systemctl is-enabled flarechain-validator &>/dev/null; then
    echo "✅ Service enabled"
    SCORE=$((SCORE + 1))
else
    echo "❌ Service not enabled"
fi

echo ""
echo "=== Readiness Score: $SCORE/$MAX_SCORE ==="

if [ "$SCORE" -eq "$MAX_SCORE" ]; then
    echo "🟢 READY TO START"
    exit 0
elif [ "$SCORE" -ge 7 ]; then
    echo "🟡 MOSTLY READY (check warnings)"
    exit 0
else
    echo "🔴 NOT READY (fix errors first)"
    exit 1
fi
```

---

## Launch Sequence

### Pre-Launch (T-24 hours)

1. **Finalize Genesis Configuration**
   ```bash
   # Build final chain spec
   ./target/release/flarechain-node build-spec --chain mainnet --raw > mainnet-raw.json
   ```

2. **Distribute Chain Spec**
   ```bash
   for VM in d1 d5 azure-*; do
       scp mainnet-raw.json $VM:/etc/flarechain/mainnet.json
   done
   ```

3. **Verify All Validators Ready**
   ```bash
   for VM in d1 d5 azure-*; do
       ssh $VM './verify-validator-ready.sh'
   done
   ```

4. **Backup Everything**
   ```bash
   # Backup keystores
   ./backup-all-keystores.sh

   # Backup chain spec
   cp mainnet-raw.json backups/mainnet-raw-$(date +%Y%m%d).json
   ```

### Launch Day (T-0)

**Coordinated start required:** All validators must start within ~30 seconds of each other.

#### Option 1: Parallel SSH (Recommended)

```bash
#!/bin/bash
# coordinated-start.sh

VALIDATORS=(d1 d5 azure-we-1 azure-we-2 ... V2-Security)

echo "=== Starting All 21 Validators in 10 seconds ==="
echo "Press Ctrl+C to abort"
sleep 10

# Start all in parallel
for VM in "${VALIDATORS[@]}"; do
    (
        echo "Starting $VM..."
        ssh $VM 'sudo systemctl start flarechain-validator'
    ) &
done

# Wait for all to start
wait

echo "=== All validators started ==="
```

#### Option 2: tmux with synchronized panes

```bash
# Create tmux session with 21 panes
tmux new-session -d -s flarechain-launch

# Split into 21 panes (automated script)
# ... tmux split-window commands ...

# Set synchronized input
tmux set-window-option -t flarechain-launch synchronize-panes on

# In synchronized panes, run:
# ssh d1 (in pane 1)
# ssh d5 (in pane 2)
# ... etc

# Then type once (sent to all):
# sudo systemctl start flarechain-validator
```

### Post-Launch Verification (T+5 min)

#### 1. Check All Nodes Started
```bash
for VM in d1 d5 azure-*; do
    ssh $VM 'systemctl is-active flarechain-validator'
done
```

#### 2. Verify Peer Connections
```bash
for VM in d1 d5 azure-*; do
    PEERS=$(ssh $VM 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
        -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}" | \
        jq -r ".result.peers"')

    echo "$VM: $PEERS peers"
done
```

**Expected:** Each validator should show 20 peers within 2-3 minutes.

#### 3. Check Block Production
```bash
# Wait for genesis (block #0)
sleep 30

# Check block height
for VM in d1 d5 azure-*; do
    BLOCK=$(ssh $VM 'curl -s http://localhost:9944 -H "Content-Type: application/json" \
        -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}" | \
        jq -r ".result.number"')

    echo "$VM: Block $BLOCK"
done
```

**Expected:** All validators should be on same block (within 1-2 blocks).

#### 4. Verify GRANDPA Finality
```bash
# Check finalized block
FINALIZED=$(curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getFinalizedHead"}' | \
    jq -r '.result')

echo "Finalized head: $FINALIZED"

# Check logs for GRANDPA messages
journalctl -u flarechain-validator -n 100 | grep -i "finalized\|grandpa"
```

**Expected:** Finality should begin within 2-3 minutes of chain start.

#### 5. Check ASF Committee
```bash
# Check logs for committee size
journalctl -u flarechain-validator -n 100 | grep -i "committee.*21\|loaded 21"
```

**Expected:** Logs should show "Loaded 21 committee members" or similar.

---

## Troubleshooting

### Chain Not Starting

**Symptoms:**
- Nodes start but no block production
- Stuck at block #0

**Diagnosis:**
```bash
# Check if genesis matches
journalctl -u flarechain-validator | grep "genesis\|chain"

# Verify all nodes using same genesis hash
curl -s http://localhost:9944 -H "Content-Type: application/json" \
    -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlockHash", "params": [0]}' | \
    jq -r '.result'
```

**Solution:** Ensure all validators use same mainnet-raw.json

### No Peer Connections

**Symptoms:**
- Node shows 0 peers
- "No peers" in logs

**Diagnosis:**
```bash
# Check reserved peers configured
cat ~/.local/share/flarechain-node/chains/flare_mainnet/reserved_peers.txt

# Check firewall
sudo ufw status | grep 30333

# Test connectivity to another validator
nc -zv <OTHER_VM_IP> 30333
```

**Solution:**
- Verify reserved_peers.txt exists and has 20 entries
- Open port 30333: `sudo ufw allow 30333/tcp`
- Check public IP if behind NAT

### Finality Not Starting

**Symptoms:**
- Blocks producing but not finalizing
- No "Finalized" messages in logs

**Diagnosis:**
```bash
# Check GRANDPA keys in keystore
ls ~/.local/share/flarechain-node/chains/flare_mainnet/keystore/ | grep 6772616e

# Check ASF committee size
journalctl -u flarechain-validator | grep -i "committee"

# Check for GRANDPA errors
journalctl -u flarechain-validator | grep -i "grandpa.*error\|grandpa.*fail"
```

**Solution:**
- Verify all 21 validators have GRANDPA keys
- Check ASF shows "21 committee members"
- Ensure 2/3+ validators (14+) are online

### Committee Size Mismatch

**Symptoms:**
- Logs show "only 16 validators" or wrong count
- ASF consensus not reaching quorum

**Diagnosis:**
```bash
# Check runtime CommitteeSize
grep -r "CommitteeSize" runtime/src/lib.rs

# Check which validators excluded
journalctl -u flarechain-validator | grep -i "excluded\|filtered\|peer.*type"
```

**Solution:**
- Verify runtime has DecentralizedDirector in `can_be_in_committee()`
- Rebuild with latest fix (commit eb9e0de1+)
- Check genesis has correct peerType for all validators

---

## Emergency Procedures

### Stop All Validators
```bash
for VM in d1 d5 azure-*; do
    ssh $VM 'sudo systemctl stop flarechain-validator'
done
```

### Purge Chain Data (Reset)
```bash
for VM in d1 d5 azure-*; do
    ssh $VM 'rm -rf ~/.local/share/flarechain-node/chains/flare_mainnet/db'
done
```

**⚠️  WARNING:** This deletes all chain data. Only use for fresh start.

### Restore Keystore from Backup
```bash
# Decrypt backup
gpg -d keystore_backups.tar.gz.gpg > keystore_backups.tar.gz
tar xzf keystore_backups.tar.gz

# Restore to specific VM
scp -r keystore_backups/<VM>/keystore/* \
    <VM>:~/.local/share/flarechain-node/chains/flare_mainnet/keystore/
```

---

## Post-Launch Monitoring

### First Hour
- Monitor block production (should be consistent 6-12s block time)
- Verify all validators producing blocks in rotation
- Check finality lag (should be <30s)
- Monitor peer connections (all should stay at 20)

### First Day
- Check for errors in logs
- Monitor resource usage (CPU, memory, disk)
- Verify no validators dropped offline
- Check transaction processing

### First Week
- Performance benchmarking
- Stability testing
- Adjust parameters if needed
- Plan first runtime upgrade

---

## Success Criteria

✅ **Chain Start Successful When:**
1. All 21 validators online
2. Each validator has 20 peer connections
3. Blocks producing consistently (6-12s)
4. GRANDPA finality active (<30s lag)
5. ASF committee size = 21
6. No errors in logs for 1 hour
7. All validators participating in consensus

---

*Document version: 1.0*
*Last updated: November 12, 2025*
*Next review: After successful mainnet launch*
