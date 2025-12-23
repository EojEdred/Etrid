'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import { initApi, getBalance, subscribeBalance } from '../lib/polkadot/api';

export interface BalanceData {
  free: string;
  reserved: string;
  frozen: string;
  total: string;
  formatted: string;
}

export interface UseBalanceReturn {
  balance: BalanceData | null;
  isLoading: boolean;
  error: string | null;
  refetch: () => Promise<void>;
}

/**
 * React hook for fetching and subscribing to ËTRID account balances
 *
 * @param address - The Polkadot address to fetch balance for (or null)
 * @returns Balance data, loading state, error state, and refetch function
 *
 * @example
 * ```tsx
 * const { balance, isLoading, error, refetch } = useBalance(address);
 *
 * if (isLoading) return <div>Loading...</div>;
 * if (error) return <div>Error: {error}</div>;
 * if (!balance) return <div>No balance data</div>;
 *
 * return <div>Balance: {balance.formatted}</div>;
 * ```
 */
export function useBalance(address: string | null): UseBalanceReturn {
  const [balance, setBalance] = useState<BalanceData | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  // Track if we're currently trying to connect to avoid duplicate connection attempts
  const connectingRef = useRef<boolean>(false);
  const unsubscribeRef = useRef<(() => void) | null>(null);

  /**
   * Fetch balance once without subscription
   */
  const fetchBalance = useCallback(async (addr: string): Promise<void> => {
    try {
      setIsLoading(true);
      setError(null);

      // Try to connect if not already connected
      if (connectingRef.current) {
        return; // Already attempting to connect
      }

      connectingRef.current = true;

      try {
        await initApi();
      } catch (apiError) {
        console.error('[useBalance] API connection failed:', apiError);
        throw new Error('Unable to connect to network');
      } finally {
        connectingRef.current = false;
      }

      // Fetch balance
      const balanceData = await getBalance(addr);
      setBalance(balanceData);
      setError(null);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to fetch balance';
      console.error('[useBalance] Error fetching balance:', err);
      setError(errorMessage);
      setBalance(null);
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Subscribe to balance updates
   */
  const subscribeToBalance = useCallback(async (addr: string): Promise<void> => {
    try {
      setIsLoading(true);
      setError(null);

      // Try to connect if not already connected
      if (connectingRef.current) {
        return; // Already attempting to connect
      }

      connectingRef.current = true;

      try {
        await initApi();
      } catch (apiError) {
        console.error('[useBalance] API connection failed:', apiError);
        throw new Error('Unable to connect to network');
      } finally {
        connectingRef.current = false;
      }

      // Subscribe to balance changes
      const unsubscribe = await subscribeBalance(addr, (balanceData) => {
        setBalance(balanceData);
        setError(null);
        setIsLoading(false);
      });

      unsubscribeRef.current = unsubscribe;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to subscribe to balance';
      console.error('[useBalance] Error subscribing to balance:', err);
      setError(errorMessage);
      setBalance(null);
      setIsLoading(false);
    }
  }, []);

  /**
   * Manual refetch function
   */
  const refetch = useCallback(async (): Promise<void> => {
    if (!address) {
      setError('No address provided');
      setBalance(null);
      return;
    }

    await fetchBalance(address);
  }, [address, fetchBalance]);

  /**
   * Main effect: Subscribe to balance when address changes
   */
  useEffect(() => {
    // Reset state if no address
    if (!address) {
      setBalance(null);
      setError(null);
      setIsLoading(false);
      return;
    }

    // Cleanup previous subscription
    if (unsubscribeRef.current) {
      unsubscribeRef.current();
      unsubscribeRef.current = null;
    }

    // Start new subscription
    subscribeToBalance(address);

    // Cleanup on unmount or address change
    return () => {
      if (unsubscribeRef.current) {
        unsubscribeRef.current();
        unsubscribeRef.current = null;
      }
    };
  }, [address, subscribeToBalance]);

  return {
    balance,
    isLoading,
    error,
    refetch,
  };
}
