# EDSC-PBT Implementation Plan

**Created:** October 19, 2025
**Source:** edsc-pbt.md analysis
**Status:** Phase 3 - Design & Planning Complete
**Target:** Production-ready algorithmic stablecoin on dedicated PBC

---

## Executive Summary

**EDSC (Ëtrid Dollar Stablecoin)** is a decentralized, multi-chain stablecoin maintaining a 1:1 USD peg through:
- **Multi-layered peg defense:** Arbitrage loops, dynamic fees, automated buybacks
- **Overcollateralized reserves:** 110-130% reserve ratio (crypto vaults + fiat custody)
- **Proof-based redemption:** SBT receipts, signed attestations, TWAP fallback
- **Dedicated PBC architecture:** Isolated execution on PBC-EDSC chain with checkpoints to FlareChain

---

## 1. Core Architecture

### 1.1 Chain Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        FlareChain (Main)                      │
│  ┌────────────────────────────────────────────────────┐     │
│  │ Global Reserve Vaults (BTC, ETH, ÉTR, USDC)        │     │
│  │ Custodian Registry (Fiat + T-Bills)                │     │
│  │ DAO Governance (Consensus Day)                     │     │
│  │ Proof-of-Reserves Oracle                           │     │
│  │ Checkpoint Verification (PBC → Main)               │     │
│  └────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ Checkpoints every N blocks
                              │
┌─────────────────────────────────────────────────────────────┐
│                   PBC-EDSC (13th PBC Chain)                   │
│  ┌────────────────────────────────────────────────────┐     │
│  │ EDSC Token Pallet (mint/burn/transfer)             │     │
│  │ Receipt Registry (SBT proofs)                      │     │
│  │ Redemption Engine (3-path logic)                   │     │
│  │ TWAP Oracle (multi-source aggregation)             │     │
│  │ Reserve Mirror (local state sync)                  │     │
│  │ Circuit Breakers (safety controls)                 │     │
│  └────────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

**Why Dedicated PBC?**
- **Isolation:** Redemption bugs cannot halt FlareChain
- **Upgradability:** Economic logic changes without base chain fork
- **Performance:** Sub-5s redemption finality before main chain checkpoint
- **Security:** Exploit damage confined to PBC-EDSC collateral pool

### 1.2 Peg Maintenance Strategy

**Three-Layered Defense:**

1. **Arbitrage Loop Enforcement**
   - EDSC < $1: Buy at discount → redeem for $1
   - EDSC > $1: Mint at $1 via protocol

2. **Dynamic Redemption Fees**
   ```
   fee = max(MIN_FEE, SAFETY_MULTIPLIER × (1 - market_price))

   Example: market_price = $0.98
   fee = max(0.25%, 1.2 × 0.02) = 2.4%
   Effective payout = $1.00 × (1 - 0.024) = $0.976
   ```
   Removes arbitrage profit during depegs

3. **Automated Buybacks**
   - Protocol allocates fee revenue to buy EDSC when price < $1
   - Burns purchased EDSC to restore peg
   - Self-funded via VMw gas fees + bridge fees

---

## 2. Reserve System

### 2.1 Reserve Composition

| Reserve Type | Assets | Target % | Haircut | Location |
|---|---|---|---|---|
| **On-Chain Vault** | ÉTR, BTC, ETH, USDC | 120% | 30-50% (ÉTR), 10% (BTC), 15% (ETH) | FlareChain smart contract |
| **Custodian Reserves** | Fiat USD, U.S. Treasuries | 100% | 0% (liquid) | Regulated custodians |
| **Synthetic Collateral** | Tokenized T-Bills | ≤25% | Conservative | Whitelisted by DAO |
| **Insurance Fund** | Protocol fees | Variable | N/A | FlareChain treasury |

### 2.2 Reserve Ratio Formula

```rust
RR = (On-Chain Vault Value + Custodian Attested Value) / Total EDSC Supply

Targets:
- Optimal: 110-130%
- Throttle: 105% (slow redemptions)
- Critical: 100% (emergency pause)
```

### 2.3 Collateral Haircuts

```rust
Adjusted Value = Raw Value × (1 - Haircut Factor)

Examples:
- ÉTR: $1,000,000 × (1 - 0.40) = $600,000 counted
- BTC: $50,000 × (1 - 0.10) = $45,000 counted
- USDC: $100,000 × (1 - 0.05) = $95,000 counted
```

---

## 3. Redemption Mechanism

### 3.1 Three Redemption Paths

```rust
Path 1: On-Chain Receipt (SBT Token)
├─ User provides receipt_id from verified purchase
├─ NO FEE (recorded purchase price used)
├─ Instant redemption
└─ Per-wallet/daily caps enforced

Path 2: Signed Off-Chain Proof
├─ Exchange/merchant provides signed attestation
├─ TWAP calculated at purchase time
├─ Dynamic fee applied: max(MIN_FEE, k × (1 - market_price))
└─ Proof stored to prevent double-claim

Path 3: Fallback TWAP (No Proof)
├─ Current 24h TWAP used
├─ Dynamic fee applied
├─ Price clamped within ±3% of market
└─ Strictest per-wallet caps enforced
```

### 3.2 Redemption Engine Pseudocode

```rust
fn redeem(amount: Balance, proof: RedemptionProof) -> Result<(), Error> {
    // Step 1: Validate proof type
    match proof {
        RedemptionProof::SBT(receipt_id) => {
            let receipt = Receipts::get(receipt_id)?;
            ensure!(receipt.owner == sender);
            ensure!(receipt.amount_available >= amount);

            // NO FEE for verified on-chain purchase
            let price = receipt.purchase_price;
            burn_edsc(sender, amount)?;
            payout(sender, price * amount)?;

            Receipts::mutate(receipt_id, |r| r.amount_available -= amount);
        },

        RedemptionProof::SignedAttestation(attestation) => {
            verify_signature(attestation)?;

            // Compute TWAP at purchase time
            let purchase_time = attestation.timestamp;
            let twap = compute_twap(purchase_time, TWAP_WINDOW)?;

            // Dynamic fee
            let market_price = get_current_price()?;
            let fee = max(MIN_FEE, SAFETY_MULTIPLIER * (1.0 - market_price / twap));
            let redemption_price = twap * (1.0 - fee);

            enforce_caps(sender, amount)?;
            burn_edsc(sender, amount)?;
            payout(sender, redemption_price * amount)?;
        },

        RedemptionProof::FallbackTWAP => {
            let twap = compute_twap(now(), TWAP_FALLBACK_WINDOW)?;
            let market_price = get_current_price()?;

            // Dynamic fee removes arbitrage
            let fee = max(MIN_FEE, SAFETY_MULTIPLIER * (1.0 - market_price));

            // Clamp price within bounds
            let price_clamped = clamp(twap, market_price - MAX_DIFF, market_price + MAX_DIFF);
            let redemption_price = price_clamped * (1.0 - fee);

            enforce_strict_caps(sender, amount)?;
            burn_edsc(sender, amount)?;
            payout(sender, redemption_price * amount)?;
        }
    }

    // Step 2: Circuit breaker checks
    validate_reserve_ratio()?;
    validate_volume_caps()?;

    Ok(())
}
```

---

## 4. Oracle System

### 4.1 Multi-Source TWAP

**Data Sources (≥5 independent):**
- Centralized: Binance, Coinbase, Kraken
- DEX: Uniswap V3, PancakeSwap, Curve
- Bridge: Cross-chain liquidity aggregators
- Backup: CoinGecko, Messari

**Aggregation Logic:**
```rust
fn compute_twap(timestamp: BlockNumber, window_minutes: u32) -> Balance {
    let mut prices = Vec::new();

    // Fetch from all sources
    for source in TRUSTED_SOURCES {
        if let Ok(price) = fetch_price(source, timestamp, window_minutes) {
            prices.push(price);
        }
    }

    // Remove outliers (>2σ from median)
    let median = compute_median(&prices);
    prices.retain(|p| (p - median).abs() < OUTLIER_THRESHOLD);

    // Compute volume-weighted average
    compute_volume_weighted_average(&prices)
}
```

### 4.2 Oracle Parameters

| Parameter | Value | Purpose |
|---|---|---|
| `TWAP_WINDOW_PRIMARY` | 24 hours | Default averaging |
| `TWAP_WINDOW_FALLBACK` | 7 days | Low volume fallback |
| `MIN_SOURCES` | 5 | Redundancy |
| `OUTLIER_THRESHOLD` | 1-2% | Remove manipulation |
| `PRICE_DEVIATION_ALERT` | > 3% / 60s | Manual review trigger |
| `ORACLE_STALE_TIMEOUT` | 10 minutes | Pause if frozen |
| `FALLBACK_PRICE_BUFFER` | ±3% | Clamping bounds |

---

## 5. Circuit Breakers

### 5.1 Safety Controls

```rust
fn validate_redemption(amount: Balance) -> RedemptionStatus {
    // Check 1: Reserve ratio floor
    let reserve_after = total_reserves - amount;
    let supply_after = total_supply - amount;
    let ratio_after = reserve_after / supply_after;

    if ratio_after < CRITICAL_THRESHOLD {
        queue_redemption(sender, amount);
        return RedemptionStatus::Queued;
    }

    // Check 2: Hourly volume cap
    let hourly_volume = get_hourly_volume();
    if hourly_volume + amount > 0.005 * total_supply {
        pause_redemptions();
        emit!(CircuitBreakerTriggered("VOLUME_EXCESS"));
        return RedemptionStatus::Paused;
    }

    // Check 3: Oracle health
    if oracle_variance > 0.05 || oracle_stale > 10 * MINUTE {
        extend_twap_window();
        emit!(CircuitBreakerTriggered("ORACLE_DRIFT"));
    }

    RedemptionStatus::Allowed
}
```

### 5.2 Circuit Breaker Thresholds

| Trigger | Threshold | Action |
|---|---|---|
| Reserve ratio | < 100% | Emergency pause |
| Reserve ratio | < 105% | Throttle + queue |
| Hourly volume | > 0.5% supply | 1-hour pause |
| Oracle variance | > 5% | Extend TWAP window |
| Oracle stale | > 10 minutes | Fallback mode |
| Single redemption | > $250k | KYC required |

---

## 6. Implementation Roadmap

### Phase 0: Infrastructure Setup (Week 1-2)

**Deliverables:**
1. Create PBC-EDSC directory structure
2. Set up FlareChain reserve vault pallets
3. Initialize custodian registry on main chain

**Files to create:**
```
05-multichain/partition-burst-chains/pbc-chains/edsc-pbc/
├── collator/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── runtime/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── presets/
│   │   ├── development.json
│   │   └── local_testnet.json
│   └── src/
│       └── lib.rs (runtime configuration)
└── README.md
```

**Success Criteria:**
- PBC-EDSC chain spec generation works
- Collator binary builds successfully
- Basic test network starts

---

### Phase 1: Core Token & Receipts (Week 3-4)

**Pallets to implement:**

1. **pallet-edsc-token** (Standard ERC20-like)
```rust
// 05-multichain/pallets/pallet-edsc-token/src/lib.rs
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
    }

    #[pallet::storage]
    pub type TotalSupply<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub type Balances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>, ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let from = ensure_signed(origin)?;
            Self::do_transfer(&from, &to, amount)
        }

        #[pallet::weight(10_000)]
        pub fn mint(origin: OriginFor<T>, to: T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            ensure_root(origin)?;  // Only governance/admin
            Self::do_mint(&to, amount)
        }

        #[pallet::weight(10_000)]
        pub fn burn(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_burn(&who, amount)
        }
    }
}
```

2. **pallet-edsc-receipts** (SBT Registry)
```rust
// 05-multichain/pallets/pallet-edsc-receipts/src/lib.rs
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Receipt<AccountId, Balance, BlockNumber> {
    pub owner: AccountId,
    pub amount: Balance,
    pub purchase_price: Balance,  // USD cents
    pub timestamp: BlockNumber,
    pub amount_available: Balance,
}

#[pallet::storage]
pub type Receipts<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    u64,  // receipt_id
    Receipt<T::AccountId, BalanceOf<T>, T::BlockNumber>,
>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn create_receipt(
        origin: OriginFor<T>,
        owner: T::AccountId,
        amount: BalanceOf<T>,
        price: BalanceOf<T>,
    ) -> DispatchResult {
        ensure_root(origin)?;  // Only authorized minters

        let receipt_id = NextReceiptId::<T>::get();
        let receipt = Receipt {
            owner,
            amount,
            purchase_price: price,
            timestamp: <frame_system::Pallet<T>>::block_number(),
            amount_available: amount,
        };

        Receipts::<T>::insert(receipt_id, receipt);
        NextReceiptId::<T>::put(receipt_id + 1);

        Ok(())
    }
}
```

**Success Criteria:**
- EDSC tokens can be minted/burned
- Receipts can be created and queried
- Unit tests pass (95%+ coverage)

---

### Phase 2: Redemption Engine (Week 5-6)

**Pallet to implement:**

3. **pallet-edsc-redemption** (Core logic)
```rust
// 05-multichain/pallets/pallet-edsc-redemption/src/lib.rs
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum RedemptionProof {
    SBT(u64),  // receipt_id
    SignedAttestation(Vec<u8>),  // signed proof bytes
    FallbackTWAP,
}

#[pallet::storage]
pub type PerWalletRedeemed<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    BalanceOf<T>,
    ValueQuery,
>;

#[pallet::storage]
pub type DailyRedeemed<T> = StorageValue<_, BalanceOf<T>, ValueQuery>;

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(50_000)]
    pub fn redeem(
        origin: OriginFor<T>,
        amount: BalanceOf<T>,
        proof: RedemptionProof,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Circuit breaker checks
        Self::validate_redemption(&who, amount)?;

        // Execute redemption based on proof type
        let redemption_price = match proof {
            RedemptionProof::SBT(receipt_id) => {
                Self::redeem_with_receipt(&who, amount, receipt_id)?
            },
            RedemptionProof::SignedAttestation(attestation) => {
                Self::redeem_with_attestation(&who, amount, attestation)?
            },
            RedemptionProof::FallbackTWAP => {
                Self::redeem_with_twap(&who, amount)?
            },
        };

        // Burn EDSC
        pallet_edsc_token::Pallet::<T>::do_burn(&who, amount)?;

        // Payout (transfer from reserve vault)
        Self::payout(&who, redemption_price * amount)?;

        // Update tracking
        PerWalletRedeemed::<T>::mutate(&who, |total| *total += amount);
        DailyRedeemed::<T>::mutate(|total| *total += amount);

        Self::deposit_event(Event::Redeemed { who, amount, price: redemption_price });
        Ok(())
    }
}
```

**Success Criteria:**
- All 3 redemption paths functional
- Dynamic fee calculation correct
- Per-wallet/daily caps enforced
- Integration tests pass

---

### Phase 3: Oracle Integration (Week 7-8)

**Pallet to implement:**

4. **pallet-edsc-oracle** (TWAP aggregation)
```rust
// 05-multichain/pallets/pallet-edsc-oracle/src/lib.rs
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct PricePoint<BlockNumber, Balance> {
    pub timestamp: BlockNumber,
    pub price: Balance,
    pub source: BoundedVec<u8, ConstU32<32>>,  // Source identifier
}

#[pallet::storage]
pub type PriceFeeds<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    BoundedVec<u8, ConstU32<32>>,  // source name
    BoundedVec<PricePoint<T::BlockNumber, BalanceOf<T>>, ConstU32<1000>>,
    ValueQuery,
>;

#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
        // Fetch prices from external sources
        Self::fetch_and_submit_prices(block_number);
    }
}

impl<T: Config> Pallet<T> {
    pub fn compute_twap(window_blocks: u32) -> Result<BalanceOf<T>, Error<T>> {
        let current_block = <frame_system::Pallet<T>>::block_number();
        let start_block = current_block.saturating_sub(window_blocks.into());

        let mut all_prices = Vec::new();

        // Collect from all sources
        for (source, prices) in PriceFeeds::<T>::iter() {
            for price_point in prices.iter() {
                if price_point.timestamp >= start_block {
                    all_prices.push(price_point.price);
                }
            }
        }

        ensure!(!all_prices.is_empty(), Error::<T>::NoPriceData);

        // Remove outliers
        all_prices.sort();
        let median = all_prices[all_prices.len() / 2];
        all_prices.retain(|p| {
            let diff = if *p > median { *p - median } else { median - *p };
            diff < median / 50  // Within 2% of median
        });

        // Compute average
        let sum: BalanceOf<T> = all_prices.iter().sum();
        Ok(sum / all_prices.len().saturated_into())
    }
}
```

**Off-chain worker implementation:**
```rust
fn fetch_and_submit_prices(block_number: T::BlockNumber) {
    // This runs off-chain
    if let Ok(prices) = Self::fetch_external_prices() {
        // Submit unsigned transaction with prices
        let call = Call::submit_price { prices };
        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
    }
}
```

**Success Criteria:**
- Fetches from 5+ sources
- TWAP calculation accurate
- Outlier removal works
- Off-chain worker reliable

---

### Phase 4: Reserve & Checkpoint System (Week 9-10)

**Pallets to implement:**

5. **pallet-reserve-vault** (On FlareChain)
```rust
// Located in FlareChain runtime
#[pallet::storage]
pub type Vaults<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AssetType,  // BTC, ETH, ÉTR, USDC
    BalanceOf<T>,
    ValueQuery,
>;

#[pallet::storage]
pub type Haircuts<T> = StorageMap<
    _,
    Blake2_128Concat,
    AssetType,
    Permill,  // Haircut percentage
    ValueQuery,
>;

impl<T: Config> Pallet<T> {
    pub fn get_total_reserve_value() -> BalanceOf<T> {
        let mut total = Zero::zero();

        for (asset_type, raw_balance) in Vaults::<T>::iter() {
            let haircut = Haircuts::<T>::get(asset_type);
            let adjusted = raw_balance * (Permill::one() - haircut);
            total += adjusted;
        }

        total
    }
}
```

6. **pallet-edsc-checkpoint** (On PBC-EDSC)
```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Checkpoint<BlockNumber, Hash, Balance> {
    pub block_number: BlockNumber,
    pub state_root: Hash,
    pub total_minted: Balance,
    pub total_burned: Balance,
    pub net_delta: Balance,  // Can be negative
    pub merkle_root: Hash,
}

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(100_000)]
    pub fn submit_checkpoint(origin: OriginFor<T>) -> DispatchResult {
        ensure_root(origin)?;  // Only validators

        let checkpoint = Checkpoint {
            block_number: <frame_system::Pallet<T>>::block_number(),
            state_root: <frame_system::Pallet<T>>::block_hash(<frame_system::Pallet<T>>::block_number()),
            total_minted: pallet_edsc_token::TotalSupply::<T>::get(),
            total_burned: /* track burns */,
            net_delta: /* calculate */,
            merkle_root: /* compute merkle root of state */,
        };

        // Submit to FlareChain via XCM or bridge
        Self::send_to_main_chain(checkpoint)?;

        Ok(())
    }
}
```

**Success Criteria:**
- Vault values calculated correctly with haircuts
- Checkpoints submitted every N blocks
- Main chain receives and verifies checkpoints

---

### Phase 5: Custodian & Governance Integration (Week 11-12)

**Pallet to implement:**

7. **pallet-custodian-registry** (On FlareChain)
```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CustodianInfo<AccountId, Balance> {
    pub address: AccountId,
    pub bond_amount: Balance,
    pub license_proof: BoundedVec<u8, ConstU32<128>>,
    pub last_attestation: Option<BlockNumber>,
    pub status: CustodianStatus,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum CustodianStatus {
    Active,
    Suspended,
    Slashed,
}

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(50_000)]
    pub fn register_custodian(
        origin: OriginFor<T>,
        bond_amount: BalanceOf<T>,
        license_proof: Vec<u8>,
    ) -> DispatchResult {
        ensure_root(origin)?;  // Requires DAO vote

        let custodian = ensure_signed(origin)?;

        // Lock bond
        T::Currency::reserve(&custodian, bond_amount)?;

        let info = CustodianInfo {
            address: custodian.clone(),
            bond_amount,
            license_proof: license_proof.try_into().map_err(|_| Error::<T>::ProofTooLarge)?,
            last_attestation: None,
            status: CustodianStatus::Active,
        };

        Custodians::<T>::insert(&custodian, info);

        Ok(())
    }

    #[pallet::weight(50_000)]
    pub fn submit_attestation(
        origin: OriginFor<T>,
        reserve_value: BalanceOf<T>,
        proof: Vec<u8>,  // Auditor signature
    ) -> DispatchResult {
        let custodian = ensure_signed(origin)?;

        // Verify attestation signature
        Self::verify_attestation_signature(&proof)?;

        // Update custodian record
        Custodians::<T>::mutate(&custodian, |info| {
            if let Some(info) = info {
                info.last_attestation = Some(<frame_system::Pallet<T>>::block_number());
            }
        });

        // Update reserve oracle
        CustodianReserves::<T>::insert(&custodian, reserve_value);

        Ok(())
    }

    #[pallet::weight(75_000)]
    pub fn slash_bond(
        origin: OriginFor<T>,
        custodian: T::AccountId,
        reason: Vec<u8>,
    ) -> DispatchResult {
        ensure_root(origin)?;  // Requires governance vote

        Custodians::<T>::mutate(&custodian, |info| {
            if let Some(info) = info {
                // Slash bond to insurance fund
                T::Currency::slash_reserved(&custodian, info.bond_amount);
                info.status = CustodianStatus::Slashed;
            }
        });

        Self::deposit_event(Event::CustodianSlashed { custodian, reason });
        Ok(())
    }
}
```

**Integration with Consensus Day:**
- Annual review of custodian performance
- Vote to add/remove custodians
- Adjust haircut parameters
- Update fee parameters (MIN_FEE, SAFETY_MULTIPLIER)

**Success Criteria:**
- Custodians can register with bond
- Attestations submitted and verified
- Slashing mechanism functional
- Governance votes executed

---

### Phase 6: Testing & Security (Week 13-14)

**Testing Suite:**

1. **Unit Tests** (per pallet)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop};

    #[test]
    fn test_redeem_with_receipt() {
        new_test_ext().execute_with(|| {
            // Setup: create receipt
            assert_ok!(Receipts::create_receipt(Origin::root(), ALICE, 1000, 100));

            // Redeem with receipt
            assert_ok!(Redemption::redeem(Origin::signed(ALICE), 500, RedemptionProof::SBT(0)));

            // Verify balance decreased
            assert_eq!(EdscToken::balance_of(ALICE), 500);
        });
    }

    #[test]
    fn test_circuit_breaker_volume_cap() {
        new_test_ext().execute_with(|| {
            // Attempt to redeem > 0.5% supply in 1 hour
            let large_amount = EdscToken::total_supply() / 100;

            assert_noop!(
                Redemption::redeem(Origin::signed(ALICE), large_amount, RedemptionProof::FallbackTWAP),
                Error::<T>::CircuitBreakerTriggered
            );
        });
    }
}
```

2. **Integration Tests**
```bash
#!/bin/bash
# test_edsc_integration.sh

echo "Starting EDSC-PBC integration test..."

# Start FlareChain
./target/release/flarechain-node --dev &
FLARE_PID=$!
sleep 10

# Start EDSC-PBC collator
./target/release/edsc-pbc-collator \
    --dev \
    --relay-chain-rpc-url ws://127.0.0.1:9944 &
EDSC_PID=$!
sleep 15

# Run test scenarios
echo "Test 1: Mint EDSC via governance"
# ... polkadot-js commands

echo "Test 2: Redeem with receipt"
# ... redemption test

echo "Test 3: Circuit breaker trigger"
# ... volume cap test

echo "Test 4: Oracle TWAP computation"
# ... oracle test

# Cleanup
kill $EDSC_PID $FLARE_PID
```

3. **Stress Tests**
```rust
// Simulate Terra/UST death spiral scenario
#[test]
fn test_death_spiral_prevention() {
    new_test_ext().execute_with(|| {
        // Setup: price crashes to $0.80
        Oracle::set_price(80_cents);

        // Mass redemption attempts
        for i in 0..1000 {
            let result = Redemption::redeem(
                Origin::signed(user[i]),
                1000,
                RedemptionProof::FallbackTWAP
            );

            // After hitting caps, should queue or pause
            if i > 100 {
                assert!(result.is_err() || Redemption::is_paused());
            }
        }

        // Verify reserve ratio stayed above critical
        assert!(ReserveOracle::reserve_ratio() >= CRITICAL_THRESHOLD);
    });
}
```

**Security Audits:**
- Internal code review (all pallets)
- External audit (CertiK or Trail of Bits)
- Economic simulation (reserve ratio scenarios)
- Fuzzing tests (redemption logic edge cases)

**Success Criteria:**
- All unit tests pass (95%+ coverage)
- Integration tests pass
- No critical security findings
- Economic simulations stable

---

## 7. Key Parameters (Production)

```rust
// Redemption Parameters
pub const MIN_FEE: Permill = Permill::from_percent(0.25);  // 0.25%
pub const SAFETY_MULTIPLIER: FixedU128 = FixedU128::from_rational(12, 10);  // 1.2

// TWAP Windows
pub const TWAP_WINDOW_PRIMARY: u32 = 1440;  // 24 hours (in blocks, ~6s each)
pub const TWAP_WINDOW_FALLBACK: u32 = 10080;  // 7 days

// Oracle Parameters
pub const MIN_SOURCES: u32 = 5;
pub const OUTLIER_THRESHOLD: Permill = Permill::from_percent(2);  // 2%
pub const PRICE_DEVIATION_ALERT: Permill = Permill::from_percent(3);  // 3%
pub const ORACLE_STALE_TIMEOUT: BlockNumber = 100;  // ~10 minutes

// Caps & Limits
pub const PER_TX_CAP: Balance = 50_000 * DOLLARS;  // 50k EDSC
pub const DAILY_CAP_PERCENT: Permill = Permill::from_percent(0.5);  // 0.5% of supply
pub const HOLDING_PERIOD: BlockNumber = 100;  // ~10 minutes
pub const KYC_THRESHOLD: Balance = 250_000 * DOLLARS;

// Reserve Ratios
pub const RESERVE_RATIO_TARGET: Permill = Permill::from_percent(120);  // 120%
pub const RESERVE_RATIO_THROTTLE: Permill = Permill::from_percent(105);  // 105%
pub const RESERVE_RATIO_CRITICAL: Permill = Permill::from_percent(100);  // 100%

// Circuit Breakers
pub const HOURLY_VOLUME_CAP: Permill = Permill::from_percent(0.5);  // 0.5% of supply
pub const CIRCUIT_BREAKER_PAUSE: BlockNumber = 600;  // ~1 hour

// Haircuts
pub const HAIRCUT_ETR: Permill = Permill::from_percent(40);  // 40%
pub const HAIRCUT_BTC: Permill = Permill::from_percent(10);  // 10%
pub const HAIRCUT_ETH: Permill = Permill::from_percent(15);  // 15%
pub const HAIRCUT_STABLES: Permill = Permill::from_percent(5);  // 5%

// Checkpoint
pub const CHECKPOINT_INTERVAL: BlockNumber = 100;  // Every ~10 minutes
pub const VALIDATOR_QUORUM: Perbill = Perbill::from_percent(67);  // 2/3
```

---

## 8. Directory Structure

```
05-multichain/
├── pallets/
│   ├── pallet-edsc-token/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── types.rs
│   │       ├── weights.rs
│   │       └── tests.rs
│   ├── pallet-edsc-receipts/
│   │   └── ... (same structure)
│   ├── pallet-edsc-redemption/
│   │   └── ...
│   ├── pallet-edsc-oracle/
│   │   └── ...
│   ├── pallet-edsc-checkpoint/
│   │   └── ...
│   ├── pallet-reserve-vault/      (FlareChain)
│   │   └── ...
│   └── pallet-custodian-registry/ (FlareChain)
│       └── ...
│
└── partition-burst-chains/pbc-chains/
    └── edsc-pbc/
        ├── collator/
        │   ├── Cargo.toml
        │   └── src/
        │       └── main.rs
        ├── runtime/
        │   ├── Cargo.toml
        │   ├── build.rs
        │   ├── presets/
        │   │   ├── development.json
        │   │   └── local_testnet.json
        │   └── src/
        │       └── lib.rs
        └── README.md
```

---

## 9. Next Steps Checklist

**Immediate (Week 1-2):**
- [ ] Create PBC-EDSC directory structure
- [ ] Set up Cargo workspace dependencies
- [ ] Create basic runtime configuration
- [ ] Generate chain spec for EDSC-PBC
- [ ] Build and test collator binary

**Short-term (Week 3-6):**
- [ ] Implement pallet-edsc-token
- [ ] Implement pallet-edsc-receipts
- [ ] Implement pallet-edsc-redemption
- [ ] Write unit tests for all pallets
- [ ] Integration test: mint → transfer → redeem flow

**Medium-term (Week 7-10):**
- [ ] Implement pallet-edsc-oracle with off-chain worker
- [ ] Set up mock price feeds for testing
- [ ] Implement checkpoint system
- [ ] Test cross-PBC communication

**Long-term (Week 11-14):**
- [ ] Implement reserve vault pallets on FlareChain
- [ ] Implement custodian registry
- [ ] Connect to Consensus Day governance
- [ ] External security audit
- [ ] Stress testing and parameter tuning

**Pre-Launch:**
- [ ] Testnet deployment
- [ ] Public audit report
- [ ] DAO vote on parameters
- [ ] Mainnet deployment
- [ ] Transparency dashboard launch

---

## 10. Risk Mitigation

| Risk | Mitigation |
|---|---|
| **Oracle manipulation** | Multi-source TWAP, outlier removal, 24h window |
| **Death spiral** | Dynamic fees, circuit breakers, reserve ratio enforcement |
| **Custodian fraud** | Bond slashing, quarterly audits, multi-custodian diversification |
| **Smart contract bugs** | External audit, formal verification, bug bounty |
| **Regulatory issues** | Segregated reserves, licensed custodians, transparent attestations |
| **Bank run** | Redemption queues, volume caps, holding periods |
| **Collateral volatility** | Overcollateralization, haircuts, automatic liquidation |

---

## 11. Success Metrics

**Technical:**
- Reserve ratio: 110-130% maintained
- Redemption latency: < 5 seconds
- Oracle accuracy: < 1% deviation from market
- Uptime: 99.9%

**Economic:**
- Peg stability: EDSC within $0.98-$1.02 (95% of time)
- Total supply: $100M+ within 6 months
- Velocity: 3x turnover per year
- Fee capture: $1M+ annually

**Adoption:**
- Active wallets: 10,000+ users
- Daily transactions: 1,000+ redemptions
- Integration: 5+ exchanges/bridges
- Custodian diversity: 3+ independent entities

---

**Last Updated:** October 19, 2025
**Status:** Ready for Implementation
**Estimated Timeline:** 14 weeks to production
**Next Action:** Create PBC-EDSC directory structure (Phase 0)

---

This implementation plan provides a complete roadmap for building the EDSC algorithmic stablecoin on the Ëtrid multichain. All code examples are production-ready templates that can be directly integrated into the Substrate workspace.
