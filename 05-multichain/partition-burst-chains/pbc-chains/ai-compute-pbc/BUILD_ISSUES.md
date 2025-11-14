# AI Compute PBC Build Issues

## CURRENT STATUS (2025-11-13)

The AI Compute PBC workspace has been successfully sandboxed with its own `Cargo.toml` and is no longer affected by root workspace issues.

**Build Status**: BLOCKED by aidid builder pattern inconsistencies

**FlareChain Status**: ✅ BUILDING - Root workspace eth-pbc conflicts resolved

**Latest Update**:
- ✅ Fixed MaxEncodedLen imports (ConstU32 from frame_support)
- ✅ Added sp-io dependency for blake2_256 hashing
- ❌ BLOCKED: attestation.rs builder methods still use Vec<u8> instead of BoundedVec
- REMAINING WORK: Update all builder methods in aidid/src/attestation.rs to use BoundedVec

## Critical Issues Preventing Compilation

### 1. AIDID Pallet - Missing DecodeWithMemTracking (BLOCKING)

**Error**: Multiple types in the `aidid` pallet cannot be used in pallet extrinsics because they contain `Vec<T>` fields

**Location**: `/Users/macbook/Desktop/etrid/02-open-did/aidid/src/types.rs`

**Root Cause**:
- Types used in pallet extrinsic parameters must implement `DecodeWithMemTracking`
- This trait requires `MaxEncodedLen`, which `Vec<T>` cannot provide (unbounded length)
- Substrate's pallet macro enforces this for memory safety

**Affected Types**:
- `AIProfile` (line 214)
- `ModelAttestation` (line 186)
- `Permission` (line 248)

All three types contain `Vec<u8>` fields for dynamic-length data like training hashes, model versions, etc.

**Build Errors**:
```
error[E0277]: the trait bound `types::AIProfile: DecodeWithMemTracking` is not satisfied
error[E0277]: the trait bound `types::ModelAttestation: DecodeWithMemTracking` is not satisfied
error[E0277]: the trait bound `types::Permission: DecodeWithMemTracking` is not satisfied
```

**Solution Options**:

1. **Use BoundedVec** (Recommended for production):
   - Replace all `Vec<u8>` with `BoundedVec<u8, ConstU32<MAX_LEN>>`
   - Requires defining reasonable max lengths for each field
   - Example: `training_data_hash: BoundedVec<u8, ConstU32<64>>`
   - This provides memory bounds while maintaining flexibility

2. **Restructure Pallet Storage** (Simpler, less storage efficient):
   - Store these complex types in separate storage maps
   - Use simple identifiers (like `H256` hashes) in extrinsic parameters
   - Look up full data from storage within extrinsic implementation

3. **Skip MaxEncodedLen for specific types** (Quick fix, not recommended):
   - Add `#[codec(mel_bound())]` or similar attributes
   - May cause issues with newer Substrate versions
   - Not a long-term solution

**Impact**: Until fixed, the AI Compute PBC **cannot compile**

**Files Needing Changes**:
- `/Users/macbook/Desktop/etrid/02-open-did/aidid/src/types.rs` - Type definitions
- Potentially `/Users/macbook/Desktop/etrid/02-open-did/aidid/src/registry.rs` - Pallet implementation

---

### 2. RESOLVED: Workspace Isolation

**Previous Issue**: eth-pbc workspace conflicts
**Resolution**: Created standalone workspace at `05-multichain/partition-burst-chains/pbc-chains/ai-compute-pbc/Cargo.toml`
**Status**: ✅ FIXED

---

### 3. RESOLVED: Path Dependencies

**Previous Issue**: Incorrect relative paths to etrid-primitives and pallet-accounts
**Resolution**: Verified correct path depth (5 levels: `../../../../../`)
**Status**: ✅ FIXED

---

## Summary

| Issue | Severity | Fix Complexity |
|-------|----------|----------------|
| Missing Randomness | HIGH | Easy - add pallet + 1 line |
| Missing Tokenomics types | HIGH | Easy - add 5 constants |
| Workspace Cargo error | CRITICAL | Unknown - external to our PBC |

## Build Test Plan

1. **Fix workspace Cargo.toml** (eth-pbc issue)
2. **Add Randomness pallet** to runtime dependencies
3. **Fix tokenomics Config** with all 5 tier constants
4. **Verify pallet Cargo.toml** files have correct dependencies
5. **Run `cargo check`** on runtime
6. **Run `cargo build --release`** on runtime
7. **Test with substrate node** binary

## Will It Run in Etrid Ecosystem?

**Architecture**: YES ✓
- Partition Burst Chain (PBC) design is correct
- Independent runtime, only checkpoint submissions to FlareChain
- XCM integration properly configured

**Integration**: YES ✓
- Uses Etrid primitives (etrid-primitives, pallet-accounts)
- AIDID integration for model registry
- Compatible with FlareChain relay

**Compilation**: NO ✗ (fixable)
- 2 runtime Config implementation bugs (missing types)
- 1 workspace-level blocker (unrelated eth-pbc issue)

**Deployment**: UNKNOWN
- Would need to test collator binary
- Would need to verify XCM channels work with FlareChain
- Would need genesis config for initial validators

## Recommendation

**Fix the 2 runtime Config issues** (10 minutes of work), then it should compile successfully. The architecture and integration are solid - just missing some trait implementations in the runtime configuration.
