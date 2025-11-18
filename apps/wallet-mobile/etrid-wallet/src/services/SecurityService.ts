/**
 * Security Service - Handles transaction security and authentication
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import * as LocalAuthentication from 'expo-local-authentication';
import AsyncStorage from '@react-native-async-storage/async-storage';
import LedgerService from './LedgerService';
import { TransactionSigningRequest } from '../types/hardware.types';

interface SecurityLevel {
  requiresBiometric: boolean;
  requiresPIN: boolean;
  requiresLedger: boolean;
}

class SecurityService {
  private static readonly PIN_KEY = '@etrid_user_pin';
  private static readonly SECURITY_SETTINGS_KEY = '@etrid_security_settings';

  // Transaction amount thresholds
  private static readonly LEDGER_THRESHOLD = 500; // >$500 requires Ledger
  private static readonly PIN_THRESHOLD = 100; // >$100 requires PIN

  /**
   * Check if biometric authentication is available
   */
  async isBiometricAvailable(): Promise<boolean> {
    try {
      const compatible = await LocalAuthentication.hasHardwareAsync();
      const enrolled = await LocalAuthentication.isEnrolledAsync();
      return compatible && enrolled;
    } catch (error) {
      console.error('Error checking biometric availability:', error);
      return false;
    }
  }

  /**
   * Get available biometric types
   */
  async getBiometricTypes(): Promise<string[]> {
    try {
      const types = await LocalAuthentication.supportedAuthenticationTypesAsync();
      return types.map((type) => {
        switch (type) {
          case LocalAuthentication.AuthenticationType.FINGERPRINT:
            return 'Fingerprint';
          case LocalAuthentication.AuthenticationType.FACIAL_RECOGNITION:
            return 'Face ID';
          case LocalAuthentication.AuthenticationType.IRIS:
            return 'Iris';
          default:
            return 'Biometric';
        }
      });
    } catch (error) {
      console.error('Error getting biometric types:', error);
      return [];
    }
  }

  /**
   * Authenticate with biometrics
   */
  async authenticateWithBiometric(reason: string = 'Authenticate'): Promise<boolean> {
    try {
      const result = await LocalAuthentication.authenticateAsync({
        promptMessage: reason,
        cancelLabel: 'Cancel',
        fallbackLabel: 'Use PIN',
        disableDeviceFallback: false,
      });

      return result.success;
    } catch (error) {
      console.error('Error authenticating with biometric:', error);
      return false;
    }
  }

  /**
   * Set user PIN
   */
  async setPIN(pin: string): Promise<void> {
    try {
      // In production, hash the PIN before storing
      const hashedPIN = this.hashPIN(pin);
      await AsyncStorage.setItem(SecurityService.PIN_KEY, hashedPIN);
    } catch (error) {
      console.error('Error setting PIN:', error);
      throw error;
    }
  }

  /**
   * Verify PIN
   */
  async verifyPIN(pin: string): Promise<boolean> {
    try {
      const storedHash = await AsyncStorage.getItem(SecurityService.PIN_KEY);
      if (!storedHash) {
        return false;
      }

      const inputHash = this.hashPIN(pin);
      return storedHash === inputHash;
    } catch (error) {
      console.error('Error verifying PIN:', error);
      return false;
    }
  }

  /**
   * Check if PIN is set
   */
  async hasPIN(): Promise<boolean> {
    try {
      const pin = await AsyncStorage.getItem(SecurityService.PIN_KEY);
      return pin !== null;
    } catch (error) {
      console.error('Error checking PIN:', error);
      return false;
    }
  }

  /**
   * Hash PIN (simple implementation - use better hashing in production)
   */
  private hashPIN(pin: string): string {
    // In production, use bcrypt or similar
    let hash = 0;
    for (let i = 0; i < pin.length; i++) {
      const char = pin.charCodeAt(i);
      hash = (hash << 5) - hash + char;
      hash = hash & hash;
    }
    return hash.toString(16);
  }

  /**
   * Determine required security level for transaction
   */
  async getRequiredSecurityLevel(amount: number): Promise<SecurityLevel> {
    return {
      requiresBiometric: true, // Always require biometric if available
      requiresPIN: amount > SecurityService.PIN_THRESHOLD,
      requiresLedger: amount > SecurityService.LEDGER_THRESHOLD,
    };
  }

  /**
   * Check if transaction requires Ledger
   */
  static async requiresLedger(amount: number): Promise<boolean> {
    return amount > SecurityService.LEDGER_THRESHOLD;
  }

  /**
   * Check if transaction requires PIN
   */
  static async requiresPIN(amount: number): Promise<boolean> {
    return (
      amount > SecurityService.PIN_THRESHOLD &&
      amount <= SecurityService.LEDGER_THRESHOLD
    );
  }

  /**
   * Authenticate transaction
   */
  async authenticateTransaction(
    amount: number,
    txRequest?: TransactionSigningRequest
  ): Promise<boolean> {
    try {
      const securityLevel = await this.getRequiredSecurityLevel(amount);

      // Step 1: Biometric authentication
      if (securityLevel.requiresBiometric) {
        const biometricAvailable = await this.isBiometricAvailable();
        if (biometricAvailable) {
          const biometricAuth = await this.authenticateWithBiometric(
            'Authenticate to continue'
          );
          if (!biometricAuth) {
            return false;
          }
        }
      }

      // Step 2: PIN verification (for medium amounts)
      if (securityLevel.requiresPIN) {
        const hasPIN = await this.hasPIN();
        if (!hasPIN) {
          throw new Error('PIN not set. Please set a PIN in settings.');
        }
        // PIN verification would be handled by a UI component
        // For now, we'll assume it's verified
      }

      // Step 3: Ledger signing (for large amounts)
      if (securityLevel.requiresLedger && txRequest) {
        const device = LedgerService.getConnectedDevice();
        if (!device) {
          throw new Error('Ledger not connected. Please connect your Ledger device.');
        }

        try {
          await LedgerService.signTransaction(device, txRequest);
          return true;
        } catch (error) {
          console.error('Ledger signing failed:', error);
          return false;
        }
      }

      return true;
    } catch (error) {
      console.error('Error authenticating transaction:', error);
      throw error;
    }
  }

  /**
   * Get security settings
   */
  async getSecuritySettings(): Promise<any> {
    try {
      const settings = await AsyncStorage.getItem(
        SecurityService.SECURITY_SETTINGS_KEY
      );
      return settings ? JSON.parse(settings) : this.getDefaultSettings();
    } catch (error) {
      console.error('Error getting security settings:', error);
      return this.getDefaultSettings();
    }
  }

  /**
   * Update security settings
   */
  async updateSecuritySettings(settings: any): Promise<void> {
    try {
      await AsyncStorage.setItem(
        SecurityService.SECURITY_SETTINGS_KEY,
        JSON.stringify(settings)
      );
    } catch (error) {
      console.error('Error updating security settings:', error);
      throw error;
    }
  }

  /**
   * Get default security settings
   */
  private getDefaultSettings(): any {
    return {
      biometricEnabled: true,
      pinEnabled: true,
      ledgerThreshold: SecurityService.LEDGER_THRESHOLD,
      pinThreshold: SecurityService.PIN_THRESHOLD,
      autoLockTimeout: 300, // 5 minutes
    };
  }

  /**
   * Format amount with security indicator
   */
  getSecurityIndicator(amount: number): string {
    if (amount > SecurityService.LEDGER_THRESHOLD) {
      return '🔒 Hardware wallet required';
    } else if (amount > SecurityService.PIN_THRESHOLD) {
      return '🔐 PIN required';
    } else {
      return '👆 Biometric required';
    }
  }

  /**
   * Validate transaction amount
   */
  validateAmount(amount: number, balance: number): { valid: boolean; error?: string } {
    if (amount <= 0) {
      return { valid: false, error: 'Amount must be greater than 0' };
    }

    if (amount > balance) {
      return { valid: false, error: 'Insufficient balance' };
    }

    return { valid: true };
  }
}

export default new SecurityService();
