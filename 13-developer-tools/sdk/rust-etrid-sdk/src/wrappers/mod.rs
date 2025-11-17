//! # Ëtrid SDK Wrappers Module
//!
//! This module contains high-level wrappers for all Ëtrid Protocol features.
//!
//! ## Core Protocol Wrappers
//!
//! - `lightning_bloc`: Layer 2 payment channels for instant transactions
//! - `distribution_pay`: Rewards distribution for validators, directors, and developers
//! - `etwasm_vm`: WebAssembly smart contract deployment and execution
//! - `ai_did`: AI agent identity and reputation management
//! - `bridge`: Cross-chain asset bridging
//! - `oracle`: Price feeds and oracle data
//! - `reserve_vault`: Collateralized lending and borrowing
//! - `staking`: Token staking and validator nomination
//! - `governance`: Democratic proposal and voting system
//! - `accounts`: Account management and token transfers
//!
//! ## External Integration Wrappers
//!
//! - `ledger_hardware`: Ledger hardware wallet integration
//! - `hyperledger_bridge`: Hyperledger Fabric bridge integration
//! - `gpu_registry`: GPU resource registration and management
//! - `gpu_nft`: GPU-backed NFT operations
//! - `eth_pbc_precompiles`: Ethereum PBC precompiled contracts

// Core protocol wrappers
pub mod lightning_bloc;
pub mod distribution_pay;
pub mod etwasm_vm;
pub mod ai_did;
pub mod bridge;
pub mod oracle;
pub mod reserve_vault;
pub mod staking;
pub mod governance;
pub mod accounts;

// External integration wrappers
pub mod ledger_hardware;
pub mod hyperledger_bridge;
pub mod gpu_registry;
pub mod gpu_nft;
pub mod eth_pbc_precompiles;

// Re-export core wrapper types
pub use lightning_bloc::{
    LightningBlocWrapper, OpenChannelParams, PaymentParams, ChannelFeeEstimate
};

pub use distribution_pay::{
    DistributionPayWrapper, ClaimRewardParams, EligibilityCheck
};

pub use etwasm_vm::{
    EtwasmVmWrapper, DeployContractParams, CallContractParams, QueryContractParams
};

pub use ai_did::{
    AiDidWrapper, RegisterAiParams, UpdateReputationParams, PermissionParams
};

pub use bridge::{
    BridgeWrapper, BridgeParams, BridgeFeeEstimate
};

pub use oracle::{
    OracleWrapper, PriceSubmission, PriceSubscription
};

pub use reserve_vault::{
    ReserveVaultWrapper, CreateVaultParams, DepositParams, BorrowParams, RepayParams
};

pub use staking::{
    StakingWrapper, BondParams, UnbondParams, NominateParams, RewardDestination,
    ValidatorStatus, ConvictionLevel
};

pub use governance::{
    GovernanceWrapper, CreateProposalParams, VoteParams, DelegateParams,
    ExecuteProposalParams, VotingThreshold
};

pub use accounts::{
    AccountsWrapper, TransferParams, BatchTransferParams, AccountInfo, TransactionType
};

// Re-export external integration types
pub use ledger_hardware::{
    LedgerDevice, DeviceInfo, LedgerError, connect_ledger,
    get_addresses, sign_transaction, verify_address, get_public_key
};

pub use hyperledger_bridge::{
    FabricNetwork, HyperledgerBridgeError,
    connect_fabric_network, bridge_asset_to_fabric, bridge_asset_from_fabric,
    query_fabric_state, verify_fabric_proof
};
