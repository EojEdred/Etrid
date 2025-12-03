'use client';

import { useState, useEffect, useCallback, useMemo } from 'react';
import { useAccount } from 'wagmi';
import { Signer } from 'ethers';
import { PublicKey } from '@solana/web3.js';
import {
  DexService,
  UnifiedSwapQuote,
  UnifiedLiquidityPoolInfo,
  SwapParams,
  LiquidityParams,
  SUPPORTED_CHAINS,
  formatTokenAmount,
  parseTokenAmount,
} from './service';

export interface UseDexServiceOptions {
  chainKey: string;
  autoRefresh?: boolean;
  refreshInterval?: number; // milliseconds
}

/**
 * Main hook for DEX service
 */
export function useDexService(options: UseDexServiceOptions) {
  const { chainKey, autoRefresh = false, refreshInterval = 30000 } = options;
  const [service, setService] = useState<DexService | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);

  useEffect(() => {
    try {
      const dexService = new DexService(chainKey);
      setService(dexService);
      setIsInitialized(true);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to initialize DEX service'));
      setIsInitialized(false);
    }
  }, [chainKey]);

  return {
    service,
    isInitialized,
    error,
    chainConfig: service?.getChainConfig(),
    wETRAddress: service?.getWETRAddress(),
    dexName: service?.getDexName(),
  };
}

/**
 * Hook for getting swap quotes
 */
export function useSwapQuote(
  chainKey: string,
  params: SwapParams | null,
  options?: { enabled?: boolean; refetchInterval?: number }
) {
  const { enabled = true, refetchInterval } = options || {};
  const { service, isInitialized } = useDexService({ chainKey });
  const [quote, setQuote] = useState<UnifiedSwapQuote | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchQuote = useCallback(async () => {
    if (!service || !params || !enabled || !isInitialized) return;

    setIsLoading(true);
    setError(null);

    try {
      const result = await service.getSwapQuote(params);
      setQuote(result);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch quote'));
      setQuote(null);
    } finally {
      setIsLoading(false);
    }
  }, [service, params, enabled, isInitialized]);

  useEffect(() => {
    fetchQuote();

    if (refetchInterval && enabled) {
      const interval = setInterval(fetchQuote, refetchInterval);
      return () => clearInterval(interval);
    }
  }, [fetchQuote, refetchInterval, enabled]);

  return {
    quote,
    isLoading,
    error,
    refetch: fetchQuote,
  };
}

/**
 * Hook for executing swaps
 */
export function useSwap(chainKey: string) {
  const { service, isInitialized } = useDexService({ chainKey });
  const { address } = useAccount();
  const [isSwapping, setIsSwapping] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [txHash, setTxHash] = useState<string | null>(null);

  const executeSwap = useCallback(
    async (params: SwapParams, signer: Signer | PublicKey) => {
      if (!service || !isInitialized) {
        throw new Error('DEX service not initialized');
      }

      setIsSwapping(true);
      setError(null);
      setTxHash(null);

      try {
        const tx = await service.executeSwap(signer, params);

        if ('hash' in tx) {
          // EVM transaction
          setTxHash(tx.hash);
          const receipt = await tx.wait();
          return { success: true, txHash: receipt?.hash, receipt };
        } else {
          // Solana transaction (needs to be signed and sent separately)
          return { success: true, transaction: tx };
        }
      } catch (err) {
        const error = err instanceof Error ? err : new Error('Swap failed');
        setError(error);
        throw error;
      } finally {
        setIsSwapping(false);
      }
    },
    [service, isInitialized]
  );

  return {
    executeSwap,
    isSwapping,
    error,
    txHash,
  };
}

/**
 * Hook for liquidity pool info
 */
export function usePoolInfo(
  chainKey: string,
  tokenA: string | null,
  tokenB: string | null,
  options?: { enabled?: boolean; refetchInterval?: number }
) {
  const { enabled = true, refetchInterval } = options || {};
  const { service, isInitialized } = useDexService({ chainKey });
  const { address } = useAccount();
  const [poolInfo, setPoolInfo] = useState<UnifiedLiquidityPoolInfo | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchPoolInfo = useCallback(async () => {
    if (!service || !tokenA || !tokenB || !enabled || !isInitialized) return;

    setIsLoading(true);
    setError(null);

    try {
      const result = await service.getLiquidityPoolInfo(tokenA, tokenB, address);
      setPoolInfo(result);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch pool info'));
      setPoolInfo(null);
    } finally {
      setIsLoading(false);
    }
  }, [service, tokenA, tokenB, address, enabled, isInitialized]);

  useEffect(() => {
    fetchPoolInfo();

    if (refetchInterval && enabled) {
      const interval = setInterval(fetchPoolInfo, refetchInterval);
      return () => clearInterval(interval);
    }
  }, [fetchPoolInfo, refetchInterval, enabled]);

  return {
    poolInfo,
    isLoading,
    error,
    refetch: fetchPoolInfo,
  };
}

/**
 * Hook for adding liquidity
 */
export function useAddLiquidity(chainKey: string) {
  const { service, isInitialized } = useDexService({ chainKey });
  const [isAdding, setIsAdding] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [txHash, setTxHash] = useState<string | null>(null);

  const addLiquidity = useCallback(
    async (params: LiquidityParams, signer: Signer | PublicKey) => {
      if (!service || !isInitialized) {
        throw new Error('DEX service not initialized');
      }

      setIsAdding(true);
      setError(null);
      setTxHash(null);

      try {
        const tx = await service.addLiquidity(signer, params);

        if ('hash' in tx) {
          setTxHash(tx.hash);
          const receipt = await tx.wait();
          return { success: true, txHash: receipt?.hash, receipt };
        } else {
          return { success: true, transaction: tx };
        }
      } catch (err) {
        const error = err instanceof Error ? err : new Error('Add liquidity failed');
        setError(error);
        throw error;
      } finally {
        setIsAdding(false);
      }
    },
    [service, isInitialized]
  );

  return {
    addLiquidity,
    isAdding,
    error,
    txHash,
  };
}

/**
 * Hook for removing liquidity
 */
export function useRemoveLiquidity(chainKey: string) {
  const { service, isInitialized } = useDexService({ chainKey });
  const [isRemoving, setIsRemoving] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  const [txHash, setTxHash] = useState<string | null>(null);

  const removeLiquidity = useCallback(
    async (
      tokenA: string,
      tokenB: string,
      liquidity: string,
      slippage: number,
      signer: Signer | PublicKey,
      recipient?: string,
      deadline?: number
    ) => {
      if (!service || !isInitialized) {
        throw new Error('DEX service not initialized');
      }

      setIsRemoving(true);
      setError(null);
      setTxHash(null);

      try {
        const tx = await service.removeLiquidity(
          signer,
          tokenA,
          tokenB,
          liquidity,
          slippage,
          recipient,
          deadline
        );

        if ('hash' in tx) {
          setTxHash(tx.hash);
          const receipt = await tx.wait();
          return { success: true, txHash: receipt?.hash, receipt };
        } else {
          return { success: true, transaction: tx };
        }
      } catch (err) {
        const error = err instanceof Error ? err : new Error('Remove liquidity failed');
        setError(error);
        throw error;
      } finally {
        setIsRemoving(false);
      }
    },
    [service, isInitialized]
  );

  return {
    removeLiquidity,
    isRemoving,
    error,
    txHash,
  };
}

/**
 * Hook for token balance
 */
export function useTokenBalance(
  chainKey: string,
  tokenAddress: string | null,
  userAddress?: string
) {
  const { service, isInitialized } = useDexService({ chainKey });
  const { address: connectedAddress } = useAccount();
  const addressToUse = userAddress || connectedAddress;

  const [balance, setBalance] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchBalance = useCallback(async () => {
    if (!service || !tokenAddress || !addressToUse || !isInitialized) return;

    setIsLoading(true);
    setError(null);

    try {
      const result = await service.getTokenBalance(tokenAddress, addressToUse);
      setBalance(result);
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to fetch balance'));
      setBalance(null);
    } finally {
      setIsLoading(false);
    }
  }, [service, tokenAddress, addressToUse, isInitialized]);

  useEffect(() => {
    fetchBalance();
  }, [fetchBalance]);

  return {
    balance,
    isLoading,
    error,
    refetch: fetchBalance,
  };
}

/**
 * Hook for token info (EVM only)
 */
export function useTokenInfo(chainKey: string, tokenAddress: string | null) {
  const { service, isInitialized, chainConfig } = useDexService({ chainKey });
  const [tokenInfo, setTokenInfo] = useState<{ symbol: string; decimals: number } | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    if (!service || !tokenAddress || !isInitialized || chainConfig?.chainType !== 'evm') {
      return;
    }

    const fetchTokenInfo = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const result = await service.getTokenInfo(tokenAddress);
        setTokenInfo(result);
      } catch (err) {
        setError(err instanceof Error ? err : new Error('Failed to fetch token info'));
        setTokenInfo(null);
      } finally {
        setIsLoading(false);
      }
    };

    fetchTokenInfo();
  }, [service, tokenAddress, isInitialized, chainConfig]);

  return {
    tokenInfo,
    isLoading,
    error,
  };
}

/**
 * Hook for price impact warnings
 */
export function usePriceImpactWarning(priceImpact: number | undefined) {
  return useMemo(() => {
    if (!priceImpact) return null;

    if (priceImpact > 15) {
      return {
        level: 'error' as const,
        message: 'Price impact is very high. Consider reducing your swap amount.',
      };
    } else if (priceImpact > 5) {
      return {
        level: 'warning' as const,
        message: 'Price impact is high. You may get a better rate with a smaller swap.',
      };
    } else if (priceImpact > 2) {
      return {
        level: 'info' as const,
        message: 'Price impact is moderate.',
      };
    }

    return null;
  }, [priceImpact]);
}

/**
 * Hook for all wETR addresses
 */
export function useWETRAddresses() {
  return useMemo(() => DexService.getAllWETRAddresses(), []);
}

/**
 * Hook for supported chains
 */
export function useSupportedChains() {
  return useMemo(() => DexService.getSupportedChains(), []);
}

/**
 * Hook for formatting token amounts
 */
export function useFormatTokenAmount() {
  return useCallback((amount: string, decimals: number) => {
    try {
      return formatTokenAmount(amount, decimals);
    } catch {
      return '0';
    }
  }, []);
}

/**
 * Hook for parsing token amounts
 */
export function useParseTokenAmount() {
  return useCallback((amount: string, decimals: number) => {
    try {
      return parseTokenAmount(amount, decimals);
    } catch {
      return '0';
    }
  }, []);
}

/**
 * Hook for slippage calculation
 */
export function useSlippageAmount(amount: string | undefined, slippage: number) {
  return useMemo(() => {
    if (!amount) return null;

    try {
      const amountBN = BigInt(amount);
      const slippageMultiplier = (100 - slippage) / 100;
      const minimumAmount = (
        amountBN * BigInt(Math.floor(slippageMultiplier * 10000)) / BigInt(10000)
      ).toString();

      return minimumAmount;
    } catch {
      return null;
    }
  }, [amount, slippage]);
}

/**
 * Hook for swap route optimization
 */
export function useOptimalRoute(
  chainKey: string,
  fromToken: string | null,
  toToken: string | null
) {
  const { service, isInitialized } = useDexService({ chainKey });
  const [route, setRoute] = useState<string[]>([]);
  const [isCalculating, setIsCalculating] = useState(false);

  useEffect(() => {
    if (!service || !fromToken || !toToken || !isInitialized) {
      setRoute([]);
      return;
    }

    const calculateRoute = async () => {
      setIsCalculating(true);
      try {
        const optimalRoute = await service.calculateOptimalRoute(
          fromToken,
          toToken,
          '1000000000000000000' // 1 token in wei
        );
        setRoute(optimalRoute);
      } catch {
        setRoute([fromToken, toToken]);
      } finally {
        setIsCalculating(false);
      }
    };

    calculateRoute();
  }, [service, fromToken, toToken, isInitialized]);

  return { route, isCalculating };
}

/**
 * Compound hook for swap UI
 */
export function useSwapUI(chainKey: string) {
  const [fromToken, setFromToken] = useState<string>('');
  const [toToken, setToToken] = useState<string>('');
  const [amount, setAmount] = useState<string>('');
  const [slippage, setSlippage] = useState<number>(0.5);

  const { service, wETRAddress } = useDexService({ chainKey });

  const swapParams: SwapParams | null = useMemo(() => {
    if (!fromToken || !toToken || !amount) return null;
    return { fromToken, toToken, amount, slippage };
  }, [fromToken, toToken, amount, slippage]);

  const { quote, isLoading: isLoadingQuote, error: quoteError, refetch } = useSwapQuote(
    chainKey,
    swapParams,
    { enabled: !!swapParams, refetchInterval: 10000 }
  );

  const { executeSwap, isSwapping, error: swapError, txHash } = useSwap(chainKey);

  const priceImpactWarning = usePriceImpactWarning(quote?.priceImpact);

  const { balance: fromBalance } = useTokenBalance(chainKey, fromToken);
  const { balance: toBalance } = useTokenBalance(chainKey, toToken);

  const hasInsufficientBalance = useMemo(() => {
    if (!fromBalance || !amount) return false;
    try {
      return BigInt(fromBalance) < BigInt(amount);
    } catch {
      return false;
    }
  }, [fromBalance, amount]);

  return {
    // State
    fromToken,
    toToken,
    amount,
    slippage,
    // Setters
    setFromToken,
    setToToken,
    setAmount,
    setSlippage,
    // Quote
    quote,
    isLoadingQuote,
    quoteError,
    refetchQuote: refetch,
    // Swap
    executeSwap,
    isSwapping,
    swapError,
    txHash,
    // Validation
    priceImpactWarning,
    hasInsufficientBalance,
    // Balances
    fromBalance,
    toBalance,
    // Config
    wETRAddress,
    service,
  };
}
