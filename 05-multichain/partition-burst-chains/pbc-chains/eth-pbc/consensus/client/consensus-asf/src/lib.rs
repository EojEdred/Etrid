//! # ASF Consensus Service for Substrate
//!
//! This crate provides the Substrate client-side implementation of ASF
//! (Adaptive Stake-weighted Finality) consensus, replacing AURA with
//! PPFA (Proposing Panel for Attestation) committee-based block production.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────┐
//! │          Substrate Service Layer                │
//! ├─────────────────────────────────────────────────┤
//! │  ┌──────────────┐        ┌──────────────┐     │
//! │  │ Import Queue │───────▶│ Block Import │     │
//! │  └──────────────┘        └──────────────┘     │
//! │         │                        │             │
//! │         ▼                        ▼             │
//! │  ┌──────────────┐        ┌──────────────┐     │
//! │  │   Verifier   │        │   Executor   │     │
//! │  └──────────────┘        └──────────────┘     │
//! │         │                        │             │
//! │         └────────┬───────────────┘             │
//! │                  ▼                             │
//! │          ┌──────────────┐                      │
//! │          │   Runtime    │                      │
//! │          │ (pallet-     │                      │
//! │          │  consensus)  │                      │
//! │          └──────────────┘                      │
//! │                                                 │
//! │  ┌──────────────────────────────────────────┐  │
//! │  │         ASF Worker (Block Authoring)     │  │
//! │  ├──────────────────────────────────────────┤  │
//! │  │  1. Query PPFA index from runtime       │  │
//! │  │  2. Check if we're current proposer     │  │
//! │  │  3. Wait for slot timing                │  │
//! │  │  4. Build block with transactions       │  │
//! │  │  5. Sign and propose                    │  │
//! │  │  6. Handle Ant blocks if needed         │  │
//! │  └──────────────────────────────────────────┘  │
//! └─────────────────────────────────────────────────┘
//! ```
//!
//! ## Components
//!
//! 1. **Block Verifier** (`verifier.rs`) - Validates blocks against ASF rules
//! 2. **Import Queue** (`import_queue.rs`) - Creates ASF-compatible block import queue
//! 3. **Block Authoring Worker** (`worker.rs`) - Background task for PPFA block production
//! 4. **Auxiliary Storage** (`aux_schema.rs`) - Persistent storage for consensus data
//! 5. **Inherent Data Providers** (`inherents.rs`) - Provides slot timing and proposer info
//!
//! ## Usage
//!
//! ```ignore
//! use sc_consensus_asf::{import_queue, start_asf};
//!
//! // Create import queue
//! let import_queue = import_queue(
//!     client.clone(),
//!     &task_manager.spawn_essential_handle(),
//!     config.prometheus_registry(),
//! )?;
//!
//! // Start ASF consensus worker
//! start_asf(StartAsfParams {
//!     client,
//!     proposer_factory,
//!     keystore,
//!     block_import,
//!     env,
//!     sync_oracle,
//!     justification_sync_link,
//!     create_inherent_data_providers,
//!     force_authoring,
//!     backoff_authoring_blocks,
//!     telemetry,
//! })?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

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

/// Errors that can occur in ASF consensus
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Client error: {0}")]
    Client(String),

    #[error("Runtime API error: {0}")]
    RuntimeApi(String),

    #[error("Block import error: {0}")]
    BlockImport(String),

    #[error("Invalid PPFA proposer: expected {expected}, got {got}")]
    InvalidProposer { expected: String, got: String },

    #[error("Invalid slot timing")]
    InvalidSlot,

    #[error("Fork choice error: {0}")]
    ForkChoice(String),

    #[error("Other error: {0}")]
    Other(String),
}

/// Consensus result type
pub type Result<T> = std::result::Result<T, Error>;

// ============================================================================
// PUBLIC API STUBS
// ============================================================================
// These will be fully implemented in future sessions

/*
/// Parameters for starting the ASF consensus worker
pub struct StartAsfParams<C, SC, E, I, SO, L, CIDP, BS> {
    /// Slot duration
    pub slot_duration: SlotDuration,
    /// Client
    pub client: Arc<C>,
    /// Select chain implementation
    pub select_chain: SC,
    /// Block import
    pub block_import: I,
    /// Environment for proposer
    pub env: E,
    /// Sync oracle
    pub sync_oracle: SO,
    /// Justification sync link
    pub justification_sync_link: L,
    /// Function to create inherent data providers
    pub create_inherent_data_providers: CIDP,
    /// Force authoring
    pub force_authoring: bool,
    /// Backoff on authoring blocks
    pub backoff_authoring_blocks: Option<BS>,
    /// Keystore
    pub keystore: KeystorePtr,
    /// Telemetry
    pub telemetry: Option<TelemetryHandle>,
}

/// Start the ASF consensus worker
pub async fn start_asf<B, C, SC, E, I, SO, L, CIDP, BS, Error>(
    params: StartAsfParams<C, SC, E, I, SO, L, CIDP, BS>,
) -> Result<()>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + AuxStore + Send + Sync + 'static,
    C::Api: AsfApi<B, AuthorityId>,
    SC: SelectChain<B> + 'static,
    E: Environment<B, Error = Error> + Send + Sync + 'static,
    E::Proposer: Proposer<B, Error = Error>,
    I: BlockImport<B, Error = Error> + Send + Sync + 'static,
    SO: SyncOracle + Send + Sync + Clone + 'static,
    L: JustificationSyncLink<B> + 'static,
    CIDP: CreateInherentDataProviders<B, ()> + Send + 'static,
    CIDP::InherentDataProviders: InherentDataProviderExt + Send,
    BS: BackoffAuthoringBlocksStrategy<NumberFor<B>> + Send + 'static,
    Error: std::error::Error + Send + From<ConsensusError> + 'static,
{
    // TODO: Implement in future session
    unimplemented!("ASF worker implementation pending")
}

/// Create an ASF import queue
pub fn import_queue<B, C, I>(
    client: Arc<C>,
    spawner: &impl SpawnEssentialNamed,
    prometheus_registry: Option<&Registry>,
) -> Result<BasicQueue<B>>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + BlockchainEvents<B> + HeaderBackend<B> + AuxStore + Send + Sync + 'static,
    C::Api: AsfApi<B, AuthorityId>,
    I: BlockImport<B> + Send + Sync + 'static,
{
    // TODO: Implement in future session
    unimplemented!("Import queue implementation pending")
}
*/

// ============================================================================
// IMPLEMENTATION ROADMAP
// ============================================================================

/// # Implementation Plan
///
/// ## Phase 1: Runtime API (COMPLETED)
/// - [x] Create `sp-consensus-asf` crate
/// - [x] Define `AsfApi` trait
/// - [x] Add inherent data types
/// - [ ] Add runtime API implementation to PBC runtimes
///
/// ## Phase 2: Block Verifier (TODO)
/// - [ ] Create `verifier.rs`
/// - [ ] Implement proposer verification
/// - [ ] Implement signature verification
/// - [ ] Implement timing verification
/// - [ ] Test verification logic
///
/// ## Phase 3: Import Queue (TODO)
/// - [ ] Create `import_queue.rs`
/// - [ ] Wire up verifier
/// - [ ] Wire up block import
/// - [ ] Test block import flow
///
/// ## Phase 4: Block Authoring Worker (TODO)
/// - [ ] Create `worker.rs`
/// - [ ] Implement proposer checking
/// - [ ] Implement slot timing
/// - [ ] Implement block building
/// - [ ] Implement signing
/// - [ ] Test authoring
///
/// ## Phase 5: Service Integration (TODO)
/// - [ ] Create `start_asf()` function
/// - [ ] Integrate with TaskManager
/// - [ ] Wire up keystore
/// - [ ] Wire up network
/// - [ ] Test full service
///
/// ## Phase 6: Collator Integration (TODO)
/// - [ ] Update btc-pbc-collator service.rs
/// - [ ] Replace AURA with ASF
/// - [ ] Test single collator
/// - [ ] Deploy to all 12 collators
/// - [ ] Network testing
///
/// ## Phase 7: Production Hardening (TODO)
/// - [ ] Error handling
/// - [ ] Logging and metrics
/// - [ ] Recovery mechanisms
/// - [ ] Performance optimization
/// - [ ] Documentation
mod _implementation_plan {}
