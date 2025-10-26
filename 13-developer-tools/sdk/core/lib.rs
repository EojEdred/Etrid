//! # ËTRID SDK - Unified Developer-Friendly API
//!
//! This SDK provides a curated, stable API for external developers building on the ËTRID Multichain Protocol.
//!
//! ## Overview
//!
//! ËTRID is a next-generation blockchain platform implementing:
//! - **E³20 Architecture**: Essential Elements to Operate Reference Implementation
//! - **FODDoS ASF Consensus**: Flexible Orchestrated Distributed Defense of Service with Ascending Scale of Finality
//! - **Hybrid Multi-layer Design**: Custom P2P (DETR) + Substrate/Polkadot SDK framework
//! - **13 Independent Chains**: 1 FlareChain (root) + 12 Partition Burst Chains (PBC)
//! - **Layer 2 Networks**: Lightning Bloc Networks for instant micropayments
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! etrid-sdk = { version = "0.1.0", features = ["accounts", "governance"] }
//! ```
//!
//! ## Feature Flags
//!
//! The SDK uses feature flags to control which components are compiled, reducing build times:
//!
//! ### Individual Components
//! - **`accounts`** - Account management system
//! - **`governance`** - Foundation DAO governance
//! - **`consensus`** - FODDoS ASF consensus algorithm
//! - **`staking`** - Peer roles and staking (Flare Nodes, Validity Nodes, Directors)
//! - **`consensus-day`** - Annual governance event (December 1)
//! - **`currency`** - Native tokens (ËTR, ËTD, VMW)
//! - **`transactions`** - Transaction processing
//! - **`p2p`** - DETR P2P networking layer
//! - **`identity`** - OpenDID decentralized identity
//! - **`security`** - Cryptographic primitives
//! - **`vm`** - ETWASM smart contract execution
//! - **`multichain`** - FlareChain + PBC runtimes
//! - **`bridges`** - Cross-chain bridge protocols
//!
//! ### Convenience Bundles
//! - **`wallet`** - Everything needed for wallet applications (`accounts`, `currency`, `transactions`, `identity`)
//! - **`validator`** - Everything needed for validator nodes (`consensus`, `staking`, `p2p`, `multichain`)
//! - **`dao`** - Everything needed for governance (`governance`, `consensus-day`, `staking`)
//! - **`full`** - All features (large compile time)
//!
//! ## Examples
//!
//! ### Building a Wallet Application
//!
//! ```rust,ignore
//! use etrid_sdk::prelude::*;
//! use etrid_sdk::accounts::{Account, AccountInfo};
//! use etrid_sdk::currency::etr;
//! use etrid_sdk::transactions::types::Transaction;
//!
//! fn create_wallet() -> Account {
//!     // Create new account with ËTRID address
//!     Account::new(/* ... */)
//! }
//!
//! fn send_etr(from: &Account, to: &Account, amount: u128) -> Transaction {
//!     // Create and sign transaction
//!     Transaction::transfer(from, to, amount)
//! }
//! ```
//!
//! ### Participating in Governance
//!
//! ```rust,ignore
//! use etrid_sdk::consensus_day::{ProposalCategory, ProposalRecord};
//! use etrid_sdk::staking::{Role, StakeRequirement};
//!
//! fn submit_proposal(account: &Account, title: String, description: String) {
//!     // Check if account meets Director requirements (≥128 ËTR)
//!     let role = Role::DecentralizedDirector;
//!     let requirement = StakeRequirement::Director;
//!
//!     // Submit proposal for Consensus Day voting
//!     let proposal = ProposalRecord::new(
//!         account.clone(),
//!         ProposalCategory::EconomicAdjustment,
//!         title,
//!         description,
//!     );
//! }
//! ```
//!
//! ### Running a Validator Node
//!
//! ```rust,ignore
//! use etrid_sdk::consensus::Validator;
//! use etrid_sdk::staking::Role;
//!
//! fn start_flare_node(account: &Account, stake: u128) {
//!     // Register as Flare Node (root chain validator)
//!     let role = Role::FlareNode;
//!     Validator::register(account, stake, role);
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────┐
//! │                     ËTRID SDK                            │
//! │            Unified Developer-Friendly API                │
//! └────────────┬─────────────────────────────────────────────┘
//!              │
//!       ┌──────┴──────────────────────────────┐
//!       │                                     │
//! ┌─────▼──────┐                      ┌──────▼──────┐
//! │   Layer 1  │                      │   Layer 2   │
//! │ FlareChain │◄────────────────────►│  Lightning  │
//! │  (Root)    │   Cross-chain        │    Bloc     │
//! └─────┬──────┘   Communication      │  Networks   │
//!       │                              └─────────────┘
//!       │
//!       ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │              12 Partition Burst Chains (PBC)            │
//! ├─────────┬─────────┬─────────┬─────────┬─────────────────┤
//! │   BTC   │   ETH   │   SOL   │   XLM   │   ... (+8)      │
//! └─────────┴─────────┴─────────┴─────────┴─────────────────┘
//! ```
//!
//! ## Token Economics
//!
//! ### Native Tokens
//!
//! - **ËTR (Utility Token)**
//!   - Purpose: Staking, gas fees, governance
//!   - Staking tiers:
//!     - Directors: ≥ 128 ËTR
//!     - Validity Nodes: ≥ 64 ËTR
//!     - Common Stake Peers: ≥ 1 ËTR
//!
//! - **ËTD (Stablecoin)**
//!   - Purpose: Stable value transactions
//!   - Peg: USD 1:1
//!
//! - **VMW (Gas Token)**
//!   - Purpose: Smart contract execution
//!
//! ## Consensus Day
//!
//! Annual governance event held on **December 1st** where all stakeholders vote on:
//! - Protocol upgrades
//! - Economic parameter adjustments
//! - Director elections
//! - Treasury allocations
//! - Fiscal minting decisions
//!
//! ## Resources
//!
//! - **Website**: https://etrid.io
//! - **Documentation**: https://docs.etrid.io
//! - **GitHub**: https://github.com/etrid/etrid
//! - **Discord**: https://discord.gg/etrid
//!
//! ## License
//!
//! Apache-2.0

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://etrid.io/logo.png",
    html_favicon_url = "https://etrid.io/favicon.ico"
)]

// ═══════════════════════════════════════════════════════════════════════════════
// ACCOUNTS - Account Management (feature: accounts)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "accounts")]
/// Account management system
///
/// Provides types and functionality for managing ËTRID accounts, addresses, and permissions.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::accounts::{Account, AccountInfo};
///
/// let account = Account::new(/* ... */);
/// let info = AccountInfo::from(&account);
/// ```
pub mod accounts {
    #[doc(inline)]
    pub use account_types::*;

    #[cfg(feature = "accounts")]
    /// Account management pallet
    pub use pallet_accounts as pallet;
}

// ═══════════════════════════════════════════════════════════════════════════════
// GOVERNANCE - Foundation DAO (feature: governance)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "governance")]
/// Foundation DAO governance system
///
/// Provides on-chain governance for the ËTRID Foundation, including proposal submission,
/// voting, and execution.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::governance::pallet;
///
/// // Submit governance proposal
/// pallet::submit_proposal(origin, proposal);
/// ```
pub mod governance {
    #[doc(inline)]
    pub use pallet_governance as pallet;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSENSUS - FODDoS ASF Algorithm (feature: consensus)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "consensus")]
/// FODDoS ASF consensus mechanism
///
/// Flexible Orchestrated Distributed Defense of Service with Ascending Scale of Finality.
/// Custom consensus algorithm with three validator tiers.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::consensus::{Validator, asf};
///
/// let validator = Validator::new(/* ... */);
/// ```
pub mod consensus {
    #[cfg(feature = "consensus")]
    #[doc(inline)]
    /// ASF consensus algorithm implementation
    pub use asf_algorithm as asf;

    #[cfg(feature = "consensus")]
    #[doc(inline)]
    /// Block production logic
    pub use block_production;

    #[cfg(feature = "consensus")]
    #[doc(inline)]
    /// Finality gadget for block confirmation
    pub use finality_gadget;

    #[cfg(feature = "consensus")]
    #[doc(inline)]
    /// Consensus coordination pallet
    pub use pallet_consensus as pallet;

    #[cfg(feature = "consensus")]
    #[doc(inline)]
    /// Validator set management
    pub use validator_management;
}

// ═══════════════════════════════════════════════════════════════════════════════
// STAKING - Peer Roles & Staking (feature: staking)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "staking")]
/// Staking and peer role management
///
/// Manages staking requirements and role assignments for the three validator tiers:
/// - **Flare Nodes**: Root chain validators
/// - **Validity Nodes**: PBC validators (≥64 ËTR)
/// - **Decentralized Directors**: Governance participants (≥128 ËTR)
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::staking::{Role, StakeRequirement, RoleRecord};
///
/// let role = Role::ValidityNode;
/// let requirement = StakeRequirement::Validity; // 64 ËTR
/// ```
pub mod staking {
    #[doc(inline)]
    pub use peer_roles_staking_types as types;

    // Re-export commonly used types at module root
    #[doc(inline)]
    pub use types::{Role, StakeRequirement, RoleRecord, RoleEvent};

    #[cfg(feature = "staking")]
    #[doc(inline)]
    /// Staking pallet
    pub use pallet_peer_roles_staking as pallet;

    #[cfg(feature = "staking")]
    #[doc(inline)]
    /// Decentralized Directors (≥128 ËTR)
    pub use decentralized_directors;

    #[cfg(feature = "staking")]
    #[doc(inline)]
    /// Flare Nodes (root chain validators)
    pub use flare_nodes;

    #[cfg(feature = "staking")]
    #[doc(inline)]
    /// Validity Nodes (PBC validators, ≥64 ËTR)
    pub use validity_nodes;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSENSUS DAY - Annual Governance Event (feature: consensus-day)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "consensus-day")]
/// Consensus Day governance event
///
/// Annual governance event held on December 1st where all stakeholders vote on protocol changes,
/// economic parameters, director elections, and fiscal minting.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::consensus_day::{ProposalCategory, ProposalRecord};
///
/// let proposal = ProposalRecord {
///     category: ProposalCategory::EconomicAdjustment,
///     title: b"Adjust inflation rate".to_vec(),
///     description: b"Proposal to reduce annual inflation...".to_vec(),
///     // ...
/// };
/// ```
pub mod consensus_day {
    #[cfg(feature = "consensus-day")]
    #[doc(inline)]
    /// Proposal registration and management
    pub use consensus_day_proposal_system as proposals;

    // Re-export commonly used types
    #[doc(inline)]
    pub use proposals::{ProposalCategory, ProposalStatus, ProposalRecord};

    #[cfg(feature = "consensus-day")]
    #[doc(inline)]
    /// Voting protocol
    pub use consensus_day_voting_protocol as voting;

    #[cfg(feature = "consensus-day")]
    #[doc(inline)]
    /// Fiscal payout distribution
    pub use consensus_day_distribution as distribution;

    #[cfg(feature = "consensus-day")]
    #[doc(inline)]
    /// Token minting logic (post-vote)
    pub use consensus_day_minting_logic as minting;

    #[cfg(feature = "consensus-day")]
    #[doc(inline)]
    /// Public query interface
    pub use consensus_day_queries as queries;
}

// ═══════════════════════════════════════════════════════════════════════════════
// CURRENCY - Native Tokens (feature: currency)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "currency")]
/// Native currency system
///
/// Three native tokens:
/// - **ËTR**: Utility token for staking, gas, governance
/// - **ËTD**: Stablecoin pegged to USD
/// - **VMW**: Gas token for smart contract execution
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::currency::{etr, etd, vmw};
///
/// let balance_etr = etr::balance_of(&account);
/// ```
pub mod currency {
    #[cfg(feature = "currency")]
    #[doc(inline)]
    /// Economic model and parameters
    pub use currency_economics as economics;

    #[cfg(feature = "currency")]
    #[doc(inline)]
    /// ËTR utility token
    pub use etr_token as etr;

    #[cfg(feature = "currency")]
    #[doc(inline)]
    /// ËTD stablecoin
    pub use etd_stablecoin as etd;

    #[cfg(feature = "currency")]
    #[doc(inline)]
    /// VMW gas token
    pub use vmw_gas as vmw;
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSACTIONS - Transaction Processing (feature: transactions)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "transactions")]
/// Transaction processing system
///
/// Handles various transaction types:
/// - Regular transfers
/// - Cross-chain bridge transactions
/// - Lightning Bloc (Layer 2) payments
/// - Smart contract calls
/// - Stake deposits
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::transactions::{types::Transaction, processor};
///
/// let tx = Transaction::transfer(from, to, amount);
/// processor::submit(tx);
/// ```
pub mod transactions {
    #[doc(inline)]
    pub use transaction_types as types;

    #[cfg(feature = "transactions")]
    #[doc(inline)]
    /// Main transaction processor
    pub use tx_processor as processor;

    #[cfg(feature = "transactions")]
    #[doc(inline)]
    /// Cross-chain bridge transactions
    pub use cross_chain_transactions as cross_chain;

    #[cfg(feature = "transactions")]
    #[doc(inline)]
    /// Layer 2 payment channel transactions
    pub use lightning_bloc_transactions as lightning_bloc;

    #[cfg(feature = "transactions")]
    #[doc(inline)]
    /// Smart contract execution transactions
    pub use smart_contract_transactions as smart_contract;

    #[cfg(feature = "transactions")]
    #[doc(inline)]
    /// Validator staking deposits
    pub use stake_deposit_transactions as stake_deposit;
}

// ═══════════════════════════════════════════════════════════════════════════════
// P2P - DETR Networking Protocol (feature: p2p)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "p2p")]
/// DETR P2P networking layer
///
/// Custom libp2p-based protocol with:
/// - Kademlia DHT for peer discovery
/// - ECIES encrypted communication
/// - Peer reputation scoring
/// - Message flow control
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::p2p::{detrp2p, aecomms};
///
/// let network = detrp2p::Network::new(/* ... */);
/// ```
pub mod p2p {
    #[cfg(feature = "p2p")]
    #[doc(inline)]
    /// ECIES encrypted communication
    pub use etrid_aecomms as aecomms;

    #[cfg(feature = "p2p")]
    #[doc(inline)]
    /// Core P2P networking with Kademlia DHT
    pub use detrp2p;

    #[cfg(feature = "p2p")]
    #[doc(inline)]
    /// Peer connection management
    pub use dpeers;

    #[cfg(feature = "p2p")]
    #[doc(inline)]
    /// Protocol message definitions
    pub use etrid_protocol;

    #[cfg(feature = "p2p")]
    #[doc(inline)]
    /// Message flow control
    pub use fluent;

    #[cfg(feature = "p2p")]
    #[doc(inline)]
    /// Peer storage and caching
    pub use etrid_p2p_stored as stored;
}

// ═══════════════════════════════════════════════════════════════════════════════
// IDENTITY - OpenDID System (feature: identity)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "identity")]
/// Decentralized identity system
///
/// OpenDID implementation for decentralized identity management with DID registry
/// and resolver.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::identity::{types::DID, registry};
///
/// let did = DID::new(/* ... */);
/// registry::register(did);
/// ```
pub mod identity {
    #[doc(inline)]
    pub use open_did_types as types;

    #[cfg(feature = "identity")]
    #[doc(inline)]
    /// DID registry pallet
    pub use open_did_registry as registry;

    #[cfg(feature = "identity")]
    #[doc(inline)]
    /// DID resolver with caching
    pub use open_did_resolver as resolver;
}

// ═══════════════════════════════════════════════════════════════════════════════
// SECURITY - Cryptographic Primitives (feature: security)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "security")]
/// Security and cryptography
///
/// Core cryptographic primitives and key management for ËTRID.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::security::{crypto, keys};
///
/// let keypair = crypto::generate_keypair();
/// keys::store(keypair);
/// ```
pub mod security {
    #[cfg(feature = "security")]
    #[doc(inline)]
    /// Core cryptographic primitives
    pub use etrid_cryptography as crypto;

    #[cfg(feature = "security")]
    #[doc(inline)]
    /// Key storage and rotation
    pub use etrid_key_management as keys;
}

// ═══════════════════════════════════════════════════════════════════════════════
// VM - ETWASM Smart Contract VM (feature: vm)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "vm")]
/// ETWASM smart contract execution
///
/// WebAssembly-based smart contract virtual machine with gas metering and sandboxing.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::vm::etwasm;
///
/// let result = etwasm::execute(contract_code, gas_limit);
/// ```
pub mod vm {
    #[cfg(feature = "vm")]
    #[doc(inline)]
    /// ETWASM VM pallet
    pub use pallet_etwasm_vm as etwasm;
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTICHAIN - FlareChain + PBCs (feature: multichain)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "multichain")]
/// Multichain infrastructure
///
/// FlareChain (root chain) and 12 Partition Burst Chains (PBCs) for different ecosystems.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::multichain::{flare_chain, pbc};
///
/// // Access FlareChain runtime
/// let runtime = flare_chain::runtime::Runtime::new();
///
/// // Access specific PBC
/// let eth_runtime = pbc::eth::Runtime::new();
/// ```
pub mod multichain {
    #[doc(inline)]
    pub use multichain_primitives as primitives;

    #[cfg(feature = "multichain")]
    /// FlareChain (root chain)
    pub mod flare_chain {
        #[doc(inline)]
        pub use flare_chain_runtime as runtime;
    }

    #[cfg(feature = "multichain")]
    /// Partition Burst Chains
    pub mod pbc {
        #[doc(inline)]
        pub use pbc_runtime as base;
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BRIDGES - Cross-Chain Protocols (feature: bridges)
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(feature = "bridges")]
/// Cross-chain bridge protocols
///
/// Bridge pallets for connecting ËTRID to external blockchains.
///
/// # Examples
///
/// ```rust,ignore
/// use etrid_sdk::bridges::{cardano, ethereum, solana};
///
/// // Bridge assets from Cardano
/// cardano::bridge_in(amount);
/// ```
pub mod bridges {
    #[cfg(feature = "bridges")]
    #[doc(inline)]
    /// Cardano bridge pallet
    pub use cardano_bridge as cardano;

    #[cfg(feature = "bridges")]
    #[doc(inline)]
    /// Chainlink oracle bridge
    pub use chainlink_bridge as chainlink;

    #[cfg(feature = "bridges")]
    #[doc(inline)]
    /// Polygon bridge pallet
    pub use polygon_bridge as polygon;

    #[cfg(feature = "bridges")]
    #[doc(inline)]
    /// Solana bridge pallet
    pub use solana_bridge as solana;

    #[cfg(feature = "bridges")]
    #[doc(inline)]
    /// Stellar bridge pallet
    pub use stellar_bridge as stellar;
}

// ═══════════════════════════════════════════════════════════════════════════════
// PRELUDE - Common Imports
// ═══════════════════════════════════════════════════════════════════════════════

/// Commonly used types and traits
///
/// Import everything you need with:
///
/// ```rust,ignore
/// use etrid_sdk::prelude::*;
/// ```
pub mod prelude {
    //! Prelude module containing commonly used types

    #[cfg(feature = "accounts")]
    #[doc(inline)]
    pub use crate::accounts::*;

    #[cfg(feature = "staking")]
    #[doc(inline)]
    pub use crate::staking::{Role, StakeRequirement, RoleRecord};

    #[cfg(feature = "consensus-day")]
    #[doc(inline)]
    pub use crate::consensus_day::{ProposalCategory, ProposalStatus, ProposalRecord};

    #[cfg(feature = "currency")]
    #[doc(inline)]
    pub use crate::currency::{etr, etd, vmw};

    #[cfg(feature = "transactions")]
    #[doc(inline)]
    pub use crate::transactions::types::*;
}
