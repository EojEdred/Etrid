# Phase 4: GRANDPA Removal - Cargo.toml Changes

**Status**: PREPARED - DO NOT APPLY YET

## Runtime Cargo.toml
**File**: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/Cargo.toml`

### Dependencies to REMOVE (Lines 113-114)

```toml
# REMOVE:
pallet-grandpa = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2509", default-features = false }
```

### Primitives to REMOVE (Line 133)

```toml
# REMOVE:
sp-consensus-grandpa = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2509", default-features = false }
```

### Feature flags to REMOVE (Lines 254, 267)

```toml
# In [features] std section, REMOVE:
"pallet-grandpa/std",          # Line 254
"sp-consensus-grandpa/std",    # Line 267
```

## Node Cargo.toml
**File**: `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/Cargo.toml`

### Client Dependencies to REMOVE (Line 39)

```toml
# REMOVE:
sc-consensus-grandpa = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2509" }
```

### Primitives to REMOVE (Line 52)

```toml
# REMOVE:
sp-consensus-grandpa = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-stable2509" }
```

## Summary

### Runtime Cargo.toml
- Remove: 1 pallet dependency (pallet-grandpa)
- Remove: 1 primitive dependency (sp-consensus-grandpa)
- Remove: 2 std feature flags

### Node Cargo.toml
- Remove: 1 client dependency (sc-consensus-grandpa)
- Remove: 1 primitive dependency (sp-consensus-grandpa)

Total: 6 dependency removals across 2 files

## Verification Commands

```bash
# After changes, verify no GRANDPA references remain:
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime
grep -i "grandpa" Cargo.toml
# Should return: No matches

cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
grep -i "grandpa" Cargo.toml
# Should return: No matches
```

## Dependency Tree Impact

Removing these dependencies will:
- Reduce binary size by ~2-3MB
- Remove GRANDPA finality gadget from runtime
- Simplify session key management (ASF-only)
- Clean up unused consensus code paths
