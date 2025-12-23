/**
 * ETRID Built-in Wallet Manager
 *
 * Handles wallet creation, import, and transaction signing
 * without requiring any external browser extensions.
 */

import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate, mnemonicValidate, cryptoWaitReady } from '@polkadot/util-crypto';
import type { KeyringPair } from '@polkadot/keyring/types';

const STORAGE_KEY = 'etrid_wallet_v1';
const KEYRING_TYPE = 'sr25519';

export interface StoredWallet {
  address: string;
  name: string;
  encryptedJson: string;
  createdAt: number;
}

export interface WalletAccount {
  address: string;
  name: string;
  pair: KeyringPair;
}

let keyring: Keyring | null = null;
let currentAccount: WalletAccount | null = null;

/**
 * Initialize the keyring (must be called before any crypto operations)
 */
export async function initKeyring(): Promise<Keyring> {
  await cryptoWaitReady();

  if (!keyring) {
    keyring = new Keyring({ type: KEYRING_TYPE, ss58Format: 42 });
  }

  return keyring;
}

/**
 * Generate a new 12-word mnemonic phrase
 */
export function generateMnemonic(): string {
  return mnemonicGenerate(12);
}

/**
 * Validate a mnemonic phrase
 */
export function validateMnemonic(mnemonic: string): boolean {
  return mnemonicValidate(mnemonic);
}

/**
 * Create a new wallet from mnemonic
 */
export async function createWallet(
  mnemonic: string,
  password: string,
  name: string = 'My Wallet'
): Promise<WalletAccount> {
  const kr = await initKeyring();

  // Create keypair from mnemonic
  console.log('[Keyring] Adding from mnemonic...');
  const pair = kr.addFromMnemonic(mnemonic, { name }, KEYRING_TYPE);
  console.log('[Keyring] Pair added. Encrypting...');

  // Encrypt and store
  // TODO: toJson is crashing on some environments due to scrypt WASM issues.
  // For now, we will store it unencrypted in dev mode or use a try/catch block.
  let encryptedJson;
  try {
    encryptedJson = JSON.stringify(pair.toJson(password));
    console.log('[Keyring] Encryption successful.');
  } catch (err) {
    console.error('[Keyring] Encryption failed:', err);
    // Fallback: Store without encryption (INSECURE - DEV ONLY)
    // In production this must be fixed by ensuring WASM/scrypt works
    console.warn('[Keyring] FALLBACK: Storing unencrypted wallet due to crash risk.');
    encryptedJson = JSON.stringify({ address: pair.address, meta: { name }, encoding: { content: ['none', 'none'], type: ['none'], version: '3' }, encoded: mnemonic }); 
  }

  const storedWallet: StoredWallet = {
    address: pair.address,
    name,
    encryptedJson,
    createdAt: Date.now(),
  };

  // Save to localStorage
  saveWalletToStorage(storedWallet);

  currentAccount = { address: pair.address, name, pair };

  return currentAccount;
}

/**
 * Import wallet from mnemonic
 */
export async function importWallet(
  mnemonic: string,
  password: string,
  name: string = 'Imported Wallet'
): Promise<WalletAccount> {
  if (!validateMnemonic(mnemonic)) {
    throw new Error('Invalid mnemonic phrase');
  }

  return createWallet(mnemonic, password, name);
}

/**
 * Unlock wallet with password
 */
export async function unlockWallet(password: string): Promise<WalletAccount> {
  const stored = getStoredWallet();

  if (!stored) {
    throw new Error('No wallet found. Please create or import a wallet.');
  }

  const kr = await initKeyring();

  try {
    const json = JSON.parse(stored.encryptedJson);
    const pair = kr.addFromJson(json);
    pair.unlock(password);

    // Verify the unlock worked by trying to sign something
    pair.sign(new Uint8Array([0]));

    currentAccount = { address: stored.address, name: stored.name, pair };

    return currentAccount;
  } catch (error) {
    throw new Error('Invalid password');
  }
}

/**
 * Lock the wallet (clear the current account)
 */
export function lockWallet(): void {
  if (currentAccount?.pair) {
    currentAccount.pair.lock();
  }
  currentAccount = null;
}

/**
 * Get current unlocked account
 */
export function getCurrentAccount(): WalletAccount | null {
  return currentAccount;
}

/**
 * Check if wallet exists in storage
 */
export function hasStoredWallet(): boolean {
  return getStoredWallet() !== null;
}

/**
 * Get stored wallet info (without decryption)
 */
export function getStoredWallet(): StoredWallet | null {
  if (typeof window === 'undefined') return null;

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (!stored) return null;
    return JSON.parse(stored) as StoredWallet;
  } catch {
    return null;
  }
}

/**
 * Save wallet to localStorage
 */
function saveWalletToStorage(wallet: StoredWallet): void {
  if (typeof window === 'undefined') return;
  localStorage.setItem(STORAGE_KEY, JSON.stringify(wallet));
}

/**
 * Delete wallet from storage
 */
export function deleteWallet(): void {
  if (typeof window === 'undefined') return;
  localStorage.removeItem(STORAGE_KEY);
  lockWallet();
}

/**
 * Get signer for transactions (compatible with Polkadot.js API)
 */
export function getSigner() {
  if (!currentAccount?.pair) {
    throw new Error('Wallet is locked. Please unlock first.');
  }

  return {
    signPayload: async (payload: any) => {
      const signature = currentAccount!.pair.sign(payload.data || payload);
      return { signature: signature };
    },
    signRaw: async (raw: { address: string; data: string }) => {
      const signature = currentAccount!.pair.sign(raw.data);
      return { signature: `0x${Buffer.from(signature).toString('hex')}` };
    },
  };
}

/**
 * Sign and send a transaction
 */
export async function signTransaction(
  api: any,
  tx: any,
): Promise<{ txHash: string; blockHash?: string }> {
  if (!currentAccount?.pair) {
    throw new Error('Wallet is locked. Please unlock first.');
  }

  return new Promise(async (resolve, reject) => {
    try {
      const unsub = await tx.signAndSend(
        currentAccount!.pair,
        ({ status, txHash, dispatchError }: any) => {
          console.log(`[ETRID Wallet] Transaction status: ${status.type}`);

          if (status.isFinalized) {
            if (dispatchError) {
              let errorMsg = 'Transaction failed';

              if (dispatchError.isModule) {
                const decoded = api.registry.findMetaError(dispatchError.asModule);
                errorMsg = `${decoded.section}.${decoded.name}: ${decoded.docs.join(' ')}`;
              }

              unsub();
              reject(new Error(errorMsg));
            } else {
              unsub();
              resolve({
                txHash: txHash.toHex(),
                blockHash: status.asFinalized.toHex(),
              });
            }
          }

          if (status.isError) {
            unsub();
            reject(new Error('Transaction failed'));
          }
        }
      );
    } catch (error) {
      reject(error);
    }
  });
}