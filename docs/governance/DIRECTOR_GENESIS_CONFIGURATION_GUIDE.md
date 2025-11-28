# Decentralized Directors - Genesis Configuration Guide

**Ëtrid Blockchain - 9 Directors Governance System**

---

## Overview

This guide explains how to configure the **initial 9 Decentralized Directors** for your Ëtrid network using genesis configuration.

### Key Numbers
- **Total Directors**: 9
- **Quorum Required**: 6/9 (2/3 majority)
- **Minimum Stake**: 128 ËTR
- **Term Length**: 365 days
- **Emergency Timelock**: 24 hours (14,400 blocks)

---

## Table of Contents

1. [Genesis Configuration Methods](#genesis-configuration-methods)
2. [Method 1: Runtime Preset (Recommended)](#method-1-runtime-preset-recommended)
3. [Method 2: Chain Spec JSON](#method-2-chain-spec-json)
4. [Method 3: Manual Registration After Launch](#method-3-manual-registration-after-launch)
5. [Validator Account Preparation](#validator-account-preparation)
6. [Verification & Testing](#verification--testing)
7. [Common Issues & Solutions](#common-issues--solutions)

---

## Genesis Configuration Methods

You have three options for configuring the initial 9 directors:

| Method | Best For | Complexity | When to Use |
|--------|----------|------------|-------------|
| **Runtime Preset** | Production | Medium | New network launch |
| **Chain Spec JSON** | Testing | Low | Local development |
| **Manual Registration** | Migration | High | Adding to existing chain |

---

## Method 1: Runtime Preset (Recommended)

### Location
`05-multichain/primearc-core/runtime/presets/`

### Step 1: Create Director Preset

Create a new file: `primearc-core-chain_mainnet_v1_with_directors.json`

```json
{
  "name": "Ëtrid Primearc Core Mainnet with Directors",
  "id": "primearc-core-chain_mainnet",
  "chainType": "Live",
  "bootNodes": [],
  "telemetryEndpoints": null,
  "protocolId": "primearc-core-chain",
  "properties": {
    "tokenSymbol": "ETR",
    "tokenDecimals": 12,
    "ss58Format": 42
  },
  "genesis": {
    "runtime": {
      "balances": {
        "balances": [
          // Each director needs 128+ ËTR minimum
          ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 200000000000000000000000],  // Alice (Director 1)
          ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 200000000000000000000000],  // Bob (Director 2)
          ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 200000000000000000000000],  // Charlie (Director 3)
          ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 200000000000000000000000],  // Dave (Director 4)
          ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 200000000000000000000000],  // Eve (Director 5)
          ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 200000000000000000000000],  // Ferdie (Director 6)
          ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", 200000000000000000000000],  // George (Director 7)
          ["5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc", 200000000000000000000000],  // Helen (Director 8)
          ["5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT", 200000000000000000000000]   // Ian (Director 9)
        ]
      },
      "peerRolesStaking": {
        "roles": [
          // Register each director with DecentralizedDirector role (role = 4)
          {
            "account": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
            "role": 4,
            "stake": 128000000000000000000000
          },
          {
            "account": "5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT",
            "role": 4,
            "stake": 128000000000000000000000
          }
        ]
      },
      "decentralizedDirectors": {
        "directors": [
          // Register each director with term start/end
          {
            "account": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            "term_start": 0,
            "term_end": 31536000,  // 365 days in seconds
            "term_active": true
          },
          {
            "account": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          {
            "account": "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          {
            "account": "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          {
            "account": "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          {
            "account": "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          {
            "account": "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          {
            "account": "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          },
          {
            "account": "5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT",
            "term_start": 0,
            "term_end": 31536000,
            "term_active": true
          }
        ],
        "current_epoch": 0
      },
      "directorElection": {
        "elected_directors": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
          "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
          "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
          "5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL",
          "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY",
          "5HpG9w8EBLe5XCrbczpwq5TSXvedjrBGCwqxK1iQ7qUsSWFc",
          "5Ck5SLSHYac6WFt5UZRSsdJjwmpSZq85fd5TRNAdZQVzEAPT"
        ],
        "current_epoch": 0,
        "next_consensus_day_block": 5572800  // 365 days from genesis
      }
    }
  }
}
```

### Step 2: Build with Preset

```bash
cd 05-multichain/primearc-core/runtime
cargo build --release --features runtime-preset
```

### Step 3: Launch Network

```bash
./target/release/primearc-core-node \
  --chain primearc-core-chain_mainnet \
  --validator \
  --name "Director-1-Alice" \
  --base-path /tmp/alice
```

---

## Method 2: Chain Spec JSON

### For Development/Testing

```bash
# Generate base chain spec
./target/release/primearc-core-node build-spec --chain development > dev-spec.json

# Edit dev-spec.json to add directors (see JSON structure above)

# Convert to raw format
./target/release/primearc-core-node build-spec \
  --chain dev-spec.json \
  --raw > dev-spec-raw.json

# Launch with custom spec
./target/release/primearc-core-node \
  --chain dev-spec-raw.json \
  --validator
```

---

## Method 3: Manual Registration After Launch

If you already have a running network, you can register directors via extrinsics:

### Step 1: Ensure Directors Have 128+ ËTR Stake

```javascript
// Using Polkadot.js API
const api = await ApiPromise.create({ provider: wsProvider });

// Check balance
const balance = await api.query.system.account(DIRECTOR_ADDRESS);
console.log("Balance:", balance.data.free.toString());

// Must have at least 128 ËTR = 128000000000000000000000 smallest units
```

### Step 2: Assign DecentralizedDirector Role

```javascript
// Requires sudo/root access
await api.tx.peerRolesStaking
  .assignRole(
    4, // DecentralizedDirector role
    "128000000000000000000000" // 128 ËTR stake
  )
  .signAndSend(directorAccount);
```

### Step 3: Register as Director

```javascript
// Each director registers themselves
await api.tx.decentralizedDirectors
  .registerDirector()
  .signAndSend(directorAccount);
```

### Step 4: Verify Registration

```javascript
// Check if director is registered
const directorProfile = await api.query.decentralizedDirectors.directors(DIRECTOR_ADDRESS);
console.log("Director Profile:", directorProfile.toHuman());

// Check all current directors
const currentEpoch = await api.query.decentralizedDirectors.currentEpoch();
const directors = await api.query.decentralizedDirectors.directorTerms(currentEpoch);
console.log("All Directors:", directors.toHuman());
```

---

## Validator Account Preparation

### Generating Director Accounts

You need 9 accounts with secret keys:

```bash
# Using subkey
for i in {1..9}; do
  subkey generate --scheme sr25519 > director-$i-keys.txt
done

# Each file contains:
# Secret phrase: <12-word mnemonic>
# Network ID: substrate
# Secret seed: 0x...
# Public key (hex): 0x...
# Account ID: 0x...
# Public key (SS58): 5...
# SS58 Address: 5...
```

### Production Security

**CRITICAL**: For mainnet, use hardware wallets or secure key management:

1. **Hardware Wallets**: Ledger, Trezor, Polkadot Vault
2. **Multi-Sig**: Use `pallet-multisig` for 6/9 threshold
3. **Cold Storage**: Keep 6 keys offline, 3 hot for operations
4. **Key Rotation**: Plan annual key rotation on Consensus Day

### Recommended Setup

```
Directors 1-3: Hot wallets (active governance)
Directors 4-6: Warm wallets (emergency response)
Directors 7-9: Cold storage (final approval)
```

This ensures:
- ✅ 3 directors can propose actions
- ✅ 6 directors required for approval (2/3)
- ✅ Even if 3 compromised, cannot reach quorum

---

## Verification & Testing

### 1. Check Director Count

```javascript
const directors = await api.query.decentralizedDirectors.directors.entries();
console.log("Total Directors:", directors.length);
// Expected: 9
```

### 2. Verify Each Director

```javascript
for (const [key, profile] of directors) {
  const address = key.args[0].toString();
  const info = profile.toHuman();
  console.log(`Director: ${address}`);
  console.log(`  Term Start: ${info.termStart}`);
  console.log(`  Term End: ${info.termEnd}`);
  console.log(`  Active: ${info.termActive}`);
}
```

### 3. Test Emergency Action Proposal

```javascript
// Propose emergency action
const action = api.createType('EmergencyAction', {
  FastTrack: { proposalId: 1 }
});

await api.tx.decentralizedDirectors
  .proposeEmergencyAction(action, "Test proposal")
  .signAndSend(director1);

// Vote on proposal (need 6 votes for quorum)
for (let i = 0; i < 6; i++) {
  await api.tx.decentralizedDirectors
    .voteEmergencyAction(0, true) // proposal_id, approve
    .signAndSend(directors[i]);
}

// Check vote count
const proposal = await api.query.decentralizedDirectors.emergencyActions(0);
console.log("Votes:", proposal.unwrap().votesFor.toNumber());
// Expected: 6 or more
```

### 4. Test Quorum Enforcement

```javascript
// Try to execute with only 5 votes (should fail)
await api.tx.decentralizedDirectors
  .executeEmergencyAction(0)
  .signAndSend(anyAccount);
// Expected error: "NotApproved" (needs 6/9)

// Vote with 6th director
await api.tx.decentralizedDirectors
  .voteEmergencyAction(0, true)
  .signAndSend(director6);

// Wait for timelock (14,400 blocks = 24 hours)
// Then execute should succeed
```

---

## Common Issues & Solutions

### Issue 1: "InsufficientStake" Error

**Problem**: Director doesn't have 128+ ËTR staked

**Solution**:
```javascript
// Check stake
const role = await api.query.peerRolesStaking.roles(DIRECTOR_ADDRESS);
console.log("Current Stake:", role.unwrap().stake.toString());

// Increase stake if needed
await api.tx.peerRolesStaking
  .increaseStake("10000000000000000000000") // Add 10 ËTR
  .signAndSend(directorAccount);
```

### Issue 2: "NoActiveRole" Error

**Problem**: Account doesn't have DecentralizedDirector role

**Solution**:
```javascript
// Assign role first (requires root)
await api.tx.sudo.sudo(
  api.tx.peerRolesStaking.assignRole(
    4, // DecentralizedDirector
    "128000000000000000000000"
  )
).signAndSend(sudoAccount);
```

### Issue 3: Only 8 Directors Showing

**Problem**: One director registration failed

**Solution**:
```bash
# Check logs for registration errors
grep "DirectorRegistered" node.log

# Re-register missing director
# (Use Method 3 steps above)
```

### Issue 4: Quorum Not Met (5/9 instead of 6/9)

**Problem**: Incorrect QUORUM_DIRECTORS constant

**Solution**:
```rust
// In decentralized-directors/src/lib.rs
pub const QUORUM_DIRECTORS: u32 = 6; // Must be 2/3 of 9

// Rebuild runtime
cargo build --release
```

### Issue 5: Term Already Ended

**Problem**: Genesis configured terms already expired

**Solution**:
```javascript
// Check current time vs term_end
const now = await api.query.timestamp.now();
const director = await api.query.decentralizedDirectors.directors(ADDRESS);
console.log("Now:", now.toNumber());
console.log("Term End:", director.unwrap().termEnd.toNumber());

// If expired, trigger new election or extend terms via governance
```

---

## Production Deployment Checklist

Before launching mainnet with 9 directors:

- [ ] Generate 9 secure validator accounts
- [ ] Distribute keys securely (3 hot, 3 warm, 3 cold)
- [ ] Each account has 200+ ËTR (128 stake + operational buffer)
- [ ] All 9 accounts registered in genesis preset
- [ ] All 9 have DecentralizedDirector role (role = 4)
- [ ] All 9 have 128 ËTR staked
- [ ] Genesis election sets elected_directors list
- [ ] Next Consensus Day calculated (365 days from launch)
- [ ] Quorum verified as 6/9 in code
- [ ] Emergency timelock set to 24 hours (14,400 blocks)
- [ ] Multi-sig configured for critical director keys
- [ ] Backup recovery plan documented
- [ ] Director responsibilities assigned
- [ ] Communication channels established
- [ ] Emergency response procedures documented
- [ ] Legal/governance framework finalized

---

## Next Steps

After genesis configuration:

1. **Test Emergency Actions**: Ensure 6/9 quorum works
2. **Schedule First Consensus Day**: Set date 365 days from launch
3. **Document Director Responsibilities**: Who handles what
4. **Setup Monitoring**: Track director activity and votes
5. **Plan Key Rotation**: Annual ceremony on Consensus Day
6. **Establish Communication**: Secure channels for coordination

---

## Support & Resources

- **Documentation**: `/11-peer-roles/ARCHITECTURE.md`
- **Pallet Code**: `/11-peer-roles/decentralized-directors/`
- **Election Pallet**: `/12-consensus-day/pallet-director-election/`
- **Integration Guide**: See INTEGRATION_GUIDE.md in each pallet

---

**Last Updated**: 2025-11-22
**Network**: Ëtrid Primearc Core 
**Directors**: 9 (updated from previous 21-director model)
**Quorum**: 6/9 (2/3 majority)
