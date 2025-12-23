/**
 * ETRID Governance React Hooks
 *
 * Convenient React hooks for integrating governance functionality
 * into web wallet components
 */

'use client';

import { useState, useEffect, useCallback, useMemo } from 'react';
import { governanceService } from './service';
import {
  ProposalCategory,
  ProposalStatus,
  ConvictionLevel,
} from './types';
import type {
  Proposal,
  ProposalFilters,
  CreateProposalParams,
  CastVoteParams,
  VoteRecord,
  VotingStats,
  VotingPower,
  VotingPowerBreakdown,
  DelegateParams,
  Delegation,
  DelegationStats,
  GovernanceStats,
  CategoryStats,
  TransactionResult,
  PaginationParams,
  PaginatedResponse,
} from './types';

// ═══════════════════════════════════════════════════════════════════════════════
// PROPOSAL HOOKS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Hook to fetch all proposals with optional filters
 * Fetches REAL data from democracy pallet on chain
 * Returns empty array if democracy pallet doesn't exist
 */
export function useProposals(filters?: ProposalFilters) {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchProposals = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getProposals(filters);
      setProposals(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [filters]);

  useEffect(() => {
    fetchProposals();
  }, [fetchProposals]);

  return {
    proposals,
    loading,
    error,
    refetch: fetchProposals,
  };
}

/**
 * Hook to fetch a single proposal by ID
 */
export function useProposal(proposalId: number | null) {
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchProposal = useCallback(async () => {
    if (proposalId === null) {
      setProposal(null);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getProposal(proposalId);
      setProposal(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [proposalId]);

  useEffect(() => {
    fetchProposal();
  }, [fetchProposal]);

  return {
    proposal,
    loading,
    error,
    refetch: fetchProposal,
  };
}

/**
 * Hook to fetch active proposals
 */
export function useActiveProposals() {
  return useProposals({
    statuses: [ProposalStatus.Active],
  });
}

/**
 * Hook to fetch proposals by category
 */
export function useProposalsByCategory(category: ProposalCategory) {
  return useProposals({
    categories: [category],
  });
}

/**
 * Hook to fetch proposals with pagination
 */
export function usePaginatedProposals(
  params: PaginationParams,
  filters?: ProposalFilters
) {
  const [result, setResult] = useState<PaginatedResponse<Proposal>>({
    data: [],
    total: 0,
    page: params.page,
    pageSize: params.pageSize,
    hasMore: false,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchProposals = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getProposalsPaginated(params, filters);
      setResult(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [params, filters]);

  useEffect(() => {
    fetchProposals();
  }, [fetchProposals]);

  return {
    ...result,
    loading,
    error,
    refetch: fetchProposals,
  };
}

/**
 * Hook to submit a new proposal
 */
export function useSubmitProposal() {
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<TransactionResult | null>(null);

  const submitProposal = useCallback(
    async (params: CreateProposalParams, signer: any) => {
      setSubmitting(true);
      setResult(null);

      try {
        const txResult = await governanceService.submitProposal(params, signer);
        setResult(txResult);
        return txResult;
      } catch (error) {
        const errorResult: TransactionResult = {
          success: false,
          error: error as any,
        };
        setResult(errorResult);
        return errorResult;
      } finally {
        setSubmitting(false);
      }
    },
    []
  );

  return {
    submitProposal,
    submitting,
    result,
  };
}

// ═══════════════════════════════════════════════════════════════════════════════
// VOTING HOOKS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Hook to cast a vote on a proposal
 */
export function useCastVote() {
  const [voting, setVoting] = useState(false);
  const [result, setResult] = useState<TransactionResult | null>(null);

  const castVote = useCallback(async (params: CastVoteParams, signer: any) => {
    setVoting(true);
    setResult(null);

    try {
      const txResult = await governanceService.castVote(params, signer);
      setResult(txResult);
      return txResult;
    } catch (error) {
      const errorResult: TransactionResult = {
        success: false,
        error: error as any,
      };
      setResult(errorResult);
      return errorResult;
    } finally {
      setVoting(false);
    }
  }, []);

  return {
    castVote,
    voting,
    result,
  };
}

/**
 * Hook to fetch votes for a proposal
 */
export function useVotes(proposalId: number | null) {
  const [votes, setVotes] = useState<VoteRecord[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchVotes = useCallback(async () => {
    if (proposalId === null) {
      setVotes([]);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getVotes(proposalId);
      setVotes(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [proposalId]);

  useEffect(() => {
    fetchVotes();
  }, [fetchVotes]);

  return {
    votes,
    loading,
    error,
    refetch: fetchVotes,
  };
}

/**
 * Hook to fetch voting statistics for a proposal
 */
export function useVotingStats(proposalId: number | null) {
  const [stats, setStats] = useState<VotingStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchStats = useCallback(async () => {
    if (proposalId === null) {
      setStats(null);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getVotingStats(proposalId);
      setStats(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [proposalId]);

  useEffect(() => {
    fetchStats();
  }, [fetchStats]);

  return {
    stats,
    loading,
    error,
    refetch: fetchStats,
  };
}

/**
 * Hook to check if an account has voted on a proposal
 */
export function useHasVoted(proposalId: number | null, account: string | null) {
  const [hasVoted, setHasVoted] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (proposalId === null || !account) {
      setHasVoted(false);
      setLoading(false);
      return;
    }

    let mounted = true;

    governanceService.hasVoted(proposalId, account).then((voted) => {
      if (mounted) {
        setHasVoted(voted);
        setLoading(false);
      }
    });

    return () => {
      mounted = false;
    };
  }, [proposalId, account]);

  return { hasVoted, loading };
}

// ═══════════════════════════════════════════════════════════════════════════════
// VOTING POWER HOOKS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Hook to fetch voting power for an account
 */
export function useVotingPower(account: string | null) {
  const [votingPower, setVotingPower] = useState<VotingPower | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchVotingPower = useCallback(async () => {
    if (!account) {
      setVotingPower(null);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getVotingPower(account);
      setVotingPower(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [account]);

  useEffect(() => {
    fetchVotingPower();
  }, [fetchVotingPower]);

  return {
    votingPower,
    loading,
    error,
    refetch: fetchVotingPower,
  };
}

/**
 * Hook to calculate voting power breakdown with conviction
 */
export function useVotingPowerBreakdown(
  balance: string,
  conviction: ConvictionLevel
): VotingPowerBreakdown {
  return useMemo(
    () => governanceService.calculateVotingPowerBreakdown(balance, conviction),
    [balance, conviction]
  );
}

// ═══════════════════════════════════════════════════════════════════════════════
// DELEGATION HOOKS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Hook to delegate voting power
 */
export function useDelegate() {
  const [delegating, setDelegating] = useState(false);
  const [result, setResult] = useState<TransactionResult | null>(null);

  const delegate = useCallback(async (params: DelegateParams, signer: any) => {
    setDelegating(true);
    setResult(null);

    try {
      const txResult = await governanceService.delegate(params, signer);
      setResult(txResult);
      return txResult;
    } catch (error) {
      const errorResult: TransactionResult = {
        success: false,
        error: error as any,
      };
      setResult(errorResult);
      return errorResult;
    } finally {
      setDelegating(false);
    }
  }, []);

  return {
    delegate,
    delegating,
    result,
  };
}

/**
 * Hook to undelegate voting power
 */
export function useUndelegate() {
  const [undelegating, setUndelegating] = useState(false);
  const [result, setResult] = useState<TransactionResult | null>(null);

  const undelegate = useCallback(async (signer: any) => {
    setUndelegating(true);
    setResult(null);

    try {
      const txResult = await governanceService.undelegate(signer);
      setResult(txResult);
      return txResult;
    } catch (error) {
      const errorResult: TransactionResult = {
        success: false,
        error: error as any,
      };
      setResult(errorResult);
      return errorResult;
    } finally {
      setUndelegating(false);
    }
  }, []);

  return {
    undelegate,
    undelegating,
    result,
  };
}

/**
 * Hook to fetch delegation for an account
 */
export function useDelegation(account: string | null) {
  const [delegation, setDelegation] = useState<Delegation | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchDelegation = useCallback(async () => {
    if (!account) {
      setDelegation(null);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getDelegation(account);
      setDelegation(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [account]);

  useEffect(() => {
    fetchDelegation();
  }, [fetchDelegation]);

  return {
    delegation,
    loading,
    error,
    refetch: fetchDelegation,
  };
}

/**
 * Hook to fetch delegation statistics for an account
 */
export function useDelegationStats(account: string | null) {
  const [stats, setStats] = useState<DelegationStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchStats = useCallback(async () => {
    if (!account) {
      setStats(null);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getDelegationStats(account);
      setStats(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [account]);

  useEffect(() => {
    fetchStats();
  }, [fetchStats]);

  return {
    stats,
    loading,
    error,
    refetch: fetchStats,
  };
}

// ═══════════════════════════════════════════════════════════════════════════════
// STATISTICS HOOKS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Hook to fetch overall governance statistics
 * Fetches REAL data from democracy pallet including:
 * - Total proposals and referendums
 * - Active/passed/rejected counts
 * - Unique voter count
 * - Treasury balance
 */
export function useGovernanceStats() {
  const [stats, setStats] = useState<GovernanceStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchStats = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getGovernanceStats();
      setStats(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchStats();
  }, [fetchStats]);

  return {
    stats,
    loading,
    error,
    refetch: fetchStats,
  };
}

/**
 * Hook to fetch statistics for a specific category
 */
export function useCategoryStats(category: ProposalCategory) {
  const [stats, setStats] = useState<CategoryStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchStats = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await governanceService.getCategoryStats(category);
      setStats(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [category]);

  useEffect(() => {
    fetchStats();
  }, [fetchStats]);

  return {
    stats,
    loading,
    error,
    refetch: fetchStats,
  };
}

// ═══════════════════════════════════════════════════════════════════════════════
// UTILITY HOOKS
// ═══════════════════════════════════════════════════════════════════════════════

/**
 * Hook to manage governance service connection
 */
export function useGovernanceConnection() {
  const [connected, setConnected] = useState(false);
  const [connecting, setConnecting] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const connect = useCallback(async () => {
    try {
      setConnecting(true);
      setError(null);
      // Connection happens automatically when needed
      setConnected(true);
    } catch (err) {
      setError(err as Error);
      setConnected(false);
    } finally {
      setConnecting(false);
    }
  }, []);

  const disconnect = useCallback(async () => {
    try {
      await governanceService.disconnect();
      setConnected(false);
    } catch (err) {
      setError(err as Error);
    }
  }, []);

  return {
    connected,
    connecting,
    error,
    connect,
    disconnect,
  };
}

/**
 * Hook for polling data at regular intervals
 */
export function useGovernancePolling<T>(
  fetchFn: () => Promise<T>,
  interval: number = 12000 // 12 seconds (2 blocks)
) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetch = useCallback(async () => {
    try {
      setError(null);
      const result = await fetchFn();
      setData(result);
      setLoading(false);
    } catch (err) {
      setError(err as Error);
      setLoading(false);
    }
  }, [fetchFn]);

  useEffect(() => {
    fetch();
    const intervalId = setInterval(fetch, interval);

    return () => clearInterval(intervalId);
  }, [fetch, interval]);

  return {
    data,
    loading,
    error,
    refetch: fetch,
  };
}

/**
 * Comprehensive governance hook - combines multiple data sources
 */
export function useGovernance(account: string | null) {
  const { proposals, loading: proposalsLoading } = useActiveProposals();
  const { votingPower, loading: votingPowerLoading } = useVotingPower(account);
  const { delegation, loading: delegationLoading } = useDelegation(account);
  const { stats: governanceStats, loading: statsLoading } = useGovernanceStats();

  const loading = proposalsLoading || votingPowerLoading || delegationLoading || statsLoading;

  return {
    proposals,
    votingPower,
    delegation,
    governanceStats,
    loading,
  };
}

/**
 * Hook to calculate time remaining from blocks
 */
export function useTimeRemaining(blocksRemaining: number) {
  const [timeRemaining, setTimeRemaining] = useState<{
    days: number;
    hours: number;
    minutes: number;
    seconds: number;
    total: number;
  }>({ days: 0, hours: 0, minutes: 0, seconds: 0, total: 0 });

  useEffect(() => {
    // ETRID block time is 6 seconds
    const BLOCK_TIME_SECONDS = 6;
    const totalSeconds = blocksRemaining * BLOCK_TIME_SECONDS;

    const days = Math.floor(totalSeconds / 86400);
    const hours = Math.floor((totalSeconds % 86400) / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;

    setTimeRemaining({ days, hours, minutes, seconds, total: totalSeconds });
  }, [blocksRemaining]);

  return timeRemaining;
}

/**
 * Format ETR balance to human-readable format
 */
export function formatETR(balance: string, decimals: number = 18): string {
  try {
    const value = BigInt(balance);
    const divisor = BigInt(10 ** decimals);
    const wholePart = value / divisor;
    const fractionalPart = value % divisor;

    const fractionalStr = fractionalPart.toString().padStart(decimals, '0');
    const displayFractional = fractionalStr.slice(0, 4).replace(/0+$/, '');

    if (displayFractional) {
      return `${wholePart.toString()}.${displayFractional}`;
    }
    return wholePart.toString();
  } catch {
    return '0';
  }
}

/**
 * Parse ETR balance to wei format
 */
export function parseETR(balance: string, decimals: number = 18): string {
  try {
    const [whole = '0', fractional = '0'] = balance.split('.');
    const paddedFractional = fractional.padEnd(decimals, '0').slice(0, decimals);
    return (BigInt(whole) * BigInt(10 ** decimals) + BigInt(paddedFractional)).toString();
  } catch {
    return '0';
  }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONSENSUS DAY HOOKS
// ═══════════════════════════════════════════════════════════════════════════════

export type ConsensusDayPhase = 'VOTING' | 'VALIDATION' | 'DISTRIBUTION' | 'IDLE';

export interface ConsensusDayPhaseInfo {
  phase: ConsensusDayPhase;
  currentBlock: number;
  phaseStartBlock: number;
  phaseEndBlock: number;
  blocksRemaining: number;
  progress: number; // 0-100%
}

/**
 * Hook to track consensus day phase
 * ETRID uses a 21-day consensus cycle with different phases
 */
export function useConsensusDayPhase() {
  const [phase, setPhase] = useState<{ phase: ConsensusDayPhase; blocksRemaining: number; isActive: boolean } | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchPhase = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      // ETRID Consensus Day phases (21 days = 302,400 blocks at 6s/block)
      // Day 1-14: Voting phase
      // Day 15-18: Validation phase
      // Day 19-21: Distribution phase

      // For now, query chain for current block and calculate phase
      // In future, this will query a specific pallet if implemented
      const api = await governanceService['ensureConnection']();
      const currentBlock = (await api.rpc.chain.getHeader()).number.toNumber();

      const cycleBlocks = 302400; // 21 days in blocks
      const blockInCycle = currentBlock % cycleBlocks;

      const votingEnd = 201600;    // Day 14
      const validationEnd = 259200; // Day 18

      let currentPhase: ConsensusDayPhase;
      let phaseEndBlock: number;

      if (blockInCycle < votingEnd) {
        currentPhase = 'VOTING';
        phaseEndBlock = votingEnd;
      } else if (blockInCycle < validationEnd) {
        currentPhase = 'VALIDATION';
        phaseEndBlock = validationEnd;
      } else {
        currentPhase = 'DISTRIBUTION';
        phaseEndBlock = cycleBlocks;
      }

      const blocksRemaining = phaseEndBlock - blockInCycle;

      setPhase({
        phase: currentPhase,
        blocksRemaining,
        isActive: true,
      });
    } catch (err) {
      console.error('Failed to fetch consensus day phase:', err);
      setError(err as Error);
      // Set a fallback idle state
      setPhase({
        phase: 'IDLE',
        blocksRemaining: 0,
        isActive: false,
      });
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchPhase();
    // Update every 12 seconds (2 blocks)
    const interval = setInterval(fetchPhase, 12000);
    return () => clearInterval(interval);
  }, [fetchPhase]);

  return {
    phase: phase || { phase: 'IDLE' as ConsensusDayPhase, blocksRemaining: 0, isActive: false },
    loading,
    error,
    refetch: fetchPhase,
  };
}

export interface RewardInfo {
  pendingRewards: string;
  claimedRewards: string;
  totalRewards: string;
  lastClaimBlock: number;
  nextClaimableBlock: number;
  apr: number;
}

/**
 * Hook to fetch reward information for an account
 */
export function useReward(account: string | null) {
  const [reward, setReward] = useState<RewardInfo | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchReward = useCallback(async () => {
    if (!account) {
      setReward(null);
      setLoading(false);
      return;
    }

    try {
      setLoading(true);
      setError(null);

      // Mock implementation - in production this would query the chain
      // Simulated reward based on account balance participation
      setReward({
        pendingRewards: '125000000000000000000', // 125 ETR
        claimedRewards: '1500000000000000000000', // 1500 ETR
        totalRewards: '1625000000000000000000', // 1625 ETR
        lastClaimBlock: Math.floor(Date.now() / 6000) - 14400, // ~1 day ago
        nextClaimableBlock: Math.floor(Date.now() / 6000) + 14400, // ~1 day from now
        apr: 12.5, // 12.5% APR
      });
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  }, [account]);

  useEffect(() => {
    fetchReward();
  }, [fetchReward]);

  return {
    reward,
    loading,
    error,
    refetch: fetchReward,
  };
}
