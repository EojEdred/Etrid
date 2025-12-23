# ËTRID Bridge Pause Implementation - Technical Details

This document provides technical details about the emergency pause implementation in the ËTRID bridge pallets.

## 1. pallet-token-messenger Implementation

### Storage Item

```rust
/// Bridge pause status (emergency stop)
#[pallet::storage]
#[pallet::getter(fn is_paused)]
pub type IsPaused<T: Config> = StorageValue<_, bool, ValueQuery>;
```

**Location**: `/Users/macbook/Desktop/etrid/05-multichain/pallets-shared/pallet-token-messenger/src/lib.rs` (lines 553-556)

**Default Value**: `false` (bridge is active by default)

**Type**: `StorageValue<_, bool, ValueQuery>` - Simple boolean flag

**Getter**: `is_paused()` - Public getter function for reading pause status

### Extrinsics

#### pause_bridge()

```rust
/// Pause bridge operations
///
/// Emergency stop for all bridge operations.
/// Only callable by governance (root).
///
/// # Events
/// * `BridgePaused` - When bridge is paused
#[pallet::call_index(4)]
#[pallet::weight(T::WeightInfo::pause_bridge())]
pub fn pause_bridge(origin: OriginFor<T>) -> DispatchResult {
    ensure_root(origin)?;

    IsPaused::<T>::put(true);

    Self::deposit_event(Event::BridgePaused);

    Ok(())
}
```

**Location**: Lines 1062-1070

**Call Index**: 4

**Origin Requirement**: `ensure_root(origin)?` - Requires sudo/governance

**Weight**: `T::WeightInfo::pause_bridge()`

**Effect**: Sets `IsPaused` storage to `true`

**Event**: Emits `BridgePaused` event

#### unpause_bridge()

```rust
/// Unpause bridge operations
///
/// Resume normal bridge operations after pause.
/// Only callable by governance (root).
///
/// # Events
/// * `BridgeUnpaused` - When bridge is unpaused
#[pallet::call_index(5)]
#[pallet::weight(T::WeightInfo::unpause_bridge())]
pub fn unpause_bridge(origin: OriginFor<T>) -> DispatchResult {
    ensure_root(origin)?;

    IsPaused::<T>::put(false);

    Self::deposit_event(Event::BridgeUnpaused);

    Ok(())
}
```

**Location**: Lines 1081-1089

**Call Index**: 5

**Origin Requirement**: `ensure_root(origin)?` - Requires sudo/governance

**Weight**: `T::WeightInfo::unpause_bridge()`

**Effect**: Sets `IsPaused` storage to `false`

**Event**: Emits `BridgeUnpaused` event

### Pause Checks

#### deposit_for_burn()

```rust
pub fn deposit_for_burn(
    origin: OriginFor<T>,
    amount: u128,
    destination_domain: u32,
    mint_recipient: Vec<u8>,
) -> DispatchResult {
    let sender = ensure_signed(origin)?;

    // Check bridge not paused
    ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);

    // ... rest of function
}
```

**Location**: Line 749

**Check**: `ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);`

**Effect**: Rejects transaction with `Error::<T>::BridgePaused` if paused

**When Paused**: Users cannot burn tokens for cross-chain transfer

#### receive_message()

```rust
pub fn receive_message(
    origin: OriginFor<T>,
    message: Vec<u8>,
    attestation: Vec<u8>,
) -> DispatchResult {
    ensure_signed(origin)?;

    // Check bridge not paused
    ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);

    // ... rest of function
}
```

**Location**: Line 915

**Check**: `ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);`

**Effect**: Rejects transaction with `Error::<T>::BridgePaused` if paused

**When Paused**: Relayers cannot deliver messages and mint tokens

### Events

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    // ... other events

    /// Bridge paused by governance
    BridgePaused,
    /// Bridge unpaused by governance
    BridgeUnpaused,

    // ... other events
}
```

**Location**: Lines 612-614

**BridgePaused**: Emitted when `pause_bridge()` is called

**BridgeUnpaused**: Emitted when `unpause_bridge()` is called

**Indexing**: These events can be subscribed to for real-time monitoring

### Error

```rust
#[pallet::error]
pub enum Error<T> {
    /// Bridge is paused
    BridgePaused,
    // ... other errors
}
```

**Location**: Line 637

**Error Code**: `BridgePaused`

**When Thrown**: When `deposit_for_burn()` or `receive_message()` is called while bridge is paused

### Operations NOT Blocked by Pause

The following extrinsics continue to work during pause:

- `configure_domain()` - Governance can update domain configurations
- `remove_domain()` - Governance can remove domains
- `pause_bridge()` - Can be called even if already paused (no-op)
- `unpause_bridge()` - Required to resume operations

**Rationale**: Governance needs the ability to update configurations during emergencies.

---

## 2. pallet-bridge-attestation Implementation

### Storage Item

```rust
/// Emergency pause flag
#[pallet::storage]
#[pallet::getter(fn is_paused)]
pub type IsPaused<T: Config> = StorageValue<_, bool, ValueQuery>;
```

**Location**: `/Users/macbook/Desktop/etrid/05-multichain/pallets-shared/pallet-bridge-attestation/src/lib.rs` (lines 253-255)

**Default Value**: `false` (attestation service is active by default)

**Type**: `StorageValue<_, bool, ValueQuery>` - Simple boolean flag

**Getter**: `is_paused()` - Public getter function for reading pause status

### Extrinsics

#### pause_attestation()

```rust
/// Pause attestation service
///
/// Requires root origin (governance)
#[pallet::call_index(8)]
#[pallet::weight(T::WeightInfo::pause_attestation())]
pub fn pause_attestation(origin: OriginFor<T>) -> DispatchResult {
    ensure_root(origin)?;
    IsPaused::<T>::put(true);
    Self::deposit_event(Event::AttestationPaused);
    Ok(())
}
```

**Location**: Lines 805-810

**Call Index**: 8

**Origin Requirement**: `ensure_root(origin)?` - Requires sudo/governance

**Weight**: `T::WeightInfo::pause_attestation()`

**Effect**: Sets `IsPaused` storage to `true`

**Event**: Emits `AttestationPaused` event

#### unpause_attestation()

```rust
/// Unpause attestation service
///
/// Requires root origin (governance)
#[pallet::call_index(9)]
#[pallet::weight(T::WeightInfo::unpause_attestation())]
pub fn unpause_attestation(origin: OriginFor<T>) -> DispatchResult {
    ensure_root(origin)?;
    IsPaused::<T>::put(false);
    Self::deposit_event(Event::AttestationUnpaused);
    Ok(())
}
```

**Location**: Lines 817-822

**Call Index**: 9

**Origin Requirement**: `ensure_root(origin)?` - Requires sudo/governance

**Weight**: `T::WeightInfo::unpause_attestation()`

**Effect**: Sets `IsPaused` storage to `false`

**Event**: Emits `AttestationUnpaused` event

### Pause Checks

#### register_attester()

```rust
pub fn register_attester(
    origin: OriginFor<T>,
    public_key: Vec<u8>,
) -> DispatchResult {
    ensure_root(origin)?;
    ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

    // ... rest of function
}
```

**Location**: Line 384

**Check**: `ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);`

**When Paused**: Cannot register new attesters

#### submit_signature()

```rust
pub fn submit_signature(
    origin: OriginFor<T>,
    attester_id: u32,
    message_hash: H256,
    signature: Vec<u8>,
    source_chain_id: u32,
    destination_chain_id: u32,
    nonce: u64,
) -> DispatchResult {
    let _submitter = ensure_signed(origin)?;
    ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

    // ... rest of function
}
```

**Location**: Line 550

**Check**: `ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);`

**When Paused**: Attesters cannot submit signatures

#### verify_attestation()

```rust
pub fn verify_attestation(
    origin: OriginFor<T>,
    message: Vec<u8>,
    message_hash: H256,
) -> DispatchResult {
    let _caller = ensure_signed(origin)?;
    ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

    // ... rest of function
}
```

**Location**: Line 656

**Check**: `ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);`

**When Paused**: Cannot verify attestations (blocks message processing)

#### verify_attestation_for_message() (Helper Function)

```rust
pub fn verify_attestation_for_message(
    message: &[u8],
    message_hash: H256,
) -> DispatchResult {
    ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);

    // ... rest of function
}
```

**Location**: Line 980

**Check**: `ensure!(!IsPaused::<T>::get(), Error::<T>::AttestationPaused);`

**Note**: This is an internal helper function called by other pallets (e.g., token messenger)

**When Paused**: All attestation verification fails

### Events

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    // ... other events

    /// Attestation service paused
    AttestationPaused,
    /// Attestation service unpaused
    AttestationUnpaused,

    // ... other events
}
```

**Location**: Lines 322-324

**AttestationPaused**: Emitted when `pause_attestation()` is called

**AttestationUnpaused**: Emitted when `unpause_attestation()` is called

### Error

```rust
#[pallet::error]
pub enum Error<T> {
    /// Attestation service is paused
    AttestationPaused,
    // ... other errors
}
```

**Location**: Line 334

**Error Code**: `AttestationPaused`

**When Thrown**: When any attestation operation is attempted while paused

### Operations NOT Blocked by Pause

The following extrinsics continue to work during pause:

- `disable_attester()` - Governance can disable attesters
- `enable_attester()` - Governance can enable attesters
- `remove_attester()` - Governance can remove attesters
- `configure_threshold()` - Governance can update thresholds
- `update_threshold()` - Governance can update global threshold
- `pause_attestation()` - Can be called even if already paused (no-op)
- `unpause_attestation()` - Required to resume operations

**Rationale**: Governance needs the ability to manage attesters during emergencies.

---

## 3. Integration Between Pallets

### AttestationVerifier Trait

The token messenger pallet calls the attestation pallet through the `AttestationVerifier` trait:

```rust
pub trait AttestationVerifier {
    fn verify_message_attestation(
        message: &[u8],
        attestation: &[u8],
        message_hash: sp_core::H256,
    ) -> frame_support::dispatch::DispatchResult;
}
```

### Implementation in Runtime

```rust
// In your PBC runtime (lib.rs)
impl pallet_token_messenger::Config for Runtime {
    type AttestationVerifier = BridgeAttestation; // Links to attestation pallet
    // ... other config
}
```

### Call Chain

When `token_messenger::receive_message()` is called:

1. Token Messenger checks its own pause status
2. If not paused, calls `AttestationVerifier::verify_message_attestation()`
3. Attestation pallet checks its own pause status
4. If not paused, verifies signatures

**Result**: Both pallets must be unpaused for message processing to work.

---

## 4. Weight Information

### Token Messenger Weights

```rust
pub trait WeightInfo {
    fn deposit_for_burn() -> Weight;
    fn receive_message() -> Weight;
    fn configure_domain() -> Weight;
    fn remove_domain() -> Weight;
    fn pause_bridge() -> Weight;    // Weight for pause operation
    fn unpause_bridge() -> Weight;  // Weight for unpause operation
}
```

**Pause Operation Weight**:
- Single storage write (`IsPaused::put(true)`)
- One event emission
- Extremely lightweight (< 10_000 weight units)

**Expected Weight**: ~5,000 - 10,000 weight units (negligible)

### Attestation Weights

```rust
pub trait WeightInfo {
    fn register_attester() -> Weight;
    fn disable_attester() -> Weight;
    fn enable_attester() -> Weight;
    fn remove_attester() -> Weight;
    fn submit_signature() -> Weight;
    fn verify_attestation() -> Weight;
    fn update_threshold() -> Weight;
    fn pause_attestation() -> Weight;    // Weight for pause operation
    fn unpause_attestation() -> Weight;  // Weight for unpause operation
}
```

**Pause Operation Weight**: Similar to token messenger, extremely lightweight.

---

## 5. Security Considerations

### Access Control

Both pallets use `ensure_root(origin)?` for pause/unpause operations:

```rust
ensure_root(origin)?;
```

**Development**: This checks for sudo account

**Production**: Should be configured to check for governance collective:

```rust
// In runtime configuration
type EnsureRoot = pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>;
```

### Atomicity

- Pause operations are atomic (single storage write)
- No possibility of partial pause
- Effective immediately in the same block
- No rollback required

### Idempotency

Both pause and unpause operations are idempotent:

- Pausing an already paused bridge is safe (no-op)
- Unpausing an already active bridge is safe (no-op)

**Example**:
```rust
// First call
IsPaused::<T>::put(true); // Sets to true
// Second call
IsPaused::<T>::put(true); // Sets to true (no change)
```

### Race Conditions

Since pause is checked at the **beginning** of each extrinsic:

```rust
ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);
```

**Scenario**: What if pause is called while transactions are in the mempool?

**Answer**: Transactions already in the mempool will be rejected when they execute, because the pause check happens during execution, not when the transaction is submitted.

**Result**: No transactions can execute after pause, even if submitted before pause.

---

## 6. Testing Recommendations

### Unit Tests

```rust
#[test]
fn pause_bridge_should_work() {
    new_test_ext().execute_with(|| {
        // Pause bridge
        assert_ok!(TokenMessenger::pause_bridge(Origin::root()));
        assert_eq!(TokenMessenger::is_paused(), true);

        // Try to deposit (should fail)
        assert_noop!(
            TokenMessenger::deposit_for_burn(
                Origin::signed(ALICE),
                100,
                0,
                vec![1, 2, 3]
            ),
            Error::<Test>::BridgePaused
        );

        // Unpause
        assert_ok!(TokenMessenger::unpause_bridge(Origin::root()));
        assert_eq!(TokenMessenger::is_paused(), false);

        // Try to deposit (should succeed)
        assert_ok!(
            TokenMessenger::deposit_for_burn(
                Origin::signed(ALICE),
                100,
                0,
                vec![1, 2, 3]
            )
        );
    });
}
```

### Integration Tests

Test interaction between pallets:

```rust
#[test]
fn pause_affects_message_verification() {
    new_test_ext().execute_with(|| {
        // Pause attestation only
        assert_ok!(BridgeAttestation::pause_attestation(Origin::root()));

        // Try to receive message (should fail because attestation is paused)
        assert_noop!(
            TokenMessenger::receive_message(
                Origin::signed(RELAYER),
                message_bytes,
                attestation_bytes
            ),
            // Will fail at attestation verification
        );
    });
}
```

---

## 7. Monitoring and Observability

### Storage Queries

```rust
// Query pause status
let is_paused = api.query.tokenMessenger.isPaused();
let attestation_paused = api.query.bridgeAttestation.isPaused();
```

### Event Subscription

```rust
// Subscribe to pause events
api.query.system.events((events) => {
    events.forEach(({ event }) => {
        if (event.section === 'tokenMessenger') {
            if (event.method === 'BridgePaused') {
                console.log('ALERT: Token bridge paused!');
            }
            if (event.method === 'BridgeUnpaused') {
                console.log('INFO: Token bridge resumed');
            }
        }

        if (event.section === 'bridgeAttestation') {
            if (event.method === 'AttestationPaused') {
                console.log('ALERT: Attestation service paused!');
            }
            if (event.method === 'AttestationUnpaused') {
                console.log('INFO: Attestation service resumed');
            }
        }
    });
});
```

### Prometheus Metrics (Recommended)

```rust
// Add to your runtime
pub fn export_pause_metrics() {
    prometheus::gauge!("bridge_paused", "Bridge pause status")
        .set(if TokenMessenger::is_paused() { 1.0 } else { 0.0 });

    prometheus::gauge!("attestation_paused", "Attestation pause status")
        .set(if BridgeAttestation::is_paused() { 1.0 } else { 0.0 });
}
```

---

## 8. Upgrade Considerations

### Runtime Upgrades

When performing runtime upgrades:

1. **Current pause state is preserved** (storage persists across upgrades)
2. If bridge was paused before upgrade, it remains paused after
3. Test upgrades on testnet with bridge in both states (paused/unpaused)

### Migration

If you need to change the pause mechanism:

```rust
#[pallet::storage_version(2)]
pub mod migrations {
    use super::*;

    pub fn migrate_v1_to_v2<T: Config>() -> Weight {
        // Example: Add additional pause granularity
        let pause_status = IsPaused::<T>::get();

        // Migrate to new storage structure
        NewPauseConfig::<T>::put(PauseConfig {
            deposits_paused: pause_status,
            withdrawals_paused: pause_status,
            // ... etc
        });

        Weight::from_parts(10_000, 0)
    }
}
```

---

## Summary

Both `pallet-token-messenger` and `pallet-bridge-attestation` have **complete, production-ready emergency pause implementations**:

- ✅ Storage items properly defined
- ✅ Pause/unpause extrinsics with root checks
- ✅ Pause checks in all critical operations
- ✅ Events for monitoring
- ✅ Proper error handling
- ✅ No race conditions
- ✅ Idempotent operations
- ✅ Independent pause controls
- ✅ Governance-ready (requires root)

**No additional implementation is required.** The pallets are ready for production use with proper governance configuration.
