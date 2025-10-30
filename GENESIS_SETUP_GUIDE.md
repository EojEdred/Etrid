# Ëtrid Genesis Setup Guide
## Complete Mainnet & EDSC Stablecoin Genesis Configuration

**Last Updated:** October 30, 2025
**Status:** ✅ Ready for Account Generation

---

## 📋 Overview

This guide covers the complete genesis setup for Ëtrid FlareChain mainnet, including:

1. **ETR Native Token** (2.5B supply) - Tokenomics distribution
2. **EDSC Stablecoin** - Multi-asset reserve infrastructure
3. **21 Validators** - Payment + session accounts
4. **Foundation Governance** - Multisig + sudo key
5. **Team Vesting** - 375M ETR with vesting schedules

---

## 🎯 What Gets Created

### ETR Token Distribution (2.5B Total)

| Account | Amount | % | Purpose |
|---------|--------|---|---------|
| DAO Treasury | 875M ETR | 35% | Protocol reserve, governance spending |
| Community LP | 250M ETR | 10% | Liquidity pools, LP incentives |
| Team Vesting | 375M ETR | 15% | Team/advisors with 3-year vesting |
| Network Expansion | 625M ETR | 25% | Partnerships, grants, ecosystem |
| Founders Pool | 125M ETR | 5% | Founders' creation incentive |
| Initial Circulating | 250M ETR | 10% | Public distribution, exchanges |
| **TOTAL** | **2,500M ETR** | **100%** | |

### EDSC Stablecoin Infrastructure

| Account | Purpose |
|---------|---------|
| Reserve Vault | Main custody for multi-asset reserves |
| Oracle Authority | Price feed signer for reserve assets |
| Custodian Manager | Manages approved custodians |
| Minter Authority | Authorized to mint/burn EDSC |
| Emergency Pause | Can pause EDSC in emergencies |
| BTC Custodian | Bitcoin reserve custody |
| ETH Custodian | Ethereum reserve custody |
| Gold Custodian | Physical gold reserve custody |
| USDC Custodian | USDC reserve custody |
| USDT Custodian | USDT reserve custody |

### Validator Accounts

- **21 Payment Accounts:** Where validator rewards are sent (cold storage)
- **21 Session Account Sets:** AURA, GRANDPA, ASF keys (hot keys on validator VMs)
- **21 Network Keys:** P2P libp2p node keys

### Foundation Governance

- **7 Multisig Signers:** Individual accounts for Foundation members
- **5-of-7 Threshold:** Requires 5 signatures for sudo operations
- **Sudo Key:** Foundation multisig controls chain governance

### Team & Advisors

- **10 Individual Accounts:** CEO, CTO, devs, advisors, marketing
- **Vesting Schedules:** 6-36 month linear vesting with optional cliffs
- **Total: 375M ETR**

---

## 🚀 Quick Start (5 Minutes)

### Step 1: Generate All Accounts

```bash
# Clone repository
cd ~/Desktop/etrid

# Run account generation script
./generate-genesis-accounts.sh
```

**Output:**
- Creates directory: `genesis-accounts-YYYYMMDD-HHMMSS/`
- Generates ~60 JSON key files
- Creates summary: `GENESIS_ACCOUNTS_SUMMARY.md`

**Time:** ~2 minutes

### Step 2: Create Genesis Configuration

```bash
# Run genesis config creator
./create-mainnet-genesis-config.sh genesis-accounts-YYYYMMDD-HHMMSS
```

**Output:**
- Creates: `flarechain_mainnet_genesis.json`
- Creates: `GENESIS_CONFIG_SUMMARY.md`
- Validates JSON structure
- Verifies token amounts

**Time:** ~10 seconds

### Step 3: Create Foundation Multisig

```bash
# Use Polkadot.js Apps or subkey to create multisig
# from the 7 signer accounts generated in Step 1

# Example with subkey:
subkey inspect --scheme Sr25519 "MULTISIG_ADDRESS"
```

Update `sudo.key` in `flarechain_mainnet_genesis.json` with multisig address.

### Step 4: Deploy to Runtime

```bash
# Copy genesis config to runtime presets
cp flarechain_mainnet_genesis.json \
   05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json

# Build binary
cargo build --release --locked

# Generate chain spec
./target/release/flarechain-node build-spec \
  --chain mainnet --raw > flarechain-raw.json
```

**Done!** You now have a complete mainnet genesis configuration.

---

## 📊 Detailed Account Breakdown

### 1. ETR Tokenomics Accounts

#### DAO Treasury (875M ETR)
- **Purpose:** Protocol reserve, governance spending, emergency funds
- **Control:** Foundation multisig (5-of-7)
- **Usage:** On-chain governance proposals
- **Lockup:** None (available immediately)

#### Community LP & Incentives (250M ETR)
- **Purpose:** DEX liquidity pools, LP rewards, farming incentives
- **Control:** DAO governance or automated smart contract
- **Usage:** Bootstrap liquidity on launch, ongoing incentives
- **Lockup:** Released over time via governance

#### Foundation / Team Vesting (375M ETR)
- **Purpose:** Team salaries, advisor compensation
- **Control:** Individual accounts with vesting schedules
- **Usage:** Monthly unlocks after cliff periods
- **Lockup:** 6-36 months linear vesting

#### Network Expansion (625M ETR)
- **Purpose:** Partnerships, grants, ecosystem development
- **Control:** Foundation multisig
- **Usage:** Strategic allocations as needed
- **Lockup:** None (discretionary spending)

#### Founders Pool (125M ETR)
- **Purpose:** Founder rewards for creation/development
- **Control:** Founder accounts
- **Usage:** Long-term holding, governance participation
- **Lockup:** Optional vesting (recommended)

#### Initial Circulating Supply (250M ETR)
- **Purpose:** Public distribution, exchange listings, airdrops
- **Control:** Foundation multisig initially
- **Usage:** Market making, exchange deposits, community distribution
- **Lockup:** None (immediately tradable)

### 2. EDSC Stablecoin Infrastructure

#### Reserve Vault Account
- **What it stores:** References to custodial accounts
- **Not stored on-chain:** Actual BTC, ETH, gold, USDC, USDT
- **Stored on-chain:** Proof-of-reserve attestations
- **Control:** Custodian multisigs (off-chain)

#### EDSC Minting Process
1. User deposits BTC/ETH/etc to custodian
2. Custodian provides proof to Oracle
3. Oracle signs attestation
4. Minter Authority mints EDSC 1:1 with USD value
5. User receives EDSC on FlareChain

#### Reserve Asset Custodians
- **BTC:** Regulated custody service (e.g., Coinbase Custody, BitGo)
- **ETH:** Regulated custody service
- **Gold:** Physical gold custodian (vaulted, audited)
- **USDC:** Circle account or custody service
- **USDT:** Tether account or custody service

**Important:** Custodian accounts generated here are **authorities**, not actual asset storage.

### 3. Validator Payment Accounts

Each of 21 validators has:

- **Payment Account (Cold):**
  - Receives block rewards
  - Receives transaction fees
  - Stored offline (hardware wallet recommended)
  - Genesis balance: 1M ETR (for initial staking)

- **Session Keys (Hot):**
  - AURA (block production)
  - GRANDPA (finality)
  - ASF (attestation)
  - Stored on validator VM
  - From existing `validator-keys-complete.json`

**Why 1M ETR each?**
- Minimum stake: 128k ETR
- Buffer for operations: 872k ETR
- Total validator allocation: 21M ETR

---

## 🔐 Security Best Practices

### Key Generation

**DO:**
- ✅ Use `subkey` from official Polkadot SDK
- ✅ Generate on air-gapped computer
- ✅ Use hardware wallets for Foundation signers
- ✅ Backup to multiple encrypted locations
- ✅ Test recovery procedures

**DON'T:**
- ❌ Use online key generators
- ❌ Generate on shared computers
- ❌ Store private keys in cloud unencrypted
- ❌ Use test accounts (Alice, Bob, etc.)
- ❌ Share private keys via email/chat

### Backup Strategy

**3-2-1 Rule:**
- **3 copies:** Original + 2 backups
- **2 different media:** USB drive + paper
- **1 off-site:** Bank vault or secure location

**Backup Contents:**
1. All JSON key files (encrypted)
2. Genesis configuration (plaintext OK)
3. Foundation multisig signer keys (separate encryption)
4. Recovery procedures documentation

### Foundation Multisig

**Recommended: 5-of-7 Threshold**

- Requires 5 of 7 signatures for any sudo operation
- Prevents single point of failure
- Allows for key rotation if compromised
- Suitable for production mainnet

**Signer Distribution:**
1. CEO/Founder
2. CTO
3. COO
4. Board Member 1
5. Board Member 2
6. Legal Counsel
7. Security Officer

**Alternative: 3-of-5 Threshold**
- More nimble for rapid iteration
- Lower security margin
- Suitable for testnet or early mainnet

---

## 📝 Vesting Schedules

### CEO/Founder (75M ETR)
- **Total:** 75,000,000 ETR (20% of team pool)
- **Cliff:** 12 months
- **Vesting:** 24 months linear (after cliff)
- **Total Time:** 36 months

### CTO (56.25M ETR)
- **Total:** 56,250,000 ETR (15% of team pool)
- **Cliff:** 12 months
- **Vesting:** 24 months linear
- **Total Time:** 36 months

### Core Developers (37.5M ETR each, 3 devs)
- **Total:** 37,500,000 ETR per dev (10% each)
- **Cliff:** 6 months
- **Vesting:** 30 months linear
- **Total Time:** 36 months

### AI Director (30M ETR)
- **Total:** 30,000,000 ETR (8% of team pool)
- **Cliff:** 6 months
- **Vesting:** 30 months linear
- **Total Time:** 36 months

### Advisors (26.25M ETR each, 3 advisors)
- **Total:** 26,250,000 ETR per advisor (7% each)
- **Cliff:** None
- **Vesting:** 36 months linear
- **Total Time:** 36 months

### Marketing Lead (23.5M ETR)
- **Total:** 23,500,000 ETR (6.27% of team pool)
- **Cliff:** None
- **Vesting:** 36 months linear
- **Total Time:** 36 months

**Total Team Allocation: 375M ETR**

---

## 🧪 Testing Before Mainnet

### Step 1: Deploy to Devnet

```bash
# Use test accounts first
./target/release/flarechain-node \
  --chain dev \
  --alice

# Verify:
# - Genesis block created
# - Balances correct
# - Validators producing blocks
# - Finality working
```

### Step 2: Internal Testnet (3-7 Validators)

```bash
# Use same genesis config
# Start multiple nodes
# Test all critical functions
```

### Step 3: Public Testnet (Ember)

```bash
# Deploy to Ember testnet
# Community testing
# Bug bounty program
# Stress testing
```

### Checklist

- [ ] Genesis block created successfully
- [ ] All token amounts correct
- [ ] Validators producing blocks
- [ ] GRANDPA finality working
- [ ] Balance transfers work
- [ ] Staking works
- [ ] Governance proposals work
- [ ] Multisig transactions work
- [ ] EDSC minting disabled (post-genesis feature)
- [ ] Vesting schedules enforced
- [ ] No test accounts remain

---

## 🚨 Common Issues & Solutions

### Issue: "subkey not found"

```bash
# Install subkey
cargo install --force --git https://github.com/paritytech/polkadot-sdk subkey
```

### Issue: "Genesis validation failed"

- Check all addresses are valid SS58 format
- Verify JSON syntax (use jq to validate)
- Ensure token amounts use correct decimals (18 for ETR)
- Check total supply matches expected (2.5B ETR)

### Issue: "Multisig address not working"

- Verify all signer addresses are correct
- Check threshold is achievable (5 ≤ 7)
- Ensure multisig derivation uses same network
- Test multisig on devnet first

### Issue: "Validators not producing blocks"

- Verify session keys inserted correctly
- Check GRANDPA keys match genesis config
- Ensure validator has minimum stake (128k ETR)
- Check firewall allows P2P port (30333)

---

## 📞 Support & Resources

**Documentation:**
- `BUILD_FIXES_SUMMARY.md` - Build process
- `VALIDATOR_QUICKSTART.md` - Validator setup
- `MAINNET_GENESIS_IMPLEMENTATION.md` - Detailed tokenomics

**Scripts:**
- `generate-genesis-accounts.sh` - Account generation
- `create-mainnet-genesis-config.sh` - Genesis config creation
- `bootstrap-validator.sh` - Validator setup automation

**Key Files:**
- `validator-keys-complete.json` - All 21 validator keys
- `flarechain_mainnet_genesis.json` - Genesis configuration
- `GENESIS_ACCOUNTS_SUMMARY.md` - Account listing

---

## ✅ Pre-Launch Checklist

### Keys & Accounts
- [ ] All keys generated securely (air-gapped or HSM)
- [ ] Foundation multisig created and tested
- [ ] All validator session keys generated
- [ ] All keys backed up (3-2-1 strategy)
- [ ] Recovery procedures documented and tested

### Genesis Configuration
- [ ] Token symbol: ETR (18 decimals)
- [ ] Total supply: 2,500,000,000 ETR
- [ ] All allocations match tokenomics
- [ ] Sudo key = Foundation multisig
- [ ] 21 validators configured
- [ ] EDSC infrastructure accounts configured
- [ ] JSON validates without errors

### Testing
- [ ] Devnet deployment successful
- [ ] Internal testnet (3-7 nodes) tested
- [ ] Public testnet (Ember) tested
- [ ] All critical functions verified
- [ ] Security audit completed
- [ ] Load testing completed

### Infrastructure
- [ ] 21 validator VMs provisioned
- [ ] Firewalls configured (port 30333 open)
- [ ] Monitoring configured
- [ ] Backup procedures in place
- [ ] Disaster recovery plan documented

### Legal & Compliance
- [ ] Token distribution legal review
- [ ] Securities compliance verified
- [ ] Terms of service finalized
- [ ] Privacy policy published
- [ ] Jurisdictional research completed

### Communication
- [ ] Genesis hash announced publicly
- [ ] Launch date communicated
- [ ] Community informed
- [ ] Exchange listings scheduled
- [ ] Documentation published

---

## 🎯 Launch Day Procedure

### T-24 hours
1. Final verification of all configurations
2. All validator nodes ready
3. Foundation multisig signers on standby
4. Communication channels active

### T-1 hour
1. All validators sync clocks
2. Final chain spec distributed
3. All nodes ready to start

### T-0 (Launch)
1. Start all 21 validators simultaneously
2. Monitor genesis block creation
3. Verify finality occurring
4. Check initial balances
5. Announce genesis hash

### T+1 hour
1. Verify block production stable
2. Check validator performance
3. Monitor P2P connections
4. Prepare status update

### T+24 hours
1. Full system health check
2. Community status update
3. Enable public access (RPCs, explorers)
4. Begin exchange listings

---

## 🎉 Congratulations!

You now have a comprehensive genesis setup system for Ëtrid FlareChain including:

✅ ETR tokenomics (2.5B supply)
✅ EDSC stablecoin infrastructure
✅ 21 validator accounts
✅ Foundation governance multisig
✅ Team vesting schedules
✅ Complete automation scripts
✅ Security best practices

**Ready for mainnet deployment!** 🚀

---

**Questions?** Review the documentation or check the generated summary files.

**Need help?** All scripts include detailed error messages and recovery procedures.

**Found a bug?** Open an issue on GitHub with full details.
