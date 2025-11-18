import { useState, useEffect, useCallback } from 'react';
import GovernanceService from '../services/GovernanceService';
import KeychainService from '../services/KeychainService';
import {
  Proposal,
  VoteChoice,
  VotingPower,
  VoteHistory,
  Delegation,
  ConvictionLevel,
  TransactionResult,
} from '../types/defi.types';

/**
 * useGovernance Hook - Manages governance proposals and voting
 */
export function useGovernance() {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [votingPower, setVotingPower] = useState<VotingPower | null>(null);
  const [voteHistory, setVoteHistory] = useState<VoteHistory[]>([]);
  const [delegations, setDelegations] = useState<Delegation[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  /**
   * Load active proposals
   */
  const loadProposals = useCallback(async (silent: boolean = false) => {
    try {
      if (!silent) {
        setLoading(true);
      }
      setError(null);

      const address = await KeychainService.getAddress();
      const proposalsData = await GovernanceService.getActiveProposals(address || undefined);
      setProposals(proposalsData);
    } catch (err) {
      console.error('Failed to load proposals:', err);
      setError(err instanceof Error ? err.message : 'Failed to load proposals');
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Get specific proposal
   */
  const getProposal = useCallback(async (proposalId: number): Promise<Proposal | null> => {
    try {
      const address = await KeychainService.getAddress();
      return await GovernanceService.getProposal(proposalId, address || undefined);
    } catch (err) {
      console.error('Failed to get proposal:', err);
      return null;
    }
  }, []);

  /**
   * Load voting power
   */
  const loadVotingPower = useCallback(async () => {
    try {
      const address = await KeychainService.getAddress();
      if (!address) {
        return;
      }

      const power = await GovernanceService.getVotingPower(address);
      setVotingPower(power);
    } catch (err) {
      console.error('Failed to load voting power:', err);
    }
  }, []);

  /**
   * Load vote history
   */
  const loadVoteHistory = useCallback(async () => {
    try {
      const address = await KeychainService.getAddress();
      if (!address) {
        return;
      }

      const history = await GovernanceService.getVoteHistory(address);
      setVoteHistory(history);
    } catch (err) {
      console.error('Failed to load vote history:', err);
    }
  }, []);

  /**
   * Load delegations
   */
  const loadDelegations = useCallback(async () => {
    try {
      const address = await KeychainService.getAddress();
      if (!address) {
        return;
      }

      const dels = await GovernanceService.getDelegations(address);
      setDelegations(dels);
    } catch (err) {
      console.error('Failed to load delegations:', err);
    }
  }, []);

  /**
   * Vote on proposal
   */
  const vote = useCallback(
    async (voteChoice: VoteChoice): Promise<TransactionResult> => {
      try {
        setLoading(true);
        const result = await GovernanceService.vote(voteChoice);

        if (result.success) {
          // Reload proposals and voting power
          await Promise.all([loadProposals(true), loadVotingPower(), loadVoteHistory()]);
        }

        return result;
      } catch (err) {
        console.error('Vote error:', err);
        return {
          success: false,
          error: err instanceof Error ? err.message : 'Failed to vote',
        };
      } finally {
        setLoading(false);
      }
    },
    [loadProposals, loadVotingPower, loadVoteHistory]
  );

  /**
   * Delegate votes
   */
  const delegate = useCallback(
    async (
      delegateTo: string,
      amount: string,
      conviction: ConvictionLevel
    ): Promise<TransactionResult> => {
      try {
        setLoading(true);
        const result = await GovernanceService.delegateVotes(delegateTo, amount, conviction);

        if (result.success) {
          await Promise.all([loadVotingPower(), loadDelegations()]);
        }

        return result;
      } catch (err) {
        console.error('Delegate error:', err);
        return {
          success: false,
          error: err instanceof Error ? err.message : 'Failed to delegate',
        };
      } finally {
        setLoading(false);
      }
    },
    [loadVotingPower, loadDelegations]
  );

  /**
   * Undelegate votes
   */
  const undelegate = useCallback(async (): Promise<TransactionResult> => {
    try {
      setLoading(true);
      const result = await GovernanceService.undelegateVotes();

      if (result.success) {
        await Promise.all([loadVotingPower(), loadDelegations()]);
      }

      return result;
    } catch (err) {
      console.error('Undelegate error:', err);
      return {
        success: false,
        error: err instanceof Error ? err.message : 'Failed to undelegate',
      };
    } finally {
      setLoading(false);
    }
  }, [loadVotingPower, loadDelegations]);

  /**
   * Calculate voting power with conviction
   */
  const calculateVotingPower = useCallback(
    (balance: number, conviction: ConvictionLevel): number => {
      return GovernanceService.calculateVotingPowerWithConviction(balance, conviction);
    },
    []
  );

  /**
   * Get conviction info
   */
  const getConvictionInfo = useCallback((level: ConvictionLevel) => {
    return GovernanceService.getConvictionInfo(level);
  }, []);

  /**
   * Refresh all data
   */
  const refresh = useCallback(async () => {
    await Promise.all([
      loadProposals(true),
      loadVotingPower(),
      loadVoteHistory(),
      loadDelegations(),
    ]);
  }, [loadProposals, loadVotingPower, loadVoteHistory, loadDelegations]);

  // Load initial data
  useEffect(() => {
    loadProposals();
    loadVotingPower();
    loadVoteHistory();
    loadDelegations();
  }, [loadProposals, loadVotingPower, loadVoteHistory, loadDelegations]);

  return {
    proposals,
    votingPower,
    voteHistory,
    delegations,
    loading,
    error,
    refresh,
    getProposal,
    vote,
    delegate,
    undelegate,
    calculateVotingPower,
    getConvictionInfo,
  };
}
