# Etrid Blockchain API Reference

**Version:** 2.0.0
**Last Updated:** 2025-11-16

## Table of Contents

1. [Overview](#overview)
2. [Pallet APIs](#pallet-apis)
   - [pallet-reserve-oracle](#pallet-reserve-oracle)
   - [pallet-reserve-vault](#pallet-reserve-vault)
   - [pallet-circuit-breaker](#pallet-circuit-breaker)
   - [pallet-custodian-registry](#pallet-custodian-registry)
   - [pallet-xcm-bridge](#pallet-xcm-bridge)
   - [pallet-validator-committee](#pallet-validator-committee)
   - [pallet-did-registry](#pallet-did-registry)
   - [pallet-aidid](#pallet-aidid)
   - [pallet-distribution-pay](#pallet-distribution-pay)
   - [pallet-lightning-bloc](#pallet-lightning-bloc)
   - [pallet-etwasm-vm](#pallet-etwasm-vm)
3. [RPC Endpoints](#rpc-endpoints)
4. [TypeScript Type Definitions](#typescript-type-definitions)
5. [Code Examples](#code-examples)

---

## Overview

The Etrid blockchain provides a comprehensive suite of pallets for managing the EDSC stablecoin system, decentralized identity, AI identities, and cross-chain messaging. This document provides detailed API documentation for all extrinsics, storage queries, and RPC endpoints.

### Architecture

- **Primearc Core**: Main execution chain (reserve management, oracles, governance)
- **PBC-EDSC**: Dedicated EDSC partition burst chain
- **Cross-chain**: DETRP2P messaging protocol for checkpoint synchronization

---

## Pallet APIs

### pallet-reserve-oracle

**Purpose:** Aggregates reserve data from vault and custodians for the EDSC system, provides price feeds, and manages reserve snapshots.

#### Extrinsics

##### `update_asset_price`

Update asset price (governance/oracle only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `symbol`: `Vec<u8>` - Asset symbol (max 16 bytes, e.g., "ETR", "BTC", "ETH")
- `price_usd_cents`: `u128` - Price in USD cents with 8 decimals (100,000,000 = $1.00)
- `source`: `Vec<u8>` - Data source identifier (max 32 bytes, e.g., "chainlink")

**Returns:** `DispatchResult`

**Events:**
- `AssetPriceUpdated { symbol, price_usd_cents, source }`

**Errors:**
- `InvalidPriceData` - Invalid symbol or source format

**Example:**
```rust
// Update BTC price to $60,000.00
ReserveOracle::update_asset_price(
    Origin::root(),
    b"BTC".to_vec(),
    6_000_000_000_000, // $60,000 with 8 decimals
    b"chainlink".to_vec()
)?;
```

---

##### `force_snapshot`

Force create reserve snapshot (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root

**Returns:** `DispatchResult`

**Events:**
- `SnapshotCreated { block_number, total_reserves, reserve_ratio }`
- `ReserveDataAggregated { vault_value, custodian_value, total_value }`

**Errors:**
- `MaxSnapshotsReached` - Maximum snapshot limit reached
- `ReserveCalculationOverflow` - Calculation overflow

**Example:**
```rust
ReserveOracle::force_snapshot(Origin::root())?;
```

---

##### `publish_checkpoint`

Publish checkpoint to PBC-EDSC (governance/automated)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `block_number`: `BlockNumberFor<T>` - Block number of snapshot

**Returns:** `DispatchResult`

**Events:**
- `CheckpointPublished { block_number, reserve_ratio }`

**Errors:**
- `SnapshotNotFound` - Snapshot doesn't exist

**Example:**
```rust
ReserveOracle::publish_checkpoint(Origin::root(), 1000)?;
```

---

##### `clear_alert`

Clear reserve ratio alert (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root

**Returns:** `DispatchResult`

**Example:**
```rust
ReserveOracle::clear_alert(Origin::root())?;
```

---

##### `add_trusted_oracle`

Add trusted oracle account

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `oracle`: `T::AccountId` - Oracle account to trust

**Returns:** `DispatchResult`

**Events:**
- `OracleAdded { oracle }`

**Example:**
```rust
ReserveOracle::add_trusted_oracle(
    Origin::root(),
    alice_account
)?;
```

---

##### `remove_trusted_oracle`

Remove trusted oracle account

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `oracle`: `T::AccountId` - Oracle account to remove

**Returns:** `DispatchResult`

**Events:**
- `OracleRemoved { oracle }`

**Example:**
```rust
ReserveOracle::remove_trusted_oracle(
    Origin::root(),
    alice_account
)?;
```

---

##### `submit_price`

Submit price from oracle source (multi-oracle aggregation)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be signed by trusted oracle
- `asset_symbol`: `Vec<u8>` - Asset symbol (max 16 bytes)
- `price`: `u128` - Price in USD cents with 8 decimals
- `source`: `Vec<u8>` - Source name (max 32 bytes)
- `confidence`: `u8` - Confidence level (0-100)

**Returns:** `DispatchResult`

**Events:**
- `PriceSubmitted { oracle, asset_symbol, price, confidence }`
- `PriceAggregated { asset_symbol, median_price, sources_count }`

**Errors:**
- `OracleNotTrusted` - Caller is not a trusted oracle
- `InvalidPrice` - Price must be > 0
- `InvalidConfidence` - Confidence must be 0-100
- `AssetSymbolTooLong` - Symbol exceeds 16 bytes
- `SourceNameTooLong` - Source exceeds 32 bytes

**Example:**
```rust
// Submit ETH price with 95% confidence
ReserveOracle::submit_price(
    Origin::signed(oracle_account),
    b"ETH".to_vec(),
    300_000_000_000, // $3,000 with 8 decimals
    b"chainlink".to_vec(),
    95 // 95% confidence
)?;
```

---

##### `check_price_staleness_manual`

Check for stale prices and trigger failover

**Parameters:**
- `origin`: `OriginFor<T>` - Must be signed
- `asset_symbol`: `Vec<u8>` - Asset symbol to check

**Returns:** `DispatchResult`

**Events:**
- `PriceStale { asset_symbol, age_blocks }`
- `FailoverTriggered { asset_symbol }`

**Errors:**
- `PriceNotFound` - No price data for asset
- `AssetSymbolTooLong` - Symbol exceeds 16 bytes

**Example:**
```rust
ReserveOracle::check_price_staleness_manual(
    Origin::signed(caller),
    b"BTC".to_vec()
)?;
```

---

#### Storage Queries

##### `latest_snapshot() -> Option<ReserveSnapshot>`

Get the latest reserve snapshot

**Returns:**
```rust
ReserveSnapshot {
    block_number: BlockNumber,
    vault_value: u128,        // USD cents
    custodian_value: u128,    // USD cents
    total_reserves: u128,     // USD cents
    total_supply: u128,       // EDSC with 18 decimals
    reserve_ratio: u16,       // Basis points (10000 = 100%)
    timestamp: u64,
}
```

---

##### `snapshots(block_number) -> Option<ReserveSnapshot>`

Get snapshot at specific block number

**Parameters:**
- `block_number`: `BlockNumberFor<T>`

---

##### `asset_prices(symbol) -> Option<AssetPrice>`

Get price for an asset

**Parameters:**
- `symbol`: `BoundedVec<u8, 16>` - Asset symbol

**Returns:**
```rust
AssetPrice {
    symbol: BoundedVec<u8, 16>,
    price_usd_cents: u128,
    last_update: u32,
    source: BoundedVec<u8, 32>,
}
```

---

##### `alert_active() -> bool`

Check if reserve ratio alert is active

---

##### `aggregated_prices(symbol) -> Option<AggregatedPrice>`

Get aggregated price from multiple oracles

**Returns:**
```rust
AggregatedPrice {
    median_price: u128,
    mean_price: u128,
    sources_count: u32,
    timestamp: BlockNumber,
    confidence_score: u8,
}
```

---

##### `trusted_oracles(oracle) -> bool`

Check if account is a trusted oracle

---

#### Helper Functions

```rust
// Get asset price in USD cents
pub fn get_asset_price(symbol: &[u8]) -> Option<u128>

// Get latest reserve ratio
pub fn get_reserve_ratio() -> Option<u16>

// Get total reserve value
pub fn get_total_reserves() -> Option<u128>

// Check if reserve ratio is healthy
pub fn is_reserve_ratio_healthy() -> bool

// Get aggregated price for an asset
pub fn get_aggregated_price(symbol: &[u8]) -> Option<AggregatedPrice<BlockNumber>>

// Check if oracle is trusted
pub fn is_oracle_trusted(oracle: &T::AccountId) -> bool
```

---

### pallet-reserve-vault

**Purpose:** Multi-asset collateral vault for EDSC backing with risk-adjusted valuations (haircuts).

#### Extrinsics

##### `deposit_collateral`

Deposit collateral into vault (anyone can deposit)

**Parameters:**
- `origin`: `OriginFor<T>` - Depositor account (signed)
- `asset_type`: `u8` - Asset type (0=ETR, 1=BTC, 2=ETH, 3=USDC, 4=USDT, 5=DAI)
- `amount`: `u128` - Amount to deposit

**Returns:** `DispatchResult`

**Events:**
- `CollateralDeposited { asset_type, amount, depositor }`
- `ReserveRatioUpdated { ratio }`

**Errors:**
- `AssetNotSupported` - Invalid asset type
- `Overflow` - Arithmetic overflow

**Example:**
```rust
// Deposit 10 BTC
ReserveVault::deposit_collateral(
    Origin::signed(alice),
    1, // BTC
    10_000_000_000_000 // 10 BTC with 12 decimals
)?;
```

---

##### `withdraw_collateral`

Withdraw collateral from vault (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `asset_type`: `u8` - Asset type
- `amount`: `u128` - Amount to withdraw
- `recipient`: `T::AccountId` - Recipient account

**Returns:** `DispatchResult`

**Events:**
- `CollateralWithdrawn { asset_type, amount, recipient }`
- `ReserveRatioUpdated { ratio }`

**Errors:**
- `AssetNotSupported` - Invalid asset type
- `InsufficientVaultBalance` - Not enough balance
- `ReserveRatioTooLow` - Would break reserve ratio

**Example:**
```rust
ReserveVault::withdraw_collateral(
    Origin::root(),
    2, // ETH
    5_000_000_000_000, // 5 ETH
    bob_account
)?;
```

---

##### `update_asset_price`

Update asset price (governance or oracle)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `asset_type`: `u8` - Asset type
- `price`: `u128` - New price in USD cents

**Returns:** `DispatchResult`

**Events:**
- `AssetPriceUpdated { asset_type, price }`
- `ReserveRatioUpdated { ratio }`

**Example:**
```rust
// Update BTC price to $60,000
ReserveVault::update_asset_price(
    Origin::root(),
    1, // BTC
    6_000_000 // $60,000 in USD cents
)?;
```

---

##### `update_haircut`

Update haircut for asset (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `asset_type`: `u8` - Asset type
- `haircut`: `Permill` - New haircut percentage

**Returns:** `DispatchResult`

**Events:**
- `HaircutUpdated { asset_type, haircut }`
- `ReserveRatioUpdated { ratio }`

**Example:**
```rust
use sp_arithmetic::Permill;

// Set BTC haircut to 15%
ReserveVault::update_haircut(
    Origin::root(),
    1, // BTC
    Permill::from_percent(15)
)?;
```

---

##### `update_custodian_value`

Update custodian-attested value (custodian registry pallet only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `value`: `u128` - New attested value in USD cents

**Returns:** `DispatchResult`

**Events:**
- `CustodianValueUpdated { value }`
- `ReserveRatioUpdated { ratio }`

**Example:**
```rust
// Update custodian value to $50M
ReserveVault::update_custodian_value(
    Origin::root(),
    5_000_000_000 // $50M in cents
)?;
```

---

#### Storage Queries

##### `vault(asset_type) -> Option<VaultEntry>`

Get vault entry for an asset

**Returns:**
```rust
VaultEntry {
    raw_balance: u128,
    haircut: Permill,
    usd_value: u128,
    adjusted_value: u128, // After haircut
}
```

---

##### `custodian_value() -> u128`

Get custodian-attested off-chain reserve value

---

##### `reserve_ratio() -> FixedU128`

Get current reserve ratio (cached)

---

##### `haircut(asset_type) -> Permill`

Get haircut configuration for asset

---

##### `asset_price(asset_type) -> u128`

Get USD price for asset (in cents)

---

#### Helper Functions

```rust
// Calculate reserve ratio
pub fn calculate_reserve_ratio() -> Result<FixedU128, DispatchError>

// Get current reserve ratio (for external pallets)
pub fn get_reserve_ratio() -> FixedU128

// Get total reserves value
pub fn get_total_reserves() -> Result<u128, DispatchError>

// Internal helper to update custodian value
pub fn do_update_custodian_value(value: u128) -> DispatchResult

// Internal helper to execute payout (called by redemption pallet)
pub fn do_payout(recipient: &T::AccountId, usd_amount: u128) -> DispatchResult
```

---

### pallet-circuit-breaker

**Purpose:** Emergency safety controls to protect the EDSC system from excessive volumes and rapid reserve changes.

#### Extrinsics

##### `activate_manual_pause`

Manually pause the circuit (governance/root only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root

**Returns:** `DispatchResult`

**Events:**
- `ManualPauseActivated`
- `StatusChanged { old_status, new_status }`

**Example:**
```rust
CircuitBreaker::activate_manual_pause(Origin::root())?;
```

---

##### `resume`

Resume operations (governance/root only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root

**Returns:** `DispatchResult`

**Events:**
- `ManualPauseDeactivated`
- `StatusChanged { old_status, new_status }`
- `CircuitReset`

**Example:**
```rust
CircuitBreaker::resume(Origin::root())?;
```

---

##### `add_to_whitelist`

Add account to whitelist (governance/root only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `account`: `T::AccountId` - Account to whitelist

**Returns:** `DispatchResult`

**Events:**
- `AccountWhitelisted { account }`

**Example:**
```rust
CircuitBreaker::add_to_whitelist(
    Origin::root(),
    treasury_account
)?;
```

---

##### `remove_from_whitelist`

Remove account from whitelist (governance/root only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `account`: `T::AccountId` - Account to remove

**Returns:** `DispatchResult`

**Events:**
- `AccountRemovedFromWhitelist { account }`

**Example:**
```rust
CircuitBreaker::remove_from_whitelist(
    Origin::root(),
    old_account
)?;
```

---

##### `reset_circuit`

Reset circuit breaker (governance/root only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root

**Returns:** `DispatchResult`

**Events:**
- `StatusChanged { old_status, new_status }`
- `CircuitReset`

**Example:**
```rust
CircuitBreaker::reset_circuit(Origin::root())?;
```

---

#### Storage Queries

##### `circuit_status() -> CircuitStatus`

Get current circuit breaker status

**Returns:**
- `Normal` (0)
- `Throttled` (1)
- `Paused` (2)
- `Emergency` (3)

---

##### `volume_tracker() -> VolumeTracker`

Get volume tracker for redemptions

**Returns:**
```rust
VolumeTracker {
    hourly_volume: u128,
    hourly_start_block: BlockNumber,
    daily_volume: u128,
    daily_start_block: BlockNumber,
}
```

---

##### `is_manually_paused() -> bool`

Check if manual pause is enabled

---

##### `whitelisted(account) -> bool`

Check if account is whitelisted

---

##### `trigger_count() -> u32`

Get total number of times circuit was triggered

---

#### Helper Functions

```rust
// Check if an operation is allowed
pub fn is_operation_allowed(account: &T::AccountId, amount: u128) -> DispatchResult

// Track volume for an operation
pub fn track_volume(amount: u128) -> DispatchResult

// Check reserve ratio and update circuit status
pub fn check_reserve_ratio(reserve_ratio: u16) -> DispatchResult

// Get current circuit status
pub fn get_status() -> CircuitStatus

// Check if account is whitelisted
pub fn is_whitelisted(account: &T::AccountId) -> bool
```

---

### pallet-custodian-registry

**Purpose:** Registry for off-chain reserve custodians with bonding, attestation, and slashing mechanisms.

#### Extrinsics

##### `register_custodian`

Register as custodian (requires bond)

**Parameters:**
- `origin`: `OriginFor<T>` - Custodian account (signed)
- `bond_amount`: `BalanceOf<T>` - Security deposit amount
- `license_proof`: `BoundedVec<u8, 256>` - Regulatory license proof (hash/CID)

**Returns:** `DispatchResult`

**Events:**
- `CustodianRegistered { custodian_id, account, bond_amount }`

**Errors:**
- `AlreadyRegistered` - Account already registered
- `InsufficientBond` - Bond below minimum

**Example:**
```rust
CustodianRegistry::register_custodian(
    Origin::signed(custodian_account),
    1_000_000_000_000, // 1M ETR bond
    b"ipfs://QmHash123...".to_vec().try_into().unwrap()
)?;
```

---

##### `approve_custodian`

Approve custodian (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `custodian_id`: `u64` - Custodian to approve

**Returns:** `DispatchResult`

**Events:**
- `CustodianApproved { custodian_id }`

**Errors:**
- `CustodianNotFound` - Custodian doesn't exist
- `NotActive` - Custodian not in pending status

**Example:**
```rust
CustodianRegistry::approve_custodian(
    Origin::root(),
    0 // custodian_id
)?;
```

---

##### `submit_attestation`

Submit reserve attestation (active custodians only)

**Parameters:**
- `origin`: `OriginFor<T>` - Custodian account (signed)
- `reserve_value`: `u128` - Attested reserve value (USD cents)
- `proof`: `BoundedVec<u8, 512>` - Proof of reserves (audit report, etc.)
- `auditor_signature`: `BoundedVec<u8, 256>` - Third-party auditor signature

**Returns:** `DispatchResult`

**Events:**
- `AttestationSubmitted { custodian_id, reserve_value }`

**Errors:**
- `CustodianNotFound` - Custodian doesn't exist
- `NotActive` - Custodian not active
- `AttestationTooEarly` - Too soon since last attestation

**Example:**
```rust
CustodianRegistry::submit_attestation(
    Origin::signed(custodian_account),
    5_000_000_000, // $50M in cents
    b"ipfs://QmAuditReport...".to_vec().try_into().unwrap(),
    b"0x1234567890abcdef...".to_vec().try_into().unwrap()
)?;
```

---

##### `suspend_custodian`

Suspend custodian (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `custodian_id`: `u64` - Custodian to suspend
- `reason`: `BoundedVec<u8, 256>` - Reason for suspension

**Returns:** `DispatchResult`

**Events:**
- `CustodianSuspended { custodian_id, reason }`

**Example:**
```rust
CustodianRegistry::suspend_custodian(
    Origin::root(),
    0,
    b"Failed audit verification".to_vec().try_into().unwrap()
)?;
```

---

##### `slash_custodian`

Slash custodian bond (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `custodian_id`: `u64` - Custodian to slash

**Returns:** `DispatchResult`

**Events:**
- `CustodianSlashed { custodian_id, slash_amount }`

**Example:**
```rust
CustodianRegistry::slash_custodian(
    Origin::root(),
    0 // custodian_id
)?;
```

---

##### `remove_custodian`

Remove custodian (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `custodian_id`: `u64` - Custodian to remove

**Returns:** `DispatchResult`

**Events:**
- `CustodianRemoved { custodian_id }`

**Example:**
```rust
CustodianRegistry::remove_custodian(
    Origin::root(),
    0 // custodian_id
)?;
```

---

#### Storage Queries

##### `custodians(custodian_id) -> Option<CustodianInfo>`

Get custodian information

**Returns:**
```rust
CustodianInfo {
    account: AccountId,
    bond_amount: Balance,
    license_proof: BoundedVec<u8, 256>,
    last_attestation: BlockNumber,
    status: CustodianStatus, // Pending, Active, Suspended, Slashed, Removed
    attested_value: u128,
    missed_attestations: u32,
}
```

---

##### `custodian_id_of(account) -> Option<u64>`

Get custodian ID for account

---

##### `attestations(custodian_id) -> BoundedVec<Attestation, 100>`

Get attestation history for custodian

---

#### Helper Functions

```rust
// Get total attested value (for external queries)
pub fn get_total_attested_value() -> u128

// Get custodian count by status
pub fn get_custodian_count_by_status(status: CustodianStatus) -> u32
```

---

### pallet-xcm-bridge

**Purpose:** Cross-chain messaging bridge using DETRP2P protocol for checkpoint synchronization.

#### Extrinsics

##### `send_checkpoint`

Send checkpoint to destination chain

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `destination_chain`: `u8` - Destination (0=Primearc Core, 1=PbcEdsc, 2+=Other)
- `reserve_ratio`: `u16` - Reserve ratio (basis points)
- `total_reserves`: `u128` - Total reserves (USD cents)
- `vault_value`: `u128` - Vault value (USD cents)
- `custodian_value`: `u128` - Custodian value (USD cents)
- `total_supply`: `u128` - Total EDSC supply

**Returns:** `DispatchResult`

**Events:**
- `MessageQueued { nonce, destination_chain, message_type }`
- `MessageSent { nonce }`

**Errors:**
- `PayloadTooLarge` - Message payload exceeds maximum size
- `Detrp2pNotConnected` - DETRP2P layer not connected

**Example:**
```rust
XcmBridge::send_checkpoint(
    Origin::root(),
    1, // PbcEdsc
    11500, // 115% reserve ratio
    57_500_000_000, // $575M reserves
    50_000_000_000, // $500M vault
    7_500_000_000,  // $75M custodian
    50_000_000_000_000_000_000_000_000_000 // 50B EDSC
)?;
```

---

##### `receive_checkpoint`

Receive and process checkpoint from Primearc Core

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `source_chain`: `u8` - Source (0=Primearc Core, 1=PbcEdsc, 2+=Other)
- `nonce`: `u64` - Message nonce
- `reserve_ratio`: `u16`
- `total_reserves`: `u128`
- `vault_value`: `u128`
- `custodian_value`: `u128`
- `total_supply`: `u128`

**Returns:** `DispatchResult`

**Events:**
- `MessageReceived { source_chain, message_type, nonce }`
- `CheckpointProcessed { block_number, reserve_ratio }`

**Errors:**
- `MessageAlreadyProcessed` - Checkpoint already received

**Example:**
```rust
XcmBridge::receive_checkpoint(
    Origin::root(),
    0, // Primearc Core
    42, // nonce
    11500, // 115%
    57_500_000_000,
    50_000_000_000,
    7_500_000_000,
    50_000_000_000_000_000_000_000_000_000
)?;
```

---

##### `mark_message_sent`

Mark message as sent (called by DETRP2P layer)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `nonce`: `u64` - Message nonce

**Returns:** `DispatchResult`

**Events:**
- `MessageSent { nonce }`

**Errors:**
- `MessageNotFound` - Message doesn't exist

---

##### `set_connection_status`

Update DETRP2P connection status

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `connected`: `bool` - Connection status

**Returns:** `DispatchResult`

**Events:**
- `ConnectionStatusChanged { connected }`

**Example:**
```rust
XcmBridge::set_connection_status(Origin::root(), true)?;
```

---

##### `cleanup_messages`

Clean up processed messages

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `nonce`: `u64` - Message nonce to cleanup

**Returns:** `DispatchResult`

**Example:**
```rust
XcmBridge::cleanup_messages(Origin::root(), 42)?;
```

---

#### Storage Queries

##### `pending_messages(nonce) -> Option<CrossChainMessage>`

Get pending outbound message

---

##### `message_status(nonce) -> MessageStatus`

Get message status (Pending, Sent, Received, Processed, Failed)

---

##### `message_nonce() -> u64`

Get current message nonce counter

---

##### `received_messages(hash) -> Option<BlockNumber>`

Get block when message was received (for deduplication)

---

##### `total_sent() -> u64`

Get total messages sent

---

##### `total_received() -> u64`

Get total messages received

---

##### `detrp2p_connected() -> bool`

Get DETRP2P connection status

---

#### Helper Functions

```rust
// Get pending message count
pub fn get_pending_count() -> u32

// Check if DETRP2P is connected
pub fn is_connected() -> bool

// Get message by nonce
pub fn get_message(nonce: u64) -> Option<CrossChainMessage<BlockNumber>>

// Get message status
pub fn get_status(nonce: u64) -> MessageStatus
```

---

### pallet-validator-committee

**Purpose:** Manages the ASF validator committee for consensus and block production.

#### Extrinsics

##### `add_validator`

Add a validator to the committee

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `validator_id`: `ValidatorId` (AccountId32) - Validator account
- `stake`: `Balance` (u128) - Validator stake amount
- `peer_type`: `u8` - Peer type (0=ValidityNode, 1=FlareNode, 2=DecentralizedDirector, 3=StakingCommon, 4=Common)

**Returns:** `DispatchResult`

**Events:**
- `ValidatorAdded { validator_id }`

**Errors:**
- `InsufficientStake` - Stake below minimum
- `ValidatorAlreadyExists` - Validator already in committee
- `CommitteeFull` - Committee at maximum capacity

**Example:**
```rust
use sp_core::crypto::AccountId32;

ValidatorCommittee::add_validator(
    Origin::root(),
    AccountId32::from([1u8; 32]),
    5_000_000_000_000, // 5M ETR stake
    0 // ValidityNode
)?;
```

---

##### `remove_validator`

Remove a validator from the committee

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `validator_id`: `ValidatorId` - Validator to remove

**Returns:** `DispatchResult`

**Events:**
- `ValidatorRemoved { validator_id }`

**Errors:**
- `ValidatorNotFound` - Validator doesn't exist

**Example:**
```rust
ValidatorCommittee::remove_validator(
    Origin::root(),
    AccountId32::from([1u8; 32])
)?;
```

---

##### `rotate_committee`

Rotate committee for new epoch

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root

**Returns:** `DispatchResult`

**Events:**
- `CommitteeRotated { epoch, committee_size }`

**Example:**
```rust
ValidatorCommittee::rotate_committee(Origin::root())?;
```

---

#### Storage Queries

##### `validators(validator_id) -> Option<StoredValidatorInfo>`

Get validator information

**Returns:**
```rust
StoredValidatorInfo {
    validator_id: ValidatorId,
    stake: Balance,
    peer_type: u8,
}
```

---

##### `committee() -> BoundedVec<ValidatorId, MaxCommitteeSize>`

Get current active committee members (ordered list)

---

##### `current_epoch() -> Epoch (u64)`

Get current epoch number

---

##### `committee_size_limit() -> u32`

Get committee size limit

---

##### `next_epoch_validators() -> BoundedVec<ValidatorId, MaxCommitteeSize>`

Get validators for next epoch

---

##### `ppfa_history((block_number, ppfa_index)) -> Option<ValidatorId>`

Get validator authorized for specific PPFA slot

---

##### `epoch_duration() -> BlockNumber`

Get epoch duration in blocks

---

#### Helper Functions

```rust
// Get all active committee members
pub fn get_committee() -> Vec<ValidatorInfo>

// Get specific validator info
pub fn get_validator(validator_id: &ValidatorId) -> Option<ValidatorInfo>

// Check if validator is in active committee
pub fn is_validator_active(validator_id: &ValidatorId) -> bool

// Get current epoch
pub fn get_current_epoch() -> Epoch

// Record PPFA authorization for a block
pub fn record_ppfa_authorization(
    block_number: BlockNumberFor<T>,
    ppfa_index: u32,
    proposer_id: ValidatorId,
)

// Check if proposer was authorized
pub fn is_proposer_authorized(
    block_number: BlockNumberFor<T>,
    ppfa_index: u32,
    proposer_id: &ValidatorId,
) -> bool

// Get next epoch start block
pub fn next_epoch_start() -> BlockNumber

// Get validators for next epoch
pub fn get_next_epoch_validators() -> Vec<ValidatorInfo>

// Set epoch duration
pub fn set_epoch_duration(duration: BlockNumber)

// Get epoch duration
pub fn get_epoch_duration() -> BlockNumber
```

---

### pallet-did-registry

**Purpose:** W3C DID-compliant decentralized identity management for Etrid blockchain.

#### Extrinsics

##### `register_did`

Register a new DID

**Parameters:**
- `origin`: `OriginFor<T>` - Owner account (signed)
- `did_identifier`: `Vec<u8>` - DID identifier (max 64 bytes, e.g., "did:etrid:alice")
- `controller`: `T::AccountId` - Controller account (can be different from owner)
- `document_hash`: `Vec<u8>` - Hash of full DID document (max 64 bytes)

**Returns:** `DispatchResult`

**Events:**
- `DidRegistered { did_hash, owner }`

**Errors:**
- `DidAlreadyExists` - DID already registered
- `DidIdentifierTooLong` - Identifier exceeds 64 bytes
- `DocumentHashTooLong` - Hash exceeds 64 bytes
- `TooManyAccessEntries` - Owner has too many DIDs

**Example:**
```rust
DidRegistry::register_did(
    Origin::signed(alice),
    b"did:etrid:alice".to_vec(),
    bob, // controller
    b"QmHash123...".to_vec() // IPFS hash
)?;
```

---

##### `update_did`

Update DID document hash

**Parameters:**
- `origin`: `OriginFor<T>` - Must be owner or controller
- `did_hash`: `H256` - DID hash
- `new_document_hash`: `Vec<u8>` - New document hash

**Returns:** `DispatchResult`

**Events:**
- `DidUpdated { did_hash, updated_by }`

**Errors:**
- `DidNotFound` - DID doesn't exist
- `NotAuthorized` - Not owner or controller
- `DidRevoked` - DID is revoked
- `DocumentHashTooLong` - Hash exceeds 64 bytes

**Example:**
```rust
let did_hash = DidRegistry::hash_did(b"did:etrid:alice");
DidRegistry::update_did(
    Origin::signed(alice),
    did_hash,
    b"QmNewHash456...".to_vec()
)?;
```

---

##### `revoke_did`

Revoke a DID

**Parameters:**
- `origin`: `OriginFor<T>` - Must be owner
- `did_hash`: `H256` - DID hash

**Returns:** `DispatchResult`

**Events:**
- `DidRevoked { did_hash, revoked_by }`

**Errors:**
- `DidNotFound` - DID doesn't exist
- `NotAuthorized` - Not the owner

**Example:**
```rust
DidRegistry::revoke_did(
    Origin::signed(alice),
    did_hash
)?;
```

---

##### `transfer_ownership`

Transfer DID ownership

**Parameters:**
- `origin`: `OriginFor<T>` - Must be owner
- `did_hash`: `H256` - DID hash
- `new_owner`: `T::AccountId` - New owner account

**Returns:** `DispatchResult`

**Events:**
- `OwnershipTransferred { did_hash, old_owner, new_owner }`

**Errors:**
- `DidNotFound` - DID doesn't exist
- `NotAuthorized` - Not the owner
- `SameOwner` - New owner same as current

**Example:**
```rust
DidRegistry::transfer_ownership(
    Origin::signed(alice),
    did_hash,
    charlie
)?;
```

---

##### `set_expiration`

Set DID expiration

**Parameters:**
- `origin`: `OriginFor<T>` - Must be owner or controller
- `did_hash`: `H256` - DID hash
- `expires_at`: `BlockNumberFor<T>` - Expiration block

**Returns:** `DispatchResult`

**Events:**
- `ExpirationSet { did_hash, expires_at }`

**Errors:**
- `DidNotFound` - DID doesn't exist
- `NotAuthorized` - Not owner or controller
- `InvalidExpiration` - Expiration in the past

**Example:**
```rust
DidRegistry::set_expiration(
    Origin::signed(alice),
    did_hash,
    current_block + 1_000_000 // Expire in ~1M blocks
)?;
```

---

##### `grant_access`

Grant access to an agent

**Parameters:**
- `origin`: `OriginFor<T>` - Must be owner
- `did_hash`: `H256` - DID hash
- `agent`: `T::AccountId` - Agent account
- `level`: `AccessLevel` - Access level (None, Reader, Writer, Admin)

**Returns:** `DispatchResult`

**Events:**
- `AccessGranted { did_hash, agent, level }`

**Errors:**
- `DidNotFound` - DID doesn't exist
- `NotAuthorized` - Not the owner

**Example:**
```rust
DidRegistry::grant_access(
    Origin::signed(alice),
    did_hash,
    charlie,
    AccessLevel::Reader
)?;
```

---

##### `revoke_access`

Revoke access from an agent

**Parameters:**
- `origin`: `OriginFor<T>` - Must be owner
- `did_hash`: `H256` - DID hash
- `agent`: `T::AccountId` - Agent account

**Returns:** `DispatchResult`

**Events:**
- `AccessRevoked { did_hash, agent }`

**Errors:**
- `DidNotFound` - DID doesn't exist
- `NotAuthorized` - Not the owner

**Example:**
```rust
DidRegistry::revoke_access(
    Origin::signed(alice),
    did_hash,
    charlie
)?;
```

---

#### Storage Queries

##### `registrations(did_hash) -> Option<DidRegistration>`

Get DID registration

**Returns:**
```rust
DidRegistration {
    did: BoundedVec<u8, 64>,
    owner: AccountId,
    controller: AccountId,
    document_hash: BoundedVec<u8, 64>,
    registered_at: BlockNumber,
    updated_at: BlockNumber,
    expires_at: Option<BlockNumber>,
    revoked: bool,
}
```

---

##### `owner_dids(owner) -> BoundedVec<H256, MaxAccessControlEntries>`

Get DIDs owned by account

---

##### `access_control(did_hash, agent) -> Option<AccessControl>`

Get access control entry

---

##### `total_dids() -> u64`

Get total number of registered DIDs

---

##### `nonce() -> u64`

Get nonce counter

---

#### Helper Functions

```rust
// Hash DID identifier for storage key
pub fn hash_did(identifier: &[u8]) -> H256

// Resolve DID (get registration)
pub fn resolve_did(did_hash: H256) -> Option<DidRegistration<T>>

// Check if DID is active
pub fn is_did_active(did_hash: H256) -> bool

// Check access level for agent
pub fn check_access(did_hash: H256, agent: &T::AccountId) -> AccessLevel

// Get DIDs owned by account
pub fn get_owner_dids(owner: &T::AccountId) -> Vec<H256>

// Get active DIDs count
pub fn active_dids_count() -> u64
```

---

### pallet-aidid

**Purpose:** World's first AI DID standard - decentralized identities for AI agents, models, and systems.

#### Extrinsics

##### `register_ai`

Register a new AI identity

**Parameters:**
- `origin`: `OriginFor<T>` - Controller account (signed)
- `identifier`: `Vec<u8>` - AI identifier (max 64 bytes, e.g., "gpt-etrid-v1")
- `ai_type`: `AIType` - AI type (LLM, VisionModel, AudioModel, MultiModal, ReinforcementLearning, AgentSystem, Other)
- `profile`: `AIProfile` - AI profile including capabilities, safety, version

**Returns:** `DispatchResult`

**Events:**
- `AIRegistered { did_hash, controller, ai_type }`

**Errors:**
- `AIAlreadyExists` - AI already registered
- `IdentifierTooLong` - Identifier exceeds 64 bytes
- `NoCapabilities` - Missing required capabilities
- `NoModalities` - Missing input/output modalities
- `InvalidContextSize` - Invalid context size
- `VersionRequired` - Version not specified
- `AlignmentMethodRequired` - Alignment method not specified
- `InvalidToxicityScore` - Toxicity score out of range
- `TooManyAIs` - Controller has too many AIs

**Example:**
```rust
AIDID::register_ai(
    Origin::signed(controller),
    b"gpt-etrid-v1".to_vec(),
    AIType::LLM,
    ai_profile // See AIProfile structure below
)?;
```

---

##### `update_profile`

Update AI profile

**Parameters:**
- `origin`: `OriginFor<T>` - Must be controller
- `did_hash`: `H256` - AI DID hash
- `new_profile`: `AIProfile` - New profile

**Returns:** `DispatchResult`

**Events:**
- `AIUpdated { did_hash, updated_by }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `NotController` - Not the controller
- (Same validation errors as register_ai)

**Example:**
```rust
AIDID::update_profile(
    Origin::signed(controller),
    ai_did_hash,
    updated_profile
)?;
```

---

##### `attest_model`

Cryptographically attest AI model

**Parameters:**
- `origin`: `OriginFor<T>` - Must be controller
- `did_hash`: `H256` - AI DID hash
- `attestation`: `ModelAttestation` - Model attestation data

**Returns:** `DispatchResult`

**Events:**
- `ModelAttested { did_hash, model_hash }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `NotController` - Not the controller
- `InvalidAttestation` - Invalid attestation data
- `InvalidBenchmarkScore` - Benchmark score out of range

**Example:**
```rust
AIDID::attest_model(
    Origin::signed(controller),
    ai_did_hash,
    ModelAttestation {
        model_hash: [1u8; 32],
        training_data_hash: b"QmTrainingData...".to_vec().try_into().unwrap(),
        version: b"v1.0.0".to_vec().try_into().unwrap(),
        training_date: 1634567890,
        reproducible: true,
        benchmarks: vec![
            Benchmark {
                name: b"MMLU".to_vec().try_into().unwrap(),
                score: 8670, // 86.70%
            }
        ].try_into().unwrap(),
    }
)?;
```

---

##### `grant_permission`

Grant permission to AI

**Parameters:**
- `origin`: `OriginFor<T>` - Must be controller
- `did_hash`: `H256` - AI DID hash
- `permission`: `Permission` - Permission to grant

**Returns:** `DispatchResult`

**Events:**
- `PermissionGranted { did_hash, permission_hash }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `NotController` - Not the controller

**Example:**
```rust
AIDID::grant_permission(
    Origin::signed(controller),
    ai_did_hash,
    Permission {
        action: b"read".to_vec().try_into().unwrap(),
        resource: b"/data/users".to_vec().try_into().unwrap(),
        conditions: vec![].try_into().unwrap(),
    }
)?;
```

---

##### `revoke_permission`

Revoke permission from AI

**Parameters:**
- `origin`: `OriginFor<T>` - Must be controller
- `did_hash`: `H256` - AI DID hash
- `permission_hash`: `H256` - Permission hash

**Returns:** `DispatchResult`

**Events:**
- `PermissionRevoked { did_hash, permission_hash }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `NotController` - Not the controller

**Example:**
```rust
AIDID::revoke_permission(
    Origin::signed(controller),
    ai_did_hash,
    permission_hash
)?;
```

---

##### `record_inference`

Record AI inference execution

**Parameters:**
- `origin`: `OriginFor<T>` - Any signed account
- `did_hash`: `H256` - AI DID hash
- `success`: `bool` - Whether inference succeeded

**Returns:** `DispatchResult`

**Events:**
- `InferenceRecorded { did_hash, success }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `AIDeactivated` - AI is deactivated

**Example:**
```rust
AIDID::record_inference(
    Origin::signed(user),
    ai_did_hash,
    true // successful inference
)?;
```

---

##### `submit_rating`

Submit user rating for AI

**Parameters:**
- `origin`: `OriginFor<T>` - Any signed account
- `did_hash`: `H256` - AI DID hash
- `rating`: `u32` - Rating (0-10000, where 10000 = 100%)

**Returns:** `DispatchResult`

**Events:**
- `RatingSubmitted { did_hash, rating }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `InvalidRating` - Rating out of range (0-10000)

**Example:**
```rust
AIDID::submit_rating(
    Origin::signed(user),
    ai_did_hash,
    9500 // 95% rating
)?;
```

---

##### `deactivate_ai`

Deactivate AI

**Parameters:**
- `origin`: `OriginFor<T>` - Must be controller
- `did_hash`: `H256` - AI DID hash

**Returns:** `DispatchResult`

**Events:**
- `AIDeactivated { did_hash }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `NotController` - Not the controller

**Example:**
```rust
AIDID::deactivate_ai(
    Origin::signed(controller),
    ai_did_hash
)?;
```

---

##### `reactivate_ai`

Reactivate AI

**Parameters:**
- `origin`: `OriginFor<T>` - Must be controller
- `did_hash`: `H256` - AI DID hash

**Returns:** `DispatchResult`

**Events:**
- `AIReactivated { did_hash }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `NotController` - Not the controller

**Example:**
```rust
AIDID::reactivate_ai(
    Origin::signed(controller),
    ai_did_hash
)?;
```

---

##### `update_pricing`

Update AI pricing model

**Parameters:**
- `origin`: `OriginFor<T>` - Must be controller
- `did_hash`: `H256` - AI DID hash
- `pricing`: `PricingModel` - New pricing model

**Returns:** `DispatchResult`

**Events:**
- `PricingUpdated { did_hash }`

**Errors:**
- `AINotFound` - AI doesn't exist
- `NotController` - Not the controller

**Example:**
```rust
AIDID::update_pricing(
    Origin::signed(controller),
    ai_did_hash,
    PricingModel {
        model_type: PricingType::PerToken,
        base_price: 1000, // In smallest unit
        volume_discount: Some(Permill::from_percent(10)),
    }
)?;
```

---

#### Storage Queries

##### `ai_identity(did_hash) -> Option<AIIdentity>`

Get AI identity

**Returns:**
```rust
AIIdentity {
    identifier: BoundedVec<u8, 64>,
    ai_type: AIType,
    controller: AccountId,
    profile: AIProfile,
    attestation: Option<ModelAttestation>,
    pricing: Option<PricingModel>,
    registered_at: BlockNumber,
    updated_at: BlockNumber,
    active: bool,
}
```

---

##### `controller_ais(controller) -> BoundedVec<H256, MaxAIsPerController>`

Get AIs owned by controller

---

##### `ai_reputation(did_hash) -> Reputation`

Get AI reputation

**Returns:**
```rust
Reputation {
    total_inferences: u64,
    successful_inferences: u64,
    user_rating: u32,      // 0-10000
    rating_count: u32,
}
```

---

##### `ai_permissions(did_hash, permission_hash) -> Option<Permission>`

Get AI permission

---

##### `total_ais() -> u64`

Get total number of registered AIs

---

#### Helper Functions

```rust
// Hash DID identifier for storage key
pub fn hash_identifier(identifier: &[u8], ai_type: &AIType) -> H256

// Hash permission for storage
pub fn hash_permission(permission: &Permission) -> H256

// Get AI identity by hash
pub fn get_ai_identity(did_hash: H256) -> Option<AIIdentity<T>>

// Check if AI has permission
pub fn has_permission(did_hash: H256, action: &[u8], resource: &[u8]) -> bool

// Get AI reputation
pub fn get_reputation(did_hash: H256) -> Reputation

// Get controller's AIs
pub fn get_controller_ais(controller: &T::AccountId) -> Vec<H256>

// Check if AI is active
pub fn is_active(did_hash: H256) -> bool
```

---

## RPC Endpoints

Etrid provides standard Substrate RPC endpoints plus custom endpoints for querying pallet state.

### Standard Substrate RPC

```bash
# Chain information
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' \
  http://localhost:9933

# Latest block
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' \
  http://localhost:9933

# Block hash at height
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlockHash", "params": [1000]}' \
  http://localhost:9933
```

### State Queries

All pallet storage can be queried via `state_getStorage` RPC:

```bash
# Get latest reserve snapshot
curl -H "Content-Type: application/json" \
  -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method": "state_call",
    "params": ["ReserveOracleApi_latest_snapshot", "0x"]
  }' \
  http://localhost:9933

# Get reserve ratio
curl -H "Content-Type: application/json" \
  -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method": "state_call",
    "params": ["ReserveVaultApi_reserve_ratio", "0x"]
  }' \
  http://localhost:9933

# Get validator committee
curl -H "Content-Type: application/json" \
  -d '{
    "id":1,
    "jsonrpc":"2.0",
    "method": "state_call",
    "params": ["ValidatorCommitteeApi_get_committee", "0x"]
  }' \
  http://localhost:9933
```

### Custom Runtime APIs

The following Runtime APIs are available (defined in runtime/src/lib.rs):

#### ValidatorCommitteeApi

```rust
// Get all active committee members
fn get_committee() -> Vec<ValidatorInfo>

// Get specific validator info
fn get_validator(validator_id: ValidatorId) -> Option<ValidatorInfo>

// Check if validator is in committee
fn is_in_committee(validator_id: ValidatorId) -> bool

// Get current epoch number
fn current_epoch() -> Epoch

// Get next epoch start block
fn next_epoch_start() -> BlockNumber

// Get validators for next epoch
fn get_next_epoch_validators() -> Vec<ValidatorInfo>

// Check if proposer was authorized
fn is_proposer_authorized(
    block_number: BlockNumber,
    ppfa_index: u32,
    proposer_id: ValidatorId,
) -> bool

// Get epoch duration
fn epoch_duration() -> BlockNumber
```

---

## TypeScript Type Definitions

### Common Types

```typescript
// Block number
type BlockNumber = number;

// Account ID (SS58 address)
type AccountId = string;

// Hash (H256)
type Hash = string;

// Balance (u128)
type Balance = string;

// Fixed point number
type FixedU128 = string;

// Permill (parts per million)
type Permill = number;
```

### pallet-reserve-oracle

```typescript
interface ReserveSnapshot {
  blockNumber: BlockNumber;
  vaultValue: string;        // u128 (USD cents)
  custodianValue: string;    // u128 (USD cents)
  totalReserves: string;     // u128 (USD cents)
  totalSupply: string;       // u128 (EDSC with 18 decimals)
  reserveRatio: number;      // u16 (basis points)
  timestamp: number;         // u64
}

interface AssetPrice {
  symbol: string;            // Max 16 bytes
  priceUsdCents: string;     // u128
  lastUpdate: number;        // u32
  source: string;            // Max 32 bytes
}

interface AggregatedPrice {
  medianPrice: string;       // u128
  meanPrice: string;         // u128
  sourcesCount: number;      // u32
  timestamp: BlockNumber;
  confidenceScore: number;   // u8 (0-100)
}

interface PriceData {
  price: string;             // u128
  timestamp: BlockNumber;
  source: string;            // Max 32 bytes
  confidence: number;        // u8 (0-100)
}
```

### pallet-reserve-vault

```typescript
enum AssetType {
  ETR = 0,
  BTC = 1,
  ETH = 2,
  USDC = 3,
  USDT = 4,
  DAI = 5,
}

interface VaultEntry {
  rawBalance: string;        // u128
  haircut: Permill;
  usdValue: string;          // u128
  adjustedValue: string;     // u128
}
```

### pallet-circuit-breaker

```typescript
enum CircuitStatus {
  Normal = 0,
  Throttled = 1,
  Paused = 2,
  Emergency = 3,
}

interface VolumeTracker {
  hourlyVolume: string;      // u128
  hourlyStartBlock: BlockNumber;
  dailyVolume: string;       // u128
  dailyStartBlock: BlockNumber;
}
```

### pallet-custodian-registry

```typescript
enum CustodianStatus {
  Pending,
  Active,
  Suspended,
  Slashed,
  Removed,
}

interface CustodianInfo {
  account: AccountId;
  bondAmount: Balance;
  licenseProof: string;      // Max 256 bytes
  lastAttestation: BlockNumber;
  status: CustodianStatus;
  attestedValue: string;     // u128 (USD cents)
  missedAttestations: number; // u32
}

interface Attestation {
  reserveValue: string;      // u128
  proof: string;             // Max 512 bytes
  auditorSignature: string;  // Max 256 bytes
  submittedAt: BlockNumber;
}
```

### pallet-xcm-bridge

```typescript
enum ChainId {
  Primearc Core = 0,
  PbcEdsc = 1,
  Other = 2,
}

enum MessageType {
  ReserveCheckpoint = 0,
  PriceUpdate = 1,
  Governance = 2,
  EmergencyPause = 3,
  Alert = 4,
}

enum MessageStatus {
  Pending,
  Sent,
  Received,
  Processed,
  Failed,
}

interface CrossChainMessage {
  source: ChainId;
  destination: ChainId;
  messageType: MessageType;
  payload: string;           // Max 1024 bytes
  blockNumber: BlockNumber;
  nonce: number;             // u64
  timestamp: number;         // u64
}

interface CheckpointPayload {
  reserveRatio: number;      // u16
  totalReserves: string;     // u128
  vaultValue: string;        // u128
  custodianValue: string;    // u128
  totalSupply: string;       // u128
  stateRoot: Hash;
}
```

### pallet-validator-committee

```typescript
enum PeerType {
  ValidityNode = 0,
  FlareNode = 1,
  DecentralizedDirector = 2,
  StakingCommon = 3,
  Common = 4,
}

interface ValidatorInfo {
  id: AccountId;             // ValidatorId
  stake: Balance;
  peerType: PeerType;
}

interface StoredValidatorInfo {
  validatorId: AccountId;
  stake: Balance;
  peerType: number;          // u8
}

type Epoch = number; // u64
```

### pallet-did-registry

```typescript
enum AccessLevel {
  None,
  Reader,
  Writer,
  Admin,
}

interface DidRegistration {
  did: string;               // Max 64 bytes
  owner: AccountId;
  controller: AccountId;
  documentHash: string;      // Max 64 bytes
  registeredAt: BlockNumber;
  updatedAt: BlockNumber;
  expiresAt?: BlockNumber;
  revoked: boolean;
}

interface AccessControl {
  didHash: Hash;
  agent: AccountId;
  level: AccessLevel;
  grantedAt: BlockNumber;
}
```

### pallet-aidid

```typescript
enum AIType {
  LLM,
  VisionModel,
  AudioModel,
  MultiModal,
  ReinforcementLearning,
  AgentSystem,
  Other,
}

enum Task {
  TextGeneration,
  Translation,
  Summarization,
  QuestionAnswering,
  CodeGeneration,
  ImageGeneration,
  ImageClassification,
  ObjectDetection,
  SpeechRecognition,
  TextToSpeech,
  Reasoning,
  Planning,
  Other,
}

enum Modality {
  Text,
  Image,
  Audio,
  Video,
  Code,
  Structured,
  Other,
}

interface Capabilities {
  tasks: Task[];
  inputModalities: Modality[];
  outputModalities: Modality[];
  languages: string[];       // ISO 639-1 codes
  maxContext?: number;       // u64
  maxOutput?: number;        // u64
}

interface SafetyProfile {
  alignmentMethod: string;   // Max 64 bytes (e.g., "RLHF")
  contentFiltering: boolean;
  biasEvaluated: boolean;
  toxicityScore: number;     // u32 (0-10000)
}

interface Restrictions {
  prohibitedTasks: Task[];
  rateLimit?: number;        // u64
  requiresHumanReview: boolean;
}

interface AIProfile {
  aiType: AIType;
  version: string;           // Max 32 bytes
  architecture: string;      // Max 128 bytes
  parameters: string;        // Max 32 bytes (e.g., "175B")
  capabilities: Capabilities;
  restrictions: Restrictions;
  safety: SafetyProfile;
}

interface Benchmark {
  name: string;              // Max 64 bytes
  score: number;             // u32 (0-10000)
}

interface ModelAttestation {
  modelHash: Uint8Array;     // 32 bytes
  trainingDataHash: string;  // Max 128 bytes
  version: string;           // Max 32 bytes
  trainingDate: number;      // u64 (Unix timestamp)
  reproducible: boolean;
  benchmarks: Benchmark[];
}

interface Permission {
  action: string;            // Max 64 bytes
  resource: string;          // Max 128 bytes
  conditions: string[];      // Max 10, each max 128 bytes
}

interface Reputation {
  totalInferences: number;   // u64
  successfulInferences: number; // u64
  userRating: number;        // u32 (0-10000)
  ratingCount: number;       // u32
}

enum PricingType {
  Free,
  PerToken,
  PerRequest,
  Subscription,
  Custom,
}

interface PricingModel {
  modelType: PricingType;
  basePrice: Balance;
  volumeDiscount?: Permill;
}

interface AIIdentity {
  identifier: string;        // Max 64 bytes
  aiType: AIType;
  controller: AccountId;
  profile: AIProfile;
  attestation?: ModelAttestation;
  pricing?: PricingModel;
  registeredAt: BlockNumber;
  updatedAt: BlockNumber;
  active: boolean;
}
```

---

## Code Examples

### Example 1: Query Reserve Data

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

async function getReserveData() {
  // Connect to node
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Get latest snapshot
  const snapshot = await api.query.reserveOracle.latestSnapshot();

  if (snapshot.isSome) {
    const data = snapshot.unwrap();
    console.log('Reserve Ratio:', data.reserveRatio.toNumber() / 100, '%');
    console.log('Total Reserves:', data.totalReserves.toString(), 'USD cents');
    console.log('Total Supply:', data.totalSupply.toString(), 'EDSC');
  }

  // Get asset price
  const btcPrice = await api.query.reserveOracle.assetPrices('BTC');
  if (btcPrice.isSome) {
    console.log('BTC Price:', btcPrice.unwrap().priceUsdCents.toString());
  }

  await api.disconnect();
}
```

### Example 2: Deposit Collateral

```typescript
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

async function depositCollateral() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  // Deposit 10 BTC
  const assetType = 1; // BTC
  const amount = '10000000000000'; // 10 BTC with 12 decimals

  const tx = api.tx.reserveVault.depositCollateral(assetType, amount);

  await tx.signAndSend(alice, ({ status, events }) => {
    if (status.isInBlock) {
      console.log(`Transaction included in block ${status.asInBlock}`);

      events.forEach(({ event }) => {
        if (event.section === 'reserveVault' && event.method === 'CollateralDeposited') {
          const [assetType, amount, depositor] = event.data;
          console.log(`Deposited ${amount} of asset ${assetType}`);
        }
      });
    }
  });

  await api.disconnect();
}
```

### Example 3: Register DID

```typescript
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { blake2AsHex } from '@polkadot/util-crypto';

async function registerDid() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');
  const bob = keyring.addFromUri('//Bob');

  const didIdentifier = 'did:etrid:alice123';
  const controller = bob.address;
  const documentHash = 'QmHash123...'; // IPFS hash

  const tx = api.tx.didRegistry.registerDid(
    didIdentifier,
    controller,
    documentHash
  );

  await tx.signAndSend(alice, ({ status, events }) => {
    if (status.isInBlock) {
      events.forEach(({ event }) => {
        if (event.section === 'didRegistry' && event.method === 'DidRegistered') {
          const [didHash, owner] = event.data;
          console.log(`DID registered with hash: ${didHash.toHex()}`);
          console.log(`Owner: ${owner.toString()}`);
        }
      });
    }
  });

  await api.disconnect();
}
```

### Example 4: Register AI Identity

```typescript
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

async function registerAI() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const controller = keyring.addFromUri('//Alice');

  // Create AI profile
  const profile = {
    aiType: { LLM: null },
    version: 'v1.0.0',
    architecture: 'transformer',
    parameters: '175B',
    capabilities: {
      tasks: [{ TextGeneration: null }],
      inputModalities: [{ Text: null }],
      outputModalities: [{ Text: null }],
      languages: ['en', 'es', 'fr'],
      maxContext: 4096,
      maxOutput: 2048,
    },
    restrictions: {
      prohibitedTasks: [],
      rateLimit: null,
      requiresHumanReview: false,
    },
    safety: {
      alignmentMethod: 'RLHF',
      contentFiltering: true,
      biasEvaluated: true,
      toxicityScore: 100, // 1%
    },
  };

  const tx = api.tx.aidid.registerAi(
    'gpt-etrid-v1',
    { LLM: null },
    profile
  );

  await tx.signAndSend(controller, ({ status, events }) => {
    if (status.isInBlock) {
      events.forEach(({ event }) => {
        if (event.section === 'aidid' && event.method === 'AIRegistered') {
          const [didHash, controller, aiType] = event.data;
          console.log(`AI registered with hash: ${didHash.toHex()}`);
        }
      });
    }
  });

  await api.disconnect();
}
```

### Example 5: Query Validator Committee

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

async function getValidatorCommittee() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // Get active committee
  const committee = await api.query.validatorCommittee.committee();
  console.log('Active validators:', committee.length);

  // Get validator details
  for (const validatorId of committee) {
    const info = await api.query.validatorCommittee.validators(validatorId);
    if (info.isSome) {
      const data = info.unwrap();
      console.log('Validator:', validatorId.toHex());
      console.log('Stake:', data.stake.toString());
      console.log('Type:', data.peerType);
    }
  }

  // Get current epoch
  const epoch = await api.query.validatorCommittee.currentEpoch();
  console.log('Current epoch:', epoch.toString());

  await api.disconnect();
}
```

### Example 6: Submit Multi-Oracle Price

```typescript
import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';

async function submitOraclePrice() {
  const wsProvider = new WsProvider('ws://localhost:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const oracle = keyring.addFromUri('//Oracle'); // Trusted oracle account

  const assetSymbol = 'ETH';
  const price = '300000000000'; // $3,000 with 8 decimals
  const source = 'chainlink';
  const confidence = 95; // 95% confidence

  const tx = api.tx.reserveOracle.submitPrice(
    assetSymbol,
    price,
    source,
    confidence
  );

  await tx.signAndSend(oracle, ({ status, events }) => {
    if (status.isInBlock) {
      events.forEach(({ event }) => {
        if (event.section === 'reserveOracle') {
          if (event.method === 'PriceSubmitted') {
            console.log('Price submitted');
          } else if (event.method === 'PriceAggregated') {
            const [symbol, medianPrice, sourcesCount] = event.data;
            console.log(`Aggregated ${symbol}: ${medianPrice} from ${sourcesCount} sources`);
          }
        }
      });
    }
  });

  await api.disconnect();
}
```

---

### pallet-distribution-pay

**Purpose:** Automated daily distribution system for rewarding validators, directors, stakers, and voters.

**Location**: `/Users/macbook/Desktop/etrid/12-consensus-day/distribution/src/lib.rs`

#### Extrinsics

##### `set_distribution_schedule`

Configure daily distribution amounts and times (governance only)

**Parameters:**
- `origin`: `OriginFor<T>` - Must be root
- `total_daily`: `u128` - Total daily distribution (default: 27,397 ÉTR)
- `voters_pct`: `u32` - Voters percentage in basis points (default: 1000 = 10%)
- `flare_nodes_pct`: `u32` - Flare nodes percentage (default: 1500 = 15%)
- `validity_nodes_pct`: `u32` - Validity nodes percentage (default: 1500 = 15%)
- `stakers_pct`: `u32` - Stakers percentage (default: 4000 = 40%)
- `directors_pct`: `u32` - Directors percentage (default: 2000 = 20%)

**Returns:** `DispatchResult`

**Events:**
- `DistributionScheduleUpdated { total_daily, allocations }`

**Example:**
```rust
DistributionPay::set_distribution_schedule(
    Origin::root(),
    27_397_000_000_000_000_000_000u128, // 27,397 ÉTR
    1000, // 10% voters
    1500, // 15% flare nodes
    1500, // 15% validity nodes
    4000, // 40% stakers
    2000, // 20% directors
)?;
```

---

##### `claim_reward`

Claim pending distribution reward

**Parameters:**
- `origin`: `OriginFor<T>` - Claimant account (signed)
- `category`: `DistributionCategory` - Category to claim from

**Returns:** `DispatchResult`

**Events:**
- `RewardClaimed { account, category, amount }`

**Errors:**
- `NoRewardAvailable` - No pending reward
- `NotEligible` - Account not eligible for category

**Example:**
```rust
// Claim validator reward
DistributionPay::claim_reward(
    Origin::signed(validator),
    DistributionCategory::FlareNodes
)?;

// Claim voter reward
DistributionPay::claim_reward(
    Origin::signed(voter),
    DistributionCategory::Voters
)?;
```

---

#### Storage Queries

##### `distribution_schedule() -> DistributionSchedule`

Get current distribution schedule

**Returns:**
```rust
DistributionSchedule {
    total_daily: u128,
    voters_pct: u32,
    flare_nodes_pct: u32,
    validity_nodes_pct: u32,
    stakers_pct: u32,
    directors_pct: u32,
}
```

---

##### `pending_rewards(account, category) -> u128`

Get pending reward for account in category

---

##### `last_distribution(category) -> Option<BlockNumber>`

Get block number of last distribution for category

---

#### Helper Functions

```rust
// Calculate coinage multiplier based on stake duration
pub fn calculate_coinage_multiplier(stake_duration_days: u64) -> u32

// Calculate reward with coinage
pub fn calculate_reward_with_coinage(base_reward: u128, stake_duration_days: u64) -> u128

// Apply penalty to reward
pub fn apply_penalty(base_reward: u128, penalty_type: PenaltyType) -> u128
```

---

### pallet-lightning-bloc

**Purpose:** Layer 2 payment channel network for instant, zero-fee transactions.

**Location**: `/Users/macbook/Desktop/etrid/07-transactions/lightning-bloc/`

#### Extrinsics

##### `open_channel`

Open a new payment channel

**Parameters:**
- `origin`: `OriginFor<T>` - Channel initiator (signed)
- `counterparty`: `T::AccountId` - Other party in channel
- `initial_balance_self`: `BalanceOf<T>` - Initial balance for self
- `initial_balance_counterparty`: `BalanceOf<T>` - Initial balance for counterparty
- `duration`: `BlockNumberFor<T>` - Channel duration in blocks

**Returns:** `DispatchResult`

**Events:**
- `ChannelOpened { channel_id, party_a, party_b, capacity }`

**Errors:**
- `InsufficientBalance` - Not enough balance to lock
- `ChannelAlreadyExists` - Channel already open between parties

**Example:**
```rust
// Open channel with 100 ÉTR from each party
LightningBloc::open_channel(
    Origin::signed(alice),
    bob,
    100_000_000_000_000_000_000, // 100 ÉTR
    100_000_000_000_000_000_000, // 100 ÉTR
    144_000, // ~1 month duration (6s blocks)
)?;
```

---

##### `update_channel`

Update channel state (off-chain payment)

**Parameters:**
- `origin`: `OriginFor<T>` - Either party (signed)
- `channel_id`: `H256` - Channel identifier
- `amount`: `BalanceOf<T>` - Payment amount
- `from_a_to_b`: `bool` - Payment direction
- `nonce`: `u64` - State nonce (anti-replay)
- `signature_a`: `Vec<u8>` - Signature from party A
- `signature_b`: `Vec<u8>` - Signature from party B

**Returns:** `DispatchResult`

**Events:**
- `ChannelUpdated { channel_id, new_balance_a, new_balance_b, nonce }`

**Errors:**
- `ChannelNotFound` - Channel doesn't exist
- `ChannelNotOpen` - Channel not in open state
- `InvalidSignature` - Invalid signature
- `InvalidNonce` - Nonce not greater than current
- `InsufficientChannelBalance` - Payment exceeds balance

**Example:**
```rust
// Alice pays Bob 10 ÉTR
LightningBloc::update_channel(
    Origin::signed(alice),
    channel_id,
    10_000_000_000_000_000_000, // 10 ÉTR
    true, // from_a_to_b
    42, // nonce
    alice_signature,
    bob_signature,
)?;
```

---

##### `close_channel_cooperative`

Close channel cooperatively (both parties agree)

**Parameters:**
- `origin`: `OriginFor<T>` - Either party (signed)
- `channel_id`: `H256` - Channel identifier
- `final_balance_a`: `BalanceOf<T>` - Final balance for party A
- `final_balance_b`: `BalanceOf<T>` - Final balance for party B
- `signature_a`: `Vec<u8>` - Signature from party A
- `signature_b`: `Vec<u8>` - Signature from party B

**Returns:** `DispatchResult`

**Events:**
- `ChannelClosed { channel_id, final_balance_a, final_balance_b }`

**Example:**
```rust
LightningBloc::close_channel_cooperative(
    Origin::signed(alice),
    channel_id,
    90_000_000_000_000_000_000, // Alice: 90 ÉTR
    110_000_000_000_000_000_000, // Bob: 110 ÉTR
    alice_signature,
    bob_signature,
)?;
```

---

##### `close_channel_unilateral`

Close channel unilaterally (dispute period enforced)

**Parameters:**
- `origin`: `OriginFor<T>` - Initiating party (signed)
- `channel_id`: `H256` - Channel identifier

**Returns:** `DispatchResult`

**Events:**
- `ChannelClosureInitiated { channel_id, initiated_by, close_height }`

**Example:**
```rust
LightningBloc::close_channel_unilateral(
    Origin::signed(alice),
    channel_id,
)?;
```

---

##### `dispute_channel`

Dispute fraudulent channel state

**Parameters:**
- `origin`: `OriginFor<T>` - Either party or watchtower (signed)
- `channel_id`: `H256` - Channel identifier
- `fraudulent_nonce`: `u64` - Fraudulent state nonce
- `correct_nonce`: `u64` - Correct state nonce
- `evidence`: `Vec<u8>` - Cryptographic evidence

**Returns:** `DispatchResult`

**Events:**
- `FraudDetected { channel_id, fraudster, detector }`
- `ChannelDisputed { channel_id, disputed_by, evidence }`

**Errors:**
- `ChannelNotFound` - Channel doesn't exist
- `InvalidEvidence` - Evidence doesn't prove fraud

**Example:**
```rust
LightningBloc::dispute_channel(
    Origin::signed(watchtower),
    channel_id,
    30, // fraudulent nonce
    50, // correct nonce
    signatures_evidence,
)?;
```

---

#### Storage Queries

##### `channels(channel_id) -> Option<PaymentChannel>`

Get channel details

**Returns:**
```rust
PaymentChannel {
    id: H256,
    party_a: AccountId,
    party_b: AccountId,
    current_balance_a: Balance,
    current_balance_b: Balance,
    nonce: u64,
    state: ChannelState, // Open, PendingClose, Closing, Closed, Disputed
    created_at: BlockNumber,
    expires_at: BlockNumber,
    capacity: Balance,
}
```

---

##### `watchtowers(watchtower_id) -> Option<Watchtower>`

Get watchtower information

---

##### `total_channels() -> u64`

Get total number of channels created

---

##### `total_volume() -> u128`

Get total transaction volume processed

---

#### Helper Functions

```rust
// Generate channel ID
pub fn generate_channel_id(party_a: &AccountId, party_b: &AccountId, nonce: u64) -> H256

// Verify channel signatures
pub fn verify_signatures(channel: &PaymentChannel, sig_a: &[u8], sig_b: &[u8]) -> bool

// Calculate routing fee
pub fn calculate_fee(amount: u128, fee_rate: u32, fee_base: u128) -> u128
```

---

### pallet-etwasm-vm

**Purpose:** EVM-compatible smart contract runtime with reentrancy protection.

**Location**: `/Users/macbook/Desktop/etrid/08-etwasm-vm/pallet/src/lib.rs`

#### Extrinsics

##### `deploy_contract`

Deploy a new smart contract

**Parameters:**
- `origin`: `OriginFor<T>` - Deployer account (signed)
- `code`: `Vec<u8>` - Contract bytecode (WASM or EVM)

**Returns:** `DispatchResult`

**Events:**
- `ContractDeployed { deployer, contract_address, code_hash }`

**Errors:**
- `CodeTooLarge` - Bytecode exceeds maximum size (1 MB)
- `InvalidBytecode` - Bytecode is malformed

**Example:**
```rust
EtwasmVM::deploy_contract(
    Origin::signed(alice),
    contract_bytecode,
)?;
```

---

##### `call_contract`

Call a deployed contract

**Parameters:**
- `origin`: `OriginFor<T>` - Caller account (signed)
- `contract_addr`: `T::AccountId` - Contract address
- `input_data`: `Vec<u8>` - Call data (function selector + args)
- `gas_limit`: `Option<VMw>` - Gas limit (default: 1M VMw)

**Returns:** `DispatchResult`

**Events:**
- `ContractCalled { caller, contract, gas_used }`
- `ContractExecuted { contract, gas_used, success }`

**Errors:**
- `ContractNotFound` - Contract doesn't exist
- `GasLimitExceeded` - Gas limit too high
- `OutOfGas` - Ran out of gas during execution
- `ExecutionFailed` - Contract reverted or errored
- `ReentrancyDetected` - Reentrancy attempt blocked
- `MaxCallDepthExceeded` - Too many nested calls

**Example:**
```rust
// Call transfer function on ERC-20 contract
let call_data = encode_erc20_transfer(recipient, amount);
EtwasmVM::call_contract(
    Origin::signed(alice),
    erc20_contract,
    call_data,
    Some(50_000), // 50K gas
)?;
```

---

##### `execute_bytecode`

Execute bytecode directly (for testing)

**Parameters:**
- `origin`: `OriginFor<T>` - Caller account (signed)
- `bytecode`: `Vec<u8>` - Bytecode to execute
- `gas_limit`: `VMw` - Gas limit

**Returns:** `DispatchResult`

**Events:**
- `ContractExecuted { contract, gas_used, success }`

**Errors:**
- `GasLimitExceeded` - Gas limit too high
- `OutOfGas` - Ran out of gas
- `ExecutionFailed` - Execution error

**Example:**
```rust
// Execute simple bytecode
EtwasmVM::execute_bytecode(
    Origin::signed(alice),
    simple_bytecode,
    10_000, // 10K gas
)?;
```

---

#### Storage Queries

##### `contract_code_hash(contract) -> Option<Hash>`

Get code hash for contract

---

##### `contract_owner(contract) -> Option<AccountId>`

Get contract owner

---

##### `contract_storage_value(contract, key) -> Option<H256>`

Get storage value at key for contract

---

##### `code_storage(code_hash) -> Option<Vec<u8>>`

Get bytecode for code hash

---

##### `gas_used() -> VMw`

Get total gas used in current block

---

#### Helper Functions

```rust
// Create VMw metering runtime
pub fn create_vmw_runtime(gas_limit: VMw) -> VmwMeteringRuntime

// Calculate net gas usage
pub fn calculate_net_gas(vmw: &VmwMeteringRuntime) -> (VMw, VMw, VMw)

// Convert account to bytes32
pub fn account_to_bytes32(account: &AccountId) -> [u8; 32]

// Charge gas for execution
pub fn charge_gas(amount: VMw) -> DispatchResult
```

**Gas Constants**:
```rust
const VMW_BLOCK_LIMIT: VMw = 10_000_000;  // 10M VMw per block
const VMW_TX_LIMIT: VMw = 1_000_000;      // 1M VMw per transaction
const VMW_PER_ETR: u128 = 1_000_000;      // 1 ÉTR = 1M VMw
```

**Opcode Gas Costs** (sample):
- ADD, SUB, NOT, LT, GT, EQ: 3 VMw
- MUL, DIV, MOD: 5 VMw
- SLOAD (warm): 200 VMw
- SSTORE (new): 20,000 VMw
- CALL: 100-9,000 VMw (depends on value transfer)
- CREATE: 32,000 VMw

---

## Summary

This API reference covers **11 pallets** with:
- **61 extrinsics** total across all pallets
- **50+ storage queries**
- **Runtime APIs** for validator committee management
- **Standard Substrate RPC endpoints**
- **Complete TypeScript type definitions**
- **6 detailed code examples**

### Pallets Documented:

1. **pallet-reserve-oracle** (8 extrinsics) - Reserve data aggregation and price feeds
2. **pallet-reserve-vault** (5 extrinsics) - Multi-asset collateral management
3. **pallet-circuit-breaker** (5 extrinsics) - Emergency safety controls
4. **pallet-custodian-registry** (6 extrinsics) - Off-chain custodian management
5. **pallet-xcm-bridge** (5 extrinsics) - Cross-chain messaging
6. **pallet-validator-committee** (3 extrinsics) - Validator committee management
7. **pallet-did-registry** (7 extrinsics) - W3C DID identity management
8. **pallet-aidid** (10 extrinsics) - AI identity management

### Missing/Empty Pallets:
- **pallet-edsc-redemption** - Directory exists but is empty (no implementation yet)

### Additional Resources:
- Polkadot.js Documentation: https://polkadot.js.org/docs/
- Substrate Documentation: https://docs.substrate.io/
- W3C DID Specification: https://www.w3.org/TR/did-core/
