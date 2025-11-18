import * as SecureStore from 'expo-secure-store';
import { Keyring } from '@polkadot/keyring';
import { KeyringPair } from '@polkadot/keyring/types';
import { mnemonicGenerate, mnemonicValidate, mnemonicToMiniSecret } from '@polkadot/util-crypto';
import { u8aToHex } from '@polkadot/util';

const KEYCHAIN_KEY = 'etrid_wallet_keypair';
const MNEMONIC_KEY = 'etrid_wallet_mnemonic';

/**
 * Service for secure keypair storage and management using Expo Secure Store
 * NEVER stores private keys in AsyncStorage - only in encrypted keychain
 */
class KeychainService {
  private keyring: Keyring;

  constructor() {
    this.keyring = new Keyring({ type: 'sr25519' });
  }

  /**
   * Generate new mnemonic phrase (12 words)
   */
  public generateMnemonic(): string {
    return mnemonicGenerate(12);
  }

  /**
   * Validate mnemonic phrase
   */
  public validateMnemonic(mnemonic: string): boolean {
    return mnemonicValidate(mnemonic);
  }

  /**
   * Create new wallet from generated mnemonic
   */
  public async createWallet(mnemonic?: string): Promise<{ address: string; mnemonic: string }> {
    try {
      // Generate mnemonic if not provided
      const phrase = mnemonic || this.generateMnemonic();

      if (!this.validateMnemonic(phrase)) {
        throw new Error('Invalid mnemonic phrase');
      }

      // Create keypair from mnemonic
      const keypair = this.keyring.addFromMnemonic(phrase);
      const address = keypair.address;

      // Store encrypted in secure store
      await this.storeKeypair(keypair, phrase);

      return { address, mnemonic: phrase };
    } catch (error) {
      console.error('Error creating wallet:', error);
      throw new Error(`Failed to create wallet: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Import wallet from mnemonic phrase
   */
  public async importWallet(mnemonic: string): Promise<string> {
    try {
      if (!this.validateMnemonic(mnemonic)) {
        throw new Error('Invalid mnemonic phrase');
      }

      // Create keypair from mnemonic
      const keypair = this.keyring.addFromMnemonic(mnemonic.trim());
      const address = keypair.address;

      // Store encrypted in secure store
      await this.storeKeypair(keypair, mnemonic);

      return address;
    } catch (error) {
      console.error('Error importing wallet:', error);
      throw new Error(`Failed to import wallet: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  /**
   * Store keypair in secure store
   */
  private async storeKeypair(keypair: KeyringPair, mnemonic: string): Promise<void> {
    try {
      // Store mnemonic (encrypted by SecureStore)
      await SecureStore.setItemAsync(MNEMONIC_KEY, mnemonic);

      // Store keypair JSON (encrypted by SecureStore)
      const keypairJson = JSON.stringify(keypair.toJson());
      await SecureStore.setItemAsync(KEYCHAIN_KEY, keypairJson);

      console.log('Keypair stored securely');
    } catch (error) {
      console.error('Error storing keypair:', error);
      throw new Error('Failed to store keypair securely');
    }
  }

  /**
   * Load keypair from secure store
   */
  public async loadKeypair(): Promise<KeyringPair | null> {
    try {
      const mnemonic = await SecureStore.getItemAsync(MNEMONIC_KEY);

      if (!mnemonic) {
        return null;
      }

      // Recreate keypair from mnemonic
      const keypair = this.keyring.addFromMnemonic(mnemonic);
      return keypair;
    } catch (error) {
      console.error('Error loading keypair:', error);
      return null;
    }
  }

  /**
   * Get stored mnemonic
   */
  public async getMnemonic(): Promise<string | null> {
    try {
      return await SecureStore.getItemAsync(MNEMONIC_KEY);
    } catch (error) {
      console.error('Error getting mnemonic:', error);
      return null;
    }
  }

  /**
   * Check if wallet exists
   */
  public async hasWallet(): Promise<boolean> {
    try {
      const mnemonic = await SecureStore.getItemAsync(MNEMONIC_KEY);
      return mnemonic !== null;
    } catch (error) {
      console.error('Error checking wallet:', error);
      return false;
    }
  }

  /**
   * Delete wallet (use with caution)
   */
  public async deleteWallet(): Promise<void> {
    try {
      await SecureStore.deleteItemAsync(KEYCHAIN_KEY);
      await SecureStore.deleteItemAsync(MNEMONIC_KEY);
      console.log('Wallet deleted');
    } catch (error) {
      console.error('Error deleting wallet:', error);
      throw new Error('Failed to delete wallet');
    }
  }

  /**
   * Get address from stored keypair
   */
  public async getAddress(): Promise<string | null> {
    try {
      const keypair = await this.loadKeypair();
      return keypair ? keypair.address : null;
    } catch (error) {
      console.error('Error getting address:', error);
      return null;
    }
  }

  /**
   * Sign message with keypair
   */
  public async signMessage(message: string): Promise<string | null> {
    try {
      const keypair = await this.loadKeypair();
      if (!keypair) {
        throw new Error('No keypair found');
      }

      const signature = keypair.sign(message);
      return u8aToHex(signature);
    } catch (error) {
      console.error('Error signing message:', error);
      return null;
    }
  }

  /**
   * Export private key (use with extreme caution, for backup only)
   */
  public async exportPrivateKey(): Promise<string | null> {
    try {
      const mnemonic = await SecureStore.getItemAsync(MNEMONIC_KEY);
      return mnemonic;
    } catch (error) {
      console.error('Error exporting private key:', error);
      return null;
    }
  }
}

export default new KeychainService();
