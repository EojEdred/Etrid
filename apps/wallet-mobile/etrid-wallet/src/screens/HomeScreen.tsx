import React from 'react';
import { View, Text, StyleSheet, ScrollView } from 'react-native';
import { Button, Card } from 'react-native-paper';
import { colors, spacing, typography } from '../theme/theme';

export default function HomeScreen({ navigation }: any) {
  return (
    <ScrollView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.greeting}>Welcome back, Eoj</Text>
      </View>

      <Card style={styles.balanceCard}>
        <Card.Content>
          <Text style={styles.label}>Total Balance</Text>
          <Text style={styles.balance}>$125,450.32</Text>
          <Text style={styles.change}>+$1,250.15 today (+1.0%) ↗</Text>
        </Card.Content>
      </Card>

      <View style={styles.quickActions}>
        <Text style={styles.sectionTitle}>Quick Actions</Text>
        <View style={styles.actionGrid}>
          <Button mode="contained" onPress={() => {}} style={styles.actionButton}>
            📤 Send
          </Button>
          <Button mode="contained" onPress={() => {}} style={styles.actionButton}>
            📥 Receive
          </Button>
          <Button mode="contained" onPress={() => {}} style={styles.actionButton}>
            🔄 Swap
          </Button>
          <Button mode="contained" onPress={() => {}} style={styles.actionButton}>
            📈 Stake
          </Button>
          <Button mode="contained" onPress={() => {}} style={styles.actionButton}>
            🏧 ATM
          </Button>
        </View>
      </View>

      <View style={styles.accounts}>
        <Text style={styles.sectionTitle}>My Accounts</Text>
        
        <Card style={styles.accountCard}>
          <Card.Content>
            <Text style={styles.accountType}>💳 Checking Account</Text>
            <Text style={styles.accountBalance}>$12,450.32</Text>
          </Card.Content>
        </Card>

        <Card style={styles.accountCard}>
          <Card.Content>
            <Text style={styles.accountType}>📈 Savings Account</Text>
            <Text style={styles.accountBalance}>$50,000.00</Text>
            <Text style={styles.accountInfo}>Earning 15% APY</Text>
          </Card.Content>
        </Card>

        <Card style={styles.accountCard}>
          <Card.Content>
            <Text style={styles.accountType}>🔒 Staking Account</Text>
            <Text style={styles.accountBalance}>10,000 ÉTR</Text>
            <Text style={styles.accountInfo}>+3.42 ÉTR daily</Text>
          </Card.Content>
        </Card>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  header: {
    padding: spacing.lg,
  },
  greeting: {
    ...typography.h2,
    color: colors.text,
  },
  balanceCard: {
    margin: spacing.lg,
    backgroundColor: colors.primary,
  },
  label: {
    ...typography.bodySmall,
    color: colors.textLight,
    marginBottom: spacing.xs,
  },
  balance: {
    ...typography.h1,
    color: '#FFFFFF',
    marginBottom: spacing.xs,
  },
  change: {
    ...typography.body,
    color: colors.success,
  },
  quickActions: {
    padding: spacing.lg,
  },
  sectionTitle: {
    ...typography.h3,
    color: colors.text,
    marginBottom: spacing.md,
  },
  actionGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: spacing.sm,
  },
  actionButton: {
    flex: 1,
    minWidth: '30%',
  },
  accounts: {
    padding: spacing.lg,
  },
  accountCard: {
    marginBottom: spacing.md,
  },
  accountType: {
    ...typography.body,
    color: colors.text,
    marginBottom: spacing.xs,
  },
  accountBalance: {
    ...typography.h3,
    color: colors.text,
  },
  accountInfo: {
    ...typography.bodySmall,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
});
