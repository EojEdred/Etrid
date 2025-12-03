/**
 * ETRID Wallet Secure Storage
 * Handles encrypted storage of wallet data in localStorage
 */

import { encrypt, decrypt } from './crypto';
import type { StoredWallet, WalletAccount } from './types';

const WALLET_STORAGE_KEY = 'etrid_wallet';
const WALLET_VERSION = '1.0.0';

/**
 * Storage error class
 */
export class StorageError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'StorageError';
  }
}

/**
 * Check if wallet exists in storage
 */
export function hasStoredWallet(): boolean {
  if (typeof window === 'undefined') return false;
  return localStorage.getItem(WALLET_STORAGE_KEY) !== null;
}

/**
 * Save wallet to encrypted storage
 * @param mnemonic - BIP39 mnemonic phrase
 * @param accounts - Array of wallet accounts
 * @param password - Encryption password
 */
export async function saveWallet(
  mnemonic: string,
  accounts: WalletAccount[],
  password: string
): Promise<void> {
  if (typeof window === 'undefined') {
    throw new StorageError('localStorage is not available');
  }

  const wallet: StoredWallet = {
    mnemonic,
    accounts: accounts.map((acc) => ({
      ...acc,
      // Convert Uint8Array to hex string for storage
      publicKey: Array.from(acc.publicKey) as any,
    })),
    createdAt: Date.now(),
    version: WALLET_VERSION,
  };

  try {
    // Encrypt the entire wallet object
    const walletJson = JSON.stringify(wallet);
    const encryptedData = await encrypt(walletJson, password);

    // Store encrypted data
    localStorage.setItem(WALLET_STORAGE_KEY, encryptedData);
  } catch (error) {
    throw new StorageError(`Failed to save wallet: ${error instanceof Error ? error.message : 'Unknown error'}`);
  }
}

/**
 * Load wallet from encrypted storage
 * @param password - Decryption password
 * @returns Stored wallet data
 */
export async function loadWallet(password: string): Promise<StoredWallet> {
  if (typeof window === 'undefined') {
    throw new StorageError('localStorage is not available');
  }

  const encryptedData = localStorage.getItem(WALLET_STORAGE_KEY);
  if (!encryptedData) {
    throw new StorageError('No wallet found in storage');
  }

  try {
    // Decrypt the wallet data
    const walletJson = await decrypt(encryptedData, password);
    const wallet: StoredWallet = JSON.parse(walletJson);

    // Convert public keys back to Uint8Array
    wallet.accounts = wallet.accounts.map((acc) => ({
      ...acc,
      publicKey: new Uint8Array(acc.publicKey as any),
    }));

    // Verify wallet version
    if (wallet.version !== WALLET_VERSION) {
      console.warn(`Wallet version mismatch: ${wallet.version} vs ${WALLET_VERSION}`);
    }

    return wallet;
  } catch (error) {
    if (error instanceof StorageError) {
      throw error;
    }
    throw new StorageError(`Failed to load wallet: ${error instanceof Error ? error.message : 'Unknown error'}`);
  }
}

/**
 * Update wallet accounts in storage
 * @param accounts - Updated array of accounts
 * @param password - Encryption password
 */
export async function updateWalletAccounts(
  accounts: WalletAccount[],
  password: string
): Promise<void> {
  const wallet = await loadWallet(password);
  await saveWallet(wallet.mnemonic, accounts, password);
}

/**
 * Delete wallet from storage
 */
export function deleteWallet(): void {
  if (typeof window === 'undefined') {
    throw new StorageError('localStorage is not available');
  }

  localStorage.removeItem(WALLET_STORAGE_KEY);
}

/**
 * Export wallet as encrypted JSON
 * @param password - Current password
 * @returns Encrypted wallet JSON string
 */
export async function exportWallet(password: string): Promise<string> {
  const wallet = await loadWallet(password);

  return JSON.stringify(
    {
      version: wallet.version,
      createdAt: wallet.createdAt,
      encrypted: localStorage.getItem(WALLET_STORAGE_KEY),
    },
    null,
    2
  );
}

/**
 * Import wallet from encrypted JSON
 * @param walletJson - Encrypted wallet JSON string
 * @param password - Password to verify decryption
 */
export async function importWallet(walletJson: string, password: string): Promise<void> {
  if (typeof window === 'undefined') {
    throw new StorageError('localStorage is not available');
  }

  try {
    const data = JSON.parse(walletJson);

    if (!data.encrypted) {
      throw new StorageError('Invalid wallet backup format');
    }

    // Verify password by attempting to decrypt
    await decrypt(data.encrypted, password);

    // If decryption succeeds, store the wallet
    localStorage.setItem(WALLET_STORAGE_KEY, data.encrypted);
  } catch (error) {
    if (error instanceof SyntaxError) {
      throw new StorageError('Invalid JSON format');
    }
    throw new StorageError(`Failed to import wallet: ${error instanceof Error ? error.message : 'Unknown error'}`);
  }
}

/**
 * Clear all wallet-related data from storage
 */
export function clearAllWalletData(): void {
  if (typeof window === 'undefined') return;

  const keysToRemove = Object.keys(localStorage).filter((key) =>
    key.startsWith('etrid_')
  );

  keysToRemove.forEach((key) => localStorage.removeItem(key));
}

/**
 * Get storage size (approximate)
 */
export function getStorageSize(): { used: number; total: number; percentage: number } {
  if (typeof window === 'undefined') {
    return { used: 0, total: 0, percentage: 0 };
  }

  let used = 0;
  for (const key in localStorage) {
    if (localStorage.hasOwnProperty(key)) {
      used += localStorage[key].length + key.length;
    }
  }

  // Most browsers allow ~5-10MB, we'll assume 5MB
  const total = 5 * 1024 * 1024;
  const percentage = (used / total) * 100;

  return { used, total, percentage };
}

/**
 * Check if storage is available
 */
export function isStorageAvailable(): boolean {
  if (typeof window === 'undefined') return false;

  try {
    const test = '__storage_test__';
    localStorage.setItem(test, test);
    localStorage.removeItem(test);
    return true;
  } catch {
    return false;
  }
}
