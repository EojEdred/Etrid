'use client';

import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';
import {
  initKeyring,
  generateMnemonic,
  validateMnemonic,
  createWallet,
  importWallet,
  unlockWallet,
  lockWallet,
  getCurrentAccount,
  hasStoredWallet,
  getStoredWallet,
  deleteWallet,
  type WalletAccount,
  type StoredWallet,
} from '@/lib/wallet/keyring';

export type WalletStatus = 'loading' | 'no_wallet' | 'locked' | 'unlocked';

interface WalletContextType {
  // State
  status: WalletStatus;
  account: WalletAccount | null;
  storedWallet: StoredWallet | null;
  error: string | null;

  // Actions
  createNewWallet: (password: string, name?: string) => Promise<{ mnemonic: string; address: string }>;
  importExistingWallet: (mnemonic: string, password: string, name?: string) => Promise<void>;
  unlock: (password: string) => Promise<void>;
  lock: () => void;
  deleteCurrentWallet: () => void;

  // Helpers
  generateNewMnemonic: () => string;
  isValidMnemonic: (mnemonic: string) => boolean;
}

const WalletContext = createContext<WalletContextType | null>(null);

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [status, setStatus] = useState<WalletStatus>('loading');
  const [account, setAccount] = useState<WalletAccount | null>(null);
  const [storedWallet, setStoredWallet] = useState<StoredWallet | null>(null);
  const [error, setError] = useState<string | null>(null);

  // Initialize on mount
  useEffect(() => {
    const init = async () => {
      try {
        await initKeyring();

        const stored = getStoredWallet();
        setStoredWallet(stored);

        if (stored) {
          setStatus('locked');
        } else {
          setStatus('no_wallet');
        }
      } catch (err) {
        console.error('Failed to initialize wallet:', err);
        setError(err instanceof Error ? err.message : 'Failed to initialize wallet');
        setStatus('no_wallet');
      }
    };

    init();
  }, []);

  const createNewWallet = useCallback(async (password: string, name?: string) => {
    setError(null);

    try {
      const mnemonic = generateMnemonic();
      const walletAccount = await createWallet(mnemonic, password, name);

      setAccount(walletAccount);
      setStoredWallet(getStoredWallet());
      setStatus('unlocked');

      return { mnemonic, address: walletAccount.address };
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to create wallet';
      setError(message);
      throw err;
    }
  }, []);

  const importExistingWallet = useCallback(async (mnemonic: string, password: string, name?: string) => {
    setError(null);

    try {
      const walletAccount = await importWallet(mnemonic, password, name);

      setAccount(walletAccount);
      setStoredWallet(getStoredWallet());
      setStatus('unlocked');
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to import wallet';
      setError(message);
      throw err;
    }
  }, []);

  const unlock = useCallback(async (password: string) => {
    setError(null);

    try {
      const walletAccount = await unlockWallet(password);
      setAccount(walletAccount);
      setStatus('unlocked');
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to unlock wallet';
      setError(message);
      throw err;
    }
  }, []);

  const lock = useCallback(() => {
    lockWallet();
    setAccount(null);
    setStatus('locked');
  }, []);

  const deleteCurrentWallet = useCallback(() => {
    deleteWallet();
    setAccount(null);
    setStoredWallet(null);
    setStatus('no_wallet');
  }, []);

  const generateNewMnemonic = useCallback(() => {
    return generateMnemonic();
  }, []);

  const isValidMnemonic = useCallback((mnemonic: string) => {
    return validateMnemonic(mnemonic);
  }, []);

  return (
    <WalletContext.Provider
      value={{
        status,
        account,
        storedWallet,
        error,
        createNewWallet,
        importExistingWallet,
        unlock,
        lock,
        deleteCurrentWallet,
        generateNewMnemonic,
        isValidMnemonic,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
}

export function useWallet() {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
}
