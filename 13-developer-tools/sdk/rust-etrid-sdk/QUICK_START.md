# Ëtrid Rust SDK - Quick Start Guide

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
etrid-sdk = { path = "../rust-etrid-sdk" }
tokio = { version = "1", features = ["full"] }
```

## Basic Usage

### 1. Connect to Node

```rust
use etrid_sdk::Client;

let client = Client::new("ws://localhost:9944").await?;
```

### 2. Account Operations

```rust
use etrid_sdk::wrappers::{AccountsWrapper, TransferParams};

let accounts = AccountsWrapper::new(client);

// Get balance
let balance = accounts.get_balance("5GrwvaEF...").await?;
println!("Free: {}", balance.free);

// Transfer
let tx = accounts.transfer(TransferParams {
    to: "5FHneW46...".to_string(),
    amount: 1_000_000_000_000,
}).await?;
```

### 3. Staking

```rust
use etrid_sdk::wrappers::{StakingWrapper, BondParams, RewardDestination};

let staking = StakingWrapper::new(client);

// Bond tokens
staking.bond(BondParams {
    amount: 1_000_000_000_000,
    reward_destination: RewardDestination::Staked,
}).await?;

// Estimate rewards
let rewards = staking.estimate_rewards(1_000_000_000_000, None).await?;
println!("APY: {}%", rewards.apy);
```

### 4. Governance

```rust
use etrid_sdk::wrappers::{GovernanceWrapper, VoteParams, ConvictionLevel};
use etrid_sdk::types::VoteType;

let gov = GovernanceWrapper::new(client);

// Vote on proposal
gov.vote(VoteParams {
    proposal_id: 1,
    vote: VoteType::Aye,
    conviction: ConvictionLevel::Locked2x,
}).await?;
```

### 5. Lightning Channels

```rust
use etrid_sdk::wrappers::{LightningBlocWrapper, OpenChannelParams};

let lightning = LightningBlocWrapper::new(client);

let channel = lightning.open_channel(OpenChannelParams {
    peer: "5GrwvaEF...".to_string(),
    capacity: 1_000_000_000_000,
    push_amount: 100_000_000_000,
}).await?;
```

### 6. Smart Contracts

```rust
use etrid_sdk::wrappers::{EtwasmVmWrapper, DeployContractParams};

let etwasm = EtwasmVmWrapper::new(client);

let deployment = etwasm.deploy_contract(DeployContractParams {
    code: wasm_code,
    salt: vec![],
    gas_limit: 1_000_000,
}).await?;
```

### 7. Cross-Chain Bridge

```rust
use etrid_sdk::wrappers::{BridgeWrapper, BridgeParams};
use etrid_sdk::types::Chain;

let bridge = BridgeWrapper::new(client);

let transfer = bridge.bridge(BridgeParams {
    destination_chain: Chain::Ethereum,
    amount: 1_000_000_000_000,
    recipient: "0x742d35...".to_string(),
}).await?;
```

### 8. Oracle Prices

```rust
use etrid_sdk::wrappers::OracleWrapper;

let oracle = OracleWrapper::new(client);

let price = oracle.get_price("ETR/USD").await?;
println!("Price: ${}", price.price);
```

### 9. Reserve Vault

```rust
use etrid_sdk::wrappers::{ReserveVaultWrapper, CreateVaultParams, DepositParams};
use etrid_sdk::types::CollateralType;

let vault = ReserveVaultWrapper::new(client);

let vault_id = vault.create_vault(CreateVaultParams {
    collateral_type: CollateralType::Eth,
}).await?;

vault.deposit_collateral(DepositParams {
    vault_id: vault_id.clone(),
    amount: 10_000_000_000_000,
}).await?;
```

### 10. Distribution Rewards

```rust
use etrid_sdk::wrappers::{DistributionPayWrapper, ClaimRewardParams};
use etrid_sdk::types::RewardCategory;

let distribution = DistributionPayWrapper::new(client);

let rewards = distribution.get_pending_rewards(RewardCategory::Validator).await?;
distribution.claim_reward(ClaimRewardParams {
    category: RewardCategory::Validator,
}).await?;
```

## All 10 Core Wrappers

1. **LightningBlocWrapper** - Layer 2 payment channels
2. **DistributionPayWrapper** - Rewards distribution
3. **EtwasmVmWrapper** - Smart contracts
4. **AiDidWrapper** - AI identity management
5. **BridgeWrapper** - Cross-chain transfers
6. **OracleWrapper** - Price feeds
7. **ReserveVaultWrapper** - Lending/borrowing
8. **StakingWrapper** - Token staking
9. **GovernanceWrapper** - Democratic voting
10. **AccountsWrapper** - Balance & transfers

## Error Handling

All methods return `Result<T, Error>`:

```rust
match accounts.transfer(params).await {
    Ok(tx_hash) => println!("Success: {}", tx_hash),
    Err(Error::InsufficientBalance { required, available }) => {
        println!("Not enough funds: need {}, have {}", required, available);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Type Safety

The SDK uses strongly-typed parameters:

```rust
// Typed enums prevent invalid states
pub enum VoteType { Aye, Nay, Abstain }
pub enum ChannelState { Pending, Open, Closing, Closed }
pub enum Chain { Ethereum, Bitcoin, Solana, ... }

// Validated structs ensure correct inputs
pub struct TransferParams {
    pub to: Address,
    pub amount: u128,  // Validated > 0
}
```

## Best Practices

1. **Always handle errors** - Don't unwrap() in production
2. **Use batch operations** - More efficient for multiple transfers
3. **Check balances first** - Prevent failed transactions
4. **Estimate fees** - Use wrapper fee estimation methods
5. **Use conviction** - Lock votes for higher voting power

## Resources

- API Documentation: `cargo doc --open`
- Examples: `rust-etrid-sdk/examples/`
- Full Report: `IMPLEMENTATION_REPORT.md`
- Main README: `README.md`

## Need Help?

- GitHub Issues: https://github.com/etrid/etrid
- Discord: https://discord.gg/etrid
- Docs: https://docs.etrid.io
