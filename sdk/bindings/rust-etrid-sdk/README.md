# Rust Ëtrid SDK

**Status:** ✅ COMPLETE (Re-export of core SDK)

## Overview

This is a convenience re-export of the core Ëtrid SDK for projects that want a standalone Rust crate.

## Installation

```toml
[dependencies]
etrid-sdk = "1.0"
```

## Usage

This crate re-exports all functionality from the main SDK at `/sdk/src/`:

```rust
use etrid_sdk::{Wallet, Client, Transaction};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to FlareChain
    let client = Client::connect("wss://flarechain.etrid.io").await?;

    // Create or import wallet
    let wallet = Wallet::from_mnemonic("your twelve word mnemonic...")?;

    // Get balance
    let balance = client.get_balance(&wallet.address).await?;
    println!("Balance: {} ETR", balance);

    // Send transaction
    let tx = Transaction::transfer()
        .from(&wallet)
        .to("5Gx...")
        .amount("1000000000000") // 1 ETR
        .chain(Chain::FlareChain)
        .build()?;

    let hash = client.submit(tx).await?;
    println!("Transaction submitted: {}", hash);

    Ok(())
}
```

## Features

Same as the core SDK:

- `wallet` - Wallet operations (enabled by default)
- `validator` - Validator management
- `dao` - DAO governance
- `full` - All features

```toml
[dependencies]
etrid-sdk = { version = "1.0", features = ["full"] }
```

## Documentation

See the main SDK documentation at `/sdk/README.md`

## Source

This is a thin wrapper around the core SDK implementation.

**Core Implementation:** `/sdk/src/lib.rs` (714 lines)
