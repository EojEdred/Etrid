/**
 * useWallet Hook
 * React hook for wallet management and blockchain interactions
 */

'use client';

import { useState, useEffect, useCallback } from 'react';
import type { ChainId } from './chains';
import { CHAINS, DEFAULT_CHAIN } from './chains';
import {
  createApi,
  getBalance,
  formatBalance,
  parseBalance,
  subscribeBalance,
} from './api';

// Dynamic import to avoid SSR issues with browser-only extension
let web3Accounts: any;
let web3Enable: any;
let web3FromAddress: any;

if (typeof window !== 'undefined') {
  import('@polkadot/extension-dapp').then((module) => {
    web3Accounts = module.web3Accounts;
    web3Enable = module.web3Enable;
    web3FromAddress = module.web3FromAddress;
  });
}

interface InjectedAccountWithMeta {
  address: string;
  meta: {
    name?: string;
    source: string;
  };
}

export interface WalletAccount extends InjectedAccountWithMeta {
  balance?: string;
  balanceRaw?: bigint;
}

export interface UseWalletReturn {
  // Account management
  accounts: WalletAccount[];
  selectedAccount: WalletAccount | null;
  selectAccount: (address: string) => void;

  // Chain management
  selectedChain: ChainId;
  setSelectedChain: (chainId: ChainId) => void;

  // Connection state
  isConnected: boolean;
  isLoading: boolean;
  error: string | null;

  // Actions
  connect: () => Promise<void>;
  disconnect: () => void;
  refreshBalance: () => Promise<void>;

  // Transaction
  sendTransaction: (to: string, amount: string) => Promise<string>;
}

export function useWallet(): UseWalletReturn {
  const [accounts, setAccounts] = useState<WalletAccount[]>([]);
  const [selectedAccount, setSelectedAccount] = useState<WalletAccount | null>(null);
  const [selectedChain, setSelectedChain] = useState<ChainId>(DEFAULT_CHAIN);
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Connect to Polkadot.js extension
  const connect = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      // Enable extension
      const extensions = await web3Enable('Ëtrid Wallet');

      if (extensions.length === 0) {
        throw new Error('No Polkadot.js extension found. Please install it first.');
      }

      // Get accounts
      const allAccounts = await web3Accounts();

      if (allAccounts.length === 0) {
        throw new Error('No accounts found in extension');
      }

      setAccounts(allAccounts);
      setSelectedAccount(allAccounts[0]);
      setIsConnected(true);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to connect wallet';
      setError(message);
      console.error('[useWallet] Connection error:', err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Disconnect wallet
  const disconnect = useCallback(() => {
    setAccounts([]);
    setSelectedAccount(null);
    setIsConnected(false);
    setError(null);
  }, []);

  // Select account
  const selectAccount = useCallback((address: string) => {
    const account = accounts.find((acc) => acc.address === address);
    if (account) {
      setSelectedAccount(account);
    }
  }, [accounts]);

  // Refresh balance for selected account
  const refreshBalance = useCallback(async () => {
    if (!selectedAccount) return;

    try {
      const balanceRaw = await getBalance(selectedChain, selectedAccount.address);
      const config = CHAINS[selectedChain];
      const balance = formatBalance(balanceRaw, config.decimals);

      setSelectedAccount((prev) =>
        prev ? { ...prev, balance, balanceRaw } : null
      );

      // Update in accounts list
      setAccounts((prev) =>
        prev.map((acc) =>
          acc.address === selectedAccount.address
            ? { ...acc, balance, balanceRaw }
            : acc
        )
      );
    } catch (err) {
      console.error('[useWallet] Failed to refresh balance:', err);
    }
  }, [selectedAccount, selectedChain]);

  // Send transaction
  const sendTransaction = useCallback(
    async (to: string, amount: string): Promise<string> => {
      if (!selectedAccount) {
        throw new Error('No account selected');
      }

      const api = await createApi(selectedChain);
      const injector = await web3FromAddress(selectedAccount.address);
      const config = CHAINS[selectedChain];

      // Parse amount to proper units
      const amountRaw = parseBalance(amount, config.decimals);

      return new Promise((resolve, reject) => {
        api.tx.balances
          .transferKeepAlive(to, amountRaw.toString())
          .signAndSend(
            selectedAccount.address,
            { signer: injector.signer },
            ({ status, txHash }) => {
              if (status.isInBlock) {
                console.log(`[useWallet] Transaction included in block`);
                resolve(txHash.toHex());
              } else if (status.isFinalized) {
                console.log(`[useWallet] Transaction finalized`);
              } else if (status.isInvalid) {
                reject(new Error('Transaction invalid'));
              }
            }
          )
          .catch(reject);
      });
    },
    [selectedAccount, selectedChain]
  );

  // Subscribe to balance updates
  useEffect(() => {
    if (!selectedAccount || !isConnected) return;

    let unsubscribe: (() => void) | undefined;

    (async () => {
      try {
        const config = CHAINS[selectedChain];

        unsubscribe = await subscribeBalance(
          selectedChain,
          selectedAccount.address,
          (balanceRaw) => {
            const balance = formatBalance(balanceRaw, config.decimals);

            setSelectedAccount((prev) =>
              prev ? { ...prev, balance, balanceRaw } : null
            );

            setAccounts((prev) =>
              prev.map((acc) =>
                acc.address === selectedAccount.address
                  ? { ...acc, balance, balanceRaw }
                  : acc
              )
            );
          }
        );
      } catch (err) {
        console.error('[useWallet] Balance subscription error:', err);
      }
    })();

    return () => {
      if (unsubscribe) unsubscribe();
    };
  }, [selectedAccount, selectedChain, isConnected]);

  // Initial balance fetch
  useEffect(() => {
    if (isConnected && selectedAccount) {
      refreshBalance();
    }
  }, [isConnected, selectedAccount, selectedChain, refreshBalance]);

  return {
    accounts,
    selectedAccount,
    selectAccount,
    selectedChain,
    setSelectedChain,
    isConnected,
    isLoading,
    error,
    connect,
    disconnect,
    refreshBalance,
    sendTransaction,
  };
}
