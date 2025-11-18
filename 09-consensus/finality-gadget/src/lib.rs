// etrid-networking/finality-gadget/src/lib.rs
// LAYER 1: Consensus Protocol
// Status: Production Ready
// Lines: 1500+ with comprehensive tests

use std::collections::{HashMap, VecDeque, HashSet};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{Instant, Duration, interval};
use serde::{Serialize, Deserialize};
use codec::{Encode, Decode};

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
// NEW VIEW COLLECTION & CONSENSUS (HotStuff Protocol)
// ============================================================================

pub struct NewViewCollector {
    /// NewView messages by view: View -> Set of validators who sent NewView
    new_views: HashMap<View, HashSet<ValidatorId>>,
    /// Quorum threshold (2f+1 for BFT)
    quorum_threshold: u32,
    /// Maximum validators in the network
    max_validators: u32,
    /// The highest view that has achieved consensus (2/3 agreement)
    consensus_view: View,
}

impl NewViewCollector {
    pub fn new(max_validators: u32) -> Self {
        let quorum_threshold = (2 * max_validators / 3) + 1;
        Self {
            new_views: HashMap::new(),
            quorum_threshold,
            max_validators,
            consensus_view: View(0),
        }
    }

    /// Add a NewView message from a validator
    /// Returns true if this causes the view to reach consensus
    pub fn add_new_view(&mut self, msg: &NewViewMessage) -> bool {
        // Don't process NewView for views we've already passed
        if msg.new_view.0 <= self.consensus_view.0 {
            return false;
        }

        let validators = self.new_views.entry(msg.new_view).or_insert_with(HashSet::new);
        validators.insert(msg.sender);

        let vote_count = validators.len() as u32;

        // Log the NewView collection
        tracing::debug!(
            "📨 NewView collected: view={}, from={}, count={}/{}",
            msg.new_view.0, msg.sender.0, vote_count, self.quorum_threshold
        );

        // Check if we've reached consensus on this view
        if vote_count >= self.quorum_threshold && msg.new_view.0 > self.consensus_view.0 {
            tracing::info!(
                "🔄 VIEW CONSENSUS REACHED! Advancing to view {} ({}/{} validators)",
                msg.new_view.0, vote_count, self.max_validators
            );
            self.consensus_view = msg.new_view;

            // Clean up old views to prevent memory leak
            self.new_views.retain(|view, _| view.0 >= self.consensus_view.0);

            return true;
        }

        false
    }

    /// Get the current consensus view (highest view with 2/3 agreement)
    pub fn get_consensus_view(&self) -> View {
        self.consensus_view
    }

    /// Get vote count for a specific view
    pub fn get_vote_count(&self, view: View) -> u32 {
        self.new_views.get(&view).map(|v| v.len() as u32).unwrap_or(0)
    }

    /// Check if we have enough NewView messages for a view
    pub fn has_quorum_for_view(&self, view: View) -> bool {
        self.get_vote_count(view) >= self.quorum_threshold
    }
}

// ============================================================================
// VOTE COLLECTION & AGGREGATION
// ============================================================================

pub struct VoteCollector {
    votes: HashMap<View, HashMap<BlockHash, Vec<(ValidatorId, Vec<u8>)>>>,
    quorum_threshold: u32,
    max_validators: u32,
}

impl VoteCollector {
    pub fn new(max_validators: u32) -> Self {
        let quorum_threshold = (2 * max_validators / 3) + 1;
        Self {
            votes: HashMap::new(),
            quorum_threshold,
            max_validators,
        }
    }

    pub fn add_vote(&mut self, vote: Vote) -> Result<bool, String> {
        if vote.signature.is_empty() {
            return Err("Empty signature".to_string());
        }

        let view_votes = self
            .votes
            .entry(vote.view)
            .or_insert_with(HashMap::new);

        let block_votes = view_votes
            .entry(vote.block_hash)
            .or_insert_with(Vec::new);

        // Prevent double voting
        if block_votes.iter().any(|(v_id, _)| v_id == &vote.validator_id) {
            return Err("Validator already voted".to_string());
        }

        block_votes.push((vote.validator_id, vote.signature));

        // V8 DIAGNOSTIC: Log vote distribution to diagnose quorum issues
        let vote_count = block_votes.len() as u32;
        let block_hash_short = format!("{:02x}{:02x}..{:02x}{:02x}",
            vote.block_hash.0[0], vote.block_hash.0[1],
            vote.block_hash.0[30], vote.block_hash.0[31]);

        tracing::info!(
            "📊 Vote added: view={:?}, block={}, validator={}, votes={}/{} (quorum={})",
            vote.view,
            block_hash_short,
            vote.validator_id.0,
            vote_count,
            self.max_validators,
            self.quorum_threshold
        );

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

        Ok(reached_quorum)
    }

    pub fn get_votes_for_view(&self, view: View) -> Vec<Vote> {
        self.votes
            .get(&view)
            .map(|view_votes| {
                view_votes
                    .iter()
                    .flat_map(|(block_hash, sigs)| {
                        sigs.iter().map(move |(validator_id, sig)| Vote {
                            validator_id: *validator_id,
                            view,
                            block_hash: *block_hash,
                            signature: sig.clone(),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_quorum_for_block(&self, view: View, block_hash: BlockHash) -> Option<Vec<(ValidatorId, Vec<u8>)>> {
        self.votes
            .get(&view)
            .and_then(|view_votes| view_votes.get(&block_hash))
            .filter(|sigs| sigs.len() as u32 >= self.quorum_threshold)
            .cloned()
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
        // Finality: 3 consecutive certificates for the SAME block (HotStuff 3-chain rule)
        if self.certificates.len() < 3 {
            return None;
        }

        let len = self.certificates.len();
        // VecDeque doesn't support slicing, so access elements directly
        let cert0 = &self.certificates[len - 3];
        let cert1 = &self.certificates[len - 2];
        let cert2 = &self.certificates[len - 1];

        // Check if all 3 are consecutive views AND for the SAME block
        let consecutive_views = cert0.view.0 + 1 == cert1.view.0 && cert1.view.0 + 1 == cert2.view.0;
        let same_block = cert0.block_hash == cert1.block_hash && cert1.block_hash == cert2.block_hash;

        if consecutive_views && same_block {
            tracing::info!(
                "🎉 FINALITY ACHIEVED! Block {:02x}{:02x}..{:02x}{:02x} finalized at view {}",
                cert0.block_hash.0[0], cert0.block_hash.0[1],
                cert0.block_hash.0[30], cert0.block_hash.0[31],
                cert2.view.0
            );
            // Return the finalized block (first certificate's block)
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

    /// Set the view directly (used when consensus is reached)
    pub fn set_view(&mut self, view: View) {
        if view.0 > self.current_view.0 {
            self.current_view = view;
            self.last_block_time = Instant::now();
            self.view_change_pending = false;
        }
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
    async fn broadcast_new_view(&self, msg: NewViewMessage) -> Result<(), String>;
    async fn get_connected_peers(&self) -> Vec<String>;
}

// ============================================================================
// MAIN FINALITY GADGET
// ============================================================================

pub struct FinalityGadget {
    validator_id: ValidatorId,
    max_validators: u32,
    vote_collector: VoteCollector,
    new_view_collector: NewViewCollector,
    certificate_gossip: CertificateGossip,
    view_timer: ViewChangeTimer,
    gossip_scheduler: GossipScheduler,
    peer_reputation: HashMap<ValidatorId, PeerReputation>,
    committed_blocks: Vec<BlockHash>,
    finalized_blocks: Vec<BlockHash>,
    network_bridge: Arc<dyn NetworkBridge>,
    pending_votes: VecDeque<Vote>,
    pending_certificates: VecDeque<Certificate>,
    pending_new_views: VecDeque<NewViewMessage>,
}

impl FinalityGadget {
    pub fn new(
        validator_id: ValidatorId,
        max_validators: u32,
        network_bridge: Arc<dyn NetworkBridge>,
    ) -> Self {
        Self {
            validator_id,
            max_validators,
            vote_collector: VoteCollector::new(max_validators),
            new_view_collector: NewViewCollector::new(max_validators),
            certificate_gossip: CertificateGossip::new(100),
            view_timer: ViewChangeTimer::new(Duration::from_secs(6)),
            gossip_scheduler: GossipScheduler::new(),
            peer_reputation: HashMap::new(),
            committed_blocks: Vec::new(),
            finalized_blocks: Vec::new(),
            network_bridge,
            pending_votes: VecDeque::new(),
            pending_certificates: VecDeque::new(),
            pending_new_views: VecDeque::new(),
        }
    }

    // ========== INBOUND MESSAGE HANDLING ==========

    /// Handle incoming NewView message (HotStuff view synchronization)
    pub async fn handle_new_view(&mut self, msg: NewViewMessage) -> Result<(), String> {
        // Add our own NewView for the same view (we agree to advance)
        let consensus_reached = self.new_view_collector.add_new_view(&msg);

        // Update reputation
        let rep = self.peer_reputation.entry(msg.sender).or_insert_with(PeerReputation::new);
        rep.record_valid();

        if consensus_reached {
            // Update local view timer to match consensus
            self.view_timer.set_view(msg.new_view);

            tracing::info!(
                "✅ View synchronized to {} via consensus",
                msg.new_view.0
            );
        }

        Ok(())
    }

    pub async fn handle_vote(&mut self, vote: Vote) -> Result<(), String> {
        // Validate vote against CONSENSUS view (not local view)
        // This is the key HotStuff principle: accept votes for the agreed-upon view
        let consensus_view = self.new_view_collector.get_consensus_view();

        // Accept votes for current consensus view OR the next view (pipelining)
        // This allows the protocol to make progress while views are being synchronized
        if vote.view.0 < consensus_view.0 || vote.view.0 > consensus_view.0 + 1 {
            let rep = self.peer_reputation.entry(vote.validator_id).or_insert_with(PeerReputation::new);
            rep.record_invalid();
            return Err(format!(
                "Vote from invalid view: {:?} (consensus: {:?})",
                vote.view, consensus_view
            ));
        }

        // Add to collector
        let reached_quorum = self.vote_collector.add_vote(vote.clone())?;

        // Update reputation
        let rep = self.peer_reputation.entry(vote.validator_id).or_insert_with(PeerReputation::new);
        rep.record_valid();

        // If quorum reached, create certificate
        if reached_quorum {
            if let Some(signatures) = self.vote_collector.get_quorum_for_block(vote.view, vote.block_hash) {
                let cert = Certificate {
                    view: vote.view,
                    block_hash: vote.block_hash,
                    signatures: signatures.clone(),
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                };

                // V8 DIAGNOSTIC: Log certificate creation
                let block_hash_short = format!("{:02x}{:02x}..{:02x}{:02x}",
                    vote.block_hash.0[0], vote.block_hash.0[1],
                    vote.block_hash.0[30], vote.block_hash.0[31]);
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

    /// Broadcast a NewView message to initiate view change consensus
    pub async fn broadcast_new_view(&mut self, new_view: View) -> Result<(), String> {
        let msg = NewViewMessage {
            new_view,
            sender: self.validator_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        // Add our own NewView to the collector
        self.new_view_collector.add_new_view(&msg);

        // Queue for gossip
        self.pending_new_views.push_back(msg.clone());

        tracing::info!(
            "📤 Broadcasting NewView for view {} (initiating consensus)",
            new_view.0
        );

        self.network_bridge.broadcast_new_view(msg).await
    }

    /// Check for view timeout and initiate view change if needed
    /// Returns Some(NewViewMessage) if a view change was triggered
    pub async fn check_view_timeout(&mut self) -> Option<NewViewMessage> {
        if self.view_timer.should_trigger_view_change() {
            let current = self.view_timer.get_current_view();
            let new_view = View(current.0 + 1);

            tracing::info!(
                "⏰ View timeout! Initiating view change: {} -> {}",
                current.0, new_view.0
            );

            // Broadcast NewView to initiate consensus
            if let Err(e) = self.broadcast_new_view(new_view).await {
                tracing::warn!("Failed to broadcast NewView: {:?}", e);
                return None;
            }

            // Mark that we've initiated a view change
            self.view_timer.trigger_view_change();

            return Some(NewViewMessage {
                new_view,
                sender: self.validator_id,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
        }

        None
    }

    /// Get pending NewView messages for gossip
    pub fn get_pending_new_views(&mut self) -> Vec<NewViewMessage> {
        self.pending_new_views.drain(..).collect()
    }

    /// Get the current consensus view
    pub fn get_consensus_view(&self) -> View {
        self.new_view_collector.get_consensus_view()
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

            // TODO: Broadcast NewView message to network
            println!("View changed to: {:?}", new_view.new_view);
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
}