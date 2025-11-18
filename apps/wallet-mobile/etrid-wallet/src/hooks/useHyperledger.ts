import { useState, useEffect, useCallback } from 'react';
import HyperledgerService, { FabricNetwork, BridgeTransaction } from '../services/HyperledgerService';
import CacheManager, { CacheKeys, CacheTTLs } from '../utils/CacheManager';
import PerformanceMonitor from '../utils/PerformanceMonitor';
import AnalyticsService from '../services/AnalyticsService';

/**
 * Hook for Hyperledger Fabric bridge operations
 */
export function useHyperledger() {
  const [networks, setNetworks] = useState<FabricNetwork[]>([]);
  const [bridgeHistory, setBridgeHistory] = useState<BridgeTransaction[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Connect to Fabric network
   */
  const connectToNetwork = useCallback(
    async (
      networkName: string,
      channel: string,
      organization: string,
      credentials: { certPem: string; keyPem: string }
    ): Promise<boolean> => {
      setLoading(true);
      setError(null);

      try {
        const network = await PerformanceMonitor.measure(
          'connect_fabric_network',
          () => HyperledgerService.connectToFabricNetwork(networkName, channel, organization, credentials),
          { networkName, channel }
        );

        setNetworks((prev) => [...prev, network]);
        AnalyticsService.trackBridge('connect_network', undefined, { networkName });
        return true;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to connect to network';
        setError(message);
        AnalyticsService.trackError('fabric_connect_failed', message, undefined, { networkName });
        return false;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  /**
   * Bridge to Fabric
   */
  const bridgeToFabric = useCallback(
    async (amount: string, networkId: string, asset: string = 'EDSC'): Promise<BridgeTransaction | null> => {
      setLoading(true);
      setError(null);

      try {
        const transaction = await PerformanceMonitor.measure(
          'bridge_to_fabric',
          () => HyperledgerService.bridgeToFabric(amount, networkId, asset),
          { amount, networkId }
        );

        // Invalidate cache
        await CacheManager.invalidatePattern('bridge_');

        // Refresh history
        await loadBridgeHistory();

        AnalyticsService.trackBridge('bridge_to_fabric', amount, { networkId, txId: transaction.txId });
        return transaction;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to bridge to Fabric';
        setError(message);
        AnalyticsService.trackError('bridge_to_fabric_failed', message, undefined, { amount, networkId });
        return null;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  /**
   * Bridge from Fabric
   */
  const bridgeFromFabric = useCallback(
    async (
      fabricTxId: string,
      amount: string,
      networkId: string,
      asset: string = 'EDSC'
    ): Promise<BridgeTransaction | null> => {
      setLoading(true);
      setError(null);

      try {
        const transaction = await PerformanceMonitor.measure(
          'bridge_from_fabric',
          () => HyperledgerService.bridgeFromFabric(fabricTxId, amount, networkId, asset),
          { fabricTxId, networkId }
        );

        // Invalidate cache
        await CacheManager.invalidatePattern('bridge_');

        // Refresh history
        await loadBridgeHistory();

        AnalyticsService.trackBridge('bridge_from_fabric', amount, { networkId, txId: fabricTxId });
        return transaction;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to bridge from Fabric';
        setError(message);
        AnalyticsService.trackError('bridge_from_fabric_failed', message, undefined, { fabricTxId, networkId });
        return null;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  /**
   * Load bridge history
   */
  const loadBridgeHistory = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const history = await CacheManager.getOrFetch(
        CacheKeys.BRIDGE_HISTORY,
        () => HyperledgerService.getBridgeHistory(),
        CacheTTLs.BRIDGE_HISTORY
      );

      setBridgeHistory(history);
      AnalyticsService.trackBridge('view_history');
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to load bridge history';
      setError(message);
      AnalyticsService.trackError('bridge_history_failed', message);
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Get transaction status
   */
  const getTransactionStatus = useCallback(
    async (txId: string): Promise<BridgeTransaction | null> => {
      try {
        return await HyperledgerService.getBridgeTransactionStatus(txId);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to get transaction status';
        setError(message);
        return null;
      }
    },
    []
  );

  /**
   * Verify Fabric transaction
   */
  const verifyFabricTransaction = useCallback(async (txId: string): Promise<boolean> => {
    try {
      return await HyperledgerService.verifyFabricTransaction(txId);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to verify transaction';
      setError(message);
      return false;
    }
  }, []);

  /**
   * Get connected networks
   */
  const loadNetworks = useCallback(() => {
    const connectedNetworks = HyperledgerService.getConnectedNetworks();
    setNetworks(connectedNetworks);
  }, []);

  /**
   * Disconnect from network
   */
  const disconnectNetwork = useCallback((networkId: string) => {
    HyperledgerService.disconnectFromNetwork(networkId);
    setNetworks((prev) => prev.filter((n) => n.id !== networkId));
  }, []);

  /**
   * Refresh data
   */
  const refresh = useCallback(async () => {
    await CacheManager.invalidatePattern('bridge_');
    await loadBridgeHistory();
    loadNetworks();
  }, [loadBridgeHistory, loadNetworks]);

  // Load initial data
  useEffect(() => {
    loadBridgeHistory();
    loadNetworks();
  }, []);

  return {
    networks,
    bridgeHistory,
    loading,
    error,
    connectToNetwork,
    bridgeToFabric,
    bridgeFromFabric,
    getTransactionStatus,
    verifyFabricTransaction,
    disconnectNetwork,
    refresh,
  };
}
