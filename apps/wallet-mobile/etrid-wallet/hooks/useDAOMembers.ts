/**
 * useDAOMembers Hook
 * Manages DAO member information and operations
 */

import { useState, useEffect, useCallback } from 'react';
import { DAOMember } from '@/types/dao';

export function useDAOMembers(daoId?: string) {
  const [members, setMembers] = useState<DAOMember[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load members when daoId changes
  useEffect(() => {
    if (daoId) {
      loadMembers(daoId);
    }
  }, [daoId]);

  /**
   * Load members for a DAO
   */
  const loadMembers = async (targetDaoId: string) => {
    setIsLoading(true);
    try {
      // TODO: Implement actual member fetching from service
      // const loadedMembers = await daoService.getMembers(targetDaoId);

      // Mock data for now
      const mockMembers: DAOMember[] = [
        {
          id: '1',
          userId: 'user-1',
          address: '0x1234567890abcdef1234567890abcdef12345678',
          username: 'Alice',
          avatarUrl: 'https://i.pravatar.cc/150?u=alice',
          role: 'owner',
          votingPower: 1000,
          joinedAt: new Date('2024-01-01'),
          proposalsCreated: 5,
          votesCast: 12,
        },
        {
          id: '2',
          userId: 'user-2',
          address: '0xabcdef1234567890abcdef1234567890abcdef12',
          username: 'Bob',
          avatarUrl: 'https://i.pravatar.cc/150?u=bob',
          role: 'member',
          votingPower: 500,
          joinedAt: new Date('2024-01-15'),
          proposalsCreated: 2,
          votesCast: 8,
        },
        {
          id: '3',
          userId: 'user-3',
          address: '0x567890abcdef1234567890abcdef1234567890ab',
          username: 'Charlie',
          avatarUrl: 'https://i.pravatar.cc/150?u=charlie',
          role: 'member',
          votingPower: 300,
          joinedAt: new Date('2024-02-01'),
          proposalsCreated: 1,
          votesCast: 5,
        },
      ];

      setMembers(mockMembers);
      setError(null);
    } catch (err: any) {
      console.error('Failed to load members:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Get member by user ID
   */
  const getMember = useCallback(
    (userId: string) => {
      return members.find((m) => m.userId === userId);
    },
    [members]
  );

  /**
   * Get members by role
   */
  const getMembersByRole = useCallback(
    (role: 'owner' | 'member' | 'voter') => {
      return members.filter((m) => m.role === role);
    },
    [members]
  );

  /**
   * Get top members by voting power
   */
  const getTopMembersByVotingPower = useCallback(
    (limit: number = 10) => {
      return [...members]
        .sort((a, b) => b.votingPower - a.votingPower)
        .slice(0, limit);
    },
    [members]
  );

  /**
   * Get most active members (by votes cast)
   */
  const getMostActiveMembers = useCallback(
    (limit: number = 10) => {
      return [...members]
        .sort((a, b) => b.votesCast - a.votesCast)
        .slice(0, limit);
    },
    [members]
  );

  /**
   * Get total voting power
   */
  const getTotalVotingPower = useCallback(() => {
    return members.reduce((sum, m) => sum + m.votingPower, 0);
  }, [members]);

  /**
   * Get member count
   */
  const getMemberCount = useCallback(() => {
    return members.length;
  }, [members]);

  /**
   * Check if user is member
   */
  const isMember = useCallback(
    (userId: string) => {
      return members.some((m) => m.userId === userId);
    },
    [members]
  );

  /**
   * Check if user is owner
   */
  const isOwner = useCallback(
    (userId: string) => {
      return members.some((m) => m.userId === userId && m.role === 'owner');
    },
    [members]
  );

  /**
   * Refresh members
   */
  const refreshMembers = useCallback(() => {
    if (daoId) {
      return loadMembers(daoId);
    }
  }, [daoId]);

  return {
    members,
    isLoading,
    error,
    getMember,
    getMembersByRole,
    getTopMembersByVotingPower,
    getMostActiveMembers,
    getTotalVotingPower,
    getMemberCount,
    isMember,
    isOwner,
    refreshMembers,
    loadMembers,
  };
}
