# PBC Wrapped Token Configuration Guide

## Overview

Each PBC bridge pallet has storage for registering wrapped tokens on external chains. The flow is:

```
1. Deploy wrapped ËTR on external chain (SPL, BEP-20, ERC-20, etc.)
2. Deploy PBC collator binary to VM
3. Configure bridge with token address via extrinsic
4. Register relayers
5. Bridge is operational
```

---

## Current Status

| Chain | PBC | Wrapped Token | Status |
|-------|-----|---------------|--------|
| Solana | sol-pbc | SPL ËTR (CA4ALvCam...) | ✅ DEPLOYED |
| BSC | bnb-pbc | BEP-20 ËTR | ⏳ TODO |
| Ethereum | eth-pbc | ERC-20 ËTR | ⏳ TODO |
| Polygon | matic-pbc | ERC-20 ËTR | ⏳ TODO |
| Tron | trx-pbc | TRC-20 ËTR | ⏳ TODO |

---

## Chain-Specific Configuration

### 1. SOLANA (Already Done)

**Wrapped Token:** SPL ËTR
**Contract Address:** `CA4ALvCam45ecioBfZ7BzPsXMf3r6BRXZ8iKdGpPmqhp`

**Storage Items:**
```rust
SupportedTokens<T>: Map<SplTokenMint, bool>      // Token enabled
TokenRates<T>: Map<SplTokenMint, u128>           // Exchange rate
UsdcMint<T>: SplTokenMint                        // USDC SPL address
UsdtMint<T>: SplTokenMint                        // USDT SPL address
```

**Configuration Extrinsics:**
```rust
// 1. Set bridge operator (root/sudo)
SolBridge::set_operator(origin, operator_account)

// 2. Add SPL ËTR token
SolBridge::add_supported_token(
    origin,
    token_mint: 0x...(CA4ALvCam... as H256),
    exchange_rate: 1_000_000_000  // 1:1 scaled by 1e9
)

// 3. Register relayers
SolBridge::register_relayer(origin, relayer_account)
```

**To Configure Your SPL ËTR:**
```bash
# Convert Solana address to H256 (32 bytes)
# CA4ALvCam45ecioBfZ7BzPsXMf3r6BRXZ8iKdGpPmqhp
# Base58 decode → pad to 32 bytes → H256

# Via polkadot.js or CLI:
solBridge.setOperator(YOUR_OPERATOR_ACCOUNT)
solBridge.addSupportedToken(TOKEN_MINT_H256, 1000000000)
solBridge.registerRelayer(RELAYER_ACCOUNT)
```

---

### 2. BSC (BNB Chain)

**Wrapped Token:** BEP-20 ËTR (to deploy)
**Contract Type:** ERC-20 compatible on BSC

**Storage Items:**
```rust
SupportedTokens<T>: Map<Bep20Contract, bool>     // H160 (20 bytes)
TokenRates<T>: Map<Bep20Contract, u128>
BusdContract<T>: Bep20Contract                   // BUSD address
```

**Configuration Extrinsics:**
```rust
// 1. Set bridge operator
BnbBridge::set_operator(origin, operator_account)

// 2. Add BEP-20 ËTR token
BnbBridge::add_supported_token(
    origin,
    token_contract: 0x1234...abcd,  // Your BEP-20 address (H160)
    exchange_rate: 1_000_000_000_000_000_000  // 1:1 scaled by 1e18
)

// 3. Register relayers
BnbBridge::register_relayer(origin, relayer_account)
```

**Deployment Steps:**
1. Deploy BEP-20 token on BSC (cost: ~$10)
2. Create PancakeSwap liquidity pool (cost: ~$150)
3. Get contract address (e.g., `0x1234...abcd`)
4. Configure BNB PBC with address

---

### 3. ETHEREUM

**Wrapped Token:** ERC-20 ËTR (to deploy)

**Storage Items:**
```rust
SupportedTokens<T>: Map<EthAddress, bool>        // H160
TokenRates<T>: Map<EthAddress, u128>
UsdcContract<T>: EthAddress
UsdtContract<T>: EthAddress
```

**Configuration Extrinsics:**
```rust
EthBridge::set_operator(origin, operator_account)
EthBridge::add_supported_token(origin, contract_address, exchange_rate)
EthBridge::register_relayer(origin, relayer_account)
```

---

### 4. POLYGON

**Wrapped Token:** ERC-20 ËTR (Polygon)

**Storage Items:**
```rust
RegisteredTokens<T>: Map<PolygonAddress, TokenInfo>
SupportedTokens<T>: Map<PolygonAddress, bool>
```

**Configuration Extrinsics:**
```rust
PolygonBridge::set_operator(origin, operator_account)
PolygonBridge::register_token(
    origin,
    contract_address: 0x...,
    symbol: b"ETR".to_vec(),
    decimals: 18,
    exchange_rate: 1_000_000_000_000_000_000
)
PolygonBridge::register_relayer(origin, relayer_account)
```

---

### 5. TRON

**Wrapped Token:** TRC-20 ËTR

**Storage Items:**
```rust
SupportedTokens<T>: Map<TronAddress, bool>
TokenRates<T>: Map<TronAddress, u128>
```

**Note:** Tron addresses are Base58 encoded, convert to bytes for storage.

---

## Full Deployment Workflow

### Step 1: Deploy Wrapped Token on External Chain

**For BSC (example):**
```solidity
// Simple ERC-20 token
contract WrappedETR is ERC20 {
    address public bridge;

    constructor() ERC20("Wrapped ETR", "wETR") {
        bridge = msg.sender;
    }

    function mint(address to, uint256 amount) external {
        require(msg.sender == bridge, "Only bridge");
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) external {
        require(msg.sender == bridge, "Only bridge");
        _burn(from, amount);
    }
}
```

**Deploy using:**
- Remix IDE
- Hardhat
- Or use existing verified ERC-20 template

### Step 2: Deploy PBC Collator to VM

```bash
# SSH to VM
ssh user@bnb-pbc-vm

# Copy binary
scp target/release/bnb-pbc-collator user@vm:/opt/etrid/

# Start node
/opt/etrid/bnb-pbc-collator \
  --chain=/opt/etrid/bnb-pbc-chainspec.json \
  --name="BNB-PBC-1" \
  --rpc-cors=all \
  --rpc-port=9944
```

### Step 3: Configure Bridge via Extrinsic

**Using polkadot.js Apps:**

1. Connect to PBC node: `ws://bnb-pbc-vm:9944`
2. Go to Developer → Extrinsics
3. Select `bnbBridge` pallet
4. Call `setOperator` (sudo)
5. Call `addSupportedToken` with your BEP-20 address
6. Call `registerRelayer` for each relayer

**Using subxt (Rust CLI):**
```rust
let api = OnlineClient::<PolkadotConfig>::from_url("ws://bnb-pbc:9944").await?;

// Set operator
let tx = etrid::tx().bnb_bridge().set_operator(operator);
api.tx().sign_and_submit_then_watch_default(&tx, &signer).await?;

// Add token
let tx = etrid::tx().bnb_bridge().add_supported_token(
    contract_address,
    1_000_000_000_000_000_000u128, // 1e18
);
api.tx().sign_and_submit_then_watch_default(&tx, &signer).await?;
```

### Step 4: Register Relayers

```rust
// For each relayer account
BnbBridge::register_relayer(origin, relayer_1)
BnbBridge::register_relayer(origin, relayer_2)
BnbBridge::register_relayer(origin, relayer_3)
```

### Step 5: Test Bridge

```bash
# Monitor events on PBC
wscat -c ws://bnb-pbc:9944
> {"jsonrpc":"2.0","method":"chain_subscribeNewHeads","params":[],"id":1}

# Test deposit flow
# 1. User deposits BEP-20 ËTR to bridge contract on BSC
# 2. Relayer detects deposit, submits to PBC
# 3. PBC verifies, mints native ËTR on Primearc
```

---

## Token Address Formats

| Chain | Type | Format | Example |
|-------|------|--------|---------|
| Solana | SPL Mint | Base58 → H256 | `CA4ALvCam...` |
| BSC | BEP-20 | H160 (20 bytes) | `0x1234...abcd` |
| Ethereum | ERC-20 | H160 (20 bytes) | `0xabcd...1234` |
| Polygon | ERC-20 | H160 (20 bytes) | `0x5678...efgh` |
| Tron | TRC-20 | Base58 → bytes | `TR7NHqj...` |

---

## Extrinsic Reference by Chain

| Chain | Set Operator | Add Token | Register Relayer |
|-------|--------------|-----------|------------------|
| Solana | `solBridge.setOperator` | `solBridge.addSupportedToken` | `solBridge.registerRelayer` |
| BSC | `bnbBridge.setOperator` | `bnbBridge.addSupportedToken` | `bnbBridge.registerRelayer` |
| Ethereum | `ethBridge.setOperator` | `ethBridge.addSupportedToken` | `ethBridge.registerRelayer` |
| Polygon | `polygonBridge.setOperator` | `polygonBridge.registerToken` | `polygonBridge.registerRelayer` |
| Tron | `trxBridge.setOperator` | `trxBridge.addSupportedToken` | `trxBridge.registerRelayer` |
| XRP | `xrpBridge.setOperator` | `xrpBridge.addSupportedToken` | `xrpBridge.registerRelayer` |
| Bitcoin | `btcBridge.setOperator` | N/A (BTC only) | `btcBridge.registerRelayer` |

---

## Quick Start: Configure Solana PBC with Existing SPL Token

Your SPL ËTR: `CA4ALvCam45ecioBfZ7BzPsXMf3r6BRXZ8iKdGpPmqhp`

```bash
# 1. Connect to Sol-PBC node
# 2. Run these extrinsics via polkadot.js or CLI:

# Set operator (sudo required)
sudo solBridge.setOperator(YOUR_SUDO_ACCOUNT)

# Add SPL ËTR token
# Convert CA4ALvCam45ecioBfZ7BzPsXMf3r6BRXZ8iKdGpPmqhp to H256
# (Base58 decode, left-pad to 32 bytes)
solBridge.addSupportedToken(
    0x0000000000000000[decoded_bytes],
    1000000000  // 1:1 rate
)

# Register relayers
solBridge.registerRelayer(RELAYER_1)
solBridge.registerRelayer(RELAYER_2)
```

---

## Summary

**Yes, each PBC can configure its own wrapped tokens!**

The pattern is identical across all chains:
1. `SupportedTokens` storage map holds token addresses
2. `add_supported_token` extrinsic registers new tokens
3. `register_relayer` authorizes accounts to submit bridge txs

Once you deploy binaries to VMs, you configure each PBC with:
- The wrapped token contract address from that chain
- Exchange rate (usually 1:1)
- Authorized relayer accounts
