//! # Generic Token Messenger Pallet
//!
//! A CCTP-style (Cross-Chain Transfer Protocol) token messenger for burn-and-mint bridging
//! across all Partition Burst Chains (PBCs) in the Etrid multichain ecosystem.
//!
//! ## Overview
//!
//! This pallet provides a generic, reusable implementation for cross-chain token transfers
//! using a burn-and-mint architecture. Unlike the EDSC-specific implementation, this pallet
//! can be configured for any token on any PBC.
//!
//! ### Key Features:
//! - Generic token support (not limited to EDSC)
//! - Burn tokens on source chain, mint on destination chain
//! - Nonce-based message ordering prevents replay attacks
//! - Domain registry for supported blockchains (both PBCs and external chains)
//! - Rate limiting with per-transaction and daily limits
//! - Emergency pause functionality for safety
//! - Attestation-based security model via integration with pallet-bridge-attestation
//!
//! ## Architecture
//!
//! ### Message Flow
//!
//! #### Outbound (Source Chain → Destination Chain):
//! ```text
//! User → deposit_for_burn()
//!     ├─ Validate domain and amount limits
//!     ├─ Burn tokens from sender's account
//!     ├─ Create CrossChainMessage with nonce
//!     ├─ Store outbound message
//!     ├─ Emit DepositForBurn event
//!     └─ Off-chain: Attesters sign message hash
//!         └─ Relayer delivers message + attestation to destination
//! ```
//!
//! #### Inbound (Destination Chain receives):
//! ```text
//! Relayer → receive_message()
//!     ├─ Verify attestation via pallet-bridge-attestation
//!     ├─ Check nonce not already used (replay protection)
//!     ├─ Validate destination domain matches this chain
//!     ├─ Parse and validate message body
//!     ├─ Mark nonce as used
//!     ├─ Mint tokens to recipient
//!     └─ Emit MessageReceived event
//! ```
//!
//! ## Message Format
//!
//! Messages follow a standardized format compatible with CCTP:
//! ```text
//! CrossChainMessage:
//! ├─ version: u32                    (message format version)
//! ├─ source_domain: u32              (source blockchain identifier)
//! ├─ destination_domain: u32         (destination blockchain identifier)
//! ├─ nonce: u64                      (unique message nonce)
//! ├─ sender: BoundedVec<u8, 64>     (sender address on source)
//! ├─ recipient: BoundedVec<u8, 64>  (recipient address on destination)
//! └─ message_body: BurnMessage
//!     ├─ version: u32
//!     ├─ burn_token: BoundedVec<u8, 64>      (token identifier)
//!     ├─ mint_recipient: BoundedVec<u8, 64>  (recipient on destination)
//!     ├─ amount: u128                        (amount with decimals)
//!     └─ memo: BoundedVec<u8, 128>           (optional metadata)
//! ```
//!
//! ## Domain IDs
//!
//! Domain IDs identify different blockchains in the ecosystem:
//! - 0: Ethereum mainnet
//! - 1: Solana mainnet
//! - 2: Etrid Primearc Core Chain (relay chain)
//! - 3: Polygon
//! - 4: BNB Chain
//! - 5: Avalanche
//! - 6: Arbitrum
//! - 7: Optimism
//! - 100-199: Etrid PBCs (e.g., 100 = EDSC, 101 = ETH-PBC, etc.)
//!
//! ## Safety Features
//!
//! 1. **Rate Limiting**: Per-transaction and daily limits prevent excessive token movement
//! 2. **Emergency Pause**: Governance can pause all operations if issues detected
//! 3. **Nonce Tracking**: Prevents replay attacks and ensures message ordering
//! 4. **Attestation Verification**: Messages must be signed by configured attesters
//! 5. **Domain Validation**: Only enabled domains can send/receive messages
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! // Configure a new domain (governance)
//! TokenMessenger::configure_domain(
//!     Origin::root(),
//!     domain_id: 100, // EDSC PBC
//!     enabled: true,
//!     max_burn_amount: 1_000_000 * 10^18, // 1M tokens
//!     daily_burn_limit: 10_000_000 * 10^18, // 10M tokens per day
//! )?;
//!
//! // User burns tokens for cross-chain transfer
//! TokenMessenger::deposit_for_burn(
//!     Origin::signed(sender),
//!     amount: 100 * 10^18, // 100 tokens
//!     destination_domain: 0, // Ethereum
//!     mint_recipient: eth_address_bytes,
//! )?;
//!
//! // Relayer delivers message on destination chain
//! TokenMessenger::receive_message(
//!     Origin::signed(relayer),
//!     message: encoded_message,
//!     attestation: attester_signatures,
//! )?;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

pub mod attestation;
pub use attestation::*;

// Token operations implementations
pub mod token_ops;
pub use token_ops::{BalancesTokenOps, PbcTokenOperations};

// Fee calculation and collection
pub mod fees;
pub use fees::*;

/// Trait for token burn/mint operations
///
/// This trait must be implemented by the runtime to provide actual token
/// burning and minting functionality. It abstracts the specific token
/// implementation (native balance, fungible asset, etc.).
pub trait TokenOperations<AccountId> {
	/// Burn tokens from an account
	///
	/// # Arguments
	/// * `account` - The account to burn tokens from
	/// * `amount` - The amount to burn (with full precision/decimals)
	fn burn_tokens(account: &AccountId, amount: u128) -> frame_support::dispatch::DispatchResult;

	/// Mint tokens to an account
	///
	/// # Arguments
	/// * `account` - The account to mint tokens to
	/// * `amount` - The amount to mint (with full precision/decimals)
	fn mint_tokens(account: &AccountId, amount: u128) -> frame_support::dispatch::DispatchResult;

	/// Get balance of an account (for validation)
	fn balance_of(account: &AccountId) -> u128;
}

/// Default implementation (no-op for testing)
impl<AccountId> TokenOperations<AccountId> for () {
	fn burn_tokens(_account: &AccountId, _amount: u128) -> frame_support::dispatch::DispatchResult {
		Ok(())
	}
	fn mint_tokens(_account: &AccountId, _amount: u128) -> frame_support::dispatch::DispatchResult {
		Ok(())
	}
	fn balance_of(_account: &AccountId) -> u128 {
		0
	}
}

/// Trait for cross-chain message attestation verification
///
/// This trait integrates with pallet-bridge-attestation to verify that
/// cross-chain messages have been properly signed by the configured
/// set of attesters using M-of-N threshold signatures.
pub trait AttestationVerifier {
	/// Verify attestation for a cross-chain message
	///
	/// # Arguments
	/// * `message` - The full encoded message bytes
	/// * `attestation` - The attestation proof (signatures)
	/// * `message_hash` - The hash of the message
	///
	/// # Returns
	/// * `Ok(())` if attestation is valid
	/// * `Err(_)` if attestation verification fails
	fn verify_message_attestation(
		message: &[u8],
		attestation: &[u8],
		message_hash: sp_core::H256,
	) -> frame_support::dispatch::DispatchResult;
}

/// Default implementation (for testing - no verification)
impl AttestationVerifier for () {
	fn verify_message_attestation(
		_message: &[u8],
		_attestation: &[u8],
		_message_hash: sp_core::H256,
	) -> frame_support::dispatch::DispatchResult {
		Ok(())
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Get, Currency, ExistenceRequirement},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Saturating;
	use sp_core::H256;
	use sp_std::vec::Vec;

	/// Domain identifier for different blockchains
	///
	/// Domains uniquely identify blockchains in the cross-chain ecosystem.
	/// PBC domains are allocated in the 100-199 range, while external chains
	/// use well-known domain IDs compatible with Circle's CCTP.
	#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Domain {
		/// Ethereum mainnet
		Ethereum = 0,
		/// Solana mainnet
		Solana = 1,
		/// Etrid Primearc Core Chain (relay chain)
		PrimearcCore = 2,
		/// Polygon
		Polygon = 3,
		/// BNB Chain
		BnbChain = 4,
		/// Avalanche
		Avalanche = 5,
		/// Arbitrum
		Arbitrum = 6,
		/// Optimism
		Optimism = 7,
		/// Base
		Base = 8,

		// Etrid PBCs (100-199 range)
		/// EDSC PBC (Euro Digital Stable Coin)
		PbcEdsc = 100,
		/// ETH PBC (Ethereum bridge chain)
		PbcEth = 101,
		/// SOL PBC (Solana bridge chain)
		PbcSol = 102,
		/// BTC PBC (Bitcoin bridge chain)
		PbcBtc = 103,
		/// USDC PBC
		PbcUsdc = 104,
		/// USDT PBC
		PbcUsdt = 105,
		/// DAI PBC
		PbcDai = 106,

		/// Custom domain (use DomainConfigs for definition)
		Custom = 255,
	}

	impl Domain {
		/// Convert Domain to u32 for message encoding
		pub fn to_u32(&self) -> u32 {
			*self as u32
		}

		/// Convert u32 to Domain
		pub fn from_u32(value: u32) -> Option<Self> {
			match value {
				0 => Some(Domain::Ethereum),
				1 => Some(Domain::Solana),
				2 => Some(Domain::PrimearcCore),
				3 => Some(Domain::Polygon),
				4 => Some(Domain::BnbChain),
				5 => Some(Domain::Avalanche),
				6 => Some(Domain::Arbitrum),
				7 => Some(Domain::Optimism),
				8 => Some(Domain::Base),
				100 => Some(Domain::PbcEdsc),
				101 => Some(Domain::PbcEth),
				102 => Some(Domain::PbcSol),
				103 => Some(Domain::PbcBtc),
				104 => Some(Domain::PbcUsdc),
				105 => Some(Domain::PbcUsdt),
				106 => Some(Domain::PbcDai),
				255 => Some(Domain::Custom),
				_ => None,
			}
		}
	}

	/// Cross-chain message format (CCTP-compatible)
	///
	/// This message format is designed to be compatible with Circle's CCTP
	/// while being extensible for Etrid's multichain needs.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CrossChainMessage {
		/// Message format version (current: 1)
		pub version: u32,
		/// Source domain identifier
		pub source_domain: u32,
		/// Destination domain identifier
		pub destination_domain: u32,
		/// Unique nonce (per source domain)
		pub nonce: u64,
		/// Sender address (flexible format for different chains)
		pub sender: BoundedVec<u8, ConstU32<64>>,
		/// Recipient address (destination chain format)
		pub recipient: BoundedVec<u8, ConstU32<64>>,
		/// Message body (contains burn/mint details)
		pub message_body: BoundedVec<u8, ConstU32<512>>,
	}

	/// Burn message body
	///
	/// Encodes the details of tokens burned on source chain that should
	/// be minted on destination chain.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct BurnMessage {
		/// Message body version
		pub version: u32,
		/// Token identifier being burned (contract address, asset ID, etc.)
		pub burn_token: BoundedVec<u8, ConstU32<64>>,
		/// Mint recipient on destination chain
		pub mint_recipient: BoundedVec<u8, ConstU32<64>>,
		/// Amount (with full precision/decimals)
		pub amount: u128,
		/// Optional memo/metadata
		pub memo: BoundedVec<u8, ConstU32<128>>,
	}

	#[derive(Decode)]
	struct LegacyBurnMessage {
		version: u32,
		burn_token: BoundedVec<u8, ConstU32<64>>,
		mint_recipient: BoundedVec<u8, ConstU32<64>>,
		amount: u128,
	}

	impl LegacyBurnMessage {
		fn into_current(self) -> BurnMessage {
			BurnMessage {
				version: self.version,
				burn_token: self.burn_token,
				mint_recipient: self.mint_recipient,
				amount: self.amount,
				memo: BoundedVec::default(),
			}
		}
	}

	/// Message status tracking
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum MessageStatus {
		/// Message sent, awaiting attestation
		Sent,
		/// Attestation received, ready to process
		Attested,
		/// Message processed and minted
		Completed,
		/// Message failed
		Failed,
	}

	impl Default for MessageStatus {
		fn default() -> Self {
			MessageStatus::Sent
		}
	}

	/// Domain configuration
	///
	/// Each domain has its own configuration for rate limits and enabled status.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct DomainConfig {
		/// Is this domain enabled for transfers?
		pub enabled: bool,
		/// Maximum burn amount per transaction (with decimals)
		pub max_burn_amount: u128,
		/// Daily burn limit (resets every 24h)
		pub daily_burn_limit: u128,
		/// Minimum burn amount (prevents dust attacks)
		pub min_burn_amount: u128,
	}

	impl Default for DomainConfig {
		fn default() -> Self {
			DomainConfig {
				enabled: false,
				max_burn_amount: 0,
				daily_burn_limit: 0,
				min_burn_amount: 0,
			}
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Token operations provider (burn/mint)
		type TokenOperations: crate::TokenOperations<Self::AccountId>;

		/// Attestation verification provider
		type AttestationVerifier: crate::AttestationVerifier;

		/// Weight information for extrinsics
		type WeightInfo: WeightInfo;

		/// Currency type for fee collection
		type Currency: frame_support::traits::Currency<Self::AccountId>;

		/// Maximum message body size (bytes)
		#[pallet::constant]
		type MaxMessageBodySize: Get<u32>;

		/// Default maximum burn amount per transaction
		#[pallet::constant]
		type MaxBurnAmount: Get<u128>;

		/// Default daily burn cap across all domains
		#[pallet::constant]
		type DailyBurnCap: Get<u128>;

		/// Minimum burn amount (prevents dust)
		#[pallet::constant]
		type MinBurnAmount: Get<u128>;

		/// Message timeout in blocks (for cleanup)
		#[pallet::constant]
		type MessageTimeout: Get<BlockNumberFor<Self>>;

		/// Blocks per day (for daily limit reset)
		/// Default: 14400 blocks (assuming 6s block time = 24 hours)
		#[pallet::constant]
		type BlocksPerDay: Get<BlockNumberFor<Self>>;

		/// Local domain ID (the domain of this chain)
		#[pallet::constant]
		type LocalDomain: Get<u32>;

		/// Fee rate in basis points (100 = 1%, 10000 = 100%)
		/// Example: 30 = 0.3% fee, 100 = 1% fee
		#[pallet::constant]
		type BridgeFeeRate: Get<u32>;

		/// Minimum fee amount (prevents too-low fees on small transfers)
		/// Should be denominated in the native currency
		#[pallet::constant]
		type MinBridgeFee: Get<u128>;

		/// Fee collector account (receives all bridge fees)
		#[pallet::constant]
		type FeeCollector: Get<Self::AccountId>;
	}

	/// Outbound messages (burned on this chain, awaiting attestation)
	#[pallet::storage]
	#[pallet::getter(fn outbound_messages)]
	pub type OutboundMessages<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // Nonce
		CrossChainMessage,
		OptionQuery,
	>;

	/// Message nonce counter (per destination domain)
	///
	/// Each destination domain has its own nonce sequence to enable
	/// parallel processing and prevent cross-domain nonce conflicts.
	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	pub type MessageNonce<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Destination domain
		u64, // Next nonce
		ValueQuery,
	>;

	/// Used nonces for inbound messages (replay protection)
	///
	/// Tracks which (source_domain, nonce) pairs have been processed
	/// to prevent replay attacks.
	#[pallet::storage]
	#[pallet::getter(fn used_nonces)]
	pub type UsedNonces<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u32, // Source domain
		Blake2_128Concat,
		u64, // Nonce
		bool,
		ValueQuery,
	>;

	/// Domain configurations
	///
	/// Stores configuration for each supported domain including rate limits.
	#[pallet::storage]
	#[pallet::getter(fn domain_config)]
	pub type DomainConfigs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Domain ID
		DomainConfig,
		OptionQuery,
	>;

	/// Supported domains list
	///
	/// Quick lookup for whether a domain is configured.
	#[pallet::storage]
	#[pallet::getter(fn supported_domains)]
	pub type SupportedDomains<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Domain ID
		bool,
		ValueQuery,
	>;

	/// Daily burn volume tracking (per domain)
	///
	/// Tracks burn volume per domain with automatic daily reset.
	/// Format: (last_reset_block, current_volume)
	#[pallet::storage]
	#[pallet::getter(fn daily_burn_volume)]
	pub type DailyBurnVolume<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Domain
		(BlockNumberFor<T>, u128), // (last_reset_block, volume)
		ValueQuery,
	>;

	/// Total outbound messages sent
	#[pallet::storage]
	#[pallet::getter(fn total_sent)]
	pub type TotalSent<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Total inbound messages received
	#[pallet::storage]
	#[pallet::getter(fn total_received)]
	pub type TotalReceived<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Total volume sent (all domains)
	#[pallet::storage]
	#[pallet::getter(fn total_volume_sent)]
	pub type TotalVolumeSent<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// Total volume received (all domains)
	#[pallet::storage]
	#[pallet::getter(fn total_volume_received)]
	pub type TotalVolumeReceived<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// Bridge pause status (emergency stop)
	#[pallet::storage]
	#[pallet::getter(fn is_paused)]
	pub type IsPaused<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Total fees collected from bridge operations
	#[pallet::storage]
	#[pallet::getter(fn total_fees_collected)]
	pub type TotalFeesCollected<T: Config> = StorageValue<_, u128, ValueQuery>;

	/// Fees collected per domain
	#[pallet::storage]
	#[pallet::getter(fn domain_fees_collected)]
	pub type DomainFeesCollected<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u32, // Domain
		u128, // Total fees collected
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Tokens burned for cross-chain transfer
		/// [nonce, sender, destination_domain, recipient, amount]
		DepositForBurn {
			nonce: u64,
			sender: T::AccountId,
			destination_domain: u32,
			recipient: Vec<u8>,
			amount: u128,
		},
		/// Message received and tokens minted
		/// [source_domain, nonce, recipient, amount]
		MessageReceived {
			source_domain: u32,
			nonce: u64,
			recipient: T::AccountId,
			amount: u128,
		},
		/// Message sent (emitted for indexing)
		/// [message_hash, nonce]
		MessageSent {
			message_hash: H256,
			nonce: u64,
		},
		/// Domain configuration updated
		/// [domain, enabled]
		DomainConfigured {
			domain: u32,
			enabled: bool,
		},
		/// Domain removed
		/// [domain]
		DomainRemoved {
			domain: u32,
		},
		/// Bridge paused by governance
		BridgePaused,
		/// Bridge unpaused by governance
		BridgeUnpaused,
		/// Daily limit exceeded (informational, tx will fail)
		/// [domain, attempted, current_volume, limit]
		DailyLimitExceeded {
			domain: u32,
			attempted: u128,
			current_volume: u128,
			limit: u128,
		},
		/// Bridge fee collected
		/// [sender, fee_collector, destination_domain, fee_amount, transfer_amount]
		FeeCollected {
			sender: T::AccountId,
			fee_collector: T::AccountId,
			destination_domain: u32,
			fee_amount: u128,
			transfer_amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Bridge is paused
		BridgePaused,
		/// Invalid domain
		InvalidDomain,
		/// Domain not enabled
		DomainNotEnabled,
		/// Domain not supported
		DomainNotSupported,
		/// Amount exceeds maximum per-transaction limit
		AmountExceedsMax,
		/// Amount below minimum
		AmountBelowMin,
		/// Daily limit exceeded
		DailyLimitExceeded,
		/// Insufficient balance to burn
		InsufficientBalance,
		/// Message already processed (replay attempt)
		MessageAlreadyProcessed,
		/// Invalid message format
		InvalidMessageFormat,
		/// Invalid message version
		InvalidMessageVersion,
		/// Invalid recipient address
		InvalidRecipient,
		/// Invalid sender address
		InvalidSender,
		/// Nonce mismatch
		NonceMismatch,
		/// Message too large
		MessageTooLarge,
		/// Attestation verification failed
		AttestationFailed,
		/// Token burn operation failed
		BurnFailed,
		/// Token mint operation failed
		MintFailed,
		/// Message destination is not this chain
		InvalidDestination,
		/// Message source equals destination
		SourceEqualsDestination,
		/// Arithmetic overflow
		ArithmeticOverflow,
		/// Invalid amount (zero)
		InvalidAmount,
		/// Fee exceeds transfer amount
		FeeExceedsAmount,
		/// Insufficient balance to pay fee
		InsufficientBalanceForFee,
		/// Fee transfer failed
		FeeTransferFailed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			// Clean up old messages periodically
			// Reset daily limits if needed (handled per-transaction)
			Weight::zero()
		}

		fn on_finalize(_n: BlockNumberFor<T>) {
			// Additional finalization logic if needed
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Burn tokens for cross-chain transfer
		///
		/// Burns tokens from the sender's account and creates a cross-chain message
		/// that can be relayed to the destination chain for minting. A bridge fee is
		/// collected from the sender in native currency and transferred to the fee collector.
		///
		/// # Arguments
		/// * `origin` - The sender (must be signed)
		/// * `amount` - Amount to burn (with full precision/decimals)
		/// * `destination_domain` - Target blockchain domain ID
		/// * `mint_recipient` - Recipient address on destination chain (bytes)
		///
		/// # Fee Collection
		/// The sender must pay a bridge fee calculated as:
		/// * `fee = max(amount * BridgeFeeRate / 10000, MinBridgeFee)`
		/// * Fee is paid in native currency and transferred to FeeCollector account
		/// * Sender needs both: sufficient tokens to burn AND sufficient native currency for fee
		///
		/// # Events
		/// * `DepositForBurn` - When tokens are successfully burned
		/// * `MessageSent` - When message is created and stored
		/// * `FeeCollected` - When bridge fee is collected
		///
		/// # Errors
		/// * `BridgePaused` - If bridge is in emergency pause
		/// * `InvalidDomain` - If destination domain is not configured
		/// * `DomainNotEnabled` - If destination domain is disabled
		/// * `AmountExceedsMax` - If amount exceeds per-transaction limit
		/// * `AmountBelowMin` - If amount below minimum
		/// * `DailyLimitExceeded` - If daily limit would be exceeded
		/// * `InsufficientBalance` - If sender doesn't have enough tokens
		/// * `InsufficientBalanceForFee` - If sender doesn't have enough native currency for fee
		/// * `FeeExceedsAmount` - If calculated fee exceeds transfer amount
		/// * `FeeTransferFailed` - If fee transfer operation fails
		/// * `BurnFailed` - If burn operation fails
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::deposit_for_burn())]
		pub fn deposit_for_burn(
			origin: OriginFor<T>,
			amount: u128,
			destination_domain: u32,
			mint_recipient: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check bridge not paused
			ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);

			// Validate amount
			ensure!(amount > 0, Error::<T>::InvalidAmount);

			// Validate destination domain
			let local_domain = T::LocalDomain::get();
			ensure!(destination_domain != local_domain, Error::<T>::SourceEqualsDestination);

			let domain_config = DomainConfigs::<T>::get(destination_domain)
				.ok_or(Error::<T>::InvalidDomain)?;
			ensure!(domain_config.enabled, Error::<T>::DomainNotEnabled);

			// Check amount limits
			ensure!(
				amount >= domain_config.min_burn_amount,
				Error::<T>::AmountBelowMin
			);
			ensure!(
				amount <= domain_config.max_burn_amount,
				Error::<T>::AmountExceedsMax
			);

			// Calculate bridge fee
			let fee_rate = T::BridgeFeeRate::get();
			let min_fee = T::MinBridgeFee::get();
			let fee_amount = FeeCalculator::calculate_fee(amount, fee_rate, min_fee);

			// Validate fee doesn't exceed amount
			ensure!(
				FeeCalculator::validate_fee(amount, fee_amount),
				Error::<T>::FeeExceedsAmount
			);

			// Check sender has sufficient token balance for burn
			let balance = T::TokenOperations::balance_of(&sender);
			ensure!(balance >= amount, Error::<T>::InsufficientBalance);

			// Check sender has sufficient native balance to pay fee
			let fee_collector = T::FeeCollector::get();
			let sender_native_balance = <T::Currency as Currency<T::AccountId>>::free_balance(&sender);

			// Convert fee amount to Balance type
			use sp_runtime::traits::TryConvert;
			let fee_balance: <<T as Config>::Currency as frame_support::traits::Currency<T::AccountId>>::Balance =
				fee_amount.try_into().map_err(|_| Error::<T>::ArithmeticOverflow)?;

			ensure!(
				sender_native_balance >= fee_balance,
				Error::<T>::InsufficientBalanceForFee
			);

			// Deduct and transfer fee to collector
			<T::Currency as Currency<T::AccountId>>::transfer(
				&sender,
				&fee_collector,
				fee_balance,
				ExistenceRequirement::KeepAlive,
			).map_err(|_| Error::<T>::FeeTransferFailed)?;

			// Update fee statistics
			TotalFeesCollected::<T>::mutate(|total| *total = total.saturating_add(fee_amount));
			DomainFeesCollected::<T>::mutate(destination_domain, |total| {
				*total = total.saturating_add(fee_amount)
			});

			// Check and update daily limit
			Self::check_and_update_daily_limit(destination_domain, amount)?;

			// Get next nonce for destination domain
			let nonce = MessageNonce::<T>::get(destination_domain);
			MessageNonce::<T>::insert(destination_domain, nonce.saturating_add(1));

			// Create burn message body
			let burn_msg = BurnMessage {
				version: 1,
				burn_token: b"GENERIC".to_vec().try_into().unwrap_or_default(),
				mint_recipient: mint_recipient.clone().try_into()
					.map_err(|_| Error::<T>::InvalidRecipient)?,
				amount,
				memo: BoundedVec::default(),
			};

			// Create cross-chain message
			let message = CrossChainMessage {
				version: 1,
				source_domain: local_domain,
				destination_domain,
				nonce,
				sender: Self::account_to_bytes(&sender),
				recipient: mint_recipient.clone().try_into()
					.map_err(|_| Error::<T>::InvalidRecipient)?,
				message_body: burn_msg.encode().try_into()
					.map_err(|_| Error::<T>::MessageTooLarge)?,
			};

			// Get message hash for indexing
			let message_hash = Self::get_message_hash(&message);

			// Store outbound message
			OutboundMessages::<T>::insert(nonce, message);

			// Update statistics
			TotalSent::<T>::mutate(|count| *count = count.saturating_add(1));
			TotalVolumeSent::<T>::mutate(|volume| *volume = volume.saturating_add(amount));

			// Burn tokens from sender's account
			T::TokenOperations::burn_tokens(&sender, amount)
				.map_err(|_| Error::<T>::BurnFailed)?;

			// Emit events
			Self::deposit_event(Event::DepositForBurn {
				nonce,
				sender: sender.clone(),
				destination_domain,
				recipient: mint_recipient,
				amount,
			});

			Self::deposit_event(Event::MessageSent {
				message_hash,
				nonce,
			});

			Self::deposit_event(Event::FeeCollected {
				sender,
				fee_collector,
				destination_domain,
				fee_amount,
				transfer_amount: amount,
			});

			Ok(())
		}

		/// Receive cross-chain message and mint tokens
		///
		/// Verifies the attestation, validates the message, and mints tokens
		/// to the recipient if all checks pass.
		///
		/// # Arguments
		/// * `origin` - The relayer (must be signed, but anyone can relay)
		/// * `message` - Encoded CrossChainMessage bytes
		/// * `attestation` - Attestation proof (signatures from attesters)
		///
		/// # Events
		/// * `MessageReceived` - When tokens are successfully minted
		///
		/// # Errors
		/// * `BridgePaused` - If bridge is in emergency pause
		/// * `InvalidMessageFormat` - If message cannot be decoded
		/// * `InvalidDestination` - If message destination is not this chain
		/// * `MessageAlreadyProcessed` - If nonce already used (replay attack)
		/// * `AttestationFailed` - If attestation verification fails
		/// * `InvalidRecipient` - If recipient address invalid
		/// * `MintFailed` - If mint operation fails
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::receive_message())]
		pub fn receive_message(
			origin: OriginFor<T>,
			message: Vec<u8>,
			attestation: Vec<u8>,
		) -> DispatchResult {
			ensure_signed(origin)?;

			// Check bridge not paused
			ensure!(!IsPaused::<T>::get(), Error::<T>::BridgePaused);

			// Decode message
			let cross_chain_msg = CrossChainMessage::decode(&mut &message[..])
				.map_err(|_| Error::<T>::InvalidMessageFormat)?;

			// Validate message version
			ensure!(cross_chain_msg.version == 1, Error::<T>::InvalidMessageVersion);

			// Verify destination is this chain
			let local_domain = T::LocalDomain::get();
			ensure!(
				cross_chain_msg.destination_domain == local_domain,
				Error::<T>::InvalidDestination
			);

			// Check source domain is supported
			ensure!(
				SupportedDomains::<T>::get(cross_chain_msg.source_domain),
				Error::<T>::DomainNotSupported
			);

			// Check nonce not already used (replay protection)
			ensure!(
				!UsedNonces::<T>::get(cross_chain_msg.source_domain, cross_chain_msg.nonce),
				Error::<T>::MessageAlreadyProcessed
			);

			// Verify attestation
			let message_hash = Self::get_message_hash(&cross_chain_msg);
			T::AttestationVerifier::verify_message_attestation(&message, &attestation, message_hash)
				.map_err(|_| Error::<T>::AttestationFailed)?;

			// Decode burn message from body
			let burn_msg = Self::decode_burn_message(&cross_chain_msg.message_body[..])?;

			// Validate amount
			ensure!(burn_msg.amount > 0, Error::<T>::InvalidAmount);

			// Mark nonce as used
			UsedNonces::<T>::insert(cross_chain_msg.source_domain, cross_chain_msg.nonce, true);

			// Update statistics
			TotalReceived::<T>::mutate(|count| *count = count.saturating_add(1));
			TotalVolumeReceived::<T>::mutate(|volume| *volume = volume.saturating_add(burn_msg.amount));

			// Convert recipient bytes to account
			let recipient_account: T::AccountId = T::AccountId::decode(&mut &burn_msg.mint_recipient[..])
				.map_err(|_| Error::<T>::InvalidRecipient)?;

			// Mint tokens to recipient
			T::TokenOperations::mint_tokens(&recipient_account, burn_msg.amount)
				.map_err(|_| Error::<T>::MintFailed)?;

			// Emit event
			Self::deposit_event(Event::MessageReceived {
				source_domain: cross_chain_msg.source_domain,
				nonce: cross_chain_msg.nonce,
				recipient: recipient_account,
				amount: burn_msg.amount,
			});

			Ok(())
		}

		/// Configure domain settings
		///
		/// Sets or updates the configuration for a destination domain.
		/// Only callable by governance (root).
		///
		/// # Arguments
		/// * `origin` - Must be root
		/// * `domain` - Domain ID to configure
		/// * `enabled` - Whether domain is enabled for transfers
		/// * `max_burn_amount` - Maximum per-transaction amount
		/// * `daily_burn_limit` - Maximum daily volume
		/// * `min_burn_amount` - Minimum transfer amount
		///
		/// # Events
		/// * `DomainConfigured` - When domain is configured
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::configure_domain())]
		pub fn configure_domain(
			origin: OriginFor<T>,
			domain: u32,
			enabled: bool,
			max_burn_amount: u128,
			daily_burn_limit: u128,
			min_burn_amount: u128,
		) -> DispatchResult {
			ensure_root(origin)?;

			let config = DomainConfig {
				enabled,
				max_burn_amount,
				daily_burn_limit,
				min_burn_amount,
			};

			DomainConfigs::<T>::insert(domain, config);
			SupportedDomains::<T>::insert(domain, true);

			Self::deposit_event(Event::DomainConfigured {
				domain,
				enabled,
			});

			Ok(())
		}

		/// Remove domain configuration
		///
		/// Removes a domain from the supported domains list.
		/// Only callable by governance (root).
		///
		/// # Arguments
		/// * `origin` - Must be root
		/// * `domain` - Domain ID to remove
		///
		/// # Events
		/// * `DomainRemoved` - When domain is removed
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::remove_domain())]
		pub fn remove_domain(
			origin: OriginFor<T>,
			domain: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

			DomainConfigs::<T>::remove(domain);
			SupportedDomains::<T>::remove(domain);

			Self::deposit_event(Event::DomainRemoved { domain });

			Ok(())
		}

		/// Pause bridge operations
		///
		/// Emergency stop for all bridge operations.
		/// Only callable by governance (root).
		///
		/// # Events
		/// * `BridgePaused` - When bridge is paused
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::pause_bridge())]
		pub fn pause_bridge(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			IsPaused::<T>::put(true);

			Self::deposit_event(Event::BridgePaused);

			Ok(())
		}

		/// Unpause bridge operations
		///
		/// Resume normal bridge operations after pause.
		/// Only callable by governance (root).
		///
		/// # Events
		/// * `BridgeUnpaused` - When bridge is unpaused
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::unpause_bridge())]
		pub fn unpause_bridge(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			IsPaused::<T>::put(false);

			Self::deposit_event(Event::BridgeUnpaused);

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Convert account to bytes for cross-chain addressing
		fn account_to_bytes(account: &T::AccountId) -> BoundedVec<u8, ConstU32<64>> {
			account.encode().try_into().unwrap_or_default()
		}

		fn decode_burn_message(data: &[u8]) -> Result<BurnMessage, Error<T>> {
			BurnMessage::decode(&mut &data[..])
				.or_else(|_| {
					LegacyBurnMessage::decode(&mut &data[..])
						.map(LegacyBurnMessage::into_current)
				})
				.map_err(|_| Error::<T>::InvalidMessageFormat)
		}

		/// Check and update daily burn limit for a domain
		///
		/// Automatically resets the daily counter if 24 hours have passed.
		fn check_and_update_daily_limit(domain: u32, amount: u128) -> DispatchResult {
			let current_block = <frame_system::Pallet<T>>::block_number();
			let (last_reset, current_volume) = DailyBurnVolume::<T>::get(domain);

			let blocks_per_day = T::BlocksPerDay::get();

			// Reset if 24 hours passed
			let (new_volume, new_reset) = if current_block.saturating_sub(last_reset) >= blocks_per_day {
				(amount, current_block)
			} else {
				(current_volume.saturating_add(amount), last_reset)
			};

			// Check limit
			let domain_config = DomainConfigs::<T>::get(domain)
				.ok_or(Error::<T>::InvalidDomain)?;

			if new_volume > domain_config.daily_burn_limit {
				Self::deposit_event(Event::DailyLimitExceeded {
					domain,
					attempted: amount,
					current_volume,
					limit: domain_config.daily_burn_limit,
				});
				return Err(Error::<T>::DailyLimitExceeded.into());
			}

			// Update volume
			DailyBurnVolume::<T>::insert(domain, (new_reset, new_volume));

			Ok(())
		}

		/// Get message hash for attestation
		pub fn get_message_hash(message: &CrossChainMessage) -> H256 {
			H256::from(sp_io::hashing::blake2_256(&message.encode()))
		}

		/// Get outbound message by nonce
		pub fn get_outbound_message(nonce: u64) -> Option<CrossChainMessage> {
			OutboundMessages::<T>::get(nonce)
		}

		/// Check if nonce is used
		pub fn is_nonce_used(source_domain: u32, nonce: u64) -> bool {
			UsedNonces::<T>::get(source_domain, nonce)
		}

		/// Get next nonce for a destination domain
		pub fn get_next_nonce(destination_domain: u32) -> u64 {
			MessageNonce::<T>::get(destination_domain)
		}

		/// Get current daily volume for a domain
		pub fn get_daily_volume(domain: u32) -> (BlockNumberFor<T>, u128) {
			DailyBurnVolume::<T>::get(domain)
		}

		/// Calculate bridge fee for a given amount
		///
		/// Returns the fee that would be charged for a bridge operation
		/// of the specified amount.
		pub fn calculate_bridge_fee(amount: u128) -> u128 {
			let fee_rate = T::BridgeFeeRate::get();
			let min_fee = T::MinBridgeFee::get();
			FeeCalculator::calculate_fee(amount, fee_rate, min_fee)
		}

		/// Get total fees collected across all domains
		pub fn get_total_fees_collected() -> u128 {
			TotalFeesCollected::<T>::get()
		}

		/// Get fees collected for a specific domain
		pub fn get_domain_fees_collected(domain: u32) -> u128 {
			DomainFeesCollected::<T>::get(domain)
		}

		/// Get the fee collector account
		pub fn get_fee_collector() -> T::AccountId {
			T::FeeCollector::get()
		}
	}
}
