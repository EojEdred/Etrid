import { useState, useEffect, useCallback } from 'react';
import { useAuth } from '../contexts/AuthContext';
import TransactionService, { Transaction } from '../services/TransactionService';

export const useTransactions = (
  limit: number = 50,
  filter?: Transaction['type']
) => {
  const { address, isAuthenticated } = useAuth();
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [hasMore, setHasMore] = useState(true);
  const [offset, setOffset] = useState(0);

  const fetchTransactions = useCallback(async (reset: boolean = false) => {
    if (!address || !isAuthenticated) {
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);

      const currentOffset = reset ? 0 : offset;
      let txs: Transaction[];

      if (filter) {
        txs = await TransactionService.getTransactionsByType(address, filter, limit);
      } else {
        txs = await TransactionService.getTransactionHistory(address, limit, currentOffset);
      }

      if (reset) {
        setTransactions(txs);
        setOffset(txs.length);
      } else {
        setTransactions(prev => [...prev, ...txs]);
        setOffset(prev => prev + txs.length);
      }

      setHasMore(txs.length === limit);
    } catch (err) {
      console.error('Error fetching transactions:', err);
      setError(err instanceof Error ? err.message : 'Failed to fetch transactions');
    } finally {
      setLoading(false);
    }
  }, [address, limit, filter, offset, isAuthenticated]);

  useEffect(() => {
    fetchTransactions(true);
  }, [address, filter]);

  const refresh = async () => {
    await fetchTransactions(true);
  };

  const loadMore = async () => {
    if (!loading && hasMore) {
      await fetchTransactions(false);
    }
  };

  return {
    transactions,
    loading,
    error,
    hasMore,
    refresh,
    loadMore,
  };
};
