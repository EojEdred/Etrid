/**
 * useUsername Hook
 * React hook for username registration, lookup, and management
 */

'use client';

import { useState, useCallback, useEffect } from 'react';
import { usernameService } from '@/lib/social/UsernameService';
import type {
  UsernameRegistration,
  UsernameAvailability,
  UsernamePricing,
} from '@/lib/social/types';

export interface UseUsernameReturn {
  // State
  availability: UsernameAvailability | null;
  pricing: UsernamePricing | null;
  registration: UsernameRegistration | null;
  isChecking: boolean;
  isRegistering: boolean;
  error: string | null;

  // Actions
  checkAvailability: (username: string) => Promise<void>;
  registerUsername: (username: string, address: string) => Promise<UsernameRegistration>;
  resolveUsername: (username: string) => Promise<string>;
  reverseResolve: (address: string) => Promise<string | null>;
  calculatePrice: (username: string) => UsernamePricing;
  validateUsername: (username: string) => { valid: boolean; error?: string };
  clearError: () => void;
}

export function useUsername(): UseUsernameReturn {
  const [availability, setAvailability] = useState<UsernameAvailability | null>(null);
  const [pricing, setPricing] = useState<UsernamePricing | null>(null);
  const [registration, setRegistration] = useState<UsernameRegistration | null>(null);
  const [isChecking, setIsChecking] = useState(false);
  const [isRegistering, setIsRegistering] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * Check username availability
   */
  const checkAvailability = useCallback(async (username: string) => {
    setIsChecking(true);
    setError(null);

    try {
      const result = await usernameService.checkAvailability(username);
      setAvailability(result);

      // Also calculate pricing
      const price = usernameService.getPrice(username);
      setPricing(price);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to check availability';
      setError(message);
      setAvailability(null);
      setPricing(null);
    } finally {
      setIsChecking(false);
    }
  }, []);

  /**
   * Register a new username
   */
  const registerUsername = useCallback(
    async (username: string, address: string): Promise<UsernameRegistration> => {
      setIsRegistering(true);
      setError(null);

      try {
        const result = await usernameService.registerUsername(username, address);
        setRegistration(result);
        return result;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to register username';
        setError(message);
        throw err;
      } finally {
        setIsRegistering(false);
      }
    },
    []
  );

  /**
   * Resolve username to address
   */
  const resolveUsername = useCallback(async (username: string): Promise<string> => {
    setError(null);
    try {
      return await usernameService.resolveUsername(username);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to resolve username';
      setError(message);
      throw err;
    }
  }, []);

  /**
   * Reverse resolve address to username
   */
  const reverseResolve = useCallback(async (address: string): Promise<string | null> => {
    setError(null);
    try {
      return await usernameService.reverseResolve(address);
    } catch (err) {
      console.error('Failed to reverse resolve:', err);
      return null;
    }
  }, []);

  /**
   * Calculate username price
   */
  const calculatePrice = useCallback((username: string): UsernamePricing => {
    return usernameService.getPrice(username);
  }, []);

  /**
   * Validate username format
   */
  const validateUsername = useCallback((username: string) => {
    return usernameService.validateUsername(username);
  }, []);

  /**
   * Clear error state
   */
  const clearError = useCallback(() => {
    setError(null);
  }, []);

  return {
    availability,
    pricing,
    registration,
    isChecking,
    isRegistering,
    error,
    checkAvailability,
    registerUsername,
    resolveUsername,
    reverseResolve,
    calculatePrice,
    validateUsername,
    clearError,
  };
}
