# ASF Consensus TODO Implementation Plan

**Date:** October 21, 2025
**Status:** Implementation Ready
**Priority:** High (Mainnet Blocker)
**Estimated Effort:** 2-3 weeks

---

## Executive Summary

This document provides detailed implementation plans for the 4 high-priority TODOs in the ASF consensus service (`05-multichain/flare-chain/node/src/asf_service.rs`). While the current placeholder logic is safe for testnet deployment and external security audit, these TODOs MUST be implemented before mainnet launch.

**Current Status:**
- ✅ **Audit Ready:** Placeholder logic is documented and safe
- ⚠️ **Mainnet Blocker:** Runtime API integration required
- 📋 **Well-Documented:** All requirements clearly specified

---

## TODO #1: Validator Committee Loading (Line 597-613)

### Current Implementation

```rust
// Create committee manager with test committee
// TODO: Load actual committee from runtime state
let mut committee = CommitteeManager::new(ppfa_params.max_committee_size);

// Initialize with test validators for now
// TODO: Query validator-management pallet for real committee
log::debug!("Initializing PPFA committee (size: {})", ppfa_params.max_committee_size);
for i in 0..3 {
    let validator_id = block_production::ValidatorId::from([i as u8; 32]);
    let validator_info = validator_management::ValidatorInfo::new(
        validator_id,
        ppfa_params.min_validator_stake,
        validator_management::PeerType::ValidityNode,
    );
    if let Err(e) = committee.add_validator(validator_info) {
        log::warn!("Failed to add test validator {}: {:?}", i, e);
    }
}
```

### Required Implementation

#### Step 1: Create Substrate Pallet Wrapper

Create `pallets/pallet-validator-management/src/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    traits::Get,
};
use frame_system::ensure_root;
use sp_std::vec::Vec;
use validator_management::{ValidatorInfo, CommitteeMember};

pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type MaxValidators: Get<u32>;
}

decl_storage! {
    trait Store for Module<T: Config> as ValidatorManagement {
        /// Active validators
        Validators get(fn validators): Vec<ValidatorInfo>;

        /// Current committee members
        Committee get(fn committee): Vec<CommitteeMember>;

        /// Current epoch number
        CurrentEpoch get(fn current_epoch): u32;
    }
}

decl_event! {
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        ValidatorAdded(AccountId),
        ValidatorRemoved(AccountId),
        CommitteeRotated(u32), // epoch
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        ValidatorNotFound,
        CommitteeFull,
        InvalidValidator,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn add_validator(
            origin,
            validator: ValidatorInfo,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let mut validators = Validators::<T>::get();
            validators.push(validator.clone());
            Validators::<T>::put(validators);

            Self::deposit_event(RawEvent::ValidatorAdded(/* ... */));
            Ok(())
        }

        #[weight = 10_000]
        pub fn rotate_committee(origin, epoch: u32) -> DispatchResult {
            ensure_root(origin)?;

            // Select new committee based on stake/reputation
            let validators = Validators::<T>::get();
            let new_committee = Self::select_committee(&validators)?;

            Committee::<T>::put(new_committee);
            CurrentEpoch::<T>::put(epoch);

            Self::deposit_event(RawEvent::CommitteeRotated(epoch));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    fn select_committee(validators: &[ValidatorInfo]) -> Result<Vec<CommitteeMember>, Error<T>> {
        // Implement stake-weighted selection logic
        // Filter by reputation >= 50
        // Sort by stake descending
        // Take top N validators up to max committee size
        todo!("Implement committee selection")
    }
}
```

#### Step 2: Add Runtime API

Create `runtime/src/apis.rs`:

```rust
sp_api::decl_runtime_apis! {
    pub trait ValidatorManagementApi {
        /// Get active validators at a specific block
        fn get_active_validators(at: BlockNumber) -> Vec<ValidatorInfo>;

        /// Get current committee members
        fn get_committee() -> Vec<CommitteeMember>;

        /// Check if a validator is in the committee
        fn is_in_committee(validator_id: ValidatorId) -> bool;
    }
}
```

Implement in `runtime/src/lib.rs`:

```rust
impl_runtime_apis! {
    impl validator_apis::ValidatorManagementApi<Block> for Runtime {
        fn get_active_validators(_at: BlockNumber) -> Vec<ValidatorInfo> {
            ValidatorManagement::validators()
        }

        fn get_committee() -> Vec<CommitteeMember> {
            ValidatorManagement::committee()
        }

        fn is_in_committee(validator_id: ValidatorId) -> bool {
            ValidatorManagement::committee()
                .iter()
                .any(|m| m.validator == validator_id)
        }
    }
}
```

#### Step 3: Update Service Layer

Replace lines 597-613 in `asf_service.rs`:

```rust
// Load actual committee from runtime state via Runtime API
let mut committee = CommitteeManager::new(ppfa_params.max_committee_size);

// Query runtime for active validators
let runtime_validators = match ppfa_client.runtime_api()
    .get_active_validators(parent_hash)
{
    Ok(validators) => validators,
    Err(e) => {
        log::error!("Failed to query validators from runtime: {:?}", e);
        log::warn!("Falling back to empty committee");
        vec![]
    }
};

log::info!("Loading {} validators from runtime", runtime_validators.len());

// Add validators to committee manager
for validator in runtime_validators {
    if let Err(e) = committee.add_validator(validator.clone()) {
        log::warn!(
            "Failed to add validator {:?}: {:?}",
            hex::encode(&validator.id.encode()[..8]),
            e
        );
    }
}

// Rotate to initialize committee for epoch 1
if let Err(e) = committee.rotate_committee(1) {
    log::error!("Failed to initialize committee rotation: {:?}", e);
    return;
}

log::info!("✅ Committee initialized with {} active validators", committee.committee_size());
```

### Dependencies

- `pallet-validator-management` must be added to runtime `Cargo.toml`
- Runtime must be recompiled with new pallet
- Runtime API must be registered
- Integration tests required

### Estimated Effort

- **Pallet Creation:** 3-4 days
- **Runtime API Integration:** 2 days
- **Service Layer Update:** 1 day
- **Testing:** 2-3 days
- **Total:** ~1.5 weeks

---

## TODO #2: Validator Key Management (Line 674-676)

### Current Implementation

```rust
// TODO: Get our validator ID from keystore
// For now, we just log the slot info
let our_validator_id = block_production::ValidatorId::from([0u8; 32]);
```

### Required Implementation

#### Step 1: Define ASF Validator Key Type

Add to `node/src/asf_service.rs`:

```rust
use sp_core::crypto::KeyTypeId;

/// ASF Validator key type identifier
pub const ASF_VALIDATOR_KEY_TYPE: KeyTypeId = KeyTypeId(*b"asfv");
```

#### Step 2: Pass Keystore to Worker

Modify the task spawn at line 585:

```rust
// Clone keystore for worker
let ppfa_keystore = keystore_container.keystore();

task_manager.spawn_essential_handle().spawn_blocking(
    "asf-ppfa-proposer",
    Some("block-authoring"),
    async move {
        // ... existing code ...
```

#### Step 3: Load Validator ID from Keystore

Replace lines 674-676:

```rust
// Get our validator ID from keystore
let our_validator_id = match Self::load_validator_identity(&ppfa_keystore).await {
    Some(id) => {
        log::info!("🔑 Loaded validator identity from keystore: {:?}", hex::encode(&id.encode()[..8]));
        id
    }
    None => {
        // Not a validator node - skip block production
        log::debug!("Not a validator (no ASF key in keystore)");
        block_production::ValidatorId::from([0u8; 32])
    }
};
```

#### Step 4: Implement Keystore Query Function

Add helper function:

```rust
/// Load validator identity from keystore
async fn load_validator_identity(
    keystore: &KeystorePtr,
) -> Option<block_production::ValidatorId> {
    use sp_core::crypto::Pair;
    use sp_core::sr25519::Pair as Sr25519Pair;

    // Query all SR25519 keys with ASF_VALIDATOR_KEY_TYPE
    let public_keys = keystore
        .sr25519_public_keys(ASF_VALIDATOR_KEY_TYPE)
        .await;

    if public_keys.is_empty() {
        return None;
    }

    // Use first available key
    let public_key = public_keys[0];

    // Convert to ValidatorId (32-byte array)
    let validator_id = block_production::ValidatorId::from(public_key.0);

    Some(validator_id)
}
```

### Dependencies

- `sp_core::crypto::KeyTypeId`
- `sp_core::sr25519::Public`
- Keystore must have ASF validator keys loaded via `author_insertKey` RPC

### Estimated Effort

- **Implementation:** 1 day
- **Testing:** 1 day
- **Key Management Docs:** 1 day
- **Total:** ~3 days

### Testing

```bash
# Insert ASF validator key via RPC
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc":"2.0",
    "method":"author_insertKey",
    "params":["asfv", "your-seed-phrase", "0x...public-key"],
    "id":1
  }' \
  http://localhost:9944
```

---

## TODO #3: Epoch Transition Logic (Line 801-812)

### Current Implementation

```rust
// TODO: Implement proper epoch transitions
if slot_count % ppfa_params.epoch_duration as u64 == 0 {
    let epoch = slot_count / ppfa_params.epoch_duration as u64;
    log::info!("🔄 Epoch transition at slot #{} (epoch #{})", slot_number, epoch);
    // TODO: Rotate committee based on runtime state
}
```

### Required Implementation

#### Step 1: Add Epoch Management to Runtime API

Extend `ValidatorManagementApi`:

```rust
sp_api::decl_runtime_apis! {
    pub trait ValidatorManagementApi {
        // ... existing methods ...

        /// Get current epoch number
        fn current_epoch() -> u32;

        /// Get next epoch start block
        fn next_epoch_start() -> BlockNumber;

        /// Get validators for next epoch (pre-computed)
        fn get_next_epoch_validators() -> Vec<ValidatorInfo>;
    }
}
```

#### Step 2: Implement Runtime-side Epoch Logic

In `pallet-validator-management`:

```rust
decl_storage! {
    trait Store for Module<T: Config> as ValidatorManagement {
        // ... existing storage ...

        /// Epoch duration in blocks
        EpochDuration get(fn epoch_duration): T::BlockNumber = T::EpochDuration::get();

        /// Next epoch validators (pre-selected)
        NextEpochValidators get(fn next_epoch_validators): Vec<ValidatorInfo>;
    }
}

impl<T: Config> Module<T> {
    /// Called by on_finalize to check for epoch boundaries
    fn check_epoch_transition(block_number: T::BlockNumber) {
        let epoch_duration = Self::epoch_duration();
        if block_number % epoch_duration == Zero::zero() {
            let new_epoch = (block_number / epoch_duration).saturated_into::<u32>();
            Self::rotate_to_next_epoch(new_epoch);
        }
    }

    fn rotate_to_next_epoch(epoch: u32) {
        // Move NextEpochValidators to Committee
        let next_validators = NextEpochValidators::<T>::get();
        let new_committee = Self::select_committee(&next_validators).expect("Committee selection");

        Committee::<T>::put(new_committee);
        CurrentEpoch::<T>::put(epoch);

        // Pre-select validators for NEXT epoch
        let all_validators = Validators::<T>::get();
        NextEpochValidators::<T>::put(all_validators);

        Self::deposit_event(RawEvent::CommitteeRotated(epoch));
    }
}
```

#### Step 3: Update Service Layer

Replace lines 801-812:

```rust
// Check for epoch transition via runtime
let current_block = ppfa_client.usage_info().chain.best_number;
let runtime_epoch = match ppfa_client.runtime_api()
    .current_epoch(parent_hash)
{
    Ok(epoch) => epoch,
    Err(e) => {
        log::warn!("Failed to query runtime epoch: {:?}", e);
        (slot_count / ppfa_params.epoch_duration as u64) as u32
    }
};

// Check if epoch changed
if runtime_epoch != current_epoch {
    log::info!(
        "🔄 Epoch transition detected: {} → {} (slot #{})",
        current_epoch,
        runtime_epoch,
        slot_number
    );

    // Query new committee from runtime
    let new_validators = match ppfa_client.runtime_api()
        .get_active_validators(parent_hash)
    {
        Ok(validators) => validators,
        Err(e) => {
            log::error!("Failed to load new epoch validators: {:?}", e);
            vec![]
        }
    };

    // Update committee manager
    let mut new_committee = CommitteeManager::new(ppfa_params.max_committee_size);
    for validator in new_validators {
        if let Err(e) = new_committee.add_validator(validator) {
            log::warn!("Failed to add validator for new epoch: {:?}", e);
        }
    }

    if let Err(e) = new_committee.rotate_committee(runtime_epoch) {
        log::error!("Failed to rotate committee: {:?}", e);
    } else {
        // Replace proposer selector with new committee
        proposer_selector = ProposerSelector::new(new_committee);
        current_epoch = runtime_epoch;

        log::info!("✅ Committee rotated for epoch {}", runtime_epoch);
        log::info!("   New committee size: {}", proposer_selector.committee_size());
    }
}
```

### Dependencies

- Pallet-validator-management with epoch logic
- Runtime API implementation
- `on_finalize` hook in runtime

### Estimated Effort

- **Runtime Epoch Logic:** 3-4 days
- **Service Layer Integration:** 2 days
- **Edge Case Handling:** 2 days (mid-epoch joins, network splits)
- **Testing:** 3 days
- **Total:** ~2 weeks

---

## TODO #4: PPFA Proposer Authorization (Line 265-267)

### Current Implementation

```rust
// ASF BLOCK VALIDATION using block-production::validation module
//
// This validates blocks according to ASF consensus rules:
// 1. Block structure (header, transactions, size)
// 2. PPFA proposer authorization (TODO: requires runtime query)
// 3. Block type validation (Queen vs Ant)
```

### Required Implementation

#### Step 1: Add PPFA Authorization Tracking to Runtime

In `pallet-validator-management`:

```rust
decl_storage! {
    trait Store for Module<T: Config> as ValidatorManagement {
        // ... existing storage ...

        /// PPFA authorization history: (block_number, ppfa_index) => validator_id
        PPFAHistory get(fn ppfa_history):
            map hasher(twox_64_concat) (T::BlockNumber, u32) => Option<ValidatorId>;
    }
}

impl<T: Config> Module<T> {
    /// Record PPFA authorization when block is produced
    pub fn record_ppfa_authorization(
        block_number: T::BlockNumber,
        ppfa_index: u32,
        proposer: ValidatorId,
    ) {
        PPFAHistory::<T>::insert((block_number, ppfa_index), proposer);
    }
}
```

#### Step 2: Extend Runtime API

```rust
sp_api::decl_runtime_apis! {
    pub trait ValidatorManagementApi {
        // ... existing methods ...

        /// Check if proposer was authorized for specific block/ppfa_index
        fn is_proposer_authorized(
            block_number: BlockNumber,
            ppfa_index: u32,
            proposer_id: ValidatorId,
        ) -> bool;
    }
}
```

Implement:

```rust
impl_runtime_apis! {
    impl validator_apis::ValidatorManagementApi<Block> for Runtime {
        // ... existing implementations ...

        fn is_proposer_authorized(
            block_number: BlockNumber,
            ppfa_index: u32,
            proposer_id: ValidatorId,
        ) -> bool {
            ValidatorManagement::ppfa_history((block_number, ppfa_index))
                .map(|authorized| authorized == proposer_id)
                .unwrap_or(false)
        }
    }
}
```

#### Step 3: Implement Block Import Validation

In `asf_service.rs`, update block import logic:

```rust
use sp_consensus::BlockImport as BlockImportTrait;

struct AsfBlockImport<I> {
    inner: I,
    client: Arc<FullClient>,
}

impl<I> BlockImportTrait<Block> for AsfBlockImport<I>
where
    I: BlockImportTrait<Block>,
{
    type Error = I::Error;

    async fn import_block(
        &mut self,
        block: BlockImportParams<Block>,
    ) -> Result<ImportResult, Self::Error> {
        // Extract PPFA digest from block header
        let ppfa_digest = self.extract_ppfa_digest(&block.header);

        if let Some((ppfa_index, proposer_id)) = ppfa_digest {
            // Query runtime to verify authorization
            let is_authorized = self.client.runtime_api()
                .is_proposer_authorized(
                    block.header.parent_hash(),
                    *block.header.number(),
                    ppfa_index,
                    proposer_id,
                )
                .unwrap_or(false);

            if !is_authorized {
                log::warn!(
                    "❌ Rejecting block #{}: Unauthorized proposer (PPFA index: {})",
                    block.header.number(),
                    ppfa_index
                );

                return Ok(ImportResult::KnownBad);
            }
        }

        // Delegate to inner block import (GRANDPA)
        self.inner.import_block(block).await
    }
}

impl AsfBlockImport {
    fn extract_ppfa_digest(&self, header: &Header) -> Option<(u32, ValidatorId)> {
        use sp_runtime::generic::DigestItem;

        for digest in header.digest().logs() {
            if let DigestItem::PreRuntime(engine_id, data) = digest {
                if engine_id == &b"PPFA"[..] {
                    // Decode PPFA digest
                    if let Ok((ppfa_index, proposer_id)) = codec::Decode::decode(&mut &data[..]) {
                        return Some((ppfa_index, proposer_id));
                    }
                }
            }
        }

        None
    }
}
```

### Dependencies

- PPFA digest must be added to block headers during production
- Runtime storage for authorization history
- Block import wrapper

### Estimated Effort

- **Runtime Authorization Tracking:** 3 days
- **PPFA Digest Implementation:** 2 days
- **Block Import Validation:** 3 days
- **Testing:** 3 days
- **Total:** ~2 weeks

---

## Implementation Roadmap

### Phase 1: Development Environment Setup (Week 1)

**Tasks:**
1. Create `pallets/pallet-validator-management/` directory structure
2. Set up Cargo.toml with dependencies
3. Add pallet to runtime Cargo.toml
4. Create test environment

**Deliverables:**
- Pallet skeleton compiles
- Runtime includes pallet
- Basic integration test passes

### Phase 2: Core Pallet Implementation (Week 1-2)

**Tasks:**
1. Implement validator storage and management
2. Add committee selection logic
3. Implement epoch transition logic
4. Add PPFA authorization tracking
5. Write comprehensive unit tests

**Deliverables:**
- Pallet with full storage and extrinsics
- Committee selection algorithm tested
- Epoch transitions working
- 80%+ test coverage

### Phase 3: Runtime API Integration (Week 2)

**Tasks:**
1. Define Runtime API traits
2. Implement API in runtime
3. Add API types to client
4. Write integration tests

**Deliverables:**
- Runtime APIs callable from node
- Type conversions working
- Integration tests passing

### Phase 4: Service Layer Updates (Week 2-3)

**Tasks:**
1. Implement keystore integration (TODO #2)
2. Update committee loading (TODO #1)
3. Implement epoch transitions (TODO #3)
4. Add PPFA authorization (TODO #4)
5. Refactor error handling

**Deliverables:**
- All 4 TODOs resolved
- Keystore integration working
- Runtime API queries functional
- Comprehensive logging

### Phase 5: Testing & Validation (Week 3)

**Tasks:**
1. Run full node with new implementation
2. Multi-node testnet deployment
3. Epoch transition testing
4. Byzantine fault testing
5. Performance benchmarking

**Deliverables:**
- 3-node testnet running
- Successful epoch transitions
- Committee rotations working
- Performance metrics documented

### Phase 6: Documentation & Handoff (Week 3)

**Tasks:**
1. Update developer documentation
2. Create operator guides
3. Document Runtime APIs
4. Write migration guide from placeholder logic

**Deliverables:**
- Complete API documentation
- Operator runbooks
- Migration guide

---

## Risk Assessment

### High Risk

1. **Runtime API Compatibility**
   - **Risk:** Breaking changes in Runtime API between development and deployment
   - **Mitigation:** Version Runtime APIs, use feature flags
   - **Impact:** High (could require runtime upgrade)

2. **Epoch Transition Edge Cases**
   - **Risk:** Network split during epoch transition
   - **Mitigation:** Implement robust finality checks, buffer zones
   - **Impact:** High (could halt block production)

### Medium Risk

1. **Keystore Key Management**
   - **Risk:** Missing or invalid keys in production
   - **Mitigation:** Pre-deployment key verification, clear error messages
   - **Impact:** Medium (node won't participate but won't crash)

2. **Committee Selection Determinism**
   - **Risk:** Different nodes selecting different committees
   - **Mitigation:** Deterministic sorting, comprehensive testing
   - **Impact:** Medium (fork risk)

### Low Risk

1. **Performance Overhead**
   - **Risk:** Runtime API calls slow down block production
   - **Mitigation:** Cache results, benchmark early
   - **Impact:** Low (can be optimized)

---

## Testing Strategy

### Unit Tests

- Validator storage and retrieval
- Committee selection algorithm
- Epoch transition logic
- PPFA authorization checks

### Integration Tests

- Runtime API calls from node
- Keystore integration
- Full committee rotation workflow
- Block import with authorization

### End-to-End Tests

- 3-node testnet with epoch transitions
- Validator join/leave during epoch
- Network partition recovery
- 24-hour continuous operation

### Performance Tests

- Runtime API call latency (target: <10ms)
- Committee selection time (target: <100ms)
- Block import overhead (target: <5% increase)

---

## Success Criteria

### Functionality

- ✅ All 4 TODOs resolved
- ✅ Runtime APIs operational
- ✅ Keystore integration working
- ✅ Epoch transitions smooth
- ✅ PPFA authorization enforced

### Performance

- ✅ Block time: 6 seconds (unchanged)
- ✅ Runtime API latency: <10ms
- ✅ Committee rotation: <1 second
- ✅ No memory leaks during 24h test

### Quality

- ✅ Test coverage: ≥80%
- ✅ All clippy warnings resolved
- ✅ Documentation complete
- ✅ Code review approved

---

## Dependencies

### External Dependencies

- `polkadot-stable2509` (already updated)
- `validator-management` crate (exists at `09-consensus/validator-management`)
- `block-production` crate (exists at `09-consensus/block-production`)

### Internal Dependencies

- FlareChain runtime must include pallet
- Runtime must implement APIs
- Node service must use APIs

### Tooling Dependencies

- Rust 1.77+
- cargo-tarpaulin (testing)
- cargo-audit (security)

---

## Appendix A: Key Type Registration

### Registering ASF Validator Key

```rust
// In runtime/src/lib.rs

sp_api::impl_runtime_apis! {
    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }
}

// Register ASF key type
pub mod opaque {
    use super::*;
    use sp_core::crypto::KeyTypeId;

    pub const ASF_KEY_TYPE: KeyTypeId = KeyTypeId(*b"asfv");

    #[derive(Default, Clone, Encode, Decode, TypeInfo)]
    pub struct SessionKeys {
        pub grandpa: GrandpaId,
        pub asf: AsfId, // New ASF validator key
    }
}
```

---

## Appendix B: Runtime API Call Examples

### From Node Service

```rust
// Get active validators
let validators = client.runtime_api()
    .get_active_validators(parent_hash)
    .map_err(|e| format!("Runtime API call failed: {:?}", e))?;

// Check committee membership
let is_member = client.runtime_api()
    .is_in_committee(parent_hash, validator_id)
    .map_err(|e| format!("Runtime API call failed: {:?}", e))?;

// Get current epoch
let epoch = client.runtime_api()
    .current_epoch(parent_hash)
    .map_err(|e| format!("Runtime API call failed: {:?}", e))?;
```

---

## Appendix C: Migration from Placeholder Logic

### Before (Placeholder - Current)

```rust
// Hardcoded 3 test validators
for i in 0..3 {
    let validator_id = block_production::ValidatorId::from([i as u8; 32]);
    committee.add_validator(validator_info);
}
```

### After (Production - Target)

```rust
// Query runtime for real validators
let validators = ppfa_client.runtime_api()
    .get_active_validators(parent_hash)?;

for validator in validators {
    committee.add_validator(validator);
}
```

### Deployment Steps

1. Deploy new runtime with `pallet-validator-management`
2. Initialize genesis validators via sudo
3. Restart nodes with updated service layer
4. Verify committee loaded from runtime
5. Monitor first epoch transition

---

**END OF IMPLEMENTATION PLAN**

**Status:** Ready for implementation
**Next Action:** Begin Phase 1 - Development Environment Setup
**Contact:** Development team for questions or clarifications
