import { useState, useEffect, useCallback } from 'react';
import ETHPBCService, { PrecompileAddress, PrecompileCall, ETHBalance } from '../services/ETHPBCService';
import CacheManager, { CacheKeys, CacheTTLs } from '../utils/CacheManager';
import PerformanceMonitor from '../utils/PerformanceMonitor';
import AnalyticsService from '../services/AnalyticsService';

/**
 * Hook for ETH PBC (Ethereum L2) operations
 */
export function useETHPBC() {
  const [balance, setBalance] = useState<ETHBalance | null>(null);
  const [precompiles, setPrecompiles] = useState<PrecompileCall[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Load ETH balance
   */
  const loadBalance = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const balanceData = await CacheManager.getOrFetch(
        CacheKeys.ETH_PBC_BALANCE,
        () => ETHPBCService.getETHBalance(),
        CacheTTLs.ETH_PBC_BALANCE
      );

      setBalance(balanceData);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to load balance';
      setError(message);
      AnalyticsService.trackError('eth_pbc_balance_failed', message);
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Wrap ETH
   */
  const wrapETH = useCallback(async (amount: string): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      await PerformanceMonitor.measure(
        'wrap_eth',
        () => ETHPBCService.wrapETH(amount),
        { amount }
      );

      // Invalidate cache
      await CacheManager.remove(CacheKeys.ETH_PBC_BALANCE);

      // Refresh balance
      await loadBalance();

      AnalyticsService.trackTransaction('swap', amount, 'ETH', true, { type: 'wrap' });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to wrap ETH';
      setError(message);
      AnalyticsService.trackTransaction('swap', amount, 'ETH', false, { type: 'wrap', error: message });
      return false;
    } finally {
      setLoading(false);
    }
  }, [loadBalance]);

  /**
   * Unwrap wETH
   */
  const unwrapETH = useCallback(async (amount: string): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      await PerformanceMonitor.measure(
        'unwrap_eth',
        () => ETHPBCService.unwrapETH(amount),
        { amount }
      );

      // Invalidate cache
      await CacheManager.remove(CacheKeys.ETH_PBC_BALANCE);

      // Refresh balance
      await loadBalance();

      AnalyticsService.trackTransaction('swap', amount, 'wETH', true, { type: 'unwrap' });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to unwrap ETH';
      setError(message);
      AnalyticsService.trackTransaction('swap', amount, 'wETH', false, { type: 'unwrap', error: message });
      return false;
    } finally {
      setLoading(false);
    }
  }, [loadBalance]);

  /**
   * Get oracle price
   */
  const getOraclePrice = useCallback(async (asset: string): Promise<string | null> => {
    try {
      const price = await ETHPBCService.getOraclePrice(asset);
      AnalyticsService.trackFeatureUsage('eth_pbc', 'oracle_price', { asset });
      return price;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to get oracle price';
      setError(message);
      AnalyticsService.trackError('oracle_price_failed', message, undefined, { asset });
      return null;
    }
  }, []);

  /**
   * Vote on proposal (via precompile)
   */
  const voteOnProposal = useCallback(async (proposalId: number, vote: boolean): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      await ETHPBCService.voteOnProposal(proposalId, vote);
      AnalyticsService.trackGovernance('vote_cast', proposalId, { vote, via: 'precompile' });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to vote on proposal';
      setError(message);
      AnalyticsService.trackError('precompile_vote_failed', message, undefined, { proposalId });
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Stake tokens (via precompile)
   */
  const stakeTokens = useCallback(async (amount: string): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      await ETHPBCService.stakeTokens(amount);
      AnalyticsService.trackTransaction('stake', amount, 'ÉTR', true, { via: 'precompile' });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to stake tokens';
      setError(message);
      AnalyticsService.trackTransaction('stake', amount, 'ÉTR', false, { via: 'precompile', error: message });
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Bridge to chain (via precompile)
   */
  const bridgeToChain = useCallback(async (targetChain: string, amount: string): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      await ETHPBCService.bridgeToChain(targetChain, amount);
      AnalyticsService.trackBridge('bridge_via_precompile', amount, { targetChain });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to bridge to chain';
      setError(message);
      AnalyticsService.trackError('precompile_bridge_failed', message, undefined, { targetChain });
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Query token (via precompile)
   */
  const queryToken = useCallback(async (tokenAddress: string): Promise<any> => {
    try {
      const result = await ETHPBCService.queryToken(tokenAddress);
      AnalyticsService.trackFeatureUsage('eth_pbc', 'query_token', { tokenAddress });
      return result;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to query token';
      setError(message);
      AnalyticsService.trackError('query_token_failed', message, undefined, { tokenAddress });
      return null;
    }
  }, []);

  /**
   * Verify state proof (via precompile)
   */
  const verifyStateProof = useCallback(async (blockHash: string, proof: string): Promise<boolean> => {
    try {
      const verified = await ETHPBCService.verifyStateProof(blockHash, proof);
      AnalyticsService.trackFeatureUsage('eth_pbc', 'verify_state_proof', { verified });
      return verified;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to verify state proof';
      setError(message);
      AnalyticsService.trackError('verify_proof_failed', message);
      return false;
    }
  }, []);

  /**
   * Generic precompile call
   */
  const callPrecompile = useCallback(
    async (address: PrecompileAddress, parameters: any[]): Promise<any> => {
      setLoading(true);
      setError(null);

      try {
        const result = await ETHPBCService.callPrecompile(address, parameters);
        AnalyticsService.trackFeatureUsage('eth_pbc', 'call_precompile', { address });
        return result;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to call precompile';
        setError(message);
        AnalyticsService.trackError('precompile_call_failed', message, undefined, { address });
        return null;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  /**
   * Get available precompiles
   */
  const loadPrecompiles = useCallback(() => {
    const available = ETHPBCService.getPrecompiles();
    setPrecompiles(available);
  }, []);

  /**
   * Get precompile details
   */
  const getPrecompile = useCallback((address: PrecompileAddress): PrecompileCall => {
    return ETHPBCService.getPrecompile(address);
  }, []);

  /**
   * Refresh data
   */
  const refresh = useCallback(async () => {
    await CacheManager.remove(CacheKeys.ETH_PBC_BALANCE);
    await loadBalance();
  }, [loadBalance]);

  // Load initial data
  useEffect(() => {
    loadBalance();
    loadPrecompiles();
  }, []);

  return {
    balance,
    precompiles,
    loading,
    error,
    wrapETH,
    unwrapETH,
    getOraclePrice,
    voteOnProposal,
    stakeTokens,
    bridgeToChain,
    queryToken,
    verifyStateProof,
    callPrecompile,
    getPrecompile,
    refresh,
  };
}
