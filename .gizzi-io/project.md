# Ëtrid Protocol - Claude Project Context

## Project Overview

Ëtrid is a next-generation multichain blockchain platform implementing the E³20 (Essential Elements to Operate) protocol with 13 core components, all at 100% Alpha Complete status.

**Status:** Alpha Complete (100%)
**Version:** 1.0.0-alpha
**Last Updated:** October 23, 2025
**Next Milestone:** Ember Testnet Launch (Q1 2026)

## Quick Facts

- **Architecture:** FlareChain relay chain + 13 Partition Burst Chains (PBCs)
- **Consensus:** Ascending Scale of Finality (ASF)
- **Governance:** Annual Consensus Day voting (December 1st each year)
- **Tokens:** ÉTR (native), ËDSC (stablecoin), VMw (gas)
- **Smart Contracts:** ËtwasmVM (WebAssembly-based with reentrancy protection)
- **Layer 2:** Lightning-Bloc payment channels
- **Testnet:** Ember (launching Q1 2026)

## Key Statistics

- **Components Complete:** 13/13 (100%)
- **Test Cases:** 412+ (87.3% coverage)
- **Documentation:** 32,000+ lines across 73+ files
- **WASM Runtimes:** 14/14 built successfully
- **Production Code:** 2.8M+ lines
- **Node Binaries:** etrid (unified), btc-pbc-collator

## Project Structure

### Core Components (E³20)
1. `01-detr-p2p/` - Lightning-Bloc payment channels
2. `02-open-did/` - Self-sovereign identity (including AIDID - world's first AI DID)
3. `03-security/` - Cryptographic primitives (Ed25519 + SPHINCS+)
4. `04-accounts/` - Account types + social recovery
5. `05-multichain/` - FlareChain + 13 PBCs + bridges
6. `06-native-currency/` - ÉTR, ËDSC, VMw tokens
7. `07-transactions/` - Ed25519 + HTLCs
8. `08-etwasm-vm/` - WebAssembly runtime
9. `09-consensus/` - ASF finality + watchtowers
10. `10-foundation/` - Stake-weighted governance
11. `11-peer-roles/` - Staking + nominations
12. `12-consensus-day/` - Annual governance event
13. `13-clients/` - Wallets, CLI, 4 SDKs

### Documentation Files (Always Reference)

**Root Level (Industry-Standard):**
- `README.md` - Project overview with Ember testnet info
- `ROADMAP.md` - Strategic roadmap with detailed Ember plans
- `QUICK_START.md` - 5-minute setup guide
- `CONTRIBUTING.md` - Contribution guidelines
- `CODE_OF_CONDUCT.md` - Community guidelines
- `SECURITY.md` - Security policy
- `CHANGELOG.md` - Version history

**Protocol & Architecture:**
- `docs/specifications/ivory-paper.md` - Complete protocol spec (44 KB)
- `docs/architecture.md` - System architecture (35 KB)
- `docs/specifications/protocol-charter.md` - Foundation charter

**Development:**
- `docs/DEVELOPER_GUIDE.md` - Complete developer guide (80 KB)
- `docs/API_REFERENCE.md` - API documentation (57 KB)
- `docs/OPERATOR_GUIDE.md` - Operations manual (60 KB)

**Current Status & Reports:**
- `docs/reports/DOCUMENTATION_AUDIT_REPORT.md` - Documentation completeness
- `docs/reports/VALIDATION_REPORT.md` - Infrastructure validation
- `docs/archive/sessions/2025-10-23/` - Session-specific reports

**Component Architecture:**
- Each component has `ARCHITECTURE.md` and `README.md` files
- Example: `01-detr-p2p/ARCHITECTURE.md` for P2P networking details

## Important Conventions

### Code Style
- **Language:** Rust (Substrate/Polkadot SDK polkadot-stable2509)
- **Formatting:** rustfmt with default settings
- **Linting:** clippy with warnings as errors
- **Testing:** Comprehensive unit + integration + property tests

### Documentation
- All pallets have inline documentation
- Architecture files in each component
- Examples in developer guide
- API reference auto-generated from code

### Naming
- **Pallets:** `pallet-{name}` (e.g., `pallet-did-registry`)
- **Crates:** `etrid-{component}` (e.g., `etrid-p2p`)
- **Types:** PascalCase
- **Functions:** snake_case
- **Constants:** SCREAMING_SNAKE_CASE

## Common Tasks

### Building
```bash
cargo build --release
cargo build --release --bin etrid  # Unified node
```

### Testing
```bash
cargo test --workspace
./scripts/test-all.sh --coverage
```

### Running Ember Development Node
```bash
./target/release/etrid --chain flare --validator --dev
# Or: make dev
# Or: ./scripts/start-testnet.sh
```

### Generating Documentation
```bash
cargo doc --no-deps --open
./scripts/generate-docs.sh
```

## Key Technologies

- **Substrate:** Polkadot SDK stable2509
- **Consensus:** Custom ASF (Ascending Scale of Finality)
- **Cryptography:** Ed25519, ECIES, post-quantum ready (SPHINCS+)
- **Smart Contracts:** WebAssembly (WASM)
- **P2P:** Custom DETR P2P (S/Kademlia + ECIES)
- **Frontend:** Next.js, React, TypeScript
- **SDKs:** Rust, JavaScript/TypeScript, Python, Swift

## Development Status

### Completed (100%)
- All 13 E³20 components at Alpha Complete
- 14/14 WASM runtime builds
- Node binaries (etrid, btc-pbc-collator)
- SDK implementations (4 languages)
- Web wallet application
- Comprehensive documentation (32,000+ lines)
- Test infrastructure (412+ tests, 87.3% coverage)
- Ember development chain specs

### In Progress
- Preparing for Ember testnet launch (Q1 2026)
- Security audit scheduling
- Community building
- Infrastructure deployment planning

### Planned (Phase 2 - Ember Testnet Q1 2026)
- Public testnet launch
- Incentivized validator program
- Bug bounty program ($50k initial)
- Developer grants ($100k pool)
- Performance optimization

## Ember Testnet

**Ember** is Ëtrid's public incentivized testnet launching in Q1 2026.

### Key Features
- FlareChain validators with ASF consensus
- All 13 PBC collators operational
- Test ÉTR faucet for developers
- Incentive rewards for participation
- Cross-chain bridge testing
- Governance practice (Consensus Day simulations)

### Infrastructure
- 3+ validator nodes (decentralized)
- 13 PBC collator nodes
- Block explorer and statistics
- Public RPC endpoints
- WebSocket connections

### How to Participate
- Run a validator node
- Test PBC bridges
- Submit bug reports
- Build DApps
- Participate in governance simulations

## Problem-Solving Approach

When working on this project:

1. **Check Documentation First**
   - `docs/specifications/ivory-paper.md` for protocol details
   - `docs/DEVELOPER_GUIDE.md` for implementation guidance
   - Component `ARCHITECTURE.md` for specific designs

2. **Follow Existing Patterns**
   - Look at similar pallets/components
   - Match code style and structure
   - Use existing abstractions

3. **Test Thoroughly**
   - Write unit tests for all functions
   - Add integration tests for cross-component features
   - Include property tests for invariants

4. **Document Changes**
   - Update inline documentation
   - Modify ARCHITECTURE.md if needed
   - Add examples for new features

## Important Notes

- **License:** Apache 2.0 (open source, commercial use allowed)
- **Founder:** Eoj Edred
- **Repository:** https://github.com/EojEdred/Etrid
- **Status:** Alpha Complete, preparing for Ember testnet
- **Root Directory:** Industry-standard (7 .md files only)

## Quick Reference Commands

```bash
# Build everything
make build

# Run all tests
make test

# Start Ember dev node
make dev

# Generate docs
make docs

# Check code quality
make check

# Clean build artifacts
make clean
```

## When in Doubt

1. Check `README.md` for project overview and Ember testnet info
2. Check `ROADMAP.md` for detailed timeline and milestones
3. Check `docs/reports/DOCUMENTATION_AUDIT_REPORT.md` for documentation completeness
4. Check component-specific `ARCHITECTURE.md` for technical details
5. Check `docs/DEVELOPER_GUIDE.md` for development patterns

## Current Session Context (October 23, 2025)

- ✅ Root directory cleaned to industry-standard (7 .md files)
- ✅ All documentation updated with Ember testnet name
- ✅ Session-specific files archived to `docs/archive/sessions/2025-10-23/`
- ✅ Deployment docs organized in `docs/deployment/`
- ✅ Technical reports organized in `docs/reports/`
- ✅ Configuration files organized in `config/`
- ✅ All core .md files reflect current state (Alpha Complete 100%)

---

**This context file ensures Claude always has comprehensive project understanding with clean, industry-standard organization.**
