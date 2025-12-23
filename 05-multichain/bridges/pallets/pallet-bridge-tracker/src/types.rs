use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::RuntimeDebug;

/// Deposit record structure
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct DepositRecord<AccountId, Balance, BlockNumber> {
    /// ĒTRID recipient
    pub user: AccountId,
    /// Amount locked (in external currency)
    pub amount: Balance,
    /// Transaction hash on external chain
    pub external_tx_hash: H256,
    /// Block number on external chain
    pub external_block_number: u32,
    /// When recorded on ĒTRID
    pub timestamp: BlockNumber,
    /// Hash of bridge message
    pub bridge_message_hash: H256,
    /// Attestation complete
    pub verified: bool,
    /// External chain confirmations
    pub confirmations: u8,
    /// Deposit status
    pub status: DepositStatus,
}

/// Deposit status enum
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum DepositStatus {
    /// Waiting for confirmations
    Pending,
    /// Attestation complete
    Verified,
    /// Wrapped tokens minted
    Processed,
    /// Conflict detected
    Disputed,
    /// Transaction failed
    Reverted,
}

/// Withdrawal record structure
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct WithdrawalRecord<AccountId, Balance, BlockNumber> {
    /// User withdrawing
    pub user: AccountId,
    /// Amount to release
    pub amount: Balance,
    /// External chain address
    pub target_address: [u8; 32],
    /// Hash of external release tx
    pub external_tx_hash: Option<H256>,
    /// When initiated
    pub timestamp: BlockNumber,
    /// Tokens released on external chain
    pub released: bool,
    /// Withdrawal status
    pub status: WithdrawalStatus,
}

/// Withdrawal status enum
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum WithdrawalStatus {
    /// Awaiting multi-sig approval
    Pending,
    /// Multi-sig signed
    Approved,
    /// Tx submitted to external chain
    InProgress,
    /// Confirmed on external chain
    Completed,
    /// External tx failed
    Failed,
}

/// Attestation set for M-of-N verification
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct AttestationSet<AccountId> {
    /// Hash of bridge message
    pub message_hash: H256,
    /// Validators who signed
    pub signers: BoundedVec<AccountId, ConstU32<100>>,
    /// ECDSA signatures
    pub signatures: BoundedVec<Vec<u8>, ConstU32<100>>,
    /// When attestation started
    pub timestamp: u32,
    /// Threshold reached
    pub verified: bool,
}

/// Reconciliation report
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct ReconciliationReport<Balance> {
    /// Report timestamp
    pub timestamp: u64,
    /// What we think is locked
    pub internal_balance: Balance,
    /// What blockchain actually shows
    pub external_balance: Balance,
    /// Difference (should be 0)
    pub discrepancy: i128,
    /// Total wrapped tokens minted
    pub wrapped_supply: Balance,
    /// true if all matches
    pub balanced: bool,
    /// Status description
    pub status: ReconciliationStatus,
}

/// Reconciliation status
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum ReconciliationStatus {
    /// All balances match
    Healthy,
    /// Minor discrepancy, investigate
    Warning,
    /// Major mismatch, halt operations
    Critical,
}
