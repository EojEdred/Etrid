/**
 * React Hooks for ETRID Wallet
 * Convenient hooks for using the wallet service in React components
 */

'use client';

import { useState, useEffect, useCallback } from 'react';
import { getWalletService, WalletError } from './service';
import type { WalletAccount, WalletBalance, ChainInfo } from './types';

/**
 * Hook for managing wallet state
 */
export function useWalletService() {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const wallet = getWalletService();

  const initialize = useCallback(async () => {
    try {
      setError(null);
      await wallet.initialize();
      setIsInitialized(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to initialize wallet');
      throw err;
    }
  }, [wallet]);

  const connect = useCallback(async () => {
    try {
      setError(null);
      await wallet.connect();
      setIsConnected(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to connect to chain');
      throw err;
    }
  }, [wallet]);

  const disconnect = useCallback(async () => {
    await wallet.disconnect();
    setIsConnected(false);
  }, [wallet]);

  return {
    wallet,
    isInitialized,
    isConnected,
    error,
    initialize,
    connect,
    disconnect,
  };
}

/**
 * Hook for wallet accounts
 */
export function useWalletAccounts() {
  const [accounts, setAccounts] = useState<WalletAccount[]>([]);
  const [isUnlocked, setIsUnlocked] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const wallet = getWalletService();

  const hasWallet = useCallback(() => {
    return wallet.hasWallet();
  }, [wallet]);

  const createWallet = useCallback(async (password: string, mnemonicWordCount: 12 | 24 = 12) => {
    try {
      setIsLoading(true);
      setError(null);

      await wallet.initialize();
      const account = await wallet.createWallet({
        mnemonicWordCount,
        password,
        accountName: 'Account 1',
      });

      setAccounts([account]);
      setIsUnlocked(true);
      return account;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to create wallet';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  const importWallet = useCallback(async (mnemonic: string, password: string) => {
    try {
      setIsLoading(true);
      setError(null);

      await wallet.initialize();
      const account = await wallet.importWallet({
        mnemonic,
        password,
        accountName: 'Imported Account',
      });

      setAccounts([account]);
      setIsUnlocked(true);
      return account;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to import wallet';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  const unlockWallet = useCallback(async (password: string) => {
    try {
      setIsLoading(true);
      setError(null);

      await wallet.initialize();
      const loadedAccounts = await wallet.unlockWallet(password);

      setAccounts(loadedAccounts);
      setIsUnlocked(true);
      return loadedAccounts;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to unlock wallet';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  const lockWallet = useCallback(() => {
    wallet.lockWallet();
    setAccounts([]);
    setIsUnlocked(false);
  }, [wallet]);

  const deriveAccount = useCallback(async (derivationPath: string, accountName: string, password: string) => {
    try {
      setIsLoading(true);
      setError(null);

      const account = await wallet.deriveAccount({ derivationPath, accountName }, password);
      setAccounts(wallet.getAccounts());
      return account;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to derive account';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  return {
    accounts,
    isUnlocked,
    isLoading,
    error,
    hasWallet,
    createWallet,
    importWallet,
    unlockWallet,
    lockWallet,
    deriveAccount,
  };
}

/**
 * Hook for account balance
 */
export function useBalance(address: string | null) {
  const [balance, setBalance] = useState<WalletBalance | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const wallet = getWalletService();

  const fetchBalance = useCallback(async () => {
    if (!address) return;

    try {
      setIsLoading(true);
      setError(null);

      await wallet.connect();
      const bal = await wallet.getBalance(address);
      setBalance(bal);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to fetch balance';
      setError(message);
    } finally {
      setIsLoading(false);
    }
  }, [address, wallet]);

  // Auto-fetch on mount and when address changes
  useEffect(() => {
    fetchBalance();
  }, [fetchBalance]);

  // Subscribe to balance updates
  useEffect(() => {
    if (!address) return;

    let unsubscribe: (() => void) | null = null;

    const subscribe = async () => {
      try {
        await wallet.connect();
        unsubscribe = await wallet.subscribeBalance(address, (bal) => {
          setBalance(bal);
        });
      } catch (err) {
        console.error('Failed to subscribe to balance:', err);
      }
    };

    subscribe();

    return () => {
      if (unsubscribe) unsubscribe();
    };
  }, [address, wallet]);

  return {
    balance,
    isLoading,
    error,
    refetch: fetchBalance,
  };
}

/**
 * Hook for chain information
 */
export function useChainInfo() {
  const [chainInfo, setChainInfo] = useState<ChainInfo | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const wallet = getWalletService();

  const fetchChainInfo = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);

      await wallet.connect();
      const info = await wallet.getChainInfo();
      setChainInfo(info);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to fetch chain info';
      setError(message);
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  useEffect(() => {
    fetchChainInfo();
  }, [fetchChainInfo]);

  return {
    chainInfo,
    isLoading,
    error,
    refetch: fetchChainInfo,
  };
}

/**
 * Hook for sending transactions
 */
export function useSendTransaction() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [txHash, setTxHash] = useState<string | null>(null);

  const wallet = getWalletService();

  const sendTransaction = useCallback(async (
    from: string,
    to: string,
    amount: string,
    password: string
  ) => {
    try {
      setIsLoading(true);
      setError(null);
      setTxHash(null);

      await wallet.connect();
      const result = await wallet.sendTransaction(from, to, amount, password);

      setTxHash(result.hash);
      return result;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Transaction failed';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  const estimateFee = useCallback(async (from: string, to: string, amount: string) => {
    try {
      await wallet.connect();
      return await wallet.estimateFee(from, to, amount);
    } catch (err) {
      console.error('Failed to estimate fee:', err);
      return null;
    }
  }, [wallet]);

  return {
    sendTransaction,
    estimateFee,
    isLoading,
    error,
    txHash,
  };
}

/**
 * Hook for wallet backup and export
 */
export function useWalletBackup() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const wallet = getWalletService();

  const exportMnemonic = useCallback(async (password: string) => {
    try {
      setIsLoading(true);
      setError(null);
      return await wallet.exportMnemonic(password);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to export mnemonic';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  const exportWallet = useCallback(async (password: string) => {
    try {
      setIsLoading(true);
      setError(null);
      return await wallet.exportWallet(password);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to export wallet';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  const importWalletBackup = useCallback(async (backupJson: string, password: string) => {
    try {
      setIsLoading(true);
      setError(null);
      await wallet.importWalletBackup(backupJson, password);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to import backup';
      setError(message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, [wallet]);

  return {
    exportMnemonic,
    exportWallet,
    importWalletBackup,
    isLoading,
    error,
  };
}
