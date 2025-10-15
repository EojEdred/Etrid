//! # LINK Bridge Pallet (Chainlink)
//!
//! Bridge pallet for Chainlink token integration with Ëtrid Multichain.
//! Supports LINK ERC-20 token on Ethereum and oracle price feed integration.
//! Includes Chainlink-specific features: Oracle data feeds, VRF, Proof of Reserve, ERC-677

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_runtime::{traits::SaturatedConversion, RuntimeDebug};
use sp_core::{H160, H256};

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};

// Use sp_core types which already have all codec traits implemented
pub type EthereumAddress = H160;
pub type EthTxHash = H256;

/// Chainlink oracle price data
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OraclePrice {
    pub price: u128,           // Price in USD (18 decimals)
    pub timestamp: u64,        // Unix timestamp
    pub round_id: u64,         // Chainlink round ID (changed from u80 to u64 for MaxEncodedLen)
    pub answered_in_round: u64, // Changed from u80 to u64
}

/// Chainlink Data Feed identifier
pub type DataFeedId = H256;

/// VRF (Verifiable Random Function) request
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[scale_info(skip_type_params(AccountId))]
pub struct VRFRequest<AccountId> {
    pub requester: AccountId,
    pub key_hash: H256,
    pub seed: u64,
    pub fee: u128,
    pub is_fulfilled: bool,
}

/// ERC-677 transferAndCall data
#[derive(Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ERC677Transfer {
    pub to: EthereumAddress,
    pub value: u128,
    pub data: [u8; 32], // Bounded data for MaxEncodedLen
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::Saturating;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;

        /// Minimum confirmations required (12 for Ethereum)
        #[pallet::constant]
        type MinConfirmations: Get<u32>;

        /// Bridge fee rate (e.g., 0.1% = 10)
        #[pallet::constant]
        type BridgeFeeRate: Get<u32>;

        /// Maximum number of oracle nodes
        #[pallet::constant]
        type MaxOracleNodes: Get<u32>;

        /// Maximum number of data feeds
        #[pallet::constant]
        type MaxDataFeeds: Get<u32>;

        /// Maximum number of VRF requests
        #[pallet::constant]
        type MaxVRFRequests: Get<u32>;

        /// Oracle price staleness threshold (in blocks)
        #[pallet::constant]
        type PriceStalenessThreshold: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// LINK deposits from Ethereum to Ëtrid
    #[pallet::storage]
    #[pallet::getter(fn link_deposits)]
    pub type LinkDeposits<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        EthTxHash,
        (T::AccountId, BalanceOf<T>, BlockNumberFor<T>),
    >;

    /// LINK withdrawals from Ëtrid to Ethereum
    #[pallet::storage]
    #[pallet::getter(fn link_withdrawals)]
    pub type LinkWithdrawals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<(EthereumAddress, BalanceOf<T>, BlockNumberFor<T>), T::MaxOracleNodes>,
        ValueQuery,
    >;

    /// Registered Chainlink oracle nodes
    #[pallet::storage]
    #[pallet::getter(fn oracle_nodes)]
    pub type OracleNodes<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        EthereumAddress,
    >;

    /// Chainlink Data Feeds (e.g., LINK/USD, ETH/USD)
    #[pallet::storage]
    #[pallet::getter(fn data_feeds)]
    pub type DataFeeds<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        DataFeedId,
        OraclePrice,
    >;

    /// LINK ERC-20 contract address on Ethereum (ERC-677 compatible)
    #[pallet::storage]
    #[pallet::getter(fn link_token_address)]
    pub type LinkTokenAddress<T: Config> = StorageValue<_, EthereumAddress, ValueQuery>;

    /// Bridge status (active/paused)
    #[pallet::storage]
    #[pallet::getter(fn bridge_active)]
    pub type BridgeActive<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Total LINK locked in bridge
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Ethereum bridge contract address
    #[pallet::storage]
    #[pallet::getter(fn eth_bridge_contract)]
    pub type EthBridgeContract<T: Config> = StorageValue<_, EthereumAddress>;

    /// Whitelisted Chainlink operators for oracle submissions
    #[pallet::storage]
    #[pallet::getter(fn whitelisted_operators)]
    pub type WhitelistedOperators<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        bool,
        ValueQuery,
    >;

    /// VRF requests by request ID
    #[pallet::storage]
    #[pallet::getter(fn vrf_requests)]
    pub type VRFRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        H256,
        VRFRequest<T::AccountId>,
    >;

    /// Proof of Reserve attestations (token => total_supply)
    #[pallet::storage]
    #[pallet::getter(fn proof_of_reserve)]
    pub type ProofOfReserve<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        EthereumAddress,
        u128,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// LINK deposited from Ethereum [who, amount, eth_tx_hash]
        LinkDeposited {
            who: T::AccountId,
            amount: BalanceOf<T>,
            eth_tx_hash: EthTxHash,
        },
        /// LINK withdrawn to Ethereum [who, amount, eth_address]
        LinkWithdrawn {
            who: T::AccountId,
            amount: BalanceOf<T>,
            eth_address: EthereumAddress,
        },
        /// Oracle price updated [feed_id, price, timestamp, round_id]
        OraclePriceUpdated {
            feed_id: DataFeedId,
            price: u128,
            timestamp: u64,
            round_id: u64,
        },
        /// Oracle node registered [node, eth_address]
        OracleNodeRegistered {
            node: T::AccountId,
            eth_address: EthereumAddress,
        },
        /// Bridge status changed [is_active]
        BridgeStatusChanged { is_active: bool },
        /// LINK token address updated [address]
        LinkTokenAddressUpdated { address: EthereumAddress },
        /// Oracle operator whitelisted [operator]
        OperatorWhitelisted { operator: T::AccountId },
        /// Oracle operator removed [operator]
        OperatorRemoved { operator: T::AccountId },
        /// VRF request created [request_id, requester]
        VRFRequested {
            request_id: H256,
            requester: T::AccountId,
        },
        /// VRF request fulfilled [request_id, randomness]
        VRFFulfilled {
            request_id: H256,
            randomness: H256,
        },
        /// Proof of Reserve updated [token, total_supply]
        ProofOfReserveUpdated {
            token: EthereumAddress,
            total_supply: u128,
        },
        /// ERC-677 transferAndCall executed [from, to, value]
        ERC677TransferExecuted {
            from: T::AccountId,
            to: EthereumAddress,
            value: u128,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Bridge is currently paused
        BridgeInactive,
        /// Amount below minimum threshold
        AmountTooLow,
        /// Amount exceeds maximum threshold
        AmountTooHigh,
        /// Insufficient balance for bridge
        InsufficientBalance,
        /// Invalid Ethereum address format
        InvalidEthAddress,
        /// Deposit already processed
        DuplicateDeposit,
        /// Withdrawal not found
        WithdrawalNotFound,
        /// Not enough Ethereum confirmations
        InsufficientConfirmations,
        /// Not a registered oracle node
        NotOracleNode,
        /// Oracle price is stale
        StalePriceData,
        /// Invalid oracle price data
        InvalidPriceData,
        /// Not a whitelisted operator
        NotWhitelistedOperator,
        /// Bridge contract not set
        BridgeContractNotSet,
        /// Data feed not found
        DataFeedNotFound,
        /// Arithmetic overflow
        Overflow,
        /// Invalid amount (zero or negative)
        InvalidAmount,
        /// VRF request already exists
        VRFRequestExists,
        /// VRF request not found
        VRFRequestNotFound,
        /// Too many oracle nodes
        TooManyOracleNodes,
        /// Too many data feeds
        TooManyDataFeeds,
        /// Too many VRF requests
        TooManyVRFRequests,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deposit LINK from Ethereum blockchain (ERC-677 compatible)
        ///
        /// Called by bridge relayers after detecting ERC-677 transferAndCall to bridge contract
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn deposit_link(
            origin: OriginFor<T>,
            beneficiary: T::AccountId,
            amount: BalanceOf<T>,
            eth_tx_hash: EthTxHash,
        ) -> DispatchResult {
            let relayer = ensure_signed(origin)?;

            // Verify bridge is active
            ensure!(BridgeActive::<T>::get(), Error::<T>::BridgeInactive);

            // Verify relayer is registered oracle node
            ensure!(
                OracleNodes::<T>::contains_key(&relayer),
                Error::<T>::NotOracleNode
            );

            // Validate amount
            ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);

            // Prevent duplicate deposits
            ensure!(
                !LinkDeposits::<T>::contains_key(&eth_tx_hash),
                Error::<T>::DuplicateDeposit
            );

            // Calculate bridge fee
            let fee_rate = T::BridgeFeeRate::get();
            let fee_amount = amount * fee_rate.into() / 1000u32.into();
            let net_amount = amount - fee_amount;

            // Mint wrapped LINK to beneficiary
            let _ = T::Currency::deposit_creating(&beneficiary, net_amount);

            // Record deposit
            let current_block = frame_system::Pallet::<T>::block_number();
            LinkDeposits::<T>::insert(
                eth_tx_hash,
                (beneficiary.clone(), amount, current_block),
            );

            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_add(amount);
            });

            Self::deposit_event(Event::LinkDeposited {
                who: beneficiary,
                amount: net_amount,
                eth_tx_hash,
            });

            Ok(())
        }

        /// Withdraw LINK to Ethereum blockchain
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn withdraw_link(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
            eth_address: EthereumAddress,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify bridge is active
            ensure!(BridgeActive::<T>::get(), Error::<T>::BridgeInactive);

            // Verify bridge contract is set
            ensure!(
                EthBridgeContract::<T>::get().is_some(),
                Error::<T>::BridgeContractNotSet
            );

            // Validate amount
            ensure!(amount > Zero::zero(), Error::<T>::InvalidAmount);

            // Verify user has sufficient balance
            let balance = T::Currency::free_balance(&who);
            ensure!(balance >= amount, Error::<T>::InsufficientBalance);

            // Calculate bridge fee
            let fee_rate = T::BridgeFeeRate::get();
            let fee_amount = amount * fee_rate.into() / 1000u32.into();
            let net_amount = amount - fee_amount;

            // Burn wrapped LINK from user
            T::Currency::withdraw(
                &who,
                amount,
                frame_support::traits::WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;

            // Record withdrawal for relayer processing
            let current_block = frame_system::Pallet::<T>::block_number();
            LinkWithdrawals::<T>::try_mutate(&who, |withdrawals| {
                withdrawals.try_push((eth_address, net_amount, current_block))
                    .map_err(|_| Error::<T>::TooManyOracleNodes)
            })?;

            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_sub(amount);
            });

            Self::deposit_event(Event::LinkWithdrawn {
                who,
                amount: net_amount,
                eth_address,
            });

            Ok(())
        }

        /// Submit oracle price data from Chainlink Data Feed
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn submit_oracle_price(
            origin: OriginFor<T>,
            feed_id: DataFeedId,
            price: u128,
            timestamp: u64,
            round_id: u64,
            answered_in_round: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify submitter is whitelisted operator
            ensure!(
                WhitelistedOperators::<T>::get(&who),
                Error::<T>::NotWhitelistedOperator
            );

            // Validate price data
            ensure!(price > 0, Error::<T>::InvalidPriceData);

            // Create oracle price
            let price_data = OraclePrice {
                price,
                timestamp,
                round_id,
                answered_in_round,
            };

            // Store oracle price
            DataFeeds::<T>::insert(feed_id, price_data);

            Self::deposit_event(Event::OraclePriceUpdated {
                feed_id,
                price,
                timestamp,
                round_id,
            });

            Ok(())
        }

        /// Register as Chainlink oracle node
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn register_oracle_node(
            origin: OriginFor<T>,
            eth_address: EthereumAddress,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            OracleNodes::<T>::insert(&who, eth_address);

            Self::deposit_event(Event::OracleNodeRegistered {
                node: who,
                eth_address,
            });

            Ok(())
        }

        /// Set bridge active status (governance only)
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn set_bridge_status(origin: OriginFor<T>, is_active: bool) -> DispatchResult {
            ensure_root(origin)?;
            
            BridgeActive::<T>::put(is_active);
            
            Self::deposit_event(Event::BridgeStatusChanged { is_active });
            
            Ok(())
        }

        /// Update LINK token contract address (ERC-677)
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn update_link_token_address(
            origin: OriginFor<T>,
            address: EthereumAddress,
        ) -> DispatchResult {
            ensure_root(origin)?;

            LinkTokenAddress::<T>::put(address);

            Self::deposit_event(Event::LinkTokenAddressUpdated { address });

            Ok(())
        }

        /// Update Ethereum bridge contract address
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn update_bridge_contract(
            origin: OriginFor<T>,
            address: EthereumAddress,
        ) -> DispatchResult {
            ensure_root(origin)?;

            EthBridgeContract::<T>::put(address);

            Ok(())
        }

        /// Whitelist Chainlink operator for oracle submissions
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn whitelist_operator(origin: OriginFor<T>, operator: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            
            WhitelistedOperators::<T>::insert(&operator, true);
            
            Self::deposit_event(Event::OperatorWhitelisted { operator });
            
            Ok(())
        }

        /// Remove operator from whitelist
        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn remove_operator(origin: OriginFor<T>, operator: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;

            WhitelistedOperators::<T>::remove(&operator);

            Self::deposit_event(Event::OperatorRemoved { operator });

            Ok(())
        }

        /// Request VRF (Verifiable Random Function) randomness
        #[pallet::call_index(9)]
        #[pallet::weight(15_000)]
        pub fn request_vrf(
            origin: OriginFor<T>,
            key_hash: H256,
            seed: u64,
            fee: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Generate request ID from seed and requester
            let request_id = H256::from_slice(&sp_io::hashing::blake2_256(&(seed, &who).encode()));

            // Ensure request doesn't exist
            ensure!(!VRFRequests::<T>::contains_key(&request_id), Error::<T>::VRFRequestExists);

            // Create VRF request
            let request = VRFRequest {
                requester: who.clone(),
                key_hash,
                seed,
                fee,
                is_fulfilled: false,
            };

            VRFRequests::<T>::insert(&request_id, request);

            Self::deposit_event(Event::VRFRequested {
                request_id,
                requester: who,
            });

            Ok(())
        }

        /// Fulfill VRF request (oracle only)
        #[pallet::call_index(10)]
        #[pallet::weight(15_000)]
        pub fn fulfill_vrf(
            origin: OriginFor<T>,
            request_id: H256,
            randomness: H256,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify oracle node
            ensure!(
                WhitelistedOperators::<T>::get(&who),
                Error::<T>::NotWhitelistedOperator
            );

            // Get request
            let mut request = VRFRequests::<T>::get(&request_id)
                .ok_or(Error::<T>::VRFRequestNotFound)?;

            // Mark as fulfilled
            request.is_fulfilled = true;
            VRFRequests::<T>::insert(&request_id, request);

            Self::deposit_event(Event::VRFFulfilled {
                request_id,
                randomness,
            });

            Ok(())
        }

        /// Update Proof of Reserve for a token (oracle only)
        #[pallet::call_index(11)]
        #[pallet::weight(10_000)]
        pub fn update_proof_of_reserve(
            origin: OriginFor<T>,
            token: EthereumAddress,
            total_supply: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify oracle node
            ensure!(
                WhitelistedOperators::<T>::get(&who),
                Error::<T>::NotWhitelistedOperator
            );

            ProofOfReserve::<T>::insert(&token, total_supply);

            Self::deposit_event(Event::ProofOfReserveUpdated {
                token,
                total_supply,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get latest oracle price for a data feed
        pub fn get_oracle_price(feed_id: DataFeedId) -> Option<OraclePrice> {
            DataFeeds::<T>::get(feed_id)
        }

        /// Check if oracle price is stale
        pub fn is_price_stale(price: &OraclePrice, current_block: BlockNumberFor<T>) -> bool {
            let staleness_threshold = T::PriceStalenessThreshold::get();
            let current_block_u32: u32 = current_block.saturated_into();

            if let Some(block_diff) = current_block_u32.checked_sub(price.timestamp as u32) {
                block_diff > staleness_threshold
            } else {
                true
            }
        }

        /// Get VRF request by ID
        pub fn get_vrf_request(request_id: H256) -> Option<VRFRequest<T::AccountId>> {
            VRFRequests::<T>::get(request_id)
        }

        /// Get Proof of Reserve for a token
        pub fn get_proof_of_reserve(token: EthereumAddress) -> u128 {
            ProofOfReserve::<T>::get(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, parameter_types};
    use sp_core::H256;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test {
            System: frame_system,
            Balances: pallet_balances,
            LinkBridge: pallet,
        }
    );

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
    }

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<u64>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = frame_support::traits::ConstU32<16>;
        type Nonce = u64;
        type Block = Block;
        type RuntimeTask = ();
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
    }

    parameter_types! {
        pub const ExistentialDeposit: u64 = 1;
    }

    impl pallet_balances::Config for Test {
        type MaxLocks = ();
        type MaxReserves = ();
        type ReserveIdentifier = [u8; 8];
        type Balance = u64;
        type RuntimeEvent = RuntimeEvent;
        type DustRemoval = ();
        type ExistentialDeposit = ExistentialDeposit;
        type AccountStore = System;
        type WeightInfo = ();
        type FreezeIdentifier = ();
        type MaxFreezes = ();
        type RuntimeHoldReason = ();
        type RuntimeFreezeReason = ();
    }

    parameter_types! {
        pub const MinConfirmations: u32 = 12;
        pub const BridgeFeeRate: u32 = 10; // 1% = 10
        pub const MaxOracleNodes: u32 = 100;
        pub const MaxDataFeeds: u32 = 50;
        pub const MaxVRFRequests: u32 = 100;
        pub const PriceStalenessThreshold: u32 = 300; // 5 minutes (assuming 1 block/second)
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type MinConfirmations = MinConfirmations;
        type BridgeFeeRate = BridgeFeeRate;
        type MaxOracleNodes = MaxOracleNodes;
        type MaxDataFeeds = MaxDataFeeds;
        type MaxVRFRequests = MaxVRFRequests;
        type PriceStalenessThreshold = PriceStalenessThreshold;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        t.into()
    }

    #[test]
    fn test_oracle_node_registration() {
        new_test_ext().execute_with(|| {
            let oracle = 1u64;
            let eth_addr = [1u8; 20];
            
            assert_ok!(LinkBridge::register_oracle_node(
                RuntimeOrigin::signed(oracle),
                eth_addr
            ));
            
            assert!(pallet::OracleNodes::<Test>::contains_key(oracle));
        });
    }

    #[test]
    fn test_oracle_price_submission() {
        new_test_ext().execute_with(|| {
            let operator = 1u64;
            let feed_id = [0u8; 32];
            let price_data = pallet::OraclePrice {
                price: 15_000_000_000_000_000_000, // $15 USD
                timestamp: 1234567890,
                round_id: 100,
                answered_in_round: 100,
            };
            
            // Whitelist operator
            assert_ok!(LinkBridge::whitelist_operator(RuntimeOrigin::root(), operator));
            
            // Submit price
            assert_ok!(LinkBridge::submit_oracle_price(
                RuntimeOrigin::signed(operator),
                feed_id,
                price_data.clone()
            ));
            
            // Verify price stored
            assert_eq!(pallet::DataFeeds::<Test>::get(feed_id), Some(price_data));
        });
    }
}
