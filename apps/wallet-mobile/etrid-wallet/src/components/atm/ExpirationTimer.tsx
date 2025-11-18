/**
 * ExpirationTimer - Countdown timer for withdrawal expiration
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React, { useState, useEffect } from 'react';
import { View, Text, StyleSheet } from 'react-native';

interface ExpirationTimerProps {
  expiresAt: string;
  onExpire?: () => void;
}

const ExpirationTimer: React.FC<ExpirationTimerProps> = ({ expiresAt, onExpire }) => {
  const [timeRemaining, setTimeRemaining] = useState<number>(0);

  useEffect(() => {
    const calculateTimeRemaining = () => {
      const now = new Date().getTime();
      const expiry = new Date(expiresAt).getTime();
      const remaining = Math.max(0, expiry - now);

      setTimeRemaining(remaining);

      if (remaining === 0 && onExpire) {
        onExpire();
      }
    };

    calculateTimeRemaining();
    const interval = setInterval(calculateTimeRemaining, 1000);

    return () => clearInterval(interval);
  }, [expiresAt, onExpire]);

  const formatTime = (milliseconds: number): string => {
    const totalSeconds = Math.floor(milliseconds / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;

    return `${minutes}m ${seconds.toString().padStart(2, '0')}s`;
  };

  const getWarningLevel = (milliseconds: number): 'safe' | 'warning' | 'critical' => {
    const minutes = milliseconds / 1000 / 60;

    if (minutes > 10) return 'safe';
    if (minutes > 5) return 'warning';
    return 'critical';
  };

  const warningLevel = getWarningLevel(timeRemaining);
  const isExpired = timeRemaining === 0;

  const getTimerColor = (): string => {
    switch (warningLevel) {
      case 'safe':
        return '#4CAF50';
      case 'warning':
        return '#FF9800';
      case 'critical':
        return '#F44336';
      default:
        return '#666';
    }
  };

  return (
    <View style={styles.container}>
      <View
        style={[
          styles.timerContainer,
          { borderColor: getTimerColor() },
          isExpired && styles.expired,
        ]}
      >
        <Text style={styles.label}>
          {isExpired ? 'EXPIRED' : 'Expires in'}
        </Text>
        {!isExpired && (
          <Text style={[styles.timer, { color: getTimerColor() }]}>
            {formatTime(timeRemaining)}
          </Text>
        )}
      </View>

      {warningLevel === 'critical' && !isExpired && (
        <Text style={styles.warning}>
          Time is running out! Use your code soon.
        </Text>
      )}

      {isExpired && (
        <Text style={styles.expiredText}>
          This withdrawal code has expired. Please create a new one.
        </Text>
      )}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
    marginVertical: 16,
  },
  timerContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingHorizontal: 20,
    paddingVertical: 12,
    borderRadius: 24,
    borderWidth: 2,
    backgroundColor: '#fff',
  },
  expired: {
    borderColor: '#F44336',
    backgroundColor: '#FFEBEE',
  },
  label: {
    fontSize: 14,
    color: '#666',
    marginRight: 8,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  timer: {
    fontSize: 20,
    fontWeight: 'bold',
    fontFamily: 'monospace',
  },
  warning: {
    marginTop: 8,
    fontSize: 12,
    color: '#F44336',
    fontWeight: '600',
    textAlign: 'center',
  },
  expiredText: {
    marginTop: 8,
    fontSize: 14,
    color: '#F44336',
    textAlign: 'center',
    paddingHorizontal: 20,
  },
});

export default ExpirationTimer;
