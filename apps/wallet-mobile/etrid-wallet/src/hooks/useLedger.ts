/**
 * useLedger - Hook for Ledger hardware wallet integration
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import { useState, useCallback, useEffect } from 'react';
import LedgerService from '../services/LedgerService';
import {
  LedgerDevice,
  HardwareAccount,
  TransactionSigningRequest,
  SignedTransaction,
} from '../types/hardware.types';

interface UseLedgerResult {
  devices: LedgerDevice[];
  connectedDevice: LedgerDevice | null;
  accounts: HardwareAccount[];
  isScanning: boolean;
  isConnecting: boolean;
  error: string | null;
  scanForDevices: () => Promise<void>;
  connectDevice: (deviceId: string) => Promise<void>;
  disconnect: () => Promise<void>;
  getAccounts: () => Promise<void>;
  signTransaction: (txRequest: TransactionSigningRequest) => Promise<SignedTransaction>;
  getBatteryLevel: () => Promise<number>;
}

export const useLedger = (): UseLedgerResult => {
  const [devices, setDevices] = useState<LedgerDevice[]>([]);
  const [connectedDevice, setConnectedDevice] = useState<LedgerDevice | null>(null);
  const [accounts, setAccounts] = useState<HardwareAccount[]>([]);
  const [isScanning, setIsScanning] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Check for existing connection
    const device = LedgerService.getConnectedDevice();
    if (device) {
      setConnectedDevice(device);
    }

    return () => {
      // Cleanup
      LedgerService.stopScanning();
    };
  }, []);

  const scanForDevices = useCallback(async () => {
    setIsScanning(true);
    setError(null);

    try {
      const foundDevices = await LedgerService.scanForDevices(5000);
      setDevices(foundDevices);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to scan for devices';
      setError(errorMessage);
    } finally {
      setIsScanning(false);
    }
  }, []);

  const connectDevice = useCallback(async (deviceId: string) => {
    setIsConnecting(true);
    setError(null);

    try {
      const device = await LedgerService.connectDevice(deviceId);
      setConnectedDevice(device);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to connect to device';
      setError(errorMessage);
      throw err;
    } finally {
      setIsConnecting(false);
    }
  }, []);

  const disconnect = useCallback(async () => {
    try {
      await LedgerService.disconnect();
      setConnectedDevice(null);
      setAccounts([]);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to disconnect';
      setError(errorMessage);
    }
  }, []);

  const getAccounts = useCallback(async () => {
    if (!connectedDevice) {
      setError('No device connected');
      return;
    }

    setError(null);

    try {
      const deviceAccounts = await LedgerService.getAccounts(connectedDevice, 5);
      setAccounts(deviceAccounts);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to get accounts';
      setError(errorMessage);
      throw err;
    }
  }, [connectedDevice]);

  const signTransaction = useCallback(
    async (txRequest: TransactionSigningRequest): Promise<SignedTransaction> => {
      if (!connectedDevice) {
        throw new Error('No device connected');
      }

      setError(null);

      try {
        const signed = await LedgerService.signTransaction(connectedDevice, txRequest);
        return signed;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to sign transaction';
        setError(errorMessage);
        throw err;
      }
    },
    [connectedDevice]
  );

  const getBatteryLevel = useCallback(async (): Promise<number> => {
    if (!connectedDevice) {
      return 0;
    }

    try {
      const level = await LedgerService.getBatteryLevel(connectedDevice);
      return level;
    } catch (err) {
      console.error('Failed to get battery level:', err);
      return 0;
    }
  }, [connectedDevice]);

  return {
    devices,
    connectedDevice,
    accounts,
    isScanning,
    isConnecting,
    error,
    scanForDevices,
    connectDevice,
    disconnect,
    getAccounts,
    signTransaction,
    getBatteryLevel,
  };
};
