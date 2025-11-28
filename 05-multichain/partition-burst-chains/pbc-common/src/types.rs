//! Common type definitions for PBC runtimes

use crate::*;

/// Address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic<RuntimeCall> =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra<RuntimeCall>>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra<RuntimeCall> = (
    frame_system::CheckNonZeroSender<Runtime<RuntimeCall>>,
    frame_system::CheckSpecVersion<Runtime<RuntimeCall>>,
    frame_system::CheckTxVersion<Runtime<RuntimeCall>>,
    frame_system::CheckGenesis<Runtime<RuntimeCall>>,
    frame_system::CheckEra<Runtime<RuntimeCall>>,
    frame_system::CheckNonce<Runtime<RuntimeCall>>,
    frame_system::CheckWeight<Runtime<RuntimeCall>>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime<RuntimeCall>>,
);

/// Block type as expected by this runtime.
pub type Block<RuntimeCall> = generic::Block<Header, UncheckedExtrinsic<RuntimeCall>>;

/// A Block signed with a Justification.
pub type SignedBlock<RuntimeCall> = generic::SignedBlock<Block<RuntimeCall>>;

/// BlockId type as expected by this runtime.
pub type BlockId<RuntimeCall> = generic::BlockId<Block<RuntimeCall>>;

/// The payload being signed in transactions.
pub type SignedPayload<RuntimeCall> =
    generic::SignedPayload<RuntimeCall, SignedExtra<RuntimeCall>>;

/// Placeholder for Runtime type (will be defined by each PBC)
pub struct Runtime<RuntimeCall>(sp_std::marker::PhantomData<RuntimeCall>);
