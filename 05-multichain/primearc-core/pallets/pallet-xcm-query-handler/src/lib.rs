//! # XCM Query Handler Pallet
//!
//! Handles XCM queries from ETH-PBC precompiles (Oracle, Governance, Staking).
//!
//! This pallet receives XCM messages from ETH-PBC containing queries about:
//! - Oracle price feeds
//! - Governance proposals
//! - Validator staking information
//!
//! It processes these queries and sends responses back via XCM.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use codec::{Decode, Encode};
    use frame_support::{pallet_prelude::*, traits::Get};
    use frame_system::pallet_prelude::*;
    use sp_core::H160;
    use sp_runtime::traits::Zero;
    use sp_std::vec::Vec;
    use xcm::latest::{prelude::*, Weight as XcmWeight};

    /// Query types from ETH-PBC precompiles
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum PrecompileQuery {
        /// Oracle: Get price of symbol in quote currency
        OraclePrice {
            symbol: [u8; 32],
            quote_currency: [u8; 32],
        },
        /// Oracle: Get last update timestamp
        OracleLastUpdate { symbol: [u8; 32] },
        /// Governance: Submit proposal
        GovernanceProposal {
            title: Vec<u8>,
            description: Vec<u8>,
            caller: H160,
        },
        /// Governance: Vote on proposal
        GovernanceVote {
            proposal_id: u64,
            support: bool,
            caller: H160,
        },
        /// Governance: Get proposal status
        GovernanceProposalStatus { proposal_id: u64 },
        /// Staking: Get validator stake
        ValidatorStake { validator_id: [u8; 32] },
        /// Staking: Check if validator is active
        ValidatorActive { validator_id: [u8; 32] },
        /// Staking: Get total staked amount
        TotalStaked,
        /// Staking: Get validator count
        ValidatorCount,
    }

    /// Response types sent back to ETH-PBC
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum PrecompileResponse {
        /// Oracle price (scaled by 1e18)
        OraclePrice(u128),
        /// Oracle last update timestamp
        OracleLastUpdate(u64),
        /// Governance proposal ID
        GovernanceProposalId(u64),
        /// Governance vote confirmed
        GovernanceVoteConfirmed,
        /// Governance proposal status (0=pending, 1=active, 2=passed, 3=failed)
        GovernanceProposalStatus(u8),
        /// Validator stake amount
        ValidatorStake(u128),
        /// Validator active status
        ValidatorActive(bool),
        /// Total staked amount
        TotalStaked(u128),
        /// Validator count
        ValidatorCount(u32),
        /// Error response
        Error(Vec<u8>),
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_xcm::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Oracle pallet for price queries (optional, use mock if not available)
        type Oracle: OracleProvider;

        /// Governance pallet for proposal queries (optional, use mock if not available)
        type Governance: GovernanceProvider<Self::AccountId>;

        /// Staking pallet for validator queries (optional, use mock if not available)
        type Staking: StakingProvider;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Query received from ETH-PBC
        QueryReceived { query_type: Vec<u8> },
        /// Response sent back to ETH-PBC
        ResponseSent { response_type: Vec<u8> },
        /// Query processing failed
        QueryFailed { error: Vec<u8> },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Invalid query format
        InvalidQuery,
        /// Oracle query failed
        OracleFailed,
        /// Governance query failed
        GovernanceFailed,
        /// Staking query failed
        StakingFailed,
        /// XCM send failed
        XcmSendFailed,
    }

    impl<T: Config> Pallet<T> {
        /// Handle incoming XCM query from ETH-PBC
        pub fn handle_query(
            origin: MultiLocation,
            query: PrecompileQuery,
        ) -> Result<PrecompileResponse, DispatchError> {
            // Process query based on type
            let response = match query {
                PrecompileQuery::OraclePrice {
                    symbol,
                    quote_currency,
                } => {
                    let price = T::Oracle::get_price(symbol, quote_currency)?;
                    PrecompileResponse::OraclePrice(price)
                }
                PrecompileQuery::OracleLastUpdate { symbol } => {
                    let timestamp = T::Oracle::get_last_update(symbol)?;
                    PrecompileResponse::OracleLastUpdate(timestamp)
                }
                PrecompileQuery::GovernanceProposal {
                    title,
                    description,
                    caller,
                } => {
                    // Convert H160 to AccountId (simplified - needs proper conversion)
                    let account = Self::h160_to_account_id(caller)?;
                    let proposal_id =
                        T::Governance::submit_proposal(account, title, description)?;
                    PrecompileResponse::GovernanceProposalId(proposal_id)
                }
                PrecompileQuery::GovernanceVote {
                    proposal_id,
                    support,
                    caller,
                } => {
                    let account = Self::h160_to_account_id(caller)?;
                    T::Governance::vote(account, proposal_id, support)?;
                    PrecompileResponse::GovernanceVoteConfirmed
                }
                PrecompileQuery::GovernanceProposalStatus { proposal_id } => {
                    let status = T::Governance::get_proposal_status(proposal_id)?;
                    PrecompileResponse::GovernanceProposalStatus(status)
                }
                PrecompileQuery::ValidatorStake { validator_id } => {
                    let stake = T::Staking::get_validator_stake(validator_id)?;
                    PrecompileResponse::ValidatorStake(stake)
                }
                PrecompileQuery::ValidatorActive { validator_id } => {
                    let is_active = T::Staking::is_validator_active(validator_id)?;
                    PrecompileResponse::ValidatorActive(is_active)
                }
                PrecompileQuery::TotalStaked => {
                    let total = T::Staking::get_total_staked()?;
                    PrecompileResponse::TotalStaked(total)
                }
                PrecompileQuery::ValidatorCount => {
                    let count = T::Staking::get_validator_count()?;
                    PrecompileResponse::ValidatorCount(count)
                }
            };

            // Send response back to ETH-PBC via XCM
            Self::send_response(origin, response.clone())?;

            Ok(response)
        }

        /// Send response back to ETH-PBC via XCM
        fn send_response(
            destination: MultiLocation,
            response: PrecompileResponse,
        ) -> DispatchResult {
            let message = Xcm(vec![
                UnpaidExecution {
                    weight_limit: WeightLimit::Unlimited,
                    check_origin: None,
                },
                Transact {
                    origin_kind: OriginKind::Native,
                    require_weight_at_most: XcmWeight::from_parts(1_000_000_000, 64 * 1024),
                    call: response.encode().into(),
                },
            ]);

            pallet_xcm::Pallet::<T>::send_xcm(Here, destination, message)
                .map_err(|_| Error::<T>::XcmSendFailed)?;

            Self::deposit_event(Event::ResponseSent {
                response_type: b"response".to_vec(),
            });

            Ok(())
        }

        /// Convert H160 (EVM address) to AccountId
        /// TODO: Implement proper H160 -> AccountId conversion based on your chain's address format
        fn h160_to_account_id(_address: H160) -> Result<T::AccountId, DispatchError> {
            // This is a placeholder - implement proper conversion
            // For example, you might use a mapping pallet or derive from H160
            Err(Error::<T>::InvalidQuery.into())
        }
    }

    /// Oracle provider trait
    pub trait OracleProvider {
        fn get_price(symbol: [u8; 32], quote: [u8; 32]) -> Result<u128, DispatchError>;
        fn get_last_update(symbol: [u8; 32]) -> Result<u64, DispatchError>;
    }

    /// Governance provider trait
    pub trait GovernanceProvider<AccountId> {
        fn submit_proposal(
            who: AccountId,
            title: Vec<u8>,
            description: Vec<u8>,
        ) -> Result<u64, DispatchError>;
        fn vote(who: AccountId, proposal_id: u64, support: bool) -> Result<(), DispatchError>;
        fn get_proposal_status(proposal_id: u64) -> Result<u8, DispatchError>;
    }

    /// Staking provider trait
    pub trait StakingProvider {
        fn get_validator_stake(validator_id: [u8; 32]) -> Result<u128, DispatchError>;
        fn is_validator_active(validator_id: [u8; 32]) -> Result<bool, DispatchError>;
        fn get_total_staked() -> Result<u128, DispatchError>;
        fn get_validator_count() -> Result<u32, DispatchError>;
    }

    /// Mock implementations for development
    pub struct MockOracle;
    impl OracleProvider for MockOracle {
        fn get_price(symbol: [u8; 32], _quote: [u8; 32]) -> Result<u128, DispatchError> {
            let price = match &symbol[..3] {
                b"BTC" => 50000_000000000000000000u128,
                b"ETH" => 3000_000000000000000000u128,
                b"SOL" => 100_000000000000000000u128,
                b"XRP" => 1_000000000000000000u128,
                _ => return Err(DispatchError::Other("Unknown symbol")),
            };
            Ok(price)
        }

        fn get_last_update(_symbol: [u8; 32]) -> Result<u64, DispatchError> {
            Ok(1700000000)
        }
    }

    pub struct MockGovernance;
    impl<AccountId> GovernanceProvider<AccountId> for MockGovernance {
        fn submit_proposal(
            _who: AccountId,
            _title: Vec<u8>,
            _description: Vec<u8>,
        ) -> Result<u64, DispatchError> {
            Ok(42) // Mock proposal ID
        }

        fn vote(
            _who: AccountId,
            _proposal_id: u64,
            _support: bool,
        ) -> Result<(), DispatchError> {
            Ok(())
        }

        fn get_proposal_status(_proposal_id: u64) -> Result<u8, DispatchError> {
            Ok(1) // Active
        }
    }

    pub struct MockStaking;
    impl StakingProvider for MockStaking {
        fn get_validator_stake(_validator_id: [u8; 32]) -> Result<u128, DispatchError> {
            Ok(1000_000000000000000000u128) // 1000 ETR
        }

        fn is_validator_active(_validator_id: [u8; 32]) -> Result<bool, DispatchError> {
            Ok(true)
        }

        fn get_total_staked() -> Result<u128, DispatchError> {
            Ok(1000000_000000000000000000u128) // 1M ETR
        }

        fn get_validator_count() -> Result<u32, DispatchError> {
            Ok(100)
        }
    }
}
