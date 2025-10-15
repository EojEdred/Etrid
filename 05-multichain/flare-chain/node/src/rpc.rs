//! RPC extensions for FlareChain

use std::sync::Arc;

use flare_chain_runtime::{opaque::Block, AccountId, Balance, Nonce};
use sc_rpc::SubscriptionTaskExecutor;
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
    /// Whether to deny unsafe calls
    pub deny_unsafe: sc_rpc::DenyUnsafe,
    /// Executor to spawn subscriptions
    pub subscription_executor: SubscriptionTaskExecutor,
}

/// Instantiate all full RPC extensions
pub fn create_full<C, P>(
    deps: FullDeps<C, P>,
) -> Result<jsonrpsee::RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut io = jsonrpsee::RpcModule::new(());
    let FullDeps {
        client,
        pool,
        deny_unsafe: _,
        subscription_executor: _,
    } = deps;

    io.merge(System::new(client.clone(), pool).into_rpc())?;
    io.merge(TransactionPayment::new(client).into_rpc())?;

    Ok(io)
}