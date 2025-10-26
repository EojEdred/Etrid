# Ëtrid SDK - Smart Contract Examples

This directory contains complete examples for interacting with Ëtrid smart contracts using the JavaScript/TypeScript SDK.

---

## 📚 Examples Overview

### 1. **ERC20 Token** (`erc20-token.ts`)
- Deploy ERC20 token contract
- Transfer tokens
- Approve spending
- Check balances
- Mint and burn tokens

### 2. **NFT (ERC721)** (`nft-erc721.ts`)
- Deploy NFT collection
- Mint NFTs with metadata
- Transfer NFTs
- Approve transfers
- Query ownership

### 3. **Simple DAO** (`simple-dao.ts`)
- Create DAO
- Add members
- Create proposals
- Vote on proposals
- Execute proposals

### 4. **Escrow** (`escrow.ts`)
- Create escrow agreement
- Deposit funds
- Confirm delivery
- Handle disputes
- Request refunds

### 5. **Complete DApp** (`complete-dapp.ts`)
- Full application example
- Multiple contract interactions
- Error handling
- Event listening

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install dependencies
npm install

# Or with yarn
yarn install
```

### Running Examples

```bash
# Run individual example
npx ts-node examples/erc20-token.ts

# Or compile and run
npm run build
node dist/examples/erc20-token.js
```

---

## 📖 Example Structure

Each example follows this pattern:

```typescript
import { EtridClient } from '../src';

async function main() {
    // 1. Connect to blockchain
    const client = new EtridClient('ws://localhost:9944');
    await client.connect();

    // 2. Setup account
    const alice = client.createAccount('//Alice');

    // 3. Interact with contract
    // ... example code ...

    // 4. Cleanup
    await client.disconnect();
}

main().catch(console.error);
```

---

## 🔑 Contract ABIs

ABIs for each contract are located in:
```
examples/abis/
├── erc20.json
├── nft.json
├── dao.json
└── escrow.json
```

To generate ABIs from your contracts:
```bash
cd /path/to/contract
cargo contract build --release
# ABI will be in target/ink/contract.json
```

---

##  Common Patterns

### Deploying a Contract

```typescript
const contract = await client.deployContract({
    abi: erc20Abi,
    wasm: erc20Wasm,
    constructor: 'new',
    args: [1000000, 'My Token', 'MTK', 18],
    gasLimit: 100000000000,
});

console.log('Contract deployed at:', contract.address);
```

### Calling Contract Methods

```typescript
// Read-only call (query)
const balance = await contract.query('balanceOf', [alice.address]);
console.log('Balance:', balance.output);

// State-changing call (transaction)
const tx = await contract.tx('transfer', [bob.address, 100], {
    gasLimit: 10000000000,
});

await tx.wait();
console.log('Transaction hash:', tx.hash);
```

### Listening to Events

```typescript
contract.on('Transfer', (event) => {
    console.log('Transfer event:', {
        from: event.from,
        to: event.to,
        value: event.value,
    });
});
```

---

## 🛠️ Advanced Usage

### Batch Transactions

```typescript
const txs = await client.batchTx([
    contract.tx('transfer', [bob.address, 100]),
    contract.tx('transfer', [charlie.address, 50]),
]);

await Promise.all(txs.map(tx => tx.wait()));
```

### Error Handling

```typescript
try {
    await contract.tx('transfer', [bob.address, 1000000]);
} catch (error) {
    if (error.message.includes('InsufficientBalance')) {
        console.error('Not enough tokens!');
    }
}
```

### Gas Estimation

```typescript
const gasEstimate = await contract.estimateGas('transfer', [bob.address, 100]);
console.log('Estimated gas:', gasEstimate);
```

---

## 📊 Example Outputs

### ERC20 Token Example
```
✅ Connected to Ëtrid node
✅ Deployed ERC20 at: 5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM
✅ Initial balance: 1000000
✅ Transferred 100 tokens
✅ New balance: 999900
```

### NFT Example
```
✅ Connected to Ëtrid node
✅ Deployed NFT collection: "Crypto Apes"
✅ Minted NFT #1 to Alice
✅ Token URI: ipfs://Qm.../1.json
✅ Transferred NFT #1 to Bob
```

---

## 🔗 Resources

- **Ëtrid Docs**: https://docs.etrid.com
- **SDK Reference**: ../README.md
- **Contract Examples**: /contracts/etwasm-examples/
- **Discord**: https://discord.gg/etrid

---

## 💡 Tips

1. **Always estimate gas** before transactions
2. **Use dry-run** for testing without spending gas
3. **Listen to events** for real-time updates
4. **Handle errors** gracefully with try-catch
5. **Test on local node** before deploying to testnet

---

## 🐛 Troubleshooting

### Connection Issues
```typescript
// Use localhost for local node
const client = new EtridClient('ws://localhost:9944');

// Use testnet endpoint for Ember
const client = new EtridClient('wss://ember-rpc.etrid.io');
```

### Gas Limit Too Low
```typescript
// Increase gas limit
const tx = await contract.tx('transfer', [bob.address, 100], {
    gasLimit: 100000000000, // Higher limit
});
```

### Contract Not Found
```typescript
// Make sure contract is deployed
if (!contract.address) {
    throw new Error('Contract not deployed');
}
```

---

**Happy Coding! 🚀**
