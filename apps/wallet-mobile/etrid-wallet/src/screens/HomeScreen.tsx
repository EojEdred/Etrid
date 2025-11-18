import React from 'react';
import { View, Text, StyleSheet, ScrollView, TouchableOpacity, RefreshControl } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../contexts/AuthContext';
import { useBalance } from '../hooks/useBalance';
import { useTransactions } from '../hooks/useTransactions';
import { BalanceCard } from '../components/BalanceCard';
import { TransactionItem } from '../components/TransactionItem';
import { colors, spacing, typography, borderRadius } from '../theme/theme';

export default function HomeScreen() {
  const navigation = useNavigation<any>();
  const { address } = useAuth();
  const { balance, loading: balanceLoading, refresh: refreshBalance } = useBalance('ETR');
  const { transactions, loading: txLoading, refresh: refreshTransactions } = useTransactions(5);

  const handleRefresh = async () => {
    await Promise.all([refreshBalance(), refreshTransactions()]);
  };

  const isRefreshing = balanceLoading || txLoading;

  return (
    <ScrollView
      style={styles.container}
      refreshControl={<RefreshControl refreshing={isRefreshing} onRefresh={handleRefresh} />}
    >
      <View style={styles.header}>
        <Text style={styles.greeting}>Welcome back, Eoj</Text>
        <TouchableOpacity style={styles.notificationButton}>
          <Text style={styles.notificationIcon}>🔔</Text>
        </TouchableOpacity>
      </View>

      <View style={styles.balanceSection}>
        <BalanceCard
          balance={balance?.balanceFormatted || '0 ETR'}
          usdValue={balance?.usdValueFormatted || '$0.00'}
          change24h={balance?.change24hPercent}
          loading={balanceLoading}
        />
      </View>

      <View style={styles.quickActions}>
        <Text style={styles.sectionTitle}>Quick Actions</Text>
        <View style={styles.actionGrid}>
          <ActionButton
            icon="📤"
            label="Send"
            onPress={() => navigation.navigate('Send')}
          />
          <ActionButton
            icon="📥"
            label="Receive"
            onPress={() => navigation.navigate('Receive')}
          />
          <ActionButton
            icon="🔄"
            label="Swap"
            onPress={() => {}}
            disabled
          />
          <ActionButton
            icon="📈"
            label="Stake"
            onPress={() => navigation.navigate('AccountsScreen')}
          />
        </View>
      </View>

      <View style={styles.transactionsSection}>
        <View style={styles.sectionHeader}>
          <Text style={styles.sectionTitle}>Recent Transactions</Text>
          <TouchableOpacity onPress={() => navigation.navigate('TransactionHistory')}>
            <Text style={styles.seeAllText}>See All →</Text>
          </TouchableOpacity>
        </View>

        {transactions.length > 0 ? (
          transactions.slice(0, 5).map(tx => (
            <TransactionItem
              key={tx.id}
              transaction={tx}
              currentAddress={address || ''}
            />
          ))
        ) : (
          <View style={styles.emptyState}>
            <Text style={styles.emptyIcon}>💸</Text>
            <Text style={styles.emptyText}>No transactions yet</Text>
            <Text style={styles.emptySubtext}>Start by receiving or buying ETR</Text>
          </View>
        )}
      </View>
    </ScrollView>
  );
}

interface ActionButtonProps {
  icon: string;
  label: string;
  onPress: () => void;
  disabled?: boolean;
}

const ActionButton: React.FC<ActionButtonProps> = ({ icon, label, onPress, disabled }) => (
  <TouchableOpacity
    style={[styles.actionButton, disabled && styles.actionButtonDisabled]}
    onPress={onPress}
    disabled={disabled}
    activeOpacity={0.7}
    accessibilityLabel={label}
    accessibilityRole="button"
  >
    <Text style={styles.actionIcon}>{icon}</Text>
    <Text style={styles.actionLabel}>{label}</Text>
  </TouchableOpacity>
);

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: spacing.lg,
    paddingTop: spacing.xl,
  },
  greeting: {
    ...typography.h2,
    color: colors.text,
  },
  notificationButton: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: colors.surface,
    justifyContent: 'center',
    alignItems: 'center',
  },
  notificationIcon: {
    fontSize: 20,
  },
  balanceSection: {
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.lg,
  },
  quickActions: {
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.lg,
  },
  sectionTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  actionGrid: {
    flexDirection: 'row',
    gap: spacing.sm,
  },
  actionButton: {
    flex: 1,
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    padding: spacing.md,
    alignItems: 'center',
  },
  actionButtonDisabled: {
    opacity: 0.5,
  },
  actionIcon: {
    fontSize: 28,
    marginBottom: spacing.xs,
  },
  actionLabel: {
    ...typography.bodySmall,
    color: colors.text,
    fontWeight: '600',
  },
  transactionsSection: {
    paddingHorizontal: spacing.lg,
    paddingBottom: spacing.xl,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  seeAllText: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '600',
  },
  emptyState: {
    paddingVertical: spacing.xxl,
    alignItems: 'center',
  },
  emptyIcon: {
    fontSize: 64,
    marginBottom: spacing.md,
  },
  emptyText: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.xs,
  },
  emptySubtext: {
    ...typography.body,
    color: colors.textSecondary,
  },
});
