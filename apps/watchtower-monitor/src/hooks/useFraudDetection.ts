'use client';

import { useState, useEffect, useCallback, useRef } from 'react';
import type { FraudAlert } from '../types';

interface FraudStats {
  totalDetections: number;
  successfulInterventions: number;
  falsePositives: number;
  accuracy: number;
  pendingAlerts: number;
}

// Mock fraud alert generator
const generateMockAlert = (): FraudAlert => {
  const types: FraudAlert['type'][] = [
    'old_state_broadcast',
    'double_spend',
    'invalid_signature',
    'unauthorized_close',
  ];
  const severities: FraudAlert['severity'][] = ['low', 'medium', 'high', 'critical'];

  const type = types[Math.floor(Math.random() * types.length)];
  const severity = severities[Math.floor(Math.random() * severities.length)];

  return {
    id: `alert-${Date.now()}-${Math.random().toString(36).substring(7)}`,
    channelId: `0x${Math.random().toString(16).substring(2, 42)}`,
    type,
    severity,
    timestamp: new Date(Date.now() - Math.random() * 86400000 * 7),
    description: getAlertDescription(type, severity),
    evidenceHash: `0x${Math.random().toString(16).substring(2, 66)}`,
    resolved: Math.random() > 0.3,
    penaltyAmount: severity === 'critical' || severity === 'high'
      ? Math.floor(Math.random() * 5000) + 1000
      : undefined,
    reportedBy: Math.random() > 0.5
      ? `0x${Math.random().toString(16).substring(2, 42)}`
      : undefined,
  };
};

const getAlertDescription = (type: FraudAlert['type'], severity: FraudAlert['severity']): string => {
  const descriptions: Record<string, string> = {
    old_state_broadcast: `Detected attempt to broadcast old channel state. ${severity === 'critical' ? 'Immediate intervention required.' : 'Monitoring for further activity.'}`,
    double_spend: `Suspicious double-spend attempt detected in channel transaction. ${severity === 'high' ? 'High risk of fraud.' : 'Under investigation.'}`,
    invalid_signature: `Invalid signature detected in commitment transaction. ${severity === 'medium' ? 'Potential security breach.' : 'Verifying authenticity.'}`,
    unauthorized_close: `Unauthorized channel closure attempt detected. ${severity === 'critical' ? 'Emergency intervention activated.' : 'Analyzing transaction.'}`,
  };
  return descriptions[type] || 'Suspicious activity detected in channel.';
};

export function useFraudDetection() {
  const [alerts, setAlerts] = useState<FraudAlert[]>([]);
  const [fraudStats, setFraudStats] = useState<FraudStats>({
    totalDetections: 0,
    successfulInterventions: 0,
    falsePositives: 0,
    accuracy: 0,
    pendingAlerts: 0,
  });
  const [isMonitoring, setIsMonitoring] = useState(true);
  const wsRef = useRef<WebSocket | null>(null);
  const detectionIntervalRef = useRef<NodeJS.Timeout | null>(null);

  // Initialize fraud detection
  const initializeFraudDetection = useCallback(() => {
    // NOTE: Mock alerts removed - connect to real blockchain data
    // TODO: Fetch real fraud alerts from Azure blockchain node
    // Example: await fetch('ws://20.186.91.207:9944')
    const initialAlerts: FraudAlert[] = [];
    setAlerts(initialAlerts);

    // Calculate initial stats
    updateStats(initialAlerts);
  }, []);

  // Update fraud statistics
  const updateStats = useCallback((alertList: FraudAlert[]) => {
    const totalDetections = alertList.length;
    const successfulInterventions = alertList.filter(
      (alert) => alert.resolved && alert.penaltyAmount
    ).length;
    const falsePositives = Math.floor(totalDetections * 0.05); // Assume 5% false positive rate
    const accuracy = totalDetections > 0
      ? ((totalDetections - falsePositives) / totalDetections) * 100
      : 0;
    const pendingAlerts = alertList.filter((alert) => !alert.resolved).length;

    setFraudStats({
      totalDetections,
      successfulInterventions,
      falsePositives,
      accuracy,
      pendingAlerts,
    });
  }, []);

  // Detect fraud in channels
  const detectFraud = useCallback(async (channelId: string) => {
    try {
      // TODO: Connect to Azure blockchain at ws://20.186.91.207:9944
      // TODO: Query watchtower pallet for fraud proofs on this channel
      // TODO: Analyze blockchain state for actual fraud attempts

      // NOTE: Mock data generation disabled - connect to real blockchain
      console.log(`Fraud detection for channel ${channelId} - awaiting blockchain connection`);

      return { fraudDetected: false };
    } catch (error) {
      console.error('Fraud detection error:', error);
      return { fraudDetected: false, error };
    }
  }, [updateStats]);

  // Intervene in fraudulent activity
  const intervene = useCallback(async (alertId: string) => {
    try {
      // In production, this would broadcast a penalty transaction
      // const response = await fetch(`/api/intervene/${alertId}`, {
      //   method: 'POST',
      // });
      // const result = await response.json();

      console.log(`Intervening in fraud alert: ${alertId}`);

      // Update alert status
      setAlerts((prev) =>
        prev.map((alert) =>
          alert.id === alertId
            ? { ...alert, resolved: true, penaltyAmount: alert.penaltyAmount || 1000 }
            : alert
        )
      );

      return { success: true };
    } catch (error) {
      console.error('Intervention error:', error);
      return { success: false, error };
    }
  }, []);

  // Mark alert as false positive
  const markAsFalsePositive = useCallback(async (alertId: string) => {
    try {
      console.log(`Marking alert ${alertId} as false positive`);

      // Update alert
      setAlerts((prev) => {
        const updated = prev.map((alert) =>
          alert.id === alertId
            ? { ...alert, resolved: true, penaltyAmount: undefined }
            : alert
        );
        updateStats(updated);
        return updated;
      });

      return { success: true };
    } catch (error) {
      console.error('Failed to mark as false positive:', error);
      return { success: false, error };
    }
  }, [updateStats]);

  // Send notification for fraud alert
  const sendNotification = useCallback((alert: FraudAlert) => {
    const config = JSON.parse(localStorage.getItem('watchtower-config') || '{}');

    // Email notification
    if (config.notifications?.email) {
      console.log('Sending email notification for alert:', alert.id);
      // In production: send email via API
    }

    // Push notification
    if (config.notifications?.push && 'Notification' in window) {
      if (Notification.permission === 'granted') {
        new Notification('Fraud Alert Detected', {
          body: alert.description,
          icon: '/icon.png',
          tag: alert.id,
        });
      }
    }

    // Webhook notification
    if (config.notifications?.webhook) {
      fetch(config.notifications.webhook, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(alert),
      }).catch((error) => console.error('Webhook notification failed:', error));
    }
  }, []);

  // Start continuous monitoring
  const startMonitoring = useCallback(() => {
    setIsMonitoring(true);

    // Set up periodic fraud detection
    detectionIntervalRef.current = setInterval(() => {
      // TODO: Connect to Azure blockchain node at ws://20.186.91.207:9944
      // TODO: Query real fraud alerts from on-chain watchtower pallet
      // TODO: Process and display real alerts instead of mock data
      // NOTE: Mock alert generation has been disabled
    }, 10000); // Check every 10 seconds
  }, [updateStats, sendNotification]);

  // Stop monitoring
  const stopMonitoring = useCallback(() => {
    setIsMonitoring(false);

    if (detectionIntervalRef.current) {
      clearInterval(detectionIntervalRef.current);
      detectionIntervalRef.current = null;
    }
  }, []);

  // Get alerts by severity
  const getAlertsBySeverity = useCallback((severity: FraudAlert['severity']) => {
    return alerts.filter((alert) => alert.severity === severity);
  }, [alerts]);

  // Get unresolved alerts
  const getUnresolvedAlerts = useCallback(() => {
    return alerts.filter((alert) => !alert.resolved);
  }, [alerts]);

  // Initialize on mount
  useEffect(() => {
    initializeFraudDetection();
    startMonitoring();

    // Request notification permission
    if ('Notification' in window && Notification.permission === 'default') {
      Notification.requestPermission();
    }

    return () => {
      stopMonitoring();
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [initializeFraudDetection, startMonitoring, stopMonitoring]);

  return {
    alerts,
    fraudStats,
    isMonitoring,
    detectFraud,
    intervene,
    markAsFalsePositive,
    startMonitoring,
    stopMonitoring,
    getAlertsBySeverity,
    getUnresolvedAlerts,
  };
}
