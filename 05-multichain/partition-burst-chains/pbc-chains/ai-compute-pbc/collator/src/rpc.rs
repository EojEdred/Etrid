//! RPC interface for AI-Compute-PBC
//!
//! Exposes JSON-RPC methods for:
//! - System info (chain name, version, health)
//! - Transaction submission
//! - Block queries
//! - GPU registry queries
//! - Job marketplace queries
//! - Model registry queries

use ai_compute_pbc_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Hash, Nonce};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use std::sync::Arc;

/// Full RPC dependencies
pub struct FullDeps<C, P> {
    /// The client instance
    pub client: Arc<C>,
    /// Transaction pool instance
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: sc_rpc::DenyUnsafe,
}

/// Instantiate all RPC extensions
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<jsonrpsee::RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = jsonrpsee::RpcModule::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe,
    } = deps;

    // System RPC (system_name, system_version, system_health, etc.)
    module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;

    // TransactionPayment RPC (payment_queryInfo, payment_queryFeeDetails)
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    // TODO: Add custom RPC methods for AI Compute pallets
    // - gpu_listActiveGpus() -> Vec<GpuNode>
    // - gpu_getProviderReputation(account) -> Reputation
    // - jobs_listPending() -> Vec<Job>
    // - jobs_getJobStatus(job_id) -> JobStatus
    // - models_listModels() -> Vec<ModelMetadata>
    // - models_getModelByAidid(aidid) -> ModelMetadata

    Ok(module)
}

// Custom RPC methods (to be implemented)
/*
#[rpc]
pub trait AiComputeRpcApi {
    /// List all active GPU nodes
    #[method(name = "gpu_listActiveGpus")]
    fn list_active_gpus(&self) -> RpcResult<Vec<GpuNode>>;

    /// Get provider reputation
    #[method(name = "gpu_getReputation")]
    fn get_reputation(&self, account: AccountId) -> RpcResult<Reputation>;

    /// List pending jobs
    #[method(name = "jobs_listPending")]
    fn list_pending_jobs(&self) -> RpcResult<Vec<Job>>;

    /// Get job status
    #[method(name = "jobs_getStatus")]
    fn get_job_status(&self, job_id: u64) -> RpcResult<JobStatus>;

    /// List AI models
    #[method(name = "models_listModels")]
    fn list_models(&self) -> RpcResult<Vec<ModelMetadata>>;
}
*/
