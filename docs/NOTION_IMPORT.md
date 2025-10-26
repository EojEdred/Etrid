# ËTRID WIKI - Master Import for Notion

> **Instructions**: Import this file into Notion → It will create the complete page hierarchy
> **Date**: October 24, 2025
> **Version**: 1.0.0

---

# 🌐 ËTRID PUBLIC HUB (etrid.org)

## 🏠 Home / Introduction

### Mission & Vision

**Ëtrid is a sovereign network of autonomous chains that coordinate through shared finality and transparent governance.**

Each chain operates independently yet contributes to the health of the whole — forming a self-balancing digital organism rather than a federation of bridges.

### Quick Start

- 📘 **Learn** → Understand the architecture
- 💻 **Build** → Start developing on Ëtrid
- 🗳️ **Govern** → Participate in Consensus Day

---

## 📖 About Ëtrid

### A Living Multichain System

Ëtrid is a sovereign network of autonomous chains that coordinate through shared finality and transparent governance.

Each chain operates independently yet contributes to the health of the whole — forming a self-balancing digital organism rather than a federation of bridges.

### Adaptive Sovereign Framework (ASF)

At the heart of Ëtrid lies the **Ascending Scale of Finality**, a consensus model that treats security as a living variable.

ASF measures the weight of participation and adjusts finality conditions dynamically, allowing the network to evolve with its users and sustain efficiency without sacrificing assurance.

### Partition Burst Chains (PBCs)

Ëtrid's architecture expands through **Partition Burst Chains** — parallel runtimes that specialize in specific domains while remaining connected to the FlareChain root.

Each PBC runs its own logic, resources, and tokens, but all share the same adaptive consensus and fiscal heartbeat.

**13 Active PBCs:**
- BTC (Bitcoin Bridge)
- ETH (Ethereum Bridge)
- DOGE, SOL, XLM, XRP, BNB, TRX, ADA, LINK, MATIC
- USDT (Multi-chain Stablecoin)
- EDSC (Ëtrid Dollar Stablecoin)

### Consensus Day

Governance within Ëtrid is built into the protocol itself.

**Consensus Day** is the system's recurring on-chain assembly: a time when participants vote, mint, and redistribute according to transparent schedules.

This event aligns economics, technical upgrades, and community direction in one verifiable cycle — proving that a decentralized network can govern and fund itself.

### E³20 Architecture

Underneath every chain runs the **E³20 Stack** — a modular design built for post-quantum resilience and interoperability.

**13 Core Components (All 100% Complete):**
1. DETR P2P (Lightning-Bloc)
2. OpenDID + AIDID
3. Security (Post-Quantum)
4. Accounts + Social Recovery
5. Multichain (FlareChain + 13 PBCs)
6. Native Currency (ÉTR, EDSC, VMw)
7. Transactions
8. ËtwasmVM
9. Consensus (ASF)
10. Foundation Governance
11. Peer Roles + Staking
12. Consensus Day
13. Clients (CLI, Web, Mobile, 4 SDKs)

### Philosophy

Ëtrid stands for **coordination over competition**.

Its purpose is to demonstrate that consensus and governance are not separate concerns — they are reflections of the same natural process: **adaptation**.

Through ASF, Consensus Day, and the E³20 foundation, Ëtrid becomes a system capable of long-term self-evolution — a living infrastructure for the multichain era.

---

## 🧬 Architecture Deep Dive

### System Overview

```
┌─────────────────────────────────────────┐
│         Ëtrid Ecosystem                  │
├─────────────────────────────────────────┤
│                                          │
│  FlareChain (Relay Chain)                │
│    ↓                                     │
│  13 Partition Burst Chains (PBCs)        │
│    ↓                                     │
│  Layer 2: Lightning-Bloc                 │
│    ↓                                     │
│  Client Layer (Web, Mobile, CLI, SDKs)   │
│                                          │
└─────────────────────────────────────────┘
```

### FlareChain (Root Chain)

**Purpose:** Coordination & finality anchor

**Features:**
- Adaptive Stake Finality (ASF) consensus
- Validator set management
- Cross-chain message routing
- Governance & treasury coordination
- State anchoring for all PBCs

### Adaptive Stake Finality (ASF)

**Dynamic consensus** that adjusts finality thresholds based on:
- Validator participation rate
- Time elapsed since block production
- Network conditions

ASF treats security as a **spectrum**, not a fixed threshold, allowing the network to evolve in real-time.

### Virtual Machine Watts (VMw)

**Energy-based gas metering** for deterministic computation costs:
- Replaces traditional gas models
- Based on measurable computation energy
- Predictable cost calculation
- Integrates with treasury fee flows

### Partition Burst Chains (PBCs)

**Specialized sovereign runtimes:**
- Each PBC has dedicated collator set
- Native blockchain bridges
- Specialized runtime for asset type
- Periodic state checkpoints to FlareChain

### Cross-Chain Security

**Three-layer security model:**
1. **ASF Consensus** - Dynamic finality at root
2. **Validity Nodes** - Cross-chain proof verification
3. **Multi-sig Custodians** - Bridge security (M-of-N threshold)

---

## 🗳️ Governance & Consensus Day

### The Core Idea

Ëtrid treats governance as **part of its operating system** — not a feature layered on top.

Through a process known as **Consensus Day**, the network regularly gathers all participants — validators, directors, and voters — to synchronize decisions, mint schedules, and fiscal distribution.

This moment of on-chain alignment forms the constitutional cycle of the system.

### Why Governance Is Built-In

Traditional blockchains separate decision-making from protocol logic.

Ëtrid removes that divide by **encoding governance directly into runtime modules**.

Proposals, votes, minting, and payouts all occur under the same cryptographic rules that secure every block.

**Governance here is not politics — it's mathematics.**

### The Consensus Day Cycle

Each epoch culminates in Consensus Day, which follows **four verifiable phases:**

| Phase | Description | Outcome |
|-------|-------------|---------|
| **Registration** | Participants register proposals and stake commitments | Active proposal queue established |
| **Voting** | Validators and voters cast ballots via Governance Runtime | Weighted results tallied in real time |
| **Minting** | Approved proposals trigger minting events through Fiscal Logic | New ÉTR / EDSC supply created within set caps |
| **Distribution** | Minted supply distributed transparently to all roles | Rewards + Treasury allocations executed |

Each phase is sealed by the **Ascending Scale of Finality**, ensuring both participation and final confirmation are cryptographically proven.

### Roles in the Governance Model

#### Foundation
- Custodian of constitutional parameters
- Sets epoch timing, fiscal caps, network bylaws
- Holds no control over outcomes; only ensures protocol integrity

#### Decentralized Directors
- Long-term stewards elected by Consensus Day results
- Responsible for policy proposals and Treasury oversight
- 9 non-hierarchical board members

#### Flare Nodes (Validators)
- Maintain ASF consensus
- Validate blocks and enforce finality conditions
- Receive rewards proportionate to uptime and participation

#### Validity Nodes
- Audit cross-chain proofs and bridge data integrity
- Serve as connective tissue ensuring interoperability remains verifiable

#### Common Stake Peers / Voters
- The community's active participants
- Their votes shape fiscal and technical direction each epoch
- Representing distributed sovereignty

### Fiscal Logic

Ëtrid's governance is **inseparable from its economy**.

Every vote connects directly to the mint and distribution logic, producing a transparent financial feedback loop:
- **Mint Authorization:** Approved proposals generate mint records on-chain
- **Distribution Scheduling:** Minted supply split across Foundation, Directors, Validators, and Voters
- **Audit Trail:** All events permanently stored — visible to every node

**The result is an autonomic economy** — one where policy, production, and payout remain synchronized.

### Treasury and Sustainability

The **Treasury** acts as the network's resource reservoir:
- **Inflows:** Transaction fees, expired deposits, VMw conversions
- **Outflows:** Scheduled distributions and grants ratified on Consensus Day

The system's sustainability is mathematical: no external authority can alter supply or flow without passing through the governance runtime.

### Event Transparency

Every governance action emits auditable events:
- `ProposalSubmitted(proposal_id, proposer)`
- `VoteCast(proposal_id, voter, ballot)`
- `MintExecuted(mint_id, amount)`
- `DistributionExecuted(mint_id, recipients)`

These events allow explorers, dashboards, and analysts to track governance health in real time — the entire decision process is **observable, immutable, and verifiable**.

### The Philosophy of Consensus Day

**"Governance should be cyclical, transparent, and self-regulating — not continuous, obscure, or external."**

By binding consensus and economics together, Ëtrid transforms governance from an afterthought into a **system heartbeat**.

It proves that sustainability isn't about centralization — it's about **rhythm**.

---

## 💰 Token Economy & Fiscal Cycle

### Three-Token System

#### ÉTR (Ëtrid Coin)
**Purpose:** Native governance & security token
- Used for staking and validation
- Governance voting weight
- Transaction fees
- **Total Supply:** 1 Billion ÉTR
- **Initial Circulation:** ~10% (100M ÉTR)
- **Emission:** Controlled by annual Consensus Day votes

#### EDSC (Ëtrid Dollar Stablecoin)
**Purpose:** Stable utility token
- 110-130% collateralized
- Treasury operations and grants
- Wages and programmatic distributions
- Pegged to USD value

#### VMw (Virtual Machine Watts)
**Purpose:** Computation metering unit
- Energy-based gas measurement
- Deterministic execution cost
- Converts to ÉTR for fees

### Fiscal Cycle Flow

```
Computation → VMw (Energy)
     ↓
VMw → ÉTR (Fees)
     ↓
Fees → Treasury
     ↓
Treasury → Consensus Day Distribution
     ↓
Distribution → Roles (Validators, Voters, Directors, Foundation)
     ↓
Funded Participants → Research & Governance
     ↓
Research → Protocol Upgrades → Cycle Restarts
```

### Distribution Model

**Consensus Day determines allocation percentages, example:**
- **Validators:** 45% (split by performance)
- **Voters:** 25% (split by participation score)
- **Directors:** 10%
- **Foundation:** 5%
- **Treasury Reserve:** 15%

### Supply Dynamics

**Hard Caps:**
- Mint cap per epoch enforced at runtime level
- No unbounded inflation possible
- All minting events recorded in Fiscal Ledger

**Burn Mechanisms:**
- Transaction fees (partial burn)
- Slashing penalties
- Expired deposits

### Economic Philosophy

**Self-Balancing Economy:**
The fiscal model where minting, distribution, and burn rates adjust to maintain equilibrium without external intervention.

---

## 🔐 Security & Trust Model

### Philosophy

Ëtrid's security model is built on a simple premise:

**A network should be able to detect, correct, and evolve from risk without human intervention.**

Instead of depending on static assumptions or external foundations, Ëtrid encodes security directly into consensus, economics, and governance.

### Layers of Assurance

| Layer | Mechanism | Purpose |
|-------|-----------|---------|
| **Consensus** | ASF | Dynamically adjusts security threshold in real time |
| **Network** | DETR P2P + Authenticated gossip | Ensures integrity and liveness of inter-chain communication |
| **Computation** | ËtwasmVM + VMw | Deterministic execution with energy-based metering |
| **Governance** | Consensus Day | Synchronized re-authorization of rules, minting, and roles |
| **Economy** | Fiscal Ledger + Hard caps | Prevents unbounded inflation or hidden issuance |
| **Identity** | OpenDID | Verifiable cryptographic identities across chains |

### ASF Consensus Security

The **Ascending Scale of Finality** ensures that as participation and time increase, the cost of re-writing history rises exponentially.

**Key Properties:**
- Validators must commit at progressively higher quorum thresholds as an epoch matures
- Late-stage finality blocks embed `GovernanceDigest` & `FiscalLedger` hashes
- Reorgs beyond finality depth are mathematically rejected, not socially debated

### Validator Integrity

**Flare Nodes (Validators):**
- Produce blocks, attest to others' blocks
- **Slashing Triggers:** Equivocation, invalid votes, missed commitments
- Rewards tied to performance metrics and honesty score

**Validity Nodes:**
- Verify cross-chain proofs between PBCs and FlareChain
- **Penalties:** Double-sign or omit proofs → slashing & reputation burn
- Periodic audit checks ensure proofs remain current

### Economic Defense

Economic incentives align long-term honesty with profit:
- **Stake weight = exposure to slashing**
- **Mint share = reward for consistent participation**
- **Burn & clawback = correction mechanism for abuse**
- **Treasury audits** occur automatically each epoch through ledger reconciliation

All financial data—fees, minting, burns, distributions—is public and queryable.

### Post-Quantum Resilience

Ëtrid anticipates the **quantum threat horizon:**
- Replaces classic elliptic-curve signatures with lattice-based (CRYSTALS-Dilithium / Falcon)
- Hash commitments via BLAKE3 / KangarooTwelve families
- Hierarchical key derivation supporting hybrid schemes (ECDSA + PQC)

The E³20 stack's modular cryptography layer allows **upgrades without runtime forks**.

### Slashing & Recovery

**Slashing Tiers:**

| Level | Trigger | Penalty |
|-------|---------|---------|
| Tier-1 | Missed blocks (repeated) | Small percentage of stake |
| Tier-2 | Equivocation / invalid attestations | Significant slash + temporary ban |
| Tier-3 | Governance fraud / cross-chain tampering | Full slash + permanent ban |

**Recovery:**
- Slashed funds redirect to treasury
- Validator may rejoin after ban period with fresh stake and proof of audit

### Governance as Security

Every Consensus Day effectively **re-authorizes the network's rulebook**.

If parameters drift, the system self-corrects by allowing participants to:
- Raise or lower ASF thresholds
- Adjust slashing ratios
- Modify mint caps or fee curves
- Elect or remove directors responsible for oversight

This built-in recalibration prevents ossification and eliminates long-term attack vectors born from stagnation.

### Threat Model Summary

| Vector | Mitigation |
|--------|------------|
| Validator collusion / 51% | ASF dynamic thresholds + stake concentration alarms |
| Governance capture | Dual quorum (citizen + security), epoch boundaries |
| Cross-chain spoofing | Validity-node attestations + merkle proof verification |
| Inflation exploit | Runtime-enforced caps + mint authorization events |
| Bridge theft | No traditional bridges; native root aggregation model |
| Key compromise | OpenDID rotation + multi-sig hot/cold separation |

---

## 🗺️ Roadmap & Milestones

### Overview

Ëtrid's roadmap is not just a timeline — it's the **structured evolution of a living network**.

Each phase builds a layer of autonomy, governance, and usability, culminating in a self-sustaining, sovereign multichain system.

### Current Status

**Phase:** Alpha Complete (100%) → Ember Testnet Preparation
**Progress:** 13/13 E³20 Components Complete
**Test Coverage:** 87.3% (412+ tests passing)
**Documentation:** 32,000+ lines across 73+ files

### Foundation Phases (1-4) ✅ COMPLETE

#### Phase 1 — Primitives & Runtime Template
- Built etrid-primitives package
- Assembled base Substrate runtime
- Created PBC Runtime Template

#### Phase 2 — ASF Consensus Integration
- Implemented Ascending Scale of Finality
- Dynamic participation weighting & finality depth logic
- ASF documentation complete

#### Phase 3 — VMw & Custom Blocks
- Integrated ËtwasmVM and VMw metering
- Established energy-based gas model
- Implemented block metadata for resource tracking

#### Phase 4 — Runtime Integration & Testing
- Unified FlareChain and PBC runtimes
- 20 integration tests passing
- Stable composite runtime foundation

### Governance Phases (5-7) 🔄 ACTIVE

#### Phase 5 — First Complete PBC (BTC ↔ ÉTR Bridge Chain)
**Status:** Active
- Building first operational Partition Burst Chain (BTC-PBC)
- HTLC channels, lightning-bloc mechanics
- Bridge logic for BTC ↔ ÉTR swaps

#### Phase 6 — Peer-Roles Layer
**Status:** In Progress
- Implementing role-based staking
- Staking, slashing, and delegation rules
- Role definitions reflected in governance runtime

#### Phase 7 — Governance Runtime (Consensus Day)
**Status:** Core Spec Complete, Code Integration Pending
- Fully specified governance logic (proposal → vote → mint → distribution)
- Runtime hooks & ASF finalize calls in development

### Public Phases (8-10) 🚀 PLANNED

#### Phase 8 — Visual & Public Infrastructure
**Status:** Active
- Building Notion → Typedream Wiki system
- Polishing Ivory Papers v2
- Creating public-facing documentation: etrid.org, docs.etrid.org, etrid.foundation

#### Phase 9 — Testnet & Explorer Launch
**Target:** Q1 2026
- Deploy FlareChain + first PBC to distributed testnet
- Launch governance simulation
- Release web explorer

#### Phase 10 — Mainnet & Foundation Activation
**Target:** Q2 2026
- Register Ëtrid Foundation on-chain
- Conduct first official Consensus Day event (mainnet)
- Begin Foundation grants + ecosystem programs
- Open developer registration for new PBC chains

### Timeline

| Quarter | Milestone | Status |
|---------|-----------|--------|
| Q3 2025 | ASF Consensus + PBC runtime stability | ✅ Complete |
| Q4 2025 | Peer-Roles Layer + Governance Runtime | 🧱 Integrating |
| Q1 2026 | Public Testnet + Explorer Launch | 🚀 Planned |
| Q2 2026 | Foundation Activation + Mainnet Go-Live | 🌍 Planned |

### Success Metrics

#### Technical KPIs
- Network Uptime: 99.9%+
- Block Time: ~6 seconds
- Finality Lag: <100 blocks
- TPS (sustained): 1,000+
- Active Validators: 50+ (Ember), 100+ (Mainnet)

#### Community KPIs
- Active Accounts: 1,000+ (Ember), 10,000+ (Mainnet)
- Daily Active Users: 100+ (Ember), 1,000+ (Mainnet)
- DApp Deployments: 50+ (Ember), 100+ (Mainnet)

---

## 👥 Community & Foundation

### Purpose

The Ëtrid ecosystem grows through **participation, not hierarchy**.

The Ëtrid Foundation exists only to keep the framework neutral, transparent, and perpetual — not to own or govern it.

Its role is to steward the values of adaptive consensus, fiscal clarity, and equal access.

### The Foundation's Charter

**Mission:** Safeguard the protocol's long-term stability, coordinate open research, and ensure that no single entity controls funding or policy.

**Mandate:**
- Maintain reference implementations of E³20 stack
- Host annual Consensus Day and publish epoch reports
- Manage foundation grants and treasury disclosures
- Preserve brand integrity and legal representation when required
- Document the network's governance outcomes for public record

### Structure & Governance

| Body | Composition | Function |
|------|-------------|----------|
| Foundation Council | Elected directors (on-chain vote) | Reviews policy proposals, supervises treasury programs |
| Executive Coordination Group | Small operations team (rotating) | Executes routine logistics: audits, publications, partnerships |
| Community Assembly | All stakers and contributors | Participates in proposals, votes, and peer-review processes |

**Rotation and term limits** prevent entrenchment; any position can be re-called through a governance proposal.

### Membership Pathways

**Open Participation:** Anyone holding or staking ÉTR can:
- Propose or support governance items
- Validate blocks or proofs (flare / validity nodes)
- Volunteer for task forces or audit committees

**Foundation Contributors:** Developers, researchers, or educators funded by grants

**Associate Members:** Independent collectives running PBCs or tooling who sign voluntary transparency agreements

**Joining does not confer ownership; every role remains governed by the runtime.**

### Transparency & Reporting

- Every epoch closes with a **Fiscal Ledger** and **Governance Digest** published by the foundation
- All financial flows — mints, burns, distributions — are viewable on-chain and mirrored in the public explorer
- **Annual Ivory Report** compiles technical progress, treasury statistics, and community metrics

**The foundation maintains zero private ledgers; all data sources originate from the chain.**

### Community Grants & Programs

The foundation allocates a portion of each epoch's treasury share to public initiatives:

| Program | Focus | Funding Cadence |
|---------|-------|-----------------|
| Core Development | Protocol engineering, security audits, PBC tooling | Rolling |
| Ecosystem Expansion | DApps, integrations, explorer services | Quarterly |
| Education & Outreach | Documentation, workshops, translations | Quarterly |
| Research Fellowships | Academic or applied studies aligned with E³20 goals | Annual |

**Grant Flow:**
1. Submit proposal → pass governance review
2. Milestone verification → partial disbursement
3. Completion proof → final payout
4. Failure → automatic clawback to treasury

### How to Get Involved

1. Join the developer channels and start with open issues tagged "good first contribution"
2. Stake ÉTR and register as a voter for the upcoming testnet Consensus Day
3. Contribute to documentation or translations
4. Draft a proposal idea for the first epoch of mainnet governance

---

# 📘 DEVELOPER PORTAL (docs.etrid.org)

## 🚀 Getting Started

### System Requirements

| Component | Recommended | Minimum |
|-----------|-------------|---------|
| OS | Linux / macOS (x86_64 / arm64) | Windows Subsystem for Linux (WSL2) |
| CPU | 4 cores | 2 cores |
| RAM | 8 GB | 4 GB |
| Storage | 100 GB SSD | 30 GB |
| Rust Toolchain | rustup +stable (≥ 1.75) | — |
| Dependencies | Git, clang, pkg-config, libssl-dev | — |

### Environment Setup

#### Install the Toolchain

```bash
# Install Rust
curl https://sh.rustup.rs -sSf | sh
rustup default stable

# Install additional targets
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version
cargo --version
```

#### Clone the Ëtrid Repository

```bash
git clone https://github.com/EojEdred/Etrid.git
cd Etrid
```

#### Build a Node

```bash
cargo build --release -p flare-chain-node
```

When finished, you'll have the executable:
```
target/release/flare-chain-node
```

### Run a Local Node

Start a development chain using the preconfigured dev key:

```bash
./target/release/flare-chain-node --dev --tmp
```

Output should show:
```
🏁 Starting Etrid FlareChain Dev Node
🎯 Consensus: ASF (Ascending Scale of Finality)
💰 Chain ID: local-flare
```

You now have a live FlareChain node running locally.

### Connect with the CLI

```bash
# Install Ëtrid CLI (once available)
cargo install etrid-cli

# Query Network State
etrid-cli query system account <ACCOUNT_ID>
etrid-cli query governance active-proposals
etrid-cli query fiscal ledger --epoch <N>

# Submit Transactions
etrid-cli tx balances transfer <TO> <AMOUNT>
etrid-cli tx governance vote <PROPOSAL_ID> yes
```

---

## 🔧 Developer SDKs

### Rust SDK (core)
- `etrid-sdk`: Provides runtime types, Subxt bindings, and high-level abstractions
- `etrid-contract`: Helpers for Etwasm smart contract deployment

### JavaScript / TypeScript SDK
- Web3-compatible provider (WASM-based)
- Exposes RPCs like `etrid_getBalance`, `etrid_submitExtrinsic`
- Built for DApp integration with browsers or Node.js

### Python SDK
- `etridpy`: Lightweight binding for automation and off-chain analytics

### Swift SDK
- iOS/macOS mobile wallet integration

---

## 📝 Writing Smart Contracts (ËtwasmVM)

Ëtrid uses **Etwasm**, a WASM runtime extended with VMw metering.

### Example (Pseudo Ink! Syntax)

```rust
#[etrid::contract]
pub mod counter {
    use super::etrid_env::*;

    #[state]
    pub struct Counter {
        value: u64,
    }

    #[constructor]
    pub fn new(init_value: u64) -> Self {
        Self { value: init_value }
    }

    #[message]
    pub fn increment(&mut self) {
        self.value += 1;
    }

    #[message]
    pub fn get(&self) -> u64 {
        self.value
    }
}
```

**Compile:**
```bash
cargo etrid build --release
```

**Deploy:**
```bash
etrid-cli tx contract deploy ./target/wasm32-unknown-unknown/release/counter.wasm
```

---

## 🧪 Testing and Simulation

### Unit Tests
```bash
cargo test --workspace
```

### Integration Tests
Located in `tests/consensus_day/`, `tests/pbc/`, and `tests/asf/`

### Performance Profiling
```bash
etrid-cli benchmark --module voting-protocol
```

---

# 🏛️ FOUNDATION HUB (etrid.foundation)

## About

The Ëtrid Foundation exists to maintain the sovereignty and continuity of the network — not through ownership, but through **stewardship**.

It safeguards the principles of adaptive consensus, fiscal transparency, and equitable participation, ensuring that the framework remains open, self-funding, and capable of perpetual evolution.

---

## Grants & Funding

### Grant Programs

| Program | Focus | Funding |
|---------|-------|---------|
| Core Development | Protocol engineering | Rolling |
| Ecosystem Expansion | DApps, integrations | Quarterly |
| Education & Outreach | Documentation, workshops | Quarterly |
| Research Fellowships | Academic studies | Annual |

### Application Process

1. **Submit Proposal** → Governance review
2. **Milestone Verification** → Partial disbursement
3. **Completion Proof** → Final payout
4. **Failure** → Automatic clawback to treasury

---

## Treasury Transparency

- **Fiscal Ledger:** On-chain access to all transactions
- **Mint Logs:** Public record of all minting events
- **Distribution Reports:** Per-epoch allocation breakdowns
- **Annual Reports:** Comprehensive treasury statistics

---

# 🧬 EVOLUTION & RESEARCH

## ËPS (Ëtrid Protocol Evolution System)

### What is ËPS?

ËPS is Ëtrid's native governance format — **not EIPs**.

It's how new logic, modules, and fiscal programs enter Consensus Day and become part of the living network.

### Types of Evolution Proposals

| Type | Description | Triggered By |
|------|-------------|--------------|
| **System Evolution (SE)** | Changes to ASF, runtime logic, or consensus parameters | Developers, Foundation, Directors |
| **Economic Evolution (EE)** | Adjustments to mint caps, splits, fee curves, or VMw rates | Treasury or Governance Runtime |
| **Fiscal Evolution (FE)** | Proposals creating or modifying grants, programs, or fiscal policies | Treasury or Community |
| **Partition Evolution (PE)** | New PBC creation, domain assignment, or decommission | PBC Founders or Council |
| **Research Evolution (RE)** | Introduces new cryptographic or computational research into E³20 core | Labs, Universities, or Grants |

### Submission & Review Lifecycle

1. **Draft Stage** → Submitted through Governance Portal
2. **Peer Review** → Open 7-day review period
3. **Activation** → Approved drafts scheduled for next Consensus Day
4. **Voting & Execution** → Standard voting procedure → Runtime upgrade or fiscal schedule executes

### The Ivory Ledger

The **Ivory Ledger** is the immutable archive of network evolution.

Each entry maps:
```
epoch → list<EvolutionProposal> → execution metadata → governance digest hash
```

This gives the network a **version-controlled memory** — anyone can reconstruct the exact moment and reason behind every change.

---

## ERA (Ëtrid Research Archive)

### Purpose

The Ëtrid Research Archive (ERA) exists to **document, verify, and preserve** every scientific and technical contribution that shapes the E³20 architecture.

It acts as a bridge between research, governance, and implementation — ensuring that all protocol evolution begins from transparent, peer-reviewed foundations.

### Structure (6 Categories)

| Category | Scope |
|----------|-------|
| **Theory** | Mathematical proofs, ASF models, consensus mechanisms |
| **Cryptography** | Quantum-safe primitives, signature schemes, zero-knowledge integrations |
| **Economics** | Fiscal modeling, VMw pricing theory, treasury equilibria |
| **Governance Science** | Voting algorithms, role distribution math, behavioral models |
| **Systems & Performance** | Benchmarks, runtime optimizations, PBC network topology research |
| **Philosophy & Ethics** | Studies on decentralized governance, sovereignty, and systems theory |

### Submission Flow

1. **Create Entry Draft** → Markdown + JSON metadata bundle
2. **Peer Review** → Reviewers selected randomly from verified experts
3. **Archival & Indexing** → Approved entries pinned in ERA registry pallet
4. **Citation in ËPS or Runtime** → Each entry can be cited in governance proposals or code

### Verification Methods

| Verification | Description |
|--------------|-------------|
| Deterministic Simulation | Authors provide script + seed for identical outputs |
| Formal Proof Attachment | Coq / Isabelle / TLA+ proofs stored alongside paper |
| Runtime Test Hash | Executable test file validating model in devnet environment |
| Cross-Review | Two independent experts verify claims using separate methods |

---

# 🗺️ ECOSYSTEM MAP

## The Living Framework

Ëtrid is not one blockchain — it is a **sovereign multichain organism** composed of interacting systems.

Each system has autonomy, but all share the same pulse: the **Ascending Scale of Finality (ASF)** and the **Consensus Day governance rhythm**.

## Ecosystem Layers

| Layer | Function | Key Entities |
|-------|----------|--------------|
| **Root Chain** | Coordination & finality anchor | FlareChain |
| **Partition Layer** | Sovereign specialized runtimes | 13 Partition Burst Chains (PBCs) |
| **Consensus & Governance** | Dynamic finality + periodic fiscal synchronization | ASF + Consensus Day |
| **Economy & Treasury** | Energy-based computation + epochal mint & distribution | ÉTR, EDSC, VMw |
| **Foundation & Community** | Neutral stewardship & grants | Ëtrid Foundation, Directors, Peers |
| **Research & Evolution** | Scientific model repository & proposal system | ERA + ËPS |

## Vertical Structure (Energy → Economics → Policy)

```
Layer 1: Computation (ËtwasmVM + VMw)
    ↓
Layer 2: Economy (Fees → Treasury)
    ↓
Layer 3: Governance (Consensus Day)
    ↓
Layer 4: Research & Feedback (ERA → ËPS)
    ↓
Cycle Repeats
```

This closed loop forms the **adaptive sovereign feedback system** — the core philosophy of Ëtrid.

## Horizontal Structure (Sovereignty & Interoperability)

### FlareChain (Root)
- Provides ASF consensus and coordination
- Stores epoch metadata, governance digests, and cross-chain proofs

### Partition Burst Chains (PBCs)
- Specialized runtimes handling specific domains (BTC bridge, AI compute, data privacy)
- Autonomously governed but synchronized through ASF and FlareChain proofs
- Each PBC can run its own micro-Consensus Day, feeding results into the global event

### Validity Nodes
- Verify proofs across PBCs and ensure cross-root consistency
- Prevent forged state roots and maintain synchronized ledger state

---

# 📖 REFERENCE MATERIALS

## Glossary

### A

**ASF (Ascending Scale of Finality)**
Dynamic consensus system that raises or relaxes finality thresholds based on validator participation and time elapsed. Finality becomes a spectrum rather than a fixed rule.

**Active Proposal**
A governance item currently in the voting phase of Consensus Day.

### B

**Block Finalization**
The process through which ASF certifies a block as irreversible once all quorum thresholds are met.

**Burst Chain (PBC)**
See Partition Burst Chain.

### C

**Consensus Day**
The periodic governance and fiscal synchronization event in Ëtrid. It unites proposals, voting, minting, and distribution into one verifiable cycle.

**Citizen Quorum**
Stake-weighted vote participation from the community; must meet a minimum threshold for governance validity.

### D

**DETR P2P Network**
Distributed peer-to-peer layer responsible for authenticated message delivery between PBCs and FlareChain.

**Director (Decentralized Director)**
Long-term elected peer responsible for policy oversight and treasury verification.

### E

**E³20 Stack**
Ëtrid's core technology stack integrating ËtwasmVM, VMw metering, DETR networking, and ASF consensus.

**ÉTR (Etrid Coin)**
The network's native governance currency. Used for staking, transactions, and fiscal distribution.

**EDSC (Etrid Dollar Stable Coin)**
Programmatically minted stable asset for treasury operations, grants, and wages. Always tied to on-chain fiscal schedules.

**ËtwasmVM**
WebAssembly-based runtime optimized for quantum resilience and energy-based metering (VMw).

**Evolution Proposal (ËPS)**
Structured proposal for upgrading protocol logic, fiscal parameters, or governance. Each approved ËPS becomes part of the Ivory Ledger.

**ERA (Etrid Research Archive)**
Open repository linking research, governance, and implementation through verifiable proofs.

### F

**Fiscal Ledger**
Per-epoch record of minted, burned, and distributed funds. Stored on-chain and publicly queryable.

**FlareChain**
Root chain coordinating PBCs and hosting ASF consensus.

### G

**Governance Digest**
Merkle commitment of all finalized decisions per epoch; written to block header during finalization.

**Governance Runtime**
On-chain logic that executes proposals, votes, and fiscal distribution.

### I

**Ivory Ledger**
Immutable chronological record of every Evolution Proposal, Fiscal Ledger, and ERA entry.

### M

**Mint Cap**
Hard-coded limit on maximum new ÉTR creation per epoch.

**Mint Event**
Governance-approved issuance of new tokens recorded in Fiscal Ledger.

### O

**OpenDID**
Decentralized identity layer used for verifiable user and contributor signatures.

### P

**PBC (Partition Burst Chain)**
Specialized sovereign runtime under FlareChain coordination. Each PBC handles a specific domain (BTC bridge, ETH bridge, etc).

**Proposal Lifecycle**
Registration → Activation → Voting → Decision → Distribution → Archival.

### Q

**Quantum-Resilient Cryptography**
Post-quantum cryptographic standards built into ËtwasmVM and E³20 Stack.

### S

**Self-Balancing Economy**
Fiscal model where minting, distribution, and burn rates adjust to maintain equilibrium.

**Slashing**
Automatic penalty for validators or nodes violating protocol rules.

### T

**Treasury**
Automated fiscal contract managing mint caps, burns, and disbursements.

### V

**VMw (Virtual Machine Watts)**
Energy-based gas unit used to measure and price computation on ËtwasmVM.

**Voter Participation Score**
Metric combining stake, consistency, and delegation to weight voter rewards.

---

## Public Explorer Guide

### Purpose

The explorer makes Ëtrid's state transparent to everyone. It visualizes blocks, epochs, and governance events so users can see what the system is doing — not just what's promised.

### Main Sections

| Section | What You See |
|---------|--------------|
| **Overview** | Network health, epoch number, validator count, current ASF stage |
| **Blocks** | Finalized blocks with VMw usage, validator signatures, transaction count |
| **Epochs** | Summaries of proposals, votes, and fiscal data per epoch |
| **Governance** | Live proposals, quorum status, decision outcomes |
| **Treasury** | Charts showing minted, burned, and distributed ÉTR/EDSC |
| **Roles** | Leaderboard of validators, validity nodes, and voters with performance metrics |
| **Research** | Indexed ERA entries and linked Evolution Proposals (ËPS) |

### Fiscal Dashboard

- **Epoch Slider:** View past epochs and observe mint cap utilization
- **Distribution Pie:** Shows how treasury allocations were split among roles
- **Burn/Mint Balance Line:** Compares supply vs. destruction over time
- **Treasury Inflow Table:** VMw fees, slashed funds, deposits returned

### Governance Dashboard

- **Proposal Cards:** Title, category, proposer identity (OpenDID), quorum status
- **Live Voting Heatmap:** Colored by yes/no weight and validator signaling
- **Phase Timer:** Countdown to next Consensus Day phase
- **Finalization Panel:** ASF thresholds, security quorum confirmation

---

## Press & Brand Kit

### Mission Summary

Ëtrid is a sovereign multichain framework that unites consensus, governance, and economics into a single adaptive system.

### Logo Usage

- **Primary Mark:** Infinity-horizon symbol merging event horizon and balance loop
- **Color Palette:**
  - Base Black: `#000000` (depth and finality)
  - Rust Silver: `#c1c7c9` (industrial precision)
  - Tech Green-Blue: `#4fe2c9` (vitality and energy)
- **Padding:** Keep at least 25% padding around logo
- **Backgrounds:** Dark gradient or pure white

### Typography

| Usage | Font | Style |
|-------|------|-------|
| Headers | Inter / Futura | Semi-bold, uppercase |
| Body | Inter / Open Sans | Regular |
| Code & Data | JetBrains Mono | Light |

### Tone of Voice

| Context | Tone |
|---------|------|
| Technical | Precise, declarative, neutral |
| Public | Confident, factual, minimal adjectives |
| Educational | Clear analogies, no jargon gatekeeping |
| Philosophical | Reflective, system-oriented, never mystical |

### Examples

✅ **Good:** "Ëtrid achieves equilibrium through adaptive consensus."

❌ **Avoid:** "Ëtrid revolutionizes everything forever."

### Press Materials

- Brand summary: One-pager of mission + architecture overview
- Media kit: Logo files (SVG + PNG), palette, typography
- Whitepaper: Ivory Papers Vol. I–III
- Contact: press@etrid.foundation

---

# 📄 IVORY PAPERS (Technical Whitepaper)

## Volume I: Conceptual Architecture

**What Ëtrid is and why it exists**

[Link to full Ivory Paper document]

---

## Volume II: Technical Specification

**ASF logic, VMw gas model, runtime hierarchy**

[Link to full Ivory Paper document]

---

## Volume III: Governance & Fiscal Mechanics

**Consensus Day, Minting, Distribution, Foundation charter**

[Link to full Ivory Paper document]

---

---

# 📋 NEXT STEPS

## Phase 1: Content Organization ✅
- [x] Create WIKI_STRUCTURE.md
- [x] Create NOTION_IMPORT.md (this document)
- [ ] Import into Notion workspace

## Phase 2: Visual Assets 🎨
- [ ] Design Ecosystem Map (Sankey diagram)
- [ ] Create Architecture Diagrams
- [ ] Design Brand Kit assets
- [ ] Create Ivory Papers cover designs

## Phase 3: Website Deployment 🚀
- [ ] Export Notion → Typedream/Gamma
- [ ] Set up custom domain (etrid.org)
- [ ] Deploy docs.etrid.org subdomain
- [ ] Configure SSL and DNS

---

**END OF NOTION IMPORT FILE**

---

> **To Import:** Copy this entire document → Paste into Notion → Notion will automatically create the page hierarchy with proper nesting.
