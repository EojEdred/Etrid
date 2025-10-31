//! # Pallet Validator Rewards - Storage Migrations
//!
//! This module contains storage migrations for upgrading the validator payment system.
//!
//! ## Migration V1
//!
//! Migrates existing validator data from pallet-validator-committee to initialize
//! the new validator rewards pallet with:
//! - Default payment account mappings (session → session initially)
//! - Validator stakes from existing committee data
//! - Initial performance metrics (zeroed, will be populated over time)
//!
//! ## Migration Safety
//!
//! - Read-only access to pallet-validator-committee
//! - Idempotent: Can be run multiple times safely
//! - Weight calculation includes all storage reads/writes

use super::*;
use frame_support::traits::OnRuntimeUpgrade;
use frame_support::weights::Weight;
use sp_std::vec::Vec;

/// Storage version for the pallet
pub const STORAGE_VERSION: u16 = 1;

pub mod v1 {
    use super::*;

    /// Migration to V1 - Initialize validator payment accounts and stakes
    pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            log::info!("🔄 Starting migration to pallet-validator-rewards v1");

            let mut reads = 0u64;
            let mut writes = 0u64;

            // Get all validators from pallet-validator-committee
            let committee_validators = pallet_validator_committee::Pallet::<T>::get_committee();
            reads += 1; // Reading committee list

            log::info!(
                "📋 Found {} validators in committee to migrate",
                committee_validators.len()
            );

            let mut migrated_count = 0u32;
            let mut total_stake = BalanceOf::<T>::zero();

            // Iterate through all validators and initialize payment data
            for validator_info in committee_validators.iter() {
                let validator_id = &validator_info.id;

                // Convert ValidatorId (AccountId32) to T::AccountId
                // This assumes T::AccountId can be decoded from ValidatorId bytes
                if let Ok(session_account) = T::AccountId::decode(&mut &validator_id.encode()[..]) {

                    // 1. Initialize payment account mapping (default: session → session)
                    // Validators can update this later via extrinsic
                    if !PaymentAccounts::<T>::contains_key(&session_account) {
                        PaymentAccounts::<T>::insert(&session_account, &session_account);
                        writes += 1;

                        log::debug!(
                            "✅ Initialized payment account for validator (default: self-pay)"
                        );
                    }

                    // 2. Initialize validator stake
                    let stake_balance: BalanceOf<T> = validator_info.stake.try_into()
                        .unwrap_or_else(|_| BalanceOf::<T>::zero());

                    ValidatorStakes::<T>::insert(&session_account, stake_balance);
                    writes += 1;
                    total_stake = total_stake.saturating_add(stake_balance);

                    // 3. Initialize performance metrics (zeroed state)
                    let initial_metrics = PerformanceMetrics {
                        blocks_authored: 0,
                        blocks_expected: 0,
                        finality_votes: 0,
                        finality_expected: 0,
                        uptime_bps: 10000, // Default to 100% uptime until measured
                        consensus_day_participation: false,
                    };
                    ValidatorPerformance::<T>::insert(&session_account, initial_metrics);
                    writes += 1;

                    migrated_count += 1;

                    log::debug!(
                        "🔧 Migrated validator with stake: {}",
                        stake_balance
                    );
                } else {
                    log::warn!("⚠️ Failed to decode validator account ID, skipping");
                }

                reads += 1; // Reading each validator info
            }

            // 4. Set total staked amount
            TotalStaked::<T>::put(total_stake);
            writes += 1;

            // 5. Initialize epoch to 0 (will increment on first epoch transition)
            if CurrentEpoch::<T>::get() == 0 {
                CurrentEpoch::<T>::put(0u32);
                writes += 1;
            }

            // 6. Initialize epoch reward pool to zero (will be funded by inflation/treasury)
            if EpochRewardPool::<T>::get().is_zero() {
                EpochRewardPool::<T>::put(BalanceOf::<T>::zero());
                writes += 1;
            }

            log::info!(
                "✅ Migration complete: {} validators migrated, total stake: {}",
                migrated_count,
                total_stake
            );

            log::info!(
                "📊 Storage operations: {} reads, {} writes",
                reads,
                writes
            );

            // Calculate weight based on actual operations
            // Read weight: 25_000 per read
            // Write weight: 100_000 per write
            Weight::from_parts(
                (reads * 25_000).saturating_add(writes * 100_000),
                0
            )
        }

        #[cfg(feature = "try-runtime")]
        fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
            use codec::Encode;

            log::info!("🔍 Pre-upgrade checks for pallet-validator-rewards v1");

            // Get current committee size
            let committee = pallet_validator_committee::Pallet::<T>::get_committee();
            let validator_count = committee.len() as u32;

            log::info!("📊 Current committee size: {}", validator_count);

            // Verify pallet-validator-committee has validators
            if validator_count == 0 {
                log::warn!("⚠️ No validators found in committee - migration will be no-op");
            }

            // Return committee size for post-upgrade verification
            Ok(validator_count.encode())
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
            use codec::Decode;

            log::info!("🔍 Post-upgrade checks for pallet-validator-rewards v1");

            // Decode expected validator count from pre-upgrade
            let expected_count = u32::decode(&mut &state[..])
                .map_err(|_| "Failed to decode validator count")?;

            // Count migrated validators
            let migrated_count = ValidatorStakes::<T>::iter().count() as u32;

            log::info!(
                "📊 Expected: {} validators, Migrated: {} validators",
                expected_count,
                migrated_count
            );

            // Verify all validators were migrated
            if migrated_count != expected_count {
                log::error!(
                    "❌ Migration incomplete: {} validators expected, {} migrated",
                    expected_count,
                    migrated_count
                );
                return Err("Migration incomplete: validator count mismatch");
            }

            // Verify total stake is non-zero if validators exist
            let total_stake = TotalStaked::<T>::get();
            if migrated_count > 0 && total_stake.is_zero() {
                log::warn!("⚠️ Total stake is zero despite having validators");
            }

            // Verify each validator has payment account and performance metrics
            let mut missing_payment_accounts = 0u32;
            let mut missing_performance = 0u32;

            for (session_account, _stake) in ValidatorStakes::<T>::iter() {
                if !PaymentAccounts::<T>::contains_key(&session_account) {
                    missing_payment_accounts += 1;
                }
                if !ValidatorPerformance::<T>::contains_key(&session_account) {
                    missing_performance += 1;
                }
            }

            if missing_payment_accounts > 0 {
                log::error!(
                    "❌ {} validators missing payment accounts",
                    missing_payment_accounts
                );
                return Err("Validators missing payment accounts");
            }

            if missing_performance > 0 {
                log::error!(
                    "❌ {} validators missing performance metrics",
                    missing_performance
                );
                return Err("Validators missing performance metrics");
            }

            log::info!("✅ Post-upgrade verification passed");
            log::info!("💰 Total staked: {}", total_stake);

            Ok(())
        }
    }
}

/// Migration tuple for runtime upgrade
pub type Migrations<T> = (v1::MigrateToV1<T>,);

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, parameter_types, traits::ConstU32};
    use sp_core::crypto::AccountId32;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    frame_support::construct_runtime!(
        pub enum Test {
            System: frame_system,
            Balances: pallet_balances,
            ValidatorCommittee: pallet_validator_committee,
            ValidatorRewards: crate,
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
        type Hash = sp_core::H256;
        type Hashing = BlakeTwo256;
        type AccountId = AccountId32;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Block = Block;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = BlockHashCount;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = pallet_balances::AccountData<u128>;
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ConstU32<16>;
        type RuntimeTask = ();
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
        type ExtensionsWeightInfo = ();
    }

    impl pallet_balances::Config for Test {
        type MaxLocks = ConstU32<50>;
        type MaxReserves = ();
        type ReserveIdentifier = [u8; 8];
        type Balance = u128;
        type RuntimeEvent = RuntimeEvent;
        type DustRemoval = ();
        type ExistentialDeposit = frame_support::traits::ConstU128<500>;
        type AccountStore = System;
        type WeightInfo = ();
        type FreezeIdentifier = ();
        type MaxFreezes = ();
        type RuntimeHoldReason = ();
        type RuntimeFreezeReason = ();
        type DoneSlashHandler = ();
    }

    impl pallet_validator_committee::Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type MaxCommitteeSize = ConstU32<100>;
        type MinValidatorStake = frame_support::traits::ConstU128<1000>;
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type Currency = Balances;
        type EpochDuration = ConstU32<2400>;
        type AnnualRewardPoolBps = ConstU32<300>; // 3%
        type ValidatorShareBps = ConstU32<5000>; // 50%
    }

    #[test]
    fn test_migration_initializes_payment_accounts() {
        // This is a placeholder test - full test would require mock pallet-validator-committee
        // with populated data
        // In production, use try-runtime for migration testing
    }
}
