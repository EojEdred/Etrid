# Network Keys Security Guide - Ëtrid

## What Are Network Keys?

Network keys in Substrate-based blockchains serve **two different purposes** - it's critical to understand the distinction:

### 1. **Network Identity Keys (libp2p keys)** 🔑
- **Purpose**: Identify nodes in the P2P network
- **Security Level**: LOW - NOT used for consensus or funds
- **Attack Surface**: Minimal - only affects network routing
- **File**: `chains/{chain_name}/network/secret_ed25519`

### 2. **Validator Session Keys** 🔐
- **Purpose**: Sign blocks and participate in consensus
- **Security Level**: CRITICAL - controls network security
- **Attack Surface**: HIGH - compromise = consensus attack
- **Types**:
  - BABE/AURA keys (block production)
  - GRANDPA keys (finality)
  - ImOnline keys (heartbeat)
  - Authority Discovery keys

## The Issue We're Facing

Our current problem is **#1 (Network Identity Keys)**, NOT the critical validator keys.

### Current Error:
```
Error: NetworkKeyNotFound("/path/to/network/secret_ed25519")
```

This is just the **libp2p peer identity** - it's like a MAC address for the P2P network. It's NOT used for:
- ❌ Signing blocks
- ❌ Consensus voting
- ❌ Controlling funds
- ❌ Network security

## Security Analysis

### Network Identity Keys - LOW RISK

**What happens if compromised:**
- Attacker can impersonate the node's P2P identity
- Other nodes might connect to the wrong peer
- **BUT**: Cannot sign blocks, steal funds, or break consensus

**What happens if publicly known:**
- Anyone can predict the node's Peer ID
- **BUT**: This is actually REQUIRED for bootnodes!
- Alice's node-key `0000...001` is intentionally public for development

**Attack Surface:**
```
LOW - An attacker with the network key can:
  ✓ Impersonate the node on P2P layer
  ✓ Receive messages intended for that peer
  ✗ Sign blocks (needs validator session keys)
  ✗ Participate in consensus (needs validator session keys)
  ✗ Access funds (needs account private keys)
```

### Validator Session Keys - CRITICAL RISK

**What happens if compromised:**
- Attacker can sign blocks as that validator
- Can participate in consensus voting
- Can cause finality stalls or forks
- **THIS IS THE REAL SECURITY CONCERN**

**Attack Surface:**
```
CRITICAL - An attacker with session keys can:
  ✓ Sign blocks
  ✓ Vote in consensus
  ✓ Potentially fork the chain
  ✓ Cause validator slashing
  ✗ Access validator's funds directly (different key)
```

---

## Our Options for Network Keys

### Option 1: Auto-Generated Keys (RECOMMENDED for Dev/Test)

**How it works:**
```bash
# Node generates keys on first startup
./flarechain-node --bob --base-path /data/bob
# Creates: /data/bob/chains/flarechain_local/network/secret_ed25519
```

**Pros:**
- ✅ Secure - randomly generated
- ✅ No management overhead
- ✅ Different for each node
- ✅ No pre-shared secrets

**Cons:**
- ⚠️ Peer ID changes if data is deleted
- ⚠️ Need to update bootnodes if Alice's ID changes

**Security:** ⭐⭐⭐⭐⭐ BEST for production
**Convenience:** ⭐⭐⭐ Good (one-time generation)

### Option 2: Predefined Keys (Current - Alice only)

**How it works:**
```bash
./flarechain-node --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001
```

**Pros:**
- ✅ Predictable Peer ID (useful for bootnodes)
- ✅ Can recreate network topology exactly

**Cons:**
- ⚠️ Keys are publicly known (in code/docs)
- ⚠️ Anyone can impersonate the node on P2P layer

**Security:** ⭐⭐ OK for development ONLY
**Convenience:** ⭐⭐⭐⭐⭐ Best for testing

**Attack Surface:**
```
Development: ACCEPTABLE
  - Known Peer IDs help with testing
  - P2P impersonation doesn't break consensus

Production: DISCOURAGED but not critical
  - Use for bootnodes only (they're public anyway)
  - Never use for validator nodes
```

### Option 3: Pre-Generated Random Keys

**How it works:**
```bash
# Generate keys offline
subkey generate-node-key > /data/bob/node-key

# Use in node
./flarechain-node --bob --node-key-file /data/bob/node-key
```

**Pros:**
- ✅ Secure random keys
- ✅ Known Peer IDs (can plan topology)
- ✅ Can backup/restore network identity

**Cons:**
- ⚠️ Need to manage key files
- ⚠️ If leaked, P2P identity is compromised (not critical)

**Security:** ⭐⭐⭐⭐ Excellent for production
**Convenience:** ⭐⭐⭐ Moderate (key management)

### Option 4: Key Derivation from Validator Identity

**How it works:**
```bash
# Derive network key from validator's well-known identity
# Alice, Bob, Charlie have known Peer IDs
```

**Pros:**
- ✅ Predictable for well-known validators
- ✅ Easy to configure

**Cons:**
- ⚠️ Ties network identity to validator identity
- ⚠️ Publicly known

**Security:** ⭐⭐ OK for dev/test
**Convenience:** ⭐⭐⭐⭐ Very good

---

## Recommended Approach

### For Development/Testing (Our Current Use Case)

**Use Option 1 (Auto-Generated) with predefined keys for bootnode:**

```bash
# Alice (bootnode) - predefined key for stable Peer ID
./flarechain-node --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001

# Bob - auto-generated key
./flarechain-node --bob \
  --base-path /data/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooW...

# Charlie - auto-generated key
./flarechain-node --charlie \
  --base-path /data/charlie \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooW...
```

**Why this is secure:**
- Alice's public key is OK (bootnodes are public anyway)
- Bob/Charlie get random secure keys
- P2P impersonation doesn't break consensus
- Easy to test and debug

### For Production Mainnet

**Use Option 3 (Pre-Generated Random Keys):**

```bash
# Generate unique key for each validator
subkey generate-node-key > /secure/validator1-node-key
chmod 600 /secure/validator1-node-key

# Use in production
./flarechain-node \
  --validator \
  --name "Validator-1" \
  --node-key-file /secure/validator1-node-key \
  --base-path /data/validator1
```

**Why this is secure:**
- Random keys for each validator
- Network identity is separate from consensus keys
- Can backup and restore if needed
- No public exposure

---

## What About Validator Session Keys?

**CRITICAL:** The validator session keys are MUCH more important than network keys!

### How to manage them securely:

1. **Generate session keys:**
```bash
# Inside the node
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys"}' \
     http://localhost:9944

# Returns public keys - store these
# Private keys stored in node's keystore
```

2. **Set session keys on-chain:**
```bash
# Via governance or validator extrinsic
pallet_session::set_keys(keys, proof)
```

3. **Security measures:**
- ✅ Keys stored in encrypted keystore
- ✅ Never expose private keys
- ✅ Use hardware security modules (HSM) for mainnet
- ✅ Rotate keys regularly
- ✅ Backup keystore securely
- ✅ Monitor for unauthorized key changes

---

## Attack Scenarios

### Scenario 1: Network Key Compromised

**Attacker has:** `secret_ed25519` (P2P key)

**Can do:**
- Impersonate node on P2P network
- Intercept P2P messages to that node
- Cause confusion in peer discovery

**Cannot do:**
- Sign blocks
- Participate in consensus
- Steal funds
- Break chain security

**Mitigation:**
- Rotate network key (generate new one)
- Update bootnode lists
- **Impact: LOW - Network disruption only**

### Scenario 2: Session Keys Compromised

**Attacker has:** Validator session keys (BABE/GRANDPA)

**Can do:**
- Sign blocks as that validator
- Vote in finality
- Potentially cause double-signing (slashing)
- Disrupt consensus if significant stake

**Cannot do:**
- Access validator's funds (different key)
- Change validator configuration
- Steal other validators' keys

**Mitigation:**
- Rotate session keys IMMEDIATELY
- Alert network of compromise
- May trigger slashing if double-signed
- **Impact: HIGH - Consensus attack vector**

### Scenario 3: Account Private Key Compromised

**Attacker has:** Validator's account private key

**Can do:**
- Transfer all funds
- Unbond stake
- Change session keys
- Change controller account

**Cannot do:**
- Retroactively sign old blocks
- Break existing consensus

**Mitigation:**
- Transfer remaining funds immediately
- Revoke validator status
- **Impact: CRITICAL - Complete loss of funds**

---

## Key Hierarchy Summary

```
Validator Node Security Layers:

1. Network Identity Key (libp2p)
   └─ Purpose: P2P routing
   └─ Security: LOW
   └─ If leaked: Network confusion only

2. Session Keys (consensus)
   ├─ BABE/ASF key (block production)
   ├─ GRANDPA key (finality)
   ├─ ImOnline key (heartbeat)
   └─ Authority Discovery key
   └─ Purpose: Consensus participation
   └─ Security: CRITICAL
   └─ If leaked: Consensus attack possible

3. Account Keys (funds)
   ├─ Stash account (holds stake)
   └─ Controller account (manages validator)
   └─ Purpose: Fund management
   └─ Security: CRITICAL
   └─ If leaked: Complete loss of funds
```

---

## Recommendation for Ëtrid

### Development/Testing (Now):
```bash
# Simple solution - let nodes auto-generate network keys
# Only Alice needs predefined key (she's the bootnode)

./flarechain-node --alice \
  --node-key 0000...001  # OK - bootnode can be public

./flarechain-node --bob \
  # Auto-generates network key on first run

./flarechain-node --charlie \
  # Auto-generates network key on first run
```

**Security Impact:** ✅ None - network keys are low risk

### Production Mainnet:
```bash
# Pre-generate random network keys for each validator
# Keep session keys in encrypted keystore
# Use HSM for critical validators
# Implement key rotation policy
```

---

## Conclusion

**Answer to your question:**

1. **"What does it entail?"**
   - Either let Substrate auto-generate keys (easiest)
   - Or pre-generate random keys with `subkey generate-node-key`
   - Just for P2P identity, not consensus

2. **"Is it an attack surface?"**
   - Network keys: **Minor** attack surface (P2P confusion only)
   - Session keys: **Critical** attack surface (consensus)
   - Account keys: **Critical** attack surface (funds)

3. **"Will presetting a config be exploitable?"**
   - For network keys: Low risk (bootnodes are public anyway)
   - For session keys: Never preset! Always generate securely
   - For account keys: Never preset! Always generate securely

**Best Practice:**
- ✅ Public/predefined network keys for bootnodes = OK
- ❌ Public/predefined session keys = CRITICAL VULNERABILITY
- ❌ Public/predefined account keys = TOTAL COMPROMISE

---

**For our current multi-node testing, using predefined network keys or auto-generated keys is perfectly fine. The security-critical keys (session keys and account keys) are completely separate and must always be generated securely.**
