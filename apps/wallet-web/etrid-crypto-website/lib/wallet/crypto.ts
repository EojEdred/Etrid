/**
 * ETRID Wallet Cryptographic Utilities
 * Handles BIP39 mnemonic generation, key derivation, and address generation
 */

import {
  mnemonicGenerate,
  mnemonicValidate,
  mnemonicToMiniSecret,
  blake2AsU8a,
  encodeAddress,
  decodeAddress,
  cryptoWaitReady,
  ed25519PairFromSeed,
} from '@polkadot/util-crypto';
import { u8aToHex, hexToU8a, stringToU8a, u8aConcat } from '@polkadot/util';
import { Keyring } from '@polkadot/keyring';
import type { KeypairType } from './types';

/**
 * Initialize crypto libraries
 * Must be called before using any crypto functions
 */
export async function initCrypto(): Promise<void> {
  await cryptoWaitReady();
}

/**
 * Generate a new BIP39 mnemonic phrase
 * @param wordCount - Number of words (12 or 24)
 * @returns Mnemonic phrase
 */
export function generateMnemonic(wordCount: 12 | 24 = 12): string {
  // 12 words = 128 bits, 24 words = 256 bits
  const strength = wordCount === 12 ? 12 : 24;
  return mnemonicGenerate(strength);
}

/**
 * Validate a BIP39 mnemonic phrase
 * @param mnemonic - Mnemonic phrase to validate
 * @returns true if valid
 */
export function validateMnemonic(mnemonic: string): boolean {
  return mnemonicValidate(mnemonic);
}

/**
 * Derive a keypair from mnemonic and derivation path
 * @param mnemonic - BIP39 mnemonic phrase
 * @param derivationPath - Derivation path (e.g., "//0", "//etrid//0")
 * @param keypairType - Type of keypair (sr25519 or ed25519)
 * @returns Keypair object with address and keys
 */
export function deriveKeypair(
  mnemonic: string,
  derivationPath: string = '',
  keypairType: KeypairType = 'sr25519',
  ss58Prefix: number = 42
): { address: string; publicKey: Uint8Array } {
  const keyring = new Keyring({ type: keypairType, ss58Format: ss58Prefix });

  // Create the keypair from mnemonic + derivation path
  const pair = keyring.addFromMnemonic(mnemonic, {}, keypairType);

  // If there's a derivation path, derive from it
  const derivedPair = derivationPath
    ? keyring.addFromUri(`${mnemonic}${derivationPath}`, {}, keypairType)
    : pair;

  return {
    address: derivedPair.address,
    publicKey: derivedPair.publicKey,
  };
}

/**
 * Generate SS58 address from public key
 * @param publicKey - Public key bytes
 * @param ss58Prefix - SS58 format prefix
 * @returns SS58 encoded address
 */
export function generateSS58Address(publicKey: Uint8Array, ss58Prefix: number = 42): string {
  return encodeAddress(publicKey, ss58Prefix);
}

/**
 * Decode SS58 address to public key
 * @param address - SS58 encoded address
 * @returns Public key bytes
 */
export function decodeSS58Address(address: string): Uint8Array {
  return decodeAddress(address);
}

/**
 * Encrypt data using a password
 * Uses AES-GCM via Web Crypto API
 * @param data - Data to encrypt
 * @param password - Encryption password
 * @returns Encrypted data as hex string
 */
export async function encrypt(data: string, password: string): Promise<string> {
  const encoder = new TextEncoder();
  const dataBuffer = encoder.encode(data);

  // Derive key from password
  const passwordBuffer = encoder.encode(password);
  const passwordHash = blake2AsU8a(passwordBuffer, 256);

  // Generate random IV
  const iv = crypto.getRandomValues(new Uint8Array(12));

  // Import key for AES-GCM
  // Create a proper ArrayBuffer from Uint8Array
  const keyBuffer = new Uint8Array(passwordHash).buffer as ArrayBuffer;
  const key = await crypto.subtle.importKey(
    'raw',
    keyBuffer,
    { name: 'AES-GCM' },
    false,
    ['encrypt']
  );

  // Encrypt
  const encryptedBuffer = await crypto.subtle.encrypt(
    { name: 'AES-GCM', iv },
    key,
    dataBuffer
  );

  // Combine IV + encrypted data
  const combined = u8aConcat(iv, new Uint8Array(encryptedBuffer));

  return u8aToHex(combined);
}

/**
 * Decrypt data using a password
 * @param encryptedHex - Encrypted data as hex string
 * @param password - Decryption password
 * @returns Decrypted data string
 */
export async function decrypt(encryptedHex: string, password: string): Promise<string> {
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

  // Derive key from password
  const passwordBuffer = encoder.encode(password);
  const passwordHash = blake2AsU8a(passwordBuffer, 256);

  // Parse encrypted data
  const combined = hexToU8a(encryptedHex);
  const iv = combined.slice(0, 12);
  const encryptedData = combined.slice(12);

  // Import key for AES-GCM
  // Create a proper ArrayBuffer from Uint8Array
  const keyBuffer = new Uint8Array(passwordHash).buffer as ArrayBuffer;
  const key = await crypto.subtle.importKey(
    'raw',
    keyBuffer,
    { name: 'AES-GCM' },
    false,
    ['decrypt']
  );

  // Decrypt
  try {
    const decryptedBuffer = await crypto.subtle.decrypt(
      { name: 'AES-GCM', iv },
      key,
      encryptedData
    );

    return decoder.decode(decryptedBuffer);
  } catch (error) {
    throw new Error('Failed to decrypt: invalid password or corrupted data');
  }
}

/**
 * Hash data using Blake2b
 * @param data - Data to hash
 * @param bitLength - Output bit length (default 256)
 * @returns Hash as hex string
 */
export function hash(data: string | Uint8Array, bitLength: 64 | 128 | 256 | 384 | 512 = 256): string {
  const input = typeof data === 'string' ? stringToU8a(data) : data;
  return u8aToHex(blake2AsU8a(input, bitLength));
}

/**
 * Generate a random seed
 * @param length - Seed length in bytes (default 32)
 * @returns Random seed as Uint8Array
 */
export function generateRandomSeed(length: number = 32): Uint8Array {
  return crypto.getRandomValues(new Uint8Array(length));
}

/**
 * Convert mnemonic to mini secret (seed)
 * @param mnemonic - BIP39 mnemonic
 * @returns Seed bytes
 */
export function mnemonicToSeed(mnemonic: string): Uint8Array {
  return mnemonicToMiniSecret(mnemonic);
}

/**
 * Create keypair from seed using Ed25519
 * @param seed - 32-byte seed
 * @returns Keypair with public and secret keys
 */
export function keypairFromSeed(seed: Uint8Array): { publicKey: Uint8Array; secretKey: Uint8Array } {
  return ed25519PairFromSeed(seed);
}

/**
 * Verify if an address is valid SS58 format
 * @param address - Address to verify
 * @param ss58Prefix - Expected SS58 prefix (optional)
 * @returns true if valid
 */
export function isValidSS58Address(address: string, ss58Prefix?: number): boolean {
  try {
    const decoded = decodeAddress(address);
    if (!decoded || decoded.length !== 32) {
      return false;
    }

    // If prefix is specified, verify it matches
    if (ss58Prefix !== undefined) {
      const encoded = encodeAddress(decoded, ss58Prefix);
      return encoded === address;
    }

    return true;
  } catch {
    return false;
  }
}
