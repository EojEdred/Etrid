//! RPC extensions for FlareChain

use std::sync::Arc;

use flare_chain_runtime::{opaque::Block, AccountId, Balance, Nonce};
use jsonrpsee::RpcModule;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Full client dependencies
pub struct FullDeps<C, P> {
    /// The client instance to use
    pub client: Arc<C>,
    /// Transaction pool instance
    pub pool: Arc<P>,
    /// Enable ASF RPC endpoints
    pub enable_asf: bool,
}

/// Instantiate all full RPC extensions
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: pallet_validator_committee_runtime_api::ValidatorCommitteeApi<Block>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcModule::new(());
    let FullDeps { client, pool, enable_asf } = deps;

    // Standard Substrate RPC
    module.merge(System::new(client.clone(), pool).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    // ASF Consensus RPC (if enabled)
    if enable_asf {
        log::info!("🔌 Enabling ASF RPC endpoints");
        module.merge(crate::asf_rpc::create_asf_rpc(client.clone()))?;
    }

    Ok(module)
}