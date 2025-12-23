# Integration Examples for Different PBCs

This document provides concrete examples of how different Partition Burst Chains (PBCs) in the Ëtrid ecosystem can integrate `pallet-token-messenger`.

## Table of Contents

1. [EDSC PBC Integration](#edsc-pbc-integration)
2. [ETH PBC Integration](#eth-pbc-integration)
3. [SOL PBC Integration](#sol-pbc-integration)
4. [BTC PBC Integration](#btc-pbc-integration)
5. [USDC PBC Integration](#usdc-pbc-integration)
6. [Cross-PBC Transfers](#cross-pbc-transfers)

---

## EDSC PBC Integration

### Overview
EDSC (Euro Digital Stable Coin) PBC uses the token messenger to bridge EDSC tokens to external chains (Ethereum, Polygon, etc.) and to other PBCs.

### Token Operations Implementation

```rust
// File: runtime/pbc-edsc/src/token_operations.rs

use pallet_token_messenger::TokenOperations;
use frame_support::dispatch::DispatchResult;

pub struct EdscTokenOperations;

impl TokenOperations<AccountId> for EdscTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // EDSC uses native balance
        let balance_amount = amount
            .try_into()
            .map_err(|_| "Amount too large")?;

        pallet_balances::Pallet::<Runtime>::burn_from(
            account,
            balance_amount,
        )?;

        Ok(())
    }

    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        let balance_amount = amount
            .try_into()
            .map_err(|_| "Amount too large")?;

        pallet_balances::Pallet::<Runtime>::mint_into(
            account,
            balance_amount,
        )?;

        Ok(())
    }

    fn balance_of(account: &AccountId) -> u128 {
        pallet_balances::Pallet::<Runtime>::free_balance(account).into()
    }
}
```

### Runtime Configuration

```rust
// File: runtime/pbc-edsc/src/lib.rs

parameter_types! {
    pub const MaxMessageBodySize: u32 = 512;
    pub const MaxBurnAmount: u128 = 1_000_000 * EDSC; // 1M EDSC
    pub const DailyBurnCap: u128 = 10_000_000 * EDSC; // 10M EDSC/day
    pub const MinBurnAmount: u128 = 1 * MILLIUNIT; // 0.001 EDSC
    pub const MessageTimeout: BlockNumber = 14400;
    pub const BlocksPerDay: BlockNumber = 14400;
    pub const LocalDomain: u32 = 100; // EDSC PBC domain
}

impl pallet_token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenOperations = EdscTokenOperations;
    type AttestationVerifier = BridgeAttestationVerifier;
    type WeightInfo = pallet_token_messenger::weights::SubstrateWeight<Runtime>;
    type MaxMessageBodySize = MaxMessageBodySize;
    type MaxBurnAmount = MaxBurnAmount;
    type DailyBurnCap = DailyBurnCap;
    type MinBurnAmount = MinBurnAmount;
    type MessageTimeout = MessageTimeout;
    type BlocksPerDay = BlocksPerDay;
    type LocalDomain = LocalDomain;
}
```

### Genesis Configuration

```rust
// File: runtime/pbc-edsc/src/genesis.rs

use pallet_token_messenger::DomainConfig;

pub fn edsc_genesis_config() -> RuntimeGenesisConfig {
    RuntimeGenesisConfig {
        // ... other configs
        token_messenger: TokenMessengerConfig {
            // Pre-configure major destinations
            domains: vec![
                // Ethereum
                (0, DomainConfig {
                    enabled: true,
                    max_burn_amount: 1_000_000 * EDSC,
                    daily_burn_limit: 5_000_000 * EDSC,
                    min_burn_amount: 1 * MILLIUNIT,
                }),
                // Polygon
                (3, DomainConfig {
                    enabled: true,
                    max_burn_amount: 500_000 * EDSC,
                    daily_burn_limit: 2_000_000 * EDSC,
                    min_burn_amount: 1 * MILLIUNIT,
                }),
                // ETH PBC
                (101, DomainConfig {
                    enabled: true,
                    max_burn_amount: 5_000_000 * EDSC,
                    daily_burn_limit: 20_000_000 * EDSC,
                    min_burn_amount: 1 * MILLIUNIT,
                }),
            ],
            ..Default::default()
        },
    }
}
```

---

## ETH PBC Integration

### Overview
ETH PBC bridges wrapped Ethereum tokens between Ethereum mainnet, other PBCs, and Layer 2s.

### Token Operations with Assets Pallet

```rust
// File: runtime/pbc-eth/src/token_operations.rs

use pallet_token_messenger::TokenOperations;
use pallet_assets::Pallet as Assets;

pub struct EthTokenOperations;

// ETH PBC uses pallet-assets for wrapped ETH
const WRAPPED_ETH_ASSET_ID: u32 = 0;

impl TokenOperations<AccountId> for EthTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Burn from asset pallet
        Assets::<Runtime>::burn_from(
            RuntimeOrigin::signed(account.clone()),
            WRAPPED_ETH_ASSET_ID,
            account.clone(),
            amount,
        )?;

        Ok(())
    }

    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Mint using asset pallet
        Assets::<Runtime>::mint_into(
            WRAPPED_ETH_ASSET_ID,
            account,
            amount,
        )?;

        Ok(())
    }

    fn balance_of(account: &AccountId) -> u128 {
        Assets::<Runtime>::balance(WRAPPED_ETH_ASSET_ID, account)
    }
}
```

### Runtime Configuration

```rust
// File: runtime/pbc-eth/src/lib.rs

parameter_types! {
    pub const LocalDomain: u32 = 101; // ETH PBC domain
    pub const MaxBurnAmount: u128 = 100 * ETHER; // 100 ETH
    pub const DailyBurnCap: u128 = 1_000 * ETHER; // 1000 ETH/day
    pub const MinBurnAmount: u128 = 10_000_000_000_000_000; // 0.01 ETH
}

impl pallet_token_messenger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenOperations = EthTokenOperations;
    type AttestationVerifier = BridgeAttestationVerifier;
    type WeightInfo = pallet_token_messenger::weights::SubstrateWeight<Runtime>;
    type MaxMessageBodySize = MaxMessageBodySize;
    type MaxBurnAmount = MaxBurnAmount;
    type DailyBurnCap = DailyBurnCap;
    type MinBurnAmount = MinBurnAmount;
    type MessageTimeout = MessageTimeout;
    type BlocksPerDay = BlocksPerDay;
    type LocalDomain = LocalDomain;
}
```

---

## SOL PBC Integration

### Overview
SOL PBC bridges Solana-based tokens (SOL, SPL tokens) to other chains.

### Token Operations

```rust
// File: runtime/pbc-sol/src/token_operations.rs

pub struct SolTokenOperations;

// SOL PBC tracks wrapped SOL balance
const WRAPPED_SOL_ASSET_ID: u32 = 0;

impl TokenOperations<AccountId> for SolTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Solana uses 9 decimals, need conversion layer
        let sol_amount = amount; // Already in lamports (base unit)

        pallet_assets::Pallet::<Runtime>::burn_from(
            RuntimeOrigin::signed(account.clone()),
            WRAPPED_SOL_ASSET_ID,
            account.clone(),
            sol_amount,
        )?;

        Ok(())
    }

    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        pallet_assets::Pallet::<Runtime>::mint_into(
            WRAPPED_SOL_ASSET_ID,
            account,
            amount,
        )?;

        Ok(())
    }

    fn balance_of(account: &AccountId) -> u128 {
        pallet_assets::Pallet::<Runtime>::balance(WRAPPED_SOL_ASSET_ID, account)
    }
}
```

### Runtime Configuration

```rust
parameter_types! {
    pub const LocalDomain: u32 = 102; // SOL PBC domain
    pub const MaxBurnAmount: u128 = 10_000 * SOL; // 10k SOL (9 decimals)
    pub const DailyBurnCap: u128 = 100_000 * SOL; // 100k SOL/day
    pub const MinBurnAmount: u128 = 1_000_000; // 0.001 SOL
}
```

---

## BTC PBC Integration

### Overview
BTC PBC handles wrapped Bitcoin with 8 decimals (satoshis).

### Token Operations

```rust
// File: runtime/pbc-btc/src/token_operations.rs

pub struct BtcTokenOperations;

const WRAPPED_BTC_ASSET_ID: u32 = 0;

impl TokenOperations<AccountId> for BtcTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // BTC uses 8 decimals (satoshis)
        // amount is in satoshis

        pallet_assets::Pallet::<Runtime>::burn_from(
            RuntimeOrigin::signed(account.clone()),
            WRAPPED_BTC_ASSET_ID,
            account.clone(),
            amount,
        )?;

        Ok(())
    }

    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        pallet_assets::Pallet::<Runtime>::mint_into(
            WRAPPED_BTC_ASSET_ID,
            account,
            amount,
        )?;

        Ok(())
    }

    fn balance_of(account: &AccountId) -> u128 {
        pallet_assets::Pallet::<Runtime>::balance(WRAPPED_BTC_ASSET_ID, account)
    }
}
```

### Runtime Configuration

```rust
parameter_types! {
    pub const LocalDomain: u32 = 103; // BTC PBC domain
    pub const MaxBurnAmount: u128 = 10 * BTC; // 10 BTC (8 decimals)
    pub const DailyBurnCap: u128 = 100 * BTC; // 100 BTC/day
    pub const MinBurnAmount: u128 = 10_000; // 0.0001 BTC
}
```

---

## USDC PBC Integration

### Overview
USDC PBC provides Circle USDC bridging with 6 decimals.

### Token Operations

```rust
// File: runtime/pbc-usdc/src/token_operations.rs

pub struct UsdcTokenOperations;

const USDC_ASSET_ID: u32 = 0;

impl TokenOperations<AccountId> for UsdcTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // USDC uses 6 decimals
        // amount is in micro-USDC

        pallet_assets::Pallet::<Runtime>::burn_from(
            RuntimeOrigin::signed(account.clone()),
            USDC_ASSET_ID,
            account.clone(),
            amount,
        )?;

        Ok(())
    }

    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        pallet_assets::Pallet::<Runtime>::mint_into(
            USDC_ASSET_ID,
            account,
            amount,
        )?;

        Ok(())
    }

    fn balance_of(account: &AccountId) -> u128 {
        pallet_assets::Pallet::<Runtime>::balance(USDC_ASSET_ID, account)
    }
}
```

### Runtime Configuration with Circle CCTP Integration

```rust
parameter_types! {
    pub const LocalDomain: u32 = 104; // USDC PBC domain
    pub const MaxBurnAmount: u128 = 1_000_000 * USDC; // 1M USDC (6 decimals)
    pub const DailyBurnCap: u128 = 10_000_000 * USDC; // 10M USDC/day
    pub const MinBurnAmount: u128 = 1_000_000; // 1 USDC
}

// USDC PBC can interoperate with Circle's CCTP directly
// because message format is compatible
```

---

## Cross-PBC Transfers

### Example: EDSC → ETH PBC → Ethereum

This example shows how a user can bridge EDSC from EDSC PBC to Ethereum via ETH PBC.

#### Step 1: EDSC PBC → ETH PBC

```rust
// User on EDSC PBC
TokenMessenger::deposit_for_burn(
    Origin::signed(user_account),
    1000 * EDSC, // Amount
    101, // ETH PBC domain
    eth_pbc_recipient_address, // AccountId on ETH PBC
)?;

// Result:
// - 1000 EDSC burned on EDSC PBC
// - Message created with nonce
// - Attesters sign off-chain
// - Relayer delivers to ETH PBC
```

#### Step 2: ETH PBC Receives

```rust
// Relayer on ETH PBC
TokenMessenger::receive_message(
    Origin::signed(relayer),
    encoded_message,
    attestation,
)?;

// Result:
// - 1000 EDSC minted on ETH PBC (as wrapped EDSC asset)
// - User now has EDSC on ETH PBC
```

#### Step 3: ETH PBC → Ethereum

```rust
// User on ETH PBC wants to bridge to Ethereum mainnet
TokenMessenger::deposit_for_burn(
    Origin::signed(user_account),
    1000 * EDSC, // Burn wrapped EDSC
    0, // Ethereum mainnet domain
    ethereum_address_bytes, // 20-byte Ethereum address
)?;

// Result:
// - Wrapped EDSC burned on ETH PBC
// - Message for Ethereum created
// - Smart contract on Ethereum receives and mints EDSC ERC20
```

### Example: Multi-Hop Arbitrage

```rust
// Arbitrage bot moving USDC: Ethereum → USDC PBC → Solana

// Step 1: Ethereum → USDC PBC
// (Done via Ethereum smart contract + attestation)

// Step 2: On USDC PBC, immediately bridge to Solana
TokenMessenger::deposit_for_burn(
    Origin::signed(bot_account),
    100_000 * USDC,
    1, // Solana domain
    solana_address_bytes,
)?;

// Result: Fast cross-chain arbitrage with two hops
```

---

## Common Patterns

### Pattern 1: Token Locking on Source

Some PBCs may want to lock tokens instead of burning:

```rust
pub struct LockingTokenOperations;

impl TokenOperations<AccountId> for LockingTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Transfer to treasury/vault instead of burning
        let treasury = TreasuryAccount::get();
        pallet_balances::Pallet::<Runtime>::transfer(
            account,
            &treasury,
            amount,
        )?;
        Ok(())
    }

    fn mint_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Transfer from treasury/vault
        let treasury = TreasuryAccount::get();
        pallet_balances::Pallet::<Runtime>::transfer(
            &treasury,
            account,
            amount,
        )?;
        Ok(())
    }
}
```

### Pattern 2: Fee Collection

Taking a bridge fee:

```rust
impl TokenOperations<AccountId> for FeeCollectingOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        // Take 0.1% fee
        let fee = amount / 1000;
        let net_amount = amount - fee;

        // Transfer fee to treasury
        pallet_balances::Pallet::<Runtime>::transfer(
            account,
            &TreasuryAccount::get(),
            fee,
        )?;

        // Burn net amount
        pallet_balances::Pallet::<Runtime>::burn_from(
            account,
            net_amount,
        )?;

        Ok(())
    }
}
```

### Pattern 3: Multi-Token Support

Supporting multiple tokens on same PBC:

```rust
pub struct MultiTokenOperations {
    token_id: u32,
}

impl MultiTokenOperations {
    pub fn new(token_id: u32) -> Self {
        Self { token_id }
    }
}

impl TokenOperations<AccountId> for MultiTokenOperations {
    fn burn_tokens(account: &AccountId, amount: u128) -> DispatchResult {
        pallet_assets::Pallet::<Runtime>::burn_from(
            RuntimeOrigin::signed(account.clone()),
            self.token_id, // Different asset per token
            account.clone(),
            amount,
        )
    }

    // ... other methods
}

// Configure multiple messenger instances for different tokens
```

---

## Testing Cross-PBC Transfers

```rust
#[test]
fn test_edsc_to_eth_pbc_transfer() {
    // Setup both runtimes
    EdscPbc::new_test_ext().execute_with(|| {
        // Configure ETH PBC as destination
        TokenMessenger::configure_domain(
            Origin::root(),
            101, // ETH PBC
            true,
            5_000_000 * EDSC,
            20_000_000 * EDSC,
            1 * MILLIUNIT,
        );

        // User burns EDSC
        assert_ok!(TokenMessenger::deposit_for_burn(
            Origin::signed(ALICE),
            1000 * EDSC,
            101,
            bob_on_eth_pbc.encode(),
        ));

        // Get message
        let message = TokenMessenger::outbound_messages(0).unwrap();
        let attestation = collect_attestations(&message);

        // Switch to ETH PBC runtime
        EthPbc::new_test_ext().execute_with(|| {
            // Relayer delivers
            assert_ok!(TokenMessenger::receive_message(
                Origin::signed(RELAYER),
                message.encode(),
                attestation,
            ));

            // Verify BOB received wrapped EDSC
            assert_eq!(
                Assets::balance(WRAPPED_EDSC_ID, &BOB),
                1000 * EDSC
            );
        });
    });
}
```

---

## Deployment Checklist per PBC

- [ ] Implement `TokenOperations` trait for your token
- [ ] Implement `AttestationVerifier` integration
- [ ] Configure `LocalDomain` constant (unique per PBC)
- [ ] Set appropriate rate limits based on token economics
- [ ] Deploy `pallet-bridge-attestation` with attester set
- [ ] Configure supported destination domains
- [ ] Test burn and mint operations
- [ ] Test attestation verification
- [ ] Deploy relayer infrastructure
- [ ] Monitor events and volumes
- [ ] Set up emergency governance procedures

---

## Support

For questions or issues:
- GitHub: https://github.com/etrid/etrid
- Discord: https://discord.gg/etrid
- Documentation: https://docs.etrid.io
