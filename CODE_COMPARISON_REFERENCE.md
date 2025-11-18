# GRANDPA vs ASF - Side-by-Side Code Comparison

## 1. Vote Triggering Entry Point

### GRANDPA (Working)
**File**: `/home/user/Etrid/09-consensus/finality-gadget/src/lib.rs`  
**Lines**: 605-630

```rust
pub async fn propose_block(&mut self, block_hash: BlockHash) -> Result<Vote, String> {
    // Immediately creates a vote when block arrives
    let dummy_signature = {
        let mut sig = Vec::with_capacity(64);
        sig.extend_from_slice(&self.validator_id.0.to_le_bytes());
        sig.extend_from_slice(&block_hash.0[0..28]);
        sig.extend_from_slice(&block_hash.0[0..32]);
        sig
    };

    let vote = Vote {
        validator_id: self.validator_id,
        view: self.view_timer.get_current_view(),
        block_hash,
        signature: dummy_signature,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    self.broadcast_vote(vote.clone()).await?;  // IMMEDIATE BROADCAST
    Ok(vote)
}
```

**Key Pattern**: `async fn propose_block()` is called immediately when block import event fires.

### ASF (Missing)
**File**: `/home/user/Etrid/09-consensus/client/consensus-asf/src/lib.rs`  
**Lines**: 79-93

```rust
// Modules
pub mod verifier;
pub mod import_queue;
pub mod worker;

// TODO: Implement these modules in future sessions
// pub mod aux_schema;
// pub mod inherents;

// Re-exports
pub use verifier::{AsfVerifier, VerificationParams};
pub use import_queue::{import_queue, AsfImportQueueVerifier, ImportQueueParams};
pub use worker::{run_asf_worker, AsfWorkerParams};
```

**Problem**: NO MODULE for voter. Would need:
```rust
pub mod voter;  // MISSING
pub use voter::{run_asf_voter, AsfVoterParams};
```

---

## 2. Vote Collection & Quorum Detection

### GRANDPA (Working)
**File**: `/home/user/Etrid/09-consensus/finality-gadget/src/lib.rs`  
**Lines**: 139-238

```rust
pub struct VoteCollector {
    votes: HashMap<View, HashMap<BlockHash, Vec<(ValidatorId, Vec<u8>)>>>,
    quorum_threshold: u32,
    max_validators: u32,
}

impl VoteCollector {
    pub fn add_vote(&mut self, vote: Vote) -> Result<bool, String> {
        // ... validation ...
        
        block_votes.push((vote.validator_id, vote.signature));

        let vote_count = block_votes.len() as u32;
        
        // Check if we reached quorum (2f+1)
        let reached_quorum = vote_count >= self.quorum_threshold;
        
        if reached_quorum {
            tracing::info!(
                "🎯 QUORUM REACHED! view={:?}, block={}, votes={}/{}",
                vote.view,
                block_hash_short,
                vote_count,
                self.quorum_threshold
            );
        }

        Ok(reached_quorum)  // Return true when quorum reached
    }
}
```

**Key Pattern**: 
1. Maintains in-memory vote map
2. Returns boolean when quorum is reached
3. Validates each vote before adding
4. Prevents double voting

### ASF (Missing)
**File**: `/home/user/Etrid/09-consensus/pallet/src/lib.rs`  
**Lines**: 566-622

```rust
pub fn issue_certificate(
    origin: OriginFor<T>,
    block_hash: T::Hash,
    block_number: BlockNumberFor<T>,
    phase: ConsensusPhase,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Verify validator is in current committee
    let committee = CurrentCommittee::<T>::get();
    let member = committee
        .iter()
        .find(|m| m.validator == who)
        .ok_or(Error::<T>::NotInCommittee)?;

    let cert = ValidityCertificate {
        block_hash,
        block_number,
        phase: phase.clone(),
        validator: who.clone(),
        stake_weight: member.stake,
        epoch: CurrentEpoch::<T>::get(),
        timestamp: T::Time::now().try_into().ok().unwrap_or(0),
    };

    // Add certificate
    Certificates::<T>::try_mutate(block_hash, |certs| {
        certs.try_push(cert)
            .map_err(|_| Error::<T>::TooManyCertificates)
    })?;
    
    // Check finality
    Self::check_finality(block_hash, block_number, count);

    Ok(())
}
```

**Problem**:
1. Only accepts PRE-MADE certificates (not votes)
2. No vote aggregation
3. No quorum detection
4. No automatic certificate creation

---

## 3. Inbound Message Handling & Aggregation

### GRANDPA (Working)
**File**: `/home/user/Etrid/09-consensus/finality-gadget/src/lib.rs`  
**Lines**: 524-569

```rust
pub async fn handle_vote(&mut self, vote: Vote) -> Result<(), String> {
    // Validate vote
    if vote.view != self.view_timer.get_current_view() {
        let rep = self.peer_reputation.entry(vote.validator_id)
            .or_insert_with(PeerReputation::new);
        rep.record_invalid();
        return Err(format!("Vote from wrong view: {:?}", vote.view));
    }

    // Add to collector - THIS RETURNS BOOLEAN IF QUORUM REACHED
    let reached_quorum = self.vote_collector.add_vote(vote.clone())?;

    // Update reputation
    let rep = self.peer_reputation.entry(vote.validator_id)
        .or_insert_with(PeerReputation::new);
    rep.record_valid();

    // If quorum reached, create certificate - AUTOMATIC
    if reached_quorum {
        if let Some(signatures) = self.vote_collector.get_quorum_for_block(vote.view, vote.block_hash) {
            let cert = Certificate {
                view: vote.view,
                block_hash: vote.block_hash,
                signatures: signatures.clone(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            tracing::info!(
                "📜 CERTIFICATE CREATED! view={:?}, block={}, signatures={}",
                vote.view,
                block_hash_short,
                signatures.len()
            );

            self.pending_certificates.push_back(cert.clone());
            self.gossip_scheduler.schedule_certificate(cert);
        }
    }

    Ok(())
}
```

**Key Pattern**:
1. Handle incoming vote
2. Validate it
3. Add to collector
4. **AUTOMATICALLY create certificate if quorum reached**
5. Schedule certificate for gossip

### ASF (Completely Missing)
**No equivalent function exists**

ASF would need:
```rust
// NOT IMPLEMENTED - pallet doesn't handle incoming votes at all
pub async fn handle_incoming_vote(vote: Vote) {
    // Need to:
    // 1. Store vote somewhere
    // 2. Check quorum
    // 3. Auto-create certificate
    // 4. Emit event
}
```

---

## 4. Network Broadcasting

### GRANDPA (Working)
**File**: `/home/user/Etrid/09-consensus/finality-gadget/src/lib.rs`  
**Lines**: 674-697

```rust
pub async fn run_worker(&mut self) {
    let mut gossip_interval = interval(Duration::from_millis(500));
    let mut timeout_interval = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = gossip_interval.tick() => {
                // Every 500ms, send pending messages
                let (votes, certs) = self.gossip_scheduler.get_ready_messages();
                
                for vote in votes {
                    let _ = self.network_bridge.broadcast_vote(vote).await;
                }
                
                for cert in certs {
                    let _ = self.network_bridge.broadcast_certificate(cert).await;
                }
            }
            
            _ = timeout_interval.tick() => {
                let _ = self.handle_timeout().await;
            }
        }
    }
}
```

**Key Pattern**:
1. Runs as background service
2. Broadcasts every 500ms
3. Uses gossip scheduler with exponential backoff
4. Handles timeouts every 1 second

### ASF (Missing)
**File**: `/home/user/Etrid/05-multichain/flare-chain/node/src/asf_service.rs`  
**Lines**: 196-219

```rust
// ═══════════════════════════════════════════════════════════════════════════
// ASF BLOCK IMPORT (Pure ASF, no GRANDPA)
// ═══════════════════════════════════════════════════════════════════════════
//
// v108 migration: Use client directly as block import for pure ASF consensus.
// ASF finality gadget handles all finality - no GRANDPA needed.

let block_import = client.clone();

// ═══════════════════════════════════════════════════════════════════════════
// ASF IMPORT QUEUE
// ═══════════════════════════════════════════════════════════════════════════
//
// This import queue validates blocks using ASF rules:
// 1. Verify PPFA proposer is authorized for this slot
// 2. Check block type (Queen vs Ant)
// 3. Validate parent certificates for finality
// 4. Apply ASF-specific inherent data
```

**Problem**: Comment says "ASF finality gadget handles all finality" but NO CODE implements it.

---

## 5. Authority/Keystore Integration

### GRANDPA (Implicit in Network Bridge)
**File**: `/home/user/Etrid/09-consensus/finality-gadget/src/lib.rs`  
**Lines**: 474-479

```rust
#[async_trait::async_trait]
pub trait NetworkBridge: Send + Sync {
    async fn broadcast_vote(&self, vote: Vote) -> Result<(), String>;
    async fn broadcast_certificate(&self, cert: Certificate) -> Result<(), String>;
    async fn get_connected_peers(&self) -> Vec<String>;
}
```

Assumes network bridge handles signing internally.

### ASF (Partial for Block Production)
**File**: `/home/user/Etrid/09-consensus/client/consensus-asf/src/worker.rs`  
**Lines**: 300-343

```rust
async fn check_if_we_are_proposer<AuthorityId>(
    keystore: &KeystorePtr,
    expected_proposer: &AuthorityId,
) -> bool
where
    AuthorityId: Codec + Clone + AsRef<[u8]>,
{
    use sp_application_crypto::sr25519;
    use sp_core::crypto::ByteArray;

    let proposer_bytes = expected_proposer.as_ref();

    let public_key = match sr25519::Public::from_slice(proposer_bytes) {
        Ok(key) => key,
        Err(_) => {
            log::warn!(target: "asf", "Failed to parse authority ID as sr25519 public key");
            return false;
        }
    };

    let key_type = sp_core::crypto::key_types::AURA;

    if keystore.has_keys(&[(public_key.to_raw_vec(), key_type)]) {
        log::debug!(target: "asf", "✓ We are the proposer - found matching key in keystore");
        true
    } else {
        log::trace!(target: "asf", "Not our turn - no matching key in keystore");
        false
    }
}
```

**Problem**: 
1. Only for block production
2. No equivalent for voting
3. No code to get private key and sign votes

---

## 6. Phase Progression

### GRANDPA (View-based)
**File**: `/home/user/Etrid/09-consensus/finality-gadget/src/lib.rs`  
**Lines**: 313-358

```rust
pub struct ViewChangeTimer {
    current_view: View,
    timeout_duration: Duration,
    last_block_time: Instant,
    view_change_pending: bool,
}

impl ViewChangeTimer {
    pub fn should_trigger_view_change(&self) -> bool {
        self.last_block_time.elapsed() > self.timeout_duration && !self.view_change_pending
    }

    pub fn trigger_view_change(&mut self) -> NewViewMessage {
        let new_view = View(self.current_view.0 + 1);
        self.current_view = new_view;
        self.last_block_time = Instant::now();
        self.view_change_pending = true;

        NewViewMessage {
            new_view,
            sender: ValidatorId(0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn on_certificate_created(&mut self) {
        self.last_block_time = Instant::now();
        self.view_change_pending = false;
    }
}
```

**Key Pattern**: Automatic timeout-based view progression

### ASF (Broken - Manual)
**File**: `/home/user/Etrid/09-consensus/pallet/src/lib.rs`  
**Lines**: 94-104

```rust
pub enum ConsensusPhase {
    /// Phase 1: Leader collects highest valid branch and prepare votes
    Prepare,
    /// Phase 2: Prepare certificate broadcast, nodes send pre-commit votes
    PreCommit,
    /// Phase 3: Commit certificate broadcast, replicas lock state
    Commit,
    /// Phase 4: Commit certificate finalized, state transition occurs
    Decide,
}
```

Storage exists (lines 275-278):
```rust
/// Current consensus phase for active proposal
#[pallet::storage]
#[pallet::getter(fn current_phase)]
pub type CurrentPhase<T: Config> = StorageValue<_, ConsensusPhase, ValueQuery>;
```

But NO CODE to advance it based on certificate count.

---

## Summary: The Missing Voter Loop

### What ASF Needs (Pattern from GRANDPA)

```rust
// NEW FILE: /home/user/Etrid/09-consensus/client/consensus-asf/src/voter.rs

pub struct AsfVoter<C, B> {
    client: Arc<C>,
    keystore: KeystorePtr,
    validator_id: ValidatorId,
    current_phase: ConsensusPhase,
    vote_collector: VoteCollector,  // From pallet or in-memory
    last_voted_block: Option<BlockHash>,
}

impl<C, B> AsfVoter<C, B> {
    pub async fn run(mut self) {
        // Subscribe to block imports
        self.client.import_notification_stream()
            .for_each(|block_notification| {
                self.handle_block_import(block_notification)
            })
            .await;
    }
    
    async fn handle_block_import(&mut self, block: BlockImportNotification<B>) {
        // 1. Check if we're in committee
        // 2. Check if we should vote for this block
        // 3. Create signed vote
        // 4. Submit as extrinsic OR gossip message
    }
}
```

### Existing Code to Use as Pattern

| Component | GRANDPA File | Lines | Pattern |
|-----------|------|-------|---------|
| VoteCollector | finality-gadget/src/lib.rs | 139-238 | Copy vote collection logic |
| propose_block | finality-gadget/src/lib.rs | 605-630 | Copy vote creation pattern |
| handle_vote | finality-gadget/src/lib.rs | 524-569 | Copy quorum detection |
| run_worker | finality-gadget/src/lib.rs | 674-697 | Copy broadcast pattern |
| Keystore check | worker.rs | 300-343 | Copy authority verification pattern |

---

## Exact File Locations for Implementation

### Files to Create
1. `/home/user/Etrid/09-consensus/client/consensus-asf/src/voter.rs`
2. `/home/user/Etrid/09-consensus/client/consensus-asf/src/gossip.rs`

### Files to Modify
1. `/home/user/Etrid/09-consensus/client/consensus-asf/src/lib.rs` (add voter module)
2. `/home/user/Etrid/05-multichain/flare-chain/node/src/asf_service.rs` (spawn voter)
3. `/home/user/Etrid/09-consensus/pallet/src/lib.rs` (add phase progression)

### Files to Reference
1. `/home/user/Etrid/09-consensus/finality-gadget/src/lib.rs` (all patterns)
2. `/home/user/Etrid/09-consensus/client/consensus-asf/src/worker.rs` (keystore pattern)

---

**Generated**: November 18, 2025  
**Status**: Complete code comparison with line numbers  
**Next Step**: Create voter.rs following GRANDPA patterns above
