# ËTRID IVORY PAPER v2.0
## Complete Protocol Specification & Foundation Governance

**Document ID**: ETRID-WP-2025-V2.0
**Status**: ACTIVE PROTOCOL SPECIFICATION
**Publication Date**: October 30, 2025
**Founder**: Eoj Edred
**License**: GPLv3 (Open Source, Non-Commercial)

---

## TABLE OF CONTENTS

1. Executive Summary
2. Vision & Mission
3. The Problem: Blockchain Centralization
4. The Solution: ËTRID FODDoS Protocol
5. Protocol Architecture (E³20)
6. Governance & Consensus Day
7. Token Economics (ÉTR, ËDSC, VMw)
8. ËTRID Dollar Stablecoin (ËDSC) Specification
9. Peer Architecture & Node Types
10. Distribution Pay System
11. Foundation Charter & Legal Framework
12. Technical Specifications & Network Parameters
13. Security, Cryptography & Post-Quantum Readiness
14. Deployment Roadmap & Milestones
15. Frequently Asked Questions
16. Appendices

---

## 1. EXECUTIVE SUMMARY

**ËTRID** is a **decentralized multichain blockchain platform** designed to achieve true democratic governance at scale. Unlike Bitcoin's immutability-first approach or Ethereum's developer-centric model, ËTRID implements **Consensus Day**: an annual, stake-weighted voting event where the community directly controls:

- The annual token inflation rate
- Protocol amendments and upgrades
- Budget allocation for development
- Selection of 9 non-hierarchical board members (Decentralized Directors)

### Key Differentiators

| Feature | ËTRID | Bitcoin | Ethereum | Others |
|---------|-------|---------|----------|--------|
| **Democratic Governance** | ✅ Annual vote on all major decisions | ❌ Developer consensus | ⚠️ Off-chain voting | ⚠️ Varies |
| **Native Stablecoin** | ✅ ËDSC (110-130% collateralized) | ❌ None | ⚠️ Requires DeFi | ⚠️ Varies |
| **Post-Quantum Crypto** | ✅ Ed25519 + SPHINCS+ hybrid | ❌ ECDSA only | ❌ ECDSA only | ❌ Most use ECDSA |
| **P2P Protocol** | ✅ DETR p2p (S/Kademlia + ECIES) | ✅ Custom P2P | ✅ Custom P2P | ✅ Varies |
| **Sidechain Architecture** | ✅ Partition Burst Chains | ❌ None | ✅ Rollups/Sidechains | ✅ Varies |
| **Smart Contracts** | ✅ WASM-based ËtwasmVM | ❌ None | ✅ Solidity/EVM | ✅ Varies |

### Launch Timeline
- **Phase 1-2**: ✅ Core infrastructure complete
- **Phase 3**: ✅ ËDSC stablecoin integration complete
- **Phase 4-5**: ✅ Partition Burst Chains & DAO registration complete
- **Phase 6-7**: ✅ Smart contracts & AI governance complete
- **Phase 8**: ✅ Mainnet launched successfully (October 2025)

### Initial Token Distribution
- **Total Supply**: 1 Billion ÉTR
- **Initial Circulation**: ~10% (100 Million ÉTR)
- **Locked for Growth**: 900 Million ÉTR (released via Consensus Day votes)
- **Annual Emission**: Voted by community on Dec 1st each year

---

## 2. VISION & MISSION

### ËTRID Vision
**A free and open decentralized democracy of stakeholders where power is distributed among millions of participants, not concentrated in the hands of a few.**

### ËTRID Mission
1. **Build** a truly decentralized blockchain with democratic governance
2. **Protect** digital rights, data sovereignty, and financial privacy
3. **Enable** self-sufficient stakeholders to reclaim power from centralized intermediaries
4. **Create** economic systems that reward participation, not just capital
5. **Maintain** technological excellence while resisting censorship and mutable forks

### Core Values
- **Decentralization First**: No entity controls >5% of voting power
- **Democratic**: All major decisions via Consensus Day supermajority
- **Open Source**: GPLv3 license ensures perpetual freedom
- **Transparent**: All transactions, governance, and code are auditable
- **Resilient**: Network continues operating even if any node is compromised

---

## 3. THE PROBLEM: BLOCKCHAIN CENTRALIZATION

### Historical Centralization Patterns

**Bitcoin** (2009):
- Originally: Truly decentralized P2P currency
- Today: Dominated by mining pools (3 pools control >50% of hash power)
- Problem: Network security depends on benevolence of pool operators

**Ethereum** (2015):
- Originally: Decentralized smart contract platform
- Today: Major client (Geth) has 80%+ market share, controlled by Ethereum Foundation
- Problem: Hard fork decisions made by core developers, not token holders

**Modern Alternatives** (2020+):
- Solana: Dominated by venture capital investors
- Polkadot: Governance but relay chain remains centralized
- Cosmos: Multiple chains but unclear governance
- Layer 2s: Often run by single company (Arbitrum = Offchain Labs, Optimism = OP Labs)

### Mutable Hardforks Under Political Pressure

Even projects with "immutable" designs have hard-forked due to political or financial pressure:
- Ethereum's DAO hard fork (2016): Reverting "immutable" transaction
- Bitcoin Classic vs Bitcoin Cash split (2017): Competing visions, no democratic process
- Staked ETH Shanghai upgrade (2023): Community concerns ignored in favor of Ethereum Foundation direction

**Root Cause**: Lack of **decentralized democratic decision-making processes**.

### The Data Wars & DCPI Threat

Centralized platforms are collecting:
- Personal location data (IoT devices)
- Biometric data (facial recognition, haptics)
- Neural/emotional data (spatial web, sentiment tracking)
- Medical & reproductive health data (wearables)

**Result**: Corporations and governments monetize intimacy while individuals lose control of their identity.

---

## 4. THE SOLUTION: ËTRID FODDoS PROTOCOL

### What is FODDoS?

**FODDoS** = **Free and Open Decentralized Democracy of Stakeholders**

Unlike "Proof of Stake" or "Proof of Work," FODDoS is a **governance model**, not just a consensus algorithm.

**Core Components**:

1. **Consensus Day** (Annual Democratic Vote)
   - Every token holder votes on major decisions
   - Vote weight = Stake / Vote Dilution
   - Results are binding for 12 months

2. **Decentralized Directors** (Non-Hierarchical Board)
   - 9 directors elected annually by Consensus Day
   - No single president or CEO
   - Equal voting power
   - Can be recalled mid-term with 66% community supermajority

3. **Distribution Pay** (Reward Network Participation)
   - Validators, stakers, and voters earn ÉTR daily
   - Earnings based on participation and stake
   - Penalty system for absenteeism

4. **Foundation Charter** (Immutable Rules)
   - Legal structure defines how DAO operates
   - GPLv3 open-source ensures code freedom
   - Bylaws cannot be broken without hard fork + community vote

---

## 5. PROTOCOL ARCHITECTURE (E³20)

### Essential Elements to Operate Reference Implementation

ËTRID consists of **13 core subsystems** (E³20):

```
┌─────────────────────────────────────────────────────────────────┐
│                     LAYER 1: CORE INFRASTRUCTURE                 │
├─────────────────────────────────────────────────────────────────┤
│ 1. DETR p2p       │ Multi-protocol P2P networking               │
│ 2. OpenDID        │ Self-sovereign identity system              │
│ 3. Blockchain Sec │ Post-quantum cryptography                   │
│ 4. Accounts       │ EBCA, RCA, SCA, SSCA account types         │
├─────────────────────────────────────────────────────────────────┤
│                    LAYER 2: CHAIN ARCHITECTURE                   │
├─────────────────────────────────────────────────────────────────┤
│ 5. Multichain     │ FlareChain + PBCs + State channels          │
│ 6. Transactions   │ Regular, smart contract, cross-chain        │
│ 7. Native Crypto  │ ÉTR, ËDSC, VMw tokens                      │
├─────────────────────────────────────────────────────────────────┤
│                    LAYER 3: COMPUTATION & CONSENSUS              │
├─────────────────────────────────────────────────────────────────┤
│ 8. ËtwasmVM       │ WebAssembly smart contract runtime          │
│ 9. Consensus      │ ASF (Ascending Scale of Finality)           │
├─────────────────────────────────────────────────────────────────┤
│                    LAYER 4: GOVERNANCE & DISTRIBUTION            │
├─────────────────────────────────────────────────────────────────┤
│ 10. Foundation    │ DAO charter, legal framework, bylaws        │
│ 11. Peer Roles    │ Flare Nodes, Validity Nodes, DDs            │
│ 12. Governance    │ Consensus Day voting, electoral process     │
│ 13. Distribution  │ Daily reward pay for validators & stakers   │
├─────────────────────────────────────────────────────────────────┤
│                    LAYER 5: CLIENT IMPLEMENTATIONS               │
├─────────────────────────────────────────────────────────────────┤
│ Clients           │ CLI, Web wallet, Mobile wallet              │
└─────────────────────────────────────────────────────────────────┘
```

### 5.1 DETR p2p: Multi-Protocol Network Layer

**DETR** = Decentralized, Encrypted, Trustless, Resilient Peer-to-Peer

**Components**:
- **DPeers**: Node discovery using S/Kademlia DHT routing
- **AEComms**: TCP-based secure communications with ECIES encryption
- **DETRP2P**: Peer tethering for application session negotiation
- **Ëtr**: Base layer protocol for block synchronization
- **Fluent**: Secure, private channels using Lingo protocol
- **StoréD**: Distributed storage using S/Kademlia DHTs and Merkle DAGs

**Security Features**:
- End-to-end encryption (ECIES) between peers
- No trust required between peers
- Sybil-resistant through reputation system
- Censorship-resistant message routing

### 5.2 OpenDID: Self-Sovereign Identity

**OpenDID** = Ëtrid Open Decentralized Identification System

**Features**:
- Users control their own identifiers
- No central issuing authority
- Credential issuance by trusted communities
- Privacy-preserving: Selective disclosure of attributes
- Interoperable with W3C DID standard

**Use Cases**:
- KYC/AML compliance without centralized database
- Sybil resistance: Verified unique humans
- Developer identity: Reputation for bug bounties
- Community credentials: Membership proof

### 5.3 Blockchain Security: Post-Quantum Cryptography

**Hash Functions**:
- SHA-3 (Keccak) for general hashing
- Blake2b for performance-critical operations

**Digital Signatures**:
- EdDSA (Ed25519) - primary signature scheme
- SPHINCS+ (lattice-based) - post-quantum backup
- Hybrid mode during quantum transition period

**Key Management**:
- HKDF-Blake2b for key derivation
- BIP39 for mnemonic seeds
- BIP44 for hierarchical deterministic wallets

**Post-Quantum Readiness**:
- Network designed for algorithm agility
- Can switch to quantum-resistant algorithms via soft fork
- Keys can be upgraded without breaking addresses

### 5.4 Account Types

**EBCA** (External Blockchain Accounts)
- Any keypair generated outside ËTRID
- Cannot validate or propose blocks
- Can receive and send funds
- Example: MetaMask wallet connected to ËTRID

**RCA** (Root Chain Accounts)
- Generated by ËTRID Key Generation Protocol
- Valid on FlareChain (main chain) only
- Can vote, stake, validate
- SS58 address format with "1" prefix

**SCA** (Side Chain Accounts)
- Accounts on specific Partition Burst Chains
- Different keypair per chain allowed
- Faster local PBC transactions
- SS58 address format with chain-specific prefix

**SSCA** (Smart Side Chain Accounts)
- Controlled by ËtwasmVM smart contracts
- No private key (code-governed)
- Can execute transactions on behalf of contract owner
- Used for autonomous systems and DAO treasuries

### 5.5 Multichain Architecture

ËTRID implements a three-layer architecture for maximum scalability and efficiency:

```
┌──────────────────────────────────────────────────────┐
│  Layer 3: Lightning Bloc (Off-Chain Channels)        │
│  • Throughput: 100,000+ TPS                          │
│  • Latency: ~100ms per transaction                   │
│  • Cost: Zero fees (off-chain)                       │
│  • Settlement: Batch every 5 minutes                 │
└───────────────────┬──────────────────────────────────┘
                    │ Batch Settlement
                    ↓
┌──────────────────────────────────────────────────────┐
│  Layer 2: Partition Burst Chains (PBCs)              │
│  • Throughput: ~5,000 TPS per PBC                    │
│  • Latency: ~2s per block                            │
│  • Checkpoints: Every 256 blocks (~51 min)           │
│  • Security: 8 Validity Nodes per PBC (PPFA)         │
└───────────────────┬──────────────────────────────────┘
                    │ State Checkpoints
                    ↓
┌──────────────────────────────────────────────────────┐
│  Layer 1: FlareChain (Main Chain)                    │
│  • Throughput: ~1,000 TPS                            │
│  • Block time: 12 seconds                            │
│  • Finality: 2 blocks (~24s) via GRANDPA             │
│  • Consensus: ASF (Ascending Scale of Finality)      │
└──────────────────────────────────────────────────────┘
```

**Total Network Capacity**: 171,000+ TPS across all layers

#### FlareChain (Layer 1: Main Chain)

**Purpose**: Root chain coordination and governance
- Stores checkpoint registry from all PBCs
- Executes Consensus Day voting
- Manages Foundation operations
- Holds reserve collateral (for ËDSC)
- Coordinates PBC validator assignments
- Provides economic finality for all layers

**Performance**:
- Block time: 12 seconds
- Finality: 2 blocks (~24 seconds) via GRANDPA
- Throughput: ~1,000 TPS
- Storage: Merkle roots only (compact state commitments)

**Consensus**: ASF (Ascending Scale of Finality)
- 5-9 Decentralized Directors serve as Flare Nodes
- 66% quorum required for finality
- Byzantine fault tolerant (tolerates up to 33% malicious nodes)

#### Partition Burst Chains (Layer 2: Sidechains)

**Purpose**: High-throughput specialized chains

**Available PBCs**:
1. **PBC-EDSC**: ËDSC stablecoin operations (minting, redemption, oracle)
2. **PBC-BTC**: Bitcoin bridge (deposits, withdrawals, atomic swaps)
3. **PBC-ETH**: Ethereum bridge (ERC-20 token transfers)
4. **PBC-SOL**: Solana bridge (SPL token transfers)
5. **PBC-XRP**: Ripple bridge
6. **PBC-BNB**: Binance Chain bridge
7. **PBC-TRX**: Tron bridge
8. **PBC-ADA**: Cardano bridge
9. **PBC-MATIC**: Polygon bridge
10. **PBC-LINK**: Chainlink oracle network
11. **PBC-USDT**: USDT stablecoin operations
12. **PBC-DOGE**: Dogecoin bridge
13. **PBC-XLM**: Stellar bridge

**Performance per PBC**:
- Block time: ~2 seconds
- Throughput: ~5,000 TPS per chain
- Combined: 14 PBCs × 5,000 TPS = **70,000 TPS**

**Consensus**: PPFA (Partition Proof of Authority)
- 8 Validity Nodes per PBC
- Rotation every 256 blocks (~8.5 minutes)
- Byzantine fault tolerant (tolerates 2/8 malicious nodes)

**State Synchronization to FlareChain**:
```rust
// Checkpoint structure submitted every 256 blocks
pub struct Checkpoint {
    pub block_number: u64,           // PBC block number
    pub state_root: Hash,            // Merkle root of entire PBC state
    pub total_supply: u128,          // Economic snapshot
    pub reserve_ratio: u16,          // For stablecoin PBCs (basis points)
    pub timestamp: u64,              // Unix timestamp
}
```

**Checkpoint Submission Process**:
1. PBC block 256 finalizes
2. Validity Nodes calculate Merkle root of all accounts/balances
3. Collator submits checkpoint extrinsic to FlareChain
4. FlareChain Directors verify signature and store checkpoint
5. GRANDPA finalizes FlareChain block containing checkpoint
6. PBC state now immutably anchored on Layer 1

**Checkpoint Frequency**: Every 256 blocks = ~51 minutes
- Emergency checkpoints: Triggered by reserve ratio < 125% (for ËDSC)
- Forced checkpoints: If checkpoint missing for 512 blocks, PBC freezes

**Security Properties**:
- Optimistic finality on PBC (~2 seconds)
- Economic finality via checkpoint (~51 minutes)
- Absolute finality via FlareChain GRANDPA (~51.4 minutes)
- Fraud proofs: Anyone can challenge invalid checkpoints (7-day period)

#### Lightning Bloc Network (Layer 3: Payment Channels)

**Purpose**: Instant, zero-fee off-chain transactions

**Architecture**:
```
Alice ↔ Bob Channel
├─ Open: Lock 10,000 ÉTR each on FlareChain/PBC
├─ Transact: Update channel state off-chain (100,000+ tx)
│   ├─ Alice → Bob: 100 ÉTR (signed state update)
│   ├─ Bob → Alice: 50 ÉTR (signed state update)
│   └─ ... (instant, zero fees)
├─ Batch: Accumulate 1,000 transactions or 5 minutes
├─ Settle: Submit compressed batch to PBC (Merkle root only)
└─ Close: Cooperatively or via dispute resolution
```

**Performance**:
- Throughput: 100,000+ TPS (off-chain)
- Latency: ~100ms per transaction
- Cost: Zero fees (off-chain)
- Settlement: Every 5 minutes or 1,000 transactions

**Transaction Batching**:
```rust
pub struct TransactionBatch {
    pub batch_id: String,
    pub transactions: Vec<OffChainTransaction>,  // Max 1,000
    pub merkle_root: Vec<u8>,                   // 32 bytes
    pub compressed_data: Vec<u8>,               // ~105 KB (30% compression)
}
```

**Batch Settlement Process**:
1. Lightning Bloc manager accumulates transactions
2. When batch reaches 1,000 tx OR 5 minutes elapse:
   - Compress transactions (150 KB → 105 KB)
   - Calculate Merkle root
3. Submit settlement extrinsic to PBC or FlareChain
4. On-chain verifies Merkle root and updates channel states
5. Channel participants can now dispute if needed

**Settlement to FlareChain Timeline**:
- Lightning transaction occurs: **Instant** (off-chain)
- Batch settlement to PBC: **5 minutes** (batch timeout)
- PBC checkpoint to FlareChain: **51 minutes** (256 blocks)
- **Total**: ~56 minutes from Lightning tx to FlareChain finality

**Security Mechanisms**:
1. **Watchtower Network**: Monitors channels for fraud attempts
2. **Emergency Withdrawals**: 24-hour timeout for unresponsive counterparty
3. **Fraud Proofs**: Challenge invalid channel closures (7-day period)
4. **Optimistic Rollup**: Assume validity unless challenged
5. **State Challenges**: Newer signed states supersede old ones

**Emergency Withdrawal Flow**:
```
User requests withdrawal → 24-hour timeout
    ↓                           ↓
Counterparty responds?      No response?
    ↓                           ↓
Approve/Reject          Force execute using
    ↓                   last known checkpoint
Settlement                      ↓
                           Settlement
```

#### Multi-Layer State Propagation

**Complete Data Flow**:
```
Lightning Transaction (Layer 3)
├─ Alice → Bob: 100 ÉTR off-chain
├─ Signed by both parties
└─ State update in ~100ms
    ↓ (After 5 min or 1,000 tx)
Batch Settlement (Layer 2)
├─ Submit to PBC-EDSC
├─ Merkle root: 0xdef456...
├─ Update channel balances on-chain
└─ Lightning state now on PBC
    ↓ (After 256 PBC blocks = 51 min)
Checkpoint Submission (Layer 1)
├─ PBC submits checkpoint to FlareChain
├─ State root: 0xabc123... (includes Lightning balances)
├─ Directors verify and store checkpoint
└─ GRANDPA finality (24 seconds)
    ↓
Result: Lightning transaction anchored on FlareChain
Total latency: ~56 minutes (instant tx, delayed finality)
```

**State Query via Merkle Proofs**:
FlareChain Directors don't store full PBC state (too large). Instead:
1. Directors store only Merkle roots (32 bytes per PBC)
2. To verify any PBC state: Request Merkle proof from PBC node
3. Verify proof against stored checkpoint
4. Proof size: ~1 KB (32 hashes × 32 bytes)

Example:
```
Query: "What's Alice's balance on PBC-EDSC?"
1. Get latest checkpoint from FlareChain: state_root = 0xabc123
2. Request Merkle proof from PBC node
3. Verify: hash(Alice's balance + Merkle path) == 0xabc123
4. Result: Alice has 9,900 ÉTR (verified against Layer 1)
```

**Throughput Summary**:
- Layer 1 (FlareChain): ~1,000 TPS
- Layer 2 (14 PBCs): ~70,000 TPS combined
- Layer 3 (Lightning): ~100,000+ TPS
- **Total Network Capacity**: 171,000+ TPS
- **Practical Capacity** (80% utilization): ~137,000 TPS

### 5.6 Native Cryptocurrency

**ÉTR** (Ëtrid Coin) - Primary Token
- Total initial supply: 1 Billion
- Annual issuance: Voted by Consensus Day
- Uses: Payments, staking, voting, collateral
- Decimals: 18 (smallest unit: 1 Wei = 0.000000000000000001 ÉTR)

**ËDSC** (Ëtrid Dollar Stablecoin) - 1:1 USD Peg
- Total supply: 50 Billion ËDSC
- Initial circulation: 5 Billion
- Locked reserve: 45 Billion (governance-controlled release)
- Collateralization: 110-130%
- Redemption paths: 3 (Treasury vault, custodian, DEX)

**VMw** (Virtual Machine Watts) - Computation Gas
- Smart contract execution cost
- 1 VMw ≈ 0.001 ÉTR (market-based)
- All VMw consumed is burned (deflationary)
- Pay-per-opcode model

### 5.7 Smart Contracts & ËtwasmVM

**ËtwasmVM** = Ëtrid WebAssembly Virtual Machine

**Features**:
- Turing-complete runtime
- Sandboxed execution environment
- Gas metering to prevent infinite loops
- State storage with rent model
- Cross-contract calls and composition

**Supported Languages**:
- Rust (primary via wasm-pack)
- C/C++ (via Emscripten)
- AssemblyScript (TypeScript to WASM)

**Economic Model**:
- Storage rent: 0.1 VMw per byte per day
- Execution: Variable based on opcodes
- State access: 64 VMw per 32-byte read/write
- Cross-contract calls: Additional gas overhead

### 5.8 Consensus Algorithm: ASF

**ASF** = Ascending Scale of Finality

**Mechanism**:
- Validators propose and vote on new blocks
- Block becomes finalized when 66% of validators attest
- Finality achieved in ~25 validator slots (~5 minutes)
- Penalties for double-signing or equivocation

**Validator Rotation**:
- Every epoch: Top 100 validators by stake become active
- Active validator set: Minimum 25, maximum 100
- New validators: Enter queue with 64 ÉTR minimum stake
- Slashing: 1% of stake for security violations

---

## 6. GOVERNANCE & CONSENSUS DAY

### 6.1 Consensus Day: Annual Democratic Vote

**Schedule**:
- **Date**: December 1st at **12:00 AM PST** (hardcoded, changes require hard fork)
- **Pre-Consensus Period**: January 1 – October 31 (proposal submission, campaigns, nominations)
- **Frequency**: Once per year
- **Participation Types**:
  - **VALIDITY Nodes** (64+ ÉTR minimum stake): Block producers and consensus participants
  - **Common Stake Peers** (1+ ÉTR minimum stake): Governance voters and proposal supporters
  - **Decentralized Directors** (128+ ÉTR minimum stake): 9 elected board members

**Voting Power Calculation**:

```
Voting Power = Staked ÉTR × Coinage
```

Where **Coinage** represents time-weighted stake (how long tokens have been staked).

**Example**:
- You stake: 1,000 ÉTR for 180 days
- Coinage multiplier: ~1.5× (increases with time staked)
- Your voting power: 1,000 × 1.5 = 1,500 effective votes
- Longer stakes = higher voting power multiplier
- Incentivizes long-term stakeholder commitment

**Penalties**:
- Failure to vote = penalties and reward redistribution
- Rewards distributed to active participants

### 6.2 Consensus Day Ballot

**Three Categories**:

#### Category 1: Fiscal Mint & Supply
- **Question**: "How many ÉTR should be minted in the next 12 months?"
- **Options**: Top 3 community proposals + 3 limit options (min/mid/max)
- **Binding Effect**: Winning proposal becomes the annual mint rate
- **Implementation**: Automatic release Jan 1st following vote

**Example Options**:
- Proposal A: 50 Million ÉTR (5% annual emission)
- Proposal B: 75 Million ÉTR (7.5% annual emission)
- Proposal C: 100 Million ÉTR (10% annual emission)
- Limit 1: Min 25M (2.5%)
- Limit 2: Mid 62.5M (6.25%)
- Limit 3: Max 125M (12.5%)

#### Category 2: Decentralized Director Elections
- **Question**: "Who should serve as the 9 Decentralized Directors?"
- **Candidates**: All accounts with ≥128 ÉTR stake
- **Criteria**: Must meet "Honest Image" standards (TBD by community)
- **Term**: 1 year (Dec 1 - Nov 30)
- **Limit**: Max 3 consecutive terms (then 1-year break required)

**DD Roles** (Non-Hierarchical):
- Oversee FlareChain security and operations
- Approve major protocol upgrades
- Manage Foundation budget
- Coordinate with custodians and validators
- Respond to security incidents

**DD Compensation**:
- Salary: X% of annual fiscal mint (TBD)
- FLARE node rewards: Y% of block production (TBD)
- Clawback for misconduct: Full stake forfeiture

#### Category 3: Protocol Amendments
- **Question**: "What protocol changes should be approved?"
- **Options**: Top 3 community proposals
- **Voting**: ≥66% supermajority required
- **Implementation**: Takes effect Jan 1st following vote
- **Audit**: Major changes require 90-day security review

**Example Amendments**:
- Proposal A: Increase validator count from 25 to 50
- Proposal B: Add new oracle source (Uniswap TWAP)
- Proposal C: Adjust penalty percentages for validators

### 6.3 Voting Mechanics

**Vote Submission**:
- On-chain voting via extrinsic
- Ballot opened: December 1, 12:00 AM PST
- **Pre-Consensus Period**: January 1 – October 31 (proposal submission and campaigning)
- **Voting Power Formula**: Staked ÉTR × Coinage (time-weighted stake)
- Failure to vote = penalties and reward redistribution

**Vote Types**:
- Single-choice: Pick 1 option
- Ranked-choice (future): Rank preferences
- Quadratic voting (future): Vote strength weighted by √stake

**Privacy** (Future Enhancement):
- Zero-knowledge proofs for voter privacy
- V otes recorded without linking to addresses
- Cryptographic verification of vote counts

### 6.4 Soft Forks vs Hard Forks

**Soft Forks** (Governance-only):
- Requires: 50% + 1 on Consensus Day
- Examples: Change oracle sources, adjust validator count
- Implementation: Network node updates, no chain split
- Time to activate: ~30 days after vote

**Hard Forks** (Code changes):
- Requires: 66% supermajority on Consensus Day
- Audit period: 90 days minimum
- Community notice: At least 30 days before activation
- Optional rollback: Within 1 hour of fork (66% validator consent)

**Emergency Hard Forks** (Security only):
- Authorized by: 5/9 Decentralized Director quorum
- Community confirmation: 24-hour flash vote
- Duration: Max 7 days
- Post-fork audit: Mandatory

---

## 7. TOKEN ECONOMICS (ÉTR, ËDSC, VMw)

### 7.1 ÉTR Tokenomics

**Initial Distribution** (1 Billion ÉTR):

| Category | Amount | % | Purpose |
|----------|--------|---|---------|
| Circulating | 100M | 10% | Launch liquidity, exchange listings |
| Founder Allocation | 50M | 5% | Eoj Edred (subject to clawback) |
| Foundation Treasury | 100M | 10% | Development, grants, bug bounties |
| Locked Growth | 750M | 75% | Governance-controlled release via Consensus Day |

**Annual Emission Schedule**:
- Year 1 (2025): Voted by Consensus Day 2024 (baseline ~50M = 5%)
- Year 2 (2026): Voted by Consensus Day 2025
- Perpetual: Voted annually (no max cap in code, only via governance)

**Emission Logic** (Pseudo-code):

```
annual_mint = consensus_day_voted_amount
daily_distribution = annual_mint / 365

daily_distribution_breakdown:
  - Registered Voting Peers: P% (participation reward)
  - FLARE Nodes: Z% (validator rewards)
  - VALIDITY Nodes: W% (sidechain validator rewards)
  - Common Stake Peers: Q% (staking rewards)
  - Decentralized Directors: V% (compensation + rewards)
  - Foundation Treasury: (P + Z + W + Q + V) capped at 100%
```

**Token Utility**:

| Use Case | Amount | Requirement |
|----------|--------|-------------|
| **Voting** | 1 ÉTR | Minimum stake to participate in Consensus Day |
| **Staking (Flare Node)** | 1 ÉTR | Optional, for validator rewards |
| **Staking (Validity Node)** | 64 ÉTR | Required to operate sidechain validator |
| **DD Candidacy** | 128 ÉTR | To run for Decentralized Director |
| **Transactions** | 0.1 ÉTR | Average transfer fee |
| **Smart Contracts** | Variable | Gas fees paid in ÉTR (converted to VMw) |
| **ËDSC Collateral** | Variable | Backing for stablecoin system |

### 7.2 ËDSC Stablecoin Economics

**Total Supply**:
- Cap: 50 Billion ËDSC
- Initial: 5 Billion circulation
- Locked: 45 Billion (released via governance)

**Peg Maintenance**:
- Target: 1 ËDSC = 1.00 USD
- Acceptable range: 0.98 - 1.02 USD
- Oracle sources: Binance, Coinbase, Kraken, Bitstamp, Gemini
- Rebalancing: Automated via redemption incentives

**Collateralization Requirements**:

```
Collateral Ratio = Total Reserve Value / ËDSC Outstanding
Required Ratio = 110% (minimum)
Optimal Ratio = 120-130%
Emergency Ratio = 90% (triggers circuit breaker)

If Ratio < 100%: Redemption fees increase to 10%
If Ratio < 90%: All redemptions paused
```

**Reserve Composition**:
- On-chain collateral (FlareChain vault): 40-50%
- Custodian-held reserves (USD, bonds, T-bills): 50-60%
- Dynamically rebalanced monthly

### 7.3 VMw Gas Token

**Economics**:
- Base rate: 1 VMw ≈ 0.001 ÉTR (market-determined)
- Adjusted: Gas price can vary based on network load
- Burned: All VMw consumed is permanently destroyed
- Market price: Determined by Uniswap/DEX if listed

**Gas Costs**:

| Operation | Cost | Example |
|-----------|------|---------|
| Simple transfer | 0.01 VMw | ~0.00001 ÉTR |
| Account creation | 1 VMw | ~0.001 ÉTR |
| Smart contract call | 100-10,000 VMw | ~0.1-10 ÉTR |
| State write (32 bytes) | 64 VMw | ~0.064 ÉTR |
| Cross-chain message | 256 VMw base | ~0.256 ÉTR + payload |

**Deflationary Mechanics**:
- Every transaction reduces total VMw supply
- No VMw minting (created via ÉTR → VMw conversion only)
- Long-term: Deflationary pressure on ÉTR as network grows

---

## 8. ËTRID DOLLAR STABLECOIN (ËDSC) SPECIFICATION

### 8.1 ËDSC Overview

**Purpose**: Provide a USD-pegged stablecoin for commerce, savings, and DeFi within ËTRID

**Key Properties**:
- 1:1 peg to USD
- Non-custodial minting (no central bank required)
- Democratic governance (Consensus Day oversees reserve ratio)
- Emergency circuit breakers (automatic pause on imbalance)

### 8.2 ËDSC Architecture: Two-Chain Model

**FlareChain (Main Chain)**:
- Reserve Vault: Stores on-chain collateral (USDC, staked ÉTR)
- Custodian Registry: Tracks off-chain reserves with BitGo/Anchorage
- Reserve Oracle: Aggregates reserve data for proof-of-reserves

**PBC-EDSC (Dedicated Chain)**:
- Primary authority for ËDSC minting/burning
- Redemption engine: Processes 3-path redemptions
- Price oracle: TWAP-based pricing from multiple sources
- Checkpoint module: Syncs state to FlareChain every 100 blocks

### 8.3 ËDSC Minting: Multi-Collateral Model

**Path 1: On-Chain Collateral**
- User sends ÉTR or USDC to Reserve Vault on FlareChain
- Vault mints equivalent ËDSC on PBC-EDSC
- Collateral locked until redemption
- Requirements: 110% collateralization

**Path 2: Custodian Deposit**
- User sends USD to BitGo/Anchorage custody account
- Custodian verifies deposit
- Custodian authorizes ËDSC minting on PBC-EDSC
- Requirements: KYC/AML compliance

**Path 3: DEX/Market-Based**
- User buys ËDSC from Uniswap/PancakeSwap liquidity pools
- No collateral required (already minted by protocol)
- Price discovery via AMM

### 8.4 ËDSC Redemption: 3-Path System

**Path 1: Treasury Redemption** (60% of daily capacity)
- User submits ËDSC redemption on PBC-EDSC
- System transfers ÉTR or USDC from FlareChain vault
- Settlement: 2-4 hours
- Fees: 0.25-0.5% (when reserves > 115%)

**Path 2: Custodian Redemption** (30% of daily capacity)
- User requests USD withdrawal via BitGo/Anchorage
- Custodian ships USD (bank wire)
- Settlement: 1-3 business days
- Fees: 0.25-1% (wire + proof-of-custody costs)

**Path 3: DEX Redemption** (10% of daily capacity)
- User sells ËDSC on Uniswap/PancakeSwap
- Receives ÉTR or other tokens
- Settlement: Instant
- Fees: 0.3% (DEX fee) + 0.25% (arbitrage buffer)

### 8.5 Dynamic Fee Schedule

**Fee Tiers Based on Reserve Ratio**:

| Reserve Ratio | Redemption Fee | Status |
|---------------|----------------|--------|
| > 130% | 0.25% | Optimal (encourage redemptions) |
| 120-130% | 0.35% | Healthy |
| 110-120% | 0.75% | Caution |
| 100-110% | 2.0% | Warning |
| 90-100% | 5.0% | Critical |
| < 90% | 🚫 PAUSED | Emergency mode |

**Rationale**:
- High reserves → Low fees → Encourages redemptions → Reduces ËDSC supply
- Low reserves → High fees → Discourages redemptions → Preserves collateral

### 8.6 Price Oracle System

**Primary Oracle** (24-hour TWAP):
- Sources: Binance, Coinbase, Kraken, Bitstamp, Gemini
- Price window: 24 hours
- Outlier rejection: Ignore prices >2% from median
- Minimum sources: 5 active feeds

**Fallback Oracle** (7-day TWAP):
- Activated: If primary oracle stale for >100 blocks
- Sources: Same 5 primary sources
- Price window: 7 days
- More resistant to short-term price manipulation

**Oracle Failure**:
- If <3 sources available: Circuit breaker pause
- Manual recovery: 5/9 DD board approval required

### 8.7 Circuit Breaker System

**Automated Safety Mechanisms**:

```
IF reserve_ratio < 100%:
  max_redemption_cap = 50% of daily capacity
  redemption_fee = 5%
ELSE IF reserve_ratio < 90%:
  PAUSE all redemptions
  activate_emergency_mode
ELSE IF pending_redemptions > 10,000:
  throttle_new_redemptions = true
ELSE IF oracle_staleness > 100_blocks:
  fallback_to_7day_twap()
```

**Manual Circuit Breaker** (5/9 DD Board):
- Can pause system for ≤7 days in emergency
- Must notify community within 1 hour
- Post-pause audit mandatory
- Cannot be extended beyond 7 days without Consensus Day vote

### 8.8 Reserve Composition & Custody

**On-Chain Reserve** (FlareChain Vault):
- USDC (Polygon/Ethereum bridged): 30%
- ÉTR (collateral pool): 10%
- Short-term T-Bills (tokenized): 5%
- Total: 45% of reserves

**Custodian Reserve** (BitGo/Anchorage):
- USD cash (bank accounts): 40%
- US Treasury bills: 10%
- Money market funds: 5%
- Total: 55% of reserves

**Rebalancing**:
- Monthly check: Is on-chain vs custodian ratio correct?
- Adjustment: Move USD to/from FlareChain vault
- Triggers: If ratio drifts >5% from target

**Proof-of-Reserves**:
- Quarterly audit by external firm
- On-chain vault state verified daily
- Custodian statements published on-chain (via oracle)
- Community can verify full reserve backing

### 8.9 CCTP Cross-Chain Integration

**Purpose**: Enable native ËDSC transfers across 8 major blockchains without wrapped tokens

**Architecture**: Circle CCTP-style burn-and-mint protocol

**Supported Blockchains**:
1. Ethereum (Domain 0)
2. Avalanche (Domain 1)
3. Optimism (Domain 2)
4. Arbitrum (Domain 3)
5. Base (Domain 4)
6. Polygon (Domain 5)
7. Solana (Domain 6)
8. Ëtrid PBC-EDSC (Domain 7)

#### CCTP Transfer Flow

**Example: Ethereum → Ëtrid (4-6 minutes total)**

```
[1] Burn on Source Chain (Ethereum)
User calls: EDSCTokenMessenger.depositForBurn()
├─ Burns EDSC tokens on Ethereum
├─ Emits MessageSent event with:
│   ├─ Nonce: Unique message ID
│   ├─ Source Domain: 0 (Ethereum)
│   ├─ Destination Domain: 7 (Ëtrid)
│   ├─ Recipient: Ëtrid address
│   └─ Amount: EDSC amount
└─ Gas cost: ~$2-5 USD

[2] Attestation (2-3 minutes)
5 Attestation Services monitor MessageSent
├─ Each attester verifies:
│   ├─ Event authenticity ✓
│   ├─ Burn confirmation ✓
│   ├─ Nonce not reused ✓
│   └─ Amount within limits ✓
├─ 3/5 attesters sign message (M-of-N threshold)
└─ Signatures stored off-chain (IPFS/Arweave)

[3] Relay (2-3 minutes)
Relayer Service collects attestations
├─ Wait for 3/5 signatures
├─ Construct receiveMessage() call
└─ Submit to Ëtrid PBC-EDSC

[4] Mint on Destination Chain (Ëtrid)
PBC-EDSC pallet_edsc_bridge_attestation
├─ Verify signatures (3/5 threshold) ✓
├─ Check nonce not used ✓
├─ Validate domain IDs ✓
├─ Mint EDSC to recipient ✓
└─ Mark nonce as used

Total: ~4-6 minutes for cross-chain transfer
```

#### Security Model

**M-of-N Attestation**:
- **M = 3** signatures required (threshold)
- **N = 5** total attestation services
- Byzantine fault tolerant (tolerates 2 malicious attesters)

**Attester Requirements**:
- Minimum stake: 10,000 ÉTR (slashed for fraud)
- Uptime: 99%+ required
- Independent infrastructure (no shared hosting)
- Rotation: Every 90 days or via governance vote

**Nonce Management**:
- Sequential nonces per source domain
- Prevents replay attacks
- Stored in `UsedNonces` mapping
- Cannot reuse old nonces

**Domain Separation**:
- Each blockchain has unique domain ID
- Messages only valid for specific domain pair
- Prevents cross-domain replay

**Rate Limiting**:
- Per-domain daily limits (e.g., 1M EDSC from Ethereum)
- Per-message maximum (e.g., 100K EDSC per tx)
- Adjustable via governance

#### Component Architecture

**Substrate Pallets (Ëtrid Side)**:
1. `pallet-edsc-bridge-token-messenger`
   - Handles burn/mint logic
   - Interfaces with pallet-assets (EDSC)
   - Emits MessageSent/MessageReceived events

2. `pallet-edsc-bridge-attestation`
   - Verifies M-of-N signatures
   - Manages attester registry
   - Enforces nonce uniqueness

**EVM Smart Contracts (External Chains)**:
1. `EDSCTokenMessenger.sol`
   - depositForBurn(): Burns EDSC on source chain
   - receiveMessage(): Mints EDSC on destination
   - Emits MessageSent event

2. `EDSCMessageTransmitter.sol`
   - receiveMessage(): Verifies attestations
   - replaceMessage(): Allows message updates (with higher fee)
   - revokeAttestation(): Emergency pause by attester

**Off-Chain Services**:
1. `attestation-service` (5 instances)
   - Monitors source chains for MessageSent events
   - Signs messages if valid
   - Publishes signatures to IPFS
   - Written in Rust, deployed as systemd service

2. `relayer-service` (decentralized)
   - Collects attestations from IPFS
   - Submits receiveMessage() to destination chain
   - Incentivized via relay fees (0.1% of amount)

#### Performance Metrics

| Metric | Target | Notes |
|--------|--------|-------|
| **Transfer Time** | 4-13 min | Depends on chain finality |
| **Ethereum → Ëtrid** | 4-6 min | 12 confirmations (2.4 min) |
| **Solana → Ëtrid** | 2-3 min | 32 confirmations (20s) |
| **Ëtrid → Ethereum** | 6-8 min | GRANDPA finality + Eth conf |
| **Cost** | $2-10 | Gas + relay fee |
| **Daily Volume** | 1M EDSC | Per-domain limit (adjustable) |

#### Integration with PBC Checkpoints

**CCTP State Synchronization**:
- CCTP bridge state (nonces, attestations) stored on PBC-EDSC
- Checkpointed to FlareChain every 256 blocks (~51 min)
- FlareChain stores Merkle root of CCTP state
- Enables cross-chain fraud proofs (7-day challenge)

**Example Checkpoint**:
```rust
pub struct EDSCCheckpoint {
    pub block_number: u64,
    pub state_root: Hash,              // Merkle root of all PBC state
    pub cctp_nonce_root: Hash,         // Merkle root of used nonces
    pub total_bridged_in: u128,        // Total EDSC minted via CCTP
    pub total_bridged_out: u128,       // Total EDSC burned via CCTP
    pub timestamp: u64,
}
```

#### Emergency Scenarios

**Scenario 1: Attester Compromise (2/5 malicious)**
- System continues operating (needs 3/5 signatures)
- Governance vote to replace malicious attesters
- Slash stakes of compromised attesters

**Scenario 2: Source Chain Reorg**
- Attesters wait for finality before signing
- Ethereum: 12 confirmations (~2.4 min)
- Polygon: 256 blocks (~8.5 min)
- Prevents false mints from reverted burns

**Scenario 3: Destination Chain Congestion**
- Relayer queues transactions
- Users can self-relay if needed (call receiveMessage directly)
- Emergency: Increase gas price for faster inclusion

**Scenario 4: Circuit Breaker Activation**
- If bridge exploited, 3/5 attesters can pause
- Requires 3/5 signatures to call pauseBridge()
- Resume requires 5/9 DD board vote + 24-hour timelock

#### User Experience

**Transfer Steps (User Perspective)**:
1. Connect wallet to source chain (e.g., MetaMask on Ethereum)
2. Approve EDSC token spending (one-time)
3. Enter amount and destination address
4. Click "Transfer to Ëtrid"
5. Wait 4-6 minutes for attestations
6. EDSC appears on Ëtrid PBC-EDSC ✓

**No Wrapped Tokens**:
- EDSC on Ethereum is native EDSC (not wEDSC)
- EDSC on Ëtrid is native EDSC (not bridged)
- Seamless UX: Same ticker, same decimals

**Comparison to Traditional Bridges**:

| Feature | CCTP (ËDSC) | Wormhole | LayerZero |
|---------|-------------|----------|-----------|
| **Wrapped Tokens** | ❌ No | ✅ Yes (wETH, wBTC) | ✅ Yes |
| **Trust Model** | 3/5 M-of-N | 19 Guardians | Oracles + Relayers |
| **Transfer Time** | 4-13 min | 15 min | 10-20 min |
| **Costs** | $2-10 | $5-15 | $3-12 |
| **Liquidity** | None (burn/mint) | Bridge pools | Bridge pools |

#### Future Enhancements

**Phase 2 (6 months)**:
- Add support for Base, zkSync, Linea
- Implement batch attestations (reduce gas)
- Mobile SDK (React Native)

**Phase 3 (12 months)**:
- Non-EVM chains (Cosmos, Near, Aptos)
- Zero-knowledge proofs for privacy
- Intent-based relaying (user specifies max time)

---

## 9. PEER ARCHITECTURE & NODE TYPES

### 9.1 Peer Categories

**Common Peers** (Network Users)
- Wallets without staked ÉTR
- Can receive/send transactions
- Cannot vote in Consensus Day
- Cannot receive Distribution Pay
- Run light clients (query via DETRP2P)

**Common Stake Peers** (Stakeholders)
- Hold ≥1 ÉTR staked in network
- Full voting rights in Consensus Day
- Eligible for Distribution Pay rewards
- Can run full nodes or light nodes
- Voting power determined by stake × coinage

**Flare Nodes** (FlareChain Validators / Layer 1)
- **Role**: Maintain FlareChain blockchain (main chain)
- **Responsibilities**:
  - Propagate FlareChain blocks across network
  - Verify and store PBC checkpoints in checkpoint registry
  - Participate in GRANDPA finality (2-block finalization)
  - Execute Consensus Day voting and governance
  - Coordinate PBC validator assignments
- **Requirements**:
  - Must be Decentralized Directors (5-9 elected board members)
  - Minimum stake: 128 ÉTR (for DD eligibility)
  - Hardware: 4-core CPU, 16 GB RAM, 500 GB SSD, 100 Mbps network
  - Uptime: 99%+ required for full rewards
- **Penalties**:
  - Missing blocks: 1% reward reduction per missed block
  - Double-signing: Slashing of stake (up to 10%)
  - Consensus failures: 2% reward reduction per failure
- **Rewards**: Z% annual mint / active Flare Node count

**Validity Nodes** (PBC Validators / Layer 2)
- **Role**: Operate Partition Burst Chain consensus (sidechains)
- **Responsibilities**:
  - Propose and validate blocks on assigned PBC
  - Calculate and submit state checkpoints to FlareChain every 256 blocks
  - Maintain PBC state (accounts, balances, contracts, Lightning channels)
  - Respond to state queries with Merkle proofs
  - Process Lightning Bloc batch settlements
- **Requirements**:
  - Minimum stake: 64 ÉTR (locked in escrow)
  - Hardware: 2-core CPU, 8 GB RAM, 100 GB SSD, 50 Mbps network
  - Rotation: 8 validators per PBC, rotate every 256 blocks (~8.5 min)
  - Must run PBC collator software (e.g., `edsc-pbc-collator`)
- **Assignment**:
  - Validators 6-21: Currently available for PBC assignment
  - Example: PBC-EDSC (validators 6-13), PBC-BTC (validators 14-21)
  - More validators added as network grows
- **Penalties**:
  - Missing checkpoint submissions: 5% reward reduction
  - Invalid state roots: Slashing of stake (up to 50%)
  - Downtime > 10%: Removal from rotation
- **Rewards**: W% annual mint / active Validity Node count

**Interaction Between Flare Nodes and Validity Nodes**:
```
┌─────────────────────────────────────────────────────────┐
│  Validity Nodes (PBC Validators 6-21)                   │
│  ├─ Validator 6: PBC-EDSC collator                      │
│  ├─ Validator 7: PBC-EDSC collator                      │
│  ├─ ... (8 total per PBC)                               │
│  └─ Validator 13: PBC-EDSC collator                     │
│                                                          │
│  Every 256 blocks (~51 minutes):                        │
│  1. Calculate Merkle root of entire PBC state           │
│  2. Create checkpoint extrinsic                         │
│  3. Submit to FlareChain                                │
└──────────────────────┬──────────────────────────────────┘
                       │
                       │ Checkpoint Extrinsic
                       ↓
┌─────────────────────────────────────────────────────────┐
│  Flare Nodes (Directors 1-5)                            │
│  ├─ Director 1: FlareChain validator                    │
│  ├─ Director 2: FlareChain validator                    │
│  ├─ ... (5-9 total)                                     │
│  └─ Director 5: FlareChain validator                    │
│                                                          │
│  For each checkpoint received:                          │
│  1. Verify collator signature                           │
│  2. Store in checkpoint registry                        │
│  3. Finalize via GRANDPA (2 blocks)                     │
│  4. Checkpoint now immutable                            │
│                                                          │
│  Storage: Merkle roots only (32 bytes per PBC)          │
│  Full PBC state NOT stored on FlareChain                │
└─────────────────────────────────────────────────────────┘
```

**Key Architectural Principle**:
> Only Decentralized Directors can serve as Flare Nodes. This prevents validator centralization and ensures governance accountability. Validators 6-21+ serve as Validity Nodes for PBCs, not as FlareChain validators.

**Why Two Separate Validator Types?**
1. **Separation of Concerns**:
   - Flare Nodes focus on governance and coordination (Layer 1)
   - Validity Nodes focus on high-throughput transactions (Layer 2)
2. **Scalability**:
   - FlareChain stays lean (1,000 TPS, 5-9 validators)
   - PBCs scale horizontally (5,000 TPS each, 8 validators per PBC)
3. **Security**:
   - FlareChain: Democratic accountability via elections
   - PBCs: Economic security via stake and checkpoints
4. **Efficiency**:
   - FlareChain: Stores only Merkle roots (compact)
   - PBCs: Store full state (larger, but localized)

**Decentralized Directors** (Governance Board)
- 9-person elected board (non-hierarchical)
- Elected annually on Consensus Day
- **Requirements**: 128 ÉTR minimum stake, must serve as OD Flare Nodes
- **Term Limits**: 1-year terms, 1-year cooldown between terms, **maximum 3 lifetime terms**
- **Compensation**: Community-voted salaries (determined annually on Consensus Day)
- **Duties**: Oversight, proposals, security decisions
- **No special voting power**: All major decisions require 66% community supermajority

**Community Developers** (Open-Source Contributors)
- Submit pull requests to ËTRID codebase
- Eligible for bug bounties (0.1-500 ÉTR)
- Track record rewards: Veteran devs get priority
- Registered with Foundation

### 9.2 Node Requirements

**Flare Node**:
- CPU: 4-core minimum (8-core recommended)
- RAM: 16 GB minimum
- Disk: 500 GB SSD (grows at ~10 GB/month)
- Network: 100 Mbps upload/download
- Uptime: 99%+ required for full rewards
- Software: Ëtrcpp C++ implementation

**Validity Node** (PBC):
- CPU: 2-core minimum
- RAM: 8 GB minimum
- Disk: 100 GB SSD (sidechain specific)
- Network: 50 Mbps minimum
- Stake: 64 ÉTR (escrow locked)
- Participation: Rotation window 256 blocks

**Light Client**:
- CPU: Any (even smartphone)
- RAM: 100-500 MB
- Disk: 0 (no blockchain storage)
- Network: 1 Mbps minimum
- Wallets, voting, staking all supported

---

## 10. DISTRIBUTION PAY SYSTEM

### 10.1 Daily Distribution Schedule

**Distribution Event**: Happens every 24 hours at fixed times (PST):

```
12:01 AM → Registered Voting Peers (participation rewards)
 4:01 AM → FLARE Nodes (validator rewards)
 6:01 AM → VALIDITY Nodes (sidechain validator rewards)
 8:01 AM → Common Stake Peers (staking rewards)
12:01 PM → Decentralized Directors (salary + rewards)
```

### 10.2 Distribution Formulas

**1. Registered Voting Peers** (Participation Reward):
```
reward = (P% × annual_mint / 365) × (your_vote_weight / total_vote_weight)

where:
  P% = Percentage allocated to voting (e.g., 10%)
  annual_mint = Voted on Consensus Day
  your_vote_weight = your_stake / vote_dilution
  total_vote_weight = sum of all vote weights
```

**2. FLARE Nodes** (Block Production):
```
reward = (Z% × annual_mint / 365) × (your_uptime / 100%) × (1 - penalties)

where:
  Z% = Percentage allocated to validators (e.g., 30%)
  your_uptime = % of blocks you produced vs. expected
  penalties = 1% per missing block + 2% per consensus failure
```

**3. VALIDITY Nodes** (Sidechain Production):
```
reward = (W% × annual_mint / 365) × (your_epoch_participation / epoch_slots) × (1 - penalties)

where:
  W% = Percentage allocated to PBC validators
  your_epoch_participation = your produced blocks
  epoch_slots = total slots in epoch (256 blocks / 8 validators = 32 per validator)
```

**4. Common Stake Peers** (Staking Reward):
```
reward = (Q% × annual_mint / 365) × (your_stake / total_stake) × coinage_multiplier

where:
  Q% = Percentage allocated to stakers (e.g., 40%)
  your_stake = your ÉTR locked
  coinage_multiplier = your_average_coin_age / 365
    (max multiplier: 2.0 for coins held >1 year)
```

**5. Decentralized Directors** (Salary + Rewards):
```
reward = (V% × annual_mint / 365 / 9) + (Z% FLARE rewards if running node)

where:
  V% = Percentage allocated to DD compensation (e.g., 5%)
  9 = Number of directors
  Additional FLARE rewards if DD also operates Flare Node
```

### 10.3 Penalties & Clawback

**Voting Penalties**:
- Absentee vote (≥2 missed Consensus Days): -0.05% annual stake

**Validator Penalties**:
- Missing block proposal: -0.1% annual stake
- Double-sign attempt: -1% annual stake
- 3 consecutive penalties: Node removal for 1 year

**DD Penalties**:
- Absentee voting (>2 missed votes): -25% compensation
- Misconduct: Full stake clawback + removal
- Security failure: Temporary suspension

**Penalty Redistribution**:
- All penalties redistributed to compliant participants
- Bonus pool: +20% for validators with 100% uptime
- Voting bonus: +50% APR for stakers who voted

---

## 11. FOUNDATION CHARTER & LEGAL FRAMEWORK

### 11.1 ËTRID Foundation Structure

**Entity Type**: Delaware Non-Profit Corporation (or equivalent)

**Governance**:
- Board of Directors: 9 Decentralized Directors (elected annually)
- Non-hierarchical: No CEO, equal voting power
- Committees: Technical, Legal, Community, Security
- Term limits: Max 3 consecutive years (then 1-year break)

**Funding**:
- Annual budget: Allocated via Consensus Day vote
- Source: Portion of fiscal mint (V%)
- Treasury: Managed by multi-sig (5/9 DD approval)
- Transparency: All expenditures published quarterly

**Legal Obligations**:
- Ensure GPLv3 open-source compliance
- Represent ËTRID in regulatory matters
- Manage intellectual property (trademarks)
- Enforce Foundation bylaws
- Dispute resolution

### 11.2 Intellectual Property & Licensing

**GPLv3 License**:
- All ËTRID code remains open-source
- Derivatives must also be open-source
- Commercial use allowed with attribution
- Patenting explicitly prohibited

**Trademarks**:
- ËTRID™: Managed by Foundation
- ËDSC™: Managed by Foundation
- FODDoS™: Managed by Foundation
- Community allowed to use with attribution

**Contributor Agreement**:
- All contributors sign Contributor License Agreement (CLA)
- Contributions licensed under GPLv3
- Contributor retains copyright
- Foundation gets perpetual license

### 11.3 Community Developer Programs

**Bug Bounty Program**:
- Low: 0.1-1 ÉTR (typos, documentation)
- Medium: 1-5 ÉTR (non-critical bugs)
- High: 5-50 ÉTR (security issues)
- Critical: 50-500 ÉTR (remote code execution, consensus failure)

**Audit Rewards**:
- Independent security audit: 100-500 ÉTR + percentage of bugs found
- Academic research: 25-200 ÉTR per published paper
- Documentation improvements: 5-50 ÉTR

**Track Record Program**:
- Veteran developers (10+ merged PRs): Priority for bug assignments
- Salary scale: High contributors eligible for Foundation staff
- Grants: Up to 100 ÉTR for major feature development

---

## 12. TECHNICAL SPECIFICATIONS & NETWORK PARAMETERS

### 12.1 Network Parameters

| Parameter | Value | Notes |
|-----------|-------|-------|
| Block time | 12 seconds | Target time between blocks |
| Finality time | ~5 minutes | ASF consensus with ~25 validators |
| Max block size | 4 MB | Substrate default |
| Network protocol | DETR p2p | Custom S/Kademlia DHT + ECIES |
| Default P2P port | 30333 | TCP listening port |
| RPC port | 9933 | HTTP JSON-RPC |
| WS port | 9944 | WebSocket RPC |
| State version | Polkadot v27+ | Post-quantum ready |
| Epoch duration | 256 blocks | ~51 minutes (12s × 256) |
| Validator rotation | Every epoch | PBC validators rotate per PPFA |

### 12.2 Account & Address Format

**SS58 Address Standard**:
- Checksum: Blake2b-160 (last 2 bytes)
- Encoding: Base58 (Bitcoin-style)
- Format: `<prefix><pubkey><checksum>`

**Address Prefixes by Chain**:
- FlareChain: `1...` prefix (SS58 prefix 0)
- PBC-EDSC: `3...` prefix (SS58 prefix 3)
- Example FlareChain: `1XYZabc...` (47 characters)

**Example Addresses**:
- Mainnet user: `1DeQnhKdyoYGjGpkkzZbRGJGnmL8SthKV8u3q`
- EDSC staker: `3EdWjSi1YvL2a8GYr2yQ98YbGbBLNhqhwXc8n`

### 12.3 Transaction Model

**Extrinsic Types**:
1. **Regular Transaction**: Transfer ÉTR between accounts
2. **Smart Contract Call**: Execute ËtwasmVM bytecode
3. **Staking Transaction**: Stake/unstake ÉTR
4. **Governance Transaction**: Vote or propose on Consensus Day
5. **Cross-Chain Message**: Send to PBC or another chain

**Transaction Fees**:
- Base fee: 0.01 ÉTR per kilobyte of encoded data
- Per-byte: 0.001 ÉTR per byte
- Weight multiplier: 0.00001 ÉTR per weight unit
- Optional tip: For faster inclusion

**Fee Example**:
```
Transfer 1000 ÉTR to another address:
- Size: 128 bytes → 0.001 × 128 = 0.128 ÉTR
- Weight: 50,000 units → 0.00001 × 50,000 = 0.5 ÉTR
- Total: 0.628 ÉTR
```

---

## 13. SECURITY, CRYPTOGRAPHY & POST-QUANTUM READINESS

### 13.1 Cryptographic Algorithms

**Hashing**:
- SHA-3 (Keccak): General-purpose hashing
- Blake2b: Performance-critical paths
- Merkle trees: State verification

**Digital Signatures**:
- EdDSA (Ed25519): Primary signature scheme
- SPHINCS+ (lattice-based): Post-quantum alternative
- Hybrid mode: Both algorithms used during transition

**Key Encryption**:
- ECIES (Elliptic Curve Integrated Encryption Scheme)
- ChaCha20-Poly1305: Stream encryption
- AES-256-GCM: Alternative symmetric encryption

**Key Derivation**:
- HKDF-Blake2b: Key stretching
- Argon2: Password hashing
- BIP39: Mnemonic seed generation

### 13.2 Post-Quantum Readiness

**Why Post-Quantum?**:
- Quantum computers (10+ years away) will break ECDSA
- ËTRID designed with algorithm agility
- Can migrate to quantum-resistant algorithms via soft fork

**Hybrid Approach**:
- Year 1-5: Ed25519 (fast, proven)
- Year 5+: Gradual migration to SPHINCS+
- Long-term: Full lattice-based cryptography

**Key Migration Path**:
1. User generates new keypair with SPHINCS+
2. Register new address on FlareChain
3. Send funds from Ed25519 address to SPHINCS+ address
4. Continue using new address
5. No disruption to network

### 13.3 Consensus Security

**Byzantine Fault Tolerance**:
- Network can tolerate <33% malicious validators
- ASF finality requires 66% quorum
- Double-signing slashes 100% of stake
- Equivocation penalties prevent censorship

**Network-Level Security**:
- DETR p2p: Secure P2P communications with ECIES
- DPeers: Sybil-resistant via reputation system
- No single point of failure (no central coordinator)

**Validator Security**:
- Minimum stake: 64 ÉTR for PBC, 1 ÉTR for Flare nodes
- Stake slashing: Up to 1% for security violations
- Removal: 3 strikes and out for 1 year
- Reputation: Validators track uptime publicly

---

## 14. DEPLOYMENT ROADMAP & MILESTONES

### Phase 1: Core Infrastructure (Weeks 1-4) ✅ COMPLETE
- ✅ DETR p2p networking
- ✅ OpenDID identity system
- ✅ Core cryptography
- ✅ FlareChain runtime
- **Status**: Production-ready

### Phase 2: Economics & Governance (Weeks 5-8) ✅ COMPLETE
- ✅ ÉTR token implementation
- ✅ Consensus Day voting system
- ✅ Distribution Pay rewards
- ✅ DD election mechanism
- **Status**: Production-ready

### Phase 3: Stablecoins & EDSC (Weeks 9-16) 🔄 IN PROGRESS
- ✅ pallet-edsc-token (minting)
- ✅ pallet-edsc-receipts (SBT proofs)
- ✅ pallet-edsc-redemption (3-path engine)
- ✅ pallet-edsc-oracle (TWAP pricing)
- 🔄 pallet-edsc-checkpoint (state sync - in progress)
- 🔄 pallet-circuit-breaker (safety - in progress)
- **Target**: Dec 2025

### Phase 4: Partition Burst Chains (Weeks 17-20) 🎯 NEXT
- Custom PBC infrastructure
- PBC governance framework
- First community PBC deployment
- **Target**: Jan 2026

### Phase 5: Legal & DAO Registration (Weeks 21-24) ⏳ PENDING
- Delaware Foundation incorporation
- Legal agreements finalized
- SEC/regulatory review (if needed)
- **Target**: Feb 2026

### Phase 6: Smart Contracts & ËtwasmVM (Weeks 25-28) ⏳ PENDING
- ËtwasmVM deployment
- Smart contract SDK/toolkit
- Developer grants program
- **Target**: Mar 2026

### Phase 7: AI Governance (Weeks 29-32) ⏳ PLANNED
- pallet-ai-authority (AI attestation)
- pallet-attestation-verifier (proof verification)
- pallet-poc-oracle (compute proofs)
- **Target**: Apr 2026

### Phase 8: Mainnet Launch (Week 32+) 🚀 GOAL
- Full mainnet deployment
- Token exchange listings (DEX + CEX)
- Foundation handoff to elected DD board
- **Target**: May 2026

---

## 15. FREQUENTLY ASKED QUESTIONS

**Q: Why is ÉTR's initial mint 1 Billion? Can it go higher?**
A: 1B is the bootstrapping amount. Annual emission is voted by Consensus Day (Dec 1st), so the community controls all future supply. There's no hard cap in code—only governance cap.

**Q: What if Consensus Day voting has low turnout?**
A: Voting is voluntary, but penalties apply for >2 missed votes (-0.05% annual stake). Vote dilution mechanism (stake / coinage) prevents whale dominance.

**Q: How is ËDSC collateralization maintained if ÉTR price crashes?**
A: Multiple collateral types (USDC, T-Bills, custodian USD) provide stability. If ÉTR drops, custodian reserves compensate. Circuit breaker pauses redemptions if ratio falls below 90%.

**Q: Can the Decentralized Directors abuse their power?**
A: No—they have no special voting power in Consensus Day. All major decisions require 66% community supermajority. Misconduct → immediate clawback of 128 ÉTR stake + removal.

**Q: What if a validator tries to double-sign?**
A: Automatic detection via ASF consensus. Slashing: 100% of annual validator rewards + removal for 1 year + potential stake clawback.

**Q: How do I run a Flare Node?**
A: 4 core CPU, 16 GB RAM, 500 GB SSD. Download Ëtrcpp, sync chain (~100 GB), and stake ≥1 ÉTR. Optional but rewards are available.

**Q: Can ËTRID hard fork like Ethereum did?**
A: Only via 66% Consensus Day supermajority + 90-day audit + 30-day community notice. Attempts to fork without consensus = rejected by network majority.

**Q: Why WASM instead of Solidity?**
A: WASM is language-agnostic (Rust, C++, Go, etc.). Solidity is EVM-only. WASM is more flexible, safer, and interoperable with non-blockchain systems.

**Q: What about privacy/anonymity?**
A: All transactions are transparent (like Bitcoin). Optional: Use zero-knowledge proofs for private voting (Consensus Day upgrade).

**Q: Is ËTRID scalable?**
A: FlareChain does ~1,000 TPS. PBCs add horizontal scaling (each PBC = separate chain). Lightning Bloc state channels for micropayments. Total throughput: 10,000+ TPS across all layers.

---

## 16. APPENDICES

### Appendix A: Glossary

| Term | Definition |
|------|-----------|
| **ASF** | Ascending Scale of Finality - consensus algorithm requiring 66% validator quorum |
| **Consensus Day** | Annual democratic vote on Dec 1st for major protocol decisions |
| **Coinage** | Average age of coins in network (used for vote dilution) |
| **DD** | Decentralized Director - elected board member |
| **DETR p2p** | Custom peer-to-peer networking protocol with encryption & DHT |
| **EBCA** | External Blockchain Account (non-ËTRID keypair) |
| **ËDSC** | Ëtrid Dollar Stablecoin (1:1 USD peg) |
| **ÉTR** | Ëtrid Coin - native token |
| **E³20** | Essential Elements to Operate (13 core subsystems) |
| **FODDoS** | Free and Open Decentralized Democracy of Stakeholders |
| **Flare Node** | Validator for FlareChain (main chain) consensus |
| **PBC** | Partition Burst Chain (sidechain for specific use cases) |
| **PPFA** | Partition Proof of Authority (8-validator rotation per PBC) |
| **RCA** | Root Chain Account (generated by ËTRID protocol) |
| **SBT** | Soulbound Token (non-transferable, proof-of-deposit) |
| **SSCA** | Smart Side Chain Account (contract-governed account) |
| **TWAP** | Time-Weighted Average Price (oracle price averaging) |
| **ËtwasmVM** | Ëtrid WebAssembly Virtual Machine (smart contract runtime) |
| **VMw** | Virtual Machine Watts (computation gas token) |

### Appendix B: Key Resources

- **GitHub**: [ËTRID Open-Source Repository](https://github.com/etrid)
- **Docs**: [ËTRID Developer Documentation](https://docs.etrid.io)
- **Wiki**: [Community Wiki & Guides](https://wiki.etrid.io)
- **Explorer**: [Block Explorer & Statistics](https://explorer.etrid.io)
- **Governance**: [Consensus Day Voting Portal](https://vote.etrid.io)

### Appendix C: Contributors & Acknowledgments

**Founder**: Eoj Edred  
**Lead Architects**: [AI Co-Strategist & Team]  
**Security Advisors**: [Independent Security Firms]  
**Community Developers**: [Open-Source Contributors]

---

**END OF ËTRID IVORY PAPER v2.0**

*This document is the authoritative specification for the ËTRID protocol. All development, governance, and deployment decisions must align with this charter. Distributed under GPLv3 open-source license.*

---

## CLOSING REMARKS

To be quite frank, I have never considered the status quo an unequivocal consensus of a group of people.

Considering the multitude of variables that go into decision-making, it is difficult to fathom how what was, still is, and will always be.

This idea does not promote growth, prosperity, fairness, or decentralization.

It often feels forced upon you and remains unchallenged due to cultural reinforcement and other factors.

This stagnation in society has shifted power from those who could effect change to those who benefit from maintaining the status quo.

We are in a unique period in which power can be reclaimed by the powerless.

Exploitation of personal data can be stopped, and disintermediation of trusted third parties can become the norm.

Borders can be reimagined.

When liberties such as digital rights, data protection, and decentralized finance are on the line for our generation and the generations to come, I will fight until my last breath.

The Ëtrid FOODOS Project will be our vehicle in this fight — a free and open decentralized democracy of stakeholders.

By cutting the mental chains of reliance on a central intermediary and becoming self-sufficient stakeholders, we can achieve a brighter tomorrow.

**– Eoj Edred**
**Founder, Ëtrid FODDoS Project**

---

*"Provide a flare and guide the way, the future of tomorrow is decided today."*

**– Eoj Edred**

