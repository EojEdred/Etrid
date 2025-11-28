//! ASF Finality Indexer
//!
//! Tracks and indexes finality events for external queries.
//! Can be connected to a database for persistence.

use codec::{Decode, Encode};
use futures::StreamExt;
use sp_runtime::traits::{Block as BlockT, NumberFor};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

/// Finality event types for indexing
#[derive(Clone, Debug)]
pub enum FinalityEvent {
    /// Block imported
    BlockImported {
        hash: [u8; 32],
        number: u32,
        parent_hash: [u8; 32],
        timestamp: u64,
    },
    /// Vote received
    VoteReceived {
        view: u64,
        block_hash: [u8; 32],
        validator_id: [u8; 32],
        timestamp: u64,
    },
    /// Certificate created
    CertificateCreated {
        view: u64,
        block_hash: [u8; 32],
        block_number: u32,
        signature_count: u32,
        timestamp: u64,
    },
    /// Block finalized (3 certificates)
    BlockFinalized {
        hash: [u8; 32],
        number: u32,
        certificates: Vec<IndexedCertificate>,
        timestamp: u64,
    },
    /// Checkpoint created
    CheckpointCreated {
        block_number: u32,
        block_hash: [u8; 32],
        signature_count: u32,
        timestamp: u64,
    },
    /// Equivocation detected
    EquivocationDetected {
        validator_id: [u8; 32],
        view: u64,
        first_block: [u8; 32],
        second_block: [u8; 32],
        timestamp: u64,
    },
    /// Fork pruned
    ForkPruned {
        block_hashes: Vec<[u8; 32]>,
        timestamp: u64,
    },
    /// View changed
    ViewChanged {
        old_view: u64,
        new_view: u64,
        timestamp: u64,
    },
}

/// Indexed certificate data
#[derive(Clone, Debug, Encode, Decode)]
pub struct IndexedCertificate {
    pub view: u64,
    pub block_hash: [u8; 32],
    pub signature_count: u32,
    pub created_at: u64,
}

/// Indexed block finality data
#[derive(Clone, Debug)]
pub struct IndexedBlockFinality {
    pub hash: [u8; 32],
    pub number: u32,
    pub finality_state: String,
    pub certificates: Vec<IndexedCertificate>,
    pub finalized_at: Option<u64>,
    pub checkpoint: bool,
}

/// Configuration for the indexer
#[derive(Clone, Debug)]
pub struct IndexerConfig {
    /// Maximum blocks to keep in memory
    pub max_blocks_in_memory: usize,
    /// Whether to persist to database
    pub persist_to_db: bool,
    /// Database URL (if persisting)
    pub database_url: Option<String>,
    /// Log level
    pub log_level: String,
}

impl Default for IndexerConfig {
    fn default() -> Self {
        Self {
            max_blocks_in_memory: 10000,
            persist_to_db: false,
            database_url: None,
            log_level: "info".to_string(),
        }
    }
}

/// ASF Finality Indexer
pub struct AsfIndexer {
    config: IndexerConfig,
    /// Event receiver
    event_rx: mpsc::UnboundedReceiver<FinalityEvent>,
    /// Indexed blocks by hash
    blocks: Arc<RwLock<std::collections::HashMap<[u8; 32], IndexedBlockFinality>>>,
    /// Blocks by number (for range queries)
    blocks_by_number: Arc<RwLock<std::collections::BTreeMap<u32, [u8; 32]>>>,
    /// Certificates by block hash
    certificates: Arc<RwLock<std::collections::HashMap<[u8; 32], Vec<IndexedCertificate>>>>,
    /// Latest finalized
    latest_finalized: Arc<RwLock<Option<(u32, [u8; 32])>>>,
    /// Latest checkpoint
    latest_checkpoint: Arc<RwLock<Option<u32>>>,
    /// Equivocations
    equivocations: Arc<RwLock<Vec<IndexedEquivocation>>>,
    /// Statistics
    stats: Arc<IndexerStats>,
}

/// Indexed equivocation
#[derive(Clone, Debug)]
pub struct IndexedEquivocation {
    pub validator_id: [u8; 32],
    pub view: u64,
    pub first_block: [u8; 32],
    pub second_block: [u8; 32],
    pub detected_at: u64,
}

/// Indexer statistics
pub struct IndexerStats {
    pub blocks_indexed: std::sync::atomic::AtomicU64,
    pub certificates_indexed: std::sync::atomic::AtomicU64,
    pub votes_indexed: std::sync::atomic::AtomicU64,
    pub equivocations_indexed: std::sync::atomic::AtomicU64,
    pub checkpoints_indexed: std::sync::atomic::AtomicU32,
    pub current_view: std::sync::atomic::AtomicU64,
}

impl Default for IndexerStats {
    fn default() -> Self {
        Self {
            blocks_indexed: std::sync::atomic::AtomicU64::new(0),
            certificates_indexed: std::sync::atomic::AtomicU64::new(0),
            votes_indexed: std::sync::atomic::AtomicU64::new(0),
            equivocations_indexed: std::sync::atomic::AtomicU64::new(0),
            checkpoints_indexed: std::sync::atomic::AtomicU32::new(0),
            current_view: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

impl AsfIndexer {
    /// Create a new indexer
    pub fn new(
        config: IndexerConfig,
        event_rx: mpsc::UnboundedReceiver<FinalityEvent>,
    ) -> Self {
        Self {
            config,
            event_rx,
            blocks: Arc::new(RwLock::new(std::collections::HashMap::new())),
            blocks_by_number: Arc::new(RwLock::new(std::collections::BTreeMap::new())),
            certificates: Arc::new(RwLock::new(std::collections::HashMap::new())),
            latest_finalized: Arc::new(RwLock::new(None)),
            latest_checkpoint: Arc::new(RwLock::new(None)),
            equivocations: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(IndexerStats::default()),
        }
    }

    /// Create the event sender for the indexer
    pub fn create_event_channel() -> (mpsc::UnboundedSender<FinalityEvent>, mpsc::UnboundedReceiver<FinalityEvent>) {
        mpsc::unbounded_channel()
    }

    /// Run the indexer
    pub async fn run(mut self) {
        log::info!("📊 ASF Indexer started");

        while let Some(event) = self.event_rx.recv().await {
            self.process_event(event).await;
        }

        log::info!("📊 ASF Indexer stopped");
    }

    /// Process a finality event
    async fn process_event(&self, event: FinalityEvent) {
        match event {
            FinalityEvent::BlockImported { hash, number, parent_hash, timestamp } => {
                self.index_block(hash, number, parent_hash, timestamp).await;
            }
            FinalityEvent::VoteReceived { view, block_hash, validator_id, timestamp } => {
                self.index_vote(view, block_hash, validator_id, timestamp).await;
            }
            FinalityEvent::CertificateCreated { view, block_hash, block_number, signature_count, timestamp } => {
                self.index_certificate(view, block_hash, block_number, signature_count, timestamp).await;
            }
            FinalityEvent::BlockFinalized { hash, number, certificates, timestamp } => {
                self.index_finality(hash, number, certificates, timestamp).await;
            }
            FinalityEvent::CheckpointCreated { block_number, block_hash, signature_count, timestamp } => {
                self.index_checkpoint(block_number, block_hash, signature_count, timestamp).await;
            }
            FinalityEvent::EquivocationDetected { validator_id, view, first_block, second_block, timestamp } => {
                self.index_equivocation(validator_id, view, first_block, second_block, timestamp).await;
            }
            FinalityEvent::ForkPruned { block_hashes, timestamp } => {
                self.index_pruned_forks(block_hashes, timestamp).await;
            }
            FinalityEvent::ViewChanged { old_view, new_view, timestamp } => {
                self.index_view_change(old_view, new_view, timestamp).await;
            }
        }
    }

    async fn index_block(&self, hash: [u8; 32], number: u32, _parent_hash: [u8; 32], _timestamp: u64) {
        let block = IndexedBlockFinality {
            hash,
            number,
            finality_state: "Pending".to_string(),
            certificates: Vec::new(),
            finalized_at: None,
            checkpoint: false,
        };

        self.blocks.write().await.insert(hash, block);
        self.blocks_by_number.write().await.insert(number, hash);
        self.stats.blocks_indexed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        log::trace!("Indexed block #{} ({:?})", number, hex::encode(&hash[..8]));
    }

    async fn index_vote(&self, view: u64, _block_hash: [u8; 32], _validator_id: [u8; 32], _timestamp: u64) {
        self.stats.votes_indexed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.stats.current_view.store(view, std::sync::atomic::Ordering::SeqCst);
    }

    async fn index_certificate(&self, view: u64, block_hash: [u8; 32], block_number: u32, signature_count: u32, timestamp: u64) {
        let cert = IndexedCertificate {
            view,
            block_hash,
            signature_count,
            created_at: timestamp,
        };

        self.certificates.write().await
            .entry(block_hash)
            .or_insert_with(Vec::new)
            .push(cert);

        // Update block finality state
        if let Some(block) = self.blocks.write().await.get_mut(&block_hash) {
            let cert_count = self.certificates.read().await
                .get(&block_hash)
                .map(|c| c.len())
                .unwrap_or(0);

            block.finality_state = match cert_count {
                1 => "PreCommitted".to_string(),
                2 => "Committed".to_string(),
                _ => "Finalized".to_string(),
            };
        }

        self.stats.certificates_indexed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        log::debug!(
            "Indexed certificate for block #{} ({:?}) - {} signatures",
            block_number,
            hex::encode(&block_hash[..8]),
            signature_count
        );
    }

    async fn index_finality(&self, hash: [u8; 32], number: u32, certificates: Vec<IndexedCertificate>, timestamp: u64) {
        // Update block state
        if let Some(block) = self.blocks.write().await.get_mut(&hash) {
            block.finality_state = "Finalized".to_string();
            block.certificates = certificates;
            block.finalized_at = Some(timestamp);
        }

        *self.latest_finalized.write().await = Some((number, hash));

        log::info!(
            "📗 Indexed finality for block #{} ({:?})",
            number,
            hex::encode(&hash[..8])
        );
    }

    async fn index_checkpoint(&self, block_number: u32, block_hash: [u8; 32], signature_count: u32, _timestamp: u64) {
        if let Some(block) = self.blocks.write().await.get_mut(&block_hash) {
            block.finality_state = "Checkpointed".to_string();
            block.checkpoint = true;
        }

        *self.latest_checkpoint.write().await = Some(block_number);
        self.stats.checkpoints_indexed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        log::info!(
            "🏁 Indexed checkpoint at block #{} ({:?}) with {} signatures",
            block_number,
            hex::encode(&block_hash[..8]),
            signature_count
        );
    }

    async fn index_equivocation(&self, validator_id: [u8; 32], view: u64, first_block: [u8; 32], second_block: [u8; 32], timestamp: u64) {
        let equivocation = IndexedEquivocation {
            validator_id,
            view,
            first_block,
            second_block,
            detected_at: timestamp,
        };

        self.equivocations.write().await.push(equivocation);
        self.stats.equivocations_indexed.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        log::warn!(
            "⚠️ Indexed equivocation: validator {:?} in view {}",
            hex::encode(&validator_id[..8]),
            view
        );
    }

    async fn index_pruned_forks(&self, block_hashes: Vec<[u8; 32]>, _timestamp: u64) {
        let mut blocks = self.blocks.write().await;
        for hash in &block_hashes {
            blocks.remove(hash);
        }

        log::debug!("Pruned {} fork blocks from index", block_hashes.len());
    }

    async fn index_view_change(&self, _old_view: u64, new_view: u64, _timestamp: u64) {
        self.stats.current_view.store(new_view, std::sync::atomic::Ordering::SeqCst);
    }

    // Query methods

    /// Get block finality info
    pub async fn get_block(&self, hash: &[u8; 32]) -> Option<IndexedBlockFinality> {
        self.blocks.read().await.get(hash).cloned()
    }

    /// Get blocks in range
    pub async fn get_blocks_in_range(&self, start: u32, end: u32) -> Vec<IndexedBlockFinality> {
        let by_number = self.blocks_by_number.read().await;
        let blocks = self.blocks.read().await;

        by_number.range(start..=end)
            .filter_map(|(_, hash)| blocks.get(hash).cloned())
            .collect()
    }

    /// Get latest finalized
    pub async fn get_latest_finalized(&self) -> Option<(u32, [u8; 32])> {
        *self.latest_finalized.read().await
    }

    /// Get certificates for block
    pub async fn get_certificates(&self, hash: &[u8; 32]) -> Vec<IndexedCertificate> {
        self.certificates.read().await.get(hash).cloned().unwrap_or_default()
    }

    /// Get equivocations
    pub async fn get_equivocations(&self) -> Vec<IndexedEquivocation> {
        self.equivocations.read().await.clone()
    }

    /// Get stats
    pub fn get_stats(&self) -> &IndexerStats {
        &self.stats
    }
}

/// Create indexer with default config
pub fn create_indexer() -> (mpsc::UnboundedSender<FinalityEvent>, AsfIndexer) {
    let (tx, rx) = AsfIndexer::create_event_channel();
    let indexer = AsfIndexer::new(IndexerConfig::default(), rx);
    (tx, indexer)
}
