# Ëtrid XCM Integration Testing Guide

## Overview

This guide covers testing the XCM integration between FlareChain and ETH-PBC, including custom EVM precompiles that enable Solidity contracts to access FlareChain services via XCM.

## Quick Start

### Prerequisites

- ✅ Zombienet installed (`bin/zombienet`)
- ✅ Polkadot relay chain binary (`bin/polkadot`)
- ✅ FlareChain node binary (`target/release/flarechain-node`)
- ⚠️ ETH-PBC node binary (see "Building ETH-PBC Node" below)

### Setup

```bash
# 1. Run setup script
./scripts/setup-zombienet.sh

# 2. Verify binaries
ls -la bin/
ls -la target/release/flarechain-node
```

## Building ETH-PBC Node

ETH-PBC currently only has a runtime. You need to create a node binary:

### Option A: Use Generic Parachain Collator Template

```bash
# Navigate to ETH-PBC
cd 05-multichain/partition-burst-chains/pbc-chains/eth-pbc

# Create node directory from template
mkdir -p node/src
cd node

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "eth-pbc-node"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "eth-pbc-node"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0.0", features = ["derive"] }
futures = "0.3"
log = "0.4"
codec = { package = "parity-scale-codec", version = "3.0.0" }
serde = { version = "1.0", features = ["derive"] }
jsonrpsee = { version = "0.16", features = ["server"] }
sc-cli = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-core = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-executor = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-network = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-service = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-telemetry = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-transaction-pool = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-transaction-pool-api = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-consensus-aura = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-consensus-aura = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-consensus = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-consensus = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sc-client-api = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-runtime = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-io = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-timestamp = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-inherents = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
sp-keyring = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
frame-system = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
pallet-transaction-payment = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }

# Cumulus
cumulus-client-cli = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-client-collator = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-client-consensus-aura = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-client-consensus-common = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-client-network = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-client-service = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-primitives-core = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-primitives-parachain-inherent = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }
cumulus-relay-chain-interface = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-stable2506" }

# Frontier (EVM)
fc-api = { git = "https://github.com/paritytech/frontier", branch = "polkadot-v1.7.0" }
fc-consensus = { git = "https://github.com/paritytech/frontier", branch = "polkadot-v1.7.0" }
fc-db = { git = "https://github.com/paritytech/frontier", branch = "polkadot-v1.7.0" }
fc-mapping-sync = { git = "https://github.com/paritytech/frontier", branch = "polkadot-v1.7.0" }
fc-rpc = { git = "https://github.com/paritytech/frontier", branch = "polkadot-v1.7.0" }
fc-rpc-core = { git = "https://github.com/paritytech/frontier", branch = "polkadot-v1.7.0" }
fc-storage = { git = "https://github.com/paritytech/frontier", branch = "polkadot-v1.7.0" }

# Local
eth-pbc-runtime = { path = "../runtime" }
EOF

# Copy main.rs from FlareChain or solochain template
# (Adapt for ETH-PBC with Frontier support)

# Build
cargo build --release
```

### Option B: Use FlareChain Node as Template (Simpler)

For initial testing, you can temporarily use FlareChain node with ETH-PBC runtime:

```bash
# Symlink for testing
cd target/release
ln -s flarechain-node eth-pbc-node
```

## Running Zombienet

### Start the Network

```bash
# From project root
./bin/zombienet spawn zombienet-xcm-test.toml
```

This will start:
- **Relay Chain (Rococo Local)**: 2 validators (Alice, Bob)
- **FlareChain Parachain (2000)**: 2 collators
- **ETH-PBC Parachain (2001)**: 2 collators
- **HRMP Channels**: Bidirectional between FlareChain ↔ ETH-PBC

### Connection Endpoints

| Component | WebSocket | RPC |
|---|---|---|
| Relay Chain (Alice) | ws://localhost:9944 | http://localhost:9933 |
| Relay Chain (Bob) | ws://localhost:9945 | http://localhost:9934 |
| FlareChain Collator 1 | ws://localhost:9946 | http://localhost:9935 |
| FlareChain Collator 2 | ws://localhost:9947 | http://localhost:9936 |
| ETH-PBC Collator 1 | ws://localhost:9948 | http://localhost:9937 |
| ETH-PBC Collator 2 | ws://localhost:9949 | http://localhost:9938 |

## Testing Custom Precompiles

### Prerequisites

- Hardhat or Foundry for contract deployment
- MetaMask or other Web3 wallet
- Node.js (for JavaScript tests)

### 1. Connect MetaMask to ETH-PBC

- Network Name: ETH-PBC Local
- RPC URL: http://localhost:9937
- Chain ID: 2001 (or check runtime config)
- Currency Symbol: ETR

### 2. Deploy Test Contracts

Example contracts are in `/contracts/etwasm-examples/`:

- **oracle-price-feed.sol**: Oracle precompile examples
- **governance-dao.sol**: Governance precompile examples
- **staking-rewards.sol**: Staking precompile examples

#### Using Hardhat

```javascript
// hardhat.config.js
module.exports = {
  networks: {
    ethPbc: {
      url: "http://localhost:9937",
      accounts: ["0x... your private key"],
      chainId: 2001
    }
  },
  solidity: "0.8.0"
};
```

```bash
# Deploy
npx hardhat run scripts/deploy-oracle.js --network ethPbc
```

#### Using Foundry

```bash
# Deploy OraclePriceFeed
forge create contracts/etwasm-examples/oracle-price-feed.sol:OraclePriceFeed \
    --rpc-url http://localhost:9937 \
    --private-key 0x...

# Interact
cast call <CONTRACT_ADDRESS> "getAssetPriceInETH(string)" "BTC" \
    --rpc-url http://localhost:9937
```

### 3. Run Integration Tests

```bash
# Test precompiles
node scripts/test-xcm-precompiles.js
```

This script tests:
- Oracle price queries (BTC, ETH, SOL)
- Governance proposal submission and voting
- Staking validator queries

### Expected Behavior

**In Mock Mode** (default):
- Precompiles return mock data immediately
- No XCM messages sent
- Fast testing without network dependencies

**In Production Mode** (with HRMP channels):
- Precompiles trigger XCM messages to FlareChain
- Response cached for subsequent calls
- 2-4 block latency for first query
- Subsequent queries use cached data

## Monitoring XCM Messages

### Using Polkadot.js Apps

1. Connect to relay chain: https://polkadot.js.org/apps/?rpc=ws://localhost:9944
2. Navigate to **Network > Parachains**
3. Monitor HRMP channels and message queue

### Using Subscan (for testnet/mainnet)

- FlareChain: (URL TBD)
- ETH-PBC: (URL TBD)

### Logs

Zombienet outputs logs to console. Watch for:

```
[ETH-PBC] XCM message sent to FlareChain: QueryOracle(BTC/ETH)
[FlareChain] Received XCM query from ETH-PBC
[FlareChain] Oracle price: BTC = 50000 ETH
[FlareChain] Sending XCM response to ETH-PBC
[ETH-PBC] XCM response received, caching result
```

## Troubleshooting

### Zombienet Won't Start

```bash
# Check binaries exist
ls -la bin/zombienet bin/polkadot target/release/flarechain-node target/release/eth-pbc-node

# Check ports are free
lsof -i :9933-9949

# Clean and restart
pkill -f zombienet
pkill -f polkadot
pkill -f flarechain-node
pkill -f eth-pbc-node
./bin/zombienet spawn zombienet-xcm-test.toml
```

### Precompile Returns Error

```
Error: execution reverted
```

**Possible causes:**
1. HRMP channels not set up (use mock mode)
2. XCM message failed (check logs)
3. Invalid parameter format (check ABI encoding)

**Solution:**
- Verify contract is using correct precompile address
- Check Zombienet logs for XCM errors
- Test with mock data first

### MetaMask Connection Issues

- Ensure ETH-PBC node is running (`curl http://localhost:9937`)
- Check Chain ID matches network config
- Try clearing MetaMask cache

### No XCM Messages Sent

**Check:**
1. HRMP channels are open: `polkadot-js-api query.hrmp.hrmpChannels 2001 2000`
2. Production XCM bridge is configured in runtime
3. Precompile is calling XCM bridge (check implementation)

## Next Steps

After successful local testing:

1. **Deploy to Testnet**
   - Register parachains on Rococo/Westend
   - Set up HRMP channels (`scripts/setup-hrmp-channels.sh`)
   - Deploy production contracts

2. **Production Deployment**
   - Register on Polkadot/Kusama
   - Configure production XCM routes
   - Enable production XCM bridge mode
   - Monitor XCM message delivery

3. **Advanced Testing**
   - Stress test XCM message queue
   - Test failover and retry logic
   - Benchmark gas costs with XCM
   - Test with real oracle data

## Resources

- **Documentation**: `docs/technical/XCM_INTEGRATION_GUIDE.md`
- **Custom Precompiles**: `docs/technical/CUSTOM_PRECOMPILES_GUIDE.md`
- **Solidity Interfaces**: `05-multichain/partition-burst-chains/pbc-chains/eth-pbc/solidity-interfaces/`
- **Example Contracts**: `contracts/etwasm-examples/`
- **Test Scripts**: `scripts/test-xcm-precompiles.js`

## Support

For issues or questions:
- Check documentation in `docs/technical/`
- Review XCM integration guide
- Check Zombienet logs for detailed errors
