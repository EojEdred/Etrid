'use client';

import React, { createContext, useContext, useState, useCallback, useEffect } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { formatBalance } from '@polkadot/util';
import {
  ChainConfig,
  PRIMEARC_CORE,
  ALL_CHAINS,
  getChainConfig,
  formatTokenAmount,
} from '@/lib/chains/config';

interface BalanceData {
  free: string;
  reserved: string;
  frozen: string;
  total: string;
  formatted: string;
}

interface ChainSelectorContextType {
  /** Currently selected chain */
  selectedChain: ChainConfig;
  /** All available chains */
  availableChains: ChainConfig[];
  /** Switch to a different chain */
  switchChain: (chainId: string) => Promise<void>;
  /** Is chain currently connected */
  isConnected: boolean;
  /** Is switching in progress */
  isSwitching: boolean;
  /** Connection error message */
  error: string | null;
  /** Get balance for current chain */
  getBalance: (address: string) => Promise<BalanceData | null>;
  /** Current chain API instance */
  api: ApiPromise | null;
  /** Refresh connection to current chain */
  reconnect: () => Promise<void>;
}

const ChainSelectorContext = createContext<ChainSelectorContextType | undefined>(undefined);

interface ChainSelectorProviderProps {
  children: React.ReactNode;
  /** Initial chain to connect to (defaults to Primearc Core) */
  initialChainId?: string;
}

/**
 * Provider for Multi-PBC Chain Selection
 *
 * Manages connection to ËTRID chains (Relay Chain + PBCs)
 * Handles chain switching and balance queries
 */
export function ChainSelectorProvider({
  children,
  initialChainId = 'primearc-core',
}: ChainSelectorProviderProps) {
  const [selectedChain, setSelectedChain] = useState<ChainConfig>(() => {
    const chain = getChainConfig(initialChainId);
    return chain || PRIMEARC_CORE;
  });

  const [api, setApi] = useState<ApiPromise | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [isSwitching, setIsSwitching] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Connect to a chain
   */
  const connectToChain = useCallback(async (chain: ChainConfig) => {
    try {
      setIsSwitching(true);
      setError(null);

      console.log(`[ChainSelector] Connecting to ${chain.name} (${chain.wsEndpoint})...`);

      // Disconnect existing connection
      if (api) {
        await api.disconnect();
        setApi(null);
        setIsConnected(false);
      }

      // Create new connection
      const provider = new WsProvider(chain.wsEndpoint, 5000); // 5s timeout
      const newApi = await ApiPromise.create({
        provider,
        throwOnConnect: true,
      });

      await newApi.isReady;

      // Verify connection
      const chainName = await newApi.rpc.system.chain();
      console.log(`[ChainSelector] Connected to ${chainName}`);

      // Configure balance formatting
      const properties = await newApi.rpc.system.properties();
      const tokenDecimals = properties.tokenDecimals.unwrapOr([chain.decimals])[0].toNumber();
      const tokenSymbol = properties.tokenSymbol.unwrapOr([chain.token])[0].toString();

      formatBalance.setDefaults({
        decimals: tokenDecimals,
        unit: tokenSymbol,
      });

      setApi(newApi);
      setIsConnected(true);
      setSelectedChain(chain);
      setError(null);

      console.log(`[ChainSelector] Successfully connected to ${chain.name}`);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to connect to chain';
      console.error(`[ChainSelector] Connection failed:`, err);
      setError(errorMessage);
      setIsConnected(false);
      setApi(null);
      throw err;
    } finally {
      setIsSwitching(false);
    }
  }, [api]);

  /**
   * Switch to a different chain
   */
  const switchChain = useCallback(async (chainId: string) => {
    const newChain = getChainConfig(chainId);
    if (!newChain) {
      throw new Error(`Chain not found: ${chainId}`);
    }

    console.log(`[ChainSelector] Switching from ${selectedChain.name} to ${newChain.name}`);
    await connectToChain(newChain);
  }, [selectedChain, connectToChain]);

  /**
   * Reconnect to current chain
   */
  const reconnect = useCallback(async () => {
    console.log(`[ChainSelector] Reconnecting to ${selectedChain.name}`);
    await connectToChain(selectedChain);
  }, [selectedChain, connectToChain]);

  /**
   * Get balance for an address on the current chain
   */
  const getBalance = useCallback(async (address: string): Promise<BalanceData | null> => {
    if (!api || !isConnected) {
      console.warn('[ChainSelector] Cannot get balance: not connected');
      return null;
    }

    try {
      const { data: balance } = await api.query.system.account(address);

      const free = balance.free.toString();
      const reserved = balance.reserved.toString();
      const frozen = balance.frozen?.toString() || '0';
      const total = (BigInt(free) + BigInt(reserved)).toString();

      return {
        free,
        reserved,
        frozen,
        total,
        formatted: formatTokenAmount(total, selectedChain.decimals),
      };
    } catch (err) {
      console.error('[ChainSelector] Error fetching balance:', err);
      return null;
    }
  }, [api, isConnected, selectedChain.decimals]);

  // Connect to initial chain on mount
  useEffect(() => {
    let isMounted = true;

    const initConnection = async () => {
      try {
        await connectToChain(selectedChain);
      } catch (err) {
        if (isMounted) {
          console.error('[ChainSelector] Initial connection failed:', err);
        }
      }
    };

    initConnection();

    return () => {
      isMounted = false;
      if (api) {
        api.disconnect().catch(console.error);
      }
    };
  }, []); // Only run on mount

  const value: ChainSelectorContextType = {
    selectedChain,
    availableChains: ALL_CHAINS,
    switchChain,
    isConnected,
    isSwitching,
    error,
    getBalance,
    api,
    reconnect,
  };

  return (
    <ChainSelectorContext.Provider value={value}>
      {children}
    </ChainSelectorContext.Provider>
  );
}

/**
 * Hook to access chain selector context
 * @throws Error if used outside ChainSelectorProvider
 */
export function useChainSelector(): ChainSelectorContextType {
  const context = useContext(ChainSelectorContext);
  if (!context) {
    throw new Error('useChainSelector must be used within ChainSelectorProvider');
  }
  return context;
}

/**
 * Hook to get balance for current chain
 * Auto-updates when chain or address changes
 */
export function useChainBalance(address: string | null) {
  const { getBalance, selectedChain, isConnected } = useChainSelector();
  const [balance, setBalance] = useState<BalanceData | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!address || !isConnected) {
      setBalance(null);
      setError(null);
      setIsLoading(false);
      return;
    }

    let isMounted = true;

    const fetchBalance = async () => {
      try {
        setIsLoading(true);
        setError(null);
        const balanceData = await getBalance(address);
        if (isMounted) {
          setBalance(balanceData);
        }
      } catch (err) {
        if (isMounted) {
          const errorMessage = err instanceof Error ? err.message : 'Failed to fetch balance';
          setError(errorMessage);
          setBalance(null);
        }
      } finally {
        if (isMounted) {
          setIsLoading(false);
        }
      }
    };

    fetchBalance();

    // Refresh balance every 15 seconds
    const interval = setInterval(fetchBalance, 15000);

    return () => {
      isMounted = false;
      clearInterval(interval);
    };
  }, [address, getBalance, selectedChain.id, isConnected]);

  return { balance, isLoading, error };
}
