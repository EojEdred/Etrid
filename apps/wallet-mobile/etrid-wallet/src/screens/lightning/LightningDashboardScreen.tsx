import React from 'react';
import { View, Text, StyleSheet, FlatList, TouchableOpacity, RefreshControl } from 'react-native';
import { colors, spacing, typography, borderRadius } from '../../theme/theme';
import { useLightning } from '../../hooks/useLightning';

export default function LightningDashboardScreen({ navigation }: any) {
  const { channels, stats, loading, refresh } = useLightning();

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Lightning-Bloc</Text>
      <Text style={styles.subtitle}>Instant Payments</Text>

      {stats && (
        <View style={styles.statsCard}>
          <View style={styles.statRow}>
            <View style={styles.stat}>
              <Text style={styles.statLabel}>Total Channels</Text>
              <Text style={styles.statValue}>{stats.totalChannels}</Text>
            </View>
            <View style={styles.stat}>
              <Text style={styles.statLabel}>Active</Text>
              <Text style={[styles.statValue, { color: colors.success }]}>
                {stats.activeChannels}
              </Text>
            </View>
          </View>
          <View style={styles.statRow}>
            <View style={styles.stat}>
              <Text style={styles.statLabel}>Total Capacity</Text>
              <Text style={styles.statValue}>{stats.totalCapacityETR.toFixed(2)} ETR</Text>
            </View>
            <View style={styles.stat}>
              <Text style={styles.statLabel}>Avg Fee</Text>
              <Text style={styles.statValue}>{stats.averageFee.toFixed(2)}%</Text>
            </View>
          </View>
        </View>
      )}

      <View style={styles.header}>
        <Text style={styles.sectionTitle}>Active Channels</Text>
        <TouchableOpacity onPress={() => navigation.navigate('OpenChannel')}>
          <Text style={styles.addButton}>+ Open</Text>
        </TouchableOpacity>
      </View>

      <FlatList
        data={channels}
        renderItem={({ item }) => (
          <View style={styles.channelCard}>
            <View style={styles.channelHeader}>
              <Text style={styles.channelTitle}>
                {item.counterpartyName || 'Channel ' + item.id.slice(0, 8)}
              </Text>
              <View style={[
                styles.statusBadge,
                { backgroundColor: item.isActive ? colors.success : colors.gray400 }
              ]}>
                <Text style={styles.statusText}>{item.status.toUpperCase()}</Text>
              </View>
            </View>

            <View style={styles.balanceRow}>
              <View style={styles.balanceItem}>
                <Text style={styles.balanceLabel}>Your Balance</Text>
                <Text style={styles.balanceValue}>
                  {item.localBalanceETR.toFixed(2)} ETR
                </Text>
              </View>
              <View style={styles.balanceItem}>
                <Text style={styles.balanceLabel}>Capacity</Text>
                <Text style={styles.balanceValue}>
                  {item.capacityETR.toFixed(2)} ETR
                </Text>
              </View>
            </View>

            <View style={styles.capacityBar}>
              <View
                style={[
                  styles.capacityFill,
                  { width: `${(item.localBalanceETR / item.capacityETR) * 100}%` }
                ]}
              />
            </View>
          </View>
        )}
        keyExtractor={item => item.id}
        refreshControl={
          <RefreshControl refreshing={loading} onRefresh={refresh} tintColor={colors.primary} />
        }
        contentContainerStyle={styles.list}
        ListEmptyComponent={
          <View style={styles.emptyState}>
            <Text style={styles.emptyText}>No channels yet</Text>
            <Text style={styles.emptySubtext}>Open a channel to start making instant payments</Text>
            <TouchableOpacity
              style={styles.openButton}
              onPress={() => navigation.navigate('OpenChannel')}
            >
              <Text style={styles.openButtonText}>Open Channel</Text>
            </TouchableOpacity>
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
    paddingHorizontal: spacing.lg,
    paddingTop: spacing.lg,
  },
  subtitle: {
    ...typography.body,
    color: colors.textSecondary,
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  statsCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  statRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: spacing.sm,
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
    ...typography.h3,
    color: colors.text,
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
  addButton: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
  },
  list: {
    paddingHorizontal: spacing.lg,
    paddingBottom: spacing.xl,
  },
  channelCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.md,
    marginBottom: spacing.md,
  },
  channelHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  channelTitle: {
    ...typography.h3,
    color: colors.text,
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
  balanceRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    marginBottom: spacing.sm,
  },
  balanceItem: {},
  balanceLabel: {
    ...typography.caption,
    color: colors.textSecondary,
    marginBottom: 4,
  },
  balanceValue: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
  },
  capacityBar: {
    height: 6,
    backgroundColor: colors.gray200,
    borderRadius: borderRadius.full,
    overflow: 'hidden',
  },
  capacityFill: {
    height: '100%',
    backgroundColor: colors.primary,
  },
  emptyState: {
    padding: spacing.xl,
    alignItems: 'center',
  },
  emptyText: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.xs,
  },
  emptySubtext: {
    ...typography.body,
    color: colors.textSecondary,
    textAlign: 'center',
    marginBottom: spacing.lg,
  },
  openButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
  },
  openButtonText: {
    ...typography.body,
    color: colors.background,
    fontWeight: '600',
  },
});
