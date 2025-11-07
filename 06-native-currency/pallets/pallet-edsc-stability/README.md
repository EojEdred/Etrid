# EDSC Stability Pallet

Stablecoin reserve management and stability mechanisms for EDSC (Ëtrid Dollar Stable Coin).

## Overview

This pallet implements the EDSC stablecoin system as specified in **Ivory Papers Vol III**. EDSC is a **treasury-backed stablecoin** pegged to $1.00 USD with 100% backing from organic user purchases. Unlike over-collateralized systems like MakerDAO, EDSC does not require upfront capital or liquidation mechanisms.

## Key Features

### 1. Treasury-Backed Model
- **Multi-Asset Treasury**: Holds BTC, ETH, SOL, USDC, and other approved cryptos
- **100% Backing**: Backed by actual purchase value (not over-collateralized)
- **No Pre-Funding**: Reserve builds organically as users purchase EDSC
- **Direct Purchase/Redemption**: Users buy from and sell to reserve at $1.00

### 2. Purchase & Redemption
- **Direct Purchase**: Users buy EDSC from reserve at $1.00 with any approved crypto
- **Direct Redemption**: Users sell EDSC back to reserve at $1.00 for crypto
- **No Liquidations**: Simple buy/sell model, no debt positions
- **Purchase/Redemption Fees**: 0.1% fees prevent spam and generate treasury revenue

### 3. Stability Mechanisms

#### Direct Reserve Buy/Sell
Reserve always transacts at $1.00:
- **Users purchase EDSC**: Send crypto to reserve → Receive EDSC at $1.00
- **Users redeem EDSC**: Burn EDSC → Receive crypto from reserve at $1.00
- **Peg maintenance**: Reserve pricing keeps market near $1.00

#### Arbitrage
Price deviations create profitable arbitrage opportunities:
- **EDSC > $1.01 on DEX**: Buy from reserve at $1.00 → Sell on DEX at $1.01 → Profit $0.01
- **EDSC < $0.99 on DEX**: Buy on DEX at $0.99 → Redeem at reserve for $1.00 → Profit $0.01
- **Result**: Arbitrage brings DEX prices back to $1.00 peg

#### Emergency Circuit Breaker
- Directors can pause if peg breaks > 10%
- Prevents reserve depletion during extreme volatility
- Protects users and system stability

#### Automatic Rebalancing
- Maintains target reserve composition
- Ensures proper diversification
- Can be triggered by anyone when deviation > 5%
- Execution handled by governance or automated systems

### 4. Treasury Integration

All fees feed the Ëtrid Treasury:
- **Purchase Fees**: 0.1% of purchase value
- **Redemption Fees**: 0.1% of redemption value
- **Rebalancing Fees**: Optional fees for rebalancing execution

These funds are distributed during Consensus Day Distribution phase.

## Storage Items

| Item | Description |
|------|-------------|
| `EDSCReserveBalance` | Total value in reserve vault |
| `CurrentReserveComposition` | Actual asset allocation |
| `TargetReserveComposition` | Governance-set target allocation |
| `BackingRatio` | System-wide backing ratio (target: 100%) |
| `CirculatingEDSCSupply` | Amount released from vault |
| `ReserveVault` | Address holding initial EDSC supply |
| `EDSCBalances` | User EDSC balances |
| `TransactionHistory` | Purchase/redemption history |
| `StabilityFees` | Accumulated fees for treasury |
| `EmergencyPaused` | Circuit breaker status |

## Extrinsics

### User Operations

#### `purchase_edsc(payment_token, payment_amount)`
Purchase EDSC from reserve with crypto at $1.00 rate.

**Example:**
- Send 1000 USDC to reserve
- Pay 1 USDC fee (0.1%)
- Receive 999 EDSC
- Reserve holds 1000 USDC backing

#### `redeem_edsc(edsc_amount, preferred_payment)`
Redeem EDSC back to reserve for crypto at $1.00 rate.

**Example:**
- Burn 999 EDSC
- Pay 0.999 USDC fee (0.1%)
- Receive 998.001 USDC from reserve

#### `get_backing_ratio()`
Check current reserve backing ratio.

### System Management

#### `trigger_rebalance()`
Anyone can trigger if deviation > 5% from target composition.

#### `emergency_pause()`
Directors pause if peg deviation > 10%.

#### `update_target_composition(new_composition)`
Governance updates target reserve allocation (typically on Consensus Day).

## Stability Mechanism Explanation

### How EDSC Maintains $1.00 Peg

#### Direct Reserve Pricing
- Reserve always buys/sells at exactly $1.00
- No interest rate adjustments needed
- Simple, predictable mechanism

#### Arbitrage Maintains DEX Prices
**When EDSC > $1.01 on Uniswap:**
1. Arbitrageur buys from reserve at $1.00
2. Sells on Uniswap at $1.01
3. Profit: $0.01 per EDSC
4. Selling pressure brings Uniswap price down to $1.00

**When EDSC < $0.99 on Uniswap:**
1. Arbitrageur buys on Uniswap at $0.99
2. Redeems with reserve at $1.00
3. Profit: $0.01 per EDSC
4. Buying pressure brings Uniswap price up to $1.00

#### Reserve Diversification

Multi-asset backing provides:
- **Reduced Volatility**: Diversification smooths out price swings
- **Peg Stability**: Multiple asset types provide stability
- **Resilience**: System doesn't depend on single asset

**Target Allocation (accumulates organically):**
- 30% USDC: Stablecoin purchases
- 25% ETH: Ethereum exposure, DeFi connectivity
- 20% BTC: Bitcoin exposure, store of value
- 15% SOL: Solana exposure
- 10% Other: Flexibility for new assets

#### Emergency Measures

**Circuit Breaker (> 10% deviation):**
- Pauses purchase/redemption
- Prevents reserve depletion
- Gives governance time to respond
- Protects users during extreme volatility

### Reserve Rebalancing

**Automatic Trigger:** When any asset deviates > 5% from target

**Example:**
- Target: 30% USDC, 25% ETH, 20% BTC, 15% SOL, 10% Other
- Current: 40% USDC, 25% ETH, 20% BTC, 10% SOL, 5% Other
- USDC deviation: 10% (exceeds 5% threshold)
- **Rebalance triggered**: Sell 10% USDC, buy SOL and Other

**Benefits:**
- Maintains diversification
- Reduces single-asset risk
- Supports long-term stability
- Transparent, rule-based execution

## Integration with Consensus Day

### Pre-Consensus Phase
- Proposals can adjust target reserve composition
- Validators vote on fee rate changes
- Directors can propose emergency measures

### Distribution Phase
Purchase/redemption fees distributed:
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
| `TargetBackingRatio` | 10000 (100%) | Target backing ratio |
| `PurchaseFee` | 10 (0.1%) | Fee on EDSC purchases |
| `RedemptionFee` | 10 (0.1%) | Fee on EDSC redemptions |
| `RebalanceThreshold` | 500 (5%) | Deviation trigger for rebalancing |
| `EmergencyPauseThreshold` | 1000 (10%) | Peg deviation for circuit breaker |
| `MinEDSCPurchase` | 100 EDSC | Minimum purchase amount |

## Events

- `EDSCPurchased`: User purchased EDSC from reserve
- `EDSCRedeemed`: User redeemed EDSC to reserve
- `ReserveRebalanced`: Reserve composition adjusted
- `EmergencyPauseActivated`: Circuit breaker triggered
- `StabilityFeesCollected`: Fees sent to treasury
- `TargetCompositionUpdated`: Governance changed allocation
- `BackingRatioUpdated`: Backing ratio changed

## Error Handling

- `SystemPaused`: Cannot execute during emergency pause
- `InsufficientReserveBalance`: Not enough crypto in reserve
- `BelowMinimumPurchase`: Purchase amount too small
- `InsufficientEDSCBalance`: Not enough EDSC to redeem
- `RebalancingNotNeeded`: Composition within tolerance
- `PegDeviationNotCritical`: Peg stable, no emergency needed

## Key Differences from Over-Collateralized Model

### ❌ OLD Model (MakerDAO-style):
- Requires $1M+ upfront capital
- Users lock collateral to borrow EDSC
- 150% over-collateralization required
- Liquidations at 120% ratio
- Interest charges on borrowed EDSC
- Complex CDP (Collateralized Debt Position) system

### ✅ NEW Model (Treasury-backed):
- No upfront capital required
- Users purchase EDSC with crypto
- 100% backing from purchase value
- No liquidations (direct buy/sell only)
- No interest charges (not a loan)
- Simple purchase/redemption system

## Testing Recommendations

1. **Purchase Tests**: Verify purchase at $1.00 with fees
2. **Redemption Tests**: Confirm redemption at $1.00 with fees
3. **Backing Ratio**: Test backing ratio calculation
4. **Rebalancing Logic**: Verify deviation detection and execution
5. **Emergency Pause**: Test circuit breaker triggers correctly
6. **Edge Cases**: Test boundary conditions, zero balances, etc.

## Security Considerations

1. **Oracle Dependency**: Price feeds must be secure and accurate
2. **Reserve Security**: Vault must be multisig with proper key management
3. **Fee Bounds**: Cap fees at reasonable levels to prevent exploitation
4. **Rebalancing Execution**: Must be atomic to prevent front-running
5. **Emergency Powers**: Director pause authority should be multi-sig
6. **Reserve Depletion**: Monitor backing ratio to prevent undercollateralization

## Future Enhancements

- Implement buyEDSC() and sellEDSC() smart contract functions
- Add reserve value tracking and transparency dashboard
- Support additional payment tokens (more bridges)
- Integration with external DEXs for rebalancing execution
- Oracle price feed integration for accurate valuations
- Flash loan protection mechanisms
- Governance-adjustable parameters via proposals

## Deployment Process

### Initial Setup:
1. Deploy EDSC contract on each chain
2. Mint 1 billion EDSC to reserve vault (multisig)
3. No backing required initially (nothing in circulation)
4. Configure approved payment tokens

### First Purchase:
1. User sends crypto to reserve
2. Reserve releases EDSC from vault
3. Backing accumulates organically
4. Backing ratio = (reserve value / circulating supply) = 100%

### Ongoing Operation:
1. Users purchase/redeem at $1.00 via reserve
2. DEX prices tracked by arbitrageurs
3. Rebalancing maintains target allocations
4. Fees accumulate for treasury distribution

## License

Apache 2.0
