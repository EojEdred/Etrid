# Consensus Day Treasury Integration - Example Flow

## Complete End-to-End Example

This document demonstrates a complete Consensus Day cycle with treasury integration.

## Scenario Setup

**Date**: December 1st, 12:00 AM PST
**Circulating Supply**: 2,500,000,000 ËTR
**Max Inflation (5%)**: 125,000,000 ËTR
**Active Validators**: 21
**Community Participants**: 1,500 stakers

---

## Phase 1: Registration (6 hours)

### Hour 0-6: Proposal Submissions and Stake Locking

#### Proposal #1: Block Explorer Development

**Proposer**: Alice (Developer)
**Category**: BudgetAllocation
**Budget Category**: Infrastructure
**Amount**: 50,000 ËTR

```rust
api.tx.consensusDay.submitProposal(
    "Build open-source block explorer with advanced analytics",
    ProposalCategory::BudgetAllocation,
    50_000_000_000_000_000_000_000, // 50k ETR
    Some(BudgetCategory::Infrastructure)
).signAndSend(alice);
```

**Events Emitted**:
```
ProposalSubmitted(0, alice, BudgetAllocation, 10_000 ETR)
BudgetAllocationCategorized(0, Infrastructure, 50_000 ETR)
```

#### Proposal #2: Security Audit

**Proposer**: Bob (Security Firm)
**Category**: BudgetAllocation
**Budget Category**: Security
**Amount**: 100,000 ËTR

```rust
api.tx.consensusDay.submitProposal(
    "Comprehensive security audit of runtime pallets",
    ProposalCategory::BudgetAllocation,
    100_000_000_000_000_000_000_000, // 100k ETR
    Some(BudgetCategory::Security)
).signAndSend(bob);
```

**Events Emitted**:
```
ProposalSubmitted(1, bob, BudgetAllocation, 10_000 ETR)
BudgetAllocationCategorized(1, Security, 100_000 ETR)
```

#### Proposal #3: Marketing Campaign

**Proposer**: Carol (Marketing Agency)
**Category**: BudgetAllocation
**Budget Category**: Marketing
**Amount**: 75,000 ËTR

```rust
api.tx.consensusDay.submitProposal(
    "Q1 marketing campaign for exchange listings",
    ProposalCategory::BudgetAllocation,
    75_000_000_000_000_000_000_000, // 75k ETR
    Some(BudgetCategory::Marketing)
).signAndSend(carol);
```

**Events Emitted**:
```
ProposalSubmitted(2, carol, BudgetAllocation, 10_000 ETR)
BudgetAllocationCategorized(2, Marketing, 75_000 ETR)
```

#### Proposal #4: Community Grants Program

**Proposer**: Dave (Community Lead)
**Category**: BudgetAllocation
**Budget Category**: CommunityGrants
**Amount**: 150,000 ËTR

```rust
api.tx.consensusDay.submitProposal(
    "Quarterly grants program for dApp developers",
    ProposalCategory::BudgetAllocation,
    150_000_000_000_000_000_000_000, // 150k ETR
    Some(BudgetCategory::CommunityGrants)
).signAndSend(dave);
```

**Events Emitted**:
```
ProposalSubmitted(3, dave, BudgetAllocation, 10_000 ETR)
BudgetAllocationCategorized(3, CommunityGrants, 150_000 ETR)
```

#### Proposal #5: Inflation Rate Adjustment

**Proposer**: Eve (Validator)
**Category**: InflationRate
**Requested Rate**: 3% (down from 4%)

```rust
api.tx.consensusDay.submitProposal(
    "Reduce annual inflation to 3% for sustainable growth",
    ProposalCategory::InflationRate,
    0, // No budget request
    None // No category for inflation proposals
).signAndSend(eve);
```

#### Stake Locking

**1,500 participants** lock stakes for voting power:

```rust
// Example: Frank locks 10,000 ETR
api.tx.consensusDay.lockStakeForVoting(
    10_000_000_000_000_000_000_000 // 10k ETR
).signAndSend(frank);

// Voting power calculated:
// Base: 10,000 ETR
// Duration bonus: +5% (staked for 6 months)
// History bonus: +4% (participated in 2 previous Consensus Days)
// Total voting power: 10,000 × 1.05 × 1.04 = 10,920
```

**Total Voting Power Registered**: 15,000,000 (1,500 participants × avg 10k ETR)

---

## Phase 2: Voting (12 hours)

### Hour 6-18: Community Voting

#### Vote Distribution

**Proposal #1 (Block Explorer - 50k ETR)**
- Yes: 8,500,000 voting power (56.7%)
- No: 3,200,000 voting power (21.3%)
- Abstain: 3,300,000 voting power (22.0%)
- Validators: 18/21 voted (85.7%)
- **Result**: ✅ APPROVED (>50% threshold, both quorums met)

**Proposal #2 (Security Audit - 100k ETR)**
- Yes: 11,200,000 voting power (74.7%)
- No: 2,100,000 voting power (14.0%)
- Abstain: 1,700,000 voting power (11.3%)
- Validators: 20/21 voted (95.2%)
- **Result**: ✅ APPROVED (>50% threshold, both quorums met)

**Proposal #3 (Marketing - 75k ETR)**
- Yes: 6,200,000 voting power (41.3%)
- No: 7,500,000 voting power (50.0%)
- Abstain: 1,300,000 voting power (8.7%)
- Validators: 19/21 voted (90.5%)
- **Result**: ❌ REJECTED (failed approval threshold)

**Proposal #4 (Community Grants - 150k ETR)**
- Yes: 9,800,000 voting power (65.3%)
- No: 3,500,000 voting power (23.3%)
- Abstain: 1,700,000 voting power (11.3%)
- Validators: 21/21 voted (100%)
- **Result**: ✅ APPROVED (>50% threshold, both quorums met)

**Proposal #5 (Inflation Rate 3%)**
- Yes: 10,100,000 voting power (67.3%)
- No: 4,200,000 voting power (28.0%)
- Abstain: 700,000 voting power (4.7%)
- Validators: 20/21 voted (95.2%)
- **Result**: ✅ APPROVED (>50% threshold, both quorums met)

#### Finalize Voting Phase

```rust
api.tx.consensusDay.advancePhase().signAndSend(anyone);
```

**Events Emitted**:
```
ProposalApproved(0, 8_500_000, 3_200_000)  // Block Explorer
ProposalApproved(1, 11_200_000, 2_100_000) // Security Audit
ProposalRejected(2, 6_200_000, 7_500_000)  // Marketing (rejected)
ProposalApproved(3, 9_800_000, 3_500_000)  // Community Grants
ProposalApproved(4, 10_100_000, 4_200_000) // Inflation Rate
PhaseAdvanced(Voting, Minting, block_12345)
```

---

## Phase 3: Minting (3 hours)

### Hour 18-21: Budget Execution and Treasury Funding

#### Automatic Execution Triggered

```rust
// Called automatically by advance_phase() or manually:
api.tx.consensusDay.executeApprovedBudgets().signAndSend(anyone);
```

#### Minting Calculations

**Circulating Supply**: 2,500,000,000 ËTR
**Max Inflation (5%)**: 125,000,000 ËTR
**Approved Budget Proposals**:
- Proposal #0: 50,000 ËTR (Infrastructure)
- Proposal #1: 100,000 ËTR (Security)
- Proposal #3: 150,000 ËTR (Community Grants)

**Total Approved**: 300,000 ËTR
**Within Cap**: ✅ Yes (0.012% of supply, well under 5%)

#### Minting Process

```rust
// Internal execution flow:

// 1. Calculate max mintable
let max_mintable = 2_500_000_000 * 5% = 125_000_000 ETR;

// 2. Process approved proposals
let mut total_minted = 0;
let mut categories = Vec::new();

// Proposal #0: Block Explorer
total_minted += 50_000;
categories.push((BudgetCategory::Infrastructure, 50_000));
TreasuryAllocations[Infrastructure] = 50_000;
emit BudgetMinted(0, 50_000);
emit TreasuryFunded(Infrastructure, 50_000);

// Proposal #1: Security Audit
total_minted += 100_000;
categories.push((BudgetCategory::Security, 100_000));
TreasuryAllocations[Security] = 100_000;
emit BudgetMinted(1, 100_000);
emit TreasuryFunded(Security, 100_000);

// Proposal #3: Community Grants
total_minted += 150_000;
categories.push((BudgetCategory::CommunityGrants, 150_000));
TreasuryAllocations[CommunityGrants] = 150_000;
emit BudgetMinted(3, 150_000);
emit TreasuryFunded(CommunityGrants, 150_000);

// 3. Transfer to treasury
T::Treasury::fund_treasury(
    &pallet_account,
    300_000 ETR,
    [
        (Infrastructure, 50_000),
        (Security, 100_000),
        (CommunityGrants, 150_000)
    ]
);

emit TreasuryTransferCompleted(300_000, 3);

// 4. Apply inflation rate
InflationRate::put(300); // 3% (from approved proposal #4)
emit InflationRateSet(400, 300); // Old: 4%, New: 3%
```

#### Events Emitted

```
BudgetMinted(0, 50_000 ETR)
TreasuryFunded(Infrastructure, 50_000 ETR)
BudgetMinted(1, 100_000 ETR)
TreasuryFunded(Security, 100_000 ETR)
BudgetMinted(3, 150_000 ETR)
TreasuryFunded(CommunityGrants, 150_000 ETR)
TreasuryTransferCompleted(300_000 ETR, 3 categories)
InflationRateSet(400, 300)
```

#### Treasury State After Minting

```
Treasury Balance: 875,300,000 ETR
  Genesis: 875,000,000 ETR
  Consensus Day Minting: 300,000 ETR

Category Allocations:
  Infrastructure: 50,000 ETR
  Security: 100,000 ETR
  CommunityGrants: 150,000 ETR
  Marketing: 0 ETR (proposal rejected)
```

---

## Phase 4: Distribution (1 hour)

### Hour 21-22: Participation Rewards

#### Reward Calculation

**Total Minted**: 300,000 ËTR
**Participation Pool (1%)**: 3,000 ËTR
**Total Voting Power**: 15,000,000

#### Individual Rewards

**Frank (voted on all 5 proposals)**:
- Voting power: 10,920
- Base reward: (10,920 / 15,000,000) × 3,000 = 2.184 ETR
- Completeness bonus: 2.184 × 1.2 = 2.621 ETR
- **Final reward**: 2.621 ETR

**Grace (voted on 3 proposals)**:
- Voting power: 5,200
- Base reward: (5,200 / 15,000,000) × 3,000 = 1.04 ETR
- No completeness bonus
- **Final reward**: 1.04 ETR

#### Claim Rewards

```rust
api.tx.consensusDay.claimParticipationReward().signAndSend(frank);
```

**Events Emitted**:
```
ParticipationRewardCalculated(frank, 2.621 ETR)
ParticipationRewardClaimed(frank, 2.621 ETR)
```

#### Director Elections

Top 9 candidates by vote count elected:
```
DirectorsElected([
    director1, director2, director3, director4, director5,
    director6, director7, director8, director9
])
```

#### Complete Consensus Day

```rust
api.tx.consensusDay.advancePhase().signAndSend(anyone);
```

**Events Emitted**:
```
ConsensusDayCompleted(2025, 300_000 ETR)
PhaseAdvanced(Distribution, Inactive, block_12500)
```

---

## Post-Consensus Day: Director Management

### Directors Review Treasury Allocations

Query current allocations:

```rust
// Infrastructure: 50,000 ETR
let infra_budget = api.query.consensusDay.treasuryAllocations(
    BudgetCategory::Infrastructure
);

// Security: 100,000 ETR
let security_budget = api.query.consensusDay.treasuryAllocations(
    BudgetCategory::Security
);

// Community Grants: 150,000 ETR
let grants_budget = api.query.consensusDay.treasuryAllocations(
    BudgetCategory::CommunityGrants
);
```

### Directors Approve Disbursements

#### Milestone 1: Block Explorer (Alice)

**Directors vote to release 20,000 ETR for Phase 1 completion**:

```rust
// Via multisig (5-of-9 directors)
api.tx.multisig.asMulti(
    5, // threshold
    [director2, director3, director4, director5], // other signatories
    api.tx.treasury.approveProposal(
        alice, // beneficiary
        20_000_000_000_000_000_000_000 // 20k ETR
    )
).signAndSend(director1);
```

#### Milestone 2: Security Audit (Bob)

**Directors approve full payment upon completion**:

```rust
api.tx.multisig.asMulti(
    5,
    [director1, director3, director4, director5],
    api.tx.treasury.approveProposal(
        bob,
        100_000_000_000_000_000_000_000 // 100k ETR
    )
).signAndSend(director2);
```

---

## Summary of Complete Flow

### Consensus Day Results

| Metric | Value |
|--------|-------|
| **Proposals Submitted** | 5 |
| **Proposals Approved** | 4 |
| **Total Minted** | 300,000 ETR (0.012% of supply) |
| **Treasury Funded** | 300,000 ETR |
| **Categories Funded** | 3 (Infrastructure, Security, CommunityGrants) |
| **Participation Rewards** | 3,000 ETR (1% of minted) |
| **Active Voters** | 1,500 |
| **Validator Participation** | 100% (21/21) |
| **New Inflation Rate** | 3% (down from 4%) |
| **Directors Elected** | 9 |

### Treasury State

| Category | Allocated | Status |
|----------|-----------|--------|
| Infrastructure | 50,000 ETR | 20k disbursed, 30k pending milestones |
| Security | 100,000 ETR | Fully disbursed |
| CommunityGrants | 150,000 ETR | Awaiting director review |
| Marketing | 0 ETR | Proposal rejected |
| **Total** | **300,000 ETR** | **Directors managing disbursements** |

### Key Achievements

1. ✅ **Democratic Budget Approval**: Community voted on all allocations
2. ✅ **Inflation Cap Respected**: 0.012% well under 5% maximum
3. ✅ **Category Transparency**: Clear breakdown of fund purposes
4. ✅ **Director Oversight**: Elected directors manage disbursements
5. ✅ **Participation Incentives**: 1,500 voters rewarded
6. ✅ **Inflation Rate Adjusted**: Reduced to 3% for sustainability

---

## Developer Takeaways

### For Proposers

**Successful proposals had**:
- Clear deliverables and milestones
- Reasonable budget requests
- Strong community support
- Active proposer engagement

**Tips**:
- Discuss proposals in governance forum first
- Break large budgets into milestones
- Specify appropriate BudgetCategory
- Engage with validators for quorum

### For Voters

**Participation rewards**:
- Proportional to voting power and activity
- 20% bonus for voting on all proposals
- Builds participation history for future bonuses

**Tips**:
- Lock stakes early in Registration phase
- Vote on all proposals for completeness bonus
- Consider long-term staking for duration bonus

### For Directors

**Responsibilities**:
- Review approved budgets by category
- Approve milestone-based disbursements
- Monitor treasury balance and utilization
- Report to community on spending

**Tools**:
- Query `treasuryAllocations` for category budgets
- Use multisig for disbursement approvals
- Track `totalTreasuryFunded` for annual metrics

---

**This example demonstrates the complete integration of Consensus Day democratic governance with ongoing treasury management, creating a transparent, accountable, and community-driven financial system for Ëtrid.**
