import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  RefreshControl,
  ActivityIndicator,
} from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useStaking } from '../../hooks/useStaking';
import { StakingCard } from '../../components/defi/StakingCard';
import { RewardsChart } from '../../components/defi/RewardsChart';

export default function StakingDashboardScreen({ navigation }: any) {
  const { stakingInfo, loading, refreshing, refresh, claimRewards } = useStaking();

  const handleClaimRewards = async () => {
    const result = await claimRewards();
    if (result.success) {
      alert('Rewards claimed successfully!');
    } else {
      alert(`Failed to claim rewards: ${result.error}`);
    }
  };

  if (loading && !stakingInfo) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={colors.primary} />
        <Text style={styles.loadingText}>Loading staking info...</Text>
      </View>
    );
  }

  return (
    <ScrollView
      style={styles.container}
      refreshControl={
        <RefreshControl refreshing={refreshing} onRefresh={refresh} tintColor={colors.primary} />
      }
    >
      <View style={styles.header}>
        <Text style={styles.title}>Staking</Text>
      </View>

      {/* Total Staked */}
      <View style={styles.totalCard}>
        <Text style={styles.label}>Total Staked</Text>
        <Text style={styles.totalAmount}>
          {stakingInfo?.totalStakedETR.toLocaleString(undefined, {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2,
          }) || '0.00'}{' '}
          ETR
        </Text>
        <Text style={styles.totalUSD}>
          ${stakingInfo?.totalStakedUSD.toLocaleString() || '0.00'}
        </Text>
      </View>

      {/* Stats Grid */}
      <View style={styles.statsGrid}>
        <View style={styles.statCard}>
          <Text style={styles.statLabel}>Current APY</Text>
          <Text style={[styles.statValue, { color: colors.success }]}>
            {stakingInfo?.currentAPY.toFixed(1) || '0.0'}%
          </Text>
        </View>

        <View style={styles.statCard}>
          <Text style={styles.statLabel}>Daily Rewards</Text>
          <Text style={styles.statValue}>
            +{stakingInfo?.dailyRewardsETR.toFixed(2) || '0.00'} ETR
          </Text>
        </View>

        <View style={styles.statCard}>
          <Text style={styles.statLabel}>Total Earned</Text>
          <Text style={styles.statValue}>
            {stakingInfo?.totalEarnedETR.toFixed(2) || '0.00'} ETR
          </Text>
          <Text style={styles.statSubtext}>
            ${stakingInfo?.totalEarnedUSD.toFixed(2) || '0.00'}
          </Text>
        </View>

        <View style={styles.statCard}>
          <Text style={styles.statLabel}>Unbonding</Text>
          <Text style={styles.statValue}>
            {stakingInfo?.unbondingPositions.length || 0}
          </Text>
          <Text style={styles.statSubtext}>
            {stakingInfo?.unbondingPeriod} days period
          </Text>
        </View>
      </View>

      {/* Rewards Chart */}
      {stakingInfo && stakingInfo.rewardsHistory.length > 0 && (
        <RewardsChart rewards={stakingInfo.rewardsHistory} days={7} />
      )}

      {/* Active Validators */}
      <View style={styles.section}>
        <View style={styles.sectionHeader}>
          <Text style={styles.sectionTitle}>Active Validators</Text>
          <TouchableOpacity onPress={() => navigation.navigate('ValidatorList')}>
            <Text style={styles.sectionLink}>View All</Text>
          </TouchableOpacity>
        </View>

        {stakingInfo?.activeValidators.map((validator, index) => (
          <StakingCard
            key={index}
            validatorStake={validator}
            onPress={() => navigation.navigate('ValidatorDetail', { address: validator.validatorAddress })}
          />
        ))}

        {stakingInfo?.activeValidators.length === 0 && (
          <View style={styles.emptyState}>
            <Text style={styles.emptyText}>No active validators</Text>
            <Text style={styles.emptySubtext}>Start staking to earn rewards</Text>
          </View>
        )}
      </View>

      {/* Actions */}
      <View style={styles.actions}>
        <TouchableOpacity
          style={[styles.actionButton, styles.primaryButton]}
          onPress={() => navigation.navigate('Stake')}
        >
          <Text style={styles.primaryButtonText}>Stake More</Text>
        </TouchableOpacity>

        <TouchableOpacity
          style={[styles.actionButton, styles.secondaryButton]}
          onPress={() => navigation.navigate('Unstake')}
        >
          <Text style={styles.secondaryButtonText}>Unstake</Text>
        </TouchableOpacity>
      </View>

      {stakingInfo && stakingInfo.totalEarnedETR > 0 && (
        <TouchableOpacity style={styles.claimButton} onPress={handleClaimRewards}>
          <Text style={styles.claimButtonText}>Claim Rewards</Text>
        </TouchableOpacity>
      )}

      <View style={{ height: spacing.xl }} />
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: colors.background,
  },
  loadingText: {
    ...typography.body,
    color: colors.textSecondary,
    marginTop: spacing.md,
  },
  header: {
    padding: spacing.lg,
    paddingBottom: spacing.md,
  },
  title: {
    ...typography.h1,
    color: colors.text,
  },
  totalCard: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.xl,
    padding: spacing.lg,
    margin: spacing.lg,
    marginTop: 0,
    alignItems: 'center',
  },
  label: {
    ...typography.body,
    color: colors.background + 'CC',
    marginBottom: spacing.xs,
  },
  totalAmount: {
    ...typography.h1,
    fontSize: 36,
    color: colors.background,
    marginBottom: spacing.xs,
  },
  totalUSD: {
    ...typography.h3,
    color: colors.background + 'CC',
  },
  statsGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  statCard: {
    width: '48%',
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginRight: '2%',
    marginBottom: spacing.sm,
  },
  statLabel: {
    ...typography.caption,
    color: colors.textSecondary,
    marginBottom: spacing.xs,
  },
  statValue: {
    ...typography.h3,
    color: colors.text,
    marginBottom: 4,
  },
  statSubtext: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  section: {
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  sectionTitle: {
    ...typography.h2,
    color: colors.text,
  },
  sectionLink: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
  },
  emptyState: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
  },
  emptyText: {
    ...typography.body,
    color: colors.text,
    marginBottom: spacing.xs,
  },
  emptySubtext: {
    ...typography.bodySmall,
    color: colors.textSecondary,
  },
  actions: {
    flexDirection: 'row',
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  actionButton: {
    flex: 1,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    alignItems: 'center',
  },
  primaryButton: {
    backgroundColor: colors.primary,
    marginRight: spacing.sm,
  },
  primaryButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
  secondaryButton: {
    backgroundColor: colors.surface,
    borderWidth: 2,
    borderColor: colors.primary,
  },
  secondaryButtonText: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
  },
  claimButton: {
    backgroundColor: colors.success,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginHorizontal: spacing.lg,
    alignItems: 'center',
  },
  claimButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
});
