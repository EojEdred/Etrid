# Decentralized Directors Pallet - Integration Guide

## Overview

This guide explains how to integrate the Decentralized Directors pallet into the Ëtrid runtime. The pallet manages governance validators with 128+ ËTR stake who have emergency powers over the network.

## Prerequisites

Before integrating this pallet, ensure you have:

1. **Staking Pallet**: `pallet-etrid-staking` must be included in your runtime
2. **Staking Types**: `peer-roles-staking-types` must be available
3. **Time Provider**: A `UnixTime` implementation (e.g., `pallet-timestamp`)

## Step 1: Add Dependency to Runtime Cargo.toml

```toml
[dependencies]
# ... other dependencies

# Decentralized Directors
pallet-decentralized-directors = {
    path = "../../11-peer-roles/decentralized-directors",
    default-features = false
}

# Required dependencies (if not already present)
peer-roles-staking-types = {
    path = "../../11-peer-roles/staking/types",
    default-features = false
}
pallet-etrid-staking = {
    path = "../../11-peer-roles/staking/pallet",
    default-features = false
}
pallet-timestamp = { workspace = true, default-features = false }

[features]
std = [
    # ... other std features
    "pallet-decentralized-directors/std",
    "peer-roles-staking-types/std",
    "pallet-etrid-staking/std",
    "pallet-timestamp/std",
]
```

## Step 2: Configure the Pallet in Runtime

Add the pallet configuration to your `runtime/src/lib.rs`:

```rust
use frame_support::parameter_types;

// Configure constants
parameter_types! {
    pub const MaxEmergencyProposals: u32 = 100;
    pub const EmergencyTimelock: u32 = 14_400; // 24 hours at 6s blocks
    pub const QuorumDirectors: u32 = 14; // 2/3 of 21 directors
}

// Implement the Config trait
impl pallet_decentralized_directors::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;

    // Use the staking pallet for role verification
    type StakingInterface = Staking;

    // Use timestamp pallet for time tracking
    type UnixTime = Timestamp;

    // Configuration constants
    type MaxEmergencyProposals = MaxEmergencyProposals;
    type EmergencyTimelock = EmergencyTimelock;
    type QuorumDirectors = QuorumDirectors;
}
```

## Step 3: Add to construct_runtime! Macro

```rust
construct_runtime!(
    pub enum Runtime
    {
        System: frame_system,
        Timestamp: pallet_timestamp,

        // Staking (required)
        Staking: pallet_etrid_staking,

        // Decentralized Directors
        DecentralizedDirectors: pallet_decentralized_directors,

        // ... other pallets
    }
);
```

## Step 4: Genesis Configuration (Optional)

If you want to initialize directors at genesis, you can create a genesis config. For now, directors register themselves after genesis through the `register_director()` extrinsic.

## Step 5: Runtime API (Optional)

If you want to expose director information via RPC, create a runtime API:

```rust
// In runtime/src/lib.rs

sp_api::impl_runtime_apis! {
    // ... other API implementations

    // Director query API
    impl self::DirectorApi<Block, AccountId> for Runtime {
        fn is_active_director(account: AccountId) -> bool {
            DecentralizedDirectors::is_active_director(&account)
        }

        fn active_director_count() -> u32 {
            DecentralizedDirectors::active_director_count()
        }

        fn get_director_profile(account: AccountId) -> Option<DirectorProfile<AccountId>> {
            DecentralizedDirectors::directors(account)
        }
    }
}
```

## Usage Examples

### 1. Register as Director

A director with 128+ ËTR stake and the `DecentralizedDirector` role can register:

```rust
// First, assign the director role via staking pallet
Staking::assign_role(
    Origin::signed(alice),
    4, // DecentralizedDirector role
    128_000_000_000_000_000_000_000, // 128 ËTR
)?;

// Then register as director
DecentralizedDirectors::register_director(Origin::signed(alice))?;
```

### 2. Propose Emergency Action

Only active directors can propose emergency actions:

```rust
use pallet_decentralized_directors::EmergencyAction;

// Fast-track a proposal
let action = EmergencyAction::FastTrack {
    proposal_id: 42
};

DecentralizedDirectors::propose_emergency_action(
    Origin::signed(alice),
    action,
    b"Critical security patch requires fast-tracking".to_vec(),
)?;
```

### 3. Vote on Emergency Action

Other directors vote on the proposal:

```rust
// Director votes yes
DecentralizedDirectors::vote_emergency_action(
    Origin::signed(bob),
    0, // proposal_id
    true, // approve
)?;

// Need 14 directors total to approve (2/3 quorum)
```

### 4. Execute Emergency Action

After 2/3 approval and 24-hour timelock:

```rust
// Anyone can execute once conditions are met
DecentralizedDirectors::execute_emergency_action(
    Origin::signed(anyone),
    0, // proposal_id
)?;
```

### 5. Veto Emergency Action (Governance)

Governance can veto during timelock period:

```rust
DecentralizedDirectors::veto_emergency_action(
    Origin::root(),
    0, // proposal_id
)?;
```

## Emergency Action Types

### 1. FastTrack
Fast-track a governance proposal (7 days → 24 hours):

```rust
EmergencyAction::FastTrack {
    proposal_id: 42
}
```

### 2. EmergencyHalt
Halt network operations immediately:

```rust
EmergencyAction::EmergencyHalt
```

### 3. ParameterChange
Change runtime parameters:

```rust
EmergencyAction::ParameterChange {
    param_name: BoundedVec::try_from(b"BlockGasLimit".to_vec()).unwrap(),
    param_value: 10_000_000,
}
```

### 4. TreasurySpend
Approve emergency treasury spending:

```rust
EmergencyAction::TreasurySpend {
    recipient: BoundedVec::try_from(alice.encode()).unwrap(),
    amount: 1_000_000_000_000_000_000_000, // 1000 ËTR
}
```

### 5. ValidatorSlash
Emergency slash a misbehaving validator:

```rust
EmergencyAction::ValidatorSlash {
    validator: BoundedVec::try_from(malicious_validator.encode()).unwrap(),
    amount: 64_000_000_000_000_000_000_000, // 64 ËTR
    reason: BoundedVec::try_from(b"Double signing detected".to_vec()).unwrap(),
}
```

## Integration with Other Pallets

### Governance Pallet Integration

To fully integrate emergency actions with governance:

```rust
// In execute_action() function, add real implementations:

match action {
    EmergencyAction::FastTrack { proposal_id } => {
        // Call governance pallet
        pallet_governance::Pallet::<T>::fast_track_proposal(proposal_id)?;
        Ok(())
    }
    // ... other actions
}
```

### Treasury Integration

For treasury spend actions:

```rust
EmergencyAction::TreasurySpend { recipient, amount } => {
    let recipient_account = T::AccountId::decode(&mut &recipient[..])?;
    pallet_treasury_etrid::Pallet::<T>::emergency_spend(
        recipient_account,
        amount,
    )?;
    Ok(())
}
```

### Staking Integration

For validator slashing:

```rust
EmergencyAction::ValidatorSlash { validator, amount, reason } => {
    let validator_account = T::AccountId::decode(&mut &validator[..])?;
    pallet_etrid_staking::Pallet::<T>::slash(
        frame_system::RawOrigin::Root.into(),
        validator_account,
        amount.saturated_into(),
    )?;
    Ok(())
}
```

## Security Considerations

### 1. Quorum Requirements
- **2/3 approval** required (14 out of 21 directors)
- Prevents small groups from abusing emergency powers

### 2. Timelock Protection
- **24-hour delay** before execution
- Gives governance time to veto if needed
- Exception: Truly critical actions may bypass timelock (requires special implementation)

### 3. Veto Mechanism
- Governance can veto any emergency action
- Prevents director overreach
- Maintains balance of power

### 4. Stake Requirements
- Directors must maintain **128+ ËTR** stake
- Economic incentive for honest behavior
- Slashing if misbehavior detected

### 5. Term Limits
- **365-day terms**
- Regular re-election on Consensus Day
- Prevents power consolidation

## Testing

Run the unit tests:

```bash
cd 11-peer-roles/decentralized-directors
cargo test --features std
```

Expected output:
```
test tests::test_register_director ... ok
test tests::test_propose_emergency_action ... ok
test tests::test_vote_emergency_action ... ok
test tests::test_execute_emergency_action_requires_quorum ... ok
test tests::test_execute_emergency_action_requires_timelock ... ok
test tests::test_veto_emergency_action ... ok
test tests::test_end_term ... ok
```

## Runtime Migration (If Upgrading)

If adding this pallet to an existing runtime, you may need a migration:

```rust
pub mod migrations {
    use super::*;

    pub struct InitializeDirectors<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for InitializeDirectors<T> {
        fn on_runtime_upgrade() -> Weight {
            // Set initial epoch
            CurrentEpoch::<T>::put(0u32);

            log::info!("Initialized Decentralized Directors pallet");
            T::DbWeight::get().writes(1)
        }
    }
}
```

## Monitoring and Observability

### Events to Monitor

1. **DirectorRegistered**: Track new directors joining
2. **EmergencyActionProposed**: Alert on emergency proposals
3. **EmergencyActionVoted**: Track voting progress
4. **EmergencyActionExecuted**: Alert on executed emergency actions
5. **EmergencyActionVetoed**: Track governance vetoes

### Metrics to Track

- Active director count
- Emergency proposal count
- Approval rate (yes votes / total votes)
- Average time to quorum
- Veto frequency

## Troubleshooting

### Problem: Cannot register as director

**Solutions:**
1. Verify you have `DecentralizedDirector` role: `Staking::role_of(account)`
2. Verify you have 128+ ËTR staked: `Staking::get_stake(account)`
3. Check if already registered: `DecentralizedDirectors::directors(account)`

### Problem: Emergency action not executing

**Solutions:**
1. Check quorum met: `proposal.votes_yes >= 14`
2. Verify timelock passed: `current_block >= proposal.executable_at`
3. Ensure not vetoed: `proposal.vetoed == false`
4. Ensure not already executed: `proposal.executed == false`

### Problem: Cannot vote on proposal

**Solutions:**
1. Verify you're an active director
2. Check you haven't already voted: `DirectorVotes::get(proposal_id, account)`
3. Verify proposal exists and is not executed/vetoed

## Future Enhancements

### Planned Features

1. **Emergency Action Categories**
   - Critical (no timelock)
   - High priority (12h timelock)
   - Normal (24h timelock)

2. **Delegation**
   - Directors can delegate voting power
   - Useful for vacations/emergencies

3. **Reputation System**
   - Track director voting patterns
   - Reward active participation
   - Penalize absence

4. **Multi-Signature Integration**
   - Require multi-sig for highest-risk actions
   - Additional security layer

## Support

For issues or questions:
- GitHub: https://github.com/etrid/etrid
- Documentation: https://docs.etrid.org
- Discord: https://discord.gg/etrid

## License

MIT License - See LICENSE file for details.
