# PBC Release Quick Start

## When Your PBC Binary is Ready

### One-Command Release (Easiest)

```bash
# From etrid root directory
./scripts/create-pbc-release.sh eth 1.0.0
```

That's it! The script will:
- ✓ Find and test your binary
- ✓ Create git tag
- ✓ Compress binary
- ✓ Generate release notes
- ✓ Create GitHub release
- ✓ Upload binary

### Supported PBCs (14 Total)

```bash
./scripts/create-pbc-release.sh eth 1.0.0          # Ethereum
./scripts/create-pbc-release.sh btc 1.0.0          # Bitcoin
./scripts/create-pbc-release.sh sol 1.0.0          # Solana
./scripts/create-pbc-release.sh trx 1.0.0          # Tron
./scripts/create-pbc-release.sh bnb 1.0.0          # BNB Chain
./scripts/create-pbc-release.sh xrp 1.0.0          # XRP Ledger
./scripts/create-pbc-release.sh ada 1.0.0          # Cardano
./scripts/create-pbc-release.sh xlm 1.0.0          # Stellar
./scripts/create-pbc-release.sh doge 1.0.0         # Dogecoin
./scripts/create-pbc-release.sh link 1.0.0         # Chainlink
./scripts/create-pbc-release.sh matic 1.0.0        # Polygon
./scripts/create-pbc-release.sh sc-usdt 1.0.0      # USDT
./scripts/create-pbc-release.sh edsc 1.0.0         # EDSC
./scripts/create-pbc-release.sh ai-compute 1.0.0   # AI Compute
```

### Custom Binary Path

If your binary is elsewhere:

```bash
./scripts/create-pbc-release.sh eth 1.0.0 /path/to/custom/eth-pbc-collator
```

## Before Running Release Script

### 1. Build Your PBC Binary

```bash
# Example for ETH PBC
cd 05-multichain/partition-burst-chains/eth-pbc-workspace
cargo build --release -p eth-pbc-collator

# Binary will be at:
# target/release/eth-pbc-collator
```

### 2. Test Binary

```bash
# Verify it works
./target/release/eth-pbc-collator --version
./target/release/eth-pbc-collator --help
```

### 3. Choose Version Number

Follow [Semantic Versioning](https://semver.org/):
- **1.0.0** - First stable release
- **1.1.0** - New features (backward compatible)
- **1.0.1** - Bug fixes only
- **2.0.0** - Breaking changes

## Manual Release (If Script Fails)

### 1. Create Tag

```bash
git tag -a eth-pbc-v1.0.0 -m "ETH PBC v1.0.0"
git push origin eth-pbc-v1.0.0
```

### 2. Compress Binary

```bash
gzip -c target/release/eth-pbc-collator > eth-pbc-collator-v1.0.0-linux-x86_64.gz
```

### 3. Create GitHub Release

```bash
gh release create eth-pbc-v1.0.0 \
  eth-pbc-collator-v1.0.0-linux-x86_64.gz \
  --title "ETH PBC Collator v1.0.0" \
  --notes "Ethereum bridge collator for Ëtrid"
```

## Release Checklist

- [ ] Binary built for Linux x86_64
- [ ] Binary tested (`--version`, `--help`)
- [ ] Version number chosen
- [ ] Run `./scripts/create-pbc-release.sh <pbc> <version>`
- [ ] Verify release on GitHub
- [ ] Update `docs/PBC_RELEASE_GUIDE.md` release matrix
- [ ] Announce to community

## Example: Full ETH PBC Release

```bash
# 1. Build binary
cd 05-multichain/partition-burst-chains/eth-pbc-workspace
cargo build --release -p eth-pbc-collator

# 2. Test it
./target/release/eth-pbc-collator --version

# 3. Return to root
cd ../../..

# 4. Create release
./scripts/create-pbc-release.sh eth 1.0.0

# Done! Check: https://github.com/EojEdred/Etrid/releases
```

## Troubleshooting

### "Binary not found"
```bash
# Make sure you built it first:
cd 05-multichain/partition-burst-chains/eth-pbc-workspace
cargo build --release -p eth-pbc-collator
```

### "Tag already exists"
```bash
# Delete old tag first:
git tag -d eth-pbc-v1.0.0
git push origin :refs/tags/eth-pbc-v1.0.0
```

### "gh command not found"
```bash
# Install GitHub CLI:
brew install gh
# or visit: https://cli.github.com/
```

## Need Help?

- Full guide: `docs/PBC_RELEASE_GUIDE.md`
- Script source: `scripts/create-pbc-release.sh`
- Example releases: https://github.com/EojEdred/Etrid/releases
