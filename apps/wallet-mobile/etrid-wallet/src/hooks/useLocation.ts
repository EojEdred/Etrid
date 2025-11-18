/**
 * useLocation - Hook for GPS and location operations
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import { useState, useEffect, useCallback } from 'react';
import * as Location from 'expo-location';
import LocationService from '../services/LocationService';
import { Coordinates } from '../types/atm.types';

interface UseLocationResult {
  location: Coordinates | null;
  loading: boolean;
  error: string | null;
  hasPermission: boolean;
  requestPermission: () => Promise<boolean>;
  getCurrentLocation: () => Promise<void>;
  watchLocation: (callback: (location: Coordinates) => void) => Promise<void>;
  stopWatching: () => void;
}

export const useLocation = (autoFetch: boolean = true): UseLocationResult => {
  const [location, setLocation] = useState<Coordinates | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [hasPermission, setHasPermission] = useState(false);
  const [subscription, setSubscription] = useState<Location.LocationSubscription | null>(
    null
  );

  useEffect(() => {
    checkPermission();
  }, []);

  useEffect(() => {
    if (autoFetch && hasPermission) {
      getCurrentLocation();
    }
  }, [hasPermission, autoFetch]);

  const checkPermission = async () => {
    const granted = await LocationService.hasLocationPermission();
    setHasPermission(granted);
  };

  const requestPermission = useCallback(async (): Promise<boolean> => {
    setError(null);

    try {
      const granted = await LocationService.requestLocationPermission();
      setHasPermission(granted);

      if (!granted) {
        setError('Location permission denied');
      }

      return granted;
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to request permission';
      setError(errorMessage);
      return false;
    }
  }, []);

  const getCurrentLocation = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const coords = await LocationService.getCurrentLocation();
      setLocation(coords);
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : 'Failed to get location';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, []);

  const watchLocation = useCallback(
    async (callback: (location: Coordinates) => void) => {
      setError(null);

      try {
        const sub = await LocationService.watchLocation((coords) => {
          setLocation(coords);
          callback(coords);
        });
        setSubscription(sub);
      } catch (err) {
        const errorMessage =
          err instanceof Error ? err.message : 'Failed to watch location';
        setError(errorMessage);
      }
    },
    []
  );

  const stopWatching = useCallback(() => {
    if (subscription) {
      subscription.remove();
      setSubscription(null);
    }
  }, [subscription]);

  useEffect(() => {
    return () => {
      stopWatching();
    };
  }, []);

  return {
    location,
    loading,
    error,
    hasPermission,
    requestPermission,
    getCurrentLocation,
    watchLocation,
    stopWatching,
  };
};
