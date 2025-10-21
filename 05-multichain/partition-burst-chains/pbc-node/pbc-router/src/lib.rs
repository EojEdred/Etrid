#![cfg_attr(not(feature = "std"), no_std)]

//! # PBC Router Pallet
//!
//! ## Overview
//!
//! The PBC Router pallet manages cross-chain communication between FlareChain and
//! the 12 Partition Burst Chains (PBCs). It provides:
//! 
//! - **State Aggregation**: Tracks state roots from all 12 PBCs
//! - **Message Routing**: Routes cross-chain messages between PBCs
//! - **PBC Registry**: Maintains metadata about each PBC
//! - **Multichain State Root**: Computes aggregate state across all chains
//!
//! ## Architecture
//!
//! ```text
//! FlareChain (Relay Chain)
//!     ↕
//! PBC Router Pallet
//!     ↕
//! 12 PBC Collators (BTC, ETH, TRX, SOL, XRP, BNB, USDT, ADA, DOGE, LINK, XLM, MATIC)
//! ```

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_core::H256;
    use sp_runtime::traits::Hash;
    use sp_std::vec::Vec;

    /// PBC identifier (0-11 for the 12 chains)
    pub type PbcId = u8;

    /// Block number type
    pub type PbcBlockNumber = u32;

    /// PBC metadata
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct PbcInfo {
        /// Human-readable name
        pub name: BoundedVec<u8, ConstU32<32>>,
        /// Latest known block number
        pub latest_block: PbcBlockNumber,
        /// Latest state root
        pub latest_state_root: H256,
        /// Whether PBC is active
        pub is_active: bool,
        /// Last update timestamp
        pub last_update: u64,
    }

    /// Cross-chain message
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct CrossChainMessage {
        /// Source PBC ID
        pub from_pbc: PbcId,
        /// Destination PBC ID (255 = FlareChain)
        pub to_pbc: PbcId,
        /// Message nonce (for ordering)
        pub nonce: u64,
        /// Message payload
        pub payload: BoundedVec<u8, ConstU32<1024>>,
    }

    /// State root submission from a PBC
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct StateRootSubmission {
        /// PBC ID
        pub pbc_id: PbcId,
        /// Block number this state root is for
        pub block_number: PbcBlockNumber,
        /// State root hash
        pub state_root: H256,
        /// Timestamp of submission
        pub timestamp: u64,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Maximum number of PBCs (default: 12)
        #[pallet::constant]
        type MaxPbcs: Get<u32>;

        /// Maximum pending messages per PBC
        #[pallet::constant]
        type MaxPendingMessages: Get<u32>;

        /// Maximum message payload size (bytes)
        #[pallet::constant]
        type MaxMessageSize: Get<u32>;

        /// Origin that can register new PBCs (typically root)
        type RegisterOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Origin that can submit state roots (collator nodes)
        type CollatorOrigin: EnsureOrigin<Self::RuntimeOrigin>;
    }

    // ==================== STORAGE ====================

    /// Registry of all PBCs
    /// 
    /// Maps PBC ID → PBC Info
    #[pallet::storage]
    #[pallet::getter(fn pbc_info)]
    pub type PbcRegistry<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PbcId,
        PbcInfo,
        OptionQuery,
    >;

    /// Latest state roots for each PBC
    /// 
    /// Maps PBC ID → (Block Number, State Root)
    #[pallet::storage]
    #[pallet::getter(fn state_root)]
    pub type StateRoots<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PbcId,
        (PbcBlockNumber, H256),
        ValueQuery,
    >;

    /// Aggregated multichain state root
    ///
    /// This is computed as Hash(StateRoot_0 + StateRoot_1 + ... + StateRoot_11)
    #[pallet::storage]
    #[pallet::getter(fn multichain_state_root)]
    pub type MultichainStateRoot<T: Config> = StorageValue<_, T::Hash, ValueQuery>;

    /// Pending cross-chain messages
    /// 
    /// Maps Destination PBC ID → List of pending messages
    #[pallet::storage]
    #[pallet::getter(fn pending_messages)]
    pub type PendingMessages<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PbcId,
        BoundedVec<CrossChainMessage, T::MaxPendingMessages>,
        ValueQuery,
    >;

    /// Message nonce counter (for ordering)
    #[pallet::storage]
    #[pallet::getter(fn message_nonce)]
    pub type MessageNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Historical state root submissions (for verification)
    #[pallet::storage]
    #[pallet::getter(fn state_history)]
    pub type StateHistory<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        PbcId,
        Blake2_128Concat,
        PbcBlockNumber,
        H256,
        OptionQuery,
    >;

    // ==================== GENESIS ====================
    // Genesis configuration removed for now - PBCs will be registered via extrinsics

    // ==================== EVENTS ====================

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New PBC registered
        PbcRegistered {
            pbc_id: PbcId,
            name: Vec<u8>,
        },
        /// PBC state root submitted
        StateRootSubmitted {
            pbc_id: PbcId,
            block_number: PbcBlockNumber,
            state_root: H256,
        },
        /// Multichain state root updated
        MultichainStateUpdated {
            new_root: T::Hash,
        },
        /// Cross-chain message sent
        MessageSent {
            from_pbc: PbcId,
            to_pbc: PbcId,
            nonce: u64,
        },
        /// Cross-chain message received
        MessageReceived {
            from_pbc: PbcId,
            to_pbc: PbcId,
            nonce: u64,
        },
        /// PBC activated
        PbcActivated {
            pbc_id: PbcId,
        },
        /// PBC deactivated
        PbcDeactivated {
            pbc_id: PbcId,
        },
    }

    // ==================== ERRORS ====================

    #[pallet::error]
    pub enum Error<T> {
        /// PBC ID already registered
        PbcAlreadyExists,
        /// PBC not found
        PbcNotFound,
        /// PBC is inactive
        PbcInactive,
        /// Invalid PBC ID (must be 0-11)
        InvalidPbcId,
        /// State root submission out of order
        InvalidBlockNumber,
        /// Message queue full
        MessageQueueFull,
        /// Message too large
        MessageTooLarge,
        /// Invalid destination PBC
        InvalidDestination,
        /// Unauthorized origin
        Unauthorized,
    }

    // ==================== CALLS ====================

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new PBC
        /// 
        /// This is called during chain initialization to register each of the 12 PBCs.
        /// 
        /// # Arguments
        /// * `pbc_id` - Unique identifier (0-11)
        /// * `name` - Human-readable name (e.g., "BTC-PBC")
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn register_pbc(
            origin: OriginFor<T>,
            pbc_id: PbcId,
            name: Vec<u8>,
        ) -> DispatchResult {
            T::RegisterOrigin::ensure_origin(origin)?;

            // Validate PBC ID
            ensure!(pbc_id < 12, Error::<T>::InvalidPbcId);

            // Check if already registered
            ensure!(
                !PbcRegistry::<T>::contains_key(pbc_id),
                Error::<T>::PbcAlreadyExists
            );

            let bounded_name: BoundedVec<u8, ConstU32<32>> = name
                .clone()
                .try_into()
                .map_err(|_| Error::<T>::MessageTooLarge)?;

            let info = PbcInfo {
                name: bounded_name,
                latest_block: 0,
                latest_state_root: H256::zero(),
                is_active: true,
                last_update: 0,
            };

            PbcRegistry::<T>::insert(pbc_id, info);

            Self::deposit_event(Event::PbcRegistered { pbc_id, name });

            Ok(())
        }

        /// Submit a PBC state root
        /// 
        /// Called by PBC collator nodes to submit state roots to FlareChain.
        /// This triggers multichain state root recomputation.
        /// 
        /// # Arguments
        /// * `pbc_id` - Which PBC this state is from
        /// * `block_number` - Block number on the PBC
        /// * `state_root` - State root hash
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn submit_state_root(
            origin: OriginFor<T>,
            pbc_id: PbcId,
            block_number: PbcBlockNumber,
            state_root: H256,
        ) -> DispatchResult {
            T::CollatorOrigin::ensure_origin(origin)?;

            // Validate PBC exists and is active
            let mut info = PbcRegistry::<T>::get(pbc_id).ok_or(Error::<T>::PbcNotFound)?;
            ensure!(info.is_active, Error::<T>::PbcInactive);

            // Verify block number is advancing
            ensure!(
                block_number > info.latest_block,
                Error::<T>::InvalidBlockNumber
            );

            // Update PBC info
            info.latest_block = block_number;
            info.latest_state_root = state_root;
            info.last_update = Self::get_timestamp();
            PbcRegistry::<T>::insert(pbc_id, info);

            // Store state root
            StateRoots::<T>::insert(pbc_id, (block_number, state_root));

            // Store in history
            StateHistory::<T>::insert(pbc_id, block_number, state_root);

            // Recompute multichain state root
            Self::update_multichain_state_root()?;

            Self::deposit_event(Event::StateRootSubmitted {
                pbc_id,
                block_number,
                state_root,
            });

            Ok(())
        }

        /// Send cross-chain message
        /// 
        /// Routes a message from one PBC to another (or to FlareChain).
        /// 
        /// # Arguments
        /// * `from_pbc` - Source PBC ID
        /// * `to_pbc` - Destination PBC ID (255 = FlareChain)
        /// * `payload` - Message data
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn send_message(
            origin: OriginFor<T>,
            from_pbc: PbcId,
            to_pbc: PbcId,
            payload: Vec<u8>,
        ) -> DispatchResult {
            T::CollatorOrigin::ensure_origin(origin)?;

            // Validate PBCs
            ensure!(
                PbcRegistry::<T>::contains_key(from_pbc),
                Error::<T>::PbcNotFound
            );
            ensure!(
                to_pbc == 255 || PbcRegistry::<T>::contains_key(to_pbc),
                Error::<T>::InvalidDestination
            );

            // Check payload size
            ensure!(
                payload.len() <= T::MaxMessageSize::get() as usize,
                Error::<T>::MessageTooLarge
            );

            let bounded_payload: BoundedVec<u8, ConstU32<1024>> = payload
                .try_into()
                .map_err(|_| Error::<T>::MessageTooLarge)?;

            // Generate nonce
            let nonce = MessageNonce::<T>::get();
            MessageNonce::<T>::put(nonce + 1);

            let message = CrossChainMessage {
                from_pbc,
                to_pbc,
                nonce,
                payload: bounded_payload,
            };

            // Add to pending messages for destination
            PendingMessages::<T>::try_mutate(to_pbc, |messages| {
                messages
                    .try_push(message.clone())
                    .map_err(|_| Error::<T>::MessageQueueFull)
            })?;

            Self::deposit_event(Event::MessageSent {
                from_pbc,
                to_pbc,
                nonce,
            });

            Ok(())
        }

        /// Process pending message
        /// 
        /// Called by destination PBC collator to process a cross-chain message.
        /// 
        /// # Arguments
        /// * `pbc_id` - Destination PBC that's processing the message
        /// * `nonce` - Message nonce to process
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn process_message(
            origin: OriginFor<T>,
            pbc_id: PbcId,
            nonce: u64,
        ) -> DispatchResult {
            T::CollatorOrigin::ensure_origin(origin)?;

            // Find and remove message
            PendingMessages::<T>::try_mutate(pbc_id, |messages| {
                if let Some(pos) = messages.iter().position(|m| m.nonce == nonce) {
                    let message = messages.remove(pos);

                    Self::deposit_event(Event::MessageReceived {
                        from_pbc: message.from_pbc,
                        to_pbc: message.to_pbc,
                        nonce: message.nonce,
                    });

                    Ok(())
                } else {
                    Err(Error::<T>::PbcNotFound)
                }
            })?;

            Ok(())
        }

        /// Activate a PBC
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn activate_pbc(origin: OriginFor<T>, pbc_id: PbcId) -> DispatchResult {
            T::RegisterOrigin::ensure_origin(origin)?;

            PbcRegistry::<T>::try_mutate(pbc_id, |maybe_info| {
                let info = maybe_info.as_mut().ok_or(Error::<T>::PbcNotFound)?;
                info.is_active = true;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::PbcActivated { pbc_id });

            Ok(())
        }

        /// Deactivate a PBC
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn deactivate_pbc(origin: OriginFor<T>, pbc_id: PbcId) -> DispatchResult {
            T::RegisterOrigin::ensure_origin(origin)?;

            PbcRegistry::<T>::try_mutate(pbc_id, |maybe_info| {
                let info = maybe_info.as_mut().ok_or(Error::<T>::PbcNotFound)?;
                info.is_active = false;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::PbcDeactivated { pbc_id });

            Ok(())
        }
    }

    // ==================== HELPERS ====================

    impl<T: Config> Pallet<T> {
        /// Get current timestamp
        fn get_timestamp() -> u64 {
            // In production, get from pallet_timestamp
            // For now, return a placeholder
            0
        }

        /// Recompute multichain state root
        /// 
        /// Aggregates state roots from all 12 PBCs into a single merkle root.
        fn update_multichain_state_root() -> DispatchResult {
            let mut combined_data = Vec::new();

            // Collect all state roots in order (PBC 0-11)
            for pbc_id in 0..12u8 {
                let (_, state_root) = StateRoots::<T>::get(pbc_id);
                combined_data.extend_from_slice(state_root.as_ref());
            }

            // Compute aggregate hash
            let new_root = T::Hashing::hash(&combined_data);

            MultichainStateRoot::<T>::put(new_root);

            Self::deposit_event(Event::MultichainStateUpdated { new_root });

            Ok(())
        }

        /// Get all active PBCs
        pub fn get_active_pbcs() -> Vec<(PbcId, PbcInfo)> {
            PbcRegistry::<T>::iter()
                .filter(|(_, info)| info.is_active)
                .collect()
        }

        /// Verify a state root for a specific PBC block
        pub fn verify_state_root(
            pbc_id: PbcId,
            block_number: PbcBlockNumber,
            claimed_root: H256,
        ) -> bool {
            StateHistory::<T>::get(pbc_id, block_number)
                .map(|stored_root| stored_root == claimed_root)
                .unwrap_or(false)
        }
    }
}