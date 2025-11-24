'use client';

import { useState, useEffect, useCallback } from 'react';
import type { FraudAlert, FraudStats } from '@/types/watchtower';

const generateMockAlerts = (count: number): FraudAlert[] => {
  const types: FraudAlert['type'][] = [
    'old_state_broadcast',
    'double_spend',
    'invalid_signature',
    'unauthorized_close',
  ];
  const severities: FraudAlert['severity'][] = ['low', 'medium', 'high', 'critical'];

  return Array.from({ length: count }, (_, i) => ({
    id: `alert-${i + 1}`,
    channelId: `0x${Math.random().toString(16).substring(2, 42)}`,
    type: types[Math.floor(Math.random() * types.length)],
    severity: severities[Math.floor(Math.random() * severities.length)],
    timestamp: new Date(Date.now() - Math.random() * 86400000),
    description: `Fraud attempt detected on channel`,
    evidenceHash: `0x${Math.random().toString(16).substring(2, 66)}`,
    resolved: Math.random() > 0.5,
    penaltyAmount: Math.random() > 0.5 ? Math.floor(Math.random() * 10000) : undefined,
  }));
};

export function useFraudDetection() {
  const [alerts, setAlerts] = useState<FraudAlert[]>([]);
  const [fraudStats, setFraudStats] = useState<FraudStats>({
    totalDetected: 0,
    byType: {},
    accuracy: 98.5,
    averageResponseTime: 245,
  });
  const [isLoading, setIsLoading] = useState(true);

  const fetchFraudAlerts = useCallback(async () => {
    try {
      setIsLoading(true);
      await new Promise((resolve) => setTimeout(resolve, 800));

      const mockAlerts = generateMockAlerts(12);
      setAlerts(mockAlerts.sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime()));

      const byType = mockAlerts.reduce((acc, alert) => {
        acc[alert.type] = (acc[alert.type] || 0) + 1;
        return acc;
      }, {} as Record<string, number>);

      setFraudStats({
        totalDetected: mockAlerts.length,
        byType,
        accuracy: 98.5,
        averageResponseTime: 245,
      });
    } catch (error) {
      console.error('Failed to fetch fraud alerts:', error);
    } finally {
      setIsLoading(false);
    }
  }, []);

  const resolveAlert = useCallback(async (alertId: string) => {
    try {
      console.log(`Resolving alert ${alertId}`);
      setAlerts((prev) =>
        prev.map((alert) =>
          alert.id === alertId ? { ...alert, resolved: true } : alert
        )
      );
      return { success: true };
    } catch (error) {
      console.error('Failed to resolve alert:', error);
      return { success: false, error };
    }
  }, []);

  const dismissAlert = useCallback(async (alertId: string) => {
    try {
      console.log(`Dismissing alert ${alertId}`);
      setAlerts((prev) => prev.filter((alert) => alert.id !== alertId));
      return { success: true };
    } catch (error) {
      console.error('Failed to dismiss alert:', error);
      return { success: false, error };
    }
  }, []);

  useEffect(() => {
    fetchFraudAlerts();

    const intervalId = setInterval(fetchFraudAlerts, 30000);

    return () => clearInterval(intervalId);
  }, [fetchFraudAlerts]);

  return {
    alerts,
    fraudStats,
    isLoading,
    resolveAlert,
    dismissAlert,
    refreshAlerts: fetchFraudAlerts,
  };
}
