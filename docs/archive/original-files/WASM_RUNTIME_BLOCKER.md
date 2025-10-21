# Ëtrid WASM Runtime Blocker - GenesisBuilder API Missing

**Date:** October 19, 2025
**Status:** 🔴 **BLOCKED - PBC Collators Cannot Start with WASM**

---

## 🎯 Objective

Successfully start PBC collators with full WASM runtime to enable bridge functionality testing.

---

## ❌ Problem Identified

### Error Message

```
Error: Service(Client(Storage("wasm call error Other: Exported method GenesisBuilder_get_preset is not found")))
```

### When It Occurs

- When trying to start any PBC collator with WASM runtime
- Both `--dev` and `--chain local` modes fail
- Attempting to use chain spec file also fails

### Root Cause

The PBC runtimes are missing the `GenesisBuilder` API implementation, which is required by Polkadot SDK `polkadot-stable2506` for chain initialization.

---

## 🔍 Technical Details

### What We Attempted

**Attempt 1: Using existing chain spec**
```bash
./target/release/btc-pbc-collator \
  --validator \
  --chain chain-specs/pbc-btc-local.json \
  --relay-chain-rpc ws://127.0.0.1:9955
```
**Result:** Chain spec format error - `runtime` field not recognized (expects `runtimeGenesis`)

**Attempt 2: Generate new chain spec**
```bash
./target/release/btc-pbc-collator build-spec --chain local --disable-default-bootnode
```
**Result:** `Error: GenesisBuilder_get_preset is not found`

**Attempt 3: Dev mode**
```bash
./target/release/btc-pbc-collator --dev --relay-chain-rpc ws://127.0.0.1:9955
```
**Result:** Same `GenesisBuilder_get_preset` error

### What Works

✅ **FlareChain starts successfully with WASM**
- FlareChain runtime includes proper GenesisBuilder implementation
- Node starts, RPC responds, block production active
- Full WASM runtime available (654KB compressed)

❌ **All 12 PBC collators fail to start**
- BTC, ETH, DOGE, SOL, XLM, XRP, BNB, TRX, ADA, LINK, MATIC, SC-USDT
- All hit the same GenesisBuilder error
- WASM runtimes compiled successfully (~270-281KB each)
- But cannot initialize due to missing runtime API

---

## 📊 Build Success vs Runtime Failure

| Component | WASM Build | Runtime Start | Issue |
|-----------|------------|---------------|-------|
| **FlareChain** | ✅ Success | ✅ Success | None |
| **BTC PBC** | ✅ Success | ❌ Fails | GenesisBuilder missing |
| **ETH PBC** | ✅ Success | ❌ Fails | GenesisBuilder missing |
| **All other PBCs** | ✅ Success | ❌ Fails | GenesisBuilder missing |

**Key Finding:** WASM compilation succeeds, but runtime initialization fails.

---

## 🎓 Understanding the GenesisBuilder API

### What It Is

The `GenesisBuilder` is a Substrate runtime API introduced in recent Polkadot SDK versions that provides:

1. **Genesis Configuration Management**
   - `GenesisBuilder_get_preset` - Returns predefined genesis configurations
   - `GenesisBuilder_build_config` - Builds genesis from JSON config
   - Replaces old chain spec `runtime` field with `runtimeGenesis`

2. **Why It's Required**
   - Modern Substrate chains use this for chain initialization
   - Enables runtime-defined genesis instead of client-defined
   - Required for `--dev` and `--chain local` modes

### What's Missing in PBC Runtimes

The PBC runtimes currently do NOT implement:

```rust
// Missing from PBC runtimes:
impl_runtime_apis! {
    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_config(json: Vec<u8>) -> sp_genesis_builder::Result {
            // Implementation needed
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            // Implementation needed
        }
    }
}
```

---

## 🛠️ Solution Options

### Option 1: Implement GenesisBuilder API (Recommended for Production)

**What to do:**
Add GenesisBuilder implementation to all 12 PBC runtimes.

**Files to modify:**
```
05-multichain/partition-burst-chains/pbc-chains/btc-pbc/runtime/src/lib.rs
05-multichain/partition-burst-chains/pbc-chains/eth-pbc/runtime/src/lib.rs
... (10 more PBC runtimes)
```

**Implementation example:**
```rust
impl_runtime_apis! {
    // ... existing APIs ...

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_config(json: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(json)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }
    }
}
```

**Estimated Effort:** 2-3 hours (implement, test, rebuild all 12 PBC runtimes)

**Benefits:**
- Production-ready solution
- Future-proof for Polkadot SDK updates
- Enables all chain initialization modes

**Drawbacks:**
- Requires code changes across 12 runtimes
- Need to rebuild all WASM runtimes (~30-40 min)
- Needs testing for each PBC

---

### Option 2: Downgrade Polkadot SDK (Not Recommended)

**What to do:**
Rollback from `polkadot-stable2506` to an older tag that doesn't require GenesisBuilder.

**Why NOT recommended:**
- ❌ Lose access to latest features
- ❌ May introduce other incompatibilities
- ❌ Not future-proof
- ❌ FlareChain already works with current SDK

---

### Option 3: Use Runtime Testing Framework (Temporary Solution)

**What to do:**
Test bridge pallets using Substrate's runtime testing framework instead of running actual nodes.

**How:**
```bash
# Run existing bridge integration tests
cargo test --test bridge_integration_tests
```

**Benefits:**
- ✅ Tests bridge pallet logic directly
- ✅ No need for running nodes
- ✅ Faster iteration
- ✅ Already implemented in codebase

**Drawbacks:**
- ❌ Doesn't test cross-chain message passing
- ❌ Doesn't validate relay chain integration
- ❌ Not end-to-end testing

**Files available:**
- `tests/bridge_integration_tests.rs` - Bridge test framework
- `run_bridge_tests.sh` - Test runner script

---

### Option 4: Test FlareChain Only (Immediate Validation)

**What to do:**
Validate WASM runtime functionality using FlareChain only, defer PBC testing.

**What can be tested:**
- ✅ WASM runtime upgrades on FlareChain
- ✅ Multi-validator consensus
- ✅ Session key management
- ✅ Peer connectivity and finality
- ✅ RPC functionality

**What cannot be tested:**
- ❌ Cross-chain bridge operations
- ❌ PBC collator functionality
- ❌ Parachain-relay chain communication

---

## 📝 Current Status Summary

### Completed ✅
1. All 13 components built with WASM (FlareChain + 12 PBCs)
2. FlareChain running successfully with WASM
3. WASM runtimes generated for all PBCs
4. Blocker identified and documented

### Blocked ❌
1. Cannot start any PBC collators
2. Cannot test bridge functionality end-to-end
3. Cannot validate parachain-relay chain integration

---

## 🚀 Recommended Path Forward

### Immediate (This Session)

**Option 3: Runtime Testing**
```bash
# Test bridge pallets at runtime level
./run_bridge_tests.sh
```

**Option 4: FlareChain Validation**
- Test WASM runtime upgrade on FlareChain
- Validate multi-validator functionality
- Document FlareChain capabilities

### Short-Term (Next Session)

**Option 1: Implement GenesisBuilder**
1. Add GenesisBuilder API to one PBC runtime (BTC) as proof of concept
2. Test that it starts successfully
3. Roll out to all 12 PBC runtimes
4. Rebuild WASM runtimes
5. Test full bridge functionality

---

## 📊 Impact Assessment

### What This Blocks

- **Bridge Testing:** Cannot test cross-chain bridge operations with live nodes
- **PBC Collators:** Cannot demonstrate parachain collator functionality
- **Integration Testing:** Cannot test FlareChain ↔ PBC communication

### What This Does NOT Block

- **WASM Runtime Upgrades:** FlareChain can demonstrate forkless upgrades
- **Consensus Testing:** FlareChain multi-validator testing works
- **Pallet Testing:** Bridge pallets can be tested via runtime tests
- **Code Quality:** All code compiles successfully

---

## 🎯 Success Metrics

### To Consider This Blocker Resolved

- [ ] All 12 PBC collators start successfully with WASM
- [ ] Can connect PBC collator to FlareChain relay chain
- [ ] Can submit bridge transaction from PBC to FlareChain
- [ ] Can verify cross-chain state updates

### Partial Success (Current State)

- [x] FlareChain starts with WASM
- [x] All WASM runtimes compile
- [x] Bridge pallets exist in codebase
- [ ] PBC collators cannot initialize

---

## 📚 Reference Information

### Related Files

**Documentation:**
- `SESSION_OCT19_CONTINUED.md` - Previous session progress
- `WASM_BUILD_PROGRESS.md` - All WASM builds completed
- `PEER_CONNECTIVITY_PROGRESS.md` - Peer connectivity fix

**Test Scripts:**
- `run_bridge_tests.sh` - Bridge runtime tests
- `scripts/deploy_local_testnet.sh` - Deployment script (also hits this error)

**Chain Specs:**
- `chain-specs/flarechain-shared.json` - FlareChain (works)
- `chain-specs/pbc-btc-local.json` - BTC PBC (outdated format)

### Substrate Resources

- [Polkadot SDK GenesisBuilder API](https://paritytech.github.io/polkadot-sdk/master/sp_genesis_builder/index.html)
- [Chain Spec Generation](https://docs.substrate.io/build/chain-spec/)
- [Runtime Upgrades](https://docs.substrate.io/maintain/runtime-upgrades/)

---

## 💡 Key Learnings

1. **WASM Compilation ≠ Runtime Initialization**
   - Successfully compiling WASM doesn't guarantee the runtime can start
   - Runtime APIs must match SDK expectations

2. **Polkadot SDK Evolution**
   - GenesisBuilder API is a relatively new requirement
   - Older chain specs using `runtime` field no longer work
   - Need `runtimeGenesis` format with GenesisBuilder support

3. **Testing Strategies**
   - Runtime tests can validate pallet logic without running nodes
   - Full integration tests require proper runtime API implementation
   - Hybrid approach: runtime tests + partial node testing

---

**Last Updated:** October 19, 2025
**Session:** Continued from WASM build completion
**Next Step:** Choose solution option and implement
