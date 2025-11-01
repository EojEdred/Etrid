# Bridge Integration Guide for pallet-etr-lock

This guide shows how to integrate all PBC bridges with the shared `pallet-etr-lock` for ETR token locking.

## Overview

When a user wants to bridge ETR to an external chain:
1. Bridge pallet calls `pallet_etr_lock::lock_for_bridge()`
2. ETR is locked on FlareChain
3. Bridge emits event for relayer
4. Relayer mints wrapped ETR on external chain

When a user wants to bridge back:
1. User burns wrapped ETR on external chain
2. Bridge pallet detects burn event
3. Bridge pallet calls `pallet_etr_lock::unlock_from_bridge()`
4. ETR is unlocked and sent to user

---

## Integration Pattern

### Step 1: Import pallet-etr-lock in your bridge pallet

```rust
// In your bridge pallet's lib.rs or Cargo.toml dependencies
use pallet_etr_lock;
```

### Step 2: Add EtrLock to your Config trait

```rust
#[pallet::config]
pub trait Config: frame_system::Config + pallet_etr_lock::Config {
    // Your existing config...
}
```

### Step 3: When user requests ETR bridge OUT

```rust
#[pallet::call_index(X)]
#[pallet::weight(100_000)]
pub fn bridge_etr_out(
    origin: OriginFor<T>,
    amount: BalanceOf<T>,
    destination_address: Vec<u8>,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // 1. Lock ETR using shared pallet
    pallet_etr_lock::Pallet::<T>::lock_for_bridge(
        frame_system::RawOrigin::Signed(who.clone()).into(),
        pallet_etr_lock::ChainId::Ethereum, // or your chain ID
        amount,
        destination_address.clone(),
    )?;

    // 2. Emit event for relayer
    Self::deposit_event(Event::EtrBridgedOut {
        user: who,
        amount,
        destination: destination_address,
    });

    Ok(())
}
```

### Step 4: When detecting burn event on external chain

```rust
#[pallet::call_index(Y)]
#[pallet::weight(100_000)]
pub fn process_burn_from_external(
    origin: OriginFor<T>,
    user: T::AccountId,
    amount: BalanceOf<T>,
    burn_tx_hash: Vec<u8>,
) -> DispatchResult {
    // Verify this is from authorized relayer/oracle
    ensure_signed(origin)?;

    // 1. Unlock ETR using shared pallet
    pallet_etr_lock::Pallet::<T>::unlock_from_bridge(
        frame_system::RawOrigin::Root.into(),
        pallet_etr_lock::ChainId::Ethereum, // or your chain ID
        amount,
    )?;

    // 2. Transfer unlocked ETR to user
    T::Currency::transfer(
        &Self::get_lock_account()?,
        &user,
        amount,
        ExistenceRequirement::KeepAlive,
    )?;

    // 3. Emit event
    Self::deposit_event(Event::EtrBridgedIn {
        user,
        amount,
        burn_tx: burn_tx_hash,
    });

    Ok(())
}
```

---

## Chain ID Mapping

Use these ChainId variants from `pallet_etr_lock::ChainId`:

| Bridge Pallet | ChainId Variant | Value |
|---------------|----------------|-------|
| `eth_bridge` | `ChainId::Ethereum` | 10 |
| `polygon_bridge` | `ChainId::Polygon` | 3 |
| `bnb_bridge` | `ChainId::BnbChain` | 11 |
| `sol_bridge` | `ChainId::Solana` | 13 |
| `pallet_bitcoin_bridge` | `ChainId::Bitcoin` | 20 |
| `pallet_cardano_bridge` | `ChainId::Cardano` | 21 |
| `stellar_bridge` | `ChainId::Stellar` | 22 |
| `xrp_bridge` | `ChainId::Ripple` | 23 |
| `pallet_doge_bridge` | `ChainId::Dogecoin` | 24 |
| `trx_bridge` | `ChainId::Tron` | 25 |
| `chainlink_bridge` | `ChainId::Chainlink` | 26 |
| `stablecoin_usdt_bridge` | `ChainId::UsdtBridge` | 30 |
| (Base - Layer 2) | `ChainId::Base` | 0 |
| (Arbitrum - Layer 2) | `ChainId::Arbitrum` | 1 |
| (Optimism - Layer 2) | `ChainId::Optimism` | 2 |
| (Avalanche) | `ChainId::Avalanche` | 12 |

---

## Complete Example: Ethereum Bridge

### File: `05-multichain/bridge-protocols/ethereum-bridge/src/lib.rs`

```rust
// Add after existing extrinsics:

/// Bridge ETR tokens to Ethereum
///
/// Locks ETR on FlareChain and emits event for relayer to mint on Ethereum
#[pallet::call_index(10)] // Use next available index
#[pallet::weight(150_000)]
pub fn bridge_etr_to_ethereum(
    origin: OriginFor<T>,
    amount: BalanceOf<T>,
    eth_destination: EthereumAddress,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Convert Ethereum address to bytes
    let destination_bytes = eth_destination.as_bytes().to_vec();

    // Lock ETR using shared locking pallet
    pallet_etr_lock::Pallet::<T>::lock_for_bridge(
        frame_system::RawOrigin::Signed(who.clone()).into(),
        pallet_etr_lock::ChainId::Ethereum,
        amount,
        destination_bytes.clone(),
    )?;

    // Emit event for relayer
    Self::deposit_event(Event::EtrBridgedToEthereum {
        from: who,
        amount,
        eth_address: eth_destination,
    });

    Ok(())
}

/// Process ETR burn from Ethereum (called by relayer)
///
/// Unlocks ETR on FlareChain when wrapped ETR is burned on Ethereum
#[pallet::call_index(11)]
#[pallet::weight(150_000)]
pub fn process_etr_burn_from_ethereum(
    origin: OriginFor<T>,
    etrid_recipient: T::AccountId,
    amount: BalanceOf<T>,
    eth_burn_tx: EthTxHash,
) -> DispatchResult {
    // Should be called by authorized relayer/oracle
    let _relayer = ensure_signed(origin)?;
    // TODO: Add relayer authorization check

    // Verify burn hasn't been processed
    ensure!(
        !ProcessedEthBurns::<T>::contains_key(&eth_burn_tx),
        Error::<T>::BurnAlreadyProcessed
    );

    // Unlock ETR
    pallet_etr_lock::Pallet::<T>::unlock_from_bridge(
        frame_system::RawOrigin::Root.into(),
        pallet_etr_lock::ChainId::Ethereum,
        amount,
    )?;

    // Get lock account
    let lock_account = pallet_etr_lock::Pallet::<T>::lock_account()
        .ok_or(Error::<T>::LockAccountNotSet)?;

    // Transfer to recipient
    T::Currency::transfer(
        &lock_account,
        &etrid_recipient,
        amount,
        ExistenceRequirement::KeepAlive,
    )?;

    // Mark as processed
    ProcessedEthBurns::<T>::insert(&eth_burn_tx, true);

    // Emit event
    Self::deposit_event(Event::EtrUnlockedFromEthereum {
        to: etrid_recipient,
        amount,
        eth_burn_tx,
    });

    Ok(())
}
```

### Add Events:

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    // ... existing events ...

    /// ETR bridged to Ethereum
    EtrBridgedToEthereum {
        from: T::AccountId,
        amount: BalanceOf<T>,
        eth_address: EthereumAddress,
    },

    /// ETR unlocked from Ethereum
    EtrUnlockedFromEthereum {
        to: T::AccountId,
        amount: BalanceOf<T>,
        eth_burn_tx: EthTxHash,
    },
}
```

### Add Storage:

```rust
/// Processed Ethereum burn transactions (to prevent replay)
#[pallet::storage]
pub type ProcessedEthBurns<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    EthTxHash,
    bool,
    ValueQuery,
>;
```

---

## Quick Integration Checklist

For each bridge pallet:

- [ ] Import `pallet_etr_lock`
- [ ] Add `pallet_etr_lock::Config` to Config trait bounds
- [ ] Create `bridge_etr_out()` extrinsic that calls `lock_for_bridge()`
- [ ] Create `process_burn()` extrinsic that calls `unlock_from_bridge()`
- [ ] Use correct ChainId for your chain
- [ ] Add events for bridge operations
- [ ] Add storage for processed transactions (prevent replay)
- [ ] Update relayer to watch for bridge events
- [ ] Test with small amounts first

---

## Testing

```rust
#[test]
fn test_etr_bridge_out() {
    new_test_ext().execute_with(|| {
        // Setup lock account
        let lock_account = 1;
        let user = 2;

        // Give lock account some ETR
        Balances::make_free_balance_be(&lock_account, 1_000_000);

        // Set lock account
        assert_ok!(EtrLock::set_lock_account(Origin::root(), lock_account));

        // Bridge ETR
        assert_ok!(YourBridge::bridge_etr_out(
            Origin::signed(user),
            100,
            vec![1, 2, 3, 4] // destination address
        ));

        // Verify locked
        assert_eq!(EtrLock::total_locked(), 100);
        assert_eq!(EtrLock::locked_for_chain(ChainId::Ethereum), 100);
    });
}
```

---

## Deployment Steps

1. **Build runtime with new pallet**
   ```bash
   cd 05-multichain/flare-chain/node
   cargo build --release
   ```

2. **Set lock account (one-time)**
   ```bash
   # Via Polkadot.js Apps:
   # Developer → Extrinsics
   # etrLock.setLockAccount(COMMUNITY_LP_POOL_ADDRESS)
   ```

3. **Update all bridge pallets** following this guide

4. **Perform runtime upgrade**

5. **Test with small amounts**

6. **Monitor locked amounts**
   ```bash
   # Query total locked
   curl -X POST -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"state_call", "params":["EtrLockApi_total_locked", "0x"]}' \
     http://localhost:9944
   ```

---

## Support

For questions:
- Check `/Users/macbook/Desktop/etrid/pallets/pallet-etr-lock/src/lib.rs`
- See runtime config in `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/runtime/src/lib.rs`
- Review deployment handoff: `/Users/macbook/Desktop/etrid/MAINNET_DEPLOYMENT_HANDOFF.md`
