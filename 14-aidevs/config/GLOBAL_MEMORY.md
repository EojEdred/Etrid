# Ëtrid AI Devs - Global Memory Context

**Last Updated:** October 24, 2025
**Purpose:** Shared knowledge base for all AI Dev agents

---

## 🎯 Project Context

### What is Ëtrid?

Ëtrid is a Layer 0 multichain blockchain protocol implementing the E³20 (Essential Elements to Operate) architecture:

- **13 Core Components:** All at 100% Alpha completion
- **Consensus:** Adaptive Stake Finality (ASF) + PPFA proposer selection
- **Architecture:** FlareChain relay chain + 13 Partition Burst Chains (PBCs)
- **Tokens:** ÉTR (native coin) + ËDSC (dollar-pegged stablecoin)
- **Smart Contracts:** ËtwasmVM (WebAssembly) with reentrancy protection
- **Identity:** OpenDID + AIDID (world's first AI DID standard)
- **Layer 2:** Lightning-Bloc payment channels with watchtowers

### Current Phase

- **Status:** Alpha Complete (100%)
- **Next Milestone:** Ember Testnet (Q1 2026)
- **Long-Term:** Mainnet Launch (Q2 2026), First Consensus Day (December 1, 2026)

---

## 📁 Workspace Structure

```
/workspace/
├── 01-detr-p2p/            # Lightning-Bloc P2P networking
├── 02-open-did/            # Identity (OpenDID + AIDID)
├── 03-security/            # Post-quantum cryptography
├── 04-accounts/            # Account types + social recovery
├── 05-multichain/          # FlareChain + 13 PBCs + bridges
├── 06-native-currency/     # ÉTR, ËDSC, VMw tokens
├── 07-transactions/        # Transaction system + Lightning-Bloc
├── 08-etwasm-vm/           # WebAssembly smart contracts
├── 09-consensus/           # ASF consensus algorithm
├── 10-foundation/          # Governance framework
├── 11-peer-roles/          # Staking and nominations
├── 12-consensus-day/       # Annual governance voting
├── 13-clients/             # Wallets, CLI, 4 SDKs
├── pallets/                # Custom Substrate pallets
├── contracts/              # Ethereum/Base/BSC smart contracts
├── docs/                   # 32,000+ lines of documentation
├── scripts/                # Build and deployment automation
└── ai-devs/                # AI Devs MCP orchestrator (YOU ARE HERE)
```

---

## 🏗️ Technical Stack

### Blockchain
- **Framework:** Substrate (Polkadot SDK stable2506)
- **Language:** Rust 1.70+
- **Consensus:** ASF (Adaptive Stake Finality)
- **Block Time:** ~6 seconds
- **Finality:** <100 blocks

### Smart Contracts
- **VM:** ËtwasmVM (custom WebAssembly runtime)
- **Languages:** Rust (ink!), AssemblyScript
- **Gas:** VMw (Virtual Machine work units)

### Bridges
- **13 External Chains:** BTC, ETH, DOGE, SOL, XLM, XRP, BNB, TRX, ADA, LINK, MATIC, USDT, ËDSC
- **Security:** 3-of-5 multi-sig watchtowers
- **Verification:** SPV proofs, event logs, state proofs

---

## 🔑 Key Concepts

### E³20 Components (All 13)

1. **DETR P2P** - Decentralized Etrid Relay Protocol (custom libp2p replacement)
2. **OpenDID** - Decentralized identity + AIDID for AI agents
3. **Security** - Post-quantum cryptography (Ed25519 + SPHINCS+)
4. **Accounts** - Multi-signature + social recovery
5. **Multichain** - FlareChain + 13 PBCs
6. **Native Currency** - ÉTR (coin) + ËDSC (stablecoin) + VMw (gas)
7. **Transactions** - Ed25519 signatures + HTLCs
8. **ËtwasmVM** - WebAssembly with reentrancy protection
9. **Consensus** - ASF + PPFA (Partial Proof of Formulated Authority)
10. **Foundation** - Stake-weighted governance
11. **Peer Roles** - Staking + nominations
12. **Consensus Day** - Annual governance event (December 1)
13. **Clients** - CLI, web, mobile + 4 SDKs

### ASF Consensus

**Adaptive Stake Finality** combines:
- Stake-weighted voting (like Tendermint)
- Coin-age weighting (stake * time held)
- PPFA proposer selection (randomized but deterministic)
- Byzantine Fault Tolerance (BFT)

**Key Features:**
- Validator committee rotation every epoch
- No mining or PoW
- Low energy consumption
- Fast finality (<100 blocks)

### Consensus Day

**Annual on-chain governance event (December 1)**

Every stakeholder votes on:
- **Budget allocation** for next year
- **Protocol upgrades** (runtime changes)
- **Board member selection** (if Foundation exists)
- **Parameter changes** (fees, limits, etc.)

Voting power: Stake-weighted (1 ÉTR = 1 vote)

### ËDSC Stablecoin

**Ëtrid Dollar** - Algorithmic stablecoin pegged to $1 USD

- **Collateral:** Multi-chain reserves (BTC, ETH, stablecoins)
- **Reserve Ratio:** Maintained >150%
- **Oracle:** Multi-source price feeds with anomaly detection
- **Redemption:** Authorized Participants (APs) can mint/redeem
- **Regulation:** Compliance framework built-in

---

## 🛠️ Build Commands

### Compilation
```bash
# Full workspace build
cargo build --release

# Specific component
cargo build --release -p flare-runtime

# WASM runtime
cargo build --release --features runtime-benchmarks
```

### Testing
```bash
# All tests
cargo test --workspace

# Specific pallet
cargo test -p pallet-did-registry

# Property-based tests
cd tests/property-based && PROPTEST_CASES=5000 cargo test --release
```

### Node Operations
```bash
# Run FlareChain validator (dev mode)
./target/release/etrid --chain flare --validator --dev

# Run PBC collator (e.g., BTC)
./target/release/btc-pbc-collator --chain btc --collator

# Purge chain data
./target/release/etrid purge-chain --chain flare --dev
```

---

## 📊 Metrics & Monitoring

### Current Stats
- **Test Coverage:** 87.3% (412+ tests)
- **Documentation:** 32,000+ lines
- **Code:** 2.8M+ lines (production)
- **Components:** 13/13 (100% complete)

### Key Metrics to Monitor
- **Block time:** Should be ~6 seconds
- **Finality lag:** Should be <100 blocks
- **Validator count:** Target 50+ for Ember testnet
- **Reserve ratio:** ËDSC should maintain >150%
- **Bridge health:** All 13 PBCs should be operational

---

## 🚨 Known Issues & TODOs

### High Priority
1. **Oracle Pallet:** Test compilation errors in `pallet-reserve-oracle`
   - File: `/workspace/pallets/pallet-reserve-oracle/src/lib.rs`
   - Issue: Missing trait implementations
   - Impact: Blocks test coverage report

2. **Infrastructure:** Ember testnet not yet deployed
   - Need: 3 validators + 13 collators
   - Timeline: Q1 2026

3. **Security Audit:** Not yet scheduled
   - Need: 2 audit firms
   - Budget: $50k-100k
   - Timeline: Q1 2026

### Medium Priority
- UI applications need deployment (4 apps)
- SDK improvements (WebSocket reconnection, batching)
- Documentation updates for Ember branding

---

## 🎯 AI Dev Agent Responsibilities

### Compiler AI
- **Primary:** Keep codebase compiling
- **Auto-trigger:** On git push, fix compilation errors
- **Escalate:** If >3 failed attempts, notify human
- **Memory:** Track frequent error patterns

### Governance AI
- **Primary:** Monitor governance proposals
- **Auto-trigger:** Generate monthly community proposals
- **Escalate:** Compliance issues, ethical concerns
- **Memory:** Track voting patterns, proposal success rates

### Runtime AI
- **Primary:** Monitor node health
- **Auto-trigger:** Restart unhealthy nodes
- **Escalate:** Runtime upgrade failures
- **Memory:** Track upgrade history, performance metrics

### Economics AI
- **Primary:** Monitor ËDSC reserve ratio
- **Auto-trigger:** Alert if ratio <150%
- **Escalate:** Bridge failures, reserve depletion
- **Memory:** Track reserve trends, bridge volumes

### Security AI
- **Primary:** Detect security threats
- **Auto-trigger:** Audit new code changes
- **Escalate:** Critical vulnerabilities, slashing events
- **Memory:** Track threat patterns, false positives

### Oracle AI
- **Primary:** Monitor price feeds
- **Auto-trigger:** Alert on price anomalies
- **Escalate:** Oracle failures, manipulation attempts
- **Memory:** Track price volatility, anomaly patterns

---

## 📚 Important Files

### Configuration
- `/workspace/Cargo.toml` - Workspace dependencies
- `/workspace/pallets/*/Cargo.toml` - Pallet configs
- `/workspace/05-multichain/flare-chain/runtime/src/lib.rs` - Runtime definition

### Documentation
- `/workspace/docs/specifications/ivory-paper.md` - Complete protocol spec
- `/workspace/docs/architecture.md` - System architecture
- `/workspace/docs/API_REFERENCE.md` - API documentation
- `/workspace/LIVING_ROADMAP.md` - Development roadmap

### Scripts
- `/workspace/scripts/build-all.sh` - Build everything
- `/workspace/scripts/test-all.sh` - Run all tests
- `/workspace/scripts/start-testnet.sh` - Launch Ember testnet

---

## 🔐 Security Protocols

### Multi-Sig Operations
All bridge operations require 3-of-5 signatures from watchtowers:
- Never approve without verifying source
- Log all signature attempts
- Escalate suspicious patterns

### Slashing Conditions
Validators can be slashed for:
- Double-signing blocks
- Extended downtime (>1 hour)
- Invalid block proposals
- Byzantine behavior

### Code Review
Before executing any code changes:
1. Verify source (git commit hash)
2. Run security audit
3. Check for malicious patterns
4. Test in isolated environment

---

## 💡 Best Practices

### When to Escalate to Humans
- Compilation errors persist after 3 fix attempts
- Security vulnerabilities detected (any severity)
- Governance proposals with compliance issues
- Reserve ratio drops below 150%
- Any action requiring >$10k value transfer

### Memory Management
- Store all executions in VectorDB
- Track patterns and anomalies
- Learn from past errors
- Share knowledge across agents

### Error Handling
- Always capture full error context
- Log to VectorDB before retrying
- Max 3 retry attempts
- Exponential backoff (5s, 15s, 45s)

---

## 🎓 Learning Resources

### Substrate Development
- Substrate Docs: https://docs.substrate.io
- Polkadot SDK: https://github.com/paritytech/polkadot-sdk
- Rust Book: https://doc.rust-lang.org/book/

### Ëtrid-Specific
- Read `/workspace/docs/specifications/ivory-paper.md` first
- Understand E³20 components
- Study ASF consensus mechanism
- Review ËDSC stablecoin design

---

**This document is the foundation of your knowledge. Consult it frequently. Update it as you learn.**

---

*Last Updated: October 24, 2025 by AI Devs Orchestrator*
