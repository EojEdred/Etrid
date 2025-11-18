import { useState, useEffect, useCallback } from 'react';
import LightningService from '../services/LightningService';
import KeychainService from '../services/KeychainService';
import {
  LightningChannel,
  OpenChannelRequest,
  LightningPayment,
  LightningStats,
  TransactionResult,
} from '../types/defi.types';

/**
 * useLightning Hook - Manages Lightning-Bloc payment channels
 */
export function useLightning() {
  const [channels, setChannels] = useState<LightningChannel[]>([]);
  const [payments, setPayments] = useState<LightningPayment[]>([]);
  const [stats, setStats] = useState<LightningStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  /**
   * Load channels
   */
  const loadChannels = useCallback(async (silent: boolean = false) => {
    try {
      if (!silent) {
        setLoading(true);
      }
      setError(null);

      const address = await KeychainService.getAddress();
      if (!address) {
        throw new Error('No wallet found');
      }

      const channelsData = await LightningService.getChannels(address);
      setChannels(channelsData);
    } catch (err) {
      console.error('Failed to load channels:', err);
      setError(err instanceof Error ? err.message : 'Failed to load channels');
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Load payments
   */
  const loadPayments = useCallback(async () => {
    try {
      const address = await KeychainService.getAddress();
      if (!address) {
        return;
      }

      const paymentsData = await LightningService.getPaymentHistory(address);
      setPayments(paymentsData);
    } catch (err) {
      console.error('Failed to load payments:', err);
    }
  }, []);

  /**
   * Load stats
   */
  const loadStats = useCallback(async () => {
    try {
      const address = await KeychainService.getAddress();
      if (!address) {
        return;
      }

      const statsData = await LightningService.getStats(address);
      setStats(statsData);
    } catch (err) {
      console.error('Failed to load stats:', err);
    }
  }, []);

  /**
   * Open channel
   */
  const openChannel = useCallback(
    async (request: OpenChannelRequest): Promise<TransactionResult> => {
      try {
        setLoading(true);
        const result = await LightningService.openChannel(request);

        if (result.success) {
          // Reload channels after successful open
          await Promise.all([loadChannels(true), loadStats()]);
        }

        return result;
      } catch (err) {
        console.error('Open channel error:', err);
        return {
          success: false,
          error: err instanceof Error ? err.message : 'Failed to open channel',
        };
      } finally {
        setLoading(false);
      }
    },
    [loadChannels, loadStats]
  );

  /**
   * Close channel
   */
  const closeChannel = useCallback(
    async (channelId: string, force: boolean = false): Promise<TransactionResult> => {
      try {
        setLoading(true);
        const result = await LightningService.closeChannel(channelId, force);

        if (result.success) {
          await Promise.all([loadChannels(true), loadStats()]);
        }

        return result;
      } catch (err) {
        console.error('Close channel error:', err);
        return {
          success: false,
          error: err instanceof Error ? err.message : 'Failed to close channel',
        };
      } finally {
        setLoading(false);
      }
    },
    [loadChannels, loadStats]
  );

  /**
   * Send instant payment
   */
  const sendPayment = useCallback(
    async (channelId: string, recipient: string, amount: string): Promise<TransactionResult> => {
      try {
        setLoading(true);
        const result = await LightningService.sendPayment(channelId, recipient, amount);

        if (result.success) {
          await Promise.all([loadChannels(true), loadPayments(), loadStats()]);
        }

        return result;
      } catch (err) {
        console.error('Send payment error:', err);
        return {
          success: false,
          error: err instanceof Error ? err.message : 'Failed to send payment',
        };
      } finally {
        setLoading(false);
      }
    },
    [loadChannels, loadPayments, loadStats]
  );

  /**
   * Find suitable channel for payment
   */
  const findChannelForPayment = useCallback(
    async (amount: string): Promise<LightningChannel | null> => {
      const address = await KeychainService.getAddress();
      if (!address) {
        return null;
      }

      return await LightningService.findChannelForPayment(address, amount);
    },
    []
  );

  /**
   * Estimate fees
   */
  const estimateOpenChannelFee = useCallback((capacity: string) => {
    return LightningService.estimateOpenChannelFee(capacity);
  }, []);

  const estimatePaymentFee = useCallback((amount: string) => {
    return LightningService.estimatePaymentFee(amount);
  }, []);

  /**
   * Check if has active channels
   */
  const hasActiveChannels = useCallback(async (): Promise<boolean> => {
    const address = await KeychainService.getAddress();
    if (!address) {
      return false;
    }

    return await LightningService.hasActiveChannels(address);
  }, []);

  /**
   * Get active channels
   */
  const getActiveChannels = useCallback(() => {
    return channels.filter(c => c.status === 'active' && c.isActive);
  }, [channels]);

  /**
   * Refresh all data
   */
  const refresh = useCallback(async () => {
    await Promise.all([loadChannels(true), loadPayments(), loadStats()]);
  }, [loadChannels, loadPayments, loadStats]);

  // Load initial data
  useEffect(() => {
    loadChannels();
    loadPayments();
    loadStats();
  }, [loadChannels, loadPayments, loadStats]);

  return {
    channels,
    payments,
    stats,
    loading,
    error,
    refresh,
    openChannel,
    closeChannel,
    sendPayment,
    findChannelForPayment,
    estimateOpenChannelFee,
    estimatePaymentFee,
    hasActiveChannels,
    getActiveChannels,
  };
}
