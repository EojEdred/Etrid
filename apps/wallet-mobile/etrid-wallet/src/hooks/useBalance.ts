import { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../contexts/AuthContext';
import EtridSDKService from '../services/EtridSDKService';
import { formatTokenAmount } from '../utils/formatters';

export interface BalanceData {
  balance: string;
  balanceFormatted: string;
  usdValue: number;
  usdValueFormatted: string;
  change24h: number;
  change24hPercent: number;
}

export const useBalance = (currency: string = 'ETR') => {
  const { address, isAuthenticated } = useAuth();
  const [balance, setBalance] = useState<BalanceData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const sdk = EtridSDKService.getInstance();

  const fetchBalance = useCallback(async () => {
    if (!address || !isAuthenticated) {
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);

      // Fetch balance from blockchain
      const rawBalance = await sdk.accounts.getBalance(address);

      // Fetch USD price
      const price = await sdk.oracle.getPrice(`${currency}/USD`);

      // Calculate USD value
      const balanceNum = parseFloat(formatTokenAmount(rawBalance, currency as any, { showSymbol: false }));
      const usdValue = balanceNum * price;

      // Mock 24h change (in production, fetch from price history)
      const change24h = Math.random() * 10 - 5; // Random between -5 and +5
      const change24hPercent = (change24h / usdValue) * 100;

      setBalance({
        balance: rawBalance,
        balanceFormatted: formatTokenAmount(rawBalance, currency as any),
        usdValue,
        usdValueFormatted: `$${usdValue.toFixed(2)}`,
        change24h,
        change24hPercent,
      });
    } catch (err) {
      console.error('Error fetching balance:', err);
      setError(err instanceof Error ? err.message : 'Failed to fetch balance');
    } finally {
      setLoading(false);
    }
  }, [address, currency, isAuthenticated]);

  useEffect(() => {
    fetchBalance();
  }, [fetchBalance]);

  const refresh = async () => {
    await fetchBalance();
  };

  return {
    balance,
    loading,
    error,
    refresh,
  };
};
