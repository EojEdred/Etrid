//! RPC extensions for Primearc Core Chain

use std::sync::Arc;

use primearc_core_runtime::{opaque::Block, AccountId, Balance, Nonce};
use jsonrpsee::RpcModule;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

use crate::asf_rpc::AsfFinalityState;
pub use governance_runtime_api::GovernanceApi as GovernanceRuntimeApi;

/// Full client dependencies
pub struct FullDeps<C, P> {
    /// The client instance to use
    pub client: Arc<C>,
    /// Transaction pool instance
    pub pool: Arc<P>,
    /// Enable ASF RPC endpoints
    pub enable_asf: bool,
    /// Enable Governance RPC endpoints
    pub enable_governance: bool,
    /// ASF Finality state for ASF RPC (required if enable_asf is true)
    pub asf_finality_state: Option<Arc<AsfFinalityState>>,
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
    C::Api: GovernanceRuntimeApi<Block, AccountId>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcModule::new(());
    let FullDeps { client, pool, enable_asf, enable_governance, asf_finality_state } = deps;

    // Standard Substrate RPC
    module.merge(System::new(client.clone(), pool).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    // ASF Consensus RPC (if enabled and state provided)
    if enable_asf {
        if let Some(finality_state) = asf_finality_state {
            log::info!("🔌 ASF RPC endpoints enabled");
            let asf_rpc = crate::asf_rpc::create_asf_rpc::<C, Block>(client.clone(), finality_state)?;
            module.merge(asf_rpc)?;
        } else {
            log::warn!("⚠️ ASF RPC enabled but no finality state provided - using default state");
            let default_state = Arc::new(AsfFinalityState::new(6));
            let asf_rpc = crate::asf_rpc::create_asf_rpc::<C, Block>(client.clone(), default_state)?;
            module.merge(asf_rpc)?;
        }
    }

    // Governance RPC (if enabled)
    if enable_governance {
        log::info!("🗳️ Governance RPC endpoints enabled");
        let governance_rpc = crate::governance_rpc::create_governance_rpc::<C, Block, AccountId>(client.clone())?;
        module.merge(governance_rpc)?;
    }

    Ok(module)
}