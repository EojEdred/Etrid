# Pallet Treasury - Quick Reference Card

## Overview
- **PalletId**: `py/trsry`
- **Control**: 9 Directors, 6-of-9 multisig
- **Emergency**: 7-of-9 threshold

## Budget Allocations (Default)
```
Development:       40% (4000 bps)
Marketing:         20% (2000 bps)
Operations:        15% (1500 bps)
Grants:            15% (1500 bps)
Emergency Reserve: 10% (1000 bps)
```

## Funding Sources
```
Transaction Fees:      50% → Treasury, 50% burned
Consensus Day Minting: 100% → Treasury
Validator Slashing:    50% → Treasury, 50% burned
Cross-Chain Fees:      10% → Treasury, 90% to relayers
```

## Extrinsics

### For Directors

#### Propose Spending
```rust
Treasury::propose_disbursement(
    origin,
    BudgetCategory::Development,
    recipient_account,
    1_000_000_000_000u128,  // 1M ËTR
    b"Description (max 256 bytes)".to_vec()
)
```

#### Approve Proposal
```rust
Treasury::approve_disbursement(origin, disbursement_id)
// Auto-executes when 6th approval received
```

#### Emergency Withdrawal
```rust
Treasury::emergency_withdrawal(
    origin,
    recipient_account,
    500_000_000_000u128,  // 500K ËTR
    b"Emergency description".to_vec()
)
// Requires 7 approvals
```

### For Governance

#### Update Budget Allocations
```rust
Treasury::set_budget_allocations(
    origin,
    BudgetAllocations {
        development_bps: 4000,
        marketing_bps: 2000,
        operations_bps: 1500,
        grants_bps: 1500,
        emergency_reserve_bps: 1000,
    }
)
```

#### Add/Remove Director
```rust
Treasury::add_director(origin, new_director_account)
Treasury::remove_director(origin, director_account)
```

## Integration Functions

### From Transaction Fee Handler
```rust
Treasury::receive_transaction_fees(fee_amount / 2)?;
```

### From Validator Slashing
```rust
Treasury::receive_slashing_proceeds(slash_amount / 2)?;
```

### From Consensus Day
```rust
Treasury::receive_consensus_day_minting(minted_amount)?;
Treasury::allocate_to_categories(origin, total_amount)?;
```

### From Bridge Pallets
```rust
Treasury::receive_cross_chain_fees(bridge_fee / 10)?;
```

## Storage Queries

```rust
// Get treasury balance
let balance = Treasury::treasury_balance();

// Get EDSC balance
let edsc = Treasury::edsc_balance();

// Get budget allocations
let allocations = Treasury::budget_allocations();

// Get category budget
let dev_budget = Treasury::category_allocation(BudgetCategory::Development);

// Get directors list
let directors = Treasury::directors();

// Get disbursement details
let disbursement = Treasury::disbursement(id);

// Get approvals
let approvals = Treasury::approvals(id);

// Get funding totals
let tx_fees_total = Treasury::funding_source_totals(FundingSource::TransactionFees);
```

## Key Events

```rust
FundsDeposited(source, amount)
DisbursementProposed(id, proposer, category, amount, recipient)
DisbursementApproved(id, director, approval_count)
DisbursementExecuted(id, recipient, amount)
EmergencyWithdrawal(amount, recipient, approvals)
BudgetAllocationsUpdated(allocations)
```

## Common Errors

```
NotDirector                     - Caller not a director
DisbursementNotFound            - Invalid disbursement ID
AlreadyApproved                 - Director already approved
InsufficientCategoryAllocation  - Category budget exceeded
ApprovalThresholdNotMet         - Need 6/9 approvals
EmergencyThresholdNotMet        - Need 7/9 approvals
DisbursementExpired             - Proposal expired (7 days)
```

## Workflow

### Normal Disbursement
1. Director proposes → Auto-approves (1/6)
2. 5 other directors approve
3. Auto-executes on 6th approval
4. Funds transferred
5. Category budget decreases

### Emergency Withdrawal
1. Director proposes emergency → Auto-approves (1/7)
2. 6 other directors approve
3. Auto-executes on 7th approval
4. Funds withdrawn from Emergency Reserve

## Configuration

```rust
parameter_types! {
    pub const TreasuryDirectorCount: u8 = 9;
    pub const TreasuryApprovalThreshold: u8 = 6;
    pub const TreasuryEmergencyThreshold: u8 = 7;
    pub const TreasuryProposalExpiration: BlockNumber = 7 * DAYS;
}

impl pallet_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type DirectorCount = TreasuryDirectorCount;
    type ApprovalThreshold = TreasuryApprovalThreshold;
    type EmergencyThreshold = TreasuryEmergencyThreshold;
    type ProposalExpiration = TreasuryProposalExpiration;
}
```

## Security Checklist

- [ ] 9 director accounts generated securely
- [ ] Director keys stored in hardware wallets
- [ ] Multisig procedures documented
- [ ] Budget limits configured
- [ ] Monitoring alerts configured
- [ ] Emergency procedures documented
- [ ] Transaction fee routing verified
- [ ] Slashing integration tested
- [ ] Consensus Day integration verified

## File Locations

```
/Users/macbook/Desktop/etrid/src/pallets/pallet-treasury/
├── Cargo.toml
├── README.md
├── INTEGRATION_GUIDE.md
├── IMPLEMENTATION_SUMMARY.md
├── QUICK_REFERENCE.md (this file)
└── src/
    ├── lib.rs
    └── migrations.rs
```

---

**Quick Help**: See README.md for detailed documentation or INTEGRATION_GUIDE.md for step-by-step integration.
