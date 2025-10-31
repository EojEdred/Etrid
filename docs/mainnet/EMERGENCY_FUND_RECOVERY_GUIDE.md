# EMERGENCY FUND RECOVERY GUIDE

**Version:** 1.0
**Last Updated:** 2025-10-31
**Classification:** CRITICAL - For Foundation Directors Only
**Required Signatures:** 7/9 Directors for Critical Operations

---

## TABLE OF CONTENTS

1. [Overview](#overview)
2. [Emergency Contact Tree](#emergency-contact-tree)
3. [Severity Classification](#severity-classification)
4. [Scenario 1: Stuck Funds in Pallet Account](#scenario-1-stuck-funds-in-pallet-account)
5. [Scenario 2: EDSC Peg Break (>10% Deviation)](#scenario-2-edsc-peg-break-10-deviation)
6. [Scenario 3: Treasury Compromise](#scenario-3-treasury-compromise)
7. [Scenario 4: Validator Payment Failure](#scenario-4-validator-payment-failure)
8. [Scenario 5: Consensus Day Failure](#scenario-5-consensus-day-failure)
9. [Code Examples](#code-examples)
10. [Testing Procedures](#testing-procedures)
11. [Post-Incident Reporting](#post-incident-reporting)

---

## OVERVIEW

This guide provides Standard Operating Procedures (SOPs) for emergency fund recovery scenarios on the Ëtrid blockchain. All procedures are designed to be executed with minimal downtime while maintaining security and decentralization principles.

**Key Principles:**
- **Security First**: All critical operations require 7/9 Director signatures
- **Test Before Execute**: All emergency procedures MUST be simulated on Ember testnet first
- **Transparency**: All emergency actions must be documented and announced to community
- **Reversibility**: Where possible, include rollback procedures
- **Speed vs Safety**: Critical scenarios may require fast action, but never skip multisig requirements

**Related Systems:**
- **Governance Pallet**: `/Users/macbook/Desktop/etrid/10-foundation/governance/pallet/src/lib.rs`
- **Validator Rewards**: `/Users/macbook/Desktop/etrid/src/pallets/pallet-validator-rewards/src/lib.rs`
- **EDSC Token**: `/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/edsc-bridge/substrate-pallets/pallet-edsc-token/src/lib.rs`
- **Circuit Breaker**: `/Users/macbook/Desktop/etrid/src/pallets/pallet-circuit-breaker/src/lib.rs`
- **Multisig System**: `/Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/common/src/multisig.rs`

---

## EMERGENCY CONTACT TREE

### Tier 1: Detection (0-5 minutes)
- **Monitoring Systems**: Alert triggered by Prometheus/Grafana
- **On-Call Engineer**: Assess severity and escalate
- **Primary Contact**: Director on-call (rotates weekly)

### Tier 2: Assessment (5-15 minutes)
- **Lead Director**: Review incident, classify severity
- **Technical Lead**: Verify root cause, estimate impact
- **If CRITICAL**: Activate Tier 3 immediately

### Tier 3: Emergency Response (15+ minutes)
- **All 9 Directors**: Emergency video call within 30 minutes
- **Community Manager**: Prepare public communication
- **Legal Counsel**: Review regulatory implications

### Communication Protocol
```
CRITICAL (Severity 1):
- Post status page update immediately
- Tweet from @EtridProtocol within 15 minutes
- Discord/Telegram announcement within 30 minutes
- Detailed blog post within 2 hours

HIGH (Severity 2):
- Post status page update within 30 minutes
- Social media announcement within 1 hour
- Detailed explanation within 24 hours

MEDIUM (Severity 3):
- Status page update within 2 hours
- Weekly update in community call
```

---

## SEVERITY CLASSIFICATION

### Severity 1: CRITICAL (Red Alert)
**Impact**: Network halt, significant fund loss (>$1M), security breach
**Response Time**: Immediate (0-15 minutes)
**Required Signatures**: 7/9 Directors
**Examples**: Treasury compromise, consensus failure, peg break >20%

### Severity 2: HIGH (Orange Alert)
**Impact**: Degraded service, moderate fund lock (<$1M), performance issues
**Response Time**: <1 hour
**Required Signatures**: 5/9 Directors
**Examples**: Validator payment failure, peg break 10-20%, pallet fund lock

### Severity 3: MEDIUM (Yellow Alert)
**Impact**: Minor service disruption, temporary delays
**Response Time**: <24 hours
**Required Signatures**: 3/9 Directors
**Examples**: Failed proposal execution, minor distribution delays

### Severity 4: LOW (Blue Alert)
**Impact**: Cosmetic issues, minimal impact
**Response Time**: Next scheduled maintenance
**Required Signatures**: Standard governance process

---

## SCENARIO 1: STUCK FUNDS IN PALLET ACCOUNT

### Detection Methods

1. **Automated Monitoring**
```bash
# Query pallet balance
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"system_account", "params":["PALLET_ACCOUNT_ID"]}' \
  http://localhost:9944

# Check if balance exceeds expected threshold
# Alert if pallet_balance > EXPECTED_BALANCE + THRESHOLD
```

2. **User Reports**
- Failed withdrawal transactions
- Transactions stuck in "Pending" state
- Balance discrepancies

3. **Runtime Event Monitoring**
- Missing `Transfer` events
- `TransferFailed` error events
- Pallet account balance anomalies

### Severity Classification

- **CRITICAL (Severity 1)**: >$1M locked, blocking validator rewards or Consensus Day distributions
- **HIGH (Severity 2)**: $100k-$1M locked, affecting subset of users
- **MEDIUM (Severity 3)**: <$100k locked, isolated incidents

### Recovery Procedure

#### Option A: Emergency Withdrawal (Runtime Call)

**Required Signatures**: 7/9 Directors
**Prerequisites**: Multisig transaction prepared and verified

```rust
// Emergency withdrawal extrinsic (requires sudo/governance)
pallet_validator_rewards::emergency_withdraw(
    origin: ensure_root(), // 7/9 multisig
    pallet_id: PalletId,
    amount: Balance,
    recipient: AccountId,
    reason: BoundedVec<u8, ConstU32<256>>, // Must document reason
)
```

**Steps:**
1. **Verify Issue** (5 minutes)
   ```bash
   # Check pallet balance
   ./scripts/query-pallet-balance.sh PALLET_ID

   # Verify expected vs actual
   ./scripts/calculate-expected-balance.sh PALLET_ID
   ```

2. **Prepare Multisig Transaction** (15 minutes)
   ```bash
   # Generate emergency withdrawal call
   cd /Users/macbook/Desktop/etrid

   # Create multisig proposal
   ./scripts/create-multisig-proposal.sh \
     --call "pallet_validator_rewards::emergency_withdraw" \
     --pallet-id "0x70616c6c65742f7265776172647300" \
     --amount "1000000000000000000" \
     --recipient "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
     --reason "Validator rewards stuck in pallet account"
   ```

3. **Collect Director Signatures** (30 minutes)
   ```bash
   # Directors sign multisig transaction
   # Minimum 7/9 required
   for director in ${DIRECTOR_ACCOUNTS[@]}; do
     ./scripts/sign-multisig.sh \
       --proposal-hash "$PROPOSAL_HASH" \
       --signer "$director"
   done
   ```

4. **Submit Transaction** (5 minutes)
   ```bash
   # Execute after 7/9 signatures collected
   ./scripts/execute-multisig.sh --proposal-hash "$PROPOSAL_HASH"
   ```

5. **Verify Execution** (5 minutes)
   ```bash
   # Check transaction success
   ./scripts/check-tx-status.sh --tx-hash "$TX_HASH"

   # Verify funds transferred to recipient
   ./scripts/query-balance.sh "$RECIPIENT_ACCOUNT"
   ```

#### Option B: Runtime Upgrade with Migration

**Required Signatures**: 7/9 Directors + 2-day voting delay
**Prerequisites**: Comprehensive testnet testing

**Use When**: Systematic issue requiring pallet logic changes

```rust
// Migration script in runtime upgrade
pub mod migrations {
    use super::*;

    pub struct MigratePalletFunds<T>(PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigratePalletFunds<T> {
        fn on_runtime_upgrade() -> Weight {
            log::info!("🔧 Migrating stuck funds from pallet account...");

            let pallet_account = PalletId(*b"pal/rewrd").into_account_truncating();
            let treasury = T::TreasuryAccount::get();
            let stuck_balance = T::Currency::free_balance(&pallet_account);

            // Transfer stuck funds to treasury
            match T::Currency::transfer(
                &pallet_account,
                &treasury,
                stuck_balance,
                ExistenceRequirement::AllowDeath,
            ) {
                Ok(_) => {
                    log::info!("✅ Successfully migrated {} to treasury", stuck_balance);
                    // Emit event
                    Pallet::<T>::deposit_event(Event::EmergencyFundsMigrated {
                        from: pallet_account,
                        to: treasury,
                        amount: stuck_balance,
                    });
                }
                Err(e) => {
                    log::error!("❌ Failed to migrate funds: {:?}", e);
                }
            }

            T::DbWeight::get().reads_writes(2, 2)
        }
    }
}
```

**Steps:**
1. **Develop Migration** (2-4 hours)
   - Write migration code
   - Add comprehensive tests
   - Document changes

2. **Test on Ember** (1-2 days)
   ```bash
   # Deploy to Ember testnet
   ./scripts/deploy-testnet-upgrade.sh --runtime-version 101

   # Verify migration successful
   ./scripts/verify-migration.sh
   ```

3. **Create Governance Proposal** (1 day)
   ```bash
   ./scripts/create-governance-proposal.sh \
     --title "Emergency Runtime Upgrade: Pallet Fund Recovery" \
     --description "..." \
     --runtime-wasm "target/release/wbuild/flarechain-runtime/flarechain_runtime.wasm"
   ```

4. **Vote and Execute** (2 days voting + 1 day grace)
   - Directors vote on proposal
   - Community review period
   - Auto-execute if passed

### Prevention Measures

1. **Code Review**: All pallet balance transfers reviewed by 2+ developers
2. **Automated Testing**: Unit tests for all balance transfer paths
3. **Monitoring**: Alert on pallet balance >10% over expected
4. **Audit Schedule**: Quarterly pallet balance audits

---

## SCENARIO 2: EDSC PEG BREAK (>10% DEVIATION)

### Detection Methods

1. **Price Oracle Monitoring**
```rust
// Check EDSC/USD price from oracle
let price = pallet_edsc_oracle::AveragePrices::<T>::get()
    .unwrap_or_default()
    .last()
    .map(|(_, price)| price);

// Calculate deviation from $1.00
let deviation = price.saturating_sub(1_000_000) / 10_000; // basis points

if deviation > 1000 { // >10% deviation
    // Trigger alert
}
```

2. **Circuit Breaker Status**
```bash
# Check circuit breaker status
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method":"circuitBreaker_status"}' \
  http://localhost:9944
```

3. **DEX Price Monitoring**
- Uniswap EDSC/USDC pool
- Real-time price feeds
- 5-minute rolling average

### Severity Classification

| Deviation | Severity | Response Time | Action |
|-----------|----------|---------------|--------|
| 10-15% | HIGH | <1 hour | Emergency pause + reserve injection |
| 15-20% | CRITICAL | <15 minutes | Circuit breaker + director call |
| >20% | CRITICAL | Immediate | Full halt + emergency governance |

### Recovery Procedure

#### Step 1: Emergency Pause (Automatic)

**Trigger**: Circuit breaker activates at >10% deviation

```rust
// Circuit breaker automatically pauses minting/redemptions
impl<T: Config> Pallet<T> {
    fn check_peg_deviation() -> DispatchResult {
        let price = Self::get_current_price()?;
        let deviation = Self::calculate_deviation(price)?;

        if deviation > T::EmergencyThreshold::get() {
            // Auto-pause system
            Status::<T>::put(CircuitStatus::Emergency);
            Self::deposit_event(Event::CircuitTriggered {
                reason: b"EDSC peg deviation >10%".to_vec(),
            });

            // Pause EDSC minting
            pallet_edsc_token::MintingPaused::<T>::put(true);

            // Throttle redemptions
            pallet_edsc_redemption::RedemptionsThrottled::<T>::put(true);
        }

        Ok(())
    }
}
```

**Automatic Actions:**
- EDSC minting paused
- Redemptions throttled (1% hourly cap)
- Alert sent to all Directors
- Public status page updated

#### Step 2: Director Assessment (0-30 minutes)

**Emergency Call Agenda:**
1. Root cause analysis (oracle failure, market panic, reserve depletion?)
2. Current reserve ratio assessment
3. Available liquidity review
4. Communication strategy approval

#### Step 3: Reserve Injection (30-60 minutes)

**Required Signatures**: 7/9 Directors
**Goal**: Restore reserve ratio >100%

```rust
// Emergency reserve injection
pallet_multiasset_reserve::inject_reserves(
    origin: ensure_root(), // 7/9 multisig
    asset: ReserveAsset::USDC,
    amount: Balance, // Amount to inject
    reason: BoundedVec<u8, ConstU32<256>>,
)
```

**Options:**
1. **Inject USDC from Treasury** ($500k - $5M available)
2. **Inject BTC from Reserve** (Convert from BTC reserve)
3. **Emergency Loan from Partners** (Aave, Compound integration)

**Steps:**
```bash
# 1. Check current reserve ratio
./scripts/check-reserve-ratio.sh

# 2. Calculate required injection
./scripts/calculate-reserve-injection.sh --target-ratio 110

# 3. Create multisig transaction
./scripts/create-multisig-proposal.sh \
  --call "pallet_multiasset_reserve::inject_reserves" \
  --asset "USDC" \
  --amount "5000000000000" \
  --reason "Emergency peg stabilization"

# 4. Collect signatures (7/9 Directors)
./scripts/collect-multisig-signatures.sh --proposal-id $PROPOSAL_ID

# 5. Execute injection
./scripts/execute-multisig.sh --proposal-id $PROPOSAL_ID
```

#### Step 4: Interest Rate Adjustment (1-2 hours)

**Goal**: Incentivize EDSC burning (redemptions) or minting

**If EDSC > $1.05 (oversupply):**
- Increase redemption APY to 5-10%
- Decrease minting incentives
- Encourage arbitrageurs to redeem

**If EDSC < $0.95 (undersupply):**
- Increase minting incentives
- Offer EDSC purchase at discount
- Encourage arbitrageurs to mint

```rust
// Adjust interest rates
pallet_edsc_redemption::set_interest_rate(
    origin: ensure_root(), // 5/9 multisig
    new_rate: Permill, // Basis points
    duration: BlockNumberFor<T>, // How long to apply
)
```

#### Step 5: Liquidity Provider Communication (2-4 hours)

**Contact:**
- Uniswap LP providers
- CEX partners (if listed)
- Market makers
- Arbitrage bots

**Request:**
- Temporary halt on large trades
- Coordinate liquidity provision
- Share market intelligence

**Template Message:**
```
Subject: URGENT: EDSC Peg Stabilization Request

Dear Liquidity Provider,

The EDSC stablecoin has experienced a >10% peg deviation due to [REASON].

Current Status:
- Price: $[PRICE]
- Reserve Ratio: [RATIO]%
- Circuit Breaker: [STATUS]

Actions Taken:
- Emergency pause activated
- Reserve injection: $[AMOUNT]
- Interest rate adjusted to [RATE]%

Request:
- Pause large trades (>$100k) for 24 hours
- Assist with liquidity provision if possible
- Report any suspicious activity

Timeline:
- Assessment: Complete
- Stabilization: 2-4 hours
- Full restoration: 24-48 hours

Thank you for your cooperation.

Ëtrid Foundation Directors
```

#### Step 6: Resume Operations (4-24 hours)

**Requirements Before Unpause:**
- [ ] Reserve ratio >105%
- [ ] Price deviation <5% for 6 hours
- [ ] Director approval (7/9)
- [ ] Public communication complete

```bash
# Verify conditions met
./scripts/verify-unpause-conditions.sh

# Create unpause proposal
./scripts/create-multisig-proposal.sh \
  --call "pallet_circuit_breaker::unpause" \
  --reason "Peg restored, reserves stabilized"

# Execute unpause
./scripts/execute-multisig.sh --proposal-id $PROPOSAL_ID
```

### Prevention Measures

1. **Diversified Reserves**: 60% USDC, 20% BTC, 10% ETH, 10% Treasury bonds
2. **Reserve Buffer**: Maintain 110-120% reserve ratio (>100% required)
3. **Automated Rebalancing**: Daily reserve rebalancing
4. **Circuit Breakers**: Auto-pause at 10% deviation
5. **Liquidity Monitoring**: Real-time DEX pool monitoring
6. **Oracle Redundancy**: 5+ price sources (Chainlink, Band, Pyth, DEXs)

---

## SCENARIO 3: TREASURY COMPROMISE

### Detection Methods

1. **Unauthorized Transaction Alert**
```bash
# Monitor treasury account for unexpected transactions
./scripts/monitor-treasury.sh --alert-on-unexpected

# Check multisig threshold signatures
./scripts/verify-multisig-signatures.sh --account $TREASURY_ACCOUNT
```

2. **Suspicious Activity Patterns**
- Transaction not matching approved proposals
- Signatures from compromised keys
- Large transfers to unknown addresses

3. **Director Reports**
- Director reports private key compromise
- Phishing attempt detected
- Suspicious multisig approval requests

### Severity Classification

**CRITICAL (Severity 1)**: Always
**Impact**: Entire treasury at risk ($10M+)
**Response Time**: Immediate (0-5 minutes)

### Recovery Procedure

#### Step 1: Emergency Freeze (0-5 minutes)

**IMMEDIATE ACTION**: Any single Director can initiate freeze

```rust
// Emergency treasury freeze (any Director can call)
pallet_treasury::emergency_freeze(
    origin: ensure_signed(director), // Any Director
    reason: BoundedVec<u8, ConstU32<256>>,
)

// Freezes all treasury operations immediately
// Requires 7/9 to unfreeze
```

**Steps:**
```bash
# Any Director can execute immediately
./scripts/emergency-freeze-treasury.sh \
  --signer $DIRECTOR_ACCOUNT \
  --reason "Suspected treasury compromise detected"

# Notifies all Directors via SMS/Email/Discord
```

**Effects:**
- All treasury transfers blocked
- Governance proposal execution paused
- Validator rewards temporarily delayed
- Emergency call activated

#### Step 2: Investigation (5-30 minutes)

**Emergency Call (All Directors + Security Team):**
1. **Identify Compromise Vector**
   - Which Director key(s) compromised?
   - How was compromise detected?
   - Scope of potential damage?

2. **Assess Immediate Risk**
   - Have any unauthorized transactions executed?
   - Are additional keys at risk?
   - Is attacker still active?

3. **Containment Decision**
   - Rotate all Director keys immediately? (YES if >2 keys compromised)
   - Create new treasury account? (YES if treasury key compromised)
   - Law enforcement involvement? (Depends on jurisdiction)

#### Step 3: Key Rotation (30-60 minutes)

**Required If**: ≥2 Director keys compromised

```bash
# Generate new Director keys (offline, air-gapped machine)
for i in {1..9}; do
  ./scripts/generate-director-key.sh --director-id $i --output-dir /secure/keys/
done

# Create new multisig account (7/9 threshold)
./scripts/create-multisig.sh \
  --threshold 7 \
  --signers ${NEW_DIRECTOR_KEYS[@]} \
  --output-address NEW_MULTISIG_ADDRESS

# Fund new multisig with minimal amount for testing
./scripts/test-multisig.sh --address $NEW_MULTISIG_ADDRESS
```

#### Step 4: Treasury Migration (1-2 hours)

**Required Signatures**: 7/9 Directors (using NEW keys)

```rust
// Migrate treasury to new multisig
pallet_treasury::migrate_treasury(
    origin: ensure_root(), // 7/9 OLD keys (if not compromised) or governance
    new_treasury: AccountId,
    reason: BoundedVec<u8, ConstU32<256>>,
)
```

**Steps:**
```bash
# 1. Create migration proposal
./scripts/create-governance-proposal.sh \
  --title "EMERGENCY: Treasury Migration Due to Security Incident" \
  --description "Migrate treasury to new multisig address due to key compromise" \
  --call "pallet_treasury::migrate_treasury" \
  --new-treasury $NEW_MULTISIG_ADDRESS

# 2. Fast-track approval (requires 7/9 Directors)
# Skip normal voting period for emergency
./scripts/emergency-approve-proposal.sh \
  --proposal-id $PROPOSAL_ID \
  --signers ${DIRECTOR_ACCOUNTS[@]}

# 3. Execute migration
./scripts/execute-proposal.sh --proposal-id $PROPOSAL_ID

# 4. Verify migration
./scripts/verify-treasury-migration.sh \
  --old-address $OLD_TREASURY \
  --new-address $NEW_MULTISIG_ADDRESS
```

#### Step 5: Forensics and Recovery (2-7 days)

1. **Forensic Analysis**
   - Analyze blockchain transactions
   - Identify stolen funds
   - Track attacker addresses
   - Document attack vector

2. **Fund Recovery Attempts**
   - Contact exchanges (freeze attacker accounts)
   - Law enforcement coordination
   - Community bounty for information
   - Possible governance vote to slash attacker

3. **Security Audit**
   - Review all key management procedures
   - Audit code for vulnerabilities
   - Third-party security assessment
   - Update security policies

#### Step 6: Communication and Transparency (Ongoing)

**Immediate (0-1 hour):**
```
URGENT: Treasury Security Incident

The Ëtrid Foundation has detected suspicious activity on the Treasury account.

Actions Taken:
- Treasury frozen immediately (no funds lost)
- Emergency Director call in progress
- Investigation underway

Your Funds Are Safe:
- User accounts not affected
- Network operating normally
- No validator rewards impacted

Updates every 2 hours on status.etrid.io

- Ëtrid Foundation Directors
```

**Detailed (2-4 hours):**
- Full incident timeline
- Root cause explanation
- Mitigation steps taken
- Prevention measures implemented

**Post-Mortem (7 days):**
- Comprehensive incident report
- Lessons learned
- Policy changes
- Compensation plan (if applicable)

### Prevention Measures

1. **Key Security**
   - Hardware wallet for all Director keys
   - Multi-factor authentication
   - Regular key rotation (quarterly)
   - No keys stored on internet-connected devices

2. **Multisig Protection**
   - 7/9 threshold (no single point of failure)
   - Geographically distributed Directors
   - Time-locked large transfers (>$1M requires 24hr delay)

3. **Monitoring**
   - 24/7 treasury account monitoring
   - Alert on any unexpected transactions
   - Weekly multisig health checks

4. **Incident Response**
   - Quarterly incident response drills
   - Emergency contact tree tested monthly
   - Hot backup multisig ready (4/6 for emergencies)

---

## SCENARIO 4: VALIDATOR PAYMENT FAILURE

### Detection Methods

1. **Missed Payment Events**
```rust
// Check if epoch ended without reward distribution
if current_epoch > last_payment_epoch + 1 {
    // Payment missed
    emit Event::ValidatorPaymentMissed {
        epoch: last_payment_epoch + 1,
        affected_validators: validator_count,
    };
}
```

2. **Validator Reports**
- Validators report missing rewards
- Payment account balance unchanged
- Reward pool not depleted

3. **Automated Monitoring**
```bash
# Check last payment epoch
./scripts/check-last-payment-epoch.sh

# Verify all validators received payment
./scripts/verify-validator-payments.sh --epoch $EPOCH
```

### Severity Classification

- **HIGH (Severity 2)**: Single epoch missed, <21 validators affected
- **CRITICAL (Severity 1)**: Multiple epochs missed, OR >50% validators affected

### Recovery Procedure

#### Step 1: Identify Root Cause (5-15 minutes)

**Possible Causes:**
1. **Pallet Account Empty**: Reward pool not funded
2. **Extrinsic Failed**: Transaction reverted
3. **Payment Account Mapping Issue**: Session → Stash mapping broken
4. **Insufficient Gas**: Transaction ran out of weight
5. **Runtime Bug**: Logic error in distribution code

```bash
# Check reward pool balance
./scripts/query-reward-pool.sh

# Check last distribution transaction
./scripts/query-last-distribution-tx.sh

# Review runtime logs
./scripts/review-validator-logs.sh --from $START_BLOCK --to $END_BLOCK
```

#### Step 2: Manual Reward Distribution (15-60 minutes)

**Required Signatures**: 5/9 Directors
**Prerequisites**: Calculate correct reward amounts

```bash
# Calculate missed rewards for epoch
./scripts/calculate-epoch-rewards.sh --epoch $MISSED_EPOCH > rewards.json

# Output format:
# {
#   "epoch": 142,
#   "total_pool": "1000000000000000000",
#   "validators": [
#     {
#       "session_account": "5GrwvaEF...",
#       "payment_account": "5FHneW46...",
#       "stake": "100000000000000000",
#       "reward": "45454545454545454",
#       "performance_multiplier": 105 // 105% due to high uptime
#     },
#     ...
#   ]
# }

# Create batch transfer transaction
./scripts/create-batch-transfer.sh --input rewards.json > batch_tx.json

# Directors sign batch transaction (5/9 required)
./scripts/create-multisig-proposal.sh \
  --call "utility::batch" \
  --calls-file batch_tx.json \
  --reason "Manual distribution for epoch $MISSED_EPOCH"

# Collect signatures
./scripts/collect-multisig-signatures.sh --proposal-id $PROPOSAL_ID

# Execute batch transfer
./scripts/execute-multisig.sh --proposal-id $PROPOSAL_ID
```

**Batch Transfer Structure:**
```rust
// Batch transfer to all validators
utility::batch(
    calls: vec![
        pallet_balances::transfer {
            dest: validator_1_payment_account,
            value: reward_1,
        },
        pallet_balances::transfer {
            dest: validator_2_payment_account,
            value: reward_2,
        },
        // ... all validators
    ]
)
```

#### Step 3: Direct Treasury Transfers (If Batch Fails)

**Fallback Option**: Individual transfers from treasury

```bash
# For each validator
for validator in $(jq -r '.validators[].payment_account' rewards.json); do
  reward=$(jq -r ".validators[] | select(.payment_account==\"$validator\") | .reward" rewards.json)

  # Create individual transfer
  ./scripts/create-multisig-proposal.sh \
    --call "pallet_balances::transfer" \
    --dest $validator \
    --amount $reward \
    --reason "Epoch $MISSED_EPOCH reward"

  # Directors sign (5/9)
  ./scripts/collect-multisig-signatures.sh --proposal-id $PROPOSAL_ID

  # Execute
  ./scripts/execute-multisig.sh --proposal-id $PROPOSAL_ID
done
```

#### Step 4: Compensation for Missed Epochs (If Multiple Epochs)

**Policy**: +5% bonus for each missed epoch as apology

```bash
# Calculate compensation
./scripts/calculate-compensation.sh \
  --missed-epochs "142,143,144" \
  --bonus-percent 5 > compensation.json

# Create compensation proposal
./scripts/create-governance-proposal.sh \
  --title "Validator Compensation for Missed Payments (Epochs 142-144)" \
  --description "Compensate validators with +5% bonus per missed epoch" \
  --calls-file compensation.json
```

#### Step 5: Fix Underlying Issue

**Option A: Runtime Upgrade (If Bug)**
```rust
// Fix payment logic in runtime upgrade
pub mod v2 {
    use super::*;

    pub fn migrate_fix_payment_logic<T: Config>() -> Weight {
        // Fix payment account mapping
        // Fix reward calculation
        // Add safeguards
    }
}
```

**Option B: Parameter Adjustment**
```bash
# If gas limit issue, increase weight limit
./scripts/create-governance-proposal.sh \
  --call "pallet_validator_rewards::set_max_distribution_weight" \
  --weight 5000000000
```

**Option C: Funding Reward Pool**
```bash
# If reward pool empty, fund from treasury
./scripts/create-multisig-proposal.sh \
  --call "pallet_balances::transfer" \
  --dest "REWARD_POOL_ACCOUNT" \
  --amount "10000000000000000000" \
  --reason "Fund reward pool for next 10 epochs"
```

### Prevention Measures

1. **Automated Monitoring**
   - Alert if epoch ends without payment event
   - Check reward pool balance weekly
   - Verify all validators received payment

2. **Redundancy**
   - Manual backup script for distributions
   - Emergency reward pool (3 months reserve)
   - Payment account mapping backup

3. **Testing**
   - Test distribution logic every runtime upgrade
   - Simulate payment failures on testnet
   - Validate reward calculations

---

## SCENARIO 5: CONSENSUS DAY FAILURE

### Detection Methods

1. **Minting Event Missing**
```rust
// Check if Consensus Day passed without minting
if current_date == consensus_day && !minting_executed {
    emit Event::ConsensusDayMintingFailed {
        date: current_date,
        reason: "Minting automation failed",
    };
}
```

2. **Distribution Not Triggered**
```bash
# Check if distribution executed after Consensus Day
./scripts/check-consensus-day-status.sh --date "2025-12-01"

# Expected: Minting completed, Distribution scheduled
```

3. **Proposal Vote Not Finalized**
```bash
# Check if votes tallied correctly
./scripts/check-proposal-votes.sh --proposal-id $PROPOSAL_ID
```

### Severity Classification

- **CRITICAL (Severity 1)**: Minting failed, no distribution
- **HIGH (Severity 2)**: Minting succeeded, distribution delayed
- **MEDIUM (Severity 3)**: Distribution delayed <24 hours

### Recovery Procedure

#### Step 1: Assess Failure Point (5-15 minutes)

**Failure Points:**
1. **Minting Logic Failed**: Proposal passed but minting didn't execute
2. **Distribution Failed**: Minted tokens not distributed
3. **Vote Tallying Error**: Incorrect vote counts
4. **Automation Failure**: Scheduled task didn't run

```bash
# Check minting status
./scripts/check-minting-status.sh --consensus-day "2025-12-01"

# Check if tokens minted
./scripts/check-total-supply.sh --before-and-after

# Check distribution schedule
./scripts/check-distribution-schedule.sh
```

#### Step 2: Manual Minting (If Automation Failed)

**Required Signatures**: 7/9 Directors
**Prerequisites**: Consensus Day vote passed with majority

```rust
// Manual minting trigger
pallet_consensus_day_minting::emergency_mint(
    origin: ensure_root(), // 7/9 multisig
    proposal_id: ProposalId,
    mint_amount: Balance, // As approved in vote
    reason: BoundedVec<u8, ConstU32<256>>,
)
```

**Steps:**
```bash
# 1. Verify proposal passed
./scripts/verify-proposal-passed.sh --proposal-id $PROPOSAL_ID

# 2. Calculate mint amount (from proposal)
MINT_AMOUNT=$(./scripts/get-proposal-mint-amount.sh --proposal-id $PROPOSAL_ID)

# 3. Create emergency mint proposal
./scripts/create-multisig-proposal.sh \
  --call "pallet_consensus_day_minting::emergency_mint" \
  --proposal-id $PROPOSAL_ID \
  --mint-amount $MINT_AMOUNT \
  --reason "Manual execution after automation failure"

# 4. Collect Director signatures (7/9)
./scripts/collect-multisig-signatures.sh --proposal-id $MULTISIG_PROPOSAL_ID

# 5. Execute minting
./scripts/execute-multisig.sh --proposal-id $MULTISIG_PROPOSAL_ID

# 6. Verify tokens minted
./scripts/verify-mint-executed.sh --expected-amount $MINT_AMOUNT
```

#### Step 3: Delayed Distribution Handling (If Distribution Delayed)

**Scenario**: Tokens minted but distribution didn't execute on schedule

```bash
# Check distribution queue
./scripts/check-distribution-queue.sh

# If distribution scheduled but not executed
# Trigger manual distribution
./scripts/trigger-distribution.sh --distribution-id $DIST_ID

# If not scheduled, create manual distribution
./scripts/create-manual-distribution.sh \
  --mint-id $MINT_ID \
  --foundation-share 40 \
  --directors-share 20 \
  --validators-share 30 \
  --voters-share 10
```

**Distribution Breakdown:**
```bash
# Calculate distribution amounts
TOTAL_MINTED=$MINT_AMOUNT

FOUNDATION_AMOUNT=$(echo "$TOTAL_MINTED * 0.40" | bc)
DIRECTORS_AMOUNT=$(echo "$TOTAL_MINTED * 0.20" | bc)
VALIDATORS_AMOUNT=$(echo "$TOTAL_MINTED * 0.30" | bc)
VOTERS_AMOUNT=$(echo "$TOTAL_MINTED * 0.10" | bc)

# Create distribution transactions
./scripts/create-batch-distribution.sh \
  --foundation-amount $FOUNDATION_AMOUNT \
  --directors-amount $DIRECTORS_AMOUNT \
  --validators-amount $VALIDATORS_AMOUNT \
  --voters-amount $VOTERS_AMOUNT \
  --output distribution_batch.json

# Execute via multisig (5/9 Directors)
./scripts/create-multisig-proposal.sh \
  --call "utility::batch" \
  --calls-file distribution_batch.json

./scripts/execute-multisig.sh --proposal-id $PROPOSAL_ID
```

#### Step 4: Rollback Procedures (If Incorrect Execution)

**CRITICAL**: Only if tokens minted incorrectly (wrong amount, wrong recipients)

**Required Signatures**: 8/9 Directors (super-majority for rollback)

```rust
// Rollback incorrect minting (DANGEROUS - use only if absolutely necessary)
pallet_consensus_day_minting::emergency_rollback(
    origin: ensure_root(), // 8/9 multisig (super-majority)
    mint_id: u64,
    reason: BoundedVec<u8, ConstU32<512>>, // Detailed justification required
)

// Burns incorrectly minted tokens
// Restores previous state
// Logs rollback event for transparency
```

**Steps:**
```bash
# 1. Verify incorrect minting
./scripts/verify-minting-error.sh --mint-id $MINT_ID

# 2. Document error thoroughly
./scripts/generate-rollback-report.sh --mint-id $MINT_ID > rollback_report.md

# 3. Create rollback proposal (requires 8/9 Directors - super-majority)
./scripts/create-multisig-proposal.sh \
  --call "pallet_consensus_day_minting::emergency_rollback" \
  --mint-id $MINT_ID \
  --reason-file rollback_report.md

# 4. Collect signatures (8/9)
./scripts/collect-multisig-signatures.sh --proposal-id $PROPOSAL_ID --required 8

# 5. Execute rollback
./scripts/execute-multisig.sh --proposal-id $PROPOSAL_ID

# 6. Re-execute minting correctly
./scripts/execute-correct-minting.sh --proposal-id $ORIGINAL_PROPOSAL_ID
```

**Rollback Communication:**
```
URGENT: Consensus Day Minting Rollback

The Ëtrid Foundation has executed an emergency rollback of Consensus Day minting
due to [SPECIFIC ERROR].

Incorrect Action:
- Mint ID: #142
- Amount: [INCORRECT_AMOUNT]
- Issue: [DESCRIPTION]

Corrective Action:
- Rollback executed at block #[BLOCK]
- Correct minting re-executed at block #[BLOCK]
- All participants will receive correct amounts

No user action required.

Full incident report: [URL]
```

#### Step 5: Post-Mortem and Prevention

1. **Incident Review** (within 48 hours)
   - Root cause analysis
   - Timeline of events
   - Response effectiveness

2. **Automation Improvements**
   - Add redundant minting triggers
   - Implement pre-flight checks
   - Add manual override capabilities

3. **Testing Enhancements**
   - Annual Consensus Day simulation on testnet
   - Failure mode testing
   - Rollback procedure testing

### Prevention Measures

1. **Redundant Automation**
   - Primary: Automatic minting on Consensus Day
   - Secondary: Backup cron job (4 hours later)
   - Tertiary: Manual monitoring by Directors

2. **Pre-Flight Checks**
   ```rust
   // Before Consensus Day minting
   fn pre_mint_checks() -> Result<(), Error> {
       // Verify proposal passed
       ensure!(proposal.status == ProposalStatus::Passed, Error::ProposalNotPassed);

       // Verify sufficient votes
       ensure!(proposal.yes_votes > proposal.no_votes, Error::InsufficientVotes);

       // Verify treasury has capacity
       ensure!(treasury_balance > MIN_BALANCE, Error::InsufficientTreasury);

       // Verify distribution accounts exist
       ensure!(foundation_account_exists, Error::DistributionAccountMissing);

       Ok(())
   }
   ```

3. **Monitoring Alerts**
   - Alert 24 hours before Consensus Day
   - Alert if minting doesn't execute within 1 hour of Consensus Day
   - Alert if distribution doesn't start within 4 hours

4. **Annual Dry Run**
   - Simulate Consensus Day on testnet annually
   - Test all failure scenarios
   - Verify Director response time

---

## CODE EXAMPLES

### Emergency Withdrawal Extrinsic

```rust
//! Emergency withdrawal function for pallet account recovery
//! Location: /Users/macbook/Desktop/etrid/src/pallets/pallet-validator-rewards/src/lib.rs

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Emergency withdrawal from pallet account
    ///
    /// **Security**: Requires root origin (7/9 multisig)
    /// **Use Case**: Recover stuck funds from pallet account
    ///
    /// # Parameters
    /// - `origin`: Must be root (7/9 Director multisig)
    /// - `amount`: Amount to withdraw
    /// - `recipient`: Destination account (typically treasury)
    /// - `reason`: Justification for withdrawal (stored on-chain)
    #[pallet::call_index(5)]
    #[pallet::weight(T::DbWeight::get().reads_writes(3, 3))]
    pub fn emergency_withdraw(
        origin: OriginFor<T>,
        amount: BalanceOf<T>,
        recipient: T::AccountId,
        reason: BoundedVec<u8, ConstU32<256>>,
    ) -> DispatchResult {
        // Verify root origin (7/9 multisig)
        ensure_root(origin)?;

        // Get pallet account
        let pallet_account: T::AccountId = T::PalletId::get().into_account_truncating();

        // Verify sufficient balance
        let pallet_balance = T::Currency::free_balance(&pallet_account);
        ensure!(pallet_balance >= amount, Error::<T>::InsufficientBalance);

        // Transfer funds to recipient
        T::Currency::transfer(
            &pallet_account,
            &recipient,
            amount,
            ExistenceRequirement::AllowDeath,
        )?;

        // Emit event
        Self::deposit_event(Event::EmergencyWithdrawal {
            from: pallet_account,
            to: recipient,
            amount,
            reason: reason.to_vec(),
        });

        Ok(())
    }
}

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Emergency withdrawal executed [from, to, amount, reason]
    EmergencyWithdrawal {
        from: T::AccountId,
        to: T::AccountId,
        amount: BalanceOf<T>,
        reason: Vec<u8>,
    },
}
```

### Emergency Treasury Freeze

```rust
//! Emergency treasury freeze function
//! Location: /Users/macbook/Desktop/etrid/10-foundation/governance/pallet/src/lib.rs

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Emergency freeze of treasury account
    ///
    /// **Security**: Any single Director can call to freeze
    /// **Security**: Requires 7/9 Directors to unfreeze
    /// **Use Case**: Respond immediately to treasury compromise
    ///
    /// # Parameters
    /// - `origin`: Must be signed by a Director
    /// - `reason`: Justification for freeze
    #[pallet::call_index(10)]
    #[pallet::weight(T::DbWeight::get().reads_writes(2, 2))]
    pub fn emergency_freeze_treasury(
        origin: OriginFor<T>,
        reason: BoundedVec<u8, ConstU32<256>>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Verify caller is a Director
        ensure!(
            Self::is_director(&who),
            Error::<T>::NotAuthorized
        );

        // Set treasury frozen flag
        TreasuryFrozen::<T>::put(true);

        // Record who froze it
        TreasuryFrozenBy::<T>::put(who.clone());

        // Emit event
        Self::deposit_event(Event::TreasuryEmergencyFrozen {
            by: who,
            reason: reason.to_vec(),
            timestamp: Self::current_timestamp(),
        });

        // Send emergency alerts to all Directors
        Self::send_emergency_alert(AlertType::TreasuryFrozen);

        Ok(())
    }

    /// Unfreeze treasury after emergency resolved
    ///
    /// **Security**: Requires 7/9 Director multisig
    ///
    /// # Parameters
    /// - `origin`: Must be root (7/9 Director multisig)
    #[pallet::call_index(11)]
    #[pallet::weight(T::DbWeight::get().reads_writes(2, 2))]
    pub fn unfreeze_treasury(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        ensure_root(origin)?;

        // Remove freeze flag
        TreasuryFrozen::<T>::kill();
        TreasuryFrozenBy::<T>::kill();

        // Emit event
        Self::deposit_event(Event::TreasuryUnfrozen {
            timestamp: Self::current_timestamp(),
        });

        Ok(())
    }
}

#[pallet::storage]
#[pallet::getter(fn treasury_frozen)]
pub type TreasuryFrozen<T: Config> = StorageValue<_, bool, ValueQuery>;

#[pallet::storage]
#[pallet::getter(fn treasury_frozen_by)]
pub type TreasuryFrozenBy<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

/// Hook to prevent treasury operations when frozen
impl<T: Config> Pallet<T> {
    fn ensure_treasury_not_frozen() -> DispatchResult {
        ensure!(
            !TreasuryFrozen::<T>::get(),
            Error::<T>::TreasuryFrozen
        );
        Ok(())
    }
}
```

### EDSC Emergency Pause

```rust
//! EDSC emergency pause mechanism
//! Location: /Users/macbook/Desktop/etrid/05-multichain/bridge-protocols/edsc-bridge/substrate-pallets/pallet-edsc-token/src/lib.rs

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Emergency pause of EDSC minting
    ///
    /// **Automatic**: Circuit breaker can call this
    /// **Manual**: 5/9 Directors can call this
    /// **Use Case**: Respond to peg deviation or security incident
    ///
    /// # Parameters
    /// - `origin`: Root or circuit breaker
    #[pallet::call_index(8)]
    #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
    pub fn pause_minting(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        // Allow root (5/9 Directors) OR circuit breaker to pause
        T::CircuitBreakerOrigin::ensure_origin(origin.clone())
            .or_else(|_| ensure_root(origin))?;

        // Set pause flag
        MintingPaused::<T>::put(true);

        // Emit event
        Self::deposit_event(Event::MintingPaused);

        Ok(())
    }

    /// Emergency pause of EDSC burning
    ///
    /// **Use Case**: Prevent mass redemptions during crisis
    #[pallet::call_index(9)]
    #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
    pub fn pause_burning(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        T::CircuitBreakerOrigin::ensure_origin(origin.clone())
            .or_else(|_| ensure_root(origin))?;

        BurningPaused::<T>::put(true);

        Self::deposit_event(Event::BurningPaused);

        Ok(())
    }

    /// Unpause minting after emergency resolved
    #[pallet::call_index(10)]
    #[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
    pub fn unpause_minting(
        origin: OriginFor<T>,
    ) -> DispatchResult {
        ensure_root(origin)?; // Requires 7/9 Directors

        MintingPaused::<T>::put(false);

        Self::deposit_event(Event::MintingUnpaused);

        Ok(())
    }
}

/// Verify minting not paused before allowing mint
fn ensure_minting_not_paused() -> DispatchResult {
    ensure!(
        !MintingPaused::<T>::get(),
        Error::<T>::MintingPaused
    );
    Ok(())
}
```

### Manual Validator Reward Distribution

```rust
//! Manual reward distribution for missed epochs
//! Location: /Users/macbook/Desktop/etrid/src/pallets/pallet-validator-rewards/src/lib.rs

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Manually distribute rewards for a specific epoch
    ///
    /// **Use Case**: Recover from automated distribution failure
    /// **Security**: Requires 5/9 Director multisig
    ///
    /// # Parameters
    /// - `origin`: Root (5/9 Directors)
    /// - `epoch`: Epoch number to distribute rewards for
    /// - `rewards`: List of (validator, amount) pairs
    #[pallet::call_index(6)]
    #[pallet::weight(T::DbWeight::get().reads_writes(
        rewards.len() as u64 * 2,
        rewards.len() as u64 * 2
    ))]
    pub fn manual_distribute_rewards(
        origin: OriginFor<T>,
        epoch: u32,
        rewards: Vec<(T::AccountId, BalanceOf<T>)>,
    ) -> DispatchResult {
        ensure_root(origin)?;

        // Verify epoch not already distributed
        ensure!(
            !EpochDistributed::<T>::contains_key(epoch),
            Error::<T>::EpochAlreadyDistributed
        );

        let mut total_distributed = BalanceOf::<T>::zero();

        // Distribute rewards to each validator
        for (validator, amount) in rewards.iter() {
            // Get payment account for validator
            let payment_account = PaymentAccounts::<T>::get(validator)
                .ok_or(Error::<T>::PaymentAccountNotFound)?;

            // Transfer reward
            T::Currency::transfer(
                &Self::reward_pool_account(),
                &payment_account,
                *amount,
                ExistenceRequirement::KeepAlive,
            )?;

            total_distributed = total_distributed.saturating_add(*amount);

            // Emit event
            Self::deposit_event(Event::RewardPaid {
                validator: validator.clone(),
                payment_account,
                epoch,
                amount: *amount,
            });
        }

        // Mark epoch as distributed
        EpochDistributed::<T>::insert(epoch, true);

        // Emit summary event
        Self::deposit_event(Event::ManualDistributionCompleted {
            epoch,
            validator_count: rewards.len() as u32,
            total_amount: total_distributed,
        });

        Ok(())
    }
}

#[pallet::storage]
#[pallet::getter(fn epoch_distributed)]
pub type EpochDistributed<T: Config> = StorageMap<_, Twox64Concat, u32, bool, ValueQuery>;
```

### Multisig Transaction Template

```rust
//! Multisig transaction creation utility
//! Location: /Users/macbook/Desktop/etrid/scripts/lib/multisig.rs

use codec::Encode;
use sp_runtime::MultiSignature;

/// Create multisig proposal for emergency action
///
/// # Parameters
/// - `call`: The extrinsic to execute
/// - `threshold`: Number of approvals required (typically 7/9)
/// - `directors`: List of Director accounts
pub fn create_multisig_proposal<T: Config>(
    call: Call<T>,
    threshold: u16,
    directors: Vec<T::AccountId>,
) -> Result<ProposalHash, &'static str> {
    // Validate threshold
    if threshold == 0 || threshold as usize > directors.len() {
        return Err("Invalid threshold");
    }

    // Encode call data
    let call_data = call.encode();

    // Hash call for unique identification
    let call_hash = blake2_256(&call_data);

    // Create multisig account
    let multisig_account = Multisig::<T>::multi_account_id(&directors, threshold);

    // Store pending proposal
    let proposal = PendingProposal {
        call_hash,
        call_data,
        approvals: vec![],
        required_approvals: threshold,
        executed: false,
        created_at: <frame_system::Pallet<T>>::block_number(),
    };

    PendingProposals::<T>::insert(call_hash, proposal);

    Ok(call_hash)
}

/// Approve multisig proposal
pub fn approve_multisig_proposal<T: Config>(
    proposal_hash: ProposalHash,
    approver: T::AccountId,
    signature: MultiSignature,
) -> DispatchResult {
    // Get proposal
    let mut proposal = PendingProposals::<T>::get(proposal_hash)
        .ok_or(Error::<T>::ProposalNotFound)?;

    // Verify approver is a Director
    ensure!(
        Directors::<T>::contains_key(&approver),
        Error::<T>::NotDirector
    );

    // Verify not already approved
    ensure!(
        !proposal.approvals.contains(&approver),
        Error::<T>::AlreadyApproved
    );

    // Verify signature
    ensure!(
        verify_signature(&proposal.call_hash, &approver, &signature),
        Error::<T>::InvalidSignature
    );

    // Add approval
    proposal.approvals.push(approver.clone());

    // Check if threshold reached
    if proposal.approvals.len() >= proposal.required_approvals as usize {
        // Execute proposal
        Self::execute_multisig_proposal(&proposal)?;
        proposal.executed = true;
    }

    // Update storage
    PendingProposals::<T>::insert(proposal_hash, proposal);

    Ok(())
}
```

---

## TESTING PROCEDURES

### Test Emergency Procedures on Ember Testnet

**Frequency**: Quarterly (every 3 months)
**Duration**: 4-6 hours
**Participants**: All 9 Directors + Technical Team

### Test Checklist

#### 1. Treasury Emergency Freeze
```bash
# 1. Setup testnet environment
./scripts/setup-ember-testnet.sh

# 2. Fund testnet treasury
./scripts/fund-testnet-treasury.sh --amount 1000000

# 3. Test freeze by single Director
./scripts/test-emergency-freeze.sh --director alice

# 4. Verify all treasury operations blocked
./scripts/verify-treasury-frozen.sh

# 5. Test unfreeze with 7/9 multisig
./scripts/test-emergency-unfreeze.sh --directors ${DIRECTORS[@]}

# 6. Verify treasury operations resumed
./scripts/verify-treasury-operational.sh
```

#### 2. Stuck Funds Recovery
```bash
# 1. Artificially create stuck funds scenario
./scripts/create-stuck-funds.sh --pallet validator_rewards --amount 1000

# 2. Detect stuck funds
./scripts/monitor-pallet-balance.sh --pallet validator_rewards --alert

# 3. Create emergency withdrawal proposal
./scripts/test-emergency-withdrawal.sh \
  --pallet validator_rewards \
  --amount 1000 \
  --recipient treasury

# 4. Execute with 7/9 multisig
./scripts/test-multisig-execution.sh --proposal-id $PROPOSAL_ID

# 5. Verify funds recovered
./scripts/verify-funds-recovered.sh --expected 1000
```

#### 3. EDSC Peg Break Response
```bash
# 1. Artificially trigger peg break (testnet only!)
./scripts/manipulate-edsc-price.sh --target-price 0.85

# 2. Verify circuit breaker activates
./scripts/verify-circuit-breaker.sh --expected-status Emergency

# 3. Test reserve injection
./scripts/test-reserve-injection.sh --amount 500000

# 4. Test interest rate adjustment
./scripts/test-interest-rate-adjustment.sh --new-rate 10

# 5. Test unpause after stabilization
./scripts/test-unpause-after-stabilization.sh
```

#### 4. Validator Payment Failure
```bash
# 1. Disable automated payment distribution
./scripts/disable-auto-distribution.sh --epoch 100

# 2. Verify payment missed alert
./scripts/verify-payment-alert.sh --epoch 100

# 3. Calculate manual rewards
./scripts/calculate-epoch-rewards.sh --epoch 100 > rewards.json

# 4. Execute manual distribution
./scripts/test-manual-distribution.sh --epoch 100 --input rewards.json

# 5. Verify all validators paid
./scripts/verify-validator-payments.sh --epoch 100
```

#### 5. Consensus Day Failure
```bash
# 1. Create approved proposal
./scripts/create-test-proposal.sh --mint-amount 1000000

# 2. Disable Consensus Day automation
./scripts/disable-consensus-day-automation.sh

# 3. Verify minting didn't execute
./scripts/verify-minting-not-executed.sh --date 2025-12-01

# 4. Execute manual minting
./scripts/test-manual-minting.sh --proposal-id $PROPOSAL_ID

# 5. Execute manual distribution
./scripts/test-manual-distribution.sh --mint-id $MINT_ID

# 6. Verify all recipients received funds
./scripts/verify-consensus-day-distribution.sh
```

### Success Criteria

All tests must pass with:
- ✅ **Response Time**: Meet severity-based SLAs
- ✅ **Multisig Execution**: 7/9 signatures collected and verified
- ✅ **Fund Recovery**: 100% of stuck funds recovered
- ✅ **Zero Downtime**: Network remains operational
- ✅ **Communication**: All stakeholders notified appropriately

### Quarterly Drill Report Template

```markdown
# Emergency Procedures Drill Report

**Date**: 2025-Q4
**Duration**: 4.5 hours
**Participants**: 9/9 Directors, 3 Technical Staff

## Scenarios Tested

### 1. Treasury Emergency Freeze
- **Status**: ✅ PASSED
- **Response Time**: 3 minutes (Target: <5 min)
- **Multisig Collection**: 12 minutes (Target: <15 min)
- **Issues**: None

### 2. Stuck Funds Recovery
- **Status**: ✅ PASSED
- **Response Time**: 8 minutes (Target: <15 min)
- **Funds Recovered**: 100%
- **Issues**: None

### 3. EDSC Peg Break
- **Status**: ⚠️ WARNING
- **Circuit Breaker**: Activated correctly
- **Reserve Injection**: 18 minutes (Target: <15 min)
- **Issues**: Multisig signing took 2 extra minutes

### 4. Validator Payment Failure
- **Status**: ✅ PASSED
- **Detection**: Immediate
- **Manual Distribution**: 22 minutes (Target: <30 min)
- **Issues**: None

### 5. Consensus Day Failure
- **Status**: ✅ PASSED
- **Manual Minting**: 15 minutes (Target: <30 min)
- **Distribution**: 28 minutes (Target: <60 min)
- **Issues**: None

## Recommendations

1. **Improve Multisig Speed**: Pre-sign emergency templates
2. **Automate Detection**: Add more monitoring alerts
3. **Update Documentation**: Clarify reserve injection steps

## Next Drill: 2026-Q1
```

---

## POST-INCIDENT REPORTING

### Immediate Report (Within 2 Hours)

**Template:**
```markdown
# INCIDENT REPORT: [SCENARIO NAME]

**Date**: 2025-10-31
**Time**: 14:35 UTC
**Severity**: CRITICAL
**Status**: RESOLVED

## Summary
Brief 2-3 sentence description of incident.

## Timeline
- 14:35 UTC: Incident detected
- 14:37 UTC: Directors alerted
- 14:40 UTC: Emergency call initiated
- 14:52 UTC: Recovery procedure executed
- 15:10 UTC: Incident resolved

## Impact
- **Users Affected**: [NUMBER]
- **Funds At Risk**: $[AMOUNT]
- **Downtime**: [DURATION]
- **Services Impacted**: [LIST]

## Actions Taken
1. [ACTION 1]
2. [ACTION 2]
3. [ACTION 3]

## Current Status
All services operational. No further action required.

## Next Steps
Detailed post-mortem report within 7 days.
```

### Detailed Post-Mortem (Within 7 Days)

**Sections:**

#### 1. Executive Summary
- What happened
- Why it happened
- What we did
- Lessons learned

#### 2. Incident Details
- Root cause analysis
- Detailed timeline
- Technical explanation
- Impact assessment

#### 3. Response Evaluation
- What went well
- What went poorly
- Response time analysis
- Communication effectiveness

#### 4. Prevention Measures
- Code changes
- Process improvements
- Monitoring enhancements
- Training needs

#### 5. Compensation Plan (If Applicable)
- Who is eligible
- How much
- When distributed
- Claim process

#### 6. Transparency Commitment
- What we're sharing publicly
- What we're keeping internal (if anything)
- Ongoing updates

### Public Communication Guidelines

**DO:**
- Be transparent about what happened
- Acknowledge mistakes
- Explain technical details (in accessible language)
- Share prevention measures
- Update frequently

**DON'T:**
- Minimize severity
- Blame individuals
- Use jargon without explanation
- Make promises you can't keep
- Hide information

**Example Public Statement:**
```
Subject: Incident Resolution: Validator Payment Failure (Oct 31)

Dear Ëtrid Community,

On October 31, 2025 at 14:35 UTC, we detected a failure in the automated
validator payment distribution system for Epoch 142.

WHAT HAPPENED:
The reward pool account ran out of funds due to a miscalculation in the
epoch funding logic, causing the automated distribution to fail.

IMPACT:
- 21 validators did not receive Epoch 142 rewards on time
- No funds were lost
- Network remained operational
- Users were not affected

RESOLUTION:
- Issue detected within 5 minutes
- Directors executed manual distribution within 30 minutes
- All 21 validators received their full rewards + 5% compensation
- Root cause identified and fixed in runtime v101

PREVENTION:
- Added automated reward pool monitoring with 48hr warning
- Implemented pre-distribution balance checks
- Increased reward pool buffer from 2 epochs to 5 epochs
- Scheduled quarterly distribution tests

We apologize for the delay in validator payments and thank our validators
for their patience. The additional 5% compensation has been distributed.

Full technical post-mortem: https://etrid.io/incidents/2025-10-31

Questions? Contact support@etrid.io

- Ëtrid Foundation Directors
```

---

## APPENDIX: EMERGENCY SCRIPTS REFERENCE

### Located in `/Users/macbook/Desktop/etrid/scripts/emergency/`

```bash
scripts/emergency/
├── freeze-treasury.sh          # Emergency treasury freeze
├── unfreeze-treasury.sh        # Unfreeze treasury (requires 7/9)
├── emergency-withdrawal.sh     # Withdraw stuck funds from pallet
├── manual-distribution.sh      # Manually distribute validator rewards
├── pause-edsc-minting.sh       # Emergency pause EDSC minting
├── unpause-edsc-minting.sh     # Unpause EDSC minting (requires 7/9)
├── inject-reserves.sh          # Inject reserves into EDSC system
├── manual-consensus-mint.sh    # Manual Consensus Day minting
├── rollback-minting.sh         # Rollback incorrect minting (requires 8/9)
├── create-multisig-proposal.sh # Create multisig transaction
├── collect-signatures.sh       # Collect Director signatures
├── execute-multisig.sh         # Execute multisig after threshold reached
├── verify-recovery.sh          # Verify recovery successful
└── generate-incident-report.sh # Generate incident report template
```

### Usage Examples

```bash
# Emergency treasury freeze (any Director)
./scripts/emergency/freeze-treasury.sh \
  --director-key /secure/keys/alice.json \
  --reason "Suspected compromise detected"

# Emergency withdrawal (requires 7/9 Directors)
./scripts/emergency/emergency-withdrawal.sh \
  --pallet validator_rewards \
  --amount 1000000000000000000 \
  --recipient 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --reason "Stuck funds recovery" \
  --director-keys /secure/keys/dir_{1..7}.json

# Manual validator distribution
./scripts/emergency/manual-distribution.sh \
  --epoch 142 \
  --rewards-file rewards.json \
  --director-keys /secure/keys/dir_{1..5}.json

# Pause EDSC minting
./scripts/emergency/pause-edsc-minting.sh \
  --director-keys /secure/keys/dir_{1..5}.json

# Inject reserves
./scripts/emergency/inject-reserves.sh \
  --asset USDC \
  --amount 5000000000000 \
  --reason "Peg stabilization" \
  --director-keys /secure/keys/dir_{1..7}.json
```

---

## DOCUMENT CONTROL

**Version History:**

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-10-31 | Ëtrid Foundation | Initial version |

**Review Schedule:**
- **Monthly**: Review by Lead Director
- **Quarterly**: Full review and drill
- **Annually**: Comprehensive update

**Distribution:**
- All 9 Directors (required)
- Technical Lead
- Security Team
- Legal Counsel (for regulatory scenarios)

**Classification**: CRITICAL - For Foundation Directors Only

**Contact:**
- Emergency Hotline: +1-XXX-XXX-XXXX (24/7)
- Email: emergency@etrid.io
- Discord: #director-emergency (private channel)

---

**END OF EMERGENCY FUND RECOVERY GUIDE**
