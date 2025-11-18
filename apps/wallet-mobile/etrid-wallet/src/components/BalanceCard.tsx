import React from 'react';
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { colors, spacing, typography, borderRadius } from '../theme/theme';

interface BalanceCardProps {
  balance: string;
  usdValue: string;
  change24h?: number;
  onPress?: () => void;
  loading?: boolean;
}

export const BalanceCard: React.FC<BalanceCardProps> = ({
  balance,
  usdValue,
  change24h,
  onPress,
  loading = false,
}) => {
  const isPositive = (change24h || 0) >= 0;

  return (
    <TouchableOpacity onPress={onPress} disabled={!onPress} activeOpacity={0.9}>
      <LinearGradient
        colors={[colors.primary, '#8B7BE7']}
        start={{ x: 0, y: 0 }}
        end={{ x: 1, y: 1 }}
        style={styles.card}
      >
        <View style={styles.content}>
          <Text style={styles.label}>Total Balance</Text>
          {loading ? (
            <Text style={styles.balance}>Loading...</Text>
          ) : (
            <>
              <Text style={styles.balance}>{balance}</Text>
              <View style={styles.footer}>
                <Text style={styles.usdValue}>{usdValue}</Text>
                {change24h !== undefined && (
                  <View style={[styles.change, isPositive ? styles.changePositive : styles.changeNegative]}>
                    <Text style={styles.changeText}>
                      {isPositive ? '+' : ''}{change24h.toFixed(2)}%
                    </Text>
                  </View>
                )}
              </View>
            </>
          )}
        </View>
      </LinearGradient>
    </TouchableOpacity>
  );
};

const styles = StyleSheet.create({
  card: {
    borderRadius: borderRadius.lg,
    padding: spacing.lg,
    shadowColor: colors.primary,
    shadowOffset: { width: 0, height: 8 },
    shadowOpacity: 0.3,
    shadowRadius: 16,
    elevation: 12,
  },
  content: {},
  label: {
    ...typography.bodySmall,
    color: 'rgba(255, 255, 255, 0.9)',
    marginBottom: spacing.xs,
  },
  balance: {
    ...typography.h1,
    fontSize: 36,
    color: colors.background,
    marginBottom: spacing.xs,
  },
  footer: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
  },
  usdValue: {
    ...typography.body,
    color: 'rgba(255, 255, 255, 0.95)',
  },
  change: {
    paddingHorizontal: spacing.sm,
    paddingVertical: spacing.xs / 2,
    borderRadius: borderRadius.sm,
  },
  changePositive: {
    backgroundColor: 'rgba(0, 184, 148, 0.3)',
  },
  changeNegative: {
    backgroundColor: 'rgba(214, 48, 49, 0.3)',
  },
  changeText: {
    ...typography.bodySmall,
    color: colors.background,
    fontWeight: '600',
  },
});
