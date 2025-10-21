# 11-peer-roles: Peer Roles & Staking Architecture

**Component:** Ëtrid Peer Role Classification & Staking System
**Type:** Identity, Staking, and Validator Management
**Status:** Production Implementation
**Last Updated:** 2025-10-20

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Diagram](#architecture-diagram)
3. [Core Components](#core-components)
4. [Peer Type Hierarchy](#peer-type-hierarchy)
5. [Staking Mechanism](#staking-mechanism)
6. [Role Assignment & Lifecycle](#role-assignment--lifecycle)
7. [Validator Operations](#validator-operations)
8. [Rewards & Slashing](#rewards--slashing)
9. [Integration Points](#integration-points)
10. [Security Considerations](#security-considerations)
11. [Future Enhancements](#future-enhancements)

---

## Overview

The Peer Roles & Staking system implements Ëtrid's multi-tiered identity and validator management framework. It defines the relationship between stake amount, network responsibilities, and governance participation, creating a meritocratic hierarchy that aligns incentives with network security.

### Key Features

- **Five-Tier Peer System**: Common → Staking Common → Validity Node → Flare Node → Director
- **Stake-Based Progression**: Higher stake unlocks greater responsibilities and rewards
- **Dynamic Role Assignment**: Validators can upgrade/downgrade based on stake
- **Economic Security**: Slashing for misbehavior protects network integrity
- **Reward Distribution**: Incentivizes honest participation and uptime
- **Governance Integration**: Role affects voting power and proposal rights

### Design Principles

1. **Meritocracy**: Stake and performance determine influence
2. **Progressive Responsibility**: Higher tiers = greater duties + rewards
3. **Economic Alignment**: Validators risk capital for network security
4. **Flexibility**: Easy stake increases/decreases without lockup
5. **Transparency**: All stakes and roles publicly visible

---

## Architecture Diagram

```text
┌─────────────────────────────────────────────────────────────────────┐
│                   PEER ROLES & STAKING SYSTEM                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                 Staking Pallet (Core)                        │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │                                                              │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │           Role Assignment Module                       │  │  │
│  │  ├────────────────────────────────────────────────────────┤  │  │
│  │  │  • assign_role(role, stake)                           │  │  │
│  │  │  • revoke_role(account)                               │  │  │
│  │  │  • get_role(account) → Option<Role>                   │  │  │
│  │  │  • get_stake(account) → Option<Balance>               │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  │                                                              │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │           Stake Management Module                      │  │  │
│  │  ├────────────────────────────────────────────────────────┤  │  │
│  │  │  • increase_stake(amount)                             │  │  │
│  │  │  • unstake(amount)                                    │  │  │
│  │  │  • slash(account, amount)                             │  │  │
│  │  │  • Currency::reserve() integration                    │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  │                                                              │  │
│  │  ┌────────────────────────────────────────────────────────┐  │  │
│  │  │           Role Verification Module                     │  │  │
│  │  ├────────────────────────────────────────────────────────┤  │  │
│  │  │  • stake_requirement(role) → StakeRequirement         │  │  │
│  │  │  • verify_role_eligibility(account, role) → bool     │  │  │
│  │  │  • check_minimum_stake(amount, role) → bool          │  │  │
│  │  └────────────────────────────────────────────────────────┘  │  │
│  │                                                              │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              ↕                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │              Role-Specific Modules                           │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │                                                              │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │  │
│  │  │ Flare Nodes │  │  Validity   │  │ Decentralzd │         │  │
│  │  │   (Root)    │  │   Nodes     │  │  Directors  │         │  │
│  │  │             │  │   (PBCs)    │  │ (Governance)│         │  │
│  │  ├─────────────┤  ├─────────────┤  ├─────────────┤         │  │
│  │  │ • Attest    │  │ • Validate  │  │ • Govern    │         │  │
│  │  │   finality  │  │   PBC       │  │   network   │         │  │
│  │  │ • Aggregate │  │   blocks    │  │ • Emergency │         │  │
│  │  │   state     │  │ • Report    │  │   actions   │         │  │
│  │  │ • Coord.    │  │   validity  │  │ • Propose   │         │  │
│  │  │   consensus │  │ • Earn fees │  │   changes   │         │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘         │  │
│  │                                                              │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              ↕                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                     Storage Layer                            │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │  • Roles<AccountId → RoleRecord>                            │  │
│  │    - account: AccountId                                     │  │
│  │    - role: Role                                             │  │
│  │    - stake: Balance                                         │  │
│  │    - last_update: BlockNumber                               │  │
│  │    - active: bool                                           │  │
│  │                                                              │  │
│  │  • SlashingHistory<Vec<SlashRecord>>                        │  │
│  │  • RewardAccumulator<AccountId → Balance>                   │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                              ↕                                      │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    Events & Errors                           │  │
│  ├──────────────────────────────────────────────────────────────┤  │
│  │  Events:                                                     │  │
│  │  • RoleAssigned(account, role_u8)                           │  │
│  │  • RoleRevoked(account, role_u8)                            │  │
│  │  • StakeIncreased(account, amount)                          │  │
│  │  • StakeDecreased(account, amount)                          │  │
│  │  • StakeSlashed(account, amount)                            │  │
│  │                                                              │  │
│  │  Errors:                                                     │  │
│  │  • RoleAlreadyAssigned                                      │  │
│  │  • InsufficientStake                                        │  │
│  │  • NoActiveRole                                             │  │
│  │  • BondNotMature                                            │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘

                                ↕

┌─────────────────────────────────────────────────────────────────────┐
│                      External Integrations                          │
├─────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐             │
│  │     ASF      │  │  Governance  │  │   Currency   │             │
│  │  Consensus   │  │    Pallet    │  │    Pallet    │             │
│  └──────────────┘  └──────────────┘  └──────────────┘             │
│         │                  │                  │                     │
│         │                  │                  │                     │
│         ▼                  ▼                  ▼                     │
│  ┌────────────────────────────────────────────────────┐            │
│  │         Staking Interface Layer                    │            │
│  │  • Validator eligibility checks                    │            │
│  │  • Voting power calculation                        │            │
│  │  • Stake reservation/unreservation                 │            │
│  │  • Reward distribution coordination                │            │
│  │  • Slashing execution                              │            │
│  └────────────────────────────────────────────────────┘            │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### 1. Staking Types (`staking/types/`)

**Purpose**: Shared type definitions and interfaces for role-based staking.

#### Role Enumeration

```rust
/// Network roles in increasing order of responsibility and stake
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Role {
    /// No stake, no privileges (observers)
    CommonPeer = 0,

    /// Minimum stake (1+ ËTR), basic voting
    CommonStakePeer = 1,

    /// PBC validators (64+ ËTR)
    ValidityNode = 2,

    /// Root chain validators (64+ ËTR)
    FlareNode = 3,

    /// Governance directors (128+ ËTR)
    DecentralizedDirector = 4,
}

impl Role {
    /// Convert u8 to Role (for extrinsic compatibility)
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Role::CommonPeer),
            1 => Some(Role::CommonStakePeer),
            2 => Some(Role::ValidityNode),
            3 => Some(Role::FlareNode),
            4 => Some(Role::DecentralizedDirector),
            _ => None,
        }
    }

    /// Get minimum stake for this role
    pub fn min_stake(&self) -> u128 {
        match self {
            Role::CommonPeer => 0,
            Role::CommonStakePeer => defaults::MIN_STAKE_COMMON,      // 1 ËTR
            Role::ValidityNode => defaults::MIN_STAKE_VALIDITY,       // 64 ËTR
            Role::FlareNode => defaults::MIN_STAKE_FLARE,             // 64 ËTR
            Role::DecentralizedDirector => defaults::MIN_STAKE_DIRECTOR, // 128 ËTR
        }
    }
}
```

#### Role Record

```rust
/// On-chain record of an account's role and stake
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Encode,
    Decode,
    TypeInfo,
    MaxEncodedLen,
)]
pub struct RoleRecord<AccountId, Balance> {
    /// Account holding this role
    pub account: AccountId,

    /// Assigned role
    pub role: Role,

    /// Total staked amount (reserved from balance)
    pub stake: Balance,

    /// Last block when stake/role was modified
    pub last_update: u32,

    /// Whether role is currently active
    pub active: bool,
}
```

#### Stake Requirements

```rust
/// Stake requirement thresholds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StakeRequirement {
    None,           // 0 ËTR
    CommonStake,    // 1 ËTR
    Validity,       // 64 ËTR
    Director,       // 128 ËTR
}

impl StakeRequirement {
    pub fn amount(&self) -> u128 {
        match self {
            StakeRequirement::None => 0,
            StakeRequirement::CommonStake => defaults::MIN_STAKE_COMMON,
            StakeRequirement::Validity => defaults::MIN_STAKE_VALIDITY,
            StakeRequirement::Director => defaults::MIN_STAKE_DIRECTOR,
        }
    }
}
```

#### Role Interface Trait

```rust
/// Interface for querying role and stake information
pub trait RoleInterface<AccountId, Balance> {
    /// Get account's current role
    fn get_role(account: &AccountId) -> Option<Role>;

    /// Get account's staked amount
    fn get_stake(account: &AccountId) -> Option<Balance>;

    /// Get stake requirement for a role
    fn stake_requirement(role: &Role) -> StakeRequirement;
}
```

#### Default Constants

```rust
pub mod defaults {
    /// 1 ËTR in smallest unit (18 decimals)
    pub const MIN_STAKE_COMMON: u128 = 1_000_000_000_000_000_000_000;

    /// 64 ËTR (Validity/Flare Nodes)
    pub const MIN_STAKE_VALIDITY: u128 = 64_000_000_000_000_000_000_000;
    pub const MIN_STAKE_FLARE: u128 = 64_000_000_000_000_000_000_000;

    /// 128 ËTR (Decentralized Directors)
    pub const MIN_STAKE_DIRECTOR: u128 = 128_000_000_000_000_000_000_000;

    /// Unbonding period in blocks (7 days at 6s = 100,800 blocks)
    pub const UNBOND_PERIOD: u32 = 100_800;
}
```

---

### 2. Staking Pallet (`staking/pallet/`)

**Purpose**: Core on-chain logic for role assignment, stake management, and slashing.

#### Configuration

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    /// Runtime event type
    type RuntimeEvent: From<Event<Self>> + IsType<...>;

    /// Currency used for staking (ËTR)
    type Currency: ReservableCurrency<Self::AccountId>;

    /// Minimum unbond period (blocks before unstake allowed)
    #[pallet::constant]
    type UnbondPeriod: Get<u32>;
}
```

#### Storage

```rust
/// Map of account to role record
#[pallet::storage]
#[pallet::getter(fn role_of)]
pub type Roles<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    RoleRecord<T::AccountId, BalanceOf<T>>,
>;
```

#### Extrinsics

##### assign_role

```rust
/// Assign a network role to an account with required stake
#[pallet::call_index(0)]
#[pallet::weight(10_000)]
pub fn assign_role(
    origin: OriginFor<T>,
    role_u8: u8,
    stake: BalanceOf<T>,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Convert u8 to Role
    let role = Role::from_u8(role_u8)
        .ok_or(Error::<T>::InsufficientStake)?;

    // Check not already assigned
    ensure!(
        !Roles::<T>::contains_key(&who),
        Error::<T>::RoleAlreadyAssigned
    );

    // Verify minimum stake
    ensure!(!stake.is_zero(), Error::<T>::InsufficientStake);

    // Reserve stake from account balance
    T::Currency::reserve(&who, stake)?;

    // Create role record
    let record = RoleRecord {
        account: who.clone(),
        role,
        stake,
        last_update: current_block_number(),
        active: true,
    };

    // Store record
    Roles::<T>::insert(&who, &record);

    // Emit event
    Self::deposit_event(Event::<T>::RoleAssigned(who, role_u8));

    Ok(())
}
```

##### increase_stake

```rust
/// Increase reserved stake for an account
#[pallet::call_index(1)]
#[pallet::weight(5_000)]
pub fn increase_stake(
    origin: OriginFor<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    Roles::<T>::try_mutate(&who, |maybe_record| {
        let record = maybe_record.as_mut()
            .ok_or(Error::<T>::NoActiveRole)?;

        // Reserve additional amount
        T::Currency::reserve(&who, amount)?;

        // Update stake
        record.stake += amount;
        record.last_update = current_block_number();

        Self::deposit_event(Event::<T>::StakeIncreased(who.clone(), amount));

        Ok(())
    })
}
```

##### unstake

```rust
/// Unstake part or all of bonded amount (after unbond period)
#[pallet::call_index(2)]
#[pallet::weight(5_000)]
pub fn unstake(
    origin: OriginFor<T>,
    amount: BalanceOf<T>,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    Roles::<T>::try_mutate(&who, |maybe_record| {
        let record = maybe_record.as_mut()
            .ok_or(Error::<T>::NoActiveRole)?;

        // Check unbond period elapsed
        let current_block = current_block_number();
        ensure!(
            current_block >= record.last_update + T::UnbondPeriod::get(),
            Error::<T>::BondNotMature
        );

        // Unreserve stake
        T::Currency::unreserve(&who, amount);

        // Update stake
        record.stake -= amount;
        record.last_update = current_block;

        Self::deposit_event(Event::<T>::StakeDecreased(who.clone(), amount));

        Ok(())
    })
}
```

##### revoke_role

```rust
/// Revoke a role completely (governance/root only)
#[pallet::call_index(3)]
#[pallet::weight(10_000)]
pub fn revoke_role(
    origin: OriginFor<T>,
    account: T::AccountId,
) -> DispatchResult {
    ensure_root(origin)?;

    if let Some(record) = Roles::<T>::take(&account) {
        // Unreserve all stake
        T::Currency::unreserve(&account, record.stake);

        let role_u8 = record.role as u8;
        Self::deposit_event(Event::<T>::RoleRevoked(account, role_u8));
    }

    Ok(())
}
```

##### slash

```rust
/// Slash a misbehaving validator by an amount
#[pallet::call_index(4)]
#[pallet::weight(10_000)]
pub fn slash(
    origin: OriginFor<T>,
    account: T::AccountId,
    amount: BalanceOf<T>,
) -> DispatchResult {
    ensure_root(origin)?;

    Roles::<T>::try_mutate(&account, |maybe_record| {
        let record = maybe_record.as_mut()
            .ok_or(Error::<T>::NoActiveRole)?;

        // Calculate actual slash amount (capped at stake)
        let slash_amount = amount.min(record.stake);

        // Execute slash
        T::Currency::slash_reserved(&account, slash_amount);

        // Update record
        record.stake -= slash_amount;
        record.active = record.stake > BalanceOf::<T>::zero();
        record.last_update = current_block_number();

        Self::deposit_event(Event::<T>::StakeSlashed(account.clone(), slash_amount));

        Ok(())
    })
}
```

#### RoleInterface Implementation

```rust
impl<T: Config> RoleInterface<T::AccountId, BalanceOf<T>> for Pallet<T> {
    fn get_role(account: &T::AccountId) -> Option<Role> {
        Roles::<T>::get(account).map(|r| r.role)
    }

    fn get_stake(account: &T::AccountId) -> Option<BalanceOf<T>> {
        Roles::<T>::get(account).map(|r| r.stake)
    }

    fn stake_requirement(role: &Role) -> StakeRequirement {
        match role {
            Role::DecentralizedDirector => StakeRequirement::Director,
            Role::ValidityNode | Role::FlareNode => StakeRequirement::Validity,
            Role::CommonStakePeer => StakeRequirement::CommonStake,
            Role::CommonPeer => StakeRequirement::None,
        }
    }
}
```

---

## Peer Type Hierarchy

### Role Comparison

| Role | Stake | Consensus | Governance | PBC Validation | Root Chain | Rewards |
|------|-------|-----------|------------|----------------|------------|---------|
| **Common Peer** | 0 ËTR | No | No | No | No | No |
| **Staking Common** | 1+ ËTR | No | Vote only | No | No | Minimal |
| **Validity Node** | 64+ ËTR | Yes (PBCs) | Vote + propose | Yes | No | Medium |
| **Flare Node** | 64+ ËTR | Yes (Root) | Vote + propose | No | Yes | High |
| **Director** | 128+ ËTR | Yes (All) | Full powers | Yes | Yes | Highest |

### Tier Progression

```text
┌─────────────────────────────────────────────────────────┐
│                    PEER TIER SYSTEM                     │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Tier 0: Common Peer                                   │
│  ┌───────────────────────────────────────────────┐    │
│  │ Stake: None                                   │    │
│  │ Capabilities: Read-only network access        │    │
│  │ Use Case: Observers, light clients           │    │
│  └───────────────────────────────────────────────┘    │
│                     ↓ +1 ËTR                           │
│  Tier 1: Staking Common Peer                          │
│  ┌───────────────────────────────────────────────┐    │
│  │ Stake: 1+ ËTR                                 │    │
│  │ Capabilities: Voting on proposals             │    │
│  │ Use Case: Community members, small holders    │    │
│  └───────────────────────────────────────────────┘    │
│                     ↓ +63 ËTR (64 total)              │
│  Tier 2: Validity Node (PBC Validator)                │
│  ┌───────────────────────────────────────────────┐    │
│  │ Stake: 64+ ËTR                                │    │
│  │ Capabilities:                                 │    │
│  │ • Validate PBC blocks                         │    │
│  │ • Issue validity certificates                 │    │
│  │ • Participate in ASF consensus (PBCs)         │    │
│  │ • Earn block rewards + fees                   │    │
│  │ • Vote + create proposals                     │    │
│  └───────────────────────────────────────────────┘    │
│                     ↓ (Parallel to Tier 2)            │
│  Tier 2: Flare Node (Root Validator)                  │
│  ┌───────────────────────────────────────────────┐    │
│  │ Stake: 64+ ËTR                                │    │
│  │ Capabilities:                                 │    │
│  │ • Validate FlareChain blocks                  │    │
│  │ • Aggregate PBC state roots                   │    │
│  │ • Participate in ASF consensus (Root)         │    │
│  │ • Coordinate cross-chain messages             │    │
│  │ • Earn block rewards + fees                   │    │
│  │ • Vote + create proposals                     │    │
│  └───────────────────────────────────────────────┘    │
│                     ↓ +64 ËTR (128 total)             │
│  Tier 3: Decentralized Director                       │
│  ┌───────────────────────────────────────────────┐    │
│  │ Stake: 128+ ËTR                               │    │
│  │ Capabilities:                                 │    │
│  │ • All Validity Node + Flare Node powers       │    │
│  │ • Emergency governance actions                │    │
│  │ • Fast-track proposals                        │    │
│  │ • Treasury management approval                │    │
│  │ • Multi-sig consensus changes                 │    │
│  │ • Highest voting power multiplier (3x)        │    │
│  │ • Elected annually on Consensus Day           │    │
│  └───────────────────────────────────────────────┘    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Role Transition

```rust
/// Example role progression for a single account

// Start as Common Peer (no stake)
let account = Alice;
assert_eq!(Staking::get_role(&account), None);

// Stake 10 ËTR → become Staking Common Peer
Staking::assign_role(account, Role::CommonStakePeer, 10_ETR)?;
assert_eq!(Staking::get_role(&account), Some(Role::CommonStakePeer));

// Increase stake to 70 ËTR → eligible for Validity Node
Staking::increase_stake(account, 60_ETR)?;
// NOTE: Must call assign_role again to upgrade
Staking::revoke_role(Root, account)?;  // Remove old role
Staking::assign_role(account, Role::ValidityNode, 70_ETR)?;
assert_eq!(Staking::get_role(&account), Some(Role::ValidityNode));

// Increase stake to 150 ËTR → eligible for Director
// (Still requires election on Consensus Day)
Staking::increase_stake(account, 80_ETR)?;
Staking::revoke_role(Root, account)?;
Staking::assign_role(account, Role::DecentralizedDirector, 150_ETR)?;
assert_eq!(Staking::get_role(&account), Some(Role::DecentralizedDirector));
```

---

## Staking Mechanism

### Stake Reservation

Ëtrid uses Substrate's `ReservableCurrency` trait to lock stake:

```rust
// Reserve stake (locks from transferable balance)
T::Currency::reserve(&who, amount)?;

// Unreserve stake (unlocks back to transferable)
T::Currency::unreserve(&who, amount);

// Slash reserved stake (permanently removes)
T::Currency::slash_reserved(&who, amount);
```

**Properties**:
- Reserved stake cannot be transferred
- Reserved stake counts for voting power
- Reserved stake earns staking rewards
- Unreserving requires unbond period (7 days)

### Unbonding Period

```rust
/// Unbonding state machine

┌──────────────┐
│ Stake Active │ ← Account has reserved stake
└──────┬───────┘
       │
       │ Call unstake(amount)
       ▼
┌──────────────┐
│  Unbonding   │ ← 7-day waiting period
│   (7 days)   │   Stake still reserved
└──────┬───────┘   Cannot re-stake
       │
       │ After UnbondPeriod blocks
       ▼
┌──────────────┐
│  Unstaked    │ ← Stake unreserved
│ (Transferable)  ← Can withdraw or re-stake
└──────────────┘
```

**Rationale**:
- Prevents rapid stake shuffling for vote manipulation
- Gives network time to respond to validator exits
- Aligns incentives with long-term network health

### Minimum Stake Enforcement

```rust
fn verify_minimum_stake(
    role: Role,
    stake: Balance,
) -> Result<(), Error> {
    let min = role.min_stake();

    if stake < min {
        return Err(Error::InsufficientStake {
            required: min,
            provided: stake,
        });
    }

    Ok(())
}
```

**Stake Thresholds** (as of 2025-10-20):
- Common Peer: **0 ËTR** (no stake)
- Staking Common: **1 ËTR** = 1,000,000,000,000,000,000,000 wei
- Validity Node: **64 ËTR** = 64,000,000,000,000,000,000,000 wei
- Flare Node: **64 ËTR** = 64,000,000,000,000,000,000,000 wei
- Director: **128 ËTR** = 128,000,000,000,000,000,000,000 wei

---

## Role Assignment & Lifecycle

### Assignment Process

```text
1. User Action: Call assign_role(role, stake)
   ↓
2. Validation:
   ├─ Role not already assigned? ✓
   ├─ Stake amount sufficient? ✓
   ├─ Account has free balance? ✓
   └─ Role enum valid? ✓
   ↓
3. Stake Reservation:
   Currency::reserve(&account, stake)
   ↓
4. Record Creation:
   RoleRecord {
     account,
     role,
     stake,
     last_update: current_block,
     active: true,
   }
   ↓
5. Storage Update:
   Roles<account> = record
   ↓
6. Event Emission:
   RoleAssigned(account, role_u8)
   ↓
7. Integration Notifications:
   ├─ ASF: Check if eligible for committee
   ├─ Governance: Update voting power
   └─ Rewards: Start reward accumulation
```

### Revocation Process

```text
1. Trigger:
   ├─ Governance call: revoke_role(account)
   ├─ Slashing reduces stake below minimum
   └─ Validator self-exits
   ↓
2. Record Removal:
   let record = Roles::take(account)
   ↓
3. Stake Unreservation:
   Currency::unreserve(&account, record.stake)
   ↓
4. Cleanup:
   ├─ Remove from validator sets
   ├─ Stop reward accumulation
   └─ Update governance voting power
   ↓
5. Event Emission:
   RoleRevoked(account, role_u8)
```

### Automatic Deactivation

```rust
/// Automatically deactivate role if stake falls below minimum

fn check_role_validity(account: &AccountId) -> DispatchResult {
    if let Some(mut record) = Roles::<T>::get(account) {
        let min_stake = record.role.min_stake();

        if record.stake < min_stake && record.active {
            // Deactivate but don't remove (allows recovery)
            record.active = false;
            Roles::<T>::insert(account, record);

            Self::deposit_event(Event::RoleDeactivated(account.clone()));
        }
    }

    Ok(())
}
```

---

## Validator Operations

### 1. Validity Nodes (PBC Validators)

**Purpose**: Validate blocks on Partition Burst Chains (12 side chains).

#### Core Responsibilities

```rust
pub trait ValidityNodeApi<AccountId> {
    /// Report validation result for a PBC block
    fn report_validation(
        who: &AccountId,
        chain_id: u32,
        block_number: u64,
        valid: bool,
    ) -> DispatchResult;

    /// Get assigned PBC for this validator
    fn assigned_chain(who: &AccountId) -> Option<u32>;

    /// Check if validator is active on chain
    fn is_active_on_chain(who: &AccountId, chain_id: u32) -> bool;
}
```

#### Validation Flow

```text
1. PBC block produced (BTC-PBC, ETH-PBC, etc.)
   ↓
2. Validity Node receives block via P2P
   ↓
3. Validation checks:
   ├─ Block header valid?
   ├─ Parent exists?
   ├─ Transactions valid?
   ├─ State transition correct?
   └─ Signature correct?
   ↓
4. Report validation:
   ValidityNode::report_validation(
     validator,
     chain_id,
     block_number,
     valid: true/false
   )
   ↓
5. If valid:
   ├─ Issue validity certificate
   ├─ Broadcast to network
   └─ Earn reward
   ↓
6. If invalid:
   ├─ Report to other validators
   ├─ Trigger investigation
   └─ Potential proposer slashing
```

#### PBC Assignment

```rust
/// Assign validators to PBCs based on stake weight

fn assign_validators_to_pbcs(
    validators: Vec<AccountId>,
) -> BTreeMap<u32, Vec<AccountId>> {
    let mut assignments = BTreeMap::new();

    // 12 PBC chains
    for chain_id in 0..12 {
        let chain_validators = validators
            .iter()
            .filter(|v| {
                // Validators rotate across chains based on stake hash
                let hash = hash(&(v, chain_id));
                hash % 12 == chain_id
            })
            .cloned()
            .collect();

        assignments.insert(chain_id, chain_validators);
    }

    assignments
}
```

---

### 2. Flare Nodes (Root Chain Validators)

**Purpose**: Validate FlareChain (root chain) and aggregate PBC state.

#### Core Responsibilities

```rust
pub trait FlareNodeApi<AccountId> {
    /// Submit state attestation from PBCs
    fn submit_attestation(
        who: &AccountId,
        state_root: [u8; 32],
        signature: [u8; 64],
    ) -> DispatchResult;

    /// Coordinate cross-chain message
    fn relay_cross_chain_message(
        who: &AccountId,
        from_chain: u32,
        to_chain: u32,
        message: Vec<u8>,
    ) -> DispatchResult;

    /// Check if part of PPFA committee
    fn is_committee_member(who: &AccountId) -> bool;
}
```

#### State Aggregation

```text
1. Collect PBC state roots:
   ┌─────────┐  ┌─────────┐  ┌─────────┐
   │ BTC-PBC │  │ ETH-PBC │  │ SOL-PBC │  ... (12 chains)
   │ Root: R₁│  │ Root: R₂│  │ Root: R₃│
   └────┬────┘  └────┬────┘  └────┬────┘
        │            │            │
        └────────────┼────────────┘
                     │
                     ▼
   ┌─────────────────────────────────┐
   │     Flare Node Aggregates       │
   │  Merkle Root = MR(R₁, R₂, ...) │
   └─────────────────────────────────┘
                     │
                     ▼
   ┌─────────────────────────────────┐
   │      FlareChain Block           │
   │  Header includes aggregated MR  │
   │  Proves all PBC states          │
   └─────────────────────────────────┘
```

#### Cross-Chain Coordination

```rust
/// Relay message from one PBC to another

fn relay_message(
    flare_node: AccountId,
    from_chain: u32,
    to_chain: u32,
    message: Vec<u8>,
) -> DispatchResult {
    // 1. Verify flare node is authorized
    ensure!(
        FlareNode::is_committee_member(&flare_node),
        Error::NotAuthorized
    );

    // 2. Verify message signature from source chain
    let signature = extract_signature(&message)?;
    verify_signature(from_chain, &message, &signature)?;

    // 3. Queue message for destination chain
    CrossChainQueue::insert((to_chain, message.hash()), message);

    // 4. Emit event
    emit(CrossChainMessageRelayed {
        from: from_chain,
        to: to_chain,
        hash: message.hash(),
    });

    Ok(())
}
```

---

### 3. Decentralized Directors (Governance Validators)

**Purpose**: Elected governors overseeing the Foundation DAO.

#### Core Responsibilities

```rust
pub trait DirectorApi<AccountId> {
    /// Register as director candidate
    fn register_director(who: &AccountId) -> DispatchResult;

    /// End director term
    fn end_term(who: &AccountId) -> DispatchResult;

    /// Execute emergency action
    fn emergency_action(
        who: &AccountId,
        action: EmergencyAction,
    ) -> DispatchResult;

    /// Approve treasury spend
    fn approve_treasury_spend(
        who: &AccountId,
        proposal_id: u32,
    ) -> DispatchResult;
}
```

#### Election Process (Consensus Day)

```text
Annual Election Timeline:

T-30 days: Nomination Period
├─ Directors with 128+ ËTR register
├─ Submit platform/manifesto
└─ Campaign to community

T-7 days: Voting Period
├─ All stakeholders vote
├─ Stake-weighted voting
└─ Can change vote until T-0

T-0: Election Day (Consensus Day)
├─ Voting closes midnight UTC
├─ Votes tallied
├─ Top 21 candidates elected
└─ Directors seated immediately

T+0 to T+365: Director Term
├─ Govern network
├─ Emergency powers
├─ Treasury oversight
└─ Strategic planning

T+365: Next Election
└─ Cycle repeats
```

#### Director Powers

```rust
pub enum DirectorPower {
    /// Fast-track critical proposals (24h voting)
    FastTrack(ProposalId),

    /// Emergency network halt
    EmergencyHalt,

    /// Emergency parameter change (requires 2/3 directors)
    EmergencyParameterChange {
        param: Parameter,
        value: u64,
    },

    /// Treasury allocation approval
    ApproveTreasurySpend {
        recipient: AccountId,
        amount: Balance,
    },

    /// Validator emergency removal
    EmergencySlash {
        validator: AccountId,
        amount: Balance,
        reason: Vec<u8>,
    },
}
```

---

## Rewards & Slashing

### Reward Distribution

```rust
/// Reward types by role

pub enum RewardType {
    /// Block production reward (proposer)
    BlockProduction { amount: Balance },

    /// Certificate issuance reward
    CertificateIssuance { count: u32 },

    /// Validation participation reward
    ValidationParticipation { blocks: u32 },

    /// Uptime reward (100% uptime in epoch)
    UptimeBonus { epoch: u32 },

    /// Governance participation reward
    GovernanceParticipation { proposals_voted: u32 },
}
```

#### Reward Calculation

```rust
fn calculate_epoch_rewards(
    validator: &AccountId,
    role: &Role,
    performance: &PerformanceMetrics,
) -> Balance {
    // Base reward by role
    let base = match role {
        Role::ValidityNode => 10_ETR,      // Per epoch
        Role::FlareNode => 15_ETR,         // Per epoch
        Role::DecentralizedDirector => 20_ETR, // Per epoch
        _ => 0,
    };

    // Performance multiplier (0.0 - 2.0)
    let uptime_multiplier = performance.uptime / 100.0;
    let block_multiplier = performance.blocks_produced / expected_blocks;
    let cert_multiplier = performance.certs_issued / expected_certs;

    let multiplier = (uptime_multiplier + block_multiplier + cert_multiplier) / 3.0;

    // Final reward
    (base as f64 * multiplier.min(2.0)) as Balance
}
```

**Example**:
- Validity Node with 95% uptime, 100% blocks, 90% certificates:
  - Base: 10 ËTR
  - Multiplier: (0.95 + 1.0 + 0.9) / 3 = 0.95
  - **Reward: 9.5 ËTR**

- Director with 100% uptime, 100% blocks, 100% certificates:
  - Base: 20 ËTR
  - Multiplier: (1.0 + 1.0 + 1.0) / 3 = 1.0
  - **Reward: 20 ËTR**

### Slashing Conditions

```rust
pub enum SlashableOffense {
    /// Double voting in consensus
    DoubleVoting {
        block_a: Hash,
        block_b: Hash,
        phase: ConsensusPhase,
        severity: SlashSeverity,
    },

    /// Invalid block proposed
    InvalidBlock {
        block_hash: Hash,
        reason: InvalidityReason,
        severity: SlashSeverity,
    },

    /// Prolonged downtime
    Downtime {
        missed_blocks: u32,
        threshold: u32,
        severity: SlashSeverity,
    },

    /// Invalid certificate issued
    InvalidCertificate {
        cert_hash: Hash,
        reason: InvalidityReason,
        severity: SlashSeverity,
    },

    /// Equivocation (conflicting messages)
    Equivocation {
        message_a: Hash,
        message_b: Hash,
        severity: SlashSeverity,
    },
}

pub enum SlashSeverity {
    Minor,    // 5% stake slashed
    Moderate, // 25% stake slashed
    Major,    // 50% stake slashed
    Critical, // 100% stake slashed + permanent ban
}
```

#### Slashing Process

```text
1. Offense Detected
   ├─ Automatic detection (double vote, downtime)
   └─ Manual report (invalid block, etc.)
   ↓
2. Evidence Collection
   ├─ Gather cryptographic proofs
   ├─ Validator signatures
   └─ Block/transaction references
   ↓
3. Verification
   ├─ Multiple validators confirm
   ├─ Reach BFT threshold (2/3+)
   └─ No false positives
   ↓
4. Slashing Execution
   ├─ Calculate slash amount
   ├─ Currency::slash_reserved(validator, amount)
   ├─ Update role record
   └─ Deactivate if stake < minimum
   ↓
5. Post-Slash
   ├─ Distribute slashed funds (treasury + reporters)
   ├─ Emit SlashEvent
   ├─ Update validator reputation
   └─ Notify governance
```

---

## Integration Points

### 1. ASF Consensus

```rust
/// ASF queries staking for validator info

trait StakingInterface {
    /// Check if account can be in PPFA committee
    fn is_eligible_for_committee(who: &AccountId) -> bool {
        if let Some(role) = Self::get_role(who) {
            role == Role::ValidityNode ||
            role == Role::FlareNode ||
            role == Role::DecentralizedDirector
        } else {
            false
        }
    }

    /// Get voting power for consensus
    fn get_consensus_weight(who: &AccountId) -> u64 {
        Self::get_stake(who)
            .map(|s| s.saturated_into())
            .unwrap_or(0)
    }
}
```

### 2. Governance

```rust
/// Governance uses staking for voting power

fn calculate_voting_power(
    who: &AccountId,
) -> VotingPower {
    let stake = Staking::get_stake(who).unwrap_or(0);
    let role = Staking::get_role(who);

    // Base power = stake
    let base = stake;

    // Role multiplier
    let multiplier = match role {
        Some(Role::DecentralizedDirector) => 3,
        Some(Role::ValidityNode | Role::FlareNode) => 2,
        Some(Role::CommonStakePeer) => 1,
        _ => 0,
    };

    base * multiplier
}
```

### 3. Treasury

```rust
/// Rewards come from treasury

trait TreasuryInterface {
    /// Pay validator reward
    fn pay_reward(
        recipient: AccountId,
        amount: Balance,
    ) -> DispatchResult {
        Treasury::transfer(recipient, amount)?;
        Ok(())
    }

    /// Receive slashed funds
    fn receive_slashed(
        amount: Balance,
    ) -> DispatchResult {
        Treasury::deposit(amount)?;
        Ok(())
    }
}
```

---

## Security Considerations

### Stake Manipulation

**Attack**: Rapidly stake/unstake to manipulate voting.

**Defense**:
- 7-day unbonding period prevents rapid cycling
- Voting power snapshot at proposal creation
- Cannot vote with unbonding stake

### Sybil Resistance

**Attack**: Create many accounts with minimal stake.

**Defense**:
- Minimum stake requirements (1 ËTR)
- Quadratic voting power (sqrt of stake)
- Committee selection favors large stakes

### Economic Security

**Formula**: Total slashable stake must exceed attack profit.

```
Attack Cost = Minimum Stake to Control 1/3 Committee
            = (Total Committee Stake / 3) + 1

For 21-validator committee with avg 100 ËTR each:
Total Stake = 2100 ËTR
Attack Cost = 700 ËTR minimum
```

**Assumption**: Attacking costs more than any benefit from attack.

### Role Downgrade Prevention

**Attack**: Malicious director downgrades to avoid slashing.

**Defense**:
- Slashing applies to reserved stake (cannot unreserve during investigation)
- Governance can force role revocation
- Unbonding period allows time for slashing execution

---

## Future Enhancements

### Planned Features

1. **Delegation**
   - Stake pools delegate to operators
   - Operators earn commission
   - Delegators share rewards

2. **Auto-Compounding**
   - Rewards automatically re-staked
   - Compound interest on stake
   - Optional for validators

3. **Reputation System**
   - Track validator performance
   - Reputation affects committee selection
   - Reputation decay for downtime

4. **Insurance Pool**
   - Validators contribute to insurance
   - Covers slashing losses for honest validators
   - Protects against bugs

5. **Stake Liquidity**
   - Liquid staking derivatives (sËTR)
   - Trade staked positions
   - Maintain validator role while liquid

6. **Multi-Role Support**
   - Single account holds multiple roles
   - Separate stakes per role
   - Independent slashing

---

## Conclusion

The Peer Roles & Staking system provides the foundation for Ëtrid's security and governance. By tying network participation to economic stake, the system creates strong incentives for honest behavior while enabling flexible, meritocratic advancement through the role hierarchy.

**Key Achievements**:
- **Security**: Stake-based BFT ensures economic alignment
- **Flexibility**: Multiple tiers accommodate diverse participants
- **Meritocracy**: Performance and stake determine influence
- **Transparency**: All roles and stakes publicly verifiable
- **Scalability**: Supports 100+ validators across 12+ chains

---

**References:**
- Ivory Papers: Peer Type Specification
- Substrate Staking: ReservableCurrency Implementation
- Polkadot Staking: NPoS Design Patterns
- Cosmos Staking: Validator Economics

**Related Components:**
- `09-consensus`: Validator selection and committee management
- `10-foundation`: Governance voting and director elections
- `03-flarechain`: Root chain validator operations
- `05-multichain/partition-burst-chains`: PBC validator operations
