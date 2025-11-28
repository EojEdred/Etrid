import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  Dimensions,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

const {width} = Dimensions.get('window');

const MARKETS = [
  {id: '1', pair: 'ÉTR/EDSC', price: '8.00', change: '+2.5%', volume: '1.2M'},
  {id: '2', pair: 'ÉTR/DOT', price: '1.25', change: '-1.2%', volume: '850K'},
  {id: '3', pair: 'EDSC/USDT', price: '1.00', change: '+0.1%', volume: '2.5M'},
];

export default function TradingScreen({navigation}: any) {
  const [activeTab, setActiveTab] = useState<'spot' | 'swap'>('spot');

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <Text style={styles.headerTitle}>Trade</Text>
        <TouchableOpacity>
          <Icon name="activity" size={24} color={colors.text} />
        </TouchableOpacity>
      </View>

      {/* Balance Overview */}
      <View style={styles.balanceCard}>
        <View style={styles.balanceRow}>
          <View>
            <Text style={styles.balanceLabel}>Trading Balance</Text>
            <Text style={styles.balanceAmount}>$5,234.56</Text>
          </View>
          <TouchableOpacity style={styles.depositButton}>
            <Icon name="plus" size={20} color={colors.text} />
            <Text style={styles.depositButtonText}>Deposit</Text>
          </TouchableOpacity>
        </View>
      </View>

      {/* Tabs */}
      <View style={styles.tabs}>
        <TouchableOpacity
          style={[styles.tab, activeTab === 'spot' && styles.tabActive]}
          onPress={() => setActiveTab('spot')}>
          <Text
            style={[
              styles.tabText,
              activeTab === 'spot' && styles.tabTextActive,
            ]}>
            Spot Trading
          </Text>
        </TouchableOpacity>
        <TouchableOpacity
          style={[styles.tab, activeTab === 'swap' && styles.tabActive]}
          onPress={() => setActiveTab('swap')}>
          <Text
            style={[
              styles.tabText,
              activeTab === 'swap' && styles.tabTextActive,
            ]}>
            Quick Swap
          </Text>
        </TouchableOpacity>
      </View>

      <ScrollView style={styles.content}>
        {/* Markets */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Markets</Text>
          {MARKETS.map(market => (
            <MarketCard key={market.id} market={market} />
          ))}
        </View>

        {/* Quick Actions */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Quick Actions</Text>
          <View style={styles.quickActions}>
            <ActionCard
              icon="repeat"
              title="Swap"
              subtitle="Exchange tokens"
              color={colors.primary}
              onPress={() => navigation.navigate('Swap')}
            />
            <ActionCard
              icon="trending-up"
              title="Lending"
              subtitle="Earn APY"
              color={colors.success}
              onPress={() => navigation.navigate('Lending')}
            />
          </View>
        </View>

        {/* Recent Trades */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Recent Trades</Text>
          <TradeItem type="buy" pair="ÉTR/EDSC" amount="50 ÉTR" price="8.00" time="2h ago" />
          <TradeItem type="sell" pair="ÉTR/DOT" amount="25 ÉTR" price="1.25" time="5h ago" />
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

function MarketCard({market}: any) {
  const isPositive = market.change.startsWith('+');

  return (
    <TouchableOpacity style={styles.marketCard}>
      <View style={styles.marketInfo}>
        <Text style={styles.marketPair}>{market.pair}</Text>
        <Text style={styles.marketVolume}>Vol: {market.volume}</Text>
      </View>
      <View style={styles.marketPriceContainer}>
        <Text style={styles.marketPrice}>${market.price}</Text>
        <View
          style={[
            styles.marketChange,
            {backgroundColor: isPositive ? colors.success + '33' : colors.error + '33'},
          ]}>
          <Icon
            name={isPositive ? 'trending-up' : 'trending-down'}
            size={12}
            color={isPositive ? colors.success : colors.error}
          />
          <Text
            style={[
              styles.marketChangeText,
              {color: isPositive ? colors.success : colors.error},
            ]}>
            {market.change}
          </Text>
        </View>
      </View>
    </TouchableOpacity>
  );
}

function ActionCard({icon, title, subtitle, color, onPress}: any) {
  return (
    <TouchableOpacity style={styles.actionCard} onPress={onPress}>
      <View style={[styles.actionIcon, {backgroundColor: color + '33'}]}>
        <Icon name={icon} size={24} color={color} />
      </View>
      <Text style={styles.actionTitle}>{title}</Text>
      <Text style={styles.actionSubtitle}>{subtitle}</Text>
    </TouchableOpacity>
  );
}

function TradeItem({type, pair, amount, price, time}: any) {
  const isBuy = type === 'buy';

  return (
    <View style={styles.tradeItem}>
      <View style={[styles.tradeType, {backgroundColor: isBuy ? colors.success + '33' : colors.error + '33'}]}>
        <Text style={[styles.tradeTypeText, {color: isBuy ? colors.success : colors.error}]}>
          {type.toUpperCase()}
        </Text>
      </View>
      <View style={styles.tradeInfo}>
        <Text style={styles.tradePair}>{pair}</Text>
        <Text style={styles.tradeTime}>{time}</Text>
      </View>
      <View style={styles.tradeDetails}>
        <Text style={styles.tradeAmount}>{amount}</Text>
        <Text style={styles.tradePrice}>@ {price}</Text>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
  header: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    padding: theme.spacing.md,
    paddingTop: 50,
  },
  headerTitle: {
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  balanceCard: {
    margin: theme.spacing.md,
    padding: theme.spacing.md,
    backgroundColor: colors.glassStrong,
    borderRadius: theme.borderRadius.md,
  },
  balanceRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  balanceLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: 4,
  },
  balanceAmount: {
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  depositButton: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 8,
    backgroundColor: colors.primary,
    paddingHorizontal: 16,
    paddingVertical: 10,
    borderRadius: theme.borderRadius.sm,
  },
  depositButtonText: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
  },
  tabs: {
    flexDirection: 'row',
    paddingHorizontal: theme.spacing.md,
    gap: theme.spacing.sm,
  },
  tab: {
    flex: 1,
    paddingVertical: 12,
    alignItems: 'center',
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
  },
  tabActive: {
    backgroundColor: colors.primary,
  },
  tabText: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.textSecondary,
  },
  tabTextActive: {
    color: colors.text,
  },
  content: {
    flex: 1,
  },
  section: {
    padding: theme.spacing.md,
  },
  sectionTitle: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginBottom: theme.spacing.md,
  },
  marketCard: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  marketInfo: {
    flex: 1,
  },
  marketPair: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  marketVolume: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  marketPriceContainer: {
    alignItems: 'flex-end',
  },
  marketPrice: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginBottom: 4,
  },
  marketChange: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 4,
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: theme.borderRadius.sm,
  },
  marketChangeText: {
    fontSize: theme.fontSize.xs,
    fontWeight: theme.fontWeight.semibold,
  },
  quickActions: {
    flexDirection: 'row',
    gap: theme.spacing.sm,
  },
  actionCard: {
    flex: 1,
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    alignItems: 'center',
  },
  actionIcon: {
    width: 48,
    height: 48,
    borderRadius: 24,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: theme.spacing.sm,
  },
  actionTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  actionSubtitle: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  tradeItem: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  tradeType: {
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: theme.borderRadius.sm,
    marginRight: theme.spacing.sm,
  },
  tradeTypeText: {
    fontSize: theme.fontSize.xs,
    fontWeight: theme.fontWeight.bold,
  },
  tradeInfo: {
    flex: 1,
  },
  tradePair: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  tradeTime: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  tradeDetails: {
    alignItems: 'flex-end',
  },
  tradeAmount: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 2,
  },
  tradePrice: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
});
