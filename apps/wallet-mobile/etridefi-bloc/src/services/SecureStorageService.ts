import * as Keychain from 'react-native-keychain';
import AsyncStorage from '@react-native-async-storage/async-storage';

export class SecureStorageService {
  /**
   * Store sensitive data securely (like private keys, credentials)
   * Uses iOS Keychain / Android Keystore
   */
  async setSecure(key: string, value: string): Promise<boolean> {
    try {
      await Keychain.setGenericPassword(key, value, {
        service: key,
        accessible: Keychain.ACCESSIBLE.WHEN_UNLOCKED_THIS_DEVICE_ONLY,
        accessControl: Keychain.ACCESS_CONTROL.BIOMETRY_ANY,
      });
      return true;
    } catch (error) {
      console.error('[SecureStorage] Error storing secure data:', error);
      return false;
    }
  }

  /**
   * Retrieve sensitive data securely
   */
  async getSecure(key: string): Promise<string | null> {
    try {
      const credentials = await Keychain.getGenericPassword({service: key});
      if (credentials) {
        return credentials.password;
      }
      return null;
    } catch (error) {
      console.error('[SecureStorage] Error retrieving secure data:', error);
      return null;
    }
  }

  /**
   * Delete sensitive data
   */
  async deleteSecure(key: string): Promise<boolean> {
    try {
      await Keychain.resetGenericPassword({service: key});
      return true;
    } catch (error) {
      console.error('[SecureStorage] Error deleting secure data:', error);
      return false;
    }
  }

  /**
   * Store non-sensitive data (settings, preferences, etc.)
   */
  async set(key: string, value: any): Promise<boolean> {
    try {
      const jsonValue = JSON.stringify(value);
      await AsyncStorage.setItem(key, jsonValue);
      return true;
    } catch (error) {
      console.error('[SecureStorage] Error storing data:', error);
      return false;
    }
  }

  /**
   * Retrieve non-sensitive data
   */
  async get<T>(key: string): Promise<T | null> {
    try {
      const jsonValue = await AsyncStorage.getItem(key);
      return jsonValue != null ? JSON.parse(jsonValue) : null;
    } catch (error) {
      console.error('[SecureStorage] Error retrieving data:', error);
      return null;
    }
  }

  /**
   * Delete non-sensitive data
   */
  async delete(key: string): Promise<boolean> {
    try {
      await AsyncStorage.removeItem(key);
      return true;
    } catch (error) {
      console.error('[SecureStorage] Error deleting data:', error);
      return false;
    }
  }

  /**
   * Clear all non-sensitive data
   */
  async clearAll(): Promise<boolean> {
    try {
      await AsyncStorage.clear();
      return true;
    } catch (error) {
      console.error('[SecureStorage] Error clearing all data:', error);
      return false;
    }
  }

  /**
   * Store wallet mnemonic securely
   */
  async setMnemonic(mnemonic: string): Promise<boolean> {
    return this.setSecure('wallet_mnemonic', mnemonic);
  }

  /**
   * Retrieve wallet mnemonic
   */
  async getMnemonic(): Promise<string | null> {
    return this.getSecure('wallet_mnemonic');
  }

  /**
   * Store private key securely
   */
  async setPrivateKey(address: string, privateKey: string): Promise<boolean> {
    return this.setSecure(`wallet_pk_${address}`, privateKey);
  }

  /**
   * Retrieve private key
   */
  async getPrivateKey(address: string): Promise<string | null> {
    return this.getSecure(`wallet_pk_${address}`);
  }
}

// Export singleton instance
export const secureStorage = new SecureStorageService();
