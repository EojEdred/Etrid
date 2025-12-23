// Main DEX Service
export { DexService, SUPPORTED_CHAINS } from './service';
export type {
  ChainConfig,
  ChainType,
  UnifiedSwapQuote,
  UnifiedLiquidityPoolInfo,
  SwapParams,
  LiquidityParams,
} from './service';

// EVM DEX Service
export { EVMDexService, EVM_DEX_CONFIGS } from './evm';
export type {
  EVMDexConfig,
  SwapQuote,
  LiquidityPoolInfo,
} from './evm';

// Solana DEX Service
export { SolanaDexService, SOLANA_DEX_CONFIG } from './solana';
export type {
  SolanaDexConfig,
  SolanaSwapQuote,
  SolanaLiquidityPoolInfo,
} from './solana';

// React Hooks
export {
  useDexService,
  useSwapQuote,
  useSwap,
  usePoolInfo,
  useAddLiquidity,
  useRemoveLiquidity,
  useTokenBalance,
  useTokenInfo,
  usePriceImpactWarning,
  useWETRAddresses,
  useSupportedChains,
  useFormatTokenAmount,
  useParseTokenAmount,
  useSlippageAmount,
  useOptimalRoute,
  useSwapUI,
} from './hooks';
export type { UseDexServiceOptions } from './hooks';

// Utility functions
export {
  formatTokenAmount,
  parseTokenAmount,
  calculatePriceRatio,
  isValidEthereumAddress,
  isValidSolanaAddress,
  getChainTypeFromAddress,
} from './service';

// ============================================
// Additional types and utilities for swap UI
// ============================================

// Network enum
export enum Network {
  ETHEREUM = 'ethereum',
  BSC = 'bsc',
  POLYGON = 'polygon',
  ARBITRUM = 'arbitrum',
  SOLANA = 'solana',
}

// Token type
export interface Token {
  address: string;
  symbol: string;
  name: string;
  decimals: number;
  logoURI?: string;
}

// wETR token constants per network
export const WETR_TOKENS: Record<Network, Token> = {
  [Network.ETHEREUM]: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    symbol: 'wETR',
    name: 'Wrapped ETR',
    decimals: 18,
    logoURI: '/tokens/wetr.png',
  },
  [Network.BSC]: {
    address: '0xcc9b37fed77a01329502f8844620577742eb0dc6',
    symbol: 'wETR',
    name: 'Wrapped ETR',
    decimals: 18,
    logoURI: '/tokens/wetr.png',
  },
  [Network.POLYGON]: {
    address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb',
    symbol: 'wETR',
    name: 'Wrapped ETR',
    decimals: 18,
    logoURI: '/tokens/wetr.png',
  },
  [Network.ARBITRUM]: {
    address: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3',
    symbol: 'wETR',
    name: 'Wrapped ETR',
    decimals: 18,
    logoURI: '/tokens/wetr.png',
  },
  [Network.SOLANA]: {
    address: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4',
    symbol: 'wETR',
    name: 'Wrapped ETR',
    decimals: 9,
    logoURI: '/tokens/wetr.png',
  },
};

// Default wETR token (Ethereum)
export const WETR_TOKEN = WETR_TOKENS[Network.ETHEREUM];

// Slippage presets
export const SLIPPAGE_PRESETS = [0.1, 0.5, 1.0, 3.0];

// Format USD value
export function formatUsdValue(value: number): string {
  if (value === 0) return '$0.00';
  if (value < 0.01) return '<$0.01';
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

// Price impact levels
export type PriceImpactLevel = 'low' | 'medium' | 'high';

export interface PriceImpactInfo {
  level: PriceImpactLevel;
  color: string;
  message: string;
}

export function getPriceImpactLevel(priceImpact: number): PriceImpactInfo {
  if (priceImpact <= 1) {
    return { level: 'low', color: 'text-green-500', message: 'Low price impact' };
  } else if (priceImpact <= 5) {
    return { level: 'medium', color: 'text-yellow-500', message: 'Moderate price impact' };
  }
  return { level: 'high', color: 'text-red-500', message: 'High price impact' };
}

// ============================================
// UI Hooks for swap components
// ============================================
import { useState, useEffect, useCallback, useMemo } from 'react';

// Hook for network selection
export function useNetworkSelection(initialNetwork: Network = Network.ETHEREUM) {
  const [network, setNetwork] = useState<Network>(initialNetwork);
  const [dex, setDex] = useState<string>('uniswap');

  const updateNetwork = useCallback((newNetwork: Network) => {
    setNetwork(newNetwork);
    // Set default DEX per network
    switch (newNetwork) {
      case Network.BSC:
        setDex('pancakeswap');
        break;
      case Network.SOLANA:
        setDex('raydium');
        break;
      default:
        setDex('uniswap');
    }
  }, []);

  return { network, dex, updateNetwork, setDex };
}

// Base tokens per network
const BASE_TOKENS: Record<Network, Token[]> = {
  [Network.ETHEREUM]: [
    WETR_TOKEN,
    { address: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2', symbol: 'WETH', name: 'Wrapped Ether', decimals: 18 },
    { address: '0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48', symbol: 'USDC', name: 'USD Coin', decimals: 6 },
    { address: '0xdAC17F958D2ee523a2206206994597C13D831ec7', symbol: 'USDT', name: 'Tether USD', decimals: 6 },
  ],
  [Network.BSC]: [
    { ...WETR_TOKEN, address: '0xcc9b37fed77a01329502f8844620577742eb0dc6' },
    { address: '0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c', symbol: 'WBNB', name: 'Wrapped BNB', decimals: 18 },
    { address: '0x8AC76a51cc950d9822D68b83fE1Ad97B32Cd580d', symbol: 'USDC', name: 'USD Coin', decimals: 18 },
    { address: '0x55d398326f99059fF775485246999027B3197955', symbol: 'USDT', name: 'Tether USD', decimals: 18 },
  ],
  [Network.POLYGON]: [
    { ...WETR_TOKEN, address: '0x5566f6fb5cdb3aadf8662f9d1218ce2fc4bc72fb' },
    { address: '0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270', symbol: 'WMATIC', name: 'Wrapped Matic', decimals: 18 },
    { address: '0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174', symbol: 'USDC', name: 'USD Coin', decimals: 6 },
  ],
  [Network.ARBITRUM]: [
    { ...WETR_TOKEN, address: '0x1A065196152C2A70e54AC06D3a3433e3D8606eF3' },
    { address: '0x82aF49447D8a07e3bd95BD0d56f35241523fBab1', symbol: 'WETH', name: 'Wrapped Ether', decimals: 18 },
    { address: '0xaf88d065e77c8cC2239327C5EDb3A432268e5831', symbol: 'USDC', name: 'USD Coin', decimals: 6 },
  ],
  [Network.SOLANA]: [
    { address: 'CA4ALvCam7N3ya8d2axp3AakwNdCdQchQNNwYSYiMRR4', symbol: 'wETR', name: 'Wrapped ETR', decimals: 9 },
    { address: 'So11111111111111111111111111111111111111112', symbol: 'SOL', name: 'Wrapped SOL', decimals: 9 },
    { address: 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', symbol: 'USDC', name: 'USD Coin', decimals: 6 },
  ],
};

// Hook for available tokens
export function useAvailableTokens(network: Network): Token[] {
  return useMemo(() => BASE_TOKENS[network] || [], [network]);
}

// Hook for slippage management
export function useSlippage(initialSlippage: number = 0.5) {
  const [slippage, setSlippage] = useState(initialSlippage);
  const [isCustom, setIsCustom] = useState(false);

  const updateSlippage = useCallback((value: number) => {
    setSlippage(value);
    setIsCustom(!SLIPPAGE_PRESETS.includes(value));
  }, []);

  const setCustom = useCallback((value: number) => {
    setSlippage(value);
    setIsCustom(true);
  }, []);

  return { slippage, isCustom, updateSlippage, setCustom };
}

// Hook for getting swap quotes
export function useQuote(
  network: Network,
  fromToken: Token | null,
  toToken: Token | null,
  amount: string,
  slippage: number
) {
  const [quote, setQuote] = useState<{
    outputAmount: string;
    priceImpact: number;
    minimumReceived: string;
    route: string[];
    estimatedGas: string;
  } | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchQuote = useCallback(async () => {
    if (!fromToken || !toToken || !amount || parseFloat(amount) === 0) {
      setQuote(null);
      return;
    }

    setLoading(true);
    setError(null);

    try {
      // Simulate quote fetching - in production this would call the actual DEX
      await new Promise(resolve => setTimeout(resolve, 500));

      // Mock quote calculation
      const inputValue = parseFloat(amount);
      const mockRate = 0.95; // Simulate a swap rate
      const outputValue = inputValue * mockRate;
      const priceImpact = inputValue > 1000 ? 2.5 : inputValue > 100 ? 0.5 : 0.1;

      setQuote({
        outputAmount: outputValue.toFixed(6),
        priceImpact,
        minimumReceived: (outputValue * (1 - slippage / 100)).toFixed(6),
        route: [fromToken.symbol, toToken.symbol],
        estimatedGas: '0.005',
      });
    } catch (err) {
      setError('Failed to fetch quote');
      setQuote(null);
    } finally {
      setLoading(false);
    }
  }, [fromToken, toToken, amount, slippage]);

  useEffect(() => {
    const debounce = setTimeout(fetchQuote, 300);
    return () => clearTimeout(debounce);
  }, [fetchQuote]);

  return { quote, loading, error, refetch: fetchQuote };
}

// ============================================
// Price tracking hooks
// ============================================

export interface PriceDataPoint {
  timestamp: number;
  price: number;
  volume?: number;
}

export type TimeRange = '1H' | '24H' | '7D' | '30D' | '1Y';

/**
 * Hook for fetching price history data
 */
export function usePriceHistory(
  network: Network,
  tokenAddress: string,
  timeRange: TimeRange
) {
  const [priceData, setPriceData] = useState<PriceDataPoint[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchPriceHistory = useCallback(async () => {
    if (!tokenAddress) {
      setPriceData([]);
      setLoading(false);
      return;
    }

    setLoading(true);
    setError(null);

    try {
      // Generate mock price history data
      const now = Date.now();
      const points: PriceDataPoint[] = [];

      let interval: number;
      let count: number;

      switch (timeRange) {
        case '1H':
          interval = 60 * 1000; // 1 minute
          count = 60;
          break;
        case '24H':
          interval = 15 * 60 * 1000; // 15 minutes
          count = 96;
          break;
        case '7D':
          interval = 60 * 60 * 1000; // 1 hour
          count = 168;
          break;
        case '30D':
          interval = 4 * 60 * 60 * 1000; // 4 hours
          count = 180;
          break;
        case '1Y':
          interval = 24 * 60 * 60 * 1000; // 1 day
          count = 365;
          break;
        default:
          interval = 60 * 60 * 1000;
          count = 24;
      }

      // Base price around $0.08 for ETR (realistic starting point)
      let basePrice = 0.08;

      for (let i = count - 1; i >= 0; i--) {
        // Add some random walk volatility
        const change = (Math.random() - 0.48) * 0.005; // Slight upward bias
        basePrice = Math.max(0.01, basePrice * (1 + change));

        points.push({
          timestamp: now - (i * interval),
          price: basePrice,
          volume: Math.random() * 1000000 + 500000,
        });
      }

      setPriceData(points);
    } catch (err) {
      setError('Failed to fetch price history');
      setPriceData([]);
    } finally {
      setLoading(false);
    }
  }, [network, tokenAddress, timeRange]);

  useEffect(() => {
    fetchPriceHistory();
  }, [fetchPriceHistory]);

  return { priceData, loading, error, refetch: fetchPriceHistory };
}

/**
 * Hook for fetching realtime price
 */
export function useRealtimePrice(network: Network, tokenAddress: string) {
  const [price, setPrice] = useState<number | null>(null);
  const [lastUpdate, setLastUpdate] = useState<Date | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchPrice = useCallback(async () => {
    if (!tokenAddress) {
      setPrice(null);
      setLoading(false);
      return;
    }

    try {
      setError(null);

      // Mock realtime price - in production this would call a price API
      // ETR price with small random variation
      const basePrice = 0.082;
      const variation = (Math.random() - 0.5) * 0.004;
      const newPrice = basePrice + variation;

      setPrice(newPrice);
      setLastUpdate(new Date());
      setLoading(false);
    } catch (err) {
      setError('Failed to fetch price');
      setLoading(false);
    }
  }, [network, tokenAddress]);

  useEffect(() => {
    fetchPrice();
    // Refresh every 10 seconds for realtime data
    const interval = setInterval(fetchPrice, 10000);
    return () => clearInterval(interval);
  }, [fetchPrice]);

  return { price, lastUpdate, loading, error, refetch: fetchPrice };
}
