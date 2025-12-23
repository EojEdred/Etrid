'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { initApi } from '../lib/polkadot/api';
import {
  getRecentTransactions,
  subscribeTransactions,
  type Transaction,
} from '../lib/polkadot/transactions';

export interface UseTransactionHistoryOptions {
  limit?: number;
  subscribe?: boolean;
}

export interface UseTransactionHistoryReturn {
  transactions: Transaction[];
  isLoading: boolean;
  error: string | null;
  refetch: () => Promise<void>;
}

/**
 * React hook for fetching and subscribing to transaction history
 *
 * @param address - The address to fetch transaction history for
 * @param options - Configuration options
 * @returns Transaction history data, loading state, error state, and refetch function
 *
 * @example
 * ```tsx
 * const { transactions, isLoading, error, refetch } = useTransactionHistory(address, {
 *   limit: 50,
 *   subscribe: true
 * });
 *
 * if (isLoading) return <div>Loading transactions...</div>;
 * if (error) return <div>Error: {error}</div>;
 *
 * return (
 *   <div>
 *     {transactions.map(tx => (
 *       <div key={tx.hash}>{tx.hash}</div>
 *     ))}
 *   </div>
 * );
 * ```
 */
export function useTransactionHistory(
  address: string | null,
  options: UseTransactionHistoryOptions = {}
): UseTransactionHistoryReturn {
  const { limit = 50, subscribe = true } = options;

  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const connectingRef = useRef<boolean>(false);
  const unsubscribeRef = useRef<(() => void) | null>(null);
  const isMountedRef = useRef<boolean>(true);

  /**
   * Fetch transactions without subscription
   */
  const fetchTransactions = useCallback(
    async (addr: string): Promise<void> => {
      try {
        setIsLoading(true);
        setError(null);

        // Ensure API is connected
        if (connectingRef.current) {
          return; // Already attempting to connect
        }

        connectingRef.current = true;

        try {
          await initApi();
        } catch (apiError) {
          console.error('[useTransactionHistory] API connection failed:', apiError);
          throw new Error('Unable to connect to network');
        } finally {
          connectingRef.current = false;
        }

        // Fetch transaction history
        console.log(`[useTransactionHistory] Fetching transactions for ${addr}...`);
        const txs = await getRecentTransactions(addr, limit);

        if (isMountedRef.current) {
          setTransactions(txs);
          setError(null);
          console.log(`[useTransactionHistory] Fetched ${txs.length} transactions`);
        }
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to fetch transaction history';
        console.error('[useTransactionHistory] Error fetching transactions:', err);

        if (isMountedRef.current) {
          setError(errorMessage);
          setTransactions([]);
        }
      } finally {
        if (isMountedRef.current) {
          setIsLoading(false);
        }
      }
    },
    [limit]
  );

  /**
   * Subscribe to new transactions
   */
  const subscribeToTransactions = useCallback(
    async (addr: string): Promise<void> => {
      try {
        setIsLoading(true);
        setError(null);

        // Ensure API is connected
        if (connectingRef.current) {
          return;
        }

        connectingRef.current = true;

        try {
          await initApi();
        } catch (apiError) {
          console.error('[useTransactionHistory] API connection failed:', apiError);
          throw new Error('Unable to connect to network');
        } finally {
          connectingRef.current = false;
        }

        // First, fetch recent transactions
        console.log(`[useTransactionHistory] Fetching initial transactions for ${addr}...`);
        const txs = await getRecentTransactions(addr, limit);

        if (isMountedRef.current) {
          setTransactions(txs);
          console.log(`[useTransactionHistory] Initial fetch: ${txs.length} transactions`);
        }

        // Then subscribe to new transactions
        if (subscribe && isMountedRef.current) {
          console.log(`[useTransactionHistory] Subscribing to new transactions for ${addr}...`);

          const unsubscribe = await subscribeTransactions(addr, (newTx) => {
            if (isMountedRef.current) {
              console.log('[useTransactionHistory] New transaction detected:', newTx.hash);

              setTransactions((prevTxs) => {
                // Check if transaction already exists
                const exists = prevTxs.some((tx) => tx.hash === newTx.hash);
                if (exists) {
                  return prevTxs;
                }

                // Add new transaction to the beginning and maintain limit
                return [newTx, ...prevTxs].slice(0, limit);
              });
            }
          });

          unsubscribeRef.current = unsubscribe;
        }

        if (isMountedRef.current) {
          setError(null);
          setIsLoading(false);
        }
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to subscribe to transactions';
        console.error('[useTransactionHistory] Error subscribing to transactions:', err);

        if (isMountedRef.current) {
          setError(errorMessage);
          setTransactions([]);
          setIsLoading(false);
        }
      }
    },
    [limit, subscribe]
  );

  /**
   * Manual refetch function
   */
  const refetch = useCallback(async (): Promise<void> => {
    if (!address) {
      setError('No address provided');
      setTransactions([]);
      return;
    }

    await fetchTransactions(address);
  }, [address, fetchTransactions]);

  /**
   * Main effect: Fetch or subscribe to transactions when address changes
   */
  useEffect(() => {
    isMountedRef.current = true;

    // Reset state if no address
    if (!address) {
      setTransactions([]);
      setError(null);
      setIsLoading(false);
      return;
    }

    // Cleanup previous subscription
    if (unsubscribeRef.current) {
      console.log('[useTransactionHistory] Cleaning up previous subscription');
      unsubscribeRef.current();
      unsubscribeRef.current = null;
    }

    // Fetch or subscribe
    if (subscribe) {
      subscribeToTransactions(address);
    } else {
      fetchTransactions(address);
    }

    // Cleanup on unmount or address change
    return () => {
      isMountedRef.current = false;

      if (unsubscribeRef.current) {
        console.log('[useTransactionHistory] Unsubscribing from transactions');
        unsubscribeRef.current();
        unsubscribeRef.current = null;
      }
    };
  }, [address, subscribe, subscribeToTransactions, fetchTransactions]);

  return {
    transactions,
    isLoading,
    error,
    refetch,
  };
}
