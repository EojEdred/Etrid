/**
 * ETRID Wallet Usage Examples
 * This file demonstrates common wallet operations
 */

import { getWalletService, WalletError } from './service';
import type { WalletAccount, WalletBalance } from './types';

/**
 * Example 1: Create a new wallet
 */
export async function exampleCreateWallet() {
  const wallet = getWalletService();

  try {
    // Initialize wallet service
    await wallet.initialize();

    // Create new wallet with 12-word mnemonic
    const account = await wallet.createWallet({
      mnemonicWordCount: 12,
      password: 'MySecurePassword123!',
      accountName: 'Main Account',
      keypairType: 'sr25519',
    });

    console.log('Wallet created successfully!');
    console.log('Address:', account.address);
    console.log('Derivation Path:', account.derivationPath);

    // Get the mnemonic (only show once!)
    const mnemonic = await wallet.exportMnemonic('MySecurePassword123!');
    console.log('IMPORTANT: Save this mnemonic phrase securely:');
    console.log(mnemonic);

    return account;
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Failed to create wallet:', error.message);
    }
    throw error;
  }
}

/**
 * Example 2: Import existing wallet
 */
export async function exampleImportWallet(mnemonic: string) {
  const wallet = getWalletService();

  try {
    await wallet.initialize();

    const account = await wallet.importWallet({
      mnemonic,
      password: 'MySecurePassword123!',
      accountName: 'Imported Account',
      keypairType: 'sr25519',
    });

    console.log('Wallet imported successfully!');
    console.log('Address:', account.address);

    return account;
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Failed to import wallet:', error.message);
    }
    throw error;
  }
}

/**
 * Example 3: Unlock wallet and check balance
 */
export async function exampleUnlockAndCheckBalance() {
  const wallet = getWalletService();

  try {
    await wallet.initialize();

    // Check if wallet exists
    if (!wallet.hasWallet()) {
      console.log('No wallet found. Create one first!');
      return;
    }

    // Unlock wallet
    const accounts = await wallet.unlockWallet('MySecurePassword123!');
    console.log(`Unlocked ${accounts.length} account(s)`);

    // Connect to chain
    await wallet.connect();

    // Get chain info
    const chainInfo = await wallet.getChainInfo();
    console.log('Connected to:', chainInfo.name);
    console.log('Token:', chainInfo.tokenSymbol);

    // Check balance for each account
    for (const account of accounts) {
      const balance = await wallet.getBalance(account.address);
      const formatted = wallet.formatBalance(balance.free, chainInfo.tokenDecimals, chainInfo.tokenSymbol);

      console.log(`\n${account.name} (${account.address})`);
      console.log('Balance:', formatted);
      console.log('Free:', balance.free);
      console.log('Reserved:', balance.reserved);
    }

    return accounts;
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Error:', error.message);
    }
    throw error;
  }
}

/**
 * Example 4: Create multiple accounts from same mnemonic
 */
export async function exampleDeriveMultipleAccounts() {
  const wallet = getWalletService();

  try {
    await wallet.initialize();
    await wallet.unlockWallet('MySecurePassword123!');

    // Derive 3 additional accounts
    const accountNames = ['Savings', 'Trading', 'Staking'];
    const derivedAccounts: WalletAccount[] = [];

    for (let i = 0; i < accountNames.length; i++) {
      const account = await wallet.deriveAccount(
        {
          derivationPath: `//${i + 1}`,
          accountName: accountNames[i],
          keypairType: 'sr25519',
        },
        'MySecurePassword123!'
      );

      console.log(`Created ${account.name}: ${account.address}`);
      derivedAccounts.push(account);
    }

    // Get all accounts
    const allAccounts = wallet.getAccounts();
    console.log(`\nTotal accounts: ${allAccounts.length}`);

    return derivedAccounts;
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Error:', error.message);
    }
    throw error;
  }
}

/**
 * Example 5: Send a transaction
 */
export async function exampleSendTransaction(toAddress: string, amount: string) {
  const wallet = getWalletService();

  try {
    await wallet.initialize();
    await wallet.unlockWallet('MySecurePassword123!');
    await wallet.connect();

    const accounts = wallet.getAccounts();
    if (accounts.length === 0) {
      console.log('No accounts found');
      return;
    }

    const fromAccount = accounts[0];

    // Check balance first
    const balance = await wallet.getBalance(fromAccount.address);
    console.log('Current balance:', wallet.formatBalance(balance.free));

    // Estimate fee
    const fee = await wallet.estimateFee(fromAccount.address, toAddress, amount);
    console.log('Estimated fee:', wallet.formatBalance(fee));

    // Check if sufficient balance
    const total = BigInt(amount) + BigInt(fee);
    if (total > BigInt(balance.free)) {
      console.log('Insufficient balance!');
      return;
    }

    // Send transaction
    console.log('Sending transaction...');
    const result = await wallet.sendTransaction(
      fromAccount.address,
      toAddress,
      amount,
      'MySecurePassword123!'
    );

    console.log('Transaction successful!');
    console.log('Hash:', result.hash);

    return result;
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Transaction failed:', error.message);
    }
    throw error;
  }
}

/**
 * Example 6: Subscribe to balance changes
 */
export async function exampleSubscribeToBalance(address: string) {
  const wallet = getWalletService();

  try {
    await wallet.initialize();
    await wallet.connect();

    console.log('Subscribing to balance changes for:', address);

    const unsubscribe = await wallet.subscribeBalance(address, (balance: WalletBalance) => {
      const formatted = wallet.formatBalance(balance.free);
      console.log('Balance updated:', formatted);
      console.log('Free:', balance.free);
      console.log('Reserved:', balance.reserved);
      console.log('Total:', balance.total);
    });

    // Unsubscribe after 60 seconds
    setTimeout(() => {
      console.log('Unsubscribing from balance updates');
      unsubscribe();
    }, 60000);

    return unsubscribe;
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Error:', error.message);
    }
    throw error;
  }
}

/**
 * Example 7: Export and import wallet backup
 */
export async function exampleBackupAndRestore() {
  const wallet = getWalletService();

  try {
    await wallet.initialize();

    // Export wallet
    const backup = await wallet.exportWallet('MySecurePassword123!');
    console.log('Wallet backup created:');
    console.log(backup);

    // Save backup to file (in real app, prompt user to download)
    // const blob = new Blob([backup], { type: 'application/json' });
    // const url = URL.createObjectURL(blob);
    // ... download logic

    // Import backup (simulating restore on different device)
    await wallet.importWalletBackup(backup, 'MySecurePassword123!');
    console.log('Wallet restored successfully!');

    return backup;
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Error:', error.message);
    }
    throw error;
  }
}

/**
 * Example 8: Complete wallet lifecycle
 */
export async function exampleCompleteFlow() {
  const wallet = getWalletService();

  try {
    console.log('=== ETRID Wallet Complete Flow ===\n');

    // 1. Initialize
    console.log('1. Initializing wallet...');
    await wallet.initialize();

    // 2. Create or unlock wallet
    let account: WalletAccount;
    if (!wallet.hasWallet()) {
      console.log('2. Creating new wallet...');
      account = await wallet.createWallet({
        mnemonicWordCount: 12,
        password: 'MySecurePassword123!',
        accountName: 'Main Account',
      });

      const mnemonic = await wallet.exportMnemonic('MySecurePassword123!');
      console.log('   Mnemonic (SAVE THIS!):', mnemonic);
    } else {
      console.log('2. Unlocking existing wallet...');
      const accounts = await wallet.unlockWallet('MySecurePassword123!');
      account = accounts[0];
    }

    console.log('   Address:', account.address);

    // 3. Connect to chain
    console.log('\n3. Connecting to ETRID chain...');
    await wallet.connect();
    const chainInfo = await wallet.getChainInfo();
    console.log('   Chain:', chainInfo.name);
    console.log('   Genesis:', chainInfo.genesisHash.slice(0, 20) + '...');

    // 4. Check balance
    console.log('\n4. Checking balance...');
    const balance = await wallet.getBalance(account.address);
    console.log('   Balance:', wallet.formatBalance(balance.free));

    // 5. Get current block
    console.log('\n5. Getting blockchain info...');
    const blockNumber = await wallet.getBlockNumber();
    console.log('   Current block:', blockNumber);

    // 6. Create backup
    console.log('\n6. Creating backup...');
    const backup = await wallet.exportWallet('MySecurePassword123!');
    console.log('   Backup size:', backup.length, 'bytes');

    // 7. Cleanup
    console.log('\n7. Cleaning up...');
    wallet.lockWallet();
    await wallet.disconnect();

    console.log('\n=== Flow completed successfully! ===');

    return {
      account,
      chainInfo,
      balance,
      blockNumber,
    };
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Flow failed:', error.message);
    }
    throw error;
  }
}

/**
 * Example 9: Error handling patterns
 */
export async function exampleErrorHandling() {
  const wallet = getWalletService();

  // Pattern 1: Try-catch with specific error types
  try {
    await wallet.unlockWallet('wrong-password');
  } catch (error) {
    if (error instanceof WalletError) {
      console.error('Wallet error:', error.message);
      // Handle wallet-specific errors
    } else {
      console.error('Unexpected error:', error);
      // Handle other errors
    }
  }

  // Pattern 2: Checking conditions before operations
  if (!wallet.hasWallet()) {
    console.log('No wallet exists, prompting user to create one...');
    return;
  }

  // Pattern 3: Validating inputs
  const address = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
  const { isValidSS58Address } = await import('./crypto');

  if (!isValidSS58Address(address)) {
    console.error('Invalid address format');
    return;
  }

  console.log('All error handling patterns demonstrated');
}

// Export all examples
export default {
  exampleCreateWallet,
  exampleImportWallet,
  exampleUnlockAndCheckBalance,
  exampleDeriveMultipleAccounts,
  exampleSendTransaction,
  exampleSubscribeToBalance,
  exampleBackupAndRestore,
  exampleCompleteFlow,
  exampleErrorHandling,
};
