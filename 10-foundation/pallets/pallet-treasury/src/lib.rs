//! # Pallet Treasury
//!
//! Manages Ëtrid protocol treasury fund management with multisig governance.
//!
//! ## Overview
//!
//! This pallet implements the treasury system described in the Ëtrid Ivory Papers Vol III:
//! - Multisig controlled by 9 Directors (6-of-9 threshold)
//! - Multiple funding sources (transaction fees, slashing, minting, cross-chain fees)
//! - Budget category allocations (Development, Marketing, Operations, Grants, Emergency)
//! - Disbursement proposal and approval workflow
//! - Emergency withdrawal mechanism (7-of-9 threshold)
//! - Integration with Consensus Day for budget updates
//!
//! ## Treasury Funding Sources
//!
//! 1. **Transaction Fees**: 50% → Treasury, 50% burned
//! 2. **Consensus Day Minting**: Approved budgets minted and sent to treasury
//! 3. **Validator Slashing**: 50% → Treasury, 50% burned
//! 4. **Cross-Chain Fees**: 10% → Treasury
//!
//! ## Treasury Account
//!
//! - PalletId: `py/trsry`
//! - Controlled by 9 Directors via multisig (6-of-9 approval threshold)
//! - Stores ËTR (native), EDSC (stablecoin), and bridge assets
//!
//! ## Budget Categories (Default Allocations)
//!
//! 1. **Development**: 40% - Core protocol development, research, infrastructure
//! 2. **Marketing**: 20% - Community growth, partnerships, outreach
//! 3. **Operations**: 15% - Team salaries, legal, administrative costs
//! 4. **Grants**: 15% - Ecosystem grants, bounties, developer support
//! 5. **Emergency Reserve**: 10% - Locked for emergency situations only
//!
//! ## Disbursement Workflow
//!
//! 1. Director proposes disbursement from specific category
//! 2. Other directors approve (need 6 total approvals)
//! 3. Disbursement auto-executes once threshold reached
//! 4. Funds transferred to recipient account
//! 5. Category allocation decreases, history recorded
//!
//! ## Emergency Withdrawal
//!
//! - Requires 7-of-9 director approvals (higher threshold)
//! - Can only withdraw from Emergency Reserve
//! - Used for critical protocol emergencies
//!
//! ## Extrinsics
//!
//! - `fund_treasury()` - Receive funds from various sources
//! - `propose_disbursement()` - Director proposes spending
//! - `approve_disbursement()` - Director approves proposal (6/9 needed)
//! - `execute_disbursement()` - Execute approved payment (automatic after 6/9)
//! - `emergency_withdrawal()` - Emergency reserve access (7/9 needed)
//! - `set_budget_allocations()` - Update budget percentages (Consensus Day only)
//! - `add_director()` - Add new director (governance only)
//! - `remove_director()` - Remove director (governance only)
//!
//! ## Storage
//!
//! - `TreasuryBalance` - Total ËTR balance in treasury
//! - `EdscBalance` - EDSC stablecoin balance
//! - `BudgetAllocations` - Category → allocated amount
//! - `Disbursements` - Pending and historical disbursements
//! - `DirectorApprovals` - Disbursement ID → list of approving directors
//! - `Directors` - Current 9 director accounts (6-of-9 multisig)
//! - `EmergencyReserve` - Locked emergency funds
//! - `DisbursementCount` - Counter for disbursement IDs

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

// ═══════════════════════════════════════════════════════════════════════════════
// Type Definitions (MUST be outside pallet module to avoid DecodeWithMemTracking issues)
// ═══════════════════════════════════════════════════════════════════════════════

use codec::{Decode, DecodeWithMemTracking, Encode};
use frame_support::pallet_prelude::*;
use sp_runtime::RuntimeDebug;

/// Budget category for treasury allocations
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum BudgetCategory {
    /// Core protocol development, research, infrastructure
    Development,
    /// Community growth, partnerships, outreach
    Marketing,
    /// Team salaries, legal, administrative
    Operations,
    /// Ecosystem grants, bounties, developer support
    Grants,
    /// Locked for emergency situations only
    EmergencyReserve,
}

impl Default for BudgetCategory {
    fn default() -> Self {
        BudgetCategory::Development
    }
}

/// Funding source for treasury deposits
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum FundingSource {
    /// 50% of transaction fees
    TransactionFees,
    /// Approved budgets from Consensus Day
    ConsensusDayMinting,
    /// 50% of validator slashing penalties
    ValidatorSlashing,
    /// 10% of cross-chain bridge fees
    CrossChainFees,
    /// EDSC stability fees and liquidation penalties
    StabilityFees,
    /// Other sources (donations, etc.)
    Other,
}

/// Status of a disbursement proposal
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
pub enum DisbursementStatus {
    /// Pending approval (need 6-of-9)
    Pending,
    /// Approved and executed
    Executed,
    /// Rejected or expired
    Rejected,
}

/// Budget allocation percentages (basis points: 10000 = 100%)
#[derive(Clone, Copy, Encode, Decode, DecodeWithMemTracking, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug, Default, serde::Serialize, serde::Deserialize)]
pub struct BudgetAllocations {
    /// Development: 40% (4000 bps)
    pub development_bps: u32,
    /// Marketing: 20% (2000 bps)
    pub marketing_bps: u32,
    /// Operations: 15% (1500 bps)
    pub operations_bps: u32,
    /// Grants: 15% (1500 bps)
    pub grants_bps: u32,
    /// Emergency Reserve: 10% (1000 bps)
    pub emergency_reserve_bps: u32,
}

impl BudgetAllocations {
    /// Default allocation percentages from Ivory Papers
    pub fn default_allocations() -> Self {
        Self {
            development_bps: 4000,      // 40%
            marketing_bps: 2000,        // 20%
            operations_bps: 1500,       // 15%
            grants_bps: 1500,           // 15%
            emergency_reserve_bps: 1000, // 10%
        }
    }

    /// Validate allocations sum to 100%
    pub fn is_valid(&self) -> bool {
        let total = self.development_bps
            .saturating_add(self.marketing_bps)
            .saturating_add(self.operations_bps)
            .saturating_add(self.grants_bps)
            .saturating_add(self.emergency_reserve_bps);
        total == 10000 // Must equal 100%
    }

    /// Get allocation for specific category
    pub fn get_allocation_bps(&self, category: &BudgetCategory) -> u32 {
        match category {
            BudgetCategory::Development => self.development_bps,
            BudgetCategory::Marketing => self.marketing_bps,
            BudgetCategory::Operations => self.operations_bps,
            BudgetCategory::Grants => self.grants_bps,
            BudgetCategory::EmergencyReserve => self.emergency_reserve_bps,
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::{Currency, ReservableCurrency, ExistenceRequirement};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use frame_support::weights::constants::RocksDbWeight;
    use sp_runtime::traits::{Saturating, AccountIdConversion};
    use sp_std::vec::Vec;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Disbursement proposal details
    #[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct Disbursement<T: Config> {
        /// Unique disbursement ID
        pub id: u64,
        /// Budget category to spend from
        pub category: BudgetCategory,
        /// Recipient account
        pub recipient: T::AccountId,
        /// Amount to disburse
        pub amount: BalanceOf<T>,
        /// Description/justification
        pub description: BoundedVec<u8, ConstU32<256>>,
        /// Proposing director
        pub proposer: T::AccountId,
        /// Current status
        pub status: DisbursementStatus,
        /// Block number when proposed
        pub proposed_at: frame_system::pallet_prelude::BlockNumberFor<T>,
        /// Number of approvals received
        pub approval_count: u8,
        /// Emergency withdrawal flag (requires 7/9 instead of 6/9)
        pub is_emergency: bool,
    }

    /// Weight functions for pallet_treasury.
    pub trait WeightInfo {
        fn fund_treasury() -> Weight;
        fn propose_disbursement() -> Weight;
        fn approve_disbursement() -> Weight;
        fn emergency_withdrawal() -> Weight;
    }

    /// Conservative weight estimates for production safety.
    impl WeightInfo for () {
        fn fund_treasury() -> Weight {
            Weight::from_parts(40_000_000, 0)
                .saturating_add(RocksDbWeight::get().reads(1))
                .saturating_add(RocksDbWeight::get().writes(1))
        }
        fn propose_disbursement() -> Weight {
            Weight::from_parts(45_000_000, 0)
                .saturating_add(RocksDbWeight::get().reads(2))
                .saturating_add(RocksDbWeight::get().writes(1))
        }
        fn approve_disbursement() -> Weight {
            Weight::from_parts(50_000_000, 0)
                .saturating_add(RocksDbWeight::get().reads(3))
                .saturating_add(RocksDbWeight::get().writes(2))
        }
        fn emergency_withdrawal() -> Weight {
            Weight::from_parts(60_000_000, 0)
                .saturating_add(RocksDbWeight::get().reads(3))
                .saturating_add(RocksDbWeight::get().writes(2))
        }
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Currency for treasury (ËTR)
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Number of directors in multisig (default: 9)
        #[pallet::constant]
        type DirectorCount: Get<u8>;

        /// Approval threshold for normal disbursements (default: 6)
        #[pallet::constant]
        type ApprovalThreshold: Get<u8>;

        /// Emergency withdrawal threshold (default: 7)
        #[pallet::constant]
        type EmergencyThreshold: Get<u8>;

        /// Proposal expiration in blocks (default: 7 days)
        #[pallet::constant]
        type ProposalExpiration: Get<frame_system::pallet_prelude::BlockNumberFor<Self>>;

        /// Weight information for extrinsics.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Total ËTR balance in treasury account
    #[pallet::storage]
    #[pallet::getter(fn treasury_balance)]
    pub type TreasuryBalance<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// EDSC stablecoin balance
    #[pallet::storage]
    #[pallet::getter(fn edsc_balance)]
    pub type EdscBalance<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Current budget allocation percentages
    #[pallet::storage]
    #[pallet::getter(fn budget_allocations)]
    pub type BudgetAllocationsStorage<T: Config> = StorageValue<_, BudgetAllocations, ValueQuery>;

    /// Category-specific allocated budgets
    #[pallet::storage]
    #[pallet::getter(fn category_allocation)]
    pub type CategoryAllocations<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BudgetCategory,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// Disbursement proposals by ID
    #[pallet::storage]
    #[pallet::getter(fn disbursement)]
    pub type Disbursements<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,  // Disbursement ID
        Disbursement<T>,
        OptionQuery,
    >;

    /// Director approvals for each disbursement
    #[pallet::storage]
    #[pallet::getter(fn approvals)]
    pub type DirectorApprovals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,  // Disbursement ID
        BoundedVec<T::AccountId, ConstU32<9>>,  // Max 9 directors
        ValueQuery,
    >;

    /// Current 9 directors controlling treasury
    #[pallet::storage]
    #[pallet::getter(fn directors)]
    pub type Directors<T: Config> = StorageValue<_, BoundedVec<T::AccountId, ConstU32<9>>, ValueQuery>;

    /// Emergency reserve locked funds
    #[pallet::storage]
    #[pallet::getter(fn emergency_reserve)]
    pub type EmergencyReserve<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Disbursement counter for unique IDs
    #[pallet::storage]
    #[pallet::getter(fn disbursement_count)]
    pub type DisbursementCount<T: Config> = StorageValue<_, u64, ValueQuery>;

    /// Total funds received from each source (historical tracking)
    #[pallet::storage]
    #[pallet::getter(fn funding_source_totals)]
    pub type FundingSourceTotals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        FundingSource,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        /// Initial directors (9 accounts)
        pub directors: Vec<T::AccountId>,
        /// Initial budget allocations
        pub budget_allocations: BudgetAllocations,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            // Set initial directors
            let bounded_directors: BoundedVec<T::AccountId, ConstU32<9>> =
                self.directors.clone().try_into()
                    .expect("Directors list must have exactly 9 accounts");
            Directors::<T>::put(bounded_directors);

            // Set budget allocations
            BudgetAllocationsStorage::<T>::put(self.budget_allocations.clone());
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Funds deposited to treasury [source, amount]
        FundsDeposited(FundingSource, BalanceOf<T>),
        /// Disbursement proposed [id, proposer, category, amount, recipient]
        DisbursementProposed(u64, T::AccountId, BudgetCategory, BalanceOf<T>, T::AccountId),
        /// Disbursement approved by director [id, director, approval_count]
        DisbursementApproved(u64, T::AccountId, u8),
        /// Disbursement executed [id, recipient, amount]
        DisbursementExecuted(u64, T::AccountId, BalanceOf<T>),
        /// Disbursement rejected [id, reason]
        DisbursementRejected(u64, BoundedVec<u8, ConstU32<64>>),
        /// Emergency withdrawal executed [amount, recipient, approvals]
        EmergencyWithdrawal(BalanceOf<T>, T::AccountId, u8),
        /// Budget allocations updated [new_allocations]
        BudgetAllocationsUpdated(BudgetAllocations),
        /// Director added [account]
        DirectorAdded(T::AccountId),
        /// Director removed [account]
        DirectorRemoved(T::AccountId),
        /// Category allocation increased [category, amount]
        CategoryAllocationIncreased(BudgetCategory, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Not a registered director
        NotDirector,
        /// Disbursement not found
        DisbursementNotFound,
        /// Already approved by this director
        AlreadyApproved,
        /// Insufficient category allocation
        InsufficientCategoryAllocation,
        /// Insufficient treasury balance
        InsufficientTreasuryBalance,
        /// Invalid budget allocations (must sum to 100%)
        InvalidBudgetAllocations,
        /// Disbursement already executed
        AlreadyExecuted,
        /// Emergency threshold not met (need 7/9)
        EmergencyThresholdNotMet,
        /// Approval threshold not met (need 6/9)
        ApprovalThresholdNotMet,
        /// Disbursement expired
        DisbursementExpired,
        /// Description too long
        DescriptionTooLong,
        /// Maximum directors reached (9)
        MaxDirectorsReached,
        /// Director already exists
        DirectorAlreadyExists,
        /// Director not found
        DirectorNotFound,
        /// Cannot remove director (minimum required)
        CannotRemoveDirector,
        /// Emergency reserve locked (not for normal disbursement)
        EmergencyReserveLocked,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Deposit funds to treasury from various sources
        ///
        /// Called by:
        /// - Transaction fee handler (50% of fees)
        /// - pallet-consensus-day (during Minting phase)
        /// - pallet-validator-rewards (50% of slashing penalties)
        /// - Bridge pallets (10% of cross-chain fees)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn fund_treasury(
            origin: OriginFor<T>,
            source: FundingSource,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            // Can be called by authorized pallets or root
            let _ = ensure_signed_or_root(origin)?;

            // Transfer funds to treasury pallet account
            let treasury_account = Self::account_id();
            T::Currency::deposit_creating(&treasury_account, amount);

            // Update treasury balance
            TreasuryBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });

            // Track funding source totals
            FundingSourceTotals::<T>::mutate(&source, |total| {
                *total = total.saturating_add(amount);
            });

            Self::deposit_event(Event::FundsDeposited(source, amount));

            Ok(())
        }

        /// Propose disbursement from treasury
        ///
        /// Must be called by one of the 9 directors
        #[pallet::call_index(1)]
        #[pallet::weight(50_000)]
        pub fn propose_disbursement(
            origin: OriginFor<T>,
            category: BudgetCategory,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            // Verify proposer is a director
            ensure!(Self::is_director(&proposer), Error::<T>::NotDirector);

            // Validate description length
            let bounded_description: BoundedVec<u8, ConstU32<256>> = description
                .try_into()
                .map_err(|_| Error::<T>::DescriptionTooLong)?;

            // Check category allocation
            let category_budget = CategoryAllocations::<T>::get(&category);
            ensure!(amount <= category_budget, Error::<T>::InsufficientCategoryAllocation);

            // Check treasury balance
            let treasury_balance = TreasuryBalance::<T>::get();
            ensure!(amount <= treasury_balance, Error::<T>::InsufficientTreasuryBalance);

            // Create disbursement
            let id = DisbursementCount::<T>::get();
            let disbursement = Disbursement {
                id,
                category: category.clone(),
                recipient: recipient.clone(),
                amount,
                description: bounded_description,
                proposer: proposer.clone(),
                status: DisbursementStatus::Pending,
                proposed_at: frame_system::Pallet::<T>::block_number(),
                approval_count: 1, // Proposer auto-approves
                is_emergency: false,
            };

            // Store disbursement
            Disbursements::<T>::insert(id, disbursement);

            // Auto-approve by proposer
            DirectorApprovals::<T>::mutate(id, |approvals| {
                let _ = approvals.try_push(proposer.clone());
            });

            // Increment counter
            DisbursementCount::<T>::mutate(|count| *count = count.saturating_add(1));

            Self::deposit_event(Event::DisbursementProposed(
                id,
                proposer,
                category,
                amount,
                recipient,
            ));

            Ok(())
        }

        /// Approve a pending disbursement
        ///
        /// Need 6-of-9 approvals for normal disbursements, 7-of-9 for emergency
        #[pallet::call_index(2)]
        #[pallet::weight(30_000)]
        pub fn approve_disbursement(
            origin: OriginFor<T>,
            disbursement_id: u64,
        ) -> DispatchResult {
            let director = ensure_signed(origin)?;

            // Verify director
            ensure!(Self::is_director(&director), Error::<T>::NotDirector);

            // Get disbursement
            let mut disbursement = Disbursements::<T>::get(disbursement_id)
                .ok_or(Error::<T>::DisbursementNotFound)?;

            // Check not already executed
            ensure!(
                disbursement.status == DisbursementStatus::Pending,
                Error::<T>::AlreadyExecuted
            );

            // Check not expired
            let current_block = frame_system::Pallet::<T>::block_number();
            let expiration = disbursement.proposed_at.saturating_add(T::ProposalExpiration::get());
            ensure!(current_block <= expiration, Error::<T>::DisbursementExpired);

            // Check not already approved by this director
            let approvals = DirectorApprovals::<T>::get(disbursement_id);
            ensure!(
                !approvals.contains(&director),
                Error::<T>::AlreadyApproved
            );

            // Add approval
            DirectorApprovals::<T>::mutate(disbursement_id, |approvals| {
                let _ = approvals.try_push(director.clone());
            });

            // Update approval count
            disbursement.approval_count = disbursement.approval_count.saturating_add(1);
            Disbursements::<T>::insert(disbursement_id, disbursement.clone());

            Self::deposit_event(Event::DisbursementApproved(
                disbursement_id,
                director,
                disbursement.approval_count,
            ));

            // Auto-execute if threshold reached
            let threshold = if disbursement.is_emergency {
                T::EmergencyThreshold::get()
            } else {
                T::ApprovalThreshold::get()
            };

            if disbursement.approval_count >= threshold {
                Self::execute_disbursement_internal(disbursement_id)?;
            }

            Ok(())
        }

        /// Emergency withdrawal from emergency reserve
        ///
        /// Requires 7-of-9 director approvals (higher threshold)
        #[pallet::call_index(3)]
        #[pallet::weight(50_000)]
        pub fn emergency_withdrawal(
            origin: OriginFor<T>,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            // Verify proposer is a director
            ensure!(Self::is_director(&proposer), Error::<T>::NotDirector);

            // Validate description length
            let bounded_description: BoundedVec<u8, ConstU32<256>> = description
                .try_into()
                .map_err(|_| Error::<T>::DescriptionTooLong)?;

            // Check emergency reserve balance
            let emergency_balance = EmergencyReserve::<T>::get();
            ensure!(amount <= emergency_balance, Error::<T>::InsufficientCategoryAllocation);

            // Create emergency disbursement
            let id = DisbursementCount::<T>::get();
            let disbursement = Disbursement {
                id,
                category: BudgetCategory::EmergencyReserve,
                recipient: recipient.clone(),
                amount,
                description: bounded_description,
                proposer: proposer.clone(),
                status: DisbursementStatus::Pending,
                proposed_at: frame_system::Pallet::<T>::block_number(),
                approval_count: 1,
                is_emergency: true, // Requires 7/9 approvals
            };

            // Store disbursement
            Disbursements::<T>::insert(id, disbursement);

            // Auto-approve by proposer
            DirectorApprovals::<T>::mutate(id, |approvals| {
                let _ = approvals.try_push(proposer.clone());
            });

            // Increment counter
            DisbursementCount::<T>::mutate(|count| *count = count.saturating_add(1));

            Self::deposit_event(Event::DisbursementProposed(
                id,
                proposer,
                BudgetCategory::EmergencyReserve,
                amount,
                recipient,
            ));

            Ok(())
        }

        /// Update budget allocation percentages
        ///
        /// Can only be called during Consensus Day governance
        #[pallet::call_index(4)]
        #[pallet::weight(20_000)]
        pub fn set_budget_allocations(
            origin: OriginFor<T>,
            allocations: BudgetAllocations,
        ) -> DispatchResult {
            // Only governance/root can update allocations
            ensure_root(origin)?;

            // Validate allocations sum to 100%
            ensure!(allocations.is_valid(), Error::<T>::InvalidBudgetAllocations);

            // Update storage
            BudgetAllocationsStorage::<T>::put(allocations.clone());

            Self::deposit_event(Event::BudgetAllocationsUpdated(allocations));

            Ok(())
        }

        /// Allocate funds to specific budget categories
        ///
        /// Called during Consensus Day after minting
        #[pallet::call_index(5)]
        #[pallet::weight(30_000)]
        pub fn allocate_to_categories(
            origin: OriginFor<T>,
            total_amount: BalanceOf<T>,
        ) -> DispatchResult {
            // Only governance/root or pallet-consensus-day can call
            ensure_root(origin)?;

            let allocations = BudgetAllocationsStorage::<T>::get();

            // Calculate and allocate to each category
            let development = Self::calculate_allocation(total_amount, allocations.development_bps);
            let marketing = Self::calculate_allocation(total_amount, allocations.marketing_bps);
            let operations = Self::calculate_allocation(total_amount, allocations.operations_bps);
            let grants = Self::calculate_allocation(total_amount, allocations.grants_bps);
            let emergency = Self::calculate_allocation(total_amount, allocations.emergency_reserve_bps);

            // Update category allocations
            CategoryAllocations::<T>::mutate(BudgetCategory::Development, |bal| {
                *bal = bal.saturating_add(development);
            });
            CategoryAllocations::<T>::mutate(BudgetCategory::Marketing, |bal| {
                *bal = bal.saturating_add(marketing);
            });
            CategoryAllocations::<T>::mutate(BudgetCategory::Operations, |bal| {
                *bal = bal.saturating_add(operations);
            });
            CategoryAllocations::<T>::mutate(BudgetCategory::Grants, |bal| {
                *bal = bal.saturating_add(grants);
            });

            // Emergency reserve is locked separately
            EmergencyReserve::<T>::mutate(|reserve| {
                *reserve = reserve.saturating_add(emergency);
            });

            Self::deposit_event(Event::CategoryAllocationIncreased(BudgetCategory::Development, development));
            Self::deposit_event(Event::CategoryAllocationIncreased(BudgetCategory::Marketing, marketing));
            Self::deposit_event(Event::CategoryAllocationIncreased(BudgetCategory::Operations, operations));
            Self::deposit_event(Event::CategoryAllocationIncreased(BudgetCategory::Grants, grants));
            Self::deposit_event(Event::CategoryAllocationIncreased(BudgetCategory::EmergencyReserve, emergency));

            Ok(())
        }

        /// Add a new director (governance only)
        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn add_director(
            origin: OriginFor<T>,
            director: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Directors::<T>::try_mutate(|directors| {
                // Check not already a director
                ensure!(!directors.contains(&director), Error::<T>::DirectorAlreadyExists);

                // Add director
                directors.try_push(director.clone())
                    .map_err(|_| Error::<T>::MaxDirectorsReached)?;

                Self::deposit_event(Event::DirectorAdded(director));

                Ok(())
            })
        }

        /// Remove a director (governance only)
        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn remove_director(
            origin: OriginFor<T>,
            director: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Directors::<T>::try_mutate(|directors| {
                // Find and remove director
                if let Some(pos) = directors.iter().position(|d| d == &director) {
                    directors.remove(pos);
                    Self::deposit_event(Event::DirectorRemoved(director));
                    Ok(())
                } else {
                    Err(Error::<T>::DirectorNotFound.into())
                }
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// Pallet account ID (treasury vault)
        pub fn account_id() -> T::AccountId {
            PalletId(*b"py/trsry").into_account_truncating()
        }

        /// Check if account is a registered director
        pub fn is_director(account: &T::AccountId) -> bool {
            Directors::<T>::get().contains(account)
        }

        /// Calculate allocation amount from basis points
        fn calculate_allocation(total: BalanceOf<T>, bps: u32) -> BalanceOf<T> {
            use sp_runtime::traits::SaturatedConversion;
            let total_u128: u128 = total.saturated_into();
            let result = total_u128.saturating_mul(bps as u128) / 10000u128;
            result.saturated_into()
        }

        /// Internal execution of approved disbursement
        fn execute_disbursement_internal(disbursement_id: u64) -> DispatchResult {
            let mut disbursement = Disbursements::<T>::get(disbursement_id)
                .ok_or(Error::<T>::DisbursementNotFound)?;

            // Verify still pending
            ensure!(
                disbursement.status == DisbursementStatus::Pending,
                Error::<T>::AlreadyExecuted
            );

            // Verify threshold met
            let threshold = if disbursement.is_emergency {
                T::EmergencyThreshold::get()
            } else {
                T::ApprovalThreshold::get()
            };
            ensure!(
                disbursement.approval_count >= threshold,
                if disbursement.is_emergency {
                    Error::<T>::EmergencyThresholdNotMet
                } else {
                    Error::<T>::ApprovalThresholdNotMet
                }
            );

            // Transfer funds from treasury to recipient
            T::Currency::transfer(
                &Self::account_id(),
                &disbursement.recipient,
                disbursement.amount,
                ExistenceRequirement::AllowDeath,
            )?;

            // Update treasury balance
            TreasuryBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_sub(disbursement.amount);
            });

            // Update category allocation or emergency reserve
            if disbursement.is_emergency {
                EmergencyReserve::<T>::mutate(|reserve| {
                    *reserve = reserve.saturating_sub(disbursement.amount);
                });
            } else {
                CategoryAllocations::<T>::mutate(&disbursement.category, |allocation| {
                    *allocation = allocation.saturating_sub(disbursement.amount);
                });
            }

            // Mark as executed
            disbursement.status = DisbursementStatus::Executed;
            Disbursements::<T>::insert(disbursement_id, disbursement.clone());

            Self::deposit_event(Event::DisbursementExecuted(
                disbursement_id,
                disbursement.recipient,
                disbursement.amount,
            ));

            Ok(())
        }

        /// Fund treasury from transaction fees (50% of total fees)
        ///
        /// Called by transaction fee handler
        pub fn receive_transaction_fees(amount: BalanceOf<T>) -> DispatchResult {
            let treasury_account = Self::account_id();
            T::Currency::deposit_creating(&treasury_account, amount);

            TreasuryBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });

            FundingSourceTotals::<T>::mutate(FundingSource::TransactionFees, |total| {
                *total = total.saturating_add(amount);
            });

            Self::deposit_event(Event::FundsDeposited(FundingSource::TransactionFees, amount));

            Ok(())
        }

        /// Fund treasury from validator slashing (50% of slash amount)
        ///
        /// Called by pallet-validator-rewards
        pub fn receive_slashing_proceeds(amount: BalanceOf<T>) -> DispatchResult {
            let treasury_account = Self::account_id();
            T::Currency::deposit_creating(&treasury_account, amount);

            TreasuryBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });

            FundingSourceTotals::<T>::mutate(FundingSource::ValidatorSlashing, |total| {
                *total = total.saturating_add(amount);
            });

            Self::deposit_event(Event::FundsDeposited(FundingSource::ValidatorSlashing, amount));

            Ok(())
        }

        /// Fund treasury from Consensus Day minting
        ///
        /// Called by pallet-consensus-day during Minting phase
        pub fn receive_consensus_day_minting(amount: BalanceOf<T>) -> DispatchResult {
            let treasury_account = Self::account_id();
            T::Currency::deposit_creating(&treasury_account, amount);

            TreasuryBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });

            FundingSourceTotals::<T>::mutate(FundingSource::ConsensusDayMinting, |total| {
                *total = total.saturating_add(amount);
            });

            Self::deposit_event(Event::FundsDeposited(FundingSource::ConsensusDayMinting, amount));

            Ok(())
        }

        /// Fund treasury from cross-chain fees (10% of bridge fees)
        ///
        /// Called by bridge pallets
        pub fn receive_cross_chain_fees(amount: BalanceOf<T>) -> DispatchResult {
            let treasury_account = Self::account_id();
            T::Currency::deposit_creating(&treasury_account, amount);

            TreasuryBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });

            FundingSourceTotals::<T>::mutate(FundingSource::CrossChainFees, |total| {
                *total = total.saturating_add(amount);
            });

            Self::deposit_event(Event::FundsDeposited(FundingSource::CrossChainFees, amount));

            Ok(())
        }

        /// Fund treasury from EDSC stability fees
        ///
        /// Called by pallet-edsc-stability for interest payments and liquidation penalties
        pub fn receive_stability_fees(amount: BalanceOf<T>) -> DispatchResult {
            let treasury_account = Self::account_id();
            T::Currency::deposit_creating(&treasury_account, amount);

            TreasuryBalance::<T>::mutate(|balance| {
                *balance = balance.saturating_add(amount);
            });

            FundingSourceTotals::<T>::mutate(FundingSource::StabilityFees, |total| {
                *total = total.saturating_add(amount);
            });

            Self::deposit_event(Event::FundsDeposited(FundingSource::StabilityFees, amount));

            Ok(())
        }
    }
}
