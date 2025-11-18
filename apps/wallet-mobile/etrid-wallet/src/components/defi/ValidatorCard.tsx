import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { Validator } from '../../types/defi.types';

interface ValidatorCardProps {
  validator: Validator;
  onPress?: () => void;
  selected?: boolean;
}

export function ValidatorCard({ validator, onPress, selected }: ValidatorCardProps) {
  return (
    <TouchableOpacity
      style={[styles.container, selected && styles.selected]}
      onPress={onPress}
      activeOpacity={0.7}
    >
      <View style={styles.header}>
        <View style={styles.info}>
          <Text style={styles.name}>{validator.name}</Text>
          <Text style={styles.address}>
            {validator.address.slice(0, 8)}...{validator.address.slice(-6)}
          </Text>
        </View>
        {validator.rank && (
          <View style={styles.rankBadge}>
            <Text style={styles.rankText}>#{validator.rank}</Text>
          </View>
        )}
      </View>

      <View style={styles.stats}>
        <View style={styles.statItem}>
          <Text style={styles.statLabel}>APY</Text>
          <Text style={[styles.statValue, { color: colors.success }]}>
            {validator.apy.toFixed(1)}%
          </Text>
        </View>
        <View style={styles.statItem}>
          <Text style={styles.statLabel}>Commission</Text>
          <Text style={styles.statValue}>{validator.commission}%</Text>
        </View>
        <View style={styles.statItem}>
          <Text style={styles.statLabel}>Uptime</Text>
          <Text style={styles.statValue}>{validator.uptime}%</Text>
        </View>
        <View style={styles.statItem}>
          <Text style={styles.statLabel}>Nominators</Text>
          <Text style={styles.statValue}>{validator.nominators}</Text>
        </View>
      </View>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.sm,
    borderWidth: 2,
    borderColor: 'transparent',
  },
  selected: {
    borderColor: colors.primary,
    backgroundColor: colors.primary + '10',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'flex-start',
    marginBottom: spacing.md,
  },
  info: {
    flex: 1,
  },
  name: {
    ...typography.h3,
    color: colors.text,
    marginBottom: 4,
  },
  address: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  rankBadge: {
    backgroundColor: colors.primary,
    paddingHorizontal: spacing.sm,
    paddingVertical: 4,
    borderRadius: borderRadius.sm,
  },
  rankText: {
    ...typography.caption,
    color: colors.background,
    fontWeight: '600',
  },
  stats: {
    flexDirection: 'row',
    justifyContent: 'space-between',
  },
  statItem: {
    alignItems: 'center',
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
});
