import * as LocalAuthentication from 'expo-local-authentication';

/**
 * Service for biometric authentication (Face ID, Touch ID, Fingerprint)
 * Handles availability checks and authentication flows
 */
class BiometricService {
  /**
   * Check if biometric hardware is available
   */
  public async isAvailable(): Promise<boolean> {
    try {
      const hasHardware = await LocalAuthentication.hasHardwareAsync();
      return hasHardware;
    } catch (error) {
      console.error('Error checking biometric availability:', error);
      return false;
    }
  }

  /**
   * Check if biometrics are enrolled (user has set up Face ID/fingerprint)
   */
  public async isEnrolled(): Promise<boolean> {
    try {
      const isEnrolled = await LocalAuthentication.isEnrolledAsync();
      return isEnrolled;
    } catch (error) {
      console.error('Error checking biometric enrollment:', error);
      return false;
    }
  }

  /**
   * Get supported authentication types
   */
  public async getSupportedTypes(): Promise<LocalAuthentication.AuthenticationType[]> {
    try {
      const types = await LocalAuthentication.supportedAuthenticationTypesAsync();
      return types;
    } catch (error) {
      console.error('Error getting supported types:', error);
      return [];
    }
  }

  /**
   * Get biometric type name for display
   */
  public async getBiometricTypeName(): Promise<string> {
    try {
      const types = await this.getSupportedTypes();

      if (types.includes(LocalAuthentication.AuthenticationType.FACIAL_RECOGNITION)) {
        return 'Face ID';
      } else if (types.includes(LocalAuthentication.AuthenticationType.FINGERPRINT)) {
        return 'Fingerprint';
      } else if (types.includes(LocalAuthentication.AuthenticationType.IRIS)) {
        return 'Iris';
      }

      return 'Biometric';
    } catch (error) {
      console.error('Error getting biometric type name:', error);
      return 'Biometric';
    }
  }

  /**
   * Authenticate user with biometrics
   */
  public async authenticate(promptMessage?: string): Promise<{
    success: boolean;
    error?: string;
  }> {
    try {
      // Check if biometrics are available and enrolled
      const available = await this.isAvailable();
      if (!available) {
        return {
          success: false,
          error: 'Biometric authentication not available',
        };
      }

      const enrolled = await this.isEnrolled();
      if (!enrolled) {
        return {
          success: false,
          error: 'No biometrics enrolled. Please set up Face ID or fingerprint in your device settings.',
        };
      }

      // Authenticate
      const biometricType = await this.getBiometricTypeName();
      const result = await LocalAuthentication.authenticateAsync({
        promptMessage: promptMessage || `Use ${biometricType} to confirm`,
        cancelLabel: 'Cancel',
        disableDeviceFallback: false,
        fallbackLabel: 'Use passcode',
      });

      if (result.success) {
        return { success: true };
      } else {
        return {
          success: false,
          error: result.error || 'Authentication failed',
        };
      }
    } catch (error) {
      console.error('Error during biometric authentication:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Authentication failed',
      };
    }
  }

  /**
   * Authenticate for transaction signing
   */
  public async authenticateForTransaction(amount: string, recipient: string): Promise<boolean> {
    const result = await this.authenticate(
      `Confirm sending ${amount} ETR to ${recipient.slice(0, 8)}...`
    );
    return result.success;
  }

  /**
   * Authenticate for sensitive actions (wallet backup, settings)
   */
  public async authenticateForSensitiveAction(action: string): Promise<boolean> {
    const result = await this.authenticate(`Confirm ${action}`);
    return result.success;
  }

  /**
   * Check if biometric authentication is supported and can be used
   */
  public async canUseBiometric(): Promise<{
    canUse: boolean;
    reason?: string;
  }> {
    try {
      const hasHardware = await this.isAvailable();
      if (!hasHardware) {
        return {
          canUse: false,
          reason: 'Your device does not support biometric authentication',
        };
      }

      const isEnrolled = await this.isEnrolled();
      if (!isEnrolled) {
        return {
          canUse: false,
          reason: 'Please set up Face ID or fingerprint in your device settings',
        };
      }

      return { canUse: true };
    } catch (error) {
      console.error('Error checking biometric capability:', error);
      return {
        canUse: false,
        reason: 'Unable to check biometric support',
      };
    }
  }
}

export default new BiometricService();
