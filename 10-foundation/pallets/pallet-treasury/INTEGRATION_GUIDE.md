# Pallet Treasury Integration Guide

Complete integration instructions for adding pallet-treasury to the Ëtrid runtime.

## 1. Add to Runtime Cargo.toml

Add the dependency to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-treasury = { path = "../pallets/pallet-treasury", default-features = false }

[features]
std = [
    # ... other pallets
    "pallet-treasury/std",
]
```

## 2. Configure in Runtime

Add to your `runtime/lib.rs`:

```rust
/// Treasury configuration constants
parameter_types! {
    pub const TreasuryDirectorCount: u8 = 9;
    pub const TreasuryApprovalThreshold: u8 = 6;
    pub const TreasuryEmergencyThreshold: u8 = 7;
    pub const TreasuryProposalExpiration: BlockNumber = 7 * DAYS; // 7 days
}

impl pallet_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type DirectorCount = TreasuryDirectorCount;
    type ApprovalThreshold = TreasuryApprovalThreshold;
    type EmergencyThreshold = TreasuryEmergencyThreshold;
    type ProposalExpiration = TreasuryProposalExpiration;
}

// Add to construct_runtime! macro
construct_runtime!(
    pub enum Runtime {
        // ... other pallets
        Treasury: pallet_treasury,
    }
);
```

## 3. Genesis Configuration

Add to `chain_spec.rs`:

```rust
use pallet_treasury::{BudgetAllocations, GenesisConfig as TreasuryConfig};

// In your genesis config function
fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AuraId, GrandpaId)>,
    directors: Vec<AccountId>,  // 9 director accounts
    endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
    GenesisConfig {
        // ... other pallets
        treasury: TreasuryConfig {
            directors: directors.clone(),
            budget_allocations: BudgetAllocations::default_allocations(),
        },
    }
}

// Example director accounts (replace with real accounts)
pub fn get_initial_directors() -> Vec<AccountId> {
    vec![
        // Director 1-9 AccountIds
        hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"].into(),
        hex!["8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"].into(),
        hex!["90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22"].into(),
        hex!["306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20"].into(),
        hex!["e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e"].into(),
        hex!["1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c"].into(),
        hex!["d17c2d7823ebf260fd138f2d7e27d114c0145d968b5ff5006125f2414fadae69"].into(),
        hex!["439660b36c6c03afafca027b910b4fecf99801834c62a5e6006f27d978de234f"].into(),
        hex!["5e639b43e0052c47447dac87d6fd2b6ec50bdd4d0f614e4299c665249bbd09d9"].into(),
    ]
}
```

## 4. Transaction Fee Integration

Integrate with the transaction fee handler to send 50% of fees to treasury:

```rust
// In your runtime's transaction payment configuration
use pallet_treasury::Pallet as Treasury;

pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
    fn on_nonzero_unbalanced(amount: NegativeImbalance) {
        let numeric_amount = amount.peek();

        // 50% to treasury, 50% burned
        let treasury_amount = numeric_amount / 2;
        let burn_amount = numeric_amount - treasury_amount;

        // Send to treasury
        let _ = Treasury::<Runtime>::receive_transaction_fees(treasury_amount.into());

        // Burn the rest
        drop(amount);
    }
}

// Use in TransactionPayment config
impl pallet_transaction_payment::Config for Runtime {
    // ... other config
    type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees>;
}
```

## 5. Consensus Day Integration

Add treasury funding during Consensus Day minting phase:

```rust
// In pallet-consensus-day minting logic
impl<T: Config> Pallet<T> {
    fn execute_minting_phase() -> DispatchResult {
        let approved_budget = Self::get_approved_budget();

        // Mint new ËTR
        let minted_amount = Self::mint_etr(approved_budget)?;

        // Send to treasury
        pallet_treasury::Pallet::<T>::receive_consensus_day_minting(minted_amount)?;

        // Allocate to budget categories
        pallet_treasury::Pallet::<T>::allocate_to_categories(
            frame_system::RawOrigin::Root.into(),
            minted_amount,
        )?;

        Ok(())
    }
}
```

## 6. Validator Slashing Integration

Add treasury funding when slashing validators:

```rust
// In pallet-validator-rewards slashing logic
impl<T: Config> Pallet<T> {
    pub fn slash_validator(
        session_account: &T::AccountId,
        slash_amount: BalanceOf<T>,
    ) -> DispatchResult {
        // Slash from validator stake
        T::Currency::slash(session_account, slash_amount);

        // 50% to treasury, 50% burned
        let treasury_amount = slash_amount / 2u128.into();

        pallet_treasury::Pallet::<T>::receive_slashing_proceeds(treasury_amount)?;

        Self::deposit_event(Event::ValidatorSlashed(
            session_account.clone(),
            slash_amount,
        ));

        Ok(())
    }
}
```

## 7. Bridge Integration

Add treasury funding from cross-chain fees:

```rust
// In bridge pallet fee handling
impl<T: Config> Pallet<T> {
    fn process_bridge_fee(total_fee: BalanceOf<T>) -> DispatchResult {
        // 10% to treasury, 90% to relayers/validators
        let treasury_amount = total_fee / 10u128.into();
        let relayer_amount = total_fee - treasury_amount;

        // Send to treasury
        pallet_treasury::Pallet::<T>::receive_cross_chain_fees(treasury_amount)?;

        // Distribute to relayers
        Self::distribute_to_relayers(relayer_amount)?;

        Ok(())
    }
}
```

## 8. Governance Integration

Add extrinsics to governance whitelist for Consensus Day:

```rust
// In governance configuration
impl pallet_consensus_day::Config for Runtime {
    type GovernanceWhitelist = GovernanceWhitelist;
    // ... other config
}

pub struct GovernanceWhitelist;
impl Contains<Call> for GovernanceWhitelist {
    fn contains(call: &Call) -> bool {
        matches!(
            call,
            Call::Treasury(pallet_treasury::Call::set_budget_allocations { .. }) |
            Call::Treasury(pallet_treasury::Call::allocate_to_categories { .. }) |
            Call::Treasury(pallet_treasury::Call::add_director { .. }) |
            Call::Treasury(pallet_treasury::Call::remove_director { .. })
        )
    }
}
```

## 9. Director Management

Commands for managing directors (governance only):

```rust
// Add director
Treasury::add_director(
    frame_system::RawOrigin::Root.into(),
    new_director_account,
)?;

// Remove director
Treasury::remove_director(
    frame_system::RawOrigin::Root.into(),
    director_to_remove,
)?;
```

## 10. Monitoring and Queries

Query treasury state:

```rust
// Get treasury balance
let balance = Treasury::treasury_balance();

// Get EDSC balance
let edsc = Treasury::edsc_balance();

// Get budget allocations
let allocations = Treasury::budget_allocations();

// Get category allocation
let dev_budget = Treasury::category_allocation(BudgetCategory::Development);

// Get directors
let directors = Treasury::directors();

// Get disbursement details
let disbursement = Treasury::disbursement(disbursement_id);

// Get approvals for disbursement
let approvals = Treasury::approvals(disbursement_id);

// Get funding source totals
let tx_fee_total = Treasury::funding_source_totals(FundingSource::TransactionFees);
```

## 11. RPC Integration

Add RPC methods for treasury queries (optional):

```rust
// In runtime API definition
pub trait TreasuryApi<BlockHash> {
    fn treasury_balance() -> Balance;
    fn category_allocations() -> Vec<(BudgetCategory, Balance)>;
    fn pending_disbursements() -> Vec<u64>;
    fn director_list() -> Vec<AccountId>;
}
```

## 12. Testing

Run integration tests:

```bash
# Build runtime with treasury
cargo build --release

# Run treasury tests
cargo test -p pallet-treasury

# Run runtime integration tests
cargo test -p etrid-runtime
```

## 13. Deployment Checklist

Before mainnet deployment:

- [ ] Configure 9 director accounts (secure key management)
- [ ] Set initial budget allocations
- [ ] Test transaction fee routing (50% to treasury)
- [ ] Test Consensus Day minting integration
- [ ] Test slashing proceeds routing
- [ ] Test bridge fee routing
- [ ] Test disbursement workflow (propose, approve, execute)
- [ ] Test emergency withdrawal (7/9 threshold)
- [ ] Verify multisig security (6/9 for normal, 7/9 for emergency)
- [ ] Document director key management procedures
- [ ] Set up monitoring for treasury balance
- [ ] Configure alerts for large disbursements

## 14. Security Considerations

1. **Director Key Security**: Use hardware wallets or multisig setups for director keys
2. **Proposal Review**: All disbursements should be reviewed by multiple directors
3. **Emergency Procedures**: Document emergency withdrawal procedures
4. **Balance Monitoring**: Monitor treasury balance and funding sources
5. **Audit Trail**: All disbursements are permanently recorded on-chain
6. **Budget Limits**: Enforce category budget limits to prevent overspending

## 15. Common Operations

### Propose Development Funding

```bash
# Director proposes development spending
treasury.propose_disbursement(
    BudgetCategory::Development,
    recipient_account,
    1000000000000,  # 1,000,000 ËTR (12 decimals)
    "Q1 2025 protocol development and testing"
)
```

### Approve Proposal

```bash
# Other directors approve (need 6 total)
treasury.approve_disbursement(disbursement_id)
```

### Emergency Withdrawal

```bash
# Emergency withdrawal (requires 7/9)
treasury.emergency_withdrawal(
    recipient_account,
    500000000000,  # 500,000 ËTR
    "Critical security patch deployment"
)
```

### Update Budget Allocations (Consensus Day)

```bash
# During Consensus Day, update budget allocations
treasury.set_budget_allocations({
    development_bps: 4500,      # Increase to 45%
    marketing_bps: 1500,        # Decrease to 15%
    operations_bps: 1500,       # 15%
    grants_bps: 1500,           # 15%
    emergency_reserve_bps: 1000 # 10%
})
```

## Support

For integration assistance, contact the Ëtrid development team or submit issues to the GitHub repository.
