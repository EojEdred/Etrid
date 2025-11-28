import React from 'react';
import {View, Text, StyleSheet, ScrollView, TouchableOpacity} from 'react-native';
import LinearGradient from 'react-native-linear-gradient';
import Icon from 'react-native-vector-icons/Feather';
import {colors, theme} from '@/theme';

export default function MoreScreen({navigation}: any) {
  return (
    <LinearGradient
      colors={[colors.gradientStart, colors.gradientEnd]}
      style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <Text style={styles.headerTitle}>More</Text>
        <TouchableOpacity onPress={() => navigation.navigate('Settings')}>
          <Icon name="settings" size={24} color={colors.text} />
        </TouchableOpacity>
      </View>

      <ScrollView style={styles.content}>
        {/* Finance Section */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Finance</Text>
          <MenuItem
            icon="credit-card"
            title="AU Bloccard"
            subtitle="Over-collateralized debit card"
            color={colors.primary}
            onPress={() => navigation.navigate('AUBloccard')}
          />
          <MenuItem
            icon="target"
            title="Savings Goals"
            subtitle="Automated savings with interest"
            color={colors.info}
            onPress={() => navigation.navigate('SavingsGoals')}
          />
          <MenuItem
            icon="dollar-sign"
            title="Fiat On/Off Ramp"
            subtitle="Buy crypto with fiat"
            color={colors.success}
            onPress={() => navigation.navigate('FiatRamp')}
          />
        </View>

        {/* Security Section */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Security & Privacy</Text>
          <MenuItem
            icon="shield"
            title="Security"
            subtitle="Biometrics, 2FA, limits"
            color={colors.warning}
            onPress={() => navigation.navigate('Security')}
          />
          <MenuItem
            icon="eye-off"
            title="Privacy"
            subtitle="Stealth addresses, mixing"
            color={colors.textSecondary}
            onPress={() => navigation.navigate('Privacy')}
          />
        </View>

        {/* Features Section */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Advanced Features</Text>
          <MenuItem
            icon="users"
            title="Multi-Signature"
            subtitle="Shared wallet management"
            color={colors.primary}
          />
          <MenuItem
            icon="briefcase"
            title="Business Tools"
            subtitle="Team, payroll, invoices"
            color={colors.info}
          />
          <MenuItem
            icon="pie-chart"
            title="Analytics"
            subtitle="Portfolio insights & tax"
            color={colors.success}
          />
        </View>

        {/* Support Section */}
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Support</Text>
          <MenuItem
            icon="help-circle"
            title="Help Center"
            subtitle="FAQs and guides"
            color={colors.textSecondary}
          />
          <MenuItem
            icon="message-circle"
            title="Contact Support"
            subtitle="Get help from our team"
            color={colors.textSecondary}
          />
        </View>

        {/* App Info */}
        <View style={styles.appInfo}>
          <Text style={styles.appVersion}>Ëtrid Wallet v1.0.0</Text>
          <Text style={styles.appDescription}>
            The most advanced DeFi mobile wallet
          </Text>
        </View>
      </ScrollView>
    </LinearGradient>
  );
}

function MenuItem({icon, title, subtitle, color, onPress}: any) {
  return (
    <TouchableOpacity style={styles.menuItem} onPress={onPress}>
      <View style={[styles.menuIcon, {backgroundColor: color + '33'}]}>
        <Icon name={icon} size={20} color={color} />
      </View>
      <View style={styles.menuInfo}>
        <Text style={styles.menuTitle}>{title}</Text>
        <Text style={styles.menuSubtitle}>{subtitle}</Text>
      </View>
      <Icon name="chevron-right" size={20} color={colors.textSecondary} />
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
  headerTitle: {
    fontSize: theme.fontSize.xxl,
    fontWeight: theme.fontWeight.bold,
    color: colors.text,
  },
  content: {
    flex: 1,
  },
  section: {
    padding: theme.spacing.md,
  },
  sectionTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.bold,
    color: colors.textSecondary,
    marginBottom: theme.spacing.md,
    textTransform: 'uppercase',
    letterSpacing: 1,
  },
  menuItem: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: colors.glass,
    padding: theme.spacing.md,
    borderRadius: theme.borderRadius.md,
    marginBottom: theme.spacing.sm,
  },
  menuIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: theme.spacing.sm,
  },
  menuInfo: {
    flex: 1,
  },
  menuTitle: {
    fontSize: theme.fontSize.md,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  menuSubtitle: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
  appInfo: {
    padding: theme.spacing.xl,
    alignItems: 'center',
  },
  appVersion: {
    fontSize: theme.fontSize.sm,
    fontWeight: theme.fontWeight.semibold,
    color: colors.text,
    marginBottom: 4,
  },
  appDescription: {
    fontSize: theme.fontSize.xs,
    color: colors.textSecondary,
  },
});
