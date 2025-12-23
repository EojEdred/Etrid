'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import {
  initApi,
  disconnectApi,
  getChainInfo,
  getCurrentBlock,
  subscribeNewBlocks,
} from '@/lib/polkadot/api';

interface ChainInfo {
  chain: string;
  nodeName: string;
  nodeVersion: string;
  tokenSymbol: string;
  tokenDecimals: number;
}

interface UsePolkadotApiReturn {
  isConnected: boolean;
  isConnecting: boolean;
  error: string | null;
  chainInfo: ChainInfo | null;
  currentBlock: number;
  reconnect: () => Promise<void>;
}

const MAX_RETRIES = 3;
const RETRY_DELAY_MS = 2000;

/**
 * React hook for managing connection to ËTRID Primearc Core Chain
 *
 * Features:
 * - Auto-connect on mount with retry logic
 * - Live block height subscription
 * - Graceful error handling
 * - Clean disconnection on unmount
 *
 * @returns {UsePolkadotApiReturn} Polkadot API state and reconnect function
 */
export function usePolkadotApi(): UsePolkadotApiReturn {
  const [isConnected, setIsConnected] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [chainInfo, setChainInfo] = useState<ChainInfo | null>(null);
  const [currentBlock, setCurrentBlock] = useState(0);

  // Track cleanup functions
  const unsubscribeRef = useRef<(() => void) | null>(null);
  const retryCountRef = useRef(0);
  const isMountedRef = useRef(true);
  const isConnectingRef = useRef(false);

  /**
   * Connect to the chain with retry logic
   */
  const connect = useCallback(async () => {
    // Prevent multiple simultaneous connection attempts (even across re-renders).
    if (isConnectingRef.current) return;

    isConnectingRef.current = true;
    setIsConnecting(true);
    setError(null);

    try {
      console.log('[usePolkadotApi] Initiating connection to ËTRID Primearc Core Chain...');

      // Initialize API connection (shared singleton in lib/polkadot/api.ts).
      await initApi();
      if (!isMountedRef.current) return;

      console.log('[usePolkadotApi] Connection established successfully');
      setIsConnected(true);
      retryCountRef.current = 0;

      // Fetch chain information
      try {
        const info = await getChainInfo();
        if (isMountedRef.current) {
          setChainInfo(info);
          console.log('[usePolkadotApi] Chain info loaded:', info);
        }
      } catch (err) {
        console.warn('[usePolkadotApi] Failed to fetch chain info:', err);
        if (isMountedRef.current) {
          setChainInfo({
            chain: 'ËTRID Primearc Core Chain',
            nodeName: 'Unknown',
            nodeVersion: 'Unknown',
            tokenSymbol: 'ETR',
            tokenDecimals: 12,
          });
        }
      }

      // Get initial block number
      try {
        const blockNumber = await getCurrentBlock();
        if (isMountedRef.current) {
          setCurrentBlock(blockNumber);
          console.log('[usePolkadotApi] Current block:', blockNumber);
        }
      } catch (err) {
        console.warn('[usePolkadotApi] Failed to fetch current block:', err);
      }

      // Subscribe to new blocks
      try {
        const unsubscribe = await subscribeNewBlocks((blockNumber, blockHash) => {
          if (isMountedRef.current) {
            setCurrentBlock(blockNumber);
            console.log(`[usePolkadotApi] New block #${blockNumber}: ${blockHash}`);
          }
        });

        unsubscribeRef.current = unsubscribe;
      } catch (err) {
        console.warn('[usePolkadotApi] Failed to subscribe to new blocks:', err);
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown connection error';
      console.error('[usePolkadotApi] Connection failed:', errorMessage);

      if (!isMountedRef.current) return;

      // Retry logic
      if (retryCountRef.current < MAX_RETRIES) {
        retryCountRef.current += 1;
        console.log(
          `[usePolkadotApi] Retrying connection (${retryCountRef.current}/${MAX_RETRIES}) in ${RETRY_DELAY_MS}ms...`
        );

        setTimeout(() => {
          if (isMountedRef.current) {
            void connect();
          }
        }, RETRY_DELAY_MS);
      } else {
        // Max retries reached - set error state
        setError(errorMessage);
        setIsConnected(false);
        console.error(
          `[usePolkadotApi] Failed to connect after ${MAX_RETRIES} attempts. Last error: ${errorMessage}`
        );
      }
    } finally {
      isConnectingRef.current = false;
      if (isMountedRef.current) {
        setIsConnecting(false);
      }
    }
  }, []);

  /**
   * Manually reconnect to the chain
   * Resets retry count and attempts connection
   */
  const reconnect = useCallback(async () => {
    console.log('[usePolkadotApi] Manual reconnection requested');

    // Reset state
    setError(null);
    retryCountRef.current = 0;

    // Cleanup existing subscriptions
    if (unsubscribeRef.current) {
      try {
        unsubscribeRef.current();
        unsubscribeRef.current = null;
      } catch (err) {
        console.warn('[usePolkadotApi] Error unsubscribing during reconnect:', err);
      }
    }

    // Force a disconnect so endpoint changes (e.g. onboarding config) take effect.
    try {
      await disconnectApi();
    } catch (err) {
      console.warn('[usePolkadotApi] Error disconnecting during reconnect:', err);
    }

    // Attempt new connection
    await connect();
  }, [connect]);

  // Auto-connect on mount
  useEffect(() => {
    isMountedRef.current = true;

    console.log('[usePolkadotApi] Hook mounted, initiating connection...');
    connect();

    // Cleanup on unmount
    return () => {
      console.log('[usePolkadotApi] Hook unmounting, cleaning up...');
      isMountedRef.current = false;

      // Unsubscribe from block updates
      if (unsubscribeRef.current) {
        try {
          unsubscribeRef.current();
          unsubscribeRef.current = null;
          console.log('[usePolkadotApi] Unsubscribed from new blocks');
        } catch (err) {
          console.warn('[usePolkadotApi] Error unsubscribing:', err);
        }
      }
    };
  }, [connect]);

  return {
    isConnected,
    isConnecting,
    error,
    chainInfo,
    currentBlock,
    reconnect,
  };
}
