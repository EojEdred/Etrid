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

**FlareChain** (Main Chain / Root)
- Stores world state
- Executes Consensus Day voting
- Manages Foundation operations
- Holds reserve collateral (for ËDSC)
- Block time: 12 seconds
- Finality: ~5 minutes via ASF consensus

**Partition Burst Chains (PBCs)** (Sidechains)
- Dedicated chains for specific purposes or communities
- Example: PBC-EDSC for ËDSC stablecoin operations
- State synchronized to FlareChain via checkpoints every N blocks
- Faster throughput than main chain
- Security guaranteed by FlareChain validator quorum

**Lightning Bloc Network** (State Channels)
- Off-chain micropayment channels
- Rapid settlement (instant)
- Batch settlement to FlareChain daily/weekly
- Reduces main chain load
- Enables microtransactions (<0.01 ÉTR)

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

**Flare Nodes** (Finality Validators)
- Maintain FlareChain blockchain
- Propagate blocks across network
- Optional stake (≥1 ÉTR) for rewards
- Penalties for missing blocks or double-signing
- Rewards: Z% annual mint / active validator count

**Validity Nodes** (Sidechain Validators)
- Operate Partition Burst Chain consensus
- Propose blocks on assigned PBC
- **Minimum stake**: 64 ÉTR (requirement)
- Rotation: 8 validators per PBC, rotate every 256 blocks
- Rewards: W% annual mint / active PBC validator count

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

