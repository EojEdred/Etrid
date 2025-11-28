/**
 * useSocialRecovery Hook
 * React hook for social recovery and guardian management
 */

'use client';

import { useState, useCallback, useEffect } from 'react';
import { createSocialRecoveryService } from '@/lib/social/SocialRecoveryService';
import type {
  Guardian,
  SocialRecoveryConfig,
  RecoveryProcess,
} from '@/lib/social/types';

export interface UseSocialRecoveryReturn {
  // State
  guardians: Guardian[];
  activeGuardians: Guardian[];
  pendingGuardians: Guardian[];
  config: SocialRecoveryConfig | null;
  activeRecovery: RecoveryProcess | null;
  pendingApprovals: RecoveryProcess[];
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchGuardians: () => Promise<void>;
  addGuardian: (address: string, username?: string) => Promise<Guardian>;
  removeGuardian: (guardianId: string) => Promise<void>;
  setThreshold: (threshold: number) => Promise<void>;
  resendInvitation: (guardianId: string) => Promise<void>;
  initiateRecovery: (newDeviceId: string, newAddress?: string) => Promise<RecoveryProcess>;
  approveRecovery: (recoveryId: string, signature: string) => Promise<void>;
  cancelRecovery: (recoveryId: string) => Promise<void>;
  getRecommendedThreshold: (guardianCount: number) => number;
  clearError: () => void;
}

export function useSocialRecovery(walletAddress: string): UseSocialRecoveryReturn {
  const [guardians, setGuardians] = useState<Guardian[]>([]);
  const [activeGuardians, setActiveGuardians] = useState<Guardian[]>([]);
  const [pendingGuardians, setPendingGuardians] = useState<Guardian[]>([]);
  const [config, setConfig] = useState<SocialRecoveryConfig | null>(null);
  const [activeRecovery, setActiveRecovery] = useState<RecoveryProcess | null>(null);
  const [pendingApprovals, setPendingApprovals] = useState<RecoveryProcess[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const service = createSocialRecoveryService(walletAddress);

  /**
   * Fetch all guardians and config
   */
  const fetchGuardians = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const allGuardians = await service.getGuardians();
      setGuardians(allGuardians);

      // Categorize guardians
      const active = allGuardians.filter((g) => g.status === 'active');
      setActiveGuardians(active);

      const pending = allGuardians.filter((g) => g.status === 'pending');
      setPendingGuardians(pending);

      // Fetch config
      const recoveryConfig = await service.getRecoveryConfig();
      setConfig(recoveryConfig);

      // Fetch active recovery
      const recovery = await service.getActiveRecovery();
      setActiveRecovery(recovery);

      // Fetch pending approvals (where current wallet is a guardian)
      const approvals = await service.getPendingApprovals();
      setPendingApprovals(approvals);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to fetch guardians';
      setError(message);
    } finally {
      setIsLoading(false);
    }
  }, [service]);

  /**
   * Add a new guardian
   */
  const addGuardian = useCallback(
    async (address: string, username?: string): Promise<Guardian> => {
      setError(null);

      try {
        const newGuardian = await service.addGuardian(address, username);
        setGuardians((prev) => [...prev, newGuardian]);
        setPendingGuardians((prev) => [...prev, newGuardian]);

        // Refresh config
        const recoveryConfig = await service.getRecoveryConfig();
        setConfig(recoveryConfig);

        return newGuardian;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to add guardian';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Remove a guardian
   */
  const removeGuardian = useCallback(
    async (guardianId: string): Promise<void> => {
      setError(null);

      try {
        await service.removeGuardian(guardianId);

        setGuardians((prev) => prev.filter((g) => g.id !== guardianId));
        setActiveGuardians((prev) => prev.filter((g) => g.id !== guardianId));
        setPendingGuardians((prev) => prev.filter((g) => g.id !== guardianId));

        // Refresh config
        const recoveryConfig = await service.getRecoveryConfig();
        setConfig(recoveryConfig);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to remove guardian';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Set recovery threshold
   */
  const setThreshold = useCallback(
    async (threshold: number): Promise<void> => {
      setError(null);

      try {
        await service.setThreshold(threshold);

        // Refresh config
        const recoveryConfig = await service.getRecoveryConfig();
        setConfig(recoveryConfig);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to set threshold';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Resend invitation to a pending guardian
   */
  const resendInvitation = useCallback(
    async (guardianId: string): Promise<void> => {
      setError(null);

      try {
        await service.resendInvitation(guardianId);

        // Update last reminder time
        setGuardians((prev) =>
          prev.map((g) =>
            g.id === guardianId ? { ...g, lastReminderAt: new Date() } : g
          )
        );
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to resend invitation';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Initiate a recovery process
   */
  const initiateRecovery = useCallback(
    async (newDeviceId: string, newAddress?: string): Promise<RecoveryProcess> => {
      setError(null);

      try {
        const recovery = await service.initiateRecovery(newDeviceId, newAddress);
        setActiveRecovery(recovery);
        return recovery;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to initiate recovery';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Approve a recovery process (as a guardian)
   */
  const approveRecovery = useCallback(
    async (recoveryId: string, signature: string): Promise<void> => {
      setError(null);

      try {
        const updatedRecovery = await service.approveRecovery(recoveryId, signature);

        // Update active recovery if it's the same one
        if (activeRecovery?.id === recoveryId) {
          setActiveRecovery(updatedRecovery);
        }

        // Remove from pending approvals
        setPendingApprovals((prev) => prev.filter((r) => r.id !== recoveryId));
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to approve recovery';
        setError(message);
        throw err;
      }
    },
    [service, activeRecovery]
  );

  /**
   * Cancel an active recovery
   */
  const cancelRecovery = useCallback(
    async (recoveryId: string): Promise<void> => {
      setError(null);

      try {
        await service.cancelRecovery(recoveryId);
        setActiveRecovery(null);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to cancel recovery';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Get recommended threshold for number of guardians
   */
  const getRecommendedThreshold = useCallback(
    (guardianCount: number): number => {
      return service.getRecommendedThreshold(guardianCount);
    },
    [service]
  );

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  // Fetch guardians on mount
  useEffect(() => {
    fetchGuardians();
  }, [fetchGuardians]);

  return {
    guardians,
    activeGuardians,
    pendingGuardians,
    config,
    activeRecovery,
    pendingApprovals,
    isLoading,
    error,
    fetchGuardians,
    addGuardian,
    removeGuardian,
    setThreshold,
    resendInvitation,
    initiateRecovery,
    approveRecovery,
    cancelRecovery,
    getRecommendedThreshold,
    clearError,
  };
}
