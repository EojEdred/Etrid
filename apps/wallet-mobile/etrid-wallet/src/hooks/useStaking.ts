import { useState, useEffect, useCallback } from 'react';
import StakingService from '../services/StakingService';
import KeychainService from '../services/KeychainService';
import { StakingInfo, StakeOptions, TransactionResult } from '../types/defi.types';

/**
 * useStaking Hook - Manages staking state and operations
 */
export function useStaking(etrPrice: number = 2.45) {
  const [stakingInfo, setStakingInfo] = useState<StakingInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [refreshing, setRefreshing] = useState(false);

  /**
   * Load staking info
   */
  const loadStakingInfo = useCallback(async (silent: boolean = false) => {
    try {
      if (!silent) {
        setLoading(true);
      }
      setError(null);

      const address = await KeychainService.getAddress();
      if (!address) {
        throw new Error('No wallet found');
      }

      const info = await StakingService.getStakingInfo(address, etrPrice);
      setStakingInfo(info);
    } catch (err) {
      console.error('Failed to load staking info:', err);
      setError(err instanceof Error ? err.message : 'Failed to load staking info');
    } finally {
      setLoading(false);
      setRefreshing(false);
    }
  }, [etrPrice]);

  /**
   * Refresh staking info
   */
  const refresh = useCallback(async () => {
    setRefreshing(true);
    await loadStakingInfo(true);
  }, [loadStakingInfo]);

  /**
   * Stake tokens
   */
  const stake = useCallback(
    async (options: StakeOptions): Promise<TransactionResult> => {
      try {
        setLoading(true);
        const result = await StakingService.stake(options);

        if (result.success) {
          // Reload staking info after successful stake
          await loadStakingInfo(true);
        }

        return result;
      } catch (err) {
        console.error('Stake error:', err);
        return {
          success: false,
          error: err instanceof Error ? err.message : 'Failed to stake',
        };
      } finally {
        setLoading(false);
      }
    },
    [loadStakingInfo]
  );

  /**
   * Unstake tokens
   */
  const unstake = useCallback(
    async (amount: string): Promise<TransactionResult> => {
      try {
        setLoading(true);
        const result = await StakingService.unstake(amount);

        if (result.success) {
          // Reload staking info after successful unstake
          await loadStakingInfo(true);
        }

        return result;
      } catch (err) {
        console.error('Unstake error:', err);
        return {
          success: false,
          error: err instanceof Error ? err.message : 'Failed to unstake',
        };
      } finally {
        setLoading(false);
      }
    },
    [loadStakingInfo]
  );

  /**
   * Withdraw unbonded tokens
   */
  const withdrawUnbonded = useCallback(async (): Promise<TransactionResult> => {
    try {
      setLoading(true);
      const result = await StakingService.withdrawUnbonded();

      if (result.success) {
        await loadStakingInfo(true);
      }

      return result;
    } catch (err) {
      console.error('Withdraw error:', err);
      return {
        success: false,
        error: err instanceof Error ? err.message : 'Failed to withdraw',
      };
    } finally {
      setLoading(false);
    }
  }, [loadStakingInfo]);

  /**
   * Claim rewards
   */
  const claimRewards = useCallback(async (): Promise<TransactionResult> => {
    try {
      setLoading(true);
      const result = await StakingService.claimRewards();

      if (result.success) {
        await loadStakingInfo(true);
      }

      return result;
    } catch (err) {
      console.error('Claim rewards error:', err);
      return {
        success: false,
        error: err instanceof Error ? err.message : 'Failed to claim rewards',
      };
    } finally {
      setLoading(false);
    }
  }, [loadStakingInfo]);

  /**
   * Estimate rewards
   */
  const estimateRewards = useCallback((amount: number, apy?: number) => {
    return StakingService.estimateRewards(amount, apy);
  }, []);

  // Load staking info on mount
  useEffect(() => {
    loadStakingInfo();
  }, [loadStakingInfo]);

  return {
    stakingInfo,
    loading,
    error,
    refreshing,
    refresh,
    stake,
    unstake,
    withdrawUnbonded,
    claimRewards,
    estimateRewards,
  };
}
