//! ASF Import Queue
//!
//! This module creates a Substrate block import queue configured for ASF consensus.
//! It integrates the ASF verifier with the standard block import pipeline.

use codec::Codec;
use sc_client_api::{backend::AuxStore, BlockBackend};
use sc_consensus::{
    import_queue::{BasicQueue, Verifier as VerifierT},
    BlockImport, BlockImportParams,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_consensus::Error as ConsensusError;
use sp_consensus_asf::AsfApi;
use sp_runtime::traits::Block as BlockT;
use std::{marker::PhantomData, sync::Arc};

use crate::{verifier::AsfVerifier, Result};

/// ASF consensus verifier for the import queue
///
/// Wraps the AsfVerifier to implement Substrate's Verifier trait
pub struct AsfImportQueueVerifier<B, C, AuthorityId> {
    verifier: AsfVerifier<B, C, AuthorityId>,
    _phantom: PhantomData<B>,
}

impl<B, C, AuthorityId> AsfImportQueueVerifier<B, C, AuthorityId>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + AuxStore,
    C::Api: AsfApi<B, AuthorityId>,
    AuthorityId: Codec + Clone,
{
    /// Create a new import queue verifier
    pub fn new(client: Arc<C>) -> Self {
        Self {
            verifier: AsfVerifier::new(client),
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<B, C, AuthorityId> VerifierT<B> for AsfImportQueueVerifier<B, C, AuthorityId>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + AuxStore + Send + Sync,
    C::Api: AsfApi<B, AuthorityId>,
    AuthorityId: Codec + Clone + Send + Sync,
{
    async fn verify(
        &self,
        block_params: BlockImportParams<B>,
    ) -> std::result::Result<BlockImportParams<B>, String> {
        self.verifier
            .verify(block_params)
            .map_err(|e| format!("ASF verification failed: {}", e))
    }
}

/// Create an ASF import queue
///
/// This function creates a basic import queue configured for ASF consensus.
/// It uses the AsfVerifier to validate blocks before importing them.
///
/// # Arguments
///
/// * `client` - Substrate client for runtime API calls
/// * `block_import` - Block import implementation (typically wrapped with GRANDPA)
/// * `spawner` - Task spawner for queue workers
/// * `registry` - Optional Prometheus registry for metrics
///
/// # Returns
///
/// A configured BasicQueue ready to receive blocks
pub fn import_queue<B, C, I, AuthorityId>(
    client: Arc<C>,
    block_import: I,
    spawner: &impl sp_core::traits::SpawnEssentialNamed,
    registry: Option<&substrate_prometheus_endpoint::Registry>,
) -> Result<BasicQueue<B>>
where
    B: BlockT,
    C: ProvideRuntimeApi<B>
        + HeaderBackend<B>
        + BlockBackend<B>
        + AuxStore
        + Send
        + Sync
        + 'static,
    C::Api: AsfApi<B, AuthorityId>,
    I: BlockImport<B, Error = ConsensusError> + Send + Sync + 'static,
    AuthorityId: Codec + Clone + Send + Sync + 'static,
{
    log::info!(
        target: "asf",
        "Creating ASF import queue"
    );

    let verifier = AsfImportQueueVerifier::new(client);

    Ok(BasicQueue::new(
        verifier,
        Box::new(block_import),
        None, // No justification import
        spawner,
        registry,
    ))
}

/// Parameters for creating an ASF import queue
///
/// This struct bundles all the parameters needed to create an import queue,
/// making it easier to pass around and configure.
pub struct ImportQueueParams<C, I> {
    /// Client for runtime API calls
    pub client: Arc<C>,
    /// Block import implementation
    pub block_import: I,
    /// Task spawner
    pub spawner: Box<dyn sp_core::traits::SpawnEssentialNamed>,
    /// Prometheus registry for metrics
    pub registry: Option<substrate_prometheus_endpoint::Registry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add unit tests for import queue
    // Tests should cover:
    // - Queue creation
    // - Block verification through queue
    // - Handling of invalid blocks
    // - Integration with block import
    // - Metrics collection
}
