import React from 'react';
import { View, Text, StyleSheet, Dimensions } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { RewardHistory } from '../../types/defi.types';

interface RewardsChartProps {
  rewards: RewardHistory[];
  days?: number;
}

const CHART_HEIGHT = 120;
const CHART_WIDTH = Dimensions.get('window').width - spacing.lg * 2 - spacing.md * 2;

export function RewardsChart({ rewards, days = 7 }: RewardsChartProps) {
  if (rewards.length === 0) {
    return (
      <View style={styles.container}>
        <Text style={styles.title}>Rewards History (Last {days} Days)</Text>
        <View style={styles.emptyState}>
          <Text style={styles.emptyText}>No rewards data yet</Text>
        </View>
      </View>
    );
  }

  // Get last N days of rewards
  const recentRewards = rewards.slice(-days);
  const maxReward = Math.max(...recentRewards.map(r => r.amountETR));

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Rewards History (Last {days} Days)</Text>

      <View style={styles.chart}>
        {recentRewards.map((reward, index) => {
          const barHeight = (reward.amountETR / maxReward) * CHART_HEIGHT;
          const barWidth = CHART_WIDTH / recentRewards.length - 4;

          return (
            <View key={index} style={styles.barContainer}>
              <View style={styles.barWrapper}>
                <View
                  style={[
                    styles.bar,
                    {
                      height: barHeight,
                      width: barWidth,
                    },
                  ]}
                />
              </View>
            </View>
          );
        })}
      </View>

      <View style={styles.legend}>
        <Text style={styles.legendText}>
          Total: {recentRewards.reduce((sum, r) => sum + r.amountETR, 0).toFixed(2)} ETR
        </Text>
        <Text style={styles.legendText}>
          Avg: {(recentRewards.reduce((sum, r) => sum + r.amountETR, 0) / recentRewards.length).toFixed(2)} ETR/day
        </Text>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  title: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  chart: {
    height: CHART_HEIGHT,
    flexDirection: 'row',
    alignItems: 'flex-end',
    justifyContent: 'space-between',
    marginBottom: spacing.md,
  },
  barContainer: {
    flex: 1,
    height: CHART_HEIGHT,
    justifyContent: 'flex-end',
    alignItems: 'center',
  },
  barWrapper: {
    width: '100%',
    alignItems: 'center',
  },
  bar: {
    backgroundColor: colors.primary,
    borderTopLeftRadius: 4,
    borderTopRightRadius: 4,
  },
  legend: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingTop: spacing.sm,
    borderTopWidth: 1,
    borderTopColor: colors.gray200,
  },
  legendText: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  emptyState: {
    height: CHART_HEIGHT,
    justifyContent: 'center',
    alignItems: 'center',
  },
  emptyText: {
    ...typography.body,
    color: colors.textSecondary,
  },
});
