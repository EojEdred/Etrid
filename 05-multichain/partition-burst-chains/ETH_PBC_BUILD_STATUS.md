# ETH-PBC Collator - Working Build Status

**Date**: November 8, 2025
**Status**: ✅ **WORKING BINARY AVAILABLE**

---

## Working Binary Location

### Local Binary
- **Path**: `/Users/macbook/Desktop/etrid-binaries/eth-pbc-collator`
- **Size**: 43MB
- **Architecture**: Linux x86-64 (ELF 64-bit LSB PIE executable)
- **Status**: Production-ready, stripped binary

### Remote VMs
The binary is deployed on VMs at:
- `/root/pbc-builds/etrid/05-multichain/partition-burst-chains/eth-pbc-workspace/eth-pbc-linux-deployment/eth-pbc-collator`

---

## Build Approach: Isolated Workspace (WORKING SOLUTION)

### Source Location
```
/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/eth-pbc-workspace/
```

### Why This Approach Works

**The Problem:**
- Frontier EVM requires `polkadot-stable2506`
- Main Ëtrid workspace uses `polkadot-stable2509`
- Mixing versions causes duplicate `sp_io` lang items (panic_impl)

**The Solution:**
- **Isolated Workspace**: Separate Cargo workspace exclusively for eth-pbc
- **Uniform Versioning**: Everything uses stable2506 throughout
- **Complete Collator**: Builds full node binary, not just runtime

### What's Included

```
eth-pbc-workspace/
├── Cargo.toml              # Workspace root (stable2506 exclusively)
├── README.md               # Workspace documentation
├── eth-pbc-runtime/        # ETH PBC runtime with Frontier EVM pallets
├── eth-pbc-collator/       # ETH PBC collator node
├── consensus/              # ASF consensus modules (copied from main)
│   ├── primitives/consensus-asf/
│   ├── client/consensus-asf/
│   ├── pallet/
│   ├── asf-algorithm/
│   └── block-production/
├── 04-accounts/pallet/     # Accounts pallet dependency
└── pallets/pallet-etr-lock/  # ETR token lock pallet
```

---

## Build Commands

### To Build from Source

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/eth-pbc-workspace
cargo build --release -p eth-pbc-collator

# Binary location after build
# target/release/eth-pbc-collator
```

### Cross-compile for Linux (from macOS)

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/eth-pbc-workspace
cargo build --release --target x86_64-unknown-linux-gnu -p eth-pbc-collator
```

---

## What the Binary Does

### Core Functionality

1. **Partition Burst Chain (PBC) Collator Node**
   - Participates in Ëtrid multichain network
   - Processes Ethereum-compatible transactions via Frontier EVM
   - Produces blocks using ASF consensus algorithm
   - Syncs with other nodes in the network
   - Submits state roots to FlareChain relay chain

2. **ASF Consensus**
   - Custom Adaptive Stake-weighted Finality consensus
   - Block production and finalization
   - Peer validation and reputation

3. **EVM Runtime**
   - Frontier EVM pallets for Ethereum compatibility
   - Supports Solidity smart contracts
   - Ethereum JSON-RPC APIs

4. **Network Environments**
   - Development: `ETH-PBC Development`
   - Local Testnet: `ETH-PBC Local Testnet` (chain ID: `eth-pbc`, `local_testnet`)

5. **P2P Networking**
   - Noise_XX_25519_ChaChaPoly_SHA256 encryption
   - libp2p for peer-to-peer communication
   - Block synchronization and state sync

6. **Database**
   - RocksDB for persistent storage
   - Stores blocks, transactions, chain state, peer info

---

## Runtime-Only Build (ALTERNATIVE APPROACH - IN PROGRESS)

An alternative approach is being developed in:
```
/Users/macbook/Desktop/etrid/05-multichain/partition-burst-chains/pbc-chains/eth-pbc/runtime
```

### Status
- ⏳ In progress - attempting to build just the runtime with tokio git patch
- Uses standalone workspace declaration within runtime Cargo.toml
- Patches tokio from git to fix substrate-prometheus-endpoint issues
- **NOT YET WORKING** - use the isolated workspace approach above for production

---

## Deployment

The working binary is ready for deployment to:
- Ëtrid validator VMs
- Partition Burst Chain collator nodes
- Test environments

### Prerequisites
- Linux x86-64 system
- RocksDB libraries
- Network connectivity to Ëtrid network bootnodes

---

## Technical Specifications

### Built With
- **Polkadot SDK**: stable2506 (tag: `polkadot-stable2506`)
- **Tokio**: 1.48.0 (async runtime)
- **Wasmtime**: 8.0.1 (WASM execution)
- **Compiler**: Cranelift backend
- **Database**: RocksDB

### Dependencies
All Polkadot SDK dependencies use stable2506:
- `sp-*` primitives → stable2506
- `sc-*` client → stable2506
- `frame-*` → stable2506
- `pallet-*` → stable2506
- **Frontier** → frontier-stable2506
- **ASF Consensus** → Uses workspace deps (stable2506)

---

## References

- **Solution Documentation**: `/Users/macbook/Desktop/etrid/docs/mainnet/ETH_PBC_SOLUTION_COMPLETE.md`
- **Original Implementation**: November 4, 2025
- **Build Verification**: November 8, 2025

---

## Next Steps

1. ✅ Binary saved to local machine
2. ✅ Binary verified on remote VMs
3. ⏭ Deploy to production collator nodes
4. ⏭ Test Ethereum compatibility
5. ⏭ Connect to FlareChain relay
