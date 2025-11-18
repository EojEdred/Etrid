/**
 * Ledger Service - Handles Ledger hardware wallet integration via Bluetooth
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import { BleManager, Device, State } from 'react-native-ble-plx';
import { Platform, PermissionsAndroid } from 'react-native';
import {
  HardwareWallet,
  HardwareAccount,
  LedgerDevice,
  TransactionSigningRequest,
  SignedTransaction,
  LedgerStatus,
  LedgerError,
  DevicePermissions,
} from '../types/hardware.types';

class LedgerService {
  private bleManager: BleManager;
  private connectedDevice: LedgerDevice | null = null;
  private isScanning = false;
  private scanTimeout: NodeJS.Timeout | null = null;

  constructor() {
    this.bleManager = new BleManager();
  }

  /**
   * Request Bluetooth permissions (Android only)
   */
  async requestPermissions(): Promise<DevicePermissions> {
    if (Platform.OS === 'android') {
      try {
        const apiLevel = Platform.Version;

        if (apiLevel >= 31) {
          // Android 12+ requires BLUETOOTH_SCAN and BLUETOOTH_CONNECT
          const granted = await PermissionsAndroid.requestMultiple([
            PermissionsAndroid.PERMISSIONS.BLUETOOTH_SCAN,
            PermissionsAndroid.PERMISSIONS.BLUETOOTH_CONNECT,
            PermissionsAndroid.PERMISSIONS.ACCESS_FINE_LOCATION,
          ]);

          return {
            bluetooth: true,
            bluetoothScan: granted['android.permission.BLUETOOTH_SCAN'] === 'granted',
            bluetoothConnect:
              granted['android.permission.BLUETOOTH_CONNECT'] === 'granted',
            location: granted['android.permission.ACCESS_FINE_LOCATION'] === 'granted',
          };
        } else {
          // Android 11 and below
          const granted = await PermissionsAndroid.request(
            PermissionsAndroid.PERMISSIONS.ACCESS_FINE_LOCATION
          );

          return {
            bluetooth: true,
            bluetoothScan: granted === 'granted',
            bluetoothConnect: granted === 'granted',
            location: granted === 'granted',
          };
        }
      } catch (error) {
        console.error('Error requesting Bluetooth permissions:', error);
        throw new Error('PERMISSION_DENIED');
      }
    }

    // iOS - permissions handled automatically
    return {
      bluetooth: true,
      bluetoothScan: true,
      bluetoothConnect: true,
    };
  }

  /**
   * Check if Bluetooth is enabled
   */
  async isBluetoothEnabled(): Promise<boolean> {
    const state = await this.bleManager.state();
    return state === State.PoweredOn;
  }

  /**
   * Scan for Ledger devices
   */
  async scanForDevices(duration: number = 5000): Promise<LedgerDevice[]> {
    try {
      // Check permissions
      const permissions = await this.requestPermissions();
      if (!permissions.bluetoothScan) {
        throw new Error('PERMISSION_DENIED');
      }

      // Check if Bluetooth is enabled
      const bluetoothEnabled = await this.isBluetoothEnabled();
      if (!bluetoothEnabled) {
        throw new Error('BLUETOOTH_OFF');
      }

      const devices: Map<string, LedgerDevice> = new Map();
      this.isScanning = true;

      // Start scanning
      this.bleManager.startDeviceScan(null, null, (error, device) => {
        if (error) {
          console.error('Scan error:', error);
          return;
        }

        if (device && this.isLedgerDevice(device)) {
          devices.set(device.id, device as LedgerDevice);
        }
      });

      // Stop scanning after duration
      await new Promise((resolve) => {
        this.scanTimeout = setTimeout(() => {
          this.bleManager.stopDeviceScan();
          this.isScanning = false;
          resolve(true);
        }, duration);
      });

      return Array.from(devices.values());
    } catch (error) {
      this.isScanning = false;
      console.error('Error scanning for devices:', error);
      throw error;
    }
  }

  /**
   * Stop scanning for devices
   */
  stopScanning(): void {
    if (this.scanTimeout) {
      clearTimeout(this.scanTimeout);
      this.scanTimeout = null;
    }
    this.bleManager.stopDeviceScan();
    this.isScanning = false;
  }

  /**
   * Check if device is a Ledger
   */
  private isLedgerDevice(device: Device): boolean {
    const name = device.name?.toLowerCase() || '';
    return (
      name.includes('nano') ||
      name.includes('ledger') ||
      name.includes('blue') ||
      device.id.toLowerCase().includes('ledger')
    );
  }

  /**
   * Connect to a Ledger device
   */
  async connectDevice(deviceId: string): Promise<LedgerDevice> {
    try {
      const permissions = await this.requestPermissions();
      if (!permissions.bluetoothConnect) {
        throw new Error('PERMISSION_DENIED');
      }

      const device = await this.bleManager.connectToDevice(deviceId);
      await device.discoverAllServicesAndCharacteristics();

      this.connectedDevice = device as LedgerDevice;

      // Get battery level
      try {
        const batteryLevel = await this.getBatteryLevel(device as LedgerDevice);
        this.connectedDevice.batteryLevel = batteryLevel;
      } catch (error) {
        console.warn('Could not read battery level:', error);
      }

      return this.connectedDevice;
    } catch (error) {
      console.error('Error connecting to device:', error);
      throw new Error('DEVICE_NOT_FOUND');
    }
  }

  /**
   * Disconnect from device
   */
  async disconnect(deviceId?: string): Promise<void> {
    try {
      if (deviceId) {
        await this.bleManager.cancelDeviceConnection(deviceId);
      } else if (this.connectedDevice) {
        await this.bleManager.cancelDeviceConnection(this.connectedDevice.id);
        this.connectedDevice = null;
      }
    } catch (error) {
      console.error('Error disconnecting device:', error);
      throw error;
    }
  }

  /**
   * Get connected device
   */
  getConnectedDevice(): LedgerDevice | null {
    return this.connectedDevice;
  }

  /**
   * Get Ledger accounts using BIP44 derivation
   */
  async getAccounts(device: LedgerDevice, count: number = 5): Promise<HardwareAccount[]> {
    try {
      if (!device) {
        throw new Error('NOT_CONNECTED');
      }

      const accounts: HardwareAccount[] = [];

      // BIP44 path for Ëtrid: m/44'/354'/0'/0/i
      // 354 is the coin type for Ëtrid
      for (let i = 0; i < count; i++) {
        const derivationPath = `m/44'/354'/0'/0/${i}`;

        // In a real implementation, you would use Ledger's API to get the address
        // For now, we'll simulate this
        const account: HardwareAccount = {
          index: i,
          address: await this.getAddressAtPath(device, derivationPath),
          publicKey: await this.getPublicKeyAtPath(device, derivationPath),
          derivationPath,
          label: `Ledger Account ${i + 1}`,
        };

        accounts.push(account);
      }

      return accounts;
    } catch (error) {
      console.error('Error getting accounts:', error);
      throw error;
    }
  }

  /**
   * Get address at derivation path
   */
  private async getAddressAtPath(
    device: LedgerDevice,
    path: string
  ): Promise<string> {
    // This would use Ledger's APDU commands to get the address
    // For now, returning a simulated address
    // In production, integrate with @ledgerhq/hw-transport-ble
    const pathHash = this.hashString(path);
    return `0x${pathHash.slice(0, 40)}`;
  }

  /**
   * Get public key at derivation path
   */
  private async getPublicKeyAtPath(
    device: LedgerDevice,
    path: string
  ): Promise<string> {
    // This would use Ledger's APDU commands to get the public key
    const pathHash = this.hashString(path + '_pub');
    return `0x${pathHash.slice(0, 66)}`;
  }

  /**
   * Sign transaction with Ledger
   */
  async signTransaction(
    device: LedgerDevice,
    txRequest: TransactionSigningRequest
  ): Promise<SignedTransaction> {
    try {
      if (!device) {
        throw new Error('NOT_CONNECTED');
      }

      // In production, this would:
      // 1. Send transaction to Ledger for user approval
      // 2. User reviews on Ledger screen
      // 3. User approves by pressing buttons
      // 4. Ledger signs and returns signature

      // Simulated signing (replace with actual Ledger integration)
      const signatureHex = await this.requestSignature(device, txRequest);
      const signature = this.hexToUint8Array(signatureHex);

      const txHash = this.hashString(
        JSON.stringify(txRequest) + Date.now().toString()
      );

      return {
        signature,
        txHash: `0x${txHash}`,
        signedAt: new Date().toISOString(),
        device: device.name || device.id,
      };
    } catch (error) {
      console.error('Error signing transaction:', error);

      if (error instanceof Error && error.message.includes('rejected')) {
        throw new Error('USER_REJECTED');
      }

      throw new Error('UNKNOWN');
    }
  }

  /**
   * Request signature from Ledger (simulated)
   */
  private async requestSignature(
    device: LedgerDevice,
    txRequest: TransactionSigningRequest
  ): Promise<string> {
    // Simulate the async signing process
    await new Promise((resolve) => setTimeout(resolve, 2000));

    // In production, this would send APDU commands to Ledger
    // and wait for user confirmation
    const dataToSign = JSON.stringify(txRequest);
    return this.hashString(dataToSign);
  }

  /**
   * Get battery level
   */
  async getBatteryLevel(device: LedgerDevice): Promise<number> {
    try {
      // This would read battery level from Ledger's battery service
      // Standard Bluetooth Battery Service UUID: 0x180F
      // For now, returning a simulated value
      return Math.floor(Math.random() * 30) + 70; // 70-100%
    } catch (error) {
      console.warn('Could not read battery level:', error);
      return 0;
    }
  }

  /**
   * Get Ledger status
   */
  async getStatus(device: LedgerDevice): Promise<LedgerStatus> {
    try {
      const batteryLevel = await this.getBatteryLevel(device);

      return {
        connected: true,
        locked: false, // Would check actual lock status
        batteryLevel,
        currentApp: {
          name: 'Ëtrid',
          version: '1.0.0',
          isOpen: true,
        },
      };
    } catch (error) {
      console.error('Error getting status:', error);
      return {
        connected: false,
        locked: false,
      };
    }
  }

  /**
   * Utility: Hash string (simple implementation)
   */
  private hashString(str: string): string {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      const char = str.charCodeAt(i);
      hash = (hash << 5) - hash + char;
      hash = hash & hash;
    }

    // Convert to hex and pad
    const hex = Math.abs(hash).toString(16);
    return hex.padEnd(64, '0');
  }

  /**
   * Utility: Convert hex to Uint8Array
   */
  private hexToUint8Array(hex: string): Uint8Array {
    const cleaned = hex.replace(/^0x/, '');
    const bytes = new Uint8Array(cleaned.length / 2);

    for (let i = 0; i < cleaned.length; i += 2) {
      bytes[i / 2] = parseInt(cleaned.slice(i, i + 2), 16);
    }

    return bytes;
  }

  /**
   * Destroy BLE manager
   */
  destroy(): void {
    this.stopScanning();
    if (this.connectedDevice) {
      this.disconnect(this.connectedDevice.id).catch(console.error);
    }
    this.bleManager.destroy();
  }
}

export default new LedgerService();
