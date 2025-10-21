# PBC (Partition Burst Chain) Issues Report

## Summary
Multiple PBC chains have compilation issues after migration to polkadot-stable2506. This report categorizes and prioritizes fixes.

## Issue Categories

### 1. **Critical Runtime Issues** (Blocking Chain Functionality)

#### A. Missing `DoneSlashHandler` Trait
**Affected**: pallet-doge-bridge, chainlink-bridge, possibly others
**Error**: `error[E0046]: not all trait items implemented, missing: DoneSlashHandler`
**Fix**:
```rust
// In pallet test mocks, add to pallet_balances::Config:
type DoneSlashHandler = ();
```
**Priority**: HIGH - Blocks test compilation

#### B. Missing Frame System Trait Items
**Affected**: Multiple PBC runtimes (TRX, MATIC, SC-USDT, XLM, BNB)
**Errors**:
- `missing: RuntimeTask, ExtensionsWeightInfo, SingleBlockMigrations, MultiBlockMigrator, PreInherents, PostInherents, PostTransactions`
- `missing: RuntimeHoldReason, RuntimeFreezeReason, FreezeIdentifier, MaxFreezes`

**Fix**: Update `frame_system::Config` implementations
```rust
impl frame_system::Config for Runtime {
    // ... existing config ...
    type RuntimeTask = ();  // NEW in stable2506
    type ExtensionsWeightInfo = ();  // NEW
    type SingleBlockMigrations = ();  // NEW
    type MultiBlockMigrator = ();  // NEW
    type PreInherents = ();  // NEW
    type PostInherents = ();  // NEW
    type PostTransactions = ();  // NEW
}
```
**Priority**: CRITICAL - Blocks all affected runtime builds

#### C. Missing Balances Trait Items
**Affected**: Multiple runtimes
**Errors**:
- `missing: RuntimeHoldReason, RuntimeFreezeReason, FreezeIdentifier, MaxFreezes, DoneSlashHandler`

**Fix**: Update `pallet_balances::Config`
```rust
impl pallet_balances::Config for Runtime {
    // ... existing config ...
    type RuntimeHoldReason = ();  // NEW in stable2506
    type RuntimeFreezeReason = ();  // NEW
    type FreezeIdentifier = ();  // NEW
    type MaxFreezes = ConstU32<0>;  // NEW
    type DoneSlashHandler = ();  // NEW
}
```
**Priority**: CRITICAL

#### D. WASM_BINARY Not Available
**Affected**: BNB-PBC, XLM-PBC, TRX-PBC chain-spec.rs files
**Error**: `error[E0425]: cannot find value WASM_BINARY in crate`
**Fix**: Runtime needs to export WASM_BINARY
```rust
// In runtime/src/lib.rs, ensure:
#[cfg(feature = "std")]
pub use sp_runtime::BuildStorage;

// And the WASM builder in build.rs:
substrate_wasm_builder::WasmBuilder::new()
    .with_current_project()
    .export_heap_base()
    .import_memory()
    .build()
```
**Priority**: HIGH - Blocks node binary compilation

#### E. Solana (SOL) PBC Runtime Errors
**Affected**: sol-pbc-runtime
**Errors**:
1. `error[E0425]: cannot find value WEIGHT_REF_TIME_PER_SECOND`
2. `error[E0433]: failed to resolve: use of undeclared crate frame_support`
3. `error[E0425]: cannot find value WASM_BINARY`

**Fix**:
```rust
// Import correct weight constant
use frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND;

// Ensure frame_support is in Cargo.toml with correct features
```
**Priority**: HIGH

### 2. **Node/Collator Issues** (Non-Critical, Templates)

#### F. FullPool Type Issues
**Affected**: Multiple collator nodes
**Status**: FALSE ALARM - These are template files that panic, not real implementations
**Action**: Mark as "TODO - Implement actual collator service"
**Priority**: LOW - These are just templates

#### G. OffchainWorkers Trait Bounds
**Affected**: bnb-pbc-collator
**Error**: `the method run exists... but its trait bounds were not satisfied`
**Cause**: API change in sc-offchain
**Priority**: MEDIUM - Once runtime is fixed

### 3. **Non-Critical Issues**

#### H. Deprecated CurrencyAdapter
**Affected**: bnb-pbc-runtime
**Warning**: `use of deprecated struct pallet_transaction_payment::CurrencyAdapter`
**Fix**: Migrate to `FungibleAdapter`
**Priority**: LOW - Just a deprecation warning

#### I. AccountId Privacy
**Affected**: XLM-PBC, TRX-PBC chain specs
**Error**: `error[E0603]: type alias AccountId is private`
**Fix**: Make AccountId public in runtime
```rust
pub type AccountId = sp_runtime::AccountId32;
```
**Priority**: LOW

## Recommended Fix Order

1. **Phase 1: Runtime Trait Updates** (Affects 6+ PBC chains)
   - Add missing frame_system::Config items
   - Add missing pallet_balances::Config items
   - Update all PBC runtimes with stable2506 requirements
   - Estimated time: 1-2 hours

2. **Phase 2: WASM Binary Fixes**
   - Fix BNB, XLM, TRX runtime WASM exports
   - Update build.rs files if needed
   - Estimated time: 30 minutes

3. **Phase 3: SOL PBC Specific**
   - Fix weight constant imports
   - Fix frame_support dependency
   - Estimated time: 20 minutes

4. **Phase 4: Bridge Pallet Tests**
   - Add DoneSlashHandler to test mocks
   - Estimated time: 15 minutes

5. **Phase 5: Deprecation Warnings** (Optional)
   - Migrate CurrencyAdapter to FungibleAdapter
   - Estimated time: 30 minutes

## Success Metrics

- [ ] All 12 PBC runtimes compile without errors
- [ ] All bridge pallets pass tests
- [ ] At least BTC-PBC, ETH-PBC, BNB-PBC collators compile
- [ ] FlareChain can connect to at least one PBC

## Notes

- Total estimated fix time: 3-4 hours for phases 1-4
- Collator service implementations are templates and can be deferred
- Focus on runtime/pallet compilation first
- Actual collator logic implementation is a separate project

## Files Requiring Updates

### Critical (Phase 1):
- `/05-multichain/partition-burst-chains/pbc-chains/*/runtime/src/lib.rs` (12 files)
- `/05-multichain/bridge-protocols/*/src/lib.rs` (test mocks)

### Important (Phase 2):
- `/05-multichain/partition-burst-chains/pbc-chains/{bnb,xlm,trx}-pbc/runtime/build.rs`
- `/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/{bnb,xlm,trx}-pbc-collator/src/chain_spec.rs`

### Medium (Phase 3):
- `/05-multichain/partition-burst-chains/pbc-chains/sol-pbc/runtime/src/lib.rs`

---

**Generated**: 2025-10-17
**Status**: Ready for systematic fixes
