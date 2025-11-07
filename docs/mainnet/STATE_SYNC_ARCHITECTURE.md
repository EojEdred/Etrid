# Ëtrid FlareChain - State Synchronization Architecture

**Date:** 2025-11-04
**Status:** 📋 Technical Documentation
**Purpose:** Explain how FlareChain Directors receive state updates from PBCs and Lightning Bloc

---

## Overview

Ëtrid's multi-layer architecture consists of three tiers:

```
┌─────────────────────────────────────────────────────────────────┐
│  Layer 3: Lightning Bloc (Payment Channels)                     │
│  - Off-chain transactions: 100,000+ TPS                         │
│  - Settlement batching every 5 minutes                          │
│  - Emergency withdrawal mechanisms                              │
└────────────────────┬────────────────────────────────────────────┘
                     │ Batch Settlement
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│  Layer 2: Partition Burst Chains (PBCs)                         │
│  - High-throughput sidechains: ~5,000 TPS each                  │
│  - Specialized use cases (ËDSC, BTC bridge, ETH bridge...)      │
│  - Checkpoints to FlareChain every N blocks                     │
└────────────────────┬────────────────────────────────────────────┘
                     │ State Checkpoints
                     ↓
┌─────────────────────────────────────────────────────────────────┐
│  Layer 1: FlareChain (Main Chain)                               │
│  - Relay chain coordination: ~1,000 TPS                         │
│  - Decentralized Directors (Flare Nodes)                        │
│  - Checkpoint verification and finality                         │
└─────────────────────────────────────────────────────────────────┘
```

**Key Question:** How do FlareChain Directors (Layer 1) get state updates from PBCs (Layer 2) and Lightning Bloc (Layer 3)?

---

## Part 1: PBC → FlareChain State Synchronization

### 1.1 Checkpoint Mechanism

PBCs use a **checkpoint pallet** to submit state commitments to FlareChain at regular intervals.

#### Checkpoint Data Structure

**Location:** `05-multichain/bridge-protocols/edsc-bridge/substrate-pallets/pallet-edsc-checkpoint/src/lib.rs`

```rust
/// Checkpoint data structure
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Checkpoint<BlockNumber, Hash> {
    /// Block number when checkpoint was created
    pub block_number: BlockNumber,

    /// Merkle root of PBC state
    pub state_root: Hash,

    /// Total EDSC supply at checkpoint (for EDSC-PBC)
    pub total_supply: u128,

    /// Reserve ratio snapshot (in basis points)
    pub reserve_ratio: u16,

    /// Timestamp of checkpoint creation
    pub timestamp: u64,
}
```

**Key Fields:**
- `state_root`: Merkle root of entire PBC state (all accounts, balances, contracts)
- `block_number`: Which PBC block this checkpoint represents
- `total_supply`: Economic snapshot (for stablecoin PBCs)
- `reserve_ratio`: Collateralization ratio (for stability monitoring)

---

### 1.2 Checkpoint Creation Process

#### Automatic Checkpoint Submission

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_finalize(n: BlockNumberFor<T>) {
        // Check if we need to create a checkpoint
        if Self::should_create_checkpoint(n) {
            let _ = Self::create_checkpoint(n);
        }
    }
}
```

**Checkpoint Trigger Conditions:**

1. **Regular Intervals:**
   ```rust
   fn should_create_checkpoint(block_number: BlockNumber) -> bool {
       block_number % CheckpointInterval::get() == 0
   }
   ```
   - Configurable via `CheckpointInterval` parameter
   - Example: Every 256 blocks (~51 minutes at 12s/block)

2. **Emergency Checkpoints:**
   ```rust
   // Triggered when reserve ratio drops below threshold
   if reserve_ratio < EmergencyThreshold::get() {
       Self::create_emergency_checkpoint();
   }
   ```
   - Critical for ËDSC stablecoin PBC
   - Immediate checkpoint if reserve ratio < 125%

---

### 1.3 Data Flow: PBC to FlareChain

```
┌──────────────────────────────────────────────────────────────┐
│                        PBC-EDSC                               │
│  (Validators 6-13 running PBC collator)                      │
└───────────────────┬──────────────────────────────────────────┘
                    │
                    │ 1. PBC Block Finalized (Block #256)
                    ↓
       ┌────────────────────────────┐
       │  Checkpoint Pallet         │
       │  on_finalize() hook        │
       └────────────┬───────────────┘
                    │
                    │ 2. Calculate State Root
                    │    - Merkle tree of all accounts
                    │    - Include balances, nonces, storage
                    ↓
       ┌────────────────────────────┐
       │  Create Checkpoint         │
       │  {                         │
       │    block: 256,             │
       │    state_root: 0xabc...,   │
       │    supply: 1M EDSC,        │
       │    ratio: 150%             │
       │  }                         │
       └────────────┬───────────────┘
                    │
                    │ 3. Submit to FlareChain
                    │    (Extrinsic from PBC collator)
                    ↓
┌──────────────────────────────────────────────────────────────┐
│                       FlareChain                              │
│  (Directors 1-5 running Flare Nodes)                         │
│                                                               │
│  ┌────────────────────────────────────┐                      │
│  │  Checkpoint Registry Pallet        │                      │
│  │  - Receive checkpoint              │                      │
│  │  - Verify signature                │                      │
│  │  - Store in ChainStorage           │                      │
│  └────────────┬───────────────────────┘                      │
│               │                                               │
│               │ 4. Verify Checkpoint                          │
│               ↓                                               │
│  ┌────────────────────────────────────┐                      │
│  │  GRANDPA Finality                  │                      │
│  │  - Include checkpoint in           │                      │
│  │    finalized FlareChain block      │                      │
│  │  - Checkpoint now immutable        │                      │
│  └────────────────────────────────────┘                      │
│                                                               │
│  Result: PBC state is now anchored    │                      │
│          on FlareChain at block #1234 │                      │
└──────────────────────────────────────────────────────────────┘
```

---

### 1.4 Checkpoint Verification by Directors

**Step 1: Collator Submits Checkpoint**

```rust
// PBC collator submits checkpoint via extrinsic
let checkpoint = Checkpoint {
    block_number: 256,
    state_root: calculate_merkle_root(), // 0xabc123...
    total_supply: 1_000_000_000_000_000_000_000_000, // 1M EDSC
    reserve_ratio: 15000, // 150% (in basis points)
    timestamp: current_timestamp(),
};

// Submit to FlareChain
submit_checkpoint_extrinsic(checkpoint);
```

**Step 2: FlareChain Directors Verify**

```rust
// In FlareChain runtime
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn submit_checkpoint(
        origin: OriginFor<T>,
        para_id: ParaId,
        checkpoint: Checkpoint<T::BlockNumber, T::Hash>,
    ) -> DispatchResult {
        // 1. Ensure caller is registered PBC collator
        let collator = ensure_signed(origin)?;
        ensure!(
            Self::is_registered_collator(para_id, &collator),
            Error::<T>::UnauthorizedCollator
        );

        // 2. Verify checkpoint signature
        ensure!(
            Self::verify_checkpoint_signature(&checkpoint, &collator),
            Error::<T>::InvalidSignature
        );

        // 3. Store checkpoint
        Checkpoints::<T>::insert(
            para_id,
            checkpoint.block_number,
            checkpoint.clone()
        );

        // 4. Emit event
        Self::deposit_event(Event::CheckpointSubmitted {
            para_id,
            block_number: checkpoint.block_number,
            state_root: checkpoint.state_root,
        });

        Ok(())
    }
}
```

**Step 3: Finality via GRANDPA**

Once the FlareChain block containing the checkpoint is finalized by GRANDPA consensus:
- Checkpoint becomes **immutable**
- PBC state is **anchored** to FlareChain
- Can be used for dispute resolution

---

### 1.5 Benefits of Checkpoint System

**1. Compact State Commitments**
- Only Merkle root transmitted (32 bytes)
- Full state doesn't need to cross chains
- Efficient for high-throughput PBCs

**2. Fraud Proof Capability**
```
If someone disputes PBC state:
1. Challenger provides Merkle proof showing state mismatch
2. Directors verify proof against stored checkpoint
3. If invalid state detected → slash malicious collator
```

**3. Fast Finality**
- PBC blocks finalize optimistically (seconds)
- Checkpoints provide economic finality (minutes)
- FlareChain GRANDPA provides absolute finality (2 blocks)

**4. Economic Security**
- Checkpoints anchor PBC value to FlareChain
- Directors can freeze PBC if checkpoints stop
- Emergency withdrawals use last known checkpoint

---

### 1.6 Checkpoint Parameters by PBC

| PBC Chain | Checkpoint Interval | Emergency Threshold | Data Included |
|-----------|---------------------|---------------------|---------------|
| **EDSC-PBC** | 256 blocks (~51 min) | Reserve ratio < 125% | state_root, total_supply, reserve_ratio |
| **BTC-PBC** | 512 blocks (~102 min) | Bridge balance mismatch | state_root, btc_locked, withdrawals_pending |
| **ETH-PBC** | 512 blocks (~102 min) | Bridge balance mismatch | state_root, eth_locked, contracts_deployed |
| **SOL-PBC** | 256 blocks (~51 min) | Validator offline | state_root, sol_locked, last_sync_timestamp |

---

## Part 2: Lightning Bloc → FlareChain State Synchronization

### 2.1 Lightning Bloc Architecture

Lightning Bloc is a **Layer 3** payment channel network operating **on top of** PBCs and FlareChain.

```
Lightning Bloc Layer (Off-Chain)
├── Payment Channels (bilateral state)
├── Multi-hop Routing (Alice → Bob → Charlie)
├── Transaction Batching (1,000 tx per batch)
└── Settlement Batching (every 5 minutes)
         ↓
    Submit to PBC or FlareChain
         ↓
    On-Chain Settlement
```

---

### 2.2 Settlement Batching Mechanism

**Location:** `07-transactions/lightning-bloc/src/batching.rs`

#### Step 1: Off-Chain Transaction Accumulation

```rust
pub struct OffChainTransaction {
    pub tx_id: String,
    pub channel_id: String,
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub nonce: u64,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}
```

Transactions happen **off-chain** between payment channel participants:
- Alice → Bob: 100 ÉTR
- Bob → Charlie: 50 ÉTR
- Charlie → Dave: 25 ÉTR

**Benefits:**
- Instant (no block confirmation)
- Zero fees (no gas)
- High throughput (100,000+ TPS)

---

#### Step 2: Transaction Batching

```rust
pub struct TransactionBatch {
    pub batch_id: String,
    pub transactions: Vec<OffChainTransaction>,
    pub created_at: u64,
    pub compressed_data: Option<Vec<u8>>,
    pub merkle_root: Vec<u8>,
}

// Batching parameters
pub const MAX_BATCH_SIZE: usize = 1000;        // 1,000 tx per batch
pub const MAX_BATCH_AGE: u64 = 300;            // 5 minutes
```

**Batch Submission Triggers:**

1. **Size trigger:** Batch reaches 1,000 transactions
2. **Time trigger:** Batch age exceeds 5 minutes

```rust
pub fn should_submit(&self, current_time: u64) -> bool {
    let age = current_time.saturating_sub(self.created_at);
    age >= MAX_BATCH_AGE || self.transactions.len() >= MAX_BATCH_SIZE
}
```

---

#### Step 3: Compression and Merkle Root

Before submitting to on-chain, batches are compressed:

```rust
pub struct CompressionResult {
    pub original_size: usize,      // 150 KB (1,000 tx × 150 bytes)
    pub compressed_size: usize,    // 105 KB (30% compression)
    pub ratio: u8,                 // 30% reduction
    pub merkle_root: Vec<u8>,      // 32 bytes for verification
}
```

**Compression Benefits:**
- Original: 150 KB per batch (1,000 tx × 150 bytes)
- Compressed: ~105 KB (30% reduction)
- On-chain footprint: Only Merkle root (32 bytes) + compressed data

---

#### Step 4: Settlement to On-Chain

```rust
pub struct BatchSettlement {
    pub settlement_id: String,
    pub batch_id: String,
    pub transaction_count: usize,    // 1,000 transactions
    pub merkle_root: Vec<u8>,        // State commitment
    pub settled_at: u64,
    pub on_chain_tx_hash: Vec<u8>,  // FlareChain tx hash
}
```

**Settlement Flow:**

```
Lightning Bloc Batch Manager
         │
         │ 1. Batch ready (1,000 tx or 5 min timeout)
         ↓
    Compress Batch
         │
         │ 2. Calculate Merkle root
         ↓
Submit to FlareChain/PBC
         │
         │ 3. Extrinsic: settle_lightning_batch()
         ↓
┌────────────────────────────────────┐
│  FlareChain/PBC Runtime            │
│  - Verify Merkle root              │
│  - Update channel states           │
│  - Emit ChannelSettled event      │
└────────────────────────────────────┘
```

---

### 2.3 Lightning Bloc Data Flow to FlareChain

**Complete Architecture:**

```
┌───────────────────────────────────────────────────────────────┐
│  LAYER 3: Lightning Bloc (Off-Chain Payment Channels)         │
│                                                                │
│  Alice ←→ Bob Channel                                         │
│  ├─ Open: 10,000 ÉTR each                                     │
│  ├─ Transaction 1: Alice → Bob (100 ÉTR)                      │
│  ├─ Transaction 2: Bob → Alice (50 ÉTR)                       │
│  ├─ Transaction 3: Alice → Bob (200 ÉTR)                      │
│  └─ ... (997 more transactions)                               │
│                                                                │
│  After 5 minutes or 1,000 transactions:                       │
│  → Create Batch                                               │
│  → Compress (150 KB → 105 KB)                                 │
│  → Calculate Merkle Root: 0xdef456...                         │
└──────────────────┬────────────────────────────────────────────┘
                   │
                   │ Submit Settlement Batch
                   │ (On-chain extrinsic)
                   ↓
┌───────────────────────────────────────────────────────────────┐
│  LAYER 2: PBC-EDSC (or FlareChain directly)                   │
│                                                                │
│  Lightning Bloc Pallet                                        │
│  ├─ Receive batch settlement                                  │
│  ├─ Verify Merkle root                                        │
│  ├─ Update channel states:                                    │
│  │   ├─ Alice: 10,000 - 250 = 9,750 ÉTR                       │
│  │   └─ Bob: 10,000 + 250 = 10,250 ÉTR                        │
│  └─ Emit Event: BatchSettled                                  │
│                                                                │
│  Every 256 blocks:                                            │
│  → Create PBC Checkpoint (includes Lightning state)           │
└──────────────────┬────────────────────────────────────────────┘
                   │
                   │ Submit Checkpoint
                   │ (State root includes Lightning balances)
                   ↓
┌───────────────────────────────────────────────────────────────┐
│  LAYER 1: FlareChain (Main Chain)                             │
│                                                                │
│  Checkpoint Registry                                          │
│  ├─ Receive PBC-EDSC checkpoint                               │
│  ├─ Verify collator signature                                 │
│  ├─ Store state root: 0xabc123...                             │
│  │   (includes Lightning Bloc channel balances)               │
│  └─ Finalize via GRANDPA                                      │
│                                                                │
│  Directors now know:                                          │
│  ✓ PBC-EDSC state at block 256                                │
│  ✓ All Lightning channel balances (via state root)            │
│  ✓ Can resolve disputes using Merkle proofs                   │
└───────────────────────────────────────────────────────────────┘
```

---

### 2.4 Optimistic Rollup for Lightning Bloc

**Location:** `07-transactions/lightning-bloc/src/optimistic_rollup.rs`

Lightning Bloc uses **optimistic assumptions** with fraud proofs:

```rust
pub struct StateCommitment {
    pub commitment_id: String,
    pub state_root: Vec<u8>,          // Lightning network state
    pub block_number: u64,
    pub transaction_count: u64,        // Transactions in this commitment
    pub timestamp: u64,
    pub challenge_deadline: u64,       // 7 days challenge period
    pub status: CommitmentStatus,
}
```

**Optimistic Assumption:**
> "We assume Lightning batch settlements are valid unless someone proves otherwise"

**Challenge Period:** 7 days

```rust
pub const OPTIMISTIC_CHALLENGE_PERIOD: u64 = 604_800; // 7 days in seconds
```

**How It Works:**

1. **Sequencer submits state commitment**
   ```rust
   let commitment = StateCommitment {
       state_root: batch_merkle_root,
       block_number: current_l2_block,
       transaction_count: 1000,
       challenge_deadline: now + 7_days,
       status: Pending,
   };
   ```

2. **Challenge period begins** (7 days)
   - Anyone can challenge with fraud proof
   - Must provide Merkle proof of invalid state transition

3. **If no challenges:**
   ```rust
   if now > commitment.challenge_deadline {
       commitment.status = Finalized;
       // State is now considered final
   }
   ```

4. **If challenged:**
   ```rust
   // Challenger provides fraud proof
   let fraud_proof = FraudProof {
       invalid_transition: StateTransition { ... },
       merkle_proof: vec![...],
       signature: vec![...],
   };

   // System resolves dispute
   if verify_fraud_proof(&fraud_proof) {
       // Slash malicious sequencer
       // Revert state commitment
   }
   ```

---

### 2.5 Emergency Withdrawal Mechanism

**Location:** `07-transactions/lightning-bloc/src/emergency.rs`

If a counterparty becomes unresponsive, users can withdraw funds using the last known checkpoint:

```rust
pub struct WithdrawalRequest {
    pub request_id: String,
    pub channel_id: String,
    pub requester: String,
    pub amount: u128,
    pub request_time: u64,
    pub timeout_deadline: u64,       // 24 hours
    pub status: WithdrawalStatus,
}
```

**Emergency Withdrawal Flow:**

```
User (Alice) in channel with Bob
Bob becomes unresponsive ❌

Alice: Submit emergency withdrawal request
       ↓
24-hour timeout period
       ↓
    [Bob responds?]
    /              \
  Yes              No
   ↓               ↓
Approve/        Force Execute
Reject          (After 24h)
   ↓               ↓
Settlement    Settlement
              using last
              checkpoint
```

**Implementation:**

```rust
// Alice requests withdrawal
emergency_system.request_withdrawal(
    "channel_alice_bob".to_string(),
    "alice".to_string(),
    5_000, // Withdraw 5,000 ÉTR
    current_time,
)?;

// Wait 24 hours...
// If Bob doesn't respond, force execute:

let force_time = current_time + 86400; // +24 hours
emergency_system.execute_expired_withdrawals(force_time);

// Result: Alice gets her 5,000 ÉTR from last known state
```

---

## Part 3: Complete State Flow Summary

### 3.1 Multi-Layer State Propagation

```
┌─────────────────────────────────────────────────────────────┐
│  LAYER 3: Lightning Bloc                                    │
│  Frequency: Continuous (100,000+ TPS off-chain)             │
│  State: Off-chain payment channel balances                  │
│                                                              │
│  Alice ↔ Bob: 1,000 transactions/second                     │
│  Off-chain state updates in milliseconds                    │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Every 5 minutes or 1,000 transactions
                     │ Settlement Batch Submission
                     ↓
┌─────────────────────────────────────────────────────────────┐
│  LAYER 2: PBCs (EDSC, BTC, ETH, etc.)                       │
│  Frequency: ~5,000 TPS per PBC                              │
│  State: PBC account balances, contracts, Lightning channels │
│                                                              │
│  ├─ EDSC-PBC: ËDSC minting, redemption, oracle pricing      │
│  ├─ BTC-PBC: Bitcoin deposits, withdrawals, bridge state    │
│  └─ ETH-PBC: Ethereum deposits, withdrawals, bridge state   │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Every 256 blocks (~51 minutes)
                     │ Checkpoint Submission
                     ↓
┌─────────────────────────────────────────────────────────────┐
│  LAYER 1: FlareChain                                        │
│  Frequency: ~1,000 TPS                                       │
│  State: Checkpoint registry, governance, validator set      │
│                                                              │
│  Directors (Flare Nodes 1-5) receive:                       │
│  ├─ PBC checkpoints every 256 blocks                        │
│  │   ├─ State root (Merkle root of all PBC state)          │
│  │   ├─ Economic data (supply, reserves, balances)          │
│  │   └─ Timestamp and block number                          │
│  │                                                           │
│  ├─ Lightning batch settlements (indirect via PBC state)    │
│  │   ├─ Included in PBC state root                          │
│  │   └─ Can be queried via Merkle proofs                    │
│  │                                                           │
│  └─ Finalize via GRANDPA consensus (2 blocks = ~24 seconds) │
│                                                              │
│  Result: Full network state anchored on FlareChain          │
└─────────────────────────────────────────────────────────────┘
```

---

### 3.2 State Update Frequency

| Layer | Update Frequency | Latency to FlareChain | Security Model |
|-------|------------------|----------------------|----------------|
| **Lightning Bloc** | Continuous (off-chain) | 5 min → PBC → 51 min → FlareChain | Optimistic (7-day challenge) |
| **PBCs** | ~2s per block | 256 blocks = ~51 minutes | Checkpoints + economic security |
| **FlareChain** | ~12s per block | Immediate (Layer 1) | GRANDPA finality (2 blocks) |

**Total Latency: Lightning → FlareChain Directors:**
- Lightning batch created: **0 seconds**
- Settlement to PBC: **5 minutes** (batch timeout)
- PBC checkpoint to FlareChain: **51 minutes** (256 blocks)
- **Total: ~56 minutes** for Lightning state to reach FlareChain
- **Emergency withdrawals:** Use last checkpoint (max 51 min old)

---

### 3.3 Example: Complete Transaction Journey

**Scenario:** Alice sends 100 ÉTR to Bob via Lightning Bloc

**Step 1: Off-Chain Transaction (Instant)**
```
Alice ↔ Bob Lightning Channel
├─ Initial state: Alice: 10,000 ÉTR, Bob: 10,000 ÉTR
├─ Alice signs: "I pay Bob 100 ÉTR, nonce=1"
├─ Bob signs: "I accept 100 ÉTR from Alice, nonce=1"
└─ New state: Alice: 9,900 ÉTR, Bob: 10,100 ÉTR

Time elapsed: ~100 milliseconds
State location: Off-chain (Lightning Bloc)
FlareChain awareness: None yet ⏳
```

**Step 2: Batch Settlement (5 minutes later)**
```
Lightning Bloc Batch Manager
├─ 1,000 transactions accumulated (including Alice → Bob)
├─ Compress batch: 150 KB → 105 KB
├─ Calculate Merkle root: 0xdef456...
└─ Submit to PBC-EDSC (or FlareChain directly)

Time elapsed: 5 minutes
State location: PBC-EDSC runtime
FlareChain awareness: None yet ⏳
```

**Step 3: PBC Checkpoint (51 minutes later)**
```
PBC-EDSC Checkpoint Pallet
├─ Block 256 finalized
├─ Calculate state root (includes Lightning channel balances)
│   └─ State root: 0xabc123... (Alice: 9,900, Bob: 10,100 included)
├─ Submit checkpoint to FlareChain
└─ Collator signs and sends extrinsic

Time elapsed: 5 + 51 = 56 minutes total
State location: FlareChain checkpoint registry
FlareChain awareness: ✅ Directors now see Alice's balance reduction
```

**Step 4: FlareChain Finality (2 blocks = 24 seconds)**
```
FlareChain GRANDPA Consensus
├─ Block #1234 includes PBC-EDSC checkpoint
├─ Validators sign block
├─ GRANDPA finalizes block #1234
└─ Checkpoint now immutable

Time elapsed: 56 + 0.4 = ~56.4 minutes total
State location: Finalized FlareChain block
FlareChain awareness: ✅ Directors have immutable record
```

**Final Result:**
- ✅ Alice's Lightning transaction is now anchored on FlareChain
- ✅ Can be used for dispute resolution
- ✅ Economic finality achieved
- ⏱️ Total time: ~56 minutes from off-chain tx to FlareChain finality
- 🎯 Off-chain was instant, on-chain finality delayed for security

---

## Part 4: Security Properties

### 4.1 Multi-Layer Security Model

**Layer 3 (Lightning Bloc):**
- **Security:** Cryptographic signatures + counterparty monitoring
- **Attack resistance:** Watchtower network detects fraud
- **Recovery:** Emergency withdrawals using last checkpoint
- **Assumption:** Optimistic (assume honest unless proven otherwise)

**Layer 2 (PBCs):**
- **Security:** Validity Node consensus (8 validators per PBC)
- **Attack resistance:** 66% BFT threshold (5/8 honest required)
- **Recovery:** Checkpoints to FlareChain every 256 blocks
- **Assumption:** Economic security via validator stakes

**Layer 1 (FlareChain):**
- **Security:** Flare Node consensus (5-9 Directors) + GRANDPA finality
- **Attack resistance:** 66% BFT threshold + social consensus
- **Recovery:** On-chain governance via proposals
- **Assumption:** Decentralized Director elections (Consensus Day)

---

### 4.2 Fraud Detection and Resolution

**Lightning Bloc Fraud:**
```
Scenario: Bob tries to close channel with old state

┌─────────────────────────────────────────────┐
│  Bob closes channel with state:             │
│  Alice: 10,000 ÉTR, Bob: 10,000 ÉTR         │
│  (nonce=0, initial state)                   │
└────────────┬────────────────────────────────┘
             │
             │ Watchtower detects fraud ⚠️
             │ (Knows latest state is nonce=100)
             ↓
┌─────────────────────────────────────────────┐
│  Watchtower submits fraud proof:            │
│  ├─ Latest signed state: nonce=100          │
│  │   Alice: 9,900 ÉTR, Bob: 10,100 ÉTR      │
│  ├─ Alice's signature on nonce=100          │
│  └─ Bob's signature on nonce=100            │
└────────────┬────────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────────┐
│  On-chain dispute resolution:               │
│  ├─ Verify signatures                       │
│  ├─ Compare nonces (100 > 0)                │
│  ├─ Slash Bob (1,000 ÉTR penalty)           │
│  └─ Close channel with correct state        │
│      Alice: 9,900, Bob: 10,100 - 1,000      │
│      = Bob: 9,100 ÉTR final                 │
└─────────────────────────────────────────────┘
```

**PBC Fraud:**
```
Scenario: Malicious collator submits invalid checkpoint

┌─────────────────────────────────────────────┐
│  Malicious Collator submits:                │
│  state_root: 0xFAKE (doesn't match real PBC)│
└────────────┬────────────────────────────────┘
             │
             │ Challenge Period (7 days)
             ↓
┌─────────────────────────────────────────────┐
│  Honest Validator challenges:               │
│  ├─ "This state root is wrong!"             │
│  ├─ Provides Merkle proof of correct state  │
│  └─ Shows discrepancy                       │
└────────────┬────────────────────────────────┘
             │
             ↓
┌─────────────────────────────────────────────┐
│  FlareChain Directors verify:               │
│  ├─ Check Merkle proofs                     │
│  ├─ Validate signatures                     │
│  ├─ Determine malicious collator            │
│  └─ Slash collator stake (10,000 ÉTR)       │
└─────────────────────────────────────────────┘
```

---

## Part 5: Key Differences from Other Chains

### 5.1 vs Ethereum Rollups

| Feature | Ëtrid (PBC + Lightning) | Ethereum (Optimistic Rollup) |
|---------|-------------------------|------------------------------|
| **L1 Chain** | FlareChain (1,000 TPS) | Ethereum (15 TPS) |
| **L2 Solution** | PBCs (5,000 TPS each) | Arbitrum/Optimism (~4,000 TPS) |
| **L3 Solution** | Lightning Bloc (100k+ TPS) | Lightning Network (separate) |
| **State Sync** | Checkpoints every 256 blocks | Batches every ~1 hour |
| **Challenge Period** | 7 days | 7 days |
| **Finality** | GRANDPA (2 blocks = 24s) | Ethereum finality (~13 min) |
| **Integration** | Native (PBCs are parachains) | Smart contracts |

---

### 5.2 vs Polkadot Parachains

| Feature | Ëtrid PBCs | Polkadot Parachains |
|---------|------------|---------------------|
| **Consensus** | PPFA (8 validators rotating) | BABE + GRANDPA (all validators) |
| **Checkpoint Frequency** | Every 256 blocks (~51 min) | Every block (~6s) |
| **Validator Count** | 8 per PBC | 300+ shared |
| **Specialization** | Domain-specific (EDSC, BTC bridge) | General-purpose |
| **Lightning Layer** | ✅ Integrated | ❌ Separate projects |
| **State Sharing** | Checkpoints to FlareChain | Full state root every block |

---

## Part 6: FAQ

### Q1: Do Directors validate every Lightning transaction?

**A: No.** Directors only validate:
- PBC checkpoints (every 256 blocks)
- Checkpoint signatures
- Merkle roots

Lightning transactions stay off-chain until batched and settled.

---

### Q2: What if a PBC stops submitting checkpoints?

**A:** FlareChain monitors checkpoint liveness:

```rust
// In FlareChain runtime
fn check_pbc_health(para_id: ParaId, current_block: BlockNumber) {
    let last_checkpoint = LastCheckpoint::<T>::get(para_id);
    let blocks_since = current_block - last_checkpoint.block_number;

    if blocks_since > MAX_CHECKPOINT_INTERVAL {
        // PBC is unresponsive
        // Freeze PBC (stop accepting new transactions)
        PBCStatus::<T>::insert(para_id, Status::Frozen);

        // Alert validators
        Self::deposit_event(Event::PBCFrozen { para_id });

        // Emergency recovery mode
        // Users can withdraw using last known checkpoint
    }
}
```

---

### Q3: How do Directors query specific Lightning channel states?

**A:** Via Merkle proof against PBC state root:

```rust
// Example: Query Alice's balance in Lightning channel

// 1. Get latest PBC checkpoint from FlareChain
let checkpoint = Checkpoints::get(ParaId::EDSC, latest_block);
let state_root = checkpoint.state_root; // 0xabc123...

// 2. Request Merkle proof from PBC node
let merkle_proof = pbc_node.query_state_proof(
    state_root,
    "lightning_channels/alice_bob/balance_alice"
);

// 3. Verify proof against state root
if verify_merkle_proof(state_root, merkle_proof, "9900") {
    println!("Alice has 9,900 ÉTR in channel (verified)");
}
```

---

### Q4: What's the throughput of the entire network?

**A:** Theoretical maximum:

```
FlareChain (Layer 1):
├─ 1,000 TPS

PBCs (Layer 2): 14 chains × 5,000 TPS = 70,000 TPS
├─ EDSC-PBC: 5,000 TPS
├─ BTC-PBC: 5,000 TPS
├─ ETH-PBC: 5,000 TPS
└─ ... (11 more PBCs)

Lightning Bloc (Layer 3): 100,000+ TPS
├─ Off-chain transactions
├─ No block limit
└─ Limited only by network bandwidth

Total Theoretical: 171,000+ TPS
├─ Layer 1: 1,000 TPS
├─ Layer 2: 70,000 TPS
└─ Layer 3: 100,000+ TPS
```

**Actual (with safety margins):**
- Layer 1: ~800 TPS (80% capacity)
- Layer 2: ~56,000 TPS (80% capacity)
- Layer 3: ~80,000 TPS (practical limit)
- **Total: ~137,000 TPS**

---

### Q5: How long until a Lightning transaction is "final"?

**A:** Depends on finality definition:

| Finality Type | Time | Security Level |
|---------------|------|----------------|
| **Off-chain finality** | ~100ms | Counterparty signature |
| **Batch settlement** | 5 minutes | PBC inclusion |
| **PBC checkpoint** | 56 minutes | FlareChain checkpoint |
| **GRANDPA finality** | 56.4 minutes | Absolute finality |

**For most use cases:** 5-minute batch settlement is sufficient
**For maximum security:** Wait 56 minutes for FlareChain finality

---

## Conclusion

### How FlareChain Directors Get State Updates

**From PBCs:**
1. **Every 256 blocks (~51 minutes):** PBC collators submit checkpoints
2. **Checkpoint contains:** Merkle root, economic data, timestamps
3. **Directors verify:** Signatures, store in registry, finalize via GRANDPA
4. **Result:** Full PBC state anchored on FlareChain

**From Lightning Bloc:**
1. **Every 5 minutes:** Lightning batches settle to PBC (or FlareChain)
2. **PBC includes Lightning state in state root**
3. **Checkpoint carries Lightning state to FlareChain**
4. **Result:** Lightning channel balances indirectly visible via Merkle proofs

### Key Takeaways

✅ **Efficient:** Only Merkle roots cross chains (32 bytes vs full state)
✅ **Secure:** Multi-layer security with fraud proofs at each level
✅ **Scalable:** 137,000 TPS across all layers
✅ **Flexible:** Emergency withdrawals available using last checkpoint
✅ **Decentralized:** Directors don't need to track every transaction

---

**Status:** Complete technical documentation
**Next Steps:** Deploy PBCs to activate Layers 2 & 3
**Timeline:** Follow PBC_DEPLOYMENT_GUIDE.md

---

**Last Updated:** 2025-11-04
