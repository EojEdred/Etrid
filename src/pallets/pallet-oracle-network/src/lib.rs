//! # Oracle Network Pallet
//!
//! Chainlink-style oracle operator registration, staking, slashing, and reputation system.
//!
//! ## Overview
//!
//! This pallet manages the oracle network infrastructure for Etrid, providing:
//! - Oracle operator registration and deregistration
//! - Economic security through staking (stake to participate)
//! - Slashing for bad data submissions
//! - Reputation scoring based on performance
//! - Request/response oracle job system
//!
//! ## Key Features
//!
//! ### Oracle Operators
//! - Register as oracle operators with minimum stake requirement
//! - Submit price feeds and data to the network
//! - Earn rewards for accurate data submissions
//! - Get slashed for malicious or inaccurate data
//!
//! ### Economic Security
//! - Minimum stake requirement (default: 1000 ETR)
//! - Slashing for bad submissions (5-50% of stake)
//! - Reputation score (0-100) affects rewards and eligibility
//!
//! ### Data Request System
//! - Off-chain oracle nodes respond to data requests
//! - Aggregation of multiple oracle responses
//! - Payment to oracles upon successful response
//!
//! ## Integration
//!
//! This pallet works with:
//! - `pallet-reserve-oracle`: Price data aggregation
//! - `pallet-edsc-oracle`: EDSC TWAP oracle
//! - Native currency system: ETR staking

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
		traits::{Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::{Permill, traits::{Saturating, CheckedAdd, CheckedSub}};
	use sp_runtime::traits::StaticLookup;
	use sp_std::vec::Vec;

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Oracle operator information
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct OracleOperator<AccountId, Balance, BlockNumber> {
		/// Operator account ID
		pub operator: AccountId,
		/// Staked amount
		pub stake: Balance,
		/// Reputation score (0-100)
		pub reputation: u8,
		/// Total successful submissions
		pub successful_submissions: u32,
		/// Total failed/slashed submissions
		pub failed_submissions: u32,
		/// Last submission block
		pub last_submission: BlockNumber,
		/// Registration block
		pub registered_at: BlockNumber,
		/// Is currently active
		pub active: bool,
		/// Total rewards earned
		pub total_rewards: Balance,
		/// Total slashed amount
		pub total_slashed: Balance,
	}

	/// Data request
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct DataRequest<AccountId, Balance, BlockNumber> {
		/// Request ID
		pub request_id: u64,
		/// Requester account
		pub requester: AccountId,
		/// Request specification (e.g., "BTC/USD", "ETH/USD")
		pub data_spec: BoundedVec<u8, ConstU32<64>>,
		/// Payment per oracle response
		pub payment_per_oracle: Balance,
		/// Minimum oracles required
		pub min_oracles: u32,
		/// Request expiration block
		pub expires_at: BlockNumber,
		/// Number of responses received
		pub responses_count: u32,
		/// Is request fulfilled
		pub fulfilled: bool,
	}

	/// Oracle response to a data request
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct OracleResponse<AccountId, BlockNumber> {
		/// Oracle operator who submitted
		pub oracle: AccountId,
		/// Response data (encoded value)
		pub data: BoundedVec<u8, ConstU32<128>>,
		/// Submission block
		pub submitted_at: BlockNumber,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Currency for staking
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Minimum stake required to become an oracle operator
		#[pallet::constant]
		type MinimumStake: Get<BalanceOf<Self>>;

		/// Maximum stake allowed
		#[pallet::constant]
		type MaximumStake: Get<BalanceOf<Self>>;

		/// Slash percentage for bad data (in Permill, e.g., 50000 = 5%)
		#[pallet::constant]
		type SlashPercentage: Get<Permill>;

		/// Minimum reputation score to remain active (0-100)
		#[pallet::constant]
		type MinimumReputation: Get<u8>;

		/// Reward per successful data submission
		#[pallet::constant]
		type SubmissionReward: Get<BalanceOf<Self>>;

		/// Maximum number of oracles in the network
		#[pallet::constant]
		type MaxOracles: Get<u32>;

		/// Maximum number of pending data requests
		#[pallet::constant]
		type MaxDataRequests: Get<u32>;
	}

	/// Registered oracle operators
	#[pallet::storage]
	#[pallet::getter(fn oracle_operators)]
	pub type OracleOperators<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		OracleOperator<T::AccountId, BalanceOf<T>, BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Total number of registered oracles
	#[pallet::storage]
	#[pallet::getter(fn oracle_count)]
	pub type OracleCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Active oracle operators list
	#[pallet::storage]
	#[pallet::getter(fn active_oracles)]
	pub type ActiveOracles<T: Config> = StorageValue<
		_,
		BoundedVec<T::AccountId, T::MaxOracles>,
		ValueQuery,
	>;

	/// Data requests
	#[pallet::storage]
	#[pallet::getter(fn data_requests)]
	pub type DataRequests<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		u64, // request_id
		DataRequest<T::AccountId, BalanceOf<T>, BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Next data request ID
	#[pallet::storage]
	#[pallet::getter(fn next_request_id)]
	pub type NextRequestId<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Oracle responses to data requests
	/// Maps: RequestId -> Oracle -> Response
	#[pallet::storage]
	pub type OracleResponses<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		u64, // request_id
		Blake2_128Concat,
		T::AccountId, // oracle
		OracleResponse<T::AccountId, BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Oracle reputation history (for analytics)
	#[pallet::storage]
	pub type ReputationHistory<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<(BlockNumberFor<T>, u8), ConstU32<100>>, // (block, reputation_score)
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Oracle operator registered [operator, stake]
		OracleRegistered {
			operator: T::AccountId,
			stake: BalanceOf<T>,
		},
		/// Oracle operator deregistered [operator]
		OracleDeregistered {
			operator: T::AccountId,
		},
		/// Stake increased [operator, additional_stake, new_total]
		StakeIncreased {
			operator: T::AccountId,
			additional: BalanceOf<T>,
			new_total: BalanceOf<T>,
		},
		/// Stake decreased [operator, amount, new_total]
		StakeDecreased {
			operator: T::AccountId,
			amount: BalanceOf<T>,
			new_total: BalanceOf<T>,
		},
		/// Oracle slashed for bad data [operator, slashed_amount, reason]
		OracleSlashed {
			operator: T::AccountId,
			slashed_amount: BalanceOf<T>,
			reason: Vec<u8>,
		},
		/// Oracle rewarded for good data [operator, reward_amount]
		OracleRewarded {
			operator: T::AccountId,
			reward: BalanceOf<T>,
		},
		/// Reputation updated [operator, old_reputation, new_reputation]
		ReputationUpdated {
			operator: T::AccountId,
			old_score: u8,
			new_score: u8,
		},
		/// Oracle deactivated due to low reputation [operator, reputation]
		OracleDeactivated {
			operator: T::AccountId,
			reputation: u8,
		},
		/// Oracle reactivated [operator]
		OracleReactivated {
			operator: T::AccountId,
		},
		/// Data request created [request_id, requester, data_spec]
		DataRequestCreated {
			request_id: u64,
			requester: T::AccountId,
			data_spec: Vec<u8>,
		},
		/// Oracle responded to data request [request_id, oracle]
		DataResponseSubmitted {
			request_id: u64,
			oracle: T::AccountId,
		},
		/// Data request fulfilled [request_id, responses_count]
		DataRequestFulfilled {
			request_id: u64,
			responses: u32,
		},
		/// Data request expired [request_id]
		DataRequestExpired {
			request_id: u64,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Oracle operator not found
		OracleNotFound,
		/// Oracle already registered
		OracleAlreadyRegistered,
		/// Insufficient stake amount
		InsufficientStake,
		/// Stake exceeds maximum allowed
		StakeExceedsMaximum,
		/// Cannot unstake below minimum
		CannotUnstakeBelowMinimum,
		/// Oracle is not active
		OracleNotActive,
		/// Reputation too low
		ReputationTooLow,
		/// Maximum oracles reached
		MaxOraclesReached,
		/// Data request not found
		DataRequestNotFound,
		/// Data request already fulfilled
		DataRequestFulfilled,
		/// Data request expired
		DataRequestExpired,
		/// Oracle already responded
		OracleAlreadyResponded,
		/// Invalid data specification
		InvalidDataSpec,
		/// Insufficient payment for oracle
		InsufficientPayment,
		/// Not the oracle operator
		NotOracleOperator,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: BlockNumberFor<T>) {
			// Check for expired data requests
			Self::process_expired_requests(n);

			// Update reputation scores periodically (every 1000 blocks)
			if n % 1000u32.into() == 0u32.into() {
				Self::update_reputation_scores();
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register as an oracle operator
		#[pallet::weight(10_000)]
		#[pallet::call_index(0)]
		pub fn register_oracle(
			origin: OriginFor<T>,
			stake: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Check not already registered
			ensure!(!OracleOperators::<T>::contains_key(&who), Error::<T>::OracleAlreadyRegistered);

			// Check stake amount
			ensure!(stake >= T::MinimumStake::get(), Error::<T>::InsufficientStake);
			ensure!(stake <= T::MaximumStake::get(), Error::<T>::StakeExceedsMaximum);

			// Check max oracles
			let count = OracleCount::<T>::get();
			ensure!(count < T::MaxOracles::get(), Error::<T>::MaxOraclesReached);

			// Reserve stake
			T::Currency::reserve(&who, stake)?;

			let current_block = <frame_system::Pallet<T>>::block_number();

			// Create operator
			let operator = OracleOperator {
				operator: who.clone(),
				stake,
				reputation: 100, // Start with perfect reputation
				successful_submissions: 0,
				failed_submissions: 0,
				last_submission: current_block,
				registered_at: current_block,
				active: true,
				total_rewards: BalanceOf::<T>::default(),
				total_slashed: BalanceOf::<T>::default(),
			};

			// Store operator
			OracleOperators::<T>::insert(&who, operator);
			OracleCount::<T>::put(count.saturating_add(1));

			// Add to active oracles
			ActiveOracles::<T>::mutate(|oracles| {
				let _ = oracles.try_push(who.clone());
			});

			Self::deposit_event(Event::OracleRegistered {
				operator: who,
				stake,
			});

			Ok(())
		}

		/// Deregister as an oracle operator
		#[pallet::weight(10_000)]
		#[pallet::call_index(1)]
		pub fn deregister_oracle(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let operator = OracleOperators::<T>::get(&who)
				.ok_or(Error::<T>::OracleNotFound)?;

			// Unreserve stake
			T::Currency::unreserve(&who, operator.stake);

			// Remove from storage
			OracleOperators::<T>::remove(&who);
			let count = OracleCount::<T>::get();
			OracleCount::<T>::put(count.saturating_sub(1));

			// Remove from active oracles
			ActiveOracles::<T>::mutate(|oracles| {
				oracles.retain(|o| o != &who);
			});

			Self::deposit_event(Event::OracleDeregistered {
				operator: who,
			});

			Ok(())
		}

		/// Increase stake
		#[pallet::weight(10_000)]
		#[pallet::call_index(2)]
		pub fn increase_stake(
			origin: OriginFor<T>,
			additional: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut operator = OracleOperators::<T>::get(&who)
				.ok_or(Error::<T>::OracleNotFound)?;

			let new_stake = operator.stake.checked_add(&additional)
				.ok_or(Error::<T>::StakeExceedsMaximum)?;
			ensure!(new_stake <= T::MaximumStake::get(), Error::<T>::StakeExceedsMaximum);

			// Reserve additional stake
			T::Currency::reserve(&who, additional)?;

			operator.stake = new_stake;
			OracleOperators::<T>::insert(&who, operator);

			Self::deposit_event(Event::StakeIncreased {
				operator: who,
				additional,
				new_total: new_stake,
			});

			Ok(())
		}

		/// Decrease stake (cannot go below minimum)
		#[pallet::weight(10_000)]
		#[pallet::call_index(3)]
		pub fn decrease_stake(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut operator = OracleOperators::<T>::get(&who)
				.ok_or(Error::<T>::OracleNotFound)?;

			let new_stake = operator.stake.checked_sub(&amount)
				.ok_or(Error::<T>::CannotUnstakeBelowMinimum)?;
			ensure!(new_stake >= T::MinimumStake::get(), Error::<T>::CannotUnstakeBelowMinimum);

			// Unreserve stake
			T::Currency::unreserve(&who, amount);

			operator.stake = new_stake;
			OracleOperators::<T>::insert(&who, operator);

			Self::deposit_event(Event::StakeDecreased {
				operator: who,
				amount,
				new_total: new_stake,
			});

			Ok(())
		}

		/// Create a data request (governance or authorized requesters)
		#[pallet::weight(10_000)]
		#[pallet::call_index(4)]
		pub fn create_data_request(
			origin: OriginFor<T>,
			data_spec: Vec<u8>,
			payment_per_oracle: BalanceOf<T>,
			min_oracles: u32,
			expiration_blocks: BlockNumberFor<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bounded_spec: BoundedVec<u8, ConstU32<64>> = data_spec.clone().try_into()
				.map_err(|_| Error::<T>::InvalidDataSpec)?;

			let request_id = NextRequestId::<T>::get();
			let current_block = <frame_system::Pallet<T>>::block_number();
			let expires_at = current_block.saturating_add(expiration_blocks);

			// Calculate total payment needed
			let total_payment = payment_per_oracle.saturating_mul(min_oracles.into());

			// Reserve payment from requester
			T::Currency::reserve(&who, total_payment)?;

			let request = DataRequest {
				request_id,
				requester: who.clone(),
				data_spec: bounded_spec,
				payment_per_oracle,
				min_oracles,
				expires_at,
				responses_count: 0,
				fulfilled: false,
			};

			DataRequests::<T>::insert(request_id, request);
			NextRequestId::<T>::put(request_id.saturating_add(1));

			Self::deposit_event(Event::DataRequestCreated {
				request_id,
				requester: who,
				data_spec,
			});

			Ok(())
		}

		/// Submit oracle response to a data request
		#[pallet::weight(10_000)]
		#[pallet::call_index(5)]
		pub fn submit_oracle_response(
			origin: OriginFor<T>,
			request_id: u64,
			data: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Verify oracle is registered and active
			let operator = OracleOperators::<T>::get(&who)
				.ok_or(Error::<T>::OracleNotFound)?;
			ensure!(operator.active, Error::<T>::OracleNotActive);

			// Get request
			let mut request = DataRequests::<T>::get(request_id)
				.ok_or(Error::<T>::DataRequestNotFound)?;

			// Check request not fulfilled or expired
			ensure!(!request.fulfilled, Error::<T>::DataRequestFulfilled);
			let current_block = <frame_system::Pallet<T>>::block_number();
			ensure!(current_block <= request.expires_at, Error::<T>::DataRequestExpired);

			// Check oracle hasn't already responded
			ensure!(
				!OracleResponses::<T>::contains_key(request_id, &who),
				Error::<T>::OracleAlreadyResponded
			);

			let bounded_data: BoundedVec<u8, ConstU32<128>> = data.try_into()
				.map_err(|_| Error::<T>::InvalidDataSpec)?;

			// Store response
			let response = OracleResponse {
				oracle: who.clone(),
				data: bounded_data,
				submitted_at: current_block,
			};
			OracleResponses::<T>::insert(request_id, &who, response);

			// Update request
			request.responses_count = request.responses_count.saturating_add(1);

			// Check if request is fulfilled
			if request.responses_count >= request.min_oracles {
				request.fulfilled = true;

				// Pay oracles
				Self::pay_responding_oracles(request_id, &request)?;

				Self::deposit_event(Event::DataRequestFulfilled {
					request_id,
					responses: request.responses_count,
				});
			}

			DataRequests::<T>::insert(request_id, request);

			// Update operator stats
			Self::record_successful_submission(&who);

			Self::deposit_event(Event::DataResponseSubmitted {
				request_id,
				oracle: who,
			});

			Ok(())
		}

		/// Slash an oracle for bad data (governance only)
		#[pallet::weight(10_000)]
		#[pallet::call_index(6)]
		pub fn slash_oracle(
			origin: OriginFor<T>,
			oracle: <T::Lookup as StaticLookup>::Source,
			reason: Vec<u8>,
		) -> DispatchResult {
			ensure_root(origin)?;

			let oracle_account = T::Lookup::lookup(oracle)?;

			let mut operator = OracleOperators::<T>::get(&oracle_account)
				.ok_or(Error::<T>::OracleNotFound)?;

			// Calculate slash amount
			let slash_amount = T::SlashPercentage::get().mul_floor(operator.stake);

			// Slash (reduce reserved amount)
			// slash_reserved returns (NegativeImbalance, Balance)
			let (_imbalance, actual_slash) = T::Currency::slash_reserved(&oracle_account, slash_amount);

			// Update operator
			operator.stake = operator.stake.saturating_sub(actual_slash);
			operator.failed_submissions = operator.failed_submissions.saturating_add(1);
			operator.total_slashed = operator.total_slashed.saturating_add(actual_slash);

			// Update reputation
			let old_reputation = operator.reputation;
			operator.reputation = operator.reputation.saturating_sub(10);

			OracleOperators::<T>::insert(&oracle_account, operator.clone());

			// Record reputation change
			Self::record_reputation_change(&oracle_account, operator.reputation);

			// Check if should deactivate
			if operator.reputation < T::MinimumReputation::get() {
				Self::deactivate_oracle(&oracle_account)?;
			}

			Self::deposit_event(Event::OracleSlashed {
				operator: oracle_account.clone(),
				slashed_amount: actual_slash,
				reason,
			});

			Self::deposit_event(Event::ReputationUpdated {
				operator: oracle_account,
				old_score: old_reputation,
				new_score: operator.reputation,
			});

			Ok(())
		}

		/// Reactivate oracle (after improving reputation)
		#[pallet::weight(10_000)]
		#[pallet::call_index(7)]
		pub fn reactivate_oracle(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let mut operator = OracleOperators::<T>::get(&who)
				.ok_or(Error::<T>::OracleNotFound)?;

			ensure!(operator.reputation >= T::MinimumReputation::get(), Error::<T>::ReputationTooLow);

			operator.active = true;
			OracleOperators::<T>::insert(&who, operator);

			// Add back to active list
			ActiveOracles::<T>::mutate(|oracles| {
				if !oracles.contains(&who) {
					let _ = oracles.try_push(who.clone());
				}
			});

			Self::deposit_event(Event::OracleReactivated {
				operator: who,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Record successful data submission
		fn record_successful_submission(oracle: &T::AccountId) {
			if let Some(mut operator) = OracleOperators::<T>::get(oracle) {
				operator.successful_submissions = operator.successful_submissions.saturating_add(1);
				operator.last_submission = <frame_system::Pallet<T>>::block_number();

				// Improve reputation slightly (up to 100)
				let old_reputation = operator.reputation;
				operator.reputation = (operator.reputation.saturating_add(1)).min(100);

				OracleOperators::<T>::insert(oracle, operator.clone());

				if old_reputation != operator.reputation {
					Self::record_reputation_change(oracle, operator.reputation);
				}
			}
		}

		/// Deactivate oracle
		fn deactivate_oracle(oracle: &T::AccountId) -> DispatchResult {
			OracleOperators::<T>::mutate(oracle, |op| {
				if let Some(operator) = op {
					operator.active = false;
				}
			});

			// Remove from active list
			ActiveOracles::<T>::mutate(|oracles| {
				oracles.retain(|o| o != oracle);
			});

			let operator = OracleOperators::<T>::get(oracle)
				.ok_or(Error::<T>::OracleNotFound)?;

			Self::deposit_event(Event::OracleDeactivated {
				operator: oracle.clone(),
				reputation: operator.reputation,
			});

			Ok(())
		}

		/// Process expired data requests
		fn process_expired_requests(_current_block: BlockNumberFor<T>) {
			// In production, iterate through pending requests
			// For now, this is a placeholder
		}

		/// Update reputation scores periodically
		fn update_reputation_scores() {
			// In production, calculate reputation based on recent performance
			// For now, this is a placeholder
		}

		/// Pay oracles who responded to a fulfilled request
		fn pay_responding_oracles(
			_request_id: u64,
			_request: &DataRequest<T::AccountId, BalanceOf<T>, BlockNumberFor<T>>,
		) -> DispatchResult {
			// In production, iterate through responses and pay each oracle
			// Unreserve from requester and transfer to oracles
			// For now, this is a placeholder
			Ok(())
		}

		/// Record reputation change for analytics
		fn record_reputation_change(oracle: &T::AccountId, new_reputation: u8) {
			let current_block = <frame_system::Pallet<T>>::block_number();
			ReputationHistory::<T>::mutate(oracle, |history| {
				let _ = history.try_push((current_block, new_reputation));
			});
		}

		/// Get oracle operator info
		pub fn get_oracle_info(oracle: &T::AccountId) -> Option<OracleOperator<T::AccountId, BalanceOf<T>, BlockNumberFor<T>>> {
			OracleOperators::<T>::get(oracle)
		}

		/// Check if oracle is active
		pub fn is_oracle_active(oracle: &T::AccountId) -> bool {
			if let Some(operator) = OracleOperators::<T>::get(oracle) {
				operator.active && operator.reputation >= T::MinimumReputation::get()
			} else {
				false
			}
		}

		/// Get all active oracles
		pub fn get_active_oracles() -> Vec<T::AccountId> {
			ActiveOracles::<T>::get().into_inner()
		}
	}
}
