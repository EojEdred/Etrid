# Director Election Pallet - Integration Guide

Complete guide for integrating `pallet-director-election` into the Ëtrid Primearc Core Chain runtime.

## Prerequisites

Before integrating, ensure you have:

1. **Staking Pallet**: `peer-roles-staking` pallet configured
2. **Runtime**: Primearc Core Chain runtime at `runtime/primearc-core-chain/`
3. **Block Time**: 6-second block time configured
4. **Roles**: DecentralizedDirector role properly configured

## Integration Steps

### Step 1: Add Dependency to Runtime Cargo.toml

Edit `/Users/macbook/Desktop/etrid/runtime/primearc-core-chain/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies

# Director Election
pallet-director-election = {
    path = "../../12-consensus-day/pallet-director-election",
    default-features = false
}

[features]
std = [
    # ... existing std features
    "pallet-director-election/std",
]
```

### Step 2: Configure Pallet in Runtime

Edit `/Users/macbook/Desktop/etrid/runtime/primearc-core-chain/src/lib.rs`:

#### 2.1 Add Parameter Types

```rust
// Near top of file with other parameter_types
parameter_types! {
    // Director Election Configuration
    // Block durations assume 6-second blocks

    // Governance period: 335 days = 335 × 24 × 3600 / 6 = 5,040,000 blocks
    pub const GovernancePeriodBlocks: BlockNumber = 5_040_000;

    // Nomination period: 30 days = 30 × 24 × 3600 / 6 = 432,000 blocks
    pub const NominationPeriodBlocks: BlockNumber = 432_000;

    // Voting period: 7 days = 7 × 24 × 3600 / 6 = 100,800 blocks
    pub const VotingPeriodBlocks: BlockNumber = 100_800;

    // Number of directors to elect
    pub const NumDirectorsToElect: u32 = 21;

    // Minimum candidate stake: 128 ËTR (18 decimals)
    pub const MinCandidateStake: u128 = 128_000_000_000_000_000_000_000;
}
```

#### 2.2 Implement Config Trait

```rust
// After other pallet configs
impl pallet_director_election::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type StakingInterface = PeerRolesStaking; // Your staking pallet
    type GovernancePeriodBlocks = GovernancePeriodBlocks;
    type NominationPeriodBlocks = NominationPeriodBlocks;
    type VotingPeriodBlocks = VotingPeriodBlocks;
    type NumDirectorsToElect = NumDirectorsToElect;
    type MinCandidateStake = MinCandidateStake;
}
```

#### 2.3 Add to construct_runtime! Macro

```rust
construct_runtime!(
    pub struct Runtime {
        // ... existing pallets
        System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,

        // Staking (must come before DirectorElection)
        PeerRolesStaking: peer_roles_staking,

        // Director Election
        DirectorElection: pallet_director_election,

        // ... other pallets
    }
);
```

### Step 3: Genesis Configuration

Edit `/Users/macbook/Desktop/etrid/runtime/primearc-core-chain/src/genesis.rs` (or where genesis is configured):

```rust
use pallet_director_election::GenesisConfig as DirectorElectionConfig;

pub fn primearc-core-chain_genesis(
    // ... existing parameters
) -> RuntimeGenesisConfig {
    RuntimeGenesisConfig {
        // ... existing pallets

        director_election: DirectorElectionConfig {
            // Set first Consensus Day to occur 365 days after genesis
            // 365 days × 24 hours × 3600 seconds / 6 seconds per block = 5,572,800 blocks
            phantom: Default::default(),
        },

        // ... other pallets
    }
}
```

### Step 4: Update Runtime Version

Increment the runtime version since this is a significant change:

```rust
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("primearc-core-chain"),
    impl_name: create_runtime_str!("primearc-core-chain"),
    authoring_version: 1,
    spec_version: 109, // Increment this
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
```

### Step 5: Build and Test

```bash
# Build runtime
cd /Users/macbook/Desktop/etrid
cargo build --release -p primearc-core-chain-runtime

# Run tests
cargo test -p primearc-core-chain-runtime
cargo test -p pallet-director-election
```

### Step 6: Initialize First Election

After deploying to chain, trigger the first election cycle:

```rust
// Via governance/sudo
DirectorElection::trigger_election(Origin::root())
```

## Usage Examples

### For Candidates

#### Register as Candidate (Nomination Phase)

```javascript
// Using Polkadot.js API
const manifesto = "I will focus on:\n" +
    "1. Scaling solutions for PBC chains\n" +
    "2. Community growth initiatives\n" +
    "3. Improved validator economics";

const tx = api.tx.directorElection.registerCandidate(manifesto);
await tx.signAndSend(candidateAccount);
```

#### Withdraw Candidacy

```javascript
const tx = api.tx.directorElection.withdrawCandidacy();
await tx.signAndSend(candidateAccount);
```

### For Voters

#### Cast Vote (Voting Phase)

```javascript
const candidateAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const tx = api.tx.directorElection.vote(candidateAddress);
await tx.signAndSend(voterAccount);
```

#### Change Vote

```javascript
const newCandidateAddress = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";
const tx = api.tx.directorElection.changeVote(newCandidateAddress);
await tx.signAndSend(voterAccount);
```

#### Remove Vote

```javascript
const tx = api.tx.directorElection.removeVote();
await tx.signAndSend(voterAccount);
```

### Query Election State

#### Get Current Phase

```javascript
const phase = await api.query.directorElection.electionPhase();
console.log("Current Phase:", phase.toHuman());

// Output example:
// {
//   "Voting": {
//     "start": "1,234,567",
//     "end": "1,335,367"
//   }
// }
```

#### Get Candidate Info

```javascript
const candidateAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
const candidate = await api.query.directorElection.candidates(candidateAddress);
console.log("Candidate:", candidate.toHuman());

// Output:
// {
//   "account": "5GrwvaEF...",
//   "stake": "128,000,000,000,000,000,000,000",
//   "manifesto": "I will focus on...",
//   "votesReceived": "45,678",
//   "registeredAt": "1,234,567"
// }
```

#### Get Elected Directors

```javascript
const directors = await api.query.directorElection.electedDirectors();
console.log("Current Directors:", directors.toHuman());

// Output: Array of 21 addresses
// ["5GrwvaEF...", "5FHneW46...", ...]
```

#### Check Voting Power

```javascript
// This is a helper function, not an RPC
// Calculate locally using staking info
const stake = await api.query.peerRolesStaking.roles(voterAddress);
const role = stake.role;

let multiplier = 0;
if (role === "DecentralizedDirector") multiplier = 3;
else if (role === "ValidityNode" || role === "FlareNode") multiplier = 2;
else if (role === "CommonStakePeer") multiplier = 1;

const votingPower = stake.stake * multiplier;
console.log("Voting Power:", votingPower);
```

## Monitoring Elections

### Watch for Phase Changes

```javascript
// Subscribe to events
api.query.system.events((events) => {
    events.forEach((record) => {
        const { event } = record;

        if (event.section === 'directorElection') {
            if (event.method === 'ElectionPhaseChanged') {
                const [oldPhase, newPhase, block] = event.data;
                console.log(`Phase changed from ${oldPhase} to ${newPhase} at block ${block}`);
            }

            if (event.method === 'ElectionCompleted') {
                const { epoch, winners, totalVotes } = event.data;
                console.log(`Election ${epoch} completed!`);
                console.log(`Winners:`, winners);
                console.log(`Total votes:`, totalVotes);
            }

            if (event.method === 'DirectorsSeated') {
                const [directors, epoch] = event.data;
                console.log(`Directors seated for epoch ${epoch}:`, directors);
            }
        }
    });
});
```

### Track Candidate Registrations

```javascript
api.query.system.events((events) => {
    events.forEach((record) => {
        const { event } = record;

        if (event.section === 'directorElection' && event.method === 'CandidateRegistered') {
            const [account, stake] = event.data;
            console.log(`New candidate: ${account} with stake ${stake}`);
        }
    });
});
```

### Monitor Vote Activity

```javascript
api.query.system.events((events) => {
    events.forEach((record) => {
        const { event } = record;

        if (event.section === 'directorElection') {
            if (event.method === 'VoteCast') {
                const [voter, candidate, votingPower] = event.data;
                console.log(`${voter} voted for ${candidate} with power ${votingPower}`);
            }

            if (event.method === 'VoteChanged') {
                const [voter, oldCandidate, newCandidate] = event.data;
                console.log(`${voter} changed vote from ${oldCandidate} to ${newCandidate}`);
            }
        }
    });
});
```

## Integration with Governance

### Use Directors for Permissioned Operations

```rust
// In your pallet
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn emergency_action(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Ensure caller is an elected director
        ensure!(
            pallet_director_election::Pallet::<T>::is_elected_director(&who),
            Error::<T>::NotDirector
        );

        // Execute emergency action
        Self::do_emergency_action()?;

        Ok(())
    }
}
```

### Multisig for Director Decisions

```rust
// Require 11/21 directors to approve
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn critical_parameter_change(
        origin: OriginFor<T>,
        new_value: u64,
    ) -> DispatchResult {
        ensure_root(origin)?; // Must go through multisig

        // Verify multisig threshold
        let approvals = DirectorApprovals::<T>::get();
        ensure!(approvals >= 11, Error::<T>::InsufficientApprovals);

        // Execute change
        CriticalParameter::<T>::put(new_value);

        Ok(())
    }
}
```

## Migration from Existing System

If you already have a director system:

### Step 1: Prepare Migration

```rust
// Create migration pallet
pub mod migrations {
    use super::*;

    pub fn migrate_directors<T: Config>() -> Weight {
        // Read old directors
        let old_directors = OldDirectorPallet::<T>::directors();

        // Set as initial elected directors
        let bounded: BoundedVec<T::AccountId, ConstU32<21>> =
            old_directors.try_into().unwrap_or_default();
        pallet_director_election::ElectedDirectors::<T>::put(bounded);

        // Set initial epoch
        pallet_director_election::CurrentEpoch::<T>::put(1);

        Weight::from_parts(100_000, 0)
    }
}
```

### Step 2: Execute Migration

```rust
#[cfg(feature = "try-runtime")]
impl<T: Config> OnRuntimeUpgrade for MigrateToDirectorElection<T> {
    fn on_runtime_upgrade() -> Weight {
        migrations::migrate_directors::<T>()
    }
}
```

## Troubleshooting

### Issue: Phase not transitioning

**Cause**: `on_initialize` not being called or block production stopped

**Solution**:
1. Check block production: `api.rpc.chain.getBlock()`
2. Verify block time: Should be 6 seconds
3. Check current block vs phase end block
4. Manually trigger if needed: `DirectorElection::trigger_election(Origin::root())`

### Issue: Cannot register as candidate

**Possible Causes**:
1. Not in Nomination phase
2. Don't have DecentralizedDirector role
3. Insufficient stake (< 128 ËTR)
4. Already registered

**Debug**:
```javascript
// Check phase
const phase = await api.query.directorElection.electionPhase();

// Check role
const roleRecord = await api.query.peerRolesStaking.roles(account);
console.log("Role:", roleRecord.role);
console.log("Stake:", roleRecord.stake);

// Check if already registered
const candidate = await api.query.directorElection.candidates(account);
```

### Issue: Cannot vote

**Possible Causes**:
1. Not in Voting phase
2. No voting power (no stake or role)
3. Already voted
4. Candidate doesn't exist

**Debug**:
```javascript
// Check phase
const phase = await api.query.directorElection.electionPhase();

// Check if already voted
const vote = await api.query.directorElection.votes(voterAccount);

// Check candidate exists
const candidate = await api.query.directorElection.candidates(candidateAccount);
```

## Testing Checklist

Before deploying to production:

- [ ] Build runtime successfully
- [ ] Run all unit tests
- [ ] Run integration tests
- [ ] Test on local testnet
- [ ] Test phase transitions
- [ ] Test with 0 candidates
- [ ] Test with < 21 candidates
- [ ] Test with > 21 candidates
- [ ] Test vote changes
- [ ] Test candidate withdrawals
- [ ] Test automatic tallying
- [ ] Verify director seating
- [ ] Test governance integration
- [ ] Monitor gas costs
- [ ] Test migration (if applicable)

## Production Deployment

### Pre-Deployment

1. **Audit Code**: Security audit recommended
2. **Test Thoroughly**: Use testnets extensively
3. **Document Parameters**: Record all configuration values
4. **Backup State**: Ensure chain state is backed up
5. **Plan Migration**: If migrating from existing system

### Deployment Steps

1. **Build Runtime**:
   ```bash
   cargo build --release -p primearc-core-chain-runtime
   ```

2. **Generate Wasm**:
   ```bash
   # Wasm blob will be in target/release/wbuild/primearc-core-chain-runtime/
   ```

3. **Runtime Upgrade**:
   ```javascript
   // Via governance
   const wasmBlob = fs.readFileSync('primearc-core-chain_runtime.compact.compressed.wasm');
   const tx = api.tx.system.setCode(wasmBlob);
   await tx.signAndSend(sudoAccount);
   ```

4. **Initialize**:
   ```javascript
   // Trigger first election cycle
   const tx = api.tx.directorElection.triggerElection();
   await tx.signAndSend(sudoAccount);
   ```

5. **Monitor**:
   - Watch for ElectionPhaseChanged events
   - Verify phase transitions
   - Check Consensus Day timing

### Post-Deployment

1. **Announce**: Notify community of election schedule
2. **Document**: Publish candidate and voting guides
3. **Monitor**: Watch for issues during first cycle
4. **Support**: Provide help for candidates and voters
5. **Iterate**: Gather feedback and plan improvements

## Support

For issues or questions:

- **Documentation**: See README.md
- **Code**: /Users/macbook/Desktop/etrid/12-consensus-day/pallet-director-election/
- **Architecture**: /Users/macbook/Desktop/etrid/11-peer-roles/ARCHITECTURE.md (lines 1035-1064)

## License

GPL-3.0
