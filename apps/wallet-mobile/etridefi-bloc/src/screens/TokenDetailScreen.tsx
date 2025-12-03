import React from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

/**
 * TokenDetailScreen - Shows details for a specific token
 * Matches iOS Swift TokenDetailView
 */
export default function TokenDetailScreen({route, navigation}: any) {
  const {symbol = 'ETR', name = 'Etrid', balance = '0.00'} = route.params || {};

  const mockPriceData = {
    price: '$2.45',
    change24h: '+5.23%',
    high24h: '$2.52',
    low24h: '$2.31',
    volume24h: '$1.2M',
    marketCap: '$245M',
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView style={styles.scrollView}>
        {/* Token Header */}
        <View style={styles.header}>
          <View style={styles.tokenIcon}>
            <Text style={styles.tokenIconText}>{symbol[0]}</Text>
          </View>
          <Text style={styles.tokenName}>{name}</Text>
          <Text style={styles.tokenSymbol}>{symbol}</Text>
        </View>

        {/* Balance Card */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Your Balance</Text>
          <Text style={styles.balanceAmount}>{balance} {symbol}</Text>
          <Text style={styles.balanceValue}>≈ $0.00 USD</Text>
        </View>

        {/* Price Info */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Price Info</Text>
          <View style={styles.priceRow}>
            <Text style={styles.priceLabel}>Price</Text>
            <Text style={styles.priceValue}>{mockPriceData.price}</Text>
          </View>
          <View style={styles.priceRow}>
            <Text style={styles.priceLabel}>24h Change</Text>
            <Text style={[styles.priceValue, styles.positive]}>{mockPriceData.change24h}</Text>
          </View>
          <View style={styles.priceRow}>
            <Text style={styles.priceLabel}>24h High</Text>
            <Text style={styles.priceValue}>{mockPriceData.high24h}</Text>
          </View>
          <View style={styles.priceRow}>
            <Text style={styles.priceLabel}>24h Low</Text>
            <Text style={styles.priceValue}>{mockPriceData.low24h}</Text>
          </View>
          <View style={styles.priceRow}>
            <Text style={styles.priceLabel}>Volume</Text>
            <Text style={styles.priceValue}>{mockPriceData.volume24h}</Text>
          </View>
          <View style={styles.priceRow}>
            <Text style={styles.priceLabel}>Market Cap</Text>
            <Text style={styles.priceValue}>{mockPriceData.marketCap}</Text>
          </View>
        </View>

        {/* Action Buttons */}
        <View style={styles.actionsRow}>
          <TouchableOpacity
            style={styles.actionButton}
            onPress={() => navigation.navigate('Home', {screen: 'Send'})}>
            <Icon name="arrow-up-right" size={20} color={colors.text} />
            <Text style={styles.actionButtonText}>Send</Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={styles.actionButton}
            onPress={() => navigation.navigate('Home', {screen: 'Receive'})}>
            <Icon name="arrow-down-left" size={20} color={colors.text} />
            <Text style={styles.actionButtonText}>Receive</Text>
          </TouchableOpacity>
          <TouchableOpacity
            style={[styles.actionButton, styles.primaryButton]}
            onPress={() => navigation.navigate('DeFi', {screen: 'Swap'})}>
            <Icon name="repeat" size={20} color={colors.text} />
            <Text style={styles.actionButtonText}>Swap</Text>
          </TouchableOpacity>
        </View>

        {/* Transaction History Placeholder */}
        <View style={styles.card}>
          <Text style={styles.cardTitle}>Recent Transactions</Text>
          <View style={styles.emptyState}>
            <Icon name="clock" size={32} color={colors.textSecondary} />
            <Text style={styles.emptyText}>No transactions yet</Text>
          </View>
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  scrollView: {
    flex: 1,
    padding: theme.spacing.md,
  },
  header: {
    alignItems: 'center',
    marginBottom: theme.spacing.lg,
  },
  tokenIcon: {
    width: 64,
    height: 64,
    borderRadius: 32,
    backgroundColor: colors.primary,
    alignItems: 'center',
    justifyContent: 'center',
    marginBottom: theme.spacing.sm,
  },
  tokenIconText: {
    fontSize: 28,
    fontWeight: '700',
    color: colors.text,
  },
  tokenName: {
    fontSize: theme.fontSize.xl,
    fontWeight: '700',
    color: colors.text,
  },
  tokenSymbol: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
  card: {
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    padding: theme.spacing.md,
    marginBottom: theme.spacing.md,
  },
  cardTitle: {
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
    color: colors.textSecondary,
    marginBottom: theme.spacing.sm,
    textTransform: 'uppercase',
    letterSpacing: 0.5,
  },
  balanceAmount: {
    fontSize: 28,
    fontWeight: '700',
    color: colors.text,
  },
  balanceValue: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
    marginTop: 4,
  },
  priceRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: theme.spacing.xs,
  },
  priceLabel: {
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
  priceValue: {
    fontSize: theme.fontSize.md,
    fontWeight: '600',
    color: colors.text,
  },
  positive: {
    color: colors.success,
  },
  actionsRow: {
    flexDirection: 'row',
    gap: theme.spacing.sm,
    marginBottom: theme.spacing.md,
  },
  actionButton: {
    flex: 1,
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    gap: 8,
  },
  primaryButton: {
    backgroundColor: colors.primary,
  },
  actionButtonText: {
    fontSize: theme.fontSize.sm,
    fontWeight: '600',
    color: colors.text,
  },
  emptyState: {
    alignItems: 'center',
    paddingVertical: theme.spacing.xl,
  },
  emptyText: {
    marginTop: theme.spacing.sm,
    fontSize: theme.fontSize.md,
    color: colors.textSecondary,
  },
});
