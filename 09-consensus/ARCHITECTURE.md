# 09-consensus: ASF Consensus Architecture

**Component:** Adaptive Stake-Weighted Finality (ASF) Consensus System
**Type:** Core Consensus Protocol
**Status:** Production Implementation
**Last Updated:** 2025-10-20

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Diagram](#architecture-diagram)
3. [Core Components](#core-components)
4. [Consensus Algorithm](#consensus-algorithm)
5. [Data Flow](#data-flow)
6. [Security Model](#security-model)
7. [Integration Points](#integration-points)
8. [Performance Characteristics](#performance-characteristics)
9. [Implementation Details](#implementation-details)
10. [Future Enhancements](#future-enhancements)

---

## Overview

The ASF (Adaptive Stake-Weighted Finality) consensus system is Ëtrid's custom Byzantine Fault Tolerant consensus protocol, implementing the FODDoS (Foundation of Decentralized Distributed Operating System) consensus mechanism as specified in the Ivory Papers. Unlike traditional Aura-based consensus, ASF provides:

- **HotStuff 4-Phase Consensus**: Prepare → PreCommit → Commit → Decide
- **PPFA Committee Rotation**: Proposing Panel for Attestation with 21 validators
- **Ascending Scale of Finality**: 5 levels (0-4) of irreversibility
- **Stake-Weighted Voting**: Byzantine fault tolerance based on stake weight
- **Adaptive Slot Timing**: Dynamic block times based on network health
- **Ant Block Support**: Secondary blocks when primary blocks fail

### Key Features

- **Byzantine Fault Tolerance**: Tolerates up to f < n/3 malicious validators
- **Deterministic Finality**: Blocks reach irreversible finality with sufficient certificates
- **Committee-Based**: PPFA panels of 21 validators rotate every epoch
- **Adaptive Performance**: Slot duration adjusts to network conditions
- **Dual Block Types**: Queen (primary) and Ant (secondary) blocks
- **Certificate Aggregation**: Validity certificates accumulate for finality

---

## Architecture Diagram

```text
┌─────────────────────────────────────────────────────────────────────┐
│                         ASF CONSENSUS SYSTEM                        │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │               ASF Algorithm (Core Logic)                     │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐           │  │
│  │  │  HotStuff  │  │   Votes    │  │   Certs    │           │  │
│  │  │  Protocol  │→→│ Collection │→→│ Generation │           │  │
│  │  └────────────┘  └────────────┘  └────────────┘           │  │
│  │        ↓               ↓                ↓                    │  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐           │  │
│  │  │  Finality  │  │   Safety   │  │  Byzantine │           │  │
│  │  │  Tracker   │  │   Checker  │  │  Detection │           │  │
│  │  └────────────┘  └────────────┘  └────────────┘           │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              ↕                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │            Validator Management (Orchestration)              │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐           │  │
│  │  │ Committee  │  │   Health   │  │  Rewards   │           │  │
│  │  │  Manager   │  │  Monitor   │  │  Manager   │           │  │
│  │  └────────────┘  └────────────┘  └────────────┘           │  │
│  │  ┌────────────┐  ┌────────────┐                            │  │
│  │  │ Networking │  │   State    │                            │  │
│  │  │  Manager   │  │   Sync     │                            │  │
│  │  └────────────┘  └────────────┘                            │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              ↕                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │          Block Production (Authoring & Validation)           │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐           │  │
│  │  │  Proposer  │  │   Author   │  │    Ant     │           │  │
│  │  │  Selector  │→→│   Worker   │  │  Handler   │           │  │
│  │  └────────────┘  └────────────┘  └────────────┘           │  │
│  │  ┌────────────┐  ┌────────────┐                            │  │
│  │  │    Slot    │  │ Validation │                            │  │
│  │  │   Timing   │  │   Logic    │                            │  │
│  │  └────────────┘  └────────────┘                            │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              ↕                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │          Runtime Pallet (On-Chain State & Logic)             │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │  • Validator Registry      • Certificate Storage            │  │
│  │  • PPFA Committee State    • Epoch Management               │  │
│  │  • Consensus Phase Tracker • Adaptive Slot Duration         │  │
│  │  • Ant Metadata Storage    • Network Health Score           │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              ↕                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │        Client Implementation (Off-Chain Processing)          │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │  • Import Queue & Verifier  • Block Authoring Worker        │  │
│  │  • Network Propagation      • Signature Verification        │  │
│  │  • Fork Choice              • Certificate Broadcasting      │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘

                                ↕

┌─────────────────────────────────────────────────────────────────────┐
│                    External Integrations                            │
├─────────────────────────────────────────────────────────────────────┤
│  • FlareChain (Root Chain)     • PBC Chains (12 Side Chains)       │
│  • Staking Pallet              • Governance Pallet                  │
│  • Bridge Infrastructure       • Network Layer (P2P)                │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. ASF Algorithm (`asf-algorithm/`)

**Purpose**: Core consensus logic implementing HotStuff 4-phase protocol and Ascending Scale of Finality.

**Key Modules**:

#### `lib.rs` - Core Types & Utilities
- **ConsensusPhase**: Prepare, PreCommit, Commit, Decide
- **FinalityLevel**: None (0-9), Weak (10-19), Moderate (20-49), Strong (50-99), Irreversible (100+)
- **BFT Threshold**: `(2/3 + 1)` calculation for votes and stake
- **Type Aliases**: Hash, BlockNumber, Balance, ValidatorId

#### `hotstuff.rs` - HotStuff State Machine
```rust
pub struct HotStuffState {
    block_hash: Hash,
    block_number: BlockNumber,
    current_phase: ConsensusPhase,
    prepare_votes: VoteCollection,
    precommit_votes: VoteCollection,
    commit_votes: VoteCollection,
    certificates: CertificateCollection,
    finalized: bool,
    epoch: u32,
}
```

**Phase Progression**:
1. **Prepare**: Leader collects highest valid branch and prepare votes
2. **PreCommit**: Prepare certificate broadcast, nodes send pre-commit votes
3. **Commit**: Commit certificate broadcast, replicas lock state
4. **Decide**: Commit certificate finalized, state transition occurs

**Key Functions**:
- `start_consensus()`: Initialize consensus for new block
- `process_vote()`: Handle incoming vote, potentially generate certificate
- `process_certificate()`: Validate and record certificate
- `advance_phase()`: Move to next consensus phase
- `is_finalized()`: Check if block has reached finality

#### `votes.rs` - Vote Management
```rust
pub struct Vote {
    block_hash: Hash,
    block_number: BlockNumber,
    phase: ConsensusPhase,
    validator: ValidatorId,
    stake_weight: Balance,
    epoch: u32,
    timestamp: u64,
    signature: [u8; 64],
}

pub struct VoteCollection {
    votes: Vec<Vote>,
    total_stake: Balance,
}
```

**Vote Validation**:
- Epoch verification (not from future)
- Stake weight > 0
- Signature verification (placeholder for production)
- Duplicate detection per validator

**Threshold Checking**:
- Count threshold: `≥ 2/3 + 1` validators
- Stake threshold: `≥ 2/3 + 1` total stake

#### `certificates.rs` - Certificate Generation
```rust
pub struct ValidityCertificate {
    block_hash: Hash,
    block_number: BlockNumber,
    phase: ConsensusPhase,
    validator: ValidatorId,
    stake_weight: Balance,
    epoch: u32,
    timestamp: u64,
    vote_aggregate: VoteAggregate,
}

pub struct CertificateCollection {
    certificates: Vec<ValidityCertificate>,
    prepare_certs: Vec<ValidityCertificate>,
    precommit_certs: Vec<ValidityCertificate>,
    commit_certs: Vec<ValidityCertificate>,
    decide_certs: Vec<ValidityCertificate>,
}
```

**Certificate Lifecycle**:
1. Votes collected in VoteCollection
2. Threshold met → VoteAggregate created
3. Certificate generated with aggregate proof
4. Certificate validated and stored
5. Finality level calculated from count

#### `finality.rs` - Finality Tracking
```rust
pub struct FinalityTracker {
    blocks: BTreeMap<Hash, BlockFinalityInfo>,
    highest_finalized: BlockNumber,
}

pub struct BlockFinalityInfo {
    block_hash: Hash,
    block_number: BlockNumber,
    certificate_count: u32,
    finality_level: FinalityLevel,
    first_seen: u64,
    finalized_at: Option<u64>,
}
```

**Finality Levels**:
- **None (0)**: 0-9 certificates - Not finalized
- **Weak (1)**: 10-19 certificates - Initial finality
- **Moderate (2)**: 20-49 certificates - Moderate security
- **Strong (3)**: 50-99 certificates - Strong security
- **Irreversible (4)**: 100+ certificates - Absolute finality

#### `safety.rs` - Safety & Byzantine Detection
```rust
pub struct SafetyChecker {
    max_byzantine: u32,           // f = n/3
    total_validators: u32,
    total_stake: Balance,
    finalized_blocks: BTreeSet<Hash>,
    ancestry: BTreeMap<Hash, Hash>,
}

pub struct ByzantineDetector {
    suspicious_validators: BTreeMap<ValidatorId, SuspicionRecord>,
    suspicion_threshold: u32,
}
```

**Safety Guarantees**:
- No two conflicting blocks can both be finalized
- Byzantine threshold: `< n/3` malicious validators
- Fork detection and resolution
- Validator misbehavior tracking

**Byzantine Detection Reasons**:
- ConflictingVotes
- InvalidSignature
- InvalidPhase
- DuplicateVote
- Unavailable
- InvalidCertificate

---

### 2. Validator Management (`validator-management/`)

**Purpose**: Orchestrates validator operations, committee management, and network coordination.

#### `committee.rs` - PPFA Committee Management
```rust
pub struct CommitteeManager {
    members: Vec<CommitteeMember>,
    epoch: u32,
    ppfa_index: u32,
    max_size: u32,
}

pub struct CommitteeMember {
    validator: ValidatorId,
    stake: Balance,
    ppfa_index: u32,
    joined_epoch: u32,
}
```

**Committee Selection**:
- Top 21 validators by stake weight
- Only ValidityNode and FlareNode types eligible
- Reputation threshold: ≥ 50/100
- Rotation every epoch (2400 blocks ≈ 4 hours)

**PPFA Rotation**:
- Proposer index advances each block
- Round-robin within committee
- Automatic failover to next validator on timeout

#### `health.rs` - Network Health Monitoring
```rust
pub struct HealthMonitor {
    network_health: u8,           // 0-100
    missed_blocks: u32,
    average_latency: u64,
    validator_uptime: BTreeMap<ValidatorId, u64>,
}
```

**Health Factors**:
- Block production rate
- Certificate generation rate
- Network latency
- Validator participation
- Fork frequency

**Adaptive Slot Adjustment**:
```
health 90-100: slot_duration = base (6s)
health 70-89:  slot_duration = base × 1.2 (7.2s)
health 50-69:  slot_duration = base × 1.5 (9s)
health 30-49:  slot_duration = base × 2.0 (12s)
health 0-29:   slot_duration = base × 3.0 (18s)
```

#### `rewards.rs` - Validator Rewards & Slashing
```rust
pub struct RewardsManager {
    reward_per_block: Balance,
    total_distributed: Balance,
    slashing_history: Vec<SlashRecord>,
}
```

**Reward Distribution**:
- Block proposer: Primary reward
- Certificate issuers: Secondary reward
- Participation bonus for committee members

**Slashing Conditions**:
- Byzantine behavior detection (3+ incidents)
- Prolonged unavailability
- Invalid certificate issuance
- Double voting

---

### 3. Block Production (`block-production/`)

**Purpose**: Handles block authoring, validation, and Ant block management.

#### `proposer.rs` - Proposer Selection
```rust
pub struct ProposerSelector {
    committee: Vec<ValidatorId>,
    ppfa_index: u32,
    slot: u64,
}
```

**Selection Logic**:
1. Query current PPFA committee from runtime
2. Calculate proposer: `committee[ppfa_index % committee.len()]`
3. Verify proposer eligibility (active, sufficient stake)
4. Return expected proposer for slot

#### `author.rs` - Block Authoring Worker
```rust
pub struct BlockAuthor {
    validator_id: ValidatorId,
    keystore: KeystorePtr,
    transaction_pool: Arc<Pool>,
    parent_hash: Hash,
}
```

**Authoring Flow**:
1. Check if we're current PPFA proposer
2. Wait for slot timing
3. Select transactions from pool (up to 1000 tx or 5 MB)
4. Build block header with proposer info
5. Execute transactions and generate state root
6. Sign block with validator key
7. Propose to network via gossip

#### `slot-timing.rs` - Adaptive Slot Management
```rust
pub struct SlotTimer {
    base_duration: u64,
    current_duration: u64,
    network_health: u8,
    last_block_time: u64,
}
```

**Timing Calculation**:
```rust
fn calculate_slot_duration(base: u64, health: u8) -> u64 {
    let factor = match health {
        90..=100 => 100,
        70..=89 => 120,
        50..=69 => 150,
        30..=49 => 200,
        _ => 300,
    };
    (base * factor) / 100
}
```

#### `ant-handler.rs` - Secondary Block (Ant) Management
```rust
pub struct AntHandler {
    max_ants_per_slot: u32,
    max_ant_depth: u32,
    ant_producers: Vec<ValidatorId>,
}
```

**Ant Block Triggering**:
- Primary proposer timeout (slot duration exceeded)
- Primary block rejected by network
- Network partition recovery

**Ant Block Rules**:
- Maximum 3 Ants per slot
- Maximum depth: 6 levels
- Ants attach to parent Queen or Ant
- Eventually merge into main chain

#### `validation.rs` - Block Validation
```rust
pub struct BlockValidator {
    min_transactions: usize,
    max_block_size: usize,
    require_parent_cert: bool,
}
```

**Validation Checks**:
1. **Header Validation**:
   - Valid parent hash
   - Correct block number (parent + 1)
   - Valid proposer (matches PPFA index)
   - Epoch consistency
   - Timestamp sanity

2. **Body Validation**:
   - Transaction validity (signatures, nonces)
   - State root matches execution
   - Block size ≤ 5 MB
   - Transaction count ≤ 1000

3. **Consensus Validation**:
   - Parent certificate present (if required)
   - Proposer signature valid
   - Slot timing correct
   - No conflicting blocks

---

### 4. Runtime Pallet (`pallet/`)

**Purpose**: On-chain state management and consensus logic execution.

#### Storage Items

```rust
// Validator registry
Validators<T>: StorageMap<AccountId, Validator<T>>

// Current PPFA committee (max 21)
CurrentCommittee<T>: StorageValue<BoundedVec<CommitteeMember<T>, 21>>

// Active validator set (max 100)
ActiveValidators<T>: StorageValue<BoundedVec<AccountId, 100>>

// Current epoch and PPFA index
CurrentEpoch<T>: StorageValue<u32>
PpfaIndex<T>: StorageValue<u32>

// Validity certificates per block (max 100)
Certificates<T>: StorageMap<Hash, BoundedVec<ValidityCertificate<T>, 100>>
CertificateCount<T>: StorageMap<Hash, u32>

// Consensus state
CurrentPhase<T>: StorageValue<ConsensusPhase>

// Adaptive slot duration (milliseconds)
SlotDuration<T>: StorageValue<u64>

// Ant metadata (max 2 per block)
Ants<T>: StorageMap<Hash, BoundedVec<AntMetadata<T>, 2>>

// Network health (0-100)
NetworkHealth<T>: StorageValue<u8>
```

#### Extrinsics

```rust
// Register as validator with stake
register_validator(peer_type: PeerType, stake: Balance)

// Issue validity certificate (committee only)
issue_certificate(block_hash: Hash, block_number: BlockNumber, phase: ConsensusPhase)

// Attach Ant block to parent
attach_ant(ant_hash: Hash, parent_hash: Hash, depth: u32)

// Force committee rotation (governance)
force_rotate_committee()

// Adjust slot duration (governance)
set_slot_duration(duration: u64)
```

#### Hooks

```rust
fn on_initialize(block_number: BlockNumber) {
    // Check for epoch rotation
    if block_number % EPOCH_DURATION == 0 {
        increment_epoch();
        rotate_committee();
    }

    // Advance PPFA index
    advance_ppfa_index();

    // Adaptive slot adjustment (every 100 blocks)
    if block_number % 100 == 0 {
        adjust_slot_duration();
    }
}
```

#### Helper Functions

```rust
// Select validator for block production (VRF-based)
select_validator(block_number) -> Option<AccountId>

// Finalize block (2/3+ votes required)
finalize_block(block_hash, block_number, votes) -> DispatchResult

// Reward validator for participation
reward_validator(who) -> DispatchResult

// Slash validator for misbehavior
slash_validator(who, reason) -> DispatchResult

// Rotate PPFA committee (stake-weighted selection)
rotate_committee()

// Calculate finality level from certificate count
calculate_finality_level(count) -> u8
```

---

### 5. Client Implementation (`client/consensus-asf/`)

**Purpose**: Off-chain consensus processing for Substrate client.

#### `verifier.rs` - Block Verification
```rust
pub struct AsfVerifier<C> {
    client: Arc<C>,
    total_validators: u32,
    total_stake: Balance,
}
```

**Verification Steps**:
1. Check proposer matches PPFA index
2. Verify proposer signature
3. Validate slot timing
4. Check parent certificate (if required)
5. Verify votes meet BFT threshold

#### `import_queue.rs` - Block Import Pipeline
```rust
pub fn import_queue<B, C>(
    client: Arc<C>,
    spawner: &impl SpawnEssentialNamed,
    registry: Option<&Registry>,
) -> Result<BasicQueue<B>>
```

**Import Flow**:
1. Block received from network
2. AsfVerifier validates block
3. Block executed against state
4. State root verified
5. Block imported to database
6. Finality notifications sent

#### `worker.rs` - Block Authoring Worker
```rust
pub async fn run_asf_worker<B, C>(params: AsfWorkerParams<B, C>)
```

**Worker Lifecycle**:
1. Subscribe to finality notifications
2. Monitor slot timing
3. Check if we're current proposer
4. Build and propose block
5. Handle votes and certificates
6. Broadcast to network

---

## Consensus Algorithm

### HotStuff 4-Phase Protocol

```text
┌──────────┐
│  Leader  │  Proposes new block B
│ Proposes │
└────┬─────┘
     │
     ▼
┌──────────────────────────────────────────────────┐
│                   PHASE 1: PREPARE               │
├──────────────────────────────────────────────────┤
│  • Validators receive block proposal            │
│  • Each validator votes if B extends best chain │
│  • Leader collects ≥2/3+1 prepare votes         │
│  • Leader generates Prepare Certificate (QC₁)   │
└────┬─────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────┐
│                PHASE 2: PRE-COMMIT               │
├──────────────────────────────────────────────────┤
│  • Leader broadcasts Prepare QC₁                │
│  • Validators verify QC₁ and send pre-commit    │
│  • Leader collects ≥2/3+1 pre-commit votes      │
│  • Leader generates PreCommit Certificate (QC₂) │
└────┬─────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────┐
│                  PHASE 3: COMMIT                 │
├──────────────────────────────────────────────────┤
│  • Leader broadcasts PreCommit QC₂               │
│  • Validators lock on block B                   │
│  • Validators send commit votes                 │
│  • Leader collects ≥2/3+1 commit votes          │
│  • Leader generates Commit Certificate (QC₃)    │
└────┬─────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────┐
│                  PHASE 4: DECIDE                 │
├──────────────────────────────────────────────────┤
│  • Leader broadcasts Commit QC₃                  │
│  • All validators finalize block B              │
│  • State transition executed                     │
│  • Block reaches initial finality (level 1)     │
└──────────────────────────────────────────────────┘
     │
     ▼
┌──────────────────────────────────────────────────┐
│          ASCENDING SCALE OF FINALITY             │
├──────────────────────────────────────────────────┤
│  • Additional certificates accumulate            │
│  • Finality level increases: 1 → 2 → 3 → 4     │
│  • At 100+ certificates: Irreversible           │
└──────────────────────────────────────────────────┘
```

### PPFA Committee Rotation

```text
Epoch 0 (Blocks 0-2399):
Committee: [V₁, V₂, V₃, ..., V₂₁]
PPFA Index cycles: 0 → 1 → 2 → ... → 20 → 0

Block 0:    Proposer = V₁  (index 0)
Block 1:    Proposer = V₂  (index 1)
Block 2:    Proposer = V₃  (index 2)
...
Block 21:   Proposer = V₁  (index 0, wrapped)

─────────────────────────────────────

Epoch 1 (Blocks 2400-4799):
• Rotate committee (re-select top 21 by stake)
• Reset PPFA index to 0
• Continue round-robin proposal

Block 2400: Proposer = V'₁ (new committee, index 0)
Block 2401: Proposer = V'₂ (index 1)
...
```

### Finality Progression

```text
Block N is proposed and enters consensus:

t=0s:   Block proposed by PPFA validator
        Phase: Prepare
        Certificates: 0
        Finality: None (level 0)

t=2s:   Prepare votes collected (≥2/3+1)
        Phase: PreCommit
        Certificates: 1 (Prepare QC)
        Finality: None (level 0)

t=4s:   PreCommit votes collected (≥2/3+1)
        Phase: Commit
        Certificates: 2 (PreCommit QC)
        Finality: None (level 0)

t=6s:   Commit votes collected (≥2/3+1)
        Phase: Decide
        Certificates: 3 (Commit QC)
        Finality: None (level 0)

t=8s:   Additional validators issue certificates
        Certificates: 10
        Finality: WEAK (level 1) ✓

t=30s:  More certificates accumulated
        Certificates: 25
        Finality: MODERATE (level 2) ✓✓

t=120s: Certificates continue accumulating
        Certificates: 60
        Finality: STRONG (level 3) ✓✓✓

t=600s: Fully propagated across network
        Certificates: 150
        Finality: IRREVERSIBLE (level 4) ✓✓✓✓
        → Block can NEVER be reverted
```

---

## Data Flow

### Block Production Flow

```text
1. PPFA Proposer Check
   ┌──────────────────┐
   │ Runtime Query    │
   │ • Current epoch  │
   │ • PPFA index     │
   │ • Committee list │
   └────────┬─────────┘
            │
            ▼
   ┌──────────────────┐
   │ Am I proposer?   │─── NO ──→ Wait for next slot
   └────────┬─────────┘
            │ YES
            ▼
2. Block Building
   ┌──────────────────┐
   │ Select Txs       │
   │ • Pool query     │
   │ • Priority sort  │
   │ • Size limit     │
   └────────┬─────────┘
            │
            ▼
   ┌──────────────────┐
   │ Execute State    │
   │ • Run txs        │
   │ • Generate root  │
   │ • Build header   │
   └────────┬─────────┘
            │
            ▼
3. Signing & Proposal
   ┌──────────────────┐
   │ Sign Block       │
   │ • Keystore sign  │
   │ • Attach parent  │
   │   certificate    │
   └────────┬─────────┘
            │
            ▼
   ┌──────────────────┐
   │ Broadcast        │
   │ • Gossip network │
   │ • Peer propagate │
   └──────────────────┘
```

### Consensus Voting Flow

```text
1. Block Reception
   ┌──────────────────┐
   │ Receive Block    │
   │ from Network     │
   └────────┬─────────┘
            │
            ▼
2. Validation
   ┌──────────────────┐
   │ Verify Block     │
   │ • Proposer OK?   │
   │ • Parent exists? │
   │ • Signature OK?  │
   │ • State valid?   │
   └────────┬─────────┘
            │
            ▼
3. Vote Casting
   ┌──────────────────┐
   │ Create Vote      │
   │ • Current phase  │
   │ • Stake weight   │
   │ • Sign vote      │
   └────────┬─────────┘
            │
            ▼
4. Vote Broadcasting
   ┌──────────────────┐
   │ Send to Leader   │
   │ and Committee    │
   └────────┬─────────┘
            │
            ▼
5. Certificate Generation (Leader)
   ┌──────────────────┐
   │ Collect Votes    │
   │ • ≥2/3+1 count?  │
   │ • ≥2/3+1 stake?  │
   └────────┬─────────┘
            │ YES
            ▼
   ┌──────────────────┐
   │ Generate Cert    │
   │ • Aggregate votes│
   │ • Create QC      │
   │ • Broadcast      │
   └────────┬─────────┘
            │
            ▼
6. Phase Advancement
   ┌──────────────────┐
   │ Advance Phase    │
   │ Prepare → Pre    │
   │ Pre → Commit     │
   │ Commit → Decide  │
   └──────────────────┘
```

### Certificate Accumulation Flow

```text
Block B enters consensus at epoch E:

┌─────────────────────────────────────────────┐
│ Initial Consensus (Phases 1-4)              │
├─────────────────────────────────────────────┤
│ Committee members (21 validators):          │
│ • Issue certificates during each phase      │
│ • Total: ~4-6 certificates initially        │
│ Finality Level: 0 (Not finalized)          │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Weak Finality (10-19 certificates)          │
├─────────────────────────────────────────────┤
│ • Other validators observe block B          │
│ • Issue attestation certificates            │
│ • Certificates stored on-chain              │
│ Finality Level: 1 (Weak)                   │
│ Security: Can withstand minor forks         │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Moderate Finality (20-49 certificates)      │
├─────────────────────────────────────────────┤
│ • Network-wide propagation continues        │
│ • PBC validators issue certificates         │
│ • FlareChain validators attest              │
│ Finality Level: 2 (Moderate)               │
│ Security: Can withstand major forks         │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Strong Finality (50-99 certificates)        │
├─────────────────────────────────────────────┤
│ • Full network consensus achieved           │
│ • All active validators participated        │
│ • Cross-chain attestations included         │
│ Finality Level: 3 (Strong)                 │
│ Security: Extremely difficult to revert     │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│ Irreversible Finality (100+ certificates)   │
├─────────────────────────────────────────────┤
│ • Historical validators attest              │
│ • Future epochs reference block B           │
│ • Checkpoint in chain history               │
│ Finality Level: 4 (Irreversible)           │
│ Security: IMPOSSIBLE to revert              │
│ Status: PERMANENT RECORD                    │
└─────────────────────────────────────────────┘
```

---

## Security Model

### Byzantine Fault Tolerance

**Assumption**: Network can tolerate up to f < n/3 malicious validators.

For committee of 21 validators:
- Maximum Byzantine: f = 7 (21/3 = 7)
- Required honest: n - f = 14
- BFT threshold: 2/3 + 1 = 15 validators

**Guarantees**:
1. **Safety**: No two conflicting blocks can both reach finality
2. **Liveness**: Progress continues as long as > 2/3 validators are honest and available
3. **Accountability**: Byzantine validators can be identified and slashed

### Attack Resistance

#### 1. Double Voting Attack
**Attack**: Malicious validator votes for two conflicting blocks in same phase.

**Defense**:
- Vote signature verification
- Duplicate vote detection in VoteCollection
- Byzantine detector tracks conflicting votes
- Slashing for confirmed double voting

#### 2. Nothing-at-Stake Attack
**Attack**: Validators vote on multiple forks since there's no cost.

**Defense**:
- Stake slashing for Byzantine behavior
- Locked state in Commit phase
- Economic incentive to vote honestly
- Reputation system penalizes bad actors

#### 3. Long-Range Attack
**Attack**: Attacker creates alternative chain history from genesis.

**Defense**:
- Checkpoint blocks with 100+ certificates are irreversible
- Social consensus on checkpoints
- Client validates against known finalized blocks
- Fork detector identifies suspicious chains

#### 4. Sybil Attack
**Attack**: Attacker controls many validator identities.

**Defense**:
- Stake-weighted voting (not 1-validator-1-vote)
- Minimum stake requirements (64 ËTR for ValidityNode)
- Committee selection based on stake weight
- Maximum committee size (21 validators)

#### 5. Eclipse Attack
**Attack**: Isolate victim node from honest network.

**Defense**:
- Diverse peer connections
- Health monitoring detects isolation
- Certificate count verification
- Cross-chain attestations from PBCs

### Slashing Conditions

```rust
pub enum SlashableOffense {
    DoubleVoting {
        block_a: Hash,
        block_b: Hash,
        phase: ConsensusPhase,
    },
    InvalidCertificate {
        certificate_hash: Hash,
        reason: &'static str,
    },
    Unavailability {
        missed_blocks: u32,
        threshold: u32,
    },
    SignatureForging {
        claimed_signer: ValidatorId,
        actual_signer: ValidatorId,
    },
}
```

**Penalty Amounts**:
- Double voting: 100% stake slashed + permanent ban
- Invalid certificate: 50% stake slashed + 30-day suspension
- Unavailability: 10% stake slashed + reputation decrease
- Signature forging: 100% stake slashed + permanent ban

---

## Integration Points

### 1. FlareChain Integration
```text
┌──────────────┐              ┌──────────────┐
│  FlareChain  │─────────────▶│  ASF Engine  │
│  (Root)      │              │  (Consensus) │
└──────────────┘              └──────────────┘
       │                             │
       │ State Aggregation           │ Finality Proofs
       │                             │
       ▼                             ▼
┌──────────────────────────────────────────┐
│         Cross-Chain Bridges               │
└──────────────────────────────────────────┘
```

**Interaction**:
- FlareChain validators participate in ASF committee
- FlareChain issues finality certificates for PBC blocks
- State roots aggregated from all PBCs
- Cross-chain message routing

### 2. PBC Chain Integration
```text
┌─────────┐  ┌─────────┐  ┌─────────┐
│ BTC-PBC │  │ ETH-PBC │  │ SOL-PBC │  ... (12 chains)
└────┬────┘  └────┬────┘  └────┬────┘
     │            │            │
     └────────────┼────────────┘
                  │
                  ▼
          ┌──────────────┐
          │  ASF Engine  │
          │  (Shared)    │
          └──────────────┘
                  │
                  ▼
          ┌──────────────┐
          │ Finality     │
          │ Aggregation  │
          └──────────────┘
```

**Interaction**:
- Each PBC uses ASF for local consensus
- ValidityNodes validate PBC blocks
- Certificates aggregate across PBCs
- FlareChain collects finality proofs

### 3. Staking Pallet Integration
```rust
// ASF queries staking for validator info
trait StakingInterface {
    fn get_validator_stake(who: &AccountId) -> Option<Balance>;
    fn is_active_validator(who: &AccountId) -> bool;
    fn slash_validator(who: &AccountId, amount: Balance);
    fn reward_validator(who: &AccountId, amount: Balance);
}
```

**Data Flow**:
- ASF queries stake amounts for BFT calculations
- Committee selection based on stake weight
- Rewards distributed via staking pallet
- Slashing executed through staking pallet

### 4. Governance Integration
```rust
// Governance can adjust consensus parameters
trait GovernanceInterface {
    fn set_committee_size(size: u32);
    fn set_epoch_duration(blocks: u32);
    fn force_committee_rotation();
    fn adjust_slot_duration(ms: u64);
    fn emergency_halt_consensus();
}
```

**Governance Powers**:
- Adjust committee size (within 4-21 range)
- Modify epoch duration
- Emergency validator removal
- Network upgrade coordination

---

## Performance Characteristics

### Throughput

**Block Production**:
- Slot duration: 6 seconds (nominal)
- Adaptive range: 6s - 18s (based on health)
- Blocks per hour: 600 (nominal) to 200 (degraded)

**Transaction Capacity**:
- Transactions per block: Up to 1000
- Block size limit: 5 MB
- Nominal TPS: ~166 tx/s (1000 tx / 6s)
- Peak TPS: ~500 tx/s (with optimizations)

**Certificate Generation**:
- Prepare phase: ~2 seconds
- PreCommit phase: ~2 seconds
- Commit phase: ~2 seconds
- Total to finality: ~6 seconds (weak)
- Full irreversibility: ~600 seconds

### Latency

**Block Finality**:
- Weak finality (10 certs): ~8-12 seconds
- Moderate finality (20 certs): ~30-60 seconds
- Strong finality (50 certs): ~2-5 minutes
- Irreversible finality (100+ certs): ~10-15 minutes

**Vote Propagation**:
- Local validator to leader: <100ms
- Leader certificate broadcast: <200ms
- Network-wide propagation: <1 second

**State Synchronization**:
- New validator sync: ~10 minutes (for 1 day of blocks)
- Full node sync: ~2 hours (for 1 month)
- Archive node sync: ~24 hours (full history)

### Resource Requirements

**Validator Node**:
- CPU: 4 cores minimum, 8 cores recommended
- RAM: 16 GB minimum, 32 GB recommended
- Storage: 500 GB SSD (grows ~10 GB/month)
- Network: 100 Mbps down, 50 Mbps up

**Committee Member (PPFA)**:
- CPU: 8 cores minimum
- RAM: 32 GB minimum
- Storage: 1 TB NVMe SSD
- Network: 500 Mbps down, 250 Mbps up

**Full Node**:
- CPU: 2 cores
- RAM: 8 GB
- Storage: 250 GB SSD
- Network: 50 Mbps down, 25 Mbps up

### Scalability

**Horizontal Scaling**:
- Validators: Up to 100 active (21 in committee)
- PBC chains: 12 parallel chains
- Total network TPS: ~2000 tx/s (12 chains × 166 tx/s)

**Vertical Scaling**:
- Block size can increase to 10 MB (requires consensus)
- Transaction count can increase to 2000 per block
- Potential TPS: ~1000 tx/s per chain

**Future Optimizations**:
- Parallel transaction execution
- Sharded state storage
- Optimistic finality hints
- Certificate batching

---

## Implementation Details

### Key Algorithms

#### 1. BFT Threshold Calculation
```rust
fn bft_threshold(total: u32) -> u32 {
    ((total * 2) / 3) + 1
}

// Examples:
// 21 validators → 15 required (71.4%)
// 100 validators → 67 required (67%)
// 3 validators → 3 required (100%)
```

#### 2. Committee Selection
```rust
fn select_committee(
    validators: Vec<ValidatorInfo>,
    committee_size: u32,
) -> Vec<CommitteeMember> {
    // Filter eligible validators
    let eligible: Vec<_> = validators
        .into_iter()
        .filter(|v| {
            v.active &&
            v.peer_type.can_be_in_committee() &&
            v.reputation >= MIN_REPUTATION_FOR_COMMITTEE &&
            v.stake >= v.peer_type.min_stake()
        })
        .collect();

    // Sort by stake (descending)
    eligible.sort_by(|a, b| b.stake.cmp(&a.stake));

    // Take top N
    eligible
        .into_iter()
        .take(committee_size as usize)
        .enumerate()
        .map(|(idx, v)| CommitteeMember {
            validator: v.id,
            stake: v.stake,
            ppfa_index: idx as u32,
            joined_epoch: current_epoch,
        })
        .collect()
}
```

#### 3. Finality Level Calculation
```rust
fn calculate_finality_level(cert_count: u32) -> FinalityLevel {
    match cert_count {
        0..=9 => FinalityLevel::None,
        10..=19 => FinalityLevel::Weak,
        20..=49 => FinalityLevel::Moderate,
        50..=99 => FinalityLevel::Strong,
        _ => FinalityLevel::Irreversible,
    }
}
```

#### 4. Adaptive Slot Duration
```rust
fn calculate_adaptive_slot(
    base_duration: u64,
    health: u8,
) -> u64 {
    let factor = match health {
        90..=100 => 100,  // 1.0x (optimal)
        70..=89 => 120,   // 1.2x (normal)
        50..=69 => 150,   // 1.5x (degraded)
        30..=49 => 200,   // 2.0x (poor)
        _ => 300,         // 3.0x (critical)
    };

    (base_duration * factor) / 100
}
```

### Critical Code Paths

#### Vote Processing
```rust
// Hot path - executed for every vote
pub fn process_vote(&mut self, vote: Vote) -> AsfResult<Option<Certificate>> {
    // 1. Validate vote (signature, epoch, stake)
    vote.validate(self.current_epoch)?;

    // 2. Get block state
    let state = self.block_states
        .get_mut(&vote.block_hash)
        .ok_or(AsfError::BlockNotFound)?;

    // 3. Verify phase match
    if vote.phase != state.current_phase {
        return Err(AsfError::InvalidPhaseTransition {
            from: state.current_phase,
            to: vote.phase,
        });
    }

    // 4. Add vote (checks for duplicates)
    state.current_votes().add_vote(vote)?;

    // 5. Check thresholds
    let votes = state.current_votes();
    if votes.meets_threshold(self.total_validators) &&
       votes.meets_stake_threshold(self.total_stake) {
        // Generate certificate
        let cert = self.cert_generator.try_generate(
            votes.votes(),
            vote.validator,
            vote.stake_weight,
            vote.timestamp,
        )?;

        // Add to collection and advance phase
        state.certificates.add_certificate(cert.clone())?;
        state.advance_phase()?;

        return Ok(Some(cert));
    }

    Ok(None)
}
```

### Testing Strategy

**Unit Tests**:
- Core algorithm tests (vote collection, certificate generation)
- Threshold calculations
- Phase transitions
- Finality level progression

**Integration Tests**:
- Multi-validator consensus simulation
- Byzantine behavior scenarios
- Network partition recovery
- Committee rotation

**Performance Tests**:
- Vote throughput benchmarks
- Certificate generation latency
- Memory usage under load
- Database query optimization

**Security Tests**:
- Double voting detection
- Invalid signature rejection
- Stake verification
- Byzantine threshold enforcement

---

## Future Enhancements

### Planned Improvements

1. **Optimistic Finality**
   - Fast finality hints for low-value transactions
   - Probabilistic confirmation before full finality
   - Risk-based finality levels

2. **Certificate Batching**
   - Aggregate multiple certificates into compact proofs
   - Reduce storage and bandwidth requirements
   - Merkle tree-based certificate compression

3. **Parallel Vote Processing**
   - Multi-threaded vote validation
   - Concurrent certificate generation
   - Lock-free data structures for hot paths

4. **Dynamic Committee Sizing**
   - Adjust committee size based on network conditions
   - Larger committees during high stake participation
   - Smaller committees during low activity periods

5. **Cross-Chain Finality Proofs**
   - Light client proofs for PBC finality
   - Compact certificates for bridge transfers
   - Merkle proof optimization

6. **Advanced Byzantine Detection**
   - Machine learning-based anomaly detection
   - Predictive validator behavior analysis
   - Automatic reputation adjustment

### Research Areas

1. **Zero-Knowledge Proofs**
   - zk-SNARKs for compact finality proofs
   - Private validator voting
   - Confidential transaction finality

2. **Quantum Resistance**
   - Post-quantum signature schemes
   - Quantum-safe certificate aggregation
   - Future-proof cryptographic primitives

3. **Cross-Shard Consensus**
   - Sharded state with cross-shard finality
   - Atomic cross-shard transactions
   - Shard committee coordination

4. **AI-Driven Optimization**
   - Adaptive slot duration via neural networks
   - Predictive validator selection
   - Automated parameter tuning

---

## Conclusion

The ASF consensus system represents a sophisticated Byzantine Fault Tolerant protocol specifically designed for Ëtrid's multi-chain architecture. By combining HotStuff's provably safe 4-phase commit protocol with an innovative Ascending Scale of Finality, ASF provides:

- **Deterministic Finality**: Blocks reach irreversible finality through certificate accumulation
- **Stake-Weighted Security**: Byzantine tolerance based on economic stake
- **Adaptive Performance**: Network health-based slot timing
- **Committee Governance**: PPFA rotation ensures decentralization
- **Multi-Chain Support**: Shared consensus across 12 PBC chains

The modular architecture separates core algorithm logic, validator orchestration, block production, runtime state, and client implementation, enabling independent testing, optimization, and evolution of each component.

---

**References:**
- Ivory Papers: FODDoS Consensus Specification
- HotStuff Paper: "HotStuff: BFT Consensus in the Lens of Blockchain" (2018)
- PBFT Paper: "Practical Byzantine Fault Tolerance" (1999)
- Tendermint Documentation: BFT Consensus Patterns
- Substrate Documentation: Consensus Framework

**Related Components:**
- `10-foundation`: Governance integration
- `11-peer-roles`: Validator staking and roles
- `05-multichain/partition-burst-chains`: PBC chain implementations
- `03-flarechain`: Root chain integration
