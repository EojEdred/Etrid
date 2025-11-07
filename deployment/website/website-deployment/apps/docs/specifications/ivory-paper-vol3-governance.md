# ËTRID IVORY PAPERS
## Volume III: Governance & Fiscal Mechanics

**Document ID**: ETRID-IP-VOL3-2025
**Status**: ACTIVE PROTOCOL SPECIFICATION
**Publication Date**: October 24, 2025
**Founder**: Eoj Edred
**License**: GPLv3 (Open Source, Non-Commercial)

---

## VOLUME III CONTENTS

1. Consensus Day - The Constitutional Event
2. Governance Roles & Responsibilities
3. Fiscal Cycle & Treasury Management
4. Token Economics (ÉTR, EDSC, VMw)
5. Proposal System & Voting Mechanisms
6. Reward Distribution & Slashing
7. Economic Sustainability Model
8. Emergency Procedures & Crisis Response

---

## 1. CONSENSUS DAY - THE CONSTITUTIONAL EVENT

### Overview

**Consensus Day** occurs annually on **December 1st** at **12:00 AM PST**. It is the network's constitutional moment where all major governance decisions are made collectively.

**Frequency**: Once per year (365 days)
**Participation**: All ÉTR stakers (VALIDITY Nodes, Common Stake Peers, Decentralized Directors)

**Pre-Consensus Period**: January 1 – October 31
- Registration for eligibility
- Proposal submission
- Campaigning for proposals and director candidates
- Preliminary voting and discussion

**Consensus Day Purpose**:
- Vote on proposals (protocol improvements, standards, parameter changes, developer commits)
- Adjust economic parameters (rewards, penalties, supply, salaries)
- Elect 9 Decentralized Directors for the upcoming year
- Distribute rewards to participating stakeholders

**Voting Power**: Staked ÉTR × Coinage
- Coinage = time-weighted stake (how long tokens have been staked)
- Longer stakes = higher voting power multiplier
- Encourages long-term stakeholder commitment

### Phase 1: Registration (6 hours)

**Purpose**: Submit proposals, lock stakes, register participation

**Actions**:
- **Proposal Submission**: Anyone with ≥ 10,000 ÉTR can submit proposals
- **Stake Locking**: Participants lock ÉTR to receive voting power
- **Validator Registration**: Validators signal participation (required for quorum)
- **Delegate Assignment**: Token holders can delegate votes to trusted parties

**Proposal Types**:
1. **Inflation Rate Adjustment** (hard cap: 0-5% annual)
2. **Parameter Changes** (block times, finality thresholds, fee structure)
3. **Budget Allocation** (grants, development, marketing, operations)
4. **Protocol Upgrades** (runtime changes, pallet additions)
5. **Director Elections** (elect 9 Decentralized Directors)
6. **Emergency Actions** (slash validators, pause chains, deploy hotfixes)

**Proposal Requirements**:
```rust
struct Proposal {
    proposer: AccountId,
    title: String,              // Max 100 chars
    description: String,        // Max 2000 chars
    category: ProposalCategory,
    budget_request: Balance,    // If funding required
    implementation_plan: Hash,  // IPFS hash to detailed plan
    bond: Balance,              // 10,000 ÉTR minimum
    supporting_validators: Vec<AccountId>, // Min 3 validators
}
```

**Proposal Bond**:
- **Amount**: 10,000 ÉTR (locked during voting)
- **Refund**: Full refund if proposal reaches 5% quorum
- **Slash**: 50% slashed if spam/malicious, 50% returned if simply unpopular

### Phase 2: Voting (12 hours)

**Purpose**: Community + validators vote on all proposals

**Voting Power Calculation**:
```rust
fn calculate_voting_power(
    staked_amount: Balance,
    stake_duration: BlockNumber, // How long stake has been locked
    participation_history: u32,  // Previous Consensus Days attended
) -> VotingPower {
    let base_power = staked_amount;

    // Stake duration bonus (max +20%)
    let duration_multiplier = 1.0 + (stake_duration / BLOCKS_PER_YEAR).min(0.2);

    // Participation history bonus (max +10%)
    let history_multiplier = 1.0 + (participation_history as f64 * 0.02).min(0.1);

    let adjusted_power = base_power * duration_multiplier * history_multiplier;

    VotingPower::new(adjusted_power as u128)
}
```

**Voting Mechanism**:
- **Vote Options**: Yes / No / Abstain
- **Vote Weight**: Proportional to voting power
- **Vote Privacy**: Votes are public (on-chain transparency)
- **Vote Changes**: Can change vote until Phase 2 ends
- **Delegation**: Votes can be delegated (revocable at any time)

**Quorum Requirements**:

Dual quorum system ensures both community and validators participate:

```rust
struct QuorumRequirements {
    community_quorum: Percent,    // 33% of circulating ÉTR must vote
    validator_quorum: Percent,    // 51% of active validators must vote
}

fn check_quorum(proposal: &Proposal) -> bool {
    let community_participation = proposal.total_votes / circulating_supply();
    let validator_participation = proposal.validator_votes.len() / active_validators();

    community_participation >= 0.33 && validator_participation >= 0.51
}
```

**Approval Threshold**:
- **Simple Majority**: >50% for budget allocations, parameter tweaks
- **Supermajority**: >66% for protocol upgrades, emergency actions
- **Unanimous Director Approval**: All 9 Directors for constitutional changes

### Phase 3: Minting (3 hours)

**Purpose**: Execute approved budgets by minting new ÉTR

**Minting Logic**:
```rust
fn execute_approved_budgets(approved_proposals: Vec<Proposal>) -> Balance {
    let mut total_mint_amount = 0u128;

    for proposal in approved_proposals {
        if proposal.category == ProposalCategory::BudgetAllocation {
            // Verify budget doesn't exceed annual cap
            let annual_cap = circulating_supply() * approved_inflation_rate / 100;

            if total_mint_amount + proposal.budget_request <= annual_cap {
                // Mint tokens
                mint_to_treasury(proposal.budget_request);
                total_mint_amount += proposal.budget_request;

                // Emit event
                emit_event(Event::BudgetMinted {
                    proposal_id: proposal.id,
                    amount: proposal.budget_request,
                    recipient: Treasury::account_id(),
                });
            }
        }
    }

    total_mint_amount
}
```

**Inflation Rate Control**:
- **Community Decides**: Consensus Day votes set annual inflation
- **Hard Cap**: 0-5% per year (enforced by protocol)
- **Default**: If no vote reaches consensus, previous year's rate continues
- **Emergency Override**: Directors can propose 0% inflation in crisis

**Minting Events**:
```rust
enum MintEvent {
    BudgetMinted { proposal_id: u64, amount: Balance, recipient: AccountId },
    InflationApplied { rate: Percent, total_minted: Balance },
    TreasuryFunded { source: MintSource, amount: Balance },
}
```

### Phase 4: Distribution (1 hour)

**Purpose**: Reward all participants who voted

**Distribution Formula**:
```rust
fn calculate_participation_reward(
    voter: &Voter,
    total_participation_pool: Balance,
) -> Balance {
    // Pool = 1% of total minted during Phase 3
    let participation_pool = total_participation_pool;

    // Individual share based on voting power used
    let voter_share = voter.voting_power_used / total_voting_power_cast;

    // Bonus for voting on all proposals (completeness bonus)
    let completeness_multiplier = if voter.voted_on_all {
        1.2
    } else {
        1.0
    };

    let base_reward = participation_pool * voter_share;
    let final_reward = base_reward * completeness_multiplier;

    Balance::from(final_reward as u128)
}
```

**Distribution Components**:
1. **Participation Rewards**: 1% of minted tokens divided among voters
2. **Validator Rewards**: Validators receive 0.5% bonus for participation
3. **Proposer Rewards**: Approved proposals earn 100 ÉTR per proposal
4. **Director Stipends**: 9 Directors receive equal shares (total: 0.2% of mint)

**Distribution Example**:

If 100M ÉTR minted during Consensus Day:
```
Total Minted:            100,000,000 ÉTR
Participation Pool (1%):   1,000,000 ÉTR → Divided among voters
Validator Bonus (0.5%):      500,000 ÉTR → Divided among validators
Director Stipends (0.2%):    200,000 ÉTR → 22,222 ÉTR per Director
Proposer Rewards:            100 ÉTR × approved proposals
Remaining:               98,300,000 ÉTR → Treasury for budgets
```

---

## 2. GOVERNANCE ROLES & RESPONSIBILITIES

### Role Hierarchy

```
┌─────────────────────────────────────────────┐
│            Community (All ÉTR Holders)      │
│  - Vote on proposals                        │
│  - Elect Directors                          │
│  - Propose changes (≥10k ÉTR)              │
└─────────────────┬───────────────────────────┘
                  │
        ┌─────────┴─────────┐
        │                   │
┌───────▼──────┐   ┌───────▼──────────┐
│  Directors   │   │    Validators    │
│  (9 members) │   │  (Active Set)    │
│  - Oversee   │   │  - Validate      │
│  - Veto      │   │  - Secure        │
│  - Emergency │   │  - Vote          │
└──────────────┘   └──────────────────┘
```

### The Community (Token Holders)

**Power**:
- Vote on all proposals during Consensus Day
- Elect 9 Decentralized Directors annually
- Propose protocol changes (requires ≥10,000 ÉTR bond)
- Delegate voting power to trusted parties

**Responsibilities**:
- Participate in annual governance
- Review proposals and technical documentation
- Report bugs, vulnerabilities, exploits
- Contribute to ecosystem growth

**Rights**:
- Receive staking rewards
- Access to all governance decisions and data
- Freedom to fork the network (GPLv3)

### Decentralized Directors (The Board)

**Composition**: 9 individuals elected annually

**Requirements**:
- **Minimum stake: 128 ÉTR**
- **Must serve as OD Flare Nodes** (Operational Director Flare Nodes)
- Must maintain active participation in network operations

**Term**: 1 year (renewable via re-election)

**Term Limits**:
- **One-year terms**
- **One-year cooldown** between terms
- **Maximum 3 lifetime terms** (cannot serve more than 3 total terms)

**Election Process**:
1. Candidates announce during Pre-Consensus Period (Jan 1 - Oct 31)
2. Community votes on Consensus Day (December 1)
3. Top 9 vote-getters elected
4. Ties resolved by stake-weighted runoff
5. Voting power = Staked ÉTR × Coinage

**Powers**:
1. **Governance Oversight**: Oversee development and consensus proposals
2. **Protocol Maintenance**: Coordinate security audits, bug bounties
3. **Representation**: Speak for Ëtrid at conferences, media, partnerships
4. **Treasury Management**: Approve budget disbursements (community-authorized)
5. **Emergency Coordination**: Coordinate response to network emergencies

**Limitations**:
- Cannot change protocol without community consensus vote
- Cannot mint tokens outside Consensus Day
- **No hierarchy** among Directors - all decisions are community-driven
- Cannot override community supermajority
- Subject to term limits (max 3 lifetime terms)

**Compensation**:
- **Community-voted salaries** (determined annually on Consensus Day)
- No predetermined amounts - stakeholders decide compensation
- Funded from Consensus Day distribution

**Accountability**:
- Regular public reports required
- Can be removed mid-term via emergency community vote
- All communications public and logged
- Bound by Foundation bylaws

### VALIDITY Nodes (Validators)

**Role**: Ëtrid Partitioned Burst Chain nodes in consensus, syncing with Flare Chain. Responsible for block production, finality attestation, and cross-chain state verification.

**Requirements**:
- **Minimum stake: 64 ÉTR**
- Hardware: 8-core CPU, 32GB RAM, 1TB SSD, 1Gbps network
- Uptime: >98% (measured over 30-day rolling window)
- Must run full nodes for FlareChain + ≥1 PBC
- Identity: Must verify on-chain identity (no KYC, just persistent identity)

**Node Statuses**:
- Registered
- Pending
- Sequenced
- Chilled
- De-Sequenced
- Re-Sequenced

**Rewards**:
```rust
fn calculate_validator_reward(
    validator: &Validator,
    epoch_duration: BlockNumber,
    total_validator_pool: Balance,
) -> Balance {
    // Base reward proportional to stake
    let stake_ratio = validator.stake / total_active_stake();
    let base_reward = total_validator_pool * stake_ratio;

    // Uptime multiplier (0.90 - 1.10)
    let uptime_multiplier = 0.9 + (validator.uptime - 0.95).max(0.0) * 2.0;

    // Finality contribution (signed blocks / expected blocks)
    let finality_multiplier = validator.blocks_signed / validator.blocks_expected;

    // Final reward
    base_reward * uptime_multiplier * finality_multiplier
}
```

**Slashing**:
- **Downtime**: -1% stake per day offline (after 24h grace period)
- **Equivocation**: -10% stake for double-signing blocks
- **Malicious Behavior**: -50% stake + removal for provable attacks
- **Censorship**: -5% stake for ignoring valid transactions

### Common Stake Peers

**Role**: Stakeholders who participate in governance without running full node infrastructure

**Requirements**:
- **Minimum stake: 1+ ÉTR**

**Privileges**:
- Voting on proposals during Consensus Day
- Campaigning for proposals
- Reporting faults and issues
- Earning rewards based on stake, coinage, and activity

**Voting Power Formula**:
```
Voting Power = Staked ÉTR × Coinage
```

Where coinage represents how long tokens have been staked (time-weighted stake)
- Must run full nodes for FlareChain + ≥3 PBCs
- Must respond to verification requests within 10 blocks

**Rewards**:
- 0.1% fee on all cross-chain transactions
- Proportional to verification work performed
- Paid from cross-chain transaction fees

**Slashing**:
- -20% stake for incorrect verification
- -50% stake for colluding to pass invalid proofs

### The Foundation (Pre-Mainnet Only)

**Role**: Bootstrap the network until first Consensus Day

**Powers**:
- Set initial parameters (inflation, fees, block times)
- Deploy initial infrastructure
- Fund initial development
- Conduct security audits

**Sunset Clause**:
- Foundation dissolves after first successful Consensus Day
- All remaining funds transferred to community treasury
- No ongoing authority or control

---

## 3. FISCAL CYCLE & TREASURY MANAGEMENT

### Treasury Architecture

**Treasury Account**: On-chain multisig controlled by 9 Directors (6-of-9 threshold)

**Funding Sources**:
1. **Transaction Fees**: 50% of all fees → Treasury (50% burned)
2. **Consensus Day Minting**: Approved budgets minted to treasury
3. **Validator Slashing**: 50% of slashed stakes → Treasury (50% burned)
4. **Cross-Chain Fees**: 10% of bridge fees → Treasury

**Treasury Balance Formula**:
```rust
fn calculate_treasury_balance() -> Balance {
    let inflows = transaction_fees * 0.5
                + consensus_day_mint
                + slashing_proceeds * 0.5
                + bridge_fees * 0.1;

    let outflows = budget_disbursements
                 + validator_rewards
                 + director_stipends
                 + emergency_expenses;

    previous_balance + inflows - outflows
}
```

### Fiscal Year Cycle

**Fiscal Year**: December 1st → November 30th (aligned with Consensus Day)

**Budget Categories**:
1. **Development** (40% of budget): Core protocol, tools, libraries
2. **Marketing** (20% of budget): Education, outreach, partnerships
3. **Operations** (15% of budget): Infrastructure, monitoring, support
4. **Grants** (15% of budget): Community projects, research, ecosystem
5. **Emergency Reserve** (10% of budget): Held for crisis response

**Budget Approval Process**:
1. **Consensus Day**: Community votes on budget allocation per category
2. **Director Oversight**: Directors approve individual disbursements
3. **Quarterly Review**: Directors publish spending reports every 3 months
4. **Annual Audit**: Third-party audit published before next Consensus Day

### Treasury Transparency

**All treasury actions emit events**:
```rust
enum TreasuryEvent {
    Deposit { source: DepositSource, amount: Balance },
    Disbursement { category: BudgetCategory, recipient: AccountId, amount: Balance },
    DirectorVote { director: AccountId, proposal_id: u64, vote: bool },
    QuarterlyReport { quarter: u8, total_spent: Balance, category_breakdown: Vec<(BudgetCategory, Balance)> },
}
```

**Public Dashboard**:
- Real-time treasury balance
- All incoming/outgoing transactions
- Budget vs. actual spending per category
- Director voting records
- Historical trends (YoY comparison)

### Mint and Burn Dynamics

**Minting Events**:
1. **Consensus Day**: Annual minting for approved budgets
2. **Emergency Minting**: Directors can mint (7/9 vote) in crisis (max 1% of supply)

**Burning Events**:
1. **Transaction Fees**: 50% of all fees burned
2. **Validator Slashing**: 50% of slashed stakes burned
3. **Deflationary Votes**: Community can vote to burn treasury reserves

**Net Supply Formula**:
```rust
fn calculate_net_supply_change(epoch: Epoch) -> i128 {
    let minted = consensus_day_mint + emergency_mint;
    let burned = transaction_fees * 0.5 + slashing_proceeds * 0.5 + voluntary_burns;

    (minted as i128) - (burned as i128)
}
```

**Target**: Long-term supply should stabilize or decrease slowly (deflationary tendency)

---

## 4. TOKEN ECONOMICS (ÉTR, EDSC, VMw)

### ÉTR (Ëtrid Token)

**Purpose**: Governance, staking, gas fees

**Total Supply**: 1 Billion ÉTR (initial), subject to inflation/deflation

**Initial Distribution**:
```
Genesis Block:           1,000,000,000 ÉTR

Allocation:
- Public Sale (40%):       400,000,000 ÉTR
- Foundation (25%):        250,000,000 ÉTR (4-year vesting)
- Team (15%):              150,000,000 ÉTR (3-year vesting)
- Validators (10%):        100,000,000 ÉTR (incentive pool)
- Ecosystem Grants (10%):  100,000,000 ÉTR (unlocked)
```

**Inflation/Deflation**:
- **Inflation**: Voted annually (0-5% cap), minted on Consensus Day
- **Deflation**: Transaction fees + slashing continuously burned
- **Net Effect**: Tends toward slight deflation long-term

**Use Cases**:
1. **Staking**: Lock ÉTR to become validator or voter
2. **Gas Fees**: Pay for transactions (measured in VMw, priced in ÉTR)
3. **Governance**: Vote on proposals during Consensus Day
4. **Collateral**: Back EDSC stablecoin (over-collateralized)

### EDSC (Ëtrid Dollar Stablecoin)

**Purpose**: Stable unit of account for payments, contracts

**Peg**: $1.00 USD (soft peg maintained by arbitrage + liquidations)

**Collateral**: Over-collateralized by multi-asset reserve (target: 150%)

**Reserve Infrastructure**:

EDSC is backed by a diversified multi-asset reserve managed through two specialized pallets:

1. **pallet-multiasset-reserve**: Manages reserve composition and automated rebalancing
2. **pallet-reserve-backed-token**: Handles EDSC minting and collateral positions

**Reserve Composition**:
```
Target Allocation (example):
- 40% ÉTR (native token)
- 30% BTC (bridged from BTC-PBC)
- 20% ETH (bridged from ETH-PBC)
- 10% Other assets (governance-approved)

Rebalancing:
- Triggered when deviation exceeds 5%
- Automated via pallet-multiasset-reserve
- Governance can adjust allocation strategy
```

**Allocation Strategies**:
- **EqualWeight**: Equal distribution across all reserve assets
- **MarketCapWeighted**: Weight by market capitalization
- **RiskAdjusted**: Weight by volatility/risk metrics
- **Custom**: Custom weights set by governance vote

**Minting Process**:
```rust
fn mint_edsc(collateral_amount: Balance) -> Result<Balance, Error> {
    let etr_price_usd = oracle::get_etr_price(); // From price oracle
    let collateral_value_usd = collateral_amount * etr_price_usd;

    // Require 150% collateralization
    let max_edsc_mintable = collateral_value_usd / 1.5;

    // Lock collateral in reserve
    lock_collateral(caller, collateral_amount)?;

    // Mint EDSC via pallet-reserve-backed-token
    mint_stablecoin(caller, max_edsc_mintable)?;

    // Reserve automatically rebalances if needed
    if should_rebalance() {
        trigger_rebalance()?;
    }

    Ok(max_edsc_mintable)
}
```

**Liquidation Process**:
```rust
fn check_liquidation(cdp: &CollateralDebtPosition) -> bool {
    let etr_price_usd = oracle::get_etr_price();
    let collateral_value = cdp.collateral_amount * etr_price_usd;
    let debt_value = cdp.edsc_minted; // 1 EDSC = $1

    let collateralization_ratio = collateral_value / debt_value;

    // Liquidate if ratio falls below 120% (updated threshold)
    if collateralization_ratio < 1.2 {
        liquidate_cdp(cdp);
        true
    } else {
        false
    }
}
```

**Collateralization Parameters**:
```
Minimum Collateral Ratio: 150%
Liquidation Threshold:     120%
Liquidation Penalty:       5%

Example:
To mint 1000 EDSC ($1000 value):
→ Requires: $1500 in collateral (150%)
→ Liquidated if collateral falls to $1200 (120%)
→ Liquidator receives: $1200 + 5% penalty = $1260
→ Position holder loses: $60 to liquidator
```

**Stability Mechanisms**:
1. **Multi-Asset Reserve**: Reduces correlation risk compared to single-asset backing
2. **Automated Rebalancing**: Maintains target allocations via pallet-multiasset-reserve
3. **Over-Collateralization**: 150% target ratio prevents undercollateralization
4. **Liquidation**: Automated liquidations at 120% ratio (updated from 110%)
5. **Liquidation Penalty**: 5% penalty incentivizes proper position management
6. **Interest Rates**: Dynamic interest on borrowed EDSC adjusts to maintain peg
7. **Arbitrage**: Price deviations create profitable arbitrage opportunities
8. **DEX Integration**: FlareSwap enables efficient ÉTR/EDSC trading and price discovery

**Reserve-Backed Token Framework**:

Beyond EDSC, the reserve infrastructure enables creation of diverse synthetic assets:

**Synthetic Asset Types**:
- **Stablecoins**: EDSC (USD), EEUR (Euro), EGBP (Pound)
- **Commodities**: EXAU (Gold), EXAG (Silver), EXOIL (Oil)
- **Equities**: ETSLA (Tesla), EAAPL (Apple), EGOOG (Google)
- **Indices**: ES&P (S&P 500), ENASDAQ (Nasdaq), EDJI (Dow Jones)

**Governance Controls**:
- Whitelisting of reserve assets (Consensus Day vote required)
- Adjustment of collateralization ratios per synthetic
- Rebalancing strategy selection
- Emergency pause of minting/liquidations

### VMw (Virtual Machine Watts)

**Purpose**: Measure computational cost (gas metering)

**Definition**: 1 VMw = 1 Watt-second of CPU energy

**Pricing**:
```rust
fn calculate_gas_price() -> Balance {
    // Dynamic pricing based on block fullness
    let block_utilization = current_block_vmw / max_block_vmw; // 0.0 - 1.0

    // Base price: 1 VMw = 0.000001 ÉTR
    let base_price = 1_000_000_000_000u128; // 1e-6 ÉTR in plancks

    // Price increases exponentially with utilization
    let surge_multiplier = if block_utilization > 0.75 {
        1.0 + ((block_utilization - 0.75) * 8.0)
    } else {
        1.0
    };

    Balance::from((base_price as f64 * surge_multiplier) as u128)
}
```

**Block Limits**:
- **Max VMw per block**: 10,000,000 VMw
- **Target utilization**: 50% (5,000,000 VMw)
- **Surge pricing**: Kicks in above 75% utilization

**Example Transaction Costs**:
```
Simple Transfer:           100 VMw   ≈ 0.0001 ÉTR   ($0.001 at $10/ÉTR)
Token Swap:              5,000 VMw   ≈ 0.005 ÉTR    ($0.05)
Complex Contract:       50,000 VMw   ≈ 0.05 ÉTR     ($0.50)
Cross-Chain Bridge:    200,000 VMw   ≈ 0.2 ÉTR      ($2.00)
```

---

## 5. PROPOSAL SYSTEM & VOTING MECHANISMS

### Proposal Lifecycle

```
1. Draft → 2. Submission → 3. Discussion → 4. Voting → 5. Execution
   (Off-chain)  (Registration)   (48h window)    (12h vote)   (Minting + Distribution)
```

### Proposal Template

**Minimum Requirements**:
```markdown
# Proposal Title (max 100 chars)

## Summary (max 300 chars)
One-paragraph overview accessible to all voters.

## Motivation
Why is this proposal necessary? What problem does it solve?

## Specification
Technical details. How will this be implemented?

## Budget Request (if applicable)
Amount: _______ ÉTR
Breakdown:
  - Development: _______ ÉTR
  - Audit: _______ ÉTR
  - Deployment: _______ ÉTR

## Implementation Timeline
Phase 1: (date range) - Milestone 1
Phase 2: (date range) - Milestone 2
...

## Success Criteria
How will we know this proposal succeeded?

## Risks & Mitigations
What could go wrong? How will we address it?

## Supporting Validators (min 3)
- Validator 1: [address]
- Validator 2: [address]
- Validator 3: [address]
```

### Voting Rules

**Vote Types**:
1. **Yes**: Support the proposal
2. **No**: Reject the proposal
3. **Abstain**: Count toward quorum but not approval

**Quadratic Voting** (Optional, for contentious issues):
```rust
fn calculate_quadratic_vote_cost(num_votes: u32) -> Balance {
    // Cost increases quadratically
    // 1 vote = 1 ÉTR, 2 votes = 4 ÉTR, 3 votes = 9 ÉTR, etc.
    let cost_per_vote = num_votes * num_votes;
    Balance::from(cost_per_vote as u128 * VOTE_COST_BASE)
}
```

**Conviction Voting** (Optional, for long-term decisions):
```rust
enum Conviction {
    None,       // 1x voting power, unlock immediately
    Locked1x,   // 1x voting power, locked 1 epoch after vote
    Locked2x,   // 2x voting power, locked 2 epochs
    Locked4x,   // 4x voting power, locked 4 epochs
    Locked8x,   // 8x voting power, locked 8 epochs
}
```

### Proposal Execution

**Automatic Execution**:
- If proposal approved with quorum → Executed in Phase 3 (Minting)
- If proposal rejected → Bond returned (if >5% quorum), proposal archived
- If proposal doesn't reach quorum → Bond slashed, proposal discarded

**Manual Execution** (for protocol upgrades):
- Directors + validators coordinate deployment
- Upgrade scheduled for specific block height
- All nodes must upgrade before deadline

---

## 6. REWARD DISTRIBUTION & SLASHING

### Validator Rewards

**Annual Validator Reward Pool**: ~3% of circulating supply

**Distribution**:
```rust
fn distribute_epoch_rewards(epoch: Epoch) {
    let total_pool = circulating_supply() * 0.03 / EPOCHS_PER_YEAR;

    for validator in active_validators() {
        let base_reward = total_pool * (validator.stake / total_stake);
        let adjusted_reward = base_reward * validator.performance_multiplier;

        // Pay 50% to validator, 50% to delegators
        pay_reward(validator.account, adjusted_reward * 0.5);
        pay_delegators(validator, adjusted_reward * 0.5);
    }
}
```

**Performance Multiplier**:
```rust
fn calculate_performance_multiplier(validator: &Validator) -> f64 {
    let uptime_score = validator.uptime; // 0.0 - 1.0
    let finality_score = validator.blocks_signed / validator.blocks_expected;
    let participation_score = if validator.voted_consensus_day { 1.1 } else { 1.0 };

    uptime_score * finality_score * participation_score
}
```

### Delegator Rewards

**Delegation Mechanism**:
- Token holders can delegate ÉTR to validators
- Validators charge commission (0-100%, set by validator)
- Rewards automatically distributed each epoch

**Commission Structure**:
```rust
fn distribute_delegation_rewards(validator: &Validator, total_reward: Balance) {
    let commission_rate = validator.commission; // e.g., 0.10 = 10%
    let commission_amount = total_reward * commission_rate;
    let delegator_pool = total_reward - commission_amount;

    // Pay commission to validator
    pay_reward(validator.account, commission_amount);

    // Distribute remaining to delegators proportionally
    for delegator in validator.delegators() {
        let delegator_share = delegator.stake / validator.total_delegated;
        let delegator_reward = delegator_pool * delegator_share;
        pay_reward(delegator.account, delegator_reward);
    }
}
```

### Slashing Conditions

**Offense Types**:

| Offense | Severity | Slash Amount | Cooldown |
|---------|----------|--------------|----------|
| Downtime (>24h) | Low | 1% per day | 7 days |
| Missed blocks (>10%) | Low | 0.5% | 3 days |
| Equivocation (double-sign) | High | 10% | 30 days |
| Invalid finality vote | High | 5% | 14 days |
| Censorship (provable) | Medium | 5% | 14 days |
| Coordinated attack | Critical | 50% + removal | Permanent ban |

**Slashing Execution**:
```rust
fn execute_slash(
    validator: AccountId,
    offense: OffenseType,
    evidence: Vec<u8>,
) -> Result<(), Error> {
    // Verify evidence
    verify_offense_evidence(offense, evidence)?;

    // Calculate slash amount
    let slash_amount = match offense {
        OffenseType::Downtime => validator.stake * 0.01,
        OffenseType::Equivocation => validator.stake * 0.10,
        OffenseType::MaliciousAttack => validator.stake * 0.50,
        // ... other offenses
    };

    // Execute slash
    burn_tokens(validator, slash_amount * 0.5); // 50% burned
    transfer_to_treasury(slash_amount * 0.5);   // 50% to treasury

    // Apply cooldown
    apply_validator_cooldown(validator, offense.cooldown_period);

    // Emit event
    emit_event(Event::ValidatorSlashed {
        validator,
        offense,
        amount: slash_amount,
        timestamp: current_block(),
    });

    Ok(())
}
```

### Appeal Process

**Validators can appeal slashing**:
1. Submit appeal within 72 hours (costs 1,000 ÉTR bond)
2. Directors review evidence (7/9 vote required to overturn)
3. If appeal successful: slash reversed, bond returned
4. If appeal rejected: bond added to treasury

---

## 7. ECONOMIC SUSTAINABILITY MODEL

### Long-Term Supply Dynamics

**Target**: Slightly deflationary long-term (~1% annual deflation)

**Inflows** (Minting):
- Consensus Day approved budgets: 0-5% per year (voted)
- Emergency minting: <0.1% per year (rare)

**Outflows** (Burning):
- Transaction fees: ~50% of fees (continuous)
- Slashing: ~50% of slashed stakes (episodic)
- Voluntary burns: Community-voted (rare)

**Net Supply Projection**:
```
Year 1:  1.00B ÉTR (genesis)
Year 2:  1.02B ÉTR (+2% voted inflation - 1% burned = +1% net)
Year 3:  1.03B ÉTR (+2% inflation - 1.5% burned = +0.5% net)
Year 5:  1.04B ÉTR (+1.5% inflation - 2% burned = -0.5% net)
Year 10: 1.00B ÉTR (back to genesis, deflationary equilibrium)
```

### Fee Market Dynamics

**Transaction Fee Formula**:
```rust
fn calculate_transaction_fee(vmw_cost: u64, priority: Priority) -> Balance {
    let base_fee = vmw_cost * get_current_vmw_price(); // Dynamic pricing

    let priority_multiplier = match priority {
        Priority::Low => 0.8,
        Priority::Normal => 1.0,
        Priority::High => 1.5,
        Priority::Urgent => 3.0,
    };

    base_fee * priority_multiplier
}
```

**Fee Allocation**:
- 50% burned (deflationary pressure)
- 40% to validators (reward for block production)
- 10% to treasury (public goods funding)

### Economic Security

**Attack Cost Analysis**:

To attack Ëtrid, an adversary must:
1. Acquire >33% of staked ÉTR (to disrupt finality)
2. Operate attack for multiple epochs before detection
3. Survive slashing and social coordination against them

**Cost**:
```
Assume 40% of supply staked (400M ÉTR)
33% of staked = 132M ÉTR
At $10/ÉTR = $1.32 Billion capital requirement

Cost of attack:
- Capital cost: $1.32B
- Slashing risk: 50% = $660M loss if caught
- Opportunity cost: 8% APY = $105M/year forgone rewards
- Reputation cost: ÉTR price crash → total loss

Total cost: >$2 Billion for 51% attack
```

**Conclusion**: Economic security increases with network value and participation

---

## 8. EMERGENCY PROCEDURES & CRISIS RESPONSE

### Emergency Powers

**Directors Emergency Authority** (7-of-9 vote):
1. **Pause Chains**: Temporarily halt block production (max 72 hours)
2. **Emergency Upgrade**: Deploy hotfix without Consensus Day vote
3. **Slash Malicious Actors**: Immediate slashing for provable attacks
4. **Emergency Mint**: Mint up to 1% of supply for critical bug bounty

### Crisis Response Protocol

**Level 1: Minor Issue** (e.g., validator downtime spike)
- Response Team: On-call validators
- Action: Monitor, alert community
- Timeline: 24-hour resolution

**Level 2: Moderate Issue** (e.g., smart contract exploit)
- Response Team: Directors + Security Team
- Action: Pause affected pallets, deploy patch
- Timeline: 48-hour resolution

**Level 3: Critical Issue** (e.g., consensus failure)
- Response Team: All Directors + Core Developers + Community
- Action: Emergency governance vote, possible chain halt
- Timeline: Immediate action, 7-day resolution

### Social Recovery

**If network becomes compromised beyond repair**:
1. **Snapshot**: Take state snapshot at last known good block
2. **Fork**: Create new chain from snapshot
3. **Migrate**: Community coordinates migration to new chain
4. **Deprecate**: Old chain deprecated, new chain becomes canonical

**GPLv3 Protection**: Anyone can fork and continue the network

---

## 9. CONCLUSION: GOVERNANCE AS A LIVING SYSTEM

### Why This Model Works

1. **Alignment**: Token holders, validators, and users all benefit from network success
2. **Adaptation**: Annual Consensus Day forces regular evolution
3. **Transparency**: All decisions on-chain, all treasury actions public
4. **Resilience**: No single point of failure, multiple layers of accountability
5. **Sustainability**: Self-funding via fees, deflationary tendency prevents inflation

### Comparison to Other Models

| Feature | Ëtrid | Bitcoin | Ethereum | Polkadot |
|---------|-------|---------|----------|----------|
| **Governance** | On-chain annual vote | Off-chain (BIPs) | Off-chain (EIPs) | On-chain (referenda) |
| **Treasury** | Autonomous | None | Foundation-controlled | Autonomous |
| **Inflation** | Voted (0-5%) | Fixed (halving) | Fixed (~1%) | Fixed (~10%) |
| **Participation** | >33% quorum | N/A | ~5% typical | ~10% typical |
| **Emergency Powers** | Directors (7/9) | None | Foundation | Council |

### The Path to Sovereignty

Ëtrid governance is designed to achieve **true decentralization** through:
- **Economic incentives** (rewards for participation)
- **Social accountability** (transparent decisions, elected directors)
- **Technical constraints** (protocol-enforced caps, slashing)
- **Regular rhythm** (annual Consensus Day prevents drift)

**The goal**: A network that governs itself, funds itself, and evolves without relying on any central authority.

---

## APPENDIX A: Consensus Day Checklist

**For Voters**:
- [ ] Review all proposals (published 7 days before Consensus Day)
- [ ] Lock ÉTR during Registration Phase (6 hours)
- [ ] Vote on all proposals during Voting Phase (12 hours)
- [ ] Claim participation rewards during Distribution Phase (1 hour)

**For Validators**:
- [ ] Signal participation during Registration Phase
- [ ] Vote on all proposals during Voting Phase
- [ ] Monitor network health during Minting Phase
- [ ] Verify reward distribution during Distribution Phase

**For Directors**:
- [ ] Review all proposals and publish recommendations
- [ ] Monitor quorum and voting during Voting Phase
- [ ] Execute approved budgets during Minting Phase
- [ ] Verify distribution accuracy during Distribution Phase

---

## APPENDIX B: Governance Formulas

### Quorum Calculation
```rust
fn quorum_met(proposal: &Proposal) -> bool {
    let community_votes = proposal.total_votes;
    let validator_votes = proposal.validator_count;

    (community_votes >= circulating_supply() * 0.33) &&
    (validator_votes >= active_validator_count() * 0.51)
}
```

### Approval Calculation
```rust
fn proposal_approved(proposal: &Proposal) -> bool {
    let yes_votes = proposal.yes_votes;
    let total_votes = proposal.yes_votes + proposal.no_votes; // Abstain doesn't count

    let threshold = match proposal.category {
        ProposalCategory::BudgetAllocation => 0.50,
        ProposalCategory::ParameterChange => 0.50,
        ProposalCategory::ProtocolUpgrade => 0.66,
        ProposalCategory::EmergencyAction => 0.66,
    };

    (yes_votes as f64 / total_votes as f64) >= threshold
}
```

---

## APPENDIX C: Further Reading

**Volume I**: Conceptual Architecture (Vision, Problem, Solution, Philosophy)
**Volume II**: Technical Specification (E³20, ASF, VMw, Runtime)

**Online Resources**:
- Governance Portal: governance.etrid.org
- Proposal Archive: proposals.etrid.org
- Treasury Dashboard: treasury.etrid.org

---

**End of Volume III**

**Ëtrid Ivory Papers Complete**

---

*"Governance is not a feature to be added. It is the fundamental architecture of coordination."*

---

**Document Status**: ACTIVE PROTOCOL SPECIFICATION
**Next Review**: Consensus Day 2026 (December 1, 2026)

---

## CLOSING REMARKS

To be quite frank, I have never considered the status quo an unequivocal consensus of a group of people.

Considering the multitude of variables that go into decision-making, it is difficult to fathom how what was, still is, and will always be.

This idea does not promote growth, prosperity, fairness, or decentralization.

It often feels forced upon you and remains unchallenged due to cultural reinforcement and other factors.

This stagnation in society has shifted power from those who could effect change to those who benefit from maintaining the status quo.

We are in a unique period in which power can be reclaimed by the powerless.

Exploitation of personal data can be stopped, and disintermediation of trusted third parties can become the norm.

Borders can be reimagined.

When liberties such as digital rights, data protection, and decentralized finance are on the line for our generation and the generations to come, I will fight until my last breath.

The Ëtrid FOODOS Project will be our vehicle in this fight — a free and open decentralized democracy of stakeholders.

By cutting the mental chains of reliance on a central intermediary and becoming self-sufficient stakeholders, we can achieve a brighter tomorrow.

**– Eoj Edred**
**Founder, Ëtrid FODDoS Project**

---

*"Provide a flare and guide the way, the future of tomorrow is decided today."*

**– Eoj Edred**

