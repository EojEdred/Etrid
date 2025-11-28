import ReactNativeBiometrics, {BiometryTypes} from 'react-native-biometrics';

export class BiometricService {
  private rnBiometrics: ReactNativeBiometrics;

  constructor() {
    this.rnBiometrics = new ReactNativeBiometrics({
      allowDeviceCredentials: true,
    });
  }

  /**
   * Check if biometric authentication is available on device
   */
  async isBiometricAvailable(): Promise<{
    available: boolean;
    biometryType: BiometryTypes | null;
  }> {
    try {
      const {available, biometryType} = await this.rnBiometrics.isSensorAvailable();
      return {available, biometryType};
    } catch (error) {
      console.error('[BiometricService] Error checking availability:', error);
      return {available: false, biometryType: null};
    }
  }

  /**
   * Authenticate user with biometrics
   */
  async authenticate(promptMessage: string = 'Authenticate to continue'): Promise<boolean> {
    try {
      const {success} = await this.rnBiometrics.simplePrompt({
        promptMessage,
        cancelButtonText: 'Cancel',
      });
      return success;
    } catch (error) {
      console.error('[BiometricService] Authentication failed:', error);
      return false;
    }
  }

  /**
   * Create biometric keys for advanced security
   */
  async createKeys(): Promise<string | null> {
    try {
      const {publicKey} = await this.rnBiometrics.createKeys();
      return publicKey;
    } catch (error) {
      console.error('[BiometricService] Error creating keys:', error);
      return null;
    }
  }

  /**
   * Delete biometric keys
   */
  async deleteKeys(): Promise<boolean> {
    try {
      const {keysDeleted} = await this.rnBiometrics.deleteKeys();
      return keysDeleted;
    } catch (error) {
      console.error('[BiometricService] Error deleting keys:', error);
      return false;
    }
  }

  /**
   * Create a biometric signature
   */
  async createSignature(payload: string, promptMessage: string): Promise<{
    success: boolean;
    signature?: string;
  }> {
    try {
      const {success, signature} = await this.rnBiometrics.createSignature({
        promptMessage,
        payload,
        cancelButtonText: 'Cancel',
      });
      return {success, signature};
    } catch (error) {
      console.error('[BiometricService] Error creating signature:', error);
      return {success: false};
    }
  }
}

// Export singleton instance
export const biometricService = new BiometricService();
