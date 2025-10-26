# Option C: Future Utilities for pbc-common

## Comprehensive List of Additional Utilities

This document outlines potential enhancements to `pbc-common` that could provide additional value to all PBCs while maintaining simplicity and avoiding complexity.

---

## Category 1: Testing Utilities

### 1.1 Mock Runtime Builder
**Purpose:** Simplify integration testing for all PBCs

**Implementation:**
```rust
// pbc-common/src/testing.rs

#[cfg(test)]
pub mod mock {
    use super::*;

    pub fn new_test_ext() -> sp_io::TestExternalities {
        frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap()
            .into()
    }

    pub fn run_to_block(n: BlockNumber) {
        while System::block_number() < n {
            System::set_block_number(System::block_number() + 1);
        }
    }

    pub struct ExtBuilder {
        balances: Vec<(AccountId, Balance)>,
    }

    impl ExtBuilder {
        pub fn new() -> Self {
            Self { balances: vec![] }
        }

        pub fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
            self.balances = balances;
            self
        }

        pub fn build(self) -> sp_io::TestExternalities {
            // Build test environment
            unimplemented!()
        }
    }
}
```

**Benefits:**
- Consistent testing across all PBCs
- Reduces test boilerplate by ~50 lines per PBC
- Easier to write integration tests

---

### 1.2 Common Test Accounts
**Purpose:** Standard test accounts for all PBCs

**Implementation:**
```rust
// pbc-common/src/testing.rs

#[cfg(test)]
pub mod accounts {
    use super::*;

    pub const ALICE: AccountId = AccountId::new([1u8; 32]);
    pub const BOB: AccountId = AccountId::new([2u8; 32]);
    pub const CHARLIE: AccountId = AccountId::new([3u8; 32]);
    pub const DAVE: AccountId = AccountId::new([4u8; 32]);
    pub const EVE: AccountId = AccountId::new([5u8; 32]);
    pub const VALIDATOR_1: AccountId = AccountId::new([100u8; 32]);
    pub const VALIDATOR_2: AccountId = AccountId::new([101u8; 32]);
    pub const BRIDGE_AUTHORITY: AccountId = AccountId::new([255u8; 32]);

    pub fn initial_balances() -> Vec<(AccountId, Balance)> {
        vec![
            (ALICE, 1_000_000_000_000_000_000), // 1 ÉTR
            (BOB, 1_000_000_000_000_000_000),
            (CHARLIE, 500_000_000_000_000_000),
        ]
    }
}
```

**Benefits:**
- Consistent test data across PBCs
- Easier to write cross-PBC tests
- Better documentation through standard accounts

---

### 1.3 Assertion Helpers
**Purpose:** Common assertions for PBC testing

**Implementation:**
```rust
// pbc-common/src/testing.rs

#[cfg(test)]
pub mod assertions {
    use super::*;

    pub fn assert_balance(account: &AccountId, expected: Balance) {
        assert_eq!(Balances::free_balance(account), expected);
    }

    pub fn assert_event_emitted<E: Into<RuntimeEvent>>(event: E) {
        let event: RuntimeEvent = event.into();
        assert!(System::events().iter().any(|e| e.event == event));
    }

    pub fn assert_last_event<E: Into<RuntimeEvent>>(event: E) {
        let event: RuntimeEvent = event.into();
        assert_eq!(System::events().last().unwrap().event, event);
    }
}
```

**Benefits:**
- Cleaner test code
- Consistent error messages
- Easier debugging

---

## Category 2: Configuration Helpers

### 2.1 Default Parameter Sets
**Purpose:** Provide sensible defaults for common parameters

**Implementation:**
```rust
// pbc-common/src/defaults.rs

pub mod defaults {
    use super::*;

    /// Default block time for PBCs (6 seconds)
    pub const MILLISECS_PER_BLOCK: u64 = 6000;

    /// Default slot duration
    pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

    /// Default existential deposit (0.001 ÉTR)
    pub const EXISTENTIAL_DEPOSIT: Balance = 1_000_000_000_000_000;

    /// Default max locks per account
    pub const MAX_LOCKS: u32 = 50;

    /// Default max validators
    pub const MAX_VALIDATORS: u32 = 100;

    /// Default ASF committee size
    pub const COMMITTEE_SIZE: u32 = 21;

    /// Default epoch duration (~4 hours at 6s/block)
    pub const EPOCH_DURATION: u32 = 2400;
}
```

**Benefits:**
- Consistency across PBCs
- Clear documentation of standard values
- Easy to override when needed

---

### 2.2 Validation Helpers
**Purpose:** Common validation functions for bridge configs

**Implementation:**
```rust
// pbc-common/src/validation.rs

pub mod validation {
    use super::*;

    /// Validate bridge deposit amount is within reasonable bounds
    pub fn validate_deposit_amount<T: Config>(
        amount: Balance,
        min: Balance,
        max: Balance,
    ) -> DispatchResult {
        ensure!(amount >= min, Error::<T>::DepositTooSmall);
        ensure!(amount <= max, Error::<T>::DepositTooLarge);
        Ok(())
    }

    /// Validate confirmation count is reasonable for blockchain
    pub fn validate_confirmations(confirmations: u32) -> Result<(), &'static str> {
        ensure!(confirmations >= 1, "At least 1 confirmation required");
        ensure!(confirmations <= 100, "Too many confirmations");
        Ok(())
    }

    /// Validate account has sufficient balance
    pub fn ensure_sufficient_balance<T: Config>(
        account: &AccountId,
        amount: Balance,
    ) -> DispatchResult {
        let free = T::Currency::free_balance(account);
        ensure!(free >= amount, Error::<T>::InsufficientBalance);
        Ok(())
    }
}
```

**Benefits:**
- Consistent validation logic
- Better error handling
- Reusable across bridge pallets

---

## Category 3: Utility Functions

### 3.1 Time Conversion Helpers
**Purpose:** Convert between blocks, time units

**Implementation:**
```rust
// pbc-common/src/time.rs

pub mod time {
    use super::*;

    pub const MILLISECS_PER_BLOCK: u64 = 6000;

    pub fn blocks_to_minutes(blocks: BlockNumber) -> BlockNumber {
        blocks * MILLISECS_PER_BLOCK / 60_000
    }

    pub fn minutes_to_blocks(minutes: BlockNumber) -> BlockNumber {
        minutes * 60_000 / MILLISECS_PER_BLOCK
    }

    pub fn blocks_to_hours(blocks: BlockNumber) -> BlockNumber {
        blocks_to_minutes(blocks) / 60
    }

    pub fn hours_to_blocks(hours: BlockNumber) -> BlockNumber {
        minutes_to_blocks(hours * 60)
    }

    pub fn blocks_to_days(blocks: BlockNumber) -> BlockNumber {
        blocks_to_hours(blocks) / 24
    }

    pub fn days_to_blocks(days: BlockNumber) -> BlockNumber {
        hours_to_blocks(days * 24)
    }
}
```

**Benefits:**
- Consistent time calculations
- Reduces errors in timelock calculations
- Self-documenting code

---

### 3.2 Balance Formatting
**Purpose:** Format balances for display/logging

**Implementation:**
```rust
// pbc-common/src/formatting.rs

#[cfg(feature = "std")]
pub mod formatting {
    use super::*;

    pub fn format_balance(balance: Balance) -> String {
        let etr = balance / 1_000_000_000_000_000_000;
        let remainder = balance % 1_000_000_000_000_000_000;
        if remainder == 0 {
            format!("{} ÉTR", etr)
        } else {
            format!("{}.{:018} ÉTR", etr, remainder)
        }
    }

    pub fn parse_balance(s: &str) -> Result<Balance, &'static str> {
        // Parse "1.5 ETR" -> Balance
        unimplemented!()
    }
}
```

**Benefits:**
- Consistent display format
- Easier debugging
- Better user-facing messages

---

## Category 4: Benchmarking Utilities

### 4.1 Benchmark Helpers
**Purpose:** Standard benchmarking utilities

**Implementation:**
```rust
// pbc-common/src/benchmarking.rs

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking {
    use super::*;

    pub fn funded_account<T: frame_system::Config>(
        name: &'static str,
        index: u32,
        balance: Balance,
    ) -> T::AccountId {
        let account: T::AccountId = account(name, index, 0);
        T::Currency::make_free_balance_be(&account, balance);
        account
    }

    pub fn setup_validators<T: Config>(count: u32) -> Vec<T::AccountId> {
        (0..count)
            .map(|i| funded_account::<T>("validator", i, 100_000_000_000_000_000_000))
            .collect()
    }
}
```

**Benefits:**
- Consistent benchmark setup
- Easier to write benchmarks
- Comparable results across PBCs

---

## Category 5: Development Tools

### 5.1 Genesis Builder Helpers
**Purpose:** Simplify genesis configuration

**Implementation:**
```rust
// pbc-common/src/genesis.rs

pub mod genesis {
    use super::*;

    pub struct GenesisBuilder {
        balances: Vec<(AccountId, Balance)>,
        validators: Vec<AccountId>,
        sudo_key: Option<AccountId>,
    }

    impl GenesisBuilder {
        pub fn new() -> Self {
            Self {
                balances: vec![],
                validators: vec![],
                sudo_key: None,
            }
        }

        pub fn with_balance(mut self, account: AccountId, balance: Balance) -> Self {
            self.balances.push((account, balance));
            self
        }

        pub fn with_validator(mut self, account: AccountId) -> Self {
            self.validators.push(account);
            self
        }

        pub fn with_sudo(mut self, account: AccountId) -> Self {
            self.sudo_key = Some(account);
            self
        }

        pub fn build<T: Config>(self) -> RuntimeGenesisConfig {
            // Build genesis config
            unimplemented!()
        }
    }
}
```

**Benefits:**
- Easier genesis setup
- Consistent across PBCs
- Better for testing

---

### 5.2 Chain Spec Helpers
**Purpose:** Generate chain specs programmatically

**Implementation:**
```rust
// pbc-common/src/chain_spec.rs

#[cfg(feature = "std")]
pub mod chain_spec {
    use super::*;

    pub struct ChainSpecBuilder {
        name: String,
        id: String,
        chain_type: sc_chain_spec::ChainType,
    }

    impl ChainSpecBuilder {
        pub fn new(name: impl Into<String>, id: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                id: id.into(),
                chain_type: sc_chain_spec::ChainType::Development,
            }
        }

        pub fn development(mut self) -> Self {
            self.chain_type = sc_chain_spec::ChainType::Development;
            self
        }

        pub fn local(mut self) -> Self {
            self.chain_type = sc_chain_spec::ChainType::Local;
            self
        }

        pub fn live(mut self) -> Self {
            self.chain_type = sc_chain_spec::ChainType::Live;
            self
        }
    }
}
```

**Benefits:**
- Programmatic chain spec generation
- Consistent across PBCs
- Easier to maintain

---

## Category 6: Monitoring & Telemetry

### 6.1 Metrics Helpers
**Purpose:** Common metrics for all PBCs

**Implementation:**
```rust
// pbc-common/src/metrics.rs

#[cfg(feature = "std")]
pub mod metrics {
    use super::*;

    pub struct PbcMetrics {
        pub total_supply: Balance,
        pub validator_count: u32,
        pub active_channels: u32,
        pub pending_deposits: u32,
    }

    impl PbcMetrics {
        pub fn collect<T: Config>() -> Self {
            Self {
                total_supply: T::Currency::total_issuance(),
                validator_count: T::Consensus::validator_count(),
                active_channels: T::Lightning::active_channel_count(),
                pending_deposits: T::Bridge::pending_deposit_count(),
            }
        }
    }
}
```

**Benefits:**
- Consistent monitoring
- Easier to set up dashboards
- Better observability

---

### 6.2 Health Check Utilities
**Purpose:** Standard health checks

**Implementation:**
```rust
// pbc-common/src/health.rs

pub mod health {
    use super::*;

    pub enum HealthStatus {
        Healthy,
        Degraded(Vec<String>),
        Unhealthy(Vec<String>),
    }

    pub fn check_runtime_health<T: Config>() -> HealthStatus {
        let mut warnings = vec![];

        // Check validator count
        if T::Consensus::validator_count() < MIN_VALIDATORS {
            warnings.push("Low validator count".to_string());
        }

        // Check bridge status
        if !T::Bridge::is_operational() {
            return HealthStatus::Unhealthy(vec!["Bridge offline".to_string()]);
        }

        if warnings.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded(warnings)
        }
    }
}
```

**Benefits:**
- Proactive monitoring
- Early problem detection
- Consistent health checks

---

## Category 7: Security Utilities

### 7.1 Rate Limiting Helpers
**Purpose:** Prevent spam/abuse

**Implementation:**
```rust
// pbc-common/src/rate_limit.rs

pub mod rate_limit {
    use super::*;

    pub struct RateLimiter<T: Config> {
        max_per_block: u32,
        max_per_account: u32,
        _phantom: PhantomData<T>,
    }

    impl<T: Config> RateLimiter<T> {
        pub fn new(max_per_block: u32, max_per_account: u32) -> Self {
            Self {
                max_per_block,
                max_per_account,
                _phantom: PhantomData,
            }
        }

        pub fn check_limit(
            &self,
            account: &AccountId,
            current_block: BlockNumber,
        ) -> DispatchResult {
            // Check rate limits
            unimplemented!()
        }
    }
}
```

**Benefits:**
- Prevent spam
- Protect against abuse
- Consistent limits

---

### 7.2 Access Control Helpers
**Purpose:** Common permission checks

**Implementation:**
```rust
// pbc-common/src/access.rs

pub mod access {
    use super::*;

    pub fn ensure_bridge_authority<T: Config>(origin: T::Origin) -> DispatchResult {
        let who = ensure_signed(origin)?;
        ensure!(
            T::BridgeAuthority::get() == who,
            Error::<T>::NotBridgeAuthority
        );
        Ok(())
    }

    pub fn ensure_validator<T: Config>(origin: T::Origin) -> DispatchResult {
        let who = ensure_signed(origin)?;
        ensure!(
            T::Consensus::is_validator(&who),
            Error::<T>::NotValidator
        );
        Ok(())
    }

    pub fn ensure_committee_member<T: Config>(origin: T::Origin) -> DispatchResult {
        let who = ensure_signed(origin)?;
        ensure!(
            T::Consensus::is_committee_member(&who),
            Error::<T>::NotCommitteeMember
        );
        Ok(())
    }
}
```

**Benefits:**
- Consistent permissions
- Better security
- Reusable across pallets

---

## Category 8: Documentation Utilities

### 8.1 Runtime Metadata Helpers
**Purpose:** Better runtime introspection

**Implementation:**
```rust
// pbc-common/src/metadata.rs

#[cfg(feature = "std")]
pub mod metadata {
    use super::*;

    pub struct RuntimeInfo {
        pub name: String,
        pub version: u32,
        pub pallet_count: usize,
        pub storage_version: u32,
    }

    impl RuntimeInfo {
        pub fn current<T: Config>() -> Self {
            Self {
                name: T::Version::spec_name().to_string(),
                version: T::Version::spec_version(),
                pallet_count: T::PalletInfo::count(),
                storage_version: T::Version::storage_version(),
            }
        }

        pub fn print(&self) {
            println!("Runtime: {} v{}", self.name, self.version);
            println!("Pallets: {}", self.pallet_count);
            println!("Storage version: {}", self.storage_version);
        }
    }
}
```

**Benefits:**
- Better introspection
- Easier debugging
- Documentation generation

---

## Category 9: Migration Utilities

### 9.1 Storage Migration Helpers
**Purpose:** Safe storage migrations

**Implementation:**
```rust
// pbc-common/src/migrations.rs

pub mod migrations {
    use super::*;

    pub struct MigrationHelper<T: Config> {
        _phantom: PhantomData<T>,
    }

    impl<T: Config> MigrationHelper<T> {
        pub fn migrate_storage_version(
            from: u32,
            to: u32,
            migration: impl FnOnce() -> Weight,
        ) -> Weight {
            let current = T::Version::storage_version();
            if current == from {
                let weight = migration();
                T::Version::set_storage_version(to);
                weight
            } else {
                Weight::zero()
            }
        }
    }
}
```

**Benefits:**
- Safe migrations
- Version tracking
- Easier upgrades

---

## Priority Recommendations

### High Priority (Implement First)
1. ✅ **Testing Utilities** (1.1, 1.2, 1.3) - Immediate value for all PBCs
2. ✅ **Default Parameters** (2.1) - Consistency across PBCs
3. ✅ **Time Helpers** (3.1) - Reduces common errors

### Medium Priority
4. **Validation Helpers** (2.2) - Better security
5. **Balance Formatting** (3.2) - Better debugging
6. **Rate Limiting** (7.1) - Security enhancement

### Low Priority (Nice to Have)
7. Genesis Builders (5.1) - Development convenience
8. Metrics (6.1) - Monitoring
9. Metadata Helpers (8.1) - Introspection

---

## Implementation Strategy

### Phase 1: Testing Foundation
- Add `src/testing.rs` to pbc-common
- Implement mock builders, test accounts, assertions
- Update BTC PBC tests to use new utilities
- Document testing patterns

### Phase 2: Common Defaults
- Add `src/defaults.rs` to pbc-common
- Define standard parameters
- Update documentation

### Phase 3: Utilities
- Add `src/time.rs`, `src/validation.rs`, `src/formatting.rs`
- Implement core utility functions
- Add examples

### Phase 4: Advanced Features
- Add monitoring, security, migration helpers
- Implement as needed based on PBC requirements

---

## Estimated Impact

**Testing Utilities:**
- Reduces test code by ~50 lines per PBC
- Total: ~650 lines saved across 13 PBCs
- Better: Consistent test patterns

**Default Parameters:**
- Reduces parameter definitions by ~20 lines per PBC
- Total: ~260 lines saved
- Better: Guaranteed consistency

**Time Helpers:**
- Prevents timelock calculation errors
- Clearer, self-documenting code
- Easier to reason about time-based logic

**Total Additional Value:**
- Code reduction: ~900+ lines
- Consistency: Enforced standards
- Maintainability: Single source of truth
- Quality: Fewer bugs through reusable utilities

---

**Created:** October 20, 2025
**Status:** Proposed enhancements for pbc-common
**Next Steps:** Implement Phase 1 (Testing Foundation) first
