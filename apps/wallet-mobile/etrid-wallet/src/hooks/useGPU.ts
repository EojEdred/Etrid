import { useState, useEffect, useCallback } from 'react';
import GPUService, { GPUSpec, GPURental, GPUSearchFilters } from '../services/GPUService';
import CacheManager, { CacheKeys, CacheTTLs } from '../utils/CacheManager';
import PerformanceMonitor from '../utils/PerformanceMonitor';
import AnalyticsService from '../services/AnalyticsService';

/**
 * Hook for GPU marketplace operations
 */
export function useGPU() {
  const [gpus, setGPUs] = useState<GPUSpec[]>([]);
  const [rentals, setRentals] = useState<GPURental[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  /**
   * Search GPUs
   */
  const searchGPUs = useCallback(async (filters: GPUSearchFilters = {}) => {
    setLoading(true);
    setError(null);

    try {
      const cacheKey = `${CacheKeys.GPU_LIST}_${JSON.stringify(filters)}`;

      const results = await CacheManager.getOrFetch(
        cacheKey,
        () => PerformanceMonitor.measure('search_gpus', () => GPUService.searchGPUs(filters)),
        CacheTTLs.GPU_LIST
      );

      setGPUs(results);
      AnalyticsService.trackGPU('search', undefined, { filters });
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to search GPUs';
      setError(message);
      AnalyticsService.trackError('gpu_search_failed', message);
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Get GPU details
   */
  const getGPUDetails = useCallback(async (gpuId: string): Promise<GPUSpec | null> => {
    try {
      const cacheKey = `gpu_details_${gpuId}`;

      const gpu = await CacheManager.getOrFetch(
        cacheKey,
        () => GPUService.getGPUDetails(gpuId),
        CacheTTLs.GPU_LIST
      );

      AnalyticsService.trackGPU('view_details', gpuId);
      return gpu;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to get GPU details';
      setError(message);
      AnalyticsService.trackError('gpu_details_failed', message, undefined, { gpuId });
      return null;
    }
  }, []);

  /**
   * Rent GPU
   */
  const rentGPU = useCallback(async (gpuId: string, durationHours: number): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      const rental = await PerformanceMonitor.measure(
        'rent_gpu',
        () => GPUService.rentGPU(gpuId, durationHours),
        { gpuId, durationHours }
      );

      // Invalidate cache
      await CacheManager.invalidatePattern('gpu_');

      // Refresh rentals
      await loadMyRentals();

      AnalyticsService.trackGPU('rent', gpuId, { durationHours, rentalId: rental.rentalId });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to rent GPU';
      setError(message);
      AnalyticsService.trackError('gpu_rent_failed', message, undefined, { gpuId, durationHours });
      return false;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Load my rentals
   */
  const loadMyRentals = useCallback(async () => {
    try {
      const results = await CacheManager.getOrFetch(
        'my_gpu_rentals',
        () => GPUService.getMyRentals(),
        300000 // 5 min cache
      );

      setRentals(results);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to load rentals';
      setError(message);
      AnalyticsService.trackError('gpu_rentals_failed', message);
    }
  }, []);

  /**
   * Extend rental
   */
  const extendRental = useCallback(async (rentalId: string, additionalHours: number): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      await GPUService.extendRental(rentalId, additionalHours);

      // Invalidate cache
      await CacheManager.remove('my_gpu_rentals');

      // Refresh rentals
      await loadMyRentals();

      AnalyticsService.trackGPU('extend_rental', undefined, { rentalId, additionalHours });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to extend rental';
      setError(message);
      AnalyticsService.trackError('gpu_extend_failed', message, undefined, { rentalId });
      return false;
    } finally {
      setLoading(false);
    }
  }, [loadMyRentals]);

  /**
   * End rental
   */
  const endRental = useCallback(async (rentalId: string): Promise<boolean> => {
    setLoading(true);
    setError(null);

    try {
      await GPUService.endRental(rentalId);

      // Invalidate cache
      await CacheManager.remove('my_gpu_rentals');

      // Refresh rentals
      await loadMyRentals();

      AnalyticsService.trackGPU('end_rental', undefined, { rentalId });
      return true;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to end rental';
      setError(message);
      AnalyticsService.trackError('gpu_end_failed', message, undefined, { rentalId });
      return false;
    } finally {
      setLoading(false);
    }
  }, [loadMyRentals]);

  /**
   * Register GPU (for providers)
   */
  const registerGPU = useCallback(async (gpuSpec: Omit<GPUSpec, 'id'>): Promise<string | null> => {
    setLoading(true);
    setError(null);

    try {
      const gpuId = await GPUService.registerGPU(gpuSpec);

      // Invalidate cache
      await CacheManager.invalidatePattern('gpu_');

      AnalyticsService.trackGPU('register', gpuId, { model: gpuSpec.model });
      return gpuId;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to register GPU';
      setError(message);
      AnalyticsService.trackError('gpu_register_failed', message);
      return null;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Refresh data
   */
  const refresh = useCallback(async () => {
    await CacheManager.invalidatePattern('gpu_');
    await searchGPUs();
    await loadMyRentals();
  }, [searchGPUs, loadMyRentals]);

  // Load initial data
  useEffect(() => {
    searchGPUs();
    loadMyRentals();
  }, []);

  return {
    gpus,
    rentals,
    loading,
    error,
    searchGPUs,
    getGPUDetails,
    rentGPU,
    extendRental,
    endRental,
    registerGPU,
    refresh,
  };
}
