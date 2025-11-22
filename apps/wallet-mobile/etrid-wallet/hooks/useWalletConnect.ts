/**
 * useWalletConnect Hook
 * Manages WalletConnect sessions and proposals
 */

import { useState, useEffect, useCallback } from 'react';
import { WalletConnectSession, WalletConnectProposal } from '@/types/dapp';
import { walletConnectService } from '@/services/WalletConnectService';

export function useWalletConnect() {
  const [sessions, setSessions] = useState<WalletConnectSession[]>([]);
  const [proposals, setProposals] = useState<WalletConnectProposal[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load sessions on mount
  useEffect(() => {
    loadSessions();
    loadProposals();
    initializeWalletConnect();
  }, []);

  /**
   * Initialize WalletConnect service
   */
  const initializeWalletConnect = async () => {
    try {
      await walletConnectService.initialize();
    } catch (err: any) {
      console.error('Failed to initialize WalletConnect:', err);
      setError(err.message);
    }
  };

  /**
   * Load all sessions
   */
  const loadSessions = async () => {
    setIsLoading(true);
    try {
      const loadedSessions = await walletConnectService.getSessions();
      setSessions(loadedSessions);
      setError(null);
    } catch (err: any) {
      console.error('Failed to load sessions:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Load pending proposals
   */
  const loadProposals = async () => {
    try {
      const loadedProposals = await walletConnectService.getProposals();
      setProposals(loadedProposals);
    } catch (err: any) {
      console.error('Failed to load proposals:', err);
    }
  };

  /**
   * Pair with dApp using URI
   */
  const pair = useCallback(async (uri: string) => {
    setIsLoading(true);
    setError(null);
    try {
      await walletConnectService.pair(uri);
      await loadProposals();
    } catch (err: any) {
      console.error('Failed to pair:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Approve a session proposal
   */
  const approveSession = useCallback(async (proposalId: string) => {
    setIsLoading(true);
    setError(null);
    try {
      const session = await walletConnectService.approveSession(proposalId);
      setSessions((prev) => [...prev, session]);
      setProposals((prev) => prev.filter((p) => p.id !== proposalId));
      return session;
    } catch (err: any) {
      console.error('Failed to approve session:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Reject a session proposal
   */
  const rejectSession = useCallback(async (proposalId: string) => {
    setIsLoading(true);
    setError(null);
    try {
      await walletConnectService.rejectSession(proposalId);
      setProposals((prev) => prev.filter((p) => p.id !== proposalId));
    } catch (err: any) {
      console.error('Failed to reject session:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Disconnect a session
   */
  const disconnect = useCallback(async (sessionId: string) => {
    setIsLoading(true);
    setError(null);
    try {
      await walletConnectService.disconnect(sessionId);
      setSessions((prev) => prev.filter((s) => s.id !== sessionId));
    } catch (err: any) {
      console.error('Failed to disconnect:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Get session by ID
   */
  const getSession = useCallback(
    (sessionId: string) => {
      return sessions.find((s) => s.id === sessionId);
    },
    [sessions]
  );

  return {
    sessions,
    proposals,
    isLoading,
    error,
    pair,
    approveSession,
    rejectSession,
    disconnect,
    getSession,
    refreshSessions: loadSessions,
    refreshProposals: loadProposals,
  };
}
