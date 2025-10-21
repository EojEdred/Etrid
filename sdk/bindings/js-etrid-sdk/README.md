# JavaScript/TypeScript Ëtrid SDK

**Status:** 📋 Planned for v1.1 (Post-Mainnet)

## Overview

JavaScript and TypeScript bindings for the Ëtrid blockchain SDK.

## Planned Features

- **WASM-based**: Compiled from Rust SDK using wasm-bindgen
- **TypeScript-first**: Full type definitions included
- **Universal**: Works in Browser and Node.js
- **Async/Await**: Promise-based modern API
- **Tree-shakeable**: ES modules for optimal bundle size

## Installation (When Available)

```bash
npm install @etrid/sdk
# or
yarn add @etrid/sdk
```

## Usage Example (Planned API)

```typescript
import { EtridClient, Wallet } from '@etrid/sdk';

// Connect to FlareChain
const client = await EtridClient.connect('wss://flarechain.etrid.io');

// Create or import wallet
const wallet = Wallet.fromMnemonic('your twelve word mnemonic...');

// Get balance
const balance = await client.getBalance(wallet.address);

// Send transaction
const tx = await client.transfer({
  from: wallet,
  to: '5Gx...',
  amount: '1000000000000', // 1 ETR
  chain: 'flarechain'
});

// Multichain operations
const btcPbc = await client.connectChain('btc-pbc');
const btcBalance = await btcPbc.getBridgedBalance(wallet.address);
```

## Roadmap

- **v1.1.0**: Initial release with core wallet functionality
- **v1.1.1**: Multichain support (all 13 PBCs)
- **v1.2.0**: DAO governance integration
- **v1.3.0**: EDSC stablecoin operations

## Development

This SDK will be implemented after mainnet deployment.

**Target Timeline:** Q1 2026 (post-mainnet)

## Temporary Alternative

Until this SDK is ready, use Polkadot.js:

```typescript
import { ApiPromise, WsProvider } from '@polkadot/api';

const provider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider });
```

See: `apps/wallet-web/etrid-crypto-website/lib/api/flarechain.ts` for working example.
