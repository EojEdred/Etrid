'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { initApi } from '../lib/polkadot/api';
import {
  getRecentTransactions,
  subscribeTransactions,
  formatTransactionValue,
  formatTimeAgo,
  truncateHash,
  type Transaction,
} from '../lib/polkadot/transactions';

export interface FormattedTransaction {
  id: string;
  type: 'send' | 'receive';
  amount: string;
  amountFormatted: string;
  token: string;
  from: string;
  fromDisplay: string;
  to: string | null;
  toDisplay: string;
  time: string;
  timestamp: number;
  status: 'confirmed' | 'pending' | 'failed';
  hash: string;
  hashDisplay: string;
  blockNumber: number;
  method: string;
  section: string;
}

export interface UseTransactionsReturn {
  transactions: FormattedTransaction[];
  isLoading: boolean;
  error: string | null;
  refetch: () => Promise<void>;
}

/**
 * React hook for fetching and subscribing to transaction history
 */
export function useTransactions(address: string | null): UseTransactionsReturn {
  const [transactions, setTransactions] = useState<FormattedTransaction[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const connectingRef = useRef<boolean>(false);
  const unsubscribeRef = useRef<(() => void) | null>(null);

  /**
   * Format raw transaction for display
   */
  const formatTransaction = useCallback(
    (tx: Transaction, userAddress: string): FormattedTransaction => {
      const isReceive = tx.to === userAddress;

      return {
        id: `${tx.blockNumber}-${tx.hash.slice(0, 8)}`,
        type: isReceive ? 'receive' : 'send',
        amount: tx.value,
        amountFormatted: formatTransactionValue(tx.value),
        token: 'ETR',
        from: tx.from,
        fromDisplay: truncateHash(tx.from),
        to: tx.to,
        toDisplay: tx.to ? truncateHash(tx.to) : '',
        time: formatTimeAgo(tx.timestamp),
        timestamp: tx.timestamp,
        status: tx.status === 'success' ? 'confirmed' : 'failed',
        hash: tx.hash,
        hashDisplay: truncateHash(tx.hash),
        blockNumber: tx.blockNumber,
        method: tx.method,
        section: tx.section,
      };
    },
    []
  );

  /**
   * Fetch transactions
   */
  const fetchTransactions = useCallback(
    async (addr: string): Promise<void> => {
      try {
        setIsLoading(true);
        setError(null);

        if (connectingRef.current) {
          return;
        }

        connectingRef.current = true;

        try {
          await initApi();
        } catch (apiError) {
          console.error('[useTransactions] API connection failed:', apiError);
          throw new Error('Unable to connect to network');
        } finally {
          connectingRef.current = false;
        }

        // Fetch recent transactions
        const rawTransactions = await getRecentTransactions(addr, 50);
        const formatted = rawTransactions.map((tx) => formatTransaction(tx, addr));

        // Sort by timestamp, newest first
        formatted.sort((a, b) => b.timestamp - a.timestamp);

        setTransactions(formatted);
        setError(null);
      } catch (err) {
        const errorMessage = err instanceof Error ? err.message : 'Failed to fetch transactions';
        console.error('[useTransactions] Error:', err);
        setError(errorMessage);
        setTransactions([]);
      } finally {
        setIsLoading(false);
      }
    },
    [formatTransaction]
  );

  /**
   * Subscribe to new transactions
   */
  const startSubscription = useCallback(
    async (addr: string): Promise<void> => {
      try {
        if (connectingRef.current) {
          return;
        }

        connectingRef.current = true;

        try {
          await initApi();
        } catch (apiError) {
          console.error('[useTransactions] API connection failed:', apiError);
          throw new Error('Unable to connect to network');
        } finally {
          connectingRef.current = false;
        }

        const unsubscribe = await subscribeTransactions(addr, (newTx) => {
          const formatted = formatTransaction(newTx, addr);
          setTransactions((prev) => {
            // Check if already exists
            if (prev.some((t) => t.hash === formatted.hash)) {
              return prev;
            }
            // Add to front
            return [formatted, ...prev].slice(0, 50);
          });
        });

        unsubscribeRef.current = unsubscribe;
      } catch (err) {
        console.error('[useTransactions] Subscription error:', err);
      }
    },
    [formatTransaction]
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
   * Main effect: Fetch and subscribe when address changes
   */
  useEffect(() => {
    if (!address) {
      setTransactions([]);
      setError(null);
      setIsLoading(false);
      return;
    }

    // Cleanup previous subscription
    if (unsubscribeRef.current) {
      unsubscribeRef.current();
      unsubscribeRef.current = null;
    }

    // Fetch initial transactions
    fetchTransactions(address);

    // Start subscription for new transactions
    startSubscription(address);

    // Cleanup on unmount or address change
    return () => {
      if (unsubscribeRef.current) {
        unsubscribeRef.current();
        unsubscribeRef.current = null;
      }
    };
  }, [address, fetchTransactions, startSubscription]);

  return {
    transactions,
    isLoading,
    error,
    refetch,
  };
}

export { formatTimeAgo, truncateHash };
