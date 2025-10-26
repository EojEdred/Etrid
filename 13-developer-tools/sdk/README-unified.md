# ËTRID SDK

**Unified developer-friendly API for building on the ËTRID Multichain Protocol**

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../LICENSE)
[![Documentation](https://docs.rs/etrid-sdk/badge.svg)](https://docs.rs/etrid-sdk)

## Overview

The ËTRID SDK provides a curated, stable API for external developers. Instead of managing 68 individual crates, you get a single dependency with feature-gated modules.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
etrid-sdk = { version = "0.1.0", features = ["wallet"] }
```

## Features

### Individual Components

- `accounts` - Account management
- `governance` - Foundation DAO
- `consensus` - FODDoS ASF consensus
- `staking` - Validator tiers & staking
- `consensus-day` - Annual governance event
- `currency` - ËTR, ËTD, VMW tokens
- `transactions` - Transaction processing
- `p2p` - DETR networking
- `identity` - OpenDID
- `security` - Cryptography
- `vm` - ETWASM smart contracts
- `multichain` - FlareChain + PBCs
- `bridges` - Cross-chain protocols

### Convenience Bundles

- `wallet` = `accounts` + `currency` + `transactions` + `identity`
- `validator` = `consensus` + `staking` + `p2p` + `multichain`
- `dao` = `governance` + `consensus-day` + `staking`
- `full` = All features (large compile time)

## Examples

### Wallet Application

```rust
use etrid_sdk::prelude::*;
use etrid_sdk::accounts::Account;
use etrid_sdk::currency::etr;

fn send_tokens(from: &Account, to: &Account, amount: u128) {
    let tx = Transaction::transfer(from, to, amount);
    // Sign and submit
}
```

### Governance Participation

```rust
use etrid_sdk::consensus_day::{ProposalCategory, ProposalRecord};
use etrid_sdk::staking::{Role, StakeRequirement};

fn submit_proposal(account: &Account) {
    // Must be Director (≥128 ËTR)
    let proposal = ProposalRecord::new(
        account.clone(),
        ProposalCategory::EconomicAdjustment,
        b"Proposal Title".to_vec(),
        b"Detailed description...".to_vec(),
    );
}
```

### Running a Validator

```rust
use etrid_sdk::consensus::Validator;
use etrid_sdk::staking::Role;

fn start_validator(account: &Account, stake: u128) {
    // Register as Flare Node
    Validator::register(account, stake, Role::FlareNode);
}
```

## Architecture

```
┌──────────────────────────────────────┐
│          ËTRID SDK (You Are Here)    │
│      Unified Developer-Friendly API  │
└────────────┬─────────────────────────┘
             │
       ┌─────┴───────┐
       │             │
  ┌────▼────┐   ┌───▼────┐
  │  Layer1 │   │ Layer2 │
  │  Flare  │◄──┤ Light. │
  │  Chain  │   │  Bloc  │
  └────┬────┘   └────────┘
       │
       ▼
  ┌──────────────────────────────┐
  │  12 Partition Burst Chains   │
  │ BTC ETH SOL XLM XRP BNB ...  │
  └──────────────────────────────┘
```

## Token Economics

| Token | Purpose | Staking Tiers |
|-------|---------|---------------|
| **ËTR** | Utility | Directors: ≥128<br>Validity: ≥64<br>Common: ≥1 |
| **ËTD** | Stablecoin | USD 1:1 peg |
| **VMW** | Gas | Smart contract execution |

## Consensus Day

**Annual governance event on December 1st**

All stakeholders vote on:
- Protocol upgrades
- Economic parameters
- Director elections
- Treasury allocations
- Fiscal minting

## Documentation

- **API Docs**: https://docs.rs/etrid-sdk
- **Guide**: https://docs.etrid.io
- **Examples**: See `/examples` directory

## Support

- **Website**: https://etrid.io
- **Discord**: https://discord.gg/etrid
- **GitHub**: https://github.com/etrid/etrid

## License

Apache-2.0
