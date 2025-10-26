# 🚀 FlareChain Mainnet Genesis - Implementation Guide

**Date:** October 24, 2025
**Status:** 🟡 READY TO IMPLEMENT
**Tokenomics Source:** `/docs/TOKEN_ALLOCATION_FOR_LIQUIDITY.md`

---

## ✅ Your Tokenomics (CONFIRMED)

### ÉTR Token Configuration

| Parameter | Value | Source |
|-----------|-------|--------|
| **Token Symbol** | `ÉTR` (with accent) | TOKEN_ALLOCATION_FOR_LIQUIDITY.md |
| **Token Decimals** | 18 (needs confirmation) | Placeholder config shows 18 |
| **Total Supply** | 2,500,000,000 (2.5B) | TOKEN_ALLOCATION_FOR_LIQUIDITY.md |
| **Chain Type** | Live (Mainnet) | - |
| **Protocol ID** | `flarechain` | - |

### Token Distribution (From Your Docs)

| Allocation | % | Amount (ÉTR) | Amount (with 18 decimals) |
|------------|---|--------------|---------------------------|
| **DAO Treasury / Protocol Reserve** | 35% | 875,000,000 | 875000000000000000000000000 |
| **Community Liquidity & LP Incentives** | 10% | 250,000,000 | 250000000000000000000000000 |
| **Foundation / Team Vesting** | 15% | 375,000,000 | 375000000000000000000000000 |
| **Network Expansion / Partnerships** | 25% | 625,000,000 | 625000000000000000000000000 |
| **Founders' Creation Pool** | 5% | 125,000,000 | 125000000000000000000000000 |
| **Initial Circulating Supply** | ~10% | 250,000,000 | 250000000000000000000000000 |
| **TOTAL** | **100%** | **2,500,000,000** | **2500000000000000000000000000** |

---

## 📝 Genesis Preset Template Created

**Location:** `05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json`

**Status:** ⚠️ **TEMPLATE ONLY - Needs Real Addresses**

### What's Already Configured:

✅ Token distribution amounts (based on your tokenomics)
✅ 7 Foundation validators (100M ETR stake each)
✅ 7 GRANDPA authorities
✅ 6-second block time
✅ Foundation multisig as sudo key

### What YOU Need to Replace:

All placeholder addresses with real ones:

```json
{
  "balances": {
    "balances": [
      // REPLACE THESE with real SS58 addresses ↓
      ["FOUNDATION_TREASURY_MULTISIG_ADDRESS_HERE", 875000000000000000000000000],
      ["COMMUNITY_LP_POOL_ADDRESS_HERE", 250000000000000000000000000],
      ["FOUNDATION_TEAM_VESTING_ADDRESS_HERE", 375000000000000000000000000],
      ["NETWORK_EXPANSION_POOL_ADDRESS_HERE", 625000000000000000000000000],
      ["FOUNDERS_POOL_ADDRESS_HERE", 125000000000000000000000000],
      ["INITIAL_CIRCULATING_SUPPLY_ADDRESS_HERE", 250000000000000000000000000]
    ]
  },
  "sudo": {
    "key": "FOUNDATION_MULTISIG_ADDRESS_HERE"  // ← REPLACE
  },
  "grandpa": {
    "authorities": [
      // REPLACE THESE with real Ed25519 public keys ↓
      ["GRANDPA_KEY_1_ED25519_HERE", 1],
      ["GRANDPA_KEY_2_ED25519_HERE", 1],
      ...
    ]
  },
  "consensus": {
    "validators": [
      // REPLACE THESE with real Sr25519 addresses ↓
      ["VALIDATOR_1_STASH_ADDRESS_HERE", 100000000000000000000000, "Foundation-NA-1"],
      ...
    ]
  }
}
```

---

## 🔐 Step-by-Step: Generate Production Keys

### CRITICAL SECURITY WARNING ⚠️

**NEVER use these for mainnet:**
- ❌ Alice, Bob, Charlie, etc. (test accounts)
- ❌ `./etrid key generate` (uses weak entropy)
- ❌ Keys generated on internet-connected machines
- ❌ Any keys from tutorials/examples

**MUST use:**
- ✅ Hardware Security Module (HSM) for Foundation validators
- ✅ Air-gapped computer for key generation
- ✅ Secure key ceremony with witnesses
- ✅ Proper key backup (multiple encrypted copies)

---

### Option 1: Using Subkey (Substrate Key Tool)

**Install subkey:**
```bash
cargo install --force --git https://github.com/paritytech/polkadot-sdk subkey
```

**Generate Foundation Multisig Members:**
```bash
# Generate 5 or 7 signers for Foundation multisig
for i in {1..7}; do
    echo "=== Generating Signer $i ==="
    subkey generate --scheme Sr25519 --network substrate --output-type json > foundation_signer_$i.json
    cat foundation_signer_$i.json
    echo ""
done
```

**Generate Validator Stash Accounts:**
```bash
# Generate 7 validator stash accounts
for i in {1..7}; do
    echo "=== Generating Validator $i Stash ==="
    subkey generate --scheme Sr25519 --network substrate --output-type json > validator_${i}_stash.json
    cat validator_${i}_stash.json
    echo ""
done
```

**Generate GRANDPA Keys (Ed25519):**
```bash
# Generate 7 GRANDPA finality keys
for i in {1..7}; do
    echo "=== Generating GRANDPA Key $i ==="
    subkey generate --scheme Ed25519 --network substrate --output-type json > grandpa_key_$i.json
    cat grandpa_key_$i.json
    echo ""
done
```

**Generate Pool Management Accounts:**
```bash
# DAO Treasury
subkey generate --scheme Sr25519 --network substrate --output-type json > dao_treasury.json

# Community LP Pool
subkey generate --scheme Sr25519 --network substrate --output-type json > community_lp_pool.json

# Foundation Team Vesting
subkey generate --scheme Sr25519 --network substrate --output-type json > team_vesting.json

# Network Expansion
subkey generate --scheme Sr25519 --network substrate --output-type json > network_expansion.json

# Founders Pool
subkey generate --scheme Sr25519 --network substrate --output-type json > founders_pool.json

# Initial Circulating
subkey generate --scheme Sr25519 --network substrate --output-type json > initial_circulating.json
```

---

### Option 2: Using Hardware Wallet (Ledger/Trezor)

**For Foundation validators, STRONGLY recommended:**

```bash
# Derive from hardware wallet
# Example with Polkadot.js:
# 1. Connect Ledger
# 2. Open Polkadot.js Apps
# 3. Settings → Manage hardware connections
# 4. Derive accounts with path: //foundation//validator//1, //foundation//validator//2, etc.
```

---

### Option 3: Multi-Party Computation (MPC) - Most Secure

For Foundation multisig, consider MPC:
- Threshold signatures (e.g., 5-of-7)
- No single point of failure
- Distributed key generation

**Tools:**
- [TSS implementation](https://github.com/binance-chain/tss-lib)
- Professional security service (recommended)

---

## 📋 Creating Foundation Multisig

Once you have 5-7 individual signer accounts:

```bash
# Using Polkadot.js Apps:
# 1. Go to Accounts → Multisig
# 2. Create new multisig
# 3. Add 5-7 signers
# 4. Set threshold (3-of-5 or 5-of-7)
# 5. Copy multisig address

# Or using subkey:
subkey inspect --scheme Sr25519 "MULTISIG_SS58_ADDRESS"
```

**Example Multisig Configuration:**
- **Threshold:** 5-of-7 (requires 5 signatures)
- **Signers:**
  - CEO / Founder
  - CTO
  - COO
  - Board Member 1
  - Board Member 2
  - Legal Counsel
  - Security Officer

---

## 🔧 Completing the Genesis Configuration

### Step 1: Fill in All Addresses

**Edit:** `05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json`

Replace ALL placeholder text with real addresses from your key generation:

```json
{
  "balances": {
    "balances": [
      // Use your REAL addresses ↓
      ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 875000000000000000000000000],  // DAO Treasury
      ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 250000000000000000000000000],  // Community LP
      ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 375000000000000000000000000],  // Team Vesting
      ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 625000000000000000000000000],  // Network Expansion
      ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 125000000000000000000000000],  // Founders Pool
      ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 250000000000000000000000000]   // Initial Circulating
    ]
  },
  "sudo": {
    "key": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"  // Foundation Multisig
  },
  "grandpa": {
    "authorities": [
      ["5FA9nQDVg267DEd8m1ZypXLBnvN7SFxYwV7ndqSYGiN9TTpu", 1],  // GRANDPA Key 1
      ["5GoNkf6WdbxCFnPdAnYYQyCjAKPJgLNxXwPjwTh6DGg6gN3E", 1],  // GRANDPA Key 2
      ["5Fe3jZRbKes6aeuQ6HkcTvQeNhkkRPTXBwmNkuAPoimGEv45", 1],  // GRANDPA Key 3
      ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 1],  // GRANDPA Key 4
      ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 1],  // GRANDPA Key 5
      ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 1],  // GRANDPA Key 6
      ["5FfBQ3kwXrbdyoqLPvcXRp7ikWydXawpNs2Ceu3WwFdhZ8W4", 1]   // GRANDPA Key 7
    ]
  },
  "consensus": {
    "validators": [
      ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 100000000000000000000000, "Foundation-NA-1"],
      ["5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", 100000000000000000000000, "Foundation-EU-1"],
      ["5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y", 100000000000000000000000, "Foundation-ASIA-1"],
      ["5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy", 100000000000000000000000, "Foundation-NA-2"],
      ["5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw", 100000000000000000000000, "Foundation-EU-2"],
      ["5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL", 100000000000000000000000, "Foundation-ASIA-2"],
      ["5FfBQ3kwXrbdyoqLPvcXRp7ikWydXawpNs2Ceu3WwFdhZ8W4", 100000000000000000000000, "Foundation-SA-1"]
    ],
    "slotDuration": 6000
  }
}
```

**⚠️ CRITICAL:** The addresses shown above are EXAMPLES - you MUST replace them with your own securely-generated addresses!

---

### Step 2: Verify Token Amounts

**Double-check all amounts add up:**

```bash
# Total should equal 2.5 billion with 18 decimals
875000000000000000000000000  # DAO Treasury (35%)
250000000000000000000000000  # Community LP (10%)
375000000000000000000000000  # Team Vesting (15%)
625000000000000000000000000  # Network Expansion (25%)
125000000000000000000000000  # Founders Pool (5%)
250000000000000000000000000  # Initial Circulating (10%)
─────────────────────────────
2500000000000000000000000000  # TOTAL = 2.5B ÉTR ✓
```

---

### Step 3: Update Runtime to Load Mainnet Preset

Already done! The runtime is configured to load `flarechain_mainnet.json`.

**Verify in:** `05-multichain/flare-chain/runtime/src/lib.rs:1000-1024`

---

### Step 4: Build Binary with Mainnet Genesis

```bash
cd /Users/macbook/Desktop/etrid

# Clean build to ensure fresh compilation
cargo clean

# Build release binary
cargo build --release --locked

# Verify binary version
./target/release/etrid --version
# Should show: etrid 0.1.0
```

---

### Step 5: Generate Mainnet Chain Specification

```bash
# Generate human-readable spec (for review)
./target/release/etrid --chain flare build-spec --chain mainnet > flarechain-spec.json

# CAREFULLY REVIEW the spec
cat flarechain-spec.json | jq '.'

# Check genesis hash (this will be permanent!)
cat flarechain-spec.json | jq '.genesis'

# Generate RAW spec (for deployment)
./target/release/etrid --chain flare build-spec --chain mainnet --raw > flarechain-raw.json

# Copy to deployment location
cp flarechain-raw.json infrastructure/ansible/files/flarechain-chainspec.json
cp target/release/etrid infrastructure/ansible/files/etrid
```

---

### Step 6: Verify Genesis Configuration

**Check critical parameters:**

```bash
# Verify token symbol
cat flarechain-spec.json | jq '.properties.tokenSymbol'
# Should show: "ÉTR"

# Verify token decimals
cat flarechain-spec.json | jq '.properties.tokenDecimals'
# Should show: 18

# Verify total balances sum
cat flarechain-spec.json | jq '.genesis.runtimeGenesis.patch.balances.balances | map(.[1]) | add'
# Should show: 2500000000000000000000000000

# Count validators
cat flarechain-spec.json | jq '.genesis.runtimeGenesis.patch.consensus.validators | length'
# Should show: 7

# Count GRANDPA authorities
cat flarechain-spec.json | jq '.genesis.runtimeGenesis.patch.grandpa.authorities | length'
# Should show: 7
```

---

## 🔒 Security Checklist (CRITICAL!)

Before deploying to mainnet, verify:

### Keys & Accounts
- [ ] All test accounts removed (no Alice, Bob, etc.)
- [ ] Foundation multisig created and tested
- [ ] Multisig threshold set (5-of-7 recommended)
- [ ] All multisig signers identified and keys secured
- [ ] Validator keys generated with HSM or air-gapped system
- [ ] GRANDPA keys generated securely (Ed25519)
- [ ] All private keys backed up in multiple secure locations
- [ ] Key recovery procedures documented
- [ ] Key access logged and monitored

### Genesis Configuration
- [ ] Token symbol verified: ÉTR
- [ ] Token decimals verified: 18
- [ ] Total supply verified: 2,500,000,000 ÉTR
- [ ] All allocations match tokenomics document
- [ ] No typos in addresses (double-check!)
- [ ] Validator stakes configured: 100M ÉTR each
- [ ] GRANDPA authorities configured: 7 total
- [ ] Sudo key set to Foundation multisig
- [ ] Block time confirmed: 6 seconds

### Testing
- [ ] Genesis JSON validates without errors
- [ ] RAW chain spec generated successfully
- [ ] Genesis hash calculated
- [ ] Test node started with chain spec
- [ ] Genesis block created successfully
- [ ] Validators can sign blocks
- [ ] GRANDPA finality works
- [ ] Balance transfers work
- [ ] Multisig transactions work

### Distribution Addresses Verified
- [ ] DAO Treasury address: _________ (875M ÉTR)
- [ ] Community LP Pool: _________ (250M ÉTR)
- [ ] Team Vesting: _________ (375M ÉTR)
- [ ] Network Expansion: _________ (625M ÉTR)
- [ ] Founders Pool: _________ (125M ÉTR)
- [ ] Initial Circulating: _________ (250M ÉTR)

---

## 📊 Mainnet vs Testnet Comparison

| Parameter | Ember Testnet | FlareChain Mainnet |
|-----------|---------------|-------------------|
| **Chain ID** | `ember_testnet` | `flarechain` |
| **Token Symbol** | `ETR` | `ÉTR` |
| **Decimals** | 12 | 18 |
| **Total Supply** | 2B (test) | 2.5B (production) |
| **Validators** | 3 | 7 |
| **Accounts** | Test (Alice, Bob) | Real (Multisig) |
| **Sudo** | Single key | Multisig (5-of-7) |
| **Purpose** | Testing | Production |
| **Value** | $0 | Real market value |

---

## 🎯 Next Steps

### Immediate (This Week)
1. **Generate all production keys** (using HSM or secure method)
2. **Create Foundation multisig** (5-of-7 threshold)
3. **Fill in genesis template** with real addresses
4. **Verify all amounts** add up correctly

### Short Term (Next 2 Weeks)
1. **Build binary** with mainnet genesis
2. **Generate chain spec** (RAW format)
3. **Internal testing** (single node)
4. **Multi-validator testing** (3-7 nodes)

### Before Launch (2-4 Weeks)
1. **Security audit** of genesis configuration
2. **Legal review** of token distribution
3. **Infrastructure provisioned** (7 validator servers)
4. **Monitoring configured**
5. **Backup procedures tested**

### Launch Day
1. **Final verification** of all configurations
2. **Start all 7 validator nodes** simultaneously
3. **Monitor genesis block** creation
4. **Verify finality** occurring
5. **Announce genesis hash** publicly

---

## 💡 Important Notes

### About Vesting

Your tokenomics mentions "3-year linear vesting" for Foundation/Team allocation (375M ÉTR).

**Genesis configuration handles initial distribution only.** You'll need:

1. **Vesting pallet** in runtime (may need to add)
2. **Or** use multisig with time-locked withdrawals
3. **Or** use separate vesting smart contract

**Recommendation:** Start with multisig control, implement vesting pallet in future runtime upgrade.

### About EDSC Token

Your tokenomics includes EDSC stablecoin (1B cap), but:

- ⚠️ **Not included in FlareChain genesis**
- EDSC is a separate token (not part of FlareChain genesis state)
- EDSC mint/burn handled by separate bridge/oracle pallets

**This is correct** - EDSC minting happens post-genesis through Proof-of-Reserve mechanism.

### About Initial Circulating Supply

Your docs mention "~15% initial circulating" but genesis shows 10% allocated to "Initial Circulating Supply" address.

**Clarify:**
- Is 250M ÉTR (10%) the actual initial circulation?
- Or should it be 375M ÉTR (15%)?
- Update genesis if needed

---

## 📞 Questions to Resolve

Before finalizing mainnet genesis:

1. **Token Decimals:** Confirm 18 decimals (currently placeholder shows 18, testnet uses 12)
2. **Initial Circulating:** Confirm 250M ÉTR (10%) or 375M ÉTR (15%)?
3. **Vesting Implementation:** How to implement 3-year vesting for team allocation?
4. **Sudo Timeline:** Keep sudo for 6-12 months, or remove from day 1?
5. **Validator Count:** Start with 7 Foundation validators, or more?

---

**Status:** 🟡 **READY TO IMPLEMENT**

**Next Action:** Generate production keys using secure method

**Timeline:** 2-4 weeks from key generation to mainnet launch

---

## 🚀 Quick Reference

**Files to Edit:**
```
05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json
```

**Commands to Run:**
```bash
# Generate keys
subkey generate --scheme Sr25519 --network substrate

# Build binary
cargo build --release --locked

# Generate chain spec
./target/release/etrid --chain flare build-spec --chain mainnet --raw > flarechain-raw.json
```

**Key Addresses Needed:**
- 1 Foundation multisig (DAO Treasury + Sudo)
- 6 pool management accounts (LP, Vesting, Expansion, Founders, Circulating)
- 7 validator stash accounts
- 7 GRANDPA authority keys (Ed25519)

---

**Questions?** Let me know which addresses you need help generating!
