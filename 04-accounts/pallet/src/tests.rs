use crate::{self as pallet_accounts, *};
use frame_support::{
    assert_noop, assert_ok, parameter_types,
    traits::ConstU32,
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Accounts: pallet_accounts,
    }
);

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
    type BlockHashCount = ConstU32<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type RuntimeTask = ();
    type ExtensionsWeightInfo = ();
}

pub struct EnsureRoot;
impl frame_support::traits::EnsureOrigin<RuntimeOrigin> for EnsureRoot {
    type Success = ();
    fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
        if let Ok(frame_system::RawOrigin::Root) = o.clone().into() {
            Ok(())
        } else {
            Err(o)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
        Ok(RuntimeOrigin::root())
    }
}

impl pallet_accounts::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type GovernanceOrigin = EnsureRoot;
}

pub fn account(id: u64) -> u64 {
    id
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();
    t.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        System::set_block_number(System::block_number() + 1);
    }
}

#[test]
fn create_recovery_works() {
    new_test_ext().execute_with(|| {
        let guardians = vec![account(2), account(3), account(4)];
        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(account(1)),
            guardians.clone(),
            2,
            10
        ));

        assert!(RecoveryConfigs::<Test>::contains_key(&account(1)));

        let config = RecoveryConfigs::<Test>::get(&account(1)).unwrap();
        assert_eq!(config.guardians.len(), 3);
        assert_eq!(config.threshold, 2);
        assert_eq!(config.delay_period, 10);

        System::assert_last_event(
            Event::RecoveryCreated {
                account: account(1),
                threshold: 2,
            }
            .into(),
        );
    });
}

#[test]
fn create_recovery_invalid_threshold() {
    new_test_ext().execute_with(|| {
        let guardians = vec![account(2), account(3)];

        // Threshold 0 should fail
        assert_noop!(
            Accounts::create_recovery(RuntimeOrigin::signed(account(1)), guardians.clone(), 0, 10),
            Error::<Test>::InvalidThreshold
        );

        // Threshold > guardians should fail
        assert_noop!(
            Accounts::create_recovery(RuntimeOrigin::signed(account(1)), guardians, 3, 10),
            Error::<Test>::ThresholdTooHigh
        );
    });
}

#[test]
fn create_recovery_no_guardians() {
    new_test_ext().execute_with(|| {
        let guardians: Vec<u64> = vec![];

        assert_noop!(
            Accounts::create_recovery(RuntimeOrigin::signed(account(1)), guardians, 1, 10),
            Error::<Test>::NoGuardians
        );
    });
}

#[test]
fn create_recovery_too_many_guardians() {
    new_test_ext().execute_with(|| {
        let guardians: Vec<u64> = (2..13).collect(); // 11 guardians (> MAX_GUARDIANS)

        assert_noop!(
            Accounts::create_recovery(RuntimeOrigin::signed(account(1)), guardians, 5, 10),
            Error::<Test>::TooManyGuardians
        );
    });
}

#[test]
fn initiate_recovery_works() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let new = account(5);
        let guardians = vec![account(2), account(3), account(4)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        // Guardian 2 initiates recovery
        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            new
        ));

        assert!(ActiveRecoveries::<Test>::contains_key(&lost));

        let recovery = ActiveRecoveries::<Test>::get(&lost).unwrap();
        assert_eq!(recovery.new_account, new);
        assert_eq!(recovery.approvals.len(), 1);
        assert_eq!(recovery.approvals[0], account(2));

        System::assert_last_event(
            Event::RecoveryInitiated {
                lost_account: lost,
                new_account: new,
                guardian: account(2),
            }
            .into(),
        );
    });
}

#[test]
fn initiate_recovery_no_config() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            Accounts::initiate_recovery(RuntimeOrigin::signed(account(2)), account(1), account(5)),
            Error::<Test>::NoRecoveryConfig
        );
    });
}

#[test]
fn initiate_recovery_not_guardian() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        // Non-guardian tries to initiate
        assert_noop!(
            Accounts::initiate_recovery(RuntimeOrigin::signed(account(99)), lost, account(5)),
            Error::<Test>::NotGuardian
        );
    });
}

#[test]
fn initiate_recovery_already_active() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            account(5)
        ));

        // Try to initiate again
        assert_noop!(
            Accounts::initiate_recovery(RuntimeOrigin::signed(account(3)), lost, account(6)),
            Error::<Test>::RecoveryAlreadyActive
        );
    });
}

#[test]
fn approve_recovery_works() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3), account(4)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            account(5)
        ));

        // Guardian 3 approves
        assert_ok!(Accounts::approve_recovery(
            RuntimeOrigin::signed(account(3)),
            lost
        ));

        let recovery = ActiveRecoveries::<Test>::get(&lost).unwrap();
        assert_eq!(recovery.approvals.len(), 2);
        assert!(recovery.approvals.contains(&account(2)));
        assert!(recovery.approvals.contains(&account(3)));

        System::assert_last_event(
            Event::RecoveryApproved {
                lost_account: lost,
                guardian: account(3),
                approvals: 2,
            }
            .into(),
        );
    });
}

#[test]
fn approve_recovery_not_guardian() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            account(5)
        ));

        // Non-guardian tries to approve
        assert_noop!(
            Accounts::approve_recovery(RuntimeOrigin::signed(account(99)), lost),
            Error::<Test>::NotGuardian
        );
    });
}

#[test]
fn approve_recovery_duplicate_approval() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            account(5)
        ));

        // Same guardian (who initiated) tries to approve twice
        assert_noop!(
            Accounts::approve_recovery(RuntimeOrigin::signed(account(2)), lost),
            Error::<Test>::AlreadyApproved
        );
    });
}

#[test]
fn approve_recovery_no_active_recovery() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        // Try to approve without initiation
        assert_noop!(
            Accounts::approve_recovery(RuntimeOrigin::signed(account(2)), lost),
            Error::<Test>::NoActiveRecovery
        );
    });
}

#[test]
fn execute_recovery_delay_not_passed() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let new = account(5);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        // Fund the lost account
        pallet_accounts::Accounts::<Test>::mutate(&lost, |acct| {
            acct.etr_balance = 1000;
        });

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            new
        ));

        assert_ok!(Accounts::approve_recovery(
            RuntimeOrigin::signed(account(3)),
            lost
        ));

        // Try to execute before delay (should fail)
        assert_noop!(
            Accounts::execute_recovery(RuntimeOrigin::signed(account(2)), lost),
            Error::<Test>::DelayNotPassed
        );
    });
}

#[test]
fn execute_recovery_threshold_not_met() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let new = account(5);
        let guardians = vec![account(2), account(3), account(4)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            new
        ));

        // Only 1 approval (initiator), threshold is 2
        run_to_block(11);

        assert_noop!(
            Accounts::execute_recovery(RuntimeOrigin::signed(account(2)), lost),
            Error::<Test>::ThresholdNotMet
        );
    });
}

#[test]
fn full_recovery_workflow() {
    new_test_ext().execute_with(|| {
        // Setup: Account 1 with 3 guardians, 2-of-3 threshold, 10 block delay
        let lost = account(1);
        let new = account(5);
        let guardians = vec![account(2), account(3), account(4)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        // Fund the lost account
        pallet_accounts::Accounts::<Test>::mutate(&lost, |acct| {
            acct.etr_balance = 1000;
            acct.etd_balance = 500;
            acct.is_validator = true;
            acct.reputation = 100;
        });

        // Guardian 2 initiates recovery
        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            new
        ));

        // Guardian 3 approves (reaches threshold)
        assert_ok!(Accounts::approve_recovery(
            RuntimeOrigin::signed(account(3)),
            lost
        ));

        // Try to execute before delay (should fail)
        assert_noop!(
            Accounts::execute_recovery(RuntimeOrigin::signed(account(2)), lost),
            Error::<Test>::DelayNotPassed
        );

        // Advance blocks past delay period
        run_to_block(11);

        // Execute recovery
        assert_ok!(Accounts::execute_recovery(
            RuntimeOrigin::signed(account(2)),
            lost
        ));

        // Verify balances transferred
        let new_data = pallet_accounts::Accounts::<Test>::get(&new);
        assert_eq!(new_data.etr_balance, 1000);
        assert_eq!(new_data.etd_balance, 500);
        assert_eq!(new_data.is_validator, true);
        assert_eq!(new_data.reputation, 100);

        // Verify lost account cleared
        let lost_data = pallet_accounts::Accounts::<Test>::get(&lost);
        assert_eq!(lost_data.etr_balance, 0);
        assert_eq!(lost_data.etd_balance, 0);

        // Verify cleanup
        assert!(!ActiveRecoveries::<Test>::contains_key(&lost));
        assert!(!RecoveryConfigs::<Test>::contains_key(&lost));

        System::assert_last_event(
            Event::RecoveryExecuted {
                lost_account: lost,
                new_account: new,
            }
            .into(),
        );
    });
}

#[test]
fn cancel_recovery_by_owner() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            account(5)
        ));

        // Owner can cancel
        assert_ok!(Accounts::cancel_recovery(
            RuntimeOrigin::signed(lost),
            lost
        ));

        // Verify cleanup
        assert!(!ActiveRecoveries::<Test>::contains_key(&lost));

        System::assert_last_event(Event::RecoveryCancelled { account: lost }.into());
    });
}

#[test]
fn cancel_recovery_not_owner() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            10
        ));

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            account(5)
        ));

        // Non-owner tries to cancel
        assert_noop!(
            Accounts::cancel_recovery(RuntimeOrigin::signed(account(99)), lost),
            Error::<Test>::NotAccountOwner
        );
    });
}

#[test]
fn recovery_with_maximum_guardians() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let guardians: Vec<u64> = (2..12).collect(); // 10 guardians (MAX_GUARDIANS)

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians.clone(),
            10,
            10
        ));

        let config = RecoveryConfigs::<Test>::get(&lost).unwrap();
        assert_eq!(config.guardians.len(), 10);
    });
}

#[test]
fn recovery_with_all_guardians_approval() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let new = account(10);
        let guardians = vec![account(2), account(3), account(4)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians.clone(),
            3,
            5
        ));

        // Fund the lost account
        pallet_accounts::Accounts::<Test>::mutate(&lost, |acct| {
            acct.etr_balance = 2000;
        });

        // Guardian 2 initiates
        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            new
        ));

        // Guardian 3 approves
        assert_ok!(Accounts::approve_recovery(
            RuntimeOrigin::signed(account(3)),
            lost
        ));

        // Guardian 4 approves
        assert_ok!(Accounts::approve_recovery(
            RuntimeOrigin::signed(account(4)),
            lost
        ));

        let recovery = ActiveRecoveries::<Test>::get(&lost).unwrap();
        assert_eq!(recovery.approvals.len(), 3);

        // Advance blocks
        run_to_block(6);

        // Execute recovery
        assert_ok!(Accounts::execute_recovery(
            RuntimeOrigin::signed(account(2)),
            lost
        ));

        // Verify
        let new_data = pallet_accounts::Accounts::<Test>::get(&new);
        assert_eq!(new_data.etr_balance, 2000);
    });
}

#[test]
fn recovery_transfers_both_token_types() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let new = account(5);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            5
        ));

        // Fund with both token types
        pallet_accounts::Accounts::<Test>::mutate(&lost, |acct| {
            acct.etr_balance = 1500;
            acct.etd_balance = 2500;
        });

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            new
        ));

        assert_ok!(Accounts::approve_recovery(
            RuntimeOrigin::signed(account(3)),
            lost
        ));

        run_to_block(6);

        assert_ok!(Accounts::execute_recovery(
            RuntimeOrigin::signed(account(2)),
            lost
        ));

        let new_data = pallet_accounts::Accounts::<Test>::get(&new);
        assert_eq!(new_data.etr_balance, 1500);
        assert_eq!(new_data.etd_balance, 2500);
    });
}

#[test]
fn recovery_preserves_validator_status_and_reputation() {
    new_test_ext().execute_with(|| {
        let lost = account(1);
        let new = account(5);
        let guardians = vec![account(2), account(3)];

        assert_ok!(Accounts::create_recovery(
            RuntimeOrigin::signed(lost),
            guardians,
            2,
            5
        ));

        // Set validator status and reputation
        pallet_accounts::Accounts::<Test>::mutate(&lost, |acct| {
            acct.etr_balance = 100;
            acct.is_validator = true;
            acct.reputation = 9999;
        });

        assert_ok!(Accounts::initiate_recovery(
            RuntimeOrigin::signed(account(2)),
            lost,
            new
        ));

        assert_ok!(Accounts::approve_recovery(
            RuntimeOrigin::signed(account(3)),
            lost
        ));

        run_to_block(6);

        assert_ok!(Accounts::execute_recovery(
            RuntimeOrigin::signed(account(2)),
            lost
        ));

        let new_data = pallet_accounts::Accounts::<Test>::get(&new);
        assert_eq!(new_data.is_validator, true);
        assert_eq!(new_data.reputation, 9999);
    });
}
