# Pallet Treasury

Ëtrid protocol treasury fund management with multisig governance.

## Overview

The treasury pallet manages all protocol funds with a secure multisig governance structure. It receives funds from multiple sources (transaction fees, slashing, minting, cross-chain fees) and distributes them according to budget allocations approved by 9 Directors using a 6-of-9 multisig threshold.

## Key Features

- **Multisig Governance**: 9 Directors control treasury with 6-of-9 approval threshold
- **Multiple Funding Sources**: Transaction fees (50%), slashing (50%), Consensus Day minting, cross-chain fees (10%)
- **Budget Categories**: Development (40%), Marketing (20%), Operations (15%), Grants (15%), Emergency Reserve (10%)
- **Disbursement Workflow**: Propose → Approve (6/9) → Auto-execute
- **Emergency Withdrawals**: 7-of-9 approval threshold for emergency reserve access
- **Consensus Day Integration**: Budget allocations updated during annual governance event

## Architecture

### Treasury Account

- **PalletId**: `py/trsry`
- **Control**: 9 Directors via multisig
- **Assets**: ËTR (native), EDSC (stablecoin), bridge assets

### Funding Sources

1. **Transaction Fees**: 50% → Treasury, 50% burned
2. **Consensus Day Minting**: Approved budgets minted and deposited
3. **Validator Slashing**: 50% → Treasury, 50% burned
4. **Cross-Chain Fees**: 10% of all bridge fees → Treasury

### Budget Categories

| Category | Default % | Purpose |
|----------|-----------|---------|
| Development | 40% | Protocol development, research, infrastructure |
| Marketing | 20% | Community growth, partnerships, outreach |
| Operations | 15% | Team salaries, legal, administrative |
| Grants | 15% | Ecosystem grants, bounties, developer support |
| Emergency Reserve | 10% | Locked for critical protocol emergencies |

## Usage

### For Directors

#### Propose Disbursement

```rust
// Director proposes spending from Development budget
let result = Treasury::propose_disbursement(
    origin,
    BudgetCategory::Development,
    recipient_account,
    amount,
    b"Q4 2024 core protocol development".to_vec(),
);
```

#### Approve Disbursement

```rust
// Other directors approve (need 6 total approvals)
Treasury::approve_disbursement(origin, disbursement_id);
// Auto-executes when 6th approval received
```

#### Emergency Withdrawal

```rust
// Emergency withdrawal from reserve (requires 7/9 approvals)
Treasury::emergency_withdrawal(
    origin,
    recipient,
    amount,
    b"Critical security patch funding".to_vec(),
);
```

### For Other Pallets

#### Fund Treasury (Transaction Fees)

```rust
// Called by transaction fee handler
Treasury::receive_transaction_fees(fee_amount / 2)?; // 50% to treasury
```

#### Fund Treasury (Slashing)

```rust
// Called by pallet-validator-rewards
Treasury::receive_slashing_proceeds(slash_amount / 2)?; // 50% to treasury
```

#### Fund Treasury (Consensus Day)

```rust
// Called by pallet-consensus-day during Minting phase
Treasury::receive_consensus_day_minting(minted_amount)?;

// Allocate to budget categories
Treasury::allocate_to_categories(origin, total_amount)?;
```

#### Fund Treasury (Cross-Chain)

```rust
// Called by bridge pallets
Treasury::receive_cross_chain_fees(bridge_fee / 10)?; // 10% to treasury
```

## Disbursement Workflow

1. **Proposal**: Any director proposes disbursement from specific category
2. **Approval**: Other directors approve (proposer auto-approves, need 6 total)
3. **Execution**: Auto-executes when approval threshold (6/9) reached
4. **Settlement**: Funds transferred to recipient, category allocation decreased

### Emergency Withdrawals

- Higher threshold: 7-of-9 approvals required
- Only from Emergency Reserve category
- Used for critical protocol emergencies

## Storage Items

### TreasuryBalance
Total ËTR balance in treasury vault.

### EdscBalance
EDSC stablecoin balance.

### BudgetAllocationsStorage
Current budget allocation percentages (basis points).

### CategoryAllocations
Available budget per category (decreased on disbursement).

### Disbursements
All disbursement proposals (pending and historical).

### DirectorApprovals
List of directors who approved each disbursement.

### Directors
Current 9 director accounts (multisig controllers).

### EmergencyReserve
Locked emergency funds (7/9 threshold for access).

### DisbursementCount
Counter for unique disbursement IDs.

### FundingSourceTotals
Historical tracking of funds received per source.

## Integration Points

### pallet-consensus-day
- Calls `receive_consensus_day_minting()` during Minting phase
- Calls `allocate_to_categories()` to distribute budget
- Calls `set_budget_allocations()` to update percentages

### pallet-validator-rewards
- Calls `receive_slashing_proceeds()` when slashing validators

### Transaction Fee Handler
- Calls `receive_transaction_fees()` for 50% of all transaction fees

### Bridge Pallets
- Call `receive_cross_chain_fees()` for 10% of bridge fees

### pallet-multiasset-reserve
- Treasury may hold EDSC for stablecoin backing
- Integration for reserve management

## Events

- `FundsDeposited(source, amount)` - Funds received
- `DisbursementProposed(id, proposer, category, amount, recipient)` - New proposal
- `DisbursementApproved(id, director, approval_count)` - Director approval
- `DisbursementExecuted(id, recipient, amount)` - Payment executed
- `EmergencyWithdrawal(amount, recipient, approvals)` - Emergency reserve access
- `BudgetAllocationsUpdated(allocations)` - Budget percentages changed
- `DirectorAdded/Removed(account)` - Director membership changed
- `CategoryAllocationIncreased(category, amount)` - Budget allocated

## Errors

- `NotDirector` - Caller is not a registered director
- `DisbursementNotFound` - Invalid disbursement ID
- `AlreadyApproved` - Director already approved this proposal
- `InsufficientCategoryAllocation` - Category budget insufficient
- `InsufficientTreasuryBalance` - Treasury vault insufficient
- `InvalidBudgetAllocations` - Allocations don't sum to 100%
- `AlreadyExecuted` - Disbursement already processed
- `EmergencyThresholdNotMet` - Need 7/9 approvals for emergency
- `ApprovalThresholdNotMet` - Need 6/9 approvals for normal
- `DisbursementExpired` - Proposal expired (7 days)

## Constants

### DirectorCount
Number of directors in multisig (default: 9).

### ApprovalThreshold
Approvals needed for normal disbursements (default: 6).

### EmergencyThreshold
Approvals needed for emergency withdrawals (default: 7).

### ProposalExpiration
Proposal expiration in blocks (default: 7 days).

## Security Considerations

1. **Multisig Protection**: All disbursements require 6-of-9 director approvals
2. **Emergency Reserve**: Higher threshold (7/9) protects emergency funds
3. **Budget Limits**: Disbursements limited by category allocations
4. **Expiration**: Proposals expire after 7 days to prevent stale approvals
5. **Funding Source Tracking**: All deposits tracked by source for transparency

## Testing

```bash
# Run unit tests
cargo test -p pallet-treasury

# Run benchmarks
cargo build --release --features runtime-benchmarks
```

## License

GPL-3.0

## Authors

Eoj Edred <contact@etrid.io>
