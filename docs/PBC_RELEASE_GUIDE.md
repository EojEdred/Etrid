# PBC Release Guide

## Overview

Ëtrid uses a **monorepo with independent versioning** for Partition Burst Chain (PBC) collators, following the Optimism OP Stack model.

## Repository Structure

```
etrid/
├── runtime/flare-chain/              # FlareChain relay chain
├── 05-multichain/
│   └── partition-burst-chains/
│       ├── eth-pbc-workspace/        # Ethereum bridge
│       ├── btc-pbc-workspace/        # Bitcoin bridge
│       ├── sol-pbc-workspace/        # Solana bridge
│       ├── trx-pbc-workspace/        # Tron bridge
│       ├── bnb-pbc-workspace/        # BNB Chain bridge
│       ├── xrp-pbc-workspace/        # XRP Ledger bridge
│       ├── ada-pbc-workspace/        # Cardano bridge
│       ├── xlm-pbc-workspace/        # Stellar bridge
│       ├── dot-pbc-workspace/        # Polkadot bridge
│       └── ai-compute-pbc-workspace/ # AI Compute chain
```

## Versioning Strategy

### FlareChain (Relay Chain)
- Tag: `flarechain-vX.Y.Z`
- Release: Contains relay chain binary and mainnet chainspec
- Example: `flarechain-v1.0.0`

### PBC Collators (Independent Versions)
- Tag: `{chain}-pbc-vX.Y.Z`
- Release: Contains specific PBC collator binary and chainspec
- Examples:
  - `eth-pbc-v1.0.0` - Ethereum bridge collator
  - `btc-pbc-v1.2.0` - Bitcoin bridge collator
  - `sol-pbc-v0.9.0` - Solana bridge collator (beta)

## Version Numbering

### Semantic Versioning (X.Y.Z)

- **Major (X)**: Breaking changes, incompatible with previous versions
  - Example: `v2.0.0` - New consensus mechanism

- **Minor (Y)**: New features, backward compatible
  - Example: `v1.1.0` - Added new RPC endpoints

- **Patch (Z)**: Bug fixes, backward compatible
  - Example: `v1.0.1` - Fixed memory leak

### Version Alignment

PBCs **do not** need to match FlareChain versions. Each PBC can evolve independently:

```
FlareChain: v1.0.0
├── ETH-PBC:  v1.2.0  (ahead - new features)
├── BTC-PBC:  v1.0.3  (bug fixes only)
├── SOL-PBC:  v0.9.0  (beta - not production ready)
└── TRX-PBC:  v1.1.0  (minor update)
```

## Release Process

### 1. Build Binary on Linux

```bash
# From project root
cd 05-multichain/partition-burst-chains/{pbc}-pbc-workspace

# Build release binary
cargo build --release -p {pbc}-pbc-collator

# Binary location
target/release/{pbc}-pbc-collator
```

### 2. Test Binary

```bash
# Verify binary works
./{pbc}-pbc-collator --version
./{pbc}-pbc-collator --help

# Test with chainspec
./{pbc}-pbc-collator --chain {pbc}-pbc-chainspec.json --tmp
```

### 3. Create Git Tag

```bash
# Create annotated tag
git tag -a eth-pbc-v1.0.0 -m "ETH PBC Collator v1.0.0

Features:
- Ethereum bridge integration
- ERC20 token support
- Gas optimization

Compatible with FlareChain v1.0.0+"

# Push tag to GitHub
git push origin eth-pbc-v1.0.0
```

### 4. Create GitHub Release

#### Option A: Using release script (recommended)

```bash
# Use automated script
./scripts/create-pbc-release.sh eth 1.0.0
```

#### Option B: Manual release

```bash
# Create release with binary
gh release create eth-pbc-v1.0.0 \
  ~/path/to/eth-pbc-collator \
  ~/path/to/eth-pbc-chainspec.json \
  --title "ETH PBC Collator v1.0.0" \
  --notes "$(cat <<'EOF'
# ETH PBC Collator v1.0.0

## Overview
Ethereum bridge collator for Ëtrid multichain network.

## Features
- Full Ethereum RPC compatibility
- ERC20 token bridge support
- Optimized gas handling
- GRANDPA finality integration

## Installation

### Download Binary
```bash
wget https://github.com/EojEdred/Etrid/releases/download/eth-pbc-v1.0.0/eth-pbc-collator
chmod +x eth-pbc-collator
```

### Download Chainspec
```bash
wget https://github.com/EojEdred/Etrid/releases/download/eth-pbc-v1.0.0/eth-pbc-chainspec.json
```

### Run Collator
```bash
./eth-pbc-collator \
  --chain eth-pbc-chainspec.json \
  --collator \
  --base-path /var/lib/eth-pbc \
  --port 30333 \
  --rpc-port 9944
```

## Requirements
- FlareChain v1.0.0 or higher
- 4GB RAM minimum
- 100GB disk space

## Documentation
- [PBC Architecture](https://github.com/EojEdred/Etrid/blob/main/docs/architecture.md)
- [Collator Setup Guide](https://github.com/EojEdred/Etrid/blob/main/docs/OPERATOR_GUIDE.md)

## Changelog
- Initial release
- Ethereum mainnet integration
- ERC20 bridge support

Built by Gizzi / Eoj Edred
EOF
)"
```

## Release Checklist

Before creating a PBC release:

- [ ] Binary built on Linux x86_64
- [ ] Binary tested and working
- [ ] Chainspec generated and validated
- [ ] Version number chosen (semantic versioning)
- [ ] Git tag created locally
- [ ] Release notes written
- [ ] Compatible FlareChain version documented
- [ ] Installation instructions verified
- [ ] Tag pushed to GitHub
- [ ] GitHub release created
- [ ] Release announcement prepared

## PBC Release Matrix

Track version compatibility between FlareChain and PBC collators:

| PBC Chain     | Current Version | Status | FlareChain Compatibility | Release Date |
|---------------|----------------|--------|-------------------------|--------------|
| ETH-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| BTC-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| SOL-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| TRX-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| BNB-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| XRP-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| ADA-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| XLM-PBC       | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| DOGE-PBC      | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| LINK-PBC      | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| MATIC-PBC     | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| SC-USDT-PBC   | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| EDSC-PBC      | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |
| AI-PBC        | v1.0.0         | ✅ Stable | v1.0.0+ | TBD |

## FAQ

### Q: Can I run multiple PBC versions with the same FlareChain version?
**A:** Yes! PBCs are independent. You can run ETH-PBC v1.2.0 with BTC-PBC v1.0.0, both connecting to FlareChain v1.0.0.

### Q: Do I need to update all PBCs when FlareChain updates?
**A:** Not necessarily. PBCs maintain backward compatibility. Update only if the FlareChain release notes indicate breaking changes.

### Q: How do I know which PBC version to use?
**A:** Check the "FlareChain Compatibility" column in the Release Matrix. Use the latest stable version compatible with your FlareChain version.

### Q: What if a PBC has a critical bug?
**A:** We'll release a patch version (X.Y.Z+1) immediately. Subscribe to GitHub releases for notifications.

## References

- [Optimism Monorepo Strategy](https://github.com/ethereum-optimism/optimism)
- [Polkadot SDK Releases](https://github.com/paritytech/polkadot-sdk/releases)
- [Semantic Versioning](https://semver.org/)
