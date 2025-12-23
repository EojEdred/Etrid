# EDSC Stablecoin Reserve Vault System Design

## 1. Reserve Vault Architecture

The EDSC reserve vault operates as a native ĒTRID system without CCTP dependency. The architecture comprises three core components:

- **Reserve Vault Contract**: Manages collateral (USDC) deposits/withdrawals and maintains reserve ratio targets (100%+ overcollateralization)
- **EDSC Minting Engine**: Issues/burns EDSC tokens maintaining 1:1 peg with USD value, integrated into the pallet system
- **Peg Stabilization Module**: Autonomous mechanism detecting and correcting price deviations through reserve rebalancing

The vault operates with the constraint that EDSC supply never exceeds available USDC reserves at 1:1 ratio. All operations execute on-chain through substrate pallets, eliminating centralized intermediaries while maintaining regulatory transparency.

## 2. 1:1 Peg Maintenance Mechanism

**Core Algorithm:**
```
Target Reserve Ratio = USDC Balance / EDSC Circulating Supply ≥ 1.0

If Price(EDSC) > $1.00:
  → Trigger EDSC minting (max supply = USDC reserves)
  → Sell EDSC to market, buying USDC
  → Stabilize to $1.00

If Price(EDSC) < $1.00:
  → Trigger EDSC burning (reduces supply)
  → Purchase EDSC from market with USDC reserves
  → Stabilize to $1.00

Fee Mechanism: 0.15% mint/burn fee → reserve buffer growth
```

The peg maintains autonomously via oracle price feeds (substrate finality oracle) triggering rebalancing when deviation exceeds ±2%. Reserve surplus accumulates as system-backed collateral insurance.

## 3. One-Sided Liquidity Pool Mechanics

The one-sided pool accepts only USDC deposits, eliminating impermanent loss for liquidity providers:

- **Deposit Phase**: Users deposit USDC → receive LP tokens representing reserve share
- **Redemption Phase**: LP holders burn tokens → claim proportional USDC + accumulated fee share
- **Reserve Backing**: 100% of pool USDC always available for EDSC redemptions at 1:1

This mechanism allows EDSC trading on DEXs while maintaining vault reserves separate. The protocol never uses reserves for liquidity mining or yield strategies—reserves are purely collateral.

## 4. Compliance Framework

**Regulatory Essentials:**
- **AML/KYC Integration**: Pallet hooks verify users against compliance oracle (future integration with regulatory partners)
- **Transaction Limits**: Progressive limits per user tier (casual: $10k/day, verified: $100k/day, institutional: unlimited)
- **Reserve Attestation**: Monthly on-chain proofs of USDC backing (cryptographic commitment to bank statements)
- **Audit Trail**: All mint/burn/withdrawal operations logged immutably on-chain with user identifiers
- **Blacklist Support**: Emergency pause of non-compliant accounts via governance

## 5. Implementation Phases

### Phase 1: Core Infrastructure (Weeks 1-4)
- Design substrate pallet structure (reserve_vault, edsc_token, peg_stabilization)
- Implement USDC deposit/withdrawal logic
- Deploy oracle price feed integration
- Unit test all reserve ratio calculations

### Phase 2: Peg Stabilization (Weeks 5-8)
- Build automated rebalancing algorithm with oracle triggers
- Implement fee collection and reserve growth mechanics
- Deploy testnet with price manipulation resistance testing
- Audit stabilization logic for edge cases

### Phase 3: One-Sided Pool & DEX Integration (Weeks 9-12)
- Develop LP token minting/burning mechanism
- Integrate with Uniswap V4 equivalent on ĒTRID (primeswap)
- Implement fee distribution to LP holders
- Test liquidity provision/withdrawal flows

### Phase 4: Compliance & Production Launch (Weeks 13-16)
- Integrate compliance oracle hooks
- Implement transaction limits and blacklist functionality
- Deploy on testnet with regulatory testing
- Full security audit (internal + external)
- Mainnet deployment with phased rollout

## 6. Core Smart Contracts

### ReserveVault Pallet
```rust
pub trait ReserveVault {
    fn deposit_usdc(account: T::AccountId, amount: u128) → DispatchResult
    fn withdraw_usdc(account: T::AccountId, amount: u128) → DispatchResult
    fn get_reserve_ratio() → Ratio
    fn emergency_pause(reason: &str) → DispatchResult
}
```

### EDSCToken Pallet
```rust
pub trait EDSCToken {
    fn mint(recipient: T::AccountId, amount: u128) → DispatchResult
    fn burn(account: T::AccountId, amount: u128) → DispatchResult
    fn total_supply() → u128
    fn balances(account: T::AccountId) → u128
}
```

### PegStabilization Pallet
```rust
pub trait PegStabilization {
    fn rebalance(current_price: u64) → DispatchResult
    fn calculate_fee(amount: u128, operation: Op) → u128
    fn oracle_price_update(price: u64) → DispatchResult
}
```

## 7. Reserve Management Strategy

**Capital Allocation:**
- 100% liquid USDC collateral (no farming/lending)
- Fee accumulation: 0.05% mint, 0.10% burn + optional trade fees
- Overcollateralization buffer: 2-5% above 1:1 ratio
- Emergency reserve: 5% of total supply held in separate multi-sig

**Monitoring Metrics:**
- Real-time reserve ratio tracking
- Cumulative fee accumulation dashboard
- Oracle price deviation alerts (>1%)
- Monthly settlement reconciliation with bank

## 8. Future Enhancement: USDT Support

Once USDC stability proven (3+ months mainnet):
- Add USDT as secondary collateral (capped 30% of total reserves)
- Implement cross-collateral arbitrage protection
- Deploy dual-peg stabilization (each stablecoin 1:1)
- Maintain separate reserve buckets with weighted ratios

---

**Design Status:** Ready for Phase 1 implementation
**Security Focus:** Reserve integrity, oracle resilience, compliance automation
**Success Metric:** EDSC trades within ±0.5% of $1.00 for 90%+ of time
