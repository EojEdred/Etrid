import NfcManager, {NfcTech, Ndef} from 'react-native-nfc-manager';

export class NFCService {
  private isInitialized = false;

  /**
   * Initialize NFC Manager
   */
  async initialize(): Promise<boolean> {
    try {
      await NfcManager.start();
      this.isInitialized = true;
      console.log('[NFC] NFC Manager initialized');
      return true;
    } catch (error) {
      console.error('[NFC] Initialization error:', error);
      return false;
    }
  }

  /**
   * Check if NFC is supported
   */
  async isSupported(): Promise<boolean> {
    try {
      return await NfcManager.isSupported();
    } catch (error) {
      console.error('[NFC] Error checking support:', error);
      return false;
    }
  }

  /**
   * Check if NFC is enabled
   */
  async isEnabled(): Promise<boolean> {
    try {
      return await NfcManager.isEnabled();
    } catch (error) {
      console.error('[NFC] Error checking if enabled:', error);
      return false;
    }
  }

  /**
   * Read NFC tag
   */
  async readTag(): Promise<string | null> {
    try {
      if (!this.isInitialized) {
        await this.initialize();
      }

      await NfcManager.requestTechnology(NfcTech.Ndef);
      const tag = await NfcManager.getTag();

      return tag?.id || null;
    } catch (error) {
      console.warn('[NFC] Read error:', error);
      return null;
    } finally {
      NfcManager.cancelTechnologyRequest();
    }
  }

  /**
   * Read NDEF message from tag
   */
  async readNdefMessage(): Promise<any[] | null> {
    try {
      if (!this.isInitialized) {
        await this.initialize();
      }

      await NfcManager.requestTechnology(NfcTech.Ndef);
      const tag = await NfcManager.getTag();

      return tag?.ndefMessage || null;
    } catch (error) {
      console.warn('[NFC] NDEF read error:', error);
      return null;
    } finally {
      NfcManager.cancelTechnologyRequest();
    }
  }

  /**
   * Write NDEF message to tag
   */
  async writeNdefMessage(message: string): Promise<boolean> {
    try {
      if (!this.isInitialized) {
        await this.initialize();
      }

      await NfcManager.requestTechnology(NfcTech.Ndef);

      const bytes = Ndef.encodeMessage([Ndef.textRecord(message)]);
      await NfcManager.ndefHandler.writeNdefMessage(bytes);

      return true;
    } catch (error) {
      console.error('[NFC] Write error:', error);
      return false;
    } finally {
      NfcManager.cancelTechnologyRequest();
    }
  }

  /**
   * Connect to Ledger hardware wallet via NFC
   */
  async connectLedger(): Promise<{success: boolean; data?: any}> {
    try {
      if (!this.isInitialized) {
        await this.initialize();
      }

      // Request NFC technology
      await NfcManager.requestTechnology(NfcTech.IsoDep);

      // Get tag info
      const tag = await NfcManager.getTag();
      console.log('[NFC] Ledger tag detected:', tag);

      // Here you would implement Ledger-specific APDU commands
      // This is a placeholder for the actual Ledger communication protocol

      return {success: true, data: tag};
    } catch (error) {
      console.error('[NFC] Ledger connection error:', error);
      return {success: false};
    } finally {
      NfcManager.cancelTechnologyRequest();
    }
  }

  /**
   * Cancel current NFC operation
   */
  cancel() {
    NfcManager.cancelTechnologyRequest();
  }

  /**
   * Clean up NFC resources
   */
  async cleanup() {
    try {
      await NfcManager.cancelTechnologyRequest();
      this.isInitialized = false;
    } catch (error) {
      console.error('[NFC] Cleanup error:', error);
    }
  }
}

// Export singleton instance
export const nfcService = new NFCService();
