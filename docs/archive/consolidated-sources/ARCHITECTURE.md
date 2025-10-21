# ËTRID ARCHITECTURE
## E³20 Protocol Specification

**Version:** 1.0-alpha  
**Last Updated:** October 11, 2025  
**Status:** Living Document

---

## Table of Contents

1. [Overview](#overview)
2. [E³20 Components](#e320-components)
3. [System Architecture](#system-architecture)
4. [Data Flow](#data-flow)
5. [Consensus Mechanism](#consensus-mechanism)
6. [Multichain Design](#multichain-design)
7. [Account System](#account-system)
8. [Smart Contracts](#smart-contracts)
9. [Governance](#governance)
10. [Security Model](#security-model)

---

## Overview

Ëtrid implements a **multichain architecture** with a main **Flare Chain** and unlimited **Partition Burst Chains (PBCs)**. The system uses **Adaptive Stake Finality (ASF)** for consensus and features an annual **Consensus Day** for on-chain governance.

### Key Principles

1. **Decentralization**: No central authority, fully distributed
2. **Democracy**: Annual stakeholder voting on all major decisions
3. **Scalability**: Horizontal scaling via PBCs
4. **Security**: Post-quantum cryptography, formal verification
5. **Sustainability**: Energy-efficient PoS, long-term economic model

---

## E³20 Components

The **Essential Elements to Operate (E³20)** protocol defines 13 mandatory components for a functional Ëtrid network:

### Layer 1: Network & Identity

#### 01. DETR p2p (Decentralized Ëtrid Networking)
**Purpose**: Multi-protocol peer networking and discovery

**Key Features:**
- DHT-based peer discovery (S/Kademlia)
- Multi-transport support (TCP, QUIC, WebSocket)
- NAT traversal (hole punching, relay)
- Peer reputation system
- Message routing and gossip protocols

**Implementation:**
- Built on libp2p
- Custom protocols: `/etr/1.0.0`, `/dpeer/1.0.0`
- Support for 10,000+ concurrent peers

**Location:** `01-detr-p2p/`

---

#### 02. OpenDID (Open Decentralized Identity)
**Purpose**: Self-sovereign identity system

**Key Features:**
- W3C DID standard compliance
- Verifiable credentials
- Privacy-preserving (zero-knowledge proofs)
- Revocation registry
- Cross-chain identity portability

**DID Format:**
```
did:etrid:1a2b3c4d5e6f...
```

**Location:** `02-opendid/`

---

### Layer 2: Security & Accounts

#### 03. Blockchain Security
**Purpose**: Post-quantum cryptography and key management

**Cryptographic Primitives:**
- **Signatures**: SPHINCS+ (post-quantum)
- **Encryption**: Kyber (post-quantum KEM)
- **Hashing**: BLAKE3
- **Key Derivation**: BIP39/BIP44 compatible

**Security Features:**
- Hardware wallet support (Ledger, Trezor, Tangem)
- Multi-signature accounts
- Time-locked transactions
- Social recovery

**Location:** `03-security/`

---

#### 04. Accounts
**Purpose**: Multi-type account system

**Account Types:**

| Type | Abbr | Purpose | Can Hold Funds? | Executable? |
|------|------|---------|----------------|-------------|
| External Blockchain Account | EBCA | User wallets | ✅ Yes | ❌ No |
| Regular Contract Account | RCA | Simple contracts | ✅ Yes | ✅ Yes |
| Smart Contract Account | SCA | Full EVM contracts | ✅ Yes | ✅ Yes |
| Stake Deposit Contract | SDCA | Staking only | ✅ Yes | ✅ Limited |

**Storage Schema:**
```rust
AccountInfo {
    nonce: u64,
    etr_balance: Balance,
    etd_balance: Balance,
    vmw_balance: Balance,
    account_type: AccountType,
    data: Vec<u8>, // Contract code if SCA
}
```

**Location:** `04-accounts/`

---

### Layer 3: Multichain & Economics

#### 05. Multichain
**Purpose**: Flare Chain + Partition Burst Chains coordination

**Architecture:**

```
                    Flare Chain (Main)
                    ┌─────────────┐
                    │   State     │
                    │   Root      │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼────┐        ┌────▼────┐       ┌────▼────┐
   │  PBC-1  │        │  PBC-2  │  ...  │  PBC-N  │
   │ (DeFi)  │        │  (NFT)  │       │ (Gaming)│
   └─────────┘        └─────────┘       └─────────┘
```

**Flare Chain:**
- Stores global state root
- Processes cross-chain transactions
- Hosts governance
- ~5 second block time

**Partition Burst Chains (PBCs):**
- Application-specific chains
- High throughput (1000+ TPS each)
- Periodic state merges to Flare
- Can have custom rules

**Cross-Chain Communication:**
- IBC-inspired protocol
- Merkle proofs for state verification
- Atomic swaps
- Message passing

**Location:** `05-multichain/`

---

#### 06. Crypto (Token Economics)
**Purpose**: Native currencies and gas mechanics

**Tokens:**

**ÉTR (Ëtrid Coin)**
- Native cryptocurrency
- Used for: transactions, staking, governance voting
- Initial supply: 1 billion
- Annual inflation: 2-5% (voted via Consensus Day)
- Divisibility: 18 decimals

**EDSC (Ëtrid Dollar Stable Coin)**
- USD-pegged stablecoin (1 EDSC = $1 USD)
- Collateralized by ÉTR reserves
- Over-collateralization ratio: 150%
- Redemption mechanism ensures peg
- Use cases: stable payments, DeFi

**VMw (VMwattage)**
- Gas token for smart contract execution
- Priced in ÉTR (dynamic based on congestion)
- Burned on use (deflationary)
- Prevents spam attacks

**Fee Structure:**
```
Total TX Fee = Base Fee + Priority Fee + VMw Cost
```

**Location:** `06-crypto/`

---

#### 07. Transactions
**Purpose**: Transaction processing and validation

**Transaction Types:**

1. **Regular Transaction**
   - Simple ÉTR/EDSC transfers
   - Lowest fees
   - ~0.001 ÉTR base fee

2. **Smart Contract Transaction**
   - Deploy or call contracts
   - Variable VMw cost
   - Data payload up to 128 KB

3. **Cross-Chain Transaction**
   - Transfer between Flare and PBCs
   - Merkle proof validation
   - Higher fees (~0.01 ÉTR)

4. **Governance Transaction**
   - Vote on proposals
   - Only during Consensus Day
   - Zero fees (incentivized)

**Transaction Structure:**
```rust
Transaction {
    from: AccountId,
    to: Option<AccountId>,
    value: Balance,
    nonce: u64,
    vmw_limit: u64,
    vmw_price: Balance,
    data: Vec<u8>,
    signature: Signature,
}
```

**Location:** `07-transactions/`

---

### Layer 4: Execution & Consensus

#### 08. ËtwasmVM
**Purpose**: WebAssembly-based smart contract runtime

**Features:**
- Turing-complete (gas-limited)
- Sandboxed execution
- Deterministic results
- Support for: Rust, C, C++, AssemblyScript contracts

**VM Specifications:**
- Max contract size: 1 MB
- Max stack depth: 1024
- Memory limit: 16 MB per contract
- Storage: Key-value store (1 KB max per entry)

**Gas Schedule:**
| Operation | VMw Cost |
|-----------|----------|
| ADD/SUB | 1 |
| MUL/DIV | 5 |
| Storage Read | 100 |
| Storage Write | 1000 |
| External Call | 5000 |

**Contract Lifecycle:**
1. Deploy → bytecode stored on-chain
2. Call → instantiate VM, execute
3. Result → state changes committed
4. Gas → VMw burned

**Location:** `08-etwasm-vm/`

---

#### 09. Consensus (ASF - Adaptive Stake Finality)
**Purpose**: Novel consensus algorithm

**Algorithm Overview:**

1. **Stake-Weighted Voting**
   - Validators weighted by staked ÉTR
   - Minimum stake: 64 ÉTR

2. **Coinage Factor**
   - Coinage = Stake × Time
   - Vote power = √(Stake × Coinage)
   - Dilutes over time to prevent centralization

3. **Block Production**
   - Validators take turns (round-robin among top 100)
   - Block proposed every 5 seconds
   - 67% of stake must sign for finality

4. **Rewards Distribution**
   - Block proposer: 10% of block reward
   - Voters: 90% split proportionally
   - Non-voting validators: slashed 1% per missed block

**Finality:**
- Probabilistic finality: 1 block (~5 sec)
- Absolute finality: 3 blocks (~15 sec)
- No reorgs after finality

**Location:** `09-consensus/`

---

### Layer 5: Governance & Roles

#### 10. Foundation (Legal/Organizational)
**Purpose**: Real-world entity management

**Foundation Structure:**
- Legal entity: Swiss Foundation (planned)
- DAO governance on-chain
- Treasury managed by multi-sig + governance
- Mandate: maintain protocol, fund development

**Treasury:**
- Funded by: 5% of block rewards
- Spending: requires governance vote
- Transparency: all txs public on-chain

**Location:** `10-foundation/`

---

#### 11. Roles (Peer Types)
**Purpose**: Define network participant roles

**Peer Roles:**

| Role | Requirements | Responsibilities | Rewards |
|------|--------------|------------------|---------|
| Common Peer | None | Run node, relay txs | None |
| Common Stake Peer | 64+ ÉTR staked | Vote on proposals | Distribution pay |
| Flare Node | 128+ ÉTR staked | Produce blocks on Flare | Block rewards |
| Validity Node | 256+ ÉTR staked | Validate cross-chain proofs | Validation fees |
| Decentralized Director | Elected | Strategic decisions | Base salary |

**Role Hierarchy:**
```
Decentralized Director (elected)
         ↑
    Validity Node (256+ ÉTR)
         ↑
    Flare Node (128+ ÉTR)
         ↑
Common Stake Peer (64+ ÉTR)
         ↑
    Common Peer (0 ÉTR)
```

**Location:** `11-roles/`

---

#### 12. Governance (Consensus Day)
**Purpose**: Annual on-chain governance event

**Consensus Day Process:**

**Phase 1: Proposal Submission (30 days before)**
- Any stake peer can submit proposals
- Proposal types:
  - Fiscal policy (inflation rate, distribution %)
  - Protocol upgrades (runtime changes)
  - Treasury spending
  - Network parameters

**Phase 2: Deliberation (20 days before)**
- Community discussion (off-chain + on-chain comments)
- Proposal refinement
- Economic impact analysis

**Phase 3: Voting (10 days before)**
- On-chain voting opens
- Vote weight = √(Stake × Coinage)
- Options: Yes, No, Abstain
- Quorum: 33% of total stake

**Phase 4: Execution (Consensus Day)**
- Results tallied
- Winning proposals auto-execute
- New ÉTR minted based on voted inflation
- Distribution occurs immediately

**Consensus Day Rewards:**
- All voters receive "distribution pay"
- Amount: % of newly minted ÉTR
- Incentivizes participation

**Location:** `12-governance/`

---

### Layer 6: User Interfaces

#### 13. Clients
**Purpose**: User-facing applications

**Client Types:**

**CLI (etrust-console)**
- Command-line wallet and node controller
- For developers and power users
- Features: send tx, query state, deploy contracts
- Built in Rust

**Web Dashboard**
- Browser-based interface
- Consensus Day voting UI
- Explorer (blocks, txs, accounts)
- Staking dashboard
- Built with React + TypeScript

**Mobile Wallet**
- iOS and Android apps
- Send/receive ÉTR/EDSC
- QR code support (Tangem SDK)
- Biometric authentication
- Built with Flutter

**Location:** `13-clients/`

---

## System Architecture

### High-Level View

```
┌─────────────────────────────────────────────────────────────┐
│                        Ëtrid Network                        │
├─────────────────────────────────────────────────────────────┤
│  Clients Layer                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐                 │
│  │   CLI    │  │   Web    │  │  Mobile  │                 │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘                 │
│       └─────────────┼─────────────┘                        │
├─────────────────────┼────────────────────────────────────────┤
│  RPC/WebSocket API  │                                      │
├─────────────────────┼────────────────────────────────────────┤
│  Runtime Layer      │                                      │
│  ┌──────────────────▼────────────────────────────────────┐ │
│  │  FRAME Runtime (Pallets)                             │ │
│  │  ┌─────────┬──────────┬──────────┬─────────┐        │ │
│  │  │Accounts │Multichain│ËtwasmVM  │Consensus│ ...    │ │
│  │  └─────────┴──────────┴──────────┴─────────┘        │ │
│  └───────────────────────────────────────────────────────┘ │
├────────────────────────────────────────────────────────────┤
│  Consensus Layer (ASF)                                     │
│  ┌──────────┬──────────┬──────────┬──────────┐            │
│  │Validator │Validator │Validator │Validator │ ...        │
│  │  Node 1  │  Node 2  │  Node 3  │  Node 4  │            │
│  └──────────┴──────────┴──────────┴──────────┘            │
├────────────────────────────────────────────────────────────┤
│  Network Layer (DETR p2p)                                  │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  libp2p (TCP, QUIC, WebSocket, DHT, Gossip)        │  │
│  └─────────────────────────────────────────────────────┘  │
├────────────────────────────────────────────────────────────┤
│  Storage Layer                                             │
│  ┌─────────────────────────────────────────────────────┐  │
│  │  RocksDB / ParityDB                                 │  │
│  └─────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────┘
```

---

## Data Flow

### Transaction Lifecycle

```
1. User submits tx via Client
         ↓
2. Client signs tx, sends to Node via RPC
         ↓
3. Node validates tx (signature, nonce, balance)
         ↓
4. Node adds tx to mempool, gossips to peers
         ↓
5. Block producer includes tx in new block
         ↓
6. Block proposed to validators
         ↓
7. Validators vote on block (ASF)
         ↓
8. Block finalized (67% stake votes yes)
         ↓
9. Runtime executes tx, updates state
         ↓
10. Result emitted as event, returned to client
```

---

## Deployment Topology

### Mainnet Configuration

**Node Types:**
- **Full Node**: Stores complete blockchain history
- **Archive Node**: Full node + full state history (for explorers)
- **Light Client**: Only block headers, queries full nodes
- **Validator Node**: Full node + participates in consensus

**Recommended Specs (Validator):**
- CPU: 8 cores @ 3.0 GHz
- RAM: 32 GB
- Storage: 1 TB NVMe SSD
- Network: 1 Gbps, static IP

**Geographic Distribution:**
- 100+ validators across 6 continents
- No more than 20% in any single jurisdiction
- Redundancy and failover

---

## Security Model

### Threat Mitigation

| Threat | Mitigation |
|--------|------------|
| 51% Attack | ASF requires 67% stake, expensive to acquire |
| Sybil Attack | Stake requirement, reputation system |
| DDoS | Rate limiting, peer reputation, libp2p DoS protection |
| Smart Contract Exploits | VM sandboxing, gas limits, formal verification |
| Quantum Computing | Post-quantum cryptography (SPHINCS+, Kyber) |
| Governance Capture | Coinage dilution, decentralized directors |

---

## Performance Metrics

**Target Specifications:**

| Metric | Target | Achieved (Testnet) |
|--------|--------|--------------------|
| Block Time | 5 sec | TBD |
| Finality Time | 15 sec | TBD |
| TPS (Flare) | 1,000 | TBD |
| TPS (PBC) | 10,000 | TBD |
| Validator Count | 100+ | TBD |
| Max Contract Size | 1 MB | TBD |

---

## Future Enhancements

- **Zero-Knowledge Proofs**: Privacy-preserving transactions
- **Sharding**: Further horizontal scalability
- **Interoperability**: Bridges to Ethereum, Bitcoin, Polkadot
- **On-Chain Governance v2**: Quadratic voting, conviction voting
- **AI/ML Integration**: On-chain inference for decentralized AI

---

## References

- Ëtrid Whitepaper (see `docs/whitepaper/`)
- Substrate Documentation: https://docs.substrate.io
- Polkadot SDK: https://github.com/paritytech/polkadot-sdk

---

**Document Status**: Living document, updated as protocol evolves

**Last Review**: October 11, 2025

**Next Review**: December 1, 2025
