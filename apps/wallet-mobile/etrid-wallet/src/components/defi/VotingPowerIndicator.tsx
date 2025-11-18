import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { ConvictionLevel, CONVICTION_LEVELS } from '../../types/defi.types';

interface VotingPowerIndicatorProps {
  balance: number;
  conviction: ConvictionLevel;
}

export function VotingPowerIndicator({ balance, conviction }: VotingPowerIndicatorProps) {
  const convictionInfo = CONVICTION_LEVELS.find(c => c.level === conviction);
  const votingPower = balance * (convictionInfo?.multiplier || 1);

  return (
    <View style={styles.container}>
      <View style={styles.row}>
        <Text style={styles.label}>Available Balance</Text>
        <Text style={styles.value}>{balance.toFixed(2)} ETR</Text>
      </View>

      <View style={styles.row}>
        <Text style={styles.label}>Conviction Multiplier</Text>
        <Text style={[styles.value, { color: colors.primary }]}>
          {convictionInfo?.multiplier || 1}x
        </Text>
      </View>

      <View style={styles.divider} />

      <View style={styles.row}>
        <Text style={styles.totalLabel}>Total Voting Power</Text>
        <Text style={styles.totalValue}>{votingPower.toFixed(2)}</Text>
      </View>

      {conviction > 0 && (
        <Text style={styles.lockInfo}>
          Tokens will be locked for {convictionInfo?.lockDays} days
        </Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginVertical: spacing.md,
  },
  row: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  label: {
    ...typography.body,
    color: colors.textSecondary,
  },
  value: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
  },
  divider: {
    height: 1,
    backgroundColor: colors.gray200,
    marginVertical: spacing.sm,
  },
  totalLabel: {
    ...typography.h3,
    color: colors.text,
  },
  totalValue: {
    ...typography.h2,
    color: colors.primary,
    fontWeight: 'bold',
  },
  lockInfo: {
    ...typography.caption,
    color: colors.warning,
    marginTop: spacing.sm,
    textAlign: 'center',
  },
});
