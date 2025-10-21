# Ëtrid Multi-Node Local Testnet Guide

This guide explains how to build, configure, and run a local multi-node Ëtrid testnet for development and testing.

## Overview

The Ëtrid multi-node setup consists of:
- **FlareChain**: Main relay chain with ASF consensus (3 nodes)
- **PBC Collators**: Partition Burst Chain collators for different blockchains (12 total)

## Quick Start

### 1. Build All Nodes

```bash
./scripts/build_all_nodes.sh
```

This builds:
- 1 FlareChain node binary
- 12 PBC collator binaries (BTC, ETH, DOGE, XLM, XRP, BNB, TRX, ADA, LINK, MATIC, SC-USDT, SOL)

**Build time**: ~15-30 minutes (depending on your machine)

### 2. Generate Chain Specifications

```bash
./scripts/generate_chain_specs.sh
```

This creates chain spec files in `chain-specs/`:
- `flarechain-dev.json` - Development chain spec
- `flarechain-local.json` - Local testnet spec
- `flarechain-local-raw.json` - Raw spec for production use
- `pbc-{name}-local.json` - PBC collator specs

### 3. Start the Testnet

```bash
./scripts/deploy_local_testnet.sh
```

This starts:
- **FlareChain Alice** (Validator) on ports 30333, 9944
- **FlareChain Bob** (Validator) on ports 30334, 9945
- **FlareChain Charlie** (Full Node) on ports 30335, 9946
- **BTC PBC Collator** on ports 40000, 8000
- **ETH PBC Collator** on ports 40001, 8001
- **DOGE PBC Collator** on ports 40002, 8002

### 4. Interact with the Network

**Using Polkadot.js Apps**:
1. Open https://polkadot.js.org/apps/
2. Connect to `ws://localhost:9944` (Alice node)
3. Explore blocks, accounts, and extrinsics

**Using curl (RPC)**:
```bash
# Get chain info
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' \
     http://localhost:9944

# Get node version
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_version"}' \
     http://localhost:9944

# Get latest block hash
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlockHash"}' \
     http://localhost:9944
```

### 5. Stop the Testnet

Press `Ctrl+C` in the terminal running `deploy_local_testnet.sh`

---

## Architecture Details

### FlareChain Nodes

#### Alice (Validator)
- **Role**: Validator node with ASF consensus
- **P2P Port**: 30333
- **RPC Port**: 9944
- **Data**: `.local-testnet/flarechain-alice`
- **Log**: `.local-testnet/logs/flarechain-alice.log`

#### Bob (Validator)
- **Role**: Validator node with ASF consensus
- **P2P Port**: 30334
- **RPC Port**: 9945
- **Data**: `.local-testnet/flarechain-bob`
- **Log**: `.local-testnet/logs/flarechain-bob.log`

#### Charlie (Full Node)
- **Role**: Non-validator full node
- **P2P Port**: 30335
- **RPC Port**: 9946
- **Data**: `.local-testnet/flarechain-charlie`
- **Log**: `.local-testnet/logs/flarechain-charlie.log`

### PBC Collators

Each PBC collator connects to FlareChain and processes transactions for its specific blockchain:

| PBC | Blockchain | P2P Port | RPC Port |
|-----|-----------|----------|----------|
| BTC | Bitcoin | 40000 | 8000 |
| ETH | Ethereum | 40001 | 8001 |
| DOGE | Dogecoin | 40002 | 8002 |
| XLM | Stellar | 40003 | 8003 |
| XRP | Ripple | 40004 | 8004 |
| BNB | Binance | 40005 | 8005 |
| TRX | Tron | 40006 | 8006 |
| ADA | Cardano | 40007 | 8007 |
| LINK | Chainlink | 40008 | 8008 |
| MATIC | Polygon | 40009 | 8009 |
| SC-USDT | Stablecoin USDT | 40010 | 8010 |
| SOL | Solana | 40011 | 8011 |

---

## Advanced Configuration

### Running Individual Nodes

#### Start FlareChain Node Manually
```bash
./target/release/flarechain-node \
  --chain local \
  --alice \
  --validator \
  --base-path /tmp/flarechain-alice \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods=unsafe
```

#### Start PBC Collator Manually
```bash
./target/release/btc-pbc-collator \
  --collator \
  --chain local \
  --base-path /tmp/pbc-btc \
  --port 40000 \
  --rpc-port 8000 \
  --rpc-cors all \
  -- \
  --chain local \
  --port 40100
```

### Purging Chain Data

To start fresh:
```bash
rm -rf .local-testnet/flarechain-*
rm -rf .local-testnet/pbc-*
```

### Custom Chain Specifications

Edit chain specs in `chain-specs/` directory:

**Example: Add more initial validators**
```json
{
  "genesis": {
    "runtime": {
      "asf": {
        "initialValidators": [
          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
        ]
      }
    }
  }
}
```

---

## Testing Checklist

### Basic Functionality
- [ ] All 3 FlareChain nodes start without errors
- [ ] Nodes discover each other (check logs for peer connections)
- [ ] Blocks are being produced (check RPC: `chain_getBlockHash`)
- [ ] Validators are participating in consensus

### PBC Collators
- [ ] Collators connect to FlareChain
- [ ] Collators produce parachain blocks
- [ ] Bridge pallets are accessible via RPC

### Cross-Chain Operations
- [ ] Can query bridge pallet state
- [ ] Can submit deposit transactions (once bridge logic is complete)
- [ ] Can submit withdrawal transactions

### Network Health
- [ ] No panic errors in logs
- [ ] Peer count increases over time
- [ ] Block time is consistent (~5 seconds)
- [ ] Finality is working (check GRANDPA logs)

---

## Troubleshooting

### Nodes won't start
**Problem**: Binary not found
```
Solution: Run ./scripts/build_all_nodes.sh
```

**Problem**: Port already in use
```
Solution: Kill existing processes:
  pkill -f flarechain-node
  pkill -f pbc-collator
```

### Nodes can't connect to each other
**Problem**: Firewall blocking connections
```
Solution: Allow ports 30333-30335, 40000-40011, 8000-8011
```

**Problem**: Wrong bootnode address
```
Solution: Check node-key matches in deployment script
```

### Collator won't connect to relay chain
**Problem**: Chain spec mismatch
```
Solution: Ensure both use same chain spec (local)
```

**Problem**: Relay chain not running
```
Solution: Start FlareChain nodes first, then collators
```

### No blocks being produced
**Problem**: No validators active
```
Solution: Check that Alice/Bob started with --validator flag
```

**Problem**: ASF consensus not working
```
Solution: Check logs for ASF-related errors
```

---

## Monitoring

### View Logs in Real-Time
```bash
# FlareChain Alice
tail -f .local-testnet/logs/flarechain-alice.log

# BTC Collator
tail -f .local-testnet/logs/pbc-btc.log

# All logs
tail -f .local-testnet/logs/*.log
```

### Check Network Status
```bash
# Number of peers
curl -s -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' \
     http://localhost:9944 | jq '.result | length'

# Latest block number
curl -s -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
     http://localhost:9944 | jq '.result.number'

# Node health
curl -s -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
     http://localhost:9944 | jq
```

---

## Next Steps

1. **Verify multi-node consensus**: Ensure all validators participate
2. **Test bridge operations**: Submit cross-chain transactions
3. **Performance testing**: Measure TPS and finality time
4. **Upgrade testing**: Test runtime upgrades without downtime
5. **Fault tolerance**: Test node failures and recovery

---

## Development Workflow

### Making Changes
1. Modify code
2. Rebuild: `cargo build --release -p flarechain-node`
3. Stop testnet: `Ctrl+C`
4. Purge data (optional): `rm -rf .local-testnet/flarechain-*`
5. Restart testnet: `./scripts/deploy_local_testnet.sh`

### Adding New PBC Collator
1. Build collator: `cargo build --release -p NEW-pbc-collator`
2. Add to `COLLATORS_TO_START` in `deploy_local_testnet.sh`
3. Assign unique ports
4. Restart testnet

---

## Production Considerations

Before mainnet deployment:

- [ ] Replace development keys with production keys
- [ ] Set up proper telemetry and monitoring
- [ ] Configure firewall and network security
- [ ] Use raw chain specs (not --chain local)
- [ ] Set up backup and disaster recovery
- [ ] Implement proper key management
- [ ] Configure reverse proxy for RPC (nginx)
- [ ] Set up SSL/TLS for RPC endpoints
- [ ] Implement rate limiting for public RPCs
- [ ] Set up log aggregation (ELK/Loki)

---

*For more information, see the main [README.md](README.md) and [Architecture Documentation](docs/architecture/ARCHITECTURE.md)*
