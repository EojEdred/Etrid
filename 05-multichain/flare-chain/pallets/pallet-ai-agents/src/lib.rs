#![cfg_attr(not(feature = "std"), no_std)]

//! # AI Agents Pallet
//!
//! The AI Agents pallet provides on-chain registry and management for autonomous AI agents
//! running on FlareChain validator nodes.
//!
//! ## Overview
//!
//! This pallet allows validators to:
//! - Register their validator DID (Decentralized Identifier)
//! - Register AI agent DIDs (6 agents per validator)
//! - Report AI agent actions on-chain for transparency
//! - Track agent reputation and performance
//! - Slash misbehaving agents
//!
//! ## DID Format
//!
//! Validator DIDs: `did:etrid:director-<name>` or `did:etrid:validitynode-<name>`
//! Agent DIDs: `did:etrid:<validator-did>:<agent-type>`
//!
//! Example:
//! - Validator: `did:etrid:director-gizzi`
//! - Agents:
//!   - `did:etrid:director-gizzi:compiler`
//!   - `did:etrid:director-gizzi:governance`
//!   - `did:etrid:director-gizzi:runtime`
//!   - `did:etrid:director-gizzi:economics`
//!   - `did:etrid:director-gizzi:security`
//!   - `did:etrid:director-gizzi:oracle`

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
	use sp_std::vec::Vec;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Agent types available in the system
	#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum AgentType {
		Compiler,
		Governance,
		Runtime,
		Economics,
		Security,
		Oracle,
	}

	impl AgentType {
		pub fn from_u8(value: u8) -> Result<Self, ()> {
			match value {
				0 => Ok(AgentType::Compiler),
				1 => Ok(AgentType::Governance),
				2 => Ok(AgentType::Runtime),
				3 => Ok(AgentType::Economics),
				4 => Ok(AgentType::Security),
				5 => Ok(AgentType::Oracle),
				_ => Err(()),
			}
		}

		pub fn to_u8(&self) -> u8 {
			match self {
				AgentType::Compiler => 0,
				AgentType::Governance => 1,
				AgentType::Runtime => 2,
				AgentType::Economics => 3,
				AgentType::Security => 4,
				AgentType::Oracle => 5,
			}
		}
	}

	/// Agent status
	#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum AgentStatus {
		Active,
		Paused,
		Slashed,
	}

	impl AgentStatus {
		pub fn from_u8(value: u8) -> Result<Self, ()> {
			match value {
				0 => Ok(AgentStatus::Active),
				1 => Ok(AgentStatus::Paused),
				2 => Ok(AgentStatus::Slashed),
				_ => Err(()),
			}
		}

		pub fn to_u8(&self) -> u8 {
			match self {
				AgentStatus::Active => 0,
				AgentStatus::Paused => 1,
				AgentStatus::Slashed => 2,
			}
		}
	}

	/// AI Agent metadata
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct AiAgent<T: Config> {
		/// Agent DID
		pub did: BoundedVec<u8, ConstU32<128>>,
		/// Agent type (Compiler, Governance, etc.)
		pub agent_type: AgentType,
		/// Owner (validator account)
		pub owner: T::AccountId,
		/// HTTP endpoint for agent API
		pub endpoint: BoundedVec<u8, ConstU32<256>>,
		/// Staked amount
		pub stake: BalanceOf<T>,
		/// Reputation score (0-1000)
		pub reputation: u32,
		/// Current status
		pub status: AgentStatus,
		/// Block number when registered
		pub registered_at: BlockNumberFor<T>,
		/// Total actions reported
		pub action_count: u64,
		/// Last action block number
		pub last_action_at: BlockNumberFor<T>,
	}

	/// Agent action record
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct AgentAction<T: Config> {
		/// Agent DID that performed the action
		pub agent_did: BoundedVec<u8, ConstU32<128>>,
		/// Action type (e.g., "compile", "generate_proposal")
		pub action: BoundedVec<u8, ConstU32<64>>,
		/// Result data (truncated to 1KB)
		pub result: BoundedVec<u8, ConstU32<1024>>,
		/// Was the action successful?
		pub success: bool,
		/// Block number when action occurred
		pub block_number: BlockNumberFor<T>,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency mechanism
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Minimum stake required to register an agent
		#[pallet::constant]
		type MinAgentStake: Get<BalanceOf<Self>>;

		/// Maximum number of agents per validator
		#[pallet::constant]
		type MaxAgentsPerValidator: Get<u32>;

		/// Reputation threshold below which agents get slashed
		#[pallet::constant]
		type SlashingThreshold: Get<u32>;

		/// Initial reputation for new agents
		#[pallet::constant]
		type InitialReputation: Get<u32>;
	}

	/// Validator AccountId → Validator DID
	#[pallet::storage]
	#[pallet::getter(fn validator_dids)]
	pub type ValidatorDids<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<u8, ConstU32<128>>,
		OptionQuery,
	>;

	/// DID → Validator AccountId (reverse lookup)
	#[pallet::storage]
	#[pallet::getter(fn did_to_validator)]
	pub type DidToValidator<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, ConstU32<128>>,
		T::AccountId,
		OptionQuery,
	>;

	/// Agent DID → Agent metadata
	#[pallet::storage]
	#[pallet::getter(fn agents)]
	pub type Agents<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedVec<u8, ConstU32<128>>, AiAgent<T>, OptionQuery>;

	/// Validator AccountId → List of agent DIDs
	#[pallet::storage]
	#[pallet::getter(fn validator_agents)]
	pub type ValidatorAgents<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<BoundedVec<u8, ConstU32<128>>, T::MaxAgentsPerValidator>,
		ValueQuery,
	>;

	/// Action ID → Action record (limited history)
	#[pallet::storage]
	#[pallet::getter(fn actions)]
	pub type Actions<T: Config> =
		StorageMap<_, Blake2_128Concat, u64, AgentAction<T>, OptionQuery>;

	/// Next action ID
	#[pallet::storage]
	#[pallet::getter(fn next_action_id)]
	pub type NextActionId<T: Config> = StorageValue<_, u64, ValueQuery>;

	/// Total number of registered validators
	#[pallet::storage]
	#[pallet::getter(fn validator_count)]
	pub type ValidatorCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Total number of registered agents
	#[pallet::storage]
	#[pallet::getter(fn agent_count)]
	pub type AgentCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Validator DID registered [who, did]
		ValidatorDidRegistered { validator: T::AccountId, did: Vec<u8> },
		/// Agent DID registered [who, agent_did, agent_type] - agent_type: 0=Compiler, 1=Governance, 2=Runtime, 3=Economics, 4=Security, 5=Oracle
		AgentDidRegistered { validator: T::AccountId, agent_did: Vec<u8>, agent_type: u8 },
		/// Agent action reported [agent_did, action, success]
		AgentActionReported { agent_did: Vec<u8>, action: Vec<u8>, success: bool },
		/// Agent reputation updated [agent_did, old_reputation, new_reputation]
		AgentReputationUpdated { agent_did: Vec<u8>, old_reputation: u32, new_reputation: u32 },
		/// Agent slashed [agent_did, reason]
		AgentSlashed { agent_did: Vec<u8>, reason: Vec<u8> },
		/// Agent status changed [agent_did, old_status, new_status] - status: 0=Active, 1=Paused, 2=Slashed
		AgentStatusChanged { agent_did: Vec<u8>, old_status: u8, new_status: u8 },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Validator DID already registered
		ValidatorDidAlreadyExists,
		/// Validator DID not set for this account
		ValidatorDidNotSet,
		/// Invalid DID format
		InvalidDidFormat,
		/// DID already exists
		DidAlreadyExists,
		/// Agent DID must be child of validator DID
		AgentDidMustBeChild,
		/// Maximum agents per validator reached
		MaxAgentsReached,
		/// Agent does not exist
		AgentNotFound,
		/// Insufficient stake
		InsufficientStake,
		/// Not the agent owner
		NotAgentOwner,
		/// Agent is already slashed
		AgentAlreadySlashed,
		/// Invalid agent type
		InvalidAgentType,
		/// DID too long
		DidTooLong,
		/// Endpoint too long
		EndpointTooLong,
		/// Action data too long
		ActionDataTooLong,
		/// Not a validator
		NotValidator,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a validator DID
		///
		/// The caller must be a validator in the session pallet.
		/// DID format: `did:etrid:director-<name>` or `did:etrid:validitynode-<name>`
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn register_validator_did(
			origin: OriginFor<T>,
			did: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure validator is in the active set
			// TODO: Check against pallet_session::Validators
			// For now, we'll allow any signed account

			// Validate DID format
			let bounded_did: BoundedVec<u8, ConstU32<128>> =
				did.clone().try_into().map_err(|_| Error::<T>::DidTooLong)?;

			ensure!(Self::is_valid_validator_did(&bounded_did), Error::<T>::InvalidDidFormat);

			// Check DID not already taken
			ensure!(
				!DidToValidator::<T>::contains_key(&bounded_did),
				Error::<T>::DidAlreadyExists
			);

			// Check validator doesn't already have a DID
			ensure!(
				!ValidatorDids::<T>::contains_key(&who),
				Error::<T>::ValidatorDidAlreadyExists
			);

			// Store mappings
			ValidatorDids::<T>::insert(&who, bounded_did.clone());
			DidToValidator::<T>::insert(&bounded_did, &who);

			// Increment counter
			ValidatorCount::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::ValidatorDidRegistered { validator: who, did });

			Ok(())
		}

		/// Register an AI agent under validator DID
		///
		/// The agent DID must be a child of the validator's DID.
		/// Format: `did:etrid:<validator-did>:<agent-type>`
		///
		/// Agent types: 0=Compiler, 1=Governance, 2=Runtime, 3=Economics, 4=Security, 5=Oracle
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn register_agent_did(
			origin: OriginFor<T>,
			agent_did: Vec<u8>,
			agent_type_u8: u8,
			endpoint: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert u8 to AgentType enum
			let agent_type = AgentType::from_u8(agent_type_u8).map_err(|_| Error::<T>::InvalidAgentType)?;

			// Get validator DID
			let validator_did =
				ValidatorDids::<T>::get(&who).ok_or(Error::<T>::ValidatorDidNotSet)?;

			// Validate agent DID format
			let bounded_agent_did: BoundedVec<u8, ConstU32<128>> =
				agent_did.clone().try_into().map_err(|_| Error::<T>::DidTooLong)?;

			let bounded_endpoint: BoundedVec<u8, ConstU32<256>> =
				endpoint.try_into().map_err(|_| Error::<T>::EndpointTooLong)?;

			// Ensure agent DID is child of validator DID
			ensure!(
				Self::is_child_did(&validator_did, &bounded_agent_did),
				Error::<T>::AgentDidMustBeChild
			);

			// Check agent DID not already registered
			ensure!(!Agents::<T>::contains_key(&bounded_agent_did), Error::<T>::DidAlreadyExists);

			// Check max agents not reached
			let current_agents = ValidatorAgents::<T>::get(&who);
			ensure!(
				current_agents.len() < T::MaxAgentsPerValidator::get() as usize,
				Error::<T>::MaxAgentsReached
			);

			// Reserve stake
			let stake = T::MinAgentStake::get();
			T::Currency::reserve(&who, stake).map_err(|_| Error::<T>::InsufficientStake)?;

			// Create agent record
			let agent = AiAgent {
				did: bounded_agent_did.clone(),
				agent_type: agent_type.clone(),
				owner: who.clone(),
				endpoint: bounded_endpoint,
				stake,
				reputation: T::InitialReputation::get(),
				status: AgentStatus::Active,
				registered_at: <frame_system::Pallet<T>>::block_number(),
				action_count: 0,
				last_action_at: <frame_system::Pallet<T>>::block_number(),
			};

			// Store agent
			Agents::<T>::insert(&bounded_agent_did, agent);

			// Add to validator's agent list
			ValidatorAgents::<T>::mutate(&who, |agents| {
				let _ = agents.try_push(bounded_agent_did.clone());
			});

			// Increment counter
			AgentCount::<T>::mutate(|count| *count = count.saturating_add(1));

			Self::deposit_event(Event::AgentDidRegistered {
				validator: who,
				agent_did: agent_did.clone(),
				agent_type: agent_type.to_u8(),
			});

			Ok(())
		}

		/// Report an AI agent action
		///
		/// This records the action on-chain for transparency and tracking.
		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn report_agent_action(
			origin: OriginFor<T>,
			agent_did: Vec<u8>,
			action: Vec<u8>,
			result: Vec<u8>,
			success: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let bounded_agent_did: BoundedVec<u8, ConstU32<128>> =
				agent_did.clone().try_into().map_err(|_| Error::<T>::DidTooLong)?;

			// Get agent
			let mut agent = Agents::<T>::get(&bounded_agent_did).ok_or(Error::<T>::AgentNotFound)?;

			// Ensure caller is agent owner
			ensure!(agent.owner == who, Error::<T>::NotAgentOwner);

			// Truncate result to 1KB
			let bounded_result: BoundedVec<u8, ConstU32<1024>> =
				result.into_iter().take(1024).collect::<Vec<_>>().try_into().map_err(|_| Error::<T>::ActionDataTooLong)?;

			let bounded_action: BoundedVec<u8, ConstU32<64>> =
				action.clone().into_iter().take(64).collect::<Vec<_>>().try_into().map_err(|_| Error::<T>::ActionDataTooLong)?;

			// Create action record
			let action_id = NextActionId::<T>::get();
			let action_record = AgentAction {
				agent_did: bounded_agent_did.clone(),
				action: bounded_action,
				result: bounded_result,
				success,
				block_number: <frame_system::Pallet<T>>::block_number(),
			};

			// Store action (limited history - old actions will be pruned)
			Actions::<T>::insert(action_id, action_record);
			NextActionId::<T>::put(action_id.saturating_add(1));

			// Update agent stats
			agent.action_count = agent.action_count.saturating_add(1);
			agent.last_action_at = <frame_system::Pallet<T>>::block_number();

			// Update reputation based on success
			let old_reputation = agent.reputation;
			if success {
				// Increase reputation (capped at 1000)
				agent.reputation = agent.reputation.saturating_add(1).min(1000);
			} else {
				// Decrease reputation
				agent.reputation = agent.reputation.saturating_sub(5);
			}

			// Check if agent should be slashed
			if agent.reputation < T::SlashingThreshold::get() && agent.status != AgentStatus::Slashed {
				agent.status = AgentStatus::Slashed;
				Self::deposit_event(Event::AgentSlashed {
					agent_did: agent_did.clone(),
					reason: b"Reputation below threshold".to_vec(),
				});
			}

			// Store updated agent
			Agents::<T>::insert(&bounded_agent_did, agent.clone());

			if old_reputation != agent.reputation {
				Self::deposit_event(Event::AgentReputationUpdated {
					agent_did: agent_did.clone(),
					old_reputation,
					new_reputation: agent.reputation,
				});
			}

			Self::deposit_event(Event::AgentActionReported { agent_did, action, success });

			Ok(())
		}

		/// Update agent status
		///
		/// Only the owner can update their agent's status.
		///
		/// Status values: 0=Active, 1=Paused, 2=Slashed
		#[pallet::call_index(3)]
		#[pallet::weight(10_000)]
		pub fn update_agent_status(
			origin: OriginFor<T>,
			agent_did: Vec<u8>,
			new_status_u8: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert u8 to AgentStatus enum
			let new_status = AgentStatus::from_u8(new_status_u8).map_err(|_| Error::<T>::InvalidAgentType)?;

			let bounded_agent_did: BoundedVec<u8, ConstU32<128>> =
				agent_did.clone().try_into().map_err(|_| Error::<T>::DidTooLong)?;

			// Get agent
			let mut agent = Agents::<T>::get(&bounded_agent_did).ok_or(Error::<T>::AgentNotFound)?;

			// Ensure caller is agent owner
			ensure!(agent.owner == who, Error::<T>::NotAgentOwner);

			// Don't allow unslas hing via this method
			ensure!(
				!(agent.status == AgentStatus::Slashed && new_status != AgentStatus::Slashed),
				Error::<T>::AgentAlreadySlashed
			);

			let old_status = agent.status.clone();
			agent.status = new_status.clone();

			Agents::<T>::insert(&bounded_agent_did, agent);

			Self::deposit_event(Event::AgentStatusChanged { agent_did: agent_did.clone(), old_status: old_status.to_u8(), new_status: new_status.to_u8() });

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config> Pallet<T> {
		/// Validate validator DID format
		///
		/// Valid formats:
		/// - `did:etrid:director-<name>`
		/// - `did:etrid:validitynode-<name>`
		fn is_valid_validator_did(did: &BoundedVec<u8, ConstU32<128>>) -> bool {
			let did_str = sp_std::str::from_utf8(did).unwrap_or("");
			did_str.starts_with("did:etrid:director-") ||
				did_str.starts_with("did:etrid:validitynode-")
		}

		/// Check if child DID is valid child of parent DID
		///
		/// Child format: `<parent-did>:<agent-type>`
		fn is_child_did(
			parent: &BoundedVec<u8, ConstU32<128>>,
			child: &BoundedVec<u8, ConstU32<128>>,
		) -> bool {
			let parent_str = sp_std::str::from_utf8(parent).unwrap_or("");
			let child_str = sp_std::str::from_utf8(child).unwrap_or("");

			// Child must start with parent and have one more colon-separated segment
			if !child_str.starts_with(parent_str) {
				return false;
			}

			// Check that child has exactly one more segment
			let remainder = &child_str[parent_str.len()..];
			remainder.starts_with(":") && remainder.matches(':').count() == 1
		}
	}
}
