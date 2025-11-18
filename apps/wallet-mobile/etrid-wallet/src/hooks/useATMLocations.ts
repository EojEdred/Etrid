/**
 * useATMLocations - Hook for fetching and managing ATM locations
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import { useState, useEffect, useCallback } from 'react';
import ATMService from '../services/ATMService';
import { ATMLocation, ATMFilter, Coordinates } from '../types/atm.types';

interface UseATMLocationsResult {
  atms: ATMLocation[];
  loading: boolean;
  error: string | null;
  refetch: () => Promise<void>;
  filterATMs: (filter: ATMFilter) => void;
  searchATMs: (query: string) => Promise<void>;
}

export const useATMLocations = (
  coordinates?: Coordinates,
  radius: number = 10
): UseATMLocationsResult => {
  const [atms, setAtms] = useState<ATMLocation[]>([]);
  const [allAtms, setAllAtms] = useState<ATMLocation[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchATMs = useCallback(async () => {
    if (!coordinates) {
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const locations = await ATMService.getATMLocations(coordinates, radius);
      const sorted = ATMService.sortByDistance(locations, coordinates);
      setAllAtms(sorted);
      setAtms(sorted);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to fetch ATMs';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [coordinates, radius]);

  useEffect(() => {
    fetchATMs();
  }, [fetchATMs]);

  const filterATMs = useCallback(
    (filter: ATMFilter) => {
      const filtered = ATMService.filterATMs(allAtms, filter);
      setAtms(filtered);
    },
    [allAtms]
  );

  const searchATMs = useCallback(async (query: string) => {
    if (!query.trim()) {
      setAtms(allAtms);
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const results = await ATMService.searchATMs(query);
      setAtms(results);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Search failed';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [allAtms]);

  return {
    atms,
    loading,
    error,
    refetch: fetchATMs,
    filterATMs,
    searchATMs,
  };
};
