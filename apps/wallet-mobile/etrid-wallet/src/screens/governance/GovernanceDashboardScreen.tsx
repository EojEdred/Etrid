import React from 'react';
import { View, Text, StyleSheet, FlatList, RefreshControl, TouchableOpacity } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useGovernance } from '../../hooks/useGovernance';
import { ProposalCard } from '../../components/defi/ProposalCard';

export default function GovernanceDashboardScreen({ navigation }: any) {
  const { proposals, votingPower, loading, refresh } = useGovernance();

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Governance</Text>

      {votingPower && (
        <View style={styles.votingPowerCard}>
          <Text style={styles.vpLabel}>Your Voting Power</Text>
          <Text style={styles.vpValue}>{votingPower.totalVotingPower.toFixed(2)}</Text>
          <Text style={styles.vpSubtext}>
            {votingPower.availableBalanceETR.toFixed(2)} ETR Available
          </Text>
        </View>
      )}

      <View style={styles.header}>
        <Text style={styles.sectionTitle}>Active Proposals</Text>
        <Text style={styles.count}>{proposals.length}</Text>
      </View>

      <FlatList
        data={proposals}
        renderItem={({ item }) => (
          <ProposalCard
            proposal={item}
            onPress={() => navigation.navigate('ProposalDetail', { proposalId: item.id })}
          />
        )}
        keyExtractor={item => item.id.toString()}
        refreshControl={
          <RefreshControl refreshing={loading} onRefresh={refresh} tintColor={colors.primary} />
        }
        contentContainerStyle={styles.list}
        ListEmptyComponent={
          <View style={styles.emptyState}>
            <Text style={styles.emptyText}>No active proposals</Text>
          </View>
        }
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  title: {
    ...typography.h1,
    color: colors.text,
    padding: spacing.lg,
    paddingBottom: spacing.md,
  },
  votingPowerCard: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.xl,
    padding: spacing.lg,
    marginHorizontal: spacing.lg,
    marginBottom: spacing.md,
    alignItems: 'center',
  },
  vpLabel: {
    ...typography.body,
    color: colors.background + 'CC',
    marginBottom: spacing.xs,
  },
  vpValue: {
    ...typography.h1,
    fontSize: 36,
    color: colors.background,
    marginBottom: spacing.xs,
  },
  vpSubtext: {
    ...typography.bodySmall,
    color: colors.background + 'CC',
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  sectionTitle: {
    ...typography.h2,
    color: colors.text,
  },
  count: {
    ...typography.h3,
    color: colors.textSecondary,
  },
  list: {
    paddingHorizontal: spacing.lg,
    paddingBottom: spacing.xl,
  },
  emptyState: {
    padding: spacing.xl,
    alignItems: 'center',
  },
  emptyText: {
    ...typography.body,
    color: colors.textSecondary,
  },
});
