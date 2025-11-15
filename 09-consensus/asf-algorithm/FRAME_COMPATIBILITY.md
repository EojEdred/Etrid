# FRAME Storage Compatibility

This document describes the FRAME storage compatibility status for ASF algorithm types.

## MaxEncodedLen Trait Support

FRAME v2 storage requires types to implement `MaxEncodedLen` to ensure bounded storage size.
The following types have been updated to support this trait:

### ✅ Fully Compatible Types

These types now derive `MaxEncodedLen` and can be used directly in FRAME storage:

| Type | Location | Notes |
|------|----------|-------|
| `ConsensusPhase` | `src/lib.rs:68` | Simple enum with 4 variants |
| `FinalityLevel` | `src/lib.rs:126` | Simple enum with 5 variants (0-4) |
| `Signature` | `src/crypto.rs:28` | Enum wrapping Sr25519/Ed25519 signatures (fixed 64 bytes) |

### ⚠️ Partially Compatible Types

These types **cannot** derive `MaxEncodedLen` due to unbounded `Vec` collections:

| Type | Location | Issue | FRAME Storage Solution |
|------|----------|-------|------------------------|
| `Vote` | `src/votes.rs:21` | Contains `Signature` (✅) but used in collections | Use `BoundedVec<Vote, MaxVotes>` in storage |
| `ValidityCertificate` | `src/certificates.rs:22` | Contains `VoteAggregate` and `AggregateSignature` | Use `BoundedVec<ValidityCertificate, MaxCerts>` |
| `VoteAggregate` | `src/votes.rs:240` | Contains `Vec<ValidatorId>` | Refactor to use `BoundedVec<ValidatorId, MaxValidators>` |
| `AggregateSignature` | `src/crypto.rs:250` | Contains `Vec<Signature>` and `Vec<ValidatorId>` | Refactor to use `BoundedVec` for both fields |

## FRAME Pallet Integration Strategy

### Approach 1: Bounded Collections (Recommended for Production)

Modify the types to use `BoundedVec` from `frame-support`:

```rust
use frame_support::BoundedVec;

#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct VoteAggregateBounded<MaxValidators: Get<u32>> {
    pub block_hash: Hash,
    pub block_number: BlockNumber,
    pub phase: ConsensusPhase,
    pub validator_count: u32,
    pub total_stake: Balance,
    pub validators: BoundedVec<ValidatorId, MaxValidators>,
}
```

**Pros:**
- Full compile-time safety
- No storage size surprises
- Optimal performance

**Cons:**
- Requires type parameter throughout codebase
- More complex API

### Approach 2: Unbounded Storage with Manual Checks (Quick Integration)

Use FRAME's unbounded storage with runtime checks:

```rust
#[pallet::storage]
#[pallet::unbounded]
pub type Votes<T: Config> = StorageMap<_, Blake2_128Concat, (ConsensusPhase, Hash), Vec<Vote>>;
```

**Pros:**
- Minimal changes to existing types
- Quick integration path

**Cons:**
- Runtime overhead for size checks
- Less compile-time safety
- Could hit storage limits

### Approach 3: Hybrid (Current Implementation)

Use simple types with `MaxEncodedLen` where possible, store complex types unbounded:

```rust
// ✅ Bounded - can calculate max size
#[pallet::storage]
pub type CurrentPhase<T: Config> = StorageValue<_, ConsensusPhase>;

#[pallet::storage]
pub type FinalityLevels<T: Config> = StorageMap<_, Blake2_128Concat, Hash, FinalityLevel>;

// ⚠️ Unbounded - needs size checks
#[pallet::storage]
#[pallet::unbounded]
pub type Certificates<T: Config> = StorageMap<_, Blake2_128Concat, Hash, Vec<ValidityCertificate>>;
```

**Pros:**
- Best of both worlds
- Gradual migration path

**Cons:**
- Mixed storage types

## Migration Path

1. **Phase 1 (COMPLETED)**: Add `MaxEncodedLen` to simple types
   - ✅ `ConsensusPhase`
   - ✅ `FinalityLevel`
   - ✅ `Signature`

2. **Phase 2 (TODO)**: Create bounded variants for complex types
   - Create `VoteAggregateBounded<MaxValidators>`
   - Create `AggregateSignatureBounded<MaxValidators>`
   - Create `ValidityCertificateBounded<MaxValidators, MaxCerts>`

3. **Phase 3 (TODO)**: Update pallet-asf to use bounded types
   - Update storage definitions
   - Add configuration bounds (`MaxValidators`, `MaxCertsPerBlock`)
   - Add runtime checks for bounds

4. **Phase 4 (TODO)**: Testing and optimization
   - Benchmark storage costs
   - Test with maximum bounds
   - Optimize for production

## Testing

All 87 library tests pass with the current MaxEncodedLen implementations:

```bash
cd /Users/macbook/Desktop/etrid/09-consensus/asf-algorithm
cargo test --lib
# Result: 87 passed; 0 failed
```

## See Also

- [FRAME Storage Documentation](https://docs.substrate.io/build/runtime-storage/)
- [BoundedVec Documentation](https://paritytech.github.io/substrate/master/frame_support/storage/bounded_vec/struct.BoundedVec.html)
- [pallet-asf Implementation](/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/pallets/pallet-asf/)
