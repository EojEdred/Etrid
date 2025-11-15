# Pallet ASF (Ascending Scale of Finality)

Production-ready Substrate pallet that integrates the ASF consensus algorithm into the FlareChain runtime.

## Overview

This pallet wraps the `asf-algorithm` crate to provide runtime storage, callable functions, and hooks for managing the ASF consensus protocol on FlareChain.

### Features

- **HotStuff 4-Phase Consensus**: Prepare → PreCommit → Commit → Decide
- **Validity Certificates**: Generation, validation, and aggregation
- **Ascending Scale of Finality**: 5 levels (0-4) based on certificate count
- **Byzantine Detection**: Automatic slashing for malicious validators
- **Validator Management**: Committee rotation and epoch transitions
- **Cryptographic Security**: Full signature verification on all votes

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      pallet-asf                              │
│  (Runtime Integration Layer)                                 │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  Storage Items:                                               │
│  • Validators (ValidatorId => Stake)                         │
│  • ValidatorSet (BoundedVec<ValidatorId>)                    │
│  • Votes (block_hash, phase => Vec<Vote>)                    │
│  • PendingCertificates (block_hash => Vec<Certificate>)      │
│  • BlockFinality (block_hash => FinalityLevel)               │
│  • SlashedValidators (ValidatorId => SlashingEvent)          │
│                                                               │
│  Extrinsics:                                                  │
│  • submit_vote(vote)                                          │
│  • submit_certificate(certificate)                            │
│  • rotate_validators(new_set)                                │
│  • slash_validator(validator, severity)                      │
│                                                               │
│  Hooks:                                                       │
│  • on_initialize: Check epoch rotation                       │
│  • on_finalize: Apply pending slashing                       │
│                                                               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    asf-algorithm                             │
│  (Core Consensus Logic)                                      │
├─────────────────────────────────────────────────────────────┤
│  • HotStuff consensus phases                                 │
│  • Certificate generation & validation                       │
│  • BFT threshold calculations                                │
│  • Cryptographic signatures                                  │
│  • Byzantine detection & slashing logic                      │
└─────────────────────────────────────────────────────────────┘
```

## Storage Items

### Active Validators

```rust
pub type Validators<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ValidatorId,
    Balance,
    OptionQuery,
>;
```

Stores the current active validators and their stake weights.

### Validator Set

```rust
pub type ValidatorSet<T: Config> = StorageValue<
    _,
    BoundedVec<ValidatorId, T::MaxValidators>,
    ValueQuery,
>;
```

Ordered list of validator IDs for BFT threshold calculations.

### Votes

```rust
pub type Votes<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    Hash,
    Blake2_128Concat,
    ConsensusPhase,
    BoundedVec<Vote, ConstU32<MAX_VOTES_PER_PHASE>>,
    ValueQuery,
>;
```

Collects votes per block and per consensus phase.

### Pending Certificates

```rust
pub type PendingCertificates<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    Hash,
    BoundedVec<Certificate, ConstU32<MAX_PENDING_CERTIFICATES>>,
    ValueQuery,
>;
```

Stores certificates awaiting finality level updates.

### Block Finality

```rust
pub type BlockFinality<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    Hash,
    FinalityLevel,
    ValueQuery,
>;
```

Tracks the finality level for each block.

### Slashed Validators

```rust
pub type SlashedValidators<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ValidatorId,
    SlashingEvent,
    OptionQuery,
>;
```

Records permanently excluded validators and their slashing details.

## Callable Functions

### submit_vote

```rust
pub fn submit_vote(
    origin: OriginFor<T>,
    vote: Vote,
) -> DispatchResult
```

Submit a vote for a block in a specific consensus phase.

**Requirements:**
- Caller must be a validator
- Vote must have valid cryptographic signature
- No duplicate votes allowed

**Emits:** `VoteSubmitted`, `ThresholdMet` (if BFT threshold reached)

### submit_certificate

```rust
pub fn submit_certificate(
    origin: OriginFor<T>,
    certificate: Certificate,
) -> DispatchResult
```

Submit a validity certificate for a block.

**Requirements:**
- Caller must be a validator
- Certificate must meet BFT threshold
- Valid aggregate signatures

**Emits:** `CertificateGenerated`, `FinalityLevelChanged`

### rotate_validators

```rust
pub fn rotate_validators(
    origin: OriginFor<T>,
    new_validators: Vec<(ValidatorId, Balance)>,
) -> DispatchResult
```

Rotate the validator set (governance/root only).

**Requires:** Root origin

**Emits:** `ValidatorSetRotated`

### slash_validator

```rust
pub fn slash_validator(
    origin: OriginFor<T>,
    validator: ValidatorId,
    severity: SlashingSeverity,
    reason: Vec<u8>,
) -> DispatchResult
```

Slash a validator for Byzantine behavior.

**Requires:** Root origin

**Severity Levels:**
- **Minor**: 5% stake slash
- **Moderate**: 15% stake slash
- **Severe**: 40% stake slash
- **Critical**: 100% stake slash + permanent exclusion

**Emits:** `ValidatorSlashed`, `ValidatorExcluded`

## Events

```rust
pub enum Event<T: Config> {
    VoteSubmitted { validator, block_hash, phase, epoch },
    CertificateGenerated { block_hash, phase, validator, certificate_count },
    FinalityLevelChanged { block_hash, old_level, new_level },
    ValidatorSlashed { validator, amount, severity, reason },
    ValidatorExcluded { validator, reason },
    ValidatorSetRotated { epoch, validator_count },
    ThresholdMet { block_hash, phase, vote_count, total_stake },
}
```

## Errors

```rust
pub enum Error<T> {
    InvalidVote,
    InvalidCertificate,
    NotValidator,
    DuplicateVote,
    InsufficientStake,
    InvalidPhaseTransition,
    TooManyCertificates,
    TooManyVotes,
    ValidatorSlashed,
    InvalidSignature,
    InvalidPhase,
    BlockNotFound,
}
```

## Configuration

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    type MaxValidators: Get<u32>;
    type MinValidatorStake: Get<BalanceOf<Self>>;
    type SlashHandler: SlashValidator<Self::AccountId, BalanceOf<Self>>;
    type FinalityNotifier: FinalityNotification<Self::BlockNumber>;
    type EpochDuration: Get<Self::BlockNumber>;
}
```

## Runtime Integration

Add to FlareChain runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-asf = { path = "pallets/pallet-asf", default-features = false }

[features]
std = [
    # ... other pallets
    "pallet-asf/std",
]
```

Add to runtime configuration in `lib.rs`:

```rust
parameter_types! {
    pub const MaxValidators: u32 = 100;
    pub const MinValidatorStake: Balance = 1_000_000;
    pub const EpochDuration: BlockNumber = 7200; // ~24 hours
}

impl pallet_asf::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxValidators = MaxValidators;
    type MinValidatorStake = MinValidatorStake;
    type SlashHandler = (); // Implement with pallet-etrid-staking
    type FinalityNotifier = (); // Implement with finality-gadget
    type EpochDuration = EpochDuration;
}

construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Asf: pallet_asf,
        // ... other pallets
    }
);
```

## Genesis Configuration

```rust
pallet_asf::GenesisConfig::<Runtime> {
    validators: vec![
        (validator_id_1, 10_000_000),
        (validator_id_2, 10_000_000),
        // ... 21 validators total for BFT threshold
    ],
    _phantom: Default::default(),
}
```

## Testing

Run tests:

```bash
cargo test -p pallet-asf
```

Test coverage includes:
- Vote submission and validation
- Duplicate vote prevention
- BFT threshold calculations
- Certificate generation and validation
- Finality level progression
- Validator rotation
- Slashing enforcement
- Epoch transitions

## Security Considerations

### Cryptographic Verification

All votes MUST include valid cryptographic signatures. The pallet verifies:
- Signature authenticity using `verify_vote_signature()`
- Validator stake weight matches storage
- No duplicate votes per validator per phase

### Byzantine Fault Tolerance

The pallet enforces BFT threshold (2/3 + 1):
- For 21 validators: 15 votes required
- Automatic threshold detection
- Stake-weighted voting support

### Slashing

Validators are slashed for:
- Invalid signatures
- Conflicting votes
- Invalid certificates
- Byzantine behavior

Critical violations result in permanent exclusion.

## Performance

- **Vote submission**: O(1) storage writes
- **Threshold checking**: O(n) where n = validator count
- **Certificate validation**: O(1) with aggregate signatures
- **Finality updates**: O(1) storage writes

## Dependencies

- `asf-algorithm`: Core consensus logic
- `validator-management`: Committee management
- `frame-support`: FRAME macros and traits
- `frame-system`: System pallet integration
- `sp-core`, `sp-runtime`, `sp-std`: Substrate primitives

## License

Apache-2.0

## Authors

ËTRID Foundation

## Contributing

See the main ËTRID repository for contribution guidelines.

## Next Steps

1. **Integrate with FlareChain Runtime**: Add pallet to `runtime/src/lib.rs`
2. **Connect SlashHandler**: Link to `pallet-etrid-staking`
3. **Connect FinalityNotifier**: Link to `finality-gadget`
4. **Add Runtime API**: Expose consensus state queries
5. **Enable Benchmarking**: Generate accurate weights
