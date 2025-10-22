# Ëtrid SDK for Rust

Rust library for interacting with the Ëtrid Protocol blockchain.

## Features

- ✅ Account management (create, import, sign)
- ✅ RPC client (WebSocket connection)
- ✅ Type-safe Substrate integration
- 🔨 Transaction building (in progress)
- 🔨 Event subscriptions (planned)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
etrid-sdk = "0.1"
```

## Quick Start

```rust
use etrid_sdk::{Client, Account};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to node
    let client = Client::new("ws://localhost:9944").await?;

    // Create account
    let account = Account::generate();
    println!("Address: {}", account.address());

    // Query chain
    let block_number = client.get_block_number().await?;
    println!("Block: {}", block_number);

    Ok(())
}
```

## Documentation

Run `cargo doc --open` to view full API documentation.

## Status

**Development Status**: Basic implementation complete, full features in progress.
