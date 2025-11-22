import React, {useState, useEffect} from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  TouchableOpacity,
  RefreshControl,
} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

export default function HomeScreen({navigation}: any) {
  const [refreshing, setRefreshing] = useState(false);
  const [totalBalance, setTotalBalance] = useState(12543.67);
  const [balanceChange, setBalanceChange] = useState(234.56);

  const onRefresh = async () => {
    setRefreshing(true);
    // Simulate API call
    setTimeout(() => {
      setRefreshing(false);
    }, 1000);
  };

  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      <ScrollView
        refreshControl={
          <RefreshControl refreshing={refreshing} onRefresh={onRefresh} />
        }>
        {/* Header */}
        <View style={styles.header}>
          <View>
            <Text style={styles.greeting}>Good Morning</Text>
            <Text style={styles.headerTitle}>Ëtrid Wallet</Text>
          </View>
          <View style={styles.headerIcons}>
            <TouchableOpacity
              style={styles.iconButton}
              onPress={() => navigation.navigate('Notifications')}>
              <Icon name="bell" size={24} color={colors.text} />
              <View style={styles.badge}>
                <Text style={styles.badgeText}>3</Text>
              </View>
            </TouchableOpacity>
            <TouchableOpacity
              style={styles.iconButton}
              onPress={() => navigation.navigate('Settings')}>
              <Icon name="settings" size={24} color={colors.text} />
            </TouchableOpacity>
          </View>
        </View>

        {/* Balance Card */}
        <View style={styles.balanceCard}>
          <Text style={styles.balanceLabel}>Total Balance</Text>
          <Text style={styles.balanceAmount}>
            ${totalBalance.toLocaleString('en-US', {minimumFractionDigits: 2})}
          </Text>
          <View style={styles.balanceChangeContainer}>
            <Icon name="trending-up" size={16} color={colors.success} />
            <Text style={styles.balanceChange}>
              +${balanceChange.toFixed(2)} (
              {((balanceChange / totalBalance) * 100).toFixed(1)}%) today
            </Text>
          </View>
        </View>

        {/* Quick Actions */}
        <View style={styles.quickActions}>
          <QuickAction
            icon="arrow-up"
            label="Send"
            onPress={() => navigation.navigate('Send')}
          />
          <QuickAction
            icon="arrow-down"
            label="Receive"
            onPress={() => navigation.navigate('Receive')}
          />
          <QuickAction
            icon="dollar-sign"
            label="Buy"
            onPress={() => navigation.navigate('FiatRamp')}
          />
          <QuickAction
            icon="repeat"
            label="Swap"
            onPress={() => navigation.navigate('Trade')}
          />
        </View>

        {/* Accounts Section */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Accounts</Text>

          <AccountCard
            icon="dollar-sign"
            name="Checking"
            balance="5,234.56 ÉTR"
            usdValue="$41,876.48"
            color={colors.etr}
            onPress={() => {}}
          />

          <AccountCard
            icon="credit-card"
            name="AU Bloccard"
            balance="$2,100 limit"
            subtitle="Health: 185%"
            color={colors.primary}
            onPress={() => navigation.navigate('More', {screen: 'AUBloccard'})}
          />

          <AccountCard
            icon="target"
            name="Savings"
            balance="$3,450 / $5,000"
            subtitle="Vacation - 69%"
            color={colors.info}
            onPress={() => navigation.navigate('More', {screen: 'SavingsGoals'})}
          />

          <AccountCard
            icon="trending-up"
            name="Lending"
            balance="2,500 ÉTR"
            subtitle="APY: 12.5%"
            color={colors.success}
            onPress={() => navigation.navigate('Trade', {screen: 'Lending'})}
          />
        </View>

        {/* Recent Transactions */}
        <View style={styles.section}>
          <View style={styles.sectionHeader}>
            <Text style={styles.sectionTitle}>Recent Transactions</Text>
            <TouchableOpacity>
              <Text style={styles.seeAllText}>See All</Text>
            </TouchableOpacity>
          </View>

          <TransactionItem
            type="send"
            title="Sent to @alice"
            amount="-50.00 ÉTR"
            usdAmount="-$400.00"
            time="2 hours ago"
            status="completed"
          />

          <TransactionItem
            type="receive"
            title="Received from @bob"
            amount="+100.00 EDSC"
            usdAmount="+$100.00"
            time="5 hours ago"
            status="completed"
          />

          <TransactionItem
            type="swap"
            title="Swapped ÉTR to EDSC"
            amount="200 ÉTR → 1,600 EDSC"
            time="1 day ago"
            status="completed"
          />
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

function QuickAction({icon, label, onPress}: any) {
  return (
    <TouchableOpacity style={styles.quickAction} onPress={onPress}>
      <View style={styles.quickActionIcon}>
        <Icon name={icon} size={24} color={colors.primary} />
      </View>
      <Text style={styles.quickActionLabel}>{label}</Text>
    </TouchableOpacity>
  );
}

function AccountCard({icon, name, balance, subtitle, usdValue, color, onPress}: any) {
  return (
    <TouchableOpacity style={styles.accountCard} onPress={onPress}>
      <View style={[styles.accountIcon, {backgroundColor: `${color}33`}]}>
        <Icon name={icon} size={20} color={color} />
      </View>
      <View style={styles.accountInfo}>
        <Text style={styles.accountName}>{name}</Text>
        <Text style={styles.accountBalance}>{balance}</Text>
        {subtitle && <Text style={styles.accountSubtitle}>{subtitle}</Text>}
        {usdValue && <Text style={styles.accountUsd}>{usdValue}</Text>}
      </View>
      <Icon name="chevron-right" size={20} color={colors.textSecondary} />
    </TouchableOpacity>
  );
}

function TransactionItem({type, title, amount, usdAmount, time, status}: any) {
  const getIcon = () => {
    switch (type) {
      case 'send':
        return 'arrow-up';
      case 'receive':
        return 'arrow-down';
      case 'swap':
        return 'repeat';
      default:
        return 'activity';
    }
  };

  const getIconColor = () => {
    switch (type) {
      case 'send':
        return colors.error;
      case 'receive':
        return colors.success;
      case 'swap':
        return colors.info;
      default:
        return colors.textSecondary;
    }
  };

  return (
    <TouchableOpacity style={styles.transactionItem}>
      <View style={[styles.transactionIcon, {backgroundColor: `${getIconColor()}33`}]}>
        <Icon name={getIcon()} size={18} color={getIconColor()} />
      </View>
      <View style={styles.transactionInfo}>
        <Text style={styles.transactionTitle}>{title}</Text>
        <Text style={styles.transactionTime}>{time}</Text>
      </View>
      <View style={styles.transactionAmountContainer}>
        <Text style={styles.transactionAmount}>{amount}</Text>
        {usdAmount && <Text style={styles.transactionUsd}>{usdAmount}</Text>}
      </View>
    </TouchableOpacity>
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
  greeting: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: 4,
  },
  headerTitle: {
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  headerIcons: {
    flexDirection: 'row',
    gap: theme.spacing.md,
  },
  iconButton: {
    position: 'relative',
  },
  badge: {
    position: 'absolute',
    top: -4,
    right: -4,
    backgroundColor: colors.error,
    borderRadius: 10,
    width: 18,
    height: 18,
    justifyContent: 'center',
    alignItems: 'center',
  },
  badgeText: {
    color: colors.text,
    fontSize: 10,
    fontWeight: theme.fontWeight.bold,
  },
  balanceCard: {
    margin: theme.spacing.md,
    padding: theme.spacing.lg,
    backgroundColor: colors.glassStrong,
    borderRadius: theme.borderRadius.xl,
    borderWidth: 1,
    borderColor: colors.border,
  },
  balanceLabel: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
    marginBottom: 8,
  },
  balanceAmount: {
    fontSize: 36,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginBottom: 8,
  },
  balanceChangeContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 4,
  },
  balanceChange: {
    fontSize: theme.fontSize.sm,
    color: colors.success,
  },
  quickActions: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    paddingHorizontal: theme.spacing.md,
    paddingVertical: theme.spacing.lg,
  },
  quickAction: {
    alignItems: 'center',
  },
  quickActionIcon: {
    width: 56,
    height: 56,
    borderRadius: 28,
    backgroundColor: colors.glass,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: 8,
  },
  quickActionLabel: {
    fontSize: theme.fontSize.xs,
    color: colors.text,
    fontWeight: theme.fontWeight.semibold,
  },
  section: {
    padding: theme.spacing.md,
  },
  sectionHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: theme.spacing.md,
  },
  sectionTitle: {
    fontSize: theme.fontSize.lg,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
    marginBottom: theme.spacing.md,
  },
  seeAllText: {
    fontSize: theme.fontSize.sm,
    color: colors.primary,
    fontWeight: theme.fontWeight.semibold,
  },
  accountCard: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: theme.spacing.md,
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  accountIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.sm,
  },
  accountInfo: {
    flex: 1,
  },
  accountName: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  accountBalance: {
    fontSize: theme.fontSize.sm,
    color: colors.textSecondary,
  },
  accountSubtitle: {
    fontSize: theme.fontSize.xs,
    color: colors.primary,
    marginTop: 2,
  },
  accountUsd: {
    fontSize: theme.fontSize.xs,
    color: colors.textMuted,
    marginTop: 2,
  },
  transactionItem: {
    flexDirection: 'row',
    alignItems: 'center',
    padding: theme.spacing.md,
    backgroundColor: colors.glass,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  transactionIcon: {
    width: 36,
    height: 36,
    borderRadius: 18,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.sm,
  },
  transactionInfo: {
    flex: 1,
  },
  transactionTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  transactionTime: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  transactionAmountContainer: {
    alignItems: 'flex-end',
  },
  transactionAmount: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 2,
  },
  transactionUsd: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
});
