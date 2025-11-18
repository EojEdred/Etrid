import { useEffect } from 'react';
import AnalyticsService from '../services/AnalyticsService';

/**
 * Hook for tracking analytics events
 */
export function useAnalytics() {
  /**
   * Track screen view
   */
  const trackScreen = (screenName: string, params?: Record<string, any>) => {
    AnalyticsService.trackScreenView(screenName, params);
  };

  /**
   * Track button click
   */
  const trackButton = (buttonName: string, screenName: string, params?: Record<string, any>) => {
    AnalyticsService.trackButtonClick(buttonName, screenName, params);
  };

  /**
   * Track transaction
   */
  const trackTransaction = (
    type: 'send' | 'receive' | 'stake' | 'unstake' | 'swap' | 'bridge' | 'gpu_rent',
    amount: string,
    asset: string,
    success: boolean,
    params?: Record<string, any>
  ) => {
    AnalyticsService.trackTransaction(type, amount, asset, success, params);
  };

  /**
   * Track error
   */
  const trackError = (
    errorType: string,
    errorMessage: string,
    screenName?: string,
    params?: Record<string, any>
  ) => {
    AnalyticsService.trackError(errorType, errorMessage, screenName, params);
  };

  return {
    trackScreen,
    trackButton,
    trackTransaction,
    trackError,
  };
}

/**
 * Hook for tracking screen views
 */
export function useScreenTracking(screenName: string, params?: Record<string, any>) {
  useEffect(() => {
    AnalyticsService.trackScreenView(screenName, params);
  }, [screenName, params]);
}
