/**
 * useWithdrawal - Hook for managing cash withdrawals
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import { useState, useCallback } from 'react';
import ATMService from '../services/ATMService';
import {
  WithdrawalRequest,
  WithdrawalResponse,
  Withdrawal,
} from '../types/atm.types';

interface UseWithdrawalResult {
  withdrawal: WithdrawalResponse | null;
  loading: boolean;
  error: string | null;
  createWithdrawal: (request: WithdrawalRequest, authToken: string) => Promise<void>;
  cancelWithdrawal: (code: string, authToken: string) => Promise<void>;
  getStatus: (code: string, authToken: string) => Promise<Withdrawal | null>;
  reset: () => void;
}

export const useWithdrawal = (): UseWithdrawalResult => {
  const [withdrawal, setWithdrawal] = useState<WithdrawalResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const createWithdrawal = useCallback(
    async (request: WithdrawalRequest, authToken: string) => {
      setLoading(true);
      setError(null);

      try {
        const response = await ATMService.createWithdrawal(request, authToken);
        setWithdrawal(response);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to create withdrawal';
        setError(errorMessage);
        throw err;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  const cancelWithdrawal = useCallback(
    async (code: string, authToken: string) => {
      setLoading(true);
      setError(null);

      try {
        await ATMService.cancelWithdrawal(code, authToken);
        setWithdrawal(null);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to cancel withdrawal';
        setError(errorMessage);
        throw err;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  const getStatus = useCallback(
    async (code: string, authToken: string): Promise<Withdrawal | null> => {
      setLoading(true);
      setError(null);

      try {
        const status = await ATMService.getWithdrawalStatus(code, authToken);
        return status;
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to get withdrawal status';
        setError(errorMessage);
        return null;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  const reset = useCallback(() => {
    setWithdrawal(null);
    setError(null);
  }, []);

  return {
    withdrawal,
    loading,
    error,
    createWithdrawal,
    cancelWithdrawal,
    getStatus,
    reset,
  };
};
