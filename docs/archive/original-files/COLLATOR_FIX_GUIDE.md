# Collator Compilation Fix Guide

**Purpose**: Document common collator compilation issues and their fixes
**Context**: Post-bridge integration, verifying collator nodes compile with ASF consensus

---

## Common Collator Issues

### Issue 1: Spawn Task Type Mismatch

**Error Pattern**:
```
error[E0271]: type mismatch resolving `<impl Future<Output = ()> as Future>::Output == ()`
  --> service.rs:XXX:XX
   |
   | task: impl Future<Output = ()> + Send + 'static,
   |                  ^^^^^^^^^^^ expected `()`, found some other type
```

**Root Cause**: Service task spawning with incorrect return types after ASF integration

**Fix**:
1. Check `service.rs` for task spawn calls
2. Ensure tasks return `()` not `Result<(), _>`
3. Update spawn calls to match ASF consensus requirements

**Example Fix**:
```rust
// Before (might fail)
task_manager.spawn_essential_handle().spawn_blocking(
    "some-task",
    Some("block-authoring"),
    run_something(), // Returns Result<(), Error>
);

// After (should work)
task_manager.spawn_essential_handle().spawn_blocking(
    "some-task",
    Some("block-authoring"),
    async move {
        if let Err(e) = run_something().await {
            log::error!("Task failed: {:?}", e);
        }
    },
);
```

### Issue 2: Missing Imports After ASF Integration

**Error Pattern**:
```
error[E0433]: failed to resolve: use of undeclared crate or module `sp_consensus_asf`
```

**Root Cause**: Collator Cargo.toml missing ASF consensus dependencies

**Fix**:
Add to `Cargo.toml`:
```toml
[dependencies]
sp-consensus-asf = { workspace = true }
sc-consensus-asf = { workspace = true }
```

### Issue 3: Runtime API Mismatch

**Error Pattern**:
```
error[E0599]: no method named `asf_api` found for type `RuntimeApi`
```

**Root Cause**: Collator trying to use ASF runtime APIs that aren't exposed

**Fix**:
Check runtime `lib.rs` has:
```rust
impl sp_consensus_asf::AsfApi<Block> for Runtime {
    fn authorities() -> Vec<AsfId> {
        Asf::authorities()
    }
}
```

---

## Verification Checklist

For each failing collator, verify:

1. ✅ Runtime compiles successfully
2. ✅ Collator Cargo.toml has all ASF dependencies
3. ✅ Service.rs properly imports ASF modules
4. ✅ Task spawning uses correct async patterns
5. ✅ Runtime API implementations match what service.rs expects

---

## Testing Individual Collators

```bash
# Test specific collator with full output
env SKIP_WASM_BUILD=1 cargo check -p btc-pbc-collator 2>&1 | less

# Test specific collator - errors only
env SKIP_WASM_BUILD=1 cargo check -p eth-pbc-collator 2>&1 | grep "error:"

# Test with color output for easier reading
env SKIP_WASM_BUILD=1 cargo check -p doge-pbc-collator --color=always 2>&1 | less -R
```

---

## Systematic Fix Approach

1. **Identify**: Run comprehensive test to find all failing collators
2. **Group**: Group failures by error type
3. **Fix Pattern**: Fix one collator of each error type
4. **Apply**: Apply same fix to other collators with same error
5. **Validate**: Re-run comprehensive test
6. **Iterate**: Repeat until all pass

---

## Expected Results After Fixes

```
PHASE 1: Testing all 12 PBC Runtimes
========================================
Testing btc-pbc-runtime... ✅ PASS
Testing eth-pbc-runtime... ✅ PASS
Testing doge-pbc-runtime... ✅ PASS
Testing xlm-pbc-runtime... ✅ PASS
Testing xrp-pbc-runtime... ✅ PASS
Testing bnb-pbc-runtime... ✅ PASS
Testing trx-pbc-runtime... ✅ PASS
Testing ada-pbc-runtime... ✅ PASS
Testing link-pbc-runtime... ✅ PASS
Testing matic-pbc-runtime... ✅ PASS
Testing sc-usdt-pbc-runtime... ✅ PASS
Testing sol-pbc-runtime... ✅ PASS

PHASE 2: Testing all 12 PBC Collators
========================================
Testing btc-pbc-collator... ✅ PASS
Testing eth-pbc-collator... ✅ PASS
Testing doge-pbc-collator... ✅ PASS
Testing xlm-pbc-collator... ✅ PASS
Testing xrp-pbc-collator... ✅ PASS
Testing bnb-pbc-collator... ✅ PASS
Testing trx-pbc-collator... ✅ PASS
Testing ada-pbc-collator... ✅ PASS
Testing link-pbc-collator... ✅ PASS
Testing matic-pbc-collator... ✅ PASS
Testing sc-usdt-pbc-collator... ✅ PASS
Testing sol-pbc-collator... ✅ PASS

========================================
FINAL RESULTS
========================================
Runtimes:  12/12 passed
Collators: 12/12 passed
Total:     24/24 components passed
========================================
✅ ALL TESTS PASSED!
```

---

## Reference: Collator File Locations

```
05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/
├── btc-pbc-collator/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── chain_spec.rs
│       ├── rpc.rs
│       └── service.rs  ← Usually where issues are
├── eth-pbc-collator/
│   └── ...
├── doge-pbc-collator/
│   └── ...
...
```

---

*Guide Created: October 18, 2025*
*For: Collator compilation issues post-ASF integration*
