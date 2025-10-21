# 05-multichain - Core Multichain Infrastructure Architecture

**Component**: 05-multichain
**Type**: Core System - Largest Component
**Status**: Phase 1 Complete - Operational
**Last Updated**: October 20, 2025

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture Hierarchy](#architecture-hierarchy)
3. [FlareChain (Main Chain)](#flarechain-main-chain)
4. [Partition Burst Chains (PBCs)](#partition-burst-chains-pbcs)
5. [Bridge Protocols](#bridge-protocols)
6. [EDSC Bridge (Phase 3 CCTP)](#edsc-bridge-phase-3-cctp)
7. [Primitives & Shared Types](#primitives--shared-types)
8. [Lightning Bloc Networks](#lightning-bloc-networks)
9. [State Aggregation Mechanism](#state-aggregation-mechanism)
10. [Cross-Chain Message Passing](#cross-chain-message-passing)
11. [Security Model](#security-model)
12. [Performance Characteristics](#performance-characteristics)
13. [Known Issues & Technical Debt](#known-issues--technical-debt)
14. [Integration Points](#integration-points)
15. [Build System & Deployment](#build-system--deployment)
16. [Future Roadmap](#future-roadmap)

---

## Overview

The **05-multichain** component is the **core infrastructure** of the Ëtrid Protocol, implementing a hierarchical multi-chain architecture with:

- **1 FlareChain** (main relay chain) - State aggregation & consensus
- **13 Partition Burst Chains (PBCs)** - Asset-specific sidechains
- **12 External Bridge Protocols** - Cross-chain connectivity
- **1 EDSC Bridge** - Native stablecoin bridge (CCTP-style)

### Component Statistics

| Metric | Value |
|--------|-------|
| Total Directory Size | 5.4 GB |
| Rust Source Files | 240+ files |
| Total Lines of Code | ~85,000+ lines |
| Built Binaries | 13 collators (47-55MB each) |
| WASM Runtimes | 13 runtimes (471-485KB compressed) |
| Bridge Pallets | 19 pallets |
| Development Time | 8 weeks (ongoing) |

### Directory Structure

```
05-multichain/
├── flare-chain/                    # Main relay chain
│   ├── runtime/                    # FlareChain runtime (986 lines)
│   ├── node/                       # FlareChain node + ASF service
│   └── ASF_INTEGRATION.md          # ASF consensus integration guide
│
├── partition-burst-chains/         # 13 PBC chains
│   ├── pbc-chains/                 # Individual PBC runtimes
│   │   ├── btc-pbc/               # Bitcoin PBC
│   │   ├── eth-pbc/               # Ethereum PBC
│   │   ├── doge-pbc/              # Dogecoin PBC
│   │   ├── sol-pbc/               # Solana PBC
│   │   ├── xlm-pbc/               # Stellar PBC
│   │   ├── xrp-pbc/               # Ripple PBC
│   │   ├── bnb-pbc/               # BNB Chain PBC
│   │   ├── trx-pbc/               # Tron PBC
│   │   ├── ada-pbc/               # Cardano PBC
│   │   ├── link-pbc/              # Chainlink PBC
│   │   ├── matic-pbc/             # Polygon PBC
│   │   ├── sc-usdt-pbc/           # USDT PBC
│   │   └── edsc-pbc/              # EDSC Stablecoin PBC
│   ├── pbc-runtime/               # Shared PBC runtime template
│   └── pbc-node/                  # PBC collator node implementations
│
├── bridge-protocols/              # External chain bridges
│   ├── bitcoin-bridge/            # BTC bridge pallet
│   ├── ethereum-bridge/           # ETH bridge pallet
│   ├── dogecoin-bridge/           # DOGE bridge pallet
│   ├── stellar-bridge/            # XLM bridge pallet
│   ├── xrp-bridge/                # XRP bridge pallet
│   ├── solana-bridge/             # SOL bridge pallet
│   ├── bnb-bridge/                # BNB bridge pallet
│   ├── tron-bridge/               # TRX bridge pallet
│   ├── cardano-bridge/            # ADA bridge pallet
│   ├── chainlink-bridge/          # LINK bridge pallet
│   ├── polygon-bridge/            # MATIC bridge pallet
│   ├── stablecoin-usdt-bridge/    # USDT bridge pallet
│   └── edsc-bridge/               # EDSC native stablecoin bridge
│       ├── substrate-pallets/     # 7 Substrate pallets
│       ├── ethereum-contracts/    # 4 Solidity contracts
│       └── services/              # Attestation & relayer services
│
├── primitives/                    # Shared type definitions
│   └── src/
│       ├── lib.rs                 # Core primitives (9,532 bytes)
│       ├── vmw.rs                 # VMw gas metering
│       ├── flare_chain_blocks.rs  # FlareChain block types
│       └── pbc_blocks.rs          # PBC block types
│
└── lightning-bloc-networks/       # Cross-chain payment channels
    ├── channel-manager/           # Channel management
    └── network/                   # Network protocol
```

---

## Architecture Hierarchy

### Three-Tier Design

```
┌─────────────────────────────────────────────────────────────────────┐
│                        FLARECHAIN (Tier 1)                          │
│                    Main Relay Chain & Consensus                     │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  ASF Consensus (PPFA + 3-Level Finality)                      │  │
│  │  State Aggregation from all 13 PBCs                           │  │
│  │  Validator Management (21-member committees)                  │  │
│  │  Bridge Pallet Integration (12 external + EDSC)               │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘
                                  ▲
                                  │ XCM / Cross-chain messages
                                  │
┌─────────────────────────────────┴─────────────────────────────────┐
│                                                                     │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │     PARTITION BURST CHAINS (Tier 2) - 13 PBCs             │   │
│  │                                                             │   │
│  │  BTC-PBC  ETH-PBC  DOGE-PBC  SOL-PBC  XLM-PBC  XRP-PBC    │   │
│  │  BNB-PBC  TRX-PBC  ADA-PBC   LINK-PBC MATIC-PBC           │   │
│  │  SC-USDT-PBC      EDSC-PBC                                │   │
│  │                                                             │   │
│  │  Each PBC:                                                 │   │
│  │  - Asset-specific logic (BTC, ETH, DOGE, etc.)            │   │
│  │  - WASM runtime (471-485KB compressed)                    │   │
│  │  - Collator node (47MB binary)                            │   │
│  │  - Connects to FlareChain relay                           │   │
│  │  - Reports state to FlareChain                            │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
                                  ▲
                                  │ Bridge Protocols
                                  │
┌─────────────────────────────────┴─────────────────────────────────┐
│                                                                     │
│  ┌────────────────────────────────────────────────────────────┐   │
│  │        EXTERNAL BLOCKCHAINS (Tier 3)                       │   │
│  │                                                             │   │
│  │  Bitcoin    Ethereum   Dogecoin   Stellar    Ripple        │   │
│  │  Solana     BNB Chain  Tron       Cardano    Polygon       │   │
│  │  Chainlink  USDT (ERC-20)         Ethereum (EDSC)          │   │
│  │                                                             │   │
│  │  Bridge Mechanisms:                                        │   │
│  │  - Multisig vaults (BTC, DOGE, etc.)                      │   │
│  │  - Smart contracts (ETH, BNB, etc.)                       │   │
│  │  - M-of-N attestation (EDSC CCTP)                         │   │
│  └────────────────────────────────────────────────────────────┘   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Data Flow

```
External Chain → Bridge Pallet → PBC Runtime → Collator → FlareChain → State Aggregation
     ▲                                                                        │
     └────────────────────────────────────────────────────────────────────────┘
                           Cross-chain Message Response
```

---

## FlareChain (Main Chain)

### Purpose

FlareChain is the **main relay chain** that:
- Aggregates state from all 13 PBCs
- Provides consensus via ASF (Ascending Scale of Finality)
- Manages cross-chain message routing
- Coordinates validator committees
- Maintains global security model

### Key Components

#### 1. Runtime (`flare-chain/runtime/src/lib.rs`)

**Size**: 986 lines
**Compiled WASM**: ~1.2MB compressed

**Integrated Pallets**:
```rust
// Core System
frame_system
pallet_timestamp
pallet_balances
pallet_transaction_payment
pallet_sudo

// Consensus (Traditional - Transitioning to ASF)
pallet_aura
pallet_grandpa

// Bridge Integration (12 external + EDSC)
pallet_bitcoin_bridge
pallet_ethereum_bridge
pallet_doge_bridge
pallet_stellar_bridge
pallet_xrp_bridge
pallet_solana_bridge
pallet_bnb_bridge
pallet_tron_bridge
pallet_cardano_bridge
pallet_chainlink_bridge
pallet_polygon_bridge
pallet_stablecoin_usdt_bridge

// EDSC Bridge (7 pallets)
pallet_edsc_token
pallet_edsc_receipts
pallet_edsc_redemption
pallet_edsc_oracle
pallet_edsc_bridge_token_messenger
pallet_edsc_bridge_attestation
pallet_xcm_bridge
```

**Runtime Configuration**:
```rust
VERSION: RuntimeVersion {
    spec_name: "etrid",
    impl_name: "etrid",
    spec_version: 100,
    impl_version: 1,
    transaction_version: 1,
}

// Block parameters
AVERAGE_ON_INITIALIZE_RATIO: 10%
MAXIMUM_BLOCK_WEIGHT: 2 seconds of compute (6s block time)
MAXIMUM_BLOCK_LENGTH: 5 MB

// Block time: 6 seconds (similar to Polkadot)
```

#### 2. Node (`flare-chain/node/`)

**Binary**: `flarechain-node` (55MB)

**Features**:
- RPC endpoints (HTTP + WebSocket)
- P2P networking (libp2p - transitioning to DETR P2P)
- Block production (AURA - transitioning to ASF PPFA)
- Finality gadget (GRANDPA + ASF hybrid)
- Telemetry & metrics

**Services**:
```rust
// Traditional service (service.rs)
new_partial()  // Shared components
new_full()     // Full node
new_light()    // Light client

// ASF service (asf_service.rs) - 705 lines
new_full_with_params(AsfParams)
- ASF block production (PPFA)
- Hybrid finality (ASF + GRANDPA)
- Validator management
- Committee coordination
```

#### 3. ASF Consensus Integration

**Document**: `flare-chain/ASF_INTEGRATION.md` (172 lines)

**Architecture**:
```
FlareChain Node
├─ ASF Block Production (PPFA)
│   ├─ Proposer selection (block-production crate)
│   ├─ Block authoring (Queen/Ant blocks)
│   └─ Transaction selection
│
├─ Hybrid Finality
│   ├─ ASF Finality Gadget (3-level)
│   │   ├─ Pre-commitment
│   │   ├─ Commitment
│   │   └─ Finality
│   └─ GRANDPA (traditional, transitional)
│
└─ Validator Management
    ├─ Committee management (PPFA panels)
    ├─ Health monitoring
    └─ Reward distribution
```

**Default ASF Parameters**:
```rust
AsfParams {
    slot_duration: 6000,              // 6 seconds
    max_committee_size: 21,           // PPFA panel size
    epoch_duration: 2400,             // ~4 hours at 6s blocks
    enable_finality_gadget: true,     // Enable 3-level finality
    min_validator_stake: 64_ETR,      // 64 ËTR for FlareNode
}
```

**Consensus Mechanism**:
- **Current**: AURA (round-robin block production) + GRANDPA (finality)
- **Target**: ASF PPFA (Proposing Panel for Attestation) + ASF Finality Gadget
- **Transition**: Hybrid mode running both systems in parallel

**Dependencies**:
```toml
asf-algorithm         # Core consensus logic (FODDoS, PPFA rotation)
block-production      # PPFA proposer selection and block authoring
etrid-finality-gadget # Three-level finality
validator-management  # Committee management
```

**Status**: Service structure complete, workers pending integration (see `/09-consensus/ARCHITECTURE.md`)

---

## Partition Burst Chains (PBCs)

### Overview

**PBCs** are asset-specific sidechains that handle transactions for individual cryptocurrencies. Each PBC:
- Connects to FlareChain as a parachain/collator
- Maintains its own state for one asset type
- Reports state updates to FlareChain
- Enables asset-specific optimizations

### PBC List (13 Total)

| PBC Name | Asset | WASM Size | Binary Size | Status |
|----------|-------|-----------|-------------|--------|
| BTC-PBC | Bitcoin | 475KB | 47MB | ✅ Built |
| ETH-PBC | Ethereum | 471KB | 47MB | ✅ Built |
| DOGE-PBC | Dogecoin | 471KB | 47MB | ✅ Built |
| SOL-PBC | Solana | 479KB | 47MB | ✅ Built |
| XLM-PBC | Stellar | 482KB | 47MB | ✅ Built |
| XRP-PBC | Ripple | 475KB | 47MB | ✅ Built |
| BNB-PBC | BNB Chain | 478KB | 47MB | ✅ Built |
| TRX-PBC | Tron | 479KB | 47MB | ✅ Built |
| ADA-PBC | Cardano | 474KB | 47MB | ✅ Built |
| LINK-PBC | Chainlink | 471KB | 47MB | ✅ Built |
| MATIC-PBC | Polygon | 485KB | 47MB | ✅ Built |
| SC-USDT-PBC | USDT | 477KB | 47MB | ✅ Built |
| EDSC-PBC | EDSC Stablecoin | TBD | TBD | 🚧 Pending |

**Total**: 12 operational + 1 pending (EDSC-PBC)

### PBC Runtime Architecture

Each PBC runtime (`pbc-chains/*/runtime/src/lib.rs`):

**Size**: ~629 lines (nearly identical across all PBCs)

**Core Structure**:
```rust
// 1. WASM binary inclusion
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// 2. Runtime version
VERSION: RuntimeVersion {
    spec_name: "btc-pbc",  // Unique per PBC
    spec_version: 100,
    impl_version: 1,
}

// 3. Standard pallets
construct_runtime! {
    System: frame_system
    Timestamp: pallet_timestamp
    Balances: pallet_balances
    TransactionPayment: pallet_transaction_payment
    Sudo: pallet_sudo

    // Asset-specific bridge pallet
    BitcoinBridge: pallet_bitcoin_bridge  // Varies per PBC
}

// 4. GenesisBuilder API (required for stable2506)
impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
    fn build_state(config: Vec<u8>) -> Result
    fn get_preset(id: &Option<PresetId>) -> Option<Vec<u8>>
    fn preset_names() -> Vec<PresetId>
}
```

**Genesis Presets** (per PBC):
```
runtime/presets/
├── development.json      # Alice, Bob, Charlie (test keys)
└── local_testnet.json    # Alice, Bob, Charlie, Dave
```

**⚠️ Security Warning**: Presets use well-known test keys. **NEVER use in production**.

### PBC Collator Nodes

**Location**: `partition-burst-chains/pbc-node/pbc-collator-nodes/*/`

Each collator:
```rust
// Example: btc-pbc-collator
use btc_pbc_runtime::Runtime;

fn main() {
    // 1. Parse CLI args
    // 2. Connect to FlareChain relay via --relay-chain-rpc
    // 3. Start collation service
    // 4. Produce PBC blocks
    // 5. Submit to FlareChain
}
```

**Startup Example**:
```bash
# Start FlareChain relay
./flarechain-node --chain chain-specs/flarechain-shared.json --alice --validator

# Start BTC PBC collator
./btc-pbc-collator \
  --dev \
  --relay-chain-rpc ws://127.0.0.1:9944 \
  --port 30334 \
  --rpc-port 9945
```

### Known Issue: 92.6% Code Duplication

**Problem**: All 12 PBC runtimes share nearly identical code (only differ in bridge pallet name).

**Impact**:
- Maintenance burden (bug fixes require 12 identical changes)
- Increased binary storage (12 × 47MB = 564MB)
- Difficult to audit (change tracking across 12 files)

**Identified in**: Internal code audit (October 2025)

**Proposed Solution** (Future Work):
```rust
// Generic PBC runtime with compile-time specialization
pub struct PbcRuntime<B: BridgePallet> {
    bridge: B,
    // ... shared logic
}

// Instantiate per asset
type BtcPbcRuntime = PbcRuntime<BitcoinBridge>;
type EthPbcRuntime = PbcRuntime<EthereumBridge>;
// etc.
```

**Estimated Effort**: 2-3 weeks to refactor
**Priority**: Medium (functional but not optimal)

---

## Bridge Protocols

### Overview

**12 external bridge protocols** enable cross-chain asset transfers between Ëtrid and external blockchains.

### Bridge List

| Bridge | Type | Chain | Mechanism |
|--------|------|-------|-----------|
| bitcoin-bridge | UTXO | Bitcoin | Multisig vault |
| ethereum-bridge | EVM | Ethereum | Smart contract lock |
| dogecoin-bridge | UTXO | Dogecoin | Multisig vault |
| stellar-bridge | Stellar | Stellar | Federation |
| xrp-bridge | XRP Ledger | Ripple | Escrow |
| solana-bridge | Solana VM | Solana | Program account |
| bnb-bridge | EVM | BNB Chain | Smart contract |
| tron-bridge | TVM | Tron | Smart contract |
| cardano-bridge | UTXO | Cardano | Plutus script |
| chainlink-bridge | Oracle | Chainlink | Data feeds |
| polygon-bridge | EVM | Polygon | Plasma bridge |
| stablecoin-usdt-bridge | ERC-20 | Ethereum | Token lock |

### Bridge Architecture

Each bridge pallet implements:

```rust
// Example: bitcoin-bridge/src/lib.rs
#[frame_support::pallet]
pub mod pallet {
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent;
        type Currency;

        // Bridge-specific config
        type MinConfirmations: Get<u32>;  // 6 for BTC
        type MinDepositAmount: Get<u64>;
        type MaxDepositAmount: Get<u64>;
        type BridgeAuthority: Get<Self::AccountId>;
    }

    // Deposit request tracking
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
    pub struct DepositRequest<AccountId, Balance> {
        pub depositor: AccountId,
        pub btc_address: BoundedVec<u8, ConstU32<64>>,
        pub btc_txid: BoundedVec<u8, ConstU32<64>>,
        pub amount_satoshi: u64,
        pub amount_etr: Balance,
        pub confirmations: u32,
        pub status: DepositStatus,
    }

    // Withdrawal request tracking
    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
    pub struct WithdrawalRequest<AccountId, Balance> {
        pub withdrawer: AccountId,
        pub btc_address: BoundedVec<u8, ConstU32<64>>,
        pub amount_satoshi: u64,
        pub amount_etr: Balance,
        pub status: WithdrawalStatus,
        pub btc_txid: Option<BoundedVec<u8, ConstU32<64>>>,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // User requests deposit
        pub fn request_deposit(
            btc_address: Vec<u8>,
            amount: u64,
        ) -> DispatchResult

        // Bridge authority confirms deposit
        pub fn confirm_deposit(
            deposit_id: u64,
            btc_txid: Vec<u8>,
        ) -> DispatchResult

        // User requests withdrawal
        pub fn request_withdrawal(
            btc_address: Vec<u8>,
            amount: Balance,
        ) -> DispatchResult

        // Bridge authority processes withdrawal
        pub fn process_withdrawal(
            withdrawal_id: u64,
            btc_txid: Vec<u8>,
        ) -> DispatchResult
    }
}
```

### Bridge Security Model

**Multi-signature Authority**:
- Each bridge has a designated `BridgeAuthority` account
- Initially controlled by a multisig (3-of-5 or 5-of-9)
- Plans to transition to decentralized oracle network

**Confirmation Requirements**:
- Bitcoin: 6 confirmations (~1 hour)
- Ethereum: 12 confirmations (~3 minutes)
- Dogecoin: 40 confirmations (~40 minutes)
- Others: Chain-specific

**Limits**:
```rust
MinDepositAmount: 0.001 BTC (100,000 satoshi)
MaxDepositAmount: 10 BTC (1,000,000,000 satoshi)
DailyWithdrawalLimit: 100 BTC per bridge
```

### Bridge Integration in FlareChain

All 12 bridge pallets are integrated into FlareChain runtime:

```rust
// flare-chain/runtime/Cargo.toml
[dependencies]
pallet-bitcoin-bridge = { path = "../../bridge-protocols/bitcoin-bridge" }
pallet-ethereum-bridge = { path = "../../bridge-protocols/ethereum-bridge" }
// ... (10 more)

// flare-chain/runtime/src/lib.rs
construct_runtime! {
    BitcoinBridge: pallet_bitcoin_bridge,
    EthereumBridge: pallet_ethereum_bridge,
    // ... (10 more)
}
```

**Note**: Bridge pallets are in FlareChain runtime, NOT in individual PBC runtimes. PBCs connect to FlareChain to access bridge functionality.

---

## EDSC Bridge (Phase 3 CCTP)

### Overview

The **EDSC Bridge** is Ëtrid's **native stablecoin bridge** enabling cross-chain transfers of EDSC (Ëtrid Stablecoin) between Ëtrid and Ethereum (and other EVM chains).

**Architecture**: CCTP-style (Circle's Cross-Chain Transfer Protocol)
**Mechanism**: Burn-and-mint with M-of-N attestation
**Status**: ✅ Phase 1 & 2 Complete (Core pallets functional)

**Document**: `bridge-protocols/edsc-bridge/README.md` (165 lines)

### Components

#### 1. Substrate Pallets (7 pallets)

**Location**: `bridge-protocols/edsc-bridge/substrate-pallets/`

| Pallet | Purpose | Status |
|--------|---------|--------|
| pallet-edsc-token | ERC-20 compatible EDSC token on Substrate | ✅ Complete |
| pallet-edsc-bridge-token-messenger | Burn/mint operations, nonce management | ✅ Complete |
| pallet-edsc-bridge-attestation | M-of-N signature validation (3-of-5) | ✅ Complete |
| pallet-edsc-receipts | Soulbound Token (SBT) receipt system | ✅ Complete |
| pallet-edsc-redemption | 3-path redemption system, dynamic fees | ✅ Complete |
| pallet-edsc-oracle | TWAP price oracle, multi-source aggregation | ✅ Complete |
| pallet-edsc-checkpoint | State checkpointing, rollback protection | 🚧 Partial |

**pallet-edsc-token**:
```rust
// ERC-20 compatible token
transfer(to, amount)
approve(spender, amount)
transfer_from(from, to, amount)

// Controlled minting (authorized minters only)
mint(to, amount)  // Restricted

// Public burning (redemption support)
burn(amount)

// Supply tracking
total_supply()
max_supply: 2.5B EDSC
```

**pallet-edsc-bridge-token-messenger**:
```rust
// Burn tokens on source chain
burn_and_send(
    destination_domain: u32,  // Ethereum=0, Solana=1, Ëtrid=2
    recipient: Vec<u8>,
    amount: Balance,
) -> DispatchResult

// Mint tokens on destination chain
receive_message(
    message: Vec<u8>,
    attestations: Vec<Signature>,  // 3-of-5 signatures
) -> DispatchResult
```

**pallet-edsc-bridge-attestation**:
```rust
// M-of-N signature validation
pub const ATTESTATION_THRESHOLD: u32 = 3;  // 3 out of 5 attesters
pub const TOTAL_ATTESTERS: u32 = 5;

// Attester registry
register_attester(attester: AccountId)
remove_attester(attester: AccountId)

// Message verification
verify_attestations(
    message_hash: H256,
    signatures: Vec<Signature>,
) -> bool
```

**pallet-edsc-receipts** (Soulbound Token System):
```rust
// Receipt structure
pub struct Receipt {
    owner: AccountId,
    purchase_price: Balance,    // Price at mint time
    amount: Balance,
    consumed: Balance,          // Partial consumption support
    expiry: BlockNumber,
    non_transferable: bool,     // Always true (SBT)
}

// Functions
mint_receipt(owner, amount, purchase_price)
consume_receipt(receipt_id, amount)  // For redemption
```

**pallet-edsc-redemption** (3-Path System):
```rust
// Path 1: Soulbound Token Proof (NO FEE)
redeem_with_sbt(receipt_id, amount)

// Path 2: Attestation (Dynamic Fee)
redeem_with_attestation(amount, proof)

// Path 3: TWAP (2x Fee Penalty)
redeem_with_twap(amount)

// Dynamic fee calculation
fee = market_deviation × base_fee
// Example: If EDSC = $0.95, fee removes arbitrage profit

// Circuit breakers
if reserve_ratio < 100%: PAUSE all redemptions
if reserve_ratio < 105%: THROTTLE (queue system)
if hourly_volume > 0.5% supply: PAUSE for 1 hour
if daily_volume > 2% supply: PAUSE for 24 hours
```

**pallet-edsc-oracle** (TWAP Price Oracle):
```rust
// Multi-source price aggregation
MIN_SOURCES: 5
TWAP_WINDOW_PRIMARY: 24 hours
TWAP_WINDOW_FALLBACK: 7 days
OUTLIER_THRESHOLD: 2% (removed if >2% from median)
ORACLE_STALE_TIMEOUT: 10 minutes

// Sources
CEX: Binance, Coinbase, Kraken
DEX: Uniswap V3, Curve, PancakeSwap
Fallback: CoinGecko, Messari

// Functions
submit_price(source, price, volume, timestamp)
get_twap() -> Balance
update_prices()  // Called every 100 blocks
```

#### 2. Ethereum Contracts (4 contracts)

**Location**: `bridge-protocols/edsc-bridge/ethereum-contracts/` (symlink to `/contracts/ethereum`)

| Contract | Purpose | Functions |
|----------|---------|-----------|
| EDSC.sol | ERC-20 EDSC token | Standard ERC-20 + controlled minting |
| EDSCTokenMessenger.sol | Burn and send controller | `burnAndSendTo(domain, recipient, amount)` |
| EDSCMessageTransmitter.sol | Message handling | `receiveMessage(message, attestations)` |
| AttesterRegistry.sol | Attester management | `addAttester()`, `removeAttester()`, `setThreshold()` |

**EDSC.sol**:
```solidity
contract EDSC is ERC20 {
    address public messageTransmitter;  // Only this can mint

    function mint(address to, uint256 amount) external {
        require(msg.sender == messageTransmitter, "Unauthorized");
        _mint(to, amount);
    }

    function burn(uint256 amount) external {
        _burn(msg.sender, amount);
    }
}
```

**EDSCTokenMessenger.sol**:
```solidity
function burnAndSendTo(
    uint32 destinationDomain,  // 0=Ethereum, 2=Ëtrid
    bytes32 mintRecipient,
    uint256 amount
) external {
    edscToken.burn(msg.sender, amount);

    bytes memory message = abi.encode(
        destinationDomain,
        mintRecipient,
        amount,
        nonce++
    );

    emit MessageSent(message);
}
```

**EDSCMessageTransmitter.sol**:
```solidity
function receiveMessage(
    bytes calldata message,
    bytes[] calldata attestations  // 3-of-5 signatures
) external {
    require(attestations.length >= threshold, "Insufficient signatures");

    bytes32 messageHash = keccak256(message);
    require(verifyAttestations(messageHash, attestations), "Invalid signatures");

    (uint32 domain, bytes32 recipient, uint256 amount, uint64 nonce) =
        abi.decode(message, (uint32, bytes32, uint256, uint64));

    require(!usedNonces[nonce], "Replay attack");
    usedNonces[nonce] = true;

    edscToken.mint(address(uint160(uint256(recipient))), amount);
}
```

#### 3. Services (TypeScript)

**Location**: `bridge-protocols/edsc-bridge/services/` (symlink to `/services`)

**attestation-service** (TypeScript + ethers.js):
```typescript
// Monitors burn events on both chains
async function monitorBurnEvents() {
    // Listen to Ethereum MessageSent events
    ethereumContract.on('MessageSent', async (message) => {
        const messageHash = keccak256(message);
        const signature = await wallet.signMessage(messageHash);
        await submitAttestation(messageHash, signature);
    });

    // Listen to Ëtrid burn_and_send events
    substrateApi.query.system.events((events) => {
        events.forEach(async ({ event }) => {
            if (event.method === 'BurnAndSend') {
                const messageHash = blake2b(event.data);
                const signature = await wallet.signMessage(messageHash);
                await submitAttestation(messageHash, signature);
            }
        });
    });
}
```

**relayer-service** (TypeScript + ethers.js):
```typescript
// Polls for fully-attested messages
async function relayMessages() {
    const attestedMessages = await getAttestedMessages();

    for (const msg of attestedMessages) {
        if (msg.attestations.length >= THRESHOLD) {
            try {
                // Submit to destination chain
                const tx = await messageTransmitter.receiveMessage(
                    msg.message,
                    msg.attestations,
                    { gasLimit: 500000 }
                );
                await tx.wait();
                console.log(`Relayed message ${msg.nonce}`);
            } catch (err) {
                console.error(`Failed to relay: ${err}`);
                // Automatic retry with exponential backoff
            }
        }
    }
}
```

### EDSC Bridge Flow

#### Ethereum → Ëtrid

```
1. User calls burnAndSendTo() on Ethereum
2. EDSC burned, MessageSent event emitted
3. 3 attestation services detect event and sign message
4. Relayer collects 3 signatures
5. Relayer calls receive_message() on Ëtrid
6. Substrate pallet verifies signatures and mints EDSC
```

#### Ëtrid → Ethereum

```
1. User calls burn_and_send() extrinsic
2. EDSC burned, event emitted
3. 3 attestation services detect and sign
4. Relayer submits to Ethereum MessageTransmitter
5. MessageTransmitter verifies signatures and mints
```

### Security Features

**M-of-N Attestation**:
- Requires 3 out of 5 attesters to sign each message
- Prevents single point of failure
- Attesters run independently

**Nonce Management**:
- Each message has unique nonce
- Used nonces tracked on-chain
- Prevents replay attacks

**Domain Separation**:
- Chain-specific message formats
- Ethereum (domain 0), Solana (domain 1), Ëtrid (domain 2)
- Prevents cross-chain replay

**Pause Controls**:
- Emergency shutdown capability
- Owner-controlled pause/unpause
- Applies to both minting and burning

**Circuit Breakers** (Redemption Path):
```rust
Reserve Ratio Enforcement:
- RR < 100%: PAUSE all redemptions
- RR < 105%: THROTTLE redemptions (queue system)
- RR < 110%: INCREASE dynamic fees

Volume Caps:
- Hourly: 0.5% of total supply
- Daily: 2% of total supply

Per-Wallet Limits:
- Path 1 (SBT): $50k daily
- Path 2 (Attestation): $25k daily
- Path 3 (TWAP): $10k daily
```

### EDSC Phase Implementation

**Phase 1** (✅ Complete):
- pallet-edsc-token
- pallet-edsc-receipts
- Basic bridge structure

**Phase 2** (✅ Complete):
- pallet-edsc-redemption (3-path logic)
- pallet-edsc-oracle (TWAP)
- Circuit breakers
- Dynamic fee calculation

**Phase 3** (🚧 In Progress):
- EDSC-PBC collator setup
- Cross-chain XCM integration
- Full bridge testing
- Production deployment

**Status Document**: `/EDSC_BRIDGE_STATUS.md` (409 lines)

### Known Gaps

**Critical Security Gaps** (from EDSC_BRIDGE_STATUS.md):
1. ❌ No reserve backing (EDSC currently unbacked)
2. ❌ No peg defense mechanism (can depeg to $0)
3. ⚠️ Oracle system complete but untested in production
4. ⚠️ Circuit breakers implemented but need stress testing
5. ❌ No custodian registry for off-chain reserves

**Next Steps**:
1. Implement pallet-reserve-vault (on-chain collateral)
2. Implement pallet-custodian-registry (off-chain reserves)
3. Complete EDSC-PBC collator
4. Stress testing (death spiral scenarios)
5. External security audit

---

## Primitives & Shared Types

### Location

`05-multichain/primitives/src/`

### Files

| File | Size | Purpose |
|------|------|---------|
| lib.rs | 9,532 bytes | Core primitives exports |
| vmw.rs | 5,058 bytes | VMw gas metering types |
| flare_chain_blocks.rs | 7,911 bytes | FlareChain block structures |
| pbc_blocks.rs | 12,707 bytes | PBC block structures |

### Key Types

**VMw (Virtual Machine Watts)** - Gas Metering Unit:
```rust
// vmw.rs
pub type Vmw = u64;

pub const VMW_PER_GAS: Vmw = 1000;  // 1 gas = 1000 VMw

pub struct VmwCost {
    pub base: Vmw,
    pub per_byte: Vmw,
}

// Operation costs
pub const STORAGE_READ: VmwCost = VmwCost { base: 50, per_byte: 1 };
pub const STORAGE_WRITE: VmwCost = VmwCost { base: 2000, per_byte: 10 };
pub const COMPUTE: VmwCost = VmwCost { base: 100, per_byte: 0 };

// Block limits
pub const MAX_BLOCK_VMW: Vmw = 10_000_000;  // 10M VMw per block
pub const MAX_TX_VMW: Vmw = 1_000_000;      // 1M VMw per transaction
```

**FlareChain Block Types**:
```rust
// flare_chain_blocks.rs
pub struct FlareChainBlock<Header, Extrinsics> {
    pub header: Header,
    pub extrinsics: Extrinsics,
    pub pbc_state_roots: Vec<H256>,  // State roots from all PBCs
    pub aggregated_state: AggregatedState,
}

pub struct AggregatedState {
    pub total_pbc_blocks: u64,
    pub total_cross_chain_messages: u64,
    pub pbc_balances: Vec<(ChainId, Balance)>,
}
```

**PBC Block Types**:
```rust
// pbc_blocks.rs
pub struct PbcBlock<Header, Extrinsics> {
    pub header: Header,
    pub extrinsics: Extrinsics,
    pub relay_parent: H256,  // FlareChain block hash
    pub bridge_messages: Vec<BridgeMessage>,
}

pub struct BridgeMessage {
    pub source: ChainId,
    pub destination: ChainId,
    pub payload: Vec<u8>,
    pub nonce: u64,
}

pub enum ChainId {
    FlareChain,
    BtcPbc,
    EthPbc,
    // ... (11 more)
}
```

### Cargo.toml

```toml
[package]
name = "etrid-primitives"
version = "0.1.0"
edition = "2021"

[dependencies]
codec = { workspace = true }
scale-info = { workspace = true }
sp-core = { workspace = true }
sp-runtime = { workspace = true }
sp-std = { workspace = true }
serde = { workspace = true, optional = true }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "sp-core/std",
    "sp-runtime/std",
    "sp-std/std",
    "serde/std",
]
```

**Usage**: Shared across FlareChain, all 13 PBCs, and bridge pallets.

---

## Lightning Bloc Networks

### Overview

**Lightning Bloc Networks** provide off-chain payment channels for fast, low-cost cross-chain transactions.

**Status**: 🚧 Early Development

### Components

**Location**: `05-multichain/lightning-bloc-networks/`

```
lightning-bloc-networks/
├── channel-manager/      # Channel lifecycle management
│   └── src/
│       └── lib.rs
└── network/              # P2P network protocol
    └── src/
```

### Channel Manager

**Purpose**: Manages payment channel lifecycles

```rust
// channel-manager/src/lib.rs (conceptual)
pub struct Channel {
    pub channel_id: H256,
    pub participants: [AccountId; 2],
    pub balances: [Balance; 2],
    pub state: ChannelState,
    pub timeout: BlockNumber,
}

pub enum ChannelState {
    Open,
    Active,
    Disputed,
    Closed,
}

// Functions
open_channel(counterparty, deposit)
update_channel(channel_id, new_state, signatures)
close_channel(channel_id, final_state)
dispute_channel(channel_id, proof)
```

### Network Protocol

**Purpose**: P2P communication between channel participants

**Features** (Planned):
- Direct peer-to-peer messaging
- State update propagation
- Dispute resolution coordination
- Integration with DETR P2P (see `/01-detr-p2p/`)

**Status**: Basic structure exists, implementation pending

---

## State Aggregation Mechanism

### How FlareChain Aggregates PBC State

```
┌─────────────────────────────────────────────────────────────┐
│                     FlareChain Block N                      │
│  ┌───────────────────────────────────────────────────────┐  │
│  │  Header:                                              │  │
│  │    - Block number: N                                  │  │
│  │    - Parent hash: H(N-1)                              │  │
│  │    - State root: Root(FlareChain + 13 PBC roots)      │  │
│  │                                                        │  │
│  │  PBC State Roots (13 entries):                        │  │
│  │    - BTC-PBC:  0x1234...  (block height: 45678)       │  │
│  │    - ETH-PBC:  0x5678...  (block height: 89012)       │  │
│  │    - DOGE-PBC: 0x9abc...  (block height: 34567)       │  │
│  │    ... (10 more)                                      │  │
│  │                                                        │  │
│  │  Aggregated Balances:                                 │  │
│  │    - Total BTC locked: 1,234.56 BTC                   │  │
│  │    - Total ETH locked: 5,678.90 ETH                   │  │
│  │    - Total ËTR minted: 10,000,000 ËTR                 │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Aggregation Process

**Step 1**: PBC collators produce blocks
```
BTC-PBC Block 45678 → State Root: 0x1234...
ETH-PBC Block 89012 → State Root: 0x5678...
...
```

**Step 2**: Collators submit state roots to FlareChain
```rust
// PBC collator code
fn submit_state_to_relay(state_root: H256, block_height: u64) {
    relay_chain_api.submit_pbc_state(
        pbc_id: ChainId::BtcPbc,
        state_root,
        block_height,
    );
}
```

**Step 3**: FlareChain validator includes PBC states in block
```rust
// FlareChain runtime
fn on_initialize(block_number: BlockNumber) {
    let pbc_states = collect_pbc_state_updates();

    // Verify each PBC state root
    for (pbc_id, state_root, height) in pbc_states {
        verify_pbc_state_transition(pbc_id, state_root, height);
    }

    // Aggregate into FlareChain state
    update_aggregated_state(pbc_states);
}
```

**Step 4**: FlareChain finalizes with aggregated state
```
FlareChain Block N finalized → All 13 PBC states at block N finalized
```

### State Verification

**Merkle Proof Verification**:
```rust
fn verify_pbc_state_transition(
    pbc_id: ChainId,
    new_state_root: H256,
    block_height: u64,
) -> bool {
    let previous_state = get_last_known_state(pbc_id);

    // 1. Verify block height is sequential
    if block_height != previous_state.height + 1 {
        return false;
    }

    // 2. Verify state root is valid transition
    // (Would check Merkle proofs in production)

    // 3. Verify collator signature
    let collator = get_pbc_collator(pbc_id);
    verify_signature(collator, new_state_root)
}
```

### Cross-PBC Queries

Users can query state across all PBCs from FlareChain:

```rust
// Query BTC balance across all chains
let btc_balance = flarechain_api.get_cross_chain_balance(
    account_id,
    AssetType::Btc,
);

// Returns:
// {
//   btc_pbc: 1.5 BTC,
//   ethereum_locked: 0.5 wBTC,
//   total: 2.0 BTC equivalent
// }
```

---

## Cross-Chain Message Passing

### XCM (Cross-Consensus Messaging)

Ëtrid uses **XCM** (Polkadot's cross-chain messaging format) for communication between FlareChain and PBCs.

### Message Flow

**Scenario**: Transfer 1 BTC from BTC-PBC to FlareChain

```
┌───────────────┐                  ┌───────────────┐
│   BTC-PBC     │                  │  FlareChain   │
└───────────────┘                  └───────────────┘
        │                                  │
        │ 1. User calls transfer()         │
        ├─────────────────────────────────►│
        │    XCM Message:                  │
        │    - Withdraw 1 BTC              │
        │    - Deposit to AccountId        │
        │                                  │
        │ 2. FlareChain validates          │
        │◄─────────────────────────────────┤
        │    XCM Response:                 │
        │    - Message accepted            │
        │    - Nonce: 12345                │
        │                                  │
        │ 3. BTC-PBC locks 1 BTC           │
        │                                  │
        │ 4. FlareChain mints 1 BTC equiv  │
        │                                  │
        │ 5. Confirmation                  │
        │◄─────────────────────────────────┤
        │    - Transfer complete           │
        │    - New balance: 1 BTC          │
        │                                  │
```

### XCM Message Structure

```rust
use xcm::v3::*;

// Example XCM transfer message
let message = Xcm(vec![
    WithdrawAsset(MultiAssets::from(vec![
        MultiAsset {
            id: AssetId::Concrete(MultiLocation {
                parents: 1,
                interior: X2(Parachain(1000), GeneralKey(b"BTC".to_vec())),
            }),
            fun: Fungible(100_000_000),  // 1 BTC in satoshis
        }
    ])),
    BuyExecution {
        fees: /* fee asset */,
        weight_limit: Unlimited
    },
    DepositAsset {
        assets: Wild(All),
        beneficiary: MultiLocation {
            parents: 0,
            interior: X1(AccountId32 {
                network: Any,
                id: account_id.into()
            }),
        },
    },
]);
```

### pallet-xcm-bridge

**Location**: `/pallets/pallet-xcm-bridge/` (referenced in FlareChain runtime)

**Purpose**: Handles XCM message routing between FlareChain and PBCs

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    // Send XCM message to PBC
    pub fn send_to_pbc(
        origin: OriginFor<T>,
        pbc_id: ChainId,
        message: Xcm<()>,
    ) -> DispatchResult

    // Receive XCM message from PBC
    pub fn receive_from_pbc(
        pbc_id: ChainId,
        message: Xcm<()>,
    ) -> DispatchResult
}
```

### Message Ordering

**FIFO per PBC**:
- Messages from each PBC processed in order
- Nonce prevents replay attacks
- Timeout mechanism for stale messages

**Priority**:
```
High Priority: Bridge deposits/withdrawals
Normal Priority: Balance transfers
Low Priority: State queries
```

---

## Security Model

### Consensus Security

**ASF (Ascending Scale of Finality)**:
- **Phase 1**: Pre-commitment (fast, probabilistic)
- **Phase 2**: Commitment (2/3 validator agreement)
- **Phase 3**: Finality (irreversible, 100% BFT threshold)

**PPFA (Proposing Panel for Attestation)**:
- 21-member rotating committee
- Leader proposes blocks
- 2/3 quorum required for finality
- Rotates every 600 blocks (~1 hour)

**Validator Requirements**:
```rust
FlareNode (Validator):
- Minimum stake: 64 ËTR
- Uptime: >95%
- Hardware: 8 CPU, 32GB RAM, 1TB SSD

ValidityNode (Light Validator):
- Minimum stake: 16 ËTR
- Uptime: >90%
- Hardware: 4 CPU, 16GB RAM, 500GB SSD
```

### Bridge Security

**Multi-signature Vaults**:
- Bitcoin bridge: 5-of-9 multisig
- Ethereum bridge: 3-of-5 multisig (transitioning to smart contract)
- All bridges: Governance-controlled authority rotation

**Confirmation Depths**:
```
Bitcoin: 6 confirmations
Ethereum: 12 confirmations
Dogecoin: 40 confirmations
Others: Chain-specific
```

**Daily Limits** (Circuit Breakers):
```
Per Bridge: 100 BTC equivalent
Per User: 10 BTC equivalent
Total System: 1000 BTC equivalent
```

### EDSC Security

**M-of-N Attestation**:
- 3-of-5 attesters required
- Independent attester nodes
- Signature verification on-chain

**Reserve Ratio Enforcement**:
```
RR = (On-chain Vault + Custodian Attested) / Total EDSC Supply

Targets:
- Optimal: 110-130%
- Throttle: 105% (slow redemptions)
- Emergency: 100% (pause all)
```

**Peg Defense**:
```rust
if market_price < $0.98:
    increase_redemption_fees()
    trigger_automated_buybacks()

if market_price > $1.02:
    reduce_minting_fees()
    increase_supply_cap()
```

### Slashing Conditions

**Validators**:
- Double-signing: 100% stake slashed
- Downtime (>5% missed blocks): 1% stake slashed per day
- Invalid state submission: 50% stake slashed

**Bridge Authorities**:
- Unauthorized withdrawal: 100% bond slashed + criminal prosecution
- Delayed processing (>24h): 10% bond slashed
- Invalid attestation: 50% bond slashed

**EDSC Attesters**:
- False attestation: 100% bond slashed
- Downtime (>10%): 5% bond slashed per week
- Collusion: Permanent ban + 100% slash

---

## Performance Characteristics

### Block Production

**FlareChain**:
- Block time: 6 seconds
- Finality time: ~18-30 seconds (ASF 3-level)
- TPS capacity: ~500-1000 TPS (estimated)
- Block size limit: 5 MB

**PBCs** (each):
- Block time: 12 seconds (slower than FlareChain)
- Finality time: Inherits from FlareChain (~30s)
- TPS capacity: ~100-200 TPS per PBC
- Block size limit: 3 MB

**Total System**:
- Combined TPS: ~2000-3000 TPS (13 chains)
- Cross-chain latency: ~30-60 seconds
- State aggregation overhead: ~5-10% per block

### Resource Usage

**FlareChain Validator**:
```
CPU: 4-8 cores actively used
RAM: 16-32 GB (state database + cache)
Disk: 500 GB (grows ~50GB/year)
Network: 100 Mbps sustained, 1 Gbps burst
```

**PBC Collator**:
```
CPU: 2-4 cores
RAM: 8-16 GB
Disk: 200 GB per PBC
Network: 50 Mbps sustained
```

**EDSC Bridge Services**:
```
Attestation Service:
- CPU: 2 cores
- RAM: 4 GB
- Disk: 50 GB
- Network: 10 Mbps

Relayer Service:
- CPU: 2 cores
- RAM: 4 GB
- Disk: 50 GB
- Network: 20 Mbps (high burst)
```

### Storage Growth

**FlareChain**:
- State size: ~10 GB initially
- Growth rate: ~50 GB/year
- Pruning: Available after 256 blocks (30 minutes)

**PBC** (each):
- State size: ~2 GB initially
- Growth rate: ~10 GB/year
- Pruning: Available after 256 blocks

**Total System**:
- Initial storage: ~40 GB (FlareChain + 13 PBCs)
- Annual growth: ~180 GB/year
- Pruned nodes: ~50 GB maintained

### Network Bandwidth

**Peak Usage** (All 13 Chains):
```
Block propagation: 50 MB/min
State sync: 200 MB/min (new nodes)
Cross-chain messages: 10 MB/min
Total: ~260 MB/min = ~4.3 MB/s
```

**Typical Usage**:
```
~50-100 MB/min sustained
~1-2 MB/s average
```

---

## Known Issues & Technical Debt

### Critical Issues

**1. PBC Code Duplication (92.6%)**

**Problem**: All 12 PBC runtimes are nearly identical (only differ in bridge pallet name).

**Impact**:
- Maintenance: Bug fixes require 12 identical changes
- Storage: 564 MB binaries (12 × 47MB)
- Auditing: Difficult to track changes across 12 files

**Example**:
```rust
// btc-pbc/runtime/src/lib.rs (629 lines)
construct_runtime! {
    BitcoinBridge: pallet_bitcoin_bridge,  // Only difference
    // ... 600+ identical lines
}

// eth-pbc/runtime/src/lib.rs (629 lines)
construct_runtime! {
    EthereumBridge: pallet_ethereum_bridge,  // Only difference
    // ... 600+ identical lines
}
```

**Proposed Solution**:
```rust
// Generic PBC runtime (future work)
pub struct PbcRuntime<B: BridgePallet> {
    bridge: PhantomData<B>,
}

type BtcPbcRuntime = PbcRuntime<BitcoinBridge>;
type EthPbcRuntime = PbcRuntime<EthereumBridge>;
// etc.
```

**Effort**: 2-3 weeks
**Priority**: Medium (functional but not optimal)

---

**2. EDSC-PBC Not Yet Built**

**Problem**: EDSC-PBC collator not yet created (13th PBC missing).

**Impact**: Cannot test EDSC cross-chain transfers in full multichain environment.

**Required**:
- Create `pbc-chains/edsc-pbc/` directory structure
- Implement EDSC-PBC runtime (similar to other PBCs)
- Build collator binary
- Integrate with FlareChain

**Effort**: 1 week
**Priority**: High (blocks EDSC Phase 3 completion)

---

**3. ASF Consensus Not Fully Integrated**

**Problem**: ASF service structure exists but workers are placeholders.

**Status** (from ASF_INTEGRATION.md):
- ✅ Service structure complete
- ✅ Import queue defined
- ✅ Configuration parameters set
- ❌ Block production worker pending
- ❌ Finality gadget worker pending
- ❌ Validator management worker pending

**Impact**: FlareChain currently uses AURA + GRANDPA (traditional Substrate consensus).

**Effort**: 2-3 weeks
**Priority**: High (core feature)

---

### Non-Critical Issues

**4. Hard-coded Genesis Keys**

**Problem**: All PBC preset files use well-known test keys (Alice, Bob, Charlie).

**Impact**: **CRITICAL SECURITY RISK if used in production**.

**Example**:
```json
// presets/development.json (all PBCs)
{
  "balances": {
    "balances": [
      ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 1000000000000000]
    ]
  }
}
```

These are publicly known keys from Substrate's test suite.

**Solution**: Generate unique keypairs for production deployment.

**Priority**: Critical before mainnet launch

---

**5. Missing Weight Benchmarks**

**Problem**: All pallets use hard-coded weights.

**Example**:
```rust
#[pallet::weight(10_000)]  // Hard-coded, not benchmarked
pub fn transfer(origin, to, amount) -> DispatchResult
```

**Impact**: Inaccurate fees, potential DoS vectors.

**Solution**: Run `frame-benchmarking` on all pallets.

**Effort**: 1 week
**Priority**: Medium (required for production)

---

**6. Limited Test Coverage**

**Problem**: Most pallets lack comprehensive unit tests.

**Example**:
```rust
// bitcoin-bridge/src/tests.rs
#[cfg(test)]
mod tests {
    // TODO: Add tests
}
```

**Impact**: Bugs may slip through to production.

**Solution**: Add unit tests (target: >80% coverage).

**Effort**: 2-3 weeks
**Priority**: High (before mainnet)

---

**7. EDSC Reserve Backing Not Implemented**

**Problem**: EDSC has no on-chain or off-chain reserves (from EDSC_BRIDGE_STATUS.md).

**Impact**: EDSC is currently unbacked and worthless.

**Required Pallets**:
- `pallet-reserve-vault` - On-chain collateral (BTC, ETH, ËTR, USDC)
- `pallet-custodian-registry` - Off-chain reserves (fiat, T-Bills)

**Effort**: 2-3 weeks
**Priority**: Critical (EDSC cannot launch without reserves)

---

### Technical Debt

**8. Lightning Bloc Networks Incomplete**

**Status**: Basic structure exists, implementation pending.

**Missing**:
- Channel lifecycle logic
- P2P network protocol
- State update mechanism
- Dispute resolution

**Effort**: 4-6 weeks
**Priority**: Low (Phase 4 feature)

---

**9. DETR P2P Not Integrated**

**Problem**: FlareChain uses libp2p (default Substrate), not DETR P2P.

**Impact**: Missing Ëtrid-specific P2P features (S/Kademlia, AEComms).

**Solution**: Integrate `/01-detr-p2p/` into FlareChain node.

**Effort**: 3-4 weeks
**Priority**: Medium (Phase 3-4 feature)

---

**10. No Runtime Upgrade Logic**

**Problem**: Missing forkless upgrade mechanism.

**Impact**: Chain upgrades require hard forks.

**Solution**: Implement `frame_support::traits::OnRuntimeUpgrade`.

**Effort**: 1 week
**Priority**: Medium (before mainnet)

---

## Integration Points

### With Other Ëtrid Components

**09-consensus** (ASF Consensus):
```
09-consensus/asf-algorithm → 05-multichain/flare-chain/node/src/asf_service.rs
09-consensus/block-production → FlareChain block authoring
09-consensus/finality-gadget → FlareChain finality
09-consensus/validator-management → FlareChain validator registry
```

**04-accounts** (Account System):
```
04-accounts/pallet → FlareChain runtime
Account types (EBCA, RCA, SSCA) → Used in all bridge pallets
DID system → Used for validator identity
```

**06-native-currency** (ËTR/ETD Tokens):
```
06-native-currency/pallet → FlareChain runtime
ËTR token → Used for staking, fees
ETD stablecoin → Used alongside EDSC
```

**07-transaction** (Transaction System):
```
07-transaction/pallet → FlareChain runtime
5 TX types → Includes LightningBloc cross-chain transactions
```

**08-etwasm-vm** (Smart Contracts):
```
08-etwasm-vm/pallet → FlareChain runtime
WASM contracts → Can call bridge pallets
```

**10-foundation** (Governance):
```
10-foundation/pallet → FlareChain runtime
Governance → Controls bridge authorities, EDSC parameters
Consensus Day → Minting, proposals, voting
```

**01-detr-p2p** (P2P Networking):
```
01-detr-p2p/ → (Future) FlareChain node networking
S/Kademlia → Secure peer discovery
AEComms → Encrypted TCP communication
```

**pallets/** (Additional Pallets):
```
pallets/pallet-xcm-bridge → FlareChain runtime (cross-chain messaging)
pallets/pallet-reserve-vault → (Future) EDSC reserve backing
pallets/pallet-custodian-registry → (Future) EDSC off-chain reserves
```

### External Integrations

**apps/wallet-web** (Web Wallet):
```typescript
// Connect to FlareChain
const api = await ApiPromise.create({
    provider: new WsProvider('ws://localhost:9944')
});

// Query EDSC balance
const balance = await api.query.edscToken.balances(accountId);

// Transfer EDSC cross-chain
await api.tx.edscBridgeTokenMessenger.burnAndSend(
    destinationDomain,
    recipient,
    amount
).signAndSend(keypair);
```

**apps/wallet-mobile** (Mobile Wallet):
```dart
// Flutter Polkadot SDK integration
final api = PolkadotDart.create('wss://flarechain.etrid.network');

// Query multichain balances
final btcBalance = await api.query.btcPbc.balances(accountId);
final ethBalance = await api.query.ethPbc.balances(accountId);
```

**services/attestation-service** (EDSC Bridge):
```typescript
// Monitor FlareChain events
api.query.system.events((events) => {
    events.forEach(({ event }) => {
        if (event.method === 'BurnAndSend') {
            signAndAttest(event.data);
        }
    });
});
```

**contracts/ethereum** (EDSC Ethereum Contracts):
```solidity
// Call from Ethereum to Ëtrid
IERC20(edscToken).approve(messenger, amount);
IEDSCTokenMessenger(messenger).burnAndSendTo(
    2,  // Ëtrid domain
    recipient,
    amount
);
```

---

## Build System & Deployment

### Building FlareChain

**Location**: `/05-multichain/flare-chain/`

```bash
# Build FlareChain node
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain
cargo build --release

# Output
# Binary: target/release/flarechain-node (55MB)
# WASM: target/release/wbuild/flare-chain-runtime/*.wasm
```

**Build time**: ~15-20 minutes (first build)
**Incremental builds**: ~2-5 minutes

---

### Building PBC Collators

**Location**: `/05-multichain/partition-burst-chains/pbc-node/pbc-collator-nodes/`

```bash
# Build all PBCs in parallel (recommended)
cd /Users/macbook/Desktop/etrid
./build_all_remaining_pbcs.sh

# Or build individual PBC
cargo build --release -p btc-pbc-collator

# Output (each PBC)
# Binary: target/release/btc-pbc-collator (47MB)
# WASM: target/release/wbuild/btc-pbc-runtime/*.wasm (471-485KB)
```

**Build time per PBC**: ~5-10 minutes
**Total time (12 PBCs in parallel)**: ~60-90 minutes

---

### Chain Spec Generation

**FlareChain**:
```bash
./target/release/flarechain-node build-spec --chain dev > flarechain-dev.json
./target/release/flarechain-node build-spec --chain dev --raw > flarechain-dev-raw.json
```

**PBCs**:
```bash
./target/release/btc-pbc-collator build-spec --chain dev > btc-pbc-dev.json
```

**Shared Chain Spec** (for multi-node testing):
```bash
# FlareChain shared spec for all PBCs to connect
./target/release/flarechain-node build-spec --chain local > chain-specs/flarechain-shared.json
```

---

### Deployment Scenarios

#### 1. Single-Node Development

```bash
# Start FlareChain in dev mode
./target/release/flarechain-node --dev --tmp

# Access
# RPC: http://localhost:9933
# WebSocket: ws://localhost:9944
# Prometheus: http://localhost:9615
```

#### 2. Multi-Node Testnet

```bash
# Alice (Validator)
./target/release/flarechain-node \
  --chain chain-specs/flarechain-shared.json \
  --alice \
  --validator \
  --base-path /tmp/alice \
  --port 30333 \
  --rpc-port 9933

# Bob (Validator)
./target/release/flarechain-node \
  --chain chain-specs/flarechain-shared.json \
  --bob \
  --validator \
  --base-path /tmp/bob \
  --port 30334 \
  --rpc-port 9934 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<Alice's peer ID>
```

#### 3. FlareChain + PBC Collators

```bash
# 1. Start FlareChain relay
./target/release/flarechain-node \
  --chain chain-specs/flarechain-shared.json \
  --alice --validator

# 2. Start BTC PBC collator
./target/release/btc-pbc-collator \
  --dev \
  --relay-chain-rpc ws://127.0.0.1:9944 \
  --port 30335 \
  --rpc-port 9945

# 3. Start ETH PBC collator
./target/release/eth-pbc-collator \
  --dev \
  --relay-chain-rpc ws://127.0.0.1:9944 \
  --port 30336 \
  --rpc-port 9946

# ... repeat for all 12 PBCs
```

#### 4. Full Multichain Test

**Script**: `/test_full_multichain.sh`

```bash
#!/bin/bash
# Starts FlareChain + all 12 PBC collators

# Start FlareChain
./target/release/flarechain-node --chain chain-specs/flarechain-shared.json --alice --validator &

# Start all PBCs
for pbc in btc eth doge sol xlm xrp bnb trx ada link matic sc-usdt; do
    ./target/release/${pbc}-pbc-collator --dev --relay-chain-rpc ws://127.0.0.1:9944 &
done

# Wait for all to start
sleep 30

# Health check
curl http://localhost:9933/health  # FlareChain
curl http://localhost:9945/health  # BTC-PBC
# ... etc
```

---

### Docker Deployment

**EDSC Bridge Docker Stack** (from EDSC README.md):

```bash
# Start full EDSC bridge infrastructure
docker-compose -f docker-compose.bridge.yml up

# Includes:
# - FlareChain node
# - Ethereum Hardhat node
# - 3 attestation services
# - 1 relayer service
# - Prometheus + Grafana
```

**Testnet Deployment**: See `/EMBER_DEPLOYMENT_CHECKLIST.md`

---

### Testing Scripts

**Location**: `/Users/macbook/Desktop/etrid/`

| Script | Purpose | Status |
|--------|---------|--------|
| test_all_chain_specs.sh | Verify chain spec generation (all 12 PBCs) | ✅ Created |
| test_bridge_basic.sh | Test FlareChain + BTC PBC bridge | ✅ Created |
| test_full_multichain.sh | Test all 13 chains simultaneously | ✅ Created |

**Usage**:
```bash
# Verify chain spec generation (100% pass rate)
./test_all_chain_specs.sh

# Test single bridge
./test_bridge_basic.sh

# Test full multichain
./test_full_multichain.sh
```

---

## Future Roadmap

### Phase 2: Testing & Integration (Current - Oct 2025)

**Tasks**:
- ✅ Chain spec generation test (100% pass rate)
- ✅ Bridge basic test created
- ✅ Full multichain test created
- ⏳ Execute multichain integration test
- ⏳ Monitor for runtime errors
- ⏳ Performance benchmarking

**Duration**: 3-5 days
**Deliverables**: Test results, performance metrics

---

### Phase 3: EDSC-PBC Implementation (Nov 2025)

**Tasks**:
1. Create EDSC-PBC runtime
2. Build EDSC-PBC collator
3. Implement reserve vault pallet
4. Implement custodian registry pallet
5. Full EDSC bridge testing
6. Stress testing (death spiral scenarios)

**Duration**: 2-3 weeks
**Deliverables**: EDSC-PBC operational, reserves backed

---

### Phase 4: Frontend Integration (Nov-Dec 2025)

**Tasks**:
1. Integrate mobile wallet (`apps/wallet-mobile/`)
2. Integrate web wallet (`apps/wallet-web/`)
3. Connect to all 13 chains
4. Implement staking UI
5. Implement governance UI (Consensus Day)
6. Testing with real users

**Duration**: 2-3 weeks
**Deliverables**: Production-ready wallets

---

### Phase 5: Performance & Security (Dec 2025)

**Tasks**:
1. ASF consensus full integration
2. Weight benchmarking (all pallets)
3. Security audit (external firm)
4. Load testing (1000+ TPS)
5. Fuzz testing
6. Bug bounty program

**Duration**: 2-3 weeks
**Deliverables**: Audit report, performance benchmarks

---

### Phase 6: Testnet Deployment (Jan 2026)

**Tasks**:
1. Deploy to Ember testnet (public)
2. Validator onboarding (50+ validators)
3. Community testing
4. Bug fixes from testnet feedback
5. Economic parameter tuning

**Duration**: 2-3 weeks
**Deliverables**: Stable testnet with >50 validators

---

### Phase 7: Mainnet Preparation (Feb 2026)

**Tasks**:
1. Final security audit
2. Mainnet genesis preparation
3. Validator recruitment (100+ validators)
4. Documentation finalization
5. Community governance setup

**Duration**: 2 weeks
**Deliverables**: Mainnet-ready codebase

---

### Phase 8: Mainnet Launch (Mar 2026)

**Tasks**:
1. Genesis block T+0
2. Initial validator set activation
3. Bridge activation (phased)
4. Monitoring and support (24/7)
5. Post-launch optimizations

**Duration**: 1 week
**Deliverables**: Mainnet LIVE

**Target Date**: March 2026

---

### Long-term Features (2026-2027)

**Q2 2026**:
- DETR P2P full integration
- Lightning Bloc Networks launch
- Additional PBCs (Avalanche, Fantom, etc.)

**Q3 2026**:
- Cross-chain smart contracts (WASM)
- Zero-knowledge proofs for bridge privacy
- Sharding for >10,000 TPS

**Q4 2026**:
- DAO governance full transition
- Treasury management by community
- Decentralized oracle network

**2027**:
- Layer 2 scaling solutions
- Mobile-first features
- Enterprise adoption program

---

## Summary

The **05-multichain** component is the **largest and most complex** part of Ëtrid, comprising:

- **FlareChain**: Main relay chain with ASF consensus
- **13 PBCs**: Asset-specific sidechains
- **12 External Bridges**: Cross-chain connectivity
- **1 EDSC Bridge**: Native stablecoin (CCTP-style)
- **Primitives**: Shared types and VMw metering
- **Lightning Bloc**: Payment channel infrastructure

**Current Status**:
- ✅ Phase 1 Infrastructure: Complete (all 13 chains built)
- ⏳ Phase 2 Testing: In progress
- 🚧 Phase 3 EDSC: Pending
- 🚧 Phase 4+ Deployment: Future

**Key Achievements**:
- All 12 PBC collators operational (47MB each)
- All WASM runtimes built (471-485KB each)
- GenesisBuilder API implemented across all chains
- Bridge pallets integrated into FlareChain
- EDSC bridge core pallets complete

**Known Issues**:
- 92.6% code duplication across PBC runtimes
- EDSC-PBC collator not yet built
- ASF consensus not fully integrated
- EDSC reserve backing pending
- Weight benchmarks missing

**Next Steps**:
- Execute full multichain integration test
- Complete EDSC-PBC implementation
- Integrate ASF consensus workers
- Launch testnet for community validation

---

**Document Version**: 1.0
**Total Lines**: ~1,500 lines
**Last Updated**: October 20, 2025
**Maintainer**: Ëtrid Foundation
**Status**: Living Document (Updated Regularly)

---

**References**:
- FlareChain ASF Integration: `flare-chain/ASF_INTEGRATION.md`
- EDSC Bridge README: `bridge-protocols/edsc-bridge/README.md`
- EDSC Status Report: `/EDSC_BRIDGE_STATUS.md`
- Project History: `/PROJECT_HISTORY.md`
- Development Roadmap: `/DEVELOPMENT_ROADMAP.md`
- Consensus Architecture: `/09-consensus/ARCHITECTURE.md`

---

**For questions or contributions, see**: `/DEVELOPER_GUIDE.md`
