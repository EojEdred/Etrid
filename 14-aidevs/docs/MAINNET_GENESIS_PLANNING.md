# 🚀 FlareChain Mainnet Genesis Planning Guide

**Date:** October 24, 2025
**Status:** 📋 PLANNING PHASE
**Testnet Status:** ✅ Ember genesis complete

---

## 📊 Testnet vs Mainnet Comparison

### Current Status

| Aspect | Ember Testnet | FlareChain Mainnet |
|--------|---------------|-------------------|
| **Chain ID** | `ember_testnet` | `flarechain` |
| **Protocol ID** | `ember` | `flarechain` |
| **Token Symbol** | `ETR` | `ÉTR` (or `ETR`) |
| **Token Decimals** | 12 | 18 (needs decision) |
| **Genesis Status** | ✅ Complete | ⚠️ Placeholder only |
| **Validators** | 3 test validators | TBD - Foundation validators |
| **Total Supply** | 2B ETR (testnet) | TBD - Production supply |
| **Sudo** | Enabled | ⚠️ Should be disabled |

---

## 🔑 Critical Mainnet Genesis Decisions Needed

### 1. Token Economics ⚠️ **DECISION REQUIRED**

**Questions to answer:**

#### Token Symbol
- **Current placeholder:** `ÉTR` (with accent)
- **Testnet uses:** `ETR` (no accent)
- **Decision:** Which symbol for mainnet?
  - `ÉTR` - Distinctive branding
  - `ETR` - Simpler, easier to type

#### Token Decimals
- **Current placeholder:** 18 decimals
- **Testnet uses:** 12 decimals
- **Common choices:**
  - 18 decimals (Ethereum-style, 1 ETR = 1,000,000,000,000,000,000 base units)
  - 12 decimals (Polkadot-style, 1 ETR = 1,000,000,000,000 base units)
  - 10 decimals (simpler)
- **Decision:** Which precision level?

#### Total Supply
- **Testnet:** 2B ETR (for testing)
- **Mainnet:** ???
- **Questions:**
  - Fixed supply or inflationary?
  - If fixed: How many total tokens?
  - If inflationary: What's the inflation rate and cap?
  - ICO/Presale supply allocation?

**Recommended approach:**
```
Total Supply: 1B - 10B ETR (choose based on economics model)

Example Distribution:
- Foundation/Treasury:  20-30%
- Team/Advisors:        10-15% (vested)
- ICO/Public Sale:      15-25%
- Ecosystem Incentives: 20-30%
- Validators/Staking:   10-20%
- Reserves:             5-10%
```

---

### 2. Initial Validators ⚠️ **DECISION REQUIRED**

**Testnet uses:** 3 test validators with test keys

**Mainnet needs:**

#### Foundation Validators
- **How many:** 5-10 recommended to start
- **Where hosted:** Multi-region, multi-cloud
- **Keys:** **MUST use HSM or secure key generation ceremony**
- **Stake:** How much ETR per validator?

**Example configuration:**
```json
{
  "validators": [
    {
      "account": "<FOUNDATION_VALIDATOR_1_ADDRESS>",
      "stake": 1000000000000000000000000,  // 1M ETR (adjust)
      "name": "Foundation-NA-1"
    },
    {
      "account": "<FOUNDATION_VALIDATOR_2_ADDRESS>",
      "stake": 1000000000000000000000000,
      "name": "Foundation-EU-1"
    },
    ...
  ]
}
```

#### GRANDPA Finality Authorities
- **Minimum:** 4 for proper Byzantine fault tolerance
- **Recommended:** 7-10 for mainnet
- **Keys:** Must be Ed25519 keys, securely generated

---

### 3. Initial Token Distribution ⚠️ **DECISION REQUIRED**

**Testnet uses:** Test accounts (Alice, Bob, etc.)

**Mainnet needs:**

#### Foundation Treasury
- **Address:** Multisig address (3-of-5 or 5-of-7)
- **Amount:** TBD based on tokenomics
- **Purpose:** Protocol development, grants, operations

#### Team & Advisors
- **Vesting:** Recommended 1-4 year cliff
- **Amount:** TBD
- **Accounts:** Individual or vesting pallet addresses

#### ICO/Presale Participants
- **If applicable:** Addresses from presale
- **Vesting:** May have lockup periods
- **Distribution:** Based on contribution amounts

#### Ecosystem Incentives
- **Purpose:** Developer grants, liquidity mining, partnerships
- **Control:** Foundation multisig
- **Amount:** TBD

#### Example Genesis Balances Structure
```json
{
  "balances": {
    "balances": [
      ["<FOUNDATION_MULTISIG>", 3000000000000000000000000000],  // 30% - Foundation
      ["<TEAM_VESTING_1>", 500000000000000000000000000],       // 5% - Team member 1
      ["<TEAM_VESTING_2>", 500000000000000000000000000],       // 5% - Team member 2
      ["<ECOSYSTEM_FUND>", 2000000000000000000000000000],      // 20% - Ecosystem
      ["<ICO_PARTICIPANT_1>", 100000000000000000000000000],    // 1% - Presale
      ...
    ]
  }
}
```

---

### 4. Governance Configuration ⚠️ **DECISION REQUIRED**

#### Sudo Key
- **Testnet:** Enabled for convenience
- **Mainnet:** ⚠️ **CRITICAL DECISION**
  - **Option 1:** No sudo (full decentralization from day 1)
  - **Option 2:** Foundation multisig as sudo (transitional period)
  - **Option 3:** Sudo with governance upgrade path

**Recommended:** Foundation multisig sudo for first 6-12 months, then remove via governance vote

#### Governance Parameters
```rust
// These go in runtime configuration
MotionDuration: 7 days,
VotingPeriod: 14 days,
EnactmentPeriod: 2 days,
MinimumDeposit: 1000 ETR,  // Adjust based on token price
```

---

### 5. Staking Parameters ⚠️ **DECISION REQUIRED**

#### Validator Economics
- **Minimum stake:** How much ETR to become validator?
- **Maximum validators:** Network capacity (start with 50-100?)
- **Session duration:** How often validator set changes (1 hour? 4 hours? 1 day?)
- **Slash amounts:** Penalties for misbehavior

#### Nominator Economics
- **Minimum nomination:** Minimum ETR to stake
- **Maximum nominators:** Per validator
- **Rewards distribution:** How rewards split between validators and nominators

**Example:**
```json
{
  "staking": {
    "minimumValidatorStake": 500000000000000000000000,  // 500K ETR
    "minimumNominatorStake": 1000000000000000000000,    // 1K ETR
    "validatorCount": 50,
    "sessionsPerEra": 6,  // 1 era = 6 hours if 1 session = 1 hour
    "slashRewardFraction": 10  // 10% of slash goes to reporter
  }
}
```

---

### 6. Security Considerations 🔒

#### Key Generation Ceremony
For mainnet validators, you MUST:
1. **Never use test keys** (Alice, Bob, etc.)
2. **Generate keys securely:**
   - Use HSM (Hardware Security Module) for Foundation validators
   - Air-gapped computers for key generation
   - Multi-party computation (MPC) for critical keys
3. **Backup keys securely:**
   - Multiple encrypted copies
   - Geographic distribution
   - Shamir's Secret Sharing for recovery

#### Multisig Configuration
Foundation treasury should use:
- **Threshold:** 3-of-5 or 5-of-7
- **Signers:** Distributed across leadership
- **Backup:** Recovery mechanism if signer unavailable

---

## 📝 Step-by-Step: Creating Mainnet Genesis

### Step 1: Define Tokenomics (First!)

Create a document with:
```markdown
# Etrid Mainnet Tokenomics

## Token Symbol: [ÉTR or ETR]
## Decimals: [18 or 12]
## Total Supply: [X ETR]

## Distribution:
- Foundation Treasury: X% (X ETR)
- Team & Advisors: X% (X ETR, Y year vest)
- ICO/Presale: X% (X ETR)
- Ecosystem: X% (X ETR)
- Validators: X% (X ETR)
- Reserves: X% (X ETR)

## Validator Economics:
- Initial validators: X
- Minimum stake: X ETR
- Maximum validators: X
- Reward rate: X% per year
```

### Step 2: Generate Production Keys 🔑

**DO NOT SKIP THIS - USE SECURE KEY GENERATION**

```bash
# WRONG - Never do this for mainnet:
./etrid key generate  # This uses test entropy!

# RIGHT - Use proper key generation:
# Option 1: Subkey with secure entropy
subkey generate --scheme Sr25519 --network substrate

# Option 2: Hardware wallet derivation
# Option 3: HSM-based key generation

# For each validator, generate:
# - Controller account (Sr25519)
# - Stash account (Sr25519)
# - Session keys (Sr25519 for BABE/AURA, Ed25519 for GRANDPA)
```

### Step 3: Create Mainnet Genesis Preset

**File:** `05-multichain/flare-chain/runtime/presets/flarechain_mainnet.json`

```json
{
  "balances": {
    "balances": [
      ["YOUR_FOUNDATION_MULTISIG_ADDRESS", 3000000000000000000000000000],
      ["YOUR_VALIDATOR_1_ADDRESS", 1000000000000000000000000],
      ["YOUR_VALIDATOR_2_ADDRESS", 1000000000000000000000000],
      // ... add all initial distribution
    ]
  },
  "sudo": {
    "key": "YOUR_FOUNDATION_MULTISIG_ADDRESS"  // or null to disable
  },
  "grandpa": {
    "authorities": [
      ["YOUR_GRANDPA_KEY_1", 1],
      ["YOUR_GRANDPA_KEY_2", 1],
      ["YOUR_GRANDPA_KEY_3", 1],
      ["YOUR_GRANDPA_KEY_4", 1],
      ["YOUR_GRANDPA_KEY_5", 1]
    ]
  },
  "consensus": {
    "validators": [
      ["YOUR_VALIDATOR_1_ADDRESS", 1000000000000000000000000, "Foundation-NA-1"],
      ["YOUR_VALIDATOR_2_ADDRESS", 1000000000000000000000000, "Foundation-EU-1"],
      ["YOUR_VALIDATOR_3_ADDRESS", 1000000000000000000000000, "Foundation-ASIA-1"],
      ["YOUR_VALIDATOR_4_ADDRESS", 1000000000000000000000000, "Foundation-NA-2"],
      ["YOUR_VALIDATOR_5_ADDRESS", 1000000000000000000000000, "Foundation-EU-2"]
    ],
    "slotDuration": 6000  // 6 seconds
  }
}
```

### Step 4: Update Runtime to Load Mainnet Preset

**File:** `05-multichain/flare-chain/runtime/src/lib.rs`

```rust
fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
    frame_support::genesis_builder_helper::get_preset::<RuntimeGenesisConfig>(id, |name| {
        match name.as_ref() {
            sp_genesis_builder::DEV_RUNTIME_PRESET => {
                Some(include_bytes!("../presets/development.json").to_vec())
            },
            sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => {
                Some(include_bytes!("../presets/local_testnet.json").to_vec())
            },
            "ember_testnet" => {
                Some(include_bytes!("../presets/ember_testnet.json").to_vec())
            },
            "flarechain_mainnet" => {  // ADD THIS
                Some(include_bytes!("../presets/flarechain_mainnet.json").to_vec())
            },
            _ => None,
        }
    })
}

fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
    vec![
        sp_genesis_builder::DEV_RUNTIME_PRESET.into(),
        sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET.into(),
        "ember_testnet".into(),
        "flarechain_mainnet".into(),  // ADD THIS
    ]
}
```

### Step 5: Update Chain Spec Configuration

**File:** `05-multichain/flare-chain/node/src/chain-spec.rs`

Add mainnet config function:

```rust
/// FlareChain mainnet config (production)
pub fn mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(
        wasm_binary,
        None,
    )
    .with_name("Ëtrid FlareChain")
    .with_id("flarechain")
    .with_chain_type(ChainType::Live)
    .with_protocol_id("flarechain")
    .with_properties({
        let mut properties = sc_service::Properties::new();
        properties.insert("tokenSymbol".into(), "ÉTR".into());  // or "ETR"
        properties.insert("tokenDecimals".into(), 18.into());   // or 12
        properties.insert("ss58Format".into(), 42.into());      // Substrate default
        properties
    })
    .with_genesis_config_preset_name("flarechain_mainnet")
    .build())
}
```

Update CLI to use it:

**File:** `05-multichain/flare-chain/node/src/cli.rs`

```rust
fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
    Ok(match id {
        "dev" => Box::new(crate::chain_spec::development_config()?),
        "local" => Box::new(crate::chain_spec::local_testnet_config()?),
        "staging" | "ember" => Box::new(crate::chain_spec::staging_testnet_config()?),
        "" | "flarechain" | "mainnet" => Box::new(crate::chain_spec::mainnet_config()?),  // UPDATE THIS
        path => Box::new(crate::chain_spec::ChainSpec::from_json_file(
            std::path::PathBuf::from(path),
        )?),
    })
}
```

### Step 6: Generate Mainnet Chain Spec

```bash
# Build binary with mainnet genesis
cargo build --release --locked

# Generate human-readable spec (for review)
./target/release/etrid --chain flare build-spec --chain mainnet > flarechain-spec.json

# Review carefully!
cat flarechain-spec.json

# Generate RAW spec for deployment
./target/release/etrid --chain flare build-spec --chain mainnet --raw > flarechain-raw.json

# Deploy to production location
cp flarechain-raw.json infrastructure/ansible/files/flarechain-chainspec.json
```

---

## ⚠️ Pre-Launch Security Checklist

Before mainnet launch, verify:

### Keys & Accounts
- [ ] All test accounts removed (no Alice, Bob, Charlie, etc.)
- [ ] Foundation multisig configured and tested
- [ ] Validator keys generated securely (HSM or air-gapped)
- [ ] GRANDPA keys generated securely
- [ ] All keys backed up in multiple secure locations
- [ ] Key recovery process documented and tested

### Genesis Configuration
- [ ] Token symbol decided and configured
- [ ] Token decimals decided and configured
- [ ] Total supply matches tokenomics document
- [ ] Initial distribution matches tokenomics document
- [ ] All allocation addresses verified (no typos!)
- [ ] Validator stakes configured correctly
- [ ] GRANDPA authorities configured correctly
- [ ] Sudo configuration decided (enabled/disabled)

### Runtime Configuration
- [ ] Sudo pallet configured for production use
- [ ] Governance parameters set appropriately
- [ ] Staking parameters set appropriately
- [ ] Treasury parameters configured
- [ ] Block time confirmed (6 seconds recommended)
- [ ] Session duration confirmed

### Infrastructure
- [ ] Validator servers provisioned (multi-region)
- [ ] Backup validators ready
- [ ] RPC nodes deployed (multiple regions)
- [ ] Monitoring configured
- [ ] Alerting configured
- [ ] Boot nodes configured in chain spec
- [ ] Telemetry endpoints configured

### Testing
- [ ] Chain spec generated successfully
- [ ] Genesis hash calculated and verified
- [ ] Test launch on single node completed
- [ ] Test launch with 3 validators completed
- [ ] Block production verified
- [ ] Finality verified
- [ ] Governance verified (if applicable)
- [ ] Staking verified

### Documentation
- [ ] Tokenomics published
- [ ] Validator requirements documented
- [ ] Nominator guide published
- [ ] RPC endpoints documented
- [ ] Genesis hash published
- [ ] Chain spec published (non-raw version)
- [ ] Launch timeline published

### Legal & Compliance
- [ ] Legal review of token distribution
- [ ] Compliance review completed
- [ ] Terms of service published
- [ ] Privacy policy published
- [ ] Disclaimers in place

---

## 🎯 Recommended Timeline

### Phase 1: Planning (4-6 weeks)
- Finalize tokenomics
- Legal review
- Community feedback
- Security audit preparations

### Phase 2: Key Generation (2 weeks)
- Generate Foundation keys (HSM)
- Generate validator keys
- Set up multisig
- Test key management procedures

### Phase 3: Genesis Preparation (2 weeks)
- Create genesis preset
- Generate chain spec
- Review and verify all configurations
- Internal testing

### Phase 4: Testnet Validation (4-8 weeks)
- Run Ember testnet
- Stress test
- Security audit
- Bug fixes

### Phase 5: Mainnet Launch (1 week)
- Final configuration freeze
- Deploy infrastructure
- Launch mainnet
- Monitor closely

**Total time: 3-5 months** from decision to launch

---

## 📞 Next Steps

### Immediate (This Week)
1. **Decide on tokenomics** - This drives everything else
2. **Review this document** with team/advisors
3. **Create tokenomics specification** document

### Short Term (Next Month)
1. **Begin key generation planning**
2. **Set up HSM infrastructure**
3. **Start legal/compliance review**
4. **Continue Ember testnet testing**

### Medium Term (2-3 Months)
1. **Generate production keys**
2. **Create mainnet genesis preset**
3. **Security audit**
4. **Internal testing**

### Long Term (3-5 Months)
1. **Final configuration review**
2. **Deploy mainnet infrastructure**
3. **Launch mainnet**
4. **Monitor and support**

---

## 💡 Key Differences: Testnet vs Mainnet

| Aspect | Testnet (Ember) | Mainnet (FlareChain) |
|--------|-----------------|----------------------|
| **Purpose** | Testing, experimentation | Production, real value |
| **Keys** | Test keys (Alice, Bob) | Secure HSM keys |
| **Supply** | Small (2B for testing) | Production economics |
| **Validators** | 3-5 Foundation | 50-100+ (Foundation + community) |
| **Sudo** | Enabled (for convenience) | Disabled or restricted |
| **Faucet** | Yes (free test tokens) | No |
| **Resets** | Can reset/restart | Permanent, immutable |
| **Token Value** | $0 (no real value) | Real market value |
| **Stakes** | Low for testing | High for security |

---

## 📚 Resources

### Substrate Documentation
- [Chain Spec Generation](https://docs.substrate.io/build/chain-spec/)
- [Genesis Configuration](https://docs.substrate.io/build/genesis-configuration/)
- [Key Management](https://docs.substrate.io/deploy/keys-and-network-operations/)

### Security Best Practices
- Use HSM for Foundation validator keys
- Multi-party computation for critical keys
- Geographic distribution of key backups
- Regular security audits

### Tokenomics Examples
- Study other successful L1 launches
- Polkadot, Avalanche, Cosmos tokenomics
- Consider long-term sustainability

---

**Status:** 📋 **PLANNING PHASE - DECISIONS NEEDED**

**Next Action:** Define tokenomics and initial distribution plan

**Questions?** All the ⚠️ **DECISION REQUIRED** sections need your input!
