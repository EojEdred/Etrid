/**
 * Hardware Wallet Types - Type definitions for Ledger integration
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import { Device } from 'react-native-ble-plx';

export interface HardwareWallet {
  id: string;
  type: 'ledger' | 'degn';
  model: string;
  name: string;
  firmware: string;
  connected: boolean;
  batteryLevel?: number; // 0-100
  lastConnected?: string;
  accounts: HardwareAccount[];
  bluetoothId?: string;
}

export interface HardwareAccount {
  index: number;
  address: string;
  publicKey: string;
  derivationPath: string; // BIP44 path: m/44'/354'/0'/0/0
  balance?: string;
  label?: string;
}

export interface LedgerDevice extends Device {
  batteryLevel?: number;
  firmware?: string;
}

export interface BluetoothScanResult {
  devices: LedgerDevice[];
  isScanning: boolean;
  error?: string;
}

export interface LedgerConnection {
  device: LedgerDevice;
  connected: boolean;
  connecting: boolean;
  error?: string;
}

export interface TransactionSigningRequest {
  from: string;
  to: string;
  amount: string;
  asset: string;
  fee: string;
  data?: string;
  nonce?: number;
  chainId?: number;
}

export interface SignedTransaction {
  signature: Uint8Array;
  txHash: string;
  signedAt: string;
  device: string;
}

export interface LedgerAppInfo {
  name: string;
  version: string;
  isOpen: boolean;
}

export interface LedgerStatus {
  connected: boolean;
  locked: boolean;
  currentApp?: LedgerAppInfo;
  batteryLevel?: number;
}

export type LedgerError =
  | 'NOT_CONNECTED'
  | 'USER_REJECTED'
  | 'TIMEOUT'
  | 'WRONG_APP'
  | 'LOCKED'
  | 'BLUETOOTH_OFF'
  | 'PERMISSION_DENIED'
  | 'DEVICE_NOT_FOUND'
  | 'UNKNOWN';

export interface DevicePermissions {
  bluetooth: boolean;
  bluetoothConnect: boolean;
  bluetoothScan: boolean;
  location?: boolean; // Required for Bluetooth on Android
}
