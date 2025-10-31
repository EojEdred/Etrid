# EDSC Stability Pallet

Stablecoin reserve management and stability mechanisms for EDSC (Ëtrid Dollar Stable Coin).

## Overview

This pallet implements the EDSC stablecoin system as specified in **Ivory Papers Vol III**. EDSC is a reserve-backed, over-collateralized stablecoin pegged to 1 USD with multi-asset backing.

## Key Features

### 1. Reserve Backing
- **Multi-Asset Reserve**: 40% ËTR, 30% sBTC, 20% sETH, 10% Other
- **150% Minimum Collateralization**: Users must maintain at least 150% collateral ratio
- **120% Liquidation Threshold**: Positions can be liquidated below 120%
- **Automatic Rebalancing**: Triggers when reserve deviates > 5% from target allocation

### 2. Minting & Burning
- **Collateralized Minting**: Lock ËTR (or other approved assets) to mint EDSC
- **Proportional Redemption**: Burn EDSC to retrieve collateral
- **Interest Accrual**: Positions accrue interest based on current rate
- **Flexible Collateral**: Users can add collateral to improve health

### 3. Stability Mechanisms

#### Interest Rate Adjustments
Dynamic interest rates help maintain the $1.00 peg:
- **EDSC > $1.01**: Lower interest rate → encourages minting → increases supply → price decreases
- **EDSC < $0.99**: Raise interest rate → encourages burning → decreases supply → price increases

#### Liquidation System
- Anyone can liquidate undercollateralized positions
- Liquidator provides EDSC, receives collateral
- 5% liquidation penalty goes to treasury
- Helps maintain system solvency

#### Emergency Circuit Breaker
- Directors can pause if peg breaks > 10%
- Prevents cascading liquidations during extreme volatility
- Protects users and system stability

#### Automatic Rebalancing
- Maintains target reserve composition
- Ensures proper diversification
- Can be triggered by anyone when deviation > 5%
- Execution handled by governance or automated systems

### 4. Treasury Integration

All stability mechanisms feed the Ëtrid Treasury:
- **Stability Fees**: Interest payments on borrowed EDSC
- **Liquidation Penalties**: 5% of liquidated collateral
- **Interest Income**: Accrued interest from all positions

These funds are distributed during Consensus Day Distribution phase.

## Storage Items

| Item | Description |
|------|-------------|
| `EDSCReserveBalance` | Total value in reserve account |
| `CurrentReserveComposition` | Actual asset allocation |
| `TargetReserveComposition` | Governance-set target allocation |
| `CollateralizationRatio` | System-wide collateral ratio |
| `InterestRate` | Current annual interest rate (basis points) |
| `TotalEDSCSupply` | Total EDSC in circulation |
| `EDSCBalances` | User EDSC balances |
| `Positions` | User collateral positions |
| `StabilityFees` | Accumulated fees for treasury |
| `EmergencyPaused` | Circuit breaker status |

## Extrinsics

### User Operations

#### `deposit_collateral_mint_edsc(collateral_amount, edsc_amount)`
Lock collateral and mint EDSC at 150% ratio.

**Example:**
- Lock 1500 ËTR
- Mint 1000 EDSC
- Collateral ratio: 150%

#### `burn_edsc_withdraw_collateral(edsc_amount)`
Burn EDSC to retrieve collateral. Must pay accrued interest.

**Example:**
- Burn 1000 EDSC
- Pay 10 EDSC in interest
- Receive ~1500 ËTR back

#### `add_collateral(amount)`
Add more collateral to improve position health.

### Liquidation

#### `liquidate_position(owner)`
Liquidate undercollateralized position (<120% ratio).

**Process:**
1. Verify position is below 120%
2. Liquidator provides EDSC to burn
3. Liquidator receives collateral minus 5% penalty
4. Penalty sent to treasury
5. Position closed

### System Management

#### `trigger_rebalance()`
Anyone can trigger if deviation > 5% from target composition.

#### `adjust_interest_rate(new_rate)`
Governance adjusts rate to maintain peg (0-50% annual).

#### `emergency_pause()`
Directors pause if peg deviation > 10%.

#### `update_target_composition(new_composition)`
Governance updates target reserve allocation (typically on Consensus Day).

## Stability Mechanism Explanation

### How EDSC Maintains $1.00 Peg

#### Supply-Side Adjustments (Interest Rates)

**When EDSC > $1.01 (Trading Above Peg):**
1. Governance lowers interest rate
2. Borrowing EDSC becomes cheaper
3. More users mint EDSC
4. Supply increases
5. Price decreases back toward $1.00

**When EDSC < $0.99 (Trading Below Peg):**
1. Governance raises interest rate
2. Borrowing EDSC becomes expensive
3. Users burn EDSC to avoid interest
4. Supply decreases
5. Price increases back toward $1.00

#### Demand-Side Support (Liquidations)

- Under-collateralized positions create automatic buying pressure
- Liquidators must acquire EDSC to liquidate positions
- Creates natural demand floor
- Maintains system solvency

#### Reserve Diversification

Multi-asset backing provides:
- **Reduced Volatility**: Diversification smooths out price swings
- **Peg Stability**: Multiple asset types provide stability
- **Resilience**: System doesn't depend on single asset

**Target Allocation:**
- 40% ËTR: Native token, governance rights
- 30% sBTC: Bitcoin exposure, store of value
- 20% sETH: Ethereum exposure, DeFi connectivity
- 10% Other: Flexibility for new assets

#### Emergency Measures

**Circuit Breaker (> 10% deviation):**
- Pauses minting/burning
- Prevents panic-driven liquidation cascades
- Gives governance time to respond
- Protects users during extreme volatility

### Reserve Rebalancing

**Automatic Trigger:** When any asset deviates > 5% from target

**Example:**
- Target: 40% ËTR, 30% sBTC, 20% sETH, 10% Other
- Current: 50% ËTR, 25% sBTC, 20% sETH, 5% Other
- ËTR deviation: 10% (exceeds 5% threshold)
- **Rebalance triggered**: Sell 10% ËTR, buy sBTC and Other

**Benefits:**
- Maintains diversification
- Reduces single-asset risk
- Supports long-term stability
- Transparent, rule-based execution

## Integration with Consensus Day

### Pre-Consensus Phase
- Proposals can adjust target reserve composition
- Validators vote on interest rate changes
- Directors can propose emergency measures

### Distribution Phase
Stability fees distributed:
1. **60%** to Active Validators
2. **20%** to Standby Validators
3. **10%** to Directors
4. **10%** to Development Fund

### Emergency Reserve
- Part of EDSC reserve can be used for peg defense
- Requires supermajority vote
- Only during severe market dislocations

## Configuration Constants

| Constant | Default | Description |
|----------|---------|-------------|
| `MinCollateralRatio` | 15000 (150%) | Minimum collateral ratio |
| `LiquidationThreshold` | 12000 (120%) | Liquidation trigger point |
| `LiquidationPenalty` | 500 (5%) | Penalty on liquidated collateral |
| `RebalanceThreshold` | 500 (5%) | Deviation trigger for rebalancing |
| `EmergencyPauseThreshold` | 1000 (10%) | Peg deviation for circuit breaker |
| `MinEDSCMint` | 100 EDSC | Minimum mint amount |
| `BaseInterestRate` | 300 (3%) | Default annual interest rate |

## Events

- `EDSCMinted`: User minted EDSC
- `EDSCBurned`: User burned EDSC
- `PositionLiquidated`: Position liquidated
- `ReserveRebalanced`: Reserve composition adjusted
- `InterestRateAdjusted`: Rate changed to maintain peg
- `EmergencyPauseActivated`: Circuit breaker triggered
- `StabilityFeesCollected`: Fees sent to treasury
- `TargetCompositionUpdated`: Governance changed allocation

## Error Handling

- `SystemPaused`: Cannot execute during emergency pause
- `BelowMinimumCollateralRatio`: Insufficient collateral
- `PositionHealthy`: Cannot liquidate healthy position
- `InsufficientEDSCBalance`: Not enough EDSC to burn
- `RebalancingNotNeeded`: Composition within tolerance
- `PegDeviationNotCritical`: Peg stable, no emergency needed

## Testing Recommendations

1. **Collateralization Tests**: Verify 150% minimum enforced
2. **Liquidation Tests**: Confirm liquidations work below 120%
3. **Interest Accrual**: Test interest calculations over time
4. **Rebalancing Logic**: Verify deviation detection and execution
5. **Emergency Pause**: Test circuit breaker triggers correctly
6. **Edge Cases**: Test boundary conditions (exactly 150%, 120%, etc.)

## Security Considerations

1. **Oracle Dependency**: Price feeds must be secure and accurate
2. **Liquidation Incentives**: 5% penalty must incentivize timely liquidations
3. **Interest Rate Bounds**: Cap at 50% to prevent extreme rates
4. **Rebalancing Execution**: Must be atomic to prevent front-running
5. **Emergency Powers**: Director pause authority should be multi-sig
6. **Collateral Safety**: Reserved tokens are locked and cannot be transferred

## Future Enhancements

- Dynamic liquidation penalties based on market conditions
- Multi-collateral support (accept ETH, BTC directly)
- Governance-adjustable parameters via proposals
- Integration with external DEXs for rebalancing execution
- Oracle price feed integration for accurate valuations
- Flash loan protection mechanisms
- Partial liquidations to improve capital efficiency

## License

Apache 2.0
