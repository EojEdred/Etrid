/**
 * useDAppDirectory Hook
 * Manages dApp discovery and directory
 */

import { useState, useEffect, useCallback } from 'react';
import { DApp, DAppCategory } from '@/types/dapp';
import { dAppDirectoryService } from '@/services/DAppDirectoryService';

export function useDAppDirectory() {
  const [featuredDApps, setFeaturedDApps] = useState<DApp[]>([]);
  const [trendingDApps, setTrendingDApps] = useState<DApp[]>([]);
  const [recentlyVisited, setRecentlyVisited] = useState<DApp[]>([]);
  const [searchResults, setSearchResults] = useState<DApp[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load featured and trending on mount
  useEffect(() => {
    loadFeaturedDApps();
    loadTrendingDApps();
    loadRecentlyVisited();
  }, []);

  /**
   * Load featured dApps
   */
  const loadFeaturedDApps = async () => {
    setIsLoading(true);
    try {
      const dApps = await dAppDirectoryService.getFeaturedDApps();
      setFeaturedDApps(dApps);
      setError(null);
    } catch (err: any) {
      console.error('Failed to load featured dApps:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  };

  /**
   * Load trending dApps
   */
  const loadTrendingDApps = async () => {
    try {
      const dApps = await dAppDirectoryService.getTrendingDApps();
      setTrendingDApps(dApps);
    } catch (err: any) {
      console.error('Failed to load trending dApps:', err);
    }
  };

  /**
   * Load recently visited dApps
   */
  const loadRecentlyVisited = async () => {
    try {
      const dApps = await dAppDirectoryService.getRecentlyVisited();
      setRecentlyVisited(dApps);
    } catch (err: any) {
      console.error('Failed to load recently visited:', err);
    }
  };

  /**
   * Search dApps
   */
  const searchDApps = useCallback(async (query: string) => {
    if (!query.trim()) {
      setSearchResults([]);
      return;
    }

    setIsLoading(true);
    try {
      const results = await dAppDirectoryService.searchDApps(query);
      setSearchResults(results);
      setError(null);
    } catch (err: any) {
      console.error('Failed to search dApps:', err);
      setError(err.message);
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Get dApps by category
   */
  const getDAppsByCategory = useCallback(async (category: DAppCategory) => {
    setIsLoading(true);
    try {
      const dApps = await dAppDirectoryService.getDAppsByCategory(category);
      setSearchResults(dApps);
      setError(null);
      return dApps;
    } catch (err: any) {
      console.error('Failed to get dApps by category:', err);
      setError(err.message);
      return [];
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Add to recently visited
   */
  const addToRecentlyVisited = useCallback(async (dApp: DApp) => {
    try {
      await dAppDirectoryService.addToRecentlyVisited(dApp);
      await loadRecentlyVisited();
    } catch (err: any) {
      console.error('Failed to add to recently visited:', err);
    }
  }, []);

  /**
   * Add custom dApp
   */
  const addCustomDApp = useCallback(async (dApp: Omit<DApp, 'id'>) => {
    setIsLoading(true);
    try {
      const newDApp = await dAppDirectoryService.addCustomDApp(dApp);
      setError(null);
      return newDApp;
    } catch (err: any) {
      console.error('Failed to add custom dApp:', err);
      setError(err.message);
      throw err;
    } finally {
      setIsLoading(false);
    }
  }, []);

  /**
   * Clear search results
   */
  const clearSearch = useCallback(() => {
    setSearchResults([]);
  }, []);

  return {
    featuredDApps,
    trendingDApps,
    recentlyVisited,
    searchResults,
    isLoading,
    error,
    searchDApps,
    getDAppsByCategory,
    addToRecentlyVisited,
    addCustomDApp,
    clearSearch,
    refreshFeatured: loadFeaturedDApps,
    refreshTrending: loadTrendingDApps,
    refreshRecent: loadRecentlyVisited,
  };
}
