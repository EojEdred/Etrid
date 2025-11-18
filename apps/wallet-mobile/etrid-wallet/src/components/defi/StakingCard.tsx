import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { ValidatorStake } from '../../types/defi.types';

interface StakingCardProps {
  validatorStake: ValidatorStake;
  onPress?: () => void;
}

export function StakingCard({ validatorStake, onPress }: StakingCardProps) {
  const getStatusColor = () => {
    switch (validatorStake.status) {
      case 'active':
        return colors.success;
      case 'waiting':
        return colors.warning;
      default:
        return colors.gray400;
    }
  };

  return (
    <TouchableOpacity
      style={styles.container}
      onPress={onPress}
      activeOpacity={onPress ? 0.7 : 1}
      disabled={!onPress}
    >
      <View style={styles.header}>
        <Text style={styles.validatorName}>{validatorStake.validatorName}</Text>
        <View style={[styles.statusBadge, { backgroundColor: getStatusColor() }]}>
          <Text style={styles.statusText}>{validatorStake.status.toUpperCase()}</Text>
        </View>
      </View>

      <View style={styles.statsRow}>
        <View style={styles.stat}>
          <Text style={styles.statLabel}>Staked</Text>
          <Text style={styles.statValue}>
            {validatorStake.stakedAmountETR.toLocaleString(undefined, {
              minimumFractionDigits: 2,
              maximumFractionDigits: 2,
            })} ETR
          </Text>
        </View>

        <View style={styles.stat}>
          <Text style={styles.statLabel}>APY</Text>
          <Text style={[styles.statValue, { color: colors.success }]}>
            {validatorStake.apy.toFixed(1)}%
          </Text>
        </View>

        <View style={styles.stat}>
          <Text style={styles.statLabel}>Commission</Text>
          <Text style={styles.statValue}>{validatorStake.commission}%</Text>
        </View>
      </View>

      <View style={styles.uptimeContainer}>
        <View style={styles.uptimeBar}>
          <View
            style={[styles.uptimeFill, { width: `${validatorStake.uptime}%` }]}
          />
        </View>
        <Text style={styles.uptimeText}>Uptime: {validatorStake.uptime}%</Text>
      </View>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  validatorName: {
    ...typography.h3,
    color: colors.text,
    flex: 1,
  },
  statusBadge: {
    paddingHorizontal: spacing.sm,
    paddingVertical: 4,
    borderRadius: borderRadius.sm,
  },
  statusText: {
    ...typography.caption,
    color: colors.background,
    fontWeight: '600',
  },
  statsRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: spacing.md,
  },
  stat: {
    flex: 1,
  },
  statLabel: {
    ...typography.caption,
    color: colors.textSecondary,
    marginBottom: 4,
  },
  statValue: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
  },
  uptimeContainer: {
    marginTop: spacing.sm,
  },
  uptimeBar: {
    height: 6,
    backgroundColor: colors.gray200,
    borderRadius: borderRadius.full,
    overflow: 'hidden',
    marginBottom: 4,
  },
  uptimeFill: {
    height: '100%',
    backgroundColor: colors.success,
  },
  uptimeText: {
    ...typography.caption,
    color: colors.textSecondary,
  },
});
