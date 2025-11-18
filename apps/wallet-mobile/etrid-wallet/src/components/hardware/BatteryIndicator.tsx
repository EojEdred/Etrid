/**
 * BatteryIndicator - Battery level display for hardware wallets
 * Ëtrid Mobile DeFi Wallet - Phase 3
 */

import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

interface BatteryIndicatorProps {
  level: number; // 0-100
  showPercentage?: boolean;
}

const BatteryIndicator: React.FC<BatteryIndicatorProps> = ({
  level,
  showPercentage = true,
}) => {
  const getBatteryColor = (level: number): string => {
    if (level > 50) return '#4CAF50';
    if (level > 20) return '#FF9800';
    return '#F44336';
  };

  const getBatteryIcon = (level: number): string => {
    if (level > 75) return '🔋';
    if (level > 50) return '🔋';
    if (level > 25) return '🪫';
    return '🪫';
  };

  return (
    <View style={styles.container}>
      {/* Battery Icon */}
      <View style={styles.batteryContainer}>
        <View style={styles.batteryBody}>
          <View
            style={[
              styles.batteryLevel,
              {
                width: `${level}%`,
                backgroundColor: getBatteryColor(level),
              },
            ]}
          />
        </View>
        <View style={styles.batteryTip} />
      </View>

      {/* Percentage */}
      {showPercentage && (
        <Text style={[styles.percentage, { color: getBatteryColor(level) }]}>
          {level}%
        </Text>
      )}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  batteryContainer: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  batteryBody: {
    width: 30,
    height: 16,
    borderWidth: 2,
    borderColor: '#666',
    borderRadius: 3,
    padding: 2,
    backgroundColor: '#f5f5f5',
  },
  batteryLevel: {
    height: '100%',
    borderRadius: 1,
  },
  batteryTip: {
    width: 2,
    height: 8,
    backgroundColor: '#666',
    marginLeft: 1,
    borderTopRightRadius: 2,
    borderBottomRightRadius: 2,
  },
  percentage: {
    fontSize: 12,
    fontWeight: '600',
    marginLeft: 6,
  },
});

export default BatteryIndicator;
