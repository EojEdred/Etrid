import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { Transaction } from '../services/TransactionService';
import { formatRelativeTime } from '../utils/formatters';
import { colors, spacing, typography, borderRadius } from '../theme/theme';

interface TransactionItemProps {
  transaction: Transaction;
  currentAddress: string;
  onPress?: () => void;
}

export const TransactionItem: React.FC<TransactionItemProps> = ({
  transaction,
  currentAddress,
  onPress,
}) => {
  const isReceived = transaction.to.toLowerCase() === currentAddress.toLowerCase();
  const isPositive = isReceived || transaction.type === 'reward';

  const getIcon = () => {
    switch (transaction.type) {
      case 'sent': return '↗️';
      case 'received': return '↙️';
      case 'staked': return '🔒';
      case 'unstaked': return '🔓';
      case 'reward': return '🎁';
      case 'bridge': return '🌉';
      default: return '💱';
    }
  };

  const getTitle = () => {
    switch (transaction.type) {
      case 'sent': return 'Sent';
      case 'received': return 'Received';
      case 'staked': return 'Staked';
      case 'unstaked': return 'Unstaked';
      case 'reward': return 'Reward';
      case 'bridge': return 'Bridge';
      default: return 'Transaction';
    }
  };

  const getSubtitle = () => {
    if (transaction.type === 'sent') return `To ${formatAddress(transaction.to)}`;
    if (transaction.type === 'received') return `From ${formatAddress(transaction.from)}`;
    return transaction.memo || formatRelativeTime(new Date(transaction.timestamp));
  };

  const formatAddress = (addr: string) => {
    return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
  };

  return (
    <TouchableOpacity
      style={styles.container}
      onPress={onPress}
      disabled={!onPress}
      activeOpacity={0.7}
    >
      <View style={styles.iconContainer}>
        <Text style={styles.icon}>{getIcon()}</Text>
      </View>

      <View style={styles.content}>
        <Text style={styles.title}>{getTitle()}</Text>
        <Text style={styles.subtitle}>{getSubtitle()}</Text>
      </View>

      <View style={styles.amountContainer}>
        <Text style={[styles.amount, isPositive ? styles.amountPositive : styles.amountNegative]}>
          {isPositive ? '+' : '-'}{transaction.amount} {transaction.currency}
        </Text>
        <Text style={styles.time}>{formatRelativeTime(new Date(transaction.timestamp))}</Text>
      </View>
    </TouchableOpacity>
  );
};

const styles = StyleSheet.create({
  container: {
    flexDirection: 'row',
    alignItems: 'center',
    paddingVertical: spacing.md,
    paddingHorizontal: spacing.lg,
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    marginBottom: spacing.sm,
  },
  iconContainer: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: colors.background,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: spacing.md,
  },
  icon: {
    fontSize: 20,
  },
  content: {
    flex: 1,
  },
  title: {
    ...typography.body,
    color: colors.text,
    fontWeight: '600',
    marginBottom: spacing.xs / 2,
  },
  subtitle: {
    ...typography.bodySmall,
    color: colors.textSecondary,
  },
  amountContainer: {
    alignItems: 'flex-end',
  },
  amount: {
    ...typography.body,
    fontWeight: '600',
    marginBottom: spacing.xs / 2,
  },
  amountPositive: {
    color: colors.success,
  },
  amountNegative: {
    color: colors.text,
  },
  time: {
    ...typography.caption,
    color: colors.textSecondary,
  },
});
