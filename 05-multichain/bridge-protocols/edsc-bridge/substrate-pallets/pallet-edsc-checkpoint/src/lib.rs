//! # EDSC Checkpoint Pallet
//!
//! Posts state commitments from PBC-EDSC to FlareChain for cross-chain synchronization.
//!
//! ## Overview
//!
//! This pallet creates periodic checkpoints of the PBC-EDSC state that can be verified
//! on FlareChain. Each checkpoint includes:
//! - Merkle root of current state
//! - Block number
//! - Total EDSC supply
//! - Reserve ratio snapshot
//! - Validator signatures
//!
//! Checkpoints are created every N blocks (configurable via CheckpointInterval).

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::Get,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Hash;
	use sp_std::vec::Vec;

	/// Checkpoint data structure
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Checkpoint<BlockNumber, Hash> {
		/// Block number when checkpoint was created
		pub block_number: BlockNumber,
		/// Merkle root of PBC-EDSC state
		pub state_root: Hash,
		/// Total EDSC supply at checkpoint
		pub total_supply: u128,
		/// Reserve ratio snapshot (in basis points, e.g., 10000 = 100%)
		pub reserve_ratio: u16,
		/// Timestamp of checkpoint creation
		pub timestamp: u64,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Checkpoint creation interval (in blocks)
		#[pallet::constant]
		type CheckpointInterval: Get<BlockNumberFor<Self>>;

		/// Maximum number of checkpoints to store
		#[pallet::constant]
		type MaxCheckpoints: Get<u32>;

		/// Minimum reserve ratio threshold (basis points) before emergency checkpoint
		#[pallet::constant]
		type EmergencyReserveThreshold: Get<u16>;
	}

	/// Storage for checkpoints by block number
	#[pallet::storage]
	#[pallet::getter(fn checkpoints)]
	pub type Checkpoints<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		Checkpoint<BlockNumberFor<T>, T::Hash>,
		OptionQuery,
	>;

	/// Latest checkpoint block number
	#[pallet::storage]
	#[pallet::getter(fn latest_checkpoint)]
	pub type LatestCheckpoint<T: Config> = StorageValue<_, BlockNumberFor<T>, OptionQuery>;

	/// Total number of checkpoints created
	#[pallet::storage]
	#[pallet::getter(fn checkpoint_count)]
	pub type CheckpointCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Emergency checkpoint flag
	#[pallet::storage]
	#[pallet::getter(fn emergency_mode)]
	pub type EmergencyMode<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Checkpoint created [block_number, state_root, total_supply, reserve_ratio]
		CheckpointCreated {
			block_number: BlockNumberFor<T>,
			state_root: T::Hash,
			total_supply: u128,
			reserve_ratio: u16,
		},
		/// Emergency checkpoint created due to low reserves
		EmergencyCheckpoint {
			block_number: BlockNumberFor<T>,
			reserve_ratio: u16,
		},
		/// Checkpoint verified and accepted
		CheckpointVerified {
			block_number: BlockNumberFor<T>,
		},
		/// Emergency mode activated
		EmergencyModeActivated,
		/// Emergency mode deactivated
		EmergencyModeDeactivated,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Checkpoint already exists for this block
		CheckpointAlreadyExists,
		/// Maximum checkpoints reached
		MaxCheckpointsReached,
		/// Checkpoint not found
		CheckpointNotFound,
		/// Invalid checkpoint data
		InvalidCheckpoint,
		/// Reserve ratio too low
		ReserveRatioTooLow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: BlockNumberFor<T>) {
			// Check if we need to create a checkpoint
			if Self::should_create_checkpoint(n) {
				let _ = Self::create_checkpoint(n);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Manually create a checkpoint (governance/root only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(0)]
		pub fn force_checkpoint(
			origin: OriginFor<T>,
			total_supply: u128,
			reserve_ratio: u16,
		) -> DispatchResult {
			ensure_root(origin)?;

			let block_number = <frame_system::Pallet<T>>::block_number();
			Self::create_checkpoint_with_data(block_number, total_supply, reserve_ratio)?;

			Ok(())
		}

		/// Activate emergency mode
		#[pallet::weight(10_000)]
		#[pallet::call_index(1)]
		pub fn activate_emergency_mode(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			EmergencyMode::<T>::put(true);
			Self::deposit_event(Event::EmergencyModeActivated);

			Ok(())
		}

		/// Deactivate emergency mode
		#[pallet::weight(10_000)]
		#[pallet::call_index(2)]
		pub fn deactivate_emergency_mode(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;

			EmergencyMode::<T>::put(false);
			Self::deposit_event(Event::EmergencyModeDeactivated);

			Ok(())
		}

		/// Verify a checkpoint (called by FlareChain validators)
		#[pallet::weight(10_000)]
		#[pallet::call_index(3)]
		pub fn verify_checkpoint(
			origin: OriginFor<T>,
			block_number: BlockNumberFor<T>,
		) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(
				Checkpoints::<T>::contains_key(&block_number),
				Error::<T>::CheckpointNotFound
			);

			Self::deposit_event(Event::CheckpointVerified { block_number });

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Check if a checkpoint should be created at this block
		fn should_create_checkpoint(n: BlockNumberFor<T>) -> bool {
			let interval = T::CheckpointInterval::get();
			n % interval == 0u32.into()
		}

		/// Create a checkpoint at the current block
		fn create_checkpoint(block_number: BlockNumberFor<T>) -> DispatchResult {
			// In a real implementation, these would be fetched from the EDSC token and oracle pallets
			let total_supply = 0u128; // TODO: Get from EdscToken pallet
			let reserve_ratio = 10000u16; // TODO: Get from reserve oracle (100% = 10000 basis points)

			Self::create_checkpoint_with_data(block_number, total_supply, reserve_ratio)
		}

		/// Create a checkpoint with specific data
		fn create_checkpoint_with_data(
			block_number: BlockNumberFor<T>,
			total_supply: u128,
			reserve_ratio: u16,
		) -> DispatchResult {
			// Check if we've exceeded max checkpoints
			let count = CheckpointCount::<T>::get();
			ensure!(count < T::MaxCheckpoints::get(), Error::<T>::MaxCheckpointsReached);

			// Check for emergency conditions
			if reserve_ratio < T::EmergencyReserveThreshold::get() {
				EmergencyMode::<T>::put(true);
				Self::deposit_event(Event::EmergencyCheckpoint {
					block_number,
					reserve_ratio,
				});
			}

			// Generate state root (Merkle root of current state)
			let state_root = Self::generate_state_root(block_number, total_supply, reserve_ratio);

			// Get current timestamp
			let timestamp = Self::get_timestamp();

			// Create checkpoint
			let checkpoint = Checkpoint {
				block_number,
				state_root,
				total_supply,
				reserve_ratio,
				timestamp,
			};

			// Store checkpoint
			Checkpoints::<T>::insert(block_number, checkpoint);
			LatestCheckpoint::<T>::put(block_number);
			CheckpointCount::<T>::put(count.saturating_add(1));

			Self::deposit_event(Event::CheckpointCreated {
				block_number,
				state_root,
				total_supply,
				reserve_ratio,
			});

			Ok(())
		}

		/// Generate Merkle root of current state
		fn generate_state_root(
			block_number: BlockNumberFor<T>,
			total_supply: u128,
			reserve_ratio: u16,
		) -> T::Hash {
			// Create a hash from block number, supply, and reserve ratio
			let mut data = Vec::new();
			data.extend_from_slice(&block_number.encode());
			data.extend_from_slice(&total_supply.encode());
			data.extend_from_slice(&reserve_ratio.encode());

			T::Hashing::hash(&data)
		}

		/// Get current timestamp (placeholder)
		fn get_timestamp() -> u64 {
			// In production, this would use pallet_timestamp
			// For now, return a placeholder
			0u64
		}

		/// Get checkpoint by block number
		pub fn get_checkpoint(
			block_number: BlockNumberFor<T>,
		) -> Option<Checkpoint<BlockNumberFor<T>, T::Hash>> {
			Checkpoints::<T>::get(block_number)
		}

		/// Check if system is in emergency mode
		pub fn is_emergency_mode() -> bool {
			EmergencyMode::<T>::get()
		}
	}
}
