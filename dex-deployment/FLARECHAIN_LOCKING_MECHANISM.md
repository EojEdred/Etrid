# 🔒 FlareChain Locking Mechanism - Honest 1:1 Backing

**Purpose:** Lock equivalent ÉTR on FlareChain when minting on DEX chains
**Goal:** Maintain honest 1:1 backing between native ÉTR and wrapped ÉTR
**Status:** Implementation guide for Substrate pallet

---

## 🎯 Why Lock on FlareChain?

### The Problem: Double Spending Supply

**Scenario without locking:**
```
FlareChain: 1,000,000,000 ÉTR (1 billion)
+ BSC:      100,000 ÉTR (minted out of thin air)
+ Polygon:  100,000 ÉTR (minted out of thin air)
+ Solana:   100,000 ÉTR (minted out of thin air)
────────────────────────────────────────────
TOTAL:      1,000,300,000 ÉTR (1.0003 billion)

Problem: You've inflated supply by 300K!
```

**Scenario WITH locking:**
```
FlareChain:
  - Circulating:   999,700,000 ÉTR
  - Locked:        300,000 ÉTR (for DEX backing)
  ──────────────────────────────────
  Total:           1,000,000,000 ÉTR ✅

DEX chains:
  - BSC:           100,000 ÉTR (backed by locked ÉTR)
  - Polygon:       100,000 ÉTR (backed by locked ÉTR)
  - Solana:        100,000 ÉTR (backed by locked ÉTR)
  ──────────────────────────────────
  Total:           300,000 ÉTR ✅

Real total supply: 1 billion ÉTR (honest!)
```

### The Solution: Lock-and-Mint

**When you mint 100K ÉTR on Polygon:**
1. Lock 100K ÉTR from Community LP Pool on FlareChain
2. Mint 100K ÉTR on Polygon
3. Now it's backed 1:1
4. Total supply unchanged (1B)

**When you burn 100K ÉTR on Polygon:**
1. Burn 100K ÉTR on Polygon
2. Unlock 100K ÉTR on FlareChain
3. Backing maintained
4. Can be re-minted elsewhere

---

## 📋 Implementation Architecture

### Components Needed:

```
┌─────────────────────────────────────────────────────────────┐
│ FLARECHAIN (Substrate)                                      │
│                                                             │
│  Pallet: pallet-dex-lock                                    │
│  ┌───────────────────────────────────────────────────────┐ │
│  │                                                         │ │
│  │  Storage:                                               │ │
│  │  • TotalLocked: Balance (how much locked total)        │ │
│  │  • LockedForChain: Map<ChainId, Balance>              │ │
│  │  • LockEvents: Vec<LockEvent>                          │ │
│  │                                                         │ │
│  │  Functions:                                             │ │
│  │  • lock_for_dex(chain_id, amount)                      │ │
│  │  • unlock_from_dex(chain_id, amount)                   │ │
│  │  • query_locked(chain_id)                              │ │
│  │                                                         │ │
│  │  Permissions:                                           │ │
│  │  • Only Foundation multisig can lock/unlock            │ │
│  │  • 6-of-9 signatures required                          │ │
│  │                                                         │ │
│  └───────────────────────────────────────────────────────┘ │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ Lock/Unlock operations
                            │
┌─────────────────────────────────────────────────────────────┐
│ DEX CHAINS (BSC, Polygon, Solana)                           │
│                                                             │
│  Smart Contracts: EtridBSC, EtridPoly, etc.                │
│  • Mint when FlareChain locks                               │
│  • Burn when need to unlock on FlareChain                  │
│  • Owner: Foundation multisig                               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🛠️ Implementation: Substrate Pallet

### Step 1: Create `pallet-dex-lock`

**File:** `/Users/macbook/Desktop/etrid/pallets/dex-lock/src/lib.rs`

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, LockableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, CheckedSub};
    use sp_std::vec::Vec;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Chain identifier for DEX chains
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub enum ChainId {
        BSC,           // Binance Smart Chain
        Ethereum,      // Ethereum mainnet
        Polygon,       // Polygon PoS
        Solana,        // Solana
        Arbitrum,      // Arbitrum L2
        Avalanche,     // Avalanche C-Chain
        Base,          // Base L2
    }

    /// Lock event record
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    pub struct LockEvent<AccountId, Balance, BlockNumber> {
        pub chain_id: ChainId,
        pub amount: Balance,
        pub locker: AccountId,
        pub block_number: BlockNumber,
        pub timestamp: u64,
        pub target_address: Vec<u8>,  // Address on target chain
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: LockableCurrency<Self::AccountId>;

        /// Foundation multisig account that can lock/unlock
        type FoundationOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Maximum amount that can be locked
        #[pallet::constant]
        type MaxLockAmount: Get<BalanceOf<Self>>;
    }

    /// Total amount locked across all chains
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// Amount locked for each specific chain
    #[pallet::storage]
    #[pallet::getter(fn locked_for_chain)]
    pub type LockedForChain<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ChainId,
        BalanceOf<T>,
        ValueQuery,
    >;

    /// History of lock events
    #[pallet::storage]
    #[pallet::getter(fn lock_events)]
    pub type LockEvents<T: Config> = StorageValue<
        _,
        Vec<LockEvent<T::AccountId, BalanceOf<T>, BlockNumberFor<T>>>,
        ValueQuery,
    >;

    /// Account that holds all locked funds (Community LP Pool or Treasury)
    #[pallet::storage]
    #[pallet::getter(fn lock_account)]
    pub type LockAccount<T: Config> = StorageValue<_, T::AccountId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// ETR locked for DEX deployment [chain_id, amount, target_address]
        ETRLocked { chain_id: ChainId, amount: BalanceOf<T>, target_address: Vec<u8> },

        /// ETR unlocked from DEX [chain_id, amount]
        ETRUnlocked { chain_id: ChainId, amount: BalanceOf<T> },

        /// Lock account set [account]
        LockAccountSet { account: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Lock account not set
        LockAccountNotSet,

        /// Insufficient balance to lock
        InsufficientBalance,

        /// Amount exceeds maximum
        ExceedsMaxLockAmount,

        /// Insufficient locked amount to unlock
        InsufficientLockedAmount,

        /// Arithmetic overflow
        ArithmeticOverflow,

        /// Arithmetic underflow
        ArithmeticUnderflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set the account that holds locked funds
        ///
        /// Typically: Community LP Pool or Foundation Treasury
        ///
        /// Requires: Foundation multisig (6-of-9)
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn set_lock_account(
            origin: OriginFor<T>,
            account: T::AccountId,
        ) -> DispatchResult {
            T::FoundationOrigin::ensure_origin(origin)?;

            LockAccount::<T>::put(account.clone());
            Self::deposit_event(Event::LockAccountSet { account });

            Ok(())
        }

        /// Lock ETR for DEX deployment on another chain
        ///
        /// This locks ÉTR on FlareChain to back wrapped ÉTR on DEX chains
        ///
        /// Arguments:
        /// - `chain_id`: Which chain (BSC, Polygon, etc.)
        /// - `amount`: How much ÉTR to lock
        /// - `target_address`: Address on target chain (for tracking)
        ///
        /// Requires: Foundation multisig (6-of-9)
        #[pallet::call_index(1)]
        #[pallet::weight(100_000)]
        pub fn lock_for_dex(
            origin: OriginFor<T>,
            chain_id: ChainId,
            amount: BalanceOf<T>,
            target_address: Vec<u8>,
        ) -> DispatchResult {
            T::FoundationOrigin::ensure_origin(origin)?;

            let lock_account = LockAccount::<T>::get().ok_or(Error::<T>::LockAccountNotSet)?;

            // Check balance
            let balance = T::Currency::free_balance(&lock_account);
            ensure!(balance >= amount, Error::<T>::InsufficientBalance);

            // Check max amount
            ensure!(amount <= T::MaxLockAmount::get(), Error::<T>::ExceedsMaxLockAmount);

            // Update storage
            let current_total = TotalLocked::<T>::get();
            let new_total = current_total
                .checked_add(&amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;
            TotalLocked::<T>::put(new_total);

            let current_chain = LockedForChain::<T>::get(&chain_id);
            let new_chain_amount = current_chain
                .checked_add(&amount)
                .ok_or(Error::<T>::ArithmeticOverflow)?;
            LockedForChain::<T>::insert(&chain_id, new_chain_amount);

            // Record event
            let lock_event = LockEvent {
                chain_id: chain_id.clone(),
                amount,
                locker: lock_account.clone(),
                block_number: <frame_system::Pallet<T>>::block_number(),
                timestamp: Self::current_timestamp(),
                target_address: target_address.clone(),
            };

            LockEvents::<T>::mutate(|events| {
                events.push(lock_event);
            });

            // Lock the currency (prevents spending)
            T::Currency::set_lock(
                *b"dexlock ",
                &lock_account,
                new_total,
                frame_support::traits::WithdrawReasons::all(),
            );

            Self::deposit_event(Event::ETRLocked {
                chain_id,
                amount,
                target_address
            });

            Ok(())
        }

        /// Unlock ETR from DEX (when tokens burned on DEX chain)
        ///
        /// This unlocks ÉTR on FlareChain when wrapped ÉTR is burned on DEX chains
        ///
        /// Arguments:
        /// - `chain_id`: Which chain
        /// - `amount`: How much to unlock
        ///
        /// Requires: Foundation multisig (6-of-9)
        #[pallet::call_index(2)]
        #[pallet::weight(100_000)]
        pub fn unlock_from_dex(
            origin: OriginFor<T>,
            chain_id: ChainId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            T::FoundationOrigin::ensure_origin(origin)?;

            let lock_account = LockAccount::<T>::get().ok_or(Error::<T>::LockAccountNotSet)?;

            // Check locked amount
            let current_chain = LockedForChain::<T>::get(&chain_id);
            ensure!(current_chain >= amount, Error::<T>::InsufficientLockedAmount);

            let current_total = TotalLocked::<T>::get();
            ensure!(current_total >= amount, Error::<T>::InsufficientLockedAmount);

            // Update storage
            let new_total = current_total
                .checked_sub(&amount)
                .ok_or(Error::<T>::ArithmeticUnderflow)?;
            TotalLocked::<T>::put(new_total);

            let new_chain_amount = current_chain
                .checked_sub(&amount)
                .ok_or(Error::<T>::ArithmeticUnderflow)?;
            LockedForChain::<T>::insert(&chain_id, new_chain_amount);

            // Update lock
            if new_total.is_zero() {
                T::Currency::remove_lock(*b"dexlock ", &lock_account);
            } else {
                T::Currency::set_lock(
                    *b"dexlock ",
                    &lock_account,
                    new_total,
                    frame_support::traits::WithdrawReasons::all(),
                );
            }

            Self::deposit_event(Event::ETRUnlocked { chain_id, amount });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> u64 {
            // Get current timestamp from pallet_timestamp
            <pallet_timestamp::Pallet<T>>::get().saturated_into::<u64>()
        }
    }
}
```

---

## 📋 How to Use the Locking Mechanism

### Scenario 1: Initial DEX Deployment

**You're deploying 100K ÉTR to Polygon:**

```bash
# Step 1: Lock on FlareChain (via multisig)
# Foundation submits transaction with 6-of-9 signatures

lock_for_dex(
  chain_id: ChainId::Polygon,
  amount: 100_000 * 10^5,  // 100K ÉTR (5 decimals)
  target_address: 0x742d35Cc6634C0532925a3b844Bc454e4438f44e
)

# This locks 100K ÉTR from Community LP Pool
# Emits event: ETRLocked

# Step 2: Deploy to Polygon
cd dex-deployment/polygon
npm run deploy:mainnet

# Mints 100K ÉTR on Polygon

# Step 3: Verify backing
# Query FlareChain:
curl -X POST -H "Content-Type: application/json" \
  --data '{"method":"dexLock_lockedForChain","params":["Polygon"]}' \
  http://localhost:9933

# Should return: 100,000 ÉTR locked

# Result:
# ✅ 100K ÉTR locked on FlareChain
# ✅ 100K ÉTR minted on Polygon
# ✅ 1:1 backing maintained
# ✅ Total supply: 1B ÉTR (unchanged)
```

### Scenario 2: Adding More Liquidity

**You want to add 500K more ÉTR to Polygon:**

```bash
# Step 1: Lock more on FlareChain
lock_for_dex(
  chain_id: ChainId::Polygon,
  amount: 500_000 * 10^5,
  target_address: 0x742d35Cc...
)

# Now 600K total locked for Polygon

# Step 2: Mint more on Polygon
# Call bridgeMint on EtridPoly contract
token.bridgeMint(
  to: <YOUR_ADDRESS>,
  amount: 500_000 * 10^18,
  txHash: <FLARECHAIN_TX_HASH>
)

# Step 3: Add to liquidity pool
# Go to QuickSwap
# Add: 500K ÉTR + $5,000 MATIC

# Result:
# ✅ 600K ÉTR locked on FlareChain
# ✅ 600K ÉTR total on Polygon
# ✅ Still 1:1 backing
```

### Scenario 3: User Bridging (Future)

**User wants to move 1000 ÉTR from FlareChain to Polygon:**

```bash
# Step 1: User locks on FlareChain
# User calls bridge pallet (not implemented yet)
bridge_to_dex(
  chain_id: ChainId::Polygon,
  amount: 1000 * 10^5,
  target_address: 0xUSER_ADDRESS
)

# Step 2: Bridge relayer sees lock event
# Calls mint on Polygon

# Step 3: User receives ÉTR on Polygon
# Can trade on QuickSwap

# Result:
# User's FlareChain balance: -1000 ÉTR
# User's Polygon balance: +1000 ÉTR
# Total locked increases by 1000
# Backing maintained
```

---

## 🔄 Current Manual Process (Before Bridge)

### For Your $50 Deployment:

**Right now, you don't have automated bridge. Do it manually:**

**Step 1: Deploy Contracts ($15.50)**
```bash
# Mints 100K ÉTR on each chain
# Total: 300K ÉTR across DEXes
```

**Step 2: Lock on FlareChain (Manual)**
```bash
# Via FlareChain runtime / Council

# Submit proposal:
"Lock 300,000 ÉTR from Community LP Pool for DEX backing"

# Requires 6-of-9 Foundation approval

# Once approved, execute:
lock_for_dex(BSC, 100K, 0xBSC_ADDRESS)
lock_for_dex(Polygon, 100K, 0xPOLYGON_ADDRESS)
lock_for_dex(Solana, 100K, SOLANA_MINT)

# Result:
# ✅ 300K ÉTR locked on FlareChain
# ✅ 300K ÉTR minted on DEX chains
# ✅ 1:1 backing
```

**Step 3: Document & Communicate**
```markdown
# Transparency report

## ÉTR Supply (1 Billion Total)

### FlareChain:
- Circulating: 999,700,000 ÉTR
- Locked for DEX backing: 300,000 ÉTR
- Total: 1,000,000,000 ÉTR ✅

### DEX Chains (Wrapped):
- BSC: 100,000 ÉTR (backed by 100K locked)
- Polygon: 100,000 ÉTR (backed by 100K locked)
- Solana: 100,000 ÉTR (backed by 100K locked)
- Total: 300,000 ÉTR ✅

### Proof of Backing:
- FlareChain transaction: <TX_HASH>
- Locked amount query: <EXPLORER_LINK>
- Verifiable on-chain ✅

### Conclusion:
Total supply: 1B ÉTR (honest, no inflation)
All wrapped ÉTR is 1:1 backed by locked native ÉTR
```

---

## 📊 Monitoring & Verification

### Public Dashboard (Recommended)

Create a simple website showing:

```html
<!-- Example: https://etrid.org/supply -->

<h1>ÉTR Supply Transparency</h1>

<h2>Native Supply (FlareChain)</h2>
<p>Total: 1,000,000,000 ÉTR</p>
<p>Circulating: 999,700,000 ÉTR</p>
<p>Locked for DEX: 300,000 ÉTR</p>

<h2>Wrapped Supply (DEX Chains)</h2>
<table>
  <tr><td>BSC:</td><td>100,000 ÉTR</td><td><a href="https://bscscan.com/token/0xABC">Verify</a></td></tr>
  <tr><td>Polygon:</td><td>100,000 ÉTR</td><td><a href="https://polygonscan.com/token/0xDEF">Verify</a></td></tr>
  <tr><td>Solana:</td><td>100,000 ÉTR</td><td><a href="https://solscan.io/token/7XYZ">Verify</a></td></tr>
</table>

<h2>Lock Proof</h2>
<p>FlareChain Lock Transaction: <a href="https://explorer.etrid.org/tx/0x123">View</a></p>
<p>Current Locked Amount: <span id="locked-amount">Loading...</span></p>

<script>
  // Query FlareChain RPC
  fetch('https://rpc.etrid.org', {
    method: 'POST',
    body: JSON.stringify({
      method: 'dexLock_totalLocked',
      params: []
    })
  })
  .then(res => res.json())
  .then(data => {
    document.getElementById('locked-amount').textContent =
      (data.result / 100000).toLocaleString() + ' ÉTR';
  });
</script>
```

---

## ✅ Checklist: Implementing Locking

### Phase 1: Pallet Development

- [ ] Create `pallet-dex-lock` with code above
- [ ] Add to runtime (Cargo.toml)
- [ ] Configure Foundation multisig as origin
- [ ] Set max lock amount (e.g., 250M ÉTR)
- [ ] Test on local testnet
- [ ] Deploy to FlareChain testnet
- [ ] Test with testnet deployments

### Phase 2: Initial Locking

- [ ] Set lock account (Community LP Pool address)
- [ ] Calculate total to lock (300K for initial deployment)
- [ ] Submit lock transaction (6-of-9 Foundation approval)
- [ ] Verify lock on-chain
- [ ] Deploy DEX contracts
- [ ] Document locking proof

### Phase 3: Transparency

- [ ] Create supply dashboard (https://etrid.org/supply)
- [ ] Add lock queries to explorer
- [ ] Publish transparency report
- [ ] Update documentation
- [ ] Communicate to community

### Phase 4: Automation (Future)

- [ ] Build cross-chain bridge
- [ ] Integrate with pallet-dex-lock
- [ ] Automatic lock on bridge deposit
- [ ] Automatic unlock on bridge withdrawal
- [ ] Relayer system
- [ ] Full automation

---

## 🎯 Summary

**The Honest Way:**
1. Deploy DEX contracts (300K ÉTR minted)
2. Lock 300K ÉTR on FlareChain (from Community LP Pool)
3. Now it's 1:1 backed
4. Total supply: 1B (no inflation)
5. Document and communicate transparently

**Result:**
- ✅ Honest accounting
- ✅ 1:1 backing provable
- ✅ Community can verify
- ✅ Professional and trustworthy
- ✅ Foundation charter compliant

Done! This is the right way to do it. 🔒
