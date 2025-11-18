import { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../contexts/AuthContext';
import EtridSDKService from '../services/EtridSDKService';
import { formatTokenAmount, formatCurrency } from '../utils/formatters';
import { SupportedAsset } from '../utils/constants';

export interface AssetBalance {
  symbol: string;
  balance: string;
  balanceFormatted: string;
  usdValue: number;
  usdValueFormatted: string;
  change24h: number;
  allocation: number;
}

export interface PortfolioData {
  assets: AssetBalance[];
  totalValue: number;
  totalValueFormatted: string;
  change24h: number;
  change24hPercent: number;
}

export const usePortfolio = () => {
  const { address, isAuthenticated } = useAuth();
  const [portfolio, setPortfolio] = useState<PortfolioData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const sdk = EtridSDKService.getInstance();

  const fetchPortfolio = useCallback(async () => {
    if (!address || !isAuthenticated) {
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);

      // Fetch all balances
      const balances = await sdk.accounts.getAllBalances(address);
      const assets: AssetBalance[] = [];
      let totalValue = 0;

      // Process each asset
      for (const [symbol, balance] of Object.entries(balances)) {
        if (parseFloat(balance) > 0) {
          const price = await sdk.oracle.getPrice(`${symbol}/USD`);
          const balanceNum = parseFloat(formatTokenAmount(balance, symbol as any, { showSymbol: false }));
          const usdValue = balanceNum * price;
          const change24h = Math.random() * 10 - 5; // Mock change

          totalValue += usdValue;

          assets.push({
            symbol,
            balance,
            balanceFormatted: formatTokenAmount(balance, symbol as any),
            usdValue,
            usdValueFormatted: formatCurrency(usdValue),
            change24h,
            allocation: 0, // Will calculate after we have total
          });
        }
      }

      // Calculate allocations
      assets.forEach(asset => {
        asset.allocation = (asset.usdValue / totalValue) * 100;
      });

      // Sort by USD value descending
      assets.sort((a, b) => b.usdValue - a.usdValue);

      // Calculate total change
      const totalChange24h = assets.reduce((sum, asset) => sum + asset.change24h, 0);
      const change24hPercent = (totalChange24h / totalValue) * 100;

      setPortfolio({
        assets,
        totalValue,
        totalValueFormatted: formatCurrency(totalValue),
        change24h: totalChange24h,
        change24hPercent,
      });
    } catch (err) {
      console.error('Error fetching portfolio:', err);
      setError(err instanceof Error ? err.message : 'Failed to fetch portfolio');
    } finally {
      setLoading(false);
    }
  }, [address, isAuthenticated]);

  useEffect(() => {
    fetchPortfolio();
  }, [fetchPortfolio]);

  const refresh = async () => {
    await fetchPortfolio();
  };

  return {
    portfolio,
    loading,
    error,
    refresh,
  };
};
