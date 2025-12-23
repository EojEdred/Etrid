# ETRID Wallet Library

Complete wallet solution for ETRID blockchain with BIP39 mnemonic support, multi-account derivation, and secure encrypted storage.

## Features

- **BIP39 Mnemonic Generation**: Create 12 or 24-word recovery phrases
- **Multi-Account Support**: Derive unlimited accounts from a single mnemonic
- **Sr25519 & Ed25519**: Support for both Substrate keypair types
- **SS58 Address Encoding**: Proper address formatting with configurable prefix
- **Secure Storage**: AES-GCM encrypted localStorage
- **Chain Connection**: Connect to ETRID via Polkadot.js API
- **Balance Queries**: Real-time balance tracking and subscriptions
- **Transaction Signing**: Send native ETR transfers
- **Wallet Import/Export**: Backup and restore functionality

## Installation

The required dependencies are already installed in the project:
- `@polkadot/api`
- `@polkadot/util-crypto`
- `@polkadot/keyring`

## Quick Start

```typescript
import { getWalletService } from '@/lib/wallet';

// Get wallet service instance
const wallet = getWalletService();

// Initialize (required before any operations)
await wallet.initialize();

// Create a new wallet
const account = await wallet.createWallet({
  mnemonicWordCount: 12,
  password: 'your-secure-password',
  accountName: 'My ETRID Account',
  keypairType: 'sr25519'
});

console.log('Address:', account.address);

// Check balance
const balance = await wallet.getBalance(account.address);
console.log('Balance:', wallet.formatBalance(balance.free));
```

## API Reference

### WalletService

#### Initialization

```typescript
// Get singleton instance
const wallet = getWalletService();

// Initialize crypto libraries
await wallet.initialize();

// Connect to ETRID chain
await wallet.connect();
```

#### Create Wallet

```typescript
const account = await wallet.createWallet({
  mnemonicWordCount: 12 | 24,  // Optional, default: 12
  password: string,              // Required for encryption
  accountName?: string,          // Optional, default: "Account 1"
  keypairType?: 'sr25519' | 'ed25519'  // Optional, default: 'sr25519'
});
```

#### Import Wallet

```typescript
const account = await wallet.importWallet({
  mnemonic: string,              // BIP39 mnemonic phrase
  password: string,              // Password to encrypt wallet
  accountName?: string,
  keypairType?: 'sr25519' | 'ed25519'
});
```

#### Unlock/Lock Wallet

```typescript
// Unlock wallet from storage
const accounts = await wallet.unlockWallet('password');

// Lock wallet (clear sensitive data from memory)
wallet.lockWallet();

// Check if wallet is unlocked
const isUnlocked = wallet.isUnlocked();

// Check if wallet exists in storage
const hasWallet = wallet.hasWallet();
```

#### Manage Accounts

```typescript
// Derive new account from same mnemonic
const newAccount = await wallet.deriveAccount({
  derivationPath: '//1',         // Must be unique
  accountName: 'Account 2',
  keypairType: 'sr25519'
}, 'password');

// Get all accounts
const accounts = wallet.getAccounts();

// Get specific account
const account = wallet.getAccount(address);

// Rename account
await wallet.renameAccount(address, 'New Name', 'password');
```

#### Query Balances

```typescript
// Get account balance
const balance = await wallet.getBalance(address);
console.log('Free:', balance.free);
console.log('Reserved:', balance.reserved);
console.log('Frozen:', balance.frozen);
console.log('Total:', balance.total);

// Format balance for display
const formatted = wallet.formatBalance(balance.free);
console.log(formatted); // e.g., "1.234 KETR"

// Subscribe to balance changes
const unsubscribe = await wallet.subscribeBalance(address, (balance) => {
  console.log('Balance updated:', balance);
});

// Stop subscription
unsubscribe();
```

#### Send Transactions

```typescript
// Send ETR transfer
const result = await wallet.sendTransaction(
  fromAddress,
  toAddress,
  '1000000000000000000',  // Amount in smallest unit (1 ETR with 18 decimals)
  'password'
);

console.log('Transaction hash:', result.hash);

// Estimate transaction fee
const fee = await wallet.estimateFee(fromAddress, toAddress, amount);
console.log('Estimated fee:', wallet.formatBalance(fee));
```

#### Chain Information

```typescript
// Get chain info
const chainInfo = await wallet.getChainInfo();
console.log('Chain:', chainInfo.name);
console.log('Genesis:', chainInfo.genesisHash);
console.log('Token:', chainInfo.tokenSymbol);
console.log('Decimals:', chainInfo.tokenDecimals);

// Get current block number
const blockNumber = await wallet.getBlockNumber();
```

#### Export/Import

```typescript
// Export mnemonic (requires password)
const mnemonic = await wallet.exportMnemonic('password');

// Export encrypted backup
const backup = await wallet.exportWallet('password');

// Import from backup
await wallet.importWalletBackup(backup, 'password');

// Delete wallet permanently
await wallet.deleteWallet();
```

### Crypto Utilities

```typescript
import {
  generateMnemonic,
  validateMnemonic,
  deriveKeypair,
  isValidSS58Address,
  encrypt,
  decrypt
} from '@/lib/wallet';

// Generate mnemonic
const mnemonic = generateMnemonic(12);

// Validate mnemonic
const isValid = validateMnemonic(mnemonic);

// Derive keypair
const keypair = deriveKeypair(mnemonic, '//0', 'sr25519', 42);

// Validate SS58 address
const isValidAddress = isValidSS58Address(address);

// Encrypt/decrypt data
const encrypted = await encrypt('sensitive data', 'password');
const decrypted = await decrypt(encrypted, 'password');
```

### Storage Utilities

```typescript
import {
  hasStoredWallet,
  isStorageAvailable,
  getStorageSize,
  clearAllWalletData
} from '@/lib/wallet';

// Check if wallet exists
const hasWallet = hasStoredWallet();

// Check storage availability
const available = isStorageAvailable();

// Get storage usage
const { used, total, percentage } = getStorageSize();

// Clear all wallet data
clearAllWalletData();
```

## Security Best Practices

1. **Never expose mnemonics**: Mnemonics are encrypted in storage and should never be displayed in production
2. **Strong passwords**: Enforce strong password requirements
3. **Memory cleanup**: Always call `lockWallet()` when done to clear sensitive data
4. **HTTPS only**: Only use wallet in secure contexts (HTTPS)
5. **Validate inputs**: Always validate addresses and amounts before transactions
6. **Backup reminders**: Prompt users to backup their mnemonic after creation

## Derivation Paths

The wallet supports custom derivation paths for multi-account:

```typescript
// Standard accounts
'//0', '//1', '//2', ...

// Named paths
'//alice', '//bob', '//charlie'

// Hierarchical paths
'//etrid//0', '//etrid//1'
```

## RPC Endpoints

The wallet automatically connects to ETRID chain with fallback:

- Primary: `wss://rpc.etrid.org`
- Fallback: `ws://157.173.200.80:9944`

## Error Handling

All wallet operations may throw `WalletError`:

```typescript
import { WalletError } from '@/lib/wallet';

try {
  await wallet.createWallet({ ... });
} catch (error) {
  if (error instanceof WalletError) {
    console.error('Wallet error:', error.message);
  }
}
```

## TypeScript Types

```typescript
interface WalletAccount {
  address: string;
  publicKey: Uint8Array;
  name: string;
  derivationPath: string;
  keypairType: 'sr25519' | 'ed25519';
  balance?: string;
}

interface WalletBalance {
  free: string;
  reserved: string;
  frozen: string;
  total: string;
}

interface ChainInfo {
  name: string;
  genesisHash: string;
  ss58Prefix: number;
  tokenSymbol: string;
  tokenDecimals: number;
}
```

## Example: Complete Wallet Flow

```typescript
import { getWalletService } from '@/lib/wallet';

async function completeWalletFlow() {
  const wallet = getWalletService();

  // 1. Initialize
  await wallet.initialize();

  // 2. Create or import wallet
  let account;
  if (!wallet.hasWallet()) {
    account = await wallet.createWallet({
      mnemonicWordCount: 12,
      password: 'SecurePassword123!',
      accountName: 'Main Account'
    });
    console.log('New wallet created!');
  } else {
    const accounts = await wallet.unlockWallet('SecurePassword123!');
    account = accounts[0];
    console.log('Wallet unlocked!');
  }

  // 3. Get chain info
  const chainInfo = await wallet.getChainInfo();
  console.log('Connected to:', chainInfo.name);

  // 4. Check balance
  const balance = await wallet.getBalance(account.address);
  console.log('Balance:', wallet.formatBalance(balance.free));

  // 5. Derive additional account
  const account2 = await wallet.deriveAccount({
    derivationPath: '//1',
    accountName: 'Savings Account'
  }, 'SecurePassword123!');

  // 6. Send transaction
  if (BigInt(balance.free) > BigInt('1000000000000000000')) {
    const result = await wallet.sendTransaction(
      account.address,
      account2.address,
      '1000000000000000000', // 1 ETR
      'SecurePassword123!'
    );
    console.log('Transfer successful:', result.hash);
  }

  // 7. Export backup
  const backup = await wallet.exportWallet('SecurePassword123!');
  console.log('Backup created (store securely!)');

  // 8. Clean up
  wallet.lockWallet();
  await wallet.disconnect();
}
```

## Testing

The wallet service can be tested in isolation:

```typescript
import { resetWalletService } from '@/lib/wallet';

// Reset singleton for testing
resetWalletService();

// Create fresh instance
const wallet = getWalletService();
```

## License

Part of the ETRID project.
