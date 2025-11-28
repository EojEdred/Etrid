/**
 * useBillSplit Hook
 * React hook for bill splitting and group expense management
 */

'use client';

import { useState, useCallback, useEffect } from 'react';
import { createBillSplitService } from '@/lib/social/BillSplitService';
import type {
  BillSplit,
  BillSplitInput,
  BillSplitSummary,
  BillSplitStatus,
} from '@/lib/social/types';

export interface UseBillSplitReturn {
  // State
  splits: BillSplit[];
  pendingSplits: BillSplit[];
  completedSplits: BillSplit[];
  owedSplits: BillSplit[];
  receivableSplits: BillSplit[];
  summary: BillSplitSummary | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchSplits: () => Promise<void>;
  createSplit: (input: BillSplitInput) => Promise<BillSplit>;
  payShare: (splitId: string, txHash: string) => Promise<void>;
  remindParticipants: (splitId: string) => Promise<void>;
  cancelSplit: (splitId: string) => Promise<void>;
  getSplit: (splitId: string) => Promise<BillSplit | null>;
  clearError: () => void;
}

export function useBillSplit(userId: string): UseBillSplitReturn {
  const [splits, setSplits] = useState<BillSplit[]>([]);
  const [pendingSplits, setPendingSplits] = useState<BillSplit[]>([]);
  const [completedSplits, setCompletedSplits] = useState<BillSplit[]>([]);
  const [owedSplits, setOwedSplits] = useState<BillSplit[]>([]);
  const [receivableSplits, setReceivableSplits] = useState<BillSplit[]>([]);
  const [summary, setSummary] = useState<BillSplitSummary | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const service = createBillSplitService(userId);

  /**
   * Fetch all bill splits
   */
  const fetchSplits = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const allSplits = await service.getSplits();
      setSplits(allSplits);

      // Categorize splits
      const pending = allSplits.filter(
        (s) => s.status === 'pending' || s.status === 'partial'
      );
      setPendingSplits(pending);

      const completed = allSplits.filter((s) => s.status === 'completed');
      setCompletedSplits(completed);

      // Get owed and receivable splits
      const owed = await service.getOwedSplits();
      setOwedSplits(owed);

      const receivable = await service.getReceivableSplits();
      setReceivableSplits(receivable);

      // Get summary
      const summaryData = await service.getSummary();
      setSummary(summaryData);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to fetch bill splits';
      setError(message);
    } finally {
      setIsLoading(false);
    }
  }, [service]);

  /**
   * Create a new bill split
   */
  const createSplit = useCallback(
    async (input: BillSplitInput): Promise<BillSplit> => {
      setError(null);

      try {
        const newSplit = await service.createSplit(input);
        setSplits((prev) => [newSplit, ...prev]);
        setPendingSplits((prev) => [newSplit, ...prev]);

        // Refresh summary
        const summaryData = await service.getSummary();
        setSummary(summaryData);

        return newSplit;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to create bill split';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Pay your share of a split
   */
  const payShare = useCallback(
    async (splitId: string, txHash: string): Promise<void> => {
      setError(null);

      try {
        const { split } = await service.payShare(splitId, txHash);

        // Update splits
        setSplits((prev) =>
          prev.map((s) => (s.id === splitId ? split : s))
        );

        // Update categorized lists
        if (split.status === 'completed') {
          setPendingSplits((prev) => prev.filter((s) => s.id !== splitId));
          setCompletedSplits((prev) => [...prev, split]);
        } else {
          setPendingSplits((prev) =>
            prev.map((s) => (s.id === splitId ? split : s))
          );
        }

        setOwedSplits((prev) => prev.filter((s) => s.id !== splitId));

        // Refresh summary
        const summaryData = await service.getSummary();
        setSummary(summaryData);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to record payment';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Send reminder to participants
   */
  const remindParticipants = useCallback(
    async (splitId: string): Promise<void> => {
      setError(null);

      try {
        await service.remindParticipants(splitId);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to send reminders';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Cancel a bill split
   */
  const cancelSplit = useCallback(
    async (splitId: string): Promise<void> => {
      setError(null);

      try {
        const cancelled = await service.cancelSplit(splitId);

        setSplits((prev) =>
          prev.map((s) => (s.id === splitId ? cancelled : s))
        );
        setPendingSplits((prev) => prev.filter((s) => s.id !== splitId));
        setOwedSplits((prev) => prev.filter((s) => s.id !== splitId));
        setReceivableSplits((prev) => prev.filter((s) => s.id !== splitId));

        // Refresh summary
        const summaryData = await service.getSummary();
        setSummary(summaryData);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to cancel split';
        setError(message);
        throw err;
      }
    },
    [service]
  );

  /**
   * Get a specific split
   */
  const getSplit = useCallback(
    async (splitId: string): Promise<BillSplit | null> => {
      setError(null);

      try {
        return await service.getSplit(splitId);
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to fetch split';
        setError(message);
        return null;
      }
    },
    [service]
  );

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  // Fetch splits on mount
  useEffect(() => {
    fetchSplits();
  }, [fetchSplits]);

  return {
    splits,
    pendingSplits,
    completedSplits,
    owedSplits,
    receivableSplits,
    summary,
    isLoading,
    error,
    fetchSplits,
    createSplit,
    payShare,
    remindParticipants,
    cancelSplit,
    getSplit,
    clearError,
  };
}
