# EVM Architecture: ETH-PBC vs FlareChain

## Overview

Ëtrid's EVM (Ethereum Virtual Machine) support has been architecturally separated to preserve the network's unique identity and optimize performance. This document explains the design decision and implementation.

---

## Architecture Decision: Why ETH-PBC, Not FlareChain?

### The Problem
Initially, EVM support (Frontier pallets) was being added to **FlareChain** (the coordination layer). This created several issues:

1. **Identity Crisis**: FlareChain would become "just another Ethereum L2" rather than Ëtrid's unique multichain protocol
2. **Performance Overhead**: EVM gas metering and state storage would slow down native operations (governance, staking, oracles)
3. **Security Risk**: EVM bugs could affect critical infrastructure (validator committee, consensus day)
4. **Architectural Confusion**: Mixed responsibilities violated separation of concerns

### The Solution
**Move EVM to ETH-PBC** (Ethereum Partition Burst Chain) where it belongs.

---

## Current Architecture (✅ Correct)

```
┌──────────────────────────────────────────────────────────────┐
│                     FLARECHAIN                                │
│              (Pure Coordination Layer)                        │
│                                                               │
│  ✓ ËtwasmVM (native WebAssembly contracts)                  │
│  ✓ Oracle Network (cross-chain price feeds)                  │
│  ✓ Governance & Consensus Day                                │
│  ✓ Staking & Validator Management                            │
│  ✓ PBC Router (coordinates all PBCs)                         │
│  ✓ Cross-chain messaging (XCM/DETRP2P)                      │
│                                                               │
│  ❌ NO EVM - Keeps FlareChain pure & fast                   │
└───────────────────────────┬───────────────────────────────────┘
                            │
                    ┌───────┴────────┐
                    │   PBC Router    │
                    └───────┬────────┘
        ┌──────────────────┼──────────────────┐
        │                  │                  │
┌───────▼────────┐  ┌──────▼─────────┐  ┌───▼──────────┐
│   BTC-PBC      │  │   ETH-PBC      │  │  SOL-PBC     │
│                │  │   ⭐ EVM HERE  │  │              │
│ • Bitcoin      │  │                │  │ • Solana     │
│   bridge       │  │ • Full EVM     │  │   SVM        │
│ • BTC logic    │  │ • Solidity     │  │ • Solana     │
│                │  │ • FlareSwap    │  │   bridge     │
│                │  │ • MetaMask     │  │              │
│                │  │ • web3.js      │  │              │
│                │  │ • ethers.js    │  │              │
└────────────────┘  └────────────────┘  └──────────────┘
```

---

## FlareChain (Coordination Layer)

### Purpose
FlareChain is the **pure native Ëtrid chain** responsible for:
- Cross-chain coordination
- Governance and consensus
- Native currency (ETR) and staking
- Oracle network
- PBC routing

### Technology Stack
| Component | Technology |
|-----------|-----------|
| Smart Contracts | ËtwasmVM (WebAssembly) |
| Consensus | ASF (Adaptive Security Framework) |
| Finality | GRANDPA |
| P2P | DETR P2P Protocol |
| Cross-chain | XCM (Cross-Consensus Messaging) |

### What FlareChain DOES NOT Have
❌ EVM (Ethereum Virtual Machine)
❌ Solidity support
❌ Ethereum JSON-RPC
❌ MetaMask compatibility (on FlareChain directly)
❌ web3.js/ethers.js support (on FlareChain directly)

### Runtime Configuration
- **Location**: `/05-multichain/flare-chain/runtime/`
- **Polkadot SDK**: `polkadot-stable2509`
- **No Frontier pallets** - pure Substrate
- **Native contracts**: ËtwasmVM only

---

## ETH-PBC (Ethereum Partition Burst Chain)

### Purpose
ETH-PBC is the **Ethereum compatibility sandbox** responsible for:
- Full EVM execution environment
- Solidity smart contracts
- Ethereum bridge
- DeFi applications (FlareSwap, etc.)
- MetaMask wallet support

### Technology Stack
| Component | Technology |
|-----------|-----------|
| Smart Contracts | EVM + Solidity |
| EVM Integration | Frontier (Polkadot-EVM) |
| Consensus | Aura (Authority Round) |
| Finality | GRANDPA |
| Features | EIP-7702 (Authorization Lists) |

### What ETH-PBC HAS
✅ Full EVM support (pallet-evm)
✅ Ethereum transaction compatibility (pallet-ethereum)
✅ Dynamic gas pricing (pallet-base-fee, pallet-dynamic-fee)
✅ EVM precompiles (standard + custom)
✅ Ethereum JSON-RPC API
✅ MetaMask support
✅ web3.js and ethers.js compatibility
✅ EIP-7702 authorization lists

### Runtime Configuration
- **Location**: `/05-multichain/partition-burst-chains/pbc-chains/eth-pbc/runtime/`
- **Polkadot SDK**: `polkadot-stable2506` (Frontier compatibility)
- **Frontier pallets**: ✅ Fully integrated
- **Chain ID**: Configurable (set at genesis)

---

## Comparison Table

| Aspect | FlareChain | ETH-PBC |
|--------|-----------|---------|
| **Purpose** | Coordination layer | Ethereum compatibility |
| **Identity** | Pure Ëtrid native | Ethereum sandbox |
| **Smart Contracts** | ËtwasmVM (WASM) | EVM (Solidity) |
| **Performance** | Fast (no EVM overhead) | Optimized for EVM |
| **Security** | Critical (isolated) | Sandboxed (EVM contained) |
| **MetaMask** | Not supported | ✅ Supported |
| **FlareSwap** | Not deployable | ✅ Deployable |
| **Oracle Access** | Native (direct) | Via XCM precompiles |
| **Maintenance** | Substrate core | Frontier + Substrate |

---

## User Experience

### Ethereum Developers
```javascript
// Connect MetaMask to ETH-PBC
const provider = new ethers.providers.Web3Provider(window.ethereum);
await provider.send("wallet_addEthereumChain", [{
  chainId: "0x...", // ETH-PBC Chain ID
  chainName: "Ëtrid ETH-PBC",
  rpcUrls: ["https://eth-pbc.etrid.io"],
  nativeCurrency: {
    name: "ETR",
    symbol: "ETR",
    decimals: 18
  },
  blockExplorerUrls: ["https://eth-pbc.explorer.etrid.io"]
}]);

// Deploy Solidity contracts as normal
const contract = await factory.deploy();
```

### Native Ëtrid Developers
```rust
// Use ËtwasmVM on FlareChain
#[etwasmvm::contract]
pub mod etrid_contract {
    // Native WebAssembly contract
    // Access to governance, oracles, staking
}
```

### Interoperability
ETH-PBC can access FlareChain services via **XCM precompiles**:

```solidity
// Solidity contract on ETH-PBC accessing FlareChain oracle
interface IEtridOracle {
    function getPriceInETH(bytes32 symbol) external view returns (uint256);
}

contract FlareSwap {
    IEtridOracle oracle = IEtridOracle(0x...); // Precompile address

    function swap() public {
        uint256 btcPrice = oracle.getPriceInETH("BTC");
        // Use price from FlareChain oracle
    }
}
```

---

## Benefits of This Architecture

### 1. **Preserves Ëtrid's Identity**
FlareChain remains a unique multichain protocol, not "Ethereum 2.0"

### 2. **Better Performance**
- FlareChain: No EVM overhead → faster governance, staking, oracles
- ETH-PBC: Optimized exclusively for EVM execution

### 3. **Improved Security**
- EVM bugs isolated to ETH-PBC
- Critical infrastructure (consensus, validators) unaffected
- Circuit breakers can pause ETH-PBC without stopping network

### 4. **Clearer Separation of Concerns**
- FlareChain: Coordination & governance
- ETH-PBC: Ethereum compatibility
- Each chain does one thing well

### 5. **Scalability**
Parallel execution:
- FlareChain processes native transactions
- ETH-PBC processes EVM transactions
- Both run simultaneously

### 6. **Developer Experience**
Clear choice:
- Want Ethereum compatibility? → Deploy to ETH-PBC
- Want native Ëtrid features? → Build on FlareChain
- Want both? → Use XCM bridges

---

## Migration Summary

### What Changed
1. ❌ **Removed from FlareChain**:
   - All Frontier pallets (pallet-evm, pallet-ethereum, pallet-base-fee, etc.)
   - EVM precompiles module
   - Ethereum JSON-RPC APIs
   - EVM configuration and gas constants

2. ✅ **Confirmed in ETH-PBC**:
   - Full Frontier EVM stack
   - Ethereum transaction support
   - Dynamic fee market
   - EIP-7702 authorization lists
   - Custom precompiles (for FlareChain access)

### Files Modified
- `/05-multichain/flare-chain/runtime/Cargo.toml` - Removed Frontier dependencies
- `/05-multichain/flare-chain/runtime/src/lib.rs` - Removed EVM configuration
- `/05-multichain/flare-chain/runtime/src/precompiles.rs` - Archived (moved to ETH-PBC)

### Files Verified (ETH-PBC)
- `/05-multichain/partition-burst-chains/pbc-chains/eth-pbc/runtime/Cargo.toml` - ✅ Has Frontier
- `/05-multichain/partition-burst-chains/pbc-chains/eth-pbc/runtime/src/lib.rs` - ✅ Complete EVM config
- `/05-multichain/partition-burst-chains/pbc-chains/eth-pbc/runtime/src/precompiles.rs` - ✅ Exists

---

## Deployment Considerations

### ETH-PBC Chain ID
When launching ETH-PBC, set a unique chain ID:
- Avoid conflicts with existing EVM chains
- Suggested: `3141` (E=3rd letter, TRID=141) or similar
- Configure in genesis/chain spec

### RPC Endpoints
Separate RPC endpoints for clarity:
- FlareChain: `https://rpc.etrid.io` (Substrate RPC)
- ETH-PBC: `https://eth-pbc.etrid.io` (Ethereum JSON-RPC)

### Block Explorers
- FlareChain: Substrate-native explorer (Polkadot.js)
- ETH-PBC: Ethereum-compatible explorer (Blockscout, Etherscan-like)

---

## Future Enhancements

### XCM Precompiles (Planned)
Custom EVM precompiles to access FlareChain from ETH-PBC:
- **Oracle Precompile**: `0x0000...0800` - Get price feeds
- **Governance Precompile**: `0x0000...0801` - Submit proposals
- **Staking Precompile**: `0x0000...0802` - Query validator info

### Additional PBCs
The architecture scales to more chains:
- SOL-PBC: Solana SVM support
- WASM-PBC: CosmWasm contracts
- TON-PBC: TON VM support

---

## Conclusion

By moving EVM to ETH-PBC, Ëtrid achieves:
✅ **Preserved Identity**: FlareChain remains uniquely Ëtrid
✅ **Better Performance**: Specialized chains for specialized tasks
✅ **Improved Security**: Isolation of EVM from critical infrastructure
✅ **Clear Architecture**: Separation of concerns
✅ **Future-Proof**: Scalable to more VMs and chains

This is the correct architectural decision for a multichain protocol.

---

## References

- [Ëtrid Architecture](../architecture.md)
- [PBC Overview](../specifications/pbc-overview.md)
- [Frontier Documentation](https://github.com/polkadot-evm/frontier)
- [XCM Specification](https://wiki.polkadot.network/docs/learn-xcm)

**Document Version**: 1.0
**Last Updated**: 2025-11-05
**Status**: ✅ Implemented
