# EDSC Pallet Architecture

## Overview

The Ëtrid Dollar Stablecoin (EDSC) system consists of 6 modular pallets that work together to provide a fully-collateralized, multi-path redemption stablecoin. This document describes the architecture, dependencies, and integration of these pallets into the FlareChain runtime.

## System Components

### 1. **pallet-edsc-token** (Core Token)
**Purpose**: Manages EDSC token issuance, burning, and supply tracking.

**Key Features**:
- Controlled minting with supply limits
- Token burning for redemptions
- Balance tracking and transfers

**Configuration Parameters**:
- `MaxSupply`: 1,000,000,000 EDSC (1 billion with 18 decimals)
- `MinBalance`: 0.000001 EDSC

**Dependencies**: None (foundation pallet)

**Location**: `/pallets/pallet-edsc-token/`

---

### 2. **pallet-edsc-receipts** (Soulbound Token Registry)
**Purpose**: Maintains non-transferable purchase receipts for transparency and auditability.

**Key Features**:
- Issues soulbound tokens (SBTs) for each EDSC purchase
- Tracks purchase history per wallet
- Receipt expiry management

**Configuration Parameters**:
- `MaxReceiptsPerWallet`: 1,000 receipts
- `ReceiptExpiryPeriod`: 5,256,000 blocks (~1 year)

**Dependencies**: None (independent registry)

**Location**: `/pallets/pallet-edsc-receipts/`

---

### 3. **pallet-edsc-redemption** (3-Path Redemption Engine)
**Purpose**: Core redemption logic implementing three redemption paths based on reserve ratio.

**Key Features**:
- **Path 1** (Reserve Ratio ≥ 100%): Direct redemption at oracle price
- **Path 2** (90% ≤ Reserve Ratio < 100%): Discounted redemption (2.5% - 10% fee)
- **Path 3** (Reserve Ratio < 90%): Emergency redemption with pro-rata settlement
- Dynamic fee calculation based on market conditions
- Throttling and emergency pause mechanisms

**Configuration Parameters**:
```rust
MinRedemptionFee: 2.5%           // Minimum redemption fee
MaxRedemptionFee: 10%            // Maximum redemption fee
EmergencyReserveRatio: 90%       // Triggers Path 3
ThrottleReserveRatio: 95%        // Activates throttling
SafetyMultiplier: 1.2x           // Safety buffer for collateral
MinRedemptionAmount: 0.01 EDSC   // Minimum redemption size
MaxPendingRedemptions: 10,000    // Queue limit
```

**Dependencies**:
- None (receives updates from oracle and vault via internal helpers)

**Location**: `/pallets/pallet-edsc-redemption/`

**Internal Helper Methods**:
- `do_update_oracle_price(price)`: Receives oracle price updates
- `do_update_reserve_ratio(ratio)`: Receives vault reserve ratio updates

---

### 4. **pallet-edsc-oracle** (TWAP Price Oracle)
**Purpose**: Provides robust market price discovery using time-weighted average pricing (TWAP) with outlier detection.

**Key Features**:
- Multi-source price aggregation (5+ sources)
- Primary (4-hour) and fallback (28-hour) TWAP windows
- Statistical outlier filtering (20% threshold)
- Staleness detection and automatic fallback
- Price feeder authorization

**Configuration Parameters**:
```rust
PrimaryTwapWindow: 14,400 blocks     // ~4 hours
FallbackTwapWindow: 100,800 blocks   // ~28 hours
MinPriceSources: 5                   // Required sources
OutlierThreshold: 20%                // Price deviation threshold
StalenessTimeout: 100 blocks         // Price expiry
MaxPriceHistory: 10,000 records      // Historical data limit
```

**Price Sources**:
- Binance, Coinbase, Kraken, Bitstamp, Gemini, Huobi, OKEx, Bitfinex

**Dependencies**:
- `pallet_edsc_redemption::Config` (to update redemption engine with prices)

**Location**: `/pallets/pallet-edsc-oracle/`

**Inter-Pallet Communication**:
```rust
// Updates redemption engine with new oracle price
pallet_edsc_redemption::Pallet::<T>::do_update_oracle_price(price)?;
```

---

### 5. **pallet-reserve-vault** (Multi-Asset Collateral Vault)
**Purpose**: Manages collateral reserves across multiple asset types with real-time reserve ratio calculation.

**Key Features**:
- Multi-asset support (BTC, ETH, SOL, ADA, XRP, MATIC, LINK, USDT, etc.)
- Oracle-based collateral valuation
- Reserve ratio calculation and monitoring
- Automatic threshold checks
- Custodian integration for off-chain reserves

**Configuration Parameters**:
```rust
MaxCollateralAssets: 50              // Supported asset types
OracleUpdateFrequency: 600 blocks    // ~10 minutes
MinCollateralRatio: 150%             // Minimum healthy ratio
EmergencyRatio: 110%                 // Emergency threshold
```

**Reserve Ratio Formula**:
```
Reserve Ratio = (On-chain Value + Custodian-Attested Value) / Total EDSC Supply
```

**Dependencies**:
- `pallet_edsc_token::Config` (to query supply)
- `pallet_edsc_redemption::Config` (to update reserve ratio)

**Location**: `/pallets/pallet-reserve-vault/`

**Inter-Pallet Communication**:
```rust
// Updates redemption engine with new reserve ratio
pallet_edsc_redemption::Pallet::<T>::do_update_reserve_ratio(ratio)?;
```

**Internal Helper Method**:
- `do_update_custodian_value(value)`: Receives custodian attestations

---

### 6. **pallet-custodian-registry** (Bonded Custodian Management)
**Purpose**: Manages authorized custodians for off-chain reserve custody with bonding and attestation requirements.

**Key Features**:
- Custodian registration with bond requirement
- Periodic attestation enforcement (24-hour intervals)
- Automatic slashing for missed attestations
- Aggregated off-chain reserve value calculation

**Configuration Parameters**:
```rust
MinBond: 1,000,000 EDSC              // Minimum bond requirement
AttestationFrequency: 7,200 blocks   // ~24 hours
SlashingPercent: 10%                 // Penalty for missed attestations
MaxCustodians: 100                   // System-wide limit
```

**Dependencies**:
- `pallet_reserve_vault::Config` (to update vault with custodian values)

**Location**: `/pallets/pallet-custodian-registry/`

**Inter-Pallet Communication**:
```rust
// Updates reserve vault with aggregated custodian value
pallet_reserve_vault::Pallet::<T>::do_update_custodian_value(total)?;
```

---

## Dependency Graph

The pallets have a clear, modular dependency structure:

```
┌─────────────────────┐
│  edsc-token         │ (Foundation - no dependencies)
└─────────────────────┘

┌─────────────────────┐
│  edsc-receipts      │ (Independent registry)
└─────────────────────┘

┌─────────────────────┐
│  edsc-redemption    │ (Core logic - receives updates)
└──────────┬──────────┘
           │
           │ do_update_oracle_price()
           │
┌──────────┴──────────┐
│  edsc-oracle        │ (Depends on: edsc-redemption)
└─────────────────────┘

┌─────────────────────┐
│  edsc-token         │
└──────────┬──────────┘
           │
           │ query supply
           │
┌──────────┴──────────┐        do_update_reserve_ratio()
│  reserve-vault      ├───────────────────────────────────┐
└──────────┬──────────┘                                   │
           │                                              │
           │ do_update_custodian_value()                  │
           │                                              ▼
┌──────────┴──────────┐                         ┌─────────────────────┐
│ custodian-registry  │                         │  edsc-redemption    │
└─────────────────────┘                         └─────────────────────┘
(Depends on: reserve-vault)                     (Receives: oracle + vault updates)
```

### Dependency Summary

| Pallet | Depends On | Used By |
|--------|-----------|---------|
| **edsc-token** | None | reserve-vault |
| **edsc-receipts** | None | None (independent) |
| **edsc-redemption** | None | oracle, reserve-vault |
| **edsc-oracle** | edsc-redemption | None |
| **reserve-vault** | edsc-token, edsc-redemption | custodian-registry |
| **custodian-registry** | reserve-vault | None |

---

## Inter-Pallet Communication Pattern

To maintain modularity while enabling necessary communication, we use **internal helper methods**:

### Pattern Overview

1. **Dependent pallets** declare their dependencies in `Config` trait:
```rust
#[pallet::config]
pub trait Config: frame_system::Config + pallet_edsc_redemption::Config {
    // ...
}
```

2. **Target pallets** expose internal helper methods:
```rust
impl<T: Config> Pallet<T> {
    pub fn do_update_oracle_price(price: u128) -> DispatchResult {
        OraclePrice::<T>::put(price);
        Self::deposit_event(Event::OraclePriceUpdated { price });
        Ok(())
    }
}
```

3. **Calling pallets** invoke helpers directly:
```rust
pallet_edsc_redemption::Pallet::<T>::do_update_oracle_price(price)?;
```

### Benefits of This Pattern

- **Modularity**: Clear separation of concerns with explicit dependencies
- **Type Safety**: Rust compiler enforces correct usage at compile time
- **No Circular Dependencies**: Dependencies flow in one direction
- **Standard Substrate Pattern**: Follows established best practices
- **Runtime Satisfaction**: Runtime type automatically satisfies all trait bounds

---

## FlareChain Runtime Integration

### Runtime Configuration

All 6 EDSC pallets are configured in the FlareChain runtime at:
`/05-multichain/flare-chain/runtime/src/lib.rs`

#### Token Configuration
```rust
impl pallet_edsc_token::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxSupply = ConstU128<1_000_000_000_000_000_000_000>;
    type MinBalance = ConstU128<1_000_000_000_000>;
}
```

#### Receipts Configuration
```rust
impl pallet_edsc_receipts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxReceiptsPerWallet = ConstU32<1000>;
    type ReceiptExpiryPeriod = ConstU32<5_256_000>;
}
```

#### Redemption Configuration
```rust
parameter_types! {
    pub const MinRedemptionFee: Permill = Permill::from_parts(2_500);
    pub const MaxRedemptionFee: Permill = Permill::from_parts(100_000);
    pub const EmergencyReserveRatio: Permill = Permill::from_parts(900_000);
    pub const ThrottleReserveRatio: Permill = Permill::from_parts(950_000);
    pub SafetyMultiplier: FixedU128 = FixedU128::from_rational(12u128, 10u128);
    pub const MinRedemptionAmount: u128 = 10_000_000_000_000_000;
    pub const MaxPendingRedemptions: u32 = 10_000;
}

impl pallet_edsc_redemption::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinRedemptionFee = MinRedemptionFee;
    type MaxRedemptionFee = MaxRedemptionFee;
    type EmergencyReserveRatio = EmergencyReserveRatio;
    type ThrottleReserveRatio = ThrottleReserveRatio;
    type SafetyMultiplier = SafetyMultiplier;
    type MinRedemptionAmount = MinRedemptionAmount;
    type MaxPendingRedemptions = MaxPendingRedemptions;
}
```

#### Oracle Configuration
```rust
parameter_types! {
    pub const PrimaryTwapWindow: u32 = 14_400;
    pub const FallbackTwapWindow: u32 = 100_800;
    pub const MinPriceSources: u32 = 5;
    pub const OutlierThreshold: Permill = Permill::from_parts(20_000);
    pub const StalenessTimeout: u32 = 100;
    pub const MaxPriceHistory: u32 = 10_000;
}

impl pallet_edsc_oracle::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type PrimaryTwapWindow = PrimaryTwapWindow;
    type FallbackTwapWindow = FallbackTwapWindow;
    type MinPriceSources = MinPriceSources;
    type OutlierThreshold = OutlierThreshold;
    type StalenessTimeout = StalenessTimeout;
    type MaxPriceHistory = MaxPriceHistory;
}
```

#### Reserve Vault Configuration
```rust
parameter_types! {
    pub const MaxCollateralAssets: u32 = 50;
    pub const OracleUpdateFrequency: u32 = 600;
    pub const MinCollateralRatio: Permill = Permill::from_parts(1_500_000);
    pub const EmergencyRatio: Permill = Permill::from_parts(1_100_000);
}

impl pallet_reserve_vault::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxCollateralAssets = MaxCollateralAssets;
    type OracleUpdateFrequency = OracleUpdateFrequency;
    type MinCollateralRatio = MinCollateralRatio;
    type EmergencyRatio = EmergencyRatio;
}
```

#### Custodian Registry Configuration
```rust
parameter_types! {
    pub const MinBond: u128 = 1_000_000_000_000_000_000_000_000;
    pub const AttestationFrequency: u32 = 7_200;
    pub const SlashingPercent: Permill = Permill::from_parts(100_000);
    pub const MaxCustodians: u32 = 100;
}

impl pallet_custodian_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MinBond = MinBond;
    type AttestationFrequency = AttestationFrequency;
    type SlashingPercent = SlashingPercent;
    type MaxCustodians = MaxCustodians;
}
```

### Runtime Construction
```rust
construct_runtime!(
    pub struct Runtime {
        // ... existing pallets ...

        // EDSC pallets (Ëtrid Dollar Stablecoin system)
        EdscToken: pallet_edsc_token,
        EdscReceipts: pallet_edsc_receipts,
        EdscRedemption: pallet_edsc_redemption,
        EdscOracle: pallet_edsc_oracle,
        ReserveVault: pallet_reserve_vault,
        CustodianRegistry: pallet_custodian_registry,
    }
);
```

---

## Operational Flow

### Normal Operations (Reserve Ratio ≥ 100%)

1. **Price Discovery**:
   - Oracle receives prices from multiple sources
   - TWAP calculation filters outliers
   - Oracle updates redemption engine: `do_update_oracle_price()`

2. **Reserve Management**:
   - Collateral deposited into reserve vault
   - Custodians attest to off-chain reserves
   - Vault calculates total reserve value
   - Vault updates redemption engine: `do_update_reserve_ratio()`

3. **Redemption (Path 1)**:
   - User requests redemption at oracle price
   - System verifies reserve ratio ≥ 100%
   - Collateral released, EDSC burned
   - Receipt issued for transparency

### Stressed Conditions (90% ≤ Reserve Ratio < 100%)

1. **Oracle Monitoring**:
   - Continuous price updates from oracle
   - Vault detects declining reserve ratio

2. **Redemption (Path 2)**:
   - Dynamic fee applied (2.5% - 10%)
   - Fee increases as ratio approaches 90%
   - Throttling may activate below 95%

3. **Custodian Response**:
   - Custodians may add reserves
   - System auto-updates reserve ratio

### Emergency Conditions (Reserve Ratio < 90%)

1. **Emergency Activation**:
   - Reserve ratio falls below 90%
   - Redemption engine switches to Path 3
   - Normal redemptions paused

2. **Pro-Rata Settlement**:
   - Users redeem at pro-rata rate
   - Equal treatment for all holders
   - System remains fair and transparent

---

## Testing Considerations

### Unit Tests
Each pallet should include tests for:
- Core functionality (mint, burn, deposit, withdraw)
- Edge cases (zero amounts, overflow)
- Access control (unauthorized access)
- State transitions

### Integration Tests
Cross-pallet interactions:
- Oracle → Redemption price updates
- Vault → Redemption reserve ratio updates
- Custodian → Vault value attestations
- Redemption path switching based on reserve ratio

### Scenario Tests
Real-world scenarios:
1. **Normal operations**: Mint, hold, redeem at 100%+ reserve ratio
2. **Market stress**: Price volatility, reserve ratio decline to 95%
3. **Emergency**: Reserve ratio drops below 90%, Path 3 activation
4. **Custodian failures**: Missed attestations, slashing
5. **Oracle failures**: Stale prices, outlier detection

---

## Maintenance and Monitoring

### Key Metrics to Monitor

1. **Reserve Ratio**: Critical for system health
   - Alert if < 110% (approaching emergency)
   - Monitor trends over time

2. **Oracle Price Staleness**: Ensure fresh price data
   - Alert if no updates for 100+ blocks
   - Monitor source availability

3. **Custodian Attestations**: Track compliance
   - Alert on missed attestations
   - Monitor aggregate off-chain reserves

4. **Redemption Queue**: Avoid bottlenecks
   - Monitor pending redemptions
   - Alert if approaching MaxPendingRedemptions

### Runtime Upgrades

When upgrading EDSC pallets:
1. Test all pallets individually
2. Test integration with other EDSC pallets
3. Test FlareChain runtime compilation
4. Run full scenario tests
5. Plan for storage migrations if needed

---

## Security Considerations

### Access Control
- **Minting**: Restricted to authorized addresses
- **Oracle Feeding**: Authorized price feeders only
- **Custodian Registration**: Bond requirement + verification
- **Emergency Actions**: Governance control

### Economic Security
- **Collateral Ratio**: Maintained above 100% in normal conditions
- **Oracle Manipulation**: TWAP + outlier detection
- **Custodian Bonds**: Skin in the game for honest behavior
- **Redemption Fees**: Prevent gaming during stress

### Technical Security
- **Overflow Protection**: All math operations checked
- **Reentrancy**: No cross-contract calls during sensitive operations
- **Storage Limits**: Bounded vectors and maps
- **Event Emissions**: Full audit trail

---

## Future Enhancements

### Planned Improvements
1. **Additional Collateral Types**: Expand beyond current 8+ assets
2. **Dynamic Oracle**: Add more price sources, chainlink integration
3. **Automated Rebalancing**: Smart collateral management
4. **Governance Integration**: Parameter updates via on-chain voting
5. **Cross-Chain Support**: Bridge EDSC to other networks

### Scalability
- Off-chain workers for oracle price aggregation
- State rent for old receipt cleanup
- Batch redemption processing

---

## Conclusion

The EDSC pallet system demonstrates a modular, secure, and maintainable architecture for a fully-collateralized stablecoin. The clear dependency structure, internal helper pattern, and comprehensive configuration make the system:

- **Modular**: Each pallet has a single responsibility
- **Extensible**: New features can be added without breaking existing code
- **Type-Safe**: Rust compiler enforces correctness
- **Maintainable**: Clear boundaries and communication patterns
- **Production-Ready**: Successfully integrated into FlareChain runtime

All 6 pallets compile successfully and are ready for testing and deployment.

---

## Quick Reference

### File Locations
- Token: `/pallets/pallet-edsc-token/src/lib.rs`
- Receipts: `/pallets/pallet-edsc-receipts/src/lib.rs`
- Redemption: `/pallets/pallet-edsc-redemption/src/lib.rs`
- Oracle: `/pallets/pallet-edsc-oracle/src/lib.rs`
- Vault: `/pallets/pallet-reserve-vault/src/lib.rs`
- Custodian: `/pallets/pallet-custodian-registry/src/lib.rs`
- Runtime: `/05-multichain/flare-chain/runtime/src/lib.rs`

### Build Commands
```bash
# Check individual pallets
cargo check -p pallet-edsc-token
cargo check -p pallet-edsc-receipts
cargo check -p pallet-edsc-redemption
cargo check -p pallet-edsc-oracle
cargo check -p pallet-reserve-vault
cargo check -p pallet-custodian-registry

# Check FlareChain runtime
cargo check -p flare-chain-runtime

# Build WASM runtime
cargo build -p flare-chain-runtime --release
```

### Deployment Checklist
- [ ] All pallets compile without errors
- [ ] FlareChain runtime compiles with all EDSC pallets
- [ ] Unit tests pass for each pallet
- [ ] Integration tests pass
- [ ] Oracle price sources configured
- [ ] Initial collateral deposited
- [ ] Custodians registered and bonded
- [ ] Governance parameters set
- [ ] Monitoring and alerts configured

---

**Document Version**: 1.0
**Last Updated**: 2025-10-20
**Status**: ✅ All pallets successfully integrated into FlareChain runtime
