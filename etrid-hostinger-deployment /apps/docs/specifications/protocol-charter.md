# ËTRID PROTOCOL CHARTER
## Free and Open Decentralized Democracy of Stakeholders (FODDoS)

**Version**: 1.0.0
**Status**: ACTIVE PROTOCOL SPECIFICATION
**Last Updated**: October 30, 2025
**Founder**: Eoj Edred
**License**: GPLv3 (Open Source, Immutable)

---

## I. PROTOCOL PREAMBLE

The **ËTRID Multichain Protocol** is a decentralized, non-hierarchical blockchain system designed to resist centralization, censorship, and unilateral control. This charter establishes the immutable rules, governance structures, and technical specifications that define ËTRID.

### Core Principles
1. **Decentralization**: No single entity controls the network
2. **Democracy**: All major decisions made via Consensus Day voting
3. **Transparency**: All code is GPLv3 open-source
4. **Immutability**: Once enshrined in this charter, rules cannot be broken without hard fork consensus
5. **Interoperability**: DETRP2P ensures coherent cross-chain communication

---

## II. PROTOCOL DEFINITION

### A. ËTRID Multichain Architecture

**Hierarchical Parallel Chain Structure**:

```
┌─────────────────────────────────────────────────────────┐
│        FLARECHAIN (Root/Main Chain)                      │
│  - World State Storage                                   │
│  - Consensus Finality (ASF)                             │
│  - Governance (Consensus Day)                           │
│  - Foundation & DAO Operations                          │
└─────────────────────────────────────────────────────────┘
         │
         ├─→ PBC-EDSC (Ëtrid Dollar Stablecoin Chain)
         │   - EDSC token operations
         │   - Redemption engine (3-path)
         │   - Oracle system
         │   - State checkpoints to FlareChain
         │
         ├─→ PBC-[Future] (Custom Partition Burst Chains)
         │   - Domain-specific sidechains
         │   - Isolated state, fast finality
         │   - Checkpoint-synced to FlareChain
         │
         └─→ Lightning Bloc Network (Micropayment Layer)
             - Off-chain state channels
             - Rapid settlement
             - Batch settlement to FlareChain
```

### B. Essential Elements to Operate (E³20)

The ËTRID Protocol consists of 13 core subsystems:

| # | System | Purpose | Status |
|---|--------|---------|--------|
| 1 | DETR p2p | Multi-protocol P2P networking (S/Kademlia DHT, ECIES encryption) | ✅ ACTIVE |
| 2 | OpenDID | Self-sovereign identity system (decentralized identifiers) | ✅ ACTIVE |
| 3 | Blockchain Security | Post-quantum cryptography, hashing, signatures | ✅ ACTIVE |
| 4 | Accounts | EBCA, RCA, RCWA, SCA, SSCA account types | ✅ ACTIVE |
| 5 | Multichain | FlareChain + PBCs + State channels | ✅ ACTIVE |
| 6 | Native Currency | ÉTR (main), ËDSC (stablecoin), VMw (computation gas) | ✅ ACTIVE |
| 7 | Transactions | Regular, smart contract, cross-chain, stake-deposit | ✅ ACTIVE |
| 8 | ËtwasmVM | WASM-based smart contract runtime (Turing-complete) | ✅ ACTIVE |
| 9 | Consensus | ASF (Ascending Scale of Finality) consensus algorithm | ✅ ACTIVE |
| 10 | Foundation | DAO charter, governance, legal framework | 🔄 FINALIZING |
| 11 | Peer Roles | Flare Nodes, Validity Nodes, Common Peers, DDs | ✅ ACTIVE |
| 12 | Governance | Consensus Day (annual vote), 9-person DD board | ✅ ACTIVE |
| 13 | Clients | CLI, Web, Mobile wallet implementations | ✅ ACTIVE |

---

## III. TOKEN ECONOMICS & NATIVE CURRENCY

### A. ÉTR (Ëtrid Coin) - Primary Token

**Token Specifications**:
- **Symbol**: ÉTR
- **Decimals**: 18
- **Initial Mint**: 1,000,000,000 ÉTR (1 Billion)
- **Max Supply**: Determined by Consensus Day vote (no hard cap in code)
- **Distribution Method**: Annual Consensus Day fiscal mint vote

**Token Uses**:
- Payment for transactions and services
- Staking for validator participation (Flare/Validity Nodes)
- Voting in Consensus Day governance
- Distribution Pay rewards for network participation
- Collateral for ËDSC stablecoin

### B. ËDSC (Ëtrid Dollar Stablecoin)

**Stablecoin Specifications**:
- **Peg**: 1 ËDSC = 1.00 USD (or IMF highest-ranked currency)
- **Total Supply**: 50 Billion ËDSC
- **Initial Circulation**: 5 Billion ËDSC
- **Locked Reserve**: 45 Billion ËDSC (governed release)
- **Collateralization**: 110-130% overcollateralization
- **Reserve Backing**: Mix of on-chain (FlareChain vault) and custodian-held reserves

**ËDSC Redemption Paths**:
1. **Path 1 - Treasury Reserve** (60% of redemptions): Direct withdrawal from FlareChain vault
2. **Path 2 - Custodian Redemption** (30%): BitGo/Anchorage Digital off-chain USD delivery
3. **Path 3 - DEX/AMM** (10%): Secondary market redemption via integrated DEX

**Dynamic Fee Schedule**:
- Base redemption fee: 0.25-10% (varies with reserve ratio)
- Higher fees when reserves < 100% collateral
- Lower fees when reserves > 120% collateral
- Emergency pause if reserves fall below 90% threshold

### C. VMw (Virtual Machine Watts) - Computation Gas

**Gas Token Specifications**:
- **Symbol**: VMw
- **Purpose**: Smart contract execution cost
- **1 VMw**: Computation cost equivalent to 1 watt-hour of compute
- **Conversion**: Market-based: ~0.001 ÉTR = 1 VMw (adjustable via governance)
- **Burn**: All VMw consumed is permanently burned (deflationary)

**Gas Model**:
- Smart contract execution: Pay-per-opcode
- State storage writes: 64 VMw per 32-byte word
- Cross-chain messages: 256 VMw base + payload size
- No minimum gas price (set by validators, market-driven)

---

## IV. GOVERNANCE & CONSENSUS DAY PROTOCOL

### A. Consensus Day Schedule

**Annual Voting Event**:
- **Date**: December 1st (hardcoded, changes require hard fork)
- **Duration**: 24-hour voting window (Dec 1, 00:00-23:59 UTC)
- **Participation**: All Common Stake Peers (≥1 ÉTR staked)
- **Voting Power**: Vote Weight = Stake / Vote Dilution
  - Vote Dilution = Total Network Stake / Coinage (average age of coins)

### B. Consensus Day Ballot & Voting

**Three Categories on Annual Ballot**:

1. **Fiscal Mint & Supply**
   - Vote on annual ÉTR issuance amount
   - Top 3 proposals make final ballot
   - 3 economic limit options (min, mid, max supply)
   - Result becomes binding mint for next 12 months

2. **Decentralized Director Elections**
   - Vote for 9-person governance board (non-hierarchical)
   - Candidates must stake ≥128 ÉTR
   - Candidates must meet "Honest Image" criteria
   - Elected DDs serve 1-year terms
   - Term limits: Maximum 3 consecutive terms

3. **Protocol Amendments**
   - Top 3 proposed changes make ballot
   - Requires ≥66% supermajority to pass
   - Changes take effect Jan 1st following vote
   - Major protocol changes: Additional 90-day audit period

### C. Decentralized Director (DD) Responsibilities

**9-Person Board Duties** (Non-Hierarchical):
- Oversee FlareChain operations and security
- Review and approve major protocol proposals
- Manage Foundation fund allocations
- Coordinate with custodians and validators
- Respond to security incidents
- Represent ËTRID in legal/regulatory contexts

**DD Compensation**:
- Annual salary: X% of annual fiscal mint
- FLARE node operation rewards: Y% of block rewards
- Clawback provisions: Misconduct results in stake forfeiture

**DD Sanctions & Penalties**:
- Absentee voting (>2 missed Consensus Days): -25% compensation
- Misconduct: Full stake clawback + removal
- Security breach: Temporary suspension pending investigation

---

## V. PEER ARCHITECTURE & NODE TYPES

### A. Peer Categories

**1. Common Peers**
- Network users without staked ÉTR
- Can create wallets, receive transfers
- Cannot vote or receive Distribution Pay
- Run light clients via DETRP2P

**2. Common Stake Peers**
- Hold ≥1 ÉTR in active stake
- Full voting rights in Consensus Day
- Eligible for Distribution Pay rewards
- Can run full or light nodes

**3. Flare Nodes** (Finality Layer)
- Maintain FlareChain consensus
- Propagate blocks across network
- Optional stake participation (≥1 ÉTR)
- Rewards: Z% of block production
- No minimum stake requirement to operate

**4. Validity Nodes** (Partition Burst Chains)
- Operate PBC sidechain consensus
- Propose and validate blocks
- **Minimum stake**: 64 ÉTR per node
- Rotated in groups of 8 every 256 blocks (PPFA - Partition Proof of Authority)
- Rewards: W% of sidechain transaction fees

**5. Community Developers**
- Registered open-source contributors
- Eligible for bug bounties (0.1-10 ÉTR per bug)
- Auditor payments: 50-500 ÉTR per audit
- Track record pay scale: Veteran developers get priority allocation

---

## VI. DISTRIBUTION PAY SYSTEM

**Daily Distribution Schedule** (All times PST):

| Recipient | Time | Distribution Formula |
|-----------|------|----------------------|
| Registered Voting Peers | 12:01 AM | P% × ÉTR / Diluted Vote % |
| FLARE Nodes | 4:01 AM | Z% × ÉTR / Blocks / Year - Penalties |
| VALIDITY Nodes | 6:01 AM | W% × ÉTR / PPFA Epoch - Penalties |
| Common Stake Peers | 8:01 AM | Q% × ÉTR × Coinage / Stake |
| Decentralized Directors | 12:01 PM | (Z% FLARE Reward) + (V% DD Compensation) |

**Key Parameters** (Set by Consensus Day):
- P%, Z%, W%, Q%, V% = Annual percentages of fiscal mint
- All percentages total ≤100% of annual mint
- Penalties apply for non-compliance and security failures
- Penalty redistribution goes to compliant participants

---

## VII. EDSC (STABLECOIN) PROTOCOL SPECIFICATION

### A. EDSC on PBC-EDSC Chain

**Primary EDSC Operations**:
- **pallet-edsc-token**: Mint/burn authority for ËDSC
- **pallet-edsc-receipts**: Issuance of Proof-of-Deposit SBTs (Soulbound Tokens)
- **pallet-edsc-redemption**: 3-path redemption engine
- **pallet-edsc-oracle**: TWAP price oracle (24h + 7d fallback)
- **pallet-edsc-checkpoint**: Sync state to FlareChain every N blocks
- **pallet-circuit-breaker**: Emergency pause system

### B. EDSC on FlareChain (Main Chain)

**Reserve & Custody Operations**:
- **pallet-reserve-vault**: On-chain collateral storage (already ✅)
- **pallet-custodian-registry**: Off-chain reserve tracking (already ✅)
- **pallet-reserve-oracle**: Reserve data aggregator (to build)

**Cross-Chain Synchronization**:
- PBC-EDSC checkpoints to FlareChain every 100 blocks
- Checkpoint contains: Total supply, reserve ratio, pending redemptions
- FlareChain validates checkpoint via 66% validator quorum
- Failure to validate triggers circuit breaker

### C. Oracle Price Feeds

**Primary Oracle Sources** (Minimum 5):
1. Binance API
2. Coinbase API
3. Kraken API
4. Bitstamp API
5. Gemini API

**Secondary Oracle Sources** (Future):
- Uniswap/PancakeSwap/Curve TWAP
- Hyperliquid (post-HyperEVM launch)
- Chainlink oracle (if available)

**Outlier Rejection**:
- If any price deviates >2% from median, mark as outlier
- Ignore top/bottom outliers before computing TWAP
- Staleness timeout: 100 blocks (marks oracle stale)

---

## VIII. LEGAL FRAMEWORK & FOUNDATION

### A. ËTRID Foundation Structure

**Entity Type**: Delaware Non-Profit Corporation (or equivalent jurisdiction)

**Mission**: Facilitate ËTRID peer network, enforce GPLv3 open-source license, execute governance decisions

**Governance**:
- Board of 9 Decentralized Directors (elected annually)
- Committees: Technical, Legal, Community, Security
- Annual budget: Funded by Consensus Day allocation

### B. Intellectual Property

**License**: GNU General Public License v3 (GPLv3)
- All code remains open-source in perpetuity
- Derivatives must also be open-source
- Commercial use allowed with proper attribution
- No patents filed; software remains patent-free

**Trademarks**:
- ËTRID™ is trademarked
- ËDSC™ is trademarked
- FODDoS™ is trademarked
- All trademarks managed by Foundation

### C. Community Developer Agreements

**Contributor License Agreement (CLA)**:
- All contributors must sign CLA
- Contributions licensed under GPLv3
- Foundation gets perpetual license
- Contributor retains copyright

**Bug Bounty Program**:
- Low severity: 0.1-1 ÉTR
- Medium severity: 1-5 ÉTR
- High severity: 5-50 ÉTR
- Critical severity: 50-500 ÉTR
- Foundation Treasury funds bounties

---

## IX. SECURITY & NETWORK SAFETY

### A. Post-Quantum Cryptography

**Hash Functions**:
- SHA-3 (Keccak) for general hashing
- Blake2b for performance-critical paths

**Signatures**:
- EdDSA (Ed25519) for most signatures
- SPHINCS+ (lattice-based) for post-quantum resistance
- Hybrid approach during transition period

**Key Derivation**:
- HKDF-Blake2b for key derivation
- BIP39 mnemonic seed phrases (128-bit minimum)
- BIP44 hierarchical deterministic wallets

### B. Circuit Breaker System

**Automated Safety Mechanisms**:
- Reserve ratio drops below 100%: 50% redemption cap
- Reserve ratio drops below 90%: Full pause (emergency mode)
- Pending redemptions exceed 10,000: Throttle new redemptions
- Oracle staleness >100 blocks: Fallback to 7-day TWAP

**Manual Interventions** (DD-authorized only):
- Emergency pause requires 5/9 DD signatures
- Pause can last maximum 7 days
- Must notify community within 1 hour
- Post-pause audit required

### C. Penalty & Slashing System

**Validator Penalties**:
- Missing block proposal: 0.1% of annual stake
- Double-sign attempt: 1% of annual stake
- 3 consecutive slashes: Node removal for 1 year

**Voter Penalties**:
- Absentee voting (no vote cast): 0.05% annual stake
- Invalid vote submission: 0.1% annual stake
- Penalties redistributed to compliant voters

---

## X. TECHNICAL SPECIFICATIONS

### A. Network Parameters

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Block Time | 12 seconds | Balance between finality and throughput |
| Finality Time | ~5 minutes | ASF consensus with ~25 validators |
| Max Block Size | 4 MB | Substrate standard, flexible via governance |
| State Version | Substrate v27+ | Post-quantum ready |
| Network Port | 30333 (default) | Standard Substrate P2P |
| RPC Port | 9933 (default) | JSON-RPC endpoint |
| WS Port | 9944 (default) | WebSocket gateway |

### B. Account Specifications

**Account Types**:

1. **EBCA** (External Blockchain Account)
   - Any non-ËTRID keypair
   - Generated outside ËTRID Key Generation Protocol
   - Can receive but not validate
   - Example: MetaMask wallet, Ethereum address

2. **RCA** (Root Chain Account)
   - Generated by ËTRID Key Generation Protocol
   - Valid on FlareChain only
   - Can vote, stake, validate

3. **SCA** (Side Chain Account)
   - Account on specific PBC
   - Different keypair per PBC allowed
   - Faster local transactions

4. **SSCA** (Smart Side Chain Account)
   - Automated smart contract accounts
   - Controlled by ËtwasmVM logic
   - No private key (code-governed)

**Account Format**:
- SS58 address format (Substrate standard)
- Checksum included for typo detection
- Examples:
  - FlareChain: `1XY...` prefix
  - PBC-EDSC: `3AB...` prefix

### C. Transaction Fees

**Base Fee Structure**:
- Extrinsic encoding: 0.01 ÉTR per kilobyte
- Byte multiplier: 0.001 ÉTR per byte
- Weight multiplier: 0.00001 ÉTR per weight unit
- Priority tip: Optional (for faster inclusion)

**Example Costs**:
- Simple transfer: ~0.1 ÉTR
- Smart contract call: 1-100 ÉTR (depends on complexity)
- Cross-chain message: 0.25-1 ÉTR

**Fee Destination**:
- 80% to active validators
- 20% to Foundation treasury

---

## XI. DEPLOYMENT ROADMAP

### Phase 1: Core Infrastructure (Weeks 1-4) ✅ COMPLETE
- ✅ DETR p2p networking stack
- ✅ OpenDID identity system
- ✅ Core cryptography & security
- ✅ FlareChain reference implementation

### Phase 2: Economics & Governance (Weeks 5-8) ✅ COMPLETE
- ✅ ÉTR token implementation
- ✅ Consensus Day voting system
- ✅ Distribution Pay engine
- ✅ DD election mechanism

### Phase 3: Stablecoins & EDSC (Weeks 9-16) ✅ COMPLETE
- ✅ pallet-edsc-token (core minting)
- ✅ pallet-edsc-receipts (SBT system)
- ✅ pallet-edsc-redemption (3-path engine)
- ✅ pallet-edsc-oracle (TWAP pricing)
- ✅ pallet-edsc-checkpoint (state sync)
- ✅ pallet-circuit-breaker (safety controls)

### Phase 4: Partition Burst Chains (Weeks 17-20) ✅ COMPLETE
- ✅ Build custom PBC infrastructure
- ✅ Implement PBC-specific governance
- ✅ Deploy first community PBC

### Phase 5: Legal & DAO Registration (Weeks 21-24) ✅ COMPLETE
- ✅ Register ËTRID Foundation (Delaware)
- ✅ Finalize legal agreements
- ✅ SEC/regulatory review (if applicable)

### Phase 6: Smart Contracts & ËtwasmVM (Weeks 25-28) ✅ COMPLETE
- ✅ Deploy ËtwasmVM runtime
- ✅ Release smart contract toolkit
- ✅ Community contract deployments

### Phase 7: AI Governance Integration (Weeks 29-32) ✅ COMPLETE
- ✅ pallet-ai-authority (AI node attestation)
- ✅ pallet-attestation-verifier (AI proof verification)
- ✅ pallet-poc-oracle (Proof-of-Computation oracle)

### Phase 8: Mainnet Launch (Week 32+) ✅ COMPLETE
- ✅ Full mainnet deployment
- ✅ Token exchange listings
- ✅ Foundation handoff to community DDs

---

## XII. PROTOCOL AMENDMENTS & HARD FORKS

### A. Amendment Process

**Soft Forks** (Governance-only, no code changes):
- Requires 50% + 1 consensus on Consensus Day ballot
- Takes effect Jan 1st following vote
- Examples: Adjust validator count, change oracle sources

**Hard Forks** (Code changes, consensus required):
- Requires 66% supermajority on Consensus Day ballot
- 90-day security audit period
- Community notice at least 30 days before deployment
- Validators must upgrade or fork off

**Emergency Hard Forks** (Security only):
- DD board (5/9 majority) can authorize emergency hard fork
- Requires 24-hour community vote confirmation
- Only for critical security vulnerabilities
- Post-fork audit mandatory

### B. Fork Management

**Version Numbers**:
- Major.Minor.Patch (e.g., 1.2.3)
- Major version: Hard fork or major protocol change
- Minor version: New features, soft fork
- Patch version: Bug fixes

**Rollback Procedures**:
- Validators can vote to rollback within 1 hour of fork
- Requires 66% consensus on rollback ballot
- State rolled back to pre-fork checkpoint

---

## XIII. CLOSING STATEMENT

The ËTRID Protocol represents a fundamental reimagining of blockchain governance. By combining decentralized democracy (Consensus Day), non-hierarchical leadership (9-person DD board), and technical innovation (DETR p2p, ASF consensus, ëtwasmVM), ËTRID offers a genuine alternative to centralized or semi-centralized blockchain projects.

This charter establishes the immutable rules. Deviation requires consensus. Evolution is democratic. The network remains free and open, in perpetuity.

**"We are in a unique period in which power can be reclaimed by the powerless."**  
— Eoj Edred, ËTRID Founder

---

## XIV. APPENDIX: ABBREVIATIONS & DEFINITIONS

| Abbreviation | Definition |
|--------------|-----------|
| **ASF** | Ascending Scale of Finality (consensus algorithm) |
| **DD** | Decentralized Director (board member) |
| **DETR p2p** | Decentralized, Encrypted, Trustless, Resilient Peer-to-Peer |
| **EBCA** | External Blockchain Account |
| **ËDSC** | Ëtrid Dollar Stablecoin |
| **ÉTR** | Ëtrid Coin (native token) |
| **E³20** | Essential Elements to Operate (Reference Implementation) |
| **FODDoS** | Free and Open Decentralized Democracy of Stakeholders |
| **PPFA** | Partition Proof of Authority (8-validator rotation) |
| **PBC** | Partition Burst Chain (sidechain) |
| **RCA** | Root Chain Account |
| **SBT** | Soulbound Token (non-transferable) |
| **SCA** | Side Chain Account |
| **SSCA** | Smart Side Chain Account |
| **TWAP** | Time-Weighted Average Price |
| **ËtwasmVM** | Ëtrid WebAssembly Virtual Machine |
| **VMw** | Virtual Machine Watts (computation gas) |

---

**END OF ËTRID PROTOCOL CHARTER**

*This document is the source of truth for all ËTRID development, governance, and deployment decisions. Changes require Consensus Day supermajority vote. Distributed under GPLv3 open-source license.*

