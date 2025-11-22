/**
 * useDAOTreasury Hook
 * Manages DAO treasury state and operations
 */

import { useState, useEffect, useCallback } from 'react';
import {
  Treasury,
  TreasuryAsset,
  TreasuryTransaction,
  TreasuryAnalytics,
  SpendProposal,
} from '@/types/dao';
import { daoTreasuryService } from '@/services/DAOTreasuryService';

export function useDAOTreasury(daoId?: string) {
  const [treasury, setTreasury] = useState<Treasury | null>(null);
  const [assets, setAssets] = useState<TreasuryAsset[]>([]);
  const [transactions, setTransactions] = useState<TreasuryTransaction[]>([]);
  const [analytics, setAnalytics] = useState<TreasuryAnalytics | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load treasury when daoId changes
  useEffect(() => {
    if (daoId) {
      loadTreasury(daoId);
      loadAnalytics(daoId);
    }
  }, [daoId]);

  /**
   * Load treasury data
   */
  const loadTreasury = async (targetDaoId: string) => {
    setIsLoading(true);
    try {
      const loadedTreasury = await daoTreasuryService.getTreasury(targetDaoId);
      setTreasury(loadedTreasury);
      setAssets(loadedTreasury.assets);
      setTransactions(loadedTreasury.transactions);
      setError(null);
    } catch (err: any) {
      console.error('Failed to load treasury:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Load treasury analytics
   */
  const loadAnalytics = async (
    targetDaoId: string,
    period: '24h' | '7d' | '30d' | 'all' = '30d'
  ) => {
    try {
      const loadedAnalytics = await daoTreasuryService.getTreasuryAnalytics(
        targetDaoId,
        period
      );
      setAnalytics(loadedAnalytics);
    } catch (err: any) {
      console.error('Failed to load analytics:', err);
    }
  };

  /**
   * Propose spending from treasury
   */
  const proposeSpend = useCallback(
    async (targetDaoId: string, spend: SpendProposal) => {
      setIsLoading(true);
      try {
        const proposal = await daoTreasuryService.proposeSpend(targetDaoId, spend);
        setError(null);
        return proposal;
      } catch (err: any) {
        console.error('Failed to propose spend:', err);
        setError(err.message);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    []
  );

  /**
   * Get treasury total value
   */
  const getTotalValue = useCallback(() => {
    return treasury?.totalValue || '0';
  }, [treasury]);

  /**
   * Get asset by symbol
   */
  const getAsset = useCallback(
    (symbol: string) => {
      return assets.find((a) => a.symbol === symbol);
    },
    [assets]
  );

  /**
   * Get transactions by type
   */
  const getTransactionsByType = useCallback(
    (type: 'inflow' | 'outflow') => {
      return transactions.filter((tx) => tx.type === type);
    },
    [transactions]
  );

  /**
   * Refresh treasury data
   */
  const refreshTreasury = useCallback(() => {
    if (daoId) {
      return loadTreasury(daoId);
    }
  }, [daoId]);

  /**
   * Refresh analytics with new period
   */
  const refreshAnalytics = useCallback(
    (period: '24h' | '7d' | '30d' | 'all' = '30d') => {
      if (daoId) {
        return loadAnalytics(daoId, period);
      }
    },
    [daoId]
  );

  return {
    treasury,
    assets,
    transactions,
    analytics,
    isLoading,
    error,
    proposeSpend,
    getTotalValue,
    getAsset,
    getTransactionsByType,
    refreshTreasury,
    refreshAnalytics,
    loadTreasury,
  };
}
