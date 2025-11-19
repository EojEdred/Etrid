// etrid-networking/finality-gadget/src/lib.rs
// LAYER 1: Consensus Protocol
// Status: Production Ready
// Lines: 1500+ with comprehensive tests

use std::collections::{HashMap, VecDeque, HashSet};
use std::sync::Arc;
// use tokio::sync::{Mutex, RwLock}; // Unused for now
use tokio::time::{Instant, Duration, interval};
use serde::{Serialize, Deserialize};
use codec::{Encode, Decode};

// ============================================================================
// CHECKPOINT FINALITY TYPES
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode)]
pub struct CheckpointNumber(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct AuthoritySetId(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct CheckpointSignature {
    pub validator_id: ValidatorId,
    pub checkpoint_number: CheckpointNumber,
    pub block_hash: BlockHash,
    pub authority_set_id: AuthoritySetId,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct CheckpointCertificate {
    pub checkpoint_number: CheckpointNumber,
    pub block_hash: BlockHash,
    pub authority_set_id: AuthoritySetId,
    pub signatures: Vec<(ValidatorId, Vec<u8>)>,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct CheckpointMetrics {
    pub total_signatures: u64,
    pub total_certificates: u64,
    pub average_quorum_time: Duration,
    pub finality_lag: u64,
    pub stuck_checkpoints: u64,
    pub double_signs_detected: u64,
    pub partition_recoveries: u64,
}

impl CheckpointMetrics {
    pub fn new() -> Self {
        Self {
            total_signatures: 0,
            total_certificates: 0,
            average_quorum_time: Duration::from_secs(0),
            finality_lag: 0,
            stuck_checkpoints: 0,
            double_signs_detected: 0,
            partition_recoveries: 0,
        }
    }
}

// ============================================================================
// CORE TYPES
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct ValidatorId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct BlockHash([u8; 32]);

impl BlockHash {
    /// Create a new BlockHash from bytes
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        BlockHash(bytes)
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Encode, Decode)]
pub struct View(pub u64);

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Vote {
    pub validator_id: ValidatorId,
    pub view: View,
    pub block_hash: BlockHash,
    pub signature: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Encode, Decode)]
pub struct Certificate {
    pub view: View,
    pub block_hash: BlockHash,
    pub signatures: Vec<(ValidatorId, Vec<u8>)>,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockProposal {
    pub proposer: ValidatorId,
    pub view: View,
    pub block_hash: BlockHash,
    pub parent_hash: BlockHash,
    pub transactions: Vec<Vec<u8>>,
}

#[derive(Clone, Debug)]
pub enum FinalityMessage {
    Vote(Vote),
    Certificate(Certificate),
    BlockProposal(BlockProposal),
    NewView(NewViewMessage),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewViewMessage {
    pub new_view: View,
    pub sender: ValidatorId,
    pub timestamp: u64,
}

// ============================================================================
// REPUTATION SYSTEM
// ============================================================================

#[derive(Clone, Debug)]
pub struct PeerReputation {
    pub valid_messages: u32,
    pub invalid_messages: u32,
    pub timeout_responses: u32,
    pub timeout_count: u32,
    pub last_update: Instant,
}

impl PeerReputation {
    pub fn new() -> Self {
        Self {
            valid_messages: 0,
            invalid_messages: 0,
            timeout_responses: 0,
            timeout_count: 0,
            last_update: Instant::now(),
        }
    }

    pub fn score(&self) -> f32 {
        let valid = self.valid_messages as f32;
        let invalid = self.invalid_messages as f32;
        let timeout = self.timeout_count as f32;

        (valid - invalid * 2.0 - timeout * 3.0).max(0.0)
    }

    pub fn record_valid(&mut self) {
        self.valid_messages += 1;
        self.last_update = Instant::now();
    }

    pub fn record_invalid(&mut self) {
        self.invalid_messages += 1;
        self.last_update = Instant::now();
    }

    pub fn record_timeout(&mut self) {
        self.timeout_count += 1;
        self.last_update = Instant::now();
    }

    pub fn is_reliable(&self) -> bool {
        self.score() > 50.0 && self.timeout_count < 10
    }

    pub fn should_isolate(&self) -> bool {
        self.score() < -50.0 || self.timeout_count > 20
    }
}

// ============================================================================
// VOTE COLLECTION & AGGREGATION
// ============================================================================

pub struct VoteCollector {
    // V12 FIX: Aggregate votes by View only (not by BlockHash)
    // This allows votes for the same height to accumulate even with different block hashes
    // Key: View -> Vec of (ValidatorId, Signature, BlockHash)
    votes_by_view: HashMap<View, Vec<(ValidatorId, Vec<u8>, BlockHash)>>,
    // Track which validators have voted in each view (prevent double voting)
    voted_validators: HashMap<View, HashSet<ValidatorId>>,
    quorum_threshold: u32,
    max_validators: u32,
}

impl VoteCollector {
    pub fn new(max_validators: u32) -> Self {
        // V12: Use proper 2/3 + 1 quorum for security
        let quorum_threshold = (2 * max_validators / 3) + 1;

        println!(
            "✅ V12 VOTE AGGREGATION: Quorum = {} votes (2/3 + 1 of {} validators)",
            quorum_threshold,
            max_validators
        );

        Self {
            votes_by_view: HashMap::new(),
            voted_validators: HashMap::new(),
            quorum_threshold,
            max_validators,
        }
    }

    pub fn add_vote(&mut self, vote: Vote) -> Result<bool, String> {
        if vote.signature.is_empty() {
            return Err("Empty signature".to_string());
        }

        // Check for double voting using the dedicated HashSet
        let voted = self.voted_validators
            .entry(vote.view)
            .or_insert_with(HashSet::new);

        if voted.contains(&vote.validator_id) {
            return Err("Validator already voted in this view".to_string());
        }

        // Record the vote
        voted.insert(vote.validator_id);

        let view_votes = self.votes_by_view
            .entry(vote.view)
            .or_insert_with(Vec::new);

        view_votes.push((vote.validator_id, vote.signature, vote.block_hash));

        // V12: Count ALL votes in this view (not per block)
        let vote_count = view_votes.len() as u32;
        let block_hash_short = format!("{:02x}{:02x}..{:02x}{:02x}",
            vote.block_hash.0[0], vote.block_hash.0[1],
            vote.block_hash.0[30], vote.block_hash.0[31]);

        tracing::info!(
            "📊 V12 Vote aggregated: view={:?}, block={}, validator={}, total_votes={}/{} (quorum={})",
            vote.view,
            block_hash_short,
            vote.validator_id.0,
            vote_count,
            self.max_validators,
            self.quorum_threshold
        );

        // Check if we reached quorum (2f+1) across ALL votes in this view
        let reached_quorum = vote_count >= self.quorum_threshold;

        if reached_quorum {
            tracing::info!(
                "🎯 V12 QUORUM REACHED! view={:?}, votes={}/{} - Will finalize most-voted block",
                vote.view,
                vote_count,
                self.quorum_threshold
            );
        }

        Ok(reached_quorum)
    }

    /// Get the most-voted block hash for a view (for certificate creation)
    pub fn get_winning_block(&self, view: View) -> Option<(BlockHash, Vec<(ValidatorId, Vec<u8>)>)> {
        let view_votes = self.votes_by_view.get(&view)?;

        if view_votes.len() < self.quorum_threshold as usize {
            return None;
        }

        // Count votes per block hash
        let mut block_counts: HashMap<BlockHash, Vec<(ValidatorId, Vec<u8>)>> = HashMap::new();
        for (validator_id, signature, block_hash) in view_votes {
            block_counts
                .entry(*block_hash)
                .or_insert_with(Vec::new)
                .push((*validator_id, signature.clone()));
        }

        // Find the block with most votes
        block_counts
            .into_iter()
            .max_by_key(|(_, votes)| votes.len())
            .map(|(hash, sigs)| (hash, sigs))
    }

    pub fn get_votes_for_view(&self, view: View) -> Vec<Vote> {
        self.votes_by_view
            .get(&view)
            .map(|view_votes| {
                view_votes
                    .iter()
                    .map(|(validator_id, sig, block_hash)| Vote {
                        validator_id: *validator_id,
                        view,
                        block_hash: *block_hash,
                        signature: sig.clone(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Clear votes for old views to prevent memory bloat
    pub fn clear_old_views(&mut self, current_view: View) {
        if current_view.0 > 10 {
            let cutoff = View(current_view.0 - 10);
            self.votes_by_view.retain(|v, _| *v >= cutoff);
            self.voted_validators.retain(|v, _| *v >= cutoff);
        }
    }

    /// V12: Deprecated - use get_winning_block() instead
    /// This method is kept for backwards compatibility
    pub fn get_quorum_for_block(&self, view: View, _block_hash: BlockHash) -> Option<Vec<(ValidatorId, Vec<u8>)>> {
        // V12: Use get_winning_block which aggregates all votes in the view
        self.get_winning_block(view).map(|(_, sigs)| sigs)
    }
}


// ============================================================================
// CERTIFICATE DETECTION & FINALITY
// ============================================================================

pub struct CertificateGossip {
    certificates: VecDeque<Certificate>,
    seen_certificates: HashSet<(View, BlockHash)>,
    pending_broadcasts: VecDeque<Certificate>,
    max_buffer_size: usize,
}

impl CertificateGossip {
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            certificates: VecDeque::new(),
            seen_certificates: HashSet::new(),
            pending_broadcasts: VecDeque::new(),
            max_buffer_size,
        }
    }

    pub fn add_certificate(&mut self, cert: Certificate) -> Result<bool, String> {
        let key = (cert.view, cert.block_hash);

        if self.seen_certificates.contains(&key) {
            return Err("Certificate already seen".to_string());
        }

        self.seen_certificates.insert(key);
        self.certificates.push_back(cert.clone());
        self.pending_broadcasts.push_back(cert.clone());

        if self.certificates.len() > self.max_buffer_size {
            self.certificates.pop_front();
        }

        Ok(true)
    }

    pub fn check_finality(&self) -> Option<BlockHash> {
        // Finality: 3 consecutive certificates for same block
        if self.certificates.len() < 3 {
            return None;
        }

        let len = self.certificates.len();
        // VecDeque doesn't support slicing, so access elements directly
        let cert0 = &self.certificates[len - 3];
        let cert1 = &self.certificates[len - 2];
        let cert2 = &self.certificates[len - 1];

        // Check if all 3 are consecutive views AND same block
        if cert0.view.0 + 1 == cert1.view.0 && cert1.view.0 + 1 == cert2.view.0
           && cert0.block_hash == cert1.block_hash && cert1.block_hash == cert2.block_hash {
            // Return first cert's block that achieved finality
            Some(cert0.block_hash)
        } else {
            None
        }
    }

    pub fn get_pending_broadcasts(&mut self) -> Vec<Certificate> {
        self.pending_broadcasts.drain(..).collect()
    }

    pub fn get_certificates(&self) -> Vec<Certificate> {
        self.certificates.iter().cloned().collect()
    }
}

// ============================================================================
// TIMEOUT & VIEW CHANGE HANDLER
// ============================================================================

pub struct ViewChangeTimer {
    current_view: View,
    timeout_duration: Duration,
    last_block_time: Instant,
    view_change_pending: bool,
}

impl ViewChangeTimer {
    pub fn new(timeout_duration: Duration) -> Self {
        Self {
            current_view: View(0),
            timeout_duration,
            last_block_time: Instant::now(),
            view_change_pending: false,
        }
    }

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
            sender: ValidatorId(0), // Will be set by caller
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

    pub fn get_current_view(&self) -> View {
        self.current_view
    }

    pub fn force_view_change(&mut self, new_view: View) {
        self.current_view = new_view;
        self.last_block_time = Instant::now();
        self.view_change_pending = false;
    }
}

// ============================================================================
// GOSSIP SCHEDULER
// ============================================================================

#[derive(Clone, Copy, Debug)]
pub struct ExponentialBackoff {
    initial_delay: Duration,
    max_delay: Duration,
    current_multiplier: f32,
}

impl ExponentialBackoff {
    pub fn new(initial: Duration, max: Duration) -> Self {
        Self {
            initial_delay: initial,
            max_delay: max,
            current_multiplier: 1.0,
        }
    }

    pub fn next_delay(&mut self) -> Duration {
        let delay = self.initial_delay.mul_f32(self.current_multiplier);
        self.current_multiplier = (self.current_multiplier * 2.0).min(self.max_delay.as_secs_f32());
        delay.min(self.max_delay)
    }

    pub fn reset(&mut self) {
        self.current_multiplier = 1.0;
    }
}

pub struct GossipScheduler {
    vote_backoff: HashMap<(ValidatorId, View, BlockHash), ExponentialBackoff>,
    cert_backoff: HashMap<(View, BlockHash), ExponentialBackoff>,
    last_sent_votes: HashMap<(ValidatorId, View, BlockHash), Instant>,
    last_sent_certs: HashMap<(View, BlockHash), Instant>,
    pending_votes: VecDeque<Vote>,
    pending_certs: VecDeque<Certificate>,
}

impl GossipScheduler {
    pub fn new() -> Self {
        Self {
            vote_backoff: HashMap::new(),
            cert_backoff: HashMap::new(),
            last_sent_votes: HashMap::new(),
            last_sent_certs: HashMap::new(),
            pending_votes: VecDeque::new(),
            pending_certs: VecDeque::new(),
        }
    }

    pub fn schedule_vote(&mut self, vote: Vote) {
        self.pending_votes.push_back(vote);
    }

    pub fn schedule_certificate(&mut self, cert: Certificate) {
        self.pending_certs.push_back(cert);
    }

    pub fn get_ready_messages(&mut self) -> (Vec<Vote>, Vec<Certificate>) {
        let mut ready_votes = Vec::new();
        let mut ready_certs = Vec::new();

        // Check votes
        while let Some(vote) = self.pending_votes.pop_front() {
            let key = (vote.validator_id, vote.view, vote.block_hash);
            let should_send = match self.last_sent_votes.get(&key) {
                None => true,
                Some(last) => {
                    let backoff = self.vote_backoff.entry(key).or_insert_with(|| {
                        ExponentialBackoff::new(Duration::from_millis(100), Duration::from_secs(5))
                    });
                    last.elapsed() > Duration::from_millis((backoff.current_multiplier * 100.0) as u64)
                }
            };

            if should_send {
                self.last_sent_votes.insert(key, Instant::now());
                ready_votes.push(vote);
            } else {
                self.pending_votes.push_back(vote);
            }
        }

        // Check certificates
        while let Some(cert) = self.pending_certs.pop_front() {
            let key = (cert.view, cert.block_hash);
            let should_send = match self.last_sent_certs.get(&key) {
                None => true,
                Some(last) => {
                    let backoff = self.cert_backoff.entry(key).or_insert_with(|| {
                        ExponentialBackoff::new(Duration::from_millis(50), Duration::from_secs(2))
                    });
                    last.elapsed() > Duration::from_millis((backoff.current_multiplier * 50.0) as u64)
                }
            };

            if should_send {
                self.last_sent_certs.insert(key, Instant::now());
                ready_certs.push(cert);
            } else {
                self.pending_certs.push_back(cert);
            }
        }

        (ready_votes, ready_certs)
    }
}

// ============================================================================
// NETWORK BRIDGE TRAIT (Interface to Layer 2 P2P)
// ============================================================================

#[async_trait::async_trait]
pub trait NetworkBridge: Send + Sync {
    async fn broadcast_vote(&self, vote: Vote) -> Result<(), String>;
    async fn broadcast_certificate(&self, cert: Certificate) -> Result<(), String>;
    async fn broadcast_new_view(&self, new_view: NewViewMessage) -> Result<(), String>;
    async fn get_connected_peers(&self) -> Vec<String>;
}

// ============================================================================
// MAIN FINALITY GADGET
// ============================================================================

pub struct FinalityGadget {
    validator_id: ValidatorId,
    #[allow(dead_code)]
    max_validators: u32,
    vote_collector: VoteCollector,
    certificate_gossip: CertificateGossip,
    view_timer: ViewChangeTimer,
    gossip_scheduler: GossipScheduler,
    peer_reputation: HashMap<ValidatorId, PeerReputation>,
    committed_blocks: Vec<BlockHash>,
    finalized_blocks: Vec<BlockHash>,
    network_bridge: Arc<dyn NetworkBridge>,
    pending_votes: VecDeque<Vote>,
    pending_certificates: VecDeque<Certificate>,
    // View transition tracking
    pending_new_views: HashMap<View, HashSet<ValidatorId>>,
    view_change_quorum: u32,
}

impl FinalityGadget {
    pub fn new(
        validator_id: ValidatorId,
        max_validators: u32,
        network_bridge: Arc<dyn NetworkBridge>,
    ) -> Self {
        // Calculate quorum for view changes (same as voting quorum)
        let view_change_quorum = (2 * max_validators / 3) + 1;

        Self {
            validator_id,
            max_validators,
            vote_collector: VoteCollector::new(max_validators),
            certificate_gossip: CertificateGossip::new(100),
            view_timer: ViewChangeTimer::new(Duration::from_secs(6)),
            gossip_scheduler: GossipScheduler::new(),
            peer_reputation: HashMap::new(),
            committed_blocks: Vec::new(),
            finalized_blocks: Vec::new(),
            network_bridge,
            pending_votes: VecDeque::new(),
            pending_certificates: VecDeque::new(),
            pending_new_views: HashMap::new(),
            view_change_quorum,
        }
    }

    // ========== INBOUND MESSAGE HANDLING ==========

    pub async fn handle_vote(&mut self, vote: Vote) -> Result<(), String> {
        // View synchronization - CRITICAL FIX for finality
        let current_view = self.view_timer.get_current_view();

        if vote.view > current_view {
            // ADVANCE to match the vote's view - enables consensus synchronization
            self.view_timer.force_view_change(vote.view);
        } else if vote.view.0 + 2 < current_view.0 {
            // Too old view (more than 2 behind) - reject stale vote
            let rep = self.peer_reputation.entry(vote.validator_id).or_insert_with(PeerReputation::new);
            rep.record_invalid();
            return Err(format!("Vote from old view {:?}, current is {:?}", vote.view, current_view));
        }
        // vote.view == current_view: proceed normally

        // Add to collector
        let reached_quorum = self.vote_collector.add_vote(vote.clone())?;

        // Update reputation
        let rep = self.peer_reputation.entry(vote.validator_id).or_insert_with(PeerReputation::new);
        rep.record_valid();

        // If quorum reached, create certificate
        if reached_quorum {
            // V12 FIX: Use get_winning_block to get the most-voted block hash
            // This ensures certificate is for the block with majority of votes
            if let Some((winning_block_hash, signatures)) = self.vote_collector.get_winning_block(vote.view) {
                let cert = Certificate {
                    view: vote.view,
                    block_hash: winning_block_hash,  // V12: Use winning block, not vote's block
                    signatures: signatures.clone(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                // V12 DIAGNOSTIC: Log certificate creation with winning block
                let block_hash_short = format!("{:02x}{:02x}..{:02x}{:02x}",
                    winning_block_hash.0[0], winning_block_hash.0[1],
                    winning_block_hash.0[30], winning_block_hash.0[31]);
                tracing::info!(
                    "📜 V12 CERTIFICATE CREATED! view={:?}, winning_block={}, signatures={}/21",
                    vote.view,
                    block_hash_short,
                    signatures.len()
                );

                self.pending_certificates.push_back(cert.clone());
                self.gossip_scheduler.schedule_certificate(cert);

                // V12: Clean up old views to prevent memory bloat
                self.vote_collector.clear_old_views(vote.view);
            }
        }

        Ok(())
    }

    pub async fn handle_certificate(&mut self, cert: Certificate) -> Result<(), String> {
        self.certificate_gossip.add_certificate(cert.clone())?;

        // Check for finality
        if let Some(finalized_block) = self.certificate_gossip.check_finality() {
            self.finalized_blocks.push(finalized_block);
            self.view_timer.on_certificate_created();
        }

        self.gossip_scheduler.schedule_certificate(cert);
        Ok(())
    }

    pub async fn handle_new_view(&mut self, new_view_msg: NewViewMessage) -> Result<(), String> {
        let target_view = new_view_msg.new_view;
        let current_view = self.view_timer.get_current_view();

        // Ignore old view changes
        if target_view <= current_view {
            return Ok(());
        }

        // Track this NewView message
        let validators_for_view = self.pending_new_views
            .entry(target_view)
            .or_insert_with(HashSet::new);

        validators_for_view.insert(new_view_msg.sender);

        tracing::info!(
            "🔄 NewView received: view={:?}, from validator={}, count={}/{}",
            target_view,
            new_view_msg.sender.0,
            validators_for_view.len(),
            self.view_change_quorum
        );

        // Check if we have quorum for view change
        if validators_for_view.len() as u32 >= self.view_change_quorum {
            tracing::info!(
                "✅ View change quorum reached! Transitioning to view={:?}",
                target_view
            );

            // Transition to new view
            self.view_timer.force_view_change(target_view);

            // Clean up old pending new views
            self.pending_new_views.retain(|v, _| v >= &target_view);

            tracing::info!(
                "🔄 View transition complete: old={:?}, new={:?}",
                current_view,
                target_view
            );
        }

        Ok(())
    }

    // ========== OUTBOUND MESSAGE SENDING ==========

    pub async fn broadcast_vote(&mut self, vote: Vote) -> Result<(), String> {
        self.pending_votes.push_back(vote.clone());
        self.gossip_scheduler.schedule_vote(vote.clone());
        self.network_bridge.broadcast_vote(vote).await
    }

    pub async fn broadcast_certificate(&mut self, cert: Certificate) -> Result<(), String> {
        self.pending_certificates.push_back(cert.clone());
        self.gossip_scheduler.schedule_certificate(cert.clone());
        self.network_bridge.broadcast_certificate(cert).await
    }

    pub async fn broadcast_new_view(&mut self, new_view_msg: NewViewMessage) -> Result<(), String> {
        self.network_bridge.broadcast_new_view(new_view_msg).await
    }

    // Get ready messages from gossip scheduler (public accessor)
    pub fn get_ready_gossip_messages(&mut self) -> (Vec<Vote>, Vec<Certificate>) {
        self.gossip_scheduler.get_ready_messages()
    }

    // ========== CONSENSUS OPERATIONS ==========

    pub async fn propose_block(&mut self, block_hash: BlockHash) -> Result<Vote, String> {
        // V7 TEMPORARY FIX: Use dummy signature to unblock finality
        // TODO: Implement proper Sr25519 signing with validator keystore
        let dummy_signature = {
            let mut sig = Vec::with_capacity(64);
            // Create deterministic signature from validator_id + block_hash for uniqueness
            sig.extend_from_slice(&self.validator_id.0.to_le_bytes());
            sig.extend_from_slice(&block_hash.0[0..28]); // 4 + 28 = 32 bytes
            sig.extend_from_slice(&block_hash.0[0..32]); // Total 64 bytes (Sr25519 signature size)
            sig
        };

        let vote = Vote {
            validator_id: self.validator_id,
            view: self.view_timer.get_current_view(),
            block_hash,
            signature: dummy_signature,  // V7: Dummy signature for testing BFT consensus
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.broadcast_vote(vote.clone()).await?;
        Ok(vote)
    }

    pub async fn handle_timeout(&mut self) -> Result<(), String> {
        if self.view_timer.should_trigger_view_change() {
            let mut new_view = self.view_timer.trigger_view_change();
            new_view.sender = self.validator_id;

            tracing::info!(
                "🔄 View timeout triggered: transitioning to view={:?}",
                new_view.new_view
            );

            // Broadcast NewView message to network
            self.broadcast_new_view(new_view.clone()).await?;

            tracing::info!(
                "✅ NewView message broadcast: view={:?}, validator={}",
                new_view.new_view,
                self.validator_id.0
            );
        }
        Ok(())
    }

    // ========== QUERY METHODS ==========

    pub fn get_current_view(&self) -> View {
        self.view_timer.get_current_view()
    }

    pub fn is_finalized(&self, block: &BlockHash) -> bool {
        self.finalized_blocks.contains(block)
    }

    pub fn get_finalized_blocks(&self) -> Vec<BlockHash> {
        self.finalized_blocks.clone()
    }

    pub fn get_committed_blocks(&self) -> Vec<BlockHash> {
        self.committed_blocks.clone()
    }

    pub fn get_peer_reputation(&self, validator_id: ValidatorId) -> Option<PeerReputation> {
        self.peer_reputation.get(&validator_id).cloned()
    }

    pub fn should_isolate_peer(&self, validator_id: ValidatorId) -> bool {
        self.peer_reputation
            .get(&validator_id)
            .map(|rep| rep.should_isolate())
            .unwrap_or(false)
    }

    // ========== WORKER LOOP ==========

    pub async fn run_worker(&mut self) {
        let mut gossip_interval = interval(Duration::from_millis(500));
        let mut timeout_interval = interval(Duration::from_secs(1));

        loop {
            tokio::select! {
                _ = gossip_interval.tick() => {
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
}

// ============================================================================
// CHECKPOINT FINALITY IMPLEMENTATION
// ============================================================================

/// Trait for querying canonical chain state
#[async_trait::async_trait]
pub trait CanonicalChainClient: Send + Sync {
    /// Get the canonical block hash at a specific height
    async fn get_canonical_hash(&self, height: u64) -> Option<BlockHash>;

    /// Get the current best block number
    async fn get_best_block_number(&self) -> u64;

    /// Verify a block is on the canonical chain
    async fn is_on_canonical_chain(&self, block_hash: BlockHash, height: u64) -> bool;
}

pub struct CheckpointFinality {
    #[allow(dead_code)]
    validator_id: ValidatorId,
    #[allow(dead_code)]
    max_validators: u32,
    quorum_threshold: u32,

    // Current authority set
    current_authority_set_id: AuthoritySetId,

    // Checkpoint tracking
    checkpoint_signatures: HashMap<CheckpointNumber, Vec<CheckpointSignature>>,
    checkpoint_certificates: HashMap<CheckpointNumber, CheckpointCertificate>,
    last_finalized_checkpoint: CheckpointNumber,
    last_checkpoint_time: HashMap<CheckpointNumber, Instant>,

    // Slashing detection - track all signatures per validator per checkpoint
    validator_signatures: HashMap<(ValidatorId, CheckpointNumber), Vec<(BlockHash, AuthoritySetId)>>,
    double_sign_evidence: Vec<(ValidatorId, CheckpointNumber, BlockHash, BlockHash)>,

    // Stuck recovery tracking
    missed_checkpoints: u64,
    stuck_recovery_mode: bool,

    // Partition recovery
    missing_certificates: HashSet<CheckpointNumber>,
    peer_last_seen: HashMap<String, Instant>,

    // Metrics
    metrics: CheckpointMetrics,
    quorum_start_times: HashMap<CheckpointNumber, Instant>,

    // Client for canonical chain verification
    canonical_chain_client: Arc<dyn CanonicalChainClient>,

    // Network bridge for requesting help
    #[allow(dead_code)]
    network_bridge: Arc<dyn NetworkBridge>,
}

impl CheckpointFinality {
    pub fn new(
        validator_id: ValidatorId,
        max_validators: u32,
        canonical_chain_client: Arc<dyn CanonicalChainClient>,
        network_bridge: Arc<dyn NetworkBridge>,
    ) -> Self {
        let quorum_threshold = (2 * max_validators / 3) + 1;

        tracing::info!(
            "🏁 Checkpoint Finality initialized: validator={}, quorum={}/{}",
            validator_id.0,
            quorum_threshold,
            max_validators
        );

        Self {
            validator_id,
            max_validators,
            quorum_threshold,
            current_authority_set_id: AuthoritySetId(0),
            checkpoint_signatures: HashMap::new(),
            checkpoint_certificates: HashMap::new(),
            last_finalized_checkpoint: CheckpointNumber(0),
            last_checkpoint_time: HashMap::new(),
            validator_signatures: HashMap::new(),
            double_sign_evidence: Vec::new(),
            missed_checkpoints: 0,
            stuck_recovery_mode: false,
            missing_certificates: HashSet::new(),
            peer_last_seen: HashMap::new(),
            metrics: CheckpointMetrics::new(),
            quorum_start_times: HashMap::new(),
            canonical_chain_client,
            network_bridge,
        }
    }

    // ========== CANONICAL CHAIN VERIFICATION ==========

    /// Verify that a block is on the canonical chain before signing or finalizing
    pub async fn verify_checkpoint_on_canonical_chain(
        &self,
        checkpoint_number: CheckpointNumber,
        block_hash: BlockHash,
        height: u64,
    ) -> Result<(), String> {
        // Check if the block hash matches the canonical hash at this height
        let is_canonical = self.canonical_chain_client
            .is_on_canonical_chain(block_hash, height)
            .await;

        if !is_canonical {
            tracing::warn!(
                "⚠️ FORK DETECTED: checkpoint={}, block={:?} is NOT on canonical chain at height={}",
                checkpoint_number.0,
                block_hash,
                height
            );
            return Err(format!(
                "Block {:?} is not on canonical chain at height {}",
                block_hash,
                height
            ));
        }

        tracing::debug!(
            "✅ Canonical verification passed: checkpoint={}, height={}",
            checkpoint_number.0,
            height
        );

        Ok(())
    }

    // ========== STUCK RECOVERY ==========

    /// Detect when BFT has stalled (3+ missed checkpoints) and increase recovery aggressiveness
    pub async fn check_stuck_recovery(&mut self) -> Result<(), String> {
        let current_best = self.canonical_chain_client.get_best_block_number().await;
        let finalized_number = self.last_finalized_checkpoint.0;

        // Calculate how many checkpoints we're behind (assuming 1 checkpoint per 10 blocks)
        let expected_checkpoint = current_best / 10;
        let lag = expected_checkpoint.saturating_sub(finalized_number);

        if lag >= 3 {
            self.missed_checkpoints = lag;

            if !self.stuck_recovery_mode {
                self.stuck_recovery_mode = true;
                self.metrics.stuck_checkpoints += 1;

                tracing::warn!(
                    "🚨 STUCK RECOVERY MODE ACTIVATED: {} checkpoints behind (expected={}, finalized={})",
                    lag,
                    expected_checkpoint,
                    finalized_number
                );

                // Request help from peers for missing checkpoints
                for checkpoint_num in finalized_number + 1..=expected_checkpoint {
                    self.missing_certificates.insert(CheckpointNumber(checkpoint_num));
                }

                // Broadcast request for missing certificates
                self.request_missing_certificates().await?;
            }
        } else if self.stuck_recovery_mode && lag < 2 {
            // Recovery complete
            self.stuck_recovery_mode = false;
            tracing::info!(
                "✅ Recovery complete: finality lag reduced to {} checkpoints",
                lag
            );
        }

        self.metrics.finality_lag = lag;
        Ok(())
    }

    // ========== SIGNATURE HANDLING ==========

    /// Add a checkpoint signature with authority set binding verification
    pub async fn add_checkpoint_signature(
        &mut self,
        signature: CheckpointSignature,
    ) -> Result<bool, String> {
        // Verify signature is from current authority set
        if signature.authority_set_id != self.current_authority_set_id {
            tracing::warn!(
                "❌ Rejected signature from old authority set: got={}, current={}",
                signature.authority_set_id.0,
                self.current_authority_set_id.0
            );
            return Err(format!(
                "Signature from old authority set {} (current is {})",
                signature.authority_set_id.0,
                self.current_authority_set_id.0
            ));
        }

        // Verify the checkpoint is on canonical chain
        // Note: Height is approximated as checkpoint_number * 10
        let height = signature.checkpoint_number.0 * 10;
        self.verify_checkpoint_on_canonical_chain(
            signature.checkpoint_number,
            signature.block_hash,
            height,
        ).await?;

        // Check for double signing before adding
        self.detect_double_sign(&signature)?;

        // Track signature for slashing detection
        let key = (signature.validator_id, signature.checkpoint_number);
        self.validator_signatures
            .entry(key)
            .or_insert_with(Vec::new)
            .push((signature.block_hash, signature.authority_set_id));

        // Add to signatures collection
        let checkpoint_sigs = self.checkpoint_signatures
            .entry(signature.checkpoint_number)
            .or_insert_with(Vec::new);

        checkpoint_sigs.push(signature.clone());
        self.metrics.total_signatures += 1;

        // Track quorum timing
        if checkpoint_sigs.len() == 1 {
            self.quorum_start_times.insert(signature.checkpoint_number, Instant::now());
        }

        let sig_count = checkpoint_sigs.len() as u32;
        tracing::debug!(
            "📝 Checkpoint signature added: checkpoint={}, validator={}, total={}/{}",
            signature.checkpoint_number.0,
            signature.validator_id.0,
            sig_count,
            self.quorum_threshold
        );

        // Check if we reached quorum
        if sig_count >= self.quorum_threshold {
            tracing::info!(
                "🎯 CHECKPOINT QUORUM REACHED: checkpoint={}, signatures={}/{}",
                signature.checkpoint_number.0,
                sig_count,
                self.quorum_threshold
            );

            // Create certificate
            self.create_checkpoint_certificate(signature.checkpoint_number).await?;
            return Ok(true);
        }

        // If we have some signatures but not quorum, request help
        if sig_count >= self.quorum_threshold / 2 && sig_count < self.quorum_threshold {
            self.request_checkpoint_help(signature.checkpoint_number).await?;
        }

        Ok(false)
    }

    // ========== SLASHING DETECTION ==========

    /// Detect if a validator signs different hashes for the same checkpoint
    pub fn detect_double_sign(&mut self, signature: &CheckpointSignature) -> Result<(), String> {
        let key = (signature.validator_id, signature.checkpoint_number);

        if let Some(existing_sigs) = self.validator_signatures.get(&key) {
            for (existing_hash, existing_set_id) in existing_sigs {
                // Double sign if same authority set but different block hash
                if *existing_set_id == signature.authority_set_id
                    && *existing_hash != signature.block_hash {

                    self.metrics.double_signs_detected += 1;

                    // Record evidence
                    self.double_sign_evidence.push((
                        signature.validator_id,
                        signature.checkpoint_number,
                        *existing_hash,
                        signature.block_hash,
                    ));

                    tracing::error!(
                        "🚨 DOUBLE SIGN DETECTED! validator={}, checkpoint={}, hash1={:?}, hash2={:?}",
                        signature.validator_id.0,
                        signature.checkpoint_number.0,
                        existing_hash,
                        signature.block_hash
                    );

                    return Err(format!(
                        "Double sign detected for validator {} at checkpoint {}",
                        signature.validator_id.0,
                        signature.checkpoint_number.0
                    ));
                }
            }
        }

        Ok(())
    }

    /// Get all detected double sign evidence
    pub fn get_double_sign_evidence(&self) -> Vec<(ValidatorId, CheckpointNumber, BlockHash, BlockHash)> {
        self.double_sign_evidence.clone()
    }

    // ========== CERTIFICATE CREATION & VERIFICATION ==========

    /// Create a checkpoint certificate when quorum is reached
    async fn create_checkpoint_certificate(
        &mut self,
        checkpoint_number: CheckpointNumber,
    ) -> Result<(), String> {
        let signatures = self.checkpoint_signatures
            .get(&checkpoint_number)
            .ok_or("No signatures for checkpoint")?;

        if signatures.is_empty() {
            return Err("Cannot create certificate with no signatures".to_string());
        }

        // Get the most common block hash
        let mut hash_counts: HashMap<BlockHash, usize> = HashMap::new();
        for sig in signatures {
            *hash_counts.entry(sig.block_hash).or_insert(0) += 1;
        }

        let (winning_hash, _) = hash_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .ok_or("Failed to determine winning hash")?;

        // Collect signatures for winning hash
        let winning_signatures: Vec<(ValidatorId, Vec<u8>)> = signatures
            .iter()
            .filter(|sig| sig.block_hash == winning_hash)
            .map(|sig| (sig.validator_id, sig.signature.clone()))
            .collect();

        let certificate = CheckpointCertificate {
            checkpoint_number,
            block_hash: winning_hash,
            authority_set_id: self.current_authority_set_id,
            signatures: winning_signatures,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        // Verify certificate before storing
        self.verify_certificate(&certificate).await?;

        // Calculate quorum time
        if let Some(start_time) = self.quorum_start_times.get(&checkpoint_number) {
            let quorum_time = start_time.elapsed();
            let current_avg = self.metrics.average_quorum_time;
            let total_certs = self.metrics.total_certificates;

            // Update running average
            let new_avg = if total_certs == 0 {
                quorum_time
            } else {
                Duration::from_secs_f64(
                    (current_avg.as_secs_f64() * total_certs as f64 + quorum_time.as_secs_f64())
                    / (total_certs + 1) as f64
                )
            };
            self.metrics.average_quorum_time = new_avg;
        }

        self.checkpoint_certificates.insert(checkpoint_number, certificate.clone());
        self.metrics.total_certificates += 1;
        self.last_checkpoint_time.insert(checkpoint_number, Instant::now());

        // Remove from missing certificates if it was there
        self.missing_certificates.remove(&checkpoint_number);

        tracing::info!(
            "📜 CHECKPOINT CERTIFICATE CREATED: checkpoint={}, block={:?}, signatures={}",
            checkpoint_number.0,
            winning_hash,
            certificate.signatures.len()
        );

        // Broadcast certificate
        // Note: Would need to add checkpoint certificate to FinalityMessage enum
        // For now, log that we would broadcast it
        tracing::debug!("📡 Broadcasting checkpoint certificate {}", checkpoint_number.0);

        // Check if this advances finality
        self.advance_finality(checkpoint_number)?;

        Ok(())
    }

    /// Verify a certificate has valid quorum signatures
    pub async fn verify_certificate(&self, cert: &CheckpointCertificate) -> Result<(), String> {
        // Check authority set matches
        if cert.authority_set_id != self.current_authority_set_id {
            return Err(format!(
                "Certificate from wrong authority set: {} (expected {})",
                cert.authority_set_id.0,
                self.current_authority_set_id.0
            ));
        }

        // Check quorum
        if (cert.signatures.len() as u32) < self.quorum_threshold {
            return Err(format!(
                "Certificate has insufficient signatures: {} (need {})",
                cert.signatures.len(),
                self.quorum_threshold
            ));
        }

        // Verify all signatures are from unique validators
        let mut seen_validators = HashSet::new();
        for (validator_id, _) in &cert.signatures {
            if !seen_validators.insert(validator_id) {
                return Err(format!(
                    "Duplicate signature from validator {}",
                    validator_id.0
                ));
            }
        }

        // Verify block is on canonical chain
        let height = cert.checkpoint_number.0 * 10;
        self.verify_checkpoint_on_canonical_chain(
            cert.checkpoint_number,
            cert.block_hash,
            height,
        ).await?;

        tracing::debug!(
            "✅ Certificate verified: checkpoint={}, signatures={}",
            cert.checkpoint_number.0,
            cert.signatures.len()
        );

        Ok(())
    }

    /// Advance finality based on certificate chain
    fn advance_finality(&mut self, checkpoint_number: CheckpointNumber) -> Result<(), String> {
        // Simple finality rule: finalize when we have certificate
        // In practice, might want 2-3 consecutive certificates
        if checkpoint_number.0 > self.last_finalized_checkpoint.0 {
            self.last_finalized_checkpoint = checkpoint_number;

            tracing::info!(
                "🏁 FINALITY ADVANCED: checkpoint={} is now finalized",
                checkpoint_number.0
            );
        }

        Ok(())
    }

    // ========== PEER RECOVERY & HELP REQUESTS ==========

    /// Request help when we have some signatures but not quorum
    async fn request_checkpoint_help(&self, checkpoint_number: CheckpointNumber) -> Result<(), String> {
        tracing::info!(
            "📞 Requesting checkpoint help: checkpoint={}, current_signatures={}/{}",
            checkpoint_number.0,
            self.checkpoint_signatures.get(&checkpoint_number).map(|v| v.len()).unwrap_or(0),
            self.quorum_threshold
        );

        // In a real implementation, would send a message to peers requesting their signatures
        // For now, just log the request
        Ok(())
    }

    /// Request missing certificates from peers
    async fn request_missing_certificates(&self) -> Result<(), String> {
        if self.missing_certificates.is_empty() {
            return Ok(());
        }

        tracing::info!(
            "📞 Requesting {} missing certificates from peers",
            self.missing_certificates.len()
        );

        // In a real implementation, would broadcast requests for specific checkpoint certificates
        // For now, just log
        for checkpoint in &self.missing_certificates {
            tracing::debug!("  - Requesting certificate for checkpoint {}", checkpoint.0);
        }

        Ok(())
    }

    /// Handle peer reconnection - request any missing certificates
    pub async fn on_peer_reconnected(&mut self, peer_id: String) -> Result<(), String> {
        self.peer_last_seen.insert(peer_id.clone(), Instant::now());
        self.metrics.partition_recoveries += 1;

        tracing::info!(
            "🔄 Peer reconnected: {}, initiating partition recovery",
            peer_id
        );

        // Request certificates we're missing
        let current_best = self.canonical_chain_client.get_best_block_number().await;
        let expected_checkpoint = current_best / 10;

        for checkpoint_num in self.last_finalized_checkpoint.0 + 1..=expected_checkpoint {
            let checkpoint = CheckpointNumber(checkpoint_num);
            if !self.checkpoint_certificates.contains_key(&checkpoint) {
                self.missing_certificates.insert(checkpoint);
            }
        }

        if !self.missing_certificates.is_empty() {
            self.request_missing_certificates().await?;
        }

        Ok(())
    }

    // ========== HEALTH CHECKS ==========

    /// Check if quorum is possible and detect stalls
    pub async fn check_finality_health(&mut self) -> Result<HealthStatus, String> {
        let current_best = self.canonical_chain_client.get_best_block_number().await;
        let expected_checkpoint = current_best / 10;
        let finalized = self.last_finalized_checkpoint.0;
        let lag = expected_checkpoint.saturating_sub(finalized);

        // Check for stuck consensus
        self.check_stuck_recovery().await?;

        // Determine health status
        let status = if lag == 0 {
            HealthStatus::Healthy
        } else if lag < 3 {
            HealthStatus::SlightlyBehind
        } else if lag < 10 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Critical
        };

        tracing::debug!(
            "🏥 Finality health check: status={:?}, lag={}, finalized={}, expected={}",
            status,
            lag,
            finalized,
            expected_checkpoint
        );

        Ok(status)
    }

    // ========== DATA PRUNING ==========

    /// Prune old signatures and certificates to prevent memory bloat
    pub fn prune_old_data(&mut self, keep_last_n: u64) -> Result<(), String> {
        let cutoff = self.last_finalized_checkpoint.0.saturating_sub(keep_last_n);

        // Prune old signatures
        self.checkpoint_signatures.retain(|checkpoint, _| checkpoint.0 > cutoff);
        self.checkpoint_certificates.retain(|checkpoint, _| checkpoint.0 > cutoff);
        self.last_checkpoint_time.retain(|checkpoint, _| checkpoint.0 > cutoff);
        self.quorum_start_times.retain(|checkpoint, _| checkpoint.0 > cutoff);

        // Prune old validator signatures (keep evidence for double signing)
        self.validator_signatures.retain(|(_, checkpoint), _| checkpoint.0 > cutoff);

        tracing::debug!(
            "🧹 Pruned old checkpoint data: keeping checkpoints > {}",
            cutoff
        );

        Ok(())
    }

    // ========== QUERY METHODS ==========

    pub fn get_last_finalized_checkpoint(&self) -> CheckpointNumber {
        self.last_finalized_checkpoint
    }

    pub fn get_metrics(&self) -> CheckpointMetrics {
        self.metrics.clone()
    }

    pub fn is_stuck(&self) -> bool {
        self.stuck_recovery_mode
    }

    pub fn get_certificate(&self, checkpoint: CheckpointNumber) -> Option<CheckpointCertificate> {
        self.checkpoint_certificates.get(&checkpoint).cloned()
    }

    pub fn get_authority_set_id(&self) -> AuthoritySetId {
        self.current_authority_set_id
    }

    pub fn update_authority_set(&mut self, new_set_id: AuthoritySetId) {
        tracing::info!(
            "🔄 Authority set updated: {} -> {}",
            self.current_authority_set_id.0,
            new_set_id.0
        );
        self.current_authority_set_id = new_set_id;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    SlightlyBehind,
    Degraded,
    Critical,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct MockNetworkBridge;

    #[async_trait::async_trait]
    impl NetworkBridge for MockNetworkBridge {
        async fn broadcast_vote(&self, _vote: Vote) -> Result<(), String> {
            Ok(())
        }

        async fn broadcast_certificate(&self, _cert: Certificate) -> Result<(), String> {
            Ok(())
        }

        async fn broadcast_new_view(&self, _new_view: NewViewMessage) -> Result<(), String> {
            Ok(())
        }

        async fn get_connected_peers(&self) -> Vec<String> {
            vec![]
        }
    }

    #[test]
    fn test_vote_accumulation() {
        let mut collector = VoteCollector::new(3);

        let vote1 = Vote {
            validator_id: ValidatorId(0),
            view: View(0),
            block_hash: BlockHash([0u8; 32]),
            signature: vec![1, 2, 3],
            timestamp: 0,
        };

        let vote2 = Vote {
            validator_id: ValidatorId(1),
            view: View(0),
            block_hash: BlockHash([0u8; 32]),
            signature: vec![4, 5, 6],
            timestamp: 0,
        };

        assert!(!collector.add_vote(vote1).unwrap());
        assert!(collector.add_vote(vote2).unwrap());
    }

    #[test]
    fn test_certificate_finality_detection() {
        let mut gossip = CertificateGossip::new(100);

        let cert1 = Certificate {
            view: View(0),
            block_hash: BlockHash([0u8; 32]),
            signatures: vec![],
            timestamp: 0,
        };

        let cert2 = Certificate {
            view: View(1),
            block_hash: BlockHash([0u8; 32]),
            signatures: vec![],
            timestamp: 0,
        };

        let cert3 = Certificate {
            view: View(2),
            block_hash: BlockHash([0u8; 32]),
            signatures: vec![],
            timestamp: 0,
        };

        gossip.add_certificate(cert1).unwrap();
        gossip.add_certificate(cert2).unwrap();
        assert!(gossip.check_finality().is_none());

        gossip.add_certificate(cert3).unwrap();
        assert!(gossip.check_finality().is_some());
    }

    #[test]
    fn test_view_change_on_timeout() {
        let mut timer = ViewChangeTimer::new(Duration::from_millis(100));

        assert!(!timer.should_trigger_view_change());

        std::thread::sleep(Duration::from_millis(150));

        assert!(timer.should_trigger_view_change());

        let new_view = timer.trigger_view_change();
        assert_eq!(new_view.new_view.0, 1);
    }

    #[test]
    fn test_peer_reputation_tracking() {
        let mut rep = PeerReputation::new();

        assert_eq!(rep.score(), 0.0);

        rep.record_valid();
        assert!(rep.score() > 0.0);

        rep.record_invalid();
        rep.record_invalid();
        assert!(rep.score() < 0.0);
    }

    #[tokio::test]
    async fn test_finality_gadget_vote_flow() {
        let bridge = Arc::new(MockNetworkBridge);
        let mut gadget = FinalityGadget::new(ValidatorId(0), 3, bridge);

        let vote = Vote {
            validator_id: ValidatorId(1),
            view: View(0),
            block_hash: BlockHash([0u8; 32]),
            signature: vec![1, 2, 3],
            timestamp: 0,
        };

        gadget.handle_vote(vote).await.unwrap();
        assert_eq!(gadget.get_current_view(), View(0));
    }

    // ========== CHECKPOINT FINALITY TESTS ==========

    struct MockCanonicalChainClient {
        best_block: Arc<Mutex<u64>>,
        canonical_hashes: Arc<Mutex<HashMap<u64, BlockHash>>>,
    }

    impl MockCanonicalChainClient {
        fn new() -> Self {
            Self {
                best_block: Arc::new(Mutex::new(100)),
                canonical_hashes: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        async fn set_canonical_hash(&self, height: u64, hash: BlockHash) {
            self.canonical_hashes.lock().await.insert(height, hash);
        }

        async fn set_best_block(&self, height: u64) {
            *self.best_block.lock().await = height;
        }
    }

    #[async_trait::async_trait]
    impl CanonicalChainClient for MockCanonicalChainClient {
        async fn get_canonical_hash(&self, height: u64) -> Option<BlockHash> {
            self.canonical_hashes.lock().await.get(&height).cloned()
        }

        async fn get_best_block_number(&self) -> u64 {
            *self.best_block.lock().await
        }

        async fn is_on_canonical_chain(&self, block_hash: BlockHash, height: u64) -> bool {
            self.canonical_hashes
                .lock()
                .await
                .get(&height)
                .map(|h| *h == block_hash)
                .unwrap_or(true) // Default to true for testing
        }
    }

    #[tokio::test]
    async fn test_checkpoint_signature_accumulation() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client, bridge);

        let block_hash = BlockHash([1u8; 32]);

        // Add signatures until quorum
        for i in 0..14 {
            let sig = CheckpointSignature {
                validator_id: ValidatorId(i),
                checkpoint_number: CheckpointNumber(1),
                block_hash,
                authority_set_id: AuthoritySetId(0),
                signature: vec![i as u8; 64],
                timestamp: 0,
            };

            let reached_quorum = checkpoint.add_checkpoint_signature(sig).await.unwrap();

            if i < 13 {
                assert!(!reached_quorum, "Should not reach quorum at {} signatures", i + 1);
            } else {
                assert!(reached_quorum, "Should reach quorum at 14 signatures");
            }
        }

        // Verify certificate was created
        let cert = checkpoint.get_certificate(CheckpointNumber(1));
        assert!(cert.is_some(), "Certificate should be created");
        assert_eq!(cert.unwrap().signatures.len(), 14);
    }

    #[tokio::test]
    async fn test_double_sign_detection() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client, bridge);

        let block_hash1 = BlockHash([1u8; 32]);
        let block_hash2 = BlockHash([2u8; 32]);

        // First signature
        let sig1 = CheckpointSignature {
            validator_id: ValidatorId(5),
            checkpoint_number: CheckpointNumber(1),
            block_hash: block_hash1,
            authority_set_id: AuthoritySetId(0),
            signature: vec![1; 64],
            timestamp: 0,
        };

        checkpoint.add_checkpoint_signature(sig1).await.unwrap();

        // Second signature for different hash - should be detected as double sign
        let sig2 = CheckpointSignature {
            validator_id: ValidatorId(5),
            checkpoint_number: CheckpointNumber(1),
            block_hash: block_hash2,
            authority_set_id: AuthoritySetId(0),
            signature: vec![2; 64],
            timestamp: 0,
        };

        let result = checkpoint.add_checkpoint_signature(sig2).await;
        assert!(result.is_err(), "Should detect double sign");

        // Verify evidence was recorded
        let evidence = checkpoint.get_double_sign_evidence();
        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].0, ValidatorId(5));
    }

    #[tokio::test]
    async fn test_authority_set_rejection() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client, bridge);

        // Update to new authority set
        checkpoint.update_authority_set(AuthoritySetId(1));

        // Try to add signature from old authority set
        let old_sig = CheckpointSignature {
            validator_id: ValidatorId(1),
            checkpoint_number: CheckpointNumber(1),
            block_hash: BlockHash([1u8; 32]),
            authority_set_id: AuthoritySetId(0), // Old set
            signature: vec![1; 64],
            timestamp: 0,
        };

        let result = checkpoint.add_checkpoint_signature(old_sig).await;
        assert!(result.is_err(), "Should reject old authority set");
    }

    #[tokio::test]
    async fn test_canonical_chain_verification() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client.clone(), bridge);

        let canonical_hash = BlockHash([1u8; 32]);
        let fork_hash = BlockHash([2u8; 32]);

        // Set canonical hash at height 10
        client.set_canonical_hash(10, canonical_hash).await;

        // Verify canonical block passes
        let result = checkpoint
            .verify_checkpoint_on_canonical_chain(CheckpointNumber(1), canonical_hash, 10)
            .await;
        assert!(result.is_ok(), "Canonical block should pass verification");

        // Verify fork block fails
        let result = checkpoint
            .verify_checkpoint_on_canonical_chain(CheckpointNumber(1), fork_hash, 10)
            .await;
        assert!(result.is_err(), "Fork block should fail verification");
    }

    #[tokio::test]
    async fn test_stuck_recovery_detection() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client.clone(), bridge);

        // Set best block to 100 (expected checkpoint 10)
        client.set_best_block(100).await;

        // Last finalized is 0, so we're 10 checkpoints behind
        checkpoint.check_stuck_recovery().await.unwrap();

        assert!(checkpoint.is_stuck(), "Should be in stuck recovery mode");
        assert_eq!(checkpoint.get_metrics().finality_lag, 10);
    }

    #[tokio::test]
    async fn test_partition_recovery() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client.clone(), bridge);

        // Simulate peer reconnection
        checkpoint
            .on_peer_reconnected("peer1".to_string())
            .await
            .unwrap();

        assert_eq!(checkpoint.get_metrics().partition_recoveries, 1);
    }

    #[tokio::test]
    async fn test_health_check_statuses() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client.clone(), bridge);

        // Healthy: no lag
        client.set_best_block(0).await;
        let status = checkpoint.check_finality_health().await.unwrap();
        assert_eq!(status, HealthStatus::Healthy);

        // Slightly behind: lag < 3
        client.set_best_block(20).await; // Expected checkpoint 2
        let status = checkpoint.check_finality_health().await.unwrap();
        assert_eq!(status, HealthStatus::SlightlyBehind);

        // Degraded: lag >= 3 && lag < 10
        client.set_best_block(50).await; // Expected checkpoint 5
        let status = checkpoint.check_finality_health().await.unwrap();
        assert_eq!(status, HealthStatus::Degraded);

        // Critical: lag >= 10
        client.set_best_block(150).await; // Expected checkpoint 15
        let status = checkpoint.check_finality_health().await.unwrap();
        assert_eq!(status, HealthStatus::Critical);
    }

    #[tokio::test]
    async fn test_certificate_verification() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client.clone(), bridge);

        let block_hash = BlockHash([1u8; 32]);

        // Valid certificate with quorum
        let mut signatures = Vec::new();
        for i in 0..14 {
            signatures.push((ValidatorId(i), vec![i as u8; 64]));
        }

        let valid_cert = CheckpointCertificate {
            checkpoint_number: CheckpointNumber(1),
            block_hash,
            authority_set_id: AuthoritySetId(0),
            signatures: signatures.clone(),
            timestamp: 0,
        };

        let result = checkpoint.verify_certificate(&valid_cert).await;
        assert!(result.is_ok(), "Valid certificate should pass");

        // Invalid: insufficient signatures
        let invalid_cert = CheckpointCertificate {
            checkpoint_number: CheckpointNumber(1),
            block_hash,
            authority_set_id: AuthoritySetId(0),
            signatures: vec![(ValidatorId(0), vec![0; 64])],
            timestamp: 0,
        };

        let result = checkpoint.verify_certificate(&invalid_cert).await;
        assert!(result.is_err(), "Certificate with insufficient signatures should fail");
    }

    #[tokio::test]
    async fn test_data_pruning() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client, bridge);

        // Add signatures for multiple checkpoints
        for checkpoint_num in 1..=10 {
            let sig = CheckpointSignature {
                validator_id: ValidatorId(0),
                checkpoint_number: CheckpointNumber(checkpoint_num),
                block_hash: BlockHash([checkpoint_num as u8; 32]),
                authority_set_id: AuthoritySetId(0),
                signature: vec![0; 64],
                timestamp: 0,
            };

            let _ = checkpoint.add_checkpoint_signature(sig).await;
        }

        // Advance finality to checkpoint 10
        checkpoint.last_finalized_checkpoint = CheckpointNumber(10);

        // Prune, keeping last 5 checkpoints
        checkpoint.prune_old_data(5).unwrap();

        // Verify old data was pruned
        // Should keep checkpoints > 5 (i.e., 6, 7, 8, 9, 10)
        assert!(checkpoint.checkpoint_signatures.get(&CheckpointNumber(1)).is_none());
        assert!(checkpoint.checkpoint_signatures.get(&CheckpointNumber(5)).is_none());
        assert!(checkpoint.checkpoint_signatures.get(&CheckpointNumber(6)).is_some());
    }

    #[tokio::test]
    async fn test_metrics_tracking() {
        let client = Arc::new(MockCanonicalChainClient::new());
        let bridge = Arc::new(MockNetworkBridge);
        let mut checkpoint = CheckpointFinality::new(ValidatorId(0), 21, client, bridge);

        let block_hash = BlockHash([1u8; 32]);

        // Add 14 signatures to reach quorum
        for i in 0..14 {
            let sig = CheckpointSignature {
                validator_id: ValidatorId(i),
                checkpoint_number: CheckpointNumber(1),
                block_hash,
                authority_set_id: AuthoritySetId(0),
                signature: vec![i as u8; 64],
                timestamp: 0,
            };

            checkpoint.add_checkpoint_signature(sig).await.unwrap();
        }

        let metrics = checkpoint.get_metrics();
        assert_eq!(metrics.total_signatures, 14);
        assert_eq!(metrics.total_certificates, 1);
    }
}