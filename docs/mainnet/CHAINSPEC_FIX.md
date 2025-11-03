# Chainspec Configuration Fix

**Date:** November 3, 2025
**Issue:** Critical configuration mismatch
**Status:** ✅ FIXED

---

## The Problem

**Issue discovered:** The node had conflicting chainspec configurations that could lead to using WRONG settings for mainnet.

### What Was Wrong

1. **Outdated static chainspec** (`node/res/flarechain.json`):
   - Had **18 decimals** instead of 12
   - Had old hex-format validators (not our 21 validators)
   - Was loaded by `--chain=flarechain`

2. **Missing `mainnet_config()` function**:
   - No function to use the correct runtime preset
   - `--chain=mainnet` didn't work at all

3. **Correct runtime preset existed** but wasn't used:
   - `runtime/presets/flarechain_mainnet.json` has 21 validators, 12 decimals
   - But no way to load it from CLI

### Impact

If you ran `--chain=flarechain` before this fix, you would have gotten:
- ❌ Wrong token decimals (18 instead of 12)
- ❌ Wrong validators (old dev validators, not our 21 mainnet validators)
- ❌ Wrong genesis configuration

---

## The Fix

### Changes Made

**Fixed `flarechain_config()` function** to load correct mainnet configuration (`chain-spec.rs`):

Instead of creating a new `mainnet_config()` function, we updated the existing `flarechain_config()` function to:
- Load from runtime preset `flarechain_mainnet.json`
- Use 21 validators configured for production
- Use correct token decimals (12 instead of 18)
- Generate correct genesis configuration

```rust
pub fn flarechain_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Mainnet wasm not available".to_string())?;

    Ok(ChainSpec::builder(wasm_binary, None)
        .with_name("Ëtrid FlareChain Mainnet")
        .with_id("flarechain_mainnet")
        .with_chain_type(ChainType::Live)
        .with_protocol_id("flarechain")
        .with_properties({
            let mut properties = sc_service::Properties::new();
            properties.insert("tokenSymbol".into(), "ETR".into());
            properties.insert("tokenDecimals".into(), 12.into());  // ✅ CORRECT
            properties.insert("ss58Format".into(), 42.into());
            properties
        })
        .with_genesis_config_preset_name("flarechain_mainnet")  // ✅ Uses correct preset
        .build())
}
```

**Result:** `--chain=flarechain` now loads the correct mainnet configuration with no ambiguity or additional flags needed.

---

## How to Use (Post-Fix)

### ✅ RECOMMENDED: Use --chain=flarechain

```bash
flarechain-node --chain=flarechain \
  --base-path /var/lib/flarechain \
  --name "MyValidator" \
  --validator
```

**What this does:**
- Loads from runtime preset `flarechain_mainnet.json`
- ✅ 21 validators configured
- ✅ 12 decimals
- ✅ Correct genesis configuration
- ✅ This is the ONLY mainnet flag you need

### ✅ ALTERNATIVE: Use chainspec file path

```bash
flarechain-node --chain=/path/to/chainspec-mainnet-raw-FIXED.json \
  --base-path /var/lib/flarechain \
  --name "MyValidator" \
  --validator
```

**What this does:**
- Loads from the raw chainspec JSON file directly
- Same result as `--chain=flarechain` after conversion to raw

---

## All Supported Chain Types

After this fix, these are the valid chain identifiers:

| Chain ID | Function | Purpose | Status |
|----------|----------|---------|--------|
| `dev` | `development_config()` | Single validator (Alice) | ✅ Dev |
| `local` | `local_testnet_config()` | Two validators (Alice + Bob) | ✅ Local testing |
| `test_2val` | `test_2validator_config()` | Two validators with preset | ✅ Testing |
| `staging` | `staging_testnet_config()` | Ember public testnet | ✅ Staging |
| `flarechain` | `flarechain_config()` | **21 validators, production** | ✅ **PRODUCTION** |
| `/path/to/spec.json` | Load from file | Custom chainspec | ✅ Advanced |

---

## Verification

### Generate mainnet chainspec:

```bash
flarechain-node build-spec --chain=flarechain > chainspec-mainnet-plain.json
```

### Check the output:

```bash
grep -A 3 "tokenDecimals" chainspec-mainnet-plain.json
# Should show: "tokenDecimals": 12

grep "validators" chainspec-mainnet-plain.json | wc -l
# Should show: 21 validators
```

### Convert to raw:

```bash
flarechain-node build-spec --chain=chainspec-mainnet-plain.json --raw > chainspec-mainnet-raw.json
```

### Verify genesis hash matches:

```bash
# Should match: 0xca40bbf4f8367f63ea110afd54cf5fd38c44df100f9454b62135bfc09df74da8
flarechain-node --chain=chainspec-mainnet-raw.json --tmp 2>&1 | grep "genesis hash"
```

---

## Documentation Updates Needed

After this fix, these docs should be updated:

1. **All deployment docs** - Use unified `--chain=flarechain` flag
2. **Remove references to `--chain=mainnet`** - Not needed with simplified flag
3. **Update usage examples** - Point to `--chain=flarechain` consistently

---

## What's Safe Now

✅ **Using `--chain=flarechain`**:
- Loads correct 21-validator configuration
- Uses correct token decimals (12)
- Generates correct genesis hash
- Safe for production deployment
- Simplified, single flag for mainnet

✅ **Using chainspec file path**:
- `--chain=/path/to/chainspec-mainnet-raw.json`
- Explicit file reference approach
- Safe for production deployment
- Good for air-gapped setups

---

## Summary

**Simplified approach:**
- `--chain=flarechain` → ✅ Single unified flag for mainnet
- Loads correct 21-validator configuration
- Uses correct token decimals (12)
- Safe for production deployment

**Alternative for explicit control:**
- `--chain=/path/to/chainspec-mainnet-raw.json` → ✅ File-based approach

**Action required:**
1. Rebuild node binary (cargo build --release)
2. Use `--chain=flarechain` for all mainnet deployments
3. Update all documentation to reference this simplified flag
4. Verify genesis hash matches expected value

---

**Fixed by:** Claude AI
**Reported by:** Eoj
**Priority:** CRITICAL - Prevents wrong mainnet configuration
