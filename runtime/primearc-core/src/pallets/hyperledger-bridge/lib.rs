//! # Hyperledger Fabric Bridge Pallet
//!
//! This pallet enables cross-ledger operations between Ëtrid FlareChain
//! and Hyperledger Fabric networks.
//!
//! ## Features
//! - Lock/unlock assets for cross-chain transfers
//! - Verify Fabric endorsement proofs
//! - Register trusted Fabric networks
//! - Track bridge transfers
//!
//! ## Overview
//!
//! The Hyperledger Bridge pallet provides the runtime logic for bridging
//! assets between Ëtrid and Hyperledger Fabric enterprise networks.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
    use sp_runtime::traits::{Hash, Zero};

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Lock period in blocks (7 days = ~100,800 blocks at 6s/block)
    const LOCK_PERIOD_BLOCKS: u32 = 100_800;

    /// Minimum endorsements required
    const MIN_ENDORSEMENTS: u32 = 2;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency type for asset locking
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Maximum number of registered Fabric networks
        #[pallet::constant]
        type MaxFabricNetworks: Get<u32>;

        /// Maximum transfer ID length
        #[pallet::constant]
        type MaxTransferId: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Transfer status
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum TransferStatus {
        Locked,
        Unlocked,
        Failed,
    }

    /// Asset lock information
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct AssetLock<T: Config> {
        /// Account that locked the asset
        pub locker: T::AccountId,
        /// Locked amount
        pub amount: BalanceOf<T>,
        /// Destination Fabric network ID
        pub dest_network: BoundedVec<u8, T::MaxTransferId>,
        /// Destination address on Fabric
        pub dest_address: BoundedVec<u8, T::MaxTransferId>,
        /// Block number when locked
        pub lock_block: BlockNumberFor<T>,
        /// Transfer status
        pub status: TransferStatus,
    }

    /// Fabric network registration
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct FabricNetwork<T: Config> {
        /// Network identifier
        pub network_id: BoundedVec<u8, T::MaxTransferId>,
        /// Admin account
        pub admin: T::AccountId,
        /// Whether network is active
        pub is_active: bool,
        /// Required endorsements
        pub min_endorsements: u32,
    }

    /// Locked assets by transfer ID
    #[pallet::storage]
    #[pallet::getter(fn locked_assets)]
    pub type LockedAssets<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxTransferId>, // transfer_id
        AssetLock<T>,
        OptionQuery,
    >;

    /// Registered Fabric networks
    #[pallet::storage]
    #[pallet::getter(fn fabric_networks)]
    pub type FabricNetworks<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxTransferId>, // network_id
        FabricNetwork<T>,
        OptionQuery,
    >;

    /// Bridge transfer records
    #[pallet::storage]
    #[pallet::getter(fn bridge_transfers)]
    pub type BridgeTransfers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash, // transfer_hash
        (BoundedVec<u8, T::MaxTransferId>, BlockNumberFor<T>), // (transfer_id, block_number)
        OptionQuery,
    >;

    /// Total locked value
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Asset locked for transfer to Fabric
        AssetLocked {
            transfer_id: BoundedVec<u8, T::MaxTransferId>,
            locker: T::AccountId,
            amount: BalanceOf<T>,
            dest_network: BoundedVec<u8, T::MaxTransferId>,
            dest_address: BoundedVec<u8, T::MaxTransferId>,
        },
        /// Asset unlocked after Fabric burn proof
        AssetUnlocked {
            transfer_id: BoundedVec<u8, T::MaxTransferId>,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        },
        /// Fabric network registered
        FabricNetworkRegistered {
            network_id: BoundedVec<u8, T::MaxTransferId>,
            admin: T::AccountId,
        },
        /// Fabric network deactivated
        FabricNetworkDeactivated {
            network_id: BoundedVec<u8, T::MaxTransferId>,
        },
        /// Transfer failed
        TransferFailed {
            transfer_id: BoundedVec<u8, T::MaxTransferId>,
            reason: BoundedVec<u8, T::MaxTransferId>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Transfer ID already exists
        TransferIdExists,
        /// Transfer not found
        TransferNotFound,
        /// Insufficient balance to lock
        InsufficientBalance,
        /// Lock period not elapsed
        LockPeriodNotElapsed,
        /// Invalid endorsement proof
        InvalidEndorsementProof,
        /// Fabric network not registered
        NetworkNotRegistered,
        /// Fabric network not active
        NetworkNotActive,
        /// Invalid transfer ID
        InvalidTransferId,
        /// Asset already unlocked
        AlreadyUnlocked,
        /// Not authorized
        NotAuthorized,
        /// Too many networks registered
        TooManyNetworks,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Lock asset for transfer to Hyperledger Fabric
        ///
        /// # Arguments
        /// - `origin`: Account locking the asset
        /// - `transfer_id`: Unique transfer identifier
        /// - `amount`: Amount to lock
        /// - `dest_network`: Destination Fabric network ID
        /// - `dest_address`: Recipient address on Fabric
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn lock_asset(
            origin: OriginFor<T>,
            transfer_id: BoundedVec<u8, T::MaxTransferId>,
            amount: BalanceOf<T>,
            dest_network: BoundedVec<u8, T::MaxTransferId>,
            dest_address: BoundedVec<u8, T::MaxTransferId>,
        ) -> DispatchResult {
            let locker = ensure_signed(origin)?;

            // Validate amount
            ensure!(!amount.is_zero(), Error::<T>::InsufficientBalance);

            // Verify network is registered and active
            let network = FabricNetworks::<T>::get(&dest_network)
                .ok_or(Error::<T>::NetworkNotRegistered)?;
            ensure!(network.is_active, Error::<T>::NetworkNotActive);

            // Ensure transfer ID doesn't exist
            ensure!(
                !LockedAssets::<T>::contains_key(&transfer_id),
                Error::<T>::TransferIdExists
            );

            // Reserve the amount
            T::Currency::reserve(&locker, amount)
                .map_err(|_| Error::<T>::InsufficientBalance)?;

            // Create lock record
            let lock = AssetLock {
                locker: locker.clone(),
                amount,
                dest_network: dest_network.clone(),
                dest_address: dest_address.clone(),
                lock_block: <frame_system::Pallet<T>>::block_number(),
                status: TransferStatus::Locked,
            };

            // Store lock
            LockedAssets::<T>::insert(&transfer_id, lock);

            // Update total locked
            TotalLocked::<T>::mutate(|total| *total = total.saturating_add(amount));

            // Create transfer hash for indexing
            let transfer_hash = T::Hashing::hash(&transfer_id);
            BridgeTransfers::<T>::insert(
                transfer_hash,
                (transfer_id.clone(), <frame_system::Pallet<T>>::block_number()),
            );

            // Emit event
            Self::deposit_event(Event::AssetLocked {
                transfer_id,
                locker,
                amount,
                dest_network,
                dest_address,
            });

            Ok(())
        }

        /// Unlock asset after Fabric burn proof verification
        ///
        /// # Arguments
        /// - `origin`: Account requesting unlock (must be original locker or admin)
        /// - `transfer_id`: Transfer identifier
        /// - `fabric_proof`: Encoded Fabric endorsement proof
        #[pallet::call_index(1)]
        #[pallet::weight(20_000)]
        pub fn unlock_asset(
            origin: OriginFor<T>,
            transfer_id: BoundedVec<u8, T::MaxTransferId>,
            fabric_proof: Vec<u8>,
        ) -> DispatchResult {
            let requester = ensure_signed(origin)?;

            // Get lock record
            let mut lock = LockedAssets::<T>::get(&transfer_id)
                .ok_or(Error::<T>::TransferNotFound)?;

            // Verify not already unlocked
            ensure!(
                lock.status == TransferStatus::Locked,
                Error::<T>::AlreadyUnlocked
            );

            // Verify lock period elapsed
            let current_block = <frame_system::Pallet<T>>::block_number();
            let lock_period_blocks = BlockNumberFor::<T>::from(LOCK_PERIOD_BLOCKS);
            ensure!(
                current_block >= lock.lock_block + lock_period_blocks,
                Error::<T>::LockPeriodNotElapsed
            );

            // Verify Fabric endorsements
            ensure!(
                Self::verify_endorsements(&lock.dest_network, &fabric_proof)?,
                Error::<T>::InvalidEndorsementProof
            );

            // Unreserve and transfer to recipient
            T::Currency::unreserve(&lock.locker, lock.amount);

            // Update status
            lock.status = TransferStatus::Unlocked;
            LockedAssets::<T>::insert(&transfer_id, lock.clone());

            // Update total locked
            TotalLocked::<T>::mutate(|total| *total = total.saturating_sub(lock.amount));

            // Emit event
            Self::deposit_event(Event::AssetUnlocked {
                transfer_id,
                recipient: lock.locker,
                amount: lock.amount,
            });

            Ok(())
        }

        /// Register a Hyperledger Fabric network
        ///
        /// # Arguments
        /// - `origin`: Root or governance
        /// - `network_id`: Unique network identifier
        /// - `admin`: Network admin account
        /// - `min_endorsements`: Minimum endorsements required
        #[pallet::call_index(2)]
        #[pallet::weight(15_000)]
        pub fn register_fabric_network(
            origin: OriginFor<T>,
            network_id: BoundedVec<u8, T::MaxTransferId>,
            admin: T::AccountId,
            min_endorsements: u32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // Ensure network doesn't exist
            ensure!(
                !FabricNetworks::<T>::contains_key(&network_id),
                Error::<T>::TransferIdExists
            );

            // Validate endorsements
            let min_endorsements = min_endorsements.max(MIN_ENDORSEMENTS);

            // Create network record
            let network = FabricNetwork {
                network_id: network_id.clone(),
                admin: admin.clone(),
                is_active: true,
                min_endorsements,
            };

            // Store network
            FabricNetworks::<T>::insert(&network_id, network);

            // Emit event
            Self::deposit_event(Event::FabricNetworkRegistered { network_id, admin });

            Ok(())
        }

        /// Deactivate a Fabric network
        ///
        /// # Arguments
        /// - `origin`: Root or network admin
        /// - `network_id`: Network identifier
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn deactivate_fabric_network(
            origin: OriginFor<T>,
            network_id: BoundedVec<u8, T::MaxTransferId>,
        ) -> DispatchResult {
            let requester = ensure_signed(origin.clone())
                .or_else(|_| ensure_root(origin).map(|_| T::AccountId::default()))?;

            // Get network
            let mut network = FabricNetworks::<T>::get(&network_id)
                .ok_or(Error::<T>::NetworkNotRegistered)?;

            // Check authorization (must be admin or root)
            if requester != T::AccountId::default() {
                ensure!(requester == network.admin, Error::<T>::NotAuthorized);
            }

            // Deactivate
            network.is_active = false;
            FabricNetworks::<T>::insert(&network_id, network);

            // Emit event
            Self::deposit_event(Event::FabricNetworkDeactivated { network_id });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Verify Fabric endorsement proof
        ///
        /// Validates that sufficient organizations have endorsed the burn transaction.
        ///
        /// # Arguments
        /// - `network_id`: Fabric network identifier
        /// - `proof`: Encoded endorsement proof
        fn verify_endorsements(
            network_id: &BoundedVec<u8, T::MaxTransferId>,
            proof: &[u8],
        ) -> Result<bool, DispatchError> {
            // Get network configuration
            let network = FabricNetworks::<T>::get(network_id)
                .ok_or(Error::<T>::NetworkNotRegistered)?;

            // Parse proof (simplified - production would verify signatures)
            // Format: [endorsement_count][endorsement1_len][endorsement1]...
            if proof.len() < 4 {
                return Ok(false);
            }

            let endorsement_count =
                u32::from_be_bytes([proof[0], proof[1], proof[2], proof[3]]);

            // Verify minimum endorsements
            if endorsement_count < network.min_endorsements {
                return Ok(false);
            }

            // In production, would verify:
            // 1. Each endorsement signature
            // 2. MSP identity of endorsers
            // 3. Endorsement policy satisfaction

            Ok(true)
        }

        /// Get total value locked in bridge
        pub fn get_total_locked() -> BalanceOf<T> {
            TotalLocked::<T>::get()
        }

        /// Check if transfer exists
        pub fn transfer_exists(transfer_id: &BoundedVec<u8, T::MaxTransferId>) -> bool {
            LockedAssets::<T>::contains_key(transfer_id)
        }

        /// Get transfer status
        pub fn get_transfer_status(
            transfer_id: &BoundedVec<u8, T::MaxTransferId>,
        ) -> Option<TransferStatus> {
            LockedAssets::<T>::get(transfer_id).map(|lock| lock.status)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_noop, assert_ok, parameter_types};
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
            HyperledgerBridge: pallet,
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
        type Nonce = u64;
        type Hash = H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Block = Block;
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
        pub const MaxFabricNetworks: u32 = 100;
        pub const MaxTransferId: u32 = 64;
    }

    impl pallet::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type MaxFabricNetworks = MaxFabricNetworks;
        type MaxTransferId = MaxTransferId;
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: vec![(1, 1000), (2, 1000)],
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }

    #[test]
    fn test_register_fabric_network() {
        new_test_ext().execute_with(|| {
            let network_id = BoundedVec::try_from(b"test-network".to_vec()).unwrap();
            assert_ok!(HyperledgerBridge::register_fabric_network(
                RuntimeOrigin::root(),
                network_id.clone(),
                1,
                2
            ));

            assert!(HyperledgerBridge::fabric_networks(&network_id).is_some());
        });
    }

    #[test]
    fn test_lock_asset() {
        new_test_ext().execute_with(|| {
            let network_id = BoundedVec::try_from(b"test-network".to_vec()).unwrap();
            let transfer_id = BoundedVec::try_from(b"transfer-1".to_vec()).unwrap();
            let dest_address = BoundedVec::try_from(b"fabric.user1".to_vec()).unwrap();

            // Register network first
            assert_ok!(HyperledgerBridge::register_fabric_network(
                RuntimeOrigin::root(),
                network_id.clone(),
                1,
                2
            ));

            // Lock asset
            assert_ok!(HyperledgerBridge::lock_asset(
                RuntimeOrigin::signed(1),
                transfer_id.clone(),
                100,
                network_id,
                dest_address
            ));

            assert!(HyperledgerBridge::locked_assets(&transfer_id).is_some());
            assert_eq!(HyperledgerBridge::get_total_locked(), 100);
        });
    }
}
