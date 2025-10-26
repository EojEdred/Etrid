# Ëtrid SDK for JavaScript/TypeScript

JavaScript/TypeScript library for interacting with the Ëtrid Protocol blockchain.

## Features

- ✅ Account management (create, import, sign)
- ✅ WebSocket RPC client
- ✅ TypeScript support with full type definitions
- ✅ Browser and Node.js compatibility
- 🔨 Transaction building (in progress)
- 🔨 Event subscriptions (planned)

## Installation

```bash
npm install @etrid/sdk
# or
yarn add @etrid/sdk
```

## Quick Start

```typescript
import { EtridClient, Account } from '@etrid/sdk';

async function main() {
  // Connect to node
  const client = new EtridClient('ws://localhost:9944');
  await client.connect();

  // Create account
  const account = Account.generate();
  console.log('Address:', account.address);

  // Query balance
  const balance = await client.query.balance(account.address);
  console.log('Balance:', balance.free.toString(), 'ETR');

  await client.disconnect();
}

main();
```

## Documentation

See TypeScript definitions for full API documentation.

## Status

**Development Status**: Basic implementation complete, full features in progress.
