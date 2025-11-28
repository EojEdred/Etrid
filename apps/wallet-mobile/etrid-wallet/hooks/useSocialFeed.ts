/**
 * useSocialFeed Hook
 * React hook for social activity feed and milestones
 */

'use client';

import { useState, useCallback, useEffect } from 'react';
import type {
  SocialActivity,
  ActivityType,
  ActivityFeedFilter,
  Milestone,
} from '@/lib/social/types';

export interface UseSocialFeedReturn {
  // State
  activities: SocialActivity[];
  filteredActivities: SocialActivity[];
  filter: ActivityFeedFilter;
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchActivities: () => Promise<void>;
  setFilter: (filter: ActivityFeedFilter) => void;
  likeActivity: (activityId: string) => Promise<void>;
  commentOnActivity: (activityId: string, comment: string) => Promise<void>;
  clearError: () => void;
}

// Mock data for development
const MOCK_ACTIVITIES: SocialActivity[] = [
  {
    id: '1',
    userId: 'user1',
    userAddress: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
    username: 'alice',
    activityType: 'milestone_reached',
    title: 'Milestone Reached!',
    description: 'Alice reached 10,000 ÉTR!',
    amount: 10000,
    createdAt: new Date('2024-11-18T10:30:00'),
    isPublic: true,
    likes: 15,
    comments: 3,
  },
  {
    id: '2',
    userId: 'user2',
    userAddress: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
    username: 'bob',
    activityType: 'transaction_sent',
    title: 'Transaction Sent',
    description: 'Bob sent 500 ÉTR to alice.etrid',
    amount: 500,
    createdAt: new Date('2024-11-18T09:15:00'),
    isPublic: true,
    likes: 2,
    comments: 0,
  },
  {
    id: '3',
    userId: 'user3',
    userAddress: '5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL',
    username: 'charlie',
    activityType: 'username_registered',
    title: 'New Username',
    description: 'Charlie registered charlie.etrid',
    createdAt: new Date('2024-11-17T16:45:00'),
    isPublic: true,
    likes: 8,
    comments: 1,
  },
  {
    id: '4',
    userId: 'user1',
    userAddress: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
    username: 'alice',
    activityType: 'staking_reward',
    title: 'Staking Rewards',
    description: 'Alice earned 125 ÉTR in staking rewards',
    amount: 125,
    createdAt: new Date('2024-11-17T08:00:00'),
    isPublic: true,
    likes: 5,
    comments: 0,
  },
  {
    id: '5',
    userId: 'user4',
    userAddress: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
    username: 'diana',
    activityType: 'bill_split_created',
    title: 'Bill Split Created',
    description: 'Diana created a bill split: Team Dinner',
    amount: 250,
    createdAt: new Date('2024-11-16T19:30:00'),
    isPublic: true,
    likes: 3,
    comments: 2,
  },
];

export function useSocialFeed(userId: string): UseSocialFeedReturn {
  const [activities, setActivities] = useState<SocialActivity[]>([]);
  const [filteredActivities, setFilteredActivities] = useState<SocialActivity[]>([]);
  const [filter, setFilterState] = useState<ActivityFeedFilter>({});
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Fetch social activities
   */
  const fetchActivities = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      // TODO: Replace with actual API call
      // const response = await fetch(`/api/social/feed?userId=${userId}`);
      // const data = await response.json();

      // Mock delay
      await new Promise((resolve) => setTimeout(resolve, 500));

      setActivities(MOCK_ACTIVITIES);
      setFilteredActivities(MOCK_ACTIVITIES);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to fetch activities';
      setError(message);
    } finally {
      setIsLoading(false);
    }
  }, [userId]);

  /**
   * Set activity filter
   */
  const setFilter = useCallback(
    (newFilter: ActivityFeedFilter) => {
      setFilterState(newFilter);

      let filtered = [...activities];

      // Filter by activity type
      if (newFilter.type && newFilter.type.length > 0) {
        filtered = filtered.filter((a) => newFilter.type!.includes(a.activityType));
      }

      // Filter by date range
      if (newFilter.dateFrom) {
        filtered = filtered.filter((a) => a.createdAt >= newFilter.dateFrom!);
      }
      if (newFilter.dateTo) {
        filtered = filtered.filter((a) => a.createdAt <= newFilter.dateTo!);
      }

      // Filter friends only
      if (newFilter.friendsOnly) {
        // TODO: Implement friends filter based on contacts
        // For now, just return all
      }

      // Apply pagination
      if (newFilter.offset !== undefined && newFilter.limit !== undefined) {
        const start = newFilter.offset;
        const end = start + newFilter.limit;
        filtered = filtered.slice(start, end);
      } else if (newFilter.limit !== undefined) {
        filtered = filtered.slice(0, newFilter.limit);
      }

      setFilteredActivities(filtered);
    },
    [activities]
  );

  /**
   * Like an activity
   */
  const likeActivity = useCallback(async (activityId: string): Promise<void> => {
    setError(null);

    try {
      // TODO: Replace with actual API call
      // await fetch(`/api/social/activities/${activityId}/like`, { method: 'POST' });

      setActivities((prev) =>
        prev.map((a) =>
          a.id === activityId ? { ...a, likes: a.likes + 1 } : a
        )
      );

      setFilteredActivities((prev) =>
        prev.map((a) =>
          a.id === activityId ? { ...a, likes: a.likes + 1 } : a
        )
      );
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to like activity';
      setError(message);
    }
  }, []);

  /**
   * Comment on an activity
   */
  const commentOnActivity = useCallback(
    async (activityId: string, comment: string): Promise<void> => {
      setError(null);

      try {
        // TODO: Replace with actual API call
        // await fetch(`/api/social/activities/${activityId}/comment`, {
        //   method: 'POST',
        //   body: JSON.stringify({ comment }),
        // });

        setActivities((prev) =>
          prev.map((a) =>
            a.id === activityId ? { ...a, comments: a.comments + 1 } : a
          )
        );

        setFilteredActivities((prev) =>
          prev.map((a) =>
            a.id === activityId ? { ...a, comments: a.comments + 1 } : a
          )
        );
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to comment on activity';
        setError(message);
      }
    },
    []
  );

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  // Fetch activities on mount
  useEffect(() => {
    fetchActivities();
  }, [fetchActivities]);

  return {
    activities,
    filteredActivities,
    filter,
    isLoading,
    error,
    fetchActivities,
    setFilter,
    likeActivity,
    commentOnActivity,
    clearError,
  };
}

/**
 * Predefined milestones
 */
export const MILESTONES: Milestone[] = [
  {
    type: 'balance',
    threshold: 1000,
    title: 'First 1K ÉTR',
    description: 'Reached 1,000 ÉTR balance',
    icon: '🎯',
  },
  {
    type: 'balance',
    threshold: 10000,
    title: 'Big Saver',
    description: 'Reached 10,000 ÉTR balance',
    icon: '💰',
  },
  {
    type: 'transactions',
    threshold: 10,
    title: 'Getting Started',
    description: 'Completed 10 transactions',
    icon: '🚀',
  },
  {
    type: 'transactions',
    threshold: 100,
    title: 'Active User',
    description: 'Completed 100 transactions',
    icon: '⭐',
  },
  {
    type: 'staking',
    threshold: 1000,
    title: 'Staking Beginner',
    description: 'Staked 1,000 ÉTR',
    icon: '🔒',
  },
  {
    type: 'governance',
    threshold: 5,
    title: 'Voter',
    description: 'Participated in 5 governance votes',
    icon: '🗳️',
  },
];
