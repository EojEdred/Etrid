# Ëtrid Developer Guide

**Version**: 1.0.0
**Last Updated**: October 22, 2025
**Status**: Alpha Complete (100%)

Welcome to the Ëtrid blockchain platform developer guide! This comprehensive guide will help you build applications, custom pallets, and smart contracts on the Ëtrid multichain ecosystem.

---

## Table of Contents

1. [Introduction to Etrid Development](#introduction-to-etrid-development)
2. [Architecture Overview](#architecture-overview)
3. [Development Environment Setup](#development-environment-setup)
4. [Building Custom Pallets](#building-custom-pallets)
5. [Developing DApps](#developing-dapps)
6. [Smart Contract Development](#smart-contract-development)
7. [SDK Usage Guide](#sdk-usage-guide)
8. [Testing and Debugging](#testing-and-debugging)
9. [Best Practices](#best-practices)
10. [Contributing to Etrid](#contributing-to-etrid)

---

## Introduction to Etrid Development

### What is Ëtrid?

Ëtrid is a next-generation multichain blockchain platform built on Substrate that implements:

- **FlareChain Relay Chain**: Main chain using Ascending Scale of Finality (ASF) consensus
- **13 Partition Burst Chains (PBCs)**: Specialized chains for cross-chain asset bridges
- **Lightning-Bloc Layer 2**: Payment channel network for instant transactions
- **ËtwasmVM**: Custom WebAssembly runtime with reentrancy protection
- **OpenDID + AIDID**: Self-sovereign identity including world's first AI DID standard
- **On-Chain Governance**: Annual Consensus Day for fiscal policy and upgrades

### E³20 Protocol Components

Ëtrid implements the Essential Elements to Operate (E³20) protocol with 13 core components:

| # | Component | Description | Location |
|---|-----------|-------------|----------|
| 01 | DETR P2P | Lightning-Bloc payment channels | `01-detr-p2p/`, `07-transactions/lightning-bloc/` |
| 02 | OpenDID | Identity system (human + AI) | `02-open-did/`, `pallets/pallet-did-registry/` |
| 03 | Security | Cryptographic primitives | `03-security/` |
| 04 | Accounts | Account types + social recovery | `04-accounts/` |
| 05 | Multichain | FlareChain + 13 PBCs + bridges | `05-multichain/` |
| 06 | Native Currency | ÉTR, EDSC, VMw tokens | `06-native-currency/` |
| 07 | Transactions | Ed25519 + HTLCs + smart/regular | `07-transactions/` |
| 08 | ËtwasmVM | WebAssembly runtime | `08-etwasm-vm/` |
| 09 | Consensus | ASF finality + watchtowers | `09-consensus/` |
| 10 | Foundation | Stake-weighted governance | `10-foundation/` |
| 11 | Peer Roles | Staking + nominations | `11-peer-roles/` |
| 12 | Consensus Day | Annual governance event | `12-consensus-day/` |
| 13 | Clients | Wallets, CLI, 4 SDKs | `13-clients/` |

### Developer Resources

- **Documentation**: `/docs/` directory and component-specific `ARCHITECTURE.md` files
- **Code Examples**: This guide + SDK quick start guides
- **API Reference**: Generated from pallet documentation
- **Community**: Discord, GitHub Discussions

---

## Architecture Overview

### System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Ëtrid Ecosystem                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              FlareChain (Relay Chain)                      │  │
│  │  - ASF Consensus (Ascending Scale of Finality)                 │  │
│  │  - Validator Set Management                                │  │
│  │  - Cross-Chain Message Routing                             │  │
│  │  - Governance & Treasury                                   │  │
│  │  - State Anchoring for all PBCs                            │  │
│  └───────────────────────────────────────────────────────────┘  │
│                            ↓ ↑                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │         13 Partition Burst Chains (PBCs)                   │  │
│  ├───────────────────────────────────────────────────────────┤  │
│  │  BTC  │ ETH  │ DOGE │ SOL  │ XLM  │ XRP  │ BNB           │  │
│  │  TRX  │ ADA  │ LINK │ MATIC│ USDT │ EDSC │               │  │
│  │                                                             │  │
│  │  Each PBC: Dedicated collator + bridge + specialized runtime│  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │              Layer 2: Lightning-Bloc                        │  │
│  │  - Payment channels (HTLC-based)                            │  │
│  │  - Multi-hop routing (up to 20 hops)                        │  │
│  │  - Watchtower network for security                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

### Core Technology Stack

**Blockchain Layer**:
- Framework: Substrate (Polkadot SDK)
- Language: Rust 1.70+
- Runtime: FRAME pallets
- VM: ËtwasmVM (WebAssembly)
- Database: RocksDB / ParityDB
- Networking: libp2p with QUIC

**Cryptography**:
- Signatures: Ed25519 (ed25519-dalek v2.2.0)
- Key Exchange: X25519 (x25519-dalek v2.0.1)
- Hashing: SHA-256 (RustCrypto)
- KDF: HKDF-SHA256 (RFC 5869)

**Client Layer**:
- Web: React, Next.js 15, TypeScript, TailwindCSS
- Mobile: Flutter 3.0+, Dart
- CLI: Rust (clap, tokio)
- SDKs: Rust, JavaScript/TypeScript, Python, Swift

### Pallet Architecture

Ëtrid uses Substrate's FRAME (Framework for Runtime Aggregation of Modularized Entities) to build custom blockchain logic. Key pallets include:

**Core Pallets** (in `/pallets/`):
- `pallet-reserve-oracle`: EDSC reserve data aggregation
- `pallet-reserve-vault`: On-chain collateral management
- `pallet-custodian-registry`: Multi-sig bridge custodians
- `pallet-did-registry`: OpenDID implementation
- `pallet-aidid`: AI Decentralized Identifiers
- `pallet-circuit-breaker`: Safety controls for EDSC
- `pallet-validator-committee`: Validator management
- `pallet-xcm-bridge`: Cross-chain messaging

**Runtime Integration**:
Each chain (FlareChain + 13 PBCs) has its own runtime that combines relevant pallets:
- FlareChain: Full feature set (governance, staking, consensus)
- PBCs: Specialized runtimes (e.g., BTC-PBC focuses on Bitcoin bridge)

### Consensus Mechanism

**Ascending Scale of Finality (ASF)**:
- Combines Proof of Stake with "coinage" (stake × time)
- Dilutes voting power over time to prevent centralization
- Fast finality: ~15 seconds (3 blocks)
- Block time: 5 seconds
- Requires 2/3+ stake for consensus

### Networking

**DETR P2P Layer** (`01-detr-p2p/`):
- Built on libp2p
- Post-quantum encryption (Component 03)
- Peer discovery and routing
- NAT traversal
- Protocol versioning

**Cross-Chain Communication**:
- XCM (Cross-Consensus Message) format
- State checkpoints from PBCs to FlareChain
- Message routing through FlareChain
- Multi-sig bridge custodians for security

---

## Development Environment Setup

### Prerequisites

#### System Requirements

- **OS**: Linux (Ubuntu 20.04+), macOS (11+), or Windows (WSL2)
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 50GB+ free space
- **CPU**: 4+ cores recommended
- **Network**: Stable internet connection

#### Required Software

**For Rust/Substrate Development**:

1. **Install Rust** (1.70+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env

   # Update to latest stable
   rustup update stable
   rustup default stable

   # Add WebAssembly target
   rustup target add wasm32-unknown-unknown

   # Add useful components
   rustup component add rustfmt clippy
   ```

2. **Install Build Dependencies**:

   **Ubuntu/Debian**:
   ```bash
   sudo apt update
   sudo apt install -y \
     build-essential \
     pkg-config \
     libssl-dev \
     git \
     clang \
     cmake \
     curl
   ```

   **macOS**:
   ```bash
   # Install Homebrew if not already installed
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

   # Install dependencies
   brew install cmake pkg-config openssl git
   ```

3. **Install Substrate Tools** (optional):
   ```bash
   cargo install --git https://github.com/paritytech/substrate subkey --force
   cargo install cargo-contract --version 3.0.1
   ```

**For JavaScript/TypeScript Development**:

1. **Install Node.js** (18+):
   ```bash
   # Using nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   source ~/.bashrc  # or ~/.zshrc
   nvm install 18
   nvm use 18

   # Verify installation
   node --version  # Should be v18.x.x+
   npm --version   # Should be v9.x.x+
   ```

2. **Install TypeScript**:
   ```bash
   npm install -g typescript ts-node
   ```

**For Mobile Development**:

1. **Install Flutter** (3.0+):
   ```bash
   # Download Flutter SDK
   git clone https://github.com/flutter/flutter.git -b stable
   export PATH="$PATH:`pwd`/flutter/bin"

   # Verify installation
   flutter doctor
   ```

2. **Platform-Specific Tools**:
   - **Android**: Install Android Studio
   - **iOS**: Install Xcode (macOS only)

**For Smart Contract Development**:

1. **Install Solidity Tools**:
   ```bash
   # Install Node.js dependencies
   npm install -g hardhat
   npm install -g @openzeppelin/contracts
   ```

### IDE Setup

#### Recommended IDEs

**VS Code** (Recommended for all development):
```bash
# Install VS Code
# Download from: https://code.visualstudio.com/
```

**Recommended Extensions**:
- **Rust**: `rust-analyzer` (essential)
- **Substrate**: `Substrate Dev Tools`
- **TypeScript**: `ESLint`, `Prettier`
- **Flutter**: `Dart`, `Flutter`
- **Solidity**: `Solidity` by Juan Blanco
- **General**: `GitLens`, `Error Lens`, `Todo Tree`

**VS Code Settings** (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "files.exclude": {
    "**/target": true,
    "**/node_modules": true
  }
}
```

#### IntelliJ IDEA / CLion

Alternative for Rust development:
- Install IntelliJ Rust plugin
- Configure Cargo project
- Enable Clippy and rustfmt integration

### Clone and Build

#### 1. Clone the Repository

```bash
# Clone Etrid repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid

# Create a feature branch
git checkout -b feature/my-feature
```

#### 2. Build Workspace

**Note**: Due to Polkadot SDK dependency stabilization, some builds may require specific steps. See `KNOWN_ISSUES.md` for current status.

**Build FlareChain**:
```bash
# Build FlareChain runtime (WASM)
cargo build --release -p flare-chain-runtime

# Build FlareChain node binary
cargo build --release -p flare-chain-node
```

**Build Specific PBC**:
```bash
# Example: Build EDSC-PBC
cargo build --release -p edsc-pbc-runtime
cargo build --release -p edsc-pbc-node

# Build BTC-PBC
cargo build --release -p btc-pbc-runtime
cargo build --release -p btc-pbc-node
```

**Build All PBC Runtimes** (requires ~30 minutes):
```bash
./scripts/build_all_wasm_runtimes.sh
```

#### 3. Run Tests

**Unit Tests**:
```bash
# Test all workspace crates
cargo test --workspace

# Test specific pallet
cargo test -p pallet-reserve-oracle

# Test with output
cargo test --workspace -- --nocapture
```

**Integration Tests**:
```bash
# Run integration test suite
cargo test -p integration-tests
```

**Coverage Report** (optional):
```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --workspace --out Html
# View coverage/index.html
```

### Running Local Development Node

#### Single Node (Development Mode)

```bash
# Run FlareChain in dev mode
./target/release/flare-chain-node \
  --dev \
  --tmp \
  --rpc-cors all \
  --rpc-methods=Unsafe \
  --rpc-external

# Node will start at:
# - HTTP RPC: http://localhost:9933
# - WebSocket: ws://localhost:9944
```

#### Multi-Node Local Testnet

```bash
# Generate chain spec
./scripts/generate_chain_specs.sh

# Deploy local testnet (3 validators)
./scripts/deploy_local_testnet.sh

# Nodes will start at:
# - Alice: ws://localhost:9944
# - Bob: ws://localhost:9945
# - Charlie: ws://localhost:9946
```

#### PBC Collator Node

```bash
# Run EDSC-PBC collator
./target/release/edsc-pbc-node \
  --collator \
  --chain=edsc-pbc-dev \
  --tmp \
  --rpc-cors all \
  --ws-port 9955
```

### Connecting to Polkadot.js Apps

1. **Open**: https://polkadot.js.org/apps/
2. **Connect**: Click top-left network selector
3. **Choose**: "Development" → "Local Node" (ws://127.0.0.1:9944)
4. **Custom Types**: Upload `chain-specs/types.json` if needed

---

## Building Custom Pallets

Pallets are modular components that define blockchain logic in Substrate. This section guides you through creating a custom pallet from scratch.

### Pallet Structure and Anatomy

A typical pallet consists of:

```
pallet-name/
├── Cargo.toml          # Dependencies and metadata
└── src/
    ├── lib.rs          # Main pallet logic
    ├── mock.rs         # Test runtime environment
    ├── tests.rs        # Unit tests
    ├── benchmarking.rs # Weight benchmarks (optional)
    └── weights.rs      # Generated weights (optional)
```

### Basic Pallet Template

Create `/pallets/pallet-voting/src/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::Currency,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    /// Configure the pallet by specifying the parameters and types.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The currency type for handling deposits.
        type Currency: Currency<Self::AccountId>;

        /// Minimum deposit to create a proposal.
        #[pallet::constant]
        type MinimumDeposit: Get<BalanceOf<Self>>;

        /// Voting period in blocks.
        #[pallet::constant]
        type VotingPeriod: Get<BlockNumberFor<Self>>;
    }

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Proposal data structure.
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Proposal<AccountId, BlockNumber> {
        /// Proposal creator.
        pub proposer: AccountId,
        /// Title of the proposal.
        pub title: BoundedVec<u8, ConstU32<256>>,
        /// Description.
        pub description: BoundedVec<u8, ConstU32<1024>>,
        /// Block when voting ends.
        pub voting_deadline: BlockNumber,
        /// Number of yes votes.
        pub votes_for: u32,
        /// Number of no votes.
        pub votes_against: u32,
        /// Whether proposal is active.
        pub active: bool,
    }

    /// The pallet struct.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Storage: Proposals by ID.
    #[pallet::storage]
    #[pallet::getter(fn proposals)]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // Proposal ID
        Proposal<T::AccountId, BlockNumberFor<T>>,
        OptionQuery,
    >;

    /// Storage: Next proposal ID.
    #[pallet::storage]
    #[pallet::getter(fn next_proposal_id)]
    pub type NextProposalId<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Storage: Track user votes (ProposalId -> AccountId -> bool).
    #[pallet::storage]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32, // Proposal ID
        Blake2_128Concat,
        T::AccountId,
        bool, // true = yes, false = no
        OptionQuery,
    >;

    /// Events emitted by the pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Proposal created [proposal_id, proposer].
        ProposalCreated {
            proposal_id: u32,
            proposer: T::AccountId,
        },
        /// Vote cast [proposal_id, voter, vote].
        VoteCast {
            proposal_id: u32,
            voter: T::AccountId,
            vote: bool,
        },
        /// Proposal finalized [proposal_id, passed].
        ProposalFinalized {
            proposal_id: u32,
            passed: bool,
        },
    }

    /// Errors emitted by the pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Proposal not found.
        ProposalNotFound,
        /// Proposal voting period ended.
        VotingPeriodEnded,
        /// User already voted.
        AlreadyVoted,
        /// Proposal is not active.
        ProposalNotActive,
        /// Title too long.
        TitleTooLong,
        /// Description too long.
        DescriptionTooLong,
    }

    /// Extrinsics (callable functions).
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new proposal.
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let proposer = ensure_signed(origin)?;

            // Validate inputs
            let bounded_title: BoundedVec<u8, ConstU32<256>> = title.try_into()
                .map_err(|_| Error::<T>::TitleTooLong)?;
            let bounded_description: BoundedVec<u8, ConstU32<1024>> = description.try_into()
                .map_err(|_| Error::<T>::DescriptionTooLong)?;

            // Reserve deposit
            let deposit = T::MinimumDeposit::get();
            T::Currency::reserve(&proposer, deposit)?;

            // Get next proposal ID
            let proposal_id = NextProposalId::<T>::get();
            let next_id = proposal_id.checked_add(1).ok_or(Error::<T>::ProposalNotFound)?;
            NextProposalId::<T>::put(next_id);

            // Calculate voting deadline
            let current_block = <frame_system::Pallet<T>>::block_number();
            let voting_deadline = current_block + T::VotingPeriod::get();

            // Create proposal
            let proposal = Proposal {
                proposer: proposer.clone(),
                title: bounded_title,
                description: bounded_description,
                voting_deadline,
                votes_for: 0,
                votes_against: 0,
                active: true,
            };

            // Store proposal
            Proposals::<T>::insert(proposal_id, proposal);

            // Emit event
            Self::deposit_event(Event::ProposalCreated {
                proposal_id,
                proposer,
            });

            Ok(())
        }

        /// Cast a vote on a proposal.
        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_id: u32,
            vote: bool, // true = yes, false = no
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            // Get proposal
            let mut proposal = Proposals::<T>::get(proposal_id)
                .ok_or(Error::<T>::ProposalNotFound)?;

            // Validate proposal is active
            ensure!(proposal.active, Error::<T>::ProposalNotActive);

            // Check voting period
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block < proposal.voting_deadline, Error::<T>::VotingPeriodEnded);

            // Check if already voted
            ensure!(!Votes::<T>::contains_key(proposal_id, &voter), Error::<T>::AlreadyVoted);

            // Update vote counts
            if vote {
                proposal.votes_for = proposal.votes_for.saturating_add(1);
            } else {
                proposal.votes_against = proposal.votes_against.saturating_add(1);
            }

            // Record vote
            Votes::<T>::insert(proposal_id, &voter, vote);
            Proposals::<T>::insert(proposal_id, proposal);

            // Emit event
            Self::deposit_event(Event::VoteCast {
                proposal_id,
                voter,
                vote,
            });

            Ok(())
        }

        /// Finalize a proposal after voting period.
        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn finalize_proposal(
            origin: OriginFor<T>,
            proposal_id: u32,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // Get proposal
            let mut proposal = Proposals::<T>::get(proposal_id)
                .ok_or(Error::<T>::ProposalNotFound)?;

            // Ensure voting period ended
            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(current_block >= proposal.voting_deadline, Error::<T>::VotingPeriodEnded);

            // Ensure still active
            ensure!(proposal.active, Error::<T>::ProposalNotActive);

            // Determine outcome
            let passed = proposal.votes_for > proposal.votes_against;

            // Mark inactive
            proposal.active = false;
            Proposals::<T>::insert(proposal_id, proposal);

            // Emit event
            Self::deposit_event(Event::ProposalFinalized {
                proposal_id,
                passed,
            });

            Ok(())
        }
    }

    /// Helper functions (not callable externally).
    impl<T: Config> Pallet<T> {
        /// Get proposal by ID.
        pub fn get_proposal(id: u32) -> Option<Proposal<T::AccountId, BlockNumberFor<T>>> {
            Proposals::<T>::get(id)
        }

        /// Check if user voted on proposal.
        pub fn has_voted(proposal_id: u32, account: &T::AccountId) -> bool {
            Votes::<T>::contains_key(proposal_id, account)
        }
    }
}
```

### Writing Tests

Create `/pallets/pallet-voting/src/mock.rs`:

```rust
use crate as pallet_voting;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure mock runtime
frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        VotingPallet: pallet_voting,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type Nonce = u64;
    type Block = Block;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
}

parameter_types! {
    pub const MinimumDeposit: u64 = 100;
    pub const VotingPeriod: u64 = 100; // 100 blocks
}

impl pallet_voting::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinimumDeposit = MinimumDeposit;
    type VotingPeriod = VotingPeriod;
}

// Build test externalities
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 10000),
            (2, 10000),
            (3, 10000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
```

Create `/pallets/pallet-voting/src/tests.rs`:

```rust
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_proposal_works() {
    new_test_ext().execute_with(|| {
        // Arrange
        let proposer = 1;
        let title = b"Test Proposal".to_vec();
        let description = b"This is a test proposal".to_vec();

        // Act
        assert_ok!(VotingPallet::create_proposal(
            RuntimeOrigin::signed(proposer),
            title.clone(),
            description.clone()
        ));

        // Assert
        let proposal = VotingPallet::proposals(0).unwrap();
        assert_eq!(proposal.proposer, proposer);
        assert_eq!(proposal.votes_for, 0);
        assert_eq!(proposal.votes_against, 0);
        assert_eq!(proposal.active, true);

        // Check event
        System::assert_last_event(
            Event::ProposalCreated {
                proposal_id: 0,
                proposer,
            }
            .into(),
        );
    });
}

#[test]
fn vote_works() {
    new_test_ext().execute_with(|| {
        // Create proposal
        assert_ok!(VotingPallet::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test".to_vec(),
            b"Test".to_vec()
        ));

        // Vote yes
        assert_ok!(VotingPallet::vote(RuntimeOrigin::signed(2), 0, true));

        // Check vote recorded
        let proposal = VotingPallet::proposals(0).unwrap();
        assert_eq!(proposal.votes_for, 1);
        assert_eq!(proposal.votes_against, 0);

        // Vote no
        assert_ok!(VotingPallet::vote(RuntimeOrigin::signed(3), 0, false));

        let proposal = VotingPallet::proposals(0).unwrap();
        assert_eq!(proposal.votes_for, 1);
        assert_eq!(proposal.votes_against, 1);
    });
}

#[test]
fn cannot_vote_twice() {
    new_test_ext().execute_with(|| {
        // Create proposal
        assert_ok!(VotingPallet::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test".to_vec(),
            b"Test".to_vec()
        ));

        // Vote once
        assert_ok!(VotingPallet::vote(RuntimeOrigin::signed(2), 0, true));

        // Try to vote again
        assert_noop!(
            VotingPallet::vote(RuntimeOrigin::signed(2), 0, false),
            Error::<Test>::AlreadyVoted
        );
    });
}

#[test]
fn finalize_proposal_works() {
    new_test_ext().execute_with(|| {
        // Create proposal
        assert_ok!(VotingPallet::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test".to_vec(),
            b"Test".to_vec()
        ));

        // Cast votes
        assert_ok!(VotingPallet::vote(RuntimeOrigin::signed(2), 0, true));
        assert_ok!(VotingPallet::vote(RuntimeOrigin::signed(3), 0, true));

        // Advance blocks past voting period
        System::set_block_number(101);

        // Finalize
        assert_ok!(VotingPallet::finalize_proposal(RuntimeOrigin::signed(1), 0));

        // Check proposal inactive
        let proposal = VotingPallet::proposals(0).unwrap();
        assert_eq!(proposal.active, false);

        // Check event
        System::assert_last_event(
            Event::ProposalFinalized {
                proposal_id: 0,
                passed: true,
            }
            .into(),
        );
    });
}

#[test]
fn cannot_vote_after_deadline() {
    new_test_ext().execute_with(|| {
        // Create proposal
        assert_ok!(VotingPallet::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test".to_vec(),
            b"Test".to_vec()
        ));

        // Advance past deadline
        System::set_block_number(101);

        // Try to vote
        assert_noop!(
            VotingPallet::vote(RuntimeOrigin::signed(2), 0, true),
            Error::<Test>::VotingPeriodEnded
        );
    });
}
```

### Integration with Runtime

Add to `/05-multichain/flare-chain/runtime/Cargo.toml`:

```toml
[dependencies]
pallet-voting = { path = "../../../pallets/pallet-voting", default-features = false }

[features]
std = [
    "pallet-voting/std",
    # ... other pallets
]
```

Add to `/05-multichain/flare-chain/runtime/src/lib.rs`:

```rust
// Configure the voting pallet
impl pallet_voting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinimumDeposit = ConstU128<1_000_000_000_000_000_000>; // 1 ETR
    type VotingPeriod = ConstU32<14400>; // ~1 day (5s blocks)
}

// Add to construct_runtime! macro
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        Balances: pallet_balances,
        VotingPallet: pallet_voting,
        // ... other pallets
    }
);
```

### Pallet Cargo.toml

Create `/pallets/pallet-voting/Cargo.toml`:

```toml
[package]
name = "pallet-voting"
version = "0.1.0"
edition = "2021"
authors = ["Etrid Foundation"]
license = "Apache-2.0"

[dependencies]
codec = { package = "parity-scale-codec", version = "3.6.1", default-features = false, features = ["derive"] }
scale-info = { version = "2.9.0", default-features = false, features = ["derive"] }

frame-support = { git = "https://github.com/paritytech/polkadot-sdk", branch = "stable2506", default-features = false }
frame-system = { git = "https://github.com/paritytech/polkadot-sdk", branch = "stable2506", default-features = false }
sp-std = { git = "https://github.com/paritytech/polkadot-sdk", branch = "stable2506", default-features = false }
sp-runtime = { git = "https://github.com/paritytech/polkadot-sdk", branch = "stable2506", default-features = false }

[dev-dependencies]
sp-core = { git = "https://github.com/paritytech/polkadot-sdk", branch = "stable2506" }
sp-io = { git = "https://github.com/paritytech/polkadot-sdk", branch = "stable2506" }
pallet-balances = { git = "https://github.com/paritytech/polkadot-sdk", branch = "stable2506" }

[features]
default = ["std"]
std = [
    "codec/std",
    "scale-info/std",
    "frame-support/std",
    "frame-system/std",
    "sp-std/std",
    "sp-runtime/std",
]
```

---

## Developing DApps

Build decentralized applications that interact with the Ëtrid blockchain using the JavaScript SDK.

### Project Setup

#### Initialize Project

```bash
# Create new project
mkdir etrid-dapp
cd etrid-dapp

# Initialize npm
npm init -y

# Install dependencies
npm install @polkadot/api @polkadot/keyring @polkadot/util-crypto
npm install react react-dom next
npm install -D typescript @types/react @types/node

# Install Etrid SDK (when published)
# npm install @etrid/sdk
```

### Connecting to Etrid Network

#### Basic Connection

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

// Connect to local node
async function connectLocal() {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });

  console.log('Connected to chain:', await api.rpc.system.chain());
  console.log('Node version:', await api.rpc.system.version());

  return api;
}

// Connect to testnet
async function connectTestnet() {
  const provider = new WsProvider('wss://testnet-rpc.etrid.io');
  const api = await ApiPromise.create({ provider });
  return api;
}

// Connect to mainnet (when available)
async function connectMainnet() {
  const provider = new WsProvider('wss://rpc.etrid.io');
  const api = await ApiPromise.create({ provider });
  return api;
}
```

#### Connection with Error Handling

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

async function connectToEtrid(endpoint: string): Promise<ApiPromise> {
  const provider = new WsProvider(endpoint);

  // Handle connection errors
  provider.on('error', (error) => {
    console.error('Provider error:', error);
  });

  provider.on('disconnected', () => {
    console.log('Disconnected from node');
  });

  provider.on('connected', () => {
    console.log('Connected to node');
  });

  try {
    const api = await ApiPromise.create({ provider });

    // Wait for chain to be ready
    await api.isReady;

    console.log('API ready. Chain:', await api.rpc.system.chain());

    return api;
  } catch (error) {
    console.error('Failed to connect:', error);
    throw error;
  }
}

// Usage
const api = await connectToEtrid('ws://127.0.0.1:9944');
```

### Reading Blockchain State

#### Query Account Balance

```typescript
import { ApiPromise } from '@polkadot/api';

async function getBalance(api: ApiPromise, address: string) {
  const { data: { free, reserved, frozen } } = await api.query.system.account(address);

  return {
    free: free.toString(),
    reserved: reserved.toString(),
    frozen: frozen.toString(),
    total: free.add(reserved).toString(),
    available: free.sub(frozen).toString(),
  };
}

// Usage
const balance = await getBalance(api, '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
console.log('Free balance:', balance.free);
```

#### Query Voting Proposals

```typescript
async function getProposals(api: ApiPromise) {
  const nextId = await api.query.votingPallet.nextProposalId();
  const proposals = [];

  for (let i = 0; i < nextId.toNumber(); i++) {
    const proposal = await api.query.votingPallet.proposals(i);
    if (proposal.isSome) {
      proposals.push({
        id: i,
        data: proposal.unwrap().toJSON(),
      });
    }
  }

  return proposals;
}

// Usage
const proposals = await getProposals(api);
proposals.forEach(p => {
  console.log(`#${p.id}: ${p.data.title}`);
  console.log(`  Votes: ${p.data.votesFor} / ${p.data.votesAgainst}`);
});
```

#### Subscribe to Balance Changes

```typescript
async function subscribeToBalance(
  api: ApiPromise,
  address: string,
  callback: (balance: any) => void
) {
  const unsubscribe = await api.query.system.account(address, ({ data }) => {
    callback({
      free: data.free.toString(),
      reserved: data.reserved.toString(),
      total: data.free.add(data.reserved).toString(),
    });
  });

  return unsubscribe;
}

// Usage
const unsub = await subscribeToBalance(api, address, (balance) => {
  console.log('Balance updated:', balance.total);
});

// Later: unsub();
```

### Submitting Transactions

#### Create and Sign Transaction

```typescript
import { Keyring } from '@polkadot/keyring';

// Create account from mnemonic
const keyring = new Keyring({ type: 'sr25519' });
const alice = keyring.addFromMnemonic('your twelve word mnemonic phrase here');

// Transfer tokens
async function transfer(api: ApiPromise, from: any, to: string, amount: string) {
  const tx = api.tx.balances.transferKeepAlive(to, amount);

  // Sign and send
  const hash = await tx.signAndSend(from);

  console.log('Transaction hash:', hash.toHex());
  return hash;
}

// Usage
const recipient = '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty';
const amount = '1000000000000000000'; // 1 ETR (18 decimals)
await transfer(api, alice, recipient, amount);
```

#### Transaction with Status Tracking

```typescript
async function transferWithTracking(
  api: ApiPromise,
  from: any,
  to: string,
  amount: string
) {
  return new Promise(async (resolve, reject) => {
    const unsub = await api.tx.balances
      .transferKeepAlive(to, amount)
      .signAndSend(from, ({ status, events, dispatchError }) => {
        console.log('Status:', status.type);

        if (status.isInBlock) {
          console.log('Included in block:', status.asInBlock.toHex());
        }

        if (status.isFinalized) {
          console.log('Finalized in block:', status.asFinalized.toHex());

          // Check for errors
          if (dispatchError) {
            if (dispatchError.isModule) {
              const decoded = api.registry.findMetaError(dispatchError.asModule);
              const { docs, name, section } = decoded;
              reject(new Error(`${section}.${name}: ${docs.join(' ')}`));
            } else {
              reject(new Error(dispatchError.toString()));
            }
          } else {
            // Success
            events.forEach(({ event }) => {
              if (api.events.balances.Transfer.is(event)) {
                const [from, to, value] = event.data;
                console.log(`Transfer: ${value} from ${from} to ${to}`);
              }
            });

            resolve(status.asFinalized);
          }

          unsub();
        }
      });
  });
}
```

#### Call Custom Pallet

```typescript
async function createProposal(
  api: ApiPromise,
  signer: any,
  title: string,
  description: string
) {
  const tx = api.tx.votingPallet.createProposal(title, description);

  return new Promise(async (resolve, reject) => {
    const unsub = await tx.signAndSend(signer, ({ status, events, dispatchError }) => {
      if (status.isFinalized) {
        if (dispatchError) {
          reject(dispatchError);
        } else {
          // Find ProposalCreated event
          const proposalEvent = events.find(({ event }) =>
            api.events.votingPallet.ProposalCreated.is(event)
          );

          if (proposalEvent) {
            const [proposalId] = proposalEvent.event.data;
            resolve(proposalId.toNumber());
          } else {
            reject(new Error('ProposalCreated event not found'));
          }
        }
        unsub();
      }
    });
  });
}

// Usage
const proposalId = await createProposal(
  api,
  alice,
  'Increase block size',
  'Proposal to increase maximum block size to 10MB'
);
console.log('Created proposal:', proposalId);
```

### Handling Events

#### Listen for Specific Events

```typescript
async function listenForTransfers(api: ApiPromise) {
  api.query.system.events((events) => {
    events.forEach((record) => {
      const { event } = record;

      if (api.events.balances.Transfer.is(event)) {
        const [from, to, value] = event.data;
        console.log(`Transfer: ${value} from ${from} to ${to}`);
      }

      if (api.events.votingPallet.VoteCast.is(event)) {
        const [proposalId, voter, vote] = event.data;
        console.log(`Vote cast on #${proposalId} by ${voter}: ${vote}`);
      }
    });
  });
}
```

### Example: Complete Token Transfer DApp

Create `pages/index.tsx`:

```typescript
import { useState, useEffect } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';

export default function TransferDApp() {
  const [api, setApi] = useState<ApiPromise | null>(null);
  const [account, setAccount] = useState<any>(null);
  const [balance, setBalance] = useState('0');
  const [recipient, setRecipient] = useState('');
  const [amount, setAmount] = useState('');
  const [status, setStatus] = useState('');

  // Connect to blockchain
  useEffect(() => {
    async function connect() {
      const provider = new WsProvider('ws://127.0.0.1:9944');
      const api = await ApiPromise.create({ provider });
      setApi(api);
      setStatus('Connected to Etrid node');
    }
    connect();
  }, []);

  // Load account
  const loadAccount = (mnemonic: string) => {
    const keyring = new Keyring({ type: 'sr25519' });
    const account = keyring.addFromMnemonic(mnemonic);
    setAccount(account);
    updateBalance(account.address);
  };

  // Update balance
  const updateBalance = async (address: string) => {
    if (!api) return;
    const { data: { free } } = await api.query.system.account(address);
    setBalance((free.toNumber() / 1e18).toFixed(4));
  };

  // Transfer tokens
  const transfer = async () => {
    if (!api || !account) return;

    setStatus('Sending transaction...');

    try {
      const amountInPlanck = (parseFloat(amount) * 1e18).toString();

      await api.tx.balances
        .transferKeepAlive(recipient, amountInPlanck)
        .signAndSend(account, ({ status }) => {
          if (status.isFinalized) {
            setStatus('Transfer complete!');
            updateBalance(account.address);
            setRecipient('');
            setAmount('');
          }
        });
    } catch (error) {
      setStatus(`Error: ${error.message}`);
    }
  };

  return (
    <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
      <h1>Etrid Token Transfer</h1>

      <div>
        <h2>Account</h2>
        <input
          type="password"
          placeholder="Enter mnemonic"
          onBlur={(e) => loadAccount(e.target.value)}
          style={{ width: '100%', padding: '8px' }}
        />
        {account && (
          <div>
            <p>Address: {account.address}</p>
            <p>Balance: {balance} ETR</p>
          </div>
        )}
      </div>

      <div>
        <h2>Transfer</h2>
        <input
          type="text"
          placeholder="Recipient address"
          value={recipient}
          onChange={(e) => setRecipient(e.target.value)}
          style={{ width: '100%', padding: '8px', marginBottom: '8px' }}
        />
        <input
          type="number"
          placeholder="Amount (ETR)"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
          style={{ width: '100%', padding: '8px', marginBottom: '8px' }}
        />
        <button
          onClick={transfer}
          disabled={!account || !recipient || !amount}
          style={{ padding: '10px 20px', cursor: 'pointer' }}
        >
          Send
        </button>
      </div>

      <div>
        <p>Status: {status}</p>
      </div>
    </div>
  );
}
```

---

## Smart Contract Development

Ëtrid supports smart contracts through the ËtwasmVM (WebAssembly runtime). You can write contracts in:
- **ink!**: Rust-based smart contract language (recommended)
- **Solidity**: For Ethereum compatibility via EVM adapter

### ink! Smart Contracts

#### Setup ink! Development

```bash
# Install cargo-contract
cargo install cargo-contract --force

# Verify installation
cargo contract --version
```

#### Create New Contract

```bash
# Create new ink! project
cargo contract new flipper
cd flipper

# Project structure:
# flipper/
# ├── Cargo.toml
# └── lib.rs
```

#### Example: Simple Storage Contract

Edit `lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod simple_storage {
    use ink::storage::Mapping;

    /// Storage contract that maps addresses to values
    #[ink(storage)]
    pub struct SimpleStorage {
        /// Mapping from AccountId to stored value
        values: Mapping<AccountId, u32>,
    }

    impl SimpleStorage {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                values: Mapping::default(),
            }
        }

        /// Store a value
        #[ink(message)]
        pub fn set(&mut self, value: u32) {
            let caller = self.env().caller();
            self.values.insert(caller, &value);
        }

        /// Get stored value
        #[ink(message)]
        pub fn get(&self) -> Option<u32> {
            let caller = self.env().caller();
            self.values.get(caller)
        }

        /// Get value for specific account
        #[ink(message)]
        pub fn get_for(&self, account: AccountId) -> Option<u32> {
            self.values.get(account)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn set_and_get_works() {
            let mut contract = SimpleStorage::new();

            // Set value
            contract.set(42);

            // Get value
            assert_eq!(contract.get(), Some(42));
        }

        #[ink::test]
        fn different_accounts_have_different_values() {
            let mut contract = SimpleStorage::new();

            // Set value as default account
            contract.set(10);

            // Change account context
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            // Bob's value should be None
            assert_eq!(contract.get(), None);

            // Set Bob's value
            contract.set(20);
            assert_eq!(contract.get(), Some(20));
        }
    }
}
```

#### Build Contract

```bash
# Build contract (generates .contract, .wasm, and metadata.json)
cargo contract build --release

# Output files in target/ink/:
# - simple_storage.contract (bundle)
# - simple_storage.wasm (WebAssembly code)
# - metadata.json (ABI)
```

#### Test Contract

```bash
# Run unit tests
cargo test

# Run with output
cargo test -- --nocapture
```

#### Deploy Contract

Using Polkadot.js Apps:

1. **Upload Code**:
   - Navigate to Developer → Contracts
   - Click "Upload & deploy code"
   - Select `simple_storage.contract`
   - Click "Upload"

2. **Deploy Instance**:
   - Choose constructor: `new()`
   - Set initial endowment (e.g., 1000 ETR)
   - Click "Deploy"
   - Copy contract address

3. **Interact**:
   - Call `set(value: u32)` to store value
   - Call `get()` to retrieve value

#### Deploy via CLI

```bash
# Deploy contract
cargo contract instantiate \
  --constructor new \
  --args \
  --suri //Alice \
  --execute

# Call contract method
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message set \
  --args 42 \
  --suri //Alice \
  --execute
```

### Advanced ink! Contract: Token

Create `erc20/lib.rs`:

```rust
#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    /// ERC-20 token contract
    #[ink(storage)]
    pub struct Erc20 {
        /// Total token supply
        total_supply: Balance,
        /// Mapping from account to balance
        balances: Mapping<AccountId, Balance>,
        /// Mapping from (owner, spender) to allowance
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    /// Events
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        /// Constructor - creates new token with initial supply
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            balances.insert(caller, &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowances: Mapping::default(),
            }
        }

        /// Returns total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns balance of account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        /// Returns allowance
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        /// Transfer tokens
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(&from, &to, value)
        }

        /// Approve spender
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);

            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });

            Ok(())
        }

        /// Transfer from allowance
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance(from, caller);

            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }

            self.transfer_from_to(&from, &to, value)?;

            // Update allowance
            self.allowances.insert((from, caller), &(allowance - value));

            Ok(())
        }

        /// Internal transfer
        fn transfer_from_to(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of(*from);

            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }

            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(*to);
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                value,
            });

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(1000);
            assert_eq!(contract.total_supply(), 1000);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(contract.balance_of(accounts.alice), 1000);
            assert_eq!(contract.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(contract.balance_of(accounts.alice), 1000);
            assert!(contract.transfer(accounts.bob, 100).is_ok());
            assert_eq!(contract.balance_of(accounts.alice), 900);
            assert_eq!(contract.balance_of(accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_insufficient_balance_fails() {
            let mut contract = Erc20::new(100);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(
                contract.transfer(accounts.bob, 200),
                Err(Error::InsufficientBalance)
            );
        }

        #[ink::test]
        fn approve_works() {
            let mut contract = Erc20::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert!(contract.approve(accounts.bob, 100).is_ok());
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 100);
        }

        #[ink::test]
        fn transfer_from_works() {
            let mut contract = Erc20::new(1000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            // Alice approves Bob
            assert!(contract.approve(accounts.bob, 100).is_ok());

            // Bob transfers from Alice to Charlie
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert!(contract.transfer_from(accounts.alice, accounts.charlie, 50).is_ok());

            assert_eq!(contract.balance_of(accounts.alice), 950);
            assert_eq!(contract.balance_of(accounts.charlie), 50);
            assert_eq!(contract.allowance(accounts.alice, accounts.bob), 50);
        }
    }
}
```

### Contract Interaction from DApp

```typescript
import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';

// Load contract
async function loadContract(api: ApiPromise, address: string, abi: any) {
  return new ContractPromise(api, abi, address);
}

// Call contract method (read-only)
async function getBalance(contract: ContractPromise, account: string) {
  const { output } = await contract.query.balanceOf(
    account,
    { gasLimit: -1 },
    account
  );

  return output?.toJSON();
}

// Execute contract method (write)
async function transfer(
  contract: ContractPromise,
  signer: any,
  to: string,
  amount: number
) {
  const gasLimit = api.registry.createType('WeightV2', {
    refTime: 10_000_000_000,
    proofSize: 100_000,
  });

  await contract.tx
    .transfer({ gasLimit }, to, amount)
    .signAndSend(signer, (result) => {
      if (result.status.isInBlock) {
        console.log('In block');
      }
      if (result.status.isFinalized) {
        console.log('Finalized');
      }
    });
}
```

---

## SDK Usage Guide

Ëtrid provides SDKs in 4 languages: Rust, JavaScript/TypeScript, Python, and Swift.

### JavaScript/TypeScript SDK

**Location**: `/13-clients/sdk/js-etrid-sdk/`

#### Installation

```bash
npm install @etrid/sdk @polkadot/api
```

#### Quick Start

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';
import { Keyring } from '@polkadot/keyring';
import {
  TransactionBuilder,
  AccountsWrapper,
  StakingWrapper,
  GovernanceWrapper,
  formatETR,
} from '@etrid/sdk';

// Connect
const provider = new WsProvider('wss://rpc.etrid.io');
const api = await ApiPromise.create({ provider });

// Create account
const keyring = new Keyring({ type: 'sr25519' });
const alice = keyring.addFromMnemonic('your mnemonic here');

// Check balance
const accounts = new AccountsWrapper(api);
const balance = await accounts.getBalance(alice.address);
console.log(`Balance: ${formatETR(balance.total)}`);

// Send transaction
const result = await new TransactionBuilder(api)
  .transferKeepAlive(recipientAddress, 1000000000000000000n) // 1 ETR
  .submit(alice);

console.log(`Success: ${result.success}`);
```

#### Key Management

```typescript
import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate } from '@polkadot/util-crypto';

// Generate new mnemonic
const mnemonic = mnemonicGenerate();
console.log('Mnemonic:', mnemonic);

// Create keyring
const keyring = new Keyring({ type: 'sr25519' });

// Add from mnemonic
const account = keyring.addFromMnemonic(mnemonic);
console.log('Address:', account.address);

// Add from seed
const alice = keyring.addFromUri('//Alice');

// Add from JSON
const json = { ... }; // Exported JSON keystore
const imported = keyring.addFromJson(json);
imported.unlock('password');
```

#### Transaction Builders

```typescript
import { TransactionBuilder } from '@etrid/sdk';

// Simple transfer
await new TransactionBuilder(api)
  .transferKeepAlive(recipient, amount)
  .submit(alice);

// Transfer with options
await new TransactionBuilder(api)
  .transfer(recipient, amount)
  .withTip(1000000n)       // Priority tip
  .withMortality(128)      // Expire after 128 blocks
  .submit(alice);

// Batch transactions
const calls = [
  api.tx.balances.transfer(recipient1, amount1),
  api.tx.balances.transfer(recipient2, amount2),
];

await new TransactionBuilder(api)
  .batchAll(calls)
  .submit(alice);

// Estimate fees
const builder = new TransactionBuilder(api)
  .transfer(recipient, amount);

const fees = await builder.estimateFees(alice.address);
console.log(`Fees: ${formatETR(fees)}`);

// Dry run (test without executing)
const { success, error } = await builder.dryRun(alice.address);
if (!success) {
  console.error(`Would fail: ${error}`);
}
```

#### Query Helpers

```typescript
import { AccountsWrapper, StakingWrapper, GovernanceWrapper } from '@etrid/sdk';

// Account queries
const accounts = new AccountsWrapper(api);

const balance = await accounts.getBalance(address);
// { total, available, reserved, locked }

const nonce = await accounts.getNonce(address);

// Subscribe to balance changes
const unsub = await accounts.subscribeBalance(address, (balance) => {
  console.log(`New balance: ${formatETR(balance.total)}`);
});

// Staking queries
const staking = new StakingWrapper(api);

const validators = await staking.getValidators();
const details = await staking.getValidatorDetails(validatorAddress);
// { totalStake, commission, active, ... }

const rewards = await staking.estimateRewards(10000000000000000000n); // 10 ETR
// { daily, monthly, yearly, apy }

// Governance queries
const governance = new GovernanceWrapper(api);

const proposals = await governance.getActiveProposals();
// [{ id, title, proposer, votesFor, votesAgainst, ... }]

const results = await governance.getProposalResults(proposalId);
// { approved, participationRate, ... }
```

#### Error Handling

```typescript
import {
  EtridError,
  InsufficientBalanceError,
  InvalidAddressError,
  TransactionError,
  ErrorHelpers,
} from '@etrid/sdk';

try {
  await accounts.transfer(alice, recipient, amount);
} catch (error) {
  if (error instanceof InsufficientBalanceError) {
    console.error(`Short by: ${formatETR(error.getShortage())}`);
  } else if (error instanceof InvalidAddressError) {
    console.error(`Invalid: ${error.address}`);
  } else if (error instanceof TransactionError) {
    console.error(`Failed: ${error.getUserMessage()}`);
  }

  // Error helpers
  console.log(`Category: ${ErrorHelpers.getCategory(error)}`);
  if (ErrorHelpers.isRetryable(error)) {
    console.log('Can retry');
  }
}
```

#### Formatters

```typescript
import {
  formatETR,
  formatETD,
  formatPercentage,
  formatAPY,
  shortenAddress,
  formatTimestamp,
  formatDuration,
  formatCompact,
} from '@etrid/sdk';

// Balance formatting
formatETR(1500000000000000000n);  // "1.5 ETR"
formatETD(1500000000000000000n);  // "1.5 ETD"

// Address formatting
shortenAddress("5Grwv...utQY");   // "5Grwv...utQY"

// Number formatting
formatPercentage(15.5);            // "15.50%"
formatAPY(12.5);                   // "12.50% APY"
formatCompact(1500000);            // "1.5M"

// Time formatting
formatDuration(3600);              // "1 hour"
formatTimestamp(1698765432);       // "Oct 31, 2023"
```

### Rust SDK

**Location**: `/13-clients/sdk/rust-etrid-sdk/`

See `/13-clients/sdk/rust-etrid-sdk/README.md` for full documentation.

```rust
use etrid_sdk::{Client, Account};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to node
    let client = Client::connect("ws://127.0.0.1:9944").await?;

    // Create account
    let account = Account::from_mnemonic("word1 word2 ...")?;

    // Get balance
    let balance = client.get_balance(&account.address()).await?;
    println!("Balance: {}", balance.free);

    // Transfer
    let tx = client.transfer(&account, recipient, 1_000_000_000_000_000_000).await?;
    println!("Transaction hash: {}", tx.hash);

    Ok(())
}
```

### Python SDK

**Location**: `/13-clients/sdk/python-etrid-sdk/`

```python
import asyncio
from etrid_sdk import Client, Account

async def main():
    # Connect
    client = await Client.connect("ws://127.0.0.1:9944")

    # Create account
    account = Account.from_mnemonic("word1 word2 ...")

    # Get balance
    balance = await client.get_balance(account.address)
    print(f"Balance: {balance.free}")

    # Transfer
    tx = await client.transfer(account, recipient, 1_000_000_000_000_000_000)
    print(f"Transaction: {tx.hash}")

asyncio.run(main())
```

### Swift SDK

**Location**: `/13-clients/sdk/swift-etrid-sdk/`

```swift
import EtridSDK

let client = try await EtridClient.connect(url: "ws://127.0.0.1:9944")

let account = try Account.fromMnemonic("word1 word2 ...")

let balance = try await client.getBalance(address: account.address)
print("Balance: \(balance.free)")

let tx = try await client.transfer(
    from: account,
    to: recipient,
    amount: 1_000_000_000_000_000_000
)
print("Transaction: \(tx.hash)")
```

---

## Testing and Debugging

### Unit Testing Pallets

Use Substrate's testing framework to write comprehensive unit tests.

**Example** (from earlier voting pallet):

```rust
// tests.rs
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_proposal_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(VotingPallet::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test".to_vec(),
            b"Description".to_vec()
        ));

        let proposal = VotingPallet::proposals(0).unwrap();
        assert_eq!(proposal.proposer, 1);
        assert_eq!(proposal.active, true);
    });
}

#[test]
fn cannot_vote_twice() {
    new_test_ext().execute_with(|| {
        assert_ok!(VotingPallet::create_proposal(
            RuntimeOrigin::signed(1),
            b"Test".to_vec(),
            b"Description".to_vec()
        ));

        assert_ok!(VotingPallet::vote(RuntimeOrigin::signed(2), 0, true));

        assert_noop!(
            VotingPallet::vote(RuntimeOrigin::signed(2), 0, false),
            Error::<Test>::AlreadyVoted
        );
    });
}
```

**Run tests**:
```bash
cargo test -p pallet-voting
cargo test -p pallet-voting -- --nocapture  # With output
```

### Integration Testing

Test interaction between multiple pallets.

Create `/tests/integration_tests.rs`:

```rust
use node_runtime::{Runtime, System, Balances, VotingPallet};
use sp_runtime::BuildStorage;

#[test]
fn end_to_end_voting_flow() {
    let mut ext = sp_io::TestExternalities::new(
        frame_system::GenesisConfig::<Runtime>::default()
            .build_storage()
            .unwrap()
    );

    ext.execute_with(|| {
        // Fund accounts
        Balances::make_free_balance_be(&1, 10000);
        Balances::make_free_balance_be(&2, 10000);
        Balances::make_free_balance_be(&3, 10000);

        // Create proposal
        assert_ok!(VotingPallet::create_proposal(
            Origin::signed(1),
            b"Test Proposal".to_vec(),
            b"A test".to_vec()
        ));

        // Vote
        assert_ok!(VotingPallet::vote(Origin::signed(2), 0, true));
        assert_ok!(VotingPallet::vote(Origin::signed(3), 0, true));

        // Advance blocks
        System::set_block_number(101);

        // Finalize
        assert_ok!(VotingPallet::finalize_proposal(Origin::signed(1), 0));

        // Verify
        let proposal = VotingPallet::proposals(0).unwrap();
        assert_eq!(proposal.active, false);
        assert!(proposal.votes_for > proposal.votes_against);
    });
}
```

### Frontend Testing

**Jest** for unit tests:

```typescript
// __tests__/utils.test.ts
import { formatETR, shortenAddress } from '../src/utils';

describe('formatETR', () => {
  it('formats balance correctly', () => {
    expect(formatETR(1500000000000000000n)).toBe('1.5 ETR');
    expect(formatETR(0n)).toBe('0 ETR');
  });
});

describe('shortenAddress', () => {
  it('shortens long addresses', () => {
    const addr = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
    expect(shortenAddress(addr)).toBe('5Grwv...utQY');
  });
});
```

**React Testing Library** for components:

```typescript
// __tests__/TransferForm.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import TransferForm from '../components/TransferForm';

describe('TransferForm', () => {
  it('validates recipient address', () => {
    render(<TransferForm />);

    const input = screen.getByPlaceholderText('Recipient address');
    fireEvent.change(input, { target: { value: 'invalid' } });

    expect(screen.getByText('Invalid address')).toBeInTheDocument();
  });

  it('disables submit when amount is empty', () => {
    render(<TransferForm />);

    const button = screen.getByText('Send');
    expect(button).toBeDisabled();
  });
});
```

### Debugging Techniques

#### Substrate Logging

Add tracing to pallet:

```rust
use frame_support::log;

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn my_function(origin: OriginFor<T>) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // Debug logging
        log::debug!("my_function called by {:?}", who);
        log::info!("Processing transaction");
        log::warn!("Potential issue detected");
        log::error!("Critical error occurred");

        Ok(())
    }
}
```

Run node with logging:

```bash
RUST_LOG=debug ./target/release/flare-chain-node --dev

# Filter specific pallet
RUST_LOG=pallet_voting=debug ./target/release/flare-chain-node --dev

# Multiple targets
RUST_LOG=pallet_voting=debug,runtime=info ./target/release/flare-chain-node --dev
```

#### Browser DevTools

For DApp debugging:

```typescript
// Add console logging
console.log('API connected:', api.isConnected);
console.log('Account address:', account.address);
console.log('Balance:', balance.toString());

// Network inspection
// Open DevTools → Network tab
// Filter: WS (WebSocket)
// Monitor RPC calls

// State inspection
window.api = api;  // Expose API globally
window.account = account;

// Then in console:
await window.api.query.system.account(window.account.address)
```

#### Polkadot.js Apps

Use as debugging tool:

1. **Explorer**: View blocks, events, extrinsics
2. **Chain State**: Query storage directly
3. **Extrinsics**: Submit transactions manually
4. **RPC Calls**: Test RPC methods
5. **Developer → JavaScript**: Execute custom queries

#### Common Issues

**Issue**: Transaction fails with `DispatchError::Module`

**Solution**:
```typescript
if (dispatchError.isModule) {
  const decoded = api.registry.findMetaError(dispatchError.asModule);
  console.error(`${decoded.section}.${decoded.name}: ${decoded.docs}`);
}
```

**Issue**: WebSocket connection fails

**Solution**:
```bash
# Check node is running
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  http://localhost:9933

# Check CORS settings
./target/release/flare-chain-node --dev --rpc-cors all --ws-external
```

**Issue**: Pallet not found in runtime

**Solution**:
- Verify pallet added to `construct_runtime!` macro
- Check `[dependencies]` in runtime `Cargo.toml`
- Rebuild runtime WASM

---

## Best Practices

### Security Considerations

#### Input Validation

Always validate inputs:

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    pub fn my_function(
        origin: OriginFor<T>,
        amount: Balance,
        recipient: T::AccountId,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;

        // Validate amount
        ensure!(amount > 0, Error::<T>::InvalidAmount);
        ensure!(amount <= MAX_AMOUNT, Error::<T>::AmountTooLarge);

        // Validate recipient
        ensure!(recipient != sender, Error::<T>::CannotSendToSelf);

        // Validate sender balance
        let balance = Self::get_balance(&sender);
        ensure!(balance >= amount, Error::<T>::InsufficientBalance);

        Ok(())
    }
}
```

#### Overflow Protection

Use saturating arithmetic:

```rust
// Bad: Can overflow
let total = a + b;

// Good: Saturating addition
let total = a.saturating_add(b);

// Good: Checked addition
let total = a.checked_add(b).ok_or(Error::<T>::Overflow)?;
```

#### Access Control

Implement proper permissions:

```rust
// Root/sudo only
ensure_root(origin)?;

// Specific account only
let sender = ensure_signed(origin)?;
ensure!(sender == Self::admin(), Error::<T>::NotAuthorized);

// Custom permission check
ensure!(Self::has_permission(&sender), Error::<T>::NotAuthorized);
```

#### Reentrancy Protection

ËtwasmVM has built-in reentrancy protection (see `/08-etwasm-vm/REENTRANCY_PROTECTION.md`), but follow Checks-Effects-Interactions pattern:

```rust
pub fn withdraw(&mut self, amount: Balance) -> DispatchResult {
    let sender = ensure_signed(origin)?;

    // 1. CHECKS
    let balance = Self::balance(&sender);
    ensure!(balance >= amount, Error::<T>::InsufficientBalance);

    // 2. EFFECTS (update state BEFORE external calls)
    Self::set_balance(&sender, balance - amount);

    // 3. INTERACTIONS (external calls LAST)
    T::Currency::transfer(&Self::account_id(), &sender, amount)?;

    Ok(())
}
```

### Performance Optimization

#### Efficient Storage

Minimize storage reads/writes:

```rust
// Bad: Multiple reads
let value1 = MyStorage::<T>::get(&key);
let value2 = MyStorage::<T>::get(&key);
let value3 = MyStorage::<T>::get(&key);

// Good: Single read, reuse
let value = MyStorage::<T>::get(&key);
// Use value multiple times

// Bad: Unnecessary writes
MyStorage::<T>::insert(&key, &value);
MyStorage::<T>::insert(&key, &value);  // Duplicate

// Good: Single write
MyStorage::<T>::insert(&key, &value);
```

Use `mutate` for read-modify-write:

```rust
// Bad: Separate read and write
let mut value = MyStorage::<T>::get(&key);
value += 1;
MyStorage::<T>::insert(&key, &value);

// Good: Atomic mutate
MyStorage::<T>::mutate(&key, |value| {
    *value += 1;
});
```

#### Bounded Collections

Use bounded vectors to prevent unbounded growth:

```rust
use frame_support::BoundedVec;

#[derive(Encode, Decode, TypeInfo)]
pub struct MyStruct {
    // Bad: Unbounded Vec
    // items: Vec<u8>,

    // Good: Bounded with max length
    items: BoundedVec<u8, ConstU32<1024>>,
}
```

#### Weight Calculation

Provide accurate weights:

```rust
#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 1))]
pub fn my_function(origin: OriginFor<T>) -> DispatchResult {
    // 2 storage reads
    let value1 = Storage1::<T>::get();
    let value2 = Storage2::<T>::get();

    // 1 storage write
    Storage3::<T>::put(value1 + value2);

    Ok(())
}
```

### Code Organization

#### Module Structure

Organize pallets logically:

```
pallet-name/
├── Cargo.toml
└── src/
    ├── lib.rs           # Main pallet logic
    ├── types.rs         # Type definitions
    ├── functions.rs     # Helper functions
    ├── mock.rs          # Test runtime
    ├── tests.rs         # Unit tests
    └── benchmarking.rs  # Benchmarks
```

#### Naming Conventions

Follow Rust naming:

```rust
// Types: PascalCase
pub struct MyStruct { }
pub enum MyEnum { }

// Functions: snake_case
pub fn my_function() { }

// Constants: SCREAMING_SNAKE_CASE
pub const MAX_VALUE: u32 = 100;

// Storage: PascalCase
#[pallet::storage]
pub type MyStorage<T> = StorageValue<_, u32>;
```

### Documentation Standards

#### Pallet Documentation

Document all public items:

```rust
/// # My Pallet
///
/// Provides functionality for X, Y, and Z.
///
/// ## Overview
///
/// This pallet allows users to:
/// - Do thing A
/// - Do thing B
/// - Query thing C
///
/// ## Interface
///
/// ### Dispatchable Functions
///
/// - `create_item` - Creates a new item
/// - `update_item` - Updates existing item
/// - `delete_item` - Removes an item
///
/// ### Public Functions
///
/// - `get_item` - Retrieves item by ID
#[pallet::pallet]
pub struct Pallet<T>(_);

/// Configuration trait for the pallet.
#[pallet::config]
pub trait Config: frame_system::Config {
    /// The overarching event type.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    /// Maximum number of items per user.
    #[pallet::constant]
    type MaxItems: Get<u32>;
}

/// Creates a new item.
///
/// # Arguments
///
/// * `origin` - The transaction origin (must be signed)
/// * `name` - Item name (max 256 bytes)
/// * `value` - Item value
///
/// # Errors
///
/// * `TooManyItems` - User already has maximum items
/// * `NameTooLong` - Name exceeds 256 bytes
///
/// # Events
///
/// * `ItemCreated` - Emitted when item successfully created
#[pallet::call_index(0)]
#[pallet::weight(10_000)]
pub fn create_item(
    origin: OriginFor<T>,
    name: Vec<u8>,
    value: u32,
) -> DispatchResult {
    // Implementation
    Ok(())
}
```

#### API Documentation

Use JSDoc for TypeScript:

```typescript
/**
 * Transfer tokens to another account.
 *
 * @param from - Sender account (KeyringPair)
 * @param to - Recipient address (SS58 format)
 * @param amount - Amount in planck (1 ETR = 10^18 planck)
 * @returns Transaction result with hash and status
 *
 * @throws {InsufficientBalanceError} If sender balance too low
 * @throws {InvalidAddressError} If recipient address invalid
 * @throws {TransactionError} If transaction fails
 *
 * @example
 * ```typescript
 * const result = await transfer(alice, bobAddress, 1000000000000000000n);
 * console.log('Transaction hash:', result.hash);
 * ```
 */
async function transfer(
  from: KeyringPair,
  to: string,
  amount: bigint
): Promise<TransactionResult> {
  // Implementation
}
```

---

## Contributing to Etrid

We welcome contributions! Please follow these guidelines.

### Code Style Guide

**Rust**:
- Follow official Rust style guide
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy -- -D warnings`
- Maximum line length: 100 characters
- Use descriptive variable names
- Add comments for complex logic

**TypeScript**:
- Use ESLint + Prettier
- Prefer `const` over `let`
- Use arrow functions
- Explicit return types for public functions
- Interface over type for object shapes

**Commit Messages**:
```
type(scope): subject

body (optional)

footer (optional)
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(pallet-voting): add delegate voting functionality

Implements vote delegation allowing users to assign their
voting power to a trusted delegate.

Closes #123
```

### Pull Request Process

1. **Fork and Branch**:
   ```bash
   git clone https://github.com/YOUR-USERNAME/etrid.git
   git checkout -b feature/my-feature
   ```

2. **Make Changes**:
   - Write code following style guide
   - Add/update tests
   - Update documentation
   - Run tests: `cargo test --workspace`
   - Run linter: `cargo clippy`

3. **Commit**:
   ```bash
   git add .
   git commit -m "feat(scope): description"
   ```

4. **Push**:
   ```bash
   git push origin feature/my-feature
   ```

5. **Open PR**:
   - Go to https://github.com/EojEdred/Etrid
   - Click "New Pull Request"
   - Select your branch
   - Fill out PR template

### Testing Requirements

All PRs must include tests:

**For Pallets**:
- Unit tests in `tests.rs`
- Cover all extrinsics
- Test error cases
- Minimum 80% coverage

**For Frontend**:
- Jest unit tests
- React Testing Library component tests
- E2E tests for critical flows

**For Smart Contracts**:
- ink! built-in tests
- Test all public methods
- Test error conditions

### Code Review Process

1. **Automated Checks**:
   - CI/CD runs tests
   - Linting must pass
   - Build must succeed

2. **Manual Review**:
   - At least 1 maintainer approval
   - Review within 3-5 business days
   - Address all feedback

3. **Merge**:
   - Squash commits
   - Merge to main
   - Deploy to testnet (if applicable)

### Getting Help

- **Discord**: https://discord.gg/etrid
- **GitHub Discussions**: https://github.com/EojEdred/Etrid/discussions
- **Documentation**: `/docs/` directory
- **Email**: dev@etrid.io

---

## Additional Resources

### Documentation

- **Architecture**: `/docs/architecture.md`
- **Component Docs**: Each component has `ARCHITECTURE.md`
- **Quick Start**: `/QUICK_START.md`
- **Contributing**: `/CONTRIBUTING.md`
- **Known Issues**: `/KNOWN_ISSUES.md`

### Example Projects

- **Simple Voting DApp**: This guide (earlier sections)
- **Token Transfer DApp**: This guide (SDK section)
- **ERC-20 Contract**: This guide (ink! section)

### External Resources

- **Substrate Docs**: https://docs.substrate.io/
- **Polkadot Wiki**: https://wiki.polkadot.network/
- **ink! Docs**: https://use.ink/
- **Polkadot.js API**: https://polkadot.js.org/docs/api/

### Community

- **Website**: https://etrid.io (coming soon)
- **Twitter**: [@EtridMultichain](https://twitter.com/EtridMultichain)
- **Discord**: https://discord.gg/etrid
- **Telegram**: https://t.me/EtridOfficial
- **GitHub**: https://github.com/EojEdred/Etrid

---

## Conclusion

This developer guide provides a comprehensive foundation for building on Ëtrid. Key takeaways:

1. **Architecture**: Understand FlareChain + 13 PBCs + Lightning-Bloc Layer 2
2. **Pallets**: Build modular blockchain logic with FRAME
3. **DApps**: Use JavaScript SDK to create user-facing applications
4. **Smart Contracts**: Write ink! contracts for advanced logic
5. **Testing**: Comprehensive test coverage ensures reliability
6. **Best Practices**: Security, performance, and code quality matter

**Next Steps**:
1. Set up your development environment
2. Explore example code in this guide
3. Build a simple pallet or DApp
4. Join the community and ask questions
5. Contribute to Ëtrid!

Happy building! 🚀

---

**Document Version**: 1.0.0
**Last Updated**: October 22, 2025
**Maintained By**: Ëtrid Foundation
**License**: Apache 2.0
