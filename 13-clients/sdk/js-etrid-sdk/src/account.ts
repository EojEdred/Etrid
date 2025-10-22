/**
 * Account management and cryptographic operations
 */

import { Keyring } from '@polkadot/keyring';
import { mnemonicGenerate } from '@polkadot/util-crypto';

/**
 * Ëtrid account
 */
export class Account {
  private keyring: Keyring;
  private pair: any;

  private constructor(mnemonic: string) {
    this.keyring = new Keyring({ type: 'sr25519' });
    this.pair = this.keyring.addFromMnemonic(mnemonic);
  }

  /**
   * Create account from mnemonic phrase
   * @param mnemonic - 12 or 24 word mnemonic phrase
   */
  static fromMnemonic(mnemonic: string): Account {
    return new Account(mnemonic);
  }

  /**
   * Generate a new random account
   */
  static generate(): Account {
    const mnemonic = mnemonicGenerate();
    return new Account(mnemonic);
  }

  /**
   * Get the account address (SS58 format)
   */
  get address(): string {
    return this.pair.address;
  }

  /**
   * Get the public key (hex format)
   */
  get publicKey(): string {
    return this.pair.publicKey.toString('hex');
  }

  /**
   * Sign a message
   */
  sign(message: Uint8Array): Uint8Array {
    return this.pair.sign(message);
  }
}
