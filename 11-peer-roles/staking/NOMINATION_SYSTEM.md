# Ëtrid Nomination System

## Overview

The Ëtrid Nomination System allows token holders to delegate their stake to validators and earn proportional rewards without running validator infrastructure. This document describes the complete nomination system architecture, economics, and best practices.

## Table of Contents

1. [Architecture](#architecture)
2. [Core Components](#core-components)
3. [Economic Model](#economic-model)
4. [Extrinsics](#extrinsics)
5. [Usage Guide](#usage-guide)
6. [Best Practices](#best-practices)
7. [Security Considerations](#security-considerations)
8. [FAQ](#faq)

## Architecture

### System Components

The nomination system consists of three primary components:

1. **Validators**: Network participants who register to accept nominations and operate validator infrastructure
2. **Nominators**: Token holders who delegate stake to validators
3. **Reward Distribution**: Automated system that distributes staking rewards proportionally

### Data Structures

#### ValidatorProfile

```rust
pub struct ValidatorProfile<AccountId, Balance> {
    pub validator: AccountId,
    pub total_stake: Balance,      // Own + nominated stake
    pub self_stake: Balance,        // Validator's own stake only
    pub nominated_stake: Balance,   // Total from all nominators
    pub nominator_count: u32,       // Number of nominators
    pub commission_rate: u8,        // 0-100 (percentage)
    pub active: bool,               // Validator status
}
```

#### Nomination

```rust
pub struct Nomination<AccountId, Balance> {
    pub nominator: AccountId,
    pub validator: AccountId,
    pub amount: Balance,
    pub rewards_earned: Balance,    // Cumulative rewards tracked
}
```

### Storage Layout

- **ValidatorProfiles**: Single map indexed by validator account
- **Nominations**: Double map indexed by (nominator, validator)
- **NominatorValidators**: Tracks up to 16 validators per nominator

## Core Components

### 1. Validator Registration

Validators must register before accepting nominations:

**Requirements:**
- Minimum self-stake: Configurable (default: 64 ËTR)
- Commission rate: 0-100%
- Sufficient free balance

**Process:**
1. Validator calls `register_validator(self_stake, commission_rate)`
2. System reserves `self_stake` from validator's balance
3. ValidatorProfile created with initial state
4. Validator becomes eligible for nominations

### 2. Nomination Management

Nominators can delegate stake to active validators:

**Constraints:**
- Minimum nomination: Configurable (default: 10 ËTR)
- Maximum validators per nominator: 16
- Maximum nominators per validator: 256
- Funds are reserved (locked) during nomination

**Operations:**
- **Create nomination**: First-time delegation to a validator
- **Increase nomination**: Add more stake to existing nomination
- **Withdraw nomination**: Remove partial or full nomination

### 3. Reward Distribution

Rewards are distributed proportionally based on stake weight:

**Formula:**
```
Total Reward = R
Commission = R × (commission_rate / 100)
Nominator Pool = R - Commission

For each nominator:
  Share = Nominator Pool × (nomination_amount / total_nominated_stake)
```

**Properties:**
- Commission paid to validator first
- Remaining rewards split proportionally among nominators
- Rewards tracked cumulatively per nomination
- Automatic balance credit (no claim needed)

## Economic Model

### Commission Structure

Validators set commission rates (0-100%) to balance:
- **Lower commission**: Attracts more nominators
- **Higher commission**: Increases validator revenue

**Examples:**

| Commission | Reward Split | Use Case |
|-----------|--------------|----------|
| 0% | 100% to nominators | Community validators |
| 5-10% | 90-95% to nominators | Competitive validators |
| 15-25% | 75-85% to nominators | Premium services |
| 100% | 100% to validator | Private/institutional |

### Stake Distribution

**Total Validator Stake = Self-Stake + Nominated Stake**

Example validator with 10% commission:
```
Self-Stake: 100 ËTR
Nominated Stake: 400 ËTR (4 nominators × 100 ËTR each)
Total Stake: 500 ËTR

Block Reward: 1000 ËTR
Commission: 100 ËTR → Validator
Nominator Pool: 900 ËTR
Each Nominator: 900 ÷ 4 = 225 ËTR
```

### Minimum Requirements

| Parameter | Default Value | Purpose |
|-----------|---------------|---------|
| MinValidatorStake | 64 ËTR | Ensures validator commitment |
| MinNominationAmount | 10 ËTR | Prevents dust nominations |
| MaxNominatorsPerValidator | 256 | Storage/performance limit |
| MaxValidatorsPerNominator | 16 | Risk diversification |

## Extrinsics

### Validator Operations

#### register_validator

```rust
pub fn register_validator(
    origin: OriginFor<T>,
    self_stake: BalanceOf<T>,
    commission_rate: u8,
) -> DispatchResult
```

**Parameters:**
- `self_stake`: Amount to self-stake (≥ MinValidatorStake)
- `commission_rate`: 0-100 (percentage)

**Errors:**
- `InvalidCommissionRate`: Commission > 100
- `InsufficientStake`: Below minimum
- `InsufficientBalance`: Not enough free balance

#### update_commission

```rust
pub fn update_commission(
    origin: OriginFor<T>,
    new_rate: u8,
) -> DispatchResult
```

**Parameters:**
- `new_rate`: New commission rate (0-100)

**Errors:**
- `InvalidCommissionRate`: Rate > 100
- `NotValidator`: Caller not registered

### Nominator Operations

#### nominate

```rust
pub fn nominate(
    origin: OriginFor<T>,
    validator: T::AccountId,
    amount: BalanceOf<T>,
) -> DispatchResult
```

**Parameters:**
- `validator`: Target validator account
- `amount`: Amount to nominate (≥ MinNominationAmount)

**Behavior:**
- Creates new nomination if first time
- Increases existing nomination if already nominated
- Reserves funds from nominator's balance

**Errors:**
- `ValidatorNotFound`: Validator not registered
- `ValidatorInactive`: Validator deactivated
- `TooManyNominators`: Validator at capacity (256)
- `TooManyValidators`: Nominator at capacity (16)
- `InsufficientNomination`: Below minimum
- `InsufficientBalance`: Not enough free balance

#### withdraw_nomination

```rust
pub fn withdraw_nomination(
    origin: OriginFor<T>,
    validator: T::AccountId,
    amount: BalanceOf<T>,
) -> DispatchResult
```

**Parameters:**
- `validator`: Target validator account
- `amount`: Amount to withdraw

**Behavior:**
- Unreserves funds immediately (no unbonding period for nominations)
- Removes nomination if fully withdrawn
- Updates validator's nominated_stake and nominator_count

**Errors:**
- `NominationNotFound`: No nomination exists
- `InsufficientNominatedStake`: Withdrawal exceeds nomination

### Internal Functions

#### distribute_rewards

```rust
pub fn distribute_rewards(
    validator: &T::AccountId,
    total_reward: BalanceOf<T>,
) -> DispatchResult
```

Called internally by consensus/reward mechanism. Not directly callable.

## Usage Guide

### For Validators

#### 1. Registration

```rust
// Register with 100 ËTR self-stake and 10% commission
Staking::register_validator(
    Origin::signed(validator_account),
    100_000_000_000_000_000_000, // 100 ËTR (18 decimals)
    10
)?;
```

#### 2. Update Commission

```rust
// Change commission to 15%
Staking::update_commission(
    Origin::signed(validator_account),
    15
)?;
```

#### 3. Monitor Nominations

Query validator profile:
```rust
let profile = ValidatorProfiles::<T>::get(&validator_account).unwrap();
println!("Total stake: {}", profile.total_stake);
println!("Nominators: {}", profile.nominator_count);
println!("Commission: {}%", profile.commission_rate);
```

### For Nominators

#### 1. Choose Validator

Considerations:
- Commission rate (lower = more rewards)
- Total stake (indicates popularity/trust)
- Uptime and performance history
- Self-stake amount (skin in the game)

#### 2. Nominate

```rust
// Nominate 50 ËTR to chosen validator
Staking::nominate(
    Origin::signed(nominator_account),
    validator_account,
    50_000_000_000_000_000_000 // 50 ËTR
)?;
```

#### 3. Increase Nomination

```rust
// Add 25 more ËTR to existing nomination
Staking::nominate(
    Origin::signed(nominator_account),
    validator_account,
    25_000_000_000_000_000_000 // 25 ËTR
)?;
```

#### 4. Diversify

```rust
// Nominate multiple validators (up to 16)
for validator in selected_validators {
    Staking::nominate(
        Origin::signed(nominator_account),
        validator,
        allocation_per_validator
    )?;
}
```

#### 5. Withdraw

```rust
// Withdraw 10 ËTR from nomination
Staking::withdraw_nomination(
    Origin::signed(nominator_account),
    validator_account,
    10_000_000_000_000_000_000 // 10 ËTR
)?;
```

#### 6. Track Rewards

```rust
let nomination = Nominations::<T>::get(&nominator_account, &validator_account).unwrap();
println!("Total rewards earned: {}", nomination.rewards_earned);
```

## Best Practices

### For Validators

1. **Competitive Commission**: Research market rates (typically 5-15%)
2. **Transparent Communication**: Publish performance metrics and plans
3. **Reliable Infrastructure**: Maintain high uptime (>99%)
4. **Gradual Commission Changes**: Avoid sudden increases
5. **Sufficient Self-Stake**: Demonstrate commitment (>10% of total stake)

### For Nominators

1. **Diversification**: Spread nominations across 5-10 validators
2. **Due Diligence**: Research validator history and reputation
3. **Regular Monitoring**: Check validator performance monthly
4. **Commission Awareness**: Balance low commission vs. validator quality
5. **Start Small**: Test with minimal amounts before large commitments

### Risk Management

**Nominator Risks:**
- Validator downtime reduces rewards
- No slashing exposure (only validators are slashed)
- Opportunity cost of locked funds

**Validator Risks:**
- Slashing for misbehavior
- Reputation damage
- Loss of nominators

**Mitigation:**
1. **Nominators**: Diversify across multiple validators
2. **Validators**: Implement robust monitoring and failover
3. **Both**: Stay informed about network upgrades

## Security Considerations

### Fund Safety

1. **No Loss of Principal**: Nominators cannot lose their staked funds to slashing
   - Only validators face slashing risk
   - Nominations can be fully withdrawn at any time

2. **Balance Reservation**: Nominated funds are reserved (not transferable)
   - Prevents accidental spending
   - Unreserved immediately upon withdrawal

3. **No Unbonding Period**: Unlike validator stakes, nominations can be withdrawn instantly
   - Provides liquidity flexibility
   - Allows quick response to validator issues

### Attack Vectors

1. **Validator Centralization**
   - Mitigation: MaxNominatorsPerValidator limit (256)
   - Encourages distribution across validators

2. **Commission Rug Pull**
   - Risk: Validator suddenly increases commission to 100%
   - Mitigation: Nominators can withdraw immediately

3. **Sybil Validators**
   - Risk: Single entity running multiple validators
   - Mitigation: MinValidatorStake requirement

4. **Dust Nominations**
   - Risk: Storage bloat from tiny nominations
   - Mitigation: MinNominationAmount requirement

## FAQ

### General Questions

**Q: What's the difference between staking and nominating?**

A: Staking refers to validators locking funds to participate in consensus. Nominating is when token holders delegate their stake to validators without running infrastructure.

**Q: Can I lose my nominated funds?**

A: No. Nominators do not face slashing risk. Only the validator's self-stake can be slashed for misbehavior. Your nomination remains safe.

**Q: Is there an unbonding period for nominations?**

A: No. Nominations can be withdrawn immediately, unlike validator stakes which have an unbonding period.

**Q: How often are rewards distributed?**

A: Rewards are distributed automatically after each block/era based on the consensus mechanism's reward schedule.

### Validator Questions

**Q: What's the optimal commission rate?**

A: Most competitive validators use 5-15%. Lower rates attract more nominators but reduce per-block revenue. Find a balance based on your costs and market rates.

**Q: Can I change commission after registration?**

A: Yes, using `update_commission`. However, sudden increases may cause nominators to withdraw.

**Q: What happens if I go offline?**

A: You won't earn block rewards during downtime, reducing rewards for your nominators. Extended downtime may result in nominators withdrawing.

**Q: How much should I self-stake?**

A: Minimum is 64 ËTR, but nominators prefer validators with substantial self-stake (20%+ of total stake) as it demonstrates commitment.

### Nominator Questions

**Q: How do I choose a validator?**

A: Consider:
1. Commission rate (lower = more rewards)
2. Uptime/reliability (check performance history)
3. Total stake (indicates trust)
4. Self-stake percentage (skin in the game)
5. Community reputation

**Q: Should I nominate one validator or multiple?**

A: Multiple validators (5-10) is recommended for diversification. If one validator underperforms, others continue earning.

**Q: What's the minimum nomination amount?**

A: Default is 10 ËTR, but this is configurable per network.

**Q: Can I withdraw part of my nomination?**

A: Yes, you can withdraw any amount up to your total nomination. The remainder stays nominated.

**Q: Do I need to claim rewards?**

A: No. Rewards are automatically credited to your free balance after each distribution.

**Q: Can I nominate if I'm running a validator?**

A: Yes, but you cannot nominate yourself. You can nominate other validators with separate funds.

### Technical Questions

**Q: How is my reward share calculated?**

A:
```
Your Share = (Total Reward - Commission) × (Your Nomination / Total Nominated Stake)
```

**Q: What happens if multiple nominators withdraw simultaneously?**

A: The validator's `nominated_stake` and `total_stake` are updated atomically per transaction. No race conditions.

**Q: Are there any front-running risks?**

A: No. Reward distribution happens deterministically based on on-chain state at the time of distribution.

**Q: What if a validator reaches max nominators (256)?**

A: New nominators cannot nominate that validator until someone withdraws. Existing nominators can still increase their nominations.

## Support and Resources

### Documentation
- Main repository: [etrid-protocol/etrid](https://github.com/etrid-protocol/etrid)
- Substrate documentation: https://docs.substrate.io

### Community
- Discord: [Join server](https://discord.gg/etrid)
- Forum: https://forum.etrid.network

### Tools
- Block explorer: https://explorer.etrid.network
- Validator dashboard: https://validators.etrid.network

---

**Version:** 1.0.0
**Last Updated:** 2025-10-22
**Component:** 11-peer-roles/staking
