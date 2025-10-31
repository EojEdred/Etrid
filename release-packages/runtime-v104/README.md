# FlareChain Runtime v104 Release

**Build Date:** 2025-10-31
**Spec Version:** 104
**Polkadot SDK:** polkadot-stable2509

## Overview

This release includes the complete treasury/payment system integration with all bridge protocols, completing the runtime upgrade from v103 to v104.

## Key Features

### Treasury Integrations
- **Bitcoin Bridge:** 10% cross-chain fee routing to treasury
- **Ethereum Bridge:** 10% cross-chain fee routing to treasury
- **Solana Bridge:** 10% cross-chain fee routing to treasury
- **Oracle Network:** 50% slashing proceeds routing to treasury

### Bridge Protocols Active
- Bitcoin (BTC)
- Ethereum (ETH)
- Solana (SOL)
- Dogecoin (DOGE)
- Stellar (XLM)
- Ripple (XRP)
- Cardano (ADA)
- Chainlink (LINK)
- Polygon (MATIC)
- BNB Chain (BNB)
- Tron (TRX)
- USDT Bridge

### EDSC Stablecoin System
- EDSC token issuance and redemption
- Multi-asset reserve management
- Custodian registry
- Oracle price feeds
- Circuit breaker mechanisms

## Files

- **flare_chain_runtime.compact.compressed.wasm** (991 KB) - Production runtime for on-chain deployment
- **flare_chain_runtime.compact.wasm** (4.7 MB) - Compact runtime
- **flare_chain_runtime.wasm** (5.0 MB) - Full runtime with debug symbols
- **checksums.txt** - SHA256 checksums for verification

## Verification

Verify file integrity using:
```bash
shasum -a 256 -c checksums.txt
```

## Deployment

### On-Chain Upgrade
```bash
# Use the compressed runtime for on-chain deployment
polkadot-js-api tx.system.setCode \
  --seed "//Alice" \
  @flare_chain_runtime.compact.compressed.wasm
```

### Local Node
```bash
# Use the full runtime for local development
./flare-chain-node --chain local --alice \
  --wasm-execution compiled \
  --runtime flare_chain_runtime.wasm
```

## Technical Details

### Dependencies Fixed
- Added `etrid-bridge-common` to workspace members
- Added `etrid-bridge-common` dependency to runtime
- Fixed trait implementations for `BridgeTreasuryInterface` and `OracleTreasuryNotifier`

### Build Environment
- Rust: 1.89.0
- Cargo: workspace-based build
- Target: wasm32-unknown-unknown
- Optimization: release profile

## Changes from v103

1. **Treasury Integration Complete**
   - All bridge pallets now route fees to treasury
   - Oracle slashing proceeds integrated
   - Payment scheduling system operational

2. **Dependency Resolution**
   - Fixed workspace configuration for bridge common crate
   - Resolved trait bound issues with concrete types
   - Updated Cargo.toml for all bridge pallets

3. **Runtime Configuration**
   - Added treasury interface adapters
   - Configured treasury types for all bridges
   - Enabled oracle treasury notifications

## Testing

Build was verified with:
```
cargo build --release -p flare-chain-runtime
```

Result: ✓ Compilation successful (0 errors, 8 warnings)

## Checksums

```
9a1be34effc3f771b932c6d66cb97b693ee4d2bd45183a2f72cc089c75df80e7  flare_chain_runtime.compact.compressed.wasm
e7d3134a4e8692d3920d24e15ccf8793b0ca17e0f05217715a194c01eb607024  flare_chain_runtime.compact.wasm
d4b229110f7cc86c4f43d02800c36873ba632543b4fa78394642ad2a1d03a4ee  flare_chain_runtime.wasm
```

## Support

For issues or questions regarding this runtime release:
- GitHub Issues: https://github.com/etrid/etrid
- Documentation: /docs/RUNTIME_UPGRADE_GUIDE.md
