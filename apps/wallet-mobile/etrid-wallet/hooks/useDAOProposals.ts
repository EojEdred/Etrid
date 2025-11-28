/**
 * useDAOProposals Hook
 * Manages DAO proposals and voting
 */

import { useState, useEffect, useCallback } from 'react';
import { Proposal, ProposalInput, ProposalFilter, VoteType, VoteBreakdown } from '@/types/dao';
import { daoProposalService } from '@/services/DAOProposalService';

export function useDAOProposals(daoId?: string) {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [activeProposals, setActiveProposals] = useState<Proposal[]>([]);
  const [passedProposals, setPassedProposals] = useState<Proposal[]>([]);
  const [rejectedProposals, setRejectedProposals] = useState<Proposal[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load proposals when daoId changes
  useEffect(() => {
    if (daoId) {
      loadProposals(daoId);
    }
  }, [daoId]);

  /**
   * Load proposals for a DAO
   */
  const loadProposals = async (targetDaoId: string, filter?: ProposalFilter) => {
    setIsLoading(true);
    try {
      const loadedProposals = await daoProposalService.getProposals(
        targetDaoId,
        filter
      );

      setProposals(loadedProposals);
      setActiveProposals(loadedProposals.filter((p) => p.status === 'active'));
      setPassedProposals(loadedProposals.filter((p) => p.status === 'passed'));
      setRejectedProposals(loadedProposals.filter((p) => p.status === 'rejected'));
      setError(null);
    } catch (err: any) {
      console.error('Failed to load proposals:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Get single proposal
   */
  const getProposal = useCallback(async (proposalId: string): Promise<Proposal | null> => {
    setIsLoading(true);
    try {
      const proposal = await daoProposalService.getProposal(proposalId);
      setError(null);
      return proposal;
    } catch (err: any) {
      console.error('Failed to get proposal:', err);
      setError(err.message);
      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Create a new proposal
   */
  const createProposal = useCallback(
    async (targetDaoId: string, proposal: ProposalInput) => {
      setIsLoading(true);
      try {
        const newProposal = await daoProposalService.createProposal(
          targetDaoId,
          proposal
        );
        setProposals((prev) => [newProposal, ...prev]);
        setActiveProposals((prev) => [newProposal, ...prev]);
        setError(null);
        return newProposal;
      } catch (err: any) {
        console.error('Failed to create proposal:', err);
        setError(err.message);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    []
  );

  /**
   * Vote on a proposal
   */
  const vote = useCallback(
    async (proposalId: string, voteType: VoteType, reason?: string) => {
      setIsLoading(true);
      try {
        await daoProposalService.vote(proposalId, voteType, reason);

        // Update the proposal in state
        const updatedProposal = await daoProposalService.getProposal(proposalId);
        if (updatedProposal) {
          setProposals((prev) =>
            prev.map((p) => (p.id === proposalId ? updatedProposal : p))
          );
          setActiveProposals((prev) =>
            prev.map((p) => (p.id === proposalId ? updatedProposal : p))
          );
        }

        setError(null);
      } catch (err: any) {
        console.error('Failed to vote:', err);
        setError(err.message);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    []
  );

  /**
   * Execute a passed proposal
   */
  const executeProposal = useCallback(async (proposalId: string) => {
    setIsLoading(true);
    try {
      await daoProposalService.executeProposal(proposalId);

      // Update proposal status
      const updatedProposal = await daoProposalService.getProposal(proposalId);
      if (updatedProposal) {
        setProposals((prev) =>
          prev.map((p) => (p.id === proposalId ? updatedProposal : p))
        );
        setPassedProposals((prev) =>
          prev.map((p) => (p.id === proposalId ? updatedProposal : p))
        );
      }

      setError(null);
    } catch (err: any) {
      console.error('Failed to execute proposal:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Cancel a proposal
   */
  const cancelProposal = useCallback(async (proposalId: string) => {
    setIsLoading(true);
    try {
      await daoProposalService.cancelProposal(proposalId);

      // Remove from active proposals
      setProposals((prev) =>
        prev.map((p) =>
          p.id === proposalId ? { ...p, status: 'cancelled' as const } : p
        )
      );
      setActiveProposals((prev) => prev.filter((p) => p.id !== proposalId));

      setError(null);
    } catch (err: any) {
      console.error('Failed to cancel proposal:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Get vote breakdown for a proposal
   */
  const getVoteBreakdown = useCallback(
    async (proposalId: string): Promise<VoteBreakdown | null> => {
      try {
        const breakdown = await daoProposalService.getVoteBreakdown(proposalId);
        return breakdown;
      } catch (err: any) {
        console.error('Failed to get vote breakdown:', err);
        return null;
      }
    },
    []
  );

  /**
   * Check if user has voted on a proposal
   */
  const hasVoted = useCallback(
    async (proposalId: string, userId: string): Promise<boolean> => {
      try {
        const vote = await daoProposalService.getUserVote(proposalId, userId);
        return vote !== null;
      } catch (err: any) {
        console.error('Failed to check vote status:', err);
        return false;
      }
    },
    []
  );

  /**
   * Get user's vote on a proposal
   */
  const getUserVote = useCallback(
    async (proposalId: string, userId: string) => {
      try {
        return await daoProposalService.getUserVote(proposalId, userId);
      } catch (err: any) {
        console.error('Failed to get user vote:', err);
        return null;
      }
    },
    []
  );

  /**
   * Refresh proposals
   */
  const refreshProposals = useCallback(() => {
    if (daoId) {
      return loadProposals(daoId);
    }
  }, [daoId]);

  return {
    proposals,
    activeProposals,
    passedProposals,
    rejectedProposals,
    isLoading,
    error,
    createProposal,
    vote,
    executeProposal,
    cancelProposal,
    getProposal,
    getVoteBreakdown,
    hasVoted,
    getUserVote,
    refreshProposals,
    loadProposals,
  };
}
