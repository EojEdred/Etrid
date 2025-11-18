import { useState, useEffect, useCallback } from 'react';
import StakingService from '../services/StakingService';
import { Validator } from '../types/defi.types';

/**
 * useValidators Hook - Manages validators list and selection
 */
export function useValidators() {
  const [validators, setValidators] = useState<Validator[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [sortBy, setSortBy] = useState<'apy' | 'commission' | 'uptime' | 'stake'>('apy');
  const [searchQuery, setSearchQuery] = useState('');

  /**
   * Load validators
   */
  const loadValidators = useCallback(async (sort?: typeof sortBy) => {
    try {
      setLoading(true);
      setError(null);

      const validatorsData = await StakingService.getValidators(sort || sortBy);
      setValidators(validatorsData);
    } catch (err) {
      console.error('Failed to load validators:', err);
      setError(err instanceof Error ? err.message : 'Failed to load validators');
    } finally {
      setLoading(false);
    }
  }, [sortBy]);

  /**
   * Refresh validators
   */
  const refresh = useCallback(async () => {
    await loadValidators();
  }, [loadValidators]);

  /**
   * Change sort order
   */
  const changeSortBy = useCallback(
    (newSort: typeof sortBy) => {
      setSortBy(newSort);
      loadValidators(newSort);
    },
    [loadValidators]
  );

  /**
   * Search validators
   */
  const searchValidators = useCallback(
    (query: string) => {
      setSearchQuery(query);
    },
    []
  );

  /**
   * Get filtered validators based on search
   */
  const filteredValidators = useCallback(() => {
    if (!searchQuery.trim()) {
      return validators;
    }

    const query = searchQuery.toLowerCase();
    return validators.filter(
      v =>
        v.name.toLowerCase().includes(query) ||
        v.address.toLowerCase().includes(query) ||
        v.identity?.display?.toLowerCase().includes(query)
    );
  }, [validators, searchQuery]);

  /**
   * Get top validators by APY
   */
  const getTopValidators = useCallback(
    (count: number = 3) => {
      return validators.slice(0, count);
    },
    [validators]
  );

  /**
   * Get validator by address
   */
  const getValidator = useCallback(
    async (address: string): Promise<Validator | null> => {
      try {
        return await StakingService.getValidatorInfo(address);
      } catch (err) {
        console.error('Failed to get validator:', err);
        return null;
      }
    },
    []
  );

  // Load validators on mount or when sortBy changes
  useEffect(() => {
    loadValidators();
  }, [loadValidators]);

  return {
    validators: filteredValidators(),
    allValidators: validators,
    loading,
    error,
    sortBy,
    searchQuery,
    refresh,
    changeSortBy,
    searchValidators,
    getTopValidators,
    getValidator,
  };
}
