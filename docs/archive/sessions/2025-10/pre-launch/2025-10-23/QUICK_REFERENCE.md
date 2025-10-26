# Ëtrid Quick Reference Card

**Fast access to all commands, scripts, and documentation**

---

## 🚀 Quick Start (30 seconds)

```bash
# Build everything
./scripts/build-all.sh --release

# Run all tests
./scripts/test-all.sh

# Start local testnet
./scripts/start-testnet.sh

# Generate & view docs
./scripts/generate-docs.sh --open
```

---

## 📚 Documentation Quick Access

| Guide | Command | Topics |
|-------|---------|--------|
| **User Guide** | `cat docs/USER_GUIDE.md` | Wallet, staking, governance |
| **API Reference** | `cat docs/API_REFERENCE.md` | All 8 pallets, RPC, types |
| **Operator Guide** | `cat docs/OPERATOR_GUIDE.md` | Validators, monitoring |
| **Developer Guide** | `cat docs/DEVELOPER_GUIDE.md` | Pallets, DApps, contracts |

---

## 🛠️ Build Commands

```bash
# Development build
./scripts/build-all.sh

# Production build (optimized)
./scripts/build-all.sh --release

# Build only Rust
./scripts/build-all.sh --skip-frontend --skip-sdk

# Build only frontend
./scripts/build-all.sh --skip-rust

# Clean and rebuild
./scripts/build-all.sh --clean --release

# Show help
./scripts/build-all.sh --help
```

**Build time:** ~8-10 minutes (release mode)

---

## 🧪 Test Commands

```bash
# Run all tests
./scripts/test-all.sh

# Run with coverage report
./scripts/test-all.sh --coverage

# Skip integration tests
./scripts/test-all.sh --skip-integration

# Skip frontend tests
./scripts/test-all.sh --skip-frontend

# Verbose output
./scripts/test-all.sh --verbose

# Rust tests only
./scripts/test-all.sh --rust-only
```

**Test time:** ~2-3 minutes (all tests)

---

## 🌐 Testnet Commands

```bash
# Start basic testnet (3 validators)
./scripts/start-testnet.sh

# Start with monitoring
./scripts/start-testnet.sh --with-monitoring

# Start with PBCs
./scripts/start-testnet.sh --with-pbcs

# Custom validator count
./scripts/start-testnet.sh --validators 5

# Clean start (purge data)
./scripts/start-testnet.sh --purge

# Stop testnet: Ctrl+C
```

**Endpoints after start:**
- RPC: `ws://localhost:9944`
- Prometheus: `http://localhost:9090`
- Grafana: `http://localhost:3001`

---

## 🚀 Deployment Commands

```bash
# Deploy to Vercel (production)
./scripts/deploy-all.sh

# Deploy to staging
./scripts/deploy-all.sh --environment staging

# Deploy to Docker
./scripts/deploy-all.sh --target docker

# Deploy specific apps
./scripts/deploy-all.sh --skip-validator --skip-watchtower

# Skip tests
./scripts/deploy-all.sh --skip-tests

# Dry run (preview)
./scripts/deploy-all.sh --dry-run

# Rollback
./scripts/deploy-all.sh --rollback
```

**Deploy time:** ~4-5 minutes per app

---

## 📖 Documentation Generation

```bash
# Generate all docs
./scripts/generate-docs.sh

# Generate and open in browser
./scripts/generate-docs.sh --open

# Rust docs only
./scripts/generate-docs.sh --rust-only

# SDK docs only
./scripts/generate-docs.sh --sdk-only

# TypeScript types only
./scripts/generate-docs.sh --types-only

# Custom output directory
./scripts/generate-docs.sh --output ./public/docs

# Deploy to GitHub Pages
./scripts/generate-docs.sh --deploy
```

**Output location:** `docs/generated/index.html`

---

## 🎥 Video Tutorial Scripts

| Tutorial | Duration | File | Topics |
|----------|----------|------|--------|
| **01: Getting Started** | 5 min | `docs/video-tutorials/01-getting-started.md` | Wallet, first transaction |
| **02: Running Validator** | 10 min | `docs/video-tutorials/02-running-validator.md` | Validator setup |
| **03: Staking** | 7 min | `docs/video-tutorials/03-staking-nominator.md` | Nominating, rewards |
| **04: Smart Contracts** | 12 min | `docs/video-tutorials/04-deploying-smart-contracts.md` | ink! contracts |
| **05: Building DApps** | 15 min | `docs/video-tutorials/05-building-dapps.md` | React + Polkadot.js |

**Total tutorial time:** 49 minutes

---

## 🔍 Common Tasks

### Search Documentation

```bash
# Find wallet setup instructions
grep -i "wallet" docs/USER_GUIDE.md

# Find staking information
grep -i "staking" docs/USER_GUIDE.md

# Find specific pallet docs
grep -A 20 "pallet-reserve-oracle" docs/API_REFERENCE.md

# Search all docs
grep -r "cross-chain" docs/
```

### View Specific Sections

```bash
# User Guide: Wallet setup (lines 100-300)
sed -n '100,300p' docs/USER_GUIDE.md

# User Guide: Staking (lines 500-700)
sed -n '500,700p' docs/USER_GUIDE.md

# API Reference: Reserve Oracle
sed -n '/pallet-reserve-oracle/,/pallet-reserve-vault/p' docs/API_REFERENCE.md
```

### Check Script Help

```bash
./scripts/build-all.sh --help
./scripts/test-all.sh --help
./scripts/start-testnet.sh --help
./scripts/deploy-all.sh --help
./scripts/generate-docs.sh --help
```

---

## 📊 Project Stats

```bash
# Count all documentation files
find docs -name "*.md" | wc -l
# Output: 212 files

# Total documentation lines
wc -l docs/*.md docs/video-tutorials/*.md
# Output: 9,721 lines

# Total automation script lines
wc -l scripts/*.sh
# Output: 2,144 lines across 17 scripts

# Check executable scripts
ls -lh scripts/*.sh | grep "^-rwxr"
# Output: 17 executable scripts
```

---

## 🎯 Typical Workflows

### New User Workflow

```bash
# 1. Read user guide
cat docs/USER_GUIDE.md | less

# 2. Follow getting started tutorial
cat docs/video-tutorials/01-getting-started.md

# 3. Get testnet tokens (visit faucet.etrid.io)

# 4. Explore staking
cat docs/video-tutorials/03-staking-nominator.md
```

### Developer Workflow

```bash
# 1. Set up development environment
./scripts/build-all.sh

# 2. Start local testnet
./scripts/start-testnet.sh

# 3. Read developer guide
cat docs/DEVELOPER_GUIDE.md

# 4. Generate API docs
./scripts/generate-docs.sh --open

# 5. Build smart contract (see tutorial 04)
cat docs/video-tutorials/04-deploying-smart-contracts.md

# 6. Build DApp (see tutorial 05)
cat docs/video-tutorials/05-building-dapps.md
```

### Operator Workflow

```bash
# 1. Read operator guide
cat docs/OPERATOR_GUIDE.md

# 2. Follow validator tutorial
cat docs/video-tutorials/02-running-validator.md

# 3. Set up monitoring
# (Follow monitoring section in operator guide)

# 4. Deploy to production
# (Follow deployment section in operator guide)
```

### CI/CD Workflow

```bash
# 1. Build
./scripts/build-all.sh --release

# 2. Test
./scripts/test-all.sh --coverage

# 3. Generate docs
./scripts/generate-docs.sh --output ./public

# 4. Deploy
./scripts/deploy-all.sh --environment production
```

---

## 🆘 Troubleshooting

### Build Issues

```bash
# Clean and rebuild
./scripts/build-all.sh --clean

# Check Rust version
rustc --version
# Required: 1.70+

# Check Node version
node --version
# Required: 18.0+

# Update dependencies
cargo update
npm install
```

### Test Failures

```bash
# Run tests with verbose output
./scripts/test-all.sh --verbose

# Run specific test
cargo test test_name -- --nocapture

# Check test coverage
./scripts/test-all.sh --coverage
```

### Testnet Issues

```bash
# Purge old data
./scripts/start-testnet.sh --purge

# Check if ports are in use
lsof -i :9944

# View logs
tail -f /tmp/validator-alice.log
```

### Deployment Issues

```bash
# Dry run first
./scripts/deploy-all.sh --dry-run

# Check environment variables
echo $VERCEL_TOKEN
echo $AWS_PROFILE

# Rollback if needed
./scripts/deploy-all.sh --rollback
```

---

## 🔗 Important Links

### Local Development

- **Testnet RPC:** ws://localhost:9944
- **Prometheus:** http://localhost:9090
- **Grafana:** http://localhost:3001

### Etrid Network

- **Testnet RPC:** wss://rpc-testnet.etrid.io
- **Mainnet RPC:** wss://rpc.etrid.io
- **Faucet:** https://faucet.etrid.io
- **Explorer:** https://explorer.etrid.io

### Documentation

- **User Guide:** docs/USER_GUIDE.md
- **API Docs:** docs/API_REFERENCE.md
- **Operator Guide:** docs/OPERATOR_GUIDE.md
- **Developer Guide:** docs/DEVELOPER_GUIDE.md

### Community

- **Discord:** discord.gg/etrid
- **Telegram:** t.me/EtridOfficial
- **Twitter:** @EtridMultichain
- **GitHub:** github.com/etrid/etrid

---

## 💡 Pro Tips

**Faster Builds:**
```bash
# Use parallel compilation
export CARGO_BUILD_JOBS=$(nproc)
./scripts/build-all.sh --release
```

**Better Logging:**
```bash
# Export detailed logs
RUST_LOG=debug ./scripts/start-testnet.sh 2>&1 | tee testnet.log
```

**Quick Searches:**
```bash
# Create aliases in ~/.bashrc or ~/.zshrc
alias etrid-docs='cat docs/USER_GUIDE.md | less'
alias etrid-build='./scripts/build-all.sh --release'
alias etrid-test='./scripts/test-all.sh'
alias etrid-dev='./scripts/start-testnet.sh'
```

**Documentation Bookmarks:**
```bash
# Create symlinks for quick access
ln -s docs/USER_GUIDE.md ~/etrid-user-guide.md
ln -s docs/DEVELOPER_GUIDE.md ~/etrid-dev-guide.md
```

---

## 📦 File Locations Summary

| Type | Location | Count |
|------|----------|-------|
| **Documentation** | `docs/*.md` | 212 files |
| **Scripts** | `scripts/*.sh` | 17 scripts |
| **Tutorials** | `docs/video-tutorials/*.md` | 5 scripts |
| **Generated Docs** | `docs/generated/` | Auto-created |
| **Contracts** | `contracts/` | User-created |
| **Apps** | `apps/` | 3+ apps |

---

## ⚡ One-Liners

```bash
# Show all documentation
ls -1 docs/*.md

# Show all scripts
ls -1 scripts/*.sh

# Show all tutorials
ls -1 docs/video-tutorials/*.md

# Count total lines of documentation
find docs -name "*.md" -exec wc -l {} + | tail -1

# Show script permissions
ls -lh scripts/*.sh | grep "^-rwxr"

# Test all scripts (syntax check)
for script in scripts/*.sh; do bash -n "$script" && echo "✓ $script"; done

# Generate complete documentation
./scripts/generate-docs.sh --open

# Full build and test
./scripts/build-all.sh --release && ./scripts/test-all.sh
```

---

## 📞 Getting Help

**Documentation:**
```bash
# Search all docs
grep -r "your search term" docs/

# View specific guide
cat docs/USER_GUIDE.md | less
```

**Script Help:**
```bash
# Any script with --help
./scripts/build-all.sh --help
```

**Community:**
- Discord: #help channel
- Telegram: @EtridSupport
- GitHub: github.com/etrid/etrid/issues

**Email:**
- General: hello@etrid.io
- Technical: dev@etrid.io
- Documentation: docs@etrid.io

---

**Quick Reference Card v1.0.0**
**Last Updated:** October 22, 2025

**Print this card or save as bookmark for quick access!**
