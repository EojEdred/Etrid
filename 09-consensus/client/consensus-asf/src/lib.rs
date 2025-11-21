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
// PUBLIC API - IMPLEMENTED
// ============================================================================
// The import_queue and worker functions are fully implemented in their
// respective modules (import_queue.rs and worker.rs) and re-exported above.

// ============================================================================
// IMPLEMENTATION ROADMAP
// ============================================================================

/// # Implementation Plan
///
/// ## Phase 1: Runtime API (COMPLETED)
/// - [x] Create `sp-consensus-asf` crate
/// - [x] Define `AsfApi` trait
/// - [x] Add inherent data types
/// - [x] Add runtime API implementation to PBC runtimes
///
/// ## Phase 2: Block Verifier (COMPLETED)
/// - [x] Create `verifier.rs`
/// - [x] Implement proposer verification
/// - [x] Implement signature verification
/// - [x] Implement timing verification
/// - [x] Test verification logic
///
/// ## Phase 3: Import Queue (COMPLETED)
/// - [x] Create `import_queue.rs`
/// - [x] Wire up verifier
/// - [x] Wire up block import
/// - [x] Test block import flow
///
/// ## Phase 4: Block Authoring Worker (COMPLETED)
/// - [x] Create `worker.rs`
/// - [x] Implement proposer checking
/// - [x] Implement slot timing
/// - [x] Implement block building
/// - [x] Implement signing
/// - [x] Test authoring
///
/// ## Phase 5: Service Integration (IN PROGRESS)
/// - [x] Create import_queue and worker functions
/// - [x] Integrate with TaskManager
/// - [x] Wire up keystore
/// - [ ] Complete full service integration
/// - [ ] Test full service
///
/// ## Phase 6: Collator Integration (IN PROGRESS)
/// - [x] Update btc-pbc-collator service.rs
/// - [x] Replace AURA with ASF in all 12 PBC collators
/// - [x] Test single collator
/// - [ ] Deploy to all 12 collators
/// - [ ] Network testing
///
/// ## Phase 7: Production Hardening (IN PROGRESS)
/// - [x] Error handling
/// - [x] Logging and metrics
/// - [ ] Recovery mechanisms
/// - [ ] Performance optimization
/// - [ ] Documentation
mod _implementation_plan {}
